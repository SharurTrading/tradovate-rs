// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Incremental array-record framing. The actor yields between records.

use tokio_tungstenite::tungstenite::Utf8Bytes;

use super::{Error, ServerMessage, decode};

pub(crate) struct RecordBatch {
    text: Utf8Bytes,
    offset: usize,
    remaining: usize,
    limit: usize,
    after_comma: bool,
    trailing_error: bool,
}

impl RecordBatch {
    pub(crate) fn new(text: Utf8Bytes, limit: usize) -> Result<Self, Error> {
        let payload = text
            .as_str()
            .strip_prefix('a')
            .ok_or(Error::EmptyServerFrame)?;
        let trimmed = payload.trim();
        if !trimmed.starts_with('[') || !trimmed.ends_with(']') {
            return Err(malformed());
        }
        let offset = 1 + payload.len() - payload.trim_start().len() + 1;
        Ok(Self {
            text,
            offset,
            remaining: limit,
            limit,
            after_comma: false,
            trailing_error: false,
        })
    }

    pub(crate) fn next(&mut self) -> Option<Result<ServerMessage, Error>> {
        if std::mem::take(&mut self.trailing_error) {
            return Some(Err(malformed()));
        }
        let tail = self.text.as_str().get(self.offset..)?.trim_start();
        if tail.trim() == "]" || tail.is_empty() {
            self.offset = self.text.len();
            return self.after_comma.then(|| {
                self.after_comma = false;
                Err(malformed())
            });
        }
        if self.remaining == 0 {
            self.offset = self.text.len();
            self.after_comma = false;
            return Some(Err(Error::TooManyMessages {
                actual_messages: self.limit.saturating_add(1),
                max_messages: self.limit,
            }));
        }
        self.remaining -= 1;
        let mut depth = 0_usize;
        let mut quoted = false;
        let mut escaped = false;
        for (index, byte) in tail.bytes().enumerate() {
            if quoted {
                if escaped {
                    escaped = false;
                } else if byte == b'\\' {
                    escaped = true;
                } else if byte == b'"' {
                    quoted = false;
                }
                continue;
            }
            match byte {
                b'"' => quoted = true,
                b'[' | b'{' => depth = depth.saturating_add(1),
                b',' | b']' if depth == 0 => {
                    let raw = tail.get(..index).unwrap_or_default();
                    let consumed = self.text.len() - tail.len() + index;
                    self.after_comma = byte == b',';
                    self.offset = if self.after_comma {
                        consumed + 1
                    } else {
                        self.text.len()
                    };
                    if !self.after_comma {
                        self.trailing_error = tail
                            .get(index + 1..)
                            .is_some_and(|tail| !tail.trim().is_empty());
                    }
                    let record = serde_json::from_str(raw)
                        .map_err(|_| malformed())
                        .and_then(decode::classify_message);
                    return Some(record);
                }
                b']' | b'}' => depth = depth.saturating_sub(1),
                _ => {}
            }
        }
        self.offset = self.text.len();
        Some(Err(malformed()))
    }
}

fn malformed() -> Error {
    Error::InvalidJson {
        kind: 'a',
        line: 1,
        column: 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn malformed_middle_record_preserves_later_completion() {
        let mut batch = RecordBatch::new(r#"a[{"d":bad},{"i":2,"s":200}]"#.into(), 4)
            .unwrap_or_else(|e| panic!("batch: {e}"));
        assert!(matches!(batch.next(), Some(Err(_))));
        assert!(matches!(batch.next(), Some(Ok(ServerMessage::Response(_)))));
        assert!(batch.next().is_none());
    }

    #[test]
    fn malformed_tails_end_without_spinning_or_claiming_continuity() {
        for text in ["a[{},]", "a[{}]trailing]", "a[{},broken]"] {
            let mut batch =
                RecordBatch::new(text.into(), 4).unwrap_or_else(|e| panic!("batch: {e}"));
            assert!(matches!(batch.next(), Some(Ok(_))));
            assert!(matches!(batch.next(), Some(Err(_))));
            assert!(batch.next().is_none());
        }
        let mut empty =
            RecordBatch::new("a[ ]  ".into(), 4).unwrap_or_else(|e| panic!("batch: {e}"));
        assert!(empty.next().is_none());
    }
}

// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Exact decoding and classification of JSON-bearing server frames.

use std::fmt;

use serde::{
    Deserialize,
    de::{DeserializeSeed, Error as _, IgnoredAny, SeqAccess, Visitor},
};
use serde_json::value::RawValue;

use super::{Error, Event, RequestId, Response, ServerFrame, ServerMessage};

pub(super) fn messages(payload: &str, max_messages_per_frame: usize) -> Result<ServerFrame, Error> {
    // Count structurally before allocation, including a lower bound for a
    // truncated array. The bounded Serde seed below remains the fail-closed
    // backstop when malformed nesting prevents an exact structural count.
    if let Some(actual_messages) = top_level_array_items(payload)
        && actual_messages > max_messages_per_frame
    {
        return Err(Error::TooManyMessages {
            actual_messages,
            max_messages: max_messages_per_frame,
        });
    }
    let values = bounded_raw_values(payload, max_messages_per_frame)
        .map_err(|error| json_error('a', &error))?;
    let messages = values
        .into_iter()
        .map(classify_message)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(ServerFrame::Messages(messages))
}

fn bounded_raw_values(
    payload: &str,
    max_messages: usize,
) -> Result<Vec<Box<RawValue>>, serde_json::Error> {
    let mut deserializer = serde_json::Deserializer::from_str(payload);
    let values = RawMessagesSeed { max_messages }.deserialize(&mut deserializer)?;
    deserializer.end()?;
    Ok(values)
}

struct RawMessagesSeed {
    max_messages: usize,
}

impl<'de> DeserializeSeed<'de> for RawMessagesSeed {
    type Value = Vec<Box<RawValue>>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(RawMessagesVisitor {
            max_messages: self.max_messages,
        })
    }
}

struct RawMessagesVisitor {
    max_messages: usize,
}

impl<'de> Visitor<'de> for RawMessagesVisitor {
    type Value = Vec<Box<RawValue>>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a bounded array of realtime messages")
    }

    fn visit_seq<A>(self, mut sequence: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let capacity = sequence.size_hint().unwrap_or(0).min(self.max_messages);
        let mut values = Vec::with_capacity(capacity);
        while values.len() < self.max_messages {
            let Some(value) = sequence.next_element::<Box<RawValue>>()? else {
                return Ok(values);
            };
            values.push(value);
        }

        // Probe beyond the limit without materializing another RawValue. A
        // malformed tail therefore cannot grow the exact-token buffer past
        // the configured message count before it is rejected.
        if sequence.next_element::<IgnoredAny>()?.is_some() {
            return Err(A::Error::custom("realtime message count limit exceeded"));
        }
        Ok(values)
    }
}

fn top_level_array_items(payload: &str) -> Option<usize> {
    let mut bytes = payload.bytes().skip_while(u8::is_ascii_whitespace);
    if bytes.next()? != b'[' {
        return None;
    }

    let mut depth = 1_usize;
    let mut in_string = false;
    let mut escaped = false;
    let mut item_started = false;
    let mut items = 0_usize;

    for byte in bytes {
        if in_string {
            if escaped {
                escaped = false;
            } else if byte == b'\\' {
                escaped = true;
            } else if byte == b'"' {
                in_string = false;
            }
            continue;
        }

        match byte {
            b'"' => {
                mark_item(depth, &mut item_started, &mut items);
                in_string = true;
            }
            b'[' | b'{' => {
                mark_item(depth, &mut item_started, &mut items);
                depth = depth.checked_add(1)?;
            }
            b']' if depth == 1 => return Some(items),
            b']' | b'}' if depth > 1 => depth -= 1,
            b',' if depth == 1 => item_started = false,
            byte if depth == 1 && !byte.is_ascii_whitespace() => {
                mark_item(depth, &mut item_started, &mut items);
            }
            _ => {}
        }
    }
    Some(items)
}

fn mark_item(depth: usize, item_started: &mut bool, items: &mut usize) {
    if depth == 1 && !*item_started {
        *item_started = true;
        *items = items.saturating_add(1);
    }
}

fn classify_message(raw: Box<RawValue>) -> Result<ServerMessage, Error> {
    let header: MessageHeader =
        serde_json::from_str(raw.get()).map_err(|error| json_error('a', &error))?;
    if header.request_id.is_some() || header.status.is_some() {
        let request_id = header
            .request_id
            .as_deref()
            .and_then(parse_unsigned)
            .map(RequestId::new)
            .ok_or(Error::InvalidResponseRequestId)?;
        let status = header
            .status
            .as_deref()
            .and_then(parse_unsigned)
            .and_then(|status| u16::try_from(status).ok())
            .ok_or(Error::InvalidResponseStatus)?;

        return Ok(ServerMessage::Response(Response {
            request_id,
            status,
            data: header.data,
        }));
    }

    let Some(event_kind) = header.event_kind else {
        return Ok(ServerMessage::Unknown(raw));
    };

    let event = match event_kind.as_str() {
        "props" => Event::Properties(header.data),
        "shutdown" => Event::Shutdown(header.data),
        "md" => Event::MarketData(header.data),
        "chart" => Event::Chart(header.data),
        "clock" => Event::Clock(header.data),
        _ => Event::Unknown {
            kind: event_kind,
            raw,
        },
    };

    Ok(ServerMessage::Event(event))
}

#[derive(Deserialize)]
struct MessageHeader {
    #[serde(rename = "i", default, deserialize_with = "present_raw")]
    request_id: Option<Box<RawValue>>,
    #[serde(rename = "s", default, deserialize_with = "present_raw")]
    status: Option<Box<RawValue>>,
    #[serde(rename = "e", default)]
    event_kind: Option<String>,
    #[serde(rename = "d", default, deserialize_with = "present_raw")]
    data: Option<Box<RawValue>>,
}

fn present_raw<'de, D>(deserializer: D) -> Result<Option<Box<RawValue>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Box::<RawValue>::deserialize(deserializer).map(Some)
}

fn parse_unsigned(raw: &RawValue) -> Option<u64> {
    serde_json::from_str(raw.get()).ok()
}

pub(super) fn close(payload: &str) -> Result<ServerFrame, Error> {
    let (code, reason): (u16, String) =
        serde_json::from_str(payload).map_err(|error| json_error('c', &error))?;

    Ok(ServerFrame::Close { code, reason })
}

fn json_error(kind: char, error: &serde_json::Error) -> Error {
    Error::InvalidJson {
        kind,
        line: error.line(),
        column: error.column(),
    }
}

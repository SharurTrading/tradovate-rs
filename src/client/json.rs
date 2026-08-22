// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Bounded JSON request encoding.

use std::io::{self, Write};

use serde::Serialize;

use crate::Error;

pub(crate) fn encode_bounded_json<T>(
    value: &T,
    endpoint: &'static str,
    limit: usize,
) -> Result<Vec<u8>, Error>
where
    T: Serialize + ?Sized,
{
    let mut writer = BoundedWriter::new(limit);
    match serde_json::to_writer(&mut writer, value) {
        Ok(()) => Ok(writer.into_inner()),
        Err(_) if writer.overflowed() => Err(Error::RequestTooLarge { endpoint, limit }),
        Err(source) => Err(Error::Encode { endpoint, source }),
    }
}

struct BoundedWriter {
    bytes: Vec<u8>,
    limit: usize,
    overflowed: bool,
}

impl BoundedWriter {
    fn new(limit: usize) -> Self {
        Self {
            bytes: Vec::with_capacity(limit.min(4_096)),
            limit,
            overflowed: false,
        }
    }

    const fn overflowed(&self) -> bool {
        self.overflowed
    }

    fn into_inner(self) -> Vec<u8> {
        self.bytes
    }
}

impl Write for BoundedWriter {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        let Some(next_len) = self.bytes.len().checked_add(bytes.len()) else {
            self.overflowed = true;
            return Err(io::Error::other("encoded request size overflowed"));
        };
        if next_len > self.limit {
            self.overflowed = true;
            return Err(io::Error::other(
                "encoded request exceeded its byte ceiling",
            ));
        }
        self.bytes.extend_from_slice(bytes);
        Ok(bytes.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
#[path = "json/tests.rs"]
mod tests;

// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Wire timestamp readers that preserve distinct provider event instants.

use std::fmt;

use jiff::Timestamp;
use serde::{Deserialize, Deserializer, de};

/// Decodes an absolute provider timestamp.
pub(crate) fn deserialize<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Timestamp, D::Error> {
    WireTimestamp::deserialize(deserializer).map(|WireTimestamp(timestamp)| timestamp)
}

/// Decodes a nullable absolute provider timestamp.
pub(crate) fn deserialize_optional<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<Timestamp>, D::Error> {
    Option::<WireTimestamp>::deserialize(deserializer)
        .map(|timestamp| timestamp.map(|WireTimestamp(instant)| instant))
}

struct WireTimestamp(Timestamp);

impl<'de> Deserialize<'de> for WireTimestamp {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_str(TimestampVisitor)
    }
}

struct TimestampVisitor;

impl de::Visitor<'_> for TimestampVisitor {
    type Value = WireTimestamp;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("an absolute timestamp string")
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
        self.visit_bytes(value.as_bytes())
    }

    fn visit_bytes<E: de::Error>(self, value: &[u8]) -> Result<Self::Value, E> {
        let time = value
            .splitn(2, |byte| matches!(byte, b'T' | b't' | b' '))
            .nth(1);
        // The raw seconds are checked before Jiff clamps a leap second to :59.
        if matches!(
            time,
            Some(
                [_, _, b':', _, _, b':', b'6', b'0', ..]
                    | [_, _, b'0'..=b'9', b'0'..=b'9', b'6', b'0', ..]
            )
        ) {
            return Err(E::custom("timestamp contains an unsupported leap second"));
        }
        jiff::fmt::temporal::DateTimeParser::new()
            .parse_timestamp(value)
            .map(WireTimestamp)
            .map_err(E::custom)
    }
}

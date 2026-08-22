// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Exact JSON-number conversion for financial values.

use std::str::FromStr;

use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de};
use serde_json::value::RawValue;

pub(crate) fn serialize<S>(value: &Decimal, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let raw = RawValue::from_string(value.to_string()).map_err(serde::ser::Error::custom)?;
    raw.serialize(serializer)
}

pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    let raw = Box::<RawValue>::deserialize(deserializer)?;
    parse(raw.get()).map_err(de::Error::custom)
}

fn parse(raw: &str) -> Result<Decimal, rust_decimal::Error> {
    Decimal::from_str(raw).or_else(|_| Decimal::from_scientific(raw))
}

pub(crate) mod option {
    use rust_decimal::Decimal;
    use serde::{Deserialize, Deserializer, Serializer, de};
    use serde_json::value::RawValue;

    #[expect(
        clippy::ref_option,
        reason = "Serde's `with` module contract passes a reference to the field type"
    )]
    pub(crate) fn serialize<S>(value: &Option<Decimal>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(value) => super::serialize(value, serializer),
            None => serializer.serialize_none(),
        }
    }

    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<Decimal>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = Option::<Box<RawValue>>::deserialize(deserializer)?;
        raw.map(|raw| super::parse(raw.get()).map_err(de::Error::custom))
            .transpose()
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use super::*;

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    struct Fixture {
        #[serde(with = "crate::decimal")]
        value: Decimal,
    }

    #[test]
    fn preserves_decimal_lexeme_without_float_round_trip() {
        let parsed: Result<Fixture, _> = serde_json::from_str("{\"value\":123456789.123456789}");
        assert!(parsed.is_ok());
        let fixture = match parsed {
            Ok(value) => value,
            Err(error) => panic!("fixture should decode: {error}"),
        };
        assert_eq!(fixture.value.to_string(), "123456789.123456789");
        let encoded = serde_json::to_string(&fixture);
        assert!(matches!(
            encoded.as_deref(),
            Ok("{\"value\":123456789.123456789}")
        ));
    }
}

// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Validated provider identity types.

use std::{fmt, str::FromStr};

use serde::{Deserialize, Deserializer, Serialize, Serializer, de};
use thiserror::Error;

/// Invalid provider identity input.
#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("invalid {kind}: {reason}")]
pub struct IdentifierError {
    kind: &'static str,
    reason: &'static str,
}

macro_rules! positive_id {
    ($name:ident, $doc:literal, $kind:literal) => {
        #[doc = $doc]
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(i64);

        impl $name {
            /// Creates an identifier from a positive provider integer.
            ///
            /// # Errors
            ///
            /// Returns [`IdentifierError`] when `value` is not positive.
            pub const fn new(value: i64) -> Result<Self, IdentifierError> {
                if value > 0 {
                    Ok(Self(value))
                } else {
                    Err(IdentifierError {
                        kind: $kind,
                        reason: "must be positive",
                    })
                }
            }

            /// Returns the provider integer.
            #[must_use]
            pub const fn get(self) -> i64 {
                self.0
            }
        }

        impl Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_i64(self.0)
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(formatter)
            }
        }

        impl TryFrom<i64> for $name {
            type Error = IdentifierError;

            fn try_from(value: i64) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let value = i64::deserialize(deserializer)?;
                Self::new(value).map_err(de::Error::custom)
            }
        }
    };
}

positive_id!(
    AccountId,
    "A Tradovate account entity identifier.",
    "account ID"
);
positive_id!(
    ContractId,
    "A Tradovate contract entity identifier.",
    "contract ID"
);
positive_id!(OrderId, "A Tradovate order entity identifier.", "order ID");
positive_id!(
    CommandId,
    "A Tradovate command entity identifier.",
    "command ID"
);
positive_id!(UserId, "A Tradovate user entity identifier.", "user ID");
positive_id!(
    PositionId,
    "A Tradovate position entity identifier.",
    "position ID"
);
positive_id!(
    ContractMaturityId,
    "A Tradovate contract-maturity entity identifier.",
    "contract maturity ID"
);
macro_rules! text_id {
    ($name:ident, $doc:literal, $kind:literal, $max:expr) => {
        #[doc = $doc]
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            /// Validates and owns a provider identifier.
            ///
            /// # Errors
            ///
            /// Returns [`IdentifierError`] for empty, padded, oversized, or
            /// control-character-containing input.
            pub fn new(raw: impl Into<String>) -> Result<Self, IdentifierError> {
                let raw = raw.into();
                validate_text($kind, &raw, $max)?;
                Ok(Self(raw))
            }

            /// Returns the provider representation.
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(&self.0)
            }
        }

        impl FromStr for $name {
            type Err = IdentifierError;

            fn from_str(raw: &str) -> Result<Self, Self::Err> {
                Self::new(raw)
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_str(&self.0)
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let value = String::deserialize(deserializer)?;
                Self::new(value).map_err(de::Error::custom)
            }
        }
    };
}

text_id!(
    Symbol,
    "A validated Tradovate contract symbol.",
    "symbol",
    64
);
text_id!(
    AccountSpec,
    "A validated Tradovate account specification.",
    "account specification",
    64
);
text_id!(
    ClientOrderId,
    "A caller-owned order correlation identifier.",
    "client order ID",
    64
);
text_id!(
    DeviceId,
    "A stable device identifier used by Tradovate authentication.",
    "device ID",
    64
);

fn validate_text(kind: &'static str, value: &str, max_len: usize) -> Result<(), IdentifierError> {
    let reason = if value.is_empty() {
        Some("must not be empty")
    } else if value.trim() != value {
        Some("must not contain surrounding whitespace")
    } else if value.len() > max_len {
        Some("is too long")
    } else if value.chars().any(char::is_control) {
        Some("must not contain control characters")
    } else {
        None
    };
    reason.map_or(Ok(()), |reason| Err(IdentifierError { kind, reason }))
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    #[test]
    fn numeric_ids_reject_non_positive_values() {
        assert!(AccountId::new(0).is_err());
        assert!(OrderId::new(-1).is_err());
    }

    #[test]
    fn text_ids_reject_padding_and_controls() {
        assert!(Symbol::new(" ES").is_err());
        assert!(ClientOrderId::new("a\nb").is_err());
    }

    proptest! {
        #[test]
        fn positive_account_ids_round_trip_through_json(value in 1_i64..=i64::MAX) {
            let id = AccountId::new(value);
            prop_assert!(id.is_ok());
            let encoded = id.as_ref().ok().and_then(|id| serde_json::to_string(id).ok());
            let decoded = encoded
                .as_deref()
                .and_then(|json| serde_json::from_str::<AccountId>(json).ok());
            prop_assert_eq!(decoded.map(AccountId::get), Some(value));
        }

        #[test]
        fn padded_symbols_are_always_rejected(core in "[A-Za-z0-9._-]{1,32}") {
            let leading_padding = format!(" {core}");
            let trailing_padding = format!("{core} ");
            prop_assert!(Symbol::new(leading_padding).is_err());
            prop_assert!(Symbol::new(trailing_padding).is_err());
        }
    }
}

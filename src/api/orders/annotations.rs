// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Bounded annotations accepted by current order commands.

use std::fmt;

use serde::Serialize;

use crate::Error;

macro_rules! bounded_annotation {
    ($name:ident, $doc:literal, $field:literal, $max:expr) => {
        #[doc = $doc]
        #[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            /// Validates and owns an annotation.
            ///
            /// # Errors
            ///
            /// Returns [`Error::InvalidRequest`] when the value is empty,
            /// padded, contains control characters, or exceeds its byte bound.
            pub fn new(value: impl Into<String>) -> Result<Self, Error> {
                let value = value.into();
                validate_annotation($field, &value, $max)?;
                Ok(Self(value))
            }

            /// Returns the validated provider representation.
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter
                    .debug_tuple(stringify!($name))
                    .field(&self.0)
                    .finish()
            }
        }
    };
}

bounded_annotation!(
    CustomTag50,
    "A non-empty order correlation tag bounded to Tradovate's 50-byte field.",
    "custom_tag_50",
    50
);
bounded_annotation!(
    OrderText,
    "A bounded non-empty operator annotation for an order command.",
    "order_text",
    256
);
bounded_annotation!(
    StrategyInstanceId,
    "A bounded caller-provided strategy instance identifier sent as `uuid`.",
    "strategy_instance_id",
    64
);

fn validate_annotation(field: &'static str, value: &str, max_bytes: usize) -> Result<(), Error> {
    let reason = if value.is_empty() {
        Some("must not be empty")
    } else if value.trim() != value {
        Some("must not contain surrounding whitespace")
    } else if value.len() > max_bytes {
        Some("exceeds the request-field safety bound")
    } else if value.chars().any(char::is_control) {
        Some("must not contain control characters")
    } else {
        None
    };
    reason.map_or(Ok(()), |reason| {
        Err(Error::InvalidRequest { field, reason })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn custom_tag_enforces_its_wire_bound() {
        assert!(CustomTag50::new("x".repeat(50)).is_ok());
        assert!(CustomTag50::new("x".repeat(51)).is_err());
    }

    #[test]
    fn annotations_reject_padding_and_controls() {
        assert!(OrderText::new(" padded").is_err());
        assert!(StrategyInstanceId::new("instance\n2").is_err());
    }
}

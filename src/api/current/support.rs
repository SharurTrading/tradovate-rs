// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Small handwritten invariants shared by generated current-API modules.

use std::fmt::{self, Write as _};

use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::IgnoredAny};
use thiserror::Error;

// Form encoding can expand each source byte to three wire bytes. Keeping the
// source below 20 KiB plus at most 1,024 separators bounds the final query to
// less than 64 KiB.
const MAX_QUERY_SOURCE_BYTES: usize = 20 * 1_024;
const MAX_QUERY_VALUES: usize = 1_024;

/// Internal repeated-key encoder for generated `OpenAPI` query structures.
pub(crate) trait CurrentQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error>;
}

/// Internal validation contract for generated request bodies.
pub(crate) trait CurrentRequest {
    fn validate_current(&self) -> Result<(), crate::Error>;
}

/// Appends one bounded form-style query value.
pub(crate) fn push_query_value(
    pairs: &mut Vec<(&'static str, String)>,
    key: &'static str,
    value: &impl fmt::Display,
) -> Result<(), crate::Error> {
    if pairs.len() >= MAX_QUERY_VALUES {
        return Err(crate::Error::InvalidRequest {
            field: "query",
            reason: "contains too many values",
        });
    }
    let used = pairs
        .iter()
        .try_fold(0_usize, |total, (key, value)| {
            total.checked_add(key.len())?.checked_add(value.len())
        })
        .and_then(|total| total.checked_add(key.len()))
        .ok_or(crate::Error::InvalidRequest {
            field: "query",
            reason: "encoded size overflowed",
        })?;
    let remaining =
        MAX_QUERY_SOURCE_BYTES
            .checked_sub(used)
            .ok_or(crate::Error::InvalidRequest {
                field: "query",
                reason: "encoded query is too large",
            })?;
    let mut encoded = BoundedQueryValue::new(remaining);
    write!(&mut encoded, "{value}").map_err(|_| crate::Error::InvalidRequest {
        field: "query",
        reason: if encoded.overflowed {
            "encoded query is too large"
        } else {
            "query value could not be formatted"
        },
    })?;
    pairs.push((key, encoded.value));
    Ok(())
}

struct BoundedQueryValue {
    value: String,
    limit: usize,
    overflowed: bool,
}

impl BoundedQueryValue {
    fn new(limit: usize) -> Self {
        Self {
            value: String::with_capacity(limit.min(256)),
            limit,
            overflowed: false,
        }
    }
}

impl fmt::Write for BoundedQueryValue {
    fn write_str(&mut self, value: &str) -> fmt::Result {
        let Some(next_len) = self.value.len().checked_add(value.len()) else {
            self.overflowed = true;
            return Err(fmt::Error);
        };
        if next_len > self.limit {
            self.overflowed = true;
            return Err(fmt::Error);
        }
        self.value.push_str(value);
        Ok(())
    }
}

/// A required generated wire-model field was not supplied.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
#[error("invalid current-API field `{field}`: {reason}")]
pub struct BuildError {
    field: &'static str,
    reason: &'static str,
}

impl BuildError {
    pub(crate) const fn missing(field: &'static str) -> Self {
        Self {
            field,
            reason: "is required",
        }
    }

    pub(crate) const fn invalid(field: &'static str, reason: &'static str) -> Self {
        Self { field, reason }
    }

    /// Returns the missing official wire-field name.
    #[must_use]
    pub const fn field(self) -> &'static str {
        self.field
    }

    /// Returns the stable validation reason.
    #[must_use]
    pub const fn reason(self) -> &'static str {
        self.reason
    }
}

/// Owned secret input for a current API request.
///
/// The value serializes for transport but has no public accessor, and its
/// debug representation is always redacted.
#[derive(Clone)]
pub struct SecretValue(SecretString);

impl SecretValue {
    /// Owns a secret for use in one of the typed request builders.
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::InvalidRequest`] when the secret is empty.
    pub fn new(value: impl Into<String>) -> Result<Self, crate::Error> {
        let value = value.into();
        if value.is_empty() {
            return Err(crate::Error::InvalidRequest {
                field: "secret",
                reason: "must not be empty",
            });
        }
        Ok(Self(SecretString::from(value)))
    }

    pub(crate) fn expose(&self) -> &str {
        self.0.expose_secret()
    }
}

impl fmt::Debug for SecretValue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("[REDACTED]")
    }
}

impl Serialize for SecretValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.0.expose_secret())
    }
}

impl<'de> Deserialize<'de> for SecretValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer)
            .and_then(|value| Self::new(value).map_err(serde::de::Error::custom))
    }
}

/// A successful response whose payload shape is not specified by the current
/// Partner `OpenAPI`.
///
/// This type proves only that a bounded, valid JSON value was received after
/// HTTP and provider-control validation. It intentionally exposes no raw data.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DocumentedAcknowledgement;

impl<'de> Deserialize<'de> for DocumentedAcknowledgement {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        IgnoredAny::deserialize(deserializer).map(|_| Self)
    }
}

impl crate::client::DocumentedMutationResponse for DocumentedAcknowledgement {
    fn mutation_outcome(&self) -> crate::client::MutationOutcome {
        crate::client::MutationOutcome::Ambiguous
    }

    fn has_success_evidence(&self) -> bool {
        false
    }
}

/// HTTP method declared by a current operation.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum HttpMethod {
    /// An idempotent retrieval operation.
    Get,
    /// A JSON-body or relaxed-method operation.
    Post,
}

/// Safety classification applied by the client execution layer.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum OperationClass {
    /// A read-only operation that may wait for rate admission.
    Query,
    /// A state-changing operation protected by ambiguity fencing.
    Mutation,
    /// Authentication, session rotation, or subscription establishment owned
    /// by a dedicated lifecycle state machine.
    Lifecycle,
}

/// How a current operation is exposed without bypassing its safety contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum OperationSurface {
    /// A schema-complete method is generated behind shared execution policy.
    Generated,
    /// A handwritten method owns additional authentication, realtime, or order invariants.
    Specialized,
    /// Typed wire models exist, but no public mutation method bypasses a pending
    /// request-aware invariant and completion-policy review.
    Modeled,
    /// The current provider contract omits grammar needed for a safe callable method.
    DocumentationBlocked,
}

/// Completeness of an operation's current official response schema.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ResponseContract {
    /// The response has a current typed schema or a defensible current entity contract.
    Typed,
    /// A named current response schema contains an untyped placeholder object.
    Incomplete,
    /// The current documentation promises JSON success but publishes no payload schema.
    Unspecified,
}

/// One named schema whose current component omits the value grammar.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SchemaGap {
    pub(crate) schema: &'static str,
    pub(crate) operations: &'static [&'static str],
    pub(crate) reason: &'static str,
}

impl SchemaGap {
    /// Returns the exact current component-schema name.
    #[must_use]
    pub const fn schema(self) -> &'static str {
        self.schema
    }

    /// Returns every affected current operation path.
    #[must_use]
    pub const fn operations(self) -> &'static [&'static str] {
        self.operations
    }

    /// Returns a stable explanation of the missing provider grammar.
    #[must_use]
    pub const fn reason(self) -> &'static str {
        self.reason
    }
}

/// One operation in the pinned current Partner `OpenAPI` contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Operation {
    pub(crate) method: HttpMethod,
    pub(crate) path: &'static str,
    pub(crate) id: &'static str,
    pub(crate) capability: &'static str,
    pub(crate) class: OperationClass,
    pub(crate) surface: OperationSurface,
    pub(crate) response_contract: ResponseContract,
}

#[cfg(test)]
#[path = "support/tests.rs"]
mod tests;

impl Operation {
    /// Returns the documented HTTP method.
    #[must_use]
    pub const fn method(self) -> HttpMethod {
        self.method
    }

    /// Returns the exact current endpoint path.
    #[must_use]
    pub const fn path(self) -> &'static str {
        self.path
    }

    /// Returns the official `OpenAPI` operation identifier.
    #[must_use]
    pub const fn operation_id(self) -> &'static str {
        self.id
    }

    /// Returns the official capability tag.
    #[must_use]
    pub const fn capability(self) -> &'static str {
        self.capability
    }

    /// Returns the client safety classification.
    #[must_use]
    pub const fn class(self) -> OperationClass {
        self.class
    }

    /// Returns how this operation is exposed by the crate.
    #[must_use]
    pub const fn surface(self) -> OperationSurface {
        self.surface
    }

    /// Returns whether the current provider contract specifies a typed payload.
    #[must_use]
    pub const fn response_contract(self) -> ResponseContract {
        self.response_contract
    }
}

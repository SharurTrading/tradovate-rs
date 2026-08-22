// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Secret API-key credential ownership.

use std::fmt;

use secrecy::{ExposeSecret, SecretString};

use crate::{ConfigError, DeviceId, Error};

// Current field limits (accessed 2026-08-22):
// https://partner.tradovate.com/api/rest-api-endpoints/authentication/access-token-request
// The pinned component identifies required fields but does not carry these maximums.
const SHORT_FIELD_LIMIT: usize = 64;
const PASSWORD_LIMIT: usize = 512;
const API_SECRET_LIMIT: usize = 8_192;

/// Explicit wire representation for Tradovate's inconsistently documented `cid`.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub(crate) enum ApiClientId {
    /// Numeric client identifier used by current official examples.
    Numeric(u64),
    /// Text client identifier accepted by the embedded `OpenAPI` schema.
    Text(String),
}

/// Credentials for Tradovate's direct API-key token flow.
///
/// The current Partner schema requires only the account name and password.
/// Application, client, secret, device, and HIBP metadata is transmitted only
/// when the corresponding builder method is called.
///
/// Secret values have no public getters and all debug output is redacted.
pub struct Credentials {
    name: SecretString,
    password: SecretString,
    app_id: Option<String>,
    app_version: Option<String>,
    client_id: Option<ApiClientId>,
    secret: Option<SecretString>,
    device_id: Option<DeviceId>,
    hibp_check: Option<bool>,
}

impl Credentials {
    /// Starts a credential builder with the account name and API password.
    pub fn builder(name: impl Into<String>, password: impl Into<String>) -> CredentialsBuilder {
        CredentialsBuilder {
            name: SecretString::from(name.into()),
            password: SecretString::from(password.into()),
            app_id: None,
            app_version: None,
            client_id: None,
            secret: None,
            device_id: None,
            hibp_check: None,
        }
    }

    pub(crate) fn name(&self) -> &str {
        self.name.expose_secret()
    }

    pub(crate) fn password(&self) -> &str {
        self.password.expose_secret()
    }

    pub(crate) fn app_id(&self) -> Option<&str> {
        self.app_id.as_deref()
    }

    pub(crate) fn app_version(&self) -> Option<&str> {
        self.app_version.as_deref()
    }

    pub(crate) const fn client_id(&self) -> Option<&ApiClientId> {
        self.client_id.as_ref()
    }

    pub(crate) fn secret(&self) -> Option<&str> {
        self.secret.as_ref().map(ExposeSecret::expose_secret)
    }

    pub(crate) const fn device_id(&self) -> Option<&DeviceId> {
        self.device_id.as_ref()
    }

    pub(crate) const fn hibp_check(&self) -> Option<bool> {
        self.hibp_check
    }
}

impl fmt::Debug for Credentials {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Credentials")
            .field("name", &"[REDACTED]")
            .field("password", &"[REDACTED]")
            .field("app_id", &self.app_id.as_ref().map(|_| "[REDACTED]"))
            .field(
                "app_version",
                &self.app_version.as_ref().map(|_| "[REDACTED]"),
            )
            .field("client_id", &self.client_id.as_ref().map(|_| "[REDACTED]"))
            .field("secret", &self.secret.as_ref().map(|_| "[REDACTED]"))
            .field("device_id", &self.device_id.as_ref().map(|_| "[REDACTED]"))
            .field(
                "hibp_check",
                &self.hibp_check.as_ref().map(|_| "[REDACTED]"),
            )
            .finish()
    }
}

/// Builder for validated [`Credentials`].
#[must_use = "a credential builder does nothing until build is called"]
pub struct CredentialsBuilder {
    name: SecretString,
    password: SecretString,
    app_id: Option<String>,
    app_version: Option<String>,
    client_id: Option<ApiClientId>,
    secret: Option<SecretString>,
    device_id: Option<DeviceId>,
    hibp_check: Option<bool>,
}

impl CredentialsBuilder {
    /// Sets the optional application identifier registered with Tradovate.
    pub fn app_id(mut self, value: impl Into<String>) -> Self {
        self.app_id = Some(value.into());
        self
    }

    /// Sets the optional application version sent to Tradovate.
    pub fn app_version(mut self, value: impl Into<String>) -> Self {
        self.app_version = Some(value.into());
        self
    }

    /// Sets an optional numeric Tradovate client identifier.
    pub fn numeric_client_id(mut self, value: u64) -> Self {
        self.client_id = Some(ApiClientId::Numeric(value));
        self
    }

    /// Sets an optional text Tradovate client identifier for compatible deployments.
    pub fn text_client_id(mut self, value: impl Into<String>) -> Self {
        self.client_id = Some(ApiClientId::Text(value.into()));
        self
    }

    /// Sets the optional API-key secret.
    pub fn secret(mut self, value: impl Into<String>) -> Self {
        self.secret = Some(SecretString::from(value.into()));
        self
    }

    /// Sets the optional stable device identifier.
    pub fn device_id(mut self, value: DeviceId) -> Self {
        self.device_id = Some(value);
        self
    }

    /// Sets the optional provider Have-I-Been-Pwned password-check flag.
    pub const fn hibp_check(mut self, enabled: bool) -> Self {
        self.hibp_check = Some(enabled);
        self
    }

    /// Validates and constructs the credentials.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Configuration`] when a required value or a supplied
    /// optional value is empty, padded with whitespace, too long, or contains
    /// control characters.
    pub fn build(self) -> Result<Credentials, Error> {
        validate_secret("name", self.name.expose_secret(), SHORT_FIELD_LIMIT)?;
        validate_secret("password", self.password.expose_secret(), PASSWORD_LIMIT)?;
        let app_id = validate_optional_text("app_id", self.app_id, SHORT_FIELD_LIMIT)?;
        let app_version =
            validate_optional_text("app_version", self.app_version, SHORT_FIELD_LIMIT)?;
        let client_id = self.client_id;
        if let Some(ApiClientId::Text(value)) = &client_id {
            validate_text("cid", value, SHORT_FIELD_LIMIT)?;
        }
        if let Some(secret) = self.secret.as_ref() {
            validate_secret("sec", secret.expose_secret(), API_SECRET_LIMIT)?;
        }
        Ok(Credentials {
            name: self.name,
            password: self.password,
            app_id,
            app_version,
            client_id,
            secret: self.secret,
            device_id: self.device_id,
            hibp_check: self.hibp_check,
        })
    }
}

impl fmt::Debug for CredentialsBuilder {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CredentialsBuilder")
            .field("name", &"[REDACTED]")
            .field("password", &"[REDACTED]")
            .field("app_id", &self.app_id.as_ref().map(|_| "[REDACTED]"))
            .field(
                "app_version",
                &self.app_version.as_ref().map(|_| "[REDACTED]"),
            )
            .field("client_id", &self.client_id.as_ref().map(|_| "[REDACTED]"))
            .field("secret", &self.secret.as_ref().map(|_| "[REDACTED]"))
            .field("device_id", &self.device_id.as_ref().map(|_| "[REDACTED]"))
            .field(
                "hibp_check",
                &self.hibp_check.as_ref().map(|_| "[REDACTED]"),
            )
            .finish()
    }
}

fn validate_optional_text(
    field: &'static str,
    value: Option<String>,
    max_len: usize,
) -> Result<Option<String>, ConfigError> {
    if let Some(value) = value.as_deref() {
        validate_text(field, value, max_len)?;
    }
    Ok(value)
}

fn validate_secret(field: &'static str, value: &str, max_len: usize) -> Result<(), ConfigError> {
    validate_text(field, value, max_len)
}

fn validate_text(field: &'static str, value: &str, max_len: usize) -> Result<(), ConfigError> {
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
    reason.map_or(Ok(()), |reason| {
        Err(ConfigError::InvalidSetting { field, reason })
    })
}

#[cfg(test)]
#[path = "credentials/tests.rs"]
mod tests;

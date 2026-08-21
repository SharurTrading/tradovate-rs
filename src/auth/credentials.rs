// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Secret API-key credential ownership.

use std::fmt;

use secrecy::{ExposeSecret, SecretString};

use crate::{ConfigError, DeviceId, Error};

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
/// Secret values have no public getters and all debug output is redacted.
pub struct Credentials {
    name: SecretString,
    password: SecretString,
    app_id: String,
    app_version: String,
    client_id: ApiClientId,
    secret: SecretString,
    device_id: DeviceId,
    hibp_check: bool,
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
            hibp_check: true,
        }
    }

    pub(crate) fn name(&self) -> &str {
        self.name.expose_secret()
    }

    pub(crate) fn password(&self) -> &str {
        self.password.expose_secret()
    }

    pub(crate) fn app_id(&self) -> &str {
        &self.app_id
    }

    pub(crate) fn app_version(&self) -> &str {
        &self.app_version
    }

    pub(crate) const fn client_id(&self) -> &ApiClientId {
        &self.client_id
    }

    pub(crate) fn secret(&self) -> &str {
        self.secret.expose_secret()
    }

    pub(crate) const fn device_id(&self) -> &DeviceId {
        &self.device_id
    }

    pub(crate) const fn hibp_check(&self) -> bool {
        self.hibp_check
    }
}

impl fmt::Debug for Credentials {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Credentials")
            .field("name", &"[REDACTED]")
            .field("password", &"[REDACTED]")
            .field("app_id", &self.app_id)
            .field("app_version", &self.app_version)
            .field("client_id", &"[REDACTED]")
            .field("secret", &"[REDACTED]")
            .field("device_id", &"[REDACTED]")
            .field("hibp_check", &self.hibp_check)
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
    hibp_check: bool,
}

impl CredentialsBuilder {
    /// Sets the application identifier registered with Tradovate.
    pub fn app_id(mut self, value: impl Into<String>) -> Self {
        self.app_id = Some(value.into());
        self
    }

    /// Sets the application version sent to Tradovate.
    pub fn app_version(mut self, value: impl Into<String>) -> Self {
        self.app_version = Some(value.into());
        self
    }

    /// Sets a numeric Tradovate client identifier.
    pub fn numeric_client_id(mut self, value: u64) -> Self {
        self.client_id = Some(ApiClientId::Numeric(value));
        self
    }

    /// Sets a text Tradovate client identifier for compatible deployments.
    pub fn text_client_id(mut self, value: impl Into<String>) -> Self {
        self.client_id = Some(ApiClientId::Text(value.into()));
        self
    }

    /// Sets the API-key secret.
    pub fn secret(mut self, value: impl Into<String>) -> Self {
        self.secret = Some(SecretString::from(value.into()));
        self
    }

    /// Sets the stable device identifier.
    pub fn device_id(mut self, value: DeviceId) -> Self {
        self.device_id = Some(value);
        self
    }

    /// Controls the provider's Have-I-Been-Pwned password check.
    pub const fn hibp_check(mut self, enabled: bool) -> Self {
        self.hibp_check = enabled;
        self
    }

    /// Validates and constructs the credentials.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Configuration`] when a required value is missing,
    /// empty, padded with whitespace, or contains control characters.
    pub fn build(self) -> Result<Credentials, Error> {
        validate_secret("name", self.name.expose_secret(), 64)?;
        validate_secret("password", self.password.expose_secret(), 64)?;
        let app_id = require_text("app_id", self.app_id, 64)?;
        let app_version = require_text("app_version", self.app_version, 64)?;
        let client_id = self.client_id.ok_or(ConfigError::InvalidSetting {
            field: "cid",
            reason: "is required",
        })?;
        if let ApiClientId::Text(value) = &client_id {
            validate_text("cid", value, 64)?;
        }
        let secret = self.secret.ok_or(ConfigError::InvalidSetting {
            field: "sec",
            reason: "is required",
        })?;
        validate_secret("sec", secret.expose_secret(), 8_192)?;
        let device_id = self.device_id.ok_or(ConfigError::InvalidSetting {
            field: "device_id",
            reason: "is required",
        })?;
        Ok(Credentials {
            name: self.name,
            password: self.password,
            app_id,
            app_version,
            client_id,
            secret,
            device_id,
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
            .field("app_id", &self.app_id)
            .field("app_version", &self.app_version)
            .field("client_id", &self.client_id.as_ref().map(|_| "[REDACTED]"))
            .field("secret", &self.secret.as_ref().map(|_| "[REDACTED]"))
            .field("device_id", &self.device_id.as_ref().map(|_| "[REDACTED]"))
            .field("hibp_check", &self.hibp_check)
            .finish()
    }
}

fn require_text(
    field: &'static str,
    value: Option<String>,
    max_len: usize,
) -> Result<String, ConfigError> {
    let value = value.ok_or(ConfigError::InvalidSetting {
        field,
        reason: "is required",
    })?;
    validate_text(field, &value, max_len)?;
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
mod tests {
    use super::*;

    #[test]
    fn debug_redacts_every_sensitive_value() {
        let builder = Credentials::builder("secret-user", "secret-password")
            .app_id("sample")
            .app_version("1.0")
            .numeric_client_id(123)
            .secret("secret-key")
            .device_id(DeviceId::new("secret-device").unwrap_or_else(|error| panic!("{error}")));
        let debug = format!("{builder:?}");
        assert!(!debug.contains("secret-user"));
        assert!(!debug.contains("secret-password"));
        assert!(!debug.contains("secret-key"));
        assert!(!debug.contains("secret-device"));
    }

    #[test]
    fn official_wire_length_limits_are_enforced() {
        let oversized = "x".repeat(65);
        let result = Credentials::builder(oversized, "password")
            .app_id("sample")
            .app_version("1.0")
            .numeric_client_id(123)
            .secret("secret-key")
            .device_id(
                DeviceId::new("synthetic-device")
                    .unwrap_or_else(|error| panic!("fixture device ID: {error}")),
            )
            .build();
        assert!(matches!(result, Err(Error::Configuration(_))));
    }
}

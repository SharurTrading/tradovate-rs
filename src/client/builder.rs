// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! REST client builder.

use std::{sync::Arc, time::Duration};

use reqwest::redirect::Policy;

use super::{Client, allocate_client_instance_id, mutation::MutationGate};
use crate::{ConfigError, EndpointSet, Error, auth::TokenStore, rate_limit::RateGovernor};

const DEFAULT_MAX_RESPONSE_BYTES: usize = 8 * 1024 * 1024;
// The current customer-document contract permits an 8 MiB data field; the
// encoded JSON envelope needs bounded headroom beyond that field itself.
const DEFAULT_MAX_REQUEST_BYTES: usize = 12 * 1024 * 1024;
const DEFAULT_REQUEST_TIMEOUT: Duration = Duration::from_secs(30);
const DEFAULT_CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
const HARD_MAX_RESPONSE_BYTES: usize = 64 * 1024 * 1024;
const HARD_MAX_REQUEST_BYTES: usize = 64 * 1024 * 1024;
const HARD_MAX_REQUEST_TIMEOUT: Duration = Duration::from_mins(5);
const HARD_MAX_CONNECT_TIMEOUT: Duration = Duration::from_mins(2);

/// Builder for a [`Client`].
#[must_use = "a client builder does nothing until build is called"]
#[derive(Clone, Debug)]
pub struct ClientBuilder {
    endpoints: EndpointSet,
    max_request_bytes: usize,
    max_response_bytes: usize,
    request_timeout: Duration,
    connect_timeout: Duration,
}

impl ClientBuilder {
    pub(crate) const fn new(endpoints: EndpointSet) -> Self {
        Self {
            endpoints,
            max_request_bytes: DEFAULT_MAX_REQUEST_BYTES,
            max_response_bytes: DEFAULT_MAX_RESPONSE_BYTES,
            request_timeout: DEFAULT_REQUEST_TIMEOUT,
            connect_timeout: DEFAULT_CONNECT_TIMEOUT,
        }
    }

    /// Sets the absolute decoded REST body ceiling.
    pub const fn max_response_bytes(mut self, bytes: usize) -> Self {
        self.max_response_bytes = bytes;
        self
    }

    /// Sets the absolute encoded REST request-body ceiling.
    pub const fn max_request_bytes(mut self, bytes: usize) -> Self {
        self.max_request_bytes = bytes;
        self
    }

    /// Sets the total timeout for one REST request.
    pub const fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = timeout;
        self
    }

    /// Sets the TCP/TLS connection timeout.
    pub const fn connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = timeout;
        self
    }

    /// Builds the client without performing network I/O.
    ///
    /// Redirects and ambient proxy discovery are disabled so credentials
    /// cannot silently cross an origin boundary.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Configuration`] for zero limits/timeouts, or a
    /// transport builder error when TLS initialization fails.
    pub fn build(self) -> Result<Client, Error> {
        if self.max_request_bytes == 0 {
            return Err(ConfigError::InvalidSetting {
                field: "max_request_bytes",
                reason: "must be positive",
            }
            .into());
        }
        if self.max_request_bytes > HARD_MAX_REQUEST_BYTES {
            return Err(ConfigError::InvalidSetting {
                field: "max_request_bytes",
                reason: "must not exceed 64 MiB",
            }
            .into());
        }
        if self.max_response_bytes == 0 {
            return Err(ConfigError::InvalidSetting {
                field: "max_response_bytes",
                reason: "must be positive",
            }
            .into());
        }
        if self.max_response_bytes > HARD_MAX_RESPONSE_BYTES {
            return Err(ConfigError::InvalidSetting {
                field: "max_response_bytes",
                reason: "must not exceed 64 MiB",
            }
            .into());
        }
        if self.request_timeout.is_zero() {
            return Err(ConfigError::InvalidSetting {
                field: "request_timeout",
                reason: "must be positive",
            }
            .into());
        }
        if self.request_timeout > HARD_MAX_REQUEST_TIMEOUT {
            return Err(ConfigError::InvalidSetting {
                field: "request_timeout",
                reason: "must not exceed five minutes",
            }
            .into());
        }
        if self.connect_timeout.is_zero() {
            return Err(ConfigError::InvalidSetting {
                field: "connect_timeout",
                reason: "must be positive",
            }
            .into());
        }
        if self.connect_timeout > HARD_MAX_CONNECT_TIMEOUT {
            return Err(ConfigError::InvalidSetting {
                field: "connect_timeout",
                reason: "must not exceed two minutes",
            }
            .into());
        }
        let http = reqwest::Client::builder()
            .no_proxy()
            .redirect(Policy::none())
            // Provider attempts are admitted and fenced by this crate. Hidden
            // protocol-NACK retries could duplicate money-moving requests.
            .retry(reqwest::retry::never())
            .timeout(self.request_timeout)
            .connect_timeout(self.connect_timeout)
            .build()
            .map_err(|source| Error::Transport { source })?;
        Ok(Client {
            http,
            endpoints: self.endpoints,
            instance_id: allocate_client_instance_id()?,
            tokens: Arc::new(TokenStore::default()),
            rate_limits: Arc::new(RateGovernor::tradovate_defaults()),
            mutation_gate: Arc::new(MutationGate::default()),
            max_request_bytes: self.max_request_bytes,
            max_response_bytes: self.max_response_bytes,
        })
    }
}

#[cfg(test)]
#[path = "builder/tests.rs"]
mod tests;

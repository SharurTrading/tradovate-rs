// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Typed, secret-safe errors.

use std::{
    fmt,
    sync::atomic::{AtomicBool, Ordering},
    time::{Duration, Instant},
};

use secrecy::{ExposeSecret, SecretSlice, SecretString};
use thiserror::Error;

/// Configuration failures detected before network activity begins.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ConfigError {
    /// A configured URL could not be parsed.
    #[error("invalid {label} URL")]
    InvalidUrl {
        /// Human-readable endpoint category.
        label: &'static str,
        /// URL parser failure.
        #[source]
        source: url::ParseError,
    },
    /// A URL contained embedded credentials.
    #[error("{label} URL must not contain credentials")]
    UrlCredentials {
        /// Human-readable endpoint category.
        label: &'static str,
    },
    /// A URL contained a query or fragment.
    #[error("{label} URL must not contain a query or fragment")]
    UrlSuffix {
        /// Human-readable endpoint category.
        label: &'static str,
    },
    /// A non-loopback URL used plaintext transport.
    #[error("{label} URL must use authenticated encryption")]
    InsecureUrl {
        /// Human-readable endpoint category.
        label: &'static str,
    },
    /// A required setting was absent or invalid.
    #[error("invalid {field}: {reason}")]
    InvalidSetting {
        /// Setting name.
        field: &'static str,
        /// Safe validation detail.
        reason: &'static str,
    },
}

/// An authentication penalty response that may permit an exact delayed retry.
///
/// The ticket is retained as a secret and is intentionally not exposed through
/// `Debug`, `Display`, or a public getter.
pub struct PenaltyTicket {
    ticket: SecretString,
    wait: Duration,
    captcha_required: bool,
    client_instance_id: u64,
    endpoint: &'static str,
    not_before: Option<Instant>,
    bound_request: Option<SecretSlice<u8>>,
    claimed: AtomicBool,
}

impl PenaltyTicket {
    pub(crate) fn new(
        ticket: String,
        wait: Duration,
        captcha_required: bool,
        client_instance_id: u64,
        endpoint: &'static str,
        not_before: Option<Instant>,
    ) -> Self {
        Self {
            ticket: SecretString::from(ticket),
            wait,
            captcha_required,
            client_instance_id,
            endpoint,
            not_before,
            bound_request: None,
            claimed: AtomicBool::new(false),
        }
    }

    /// Returns the provider-mandated delay before an exact retry.
    #[must_use]
    pub const fn wait(&self) -> Duration {
        self.wait
    }

    /// Reports whether operator captcha completion is required.
    #[must_use]
    pub const fn captcha_required(&self) -> bool {
        self.captcha_required
    }

    pub(crate) fn bind_request(&mut self, request: Vec<u8>) {
        self.bound_request = Some(SecretSlice::from(request.into_boxed_slice()));
    }

    pub(crate) fn begin_claim_for_request(
        &self,
        client_instance_id: u64,
        endpoint: &'static str,
        request: &[u8],
    ) -> Option<PenaltyClaim<'_>> {
        let request_matches = self
            .bound_request
            .as_ref()
            .is_some_and(|bound| bound.expose_secret() == request);
        let delay_elapsed = self
            .not_before
            .is_some_and(|not_before| Instant::now() >= not_before);
        let valid = self.client_instance_id == client_instance_id
            && self.endpoint == endpoint
            && request_matches
            && delay_elapsed;
        if valid
            && self
                .claimed
                .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
        {
            Some(PenaltyClaim {
                ticket: self,
                armed: false,
            })
        } else {
            None
        }
    }
}

/// Cancellation-safe ownership of one exact penalty-ticket retry.
pub(crate) struct PenaltyClaim<'a> {
    ticket: &'a PenaltyTicket,
    armed: bool,
}

impl PenaltyClaim<'_> {
    pub(crate) fn expose_ticket(&self) -> &str {
        self.ticket.ticket.expose_secret()
    }

    pub(crate) fn arm(&mut self) {
        self.armed = true;
    }

    pub(crate) fn release_unsent(mut self) {
        self.ticket.claimed.store(false, Ordering::Release);
        self.armed = true;
    }
}

impl Drop for PenaltyClaim<'_> {
    fn drop(&mut self) {
        if !self.armed {
            self.ticket.claimed.store(false, Ordering::Release);
        }
    }
}

impl fmt::Debug for PenaltyTicket {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PenaltyTicket")
            .field("ticket", &"[REDACTED]")
            .field("wait", &self.wait)
            .field("captcha_required", &self.captcha_required)
            .field("client_bound", &true)
            .field("endpoint", &self.endpoint)
            .field("retry_deadline_valid", &self.not_before.is_some())
            .field("bound_request", &self.bound_request.as_ref().map(|_| true))
            .field("claimed", &self.claimed.load(Ordering::Acquire))
            .finish_non_exhaustive()
    }
}

/// Failures returned by REST, authentication, and request validation.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    /// Client configuration is invalid.
    #[error(transparent)]
    Configuration(#[from] ConfigError),
    /// Credentials or a request value failed local validation.
    #[error("invalid {field}: {reason}")]
    InvalidRequest {
        /// Invalid field name.
        field: &'static str,
        /// Safe validation detail.
        reason: &'static str,
    },
    /// The client has no authenticated session.
    #[error("the client is not authenticated")]
    Unauthenticated,
    /// A provider authentication response contained an unusable session.
    #[error("the provider returned an invalid authentication session: {reason}")]
    InvalidAuthenticationResponse {
        /// Stable, secret-free validation reason.
        reason: &'static str,
    },
    /// Another access-token renewal already owns the single renewal slot.
    #[error("an access-token renewal is already in progress")]
    RenewalInProgress,
    /// The HTTP transport failed.
    #[error("HTTP transport failed")]
    Transport {
        /// Underlying transport error.
        #[source]
        source: reqwest::Error,
    },
    /// A request could not be encoded before transmission.
    #[error("failed to encode {endpoint} request")]
    Encode {
        /// Provider endpoint.
        endpoint: &'static str,
        /// JSON encoder failure.
        #[source]
        source: serde_json::Error,
    },
    /// A serialized request exceeded the configured byte ceiling.
    #[error("{endpoint} request exceeded {limit} bytes")]
    RequestTooLarge {
        /// Provider endpoint.
        endpoint: &'static str,
        /// Configured byte ceiling.
        limit: usize,
    },
    /// The provider returned a non-success HTTP status.
    #[error("{endpoint} returned HTTP {status}")]
    HttpStatus {
        /// Provider endpoint.
        endpoint: &'static str,
        /// HTTP status code.
        status: u16,
    },
    /// The provider rejected a syntactically successful request.
    #[error("{endpoint} rejected the request")]
    Business {
        /// Provider endpoint.
        endpoint: &'static str,
    },
    /// The provider rejected an order command with a documented reason.
    #[error("{endpoint} rejected the order command")]
    OrderRejected {
        /// Provider endpoint.
        endpoint: &'static str,
        /// Current Partner rejection category; free-form failure text is not retained.
        reason: crate::api::OrderFailureReason,
    },
    /// A reserved provider control field had an invalid or contradictory shape.
    #[error("{endpoint} returned a malformed provider control envelope")]
    InvalidProviderControl {
        /// Provider endpoint.
        endpoint: &'static str,
    },
    /// The provider reported structured validation failures.
    #[error("{endpoint} reported {count} request validation failure(s)")]
    Violations {
        /// Provider endpoint.
        endpoint: &'static str,
        /// Number of provider-reported failures; diagnostic text is omitted.
        count: usize,
    },
    /// Authentication received a delayed retry ticket with an exact retry API.
    #[error("the provider imposed a penalty ticket")]
    Penalty(PenaltyTicket),
    /// The provider imposed a cooldown on an endpoint without a safe retry API.
    #[error("{endpoint} is provider-penalized for {retry_after:?}")]
    ProviderPenalty {
        /// Provider endpoint.
        endpoint: &'static str,
        /// Full provider-declared delay.
        retry_after: Duration,
        /// Whether third-party retry is locked pending operator recovery.
        captcha_required: bool,
    },
    /// Local admission refused an immediate request.
    #[error("{endpoint} is locally rate limited for {retry_after:?}")]
    LocalRateLimit {
        /// Provider endpoint.
        endpoint: &'static str,
        /// Minimum delay before capacity may be available.
        retry_after: Duration,
    },
    /// The provider reported user-level rate exhaustion.
    #[error("{endpoint} is provider-rate-limited for at least {retry_after:?}")]
    ProviderRateLimit {
        /// Provider endpoint.
        endpoint: &'static str,
        /// Provider-declared or conservative fallback delay.
        retry_after: Duration,
    },
    /// A response exceeded the configured byte ceiling.
    #[error("{endpoint} response exceeded {limit} bytes")]
    ResponseTooLarge {
        /// Provider endpoint.
        endpoint: &'static str,
        /// Configured byte ceiling.
        limit: usize,
    },
    /// A provider response did not match its documented shape.
    #[error("failed to decode {endpoint} response")]
    Decode {
        /// Provider endpoint.
        endpoint: &'static str,
        /// JSON decoder failure.
        #[source]
        source: serde_json::Error,
    },
    /// A money-moving request may have reached the provider.
    #[error("{endpoint} outcome is uncertain; reconcile before retrying")]
    AmbiguousMutation {
        /// Provider endpoint.
        endpoint: &'static str,
    },
    /// Mutations are fenced until the caller reconciles an uncertain outcome.
    #[error("{endpoint} is blocked until provider state is reconciled")]
    MutationReconciliationRequired {
        /// Mutation endpoint refused before transmission.
        endpoint: &'static str,
    },
    /// Another mutation already owns this client's single in-flight slot.
    #[error("{endpoint} is blocked while another mutation is in flight")]
    MutationInProgress {
        /// Mutation endpoint refused before transmission.
        endpoint: &'static str,
    },
    /// An older authentication result lost a revision race.
    #[error("authentication result was superseded by a newer session attempt")]
    SupersededAuthentication,
}

#[cfg(test)]
#[path = "error/tests.rs"]
mod tests;

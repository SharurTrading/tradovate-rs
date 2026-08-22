// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Secret-safe failures for one real-time socket generation.

use std::time::Duration;

use thiserror::Error;

use super::{CodecError, ConnectionId, DisconnectReason, RequestId, ResyncReason, SocketKind};

/// The typed event family whose payload was invalid.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum RealtimeEventKind {
    /// Initial user synchronization data.
    Bootstrap,
    /// A user entity property delta.
    Properties,
    /// A graceful provider shutdown event.
    Shutdown,
    /// Quote, depth, or histogram data.
    MarketData,
    /// Regular-bar or tick-chart data.
    Chart,
    /// A future or otherwise unsupported event family.
    Unsupported,
}

/// Stable, secret-safe reasons for rejecting a realtime payload.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum RealtimePayloadError {
    /// A required `d` payload or required field was absent.
    MissingData,
    /// The payload did not match its documented JSON shape.
    Malformed,
    /// A named collection or text ceiling was exceeded.
    LimitExceeded,
    /// A provider identity, date, timestamp, or exact number was invalid.
    InvalidValue,
    /// A full depth packet was not ordered as documented.
    InvalidOrder,
}

/// Failures returned by real-time connection and request operations.
///
/// Variants retain only structural metadata. Bearer tokens, request bodies,
/// raw server frames, and server-provided text are excluded from `Debug` and
/// `Display` output.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Error)]
#[non_exhaustive]
pub enum RealtimeError {
    /// A real-time configuration value is invalid.
    #[error("invalid {field}: {reason}")]
    InvalidConfiguration {
        /// Invalid configuration field.
        field: &'static str,
        /// Stable validation reason.
        reason: &'static str,
    },
    /// No authenticated REST session is installed.
    #[error("the client is not authenticated")]
    Unauthenticated,
    /// No caller-owned Tokio runtime is active.
    #[error("real-time operations require an active Tokio runtime")]
    RuntimeUnavailable,
    /// Socket establishment did not complete before its deadline.
    #[error("real-time socket establishment timed out")]
    ConnectTimeout,
    /// The transport failed without exposing raw wire content.
    #[error("real-time socket transport failed")]
    Transport,
    /// The server did not send its open frame before the setup deadline.
    #[error("real-time server open frame timed out")]
    OpenTimeout,
    /// Authorization did not complete before the setup deadline.
    #[error("real-time authorization timed out")]
    AuthorizationTimeout,
    /// The server rejected authorization.
    #[error("real-time authorization returned status {status}")]
    AuthorizationRejected {
        /// HTTP-style provider status.
        status: u16,
    },
    /// Authorization returned a provider penalty control response.
    #[error("real-time authorization is provider-penalized for {retry_after:?}")]
    AuthorizationPenalty {
        /// Provider-declared delay before another authorization attempt.
        retry_after: Duration,
        /// Whether only operator-driven captcha recovery is permitted.
        captcha_required: bool,
    },
    /// The mandatory user synchronization did not complete before readiness.
    #[error("user synchronization timed out")]
    UserSyncTimeout,
    /// The server rejected the mandatory user synchronization.
    #[error("user synchronization returned status {status}")]
    UserSyncRejected {
        /// HTTP-style provider status.
        status: u16,
    },
    /// The user-sync response did not contain a complete snapshot object.
    #[error("user synchronization returned an invalid bootstrap payload")]
    UserSyncInvalidBootstrap,
    /// A server event failed its typed bounded wire contract.
    #[error("invalid {kind:?} realtime payload: {reason:?}")]
    InvalidEvent {
        /// Event family that failed validation.
        kind: RealtimeEventKind,
        /// Stable validation category.
        reason: RealtimePayloadError,
    },
    /// A typed realtime request is invalid.
    #[error("invalid realtime request field {field}: {reason}")]
    InvalidRequest {
        /// Invalid request field.
        field: &'static str,
        /// Stable validation reason.
        reason: &'static str,
    },
    /// A successful correlated response did not match its typed operation schema.
    #[error("{operation} returned an invalid typed response")]
    InvalidTypedResponse {
        /// Public-safe provider operation name.
        operation: &'static str,
    },
    /// The provider declined user sync with a penalty control response.
    #[error("user synchronization is rate-limited for {retry_after:?}")]
    UserSyncPenalty {
        /// Provider-declared delay before a new connection attempt.
        retry_after: Duration,
        /// Whether only operator-driven captcha recovery is permitted.
        captcha_required: bool,
    },
    /// A realtime request was refused locally before transmission.
    #[error("{endpoint} is locally rate-limited for {retry_after:?}")]
    LocalRateLimit {
        /// Provider endpoint refused at the sole-writer boundary.
        endpoint: &'static str,
        /// Minimum delay before capacity may be available.
        retry_after: Duration,
    },
    /// The server sent a message invalid for the current protocol state.
    #[error("real-time protocol state was violated")]
    Protocol,
    /// SockJS-derived framing failed.
    #[error(transparent)]
    Codec(#[from] CodecError),
    /// The bounded actor command channel is no longer available.
    #[error("real-time actor is not accepting commands")]
    ActorStopped,
    /// The spawned actor panicked or was externally aborted.
    #[error("real-time actor task failed")]
    ActorTaskFailed,
    /// The configured pending-request ceiling was reached.
    #[error("pending real-time request limit {limit} reached")]
    PendingLimitReached {
        /// Configured hard pending ceiling.
        limit: usize,
    },
    /// A request could not enter the actor queue before its pre-send deadline.
    #[error("real-time request queue admission timed out before transmission")]
    RequestQueueTimeout,
    /// An operation was attempted on the wrong real-time service.
    #[error("operation requires {expected:?} socket, not {actual:?}")]
    WrongSocketKind {
        /// Required service.
        expected: SocketKind,
        /// Connected service.
        actual: SocketKind,
    },
    /// A request expired and was removed from correlation state.
    #[error("real-time request {request_id} timed out")]
    RequestTimeout {
        /// Expired request identifier.
        request_id: RequestId,
    },
    /// A correlated provider response rejected the request.
    #[error("real-time request {request_id} returned status {status}")]
    ProviderRejected {
        /// Rejected request identifier.
        request_id: RequestId,
        /// HTTP-style provider status.
        status: u16,
    },
    /// A correlated response carried a business-level failure control.
    #[error("real-time request {request_id} was rejected by a provider control")]
    ProviderBusinessFailure {
        /// Rejected request identifier.
        request_id: RequestId,
        /// Number of structured violations when supplied by the provider.
        violation_count: Option<usize>,
    },
    /// A correlated response carried a validated penalty control.
    #[error("real-time request {request_id} is provider-penalized for {retry_after:?}")]
    ProviderPenalty {
        /// Penalized request identifier.
        request_id: RequestId,
        /// Provider-declared delay before a new safe request may be attempted.
        retry_after: Duration,
        /// Whether only operator-driven captcha recovery is permitted.
        captcha_required: bool,
    },
    /// The provider exhausted the shared user-level real-time request budget.
    #[error("real-time request {request_id} is provider-rate-limited for {retry_after:?}")]
    ProviderRateLimit {
        /// Rejected request identifier.
        request_id: RequestId,
        /// Official conservative cooldown.
        retry_after: Duration,
    },
    /// A request may have reached the provider without a trustworthy outcome.
    #[error("real-time request {request_id} outcome is uncertain; reconcile before retrying")]
    RequestOutcomeUncertain {
        /// Request requiring provider-state reconciliation.
        request_id: RequestId,
    },
    /// No inbound traffic arrived before the liveness deadline.
    #[error("real-time socket liveness timed out")]
    LivenessTimeout,
    /// The server closed the logical or physical connection.
    #[error("real-time server closed the connection")]
    ServerClosed,
    /// This generation disconnected and all pending requests were failed.
    #[error("real-time connection {connection_id} disconnected: {reason:?}")]
    Disconnected {
        /// Stopped socket generation.
        connection_id: ConnectionId,
        /// Stable termination category.
        reason: DisconnectReason,
    },
    /// An event could not be delivered and projections must be rebuilt.
    #[error("real-time connection {connection_id} requires resynchronization: {reason:?}")]
    ResyncRequired {
        /// Invalidated socket generation.
        connection_id: ConnectionId,
        /// Condition requiring resynchronization.
        reason: ResyncReason,
    },
    /// All request identifiers for this generation were exhausted.
    #[error("real-time request identifier space exhausted")]
    RequestIdExhausted,
}

impl RealtimeError {
    pub(super) const fn disconnect_reason(self) -> DisconnectReason {
        match self {
            Self::AuthorizationTimeout
            | Self::AuthorizationRejected { .. }
            | Self::AuthorizationPenalty { .. } => DisconnectReason::Authentication,
            Self::UserSyncTimeout
            | Self::UserSyncRejected { .. }
            | Self::UserSyncInvalidBootstrap
            | Self::UserSyncPenalty { .. } => DisconnectReason::Bootstrap,
            Self::LivenessTimeout => DisconnectReason::LivenessTimeout,
            Self::RequestTimeout { .. } => DisconnectReason::RequestTimeout,
            Self::ServerClosed => DisconnectReason::ServerClosed,
            Self::Protocol | Self::Codec(_) | Self::InvalidEvent { .. } => {
                DisconnectReason::Protocol
            }
            Self::ActorStopped | Self::ActorTaskFailed => DisconnectReason::ActorStopped,
            Self::InvalidConfiguration { .. }
            | Self::Unauthenticated
            | Self::RuntimeUnavailable
            | Self::ConnectTimeout
            | Self::OpenTimeout
            | Self::Transport
            | Self::InvalidRequest { .. }
            | Self::InvalidTypedResponse { .. }
            | Self::LocalRateLimit { .. }
            | Self::PendingLimitReached { .. }
            | Self::RequestQueueTimeout
            | Self::WrongSocketKind { .. }
            | Self::ProviderRejected { .. }
            | Self::ProviderBusinessFailure { .. }
            | Self::ProviderPenalty { .. }
            | Self::ProviderRateLimit { .. }
            | Self::RequestOutcomeUncertain { .. }
            | Self::Disconnected { .. }
            | Self::ResyncRequired { .. }
            | Self::RequestIdExhausted => DisconnectReason::Transport,
        }
    }
}

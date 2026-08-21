// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Typed top-level realtime event families and documentation boundaries.
//!
//! Contract reviewed 2026-08-21 against the current Partner architecture:
//! <https://partner.tradovate.com/overview/core-concepts/architecture-overview>.

pub(super) mod decode;

use std::fmt;

use super::{
    RequestId, chart::ChartEvent, market_data::MarketDataEvent, user_stream::UserStreamEvent,
};

const MAX_PROVIDER_CODE_BYTES: usize = 128;
const MAX_SHUTDOWN_REASON_BYTES: usize = 1_024;

/// A validated, bounded provider-defined response code.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ProviderCode(String);

impl ProviderCode {
    pub(super) fn from_wire(value: String) -> Option<Self> {
        valid_text(&value, MAX_PROVIDER_CODE_BYTES).then_some(Self(value))
    }

    /// Returns the provider representation.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ProviderCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// The current documented reason for a graceful server shutdown.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ShutdownReason {
    /// Scheduled provider maintenance.
    Maintenance,
    /// The connection quota was reached.
    ConnectionQuotaReached,
    /// The source-IP quota was reached.
    IpQuotaReached,
    /// A forward-compatible provider code.
    Unknown(ProviderCode),
}

/// A typed graceful-shutdown notification.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub struct ShutdownEvent {
    pub(super) reason_code: ShutdownReason,
    pub(super) explanation: Option<String>,
}

impl ShutdownEvent {
    /// Returns the provider shutdown category.
    #[must_use]
    pub const fn reason_code(&self) -> &ShutdownReason {
        &self.reason_code
    }

    /// Returns the optional bounded human explanation.
    #[must_use]
    pub fn explanation(&self) -> Option<&str> {
        self.explanation.as_deref()
    }
}

/// A current capability whose wire contract is incomplete in Partner docs.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum DocumentationBlockedCapability {
    /// B2B `splitResponses: true` has no documented completion marker.
    B2bSplitUserSync,
    /// The replay `clock` event is named but its payload schema is absent.
    ReplayClockPayload,
    /// Replay session startup and control operations are not currently published.
    ReplayControl,
}

/// Current Partner realtime capabilities intentionally withheld until their
/// wire contracts are published or confirmed by a synthetic staging fixture.
pub const DOCUMENTATION_BLOCKED_CAPABILITIES: &[DocumentationBlockedCapability] = &[
    DocumentationBlockedCapability::B2bSplitUserSync,
    DocumentationBlockedCapability::ReplayClockPayload,
    DocumentationBlockedCapability::ReplayControl,
];

/// Metadata for an event whose payload cannot safely be public yet.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DocumentationBlockedEvent {
    capability: DocumentationBlockedCapability,
}

impl DocumentationBlockedEvent {
    pub(super) const fn new(capability: DocumentationBlockedCapability) -> Self {
        Self { capability }
    }

    /// Returns the current documentation gap represented by the event.
    #[must_use]
    pub const fn capability(self) -> DocumentationBlockedCapability {
        self.capability
    }
}

/// Metadata for a response that arrived after its local request was gone.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UnmatchedResponse {
    request_id: RequestId,
    status: u16,
}

impl UnmatchedResponse {
    pub(super) const fn new(request_id: RequestId, status: u16) -> Self {
        Self { request_id, status }
    }

    /// Returns the unmatched request identifier.
    #[must_use]
    pub const fn request_id(self) -> RequestId {
        self.request_id
    }

    /// Returns the response's HTTP-style status.
    #[must_use]
    pub const fn status(self) -> u16 {
        self.status
    }
}

/// A validated typed realtime payload.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum RealtimeEventPayload {
    /// Initial user state or a provider entity delta.
    User(UserStreamEvent),
    /// Quotes, depth, or histogram updates.
    MarketData(MarketDataEvent),
    /// Regular-bar or compact tick-chart packets.
    Chart(ChartEvent),
    /// A graceful provider shutdown notification.
    Shutdown(ShutdownEvent),
    /// A late response with its raw data deliberately discarded.
    UnmatchedResponse(UnmatchedResponse),
    /// A current-documented family whose payload schema is not published.
    DocumentationBlocked(DocumentationBlockedEvent),
    /// A bounded forward-compatible event kind whose raw body is not exposed.
    Unsupported(ProviderCode),
}

impl RealtimeEventPayload {
    pub(super) fn requires_resync(&self) -> bool {
        match self {
            Self::DocumentationBlocked(_) | Self::Unsupported(_) => true,
            Self::User(UserStreamEvent::Bootstrap(bootstrap)) => {
                bootstrap.entities().iter().any(|batch| {
                    matches!(
                        batch,
                        super::user_stream::UserEntityBatch::Unsupported { .. }
                    )
                })
            }
            Self::User(UserStreamEvent::Properties(events)) => events.iter().any(|event| {
                matches!(
                    event.event_type(),
                    super::user_stream::PropertyEventType::Unknown(_)
                ) || matches!(
                    event.entities(),
                    super::user_stream::UserEntityBatch::Unsupported { .. }
                )
            }),
            Self::MarketData(_)
            | Self::Chart(_)
            | Self::Shutdown(_)
            | Self::UnmatchedResponse(_) => false,
        }
    }
}

pub(super) fn validate_shutdown_explanation(value: &str) -> bool {
    valid_text(value, MAX_SHUTDOWN_REASON_BYTES)
}

fn valid_text(value: &str, maximum: usize) -> bool {
    !value.is_empty()
        && value.len() <= maximum
        && value.trim() == value
        && !value.chars().any(char::is_control)
}

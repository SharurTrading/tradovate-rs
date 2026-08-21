// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Public real-time connection identity, state, and typed notice types.

use std::fmt;

use serde_json::value::RawValue;

use super::{Event, Response};

/// Selects one Tradovate real-time service.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum SocketKind {
    /// User, account, order, and position synchronization.
    User,
    /// Quotes, depth, histograms, and charts.
    MarketData,
    /// Historical market replay and its simulated user stream.
    Replay,
}

/// A process-local identifier for exactly one socket generation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ConnectionId(u64);

impl ConnectionId {
    pub(super) const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the integer connection identifier.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl fmt::Display for ConnectionId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

/// Why a live connection stopped.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum DisconnectReason {
    /// The owning handle requested shutdown or was dropped.
    Shutdown,
    /// Socket establishment or I/O failed.
    Transport,
    /// The server violated the expected protocol state.
    Protocol,
    /// No inbound traffic arrived before the liveness deadline.
    LivenessTimeout,
    /// An admitted request did not complete before its deadline.
    RequestTimeout,
    /// The server closed the WebSocket or logical session.
    ServerClosed,
    /// Authentication failed or timed out.
    Authentication,
    /// Mandatory user synchronization failed before readiness.
    Bootstrap,
    /// The spawned actor stopped unexpectedly.
    ActorStopped,
}

/// Why consumers must discard local projections and obtain a fresh snapshot.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ResyncReason {
    /// The bounded event queue filled before the consumer drained it.
    EventBufferOverflow,
    /// The active transport ended unexpectedly, leaving an unknown event gap.
    ConnectionLost,
    /// A caller abandoned an admitted request before observing its outcome.
    RequestAbandoned,
    /// A blocked writer could not preserve the required heartbeat schedule.
    HeartbeatDeadlineMissed,
}

/// Observable lifecycle state for one socket generation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum RealtimeState {
    /// The actor is establishing and authorizing the socket.
    Connecting {
        /// The generation being established.
        connection_id: ConnectionId,
    },
    /// The socket is authorized and accepts application requests.
    Ready {
        /// The active generation.
        connection_id: ConnectionId,
    },
    /// The generation stopped through caller-requested shutdown before a gap.
    Closed {
        /// The stopped generation.
        connection_id: ConnectionId,
        /// Stable, secret-safe termination category.
        reason: DisconnectReason,
    },
    /// An event was not delivered or the transport gap is unknown; consumers
    /// must reacquire a snapshot before accepting replacement-generation deltas.
    ResyncRequired {
        /// The affected generation.
        connection_id: ConnectionId,
        /// The condition that invalidated the projection.
        reason: ResyncReason,
    },
}

/// A server item delivered with its immutable socket-generation fence.
#[derive(Clone, Debug)]
pub struct RealtimeEvent {
    connection_id: ConnectionId,
    payload: RealtimeEventPayload,
}

impl RealtimeEvent {
    pub(super) const fn new(connection_id: ConnectionId, payload: RealtimeEventPayload) -> Self {
        Self {
            connection_id,
            payload,
        }
    }

    /// Returns the socket generation that received this item.
    #[cfg(test)]
    #[must_use]
    pub const fn connection_id(&self) -> ConnectionId {
        self.connection_id
    }

    /// Returns the received payload.
    #[cfg(test)]
    #[must_use]
    pub const fn payload(&self) -> &RealtimeEventPayload {
        &self.payload
    }

    /// Consumes the envelope and returns its payload.
    #[cfg(test)]
    #[must_use]
    pub fn into_payload(self) -> RealtimeEventPayload {
        self.payload
    }
}

/// An unsolicited or uncorrelated item received from the server.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum RealtimeEventPayload {
    /// The complete initial user synchronization response.
    Bootstrap(Response),
    /// A documented or forward-compatible Tradovate event.
    Event(Event),
    /// A response whose request already timed out or was unknown locally.
    UnmatchedResponse(Response),
    /// A forward-compatible message object without response/event markers.
    Unknown(Box<RawValue>),
}

/// A payload-free realtime notice fenced to one immutable connection generation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RealtimeNotice {
    connection_id: ConnectionId,
    kind: RealtimeNoticeKind,
}

impl RealtimeNotice {
    pub(super) fn from_event(event: RealtimeEvent) -> Self {
        let kind = match event.payload {
            RealtimeEventPayload::Bootstrap(response) => {
                drop(response);
                RealtimeNoticeKind::BootstrapComplete
            }
            RealtimeEventPayload::Event(Event::Properties(data)) => {
                drop(data);
                RealtimeNoticeKind::Properties
            }
            RealtimeEventPayload::Event(Event::Shutdown(data)) => {
                drop(data);
                RealtimeNoticeKind::Shutdown
            }
            RealtimeEventPayload::Event(Event::MarketData(data)) => {
                drop(data);
                RealtimeNoticeKind::MarketData
            }
            RealtimeEventPayload::Event(Event::Chart(data)) => {
                drop(data);
                RealtimeNoticeKind::Chart
            }
            RealtimeEventPayload::Event(Event::Clock(data)) => {
                drop(data);
                RealtimeNoticeKind::Clock
            }
            RealtimeEventPayload::Event(Event::Unknown { kind, raw }) => {
                drop((kind, raw));
                RealtimeNoticeKind::Unknown
            }
            RealtimeEventPayload::Unknown(raw) => {
                drop(raw);
                RealtimeNoticeKind::Unknown
            }
            RealtimeEventPayload::UnmatchedResponse(response) => {
                drop(response);
                RealtimeNoticeKind::UnmatchedResponse
            }
        };
        Self {
            connection_id: event.connection_id,
            kind,
        }
    }

    /// Returns the socket generation that received this notice.
    #[must_use]
    pub const fn connection_id(self) -> ConnectionId {
        self.connection_id
    }

    /// Returns the typed family of the discarded provider payload.
    #[must_use]
    pub const fn kind(self) -> RealtimeNoticeKind {
        self.kind
    }
}

/// A realtime event family whose raw provider payload remains crate-private.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum RealtimeNoticeKind {
    /// The mandatory user bootstrap passed structural validation.
    BootstrapComplete,
    /// Account or order properties changed.
    Properties,
    /// The provider announced shutdown.
    Shutdown,
    /// Market data arrived; typed values are not yet public.
    MarketData,
    /// Chart data arrived; typed values are not yet public.
    Chart,
    /// Replay clock data arrived; typed values are not yet public.
    Clock,
    /// A response arrived after local correlation ended.
    UnmatchedResponse,
    /// An unsupported forward-compatible event arrived.
    Unknown,
}

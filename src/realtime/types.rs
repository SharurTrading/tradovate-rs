// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Public realtime connection identity, lifecycle, and event envelopes.

use std::{fmt, time::Duration};

use super::event::RealtimeEventPayload;

/// Selects one Tradovate realtime service.
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
    /// The server violated the expected protocol state or typed payload contract.
    Protocol,
    /// A transmitted active WebSocket probe received no matching pong in time.
    LivenessTimeout,
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
    /// An application record could not be decoded; the transport remains active.
    MalformedRecord,
    /// The retained close-notice point filled before the consumer drained it;
    /// additional provider shutdown notices were refused with this marker.
    EventBufferOverflow,
    /// The active transport ended unexpectedly, leaving an unknown event gap.
    ConnectionLost,
    /// A caller abandoned an admitted request before observing its outcome.
    RequestAbandoned,
    /// A blocked writer could not preserve the required heartbeat schedule.
    HeartbeatDeadlineMissed,
    /// A provider event could not be represented without dropping semantics.
    UnsupportedEvent,
}

/// Observed retained-event lag for one socket generation.
///
/// Accepted venue events are retained in source order until the consumer
/// delivers them, so a slow receiver observes lag rather than a locally
/// fabricated gap. This snapshot is evidence for the consuming platform —
/// which alone decides whether a delayed event may still authorize trading —
/// and never installs a continuity gap by itself. Lifecycle evidence (shutdown
/// notices, continuity markers, generation end) is not backlog.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EventBacklog {
    depth: usize,
    oldest_event_age: Option<Duration>,
}

impl EventBacklog {
    pub(super) const fn new(depth: usize, oldest_event_age: Option<Duration>) -> Self {
        Self {
            depth,
            oldest_event_age,
        }
    }

    /// Number of accepted venue events retained but not yet delivered.
    #[must_use]
    pub const fn depth(&self) -> usize {
        self.depth
    }

    /// How long the oldest retained, undelivered venue event has waited;
    /// [`None`] when nothing is retained.
    #[must_use]
    pub const fn oldest_event_age(&self) -> Option<Duration> {
        self.oldest_event_age
    }
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
    /// The generation ended through shutdown or failed establishment.
    /// The `reason` field identifies its termination category.
    Closed {
        /// The stopped generation.
        connection_id: ConnectionId,
        /// Stable, secret-safe termination category.
        reason: DisconnectReason,
    },
    /// The active generation ended and consumer projections require recovery.
    /// Local nonterminal gaps instead arrive as `ContinuityGap` events while
    /// this latest-state view remains `Ready`.
    ResyncRequired {
        /// The affected generation.
        connection_id: ConnectionId,
        /// The condition that invalidated the projection.
        reason: ResyncReason,
    },
}

/// A typed server item fenced to one immutable socket generation.
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
    #[must_use]
    pub const fn connection_id(&self) -> ConnectionId {
        self.connection_id
    }

    /// Returns the validated typed payload.
    #[must_use]
    pub const fn payload(&self) -> &RealtimeEventPayload {
        &self.payload
    }

    /// Consumes the envelope and returns its validated typed payload.
    #[must_use]
    pub fn into_payload(self) -> RealtimeEventPayload {
        self.payload
    }
}

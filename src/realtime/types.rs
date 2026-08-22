// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Public realtime connection identity, lifecycle, and event envelopes.

use std::fmt;

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
    /// A provider event could not be represented without dropping semantics.
    UnsupportedEvent,
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
    /// A transport gap invalidated local projections.
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

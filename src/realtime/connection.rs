// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Public connection handle and [`Client`](crate::Client) integration.

mod operations;

use std::{
    fmt,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
    time::Duration,
};

use tokio::sync::{mpsc, oneshot, watch};
use tokio_util::{sync::CancellationToken, task::TaskTracker};

use super::{
    ConnectionId, FrameCodec, RealtimeConfig, RealtimeError, RealtimeEvent, RealtimeState,
    SocketKind, UserSyncConfig,
    actor::{self, ActorInput, Command},
};
use crate::{Client, auth::TokenKind};

static NEXT_CONNECTION_ID: AtomicU64 = AtomicU64::new(1);

/// An authorized, single-generation Tradovate real-time connection.
///
/// One spawned actor owns the socket and all request-correlation state. The
/// handle is intentionally not cloneable because it owns the sole event
/// receiver and tracked actor lifetime. Operational sessions are cloneable.
pub struct RealtimeConnection {
    connection_id: ConnectionId,
    kind: SocketKind,
    commands: mpsc::Sender<Command>,
    events: Arc<super::delivery::Delivery>,
    state: watch::Receiver<RealtimeState>,
    cancellation: CancellationToken,
    request_abandoned: Arc<tokio::sync::Notify>,
    tasks: TaskTracker,
    admission: Arc<super::admission::Admission>,
    request_timeout: Duration,
    codec: FrameCodec,
}

impl RealtimeConnection {
    /// Returns the immutable identifier for this socket generation.
    #[must_use]
    pub const fn connection_id(&self) -> ConnectionId {
        self.connection_id
    }

    /// Returns the provider real-time service owned by this connection.
    #[must_use]
    pub const fn socket_kind(&self) -> SocketKind {
        self.kind
    }

    /// Returns the latest lifecycle state without waiting.
    #[must_use]
    pub fn state(&self) -> RealtimeState {
        *self.state.borrow()
    }

    /// Waits for and returns the next lifecycle-state change.
    ///
    /// # Errors
    ///
    /// Returns [`RealtimeError::ActorStopped`] if the actor exited without
    /// publishing another state.
    pub async fn state_changed(&mut self) -> Result<RealtimeState, RealtimeError> {
        self.state
            .changed()
            .await
            .map_err(|_| RealtimeError::ActorStopped)?;
        Ok(*self.state.borrow_and_update())
    }

    /// Receives the next bounded unsolicited server item.
    ///
    /// `None` means the actor has stopped. Inspect [`Self::state`] for the
    /// terminal category. Accepted events precede retained continuity gaps and
    /// generation-ended evidence. A gap fences data, not requests or keepalives;
    /// acknowledge it only after installing recovery. Replacements are caller-owned.
    pub async fn recv_event(&mut self) -> Option<RealtimeEvent> {
        self.events.recv().await
    }

    /// Captures this immutable generation for concurrent subscription work.
    ///
    /// Capture before dispatching a task. The handle has no lifecycle or recovery
    /// authority and cannot migrate onto a replacement socket.
    #[must_use]
    pub fn session(&self) -> super::RealtimeSession {
        super::RealtimeSession {
            connection_id: self.connection_id,
            kind: self.kind,
            commands: self.commands.clone(),
            state: self.state.clone(),
            cancellation: self.cancellation.clone(),
            request_abandoned: Arc::clone(&self.request_abandoned),
            request_timeout: self.request_timeout,
            codec: self.codec,
            tasks: self.tasks.clone(),
            admission: Arc::clone(&self.admission),
        }
    }

    /// Acknowledges a delivered gap after the caller installs snapshot/reconciliation
    /// recovery. Stale, undelivered and ended-generation markers return `false`.
    #[must_use]
    pub fn acknowledge_continuity_gap(&mut self, gap: super::ContinuityGap) -> bool {
        self.events.acknowledge(gap)
    }

    #[cfg(test)]
    pub(super) async fn request_non_mutating(
        &self,
        endpoint: &'static str,
        query: &str,
        body: &str,
    ) -> Result<super::Response, RealtimeError> {
        self.session()
            .request_non_mutating(endpoint, query, body)
            .await
    }

    /// Cancels the connection and joins its actor task.
    ///
    /// # Errors
    ///
    /// Returns the actor's terminal typed error, or
    /// [`RealtimeError::ActorTaskFailed`] if the spawned task panicked or was
    /// externally aborted.
    pub async fn shutdown(self) -> Result<(), RealtimeError> {
        self.admission.end();
        self.cancellation.cancel();
        self.tasks.wait().await;
        self.events.result()
    }
}

impl fmt::Debug for RealtimeConnection {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RealtimeConnection")
            .field("connection_id", &self.connection_id)
            .field("kind", &self.kind)
            .field("state", &self.state())
            .field("request_timeout", &self.request_timeout)
            .finish_non_exhaustive()
    }
}

impl Drop for RealtimeConnection {
    fn drop(&mut self) {
        self.admission.end();
        self.cancellation.cancel();
    }
}

impl Client {
    /// Establishes and authorizes one Tradovate real-time socket generation.
    ///
    /// User and replay sockets use the access token. Market-data sockets use
    /// the market-data token when installed and otherwise fall back to the
    /// access token. The method spawns onto the caller's active Tokio runtime
    /// and returns only after authorization succeeds.
    ///
    /// User sockets use [`UserSyncConfig::default`], which explicitly requests
    /// all pinned current entity families in one unsplit bootstrap. Use
    /// [`Self::connect_user_realtime`] for documented filters or sharding.
    ///
    /// # Errors
    ///
    /// Returns a configuration, authentication, setup, transport, or protocol
    /// error. Token values are never retained by the error.
    pub async fn connect_realtime(
        &self,
        kind: SocketKind,
        config: RealtimeConfig,
    ) -> Result<RealtimeConnection, RealtimeError> {
        self.connect_realtime_inner(kind, config, UserSyncConfig::default())
            .await
    }

    /// Establishes a user socket with a validated current synchronization profile.
    ///
    /// The profile always uses a single response. User/account filters,
    /// point-in-time cutoff, all current entity families, socket sharding, and
    /// the full-organization flag are available through [`UserSyncConfig`].
    /// B2B `splitResponses: true` remains documentation-blocked because the
    /// provider does not publish a safe multipart completion marker.
    ///
    /// # Errors
    ///
    /// Returns a configuration, authentication, setup, transport, or protocol
    /// error. Token values are never retained by the error.
    pub async fn connect_user_realtime(
        &self,
        config: RealtimeConfig,
        user_sync: UserSyncConfig,
    ) -> Result<RealtimeConnection, RealtimeError> {
        self.connect_realtime_inner(SocketKind::User, config, user_sync)
            .await
    }

    async fn connect_realtime_inner(
        &self,
        kind: SocketKind,
        config: RealtimeConfig,
        user_sync: UserSyncConfig,
    ) -> Result<RealtimeConnection, RealtimeError> {
        let config = config.validate()?;
        let user_sync = user_sync.validate()?;
        let runtime =
            tokio::runtime::Handle::try_current().map_err(|_| RealtimeError::RuntimeUnavailable)?;
        let (url, token_kind) = match kind {
            SocketKind::User => (self.endpoints.user_websocket(), TokenKind::Access),
            SocketKind::MarketData => (
                self.endpoints.market_data_websocket(),
                TokenKind::MarketData,
            ),
            SocketKind::Replay => (self.endpoints.replay_websocket(), TokenKind::Access),
        };
        let token = self
            .tokens
            .snapshot(token_kind)
            .map_err(|_| RealtimeError::Unauthenticated)?;
        let codec = FrameCodec::new(
            config.frame_bytes_limit(),
            config.messages_per_frame_limit(),
        )?;
        let connection_id = next_connection_id();
        let (commands, command_receiver) = mpsc::channel(config.command_channel_capacity());
        let events = Arc::new(super::delivery::Delivery::new(
            connection_id,
            config.event_channel_capacity(),
        ));
        let initial_state = RealtimeState::Connecting { connection_id };
        let (state_sender, state) = watch::channel(initial_state);
        let (ready_sender, ready) = oneshot::channel();
        let cancellation = CancellationToken::new();
        let admission = Arc::new(super::admission::Admission::new());
        let request_abandoned = Arc::new(tokio::sync::Notify::new());
        let cancellation_guard = cancellation.clone().drop_guard();
        let input = ActorInput {
            connection_id,
            kind,
            url: url.as_str().to_owned(),
            token,
            tokens: Arc::clone(&self.tokens),
            config,
            user_sync,
            commands: command_receiver,
            events: Arc::clone(&events),
            state: state_sender,
            ready: ready_sender,
            cancellation: cancellation.clone(),
            request_abandoned: Arc::clone(&request_abandoned),
            rate_limits: Arc::clone(&self.rate_limits),
            admission: Arc::clone(&admission),
        };
        let tasks = TaskTracker::new();
        // The tracker owns termination accounting even if connect/shutdown is
        // cancelled or the final owner is dropped outside any runtime context.
        let actor = tasks.spawn_on(actor::tracked(input), &runtime);
        drop(actor);
        tasks.close();

        match ready.await {
            Ok(Ok(())) => Ok(RealtimeConnection {
                connection_id,
                kind,
                commands,
                events,
                state,
                cancellation: cancellation_guard.disarm(),
                request_abandoned,
                tasks,
                admission,
                request_timeout: config.request_deadline(),
                codec,
            }),
            Ok(Err(error)) => {
                drop(cancellation_guard);
                tasks.wait().await;
                Err(error)
            }
            Err(_) => {
                drop(cancellation_guard);
                tasks.wait().await;
                let result = events.result();
                match result {
                    Ok(()) => Err(RealtimeError::ActorStopped),
                    Err(error) => Err(error),
                }
            }
        }
    }
}

impl RealtimeState {
    pub(super) const fn reason_or_stopped(self) -> crate::realtime::DisconnectReason {
        match self {
            Self::Closed { reason, .. } => reason,
            Self::Connecting { .. } | Self::Ready { .. } | Self::ResyncRequired { .. } => {
                crate::realtime::DisconnectReason::ActorStopped
            }
        }
    }
}

fn next_connection_id() -> ConnectionId {
    loop {
        let value = NEXT_CONNECTION_ID.fetch_add(1, Ordering::Relaxed);
        if value != 0 {
            return ConnectionId::new(value);
        }
    }
}

#[cfg(test)]
#[path = "connection/tests.rs"]
mod tests;

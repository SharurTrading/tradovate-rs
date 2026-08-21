// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Public connection handle and [`Client`](crate::Client) integration.

use std::{
    fmt,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
    time::Duration,
};

use tokio::{
    sync::{mpsc, oneshot, watch},
    task::JoinHandle,
    time::Instant,
};
use tokio_util::sync::CancellationToken;

use super::{
    ConnectionId, FrameCodec, RealtimeConfig, RealtimeError, RealtimeEvent, RealtimeState,
    Response, SocketKind, UserSyncConfig,
    actor::{self, ActorInput, Command},
};
use crate::{Client, auth::TokenKind};

static NEXT_CONNECTION_ID: AtomicU64 = AtomicU64::new(1);

/// An authorized, single-generation Tradovate real-time connection.
///
/// One spawned actor owns the socket and all request-correlation state. The
/// handle is intentionally not cloneable because it owns the sole event
/// receiver and actor join handle.
pub struct RealtimeConnection {
    connection_id: ConnectionId,
    kind: SocketKind,
    commands: mpsc::Sender<Command>,
    events: mpsc::Receiver<RealtimeEvent>,
    state: watch::Receiver<RealtimeState>,
    cancellation: CancellationToken,
    request_abandoned: CancellationToken,
    actor: Option<JoinHandle<Result<(), RealtimeError>>>,
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
    /// terminal category, then establish any required recovery boundary before
    /// accepting events from a replacement connection.
    pub async fn recv_event(&mut self) -> Option<RealtimeEvent> {
        self.events.recv().await
    }

    /// Sends one crate-validated, non-money-moving request and waits for its response.
    ///
    /// The actor assigns the request identifier. Requests are never replayed,
    /// including after timeout or disconnect. Borrowed fields are validated
    /// before queue allocation; the request deadline also bounds waiting for
    /// command-queue capacity. The sole writer performs final rate admission
    /// immediately before transmission. Once admitted, dropping
    /// this future invalidates the entire socket generation because the caller can
    /// no longer observe the request's outcome.
    ///
    /// # Errors
    ///
    /// Returns a typed validation, capacity, timeout, protocol, or disconnect
    /// failure. Errors never retain `body`.
    pub(super) async fn request_non_mutating(
        &self,
        endpoint: &'static str,
        query: &str,
        body: &str,
    ) -> Result<Response, RealtimeError> {
        self.codec.validate_request(endpoint, query, body)?;
        let deadline = Instant::now().checked_add(self.request_timeout).ok_or(
            RealtimeError::InvalidConfiguration {
                field: "request_timeout",
                reason: "is too large for a monotonic deadline",
            },
        )?;
        let disconnected = RealtimeError::Disconnected {
            connection_id: self.connection_id,
            reason: self.state().reason_or_stopped(),
        };
        let permit =
            reserve_command_slot(&self.commands, &self.cancellation, deadline, disconnected)
                .await?;
        let (reply, response) = oneshot::channel();
        let mut wait_guard = RequestWaitGuard::new(self.request_abandoned.clone());
        permit.send(Command::Request {
            endpoint,
            query: query.to_owned(),
            body: body.to_owned(),
            deadline,
            reply,
        });
        wait_guard.arm();

        let Ok(result) = response.await else {
            return Err(RealtimeError::ActorStopped);
        };
        wait_guard.complete();
        result
    }

    /// Cancels the connection and joins its actor task.
    ///
    /// # Errors
    ///
    /// Returns the actor's terminal typed error, or
    /// [`RealtimeError::ActorTaskFailed`] if the spawned task panicked or was
    /// externally aborted.
    pub async fn shutdown(mut self) -> Result<(), RealtimeError> {
        self.cancellation.cancel();
        let Some(actor) = self.actor.take() else {
            return Ok(());
        };
        actor.await.map_err(|_| RealtimeError::ActorTaskFailed)?
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
        self.cancellation.cancel();
        if let Some(actor) = self.actor.take() {
            actor.abort();
        }
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
        let (event_sender, events) = mpsc::channel(config.event_channel_capacity());
        let initial_state = RealtimeState::Connecting { connection_id };
        let (state_sender, state) = watch::channel(initial_state);
        let (ready_sender, ready) = oneshot::channel();
        let cancellation = CancellationToken::new();
        let request_abandoned = CancellationToken::new();
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
            events: event_sender,
            state: state_sender,
            ready: ready_sender,
            cancellation: cancellation.clone(),
            request_abandoned: request_abandoned.clone(),
            rate_limits: Arc::clone(&self.rate_limits),
        };
        let mut actor = runtime.spawn(actor::run(input));

        match ready.await {
            Ok(Ok(())) => Ok(RealtimeConnection {
                connection_id,
                kind,
                commands,
                events,
                state,
                cancellation: cancellation_guard.disarm(),
                request_abandoned,
                actor: Some(actor),
                request_timeout: config.request_deadline(),
                codec,
            }),
            Ok(Err(error)) => {
                drop(cancellation_guard);
                let _ = (&mut actor).await;
                Err(error)
            }
            Err(_) => {
                drop(cancellation_guard);
                let result = (&mut actor)
                    .await
                    .map_err(|_| RealtimeError::ActorTaskFailed)?;
                match result {
                    Ok(()) => Err(RealtimeError::ActorStopped),
                    Err(error) => Err(error),
                }
            }
        }
    }
}

impl RealtimeState {
    const fn reason_or_stopped(self) -> crate::realtime::DisconnectReason {
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

struct RequestWaitGuard {
    poison: CancellationToken,
    armed: bool,
}

impl RequestWaitGuard {
    fn new(poison: CancellationToken) -> Self {
        Self {
            poison,
            armed: false,
        }
    }

    fn arm(&mut self) {
        self.armed = true;
    }

    fn complete(mut self) {
        self.armed = false;
    }
}

impl Drop for RequestWaitGuard {
    fn drop(&mut self) {
        if self.armed {
            self.poison.cancel();
        }
    }
}

async fn reserve_command_slot<'a>(
    commands: &'a mpsc::Sender<Command>,
    cancellation: &CancellationToken,
    deadline: Instant,
    disconnected: RealtimeError,
) -> Result<mpsc::Permit<'a, Command>, RealtimeError> {
    tokio::select! {
        biased;
        () = cancellation.cancelled() => Err(disconnected),
        () = tokio::time::sleep_until(deadline) => Err(RealtimeError::RequestQueueTimeout),
        result = commands.reserve() => result.map_err(|_| RealtimeError::ActorStopped),
    }
}

#[cfg(test)]
#[path = "connection/tests.rs"]
mod tests;

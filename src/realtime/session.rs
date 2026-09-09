// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Generation-bound operational request admission, independent of event reception.

use super::{ConnectionId, FrameCodec, RealtimeError, Response, SocketKind, actor::Command};
use std::{sync::Arc, time::Duration};
use tokio::{
    sync::{mpsc, oneshot},
    time::Instant,
};
use tokio_util::sync::CancellationToken;

/// A cloneable operational handle for exactly one immutable socket generation.
///
/// This does not keep the connection owner alive or authorize teardown/recovery.
/// Unknown subscription outcomes remain the caller's responsibility until exact
/// provider evidence or `GenerationEnded`; no subscription set is stored here.
/// Queue admission cancelled by teardown returns `StaleGeneration` before enqueue;
/// the owner's retained `GenerationEnded` event carries the actual terminal result.
#[derive(Clone)]
pub struct RealtimeSession {
    pub(super) connection_id: ConnectionId,
    pub(super) kind: SocketKind,
    pub(super) commands: mpsc::Sender<Command>,
    pub(super) cancellation: CancellationToken,
    pub(super) request_abandoned: Arc<tokio::sync::Notify>,
    pub(super) request_timeout: Duration,
    pub(super) codec: FrameCodec,
    pub(super) tasks: tokio_util::task::TaskTracker,
    pub(super) admission: Arc<super::admission::Admission>,
}

impl RealtimeSession {
    /// Waits for actual producer termination, including tracked cancellation
    /// cleanup. This never requests disconnect or reports success for a pending
    /// subscription; it proves only that this generation can no longer produce.
    pub async fn wait_ended(&self) {
        self.tasks.wait().await;
    }

    /// Returns the captured socket generation.
    #[must_use]
    pub const fn connection_id(&self) -> ConnectionId {
        self.connection_id
    }

    /// Returns this generation's provider service.
    #[must_use]
    pub const fn socket_kind(&self) -> SocketKind {
        self.kind
    }

    fn ensure_active(&self) -> Result<(), RealtimeError> {
        if self.cancellation.is_cancelled() || self.commands.is_closed() {
            Err(RealtimeError::StaleGeneration {
                connection_id: self.connection_id,
            })
        } else {
            Ok(())
        }
    }
    /// Sends one crate-validated, non-money-moving request and waits for its response.
    ///
    /// The actor assigns the request identifier. Requests are never replayed,
    /// including after timeout or disconnect. Borrowed fields are validated
    /// before queue allocation; the request deadline also bounds waiting for
    /// command-queue capacity. The sole writer performs final rate admission
    /// immediately before transmission. Cancellation after admission leaves an
    /// unknown subscription outcome. It does not end this generation.
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
        self.ensure_active()?;
        self.codec.validate_request(endpoint, query, body)?;
        let deadline = Instant::now().checked_add(self.request_timeout).ok_or(
            RealtimeError::InvalidConfiguration {
                field: "request_timeout",
                reason: "is too large for a monotonic deadline",
            },
        )?;
        let permit = reserve_command_slot(
            &self.commands,
            &self.cancellation,
            deadline,
            self.connection_id,
        )
        .await?;
        self.ensure_active()?;
        let (reply, response) = oneshot::channel();
        let invocation = super::admission::Invocation::new();
        let mut wait_guard =
            RequestWaitGuard::new(Arc::clone(&self.request_abandoned), invocation.clone());
        let request_id = self.admission.enqueue(self.connection_id, |request_id| {
            permit.send(Command::Request {
                connection_id: self.connection_id,
                request_id,
                invocation,
                endpoint,
                query: query.to_owned(),
                body: body.to_owned(),
                deadline,
                reply,
            });
        })?;
        wait_guard.arm();
        let result = tokio::select! {
            biased;
            result = response => result.unwrap_or(Err(RealtimeError::RequestOutcomeUncertain { request_id })),
            () = tokio::time::sleep_until(deadline) => {
                if wait_guard.invocation.abandon() { Err(RealtimeError::RequestQueueTimeout) }
                else { Err(RealtimeError::RequestOutcomeUncertain { request_id }) }
            }
        };
        wait_guard.complete();
        result
    }
}

struct RequestWaitGuard {
    notify: Arc<tokio::sync::Notify>,
    armed: bool,
    invocation: super::admission::Invocation,
}

impl RequestWaitGuard {
    fn new(notify: Arc<tokio::sync::Notify>, invocation: super::admission::Invocation) -> Self {
        Self {
            notify,
            armed: false,
            invocation,
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
            self.invocation.abandon();
            self.notify.notify_one();
        }
    }
}

pub(super) async fn reserve_command_slot<'a>(
    commands: &'a mpsc::Sender<Command>,
    cancellation: &CancellationToken,
    deadline: Instant,
    connection_id: ConnectionId,
) -> Result<mpsc::Permit<'a, Command>, RealtimeError> {
    tokio::select! {
        biased;
        () = cancellation.cancelled() => Err(RealtimeError::StaleGeneration { connection_id }),
        () = tokio::time::sleep_until(deadline) => Err(RealtimeError::RequestQueueTimeout),
        result = commands.reserve() => result.map_err(|_| RealtimeError::StaleGeneration { connection_id }),
    }
}

impl std::fmt::Debug for RealtimeSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RealtimeSession")
            .field("connection_id", &self.connection_id)
            .field("kind", &self.kind)
            .finish_non_exhaustive()
    }
}

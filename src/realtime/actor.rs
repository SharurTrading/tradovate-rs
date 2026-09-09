// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Single-owner WebSocket actor and request correlation.

mod handshake;
mod lifecycle;
mod pending;
mod receive;
mod response;
mod task;
mod writer;
pub(super) use task::tracked;

use futures_util::{SinkExt, StreamExt};
use tokio::{
    sync::{mpsc, oneshot, watch},
    time::{self, Instant},
};
use tokio_tungstenite::tungstenite::Message;
use tokio_util::sync::CancellationToken;

use self::handshake::{AuthorizationFence, EstablishInput, SocketReader, SocketWriter};
use self::lifecycle::{
    normalize_shutdown, publish_active_terminal, publish_setup_failure, publish_shutdown,
};
use self::pending::{PendingReply, PendingRequests, wait_for_deadline};
use super::{
    ConnectionId, FrameCodec, RealtimeConfig, RealtimeError, RealtimeState, Response, ResyncReason,
    ServerMessage, event::decode,
};
use crate::{
    auth::{TokenSnapshot, TokenStore},
    rate_limit::RateGovernor,
};

pub(super) struct ActorInput {
    pub(super) connection_id: ConnectionId,
    pub(super) kind: crate::realtime::SocketKind,
    pub(super) url: String,
    pub(super) token: TokenSnapshot,
    pub(super) tokens: std::sync::Arc<TokenStore>,
    pub(super) config: RealtimeConfig,
    pub(super) user_sync: crate::realtime::UserSyncConfig,
    pub(super) admission: std::sync::Arc<super::admission::Admission>,
    pub(super) commands: mpsc::Receiver<Command>,
    pub(super) events: std::sync::Arc<super::delivery::Delivery>,
    pub(super) state: watch::Sender<RealtimeState>,
    pub(super) ready: oneshot::Sender<Result<(), RealtimeError>>,
    pub(super) cancellation: CancellationToken,
    pub(super) request_abandoned: std::sync::Arc<tokio::sync::Notify>,
    pub(super) rate_limits: std::sync::Arc<RateGovernor>,
}

pub(super) enum Command {
    Request {
        connection_id: ConnectionId,
        request_id: super::RequestId,
        invocation: super::admission::Invocation,
        endpoint: &'static str,
        query: String,
        body: String,
        deadline: Instant,
        reply: oneshot::Sender<Result<Response, RealtimeError>>,
    },
}

enum Wake {
    Shutdown,
    Abandoned,
    Deadline,
    Probe,
    Heartbeat,
    Record,
    Command(Option<Command>),
    Socket(Option<Result<Message, tokio_tungstenite::tungstenite::Error>>),
}

struct Actor {
    connection_id: ConnectionId,
    config: RealtimeConfig,
    codec: FrameCodec,
    writer: SocketWriter,
    reader: SocketReader,
    commands: mpsc::Receiver<Command>,
    events: std::sync::Arc<super::delivery::Delivery>,
    cancellation: CancellationToken,
    request_abandoned: std::sync::Arc<tokio::sync::Notify>,
    pending: PendingRequests,
    last_received: Instant,
    probe: Option<(Vec<u8>, Instant)>,
    next_probe: u64,
    batch: Option<super::codec::RecordBatch>,
    batch_epoch: Option<u64>,
    heartbeat: time::Interval,
    rate_limits: std::sync::Arc<RateGovernor>,
}

pub(super) async fn run(input: ActorInput) -> Result<(), RealtimeError> {
    let ActorInput {
        connection_id,
        kind,
        url,
        token,
        tokens,
        config,
        user_sync,
        commands,
        events,
        state,
        ready,
        cancellation,
        request_abandoned,
        rate_limits,
        admission,
    } = input;
    let established = handshake::establish(EstablishInput {
        connection_id,
        kind,
        url: &url,
        authorization: AuthorizationFence::new(&token, &tokens),
        config,
        sync_config: &user_sync,
        cancellation: &cancellation,
        rate_limits: &rate_limits,
    })
    .await;
    drop(token);
    drop(tokens);
    let established = match established {
        Ok(socket) => socket,
        Err(error) => {
            publish_setup_failure(&state, connection_id, error);
            let _ready_result = ready.send(Err(error));
            return Err(error);
        }
    };
    let codec = FrameCodec::new(
        config.frame_bytes_limit(),
        config.messages_per_frame_limit(),
    )?;
    let heartbeat = established.heartbeat;
    let mut actor = Actor {
        connection_id,
        config,
        codec,
        writer: established.writer,
        reader: established.reader,
        commands,
        events,
        cancellation,
        request_abandoned,
        pending: PendingRequests::with_capacity(config.pending_requests_limit()),
        last_received: Instant::now(),
        probe: None,
        next_probe: 0,
        batch: None,
        batch_epoch: Some(1),
        heartbeat,
        rate_limits,
    };
    if let Err(error) = actor.publish_established(established.bootstrap, established.staged) {
        publish_setup_failure(&state, connection_id, error);
        let _ready_result = ready.send(Err(error));
        return Err(error);
    }
    admission.activate(established.next_request_id);
    let _previous_state = state.send_replace(RealtimeState::Ready { connection_id });
    if ready.send(Ok(())).is_err() {
        drop(actor);
        publish_shutdown(&state, connection_id);
        return Ok(());
    }
    let result = normalize_shutdown(actor.event_loop().await);
    actor.pending.drain_uncertain();
    admission.end();
    actor.cancellation.cancel();
    let delivery = std::sync::Arc::clone(&actor.events);
    drop(actor); // Both socket halves and every pending producer stop before this evidence.
    delivery.end(result);
    publish_active_terminal(&state, connection_id, result);
    result
}

impl Actor {
    fn publish_established(
        &mut self,
        bootstrap: Option<Response>,
        staged: Vec<ServerMessage>,
    ) -> Result<(), RealtimeError> {
        if let Some(bootstrap) = bootstrap {
            let payload =
                super::bounded::with_limit(self.config.collection_entries_limit(), || {
                    decode::bootstrap(&bootstrap)
                })?;
            if payload.requires_resync() {
                return Err(RealtimeError::ResyncRequired {
                    connection_id: self.connection_id,
                    reason: ResyncReason::UnsupportedEvent,
                });
            }
            self.events.publish(payload);
        }
        for message in staged {
            self.handle_server_message(message);
        }
        Ok(())
    }

    async fn event_loop(&mut self) -> Result<(), RealtimeError> {
        loop {
            let pending_deadline = self.pending.next_deadline();
            let liveness_deadline = self.probe.as_ref().map_or_else(
                || self.last_received + self.config.liveness_deadline(),
                |(_, deadline)| *deadline,
            );
            let wake = tokio::select! {
                biased;
                () = self.cancellation.cancelled() => Wake::Shutdown,
                () = self.request_abandoned.notified() => Wake::Abandoned,
                () = wait_for_deadline(pending_deadline) => Wake::Deadline,
                () = time::sleep_until(liveness_deadline) => Wake::Probe,
                _ = self.heartbeat.tick() => Wake::Heartbeat,
                wake = async {
                    tokio::select! {
                        command = self.commands.recv() => Wake::Command(command),
                        message = self.reader.next(), if self.batch.is_none() => Wake::Socket(message),
                        () = std::future::ready(()), if self.batch.is_some() => Wake::Record,
                    }
                } => wake,
            };
            match wake {
                Wake::Shutdown => {
                    self.close().await;
                    return Ok(());
                }
                Wake::Abandoned => self.pending.reap_cancelled(),
                Wake::Deadline => {
                    self.pending.expire();
                }
                Wake::Probe => self.check_liveness().await?,
                Wake::Heartbeat => {
                    self.send_heartbeat().await?;
                }
                Wake::Command(Some(command)) => self.handle_command(command).await?,
                Wake::Command(None) => return Err(RealtimeError::ActorStopped),
                Wake::Socket(message) => self.handle_socket(message).await?,
                Wake::Record => {
                    self.handle_next_record();
                    tokio::task::yield_now().await;
                }
            }
        }
    }

    async fn handle_command(&mut self, command: Command) -> Result<(), RealtimeError> {
        match command {
            Command::Request {
                connection_id,
                request_id,
                invocation,
                endpoint,
                query,
                body,
                deadline,
                reply,
            } => {
                if connection_id != self.connection_id || self.cancellation.is_cancelled() {
                    drop(reply.send(Err(RealtimeError::StaleGeneration { connection_id })));
                    return Ok(());
                }
                if reply.is_closed() {
                    return Ok(());
                }
                if self.pending.len() >= self.config.pending_requests_limit() {
                    drop(reply.send(Err(RealtimeError::PendingLimitReached {
                        limit: self.config.pending_requests_limit(),
                    })));
                    return Ok(());
                }
                if deadline <= Instant::now() {
                    drop(reply.send(Err(RealtimeError::RequestTimeout { request_id })));
                    return Ok(());
                }
                let frame = match self
                    .codec
                    .encode_request(endpoint, request_id, &query, &body)
                {
                    Ok(frame) => frame,
                    Err(error) => {
                        drop(reply.send(Err(error.into())));
                        return Ok(());
                    }
                };
                let retry_after = self.rate_limits.try_admit_authenticated(endpoint);
                if !retry_after.is_zero() {
                    drop(reply.send(Err(RealtimeError::LocalRateLimit {
                        endpoint,
                        retry_after,
                    })));
                    return Ok(());
                }
                if deadline <= Instant::now() || !invocation.start() {
                    drop(reply.send(Err(RealtimeError::RequestQueueTimeout)));
                    return Ok(());
                }
                if let Err(error) = self.send_message(Message::text(frame)).await {
                    drop(reply.send(Err(RealtimeError::RequestOutcomeUncertain { request_id })));
                    return Err(error);
                }
                self.pending.insert(request_id, endpoint, deadline, reply);
                Ok(())
            }
        }
    }

    async fn send_heartbeat(&mut self) -> Result<(), RealtimeError> {
        let frame = self.codec.encode_heartbeat()?;
        self.send_message(Message::text(frame)).await
    }

    async fn send_message(&mut self, message: Message) -> Result<(), RealtimeError> {
        let operation_deadline = Instant::now()
            .checked_add(self.config.write_deadline())
            .ok_or(RealtimeError::InvalidConfiguration {
                field: "request_timeout",
                reason: "is too large for a monotonic deadline",
            })?;
        let control =
            writer::SendControl::new(self.connection_id, &self.cancellation, operation_deadline);
        writer::send(&mut self.writer, message, control).await
    }

    async fn close(&mut self) {
        let close = self.writer.send(Message::Close(None));
        drop(time::timeout(self.config.request_deadline(), close).await);
    }
}

#[cfg(test)]
mod tests;

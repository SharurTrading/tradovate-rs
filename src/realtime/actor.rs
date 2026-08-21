// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Single-owner WebSocket actor and request correlation.

mod handshake;
mod lifecycle;
mod pending;
mod response;
mod writer;

use futures_util::{SinkExt, StreamExt};
use tokio::{
    sync::{mpsc, oneshot, watch},
    time::{self, Instant},
};
use tokio_tungstenite::tungstenite::Message;
use tokio_util::sync::CancellationToken;

use self::handshake::{AuthorizationFence, SocketReader, SocketWriter};
use self::lifecycle::{
    normalize_shutdown, publish_active_terminal, publish_setup_failure, publish_shutdown,
};
use self::pending::{PendingReply, PendingRequests, wait_for_deadline};
use super::{
    ConnectionId, FrameCodec, RealtimeConfig, RealtimeError, RealtimeEvent, RealtimeEventPayload,
    RealtimeState, Response, ResyncReason, ServerFrame, ServerMessage,
};
use crate::auth::{TokenSnapshot, TokenStore};
use crate::rate_limit::RateGovernor;

pub(super) struct ActorInput {
    pub(super) connection_id: ConnectionId,
    pub(super) kind: crate::realtime::SocketKind,
    pub(super) url: String,
    pub(super) token: TokenSnapshot,
    pub(super) tokens: std::sync::Arc<TokenStore>,
    pub(super) config: RealtimeConfig,
    pub(super) commands: mpsc::Receiver<Command>,
    pub(super) events: mpsc::Sender<RealtimeEvent>,
    pub(super) state: watch::Sender<RealtimeState>,
    pub(super) ready: oneshot::Sender<Result<(), RealtimeError>>,
    pub(super) cancellation: CancellationToken,
    pub(super) request_abandoned: CancellationToken,
    pub(super) rate_limits: std::sync::Arc<RateGovernor>,
}

pub(super) enum Command {
    Request {
        endpoint: &'static str,
        query: String,
        body: String,
        deadline: Instant,
        reply: oneshot::Sender<Result<Response, RealtimeError>>,
    },
}

struct Actor {
    connection_id: ConnectionId,
    config: RealtimeConfig,
    codec: FrameCodec,
    writer: SocketWriter,
    reader: SocketReader,
    commands: mpsc::Receiver<Command>,
    events: mpsc::Sender<RealtimeEvent>,
    cancellation: CancellationToken,
    request_abandoned: CancellationToken,
    pending: PendingRequests,
    last_received: Instant,
    heartbeat: time::Interval,
    heartbeat_deadline: Instant,
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
        commands,
        events,
        state,
        ready,
        cancellation,
        request_abandoned,
        rate_limits,
    } = input;
    let established = handshake::establish(
        connection_id,
        kind,
        &url,
        AuthorizationFence::new(&token, &tokens),
        config,
        &cancellation,
        &rate_limits,
    )
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
    let heartbeat_deadline = established.heartbeat_deadline;
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
        pending: PendingRequests::with_capacity(
            config.pending_requests_limit(),
            established.next_request_id,
        ),
        last_received: Instant::now(),
        heartbeat,
        heartbeat_deadline,
        rate_limits,
    };
    if let Err(error) = actor.publish_established(established.bootstrap, established.staged) {
        publish_setup_failure(&state, connection_id, error);
        let _ready_result = ready.send(Err(error));
        return Err(error);
    }
    let _previous_state = state.send_replace(RealtimeState::Ready { connection_id });
    if ready.send(Ok(())).is_err() {
        publish_shutdown(&state, connection_id);
        return Ok(());
    }
    let result = normalize_shutdown(actor.event_loop().await);
    actor.pending.drain_uncertain();
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
            self.publish_event(RealtimeEventPayload::Bootstrap(bootstrap))?;
        }
        for message in staged {
            self.handle_server_message(message)?;
        }
        Ok(())
    }

    async fn event_loop(&mut self) -> Result<(), RealtimeError> {
        loop {
            let pending_deadline = self.pending.next_deadline();
            let liveness_deadline = self
                .last_received
                .checked_add(self.config.liveness_deadline())
                .ok_or(RealtimeError::InvalidConfiguration {
                    field: "liveness_timeout",
                    reason: "is too large for a monotonic deadline",
                })?;
            tokio::select! {
                biased;
                () = self.request_abandoned.cancelled() => {
                    return Err(RealtimeError::ResyncRequired {
                        connection_id: self.connection_id,
                        reason: ResyncReason::RequestAbandoned,
                    });
                }
                () = self.cancellation.cancelled() => {
                    self.close().await;
                    return Ok(());
                }
                () = wait_for_deadline(pending_deadline) => {
                    if let Some(request_id) = self.pending.expire() {
                        return Err(RealtimeError::RequestTimeout { request_id });
                    }
                }
                () = time::sleep_until(liveness_deadline) => {
                    return Err(RealtimeError::LivenessTimeout);
                }
                _ = self.heartbeat.tick() => {
                    self.advance_heartbeat_deadline()?;
                    self.send_heartbeat().await?;
                }
                message = self.reader.next() => self.handle_socket(message).await?,
                command = self.commands.recv() => {
                    let Some(command) = command else {
                        self.close().await;
                        return Ok(());
                    };
                    self.handle_command(command).await?;
                }
            }
        }
    }

    async fn handle_socket(
        &mut self,
        message: Option<Result<Message, tokio_tungstenite::tungstenite::Error>>,
    ) -> Result<(), RealtimeError> {
        let message = message
            .ok_or(RealtimeError::ServerClosed)?
            .map_err(|_| RealtimeError::Transport)?;
        self.last_received = Instant::now();
        match message {
            Message::Text(text) => self.handle_frame(self.codec.decode(text.as_str())?),
            Message::Ping(payload) => self.send_message(Message::Pong(payload)).await,
            Message::Pong(_) => Ok(()),
            Message::Close(_) => Err(RealtimeError::ServerClosed),
            Message::Binary(_) | Message::Frame(_) => Err(RealtimeError::Protocol),
        }
    }

    fn handle_frame(&mut self, frame: ServerFrame) -> Result<(), RealtimeError> {
        match frame {
            ServerFrame::Heartbeat => Ok(()),
            ServerFrame::Messages(messages) => {
                for message in messages {
                    self.handle_server_message(message)?;
                }
                Ok(())
            }
            ServerFrame::Close { code, reason } => {
                tracing::debug!(
                    code,
                    reason_length = reason.len(),
                    "Tradovate logical WebSocket close"
                );
                Err(RealtimeError::ServerClosed)
            }
            ServerFrame::Open => Err(RealtimeError::Protocol),
        }
    }

    fn handle_server_message(&mut self, message: ServerMessage) -> Result<(), RealtimeError> {
        let payload = match message {
            ServerMessage::Response(response) => {
                let request_id = response.request_id();
                match self.pending.remove_for_response(request_id, Instant::now()) {
                    Some(PendingReply::Active { endpoint, reply }) => {
                        match response::classify(response, request_id, endpoint, &self.rate_limits)
                        {
                            response::Disposition::Complete(result) => {
                                drop(reply.send(result));
                                return Ok(());
                            }
                            response::Disposition::Terminate(error) => {
                                drop(reply.send(Err(error)));
                                return Err(error);
                            }
                        }
                    }
                    Some(PendingReply::Expired(reply)) => {
                        let error = RealtimeError::RequestTimeout { request_id };
                        drop(reply.send(Err(error)));
                        return Err(error);
                    }
                    None => {}
                }
                RealtimeEventPayload::UnmatchedResponse(response)
            }
            ServerMessage::Event(event) => RealtimeEventPayload::Event(event),
            ServerMessage::Unknown(raw) => RealtimeEventPayload::Unknown(raw),
        };
        self.publish_event(payload)
    }

    async fn handle_command(&mut self, command: Command) -> Result<(), RealtimeError> {
        match command {
            Command::Request {
                endpoint,
                query,
                body,
                deadline,
                reply,
            } => {
                if reply.is_closed() {
                    return Ok(());
                }
                if self.pending.len() >= self.config.pending_requests_limit() {
                    drop(reply.send(Err(RealtimeError::PendingLimitReached {
                        limit: self.config.pending_requests_limit(),
                    })));
                    return Ok(());
                }
                let request_id = self.pending.allocate_request_id()?;
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
                if let Err(error) = self.send_before(frame, deadline).await {
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

    fn advance_heartbeat_deadline(&mut self) -> Result<(), RealtimeError> {
        self.heartbeat_deadline = self
            .heartbeat_deadline
            .checked_add(writer::HEARTBEAT_PERIOD)
            .ok_or(RealtimeError::InvalidConfiguration {
                field: "heartbeat_period",
                reason: "is too large for a monotonic deadline",
            })?;
        Ok(())
    }

    async fn send_message(&mut self, message: Message) -> Result<(), RealtimeError> {
        let operation_deadline = Instant::now()
            .checked_add(self.config.request_deadline())
            .ok_or(RealtimeError::InvalidConfiguration {
                field: "request_timeout",
                reason: "is too large for a monotonic deadline",
            })?;
        let control = writer::SendControl::new(
            self.connection_id,
            &self.cancellation,
            &self.request_abandoned,
            self.heartbeat_deadline,
            operation_deadline,
        );
        writer::send(&mut self.writer, message, control).await
    }

    async fn send_before(&mut self, frame: String, deadline: Instant) -> Result<(), RealtimeError> {
        let control = writer::SendControl::new(
            self.connection_id,
            &self.cancellation,
            &self.request_abandoned,
            self.heartbeat_deadline,
            deadline,
        );
        writer::send(&mut self.writer, Message::text(frame), control).await
    }

    fn publish_event(&self, payload: RealtimeEventPayload) -> Result<(), RealtimeError> {
        let event = RealtimeEvent::new(self.connection_id, payload);
        match self.events.try_send(event) {
            Ok(()) => Ok(()),
            Err(mpsc::error::TrySendError::Full(_)) => Err(RealtimeError::ResyncRequired {
                connection_id: self.connection_id,
                reason: ResyncReason::EventBufferOverflow,
            }),
            Err(mpsc::error::TrySendError::Closed(_)) => Err(RealtimeError::ActorStopped),
        }
    }

    async fn close(&mut self) {
        let close = self.writer.send(Message::Close(None));
        drop(time::timeout(self.config.request_deadline(), close).await);
    }
}

#[cfg(test)]
mod tests;

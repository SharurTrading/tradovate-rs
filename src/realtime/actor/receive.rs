// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Active transport liveness and nonterminal application-record handling.

use tokio::time::Instant;
use tokio_tungstenite::tungstenite::{Error as SocketError, Message};

use super::{Actor, PendingReply, RealtimeError, ResyncReason, ServerMessage, decode, response};
use crate::realtime::{ServerFrame, codec::RecordBatch};

impl Actor {
    pub(super) async fn check_liveness(&mut self) -> Result<(), RealtimeError> {
        if self.probe.is_some() {
            return Err(RealtimeError::LivenessTimeout);
        }
        self.next_probe = self
            .next_probe
            .checked_add(1)
            .ok_or(RealtimeError::RequestIdExhausted)?;
        let payload = self.next_probe.to_be_bytes().to_vec();
        self.send_message(Message::Ping(payload.clone().into()))
            .await?;
        self.probe = Some((payload, Instant::now() + self.config.liveness_deadline()));
        Ok(())
    }

    pub(super) async fn handle_socket(
        &mut self,
        message: Option<Result<Message, SocketError>>,
    ) -> Result<(), RealtimeError> {
        let message = message
            .ok_or(RealtimeError::ServerClosed)?
            .map_err(|_| RealtimeError::Transport)?;
        self.last_received = Instant::now();
        match message {
            Message::Text(text) if text.starts_with('a') => {
                match RecordBatch::new(text, self.config.messages_per_frame_limit()) {
                    Ok(batch) => {
                        self.batch_epoch = self.events.epoch();
                        self.batch = Some(batch);
                    }
                    Err(_) => self.events.gap(ResyncReason::MalformedRecord),
                }
                Ok(())
            }
            Message::Text(text) => match self.codec.decode(text.as_str()) {
                Ok(ServerFrame::Heartbeat) => Ok(()),
                Ok(ServerFrame::Close { code, reason }) => {
                    tracing::debug!(
                        code,
                        reason_length = reason.len(),
                        "Tradovate logical WebSocket close"
                    );
                    Err(RealtimeError::ServerClosed)
                }
                _ => {
                    self.events.gap(ResyncReason::MalformedRecord);
                    Ok(())
                }
            },
            Message::Ping(payload) => self.send_message(Message::Pong(payload)).await,
            Message::Pong(payload) => {
                if self
                    .probe
                    .as_ref()
                    .is_some_and(|(expected, _)| expected.as_slice() == payload.as_ref())
                {
                    self.probe = None;
                }
                Ok(())
            }
            Message::Close(_) => Err(RealtimeError::ServerClosed),
            Message::Binary(_) | Message::Frame(_) => {
                self.events.gap(ResyncReason::MalformedRecord);
                Ok(())
            }
        }
    }

    pub(super) fn handle_next_record(&mut self) {
        match self.batch.as_mut().and_then(RecordBatch::next) {
            Some(Ok(message)) => self.handle_server_message(message),
            Some(Err(_)) => {
                self.events
                    .gap_at(self.batch_epoch, ResyncReason::MalformedRecord);
            }
            None => {
                self.batch = None;
                // The reader was deliberately parked while decoding this bounded
                // message. Buffered work is not evidence of silence, and a pong
                // behind it must receive a full read opportunity before expiry.
                let now = Instant::now();
                self.last_received = now;
                if let Some((_, deadline)) = &mut self.probe {
                    *deadline = now + self.config.liveness_deadline();
                }
            }
        }
    }

    pub(super) fn handle_server_message(&mut self, message: ServerMessage) {
        let message = match message {
            ServerMessage::Response(response) => {
                let request_id = response.request_id();
                match self.pending.remove_for_response(request_id, Instant::now()) {
                    Some(PendingReply::Active { endpoint, reply }) => {
                        let result =
                            response::classify(response, request_id, endpoint, &self.rate_limits);
                        if matches!(result, Err(RealtimeError::Protocol)) {
                            self.events
                                .gap_at(self.batch_epoch, ResyncReason::MalformedRecord);
                        }
                        drop(reply.send(result));
                        return;
                    }
                    Some(PendingReply::Expired(reply)) => {
                        drop(
                            reply.send(Err(RealtimeError::RequestOutcomeUncertain { request_id })),
                        );
                    }
                    None => {}
                }
                ServerMessage::Response(response)
            }
            message => message,
        };
        match crate::realtime::bounded::with_limit(self.config.collection_entries_limit(), || {
            decode::message(message)
        }) {
            Ok(payload) => self.events.publish_at(self.batch_epoch, payload),
            Err(_) => self
                .events
                .gap_at(self.batch_epoch, ResyncReason::MalformedRecord),
        }
    }
}

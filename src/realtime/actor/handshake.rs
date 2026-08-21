// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Socket establishment and authorization state machine.

mod io;
mod user_sync;

use futures_util::{StreamExt, stream::SplitSink, stream::SplitStream};
use tokio::{net::TcpStream, time};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, tungstenite::Message};
use tokio_util::sync::CancellationToken;

use self::io::{cancelled, connect_and_open, send_heartbeat_aware, socket_result};
use super::writer::HEARTBEAT_PERIOD;
use crate::{
    auth::{TokenSnapshot, TokenStore},
    provider_control::{self, ResponseControl},
    rate_limit::RateGovernor,
    realtime::{
        ConnectionId, FrameCodec, RealtimeConfig, RealtimeError, RequestId, Response, ServerFrame,
        ServerMessage, SocketKind,
    },
};

pub(super) type Socket = WebSocketStream<MaybeTlsStream<TcpStream>>;
pub(super) type SocketWriter = SplitSink<Socket, Message>;
pub(super) type SocketReader = SplitStream<Socket>;

pub(super) struct AuthorizationFence<'a> {
    snapshot: &'a TokenSnapshot,
    store: &'a TokenStore,
}

impl<'a> AuthorizationFence<'a> {
    pub(super) const fn new(snapshot: &'a TokenSnapshot, store: &'a TokenStore) -> Self {
        Self { snapshot, store }
    }
}

pub(super) struct Established {
    pub(super) writer: SocketWriter,
    pub(super) reader: SocketReader,
    pub(super) heartbeat: time::Interval,
    pub(super) heartbeat_deadline: time::Instant,
    pub(super) bootstrap: Option<Response>,
    pub(super) staged: Vec<ServerMessage>,
    pub(super) next_request_id: u64,
}

#[derive(Clone, Copy)]
struct ResponseWait<'a> {
    connection_id: ConnectionId,
    codec: FrameCodec,
    request_id: RequestId,
    deadline: time::Instant,
    timeout_error: RealtimeError,
    cancellation: &'a CancellationToken,
    staged_limit: usize,
}

pub(super) async fn establish(
    connection_id: ConnectionId,
    kind: SocketKind,
    url: &str,
    authorization: AuthorizationFence<'_>,
    config: RealtimeConfig,
    cancellation: &CancellationToken,
    rate_limits: &RateGovernor,
) -> Result<Established, RealtimeError> {
    let codec = FrameCodec::new(
        config.frame_bytes_limit(),
        config.messages_per_frame_limit(),
    )?;
    let mut socket = connect_and_open(
        connection_id,
        url,
        codec,
        config.frame_bytes_limit(),
        config.request_deadline(),
        cancellation,
    )
    .await?;
    let mut heartbeat_deadline = next_heartbeat_deadline()?;
    let mut heartbeat = time::interval_at(heartbeat_deadline, HEARTBEAT_PERIOD);
    heartbeat.set_missed_tick_behavior(time::MissedTickBehavior::Skip);
    if !authorization.store.is_current(authorization.snapshot) {
        return Err(RealtimeError::Unauthenticated);
    }
    let authorize = codec.encode_authorize(RequestId::new(1), authorization.snapshot.expose())?;
    let authorization_deadline = deadline(config.request_deadline())?;
    let retry_after = rate_limits.try_admit_authenticated("authorize");
    if !retry_after.is_zero() {
        return Err(RealtimeError::LocalRateLimit {
            endpoint: "authorize",
            retry_after,
        });
    }
    send_heartbeat_aware(
        connection_id,
        &mut socket,
        Message::text(authorize),
        heartbeat_deadline,
        authorization_deadline,
        RealtimeError::AuthorizationTimeout,
        cancellation,
    )
    .await?;
    let mut staged = Vec::new();
    let staged_limit = if matches!(kind, SocketKind::User) {
        config.event_channel_capacity().saturating_sub(1)
    } else {
        config.event_channel_capacity()
    };
    let authorization = wait_for_response(
        &mut socket,
        &mut heartbeat,
        &mut heartbeat_deadline,
        ResponseWait {
            connection_id,
            codec,
            request_id: RequestId::new(1),
            deadline: authorization_deadline,
            timeout_error: RealtimeError::AuthorizationTimeout,
            cancellation,
            staged_limit,
        },
        &mut staged,
    )
    .await?;
    validate_authorization(&authorization, rate_limits)?;
    let (bootstrap, next_request_id) = if matches!(kind, SocketKind::User) {
        let sync_deadline = deadline(config.request_deadline())?;
        let (bootstrap, next_request_id) = user_sync::perform(
            &mut socket,
            &mut heartbeat,
            &mut heartbeat_deadline,
            rate_limits,
            &mut staged,
            ResponseWait {
                connection_id,
                codec,
                request_id: RequestId::new(2),
                deadline: sync_deadline,
                timeout_error: RealtimeError::UserSyncTimeout,
                cancellation,
                staged_limit,
            },
        )
        .await?;
        (Some(bootstrap), next_request_id)
    } else {
        (None, 2)
    };
    let (writer, reader) = socket.split();
    Ok(Established {
        writer,
        reader,
        heartbeat,
        heartbeat_deadline,
        bootstrap,
        staged,
        next_request_id,
    })
}

fn validate_authorization(
    authorization: &Response,
    rate_limits: &RateGovernor,
) -> Result<(), RealtimeError> {
    if authorization.status() == 429 {
        let retry_after = std::time::Duration::from_hours(1);
        rate_limits.apply_global_cooldown(retry_after);
        return Err(RealtimeError::ProviderRateLimit {
            request_id: authorization.request_id(),
            retry_after,
        });
    }
    if authorization.status() != 200 {
        return Err(RealtimeError::AuthorizationRejected {
            status: authorization.status(),
        });
    }
    match provider_control::inspect(authorization.data()) {
        Ok(ResponseControl::Payload) => Ok(()),
        Ok(ResponseControl::BusinessFailure { .. }) => {
            Err(RealtimeError::AuthorizationRejected { status: 200 })
        }
        Ok(ResponseControl::Penalty(penalty)) => {
            let (ticket, retry_after, captcha_required) = penalty.into_parts();
            drop(ticket);
            if captcha_required {
                rate_limits.apply_captcha_lockout("authorize", retry_after);
            } else {
                rate_limits.apply_endpoint_cooldown("authorize", retry_after);
            }
            Err(RealtimeError::AuthorizationPenalty {
                retry_after,
                captcha_required,
            })
        }
        Err(_) => Err(RealtimeError::Protocol),
    }
}

async fn wait_for_response(
    socket: &mut Socket,
    heartbeat: &mut time::Interval,
    heartbeat_deadline: &mut time::Instant,
    wait: ResponseWait<'_>,
    staged: &mut Vec<ServerMessage>,
) -> Result<Response, RealtimeError> {
    loop {
        tokio::select! {
            biased;
            () = wait.cancellation.cancelled() => return Err(cancelled(wait.connection_id)),
            () = time::sleep_until(wait.deadline) => {
                return Err(wait.timeout_error);
            }
            _ = heartbeat.tick() => {
                advance_heartbeat(heartbeat, heartbeat_deadline)?;
                let frame = wait.codec.encode_heartbeat()?;
                send_heartbeat_aware(
                    wait.connection_id,
                    socket,
                    Message::text(frame),
                    *heartbeat_deadline,
                    wait.deadline,
                    wait.timeout_error,
                    wait.cancellation,
                ).await?;
            }
            message = socket.next() => {
                let message = socket_result(message)?;
                match message {
                    Message::Text(text) => {
                        if let Some(response) = correlated_response(
                            wait.codec.decode(text.as_str())?,
                            wait.request_id,
                            wait.connection_id,
                            wait.staged_limit,
                            staged,
                        )? {
                            return Ok(response);
                        }
                    }
                    Message::Ping(payload) => {
                        send_heartbeat_aware(
                            wait.connection_id,
                            socket,
                            Message::Pong(payload),
                            *heartbeat_deadline,
                            wait.deadline,
                            wait.timeout_error,
                            wait.cancellation,
                        ).await?;
                    }
                    Message::Pong(_) => {}
                    Message::Close(_) => return Err(RealtimeError::ServerClosed),
                    Message::Binary(_) | Message::Frame(_) => {
                        return Err(RealtimeError::Protocol);
                    }
                }
            }
        }
    }
}

fn correlated_response(
    frame: ServerFrame,
    request_id: RequestId,
    connection_id: ConnectionId,
    staged_limit: usize,
    staged: &mut Vec<ServerMessage>,
) -> Result<Option<Response>, RealtimeError> {
    match frame {
        ServerFrame::Heartbeat => Ok(None),
        ServerFrame::Messages(messages) => {
            let mut correlated = None;
            for message in messages {
                match message {
                    ServerMessage::Response(response) => {
                        if correlated.is_some() || response.request_id() != request_id {
                            return Err(RealtimeError::Protocol);
                        }
                        correlated = Some(response);
                    }
                    message => {
                        if staged.len() >= staged_limit {
                            return Err(RealtimeError::ResyncRequired {
                                connection_id,
                                reason: crate::realtime::ResyncReason::EventBufferOverflow,
                            });
                        }
                        staged.push(message);
                    }
                }
            }
            Ok(correlated)
        }
        ServerFrame::Close { .. } => Err(RealtimeError::ServerClosed),
        ServerFrame::Open => Err(RealtimeError::Protocol),
    }
}

fn next_heartbeat_deadline() -> Result<time::Instant, RealtimeError> {
    time::Instant::now()
        .checked_add(HEARTBEAT_PERIOD)
        .ok_or(RealtimeError::InvalidConfiguration {
            field: "heartbeat_period",
            reason: "is too large for a monotonic deadline",
        })
}

fn advance_heartbeat(
    heartbeat: &mut time::Interval,
    heartbeat_deadline: &mut time::Instant,
) -> Result<(), RealtimeError> {
    *heartbeat_deadline = heartbeat_deadline.checked_add(HEARTBEAT_PERIOD).ok_or(
        RealtimeError::InvalidConfiguration {
            field: "heartbeat_period",
            reason: "is too large for a monotonic deadline",
        },
    )?;
    heartbeat.reset_at(*heartbeat_deadline);
    Ok(())
}

fn deadline(timeout: std::time::Duration) -> Result<time::Instant, RealtimeError> {
    time::Instant::now()
        .checked_add(timeout)
        .ok_or(RealtimeError::InvalidConfiguration {
            field: "request_timeout",
            reason: "is too large for a monotonic deadline",
        })
}

// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Bounded setup-phase socket I/O.

use futures_util::{SinkExt, StreamExt};
use tokio::time;
use tokio_tungstenite::{
    connect_async_with_config,
    tungstenite::{Message, protocol::WebSocketConfig},
};
use tokio_util::sync::CancellationToken;

use super::{Socket, deadline};
use crate::realtime::{
    ConnectionId, DisconnectReason, FrameCodec, RealtimeError, ResyncReason, ServerFrame,
};

pub(super) async fn connect_and_open(
    connection_id: ConnectionId,
    url: &str,
    codec: FrameCodec,
    frame_bytes_limit: usize,
    timeout: std::time::Duration,
    cancellation: &CancellationToken,
) -> Result<Socket, RealtimeError> {
    let socket_config = websocket_config(frame_bytes_limit);
    let connect = connect_async_with_config(url, Some(socket_config), false);
    let (mut socket, _) = tokio::select! {
        () = cancellation.cancelled() => return Err(cancelled(connection_id)),
        result = time::timeout(timeout, connect) => {
            result.map_err(|_| RealtimeError::ConnectTimeout)?
                .map_err(|_| RealtimeError::Transport)?
        }
    };
    wait_for_open(connection_id, &mut socket, codec, timeout, cancellation).await?;
    Ok(socket)
}

async fn wait_for_open(
    connection_id: ConnectionId,
    socket: &mut Socket,
    codec: FrameCodec,
    timeout: std::time::Duration,
    cancellation: &CancellationToken,
) -> Result<(), RealtimeError> {
    let open_deadline = deadline(timeout)?;
    loop {
        let message = receive_before(
            connection_id,
            socket,
            open_deadline,
            RealtimeError::OpenTimeout,
            cancellation,
        )
        .await?;
        match message {
            Message::Text(text) => match codec.decode(text.as_str())? {
                ServerFrame::Open => return Ok(()),
                ServerFrame::Heartbeat => {}
                ServerFrame::Messages(_) | ServerFrame::Close { .. } => {
                    return Err(RealtimeError::Protocol);
                }
            },
            Message::Ping(payload) => {
                send_before(
                    connection_id,
                    socket,
                    Message::Pong(payload),
                    open_deadline,
                    RealtimeError::OpenTimeout,
                    cancellation,
                )
                .await?;
            }
            Message::Pong(_) => {}
            Message::Close(_) => return Err(RealtimeError::ServerClosed),
            Message::Binary(_) | Message::Frame(_) => return Err(RealtimeError::Protocol),
        }
    }
}

async fn receive_before(
    connection_id: ConnectionId,
    socket: &mut Socket,
    deadline: time::Instant,
    timeout_error: RealtimeError,
    cancellation: &CancellationToken,
) -> Result<Message, RealtimeError> {
    tokio::select! {
        () = cancellation.cancelled() => Err(cancelled(connection_id)),
        () = time::sleep_until(deadline) => Err(timeout_error),
        message = socket.next() => socket_result(message),
    }
}

async fn send_before(
    connection_id: ConnectionId,
    socket: &mut Socket,
    message: Message,
    deadline: time::Instant,
    timeout_error: RealtimeError,
    cancellation: &CancellationToken,
) -> Result<(), RealtimeError> {
    tokio::select! {
        () = cancellation.cancelled() => Err(cancelled(connection_id)),
        () = time::sleep_until(deadline) => Err(timeout_error),
        result = socket.send(message) => result.map_err(|_| RealtimeError::Transport),
    }
}

pub(super) async fn send_heartbeat_aware(
    connection_id: ConnectionId,
    socket: &mut Socket,
    message: Message,
    heartbeat_deadline: time::Instant,
    operation_deadline: time::Instant,
    timeout_error: RealtimeError,
    cancellation: &CancellationToken,
) -> Result<(), RealtimeError> {
    tokio::select! {
        biased;
        () = cancellation.cancelled() => Err(cancelled(connection_id)),
        () = time::sleep_until(heartbeat_deadline) => {
            Err(RealtimeError::ResyncRequired {
                connection_id,
                reason: ResyncReason::HeartbeatDeadlineMissed,
            })
        }
        () = time::sleep_until(operation_deadline) => Err(timeout_error),
        result = socket.send(message) => result.map_err(|_| RealtimeError::Transport),
    }
}

pub(super) fn socket_result(
    message: Option<Result<Message, tokio_tungstenite::tungstenite::Error>>,
) -> Result<Message, RealtimeError> {
    message
        .ok_or(RealtimeError::ServerClosed)?
        .map_err(|_| RealtimeError::Transport)
}

fn websocket_config(max_frame_bytes: usize) -> WebSocketConfig {
    let read_buffer_size = max_frame_bytes.min(128 * 1024);
    let write_buffer_size = max_frame_bytes.min(64 * 1024);
    let max_write_buffer_size = write_buffer_size.saturating_add(max_frame_bytes);
    WebSocketConfig::default()
        .read_buffer_size(read_buffer_size)
        .write_buffer_size(write_buffer_size)
        .max_write_buffer_size(max_write_buffer_size)
        .max_message_size(Some(max_frame_bytes))
        .max_frame_size(Some(max_frame_bytes))
}

pub(super) const fn cancelled(connection_id: ConnectionId) -> RealtimeError {
    RealtimeError::Disconnected {
        connection_id,
        reason: DisconnectReason::Shutdown,
    }
}

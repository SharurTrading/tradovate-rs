// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Heartbeat-aware access to the generation's sole WebSocket writer.

use std::{future::Future, time::Duration};

use futures_util::SinkExt;
use tokio::time::{self, Instant};
use tokio_tungstenite::tungstenite::{Error as SocketError, Message};
use tokio_util::sync::CancellationToken;

use super::handshake::SocketWriter;
use crate::realtime::{ConnectionId, DisconnectReason, RealtimeError, ResyncReason};

pub(super) const HEARTBEAT_PERIOD: Duration = Duration::from_millis(2_500);

pub(super) struct SendControl<'a> {
    connection_id: ConnectionId,
    cancellation: &'a CancellationToken,
    request_abandoned: &'a CancellationToken,
    heartbeat_deadline: Instant,
    operation_deadline: Instant,
}

impl<'a> SendControl<'a> {
    pub(super) const fn new(
        connection_id: ConnectionId,
        cancellation: &'a CancellationToken,
        request_abandoned: &'a CancellationToken,
        heartbeat_deadline: Instant,
        operation_deadline: Instant,
    ) -> Self {
        Self {
            connection_id,
            cancellation,
            request_abandoned,
            heartbeat_deadline,
            operation_deadline,
        }
    }
}

pub(super) async fn send(
    writer: &mut SocketWriter,
    message: Message,
    control: SendControl<'_>,
) -> Result<(), RealtimeError> {
    wait_for_write(writer.send(message), control).await
}

async fn wait_for_write<F>(write: F, control: SendControl<'_>) -> Result<(), RealtimeError>
where
    F: Future<Output = Result<(), SocketError>>,
{
    tokio::select! {
        biased;
        () = control.request_abandoned.cancelled() => Err(RealtimeError::ResyncRequired {
            connection_id: control.connection_id,
            reason: ResyncReason::RequestAbandoned,
        }),
        () = control.cancellation.cancelled() => Err(RealtimeError::Disconnected {
            connection_id: control.connection_id,
            reason: DisconnectReason::Shutdown,
        }),
        () = time::sleep_until(control.heartbeat_deadline) => {
            Err(RealtimeError::ResyncRequired {
                connection_id: control.connection_id,
                reason: ResyncReason::HeartbeatDeadlineMissed,
            })
        },
        () = time::sleep_until(control.operation_deadline) => Err(RealtimeError::Transport),
        result = write => result.map_err(|_| RealtimeError::Transport),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn blocked_write_terminates_at_the_heartbeat_deadline() {
        let cancellation = CancellationToken::new();
        let request_abandoned = CancellationToken::new();
        let now = Instant::now();
        let control = SendControl::new(
            ConnectionId::new(7),
            &cancellation,
            &request_abandoned,
            now + std::time::Duration::from_millis(10),
            now + std::time::Duration::from_secs(1),
        );
        let write = std::future::pending::<Result<(), SocketError>>();

        assert!(matches!(
            wait_for_write(write, control).await,
            Err(RealtimeError::ResyncRequired {
                reason: ResyncReason::HeartbeatDeadlineMissed,
                ..
            })
        ));
    }

    #[tokio::test]
    async fn abandoned_request_preempts_a_blocked_write() {
        let cancellation = CancellationToken::new();
        let request_abandoned = CancellationToken::new();
        request_abandoned.cancel();
        let now = Instant::now();
        let control = SendControl::new(
            ConnectionId::new(8),
            &cancellation,
            &request_abandoned,
            now + std::time::Duration::from_secs(1),
            now + std::time::Duration::from_secs(1),
        );
        let write = std::future::pending::<Result<(), SocketError>>();

        assert!(matches!(
            wait_for_write(write, control).await,
            Err(RealtimeError::ResyncRequired {
                reason: ResyncReason::RequestAbandoned,
                ..
            })
        ));
    }
}

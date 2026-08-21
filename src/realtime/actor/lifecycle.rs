// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Terminal lifecycle-state publication for setup and active generations.

use tokio::sync::watch;

use crate::realtime::{ConnectionId, DisconnectReason, RealtimeError, RealtimeState, ResyncReason};

pub(super) fn publish_setup_failure(
    state: &watch::Sender<RealtimeState>,
    connection_id: ConnectionId,
    error: RealtimeError,
) {
    replace(
        state,
        RealtimeState::Closed {
            connection_id,
            reason: error.disconnect_reason(),
        },
    );
}

pub(super) fn publish_shutdown(state: &watch::Sender<RealtimeState>, connection_id: ConnectionId) {
    replace(
        state,
        RealtimeState::Closed {
            connection_id,
            reason: DisconnectReason::Shutdown,
        },
    );
}

pub(super) fn normalize_shutdown(result: Result<(), RealtimeError>) -> Result<(), RealtimeError> {
    match result {
        Err(RealtimeError::Disconnected {
            reason: DisconnectReason::Shutdown,
            ..
        }) => Ok(()),
        result => result,
    }
}

pub(super) fn publish_active_terminal(
    state: &watch::Sender<RealtimeState>,
    connection_id: ConnectionId,
    result: Result<(), RealtimeError>,
) {
    let terminal = match result {
        Ok(()) => RealtimeState::Closed {
            connection_id,
            reason: DisconnectReason::Shutdown,
        },
        Err(RealtimeError::ResyncRequired { reason, .. }) => RealtimeState::ResyncRequired {
            connection_id,
            reason,
        },
        Err(_) => RealtimeState::ResyncRequired {
            connection_id,
            reason: ResyncReason::ConnectionLost,
        },
    };
    replace(state, terminal);
}

fn replace(state: &watch::Sender<RealtimeState>, terminal: RealtimeState) {
    let _previous_state = state.send_replace(terminal);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn graceful_cancellation_of_a_blocked_writer_remains_graceful() {
        let error = RealtimeError::Disconnected {
            connection_id: ConnectionId::new(9),
            reason: DisconnectReason::Shutdown,
        };

        assert_eq!(normalize_shutdown(Err(error)), Ok(()));
    }

    #[test]
    fn request_abandonment_is_not_normalized_as_shutdown() {
        let error = RealtimeError::ResyncRequired {
            connection_id: ConnectionId::new(9),
            reason: ResyncReason::RequestAbandoned,
        };

        assert_eq!(normalize_shutdown(Err(error)), Err(error));
    }
}

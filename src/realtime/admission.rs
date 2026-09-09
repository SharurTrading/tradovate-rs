// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Linearized generation admission and queued-versus-started request ownership.

use super::{ConnectionId, RealtimeError, RequestId};
use parking_lot::Mutex;
use std::sync::{
    Arc,
    atomic::{AtomicU8, Ordering},
};

pub(super) struct Admission {
    state: Mutex<Option<u64>>,
}

impl Admission {
    pub(super) fn new() -> Self {
        Self {
            state: Mutex::new(None),
        }
    }

    pub(super) fn activate(&self, next: u64) {
        *self.state.lock() = Some(next);
    }

    pub(super) fn end(&self) {
        *self.state.lock() = None;
    }

    pub(super) fn enqueue(
        &self,
        connection_id: ConnectionId,
        send: impl FnOnce(RequestId),
    ) -> Result<RequestId, RealtimeError> {
        let mut state = self.state.lock();
        let next = state
            .as_mut()
            .ok_or(RealtimeError::StaleGeneration { connection_id })?;
        let id = RequestId::new(*next);
        *next = next
            .checked_add(1)
            .ok_or(RealtimeError::RequestIdExhausted)?;
        // No await: closing this gate and enqueueing are linearized by one lock.
        send(id);
        Ok(id)
    }
}

const QUEUED: u8 = 0;
const STARTED: u8 = 1;
const ABANDONED: u8 = 2;

#[derive(Clone)]
pub(super) struct Invocation(Arc<AtomicU8>);

impl Invocation {
    pub(super) fn new() -> Self {
        Self(Arc::new(AtomicU8::new(QUEUED)))
    }

    pub(super) fn start(&self) -> bool {
        self.0
            .compare_exchange(QUEUED, STARTED, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
    }

    /// Returns true only when abandonment atomically prevents transmission.
    pub(super) fn abandon(&self) -> bool {
        match self
            .0
            .compare_exchange(QUEUED, ABANDONED, Ordering::AcqRel, Ordering::Acquire)
        {
            Ok(_) | Err(ABANDONED) => true,
            Err(_) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn queued_abandonment_prevents_start_but_started_outcomes_stay_unknown() {
        let queued = Invocation::new();
        assert!(queued.abandon());
        assert!(!queued.start());
        let started = Invocation::new();
        assert!(started.start());
        assert!(!started.abandon());
        assert!(!started.start());
    }

    #[test]
    fn ended_admission_never_enqueues_or_reuses_an_identity() {
        let gate = Admission::new();
        gate.activate(2);
        assert_eq!(
            gate.enqueue(ConnectionId::new(1), |_| {}),
            Ok(RequestId::new(2))
        );
        assert_eq!(
            gate.enqueue(ConnectionId::new(1), |_| {}),
            Ok(RequestId::new(3))
        );
        gate.end();
        assert!(matches!(
            gate.enqueue(ConnectionId::new(1), |_| panic!("stale enqueue")),
            Err(RealtimeError::StaleGeneration { .. })
        ));
    }
}

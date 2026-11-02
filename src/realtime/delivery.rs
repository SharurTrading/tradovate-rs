// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! One generation's ordered venue events, loss notifications and terminal evidence.
//!
//! Valid records remain deliverable after a loss. The first pending shutdown
//! notice has its own budget; refusing another notice retains an ordered gap.

use std::collections::VecDeque;

use parking_lot::Mutex;
use tokio::sync::Notify;

use super::{ConnectionId, RealtimeError, RealtimeEvent, RealtimeEventPayload, ResyncReason};

/// A source-ordered notification of lost continuity on one socket.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ContinuityGap {
    connection_id: ConnectionId,
    reason: ResyncReason,
}

impl ContinuityGap {
    /// Returns the socket whose data continuity was lost.
    #[must_use]
    pub const fn connection_id(self) -> ConnectionId {
        self.connection_id
    }

    /// Returns the cause of this loss notification.
    #[must_use]
    pub const fn reason(self) -> ResyncReason {
        self.reason
    }
}

struct Retained {
    accepted_at: Option<std::time::Instant>,
    event: RealtimeEvent,
}

struct Queue {
    events: VecDeque<Retained>,
    shutdown_pending: bool,
    ended: Option<Result<(), RealtimeError>>,
    end_delivered: bool,
}

pub(super) struct Delivery {
    connection_id: ConnectionId,
    queue: Mutex<Queue>,
    changed: Notify,
}

impl Delivery {
    pub(super) fn new(connection_id: ConnectionId) -> Self {
        Self {
            connection_id,
            queue: Mutex::new(Queue {
                events: VecDeque::new(),
                shutdown_pending: false,
                ended: None,
                end_delivered: false,
            }),
            changed: Notify::new(),
        }
    }

    pub(super) fn publish(&self, payload: RealtimeEventPayload) {
        let mut queue = self.queue.lock();
        if queue.ended.is_some() {
            return;
        }
        let shutdown = matches!(payload, RealtimeEventPayload::Shutdown(_));
        if shutdown && queue.shutdown_pending {
            self.install_gap(&mut queue, ResyncReason::EventBufferOverflow);
        } else {
            let unsupported = payload.requires_resync();
            queue.events.push_back(Retained {
                accepted_at: (!shutdown).then(std::time::Instant::now),
                event: RealtimeEvent::new(self.connection_id, payload),
            });
            queue.shutdown_pending |= shutdown;
            if unsupported {
                self.install_gap(&mut queue, ResyncReason::UnsupportedEvent);
            }
        }
        self.changed.notify_one();
    }

    pub(super) fn gap(&self, reason: ResyncReason) {
        let mut queue = self.queue.lock();
        if queue.ended.is_none() {
            self.install_gap(&mut queue, reason);
            self.changed.notify_one();
        }
    }

    fn install_gap(&self, queue: &mut Queue, reason: ResyncReason) {
        queue.events.push_back(Retained {
            accepted_at: None,
            event: RealtimeEvent::new(
                self.connection_id,
                RealtimeEventPayload::ContinuityGap(ContinuityGap {
                    connection_id: self.connection_id,
                    reason,
                }),
            ),
        });
    }

    /// Observes retained venue-event lag without consuming events. Lifecycle
    /// evidence (notices, markers, generation end) is not backlog: depth counts
    /// accepted venue events awaiting delivery, and the oldest-event age is how
    /// long the oldest such event has waited.
    pub(super) fn backlog(&self) -> super::EventBacklog {
        let queue = self.queue.lock();
        let oldest = queue
            .events
            .iter()
            .find_map(|retained| retained.accepted_at);
        super::EventBacklog::new(
            queue
                .events
                .iter()
                .filter(|event| event.accepted_at.is_some())
                .count(),
            oldest.map(|accepted_at| accepted_at.elapsed()),
        )
    }

    pub(super) fn queued_event_count(&self) -> usize {
        let queue = self.queue.lock();
        queue.events.len() + usize::from(queue.ended.is_some() && !queue.end_delivered)
    }

    /// Called only after both socket halves and their pending producers are dropped.
    pub(super) fn end(&self, result: Result<(), RealtimeError>) {
        self.end_if_absent(result);
    }

    pub(super) fn end_if_absent(&self, result: Result<(), RealtimeError>) -> bool {
        let mut queue = self.queue.lock();
        if queue.ended.is_some() {
            return false;
        }
        queue.ended = Some(result);
        self.changed.notify_one();
        true
    }

    pub(super) fn result(&self) -> Result<(), RealtimeError> {
        self.queue
            .lock()
            .ended
            .unwrap_or(Err(RealtimeError::ActorTaskFailed))
    }

    pub(super) async fn recv(&self) -> Option<RealtimeEvent> {
        loop {
            let changed = self.changed.notified();
            {
                let mut queue = self.queue.lock();
                if let Some(retained) = queue.events.pop_front() {
                    if matches!(retained.event.payload(), RealtimeEventPayload::Shutdown(_)) {
                        queue.shutdown_pending = false;
                    }
                    return Some(retained.event);
                }
                if let Some(result) = queue.ended {
                    if queue.end_delivered {
                        return None;
                    }
                    queue.end_delivered = true;
                    return Some(RealtimeEvent::new(
                        self.connection_id,
                        RealtimeEventPayload::GenerationEnded(result),
                    ));
                }
            }
            changed.await;
        }
    }
}

#[cfg(test)]
#[path = "delivery/tests.rs"]
mod tests;

// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! One generation's accepted venue-event retention and lifecycle evidence.
//!
//! Accepted venue events are retained in source order without an event-count
//! cutoff: a slow consumer observes lag, never a locally fabricated gap. Only
//! genuine loss (malformed or unrepresentable records) fences publication with
//! a continuity marker after the accepted prefix. Retained-event memory is
//! bounded by the provider's own sending rate while the consumer stays parked;
//! outbound work remains separately bounded (`TV-PAT-BOUNDS-01`).

use std::collections::VecDeque;

use parking_lot::Mutex;
use tokio::sync::Notify;

use super::{ConnectionId, RealtimeError, RealtimeEvent, RealtimeEventPayload, ResyncReason};

/// An opaque recovery marker for exactly one discontinuity on one socket.
///
/// If this generation exhausts its marker sequence, acknowledgement permanently
/// fails closed. The caller then needs a replacement connection for recovery.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ContinuityGap {
    connection_id: ConnectionId,
    sequence: u64,
    reason: ResyncReason,
}

impl ContinuityGap {
    /// Returns the socket whose data continuity was lost.
    #[must_use]
    pub const fn connection_id(self) -> ConnectionId {
        self.connection_id
    }

    /// Returns the first cause since the last acknowledged recovery boundary.
    #[must_use]
    pub const fn reason(self) -> ResyncReason {
        self.reason
    }
}

struct Retained {
    accepted_at: std::time::Instant,
    event: RealtimeEvent,
}

/// A graceful-shutdown notice retained at its ordered position.
struct Notice {
    before: usize,
    clean: bool,
    event: RealtimeEvent,
}

struct Queue {
    events: VecDeque<Retained>,
    notices: VecDeque<Notice>,
    gap: Option<(ContinuityGap, bool)>,
    next_gap: u64,
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
                notices: VecDeque::new(),
                gap: None,
                next_gap: 1,
                ended: None,
                end_delivered: false,
            }),
            changed: Notify::new(),
        }
    }

    pub(super) fn epoch(&self) -> Option<u64> {
        let queue = self.queue.lock();
        (queue.gap.is_none() && queue.ended.is_none()).then_some(queue.next_gap)
    }

    pub(super) fn publish(&self, payload: RealtimeEventPayload) {
        self.publish_at(self.epoch(), payload);
    }

    pub(super) fn publish_at(&self, epoch: Option<u64>, payload: RealtimeEventPayload) {
        let mut queue = self.queue.lock();
        if queue.ended.is_some() {
            return;
        }
        if matches!(payload, RealtimeEventPayload::Shutdown(_)) {
            // Every notice is accepted provider evidence. Each is retained once
            // at its ordered position; none displaces retained venue data.
            let notice = Notice {
                before: queue.events.len(),
                clean: queue.gap.is_none(),
                event: RealtimeEvent::new(self.connection_id, payload),
            };
            queue.notices.push_back(notice);
            self.changed.notify_one();
            return;
        }
        if epoch != Some(queue.next_gap) || queue.gap.is_some() || queue.ended.is_some() {
            return;
        }
        let unsupported = payload.requires_resync();
        queue.events.push_back(Retained {
            accepted_at: std::time::Instant::now(),
            event: RealtimeEvent::new(self.connection_id, payload),
        });
        if unsupported {
            self.install_gap(&mut queue, ResyncReason::UnsupportedEvent);
        }
        self.changed.notify_one();
    }

    pub(super) fn gap(&self, reason: ResyncReason) {
        self.gap_at(self.epoch(), reason);
    }

    pub(super) fn gap_at(&self, epoch: Option<u64>, reason: ResyncReason) {
        let mut queue = self.queue.lock();
        if epoch != Some(queue.next_gap) {
            return;
        }
        self.install_gap(&mut queue, reason);
        self.changed.notify_one();
    }

    fn install_gap(&self, queue: &mut Queue, reason: ResyncReason) {
        if queue.gap.is_none() && queue.ended.is_none() {
            queue.gap = Some((
                ContinuityGap {
                    connection_id: self.connection_id,
                    sequence: queue.next_gap,
                    reason,
                },
                false,
            ));
            // Exhaustion leaves the final marker unacknowledgeable, never reuses it.
            queue.next_gap = queue.next_gap.saturating_add(1);
        }
    }

    pub(super) fn acknowledge(&self, marker: ContinuityGap) -> bool {
        let mut queue = self.queue.lock();
        if queue.ended.is_none() && queue.next_gap != u64::MAX && queue.gap == Some((marker, true))
        {
            queue.gap = None;
            return true;
        }
        false
    }

    /// Observes retained venue-event lag without consuming events. Lifecycle
    /// evidence (notices, markers, generation end) is not backlog: depth counts
    /// accepted venue events awaiting delivery, and the oldest-event age is how
    /// long the oldest such event has waited.
    pub(super) fn backlog(&self) -> super::EventBacklog {
        let queue = self.queue.lock();
        let oldest = queue.events.front().map(|retained| retained.accepted_at);
        super::EventBacklog::new(
            queue.events.len(),
            oldest.map(|accepted_at| accepted_at.elapsed()),
        )
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
                if queue
                    .notices
                    .front()
                    .is_some_and(|notice| notice.before == 0 && notice.clean)
                {
                    return queue.notices.pop_front().map(|notice| notice.event);
                }
                if let Some(retained) = queue.events.pop_front() {
                    // Every pending notice counts this event ahead of it, not
                    // only the front one; a front-only decrement leaves deeper
                    // notices with stale counts and lets later events overtake
                    // them.
                    for notice in &mut queue.notices {
                        notice.before = notice.before.saturating_sub(1);
                    }
                    return Some(retained.event);
                }
                if let Some((gap, delivered)) = &mut queue.gap
                    && !*delivered
                {
                    *delivered = true;
                    let gap = *gap;
                    // Notices published while this gap was pending were held
                    // behind the marker; with the marker delivered their
                    // hold is resolved, so they resume ordered delivery.
                    for notice in &mut queue.notices {
                        notice.clean = true;
                    }
                    return Some(RealtimeEvent::new(
                        self.connection_id,
                        RealtimeEventPayload::ContinuityGap(gap),
                    ));
                }
                if let Some(notice) = queue.notices.pop_front() {
                    return Some(notice.event);
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

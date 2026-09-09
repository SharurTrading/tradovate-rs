// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! One generation's accepted event prefix and independently retained boundaries.

use std::collections::VecDeque;

use parking_lot::Mutex;
use tokio::sync::Notify;

use super::{ConnectionId, RealtimeError, RealtimeEvent, RealtimeEventPayload, ResyncReason};

/// An opaque recovery marker for exactly one discontinuity on one socket.
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

struct Queue {
    events: VecDeque<RealtimeEvent>,
    gap: Option<(ContinuityGap, bool)>,
    next_gap: u64,
    shutdown: Option<(usize, bool, RealtimeEvent)>,
    ended: Option<Result<(), RealtimeError>>,
    end_delivered: bool,
}

pub(super) struct Delivery {
    connection_id: ConnectionId,
    capacity: usize,
    queue: Mutex<Queue>,
    changed: Notify,
}

impl Delivery {
    pub(super) fn new(connection_id: ConnectionId, capacity: usize) -> Self {
        Self {
            connection_id,
            capacity,
            queue: Mutex::new(Queue {
                events: VecDeque::new(),
                gap: None,
                next_gap: 1,
                shutdown: None,
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
            if queue.shutdown.is_none() {
                queue.shutdown = Some((
                    queue.events.len(),
                    queue.gap.is_none(),
                    RealtimeEvent::new(self.connection_id, payload),
                ));
            } else {
                // Retain the first close notice. Additional notices cannot grow
                // an unbounded lifecycle queue; signal lost semantics explicitly.
                self.install_gap(&mut queue, ResyncReason::EventBufferOverflow);
            }
            self.changed.notify_one();
            return;
        }
        if epoch != Some(queue.next_gap) || queue.gap.is_some() || queue.ended.is_some() {
            return;
        }
        if queue.events.len() == self.capacity {
            self.install_gap(&mut queue, ResyncReason::EventBufferOverflow);
        } else {
            let unsupported = payload.requires_resync();
            queue
                .events
                .push_back(RealtimeEvent::new(self.connection_id, payload));
            if unsupported {
                self.install_gap(&mut queue, ResyncReason::UnsupportedEvent);
            }
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
                    .shutdown
                    .as_ref()
                    .is_some_and(|(before, before_gap, _)| *before == 0 && *before_gap)
                {
                    return queue.shutdown.take().map(|(_, _, event)| event);
                }
                if let Some(event) = queue.events.pop_front() {
                    if let Some((before, _, _)) = &mut queue.shutdown {
                        *before = before.saturating_sub(1);
                    }
                    return Some(event);
                }
                if let Some((gap, delivered)) = &mut queue.gap
                    && !*delivered
                {
                    *delivered = true;
                    return Some(RealtimeEvent::new(
                        self.connection_id,
                        RealtimeEventPayload::ContinuityGap(*gap),
                    ));
                }
                if let Some((_, _, event)) = queue.shutdown.take() {
                    return Some(event);
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
mod tests {
    use super::*;

    #[tokio::test]
    async fn acknowledgement_rejects_stale_producers_and_markers() {
        let delivery = Delivery::new(ConnectionId::new(1), 1);
        let before_gap = delivery.epoch();
        delivery.gap(ResyncReason::MalformedRecord);
        let Some(event) = delivery.recv().await else {
            panic!("missing gap");
        };
        let RealtimeEventPayload::ContinuityGap(first) = event.into_payload() else {
            panic!("wrong event");
        };
        assert!(delivery.acknowledge(first));
        // Work captured before recovery cannot reenter or reset the new boundary.
        delivery.gap_at(before_gap, ResyncReason::MalformedRecord);
        assert!(delivery.epoch().is_some());
        delivery.publish_at(
            before_gap,
            RealtimeEventPayload::UnmatchedResponse(super::super::event::UnmatchedResponse::new(
                super::super::RequestId::new(3),
                200,
            )),
        );
        assert!(delivery.queue.lock().events.is_empty());
        delivery.gap(ResyncReason::EventBufferOverflow);
        assert!(!delivery.acknowledge(first));
        let second = delivery.queue.lock().gap.map(|(gap, _)| gap);
        assert!(
            !delivery.acknowledge(second.unwrap_or(first)),
            "undelivered marker"
        );
        assert!(delivery.recv().await.is_some());
        assert!(
            !delivery.acknowledge(first),
            "delayed first acknowledgement"
        );
    }
}

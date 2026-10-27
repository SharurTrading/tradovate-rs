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
                    if let Some(notice) = queue.notices.front_mut() {
                        notice.before = notice.before.saturating_sub(1);
                    }
                    return Some(retained.event);
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
mod tests {
    use super::*;

    fn shutdown_fixture() -> RealtimeEventPayload {
        RealtimeEventPayload::Shutdown(super::super::ShutdownEvent {
            reason_code: super::super::ShutdownReason::Maintenance,
            explanation: None,
        })
    }

    fn accepted_fixture() -> RealtimeEventPayload {
        RealtimeEventPayload::UnmatchedResponse(super::super::event::UnmatchedResponse::new(
            super::super::RequestId::new(3),
            200,
        ))
    }

    #[tokio::test]
    async fn retained_events_have_no_count_cutoff_and_report_backlog() {
        const COUNT: usize = 1_000;
        let delivery = Delivery::new(ConnectionId::new(1));
        for _ in 0..COUNT {
            delivery.publish(accepted_fixture());
        }
        let backlog = delivery.backlog();
        assert_eq!(backlog.depth(), COUNT);
        assert!(backlog.oldest_event_age().is_some());
        for _ in 0..COUNT {
            assert!(matches!(
                delivery.recv().await.map(RealtimeEvent::into_payload),
                Some(RealtimeEventPayload::UnmatchedResponse(_))
            ));
        }
        let drained = delivery.backlog();
        assert_eq!(drained.depth(), 0);
        assert_eq!(drained.oldest_event_age(), None);
        delivery.end(Ok(()));
        assert!(matches!(
            delivery.recv().await.map(RealtimeEvent::into_payload),
            Some(RealtimeEventPayload::GenerationEnded(Ok(())))
        ));
        assert!(delivery.recv().await.is_none());
    }

    #[tokio::test]
    async fn lifecycle_notices_keep_prefix_order_and_exactly_one_end() {
        let delivery = Delivery::new(ConnectionId::new(1));
        delivery.publish(accepted_fixture());
        delivery.publish(shutdown_fixture());
        // Data accepted after a notice stays behind it in source order.
        delivery.publish(accepted_fixture());
        delivery.publish(shutdown_fixture());
        delivery.gap(ResyncReason::MalformedRecord);
        delivery.end(Err(RealtimeError::ServerClosed));
        assert!(matches!(
            delivery.recv().await.map(RealtimeEvent::into_payload),
            Some(RealtimeEventPayload::UnmatchedResponse(_))
        ));
        assert!(matches!(
            delivery.recv().await.map(RealtimeEvent::into_payload),
            Some(RealtimeEventPayload::Shutdown(_))
        ));
        assert!(matches!(
            delivery.recv().await.map(RealtimeEvent::into_payload),
            Some(RealtimeEventPayload::UnmatchedResponse(_))
        ));
        assert!(matches!(
            delivery.recv().await.map(RealtimeEvent::into_payload),
            Some(RealtimeEventPayload::ContinuityGap(_))
        ));
        assert!(matches!(
            delivery.recv().await.map(RealtimeEvent::into_payload),
            Some(RealtimeEventPayload::Shutdown(_))
        ));
        assert!(matches!(
            delivery.recv().await.map(RealtimeEvent::into_payload),
            Some(RealtimeEventPayload::GenerationEnded(Err(
                RealtimeError::ServerClosed
            )))
        ));
        assert!(delivery.recv().await.is_none());
    }

    #[tokio::test]
    async fn acknowledgement_rejects_stale_producers_and_markers() {
        let delivery = Delivery::new(ConnectionId::new(1));
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
        delivery.gap(ResyncReason::ConnectionLost);
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

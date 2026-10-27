// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

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

#[derive(Debug)]
enum ExpectedItem {
    Event,
    Notice,
}

impl ExpectedItem {
    fn matches(&self, payload: &RealtimeEventPayload) -> bool {
        match self {
            Self::Event => matches!(payload, RealtimeEventPayload::UnmatchedResponse(_)),
            Self::Notice => matches!(payload, RealtimeEventPayload::Shutdown(_)),
        }
    }
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
async fn first_close_notice_is_retained_and_saturation_installs_explicit_loss() {
    let delivery = Delivery::new(ConnectionId::new(1));
    delivery.publish(accepted_fixture());
    delivery.publish(shutdown_fixture());
    // Data accepted after a notice stays behind it in source order.
    delivery.publish(accepted_fixture());
    // The retained-notice point holds one notice; the first additional one
    // installs explicit loss and the rest stay covered by that same marker.
    for _ in 0..4 {
        delivery.publish(shutdown_fixture());
    }
    // Saturation preserves the retained venue events untouched.
    assert_eq!(delivery.backlog().depth(), 2);
    for expected in [
        ExpectedItem::Event,
        ExpectedItem::Notice,
        ExpectedItem::Event,
    ] {
        let Some(payload) = delivery.recv().await.map(RealtimeEvent::into_payload) else {
            panic!("missing item before {expected:?}")
        };
        assert!(
            expected.matches(&payload),
            "got {payload:?} for {expected:?}"
        );
    }
    let Some(event) = delivery.recv().await else {
        panic!("missing saturation marker");
    };
    let RealtimeEventPayload::ContinuityGap(marker) = event.into_payload() else {
        panic!("saturation did not install explicit loss");
    };
    assert_eq!(marker.reason(), ResyncReason::EventBufferOverflow);
    assert!(delivery.acknowledge(marker), "delivered marker");
    delivery.end(Err(RealtimeError::ServerClosed));
    assert!(matches!(
        delivery.recv().await.map(RealtimeEvent::into_payload),
        Some(RealtimeEventPayload::GenerationEnded(Err(
            RealtimeError::ServerClosed
        )))
    ));
    assert!(delivery.recv().await.is_none());
}

#[tokio::test]
async fn data_published_after_saturation_never_crosses_the_marker() {
    let delivery = Delivery::new(ConnectionId::new(1));
    delivery.publish(accepted_fixture());
    delivery.publish(shutdown_fixture());
    delivery.publish(accepted_fixture());
    // The second notice saturates the retained-notice point and installs
    // explicit loss instead of unbounded queue growth.
    delivery.publish(shutdown_fixture());
    // Publication offered once the marker is pending is dropped: stale
    // buffered records must not cross the recovery boundary.
    delivery.publish(accepted_fixture());
    delivery.end(Ok(()));
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
    let Some(event) = delivery.recv().await else {
        panic!("missing saturation marker");
    };
    let RealtimeEventPayload::ContinuityGap(marker) = event.into_payload() else {
        panic!("expected the saturation marker");
    };
    assert_eq!(marker.reason(), ResyncReason::EventBufferOverflow);
    assert!(matches!(
        delivery.recv().await.map(RealtimeEvent::into_payload),
        Some(RealtimeEventPayload::GenerationEnded(Ok(())))
    ));
    assert!(delivery.recv().await.is_none());
}

#[tokio::test]
async fn publication_after_generation_end_is_dropped_but_terminal_evidence_is_retained() {
    let delivery = Delivery::new(ConnectionId::new(1));
    delivery.publish(accepted_fixture());
    delivery.end(Err(RealtimeError::ServerClosed));
    // A producer outrunning its own generation is rejected as stale work: the
    // offered items never deliver, and the accepted prefix plus the exact
    // terminal error survive.
    delivery.publish(accepted_fixture());
    delivery.publish(shutdown_fixture());
    assert!(matches!(
        delivery.recv().await.map(RealtimeEvent::into_payload),
        Some(RealtimeEventPayload::UnmatchedResponse(_))
    ));
    assert!(matches!(
        delivery.recv().await.map(RealtimeEvent::into_payload),
        Some(RealtimeEventPayload::GenerationEnded(Err(
            RealtimeError::ServerClosed
        )))
    ));
    assert!(delivery.recv().await.is_none());
    assert!(delivery.recv().await.is_none());
}

#[tokio::test]
async fn notice_published_during_pending_gap_follows_the_marker() {
    let delivery = Delivery::new(ConnectionId::new(1));
    delivery.publish(accepted_fixture());
    delivery.gap(ResyncReason::MalformedRecord);
    // Accepted while the gap is pending: held behind the marker, ahead of
    // the terminal result.
    delivery.publish(shutdown_fixture());
    delivery.end(Ok(()));
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
        Some(RealtimeEventPayload::GenerationEnded(Ok(())))
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
    let Some(event) = delivery.recv().await else {
        panic!("missing second marker");
    };
    let RealtimeEventPayload::ContinuityGap(second_delivered) = event.into_payload() else {
        panic!("stale work crossed the recovery boundary");
    };
    assert_eq!(second_delivered.reason(), ResyncReason::ConnectionLost);
    assert!(
        !delivery.acknowledge(first),
        "delayed first acknowledgement"
    );
    // The stale-published event and the refused stale marker request stay
    // fenced: draining the ended generation yields only the terminal evidence.
    delivery.end(Ok(()));
    assert!(matches!(
        delivery.recv().await.map(RealtimeEvent::into_payload),
        Some(RealtimeEventPayload::GenerationEnded(Ok(())))
    ));
    assert!(delivery.recv().await.is_none());
}

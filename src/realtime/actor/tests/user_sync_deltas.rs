// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Bootstrap-adjacent user delta staging and publication ordering.

use super::*;
use crate::realtime::UserEntityBatch;

#[tokio::test]
async fn co_batched_user_sync_delta_is_published_after_bootstrap() {
    let (listener, url) = bind().await;
    let server = tokio::spawn(async move {
        let mut socket = accept(listener).await;
        send_text(&mut socket, "o").await;
        let _authorization = next_text(&mut socket).await;
        send_text(
            &mut socket,
            r#"a[{"i":1,"s":200},{"e":"props","d":{"entityType":"order","eventType":"Updated","entity":{"id":10,"accountId":20,"contractId":30,"timestamp":"2026-08-21T00:00:00Z","action":"Buy","ordStatus":"Working","admin":false}}}]"#,
        )
        .await;
        let _sync = next_text(&mut socket).await;
        send_text(
            &mut socket,
            r#"a[{"i":2,"s":200,"d":{"users":[],"contractGroups":[]}},{"e":"props","d":{"entityType":"order","eventType":"Updated","entity":{"id":11,"accountId":20,"contractId":30,"timestamp":"2026-08-21T00:00:01Z","action":"Buy","ordStatus":"Working","admin":false}}}]"#,
        )
        .await;
        expect_close(&mut socket).await;
    });
    let client = authenticated_client(&url, "access", None);
    let mut connection = connect(&client, SocketKind::User, RealtimeConfig::default()).await;

    assert!(matches!(
        connection
            .recv_event()
            .await
            .map(RealtimeEvent::into_payload),
        Some(RealtimeEventPayload::User(UserStreamEvent::Bootstrap(_)))
    ));
    assert!(matches!(
        connection
            .recv_event()
            .await
            .map(RealtimeEvent::into_payload),
        Some(RealtimeEventPayload::User(UserStreamEvent::Properties(_)))
    ));
    assert!(matches!(
        connection
            .recv_event()
            .await
            .map(RealtimeEvent::into_payload),
        Some(RealtimeEventPayload::User(UserStreamEvent::Properties(_)))
    ));
    assert!(connection.shutdown().await.is_ok());
    join(server).await;
}

#[tokio::test]
async fn events_staged_during_user_sync_are_published_after_bootstrap() {
    let order_delta = |id: i64, timestamp: &str| {
        format!(
            r#"{{"e":"props","d":{{"entityType":"order","eventType":"Updated","entity":{{"id":{id},"accountId":20,"contractId":30,"timestamp":"{timestamp}","action":"Buy","ordStatus":"Working","admin":false}}}}}}"#
        )
    };
    let (listener, url) = bind().await;
    let server = tokio::spawn(async move {
        let mut socket = accept(listener).await;
        send_text(&mut socket, "o").await;
        let _authorization = next_text(&mut socket).await;
        send_text(&mut socket, r#"a[{"i":1,"s":200}]"#).await;
        let _sync = next_text(&mut socket).await;
        // Deltas that arrive before the synchronization response are staged,
        // whatever their count, and published behind the bootstrap.
        send_text(
            &mut socket,
            &format!("a[{}]", order_delta(10, "2026-08-21T00:00:00Z")),
        )
        .await;
        send_text(
            &mut socket,
            &format!(
                r#"a[{{"i":2,"s":200,"d":{{"users":[],"contractGroups":[]}}}},{}]"#,
                order_delta(11, "2026-08-21T00:00:01Z")
            ),
        )
        .await;
        expect_close(&mut socket).await;
    });
    let client = authenticated_client(&url, "access", None);
    let mut connection = connect(&client, SocketKind::User, RealtimeConfig::default()).await;

    assert!(matches!(
        connection
            .recv_event()
            .await
            .map(RealtimeEvent::into_payload),
        Some(RealtimeEventPayload::User(UserStreamEvent::Bootstrap(_)))
    ));
    for expected in [10_i64, 11] {
        let Some(event) = connection.recv_event().await else {
            panic!("staged delta {expected} must be retained");
        };
        assert_eq!(event.connection_id(), connection.connection_id());
        let RealtimeEventPayload::User(UserStreamEvent::Properties(events)) = event.into_payload()
        else {
            panic!("staged delta {expected} must stay a typed properties event");
        };
        assert_eq!(events.len(), 1);
        assert!(matches!(events[0].entities(),
        UserEntityBatch::Orders(orders)
            if orders.first().is_some_and(|order| order.id().is_some_and(
                |id| id.get() == expected
            ))));
    }
    assert!(connection.shutdown().await.is_ok());
    join(server).await;
}

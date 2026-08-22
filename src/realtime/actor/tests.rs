// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use std::time::Duration;

use futures_util::SinkExt;
use tokio::time;
use tokio_tungstenite::tungstenite::Message;

use super::*;
use crate::realtime::{
    DocumentationBlockedCapability, RealtimeEventPayload, RequestId, SocketKind, UserStreamEvent,
};

mod authentication;
mod cancellation;
mod market_data;
mod support;
mod user_sync;

use support::*;

#[tokio::test]
async fn authorizes_before_requests_and_correlates_mixed_frames() {
    let (listener, url) = bind().await;
    let server = tokio::spawn(async move {
        let mut socket = accept(listener).await;
        send_text(&mut socket, "o").await;
        assert_eq!(next_text(&mut socket).await, "authorize\n1\n\naccess");
        send_text(&mut socket, r#"a[{"i":1,"s":200}]"#).await;
        expect_user_sync(&mut socket).await;
        assert_eq!(next_text(&mut socket).await, "contract/item\n3\nid=42\n");
        send_text(
            &mut socket,
            r#"a[{"i":3,"s":200,"d":{"name":"ES"}},{"e":"md","d":{"quotes":[{"timestamp":"2026-08-21T00:00:00Z","contractId":42,"entries":{"Trade":{"price":5000.25,"size":2}}}]}}]"#,
        )
        .await;
        expect_close(&mut socket).await;
    });
    let client = authenticated_client(&url, "access", Some("market-data"));
    let mut connection = connect(&client, SocketKind::User, RealtimeConfig::default()).await;
    let connection_id = connection.connection_id();

    let response = connection
        .request_non_mutating("contract/item", "id=42", "")
        .await;
    let Ok(response) = response else {
        panic!("request must receive its correlated response");
    };
    assert_eq!(response.request_id(), RequestId::new(3));
    assert_eq!(response.status(), 200);

    assert!(matches!(
        connection
            .recv_event()
            .await
            .map(RealtimeEvent::into_payload),
        Some(RealtimeEventPayload::User(UserStreamEvent::Bootstrap(_)))
    ));
    let Some(event) = connection.recv_event().await else {
        panic!("mixed frame must publish its event");
    };
    assert_eq!(event.connection_id(), connection_id);
    assert!(matches!(
        event.payload(),
        RealtimeEventPayload::MarketData(_)
    ));
    assert!(connection.shutdown().await.is_ok());
    join(server).await;
}

#[tokio::test]
async fn authorization_stages_event_only_and_co_batched_frames_in_order() {
    let (listener, url) = bind().await;
    let server = tokio::spawn(async move {
        let mut socket = accept(listener).await;
        send_text(&mut socket, "o").await;
        let _authorization = next_text(&mut socket).await;
        send_text(
            &mut socket,
            r#"a[{"e":"md","d":{"quotes":[{"timestamp":"2026-08-21T00:00:00Z","contractId":42,"entries":{"Bid":{"price":5000.00,"size":3}}}]}}]"#,
        )
        .await;
        send_text(
            &mut socket,
            r#"a[{"i":1,"s":200},{"e":"chart","d":{"charts":[{"id":9,"eoh":true}]}}]"#,
        )
        .await;
        expect_close(&mut socket).await;
    });
    let client = authenticated_client(&url, "access", Some("market-data"));
    let mut connection = connect(&client, SocketKind::MarketData, RealtimeConfig::default()).await;

    assert!(matches!(
        connection
            .recv_event()
            .await
            .map(RealtimeEvent::into_payload),
        Some(RealtimeEventPayload::MarketData(_))
    ));
    assert!(matches!(
        connection
            .recv_event()
            .await
            .map(RealtimeEvent::into_payload),
        Some(RealtimeEventPayload::Chart(_))
    ));
    assert!(connection.shutdown().await.is_ok());
    join(server).await;
}

#[tokio::test]
async fn event_overflow_terminates_with_resync_required() {
    let (listener, url) = bind().await;
    let server = tokio::spawn(async move {
        let mut socket = accept(listener).await;
        authorize(&mut socket).await;
        send_text(
            &mut socket,
            r#"a[{"e":"md","d":{"quotes":[{"timestamp":"2026-08-21T00:00:00Z","contractId":42,"entries":{"Trade":{"price":5000.25,"size":2}}}]}},{"e":"chart","d":{"charts":[{"id":9,"eoh":true}]}}]"#,
        )
        .await;
    });
    let client = authenticated_client(&url, "access", None);
    let config = RealtimeConfig::default()
        .event_capacity(1)
        .request_timeout(Duration::from_secs(1));
    let mut connection = connect(&client, SocketKind::User, config).await;

    let state = await_terminal_state(&mut connection).await;
    assert!(matches!(
        state,
        RealtimeState::ResyncRequired {
            reason: ResyncReason::EventBufferOverflow,
            ..
        }
    ));
    assert!(connection.recv_event().await.is_some());
    assert!(connection.recv_event().await.is_none());
    assert!(matches!(
        connection.shutdown().await,
        Err(RealtimeError::ResyncRequired {
            reason: ResyncReason::EventBufferOverflow,
            ..
        })
    ));
    join(server).await;
}

#[tokio::test]
async fn admitted_request_timeout_ends_the_generation() {
    let (listener, url) = bind().await;
    let server = tokio::spawn(async move {
        let mut socket = accept(listener).await;
        authorize(&mut socket).await;
        assert_eq!(next_text(&mut socket).await, "slow\n3\n\n{}");
        time::sleep(Duration::from_millis(400)).await;
        let _late_result = socket
            .send(Message::text(r#"a[{"i":3,"s":200,"d":{"late":true}}]"#))
            .await;
    });
    let client = authenticated_client(&url, "access", None);
    let config = RealtimeConfig::default()
        .request_timeout(Duration::from_millis(250))
        .liveness_timeout(Duration::from_secs(2));
    let connection = connect(&client, SocketKind::User, config).await;

    assert!(matches!(
        connection.request_non_mutating("slow", "", "{}").await,
        Err(RealtimeError::RequestTimeout {
            request_id
        }) if request_id == RequestId::new(3)
    ));
    assert!(matches!(
        connection.shutdown().await,
        Err(RealtimeError::RequestTimeout { request_id })
            if request_id == RequestId::new(3)
    ));
    join(server).await;
}

#[tokio::test]
async fn oversized_request_is_rejected_before_actor_enqueue() {
    let (listener, url) = bind().await;
    let server = tokio::spawn(async move {
        let mut socket = accept(listener).await;
        send_text(&mut socket, "o").await;
        let _authorization = next_text(&mut socket).await;
        send_text(&mut socket, r#"a[{"i":1,"s":200}]"#).await;
        expect_close(&mut socket).await;
    });
    let client = authenticated_client(&url, "access", Some("market-data"));
    let config = RealtimeConfig::default().max_frame_bytes(128);
    let connection = connect(&client, SocketKind::MarketData, config).await;
    let body = "x".repeat(128);

    assert!(matches!(
        connection.request_non_mutating("fixture", "", &body).await,
        Err(RealtimeError::Codec(
            crate::realtime::CodecError::FrameTooLarge { .. }
        ))
    ));
    assert!(connection.shutdown().await.is_ok());
    join(server).await;
}

#[tokio::test]
async fn heartbeat_runs_without_application_traffic() {
    let (listener, url) = bind().await;
    let server = tokio::spawn(async move {
        let mut socket = accept(listener).await;
        authorize(&mut socket).await;
        let heartbeat = time::timeout(Duration::from_secs(3), next_text(&mut socket)).await;
        let Ok(heartbeat) = heartbeat else {
            panic!("heartbeat must arrive every 2.5 seconds");
        };
        assert_eq!(heartbeat, "[]");
        send_text(
            &mut socket,
            r#"a[{"e":"props","d":{"entityType":"order","eventType":"Updated","entity":{"id":10,"accountId":20,"contractId":30,"timestamp":"2026-08-21T00:00:00Z","action":"Buy","ordStatus":"Working","admin":false}}}]"#,
        )
        .await;
        expect_close(&mut socket).await;
    });
    let client = authenticated_client(&url, "access", None);
    let config = RealtimeConfig::default()
        .request_timeout(Duration::from_secs(4))
        .liveness_timeout(Duration::from_secs(5));
    let mut connection = connect(&client, SocketKind::User, config).await;

    assert!(matches!(
        connection
            .recv_event()
            .await
            .map(RealtimeEvent::into_payload),
        Some(RealtimeEventPayload::User(UserStreamEvent::Bootstrap(_)))
    ));
    let event = time::timeout(Duration::from_secs(4), connection.recv_event()).await;
    assert!(matches!(event, Ok(Some(_))));
    assert!(connection.shutdown().await.is_ok());
    join(server).await;
}

#[tokio::test]
async fn documentation_blocked_replay_clock_invalidates_the_generation() {
    let (listener, url) = bind().await;
    let server = tokio::spawn(async move {
        let mut socket = accept(listener).await;
        send_text(&mut socket, "o").await;
        let _authorization = next_text(&mut socket).await;
        send_text(&mut socket, r#"a[{"i":1,"s":200}]"#).await;
        send_text(&mut socket, r#"a[{"e":"clock","d":{"undocumented":1}}]"#).await;
    });
    let client = authenticated_client(&url, "access", None);
    let mut connection = connect(&client, SocketKind::Replay, RealtimeConfig::default()).await;

    let event = connection.recv_event().await;
    assert!(matches!(
        event.map(RealtimeEvent::into_payload),
        Some(RealtimeEventPayload::DocumentationBlocked(metadata))
            if metadata.capability() == DocumentationBlockedCapability::ReplayClockPayload
    ));
    let state = await_terminal_state(&mut connection).await;
    assert!(matches!(
        state,
        RealtimeState::ResyncRequired {
            reason: ResyncReason::UnsupportedEvent,
            ..
        }
    ));
    assert!(matches!(
        connection.shutdown().await,
        Err(RealtimeError::ResyncRequired {
            reason: ResyncReason::UnsupportedEvent,
            ..
        })
    ));
    join(server).await;
}

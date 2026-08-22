// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use super::*;
use crate::realtime::{UserSyncConfig, UserSyncEntityType, UserSyncShardBy, UserSyncSharding};

#[tokio::test]
async fn custom_user_sync_profile_reaches_the_single_owner_handshake() {
    let (listener, url) = bind().await;
    let server = tokio::spawn(async move {
        let mut socket = accept(listener).await;
        send_text(&mut socket, "o").await;
        let _authorization = next_text(&mut socket).await;
        send_text(&mut socket, r#"a[{"i":1,"s":200}]"#).await;
        let sync = next_text(&mut socket).await;
        assert!(sync.starts_with("user/syncrequest\n2\n\n"));
        assert!(sync.contains(r#""splitResponses":false"#));
        assert!(sync.contains(r#""entityTypes":["order","fill"]"#));
        assert!(sync.contains(r#""expressionType":"modAccountId""#));
        assert!(sync.contains(r#""divisor":3,"remainder":1"#));
        assert!(sync.contains(r#""fullOrgSnapshot":true"#));
        send_text(
            &mut socket,
            r#"a[{"i":2,"s":200,"d":{"users":[],"contractGroups":[]}}]"#,
        )
        .await;
        expect_close(&mut socket).await;
    });
    let client = authenticated_client(&url, "access", None);
    let shard = UserSyncSharding::new(UserSyncShardBy::AccountId, 3, 1)
        .unwrap_or_else(|error| panic!("fixture shard: {error}"));
    let sync = UserSyncConfig::new(vec![UserSyncEntityType::Order, UserSyncEntityType::Fill])
        .and_then(|config| config.sharding(shard))
        .map(|config| config.full_org_snapshot(true));
    let Ok(sync) = sync else {
        panic!("fixture sync profile must validate");
    };
    let connection = client
        .connect_user_realtime(RealtimeConfig::default(), sync)
        .await;
    let Ok(connection) = connection else {
        panic!("custom user socket must become ready");
    };
    assert!(connection.shutdown().await.is_ok());
    join(server).await;
}

#[tokio::test]
async fn status_only_user_sync_never_publishes_ready() {
    let (listener, url) = bind().await;
    let server = tokio::spawn(async move {
        let mut socket = accept(listener).await;
        send_text(&mut socket, "o").await;
        let _authorization = next_text(&mut socket).await;
        send_text(&mut socket, r#"a[{"i":1,"s":200}]"#).await;
        let _sync = next_text(&mut socket).await;
        send_text(&mut socket, r#"a[{"i":2,"s":200}]"#).await;
    });
    let client = authenticated_client(&url, "access", None);

    assert!(matches!(
        client
            .connect_realtime(SocketKind::User, RealtimeConfig::default())
            .await,
        Err(RealtimeError::UserSyncInvalidBootstrap)
    ));
    join(server).await;
}

#[tokio::test]
async fn arbitrary_object_user_sync_never_publishes_ready() {
    let (listener, url) = bind().await;
    let server = tokio::spawn(async move {
        let mut socket = accept(listener).await;
        send_text(&mut socket, "o").await;
        let _authorization = next_text(&mut socket).await;
        send_text(&mut socket, r#"a[{"i":1,"s":200}]"#).await;
        let _sync = next_text(&mut socket).await;
        send_text(&mut socket, r#"a[{"i":2,"s":200,"d":{"unrelated":[]}}]"#).await;
    });
    let client = authenticated_client(&url, "access", None);

    assert!(matches!(
        client
            .connect_realtime(SocketKind::User, RealtimeConfig::default())
            .await,
        Err(RealtimeError::UserSyncInvalidBootstrap)
    ));
    join(server).await;
}

#[tokio::test]
async fn unknown_collection_alongside_snapshot_invalidates_before_ready() {
    let (listener, url) = bind().await;
    let server = tokio::spawn(async move {
        let mut socket = accept(listener).await;
        send_text(&mut socket, "o").await;
        let _authorization = next_text(&mut socket).await;
        send_text(&mut socket, r#"a[{"i":1,"s":200}]"#).await;
        let _sync = next_text(&mut socket).await;
        send_text(
            &mut socket,
            r#"a[{"i":2,"s":200,"d":{"users":[],"contractGroups":[],"futureEntities":[]}}]"#,
        )
        .await;
    });
    let client = authenticated_client(&url, "access", None);

    assert!(matches!(
        client
            .connect_realtime(SocketKind::User, RealtimeConfig::default())
            .await,
        Err(RealtimeError::ResyncRequired {
            reason: ResyncReason::UnsupportedEvent,
            ..
        })
    ));
    join(server).await;
}

#[tokio::test]
async fn scalar_user_sync_collection_never_publishes_ready() {
    let (listener, url) = bind().await;
    let server = tokio::spawn(async move {
        let mut socket = accept(listener).await;
        send_text(&mut socket, "o").await;
        let _authorization = next_text(&mut socket).await;
        send_text(&mut socket, r#"a[{"i":1,"s":200}]"#).await;
        let _sync = next_text(&mut socket).await;
        send_text(&mut socket, r#"a[{"i":2,"s":200,"d":{"users":17}}]"#).await;
    });
    let client = authenticated_client(&url, "access", None);

    assert!(matches!(
        client
            .connect_realtime(SocketKind::User, RealtimeConfig::default())
            .await,
        Err(RealtimeError::UserSyncInvalidBootstrap)
    ));
    join(server).await;
}

#[tokio::test]
async fn scalar_user_sync_entity_never_publishes_ready() {
    let (listener, url) = bind().await;
    let server = tokio::spawn(async move {
        let mut socket = accept(listener).await;
        send_text(&mut socket, "o").await;
        let _authorization = next_text(&mut socket).await;
        send_text(&mut socket, r#"a[{"i":1,"s":200}]"#).await;
        let _sync = next_text(&mut socket).await;
        send_text(&mut socket, r#"a[{"i":2,"s":200,"d":{"users":[{},17]}}]"#).await;
    });
    let client = authenticated_client(&url, "access", None);

    assert!(matches!(
        client
            .connect_realtime(SocketKind::User, RealtimeConfig::default())
            .await,
        Err(RealtimeError::UserSyncInvalidBootstrap)
    ));
    join(server).await;
}

#[tokio::test]
async fn user_sync_control_payload_never_publishes_ready() {
    let (listener, url) = bind().await;
    let server = tokio::spawn(async move {
        let mut socket = accept(listener).await;
        send_text(&mut socket, "o").await;
        let _authorization = next_text(&mut socket).await;
        send_text(&mut socket, r#"a[{"i":1,"s":200}]"#).await;
        let _sync = next_text(&mut socket).await;
        send_text(
            &mut socket,
            r#"a[{"i":2,"s":200,"d":{"p-ticket":"synthetic","p-time":15,"p-captcha":true}}]"#,
        )
        .await;
    });
    let client = authenticated_client(&url, "access", None);

    assert!(matches!(
        client
            .connect_realtime(SocketKind::User, RealtimeConfig::default())
            .await,
        Err(RealtimeError::UserSyncPenalty {
            retry_after,
            captcha_required: true
        }) if retry_after == Duration::from_secs(15)
    ));
    join(server).await;
}

#[tokio::test]
async fn user_sync_429_installs_the_official_global_cooldown() {
    let (listener, url) = bind().await;
    let server = tokio::spawn(async move {
        let mut socket = accept(listener).await;
        send_text(&mut socket, "o").await;
        let _authorization = next_text(&mut socket).await;
        send_text(&mut socket, r#"a[{"i":1,"s":200}]"#).await;
        let _sync = next_text(&mut socket).await;
        send_text(&mut socket, r#"a[{"i":2,"s":429}]"#).await;
    });
    let client = authenticated_client(&url, "access", None);

    assert!(matches!(
        client
            .connect_realtime(SocketKind::User, RealtimeConfig::default())
            .await,
        Err(RealtimeError::ProviderRateLimit {
            retry_after,
            ..
        }) if retry_after == Duration::from_hours(1)
    ));
    assert!(client.rate_limits.try_admit_authenticated("fixture") >= Duration::from_mins(59));
    join(server).await;
}

#[tokio::test]
async fn user_sync_penalty_ends_setup_without_an_automatic_retry() {
    let (listener, url) = bind().await;
    let server = tokio::spawn(async move {
        let mut socket = accept(listener).await;
        send_text(&mut socket, "o").await;
        let _authorization = next_text(&mut socket).await;
        send_text(&mut socket, r#"a[{"i":1,"s":200}]"#).await;
        let original = next_text(&mut socket).await;
        assert!(original.starts_with("user/syncrequest\n2\n\n"));
        send_text(
            &mut socket,
            r#"a[{"i":2,"s":200,"d":{"p-ticket":"synthetic","p-time":0}}]"#,
        )
        .await;
        let next = time::timeout(Duration::from_millis(250), socket.next()).await;
        assert!(!matches!(next, Ok(Some(Ok(Message::Text(_))))));
    });
    let client = authenticated_client(&url, "access", None);
    assert!(matches!(
        client
            .connect_realtime(SocketKind::User, RealtimeConfig::default())
            .await,
        Err(RealtimeError::UserSyncPenalty {
            retry_after,
            captcha_required: false,
        }) if retry_after.is_zero()
    ));
    join(server).await;
}

#[tokio::test]
async fn user_sync_penalty_mixed_with_snapshot_evidence_fails_closed() {
    let (listener, url) = bind().await;
    let server = tokio::spawn(async move {
        let mut socket = accept(listener).await;
        send_text(&mut socket, "o").await;
        let _authorization = next_text(&mut socket).await;
        send_text(&mut socket, r#"a[{"i":1,"s":200}]"#).await;
        let _sync = next_text(&mut socket).await;
        send_text(
            &mut socket,
            r#"a[{"i":2,"s":200,"d":{"p-ticket":"synthetic","p-time":0,"users":[]}}]"#,
        )
        .await;
    });
    let client = authenticated_client(&url, "access", None);

    assert!(matches!(
        client
            .connect_realtime(SocketKind::User, RealtimeConfig::default())
            .await,
        Err(RealtimeError::UserSyncInvalidBootstrap)
    ));
    join(server).await;
}

#[tokio::test]
async fn unrepresentable_user_sync_penalty_never_creates_an_unbounded_wait() {
    let (listener, url) = bind().await;
    let server = tokio::spawn(async move {
        let mut socket = accept(listener).await;
        send_text(&mut socket, "o").await;
        let _authorization = next_text(&mut socket).await;
        send_text(&mut socket, r#"a[{"i":1,"s":200}]"#).await;
        let _sync = next_text(&mut socket).await;
        send_text(
            &mut socket,
            r#"a[{"i":2,"s":200,"d":{"p-ticket":"synthetic","p-time":18446744073709551615}}]"#,
        )
        .await;
    });
    let client = authenticated_client(&url, "access", None);

    let result = time::timeout(
        Duration::from_secs(1),
        client.connect_realtime(SocketKind::User, RealtimeConfig::default()),
    )
    .await;
    assert!(matches!(
        result,
        Ok(Err(RealtimeError::UserSyncPenalty {
            retry_after,
            captcha_required: false,
        })) if retry_after == Duration::from_secs(u64::MAX)
    ));
    assert!(
        client
            .rate_limits
            .try_admit_authenticated("/user/syncrequest")
            >= Duration::from_mins(59)
    );
    join(server).await;
}

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
async fn user_sync_staging_reserves_capacity_for_bootstrap() {
    let (listener, url) = bind().await;
    let server = tokio::spawn(async move {
        let mut socket = accept(listener).await;
        send_text(&mut socket, "o").await;
        let _authorization = next_text(&mut socket).await;
        send_text(&mut socket, r#"a[{"i":1,"s":200}]"#).await;
        let _sync = next_text(&mut socket).await;
        send_text(
            &mut socket,
            r#"a[{"i":2,"s":200,"d":{"users":[],"contractGroups":[]}},{"e":"props","d":{"entityType":"order","eventType":"Updated","entity":{"id":11,"accountId":20,"contractId":30,"timestamp":"2026-08-21T00:00:01Z","action":"Buy","ordStatus":"Working","admin":false}}}]"#,
        )
        .await;
    });
    let client = authenticated_client(&url, "access", None);
    let config = RealtimeConfig::default().event_capacity(1);

    assert!(matches!(
        client.connect_realtime(SocketKind::User, config).await,
        Err(RealtimeError::ResyncRequired {
            reason: ResyncReason::EventBufferOverflow,
            ..
        })
    ));
    join(server).await;
}

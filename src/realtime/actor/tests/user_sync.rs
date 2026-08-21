// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

use super::*;

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
            r#"a[{"i":1,"s":200},{"e":"props","d":{"phase":"authorization"}}]"#,
        )
        .await;
        let _sync = next_text(&mut socket).await;
        send_text(
            &mut socket,
            r#"a[{"i":2,"s":200,"d":{"users":[]}},{"e":"props","d":{"entityType":"order","eventType":"Updated"}}]"#,
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
        Some(RealtimeEventPayload::Bootstrap(_))
    ));
    assert!(matches!(
        connection
            .recv_event()
            .await
            .map(RealtimeEvent::into_payload),
        Some(RealtimeEventPayload::Event(Event::Properties(Some(_))))
    ));
    assert!(matches!(
        connection
            .recv_event()
            .await
            .map(RealtimeEvent::into_payload),
        Some(RealtimeEventPayload::Event(Event::Properties(Some(_))))
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
            r#"a[{"i":2,"s":200,"d":{"users":[]}},{"e":"props","d":{}}]"#,
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

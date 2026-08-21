// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

use tokio::sync::oneshot;

use super::*;

#[tokio::test]
async fn socket_kind_selects_the_required_token_with_market_data_fallback() {
    assert_token(SocketKind::User, Some("md"), "access").await;
    assert_token(SocketKind::Replay, Some("md"), "access").await;
    assert_token(SocketKind::MarketData, Some("md"), "md").await;
    assert_token(SocketKind::MarketData, None, "access").await;
}

#[tokio::test]
async fn authorization_rejection_never_yields_a_ready_handle() {
    let (listener, url) = bind().await;
    let server = tokio::spawn(async move {
        let mut socket = accept(listener).await;
        send_text(&mut socket, "o").await;
        let authorization = next_text(&mut socket).await;
        assert!(authorization.ends_with("access"));
        send_text(&mut socket, r#"a[{"i":1,"s":401}]"#).await;
    });
    let client = authenticated_client(&url, "access", None);

    let result = client
        .connect_realtime(SocketKind::User, RealtimeConfig::default())
        .await;
    assert!(matches!(
        result,
        Err(RealtimeError::AuthorizationRejected { status: 401 })
    ));
    join(server).await;
}

#[tokio::test]
async fn authorization_business_control_never_yields_a_ready_handle() {
    let (listener, url) = bind().await;
    let server = tokio::spawn(async move {
        let mut socket = accept(listener).await;
        send_text(&mut socket, "o").await;
        let _authorization = next_text(&mut socket).await;
        send_text(
            &mut socket,
            r#"a[{"i":1,"s":200,"d":{"errorText":"synthetic rejection"}}]"#,
        )
        .await;
    });
    let client = authenticated_client(&url, "access", None);

    assert!(matches!(
        client
            .connect_realtime(SocketKind::User, RealtimeConfig::default())
            .await,
        Err(RealtimeError::AuthorizationRejected { status: 200 })
    ));
    join(server).await;
}

#[tokio::test]
async fn authorization_captcha_installs_a_shared_lockout() {
    let (listener, url) = bind().await;
    let server = tokio::spawn(async move {
        let mut socket = accept(listener).await;
        send_text(&mut socket, "o").await;
        let _authorization = next_text(&mut socket).await;
        send_text(
            &mut socket,
            r#"a[{"i":1,"s":200,"d":{"p-ticket":"synthetic","p-time":0,"p-captcha":true}}]"#,
        )
        .await;
    });
    let client = authenticated_client(&url, "access", None);

    assert!(matches!(
        client
            .connect_realtime(SocketKind::User, RealtimeConfig::default())
            .await,
        Err(RealtimeError::AuthorizationPenalty {
            retry_after,
            captcha_required: true,
        }) if retry_after.is_zero()
    ));
    assert!(client.rate_limits.try_admit_authenticated("authorize") >= Duration::from_mins(59));
    join(server).await;
}

#[tokio::test]
async fn authorization_429_installs_the_official_global_cooldown() {
    let (listener, url) = bind().await;
    let server = tokio::spawn(async move {
        let mut socket = accept(listener).await;
        send_text(&mut socket, "o").await;
        let _authorization = next_text(&mut socket).await;
        send_text(&mut socket, r#"a[{"i":1,"s":429}]"#).await;
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
async fn rotated_token_is_rejected_before_authorization_is_sent() {
    let (listener, url) = bind().await;
    let (accepted, socket_accepted) = oneshot::channel();
    let (release_open, open_released) = oneshot::channel();
    let server = tokio::spawn(async move {
        let mut socket = accept(listener).await;
        assert!(accepted.send(()).is_ok());
        assert!(open_released.await.is_ok());
        send_text(&mut socket, "o").await;
        let message = time::timeout(Duration::from_secs(1), socket.next()).await;
        assert!(!matches!(message, Ok(Some(Ok(Message::Text(_))))));
    });
    let client = authenticated_client(&url, "access", None);
    let connecting = tokio::spawn({
        let client = client.clone();
        async move {
            client
                .connect_realtime(SocketKind::User, RealtimeConfig::default())
                .await
        }
    });

    assert!(socket_accepted.await.is_ok());
    let _superseding_authentication = client.tokens.begin_authentication();
    assert!(release_open.send(()).is_ok());

    let result = join_value(connecting).await;
    assert!(matches!(result, Err(RealtimeError::Unauthenticated)));
    join(server).await;
}

async fn assert_token(kind: SocketKind, market_data: Option<&str>, expected: &str) {
    let (listener, url) = bind().await;
    let expected_authorization = format!("authorize\n1\n\n{expected}");
    let server = tokio::spawn(async move {
        let mut socket = accept(listener).await;
        send_text(&mut socket, "o").await;
        let authorization = next_text(&mut socket).await;
        send_text(&mut socket, r#"a[{"i":1,"s":200}]"#).await;
        if matches!(kind, SocketKind::User) {
            expect_user_sync(&mut socket).await;
        }
        expect_close(&mut socket).await;
        authorization
    });
    let client = authenticated_client(&url, "access", market_data);
    let connection = connect(&client, kind, RealtimeConfig::default()).await;
    assert!(connection.shutdown().await.is_ok());
    let authorization = join_value(server).await;
    assert_eq!(authorization, expected_authorization);
}

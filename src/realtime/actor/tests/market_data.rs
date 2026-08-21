// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

use super::*;
use crate::{Symbol, realtime::MarketDataChannel};

#[tokio::test]
async fn typed_market_data_commands_encode_validated_symbols() {
    let (listener, url) = bind().await;
    let server = tokio::spawn(async move {
        let mut socket = accept(listener).await;
        authorize_market_data(&mut socket).await;
        assert_eq!(
            next_text(&mut socket).await,
            "md/subscribeQuote\n2\n\n{\"symbol\":\"ESZ6\"}"
        );
        send_text(&mut socket, r#"a[{"i":2,"s":200}]"#).await;
        assert_eq!(
            next_text(&mut socket).await,
            "md/unsubscribeQuote\n3\n\n{\"symbol\":\"ESZ6\"}"
        );
        send_text(&mut socket, r#"a[{"i":3,"s":200}]"#).await;
        expect_close(&mut socket).await;
    });
    let client = authenticated_client(&url, "access", Some("market-data"));
    let connection = connect(&client, SocketKind::MarketData, RealtimeConfig::default()).await;
    let symbol = Symbol::new("ESZ6").unwrap_or_else(|error| panic!("fixture symbol: {error}"));

    assert!(
        connection
            .subscribe_market_data(MarketDataChannel::Quotes, &symbol)
            .await
            .is_ok()
    );
    assert!(
        connection
            .unsubscribe_market_data(MarketDataChannel::Quotes, &symbol)
            .await
            .is_ok()
    );
    assert!(connection.shutdown().await.is_ok());
    join(server).await;
}

#[tokio::test]
async fn typed_market_data_command_rejects_a_user_socket_locally() {
    let (listener, url) = bind().await;
    let server = tokio::spawn(async move {
        let mut socket = accept(listener).await;
        authorize(&mut socket).await;
        expect_close(&mut socket).await;
    });
    let client = authenticated_client(&url, "access", None);
    let connection = connect(&client, SocketKind::User, RealtimeConfig::default()).await;
    let symbol = Symbol::new("ESZ6").unwrap_or_else(|error| panic!("fixture symbol: {error}"));

    assert!(matches!(
        connection
            .subscribe_market_data(MarketDataChannel::Quotes, &symbol)
            .await,
        Err(RealtimeError::WrongSocketKind {
            expected: SocketKind::MarketData,
            actual: SocketKind::User
        })
    ));
    assert!(connection.shutdown().await.is_ok());
    join(server).await;
}

#[tokio::test]
async fn websocket_429_uses_the_official_one_hour_cooldown() {
    let (listener, url) = bind().await;
    let server = tokio::spawn(async move {
        let mut socket = accept(listener).await;
        authorize_market_data(&mut socket).await;
        let _request = next_text(&mut socket).await;
        send_text(&mut socket, r#"a[{"i":2,"s":429}]"#).await;
        expect_close(&mut socket).await;
    });
    let client = authenticated_client(&url, "access", Some("market-data"));
    let connection = connect(&client, SocketKind::MarketData, RealtimeConfig::default()).await;
    let symbol = Symbol::new("ESZ6").unwrap_or_else(|error| panic!("fixture symbol: {error}"));

    assert!(matches!(
        connection
            .subscribe_market_data(MarketDataChannel::Quotes, &symbol)
            .await,
        Err(RealtimeError::ProviderRateLimit {
            retry_after,
            ..
        }) if retry_after == Duration::from_hours(1)
    ));
    assert!(connection.shutdown().await.is_ok());
    join(server).await;
}

#[tokio::test]
async fn correlated_business_control_terminates_the_generation() {
    let (listener, url) = bind().await;
    let server = tokio::spawn(async move {
        let mut socket = accept(listener).await;
        authorize_market_data(&mut socket).await;
        let _request = next_text(&mut socket).await;
        send_text(
            &mut socket,
            r#"a[{"i":2,"s":200,"d":{"errorText":"synthetic rejection"}}]"#,
        )
        .await;
    });
    let client = authenticated_client(&url, "access", Some("market-data"));
    let connection = connect(&client, SocketKind::MarketData, RealtimeConfig::default()).await;
    let symbol = Symbol::new("ESZ6").unwrap_or_else(|error| panic!("fixture symbol: {error}"));

    assert!(matches!(
        connection
            .subscribe_market_data(MarketDataChannel::Quotes, &symbol)
            .await,
        Err(RealtimeError::ProviderBusinessFailure { .. })
    ));
    assert!(matches!(
        connection.shutdown().await,
        Err(RealtimeError::ProviderBusinessFailure { .. })
    ));
    join(server).await;
}

#[tokio::test]
async fn correlated_penalty_control_installs_endpoint_cooldown() {
    let (listener, url) = bind().await;
    let server = tokio::spawn(async move {
        let mut socket = accept(listener).await;
        authorize_market_data(&mut socket).await;
        let _request = next_text(&mut socket).await;
        send_text(
            &mut socket,
            r#"a[{"i":2,"s":200,"d":{"p-ticket":"synthetic","p-time":15}}]"#,
        )
        .await;
    });
    let client = authenticated_client(&url, "access", Some("market-data"));
    let connection = connect(&client, SocketKind::MarketData, RealtimeConfig::default()).await;
    let symbol = Symbol::new("ESZ6").unwrap_or_else(|error| panic!("fixture symbol: {error}"));

    assert!(matches!(
        connection
            .subscribe_market_data(MarketDataChannel::Quotes, &symbol)
            .await,
        Err(RealtimeError::ProviderPenalty {
            retry_after,
            captcha_required: false,
            ..
        }) if retry_after == Duration::from_secs(15)
    ));
    assert!(
        client
            .rate_limits
            .try_admit_authenticated("md/subscribeQuote")
            >= Duration::from_secs(14)
    );
    join(server).await;
}

async fn authorize_market_data(socket: &mut WebSocketStream<tokio::net::TcpStream>) {
    send_text(socket, "o").await;
    assert!(next_text(socket).await.starts_with("authorize\n1\n\n"));
    send_text(socket, r#"a[{"i":1,"s":200}]"#).await;
}

// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use httpmock::prelude::*;
use tokio::{
    net::{TcpListener, TcpStream},
    time::timeout,
};
use tokio_tungstenite::{WebSocketStream, tungstenite::Message};
use tradovate_client::{
    Client, ContractId, EndpointSet,
    api::current::{SecretValue, authentication::OAuthToken},
    realtime::{RealtimeConfig, RealtimeConnection, RealtimeEvent, SocketKind},
};

pub type Socket = WebSocketStream<TcpStream>;

pub async fn fixture() -> (Client, TcpListener) {
    let http = MockServer::start_async().await;
    http.mock_async(|when, then| {
        when.method(POST).path("/v1/auth/oauthtoken");
        then.status(200)
            .body(r#"{"access_token":"synthetic-access","token_type":"Bearer","expires_in":3600}"#);
    })
    .await;
    http.mock_async(|when, then| {
        when.method(GET).path("/v1/auth/me");
        then.status(200).body(r#"{"userId":7}"#);
    })
    .await;
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .unwrap_or_else(|e| panic!("bind: {e}"));
    let addr = listener
        .local_addr()
        .unwrap_or_else(|e| panic!("address: {e}"));
    let ws = format!("ws://{addr}/v1/websocket");
    let endpoints = EndpointSet::custom(&format!("{}/v1", http.base_url()), &ws, &ws, &ws)
        .unwrap_or_else(|e| panic!("endpoints: {e}"));
    let client = Client::builder_with_endpoints(endpoints)
        .build()
        .unwrap_or_else(|e| panic!("client: {e}"));
    let secret = |s: &str| SecretValue::new(s).unwrap_or_else(|e| panic!("fixture secret: {e}"));
    let grant = OAuthToken::builder()
        .grant_type("authorization_code")
        .code(secret("synthetic-code"))
        .client_id("synthetic-client")
        .client_secret(secret("synthetic-secret"))
        .build()
        .unwrap_or_else(|e| panic!("grant: {e}"));
    client
        .authenticate_oauth(&grant)
        .await
        .unwrap_or_else(|e| panic!("authentication: {e}"));
    (client, listener)
}

pub async fn accept(listener: &TcpListener) -> Socket {
    let (stream, _) = timeout(Duration::from_secs(5), listener.accept())
        .await
        .unwrap_or_else(|e| panic!("accept timeout: {e}"))
        .unwrap_or_else(|e| panic!("accept: {e}"));
    tokio_tungstenite::accept_async(stream)
        .await
        .unwrap_or_else(|e| panic!("upgrade: {e}"))
}

pub async fn authorize(socket: &mut Socket) {
    send(socket, "o").await;
    assert_eq!(text(socket).await, "authorize\n1\n\nsynthetic-access");
    send(socket, r#"a[{"i":1,"s":200}]"#).await;
}

pub async fn send(socket: &mut Socket, text: &str) {
    socket
        .send(Message::text(text))
        .await
        .unwrap_or_else(|e| panic!("send: {e}"));
}

pub async fn text(socket: &mut Socket) -> String {
    loop {
        match timeout(Duration::from_secs(10), socket.next()).await {
            Ok(Some(Ok(Message::Text(text)))) if text == "[]" => {}
            Ok(Some(Ok(Message::Text(text)))) => return text.to_string(),
            Ok(Some(Ok(Message::Ping(data)))) => {
                socket
                    .send(Message::Pong(data))
                    .await
                    .unwrap_or_else(|e| panic!("pong: {e}"));
            }
            other => panic!("expected request: {other:?}"),
        }
    }
}

pub async fn closed(socket: &mut Socket) {
    loop {
        match timeout(Duration::from_secs(5), socket.next()).await {
            Ok(Some(Ok(Message::Close(_))) | None) => return,
            Ok(Some(Ok(Message::Text(text)))) if text == "[]" => {}
            Ok(Some(Ok(Message::Ping(data)))) => {
                socket
                    .send(Message::Pong(data))
                    .await
                    .unwrap_or_else(|e| panic!("pong: {e}"));
            }
            other => panic!("expected close: {other:?}"),
        }
    }
}

pub async fn connect(client: &Client, config: RealtimeConfig) -> RealtimeConnection {
    client
        .connect_realtime(SocketKind::MarketData, config)
        .await
        .unwrap_or_else(|e| panic!("connect: {e}"))
}

pub async fn event(connection: &mut RealtimeConnection) -> RealtimeEvent {
    timeout(Duration::from_secs(10), connection.recv_event())
        .await
        .unwrap_or_else(|e| panic!("event timeout: {e}"))
        .unwrap_or_else(|| panic!("missing event"))
}

pub fn contract(id: i64) -> ContractId {
    ContractId::new(id).unwrap_or_else(|e| panic!("contract: {e}"))
}

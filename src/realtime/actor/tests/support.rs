// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

use std::{sync::Arc, time::Duration};

use futures_util::{SinkExt, StreamExt};
use jiff::Timestamp;
use tokio::{net::TcpListener, task::JoinHandle, time};
use tokio_tungstenite::{WebSocketStream, accept_async, tungstenite::Message};

use super::*;
use crate::{
    Client, EndpointSet, UserId,
    auth::{InstalledSession, SessionInfo, TokenStore},
};

pub(super) async fn authorize(socket: &mut WebSocketStream<tokio::net::TcpStream>) {
    send_text(socket, "o").await;
    let authorization = next_text(socket).await;
    assert!(authorization.starts_with("authorize\n1\n\n"));
    send_text(socket, r#"a[{"i":1,"s":200}]"#).await;
    expect_user_sync(socket).await;
}

pub(super) async fn expect_user_sync(socket: &mut WebSocketStream<tokio::net::TcpStream>) {
    let request = next_text(socket).await;
    assert!(request.starts_with("user/syncrequest\n2\n\n"));
    assert!(request.contains(r#""splitResponses":false"#));
    assert!(request.contains(r#""entityTypes":["#));
    send_text(
        socket,
        r#"a[{"i":2,"s":200,"d":{"users":[],"contractGroups":[]}}]"#,
    )
    .await;
}

pub(super) async fn await_terminal_state(
    connection: &mut crate::realtime::RealtimeConnection,
) -> RealtimeState {
    loop {
        let current = connection.state();
        if !matches!(
            current,
            RealtimeState::Connecting { .. } | RealtimeState::Ready { .. }
        ) {
            return current;
        }
        let changed = time::timeout(Duration::from_secs(1), connection.state_changed()).await;
        let Ok(Ok(_)) = changed else {
            panic!("actor must publish a terminal state");
        };
    }
}

pub(super) async fn connect(
    client: &Client,
    kind: SocketKind,
    config: RealtimeConfig,
) -> crate::realtime::RealtimeConnection {
    let result = client.connect_realtime(kind, config).await;
    let Ok(connection) = result else {
        panic!("fixture connection must authorize");
    };
    connection
}

pub(super) fn authenticated_client(url: &str, access: &str, market_data: Option<&str>) -> Client {
    let endpoints = EndpointSet::custom("http://127.0.0.1:1/v1", url, url, url);
    let Ok(endpoints) = endpoints else {
        panic!("fixture endpoints must validate");
    };
    let Ok(expires_at) = "2030-01-01T00:00:00Z".parse::<Timestamp>() else {
        panic!("fixture timestamp must parse");
    };
    let Ok(user_id) = UserId::new(1) else {
        panic!("fixture user ID must validate");
    };
    let store = Arc::new(TokenStore::default());
    let attempt = store.begin_authentication();
    let session = InstalledSession::try_new(
        access.to_owned(),
        market_data.map(str::to_owned),
        SessionInfo::new(user_id, expires_at, market_data.is_some()),
    );
    let Ok(session) = session else {
        panic!("fixture session must validate");
    };
    assert!(attempt.commit(session).is_ok());
    Client {
        http: reqwest::Client::new(),
        endpoints,
        instance_id: 1,
        tokens: store,
        max_response_bytes: 1024,
        max_request_bytes: 8 * 1024 * 1024,
        rate_limits: Arc::new(crate::rate_limit::RateGovernor::tradovate_defaults()),
        mutation_gate: Arc::new(crate::client::MutationGate::default()),
    }
}

pub(super) async fn bind() -> (TcpListener, String) {
    let listener = TcpListener::bind("127.0.0.1:0").await;
    let Ok(listener) = listener else {
        panic!("fixture listener must bind");
    };
    let Ok(address) = listener.local_addr() else {
        panic!("fixture listener must have an address");
    };
    (listener, format!("ws://{address}/v1/websocket"))
}

pub(super) async fn accept(listener: TcpListener) -> WebSocketStream<tokio::net::TcpStream> {
    let accepted = listener.accept().await;
    let Ok((stream, _)) = accepted else {
        panic!("fixture must accept a TCP connection");
    };
    let socket = accept_async(stream).await;
    let Ok(socket) = socket else {
        panic!("fixture WebSocket handshake must succeed");
    };
    socket
}

pub(super) async fn next_text(socket: &mut WebSocketStream<tokio::net::TcpStream>) -> String {
    let message = socket.next().await;
    let Some(Ok(Message::Text(text))) = message else {
        panic!("fixture expected a text message");
    };
    text.as_str().to_owned()
}

pub(super) async fn expect_close(socket: &mut WebSocketStream<tokio::net::TcpStream>) {
    let message = time::timeout(Duration::from_secs(1), socket.next()).await;
    assert!(matches!(message, Ok(Some(Ok(Message::Close(_))) | None)));
}

pub(super) async fn send_text(socket: &mut WebSocketStream<tokio::net::TcpStream>, text: &str) {
    send_message(socket, Message::text(text)).await;
}

pub(super) async fn send_message(
    socket: &mut WebSocketStream<tokio::net::TcpStream>,
    message: Message,
) {
    assert!(socket.send(message).await.is_ok());
}

pub(super) async fn join(handle: JoinHandle<()>) {
    assert!(handle.await.is_ok());
}

pub(super) async fn join_value<T>(handle: JoinHandle<T>) -> T {
    let result = handle.await;
    let Ok(value) = result else {
        panic!("fixture server task must complete");
    };
    value
}

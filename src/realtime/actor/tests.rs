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
    realtime::{Event, RealtimeEventPayload, RequestId, SocketKind},
};

mod authentication;
mod cancellation;
mod market_data;
mod user_sync;

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
            r#"a[{"i":3,"s":200,"d":{"name":"ES"}},{"e":"md","d":{"price":"5000.25"}}]"#,
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
        Some(RealtimeEventPayload::Bootstrap(_))
    ));
    let Some(event) = connection.recv_event().await else {
        panic!("mixed frame must publish its event");
    };
    assert_eq!(event.connection_id(), connection_id);
    assert!(matches!(
        event.payload(),
        RealtimeEventPayload::Event(Event::MarketData(Some(_)))
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
        send_text(&mut socket, r#"a[{"e":"md","d":{"n":1}}]"#).await;
        send_text(
            &mut socket,
            r#"a[{"i":1,"s":200},{"e":"chart","d":{"n":2}}]"#,
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
        Some(RealtimeEventPayload::Event(Event::MarketData(Some(_))))
    ));
    assert!(matches!(
        connection
            .recv_event()
            .await
            .map(RealtimeEvent::into_payload),
        Some(RealtimeEventPayload::Event(Event::Chart(Some(_))))
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
            r#"a[{"e":"md","d":{"n":1}},{"e":"chart","d":{"n":2}}]"#,
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
        send_text(&mut socket, r#"a[{"e":"clock","d":{"t":"now"}}]"#).await;
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
        Some(RealtimeEventPayload::Bootstrap(_))
    ));
    let event = time::timeout(Duration::from_secs(4), connection.recv_event()).await;
    assert!(matches!(event, Ok(Some(_))));
    assert!(connection.shutdown().await.is_ok());
    join(server).await;
}

async fn authorize(socket: &mut WebSocketStream<tokio::net::TcpStream>) {
    send_text(socket, "o").await;
    let authorization = next_text(socket).await;
    assert!(authorization.starts_with("authorize\n1\n\n"));
    send_text(socket, r#"a[{"i":1,"s":200}]"#).await;
    expect_user_sync(socket).await;
}

async fn expect_user_sync(socket: &mut WebSocketStream<tokio::net::TcpStream>) {
    let request = next_text(socket).await;
    assert!(request.starts_with("user/syncrequest\n2\n\n"));
    assert!(request.contains(r#""splitResponses":false"#));
    assert!(request.contains(r#""entityTypes":["#));
    send_text(socket, r#"a[{"i":2,"s":200,"d":{"users":[]}}]"#).await;
}

async fn await_terminal_state(
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

async fn connect(
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

fn authenticated_client(url: &str, access: &str, market_data: Option<&str>) -> Client {
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
        rate_limits: Arc::new(crate::rate_limit::RateGovernor::tradovate_defaults()),
        mutation_gate: Arc::new(crate::client::MutationGate::default()),
    }
}

async fn bind() -> (TcpListener, String) {
    let listener = TcpListener::bind("127.0.0.1:0").await;
    let Ok(listener) = listener else {
        panic!("fixture listener must bind");
    };
    let Ok(address) = listener.local_addr() else {
        panic!("fixture listener must have an address");
    };
    (listener, format!("ws://{address}/v1/websocket"))
}

async fn accept(listener: TcpListener) -> WebSocketStream<tokio::net::TcpStream> {
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

async fn next_text(socket: &mut WebSocketStream<tokio::net::TcpStream>) -> String {
    let message = socket.next().await;
    let Some(Ok(Message::Text(text))) = message else {
        panic!("fixture expected a text message");
    };
    text.as_str().to_owned()
}

async fn expect_close(socket: &mut WebSocketStream<tokio::net::TcpStream>) {
    let message = time::timeout(Duration::from_secs(1), socket.next()).await;
    assert!(matches!(message, Ok(Some(Ok(Message::Close(_))) | None)));
}

async fn send_text(socket: &mut WebSocketStream<tokio::net::TcpStream>, text: &str) {
    send_message(socket, Message::text(text)).await;
}

async fn send_message(socket: &mut WebSocketStream<tokio::net::TcpStream>, message: Message) {
    assert!(socket.send(message).await.is_ok());
}

async fn join(handle: JoinHandle<()>) {
    assert!(handle.await.is_ok());
}

async fn join_value<T>(handle: JoinHandle<T>) -> T {
    let result = handle.await;
    let Ok(value) = result else {
        panic!("fixture server task must complete");
    };
    value
}

// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use tokio::sync::oneshot;

use super::*;

#[tokio::test]
async fn disconnect_drains_every_pending_request() {
    let (listener, url) = bind().await;
    let server = tokio::spawn(async move {
        let mut socket = accept(listener).await;
        authorize(&mut socket).await;
        assert_eq!(next_text(&mut socket).await, "order/placeorder\n3\n\n{}");
        send_message(&mut socket, Message::Close(None)).await;
    });
    let client = authenticated_client(&url, "access", None);
    let mut connection = connect(&client, SocketKind::User, RealtimeConfig::default()).await;
    assert!(matches!(
        connection
            .request_non_mutating("order/placeorder", "", "{}")
            .await,
        Err(RealtimeError::RequestOutcomeUncertain { request_id })
            if request_id == RequestId::new(3)
    ));
    assert!(matches!(
        await_terminal_state(&mut connection).await,
        RealtimeState::ResyncRequired {
            reason: ResyncReason::ConnectionLost,
            ..
        }
    ));
    assert!(matches!(
        connection.shutdown().await,
        Err(RealtimeError::ServerClosed)
    ));
    join(server).await;
}

#[tokio::test]
async fn abandoning_an_admitted_request_preserves_the_generation() {
    let (listener, url) = bind().await;
    let (admitted, request_admitted) = oneshot::channel();
    let (release, server_released) = oneshot::channel();
    let server = tokio::spawn(async move {
        let mut socket = accept(listener).await;
        authorize(&mut socket).await;
        assert_eq!(next_text(&mut socket).await, "contract/item\n3\n\n{}");
        assert!(admitted.send(()).is_ok());
        assert!(server_released.await.is_ok());
        drop(socket);
    });
    let client = authenticated_client(&url, "access", None);
    let connection = connect(&client, SocketKind::User, RealtimeConfig::default()).await;
    let mut request = Box::pin(connection.request_non_mutating("contract/item", "", "{}"));

    let admission = time::timeout(Duration::from_secs(1), async {
        tokio::select! {
            result = &mut request => panic!("request completed before admission: {result:?}"),
            result = request_admitted => result,
        }
    })
    .await;
    assert!(matches!(admission, Ok(Ok(()))));
    drop(request);

    tokio::task::yield_now().await;
    assert!(matches!(connection.state(), RealtimeState::Ready { .. }));
    assert!(connection.shutdown().await.is_ok());
    assert!(release.send(()).is_ok());
    join(server).await;
}

#[tokio::test(start_paused = true)]
async fn received_reply_settles_before_blocked_retirement_close() {
    use std::{sync::Arc, task::Poll};

    use futures_util::FutureExt;
    use tokio_tungstenite::{
        MaybeTlsStream, WebSocketStream,
        tungstenite::protocol::{Role, WebSocketConfig},
    };

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .unwrap_or_else(|error| panic!("listener: {error}"));
    let address = listener
        .local_addr()
        .unwrap_or_else(|error| panic!("address: {error}"));
    let (stream, accepted) =
        tokio::join!(tokio::net::TcpStream::connect(address), listener.accept());
    let stream = stream.unwrap_or_else(|error| panic!("connect: {error}"));
    let (peer, _) = accepted.unwrap_or_else(|error| panic!("accept: {error}"));
    // Hold outbound bytes above socket capacity without flushing until close.
    let buffer = 32 * 1024 * 1024;
    let socket = WebSocketStream::from_raw_socket(
        MaybeTlsStream::Plain(stream),
        Role::Client,
        Some(
            WebSocketConfig::default()
                .write_buffer_size(buffer * 2)
                .max_write_buffer_size(buffer * 3),
        ),
    )
    .await;
    let (mut writer, reader) = socket.split();
    writer
        .feed(Message::Binary(vec![0; buffer].into()))
        .await
        .unwrap_or_else(|error| panic!("queue outbound fixture: {error}"));
    let connection_id = ConnectionId::new(1);
    let events = Arc::new(crate::realtime::delivery::Delivery::new(connection_id));
    let (_commands, commands) = tokio::sync::mpsc::channel(1);
    let cancellation = CancellationToken::new();
    cancellation.cancel();
    let (reply, mut answered) = oneshot::channel();
    let mut pending = PendingRequests::with_capacity(1);
    pending.insert(
        RequestId::new(3),
        "contract/item",
        Instant::now() + Duration::from_secs(1),
        reply,
    );
    let mut actor = Actor {
        connection_id,
        config: RealtimeConfig::default(),
        codec: FrameCodec::new(1024, 10).unwrap_or_else(|error| panic!("codec: {error}")),
        writer, reader, commands, events: Arc::clone(&events), cancellation,
        request_abandoned: Arc::new(tokio::sync::Notify::new()),
        pending, last_received: Instant::now(), probe: None, next_probe: 0,
        batch: Some(crate::realtime::codec::RecordBatch::new(
            r#"a[{"e":"md","d":null},{"e":"chart","d":{"charts":[{"id":7,"eoh":true}]}},{"i":3,"s":200}]"#.into(), 10,
        ).unwrap_or_else(|error| panic!("batch: {error}"))),
        heartbeat: time::interval(Duration::from_hours(1)),
        rate_limits: Arc::new(crate::rate_limit::RateGovernor::tradovate_defaults()),
    };
    let mut retirement = Box::pin(actor.event_loop());
    // Every batch yield is explicitly polled; the unread peer keeps close pending.
    for _ in 0..12 {
        assert!(matches!(
            futures_util::poll!(retirement.as_mut()),
            Poll::Pending
        ));
    }
    assert!(
        matches!(answered.try_recv(), Ok(Ok(response)) if response.request_id() == RequestId::new(3))
    );
    assert!(matches!(
        events
            .recv()
            .now_or_never()
            .flatten()
            .map(RealtimeEvent::into_payload),
        Some(RealtimeEventPayload::ContinuityGap(_))
    ));
    assert!(matches!(
        events
            .recv()
            .now_or_never()
            .flatten()
            .map(RealtimeEvent::into_payload),
        Some(RealtimeEventPayload::Chart(_))
    ));
    assert!(events.recv().now_or_never().is_none());
    time::advance(Duration::from_secs(10)).await;
    let result = retirement.await;
    assert_eq!(result, Ok(()));
    actor.finish_generation(result).await;
    assert!(matches!(
        events.recv().await.map(RealtimeEvent::into_payload),
        Some(RealtimeEventPayload::GenerationEnded(Ok(())))
    ));
    assert!(events.recv().await.is_none());
    drop(peer);
}

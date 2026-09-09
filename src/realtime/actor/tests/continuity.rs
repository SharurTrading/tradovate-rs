// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use super::*;

#[tokio::test]
async fn timeout_does_not_close_a_healthy_socket() {
    let (listener, url) = bind().await;
    let server = tokio::spawn(async move {
        let mut socket = accept(listener).await;
        authorize(&mut socket).await;
        assert_eq!(next_text(&mut socket).await, "slow\n3\n\n{}");
        assert_eq!(next_text(&mut socket).await, "fast\n4\n\n{}");
        send_text(&mut socket, r#"a[{"i":4,"s":200},{"i":3,"s":200}]"#).await;
        expect_close(&mut socket).await;
    });
    let client = authenticated_client(&url, "access", None);
    let config = RealtimeConfig::default().request_timeout(Duration::from_millis(100));
    let connection = connect(&client, SocketKind::User, config).await;
    assert!(
        connection
            .request_non_mutating("slow", "", "{}")
            .await
            .is_err()
    );
    assert!(
        connection
            .request_non_mutating("fast", "", "{}")
            .await
            .is_ok()
    );
    assert!(connection.shutdown().await.is_ok());
    join(server).await;
}

#[tokio::test]
async fn cancellation_does_not_close_a_healthy_socket() {
    let (listener, url) = bind().await;
    let (admitted, admission) = oneshot::channel();
    let server = tokio::spawn(async move {
        let mut socket = accept(listener).await;
        authorize(&mut socket).await;
        assert_eq!(next_text(&mut socket).await, "slow\n3\n\n{}");
        assert!(admitted.send(()).is_ok());
        assert_eq!(next_text(&mut socket).await, "fast\n4\n\n{}");
        send_text(&mut socket, r#"a[{"i":4,"s":200},{"i":3,"s":200}]"#).await;
        expect_close(&mut socket).await;
    });
    let client = authenticated_client(&url, "access", None);
    let connection = connect(&client, SocketKind::User, RealtimeConfig::default()).await;
    let mut slow = Box::pin(connection.request_non_mutating("slow", "", "{}"));
    tokio::select! {
        result = &mut slow => panic!("unexpected completion: {result:?}"),
        result = admission => assert!(result.is_ok()),
    }
    drop(slow);
    assert!(
        connection
            .request_non_mutating("fast", "", "{}")
            .await
            .is_ok()
    );
    assert!(connection.shutdown().await.is_ok());
    join(server).await;
}

#[tokio::test]
async fn malformed_record_does_not_hide_a_co_batched_completion() {
    let (listener, url) = bind().await;
    let server = tokio::spawn(async move {
        let mut socket = accept(listener).await;
        authorize(&mut socket).await;
        let _request = next_text(&mut socket).await;
        send_text(
            &mut socket,
            r#"a[{"i":"bad","s":200},{"e":"md","d":null},{"i":3,"s":200}]"#,
        )
        .await;
        expect_close(&mut socket).await;
    });
    let client = authenticated_client(&url, "access", None);
    let connection = connect(&client, SocketKind::User, RealtimeConfig::default()).await;
    assert!(
        connection
            .request_non_mutating("fast", "", "{}")
            .await
            .is_ok()
    );
    assert!(connection.shutdown().await.is_ok());
    join(server).await;
}

#[tokio::test]
async fn ordinary_silence_does_not_close_a_healthy_socket() {
    let (listener, url) = bind().await;
    let server = tokio::spawn(async move {
        let mut socket = accept(listener).await;
        authorize(&mut socket).await;
        loop {
            match socket.next().await {
                Some(Ok(Message::Ping(payload))) => {
                    send_message(&mut socket, Message::Pong(payload)).await;
                }
                Some(Ok(Message::Text(text))) if text == "[]" => {}
                Some(Ok(Message::Text(_))) => {
                    send_text(&mut socket, r#"a[{"i":3,"s":200}]"#).await;
                    expect_close(&mut socket).await;
                    break;
                }
                other => panic!("unexpected socket end: {other:?}"),
            }
        }
    });
    let client = authenticated_client(&url, "access", None);
    let config = RealtimeConfig::default().liveness_timeout(Duration::from_millis(20));
    let connection = connect(&client, SocketKind::User, config).await;
    time::sleep(Duration::from_millis(80)).await;
    assert!(
        connection
            .request_non_mutating("fast", "", "{}")
            .await
            .is_ok()
    );
    assert!(connection.shutdown().await.is_ok());
    join(server).await;
}

#[test]
fn burst_capacities_are_independent_of_payload_ceilings() {
    assert!(
        RealtimeConfig::default()
            .command_capacity(8_192)
            .max_pending_requests(8_192)
            .event_capacity(65_536)
            .validate()
            .is_ok()
    );
}

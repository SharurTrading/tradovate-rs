// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Realtime control bounds fail with typed errors, never silent recovery.

#[path = "support/realtime.rs"]
mod support;

use support::*;
use tradovate_client::realtime::{MarketDataChannel, RealtimeConfig, RealtimeEventPayload};

/// Outbound admission stays bounded even though inbound venue events do not:
/// a full pending-request table refuses the new subscription with a typed
/// error before transmission, and the admitted one still completes.
#[tokio::test]
async fn pending_limit_refuses_new_work_with_a_typed_error() {
    use tradovate_client::realtime::RealtimeError;
    let (client, listener) = fixture().await;
    let (first_seen, seen) = tokio::sync::oneshot::channel();
    let (release, wait) = tokio::sync::oneshot::channel();
    let server = tokio::spawn(async move {
        let mut socket = accept(&listener).await;
        authorize(&mut socket).await;
        let first = text(&mut socket).await;
        assert!(first.starts_with("md/subscribeQuote\n2\n"));
        // Acknowledge only after the refusal below, so the first subscription's
        // pending slot stays occupied until then on every scheduler.
        assert!(first_seen.send(()).is_ok());
        assert!(wait.await.is_ok());
        send(&mut socket, r#"a[{"i":2,"s":200}]"#).await;
        closed(&mut socket).await;
    });
    let connection = connect(&client, RealtimeConfig::default().max_pending_requests(1)).await;
    let session = connection.session();
    let first = tokio::spawn(async move {
        session
            .subscribe_market_data(MarketDataChannel::Quotes, &contract(42))
            .await
    });
    assert!(seen.await.is_ok());
    assert!(matches!(
        connection
            .subscribe_market_data(MarketDataChannel::DepthOfMarket, &contract(43))
            .await,
        Err(RealtimeError::PendingLimitReached { limit: 1 })
    ));
    // Release the acknowledgment so the admitted subscription still completes.
    assert!(release.send(()).is_ok());
    assert!(first.await.unwrap_or_else(|e| panic!("join: {e}")).is_ok());
    assert!(connection.shutdown().await.is_ok());
    assert!(server.await.is_ok());
}

#[tokio::test]
async fn failed_active_probe_retains_the_original_liveness_error() {
    use futures_util::StreamExt;
    use std::time::Duration;
    use tokio_tungstenite::tungstenite::Message;
    use tradovate_client::realtime::RealtimeError;
    let (client, listener) = fixture().await;
    let (release, wait) = tokio::sync::oneshot::channel();
    let server = tokio::spawn(async move {
        let mut socket = accept(&listener).await;
        authorize(&mut socket).await;
        assert!(matches!(socket.next().await, Some(Ok(Message::Ping(_)))));
        // No further polls or flush: tungstenite's automatic pong stays unsent.
        assert!(wait.await.is_ok());
    });
    let mut connection = connect(
        &client,
        RealtimeConfig::default().liveness_timeout(Duration::from_millis(40)),
    )
    .await;
    let session = connection.session();
    assert!(matches!(
        event(&mut connection).await.payload(),
        RealtimeEventPayload::GenerationEnded(Err(RealtimeError::LivenessTimeout))
    ));
    session.wait_ended().await;
    assert!(matches!(
        session
            .subscribe_market_data(MarketDataChannel::Quotes, &contract(42))
            .await,
        Err(RealtimeError::StaleGeneration { .. })
    ));
    assert_eq!(
        connection.shutdown().await,
        Err(RealtimeError::LivenessTimeout)
    );
    assert!(release.send(()).is_ok());
    assert!(server.await.is_ok());
}

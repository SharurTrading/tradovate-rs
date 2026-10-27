// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Credential-free public API stress fixtures on a single-thread Tokio runtime.

#[path = "support/realtime.rs"]
mod support;

use futures_util::{StreamExt, stream::FuturesUnordered};
use support::*;
use tradovate_client::realtime::{MarketDataChannel, RealtimeConfig, RealtimeEventPayload};

const EVENTS: usize = 20_000;

async fn burst(active_consumer: bool, liveness: std::time::Duration) {
    let (client, listener) = fixture().await;
    let (sent, received) = tokio::sync::oneshot::channel();
    let server = tokio::spawn(async move {
        let mut socket = accept(&listener).await;
        authorize(&mut socket).await;
        let request = text(&mut socket).await;
        assert!(request.starts_with("md/subscribeQuote\n2\n"));
        let records = (1..=EVENTS)
            .map(|id| format!(r#"{{"e":"chart","d":{{"charts":[{{"id":{id},"eoh":true}}]}}}}"#))
            .collect::<Vec<_>>()
            .join(",");
        // A completion after the data must survive both buffering and decoding.
        send(&mut socket, &format!(r#"a[{records},{{"i":2,"s":200}}]"#)).await;
        assert!(sent.send(()).is_ok());
        closed(&mut socket).await;
    });
    let config = RealtimeConfig::default().liveness_timeout(liveness);
    let mut connection = connect(&client, config).await;
    let generation = connection.connection_id();
    let session = connection.session();
    let request = tokio::spawn(async move {
        session
            .subscribe_market_data(MarketDataChannel::Quotes, &contract(42))
            .await
    });
    assert!(received.await.is_ok());
    if active_consumer {
        drain_burst(&mut connection, generation, EVENTS).await;
    }
    // For a paused consumer this completion proves the entire burst was decoded
    // before draining, not merely copied into a kernel buffer.
    assert!(
        request
            .await
            .unwrap_or_else(|e| panic!("request join: {e}"))
            .is_ok()
    );
    if !active_consumer {
        drain_burst(&mut connection, generation, EVENTS).await;
    }
    assert!(connection.shutdown().await.is_ok());
    assert!(server.await.is_ok());
}

async fn drain_burst(
    connection: &mut tradovate_client::realtime::RealtimeConnection,
    generation: tradovate_client::realtime::ConnectionId,
    total: usize,
) {
    for expected in 1..=total {
        let item = event(connection).await;
        assert_eq!(item.connection_id(), generation);
        let payload = item.into_payload();
        let RealtimeEventPayload::Chart(chart) = payload else {
            panic!("unexpected event: {payload:?}");
        };
        assert!(
            matches!(&chart.packets()[0], tradovate_client::realtime::ChartPacket::EndOfHistory(id)
            if id.get() == i64::try_from(expected).unwrap_or_default())
        );
    }
}

#[tokio::test]
async fn twenty_thousand_events_with_a_paused_consumer() {
    burst(false, std::time::Duration::from_secs(10)).await;
}

#[tokio::test]
async fn twenty_thousand_events_through_a_concurrent_drain() {
    burst(true, std::time::Duration::from_secs(10)).await;
}

#[tokio::test]
async fn buffered_burst_does_not_manufacture_a_failed_probe() {
    burst(true, std::time::Duration::from_millis(10)).await;
}

/// The former 65,536-event delivery capacity no longer exists. A receiver
/// stalled across multiple frames past that count still drains every accepted
/// event exactly once, in order, with no continuity gap and observable lag.
#[tokio::test]
async fn stalled_receiver_past_former_capacity_drains_every_event_in_order() {
    const TOTAL: usize = 70_000;
    const FIRST_FRAME: usize = 40_000;
    let (client, listener) = fixture().await;
    let (sent, received) = tokio::sync::oneshot::channel();
    let server = tokio::spawn(async move {
        let mut socket = accept(&listener).await;
        authorize(&mut socket).await;
        let request = text(&mut socket).await;
        assert!(request.starts_with("md/subscribeQuote\n2\n"));
        // Two frames keep each under the per-frame record limit; together they
        // exceed the former event-queue capacity.
        for (start, end) in [(0, FIRST_FRAME), (FIRST_FRAME, TOTAL)] {
            let records = (start..end)
                .map(|id| {
                    format!(
                        r#"{{"e":"chart","d":{{"charts":[{{"id":{},"eoh":true}}]}}}}"#,
                        { id + 1 }
                    )
                })
                .collect::<Vec<_>>()
                .join(",");
            send(&mut socket, &format!(r"a[{records}]")).await;
        }
        send(&mut socket, r#"a[{"i":2,"s":200}]"#).await;
        assert!(sent.send(()).is_ok());
        closed(&mut socket).await;
    });
    let mut connection = connect(&client, RealtimeConfig::default()).await;
    let generation = connection.connection_id();
    let session = connection.session();
    let request = tokio::spawn(async move {
        session
            .subscribe_market_data(MarketDataChannel::Quotes, &contract(42))
            .await
    });
    assert!(received.await.is_ok());
    // The completion can only resolve after every record of both frames was
    // decoded, so the backlog observation below is deterministic.
    assert!(
        request
            .await
            .unwrap_or_else(|e| panic!("request join: {e}"))
            .is_ok()
    );
    let stalled = connection.backlog();
    assert_eq!(stalled.depth(), TOTAL);
    assert!(stalled.oldest_event_age().is_some());
    drain_burst(&mut connection, generation, TOTAL).await;
    let drained = connection.backlog();
    assert_eq!(drained.depth(), 0);
    assert_eq!(drained.oldest_event_age(), None);
    assert!(connection.shutdown().await.is_ok());
    assert!(server.await.is_ok());
}

/// One bootstrap collection larger than every former fixed decoded-collection
/// cap (16,384 entities) decodes fully, in order, through the public API.
#[tokio::test]
async fn bootstrap_larger_than_former_collection_caps_decodes_fully() {
    use tradovate_client::realtime::{UserEntityBatch, UserStreamEvent, UserSyncConfig};
    const ORDERS: i64 = 20_000;
    let (client, listener) = fixture().await;
    let server = tokio::spawn(async move {
        let mut socket = accept(&listener).await;
        authorize(&mut socket).await;
        let sync = text(&mut socket).await;
        assert!(sync.starts_with("user/syncrequest\n2\n\n"));
        let orders = (1..=ORDERS)
            .map(|id| {
                format!(
                    r#"{{"id":{id},"accountId":20,"timestamp":"2026-08-21T00:00:00Z","action":"Buy","ordStatus":"Working","admin":false}}"#
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        send(
            &mut socket,
            &format!(
                r#"a[{{"i":2,"s":200,"d":{{"users":[],"contractGroups":[],"orders":[{orders}]}}}}]"#
            ),
        )
        .await;
        closed(&mut socket).await;
    });
    let mut connection = client
        .connect_user_realtime(RealtimeConfig::default(), UserSyncConfig::default())
        .await
        .unwrap_or_else(|e| panic!("user connect: {e}"));
    let payload = event(&mut connection).await.into_payload();
    let RealtimeEventPayload::User(UserStreamEvent::Bootstrap(bootstrap)) = payload else {
        panic!("expected bootstrap, got: {payload:?}");
    };
    let orders = bootstrap
        .entities()
        .iter()
        .find_map(|batch| match batch {
            UserEntityBatch::Orders(orders) => Some(orders),
            _ => None,
        })
        .unwrap_or_else(|| panic!("orders batch missing"));
    assert_eq!(orders.len(), 20_000);
    assert!(
        orders
            .first()
            .is_some_and(|order| order.id().is_some_and(|id| id.get() == 1))
            && orders
                .last()
                .is_some_and(|order| order.id().is_some_and(|id| id.get() == ORDERS))
    );
    assert!(connection.shutdown().await.is_ok());
    assert!(server.await.is_ok());
}

#[tokio::test]
async fn three_thousand_concurrent_subscriptions_and_events() {
    const SUBSCRIPTIONS: usize = 3_000;
    let (client, listener) = fixture().await;
    let server = tokio::spawn(async move {
        let mut socket = accept(&listener).await;
        authorize(&mut socket).await;
        let mut identities = std::collections::HashSet::new();
        let mut records = Vec::new();
        for _ in 0..SUBSCRIPTIONS {
            let request = text(&mut socket).await;
            let id = request.lines().nth(1).unwrap_or_default();
            assert!(identities.insert(id.to_owned()), "request identity reused");
            records.push(format!(
                r#"{{"i":{id},"s":200}},{{"e":"chart","d":{{"charts":[{{"id":9,"eoh":true}}]}}}}"#
            ));
        }
        // No acknowledgements until all 3,000 invocations are outstanding.
        send(&mut socket, &format!("a[{}]", records.join(","))).await;
        assert!(
            text(&mut socket)
                .await
                .starts_with("md/unsubscribeQuote\n3002\n")
        );
        send(&mut socket, r#"a[{"i":3002,"s":200}]"#).await;
        closed(&mut socket).await;
    });
    let mut connection = connect(
        &client,
        RealtimeConfig::default()
            .command_capacity(8_192)
            .max_pending_requests(8_192),
    )
    .await;
    let mut requests = FuturesUnordered::new();
    for id in 1..=SUBSCRIPTIONS {
        let session = connection.session();
        requests.push(async move {
            session
                .subscribe_market_data(
                    MarketDataChannel::Quotes,
                    &contract(i64::try_from(id).unwrap_or_default()),
                )
                .await
        });
    }
    while let Some(result) = requests.next().await {
        assert!(result.is_ok(), "{result:?}");
    }
    for _ in 0..SUBSCRIPTIONS {
        assert!(matches!(
            event(&mut connection).await.payload(),
            RealtimeEventPayload::Chart(_)
        ));
    }
    assert!(
        connection
            .unsubscribe_market_data(MarketDataChannel::Quotes, &contract(42))
            .await
            .is_ok()
    );
    assert!(connection.shutdown().await.is_ok());
    assert!(server.await.is_ok());
}

#[tokio::test]
async fn continuity_gap_delivers_prefix_exactly_one_ack_and_recovered_data() {
    let (client, listener) = fixture().await;
    let server = tokio::spawn(async move {
        let mut socket = accept(&listener).await;
        authorize(&mut socket).await;
        let _request = text(&mut socket).await;
        // The malformed record fences data publication: the co-batched record
        // behind it cannot cross the gap, and later frames resume after the
        // caller acknowledges the exact marker.
        send(&mut socket, r#"a[{"e":"chart","d":{"charts":[{"id":1,"eoh":true}]}},{"e":"md","d":null},{"e":"chart","d":{"charts":[{"id":3,"eoh":true}]}},{"i":2,"s":200}]"#).await;
        let _second = text(&mut socket).await;
        send(
            &mut socket,
            r#"a[{"e":"chart","d":{"charts":[{"id":99,"eoh":true}]}},{"i":3,"s":200}]"#,
        )
        .await;
        closed(&mut socket).await;
    });
    let mut connection = connect(&client, RealtimeConfig::default()).await;
    let generation = connection.connection_id();
    let id = contract(42);
    assert!(
        connection
            .subscribe_market_data(MarketDataChannel::Quotes, &id)
            .await
            .is_ok()
    );
    assert!(matches!(
        event(&mut connection).await.payload(),
        RealtimeEventPayload::Chart(_)
    ));
    let RealtimeEventPayload::ContinuityGap(gap) = event(&mut connection).await.into_payload()
    else {
        panic!("missing ordered gap");
    };
    assert_eq!(
        gap.reason(),
        tradovate_client::realtime::ResyncReason::MalformedRecord
    );
    assert!(matches!(
        connection.state(),
        tradovate_client::realtime::RealtimeState::Ready { .. }
    ));
    assert!(connection.acknowledge_continuity_gap(gap));
    assert!(!connection.acknowledge_continuity_gap(gap));
    assert!(
        connection
            .subscribe_market_data(MarketDataChannel::DepthOfMarket, &id)
            .await
            .is_ok()
    );
    let recovered = event(&mut connection).await;
    assert_eq!(recovered.connection_id(), generation);
    let RealtimeEventPayload::Chart(chart) = recovered.into_payload() else {
        panic!("data did not resume");
    };
    assert!(
        matches!(&chart.packets()[0], tradovate_client::realtime::ChartPacket::EndOfHistory(id) if id.get() == 99)
    );
    assert!(connection.shutdown().await.is_ok());
    assert!(server.await.is_ok());
}

/// A frame with more records than the configured per-frame decode limit is a
/// genuine record-safety loss: the accepted prefix is retained, the excess
/// records install an explicit continuity failure, and the socket stays usable.
#[tokio::test]
async fn records_above_the_configured_frame_limit_fail_explicitly() {
    use tradovate_client::realtime::ResyncReason;
    let (client, listener) = fixture().await;
    let server = tokio::spawn(async move {
        let mut socket = accept(&listener).await;
        authorize(&mut socket).await;
        let _request = text(&mut socket).await;
        // The completion rides inside the configured two-record budget; the
        // third chart record exceeds the limit and fails explicitly.
        send(&mut socket, r#"a[{"i":2,"s":200},{"e":"chart","d":{"charts":[{"id":1,"eoh":true}]}},{"e":"chart","d":{"charts":[{"id":2,"eoh":true}]}}]"#).await;
        let _second = text(&mut socket).await;
        send(
            &mut socket,
            r#"a[{"e":"chart","d":{"charts":[{"id":9,"eoh":true}]}},{"i":3,"s":200}]"#,
        )
        .await;
        closed(&mut socket).await;
    });
    let mut connection =
        connect(&client, RealtimeConfig::default().max_messages_per_frame(2)).await;
    let id = contract(42);
    assert!(
        connection
            .subscribe_market_data(MarketDataChannel::Quotes, &id)
            .await
            .is_ok()
    );
    assert!(matches!(
        event(&mut connection).await.payload(),
        RealtimeEventPayload::Chart(_)
    ));
    let RealtimeEventPayload::ContinuityGap(gap) = event(&mut connection).await.into_payload()
    else {
        panic!("missing explicit frame-limit failure");
    };
    assert_eq!(gap.reason(), ResyncReason::MalformedRecord);
    assert!(connection.acknowledge_continuity_gap(gap));
    assert!(
        connection
            .subscribe_market_data(MarketDataChannel::DepthOfMarket, &id)
            .await
            .is_ok()
    );
    let recovered = event(&mut connection).await.into_payload();
    assert!(matches!(
        &recovered,
        RealtimeEventPayload::Chart(chart)
            if matches!(&chart.packets()[0], tradovate_client::realtime::ChartPacket::EndOfHistory(id) if id.get() == 9)
    ));
    assert!(connection.shutdown().await.is_ok());
    assert!(server.await.is_ok());
}

#[tokio::test]
async fn malformed_records_keep_completions_and_refusal_errors_intact() {
    use tradovate_client::realtime::{RealtimeError, ResyncReason};
    let (client, listener) = fixture().await;
    let server = tokio::spawn(async move {
        let mut socket = accept(&listener).await;
        authorize(&mut socket).await;
        let _first = text(&mut socket).await;
        // Even invalid JSON within a balanced record must not hide the next record.
        send(&mut socket, r#"a[{"e":"md","d":bad},{"i":"invalid","s":200},{"i":2,"s":200,"d":{"errorText":"synthetic refusal"}}]"#).await;
        let _second = text(&mut socket).await;
        send(&mut socket, r#"a[{"i":3,"s":200}]"#).await;
        closed(&mut socket).await;
    });
    let mut connection = connect(&client, RealtimeConfig::default()).await;
    let id = contract(42);
    assert!(matches!(
        connection
            .subscribe_market_data(MarketDataChannel::Quotes, &id)
            .await,
        Err(RealtimeError::ProviderBusinessFailure { .. })
    ));
    let RealtimeEventPayload::ContinuityGap(gap) = event(&mut connection).await.into_payload()
    else {
        panic!("missing gap");
    };
    assert_eq!(gap.reason(), ResyncReason::MalformedRecord);
    // Requests remain available even before data recovery is acknowledged.
    assert!(
        connection
            .subscribe_market_data(MarketDataChannel::DepthOfMarket, &id)
            .await
            .is_ok()
    );
    assert!(connection.acknowledge_continuity_gap(gap));
    assert!(connection.shutdown().await.is_ok());
    assert!(server.await.is_ok());
}

/// Outbound admission stays bounded even though inbound venue events do not:
/// a full pending-request table refuses the new subscription with a typed
/// error before transmission, and the admitted one still completes.
#[tokio::test]
async fn pending_limit_refuses_new_work_with_a_typed_error() {
    use tradovate_client::realtime::RealtimeError;
    let (client, listener) = fixture().await;
    let (first_seen, seen) = tokio::sync::oneshot::channel();
    let server = tokio::spawn(async move {
        let mut socket = accept(&listener).await;
        authorize(&mut socket).await;
        let first = text(&mut socket).await;
        assert!(first.starts_with("md/subscribeQuote\n2\n"));
        // Never acknowledge the first subscription, so its pending slot stays
        // occupied while the second request is refused locally.
        assert!(first_seen.send(()).is_ok());
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

/// An unanswered active probe expires even when successive valid batches arrive.
#[tokio::test]
async fn continuous_batches_cannot_renew_an_unanswered_probe_forever() {
    use std::time::Duration;
    use tokio::io::AsyncWriteExt;
    use tokio_tungstenite::tungstenite::Message;
    use tradovate_client::realtime::RealtimeError;
    let (client, listener) = fixture().await;
    let config = RealtimeConfig::default().liveness_timeout(Duration::from_millis(40));
    // Keep real TCP latency outside the liveness clock. In particular, delayed
    // ACKs must not consume the entire probe window before a second batch arrives.
    tokio::time::pause();
    let (connection, records, result) = with_manual_clock(async {
        let (mut connection, mut socket) = tokio::join!(connect(&client, config), async {
            let mut socket = accept(&listener).await;
            authorize(&mut socket).await;
            socket
        });
        // Cross the idle deadline, including Tokio's millisecond timer rounding.
        tokio::time::advance(Duration::from_millis(41)).await;
        assert!(matches!(socket.next().await, Some(Ok(Message::Ping(_)))));
        let payload = br#"a[{"e":"chart","d":{"charts":[{"id":9,"eoh":true}]}}]"#;
        let mut frame = vec![0x81, u8::try_from(payload.len()).unwrap_or_default()];
        frame.extend_from_slice(payload);
        for records in 0..10 {
            // Bypass tungstenite so its queued automatic Pong stays unsent.
            // A terminal actor may already have closed TCP at this step.
            let sent = socket.get_mut().write_all(&frame).await;
            match event(&mut connection).await.into_payload() {
                RealtimeEventPayload::Chart(_) => assert!(sent.is_ok(), "{sent:?}"),
                RealtimeEventPayload::GenerationEnded(result) => {
                    return (connection, records, result);
                }
                other => panic!("unexpected event: {other:?}"),
            }
            // Each accepted batch precedes the next clock step. More than two
            // full probe windows cannot be sustained by renewing read grace.
            tokio::time::advance(Duration::from_millis(10)).await;
        }
        panic!("ten batches renewed an unanswered probe beyond its deadline");
    })
    .await;
    tokio::time::resume();
    assert!(records >= 2, "only {records} batches exercised read grace");
    assert_eq!(result, Err(RealtimeError::LivenessTimeout));
    assert_eq!(
        connection.shutdown().await,
        Err(RealtimeError::LivenessTimeout)
    );
}

/// A paused Tokio clock normally jumps to the next timer when TCP is pending.
/// Keep the current-thread runtime runnable so only explicit advance calls move
/// time, with a separate real-time watchdog for fixture failures. No task is spawned.
async fn with_manual_clock<T>(operation: impl std::future::Future<Output = T>) -> T {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    tokio::pin!(operation);
    loop {
        assert!(std::time::Instant::now() < deadline, "fixture I/O stalled");
        tokio::select! {
            biased;
            result = &mut operation => return result,
            () = tokio::task::yield_now() => {},
        }
    }
}

#[tokio::test]
async fn remote_close_retains_prefix_gap_and_end_in_order() {
    use futures_util::SinkExt;
    use tokio_tungstenite::tungstenite::Message;
    use tradovate_client::realtime::RealtimeError;
    let (client, listener) = fixture().await;
    let server = tokio::spawn(async move {
        let mut socket = accept(&listener).await;
        authorize(&mut socket).await;
        let _request = text(&mut socket).await;
        send(&mut socket, r#"a[{"e":"chart","d":{"charts":[{"id":1,"eoh":true}]}},{"e":"md","d":null},{"e":"shutdown","d":{"reasonCode":"Maintenance"}},{"i":2,"s":200}]"#).await;
        assert!(socket.send(Message::Close(None)).await.is_ok());
    });
    let mut connection = connect(&client, RealtimeConfig::default()).await;
    assert!(
        connection
            .subscribe_market_data(MarketDataChannel::Quotes, &contract(42))
            .await
            .is_ok()
    );
    assert!(matches!(
        event(&mut connection).await.payload(),
        RealtimeEventPayload::Chart(_)
    ));
    let RealtimeEventPayload::ContinuityGap(gap) = event(&mut connection).await.into_payload()
    else {
        panic!("missing gap");
    };
    assert!(matches!(
        event(&mut connection).await.payload(),
        RealtimeEventPayload::Shutdown(_)
    ));
    assert!(matches!(
        event(&mut connection).await.payload(),
        RealtimeEventPayload::GenerationEnded(Err(RealtimeError::ServerClosed))
    ));
    assert!(!connection.acknowledge_continuity_gap(gap));
    assert!(connection.recv_event().await.is_none());
    assert_eq!(
        connection.shutdown().await,
        Err(RealtimeError::ServerClosed)
    );
    assert!(server.await.is_ok());
}

#[tokio::test]
async fn stale_sessions_and_chart_ids_cannot_attach_to_a_replacement() {
    use tradovate_client::{
        Symbol,
        realtime::{ChartElementUnit, ChartRequest, ChartUnderlyingType, RealtimeError},
    };
    let (client, listener) = fixture().await;
    let server = tokio::spawn(async move {
        let mut first = accept(&listener).await;
        authorize(&mut first).await;
        assert!(text(&mut first).await.starts_with("md/getChart\n2\n"));
        send(
            &mut first,
            r#"a[{"i":2,"s":200,"d":{"historicalId":8,"realtimeId":9}}]"#,
        )
        .await;
        closed(&mut first).await;
        let mut second = accept(&listener).await;
        authorize(&mut second).await;
        // Stale chart cancellation and stale callbacks must enqueue nothing.
        assert!(
            text(&mut second)
                .await
                .starts_with("md/subscribeQuote\n2\n")
        );
        send(&mut second, r#"a[{"i":2,"s":200}]"#).await;
        closed(&mut second).await;
    });
    let first = connect(&client, RealtimeConfig::default()).await;
    let stale = first.session();
    let symbol = Symbol::new("ESZ6").unwrap_or_else(|e| panic!("symbol: {e}"));
    let request = ChartRequest::for_symbol(
        symbol,
        ChartUnderlyingType::MinuteBar,
        1,
        ChartElementUnit::UnderlyingUnits,
    )
    .as_much_as_elements(10)
    .build()
    .unwrap_or_else(|e| panic!("chart request: {e}"));
    let chart = first
        .get_chart(&request)
        .await
        .unwrap_or_else(|e| panic!("chart: {e}"));
    assert_eq!(chart.realtime_id().connection_id(), first.connection_id());
    assert!(first.shutdown().await.is_ok());
    let second = connect(&client, RealtimeConfig::default()).await;
    assert_ne!(stale.connection_id(), second.connection_id());
    assert!(matches!(
        stale
            .subscribe_market_data(MarketDataChannel::Quotes, &contract(42))
            .await,
        Err(RealtimeError::StaleGeneration { .. })
    ));
    assert!(matches!(
        second.cancel_chart(chart.realtime_id()).await,
        Err(RealtimeError::StaleGeneration { .. })
    ));
    assert!(
        second
            .subscribe_market_data(MarketDataChannel::Quotes, &contract(42))
            .await
            .is_ok()
    );
    assert!(second.shutdown().await.is_ok());
    assert!(server.await.is_ok());
}

#[test]
fn final_owner_drop_outside_tokio_keeps_tracked_teardown() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap_or_else(|e| panic!("runtime: {e}"));
    let (connection, server) = runtime.block_on(async {
        let (client, listener) = fixture().await;
        let server = tokio::spawn(async move {
            let mut socket = accept(&listener).await;
            authorize(&mut socket).await;
            closed(&mut socket).await;
        });
        (connect(&client, RealtimeConfig::default()).await, server)
    });
    let session = connection.session();
    drop(connection);
    runtime.block_on(async {
        session.wait_ended().await;
        assert!(server.await.is_ok());
    });
}

#[tokio::test]
async fn cancelling_shutdown_keeps_join_accounting() {
    let (client, listener) = fixture().await;
    let server = tokio::spawn(async move {
        let mut socket = accept(&listener).await;
        authorize(&mut socket).await;
        closed(&mut socket).await;
    });
    let connection = connect(&client, RealtimeConfig::default()).await;
    let session = connection.session();
    let mut shutdown = Box::pin(connection.shutdown());
    assert!(futures_util::poll!(&mut shutdown).is_pending());
    drop(shutdown);
    session.wait_ended().await;
    assert!(server.await.is_ok());
}

#[tokio::test]
async fn cancelled_replacement_handshake_cannot_authorize_late() {
    use futures_util::StreamExt;
    use std::time::Duration;
    use tokio_tungstenite::tungstenite::Message;
    use tradovate_client::realtime::SocketKind;
    let (client, listener) = fixture().await;
    let (accepted, ready) = tokio::sync::oneshot::channel();
    let server = tokio::spawn(async move {
        let mut first = accept(&listener).await;
        authorize(&mut first).await;
        closed(&mut first).await;
        let mut replacement = accept(&listener).await;
        assert!(accepted.send(()).is_ok());
        // Cancellation happens while the caller-owned replacement awaits `o`.
        let end = tokio::time::timeout(Duration::from_secs(2), replacement.next()).await;
        assert!(!matches!(end, Ok(Some(Ok(Message::Text(_))))));
        assert!(end.is_ok(), "cancelled setup did not close its socket");
    });
    let first = connect(&client, RealtimeConfig::default()).await;
    assert!(first.shutdown().await.is_ok());
    let mut replacement =
        Box::pin(client.connect_realtime(SocketKind::MarketData, RealtimeConfig::default()));
    tokio::select! {
        result = &mut replacement => panic!("premature readiness: {result:?}"),
        result = ready => assert!(result.is_ok()),
    }
    drop(replacement);
    assert!(server.await.is_ok());
}

#[test]
fn runtime_shutdown_publishes_termination_after_producers_are_dropped() {
    use tradovate_client::realtime::RealtimeError;
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap_or_else(|e| panic!("runtime: {e}"));
    let (mut connection, server) = runtime.block_on(async {
        let (client, listener) = fixture().await;
        let server = tokio::spawn(async move {
            let mut socket = accept(&listener).await;
            authorize(&mut socket).await;
            std::future::pending::<()>().await;
        });
        (connect(&client, RealtimeConfig::default()).await, server)
    });
    let session = connection.session();
    drop(runtime);
    let observer = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap_or_else(|e| panic!("observer runtime: {e}"));
    observer.block_on(async {
        session.wait_ended().await;
        assert!(server.await.is_err());
        assert!(matches!(
            event(&mut connection).await.payload(),
            RealtimeEventPayload::GenerationEnded(Err(RealtimeError::ActorTaskFailed))
        ));
        assert_eq!(
            connection.shutdown().await,
            Err(RealtimeError::ActorTaskFailed)
        );
    });
}

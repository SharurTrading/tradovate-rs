// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Ordered loss and finite retirement through the public socket API.

#[path = "support/realtime.rs"]
mod support;

use support::*;
use tradovate_client::realtime::{
    ChartPacket, MarketDataChannel, RealtimeConfig, RealtimeEventPayload, ResyncReason,
};

#[tokio::test]
async fn retirement_finishes_received_suffix_and_known_reply_before_terminal_evidence() {
    const SUFFIX: i64 = 1_024;
    let (client, listener) = fixture().await;
    let server = tokio::spawn(async move {
        let mut socket = accept(&listener).await;
        authorize(&mut socket).await;
        assert!(
            text(&mut socket)
                .await
                .starts_with("md/subscribeQuote\n2\n")
        );
        let charts = (1..=SUFFIX)
            .map(|id| format!(r#"{{"e":"chart","d":{{"charts":[{{"id":{id},"eoh":true}}]}}}}"#))
            .collect::<Vec<_>>()
            .join(",");
        send(
            &mut socket,
            &format!(r#"a[{{"e":"md","d":null}},{charts},{{"i":2,"s":200}}]"#),
        )
        .await;
        closed(&mut socket).await;
    });
    let mut connection = connect(&client, RealtimeConfig::default()).await;
    let session = connection.session();
    let request = tokio::spawn(async move {
        session
            .subscribe_market_data(MarketDataChannel::Quotes, &contract(42))
            .await
    });
    let loss = event(&mut connection).await;
    assert!(
        matches!(loss.payload(), RealtimeEventPayload::ContinuityGap(gap) if gap.reason() == ResyncReason::MalformedRecord)
    );
    assert!(
        connection.backlog().depth()
            < usize::try_from(SUFFIX).unwrap_or_else(|error| panic!("suffix count: {error}"))
    );
    assert!(connection.retire().await.is_ok());
    assert!(
        request
            .await
            .unwrap_or_else(|error| panic!("request task: {error}"))
            .is_ok()
    );
    for expected in 1..=SUFFIX {
        let payload = event(&mut connection).await.into_payload();
        assert!(matches!(payload, RealtimeEventPayload::Chart(chart)
            if matches!(&chart.packets()[0], ChartPacket::EndOfHistory(id) if id.get() == expected)));
    }
    assert!(matches!(
        event(&mut connection).await.payload(),
        RealtimeEventPayload::GenerationEnded(Ok(()))
    ));
    assert!(connection.recv_event().await.is_none());
    assert!(server.await.is_ok());
}

#[tokio::test]
async fn unsupported_notification_and_loss_precede_the_valid_suffix() {
    let (client, listener) = fixture().await;
    let server = tokio::spawn(async move {
        let mut socket = accept(&listener).await;
        authorize(&mut socket).await;
        let _request = text(&mut socket).await;
        send(&mut socket, r#"a[{"e":"future-kind","d":{}},{"e":"chart","d":{"charts":[{"id":7,"eoh":true}]}},{"i":2,"s":200}]"#).await;
        closed(&mut socket).await;
    });
    let mut connection = connect(&client, RealtimeConfig::default()).await;
    assert!(
        connection
            .subscribe_market_data(MarketDataChannel::Quotes, &contract(42))
            .await
            .is_ok()
    );
    assert!(
        matches!(event(&mut connection).await.payload(), RealtimeEventPayload::Unsupported(code) if code.as_str() == "future-kind")
    );
    assert!(
        matches!(event(&mut connection).await.payload(), RealtimeEventPayload::ContinuityGap(gap) if gap.reason() == ResyncReason::UnsupportedEvent)
    );
    assert!(
        matches!(event(&mut connection).await.payload(), RealtimeEventPayload::Chart(chart)
        if matches!(&chart.packets()[0], ChartPacket::EndOfHistory(id) if id.get() == 7))
    );
    assert!(connection.shutdown().await.is_ok());
    assert!(server.await.is_ok());
}

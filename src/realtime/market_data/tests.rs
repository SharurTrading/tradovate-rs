// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use std::str::FromStr;

use serde_json::value::RawValue;

use super::*;
use crate::{Decimal, realtime::RealtimePayloadError};

#[test]
fn quote_depth_and_histogram_decode_without_float_round_trip() {
    let raw = raw(r#"{
          "quotes":[{"timestamp":"2026-08-21T00:00:00Z","contractId":42,
            "entries":{"Trade":{"price":1.234567890123456789e2,"size":2.5},"FutureField":{"size":3}}}],
          "doms":[{"contractId":42,"timestamp":"2026-08-21T00:00:00Z",
            "bids":[{"price":123.5,"size":1},{"price":123.25,"size":2}],
            "offers":[{"price":123.75,"size":3},{"price":124,"size":4}]}],
          "histograms":[{"contractId":42,"timestamp":"2026-08-21T00:00:00Z",
            "tradeDate":{"year":2026,"month":8,"day":21},"base":123.25,
            "items":{"-2":10.125,"3":7},"refresh":true}]
        }"#);
    let decoded = decode::event(Some(&raw));
    let Ok(event) = decoded else {
        panic!("current market-data fixture must decode");
    };

    assert_eq!(event.quotes().len(), 1);
    assert_eq!(event.depth().len(), 1);
    assert_eq!(event.histograms().len(), 1);
    let expected = Decimal::from_str("123.4567890123456789");
    assert!(matches!(
        (event.quotes()[0].entries()[1].price(), expected),
        (Some(actual), Ok(expected)) if actual == &expected
    ));
    assert!(matches!(
        event.quotes()[0].entries()[0].kind(),
        QuoteEntryKind::Unknown(code) if code.as_str() == "FutureField"
    ));
    assert!(event.histograms()[0].is_refresh());
    assert_eq!(event.histograms()[0].items().count(), 2);
}

#[test]
fn unordered_full_depth_packet_is_rejected() {
    let raw = raw(
        r#"{"doms":[{"contractId":42,"timestamp":"2026-08-21T00:00:00Z",
        "bids":[{"price":100,"size":1},{"price":101,"size":1}],"offers":[]}]}"#,
    );

    assert!(matches!(
        decode::event(Some(&raw)),
        Err(RealtimeError::InvalidEvent {
            reason: RealtimePayloadError::InvalidOrder,
            ..
        })
    ));
}

#[test]
fn quote_entry_without_price_or_size_is_rejected() {
    let raw = raw(
        r#"{"quotes":[{"timestamp":"2026-08-21T00:00:00Z","contractId":42,
        "entries":{"Trade":{}}}]}"#,
    );
    assert!(matches!(
        decode::event(Some(&raw)),
        Err(RealtimeError::InvalidEvent {
            reason: RealtimePayloadError::MissingData,
            ..
        })
    ));
}

#[test]
fn quote_entries_above_the_former_fixed_cap_decode_fully() {
    let entries = (0..200)
        .map(|index| format!(r#""Future{index}":{{"size":1}}"#))
        .collect::<Vec<_>>()
        .join(",");
    let payload = format!(
        r#"{{"quotes":[{{"timestamp":"2026-08-21T00:00:00Z","contractId":42,"entries":{{{entries}}}}}]}}"#
    );
    let raw = raw(&payload);

    let decoded = decode::event(Some(&raw));
    let Ok(event) = decoded else {
        panic!("a 200-entry quote exceeds every former fixed cap and must decode");
    };
    assert_eq!(event.quotes()[0].entries().len(), 200);
}

#[test]
fn configured_quote_entry_bound_is_a_typed_limit_failure() {
    let raw = raw(
        r#"{"quotes":[{"timestamp":"2026-08-21T00:00:00Z","contractId":42,
        "entries":{"a":{"size":1},"b":{"size":1},"c":{"size":1}}}]}"#,
    );

    let decoded = crate::realtime::bounded::with_limit(2, || decode::event(Some(&raw)));
    assert!(matches!(
        decoded,
        Err(RealtimeError::InvalidEvent {
            reason: RealtimePayloadError::LimitExceeded,
            ..
        })
    ));
}

fn raw(value: &str) -> Box<RawValue> {
    let result = serde_json::from_str(value);
    let Ok(raw) = result else {
        panic!("test JSON must be valid");
    };
    raw
}

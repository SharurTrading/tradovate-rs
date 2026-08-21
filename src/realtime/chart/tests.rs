// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

use std::str::FromStr;

use jiff::Timestamp;
use serde_json::value::RawValue;

use super::*;
use crate::{Decimal, Symbol, realtime::RealtimePayloadError};

#[test]
fn chart_builder_enforces_tick_and_time_range_contracts() {
    let symbol = Symbol::new("ESZ6");
    let Ok(symbol) = symbol else {
        panic!("fixture symbol must validate");
    };
    assert!(
        ChartRequest::for_symbol(
            symbol.clone(),
            ChartUnderlyingType::Tick,
            2,
            ChartElementUnit::UnderlyingUnits,
        )
        .as_much_as_elements(100)
        .build()
        .is_err()
    );
    assert!(
        ChartRequest::for_symbol(
            symbol.clone(),
            ChartUnderlyingType::Tick,
            1,
            ChartElementUnit::UnderlyingUnits,
        )
        .build()
        .is_err()
    );
    let request = ChartRequest::for_symbol(
        symbol,
        ChartUnderlyingType::Tick,
        1,
        ChartElementUnit::UnderlyingUnits,
    )
    .as_much_as_elements(100)
    .build();
    let Ok(request) = request else {
        panic!("valid tick request must build");
    };
    let encoded = request::encode(&request);
    assert!(matches!(
        encoded.as_deref(),
        Ok(value)
            if value.contains(r#""underlyingType":"Tick""#)
                && value.contains(r#""asMuchAsElements":100"#)
    ));
}

#[test]
fn closest_tick_id_is_validated_typed_and_encoded_as_an_exact_integer() {
    assert!(TickId::new(0).is_err());
    assert!(TickId::new(-1).is_err());

    let tick_id = TickId::new(11_768_401);
    let symbol = Symbol::new("ESZ6");
    let (Ok(tick_id), Ok(symbol)) = (tick_id, symbol) else {
        panic!("fixture values must validate");
    };
    let request = ChartRequest::for_symbol(
        symbol,
        ChartUnderlyingType::Tick,
        1,
        ChartElementUnit::UnderlyingUnits,
    )
    .closest_tick_id(tick_id)
    .build();
    let Ok(request) = request else {
        panic!("typed tick boundary must build");
    };

    assert_eq!(request.time_range().closest_tick_id(), Some(tick_id));
    let encoded = request::encode(&request);
    assert!(matches!(
        encoded.as_deref(),
        Ok(value) if value.contains(r#""closestTickId":11768401"#)
    ));
}

#[test]
fn compact_tick_packet_reconstructs_exact_values_and_preserves_order() {
    let raw = raw(r#"{"charts":[
          {"id":31,"s":"db","td":20260821,"bp":11917,"bt":1563421179735,"ts":0.25,
           "tks":[{"t":2,"p":1,"s":3,"b":-1,"a":0,"bs":0,"as":0,"id":11768401},
                  {"t":1,"p":0,"s":2,"id":11768402}]},
          {"id":31,"eoh":true}
        ]}"#);
    let decoded = decode::event(Some(&raw));
    let Ok(event) = decoded else {
        panic!("current tick packet must decode");
    };
    let [ChartPacket::Ticks(packet), ChartPacket::EndOfHistory(id)] = event.packets() else {
        panic!("fixture must preserve tick then end-of-history packet order");
    };
    assert_eq!(id.get(), 31);
    assert_eq!(packet.ticks().len(), 2);
    assert!(packet.ticks()[0].timestamp() > packet.ticks()[1].timestamp());
    let expected_price = Decimal::from_str("2979.5");
    assert!(matches!(expected_price, Ok(value) if packet.ticks()[0].price() == &value));
    assert_eq!(packet.ticks()[0].bid_size(), Some(&Decimal::ZERO));
    assert_eq!(packet.ticks()[0].ask_size(), Some(&Decimal::ZERO));
}

#[test]
fn regular_bar_packet_is_exact_and_typed() {
    let raw = raw(r#"{"charts":[{"id":9,"td":20260821,"bars":[{
          "timestamp":"2026-08-21T00:00:00Z","open":1.1,"high":2.2,"low":0.5,"close":2,
          "upVolume":10.25,"downVolume":3.5,"upTicks":4,"downTicks":2,
          "bidVolume":8.125,"offerVolume":5.625}]}]}"#);
    let decoded = decode::event(Some(&raw));
    let Ok(event) = decoded else {
        panic!("current bar packet must decode");
    };
    let [ChartPacket::Bars(packet)] = event.packets() else {
        panic!("fixture must contain one bar packet");
    };
    assert_eq!(packet.trade_date().year(), 2026);
    assert_eq!(packet.bars().len(), 1);
    assert_eq!(packet.bars()[0].offer_volume().to_string(), "5.625");
}

#[test]
fn tick_overflow_and_nonpositive_tick_size_fail_closed() {
    for fixture in [
        r#"{"charts":[{"id":31,"s":"db","td":20260821,"bp":9223372036854775807,
           "bt":0,"ts":0.25,"tks":[{"t":0,"p":1,"s":1,"id":1}]}]}"#,
        r#"{"charts":[{"id":31,"s":"db","td":20260821,"bp":1,
           "bt":0,"ts":0,"tks":[{"t":0,"p":1,"s":1,"id":1}]}]}"#,
    ] {
        let raw = raw(fixture);
        assert!(matches!(
            decode::event(Some(&raw)),
            Err(RealtimeError::InvalidEvent {
                reason: RealtimePayloadError::InvalidValue,
                ..
            })
        ));
    }
}

#[test]
fn timestamp_builder_field_round_trips() {
    let timestamp = "2026-08-21T00:00:00Z".parse::<Timestamp>();
    let symbol = Symbol::new("ESZ6");
    let (Ok(timestamp), Ok(symbol)) = (timestamp, symbol) else {
        panic!("fixture values must validate");
    };
    let request = ChartRequest::for_symbol(
        symbol,
        ChartUnderlyingType::MinuteBar,
        1,
        ChartElementUnit::UnderlyingUnits,
    )
    .closest_timestamp(timestamp)
    .build();
    assert!(request.is_ok());
}

#[test]
fn oversized_chart_packet_array_is_a_typed_limit_failure() {
    let packets = (0..=super::decode::MAX_PACKETS)
        .map(|_| "{}")
        .collect::<Vec<_>>()
        .join(",");
    let payload = format!(r#"{{"charts":[{packets}]}}"#);
    let raw = raw(&payload);

    assert!(matches!(
        decode::event(Some(&raw)),
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

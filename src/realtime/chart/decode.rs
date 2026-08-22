// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Bounded exact decoding and checked compact-tick reconstruction.

use jiff::Timestamp;
use serde::Deserialize;
use serde_json::value::RawValue;

use super::{
    Bar, BarPacket, ChartEvent, ChartPacket, ChartSubscriptionId, Tick, TickId, TickPacket,
};
use crate::Decimal;
use crate::realtime::bounded::{self, BoundedVec, DecodeError};
use crate::realtime::market_data::TradeDate;
use crate::realtime::{ProviderCode, RealtimeError, RealtimeEventKind, RealtimePayloadError};

pub(super) const MAX_PACKETS: usize = 4_096;
const MAX_BARS_PER_PACKET: usize = 65_536;
const MAX_TICKS_PER_PACKET: usize = 65_536;

#[derive(Deserialize)]
struct WireEvent {
    charts: BoundedVec<Box<RawValue>, MAX_PACKETS>,
}

#[derive(Deserialize)]
struct PacketHeader {
    id: i64,
    #[serde(default)]
    eoh: bool,
    #[serde(default, deserialize_with = "present_raw")]
    bars: Option<Box<RawValue>>,
    #[serde(default, deserialize_with = "present_raw")]
    tks: Option<Box<RawValue>>,
}

#[derive(Deserialize)]
struct WireBarPacket {
    id: i64,
    td: i64,
    bars: BoundedVec<WireBar, MAX_BARS_PER_PACKET>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct WireBar {
    timestamp: Timestamp,
    #[serde(with = "crate::decimal")]
    open: Decimal,
    #[serde(with = "crate::decimal")]
    high: Decimal,
    #[serde(with = "crate::decimal")]
    low: Decimal,
    #[serde(with = "crate::decimal")]
    close: Decimal,
    #[serde(with = "crate::decimal")]
    up_volume: Decimal,
    #[serde(with = "crate::decimal")]
    down_volume: Decimal,
    #[serde(with = "crate::decimal")]
    up_ticks: Decimal,
    #[serde(with = "crate::decimal")]
    down_ticks: Decimal,
    #[serde(with = "crate::decimal")]
    bid_volume: Decimal,
    #[serde(with = "crate::decimal")]
    offer_volume: Decimal,
}

#[derive(Deserialize)]
struct WireTickPacket {
    id: i64,
    s: String,
    td: i64,
    bp: i64,
    bt: i64,
    #[serde(with = "crate::decimal")]
    ts: Decimal,
    tks: BoundedVec<WireTick, MAX_TICKS_PER_PACKET>,
}

#[derive(Clone, Copy, Deserialize)]
struct WireTick {
    t: i64,
    p: i64,
    #[serde(with = "crate::decimal")]
    s: Decimal,
    #[serde(default)]
    b: Option<i64>,
    #[serde(default)]
    a: Option<i64>,
    #[serde(default, with = "crate::decimal::option")]
    bs: Option<Decimal>,
    #[serde(default, rename = "as", with = "crate::decimal::option")]
    ask_size: Option<Decimal>,
    id: i64,
}

pub(crate) fn event(data: Option<&RawValue>) -> Result<ChartEvent, RealtimeError> {
    let data = data.ok_or_else(|| invalid(RealtimePayloadError::MissingData))?;
    let wire = bounded::from_str::<WireEvent>(data.get()).map_err(decode_error)?;
    if wire.charts.is_empty() {
        return Err(invalid(RealtimePayloadError::MissingData));
    }
    let packets = wire
        .charts
        .into_vec()
        .into_iter()
        .map(|packet| decode_packet(&packet))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(ChartEvent {
        packets: packets.into_boxed_slice(),
    })
}

fn decode_packet(raw: &RawValue) -> Result<ChartPacket, RealtimeError> {
    let header = bounded::from_str::<PacketHeader>(raw.get()).map_err(decode_error)?;
    let subscription_id = ChartSubscriptionId::from_wire(header.id)
        .map_err(|_| invalid(RealtimePayloadError::InvalidValue))?;
    match (header.eoh, header.bars.is_some(), header.tks.is_some()) {
        (true, false, false) => Ok(ChartPacket::EndOfHistory(subscription_id)),
        (false, true, false) => bar_packet(raw).map(ChartPacket::Bars),
        (false, false, true) => tick_packet(raw).map(ChartPacket::Ticks),
        _ => Err(invalid(RealtimePayloadError::Malformed)),
    }
}

fn bar_packet(raw: &RawValue) -> Result<BarPacket, RealtimeError> {
    let wire = bounded::from_str::<WireBarPacket>(raw.get()).map_err(decode_error)?;
    let subscription_id = ChartSubscriptionId::from_wire(wire.id)
        .map_err(|_| invalid(RealtimePayloadError::InvalidValue))?;
    let bars = wire
        .bars
        .into_vec()
        .into_iter()
        .map(|bar| Bar {
            timestamp: bar.timestamp,
            open: bar.open,
            high: bar.high,
            low: bar.low,
            close: bar.close,
            up_volume: bar.up_volume,
            down_volume: bar.down_volume,
            up_ticks: bar.up_ticks,
            down_ticks: bar.down_ticks,
            bid_volume: bar.bid_volume,
            offer_volume: bar.offer_volume,
        })
        .collect::<Vec<_>>();
    Ok(BarPacket {
        subscription_id,
        trade_date: compact_trade_date(wire.td)?,
        bars: bars.into_boxed_slice(),
    })
}

fn tick_packet(raw: &RawValue) -> Result<TickPacket, RealtimeError> {
    let wire = bounded::from_str::<WireTickPacket>(raw.get()).map_err(decode_error)?;
    if wire.ts <= Decimal::ZERO {
        return Err(invalid(RealtimePayloadError::InvalidValue));
    }
    let subscription_id = ChartSubscriptionId::from_wire(wire.id)
        .map_err(|_| invalid(RealtimePayloadError::InvalidValue))?;
    let source = ProviderCode::from_wire(wire.s)
        .ok_or_else(|| invalid(RealtimePayloadError::InvalidValue))?;
    let ticks = wire
        .tks
        .into_vec()
        .into_iter()
        .map(|tick| reconstruct_tick(wire.bp, wire.bt, wire.ts, tick))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(TickPacket {
        subscription_id,
        source,
        trade_date: compact_trade_date(wire.td)?,
        tick_size: wire.ts,
        ticks: ticks.into_boxed_slice(),
    })
}

fn reconstruct_tick(
    base_price: i64,
    base_timestamp: i64,
    tick_size: Decimal,
    wire: WireTick,
) -> Result<Tick, RealtimeError> {
    let timestamp_millis = base_timestamp
        .checked_add(wire.t)
        .ok_or_else(|| invalid(RealtimePayloadError::InvalidValue))?;
    let timestamp = Timestamp::from_millisecond(timestamp_millis)
        .map_err(|_| invalid(RealtimePayloadError::InvalidValue))?;
    let price = exact_price(base_price, wire.p, tick_size)?;
    let bid_price = wire
        .b
        .map(|offset| exact_price(base_price, offset, tick_size))
        .transpose()?;
    let ask_price = wire
        .a
        .map(|offset| exact_price(base_price, offset, tick_size))
        .transpose()?;
    let id =
        TickId::from_wire(wire.id).ok_or_else(|| invalid(RealtimePayloadError::InvalidValue))?;
    Ok(Tick {
        id,
        timestamp,
        price,
        size: wire.s,
        bid_price,
        bid_size: wire.bs,
        ask_price,
        ask_size: wire.ask_size,
    })
}

fn exact_price(base: i64, relative: i64, tick_size: Decimal) -> Result<Decimal, RealtimeError> {
    let ticks = base
        .checked_add(relative)
        .ok_or_else(|| invalid(RealtimePayloadError::InvalidValue))?;
    Decimal::from(ticks)
        .checked_mul(tick_size)
        .ok_or_else(|| invalid(RealtimePayloadError::InvalidValue))
}

fn compact_trade_date(value: i64) -> Result<TradeDate, RealtimeError> {
    if value <= 0 {
        return Err(invalid(RealtimePayloadError::InvalidValue));
    }
    let year =
        i16::try_from(value / 10_000).map_err(|_| invalid(RealtimePayloadError::InvalidValue))?;
    let month = u8::try_from((value / 100) % 100)
        .map_err(|_| invalid(RealtimePayloadError::InvalidValue))?;
    let day = u8::try_from(value % 100).map_err(|_| invalid(RealtimePayloadError::InvalidValue))?;
    validate_date(year, month, day)?;
    Ok(TradeDate::from_parts(year, month, day))
}

fn validate_date(year: i16, month: u8, day: u8) -> Result<(), RealtimeError> {
    let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let maximum = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if leap => 29,
        2 => 28,
        _ => return Err(invalid(RealtimePayloadError::InvalidValue)),
    };
    if day == 0 || day > maximum {
        return Err(invalid(RealtimePayloadError::InvalidValue));
    }
    Ok(())
}

fn present_raw<'de, D>(deserializer: D) -> Result<Option<Box<RawValue>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Box::<RawValue>::deserialize(deserializer).map(Some)
}

fn decode_error(error: DecodeError) -> RealtimeError {
    let reason = match error {
        DecodeError::LimitExceeded => RealtimePayloadError::LimitExceeded,
        DecodeError::Malformed => RealtimePayloadError::Malformed,
    };
    invalid(reason)
}

fn invalid(reason: RealtimePayloadError) -> RealtimeError {
    RealtimeError::InvalidEvent {
        kind: RealtimeEventKind::Chart,
        reason,
    }
}

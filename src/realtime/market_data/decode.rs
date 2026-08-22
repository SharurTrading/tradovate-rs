// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Bounded exact decoding for current Partner market-data payloads.

use std::collections::BTreeMap;

use jiff::Timestamp;
use serde::Deserialize;
use serde_json::value::RawValue;

use super::{
    DepthLevel, DepthOfMarket, Histogram, MarketDataEvent, Quote, QuoteEntry, QuoteEntryKind,
    TradeDate,
};
use crate::realtime::bounded::{self, BoundedMap, BoundedVec, DecodeError};
use crate::realtime::{ProviderCode, RealtimeError, RealtimeEventKind, RealtimePayloadError};
use crate::{ContractId, Decimal};

const MAX_QUOTES: usize = 4_096;
pub(super) const MAX_QUOTE_ENTRIES: usize = 64;
const MAX_DOM_PACKETS: usize = 4_096;
const MAX_DEPTH_LEVELS: usize = 16_384;
const MAX_HISTOGRAMS: usize = 4_096;
const MAX_HISTOGRAM_ITEMS: usize = 65_536;

#[derive(Deserialize)]
struct WireEvent {
    #[serde(default)]
    quotes: BoundedVec<WireQuote, MAX_QUOTES>,
    #[serde(default)]
    doms: BoundedVec<WireDepth, MAX_DOM_PACKETS>,
    #[serde(default)]
    histograms: BoundedVec<WireHistogram, MAX_HISTOGRAMS>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct WireQuote {
    timestamp: Timestamp,
    contract_id: ContractId,
    entries: BoundedMap<String, WireQuoteEntry, MAX_QUOTE_ENTRIES>,
}

#[derive(Deserialize)]
struct WireQuoteEntry {
    #[serde(default, with = "crate::decimal::option")]
    price: Option<Decimal>,
    #[serde(default, with = "crate::decimal::option")]
    size: Option<Decimal>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct WireDepth {
    contract_id: ContractId,
    timestamp: Timestamp,
    bids: BoundedVec<WireDepthLevel, MAX_DEPTH_LEVELS>,
    offers: BoundedVec<WireDepthLevel, MAX_DEPTH_LEVELS>,
}

#[derive(Clone, Copy, Deserialize)]
struct WireDepthLevel {
    #[serde(with = "crate::decimal")]
    price: Decimal,
    #[serde(with = "crate::decimal")]
    size: Decimal,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct WireHistogram {
    contract_id: ContractId,
    timestamp: Timestamp,
    trade_date: WireTradeDate,
    #[serde(with = "crate::decimal")]
    base: Decimal,
    items: BoundedMap<String, ExactDecimal, MAX_HISTOGRAM_ITEMS>,
    refresh: bool,
}

#[derive(Clone, Copy, Deserialize)]
struct WireTradeDate {
    year: i16,
    month: u8,
    day: u8,
}

#[derive(Deserialize)]
#[serde(transparent)]
struct ExactDecimal(#[serde(with = "crate::decimal")] Decimal);

pub(crate) fn event(data: Option<&RawValue>) -> Result<MarketDataEvent, RealtimeError> {
    let data = data.ok_or_else(|| invalid(RealtimePayloadError::MissingData))?;
    let wire = bounded::from_str::<WireEvent>(data.get()).map_err(decode_error)?;
    if wire.quotes.is_empty() && wire.doms.is_empty() && wire.histograms.is_empty() {
        return Err(invalid(RealtimePayloadError::MissingData));
    }

    let quotes = wire
        .quotes
        .into_vec()
        .into_iter()
        .map(quote)
        .collect::<Result<Vec<_>, _>>()?;
    let depth = wire
        .doms
        .into_vec()
        .into_iter()
        .map(depth)
        .collect::<Result<Vec<_>, _>>()?;
    let histograms = wire
        .histograms
        .into_vec()
        .into_iter()
        .map(histogram)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(MarketDataEvent {
        quotes: quotes.into_boxed_slice(),
        depth: depth.into_boxed_slice(),
        histograms: histograms.into_boxed_slice(),
    })
}

fn quote(wire: WireQuote) -> Result<Quote, RealtimeError> {
    if wire.entries.is_empty() {
        return Err(invalid(RealtimePayloadError::MissingData));
    }
    let entries = wire
        .entries
        .into_map()
        .into_iter()
        .map(|(kind, value)| {
            if value.price.is_none() && value.size.is_none() {
                return Err(invalid(RealtimePayloadError::MissingData));
            }
            Ok(QuoteEntry {
                kind: quote_kind(kind)?,
                price: value.price,
                size: value.size,
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Quote {
        timestamp: wire.timestamp,
        contract_id: wire.contract_id,
        entries: entries.into_boxed_slice(),
    })
}

fn quote_kind(value: String) -> Result<QuoteEntryKind, RealtimeError> {
    Ok(match value.as_str() {
        "Bid" => QuoteEntryKind::Bid,
        "Offer" => QuoteEntryKind::Offer,
        "Trade" => QuoteEntryKind::Trade,
        "TotalTradeVolume" => QuoteEntryKind::TotalTradeVolume,
        "OpenInterest" => QuoteEntryKind::OpenInterest,
        "OpeningPrice" => QuoteEntryKind::OpeningPrice,
        "HighPrice" => QuoteEntryKind::HighPrice,
        "LowPrice" => QuoteEntryKind::LowPrice,
        "SettlementPrice" => QuoteEntryKind::SettlementPrice,
        _ => QuoteEntryKind::Unknown(
            ProviderCode::from_wire(value)
                .ok_or_else(|| invalid(RealtimePayloadError::InvalidValue))?,
        ),
    })
}

fn depth(wire: WireDepth) -> Result<DepthOfMarket, RealtimeError> {
    let bids = wire
        .bids
        .into_vec()
        .into_iter()
        .map(level)
        .collect::<Vec<_>>();
    let offers = wire
        .offers
        .into_vec()
        .into_iter()
        .map(level)
        .collect::<Vec<_>>();
    if !bids.windows(2).all(|pair| pair[0].price >= pair[1].price)
        || !offers.windows(2).all(|pair| pair[0].price <= pair[1].price)
    {
        return Err(invalid(RealtimePayloadError::InvalidOrder));
    }
    Ok(DepthOfMarket {
        contract_id: wire.contract_id,
        timestamp: wire.timestamp,
        bids: bids.into_boxed_slice(),
        offers: offers.into_boxed_slice(),
    })
}

fn level(wire: WireDepthLevel) -> DepthLevel {
    DepthLevel {
        price: wire.price,
        size: wire.size,
    }
}

fn histogram(wire: WireHistogram) -> Result<Histogram, RealtimeError> {
    let trade_date = trade_date(wire.trade_date)?;
    let items = wire
        .items
        .into_map()
        .into_iter()
        .map(|(offset, value)| {
            offset
                .parse::<i64>()
                .map(|offset| (offset, value.0))
                .map_err(|_| invalid(RealtimePayloadError::InvalidValue))
        })
        .collect::<Result<BTreeMap<_, _>, _>>()?;
    Ok(Histogram {
        contract_id: wire.contract_id,
        timestamp: wire.timestamp,
        trade_date,
        base: wire.base,
        items,
        refresh: wire.refresh,
    })
}

fn trade_date(wire: WireTradeDate) -> Result<TradeDate, RealtimeError> {
    let maximum = days_in_month(wire.year, wire.month)
        .ok_or_else(|| invalid(RealtimePayloadError::InvalidValue))?;
    if wire.day == 0 || wire.day > maximum {
        return Err(invalid(RealtimePayloadError::InvalidValue));
    }
    Ok(TradeDate {
        year: wire.year,
        month: wire.month,
        day: wire.day,
    })
}

fn days_in_month(year: i16, month: u8) -> Option<u8> {
    let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => Some(31),
        4 | 6 | 9 | 11 => Some(30),
        2 if leap => Some(29),
        2 => Some(28),
        _ => None,
    }
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
        kind: RealtimeEventKind::MarketData,
        reason,
    }
}

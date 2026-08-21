// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Typed market-data commands and payloads.
//!
//! Contract reviewed 2026-08-21 against the current Partner market-data
//! request reference:
//! <https://partner.tradovate.com/overview/core-concepts/web-sockets/market-data/market-data-request-reference>.

mod data;
pub(super) mod decode;

use serde::Serialize;

use super::{RealtimeConnection, RealtimeError, SocketKind};
use crate::{ContractId, Symbol};

pub use data::{
    DepthLevel, DepthOfMarket, Histogram, MarketDataEvent, Quote, QuoteEntry, QuoteEntryKind,
    TradeDate,
};

/// A provider market-data stream family with symmetric commands.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum MarketDataChannel {
    /// Top-of-book quotes, trades, volume, open interest, and session prices.
    Quotes,
    /// Depth-of-market bids and offers.
    DepthOfMarket,
    /// Price-distribution histogram updates.
    Histogram,
}

impl MarketDataChannel {
    const fn subscribe_endpoint(self) -> &'static str {
        match self {
            Self::Quotes => "md/subscribeQuote",
            Self::DepthOfMarket => "md/subscribeDOM",
            Self::Histogram => "md/subscribeHistogram",
        }
    }

    const fn unsubscribe_endpoint(self) -> &'static str {
        match self {
            Self::Quotes => "md/unsubscribeQuote",
            Self::DepthOfMarket => "md/unsubscribeDOM",
            Self::Histogram => "md/unsubscribeHistogram",
        }
    }
}

/// A borrowed, explicitly typed market-data subscription identity.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum MarketDataTarget<'a> {
    /// A validated provider contract symbol.
    Symbol(&'a Symbol),
    /// A validated provider contract identifier.
    ContractId(&'a ContractId),
}

impl<'a> From<&'a Symbol> for MarketDataTarget<'a> {
    fn from(value: &'a Symbol) -> Self {
        Self::Symbol(value)
    }
}

impl<'a> From<&'a ContractId> for MarketDataTarget<'a> {
    fn from(value: &'a ContractId) -> Self {
        Self::ContractId(value)
    }
}

#[derive(Serialize)]
#[serde(untagged)]
enum WireTarget<'a> {
    Symbol(&'a str),
    ContractId(i64),
}

#[derive(Serialize)]
struct TargetRequest<'a> {
    symbol: WireTarget<'a>,
}

impl RealtimeConnection {
    /// Subscribes to one market-data family by symbol or contract ID.
    ///
    /// The connection retains no canonical subscription set. Callers keep the
    /// desired set and replay it after creating a replacement generation.
    ///
    /// # Errors
    ///
    /// Returns a socket-kind, encoding, capacity, provider, timeout, protocol,
    /// or disconnect failure. A request timeout ends the generation.
    pub async fn subscribe_market_data<'a>(
        &self,
        channel: MarketDataChannel,
        target: impl Into<MarketDataTarget<'a>>,
    ) -> Result<(), RealtimeError> {
        self.require_market_data_socket()?;
        self.send_target_request(channel.subscribe_endpoint(), target.into())
            .await
    }

    /// Unsubscribes one symbol or contract ID from a market-data family.
    ///
    /// # Errors
    ///
    /// Returns a socket-kind, encoding, capacity, provider, timeout, protocol,
    /// or disconnect failure. A request timeout ends the generation.
    pub async fn unsubscribe_market_data<'a>(
        &self,
        channel: MarketDataChannel,
        target: impl Into<MarketDataTarget<'a>>,
    ) -> Result<(), RealtimeError> {
        self.require_market_data_socket()?;
        self.send_target_request(channel.unsubscribe_endpoint(), target.into())
            .await
    }

    pub(super) fn require_market_data_socket(&self) -> Result<(), RealtimeError> {
        if matches!(self.socket_kind(), SocketKind::MarketData) {
            Ok(())
        } else {
            Err(RealtimeError::WrongSocketKind {
                expected: SocketKind::MarketData,
                actual: self.socket_kind(),
            })
        }
    }

    async fn send_target_request(
        &self,
        endpoint: &'static str,
        target: MarketDataTarget<'_>,
    ) -> Result<(), RealtimeError> {
        let symbol = match target {
            MarketDataTarget::Symbol(symbol) => WireTarget::Symbol(symbol.as_str()),
            MarketDataTarget::ContractId(contract_id) => WireTarget::ContractId(contract_id.get()),
        };
        let body = serde_json::to_string(&TargetRequest { symbol })
            .map_err(|_| RealtimeError::Protocol)?;
        self.request_non_mutating(endpoint, "", &body).await?;
        Ok(())
    }
}

#[cfg(test)]
#[path = "market_data/tests.rs"]
mod tests;

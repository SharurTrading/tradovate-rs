// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Typed market-data subscription commands.
//!
//! Contract reviewed 2026-08-21 against Tradovate's official market-data
//! request reference: <https://partner.tradovate.com/overview/core-concepts/web-sockets/market-data/market-data-request-reference>.

use serde::Serialize;

use super::{RealtimeConnection, RealtimeError, SocketKind};
use crate::Symbol;

/// A provider market-data stream family with symmetric subscribe/unsubscribe commands.
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

#[derive(Serialize)]
struct SymbolRequest<'a> {
    symbol: &'a str,
}

impl RealtimeConnection {
    /// Subscribes to one validated symbol and market-data family.
    ///
    /// The connection retains no canonical subscription set. Callers keep the
    /// desired set and replay it explicitly after creating a new generation.
    ///
    /// # Errors
    ///
    /// Returns a socket-kind, encoding, capacity, provider, timeout, protocol,
    /// or disconnect failure. A request that times out ends the generation.
    pub async fn subscribe_market_data(
        &self,
        channel: MarketDataChannel,
        symbol: &Symbol,
    ) -> Result<(), RealtimeError> {
        self.require_market_data_socket()?;
        self.send_symbol_request(channel.subscribe_endpoint(), symbol)
            .await
    }

    /// Unsubscribes one validated symbol from a market-data family.
    ///
    /// # Errors
    ///
    /// Returns a socket-kind, encoding, capacity, provider, timeout, protocol,
    /// or disconnect failure. A request that times out ends the generation.
    pub async fn unsubscribe_market_data(
        &self,
        channel: MarketDataChannel,
        symbol: &Symbol,
    ) -> Result<(), RealtimeError> {
        self.require_market_data_socket()?;
        self.send_symbol_request(channel.unsubscribe_endpoint(), symbol)
            .await
    }

    fn require_market_data_socket(&self) -> Result<(), RealtimeError> {
        if matches!(self.socket_kind(), SocketKind::MarketData) {
            Ok(())
        } else {
            Err(RealtimeError::WrongSocketKind {
                expected: SocketKind::MarketData,
                actual: self.socket_kind(),
            })
        }
    }

    async fn send_symbol_request(
        &self,
        endpoint: &'static str,
        symbol: &Symbol,
    ) -> Result<(), RealtimeError> {
        let body = serde_json::to_string(&SymbolRequest {
            symbol: symbol.as_str(),
        })
        .map_err(|_| RealtimeError::Protocol)?;
        self.request_non_mutating(endpoint, "", &body).await?;
        Ok(())
    }
}

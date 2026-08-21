// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Exact typed quote, depth, and histogram values.

use std::collections::BTreeMap;

use jiff::Timestamp;

use crate::{ContractId, Decimal};

use super::super::ProviderCode;

/// A calendar trade date supplied with a histogram or chart packet.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TradeDate {
    pub(super) year: i16,
    pub(super) month: u8,
    pub(super) day: u8,
}

impl TradeDate {
    pub(crate) const fn from_parts(year: i16, month: u8, day: u8) -> Self {
        Self { year, month, day }
    }

    /// Returns the year.
    #[must_use]
    pub const fn year(self) -> i16 {
        self.year
    }

    /// Returns the month in `1..=12`.
    #[must_use]
    pub const fn month(self) -> u8 {
        self.month
    }

    /// Returns the day of month.
    #[must_use]
    pub const fn day(self) -> u8 {
        self.day
    }
}

/// A documented or forward-compatible quote entry kind.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum QuoteEntryKind {
    /// Best bid.
    Bid,
    /// Best offer.
    Offer,
    /// Last trade.
    Trade,
    /// Cumulative traded volume.
    TotalTradeVolume,
    /// Open interest.
    OpenInterest,
    /// Session opening price.
    OpeningPrice,
    /// Session high price.
    HighPrice,
    /// Session low price.
    LowPrice,
    /// Settlement price.
    SettlementPrice,
    /// A bounded future provider entry name.
    Unknown(ProviderCode),
}

/// One quote field carrying price, size, or both.
#[derive(Clone, Debug, PartialEq)]
pub struct QuoteEntry {
    pub(super) kind: QuoteEntryKind,
    pub(super) price: Option<Decimal>,
    pub(super) size: Option<Decimal>,
}

impl QuoteEntry {
    /// Returns the entry's provider semantic.
    #[must_use]
    pub const fn kind(&self) -> &QuoteEntryKind {
        &self.kind
    }

    /// Returns the exact price when present.
    #[must_use]
    pub const fn price(&self) -> Option<&Decimal> {
        self.price.as_ref()
    }

    /// Returns the exact size when present.
    #[must_use]
    pub const fn size(&self) -> Option<&Decimal> {
        self.size.as_ref()
    }
}

/// One quote update for a validated contract.
#[derive(Clone, Debug, PartialEq)]
pub struct Quote {
    pub(super) timestamp: Timestamp,
    pub(super) contract_id: ContractId,
    pub(super) entries: Box<[QuoteEntry]>,
}

impl Quote {
    /// Returns the provider timestamp.
    #[must_use]
    pub const fn timestamp(&self) -> &Timestamp {
        &self.timestamp
    }

    /// Returns the contract identity.
    #[must_use]
    pub const fn contract_id(&self) -> ContractId {
        self.contract_id
    }

    /// Returns the bounded quote entries.
    #[must_use]
    pub const fn entries(&self) -> &[QuoteEntry] {
        &self.entries
    }
}

/// One exact price/size depth level.
#[derive(Clone, Debug, PartialEq)]
pub struct DepthLevel {
    pub(super) price: Decimal,
    pub(super) size: Decimal,
}

impl DepthLevel {
    /// Returns the exact price.
    #[must_use]
    pub const fn price(&self) -> &Decimal {
        &self.price
    }

    /// Returns the exact size.
    #[must_use]
    pub const fn size(&self) -> &Decimal {
        &self.size
    }
}

/// A full documented depth packet with ordered sides.
#[derive(Clone, Debug, PartialEq)]
pub struct DepthOfMarket {
    pub(super) contract_id: ContractId,
    pub(super) timestamp: Timestamp,
    pub(super) bids: Box<[DepthLevel]>,
    pub(super) offers: Box<[DepthLevel]>,
}

impl DepthOfMarket {
    /// Returns the contract identity.
    #[must_use]
    pub const fn contract_id(&self) -> ContractId {
        self.contract_id
    }

    /// Returns the provider timestamp.
    #[must_use]
    pub const fn timestamp(&self) -> &Timestamp {
        &self.timestamp
    }

    /// Returns bids in provider-documented descending price order.
    #[must_use]
    pub const fn bids(&self) -> &[DepthLevel] {
        &self.bids
    }

    /// Returns offers in provider-documented ascending price order.
    #[must_use]
    pub const fn offers(&self) -> &[DepthLevel] {
        &self.offers
    }
}

/// An exact price-distribution histogram update.
#[derive(Clone, Debug, PartialEq)]
pub struct Histogram {
    pub(super) contract_id: ContractId,
    pub(super) timestamp: Timestamp,
    pub(super) trade_date: TradeDate,
    pub(super) base: Decimal,
    pub(super) items: BTreeMap<i64, Decimal>,
    pub(super) refresh: bool,
}

impl Histogram {
    /// Returns the contract identity.
    #[must_use]
    pub const fn contract_id(&self) -> ContractId {
        self.contract_id
    }

    /// Returns the provider timestamp.
    #[must_use]
    pub const fn timestamp(&self) -> &Timestamp {
        &self.timestamp
    }

    /// Returns the histogram trade date.
    #[must_use]
    pub const fn trade_date(&self) -> TradeDate {
        self.trade_date
    }

    /// Returns the exact base price.
    #[must_use]
    pub const fn base(&self) -> &Decimal {
        &self.base
    }

    /// Iterates signed provider buckets and their exact values.
    #[must_use]
    pub fn items(&self) -> impl ExactSizeIterator<Item = (i64, &Decimal)> {
        self.items.iter().map(|(offset, value)| (*offset, value))
    }

    /// Returns whether this packet replaces the prior histogram.
    #[must_use]
    pub const fn is_refresh(&self) -> bool {
        self.refresh
    }
}

/// A bounded market-data event; at least one collection is nonempty.
#[derive(Clone, Debug, PartialEq)]
pub struct MarketDataEvent {
    pub(super) quotes: Box<[Quote]>,
    pub(super) depth: Box<[DepthOfMarket]>,
    pub(super) histograms: Box<[Histogram]>,
}

impl MarketDataEvent {
    /// Returns quote updates in provider order.
    #[must_use]
    pub const fn quotes(&self) -> &[Quote] {
        &self.quotes
    }

    /// Returns full depth packets in provider order.
    #[must_use]
    pub const fn depth(&self) -> &[DepthOfMarket] {
        &self.depth
    }

    /// Returns histogram updates in provider order.
    #[must_use]
    pub const fn histograms(&self) -> &[Histogram] {
        &self.histograms
    }
}

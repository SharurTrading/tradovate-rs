// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Public exact chart packet values.

use jiff::Timestamp;

use crate::Decimal;
use crate::realtime::{ProviderCode, RealtimeError};

use super::super::market_data::TradeDate;

macro_rules! chart_id {
    ($name:ident, $doc:literal) => {
        #[doc = $doc]
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(ChartSubscriptionId);

        impl $name {
            pub(super) fn from_wire(value: i64) -> Result<Self, RealtimeError> {
                ChartSubscriptionId::from_wire(value).map(Self)
            }

            /// Returns the provider integer.
            #[must_use]
            pub const fn get(self) -> i64 {
                self.0.get()
            }

            /// Returns the common packet subscription identity.
            #[must_use]
            pub const fn subscription_id(self) -> ChartSubscriptionId {
                self.0
            }
        }
    };
}

/// A chart stream identifier carried by data packets.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ChartSubscriptionId(i64);

impl ChartSubscriptionId {
    pub(super) fn from_wire(value: i64) -> Result<Self, RealtimeError> {
        if value > 0 {
            Ok(Self(value))
        } else {
            Err(RealtimeError::InvalidTypedResponse {
                operation: "chart subscription ID",
            })
        }
    }

    /// Returns the provider integer.
    #[must_use]
    pub const fn get(self) -> i64 {
        self.0
    }
}

chart_id!(HistoricalChartId, "A historical chart stream identifier.");
chart_id!(RealtimeChartId, "A realtime chart stream identifier.");

/// A provider tick identifier.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TickId(i64);

impl TickId {
    /// Creates a provider tick identifier from its positive wire integer.
    ///
    /// # Errors
    ///
    /// Returns [`RealtimeError::InvalidRequest`] when `value` is not positive.
    pub const fn new(value: i64) -> Result<Self, RealtimeError> {
        if value > 0 {
            Ok(Self(value))
        } else {
            Err(RealtimeError::InvalidRequest {
                field: "tick_id",
                reason: "must be positive",
            })
        }
    }

    pub(super) const fn from_wire(value: i64) -> Option<Self> {
        match Self::new(value) {
            Ok(id) => Some(id),
            Err(_) => None,
        }
    }

    /// Returns the provider integer.
    #[must_use]
    pub const fn get(self) -> i64 {
        self.0
    }
}

/// One exact regular chart bar.
#[derive(Clone, Debug, PartialEq)]
pub struct Bar {
    pub(super) timestamp: Timestamp,
    pub(super) open: Decimal,
    pub(super) high: Decimal,
    pub(super) low: Decimal,
    pub(super) close: Decimal,
    pub(super) up_volume: Decimal,
    pub(super) down_volume: Decimal,
    pub(super) up_ticks: Decimal,
    pub(super) down_ticks: Decimal,
    pub(super) bid_volume: Decimal,
    pub(super) offer_volume: Decimal,
}

impl Bar {
    /// Returns the bar timestamp.
    #[must_use]
    pub const fn timestamp(&self) -> &Timestamp {
        &self.timestamp
    }
    /// Returns the exact open.
    #[must_use]
    pub const fn open(&self) -> &Decimal {
        &self.open
    }
    /// Returns the exact high.
    #[must_use]
    pub const fn high(&self) -> &Decimal {
        &self.high
    }
    /// Returns the exact low.
    #[must_use]
    pub const fn low(&self) -> &Decimal {
        &self.low
    }
    /// Returns the exact close.
    #[must_use]
    pub const fn close(&self) -> &Decimal {
        &self.close
    }
    /// Returns the exact up volume.
    #[must_use]
    pub const fn up_volume(&self) -> &Decimal {
        &self.up_volume
    }
    /// Returns the exact down volume.
    #[must_use]
    pub const fn down_volume(&self) -> &Decimal {
        &self.down_volume
    }
    /// Returns the exact up-tick value.
    #[must_use]
    pub const fn up_ticks(&self) -> &Decimal {
        &self.up_ticks
    }
    /// Returns the exact down-tick value.
    #[must_use]
    pub const fn down_ticks(&self) -> &Decimal {
        &self.down_ticks
    }
    /// Returns exact bid volume.
    #[must_use]
    pub const fn bid_volume(&self) -> &Decimal {
        &self.bid_volume
    }
    /// Returns exact offer volume.
    #[must_use]
    pub const fn offer_volume(&self) -> &Decimal {
        &self.offer_volume
    }
}

/// One regular-bar chart packet.
#[derive(Clone, Debug, PartialEq)]
pub struct BarPacket {
    pub(super) subscription_id: ChartSubscriptionId,
    pub(super) trade_date: TradeDate,
    pub(super) bars: Box<[Bar]>,
}

impl BarPacket {
    /// Returns the source subscription.
    #[must_use]
    pub const fn subscription_id(&self) -> ChartSubscriptionId {
        self.subscription_id
    }
    /// Returns the packet trade date.
    #[must_use]
    pub const fn trade_date(&self) -> TradeDate {
        self.trade_date
    }
    /// Returns bars in provider order.
    #[must_use]
    pub const fn bars(&self) -> &[Bar] {
        &self.bars
    }
}

/// One reconstructed exact trade tick.
#[derive(Clone, Debug, PartialEq)]
pub struct Tick {
    pub(super) id: TickId,
    pub(super) timestamp: Timestamp,
    pub(super) price: Decimal,
    pub(super) size: Decimal,
    pub(super) bid_price: Option<Decimal>,
    pub(super) bid_size: Option<Decimal>,
    pub(super) ask_price: Option<Decimal>,
    pub(super) ask_size: Option<Decimal>,
}

impl Tick {
    /// Returns the provider tick identity.
    #[must_use]
    pub const fn id(&self) -> TickId {
        self.id
    }
    /// Returns the reconstructed timestamp.
    #[must_use]
    pub const fn timestamp(&self) -> &Timestamp {
        &self.timestamp
    }
    /// Returns the reconstructed exact trade price.
    #[must_use]
    pub const fn price(&self) -> &Decimal {
        &self.price
    }
    /// Returns the exact trade volume.
    #[must_use]
    pub const fn size(&self) -> &Decimal {
        &self.size
    }
    /// Returns the reconstructed exact bid price.
    #[must_use]
    pub const fn bid_price(&self) -> Option<&Decimal> {
        self.bid_price.as_ref()
    }
    /// Returns the exact bid size.
    #[must_use]
    pub const fn bid_size(&self) -> Option<&Decimal> {
        self.bid_size.as_ref()
    }
    /// Returns the reconstructed exact ask price.
    #[must_use]
    pub const fn ask_price(&self) -> Option<&Decimal> {
        self.ask_price.as_ref()
    }
    /// Returns the exact ask size.
    #[must_use]
    pub const fn ask_size(&self) -> Option<&Decimal> {
        self.ask_size.as_ref()
    }
}

/// One reconstructed compact tick packet.
#[derive(Clone, Debug, PartialEq)]
pub struct TickPacket {
    pub(super) subscription_id: ChartSubscriptionId,
    pub(super) source: ProviderCode,
    pub(super) trade_date: TradeDate,
    pub(super) tick_size: Decimal,
    pub(super) ticks: Box<[Tick]>,
}

impl TickPacket {
    /// Returns the source subscription.
    #[must_use]
    pub const fn subscription_id(&self) -> ChartSubscriptionId {
        self.subscription_id
    }
    /// Returns the bounded packet source code.
    #[must_use]
    pub const fn source(&self) -> &ProviderCode {
        &self.source
    }
    /// Returns the packet trade date.
    #[must_use]
    pub const fn trade_date(&self) -> TradeDate {
        self.trade_date
    }
    /// Returns the exact contract tick size.
    #[must_use]
    pub const fn tick_size(&self) -> &Decimal {
        &self.tick_size
    }
    /// Returns reconstructed ticks in provider arrival order.
    #[must_use]
    pub const fn ticks(&self) -> &[Tick] {
        &self.ticks
    }
}

/// One typed packet in a chart event.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum ChartPacket {
    /// Regular OHLC bars.
    Bars(BarPacket),
    /// Reconstructed compact ticks.
    Ticks(TickPacket),
    /// Historical delivery is complete for a subscription.
    EndOfHistory(ChartSubscriptionId),
}

/// A bounded chart event preserving provider packet order.
#[derive(Clone, Debug, PartialEq)]
pub struct ChartEvent {
    pub(super) packets: Box<[ChartPacket]>,
}

impl ChartEvent {
    /// Returns packets in provider arrival order.
    #[must_use]
    pub const fn packets(&self) -> &[ChartPacket] {
        &self.packets
    }
}

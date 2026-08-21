// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Validated current Partner `md/getChart` request builder.

use jiff::Timestamp;
use serde::Serialize;

use crate::realtime::RealtimeError;
use crate::{ContractId, Symbol};

use super::TickId;

/// The provider chart aggregation family.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ChartUnderlyingType {
    /// Compact trades.
    Tick,
    /// Daily bars.
    DailyBar,
    /// Minute bars.
    MinuteBar,
    /// Custom aggregation.
    Custom,
    /// Depth-of-market aggregation.
    DepthOfMarket,
}

impl ChartUnderlyingType {
    const fn wire(self) -> &'static str {
        match self {
            Self::Tick => "Tick",
            Self::DailyBar => "DailyBar",
            Self::MinuteBar => "MinuteBar",
            Self::Custom => "Custom",
            Self::DepthOfMarket => "DOM",
        }
    }
}

/// The unit used for a chart element size.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ChartElementUnit {
    /// Traded volume.
    Volume,
    /// Price range.
    Range,
    /// Native underlying units.
    UnderlyingUnits,
    /// Renko aggregation.
    Renko,
    /// Momentum range.
    MomentumRange,
    /// Point-and-figure aggregation.
    PointAndFigure,
    /// Order-flow-analysis range.
    OfaRange,
}

impl ChartElementUnit {
    const fn wire(self) -> &'static str {
        match self {
            Self::Volume => "Volume",
            Self::Range => "Range",
            Self::UnderlyingUnits => "UnderlyingUnits",
            Self::Renko => "Renko",
            Self::MomentumRange => "MomentumRange",
            Self::PointAndFigure => "PointAndFigure",
            Self::OfaRange => "OFARange",
        }
    }
}

/// A validated chart aggregation description.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ChartDescription {
    underlying_type: ChartUnderlyingType,
    element_size: u32,
    element_size_unit: ChartElementUnit,
    with_histogram: bool,
}

impl ChartDescription {
    /// Returns the aggregation family.
    #[must_use]
    pub const fn underlying_type(self) -> ChartUnderlyingType {
        self.underlying_type
    }

    /// Returns the positive element size.
    #[must_use]
    pub const fn element_size(self) -> u32 {
        self.element_size
    }

    /// Returns the aggregation unit.
    #[must_use]
    pub const fn element_size_unit(self) -> ChartElementUnit {
        self.element_size_unit
    }

    /// Returns whether the documented histogram extension was requested.
    #[must_use]
    pub const fn with_histogram(self) -> bool {
        self.with_histogram
    }
}

/// A validated chart history boundary with at least one selector.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChartTimeRange {
    closest_timestamp: Option<Timestamp>,
    closest_tick_id: Option<TickId>,
    as_far_as_timestamp: Option<Timestamp>,
    as_much_as_elements: Option<u32>,
}

impl ChartTimeRange {
    /// Returns the timestamp nearest the requested boundary.
    #[must_use]
    pub const fn closest_timestamp(&self) -> Option<&Timestamp> {
        self.closest_timestamp.as_ref()
    }

    /// Returns the nearest provider tick ID.
    #[must_use]
    pub const fn closest_tick_id(&self) -> Option<TickId> {
        self.closest_tick_id
    }

    /// Returns the far history timestamp boundary.
    #[must_use]
    pub const fn as_far_as_timestamp(&self) -> Option<&Timestamp> {
        self.as_far_as_timestamp.as_ref()
    }

    /// Returns the maximum requested element count.
    #[must_use]
    pub const fn as_much_as_elements(&self) -> Option<u32> {
        self.as_much_as_elements
    }
}

#[derive(Clone, Debug)]
enum OwnedTarget {
    Symbol(Symbol),
    ContractId(ContractId),
}

/// A validated `md/getChart` request.
#[derive(Clone, Debug)]
pub struct ChartRequest {
    target: OwnedTarget,
    description: ChartDescription,
    time_range: ChartTimeRange,
}

impl ChartRequest {
    /// Starts a chart request for a symbol.
    pub fn for_symbol(
        symbol: Symbol,
        underlying_type: ChartUnderlyingType,
        element_size: u32,
        element_size_unit: ChartElementUnit,
    ) -> ChartRequestBuilder {
        ChartRequestBuilder::new(
            OwnedTarget::Symbol(symbol),
            underlying_type,
            element_size,
            element_size_unit,
        )
    }

    /// Starts a chart request for a contract ID.
    pub fn for_contract(
        contract_id: ContractId,
        underlying_type: ChartUnderlyingType,
        element_size: u32,
        element_size_unit: ChartElementUnit,
    ) -> ChartRequestBuilder {
        ChartRequestBuilder::new(
            OwnedTarget::ContractId(contract_id),
            underlying_type,
            element_size,
            element_size_unit,
        )
    }

    /// Returns the chart aggregation description.
    #[must_use]
    pub const fn description(&self) -> ChartDescription {
        self.description
    }

    /// Returns the validated history boundary.
    #[must_use]
    pub const fn time_range(&self) -> &ChartTimeRange {
        &self.time_range
    }
}

/// Builder for [`ChartRequest`].
#[derive(Clone, Debug)]
#[must_use = "a chart request builder does nothing until build is called"]
pub struct ChartRequestBuilder {
    target: OwnedTarget,
    description: ChartDescription,
    closest_timestamp: Option<Timestamp>,
    closest_tick_id: Option<TickId>,
    as_far_as_timestamp: Option<Timestamp>,
    as_much_as_elements: Option<u32>,
}

impl ChartRequestBuilder {
    fn new(
        target: OwnedTarget,
        underlying_type: ChartUnderlyingType,
        element_size: u32,
        element_size_unit: ChartElementUnit,
    ) -> Self {
        Self {
            target,
            description: ChartDescription {
                underlying_type,
                element_size,
                element_size_unit,
                with_histogram: false,
            },
            closest_timestamp: None,
            closest_tick_id: None,
            as_far_as_timestamp: None,
            as_much_as_elements: None,
        }
    }

    /// Includes the provider's chart histogram extension.
    pub const fn with_histogram(mut self, value: bool) -> Self {
        self.description.with_histogram = value;
        self
    }

    /// Selects history nearest a timestamp.
    pub fn closest_timestamp(mut self, value: Timestamp) -> Self {
        self.closest_timestamp = Some(value);
        self
    }

    /// Selects history nearest the validated provider tick ID.
    pub const fn closest_tick_id(mut self, value: TickId) -> Self {
        self.closest_tick_id = Some(value);
        self
    }

    /// Selects the far timestamp boundary.
    pub fn as_far_as_timestamp(mut self, value: Timestamp) -> Self {
        self.as_far_as_timestamp = Some(value);
        self
    }

    /// Limits the number of returned elements.
    pub const fn as_much_as_elements(mut self, value: u32) -> Self {
        self.as_much_as_elements = Some(value);
        self
    }

    /// Validates and builds the request.
    ///
    /// # Errors
    ///
    /// Rejects a zero element size, tick element size other than one, zero
    /// element count, or an empty time range. [`TickId::new`] validates tick
    /// identities before they can enter the builder.
    pub fn build(self) -> Result<ChartRequest, RealtimeError> {
        validate(&self)?;
        Ok(ChartRequest {
            target: self.target,
            description: self.description,
            time_range: ChartTimeRange {
                closest_timestamp: self.closest_timestamp,
                closest_tick_id: self.closest_tick_id,
                as_far_as_timestamp: self.as_far_as_timestamp,
                as_much_as_elements: self.as_much_as_elements,
            },
        })
    }
}

fn validate(builder: &ChartRequestBuilder) -> Result<(), RealtimeError> {
    if builder.description.element_size == 0 {
        return Err(invalid("element_size", "must be positive"));
    }
    if matches!(
        builder.description.underlying_type,
        ChartUnderlyingType::Tick
    ) && builder.description.element_size != 1
    {
        return Err(invalid("element_size", "must equal one for tick charts"));
    }
    if builder.as_much_as_elements == Some(0) {
        return Err(invalid("as_much_as_elements", "must be positive"));
    }
    if builder.closest_timestamp.is_none()
        && builder.closest_tick_id.is_none()
        && builder.as_far_as_timestamp.is_none()
        && builder.as_much_as_elements.is_none()
    {
        return Err(invalid("time_range", "must contain at least one boundary"));
    }
    Ok(())
}

fn invalid(field: &'static str, reason: &'static str) -> RealtimeError {
    RealtimeError::InvalidRequest { field, reason }
}

pub(super) fn encode(request: &ChartRequest) -> Result<String, RealtimeError> {
    let symbol = match &request.target {
        OwnedTarget::Symbol(symbol) => WireTarget::Symbol(symbol.as_str()),
        OwnedTarget::ContractId(contract_id) => WireTarget::ContractId(contract_id.get()),
    };
    serde_json::to_string(&WireRequest {
        symbol,
        chart_description: WireDescription {
            underlying_type: request.description.underlying_type.wire(),
            element_size: request.description.element_size,
            element_size_unit: request.description.element_size_unit.wire(),
            with_histogram: request.description.with_histogram,
        },
        time_range: WireTimeRange {
            closest_timestamp: request.time_range.closest_timestamp.as_ref(),
            closest_tick_id: request.time_range.closest_tick_id.map(TickId::get),
            as_far_as_timestamp: request.time_range.as_far_as_timestamp.as_ref(),
            as_much_as_elements: request.time_range.as_much_as_elements,
        },
    })
    .map_err(|_| RealtimeError::Protocol)
}

#[derive(Serialize)]
#[serde(untagged)]
enum WireTarget<'a> {
    Symbol(&'a str),
    ContractId(i64),
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct WireRequest<'a> {
    symbol: WireTarget<'a>,
    chart_description: WireDescription<'a>,
    time_range: WireTimeRange<'a>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct WireDescription<'a> {
    underlying_type: &'a str,
    element_size: u32,
    element_size_unit: &'a str,
    with_histogram: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct WireTimeRange<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    closest_timestamp: Option<&'a Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    closest_tick_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    as_far_as_timestamp: Option<&'a Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    as_much_as_elements: Option<u32>,
}

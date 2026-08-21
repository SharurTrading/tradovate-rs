// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Validated chart requests and exact regular/tick packet types.
//!
//! Contract reviewed 2026-08-21 against the current Partner market-data request
//! reference and tick-chart guide:
//! <https://partner.tradovate.com/overview/core-concepts/web-sockets/market-data/market-data-request-reference>
//! and <https://partner.tradovate.com/overview/core-concepts/web-sockets/market-data/tick-charts>.

mod data;
pub(super) mod decode;
mod request;

use serde::Deserialize;

use super::{RealtimeConnection, RealtimeError};

pub use data::{
    Bar, BarPacket, ChartEvent, ChartPacket, ChartSubscriptionId, HistoricalChartId,
    RealtimeChartId, Tick, TickId, TickPacket,
};
pub use request::{
    ChartDescription, ChartElementUnit, ChartRequest, ChartRequestBuilder, ChartTimeRange,
    ChartUnderlyingType,
};

/// The two provider IDs allocated for one chart request.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ChartSubscription {
    historical_id: HistoricalChartId,
    realtime_id: RealtimeChartId,
}

impl ChartSubscription {
    /// Returns the historical stream ID.
    #[must_use]
    pub const fn historical_id(self) -> HistoricalChartId {
        self.historical_id
    }

    /// Returns the realtime stream ID required for cancellation.
    #[must_use]
    pub const fn realtime_id(self) -> RealtimeChartId {
        self.realtime_id
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct WireSubscription {
    historical_id: i64,
    realtime_id: i64,
}

impl RealtimeConnection {
    /// Requests historical plus realtime chart streams.
    ///
    /// The caller owns the returned IDs and must explicitly cancel the
    /// realtime ID. Chart packets preserve provider arrival order.
    ///
    /// # Errors
    ///
    /// Returns a socket-kind, encoding, capacity, provider, timeout, protocol,
    /// disconnect, or typed response failure.
    pub async fn get_chart(
        &self,
        request: &ChartRequest,
    ) -> Result<ChartSubscription, RealtimeError> {
        self.require_market_data_socket()?;
        let body = request::encode(request)?;
        let response = self.request_non_mutating("md/getChart", "", &body).await?;
        let data = response.data().ok_or(RealtimeError::InvalidTypedResponse {
            operation: "md/getChart",
        })?;
        let wire = serde_json::from_str::<WireSubscription>(data.get()).map_err(|_| {
            RealtimeError::InvalidTypedResponse {
                operation: "md/getChart",
            }
        })?;
        Ok(ChartSubscription {
            historical_id: HistoricalChartId::from_wire(wire.historical_id)?,
            realtime_id: RealtimeChartId::from_wire(wire.realtime_id)?,
        })
    }

    /// Cancels a realtime chart subscription.
    ///
    /// The current Partner prose requires `realtimeId`; its parameter comment
    /// inconsistently says historical ID. This API accepts only the typed
    /// realtime ID and records the discrepancy for staging verification.
    ///
    /// # Errors
    ///
    /// Returns a socket-kind, encoding, capacity, provider, timeout, protocol,
    /// or disconnect failure.
    pub async fn cancel_chart(&self, id: RealtimeChartId) -> Result<(), RealtimeError> {
        self.require_market_data_socket()?;
        let body = serde_json::to_string(&CancelChart {
            subscription_id: id.get(),
        })
        .map_err(|_| RealtimeError::Protocol)?;
        self.request_non_mutating("md/cancelChart", "", &body)
            .await?;
        Ok(())
    }
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct CancelChart {
    subscription_id: i64,
}

#[cfg(test)]
#[path = "chart/tests.rs"]
mod tests;

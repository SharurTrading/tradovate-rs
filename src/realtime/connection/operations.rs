// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Convenience methods which capture the connection generation before dispatch.

use crate::realtime::{
    ChartRequest, ChartSubscription, MarketDataChannel, MarketDataTarget, RealtimeChartId,
    RealtimeConnection, RealtimeError,
};

impl RealtimeConnection {
    /// Subscribes on this generation. The caller owns subscription truth.
    ///
    /// # Errors
    /// Returns validation, admission, provider, or uncertain-outcome failures.
    pub async fn subscribe_market_data<'a>(
        &self,
        channel: MarketDataChannel,
        target: impl Into<MarketDataTarget<'a>>,
    ) -> Result<(), RealtimeError> {
        self.session().subscribe_market_data(channel, target).await
    }

    /// Unsubscribes on this generation. Timeout does not prove unsubscription.
    ///
    /// # Errors
    /// Returns validation, admission, provider, or uncertain-outcome failures.
    pub async fn unsubscribe_market_data<'a>(
        &self,
        channel: MarketDataChannel,
        target: impl Into<MarketDataTarget<'a>>,
    ) -> Result<(), RealtimeError> {
        self.session()
            .unsubscribe_market_data(channel, target)
            .await
    }

    /// Requests generation-bound historical and realtime chart streams.
    ///
    /// # Errors
    /// Returns validation, admission, provider, or uncertain-outcome failures.
    pub async fn get_chart(
        &self,
        request: &ChartRequest,
    ) -> Result<ChartSubscription, RealtimeError> {
        self.session().get_chart(request).await
    }

    /// Cancels a chart on this generation.
    ///
    /// # Errors
    /// Returns validation, admission, provider, or uncertain-outcome failures.
    pub async fn cancel_chart(&self, id: RealtimeChartId) -> Result<(), RealtimeError> {
        self.session().cancel_chart(id).await
    }
}

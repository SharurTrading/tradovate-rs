// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Validated entity writes for current subscription records.

use serde::Serialize;

use crate::{
    Client, Error,
    api::current::{
        support::CurrentRequest,
        users::{MarketDataSubscription, TradovateSubscription},
    },
    client::MutationAssessment,
};

use super::validation::{ordered_dates, same_date};

/// A market-data subscription create request with no caller-supplied entity ID.
#[derive(Clone, Debug, Serialize)]
#[serde(transparent)]
pub struct CreateMarketDataSubscriptionRequest(MarketDataSubscription);

impl CreateMarketDataSubscriptionRequest {
    /// Validates a generated entity for create semantics.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidRequest`] for an existing ID or invalid billing period.
    pub fn new(value: MarketDataSubscription) -> Result<Self, Error> {
        validate_market_data(&value, false)?;
        Ok(Self(value))
    }

    /// Returns the validated wire entity.
    #[must_use]
    pub const fn entity(&self) -> &MarketDataSubscription {
        &self.0
    }
}

impl TryFrom<MarketDataSubscription> for CreateMarketDataSubscriptionRequest {
    type Error = Error;

    fn try_from(value: MarketDataSubscription) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl CurrentRequest for CreateMarketDataSubscriptionRequest {
    fn validate_current(&self) -> Result<(), Error> {
        validate_market_data(&self.0, false)
    }
}

/// A market-data subscription update request that requires its entity ID.
#[derive(Clone, Debug, Serialize)]
#[serde(transparent)]
pub struct UpdateMarketDataSubscriptionRequest(MarketDataSubscription);

impl UpdateMarketDataSubscriptionRequest {
    /// Validates a generated entity for update semantics.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidRequest`] for a missing ID or invalid billing period.
    pub fn new(value: MarketDataSubscription) -> Result<Self, Error> {
        validate_market_data(&value, true)?;
        Ok(Self(value))
    }

    /// Returns the validated wire entity.
    #[must_use]
    pub const fn entity(&self) -> &MarketDataSubscription {
        &self.0
    }
}

impl TryFrom<MarketDataSubscription> for UpdateMarketDataSubscriptionRequest {
    type Error = Error;

    fn try_from(value: MarketDataSubscription) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl CurrentRequest for UpdateMarketDataSubscriptionRequest {
    fn validate_current(&self) -> Result<(), Error> {
        validate_market_data(&self.0, true)
    }
}

/// A Tradovate subscription create request with no caller-supplied entity ID.
#[derive(Clone, Debug, Serialize)]
#[serde(transparent)]
pub struct CreateTradovateSubscriptionRequest(TradovateSubscription);

impl CreateTradovateSubscriptionRequest {
    /// Validates a generated entity for create semantics and date ordering.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidRequest`] for an existing ID or invalid dates.
    pub fn new(value: TradovateSubscription) -> Result<Self, Error> {
        validate_tradovate(&value)?;
        Ok(Self(value))
    }

    /// Returns the validated wire entity.
    #[must_use]
    pub const fn entity(&self) -> &TradovateSubscription {
        &self.0
    }
}

impl TryFrom<TradovateSubscription> for CreateTradovateSubscriptionRequest {
    type Error = Error;

    fn try_from(value: TradovateSubscription) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl CurrentRequest for CreateTradovateSubscriptionRequest {
    fn validate_current(&self) -> Result<(), Error> {
        validate_tradovate(&self.0)
    }
}

impl Client {
    /// Creates one current market-data subscription entity.
    ///
    /// # Errors
    ///
    /// Returns validation, authentication, rate, transport, provider-control,
    /// decoding, or ambiguity errors. Success requires a new response ID and
    /// exact stable subscription targets and values.
    pub async fn market_data_subscription_create(
        &self,
        request: &CreateMarketDataSubscriptionRequest,
    ) -> Result<MarketDataSubscription, Error> {
        self.post_reviewed_mutation(
            "/marketDataSubscription/create",
            request,
            assess_market_data_create,
        )
        .await
    }

    /// Updates one current market-data subscription entity.
    ///
    /// # Errors
    ///
    /// Returns validation, authentication, rate, transport, provider-control,
    /// decoding, or ambiguity errors. Success requires the exact response ID
    /// and exact stable subscription targets and values.
    pub async fn market_data_subscription_update(
        &self,
        request: &UpdateMarketDataSubscriptionRequest,
    ) -> Result<MarketDataSubscription, Error> {
        self.post_reviewed_mutation(
            "/marketDataSubscription/update",
            request,
            assess_market_data_update,
        )
        .await
    }

    /// Creates one current Tradovate membership subscription entity.
    ///
    /// # Errors
    ///
    /// Returns validation, authentication, rate, transport, provider-control,
    /// decoding, or ambiguity errors. Success requires a new response ID and
    /// exact user, plan, account, date, and monetary values.
    pub async fn tradovate_subscription_create(
        &self,
        request: &CreateTradovateSubscriptionRequest,
    ) -> Result<TradovateSubscription, Error> {
        self.post_reviewed_mutation(
            "/tradovateSubscription/create",
            request,
            assess_tradovate_create,
        )
        .await
    }
}

fn validate_market_data(value: &MarketDataSubscription, update: bool) -> Result<(), Error> {
    value.validate_current()?;
    match (update, value.id().is_some()) {
        (false, true) => return Err(id_error("must be absent when creating")),
        (true, false) => return Err(id_error("is required when updating")),
        _ => {}
    }
    if *value.year() <= 0 {
        return Err(Error::InvalidRequest {
            field: "year",
            reason: "must be positive",
        });
    }
    if !(1..=12).contains(value.month()) {
        return Err(Error::InvalidRequest {
            field: "month",
            reason: "must be between 1 and 12",
        });
    }
    Ok(())
}

fn validate_tradovate(value: &TradovateSubscription) -> Result<(), Error> {
    value.validate_current()?;
    if value.id().is_some() {
        return Err(id_error("must be absent when creating"));
    }
    ordered_dates(value.start_date(), value.expiration_date())
}

fn id_error(reason: &'static str) -> Error {
    Error::InvalidRequest {
        field: "id",
        reason,
    }
}

fn assess_market_data_create(
    response: &MarketDataSubscription,
    request: &CreateMarketDataSubscriptionRequest,
) -> MutationAssessment {
    assess_entity(
        response.id().is_some(),
        same_market_data(response, request.entity()),
    )
}

fn assess_market_data_update(
    response: &MarketDataSubscription,
    request: &UpdateMarketDataSubscriptionRequest,
) -> MutationAssessment {
    let exact_id = response.id() == request.entity().id();
    assess_entity(
        response.id().is_some(),
        exact_id && same_market_data(response, request.entity()),
    )
}

fn assess_tradovate_create(
    response: &TradovateSubscription,
    request: &CreateTradovateSubscriptionRequest,
) -> MutationAssessment {
    assess_entity(
        response.id().is_some(),
        same_tradovate(response, request.entity()),
    )
}

fn assess_entity(has_id: bool, exact: bool) -> MutationAssessment {
    if has_id && exact {
        MutationAssessment::success()
    } else {
        MutationAssessment::ambiguous(has_id)
    }
}

fn same_market_data(left: &MarketDataSubscription, right: &MarketDataSubscription) -> bool {
    left.user_id() == right.user_id()
        && left.plan_price() == right.plan_price()
        && left.market_data_subscription_plan_id() == right.market_data_subscription_plan_id()
        && left.year() == right.year()
        && left.month() == right.month()
        && left.expired() == right.expired()
        && optional_matches(right.account_id(), left.account_id())
        && optional_matches(right.cash_balance_log_id(), left.cash_balance_log_id())
        && optional_matches(
            right.renewal_credit_card_id(),
            left.renewal_credit_card_id(),
        )
        && optional_matches(right.renewal_account_id(), left.renewal_account_id())
}

fn same_tradovate(left: &TradovateSubscription, right: &TradovateSubscription) -> bool {
    left.user_id() == right.user_id()
        && left.plan_price() == right.plan_price()
        && left.tradovate_subscription_plan_id() == right.tradovate_subscription_plan_id()
        && same_date(left.start_date(), right.start_date())
        && same_date(left.expiration_date(), right.expiration_date())
        && left.paid_amount() == right.paid_amount()
        && optional_matches(right.account_id(), left.account_id())
        && optional_matches(right.cash_balance_log_id(), left.cash_balance_log_id())
        && optional_matches(right.cancelled_renewal(), left.cancelled_renewal())
        && optional_matches(right.cancel_reason(), left.cancel_reason())
}

fn optional_matches<T: PartialEq + ?Sized>(requested: Option<&T>, returned: Option<&T>) -> bool {
    requested.is_none_or(|value| returned == Some(value))
}

#[cfg(test)]
#[path = "tests/subscriptions.rs"]
mod tests;

// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Subscription purchase and bulk-cancellation mutations.

use std::collections::BTreeSet;

use crate::api::current::users::{
    AddEntitlementSubscription, AddMarketDataSubscription, AddTradovateSubscription,
    CancelEverything, CancelEverythingResponse, EntitlementSubscriptionResponse,
    EntitlementSubscriptionResponseErrorCode, MarketDataSubscriptionResponse,
    MarketDataSubscriptionResponseErrorCode, TradovateSubscriptionResponse,
    TradovateSubscriptionResponseErrorCode,
};
use crate::{Client, Error, UserId, client::MutationAssessment};

use super::validation::{effective_user, has_error};

/// A market-data purchase request with validated month and unique plan IDs.
#[derive(Clone, Debug, serde::Serialize)]
#[serde(transparent)]
pub struct MarketDataSubscriptionPurchase(AddMarketDataSubscription);

impl TryFrom<AddMarketDataSubscription> for MarketDataSubscriptionPurchase {
    type Error = Error;

    fn try_from(request: AddMarketDataSubscription) -> Result<Self, Self::Error> {
        validate_market_data_request(&request)?;
        Ok(Self(request))
    }
}

impl crate::api::current::support::CurrentRequest for MarketDataSubscriptionPurchase {
    fn validate_current(&self) -> Result<(), Error> {
        validate_market_data_request(&self.0)
    }
}

/// A bulk-cancellation request with a non-empty target family selection.
#[derive(Clone, Debug, serde::Serialize)]
#[serde(transparent)]
pub struct ValidatedCancelEverything(CancelEverything);

impl TryFrom<CancelEverything> for ValidatedCancelEverything {
    type Error = Error;

    fn try_from(request: CancelEverything) -> Result<Self, Self::Error> {
        validate_cancel_everything(&request)?;
        Ok(Self(request))
    }
}

impl crate::api::current::support::CurrentRequest for ValidatedCancelEverything {
    fn validate_current(&self) -> Result<(), Error> {
        validate_cancel_everything(&self.0)
    }
}

impl Client {
    /// Purchases one entitlement subscription for the exact requested user.
    ///
    /// # Errors
    ///
    /// Returns local validation, authentication, rate, transport, provider,
    /// decoding, or ambiguous-mutation errors. Resolution requires the known
    /// `Success` code and a new subscription matching the entitlement, user,
    /// and any explicitly selected account.
    pub async fn user_plugin_add_entitlement_subscription(
        &self,
        request: &AddEntitlementSubscription,
    ) -> Result<EntitlementSubscriptionResponse, Error> {
        let user_id = effective_user(self, request.user_id().copied())?;
        self.post_reviewed_mutation(
            "/userPlugin/addentitlementsubscription",
            request,
            move |response, request| assess_entitlement(response, request, user_id),
        )
        .await
    }

    /// Purchases the requested market-data subscription plan or plans.
    ///
    /// # Errors
    ///
    /// Returns local validation, authentication, rate, transport, provider,
    /// decoding, or ambiguous-mutation errors. The current response carries a
    /// single subscription, so a multi-plan request necessarily requires
    /// reconciliation instead of being reported as blanket success.
    pub async fn user_add_market_data_subscription(
        &self,
        request: &MarketDataSubscriptionPurchase,
    ) -> Result<MarketDataSubscriptionResponse, Error> {
        let user_id = effective_user(self, request.0.user_id().copied())?;
        self.post_reviewed_mutation(
            "/user/addmarketdatasubscription",
            request,
            move |response, request| assess_market_data(response, request, user_id),
        )
        .await
    }

    /// Purchases one Tradovate membership subscription for the exact user.
    ///
    /// # Errors
    ///
    /// Returns local validation, authentication, rate, transport, provider,
    /// decoding, or ambiguous-mutation errors. Resolution requires the known
    /// `Success` code and a new subscription matching the requested plan,
    /// effective user, and any explicitly selected account.
    pub async fn user_add_tradovate_subscription(
        &self,
        request: &AddTradovateSubscription,
    ) -> Result<TradovateSubscriptionResponse, Error> {
        let user_id = effective_user(self, request.user_id().copied())?;
        self.post_reviewed_mutation(
            "/user/addtradovatesubscription",
            request,
            move |response, request| assess_tradovate(response, request, user_id),
        )
        .await
    }

    /// Cancels the selected subscription families for the requested users.
    ///
    /// The current response lists affected resource IDs but does not associate
    /// them with the requested users or prove that every target was processed.
    /// Consequently, every transmitted 2xx response is fenced as ambiguous and
    /// the caller must reconcile the authoritative subscription collections.
    ///
    /// # Errors
    ///
    /// Returns local validation, authentication, rate, transport, provider,
    /// decoding, or ambiguous-mutation errors. This method never reports a 2xx
    /// response as definitive completion under the pinned contract.
    pub async fn user_cancel_everything(
        &self,
        request: &ValidatedCancelEverything,
    ) -> Result<CancelEverythingResponse, Error> {
        self.post_reviewed_mutation("/user/canceleverything", request, assess_cancel_everything)
            .await
    }
}

fn assess_entitlement(
    response: &EntitlementSubscriptionResponse,
    request: &AddEntitlementSubscription,
    user_id: UserId,
) -> MutationAssessment {
    let entity = response.entitlement_subscription();
    let exact = entity.is_some_and(|subscription| {
        subscription.id().is_some()
            && subscription.entitlement_id() == Some(request.entitlement_id())
            && subscription.user_id() == &user_id
            && request
                .account_id()
                .is_none_or(|account| subscription.account_id() == Some(account))
    });
    assess_subscription_code(
        response.error_text(),
        response.error_code(),
        exact,
        entity.is_some(),
        |code| matches!(code, EntitlementSubscriptionResponseErrorCode::Success),
        |code| matches!(code, EntitlementSubscriptionResponseErrorCode::Unknown(_)),
    )
}

fn assess_market_data(
    response: &MarketDataSubscriptionResponse,
    request: &MarketDataSubscriptionPurchase,
    user_id: UserId,
) -> MutationAssessment {
    let request = &request.0;
    let plans = request.market_data_subscription_plan_ids();
    let entity = response.market_data_subscription();
    let matches_returned = entity.is_some_and(|subscription| {
        subscription.id().is_some()
            && plans.contains(subscription.market_data_subscription_plan_id())
            && subscription.user_id() == &user_id
            && subscription.year() == request.year()
            && subscription.month() == request.month()
            && request
                .account_id()
                .is_none_or(|account| subscription.account_id() == Some(account))
    });
    let exact = plans.len() == 1 && matches_returned;
    assess_subscription_code(
        response.error_text(),
        response.error_code(),
        exact,
        entity.is_some(),
        |code| matches!(code, MarketDataSubscriptionResponseErrorCode::Success),
        |code| matches!(code, MarketDataSubscriptionResponseErrorCode::Unknown(_)),
    )
}

fn assess_tradovate(
    response: &TradovateSubscriptionResponse,
    request: &AddTradovateSubscription,
    user_id: UserId,
) -> MutationAssessment {
    let entity = response.tradovate_subscription();
    let exact = entity.is_some_and(|subscription| {
        subscription.id().is_some()
            && subscription.tradovate_subscription_plan_id()
                == request.tradovate_subscription_plan_id()
            && subscription.user_id() == &user_id
            && request
                .account_id()
                .is_none_or(|account| subscription.account_id() == Some(account))
    });
    assess_subscription_code(
        response.error_text(),
        response.error_code(),
        exact,
        entity.is_some(),
        |code| matches!(code, TradovateSubscriptionResponseErrorCode::Success),
        |code| matches!(code, TradovateSubscriptionResponseErrorCode::Unknown(_)),
    )
}

fn assess_subscription_code<C>(
    error: Option<&str>,
    code: Option<&C>,
    exact: bool,
    entity_evidence: bool,
    success: impl Fn(&C) -> bool,
    unknown: impl Fn(&C) -> bool,
) -> MutationAssessment {
    let known_success = code.is_some_and(&success);
    if has_error(error) {
        return if entity_evidence || known_success {
            MutationAssessment::ambiguous(true)
        } else {
            MutationAssessment::rejected()
        };
    }
    match code {
        Some(code) if success(code) && exact => MutationAssessment::success(),
        Some(code) if success(code) || unknown(code) || entity_evidence => {
            MutationAssessment::ambiguous(known_success || entity_evidence)
        }
        Some(_) => MutationAssessment::rejected(),
        None => MutationAssessment::ambiguous(entity_evidence),
    }
}

fn validate_market_data_request(request: &AddMarketDataSubscription) -> Result<(), Error> {
    if !(1..=12).contains(request.month()) {
        return Err(Error::InvalidRequest {
            field: "month",
            reason: "must be between 1 and 12",
        });
    }
    let unique = request
        .market_data_subscription_plan_ids()
        .iter()
        .collect::<BTreeSet<_>>();
    if unique.len() != request.market_data_subscription_plan_ids().len() {
        return Err(Error::InvalidRequest {
            field: "marketDataSubscriptionPlanIds",
            reason: "must not contain duplicate plan IDs",
        });
    }
    Ok(())
}

fn validate_cancel_everything(request: &CancelEverything) -> Result<(), Error> {
    let selected = [
        request.tradovate_subscriptions(),
        request.user_plugins(),
        request.market_data_subscriptions(),
        request.trading_permissions(),
    ]
    .into_iter()
    .any(|flag| flag == Some(&true));
    if !selected {
        return Err(Error::InvalidRequest {
            field: "cancellationFamilies",
            reason: "at least one cancellation family must be selected",
        });
    }
    let unique = request.user_ids().iter().collect::<BTreeSet<_>>();
    if unique.len() != request.user_ids().len() {
        return Err(Error::InvalidRequest {
            field: "userIds",
            reason: "must not contain duplicate user IDs",
        });
    }
    Ok(())
}

fn assess_cancel_everything(
    response: &CancelEverythingResponse,
    _: &ValidatedCancelEverything,
) -> MutationAssessment {
    let evidence = !response.tradovate_subscription_ids().is_empty()
        || !response.user_plugin_ids().is_empty()
        || !response.market_data_subscription_ids().is_empty()
        || !response.trading_permission_ids().is_empty();
    MutationAssessment::ambiguous(evidence)
}

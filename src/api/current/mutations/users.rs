// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Trading-permission, subscription, and user-administration mutations.

use crate::{
    Client, Error,
    api::current::users::{
        AcceptTradingPermission, CancelTradovateSubscription, ChangePluginPermission,
        CreateTradingPermission, ExpireUserLockout, RequestTradingPermission,
        RevokeTradingPermission, RevokeTradingPermissions, SimpleResponse,
        TradingPermissionResponse, TradovateSubscriptionResponse,
        TradovateSubscriptionResponseErrorCode,
    },
    client::MutationAssessment,
};

use super::common::{exact_entity, simple_ok};

impl Client {
    /// Accepts one current trading-permission record.
    ///
    /// # Errors
    ///
    /// Returns local validation, authentication, rate, transport, provider,
    /// decoding, or ambiguous-mutation errors. The response must echo the exact
    /// requested trading-permission identity before the mutation is resolved.
    pub async fn user_accept_trading_permission(
        &self,
        request: &AcceptTradingPermission,
    ) -> Result<TradingPermissionResponse, Error> {
        self.post_reviewed_mutation(
            "/user/accepttradingpermission",
            request,
            assess_accept_trading_permission,
        )
        .await
    }

    /// Cancels one current Tradovate subscription.
    ///
    /// # Errors
    ///
    /// Returns local validation, authentication, rate, transport, provider,
    /// decoding, or ambiguous-mutation errors. Completion requires a successful
    /// response that echoes the requested subscription identity.
    pub async fn user_cancel_tradovate_subscription(
        &self,
        request: &CancelTradovateSubscription,
    ) -> Result<TradovateSubscriptionResponse, Error> {
        self.post_reviewed_mutation(
            "/user/canceltradovatesubscription",
            request,
            assess_cancel_subscription,
        )
        .await
    }

    /// Changes a user's permission for one named plugin.
    ///
    /// # Errors
    ///
    /// Returns local validation, authentication, rate, transport, provider,
    /// decoding, or ambiguous-mutation errors. Only `ok: true` proves success;
    /// `ok: false` is returned as a definitive business rejection.
    pub async fn user_plugin_change_plugin_permission(
        &self,
        request: &ChangePluginPermission,
    ) -> Result<SimpleResponse, Error> {
        self.post_reviewed_mutation(
            "/userPlugin/changepluginpermission",
            request,
            assess_change_plugin_permission,
        )
        .await
    }

    /// Creates a trading permission for an exact account and user pair.
    ///
    /// # Errors
    ///
    /// Returns local validation, authentication, rate, transport, provider,
    /// decoding, or ambiguous-mutation errors. Both response identities must
    /// match the request before the mutation is resolved.
    pub async fn user_create_trading_permission(
        &self,
        request: &CreateTradingPermission,
    ) -> Result<TradingPermissionResponse, Error> {
        self.post_reviewed_mutation(
            "/user/createtradingpermission",
            request,
            assess_create_trading_permission,
        )
        .await
    }

    /// Expires one user's current lockout.
    ///
    /// # Errors
    ///
    /// Returns local validation, authentication, rate, transport, provider,
    /// decoding, or ambiguous-mutation errors. The live REST environment is
    /// rejected locally; only `ok: true` proves success in demo.
    pub async fn user_expire_user_lockout(
        &self,
        request: &ExpireUserLockout,
    ) -> Result<SimpleResponse, Error> {
        if !self.endpoints.permits_demo_only_rest() {
            return Err(Error::InvalidRequest {
                field: "environment",
                reason: "expireUserLockout requires the demo REST environment",
            });
        }
        self.post_reviewed_mutation(
            "/user/expireuserlockout",
            request,
            assess_expire_user_lockout,
        )
        .await
    }

    /// Requests trading permission for an account and CTA contact.
    ///
    /// # Errors
    ///
    /// Returns local validation, authentication, rate, transport, provider,
    /// decoding, or ambiguous-mutation errors. The response must echo the exact
    /// account, CTA contact, and CTA email supplied in the request.
    pub async fn user_request_trading_permission(
        &self,
        request: &RequestTradingPermission,
    ) -> Result<TradingPermissionResponse, Error> {
        self.post_reviewed_mutation(
            "/user/requesttradingpermission",
            request,
            assess_request_trading_permission,
        )
        .await
    }

    /// Revokes one current trading-permission record.
    ///
    /// # Errors
    ///
    /// Returns local validation, authentication, rate, transport, provider,
    /// decoding, or ambiguous-mutation errors. The response must echo the exact
    /// requested trading-permission identity before resolution.
    pub async fn user_revoke_trading_permission(
        &self,
        request: &RevokeTradingPermission,
    ) -> Result<TradingPermissionResponse, Error> {
        self.post_reviewed_mutation(
            "/user/revoketradingpermission",
            request,
            assess_revoke_trading_permission,
        )
        .await
    }

    /// Revokes a non-empty collection of trading-permission records.
    ///
    /// # Errors
    ///
    /// Returns local validation, authentication, rate, transport, provider,
    /// decoding, or ambiguous-mutation errors. Only `ok: true` proves success.
    pub async fn user_revoke_trading_permissions(
        &self,
        request: &RevokeTradingPermissions,
    ) -> Result<SimpleResponse, Error> {
        self.post_reviewed_mutation(
            "/user/revoketradingpermissions",
            request,
            assess_revoke_trading_permissions,
        )
        .await
    }
}

fn assess_accept_trading_permission(
    response: &TradingPermissionResponse,
    request: &AcceptTradingPermission,
) -> MutationAssessment {
    let exact = response
        .trading_permission()
        .and_then(|permission| permission.id())
        .is_some_and(|id| id == request.trading_permission_id());
    exact_entity(response.error_text(), exact)
}

fn assess_cancel_subscription(
    response: &TradovateSubscriptionResponse,
    request: &CancelTradovateSubscription,
) -> MutationAssessment {
    let exact = response
        .tradovate_subscription()
        .and_then(|subscription| subscription.id())
        .is_some_and(|id| id == request.tradovate_subscription_id());
    let code_success = matches!(
        response.error_code(),
        Some(TradovateSubscriptionResponseErrorCode::Success)
    );
    if response.error_text().is_some_and(|error| !error.is_empty()) {
        return if exact || code_success {
            MutationAssessment::ambiguous(true)
        } else {
            MutationAssessment::rejected()
        };
    }
    match response.error_code() {
        None | Some(TradovateSubscriptionResponseErrorCode::Success) if exact => {
            MutationAssessment::success()
        }
        Some(TradovateSubscriptionResponseErrorCode::Success) => {
            MutationAssessment::ambiguous(true)
        }
        Some(code) if is_known_subscription_rejection(code) && exact => {
            MutationAssessment::ambiguous(true)
        }
        Some(code) if is_known_subscription_rejection(code) => MutationAssessment::rejected(),
        Some(_) => MutationAssessment::ambiguous(exact),
        None => MutationAssessment::ambiguous(false),
    }
}

fn is_known_subscription_rejection(code: &TradovateSubscriptionResponseErrorCode) -> bool {
    matches!(
        code,
        TradovateSubscriptionResponseErrorCode::ConflictWithExisting
            | TradovateSubscriptionResponseErrorCode::DowngradeNotAllowed
            | TradovateSubscriptionResponseErrorCode::IncompatibleCmeMarketDataSubscriptionPlans
            | TradovateSubscriptionResponseErrorCode::IncorrectPaymentMethod
            | TradovateSubscriptionResponseErrorCode::InsufficientFunds
            | TradovateSubscriptionResponseErrorCode::PaymentProviderError
            | TradovateSubscriptionResponseErrorCode::PlanDiscontinued
            | TradovateSubscriptionResponseErrorCode::SingleTrialOnly
            | TradovateSubscriptionResponseErrorCode::UnknownError
    )
}

fn assess_change_plugin_permission(
    response: &SimpleResponse,
    _: &ChangePluginPermission,
) -> MutationAssessment {
    simple_ok(response)
}

fn assess_create_trading_permission(
    response: &TradingPermissionResponse,
    request: &CreateTradingPermission,
) -> MutationAssessment {
    let exact = response.trading_permission().is_some_and(|permission| {
        permission.account_id() == request.account_id() && permission.user_id() == request.user_id()
    });
    exact_entity(response.error_text(), exact)
}

fn assess_expire_user_lockout(
    response: &SimpleResponse,
    _: &ExpireUserLockout,
) -> MutationAssessment {
    simple_ok(response)
}

fn assess_request_trading_permission(
    response: &TradingPermissionResponse,
    request: &RequestTradingPermission,
) -> MutationAssessment {
    let exact = response.trading_permission().is_some_and(|permission| {
        permission.account_id() == request.account_id()
            && permission.cta_contact() == request.cta_contact()
            && permission.cta_email() == request.cta_email()
    });
    exact_entity(response.error_text(), exact)
}

fn assess_revoke_trading_permission(
    response: &TradingPermissionResponse,
    request: &RevokeTradingPermission,
) -> MutationAssessment {
    let exact = response
        .trading_permission()
        .and_then(|permission| permission.id())
        .is_some_and(|id| id == request.trading_permission_id());
    exact_entity(response.error_text(), exact)
}

fn assess_revoke_trading_permissions(
    response: &SimpleResponse,
    _: &RevokeTradingPermissions,
) -> MutationAssessment {
    simple_ok(response)
}

#[cfg(test)]
#[path = "users/tests.rs"]
mod tests;

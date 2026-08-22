// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Demo-account reset and position-limit deletion mutations.

use crate::{
    Client, Error,
    api::current::{
        accounting::ResetDemoAccountState,
        risks::{
            DeleteResultResponse, DeleteUserAccountPositionLimit, DeleteUserAccountRiskParameter,
        },
        users::SimpleResponse,
    },
    client::MutationAssessment,
};

use super::common::{delete_result, simple_ok};

impl Client {
    /// Resets the requested demo accounts to the supplied trade date.
    ///
    /// # Errors
    ///
    /// Returns local validation, authentication, rate, transport, provider,
    /// decoding, or ambiguous-mutation errors. The live REST environment is
    /// rejected locally; only `ok: true` proves success in demo.
    pub async fn account_reset_demo_account_state(
        &self,
        request: &ResetDemoAccountState,
    ) -> Result<SimpleResponse, Error> {
        if !self.endpoints.permits_demo_only_rest() {
            return Err(Error::InvalidRequest {
                field: "environment",
                reason: "resetDemoAccountState requires the demo REST environment",
            });
        }
        self.post_reviewed_mutation(
            "/account/resetdemoaccountstate",
            request,
            assess_reset_demo_account_state,
        )
        .await
    }

    /// Deletes one user-account position-limit record.
    ///
    /// # Errors
    ///
    /// Returns local validation, authentication, rate, transport, provider,
    /// decoding, or ambiguous-mutation errors. Only `success: true` proves the
    /// deletion; `success: false` is a definitive business rejection.
    pub async fn user_account_position_limit_delete_user_account_position_limit(
        &self,
        request: &DeleteUserAccountPositionLimit,
    ) -> Result<DeleteResultResponse, Error> {
        self.post_reviewed_mutation(
            "/userAccountPositionLimit/deleteuseraccountpositionlimit",
            request,
            assess_delete_position_limit,
        )
        .await
    }

    /// Deletes one user-account risk-parameter record through the current
    /// `userAccountPositionLimit` endpoint family.
    ///
    /// # Errors
    ///
    /// Returns local validation, authentication, rate, transport, provider,
    /// decoding, or ambiguous-mutation errors. Only `success: true` proves the
    /// deletion; `success: false` is a definitive business rejection.
    pub async fn user_account_position_limit_delete_user_account_risk_parameter(
        &self,
        request: &DeleteUserAccountRiskParameter,
    ) -> Result<DeleteResultResponse, Error> {
        self.post_reviewed_mutation(
            "/userAccountPositionLimit/deleteuseraccountriskparameter",
            request,
            assess_delete_risk_parameter,
        )
        .await
    }
}

fn assess_reset_demo_account_state(
    response: &SimpleResponse,
    _: &ResetDemoAccountState,
) -> MutationAssessment {
    simple_ok(response)
}

fn assess_delete_position_limit(
    response: &DeleteResultResponse,
    _: &DeleteUserAccountPositionLimit,
) -> MutationAssessment {
    delete_result(response)
}

fn assess_delete_risk_parameter(
    response: &DeleteResultResponse,
    _: &DeleteUserAccountRiskParameter,
) -> MutationAssessment {
    delete_result(response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AccountId, Environment, api::current::users::TradeDate};

    #[tokio::test]
    async fn demo_reset_is_rejected_on_the_live_rest_environment() {
        let client = Client::builder(Environment::Live)
            .build()
            .unwrap_or_else(|error| panic!("client fixture: {error}"));
        let trade_date = TradeDate::builder()
            .year(2026)
            .month(8)
            .day(21)
            .build()
            .unwrap_or_else(|error| panic!("trade date fixture: {error}"));
        let account =
            AccountId::new(7).unwrap_or_else(|error| panic!("account ID fixture: {error}"));
        let request = ResetDemoAccountState::builder()
            .account_ids(vec![account])
            .reset_trade_date(trade_date)
            .build()
            .unwrap_or_else(|error| panic!("reset request fixture: {error}"));

        let result = client.account_reset_demo_account_state(&request).await;
        assert!(matches!(
            result,
            Err(Error::InvalidRequest {
                field: "environment",
                ..
            })
        ));
    }
}

// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Account risk-status mutations and partner-only administrative actions.

use std::collections::HashSet;

use serde::Serialize;

use crate::{
    AccountId, Client, Error,
    api::current::{
        risks::{
            AccountRiskStatusResponse, ResetAutoLiqStatus,
            SetAdminAutoLiqActionAdminActionReasonCode, SwitchRiskCategory, UpdateMaxNetLiq,
        },
        support::CurrentRequest,
    },
    client::{DocumentedMutationResponse, MutationAssessment, MutationOutcome},
};

const MAX_ADMIN_REASON_CHARS: usize = 8_192;
const MAX_SWITCH_ACCOUNTS: usize = 10_000;

/// A partner-supported persistent administrative auto-liquidation action.
///
/// Internal provider actions are deliberately absent from this request enum.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[non_exhaustive]
pub enum PartnerAdminAutoLiqAction {
    /// Clears restrictions and permits trading to resume.
    Normal,
    /// Rejects all new orders without liquidating open positions.
    LockTradingImmediately,
    /// Liquidates all open positions and locks the account.
    LiquidateImmediately,
    /// Permits closing orders but rejects position-opening orders.
    LiquidateOnlyModeImmediately,
}

/// A validated demo-administrator request to set persistent account state.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetAdminAutoLiqActionRequest {
    account_id: AccountId,
    admin_action: PartnerAdminAutoLiqAction,
    admin_action_reason_code: SetAdminAutoLiqActionAdminActionReasonCode,
    #[serde(skip_serializing_if = "Option::is_none")]
    admin_action_reason: Option<String>,
}

impl SetAdminAutoLiqActionRequest {
    /// Starts a validated request builder.
    pub fn builder() -> SetAdminAutoLiqActionRequestBuilder {
        SetAdminAutoLiqActionRequestBuilder::default()
    }

    /// Returns the target account identity.
    #[must_use]
    pub const fn account_id(&self) -> AccountId {
        self.account_id
    }

    /// Returns the persistent partner action.
    #[must_use]
    pub const fn admin_action(&self) -> PartnerAdminAutoLiqAction {
        self.admin_action
    }

    /// Returns the provider reason code.
    #[must_use]
    pub const fn admin_action_reason_code(&self) -> &SetAdminAutoLiqActionAdminActionReasonCode {
        &self.admin_action_reason_code
    }

    /// Returns the optional operator-supplied reason.
    #[must_use]
    pub fn admin_action_reason(&self) -> Option<&str> {
        self.admin_action_reason.as_deref()
    }
}

impl CurrentRequest for SetAdminAutoLiqActionRequest {
    fn validate_current(&self) -> Result<(), Error> {
        validate_admin_reason(
            &self.admin_action_reason_code,
            self.admin_action_reason.as_deref(),
        )
    }
}

/// Builder for [`SetAdminAutoLiqActionRequest`].
#[derive(Clone, Debug, Default)]
#[must_use = "a request builder does nothing until build is called"]
pub struct SetAdminAutoLiqActionRequestBuilder {
    account_id: Option<AccountId>,
    admin_action: Option<PartnerAdminAutoLiqAction>,
    admin_action_reason_code: Option<SetAdminAutoLiqActionAdminActionReasonCode>,
    admin_action_reason: Option<String>,
}

impl SetAdminAutoLiqActionRequestBuilder {
    /// Sets the affected account.
    pub fn account_id(mut self, value: AccountId) -> Self {
        self.account_id = Some(value);
        self
    }

    /// Sets one partner-supported persistent action.
    pub fn admin_action(mut self, value: PartnerAdminAutoLiqAction) -> Self {
        self.admin_action = Some(value);
        self
    }

    /// Sets the provider reason code.
    pub fn admin_action_reason_code(
        mut self,
        value: SetAdminAutoLiqActionAdminActionReasonCode,
    ) -> Self {
        self.admin_action_reason_code = Some(value);
        self
    }

    /// Sets an operator audit reason of at most 8,192 characters.
    pub fn admin_action_reason(mut self, value: impl Into<String>) -> Self {
        self.admin_action_reason = Some(value.into());
        self
    }

    /// Validates required fields and cross-field reason rules.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidRequest`] for a missing field, an undocumented
    /// reason code, a missing `Other` reason, or a reason over 8,192 characters.
    pub fn build(self) -> Result<SetAdminAutoLiqActionRequest, Error> {
        let request = SetAdminAutoLiqActionRequest {
            account_id: self.account_id.ok_or(Error::InvalidRequest {
                field: "accountId",
                reason: "is required",
            })?,
            admin_action: self.admin_action.ok_or(Error::InvalidRequest {
                field: "adminAction",
                reason: "is required",
            })?,
            admin_action_reason_code: self.admin_action_reason_code.ok_or(
                Error::InvalidRequest {
                    field: "adminActionReasonCode",
                    reason: "is required",
                },
            )?,
            admin_action_reason: self.admin_action_reason,
        };
        request.validate_current()?;
        Ok(request)
    }
}

impl DocumentedMutationResponse for AccountRiskStatusResponse {
    fn mutation_outcome(&self) -> MutationOutcome {
        MutationOutcome::Ambiguous
    }

    fn has_success_evidence(&self) -> bool {
        self.account_risk_status().is_some()
    }
}

impl Client {
    /// Resets one account's auto-liquidation status.
    ///
    /// # Errors
    ///
    /// Returns validation, authentication, rate, transport, provider-control,
    /// decoding, or ambiguous-mutation errors. The response does not echo the
    /// account ID, so an apparent success remains latched pending authoritative
    /// account-risk-status reconciliation.
    pub async fn account_risk_status_reset_auto_liq_status(
        &self,
        request: &ResetAutoLiqStatus,
    ) -> Result<AccountRiskStatusResponse, Error> {
        self.post_unresolved_mutation("/accountRiskStatus/resetautoliqstatus", request)
            .await
    }

    /// Sets a persistent partner action for one demo account.
    ///
    /// This endpoint requires organization-administrator authority. Internal
    /// provider actions cannot be represented by the request type.
    ///
    /// # Errors
    ///
    /// Returns a local error outside the demo environment or for an invalid
    /// reason, plus authentication, rate, transport, provider-control,
    /// decoding, or ambiguity errors. The response cannot correlate its status
    /// object to the request account, so apparent success requires reconciliation.
    pub async fn account_risk_status_set_admin_auto_liq_action(
        &self,
        request: &SetAdminAutoLiqActionRequest,
    ) -> Result<AccountRiskStatusResponse, Error> {
        request.validate_current()?;
        if !self.endpoints.permits_demo_only_rest() {
            return Err(Error::InvalidRequest {
                field: "environment",
                reason: "setAdminAutoLiqAction requires the demo REST environment",
            });
        }
        self.post_unresolved_mutation("/accountRiskStatus/setadminautoliqaction", request)
            .await
    }

    /// Switches 1 to 10,000 unique accounts to one risk category.
    ///
    /// The current endpoint is limited to 5,000 calls per hour and carries a
    /// one-second provider back-off. Call it outside trading hours or only when
    /// affected accounts have no open positions.
    ///
    /// # Errors
    ///
    /// Returns validation, authentication, rate, transport, provider-control,
    /// decoding, rejection, or ambiguity errors. `ok: true` does not identify
    /// which accounts changed, so it remains latched pending reconciliation.
    pub async fn account_risk_status_switch_risk_category(
        &self,
        request: &SwitchRiskCategory,
    ) -> Result<crate::api::current::users::SimpleResponse, Error> {
        validate_switch_accounts(request)?;
        self.post_reviewed_mutation(
            "/accountRiskStatus/switchriskcategory",
            request,
            assess_switch_risk_category,
        )
        .await
    }

    /// Updates one account's maximum net liquidation threshold.
    ///
    /// # Errors
    ///
    /// Returns validation, authentication, rate, transport, provider-control,
    /// decoding, or ambiguity errors. Because the response omits the request
    /// account identity, apparent success requires account-scoped reconciliation.
    pub async fn account_risk_status_update_max_net_liq(
        &self,
        request: &UpdateMaxNetLiq,
    ) -> Result<AccountRiskStatusResponse, Error> {
        self.post_unresolved_mutation("/accountRiskStatus/updatemaxnetliq", request)
            .await
    }
}

fn validate_admin_reason(
    code: &SetAdminAutoLiqActionAdminActionReasonCode,
    reason: Option<&str>,
) -> Result<(), Error> {
    if matches!(code, SetAdminAutoLiqActionAdminActionReasonCode::Unknown(_)) {
        return Err(Error::InvalidRequest {
            field: "adminActionReasonCode",
            reason: "must be a documented current provider value",
        });
    }
    if reason.is_some_and(|value| value.chars().count() > MAX_ADMIN_REASON_CHARS) {
        return Err(Error::InvalidRequest {
            field: "adminActionReason",
            reason: "must not exceed 8192 characters",
        });
    }
    if matches!(code, SetAdminAutoLiqActionAdminActionReasonCode::Other)
        && reason.is_none_or(|value| value.trim().is_empty())
    {
        return Err(Error::InvalidRequest {
            field: "adminActionReason",
            reason: "must be nonempty when adminActionReasonCode is Other",
        });
    }
    Ok(())
}

fn validate_switch_accounts(request: &SwitchRiskCategory) -> Result<(), Error> {
    let accounts = request.account_ids();
    if accounts.is_empty() || accounts.len() > MAX_SWITCH_ACCOUNTS {
        return Err(Error::InvalidRequest {
            field: "accountIds",
            reason: "must contain between 1 and 10000 accounts",
        });
    }
    let mut unique = HashSet::with_capacity(accounts.len());
    if !accounts
        .iter()
        .copied()
        .all(|account| unique.insert(account))
    {
        return Err(Error::InvalidRequest {
            field: "accountIds",
            reason: "must contain unique account identities",
        });
    }
    Ok(())
}

fn assess_switch_risk_category(
    response: &crate::api::current::users::SimpleResponse,
    _: &SwitchRiskCategory,
) -> MutationAssessment {
    if *response.ok() {
        MutationAssessment::ambiguous(true)
    } else {
        MutationAssessment::rejected()
    }
}

#[cfg(test)]
#[path = "account_status/tests.rs"]
mod tests;

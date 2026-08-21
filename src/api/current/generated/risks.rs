// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary
// @generated
// Generator: tools/generate_openapi.py
// Source: https://partner.tradovate.com/openapi.json (snapshot 2026-08-21, sha256 37caeccf4b0913460a788fcaf4c902497059b8ffe6f6355512e6c08eaacde769)

// Provider wire fields remain schema-auditable even when they repeat
// their type name; wide schema-faithful builders remain one generated
// unit so regeneration and source review cannot drift field subsets.
#![allow(clippy::struct_field_names, clippy::too_many_lines)]

//! Current risk-control operations and wire models.

/// Current wire model `AccountRiskStatusResponse`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct AccountRiskStatusResponse {
    #[serde(rename = "errorText", default, skip_serializing_if = "Option::is_none")]
    error_text: Option<String>,
    #[serde(
        rename = "accountRiskStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    account_risk_status: Option<super::users::AccountRiskStatus>,
}

impl AccountRiskStatusResponse {
    /// Returns wire field `errorText`.
    #[must_use]
    pub fn error_text(&self) -> Option<&str> {
        self.error_text.as_deref()
    }

    /// Returns wire field `accountRiskStatus`.
    #[must_use]
    pub fn account_risk_status(&self) -> Option<&super::users::AccountRiskStatus> {
        self.account_risk_status.as_ref()
    }

    /// Starts a builder for [`AccountRiskStatusResponse`].
    pub fn builder() -> AccountRiskStatusResponseBuilder {
        AccountRiskStatusResponseBuilder::default()
    }
}

/// Builder for [`AccountRiskStatusResponse`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct AccountRiskStatusResponseBuilder {
    error_text: Option<String>,
    account_risk_status: Option<super::users::AccountRiskStatus>,
}

impl AccountRiskStatusResponseBuilder {
    /// Sets wire field `errorText`.
    pub fn error_text(mut self, value: impl Into<String>) -> Self {
        self.error_text = Some(value.into());
        self
    }

    /// Sets wire field `accountRiskStatus`.
    pub fn account_risk_status(mut self, value: super::users::AccountRiskStatus) -> Self {
        self.account_risk_status = Some(value);
        self
    }

    /// Validates required fields and builds [`AccountRiskStatusResponse`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<AccountRiskStatusResponse, crate::api::current::BuildError> {
        Ok(AccountRiskStatusResponse {
            error_text: self.error_text,
            account_risk_status: self.account_risk_status,
        })
    }
}

/// Current wire model `ContractMargin`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ContractMargin {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    id: Option<super::ids::ContractMarginId>,
    #[serde(rename = "initialMargin")]
    #[serde(with = "crate::decimal")]
    initial_margin: crate::Decimal,
    #[serde(rename = "maintenanceMargin")]
    #[serde(with = "crate::decimal")]
    maintenance_margin: crate::Decimal,
    #[serde(rename = "timestamp")]
    timestamp: jiff::Timestamp,
}

impl ContractMargin {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> Option<&super::ids::ContractMarginId> {
        self.id.as_ref()
    }

    /// Returns wire field `initialMargin`.
    #[must_use]
    pub fn initial_margin(&self) -> &crate::Decimal {
        &self.initial_margin
    }

    /// Returns wire field `maintenanceMargin`.
    #[must_use]
    pub fn maintenance_margin(&self) -> &crate::Decimal {
        &self.maintenance_margin
    }

    /// Returns wire field `timestamp`.
    #[must_use]
    pub fn timestamp(&self) -> &jiff::Timestamp {
        &self.timestamp
    }

    /// Starts a builder for [`ContractMargin`].
    pub fn builder() -> ContractMarginBuilder {
        ContractMarginBuilder::default()
    }
}

/// Builder for [`ContractMargin`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ContractMarginBuilder {
    id: Option<super::ids::ContractMarginId>,
    initial_margin: Option<crate::Decimal>,
    maintenance_margin: Option<crate::Decimal>,
    timestamp: Option<jiff::Timestamp>,
}

impl ContractMarginBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::ContractMarginId) -> Self {
        self.id = Some(value);
        self
    }

    /// Sets wire field `initialMargin`.
    pub fn initial_margin(mut self, value: crate::Decimal) -> Self {
        self.initial_margin = Some(value);
        self
    }

    /// Sets wire field `maintenanceMargin`.
    pub fn maintenance_margin(mut self, value: crate::Decimal) -> Self {
        self.maintenance_margin = Some(value);
        self
    }

    /// Sets wire field `timestamp`.
    pub fn timestamp(mut self, value: jiff::Timestamp) -> Self {
        self.timestamp = Some(value);
        self
    }

    /// Validates required fields and builds [`ContractMargin`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ContractMargin, crate::api::current::BuildError> {
        let initial_margin = self
            .initial_margin
            .ok_or(crate::api::current::BuildError::missing("initialMargin"))?;
        let maintenance_margin =
            self.maintenance_margin
                .ok_or(crate::api::current::BuildError::missing(
                    "maintenanceMargin",
                ))?;
        let timestamp = self
            .timestamp
            .ok_or(crate::api::current::BuildError::missing("timestamp"))?;
        Ok(ContractMargin {
            id: self.id,
            initial_margin,
            maintenance_margin,
            timestamp,
        })
    }
}

/// Current wire model `DeleteResultResponse`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct DeleteResultResponse {
    #[serde(rename = "errorText", default, skip_serializing_if = "Option::is_none")]
    error_text: Option<String>,
    #[serde(rename = "success", default, skip_serializing_if = "Option::is_none")]
    success: Option<bool>,
}

impl DeleteResultResponse {
    /// Returns wire field `errorText`.
    #[must_use]
    pub fn error_text(&self) -> Option<&str> {
        self.error_text.as_deref()
    }

    /// Returns wire field `success`.
    #[must_use]
    pub fn success(&self) -> Option<&bool> {
        self.success.as_ref()
    }

    /// Starts a builder for [`DeleteResultResponse`].
    pub fn builder() -> DeleteResultResponseBuilder {
        DeleteResultResponseBuilder::default()
    }
}

/// Builder for [`DeleteResultResponse`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct DeleteResultResponseBuilder {
    error_text: Option<String>,
    success: Option<bool>,
}

impl DeleteResultResponseBuilder {
    /// Sets wire field `errorText`.
    pub fn error_text(mut self, value: impl Into<String>) -> Self {
        self.error_text = Some(value.into());
        self
    }

    /// Sets wire field `success`.
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Validates required fields and builds [`DeleteResultResponse`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<DeleteResultResponse, crate::api::current::BuildError> {
        Ok(DeleteResultResponse {
            error_text: self.error_text,
            success: self.success,
        })
    }
}

/// Current wire model `DeleteUserAccountPositionLimit`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct DeleteUserAccountPositionLimit {
    #[serde(rename = "userAccountPositionLimitId")]
    user_account_position_limit_id: super::ids::UserAccountPositionLimitId,
}

impl DeleteUserAccountPositionLimit {
    /// Returns wire field `userAccountPositionLimitId`.
    #[must_use]
    pub fn user_account_position_limit_id(&self) -> &super::ids::UserAccountPositionLimitId {
        &self.user_account_position_limit_id
    }

    /// Starts a builder for [`DeleteUserAccountPositionLimit`].
    pub fn builder() -> DeleteUserAccountPositionLimitBuilder {
        DeleteUserAccountPositionLimitBuilder::default()
    }
}

/// Builder for [`DeleteUserAccountPositionLimit`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct DeleteUserAccountPositionLimitBuilder {
    user_account_position_limit_id: Option<super::ids::UserAccountPositionLimitId>,
}

impl DeleteUserAccountPositionLimitBuilder {
    /// Sets wire field `userAccountPositionLimitId`.
    pub fn user_account_position_limit_id(
        mut self,
        value: super::ids::UserAccountPositionLimitId,
    ) -> Self {
        self.user_account_position_limit_id = Some(value);
        self
    }

    /// Validates required fields and builds [`DeleteUserAccountPositionLimit`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<DeleteUserAccountPositionLimit, crate::api::current::BuildError> {
        let user_account_position_limit_id =
            self.user_account_position_limit_id
                .ok_or(crate::api::current::BuildError::missing(
                    "userAccountPositionLimitId",
                ))?;
        Ok(DeleteUserAccountPositionLimit {
            user_account_position_limit_id,
        })
    }
}

impl crate::api::current::support::CurrentRequest for DeleteUserAccountPositionLimit {
    fn validate_current(&self) -> Result<(), crate::Error> {
        Ok(())
    }
}

/// Current wire model `DeleteUserAccountRiskParameter`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct DeleteUserAccountRiskParameter {
    #[serde(rename = "userAccountRiskParameterId")]
    user_account_risk_parameter_id: super::ids::UserAccountRiskParameterId,
}

impl DeleteUserAccountRiskParameter {
    /// Returns wire field `userAccountRiskParameterId`.
    #[must_use]
    pub fn user_account_risk_parameter_id(&self) -> &super::ids::UserAccountRiskParameterId {
        &self.user_account_risk_parameter_id
    }

    /// Starts a builder for [`DeleteUserAccountRiskParameter`].
    pub fn builder() -> DeleteUserAccountRiskParameterBuilder {
        DeleteUserAccountRiskParameterBuilder::default()
    }
}

/// Builder for [`DeleteUserAccountRiskParameter`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct DeleteUserAccountRiskParameterBuilder {
    user_account_risk_parameter_id: Option<super::ids::UserAccountRiskParameterId>,
}

impl DeleteUserAccountRiskParameterBuilder {
    /// Sets wire field `userAccountRiskParameterId`.
    pub fn user_account_risk_parameter_id(
        mut self,
        value: super::ids::UserAccountRiskParameterId,
    ) -> Self {
        self.user_account_risk_parameter_id = Some(value);
        self
    }

    /// Validates required fields and builds [`DeleteUserAccountRiskParameter`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<DeleteUserAccountRiskParameter, crate::api::current::BuildError> {
        let user_account_risk_parameter_id =
            self.user_account_risk_parameter_id
                .ok_or(crate::api::current::BuildError::missing(
                    "userAccountRiskParameterId",
                ))?;
        Ok(DeleteUserAccountRiskParameter {
            user_account_risk_parameter_id,
        })
    }
}

impl crate::api::current::support::CurrentRequest for DeleteUserAccountRiskParameter {
    fn validate_current(&self) -> Result<(), crate::Error> {
        Ok(())
    }
}

/// Current wire model `PermissionedAccountAutoLiq`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct PermissionedAccountAutoLiq {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    id: Option<super::ids::PermissionedAccountAutoLiqId>,
    #[serde(
        rename = "marginPercentageAlert",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    margin_percentage_alert: Option<crate::Decimal>,
    #[serde(
        rename = "dailyLossPercentageAlert",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    daily_loss_percentage_alert: Option<crate::Decimal>,
    #[serde(
        rename = "dailyLossAlert",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    daily_loss_alert: Option<crate::Decimal>,
    #[serde(
        rename = "marginPercentageLiqOnly",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    margin_percentage_liq_only: Option<crate::Decimal>,
    #[serde(
        rename = "dailyLossPercentageLiqOnly",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    daily_loss_percentage_liq_only: Option<crate::Decimal>,
    #[serde(
        rename = "dailyLossLiqOnly",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    daily_loss_liq_only: Option<crate::Decimal>,
    #[serde(
        rename = "marginPercentageAutoLiq",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    margin_percentage_auto_liq: Option<crate::Decimal>,
    #[serde(
        rename = "dailyLossPercentageAutoLiq",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    daily_loss_percentage_auto_liq: Option<crate::Decimal>,
    #[serde(
        rename = "dailyLossAutoLiq",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    daily_loss_auto_liq: Option<crate::Decimal>,
    #[serde(
        rename = "weeklyLossAutoLiq",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    weekly_loss_auto_liq: Option<crate::Decimal>,
    #[serde(
        rename = "dailyProfitAutoLiq",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    daily_profit_auto_liq: Option<crate::Decimal>,
    #[serde(
        rename = "weeklyProfitAutoLiq",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    weekly_profit_auto_liq: Option<crate::Decimal>,
    #[serde(
        rename = "changesLocked",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    changes_locked: Option<bool>,
}

impl PermissionedAccountAutoLiq {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> Option<&super::ids::PermissionedAccountAutoLiqId> {
        self.id.as_ref()
    }

    /// Returns wire field `marginPercentageAlert`.
    #[must_use]
    pub fn margin_percentage_alert(&self) -> Option<&crate::Decimal> {
        self.margin_percentage_alert.as_ref()
    }

    /// Returns wire field `dailyLossPercentageAlert`.
    #[must_use]
    pub fn daily_loss_percentage_alert(&self) -> Option<&crate::Decimal> {
        self.daily_loss_percentage_alert.as_ref()
    }

    /// Returns wire field `dailyLossAlert`.
    #[must_use]
    pub fn daily_loss_alert(&self) -> Option<&crate::Decimal> {
        self.daily_loss_alert.as_ref()
    }

    /// Returns wire field `marginPercentageLiqOnly`.
    #[must_use]
    pub fn margin_percentage_liq_only(&self) -> Option<&crate::Decimal> {
        self.margin_percentage_liq_only.as_ref()
    }

    /// Returns wire field `dailyLossPercentageLiqOnly`.
    #[must_use]
    pub fn daily_loss_percentage_liq_only(&self) -> Option<&crate::Decimal> {
        self.daily_loss_percentage_liq_only.as_ref()
    }

    /// Returns wire field `dailyLossLiqOnly`.
    #[must_use]
    pub fn daily_loss_liq_only(&self) -> Option<&crate::Decimal> {
        self.daily_loss_liq_only.as_ref()
    }

    /// Returns wire field `marginPercentageAutoLiq`.
    #[must_use]
    pub fn margin_percentage_auto_liq(&self) -> Option<&crate::Decimal> {
        self.margin_percentage_auto_liq.as_ref()
    }

    /// Returns wire field `dailyLossPercentageAutoLiq`.
    #[must_use]
    pub fn daily_loss_percentage_auto_liq(&self) -> Option<&crate::Decimal> {
        self.daily_loss_percentage_auto_liq.as_ref()
    }

    /// Returns wire field `dailyLossAutoLiq`.
    #[must_use]
    pub fn daily_loss_auto_liq(&self) -> Option<&crate::Decimal> {
        self.daily_loss_auto_liq.as_ref()
    }

    /// Returns wire field `weeklyLossAutoLiq`.
    #[must_use]
    pub fn weekly_loss_auto_liq(&self) -> Option<&crate::Decimal> {
        self.weekly_loss_auto_liq.as_ref()
    }

    /// Returns wire field `dailyProfitAutoLiq`.
    #[must_use]
    pub fn daily_profit_auto_liq(&self) -> Option<&crate::Decimal> {
        self.daily_profit_auto_liq.as_ref()
    }

    /// Returns wire field `weeklyProfitAutoLiq`.
    #[must_use]
    pub fn weekly_profit_auto_liq(&self) -> Option<&crate::Decimal> {
        self.weekly_profit_auto_liq.as_ref()
    }

    /// Returns wire field `changesLocked`.
    #[must_use]
    pub fn changes_locked(&self) -> Option<&bool> {
        self.changes_locked.as_ref()
    }

    /// Starts a builder for [`PermissionedAccountAutoLiq`].
    pub fn builder() -> PermissionedAccountAutoLiqBuilder {
        PermissionedAccountAutoLiqBuilder::default()
    }
}

/// Builder for [`PermissionedAccountAutoLiq`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct PermissionedAccountAutoLiqBuilder {
    id: Option<super::ids::PermissionedAccountAutoLiqId>,
    margin_percentage_alert: Option<crate::Decimal>,
    daily_loss_percentage_alert: Option<crate::Decimal>,
    daily_loss_alert: Option<crate::Decimal>,
    margin_percentage_liq_only: Option<crate::Decimal>,
    daily_loss_percentage_liq_only: Option<crate::Decimal>,
    daily_loss_liq_only: Option<crate::Decimal>,
    margin_percentage_auto_liq: Option<crate::Decimal>,
    daily_loss_percentage_auto_liq: Option<crate::Decimal>,
    daily_loss_auto_liq: Option<crate::Decimal>,
    weekly_loss_auto_liq: Option<crate::Decimal>,
    daily_profit_auto_liq: Option<crate::Decimal>,
    weekly_profit_auto_liq: Option<crate::Decimal>,
    changes_locked: Option<bool>,
}

impl PermissionedAccountAutoLiqBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::PermissionedAccountAutoLiqId) -> Self {
        self.id = Some(value);
        self
    }

    /// Sets wire field `marginPercentageAlert`.
    pub fn margin_percentage_alert(mut self, value: crate::Decimal) -> Self {
        self.margin_percentage_alert = Some(value);
        self
    }

    /// Sets wire field `dailyLossPercentageAlert`.
    pub fn daily_loss_percentage_alert(mut self, value: crate::Decimal) -> Self {
        self.daily_loss_percentage_alert = Some(value);
        self
    }

    /// Sets wire field `dailyLossAlert`.
    pub fn daily_loss_alert(mut self, value: crate::Decimal) -> Self {
        self.daily_loss_alert = Some(value);
        self
    }

    /// Sets wire field `marginPercentageLiqOnly`.
    pub fn margin_percentage_liq_only(mut self, value: crate::Decimal) -> Self {
        self.margin_percentage_liq_only = Some(value);
        self
    }

    /// Sets wire field `dailyLossPercentageLiqOnly`.
    pub fn daily_loss_percentage_liq_only(mut self, value: crate::Decimal) -> Self {
        self.daily_loss_percentage_liq_only = Some(value);
        self
    }

    /// Sets wire field `dailyLossLiqOnly`.
    pub fn daily_loss_liq_only(mut self, value: crate::Decimal) -> Self {
        self.daily_loss_liq_only = Some(value);
        self
    }

    /// Sets wire field `marginPercentageAutoLiq`.
    pub fn margin_percentage_auto_liq(mut self, value: crate::Decimal) -> Self {
        self.margin_percentage_auto_liq = Some(value);
        self
    }

    /// Sets wire field `dailyLossPercentageAutoLiq`.
    pub fn daily_loss_percentage_auto_liq(mut self, value: crate::Decimal) -> Self {
        self.daily_loss_percentage_auto_liq = Some(value);
        self
    }

    /// Sets wire field `dailyLossAutoLiq`.
    pub fn daily_loss_auto_liq(mut self, value: crate::Decimal) -> Self {
        self.daily_loss_auto_liq = Some(value);
        self
    }

    /// Sets wire field `weeklyLossAutoLiq`.
    pub fn weekly_loss_auto_liq(mut self, value: crate::Decimal) -> Self {
        self.weekly_loss_auto_liq = Some(value);
        self
    }

    /// Sets wire field `dailyProfitAutoLiq`.
    pub fn daily_profit_auto_liq(mut self, value: crate::Decimal) -> Self {
        self.daily_profit_auto_liq = Some(value);
        self
    }

    /// Sets wire field `weeklyProfitAutoLiq`.
    pub fn weekly_profit_auto_liq(mut self, value: crate::Decimal) -> Self {
        self.weekly_profit_auto_liq = Some(value);
        self
    }

    /// Sets wire field `changesLocked`.
    pub fn changes_locked(mut self, value: bool) -> Self {
        self.changes_locked = Some(value);
        self
    }

    /// Validates required fields and builds [`PermissionedAccountAutoLiq`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<PermissionedAccountAutoLiq, crate::api::current::BuildError> {
        Ok(PermissionedAccountAutoLiq {
            id: self.id,
            margin_percentage_alert: self.margin_percentage_alert,
            daily_loss_percentage_alert: self.daily_loss_percentage_alert,
            daily_loss_alert: self.daily_loss_alert,
            margin_percentage_liq_only: self.margin_percentage_liq_only,
            daily_loss_percentage_liq_only: self.daily_loss_percentage_liq_only,
            daily_loss_liq_only: self.daily_loss_liq_only,
            margin_percentage_auto_liq: self.margin_percentage_auto_liq,
            daily_loss_percentage_auto_liq: self.daily_loss_percentage_auto_liq,
            daily_loss_auto_liq: self.daily_loss_auto_liq,
            weekly_loss_auto_liq: self.weekly_loss_auto_liq,
            daily_profit_auto_liq: self.daily_profit_auto_liq,
            weekly_profit_auto_liq: self.weekly_profit_auto_liq,
            changes_locked: self.changes_locked,
        })
    }
}

/// Current wire model `ProductMargin`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ProductMargin {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    id: Option<super::ids::ProductMarginId>,
    #[serde(rename = "initialMargin")]
    #[serde(with = "crate::decimal")]
    initial_margin: crate::Decimal,
    #[serde(rename = "maintenanceMargin")]
    #[serde(with = "crate::decimal")]
    maintenance_margin: crate::Decimal,
    #[serde(rename = "timestamp")]
    timestamp: jiff::Timestamp,
}

impl ProductMargin {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> Option<&super::ids::ProductMarginId> {
        self.id.as_ref()
    }

    /// Returns wire field `initialMargin`.
    #[must_use]
    pub fn initial_margin(&self) -> &crate::Decimal {
        &self.initial_margin
    }

    /// Returns wire field `maintenanceMargin`.
    #[must_use]
    pub fn maintenance_margin(&self) -> &crate::Decimal {
        &self.maintenance_margin
    }

    /// Returns wire field `timestamp`.
    #[must_use]
    pub fn timestamp(&self) -> &jiff::Timestamp {
        &self.timestamp
    }

    /// Starts a builder for [`ProductMargin`].
    pub fn builder() -> ProductMarginBuilder {
        ProductMarginBuilder::default()
    }
}

/// Builder for [`ProductMargin`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ProductMarginBuilder {
    id: Option<super::ids::ProductMarginId>,
    initial_margin: Option<crate::Decimal>,
    maintenance_margin: Option<crate::Decimal>,
    timestamp: Option<jiff::Timestamp>,
}

impl ProductMarginBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::ProductMarginId) -> Self {
        self.id = Some(value);
        self
    }

    /// Sets wire field `initialMargin`.
    pub fn initial_margin(mut self, value: crate::Decimal) -> Self {
        self.initial_margin = Some(value);
        self
    }

    /// Sets wire field `maintenanceMargin`.
    pub fn maintenance_margin(mut self, value: crate::Decimal) -> Self {
        self.maintenance_margin = Some(value);
        self
    }

    /// Sets wire field `timestamp`.
    pub fn timestamp(mut self, value: jiff::Timestamp) -> Self {
        self.timestamp = Some(value);
        self
    }

    /// Validates required fields and builds [`ProductMargin`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ProductMargin, crate::api::current::BuildError> {
        let initial_margin = self
            .initial_margin
            .ok_or(crate::api::current::BuildError::missing("initialMargin"))?;
        let maintenance_margin =
            self.maintenance_margin
                .ok_or(crate::api::current::BuildError::missing(
                    "maintenanceMargin",
                ))?;
        let timestamp = self
            .timestamp
            .ok_or(crate::api::current::BuildError::missing("timestamp"))?;
        Ok(ProductMargin {
            id: self.id,
            initial_margin,
            maintenance_margin,
            timestamp,
        })
    }
}

/// Current wire model `ResetAutoLiqStatus`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ResetAutoLiqStatus {
    #[serde(rename = "accountId")]
    account_id: crate::AccountId,
}

impl ResetAutoLiqStatus {
    /// Returns wire field `accountId`.
    #[must_use]
    pub fn account_id(&self) -> &crate::AccountId {
        &self.account_id
    }

    /// Starts a builder for [`ResetAutoLiqStatus`].
    pub fn builder() -> ResetAutoLiqStatusBuilder {
        ResetAutoLiqStatusBuilder::default()
    }
}

/// Builder for [`ResetAutoLiqStatus`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ResetAutoLiqStatusBuilder {
    account_id: Option<crate::AccountId>,
}

impl ResetAutoLiqStatusBuilder {
    /// Sets wire field `accountId`.
    pub fn account_id(mut self, value: crate::AccountId) -> Self {
        self.account_id = Some(value);
        self
    }

    /// Validates required fields and builds [`ResetAutoLiqStatus`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ResetAutoLiqStatus, crate::api::current::BuildError> {
        let account_id = self
            .account_id
            .ok_or(crate::api::current::BuildError::missing("accountId"))?;
        Ok(ResetAutoLiqStatus { account_id })
    }
}

impl crate::api::current::support::CurrentRequest for ResetAutoLiqStatus {
    fn validate_current(&self) -> Result<(), crate::Error> {
        Ok(())
    }
}

/// Current wire model `SetAccountNotes`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct SetAccountNotes {
    #[serde(rename = "accountId")]
    account_id: crate::AccountId,
}

impl SetAccountNotes {
    /// Returns wire field `accountId`.
    #[must_use]
    pub fn account_id(&self) -> &crate::AccountId {
        &self.account_id
    }

    /// Starts a builder for [`SetAccountNotes`].
    pub fn builder() -> SetAccountNotesBuilder {
        SetAccountNotesBuilder::default()
    }
}

/// Builder for [`SetAccountNotes`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct SetAccountNotesBuilder {
    account_id: Option<crate::AccountId>,
}

impl SetAccountNotesBuilder {
    /// Sets wire field `accountId`.
    pub fn account_id(mut self, value: crate::AccountId) -> Self {
        self.account_id = Some(value);
        self
    }

    /// Validates required fields and builds [`SetAccountNotes`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<SetAccountNotes, crate::api::current::BuildError> {
        let account_id = self
            .account_id
            .ok_or(crate::api::current::BuildError::missing("accountId"))?;
        Ok(SetAccountNotes { account_id })
    }
}

impl crate::api::current::support::CurrentRequest for SetAccountNotes {
    fn validate_current(&self) -> Result<(), crate::Error> {
        Ok(())
    }
}

/// Current wire model `SetAdminAutoLiqAction`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct SetAdminAutoLiqAction {
    #[serde(rename = "accountId")]
    account_id: crate::AccountId,
    #[serde(rename = "adminAction")]
    admin_action: SetAdminAutoLiqActionAdminAction,
    #[serde(rename = "adminActionReasonCode")]
    admin_action_reason_code: SetAdminAutoLiqActionAdminActionReasonCode,
    #[serde(
        rename = "adminActionReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    admin_action_reason: Option<String>,
}

impl SetAdminAutoLiqAction {
    /// Returns wire field `accountId`.
    #[must_use]
    pub fn account_id(&self) -> &crate::AccountId {
        &self.account_id
    }

    /// Returns wire field `adminAction`.
    #[must_use]
    pub fn admin_action(&self) -> &SetAdminAutoLiqActionAdminAction {
        &self.admin_action
    }

    /// Returns wire field `adminActionReasonCode`.
    #[must_use]
    pub fn admin_action_reason_code(&self) -> &SetAdminAutoLiqActionAdminActionReasonCode {
        &self.admin_action_reason_code
    }

    /// Returns wire field `adminActionReason`.
    #[must_use]
    pub fn admin_action_reason(&self) -> Option<&str> {
        self.admin_action_reason.as_deref()
    }

    /// Starts a builder for [`SetAdminAutoLiqAction`].
    pub fn builder() -> SetAdminAutoLiqActionBuilder {
        SetAdminAutoLiqActionBuilder::default()
    }
}

/// Builder for [`SetAdminAutoLiqAction`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct SetAdminAutoLiqActionBuilder {
    account_id: Option<crate::AccountId>,
    admin_action: Option<SetAdminAutoLiqActionAdminAction>,
    admin_action_reason_code: Option<SetAdminAutoLiqActionAdminActionReasonCode>,
    admin_action_reason: Option<String>,
}

impl SetAdminAutoLiqActionBuilder {
    /// Sets wire field `accountId`.
    pub fn account_id(mut self, value: crate::AccountId) -> Self {
        self.account_id = Some(value);
        self
    }

    /// Sets wire field `adminAction`.
    pub fn admin_action(mut self, value: SetAdminAutoLiqActionAdminAction) -> Self {
        self.admin_action = Some(value);
        self
    }

    /// Sets wire field `adminActionReasonCode`.
    pub fn admin_action_reason_code(
        mut self,
        value: SetAdminAutoLiqActionAdminActionReasonCode,
    ) -> Self {
        self.admin_action_reason_code = Some(value);
        self
    }

    /// Sets wire field `adminActionReason`.
    pub fn admin_action_reason(mut self, value: impl Into<String>) -> Self {
        self.admin_action_reason = Some(value.into());
        self
    }

    /// Validates required fields and builds [`SetAdminAutoLiqAction`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<SetAdminAutoLiqAction, crate::api::current::BuildError> {
        let account_id = self
            .account_id
            .ok_or(crate::api::current::BuildError::missing("accountId"))?;
        let admin_action = self
            .admin_action
            .ok_or(crate::api::current::BuildError::missing("adminAction"))?;
        let admin_action_reason_code =
            self.admin_action_reason_code
                .ok_or(crate::api::current::BuildError::missing(
                    "adminActionReasonCode",
                ))?;
        Ok(SetAdminAutoLiqAction {
            account_id,
            admin_action,
            admin_action_reason_code,
            admin_action_reason: self.admin_action_reason,
        })
    }
}

impl crate::api::current::support::CurrentRequest for SetAdminAutoLiqAction {
    fn validate_current(&self) -> Result<(), crate::Error> {
        Ok(())
    }
}

/// Current provider values for `SetAdminAutoLiqActionAdminAction`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum SetAdminAutoLiqActionAdminAction {
    /// Provider value `AgreedOnLiqOnlyModeByAutoLiq`.
    AgreedOnLiqOnlyModeByAutoLiq,
    /// Provider value `AgreedOnLiquidationByAutoLiq`.
    AgreedOnLiquidationByAutoLiq,
    /// Provider value `DisableAutoLiq`.
    DisableAutoLiq,
    /// Provider value `LiquidateImmediately`.
    LiquidateImmediately,
    /// Provider value `LiquidateOnlyModeImmediately`.
    LiquidateOnlyModeImmediately,
    /// Provider value `LockTradingImmediately`.
    LockTradingImmediately,
    /// Provider value `Normal`.
    Normal,
    /// Provider value `PlaceAutoLiqOnHold`.
    PlaceAutoLiqOnHold,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl SetAdminAutoLiqActionAdminAction {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::AgreedOnLiqOnlyModeByAutoLiq => "AgreedOnLiqOnlyModeByAutoLiq",
            Self::AgreedOnLiquidationByAutoLiq => "AgreedOnLiquidationByAutoLiq",
            Self::DisableAutoLiq => "DisableAutoLiq",
            Self::LiquidateImmediately => "LiquidateImmediately",
            Self::LiquidateOnlyModeImmediately => "LiquidateOnlyModeImmediately",
            Self::LockTradingImmediately => "LockTradingImmediately",
            Self::Normal => "Normal",
            Self::PlaceAutoLiqOnHold => "PlaceAutoLiqOnHold",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for SetAdminAutoLiqActionAdminAction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if matches!(self, Self::Unknown(_)) {
            return Err(serde::ser::Error::custom(
                "undocumented enum values cannot be sent",
            ));
        }
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for SetAdminAutoLiqActionAdminAction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "AgreedOnLiqOnlyModeByAutoLiq" => Self::AgreedOnLiqOnlyModeByAutoLiq,
            "AgreedOnLiquidationByAutoLiq" => Self::AgreedOnLiquidationByAutoLiq,
            "DisableAutoLiq" => Self::DisableAutoLiq,
            "LiquidateImmediately" => Self::LiquidateImmediately,
            "LiquidateOnlyModeImmediately" => Self::LiquidateOnlyModeImmediately,
            "LockTradingImmediately" => Self::LockTradingImmediately,
            "Normal" => Self::Normal,
            "PlaceAutoLiqOnHold" => Self::PlaceAutoLiqOnHold,
            _ => Self::Unknown(value),
        })
    }
}

/// Current provider values for `SetAdminAutoLiqActionAdminActionReasonCode`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum SetAdminAutoLiqActionAdminActionReasonCode {
    /// Provider value `CloseAMLDecision`.
    CloseAmlDecision,
    /// Provider value `CloseComplianceDecision`.
    CloseComplianceDecision,
    /// Provider value `CloseCustomerRequest`.
    CloseCustomerRequest,
    /// Provider value `CloseDeceased`.
    CloseDeceased,
    /// Provider value `CloseEscheatment`.
    CloseEscheatment,
    /// Provider value `CloseFraud`.
    CloseFraud,
    /// Provider value `CloseInactiveUnfundedAccount`.
    CloseInactiveUnfundedAccount,
    /// Provider value `FullBalanceWithdrawal`.
    FullBalanceWithdrawal,
    /// Provider value `Other`.
    Other,
    /// Provider value `RestrictedAMLRequest`.
    RestrictedAmlRequest,
    /// Provider value `RestrictedAccountsRequest`.
    RestrictedAccountsRequest,
    /// Provider value `RestrictedComplianceRequest`.
    RestrictedComplianceRequest,
    /// Provider value `RestrictedRiskRestriction`.
    RestrictedRiskRestriction,
    /// Provider value `RestrictedTreasuryRequest`.
    RestrictedTreasuryRequest,
    /// Provider value `RestrictedW8Expired`.
    RestrictedW8Expired,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl SetAdminAutoLiqActionAdminActionReasonCode {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::CloseAmlDecision => "CloseAMLDecision",
            Self::CloseComplianceDecision => "CloseComplianceDecision",
            Self::CloseCustomerRequest => "CloseCustomerRequest",
            Self::CloseDeceased => "CloseDeceased",
            Self::CloseEscheatment => "CloseEscheatment",
            Self::CloseFraud => "CloseFraud",
            Self::CloseInactiveUnfundedAccount => "CloseInactiveUnfundedAccount",
            Self::FullBalanceWithdrawal => "FullBalanceWithdrawal",
            Self::Other => "Other",
            Self::RestrictedAmlRequest => "RestrictedAMLRequest",
            Self::RestrictedAccountsRequest => "RestrictedAccountsRequest",
            Self::RestrictedComplianceRequest => "RestrictedComplianceRequest",
            Self::RestrictedRiskRestriction => "RestrictedRiskRestriction",
            Self::RestrictedTreasuryRequest => "RestrictedTreasuryRequest",
            Self::RestrictedW8Expired => "RestrictedW8Expired",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for SetAdminAutoLiqActionAdminActionReasonCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if matches!(self, Self::Unknown(_)) {
            return Err(serde::ser::Error::custom(
                "undocumented enum values cannot be sent",
            ));
        }
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for SetAdminAutoLiqActionAdminActionReasonCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "CloseAMLDecision" => Self::CloseAmlDecision,
            "CloseComplianceDecision" => Self::CloseComplianceDecision,
            "CloseCustomerRequest" => Self::CloseCustomerRequest,
            "CloseDeceased" => Self::CloseDeceased,
            "CloseEscheatment" => Self::CloseEscheatment,
            "CloseFraud" => Self::CloseFraud,
            "CloseInactiveUnfundedAccount" => Self::CloseInactiveUnfundedAccount,
            "FullBalanceWithdrawal" => Self::FullBalanceWithdrawal,
            "Other" => Self::Other,
            "RestrictedAMLRequest" => Self::RestrictedAmlRequest,
            "RestrictedAccountsRequest" => Self::RestrictedAccountsRequest,
            "RestrictedComplianceRequest" => Self::RestrictedComplianceRequest,
            "RestrictedRiskRestriction" => Self::RestrictedRiskRestriction,
            "RestrictedTreasuryRequest" => Self::RestrictedTreasuryRequest,
            "RestrictedW8Expired" => Self::RestrictedW8Expired,
            _ => Self::Unknown(value),
        })
    }
}

/// Current wire model `SwitchRiskCategory`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct SwitchRiskCategory {
    #[serde(rename = "accountIds")]
    account_ids: Vec<crate::AccountId>,
    #[serde(rename = "riskCategoryId")]
    risk_category_id: super::ids::RiskCategoryId,
}

impl SwitchRiskCategory {
    /// Returns wire field `accountIds`.
    #[must_use]
    pub fn account_ids(&self) -> &[crate::AccountId] {
        &self.account_ids
    }

    /// Returns wire field `riskCategoryId`.
    #[must_use]
    pub fn risk_category_id(&self) -> &super::ids::RiskCategoryId {
        &self.risk_category_id
    }

    /// Starts a builder for [`SwitchRiskCategory`].
    pub fn builder() -> SwitchRiskCategoryBuilder {
        SwitchRiskCategoryBuilder::default()
    }
}

/// Builder for [`SwitchRiskCategory`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct SwitchRiskCategoryBuilder {
    account_ids: Option<Vec<crate::AccountId>>,
    risk_category_id: Option<super::ids::RiskCategoryId>,
}

impl SwitchRiskCategoryBuilder {
    /// Sets wire field `accountIds`.
    pub fn account_ids(mut self, value: Vec<crate::AccountId>) -> Self {
        self.account_ids = Some(value);
        self
    }

    /// Sets wire field `riskCategoryId`.
    pub fn risk_category_id(mut self, value: super::ids::RiskCategoryId) -> Self {
        self.risk_category_id = Some(value);
        self
    }

    /// Validates required fields and builds [`SwitchRiskCategory`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<SwitchRiskCategory, crate::api::current::BuildError> {
        let account_ids = self
            .account_ids
            .ok_or(crate::api::current::BuildError::missing("accountIds"))?;
        if account_ids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "accountIds",
                "must not be empty",
            ));
        }
        let risk_category_id = self
            .risk_category_id
            .ok_or(crate::api::current::BuildError::missing("riskCategoryId"))?;
        Ok(SwitchRiskCategory {
            account_ids,
            risk_category_id,
        })
    }
}

impl crate::api::current::support::CurrentRequest for SwitchRiskCategory {
    fn validate_current(&self) -> Result<(), crate::Error> {
        if self.account_ids.is_empty() {
            return Err(crate::Error::InvalidRequest {
                field: "accountIds",
                reason: "must not be empty",
            });
        }
        Ok(())
    }
}

/// Current wire model `UpdateMaxNetLiq`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct UpdateMaxNetLiq {
    #[serde(rename = "accountId")]
    account_id: crate::AccountId,
    #[serde(rename = "maxNetLiq")]
    #[serde(with = "crate::decimal")]
    max_net_liq: crate::Decimal,
}

impl UpdateMaxNetLiq {
    /// Returns wire field `accountId`.
    #[must_use]
    pub fn account_id(&self) -> &crate::AccountId {
        &self.account_id
    }

    /// Returns wire field `maxNetLiq`.
    #[must_use]
    pub fn max_net_liq(&self) -> &crate::Decimal {
        &self.max_net_liq
    }

    /// Starts a builder for [`UpdateMaxNetLiq`].
    pub fn builder() -> UpdateMaxNetLiqBuilder {
        UpdateMaxNetLiqBuilder::default()
    }
}

/// Builder for [`UpdateMaxNetLiq`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct UpdateMaxNetLiqBuilder {
    account_id: Option<crate::AccountId>,
    max_net_liq: Option<crate::Decimal>,
}

impl UpdateMaxNetLiqBuilder {
    /// Sets wire field `accountId`.
    pub fn account_id(mut self, value: crate::AccountId) -> Self {
        self.account_id = Some(value);
        self
    }

    /// Sets wire field `maxNetLiq`.
    pub fn max_net_liq(mut self, value: crate::Decimal) -> Self {
        self.max_net_liq = Some(value);
        self
    }

    /// Validates required fields and builds [`UpdateMaxNetLiq`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<UpdateMaxNetLiq, crate::api::current::BuildError> {
        let account_id = self
            .account_id
            .ok_or(crate::api::current::BuildError::missing("accountId"))?;
        let max_net_liq = self
            .max_net_liq
            .ok_or(crate::api::current::BuildError::missing("maxNetLiq"))?;
        Ok(UpdateMaxNetLiq {
            account_id,
            max_net_liq,
        })
    }
}

impl crate::api::current::support::CurrentRequest for UpdateMaxNetLiq {
    fn validate_current(&self) -> Result<(), crate::Error> {
        Ok(())
    }
}

/// Current wire model `UpdateUserAutoLiq`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct UpdateUserAutoLiq {
    #[serde(rename = "accountId")]
    account_id: crate::AccountId,
    #[serde(rename = "asOwner", default, skip_serializing_if = "Option::is_none")]
    as_owner: Option<bool>,
    #[serde(
        rename = "changesLocked",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    changes_locked: Option<bool>,
    #[serde(
        rename = "marginPercentageAlert",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    margin_percentage_alert: Option<crate::Decimal>,
    #[serde(
        rename = "dailyLossPercentageAlert",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    daily_loss_percentage_alert: Option<crate::Decimal>,
    #[serde(
        rename = "dailyLossAlert",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    daily_loss_alert: Option<crate::Decimal>,
    #[serde(
        rename = "marginPercentageLiqOnly",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    margin_percentage_liq_only: Option<crate::Decimal>,
    #[serde(
        rename = "dailyLossPercentageLiqOnly",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    daily_loss_percentage_liq_only: Option<crate::Decimal>,
    #[serde(
        rename = "dailyLossLiqOnly",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    daily_loss_liq_only: Option<crate::Decimal>,
    #[serde(
        rename = "marginPercentageAutoLiq",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    margin_percentage_auto_liq: Option<crate::Decimal>,
    #[serde(
        rename = "dailyLossPercentageAutoLiq",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    daily_loss_percentage_auto_liq: Option<crate::Decimal>,
    #[serde(
        rename = "dailyLossAutoLiq",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    daily_loss_auto_liq: Option<crate::Decimal>,
    #[serde(
        rename = "weeklyLossAutoLiq",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    weekly_loss_auto_liq: Option<crate::Decimal>,
    #[serde(
        rename = "flattenTimestamp",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    flatten_timestamp: Option<jiff::Timestamp>,
    #[serde(
        rename = "trailingMaxDrawdown",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    trailing_max_drawdown: Option<crate::Decimal>,
    #[serde(
        rename = "trailingMaxDrawdownLimit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    trailing_max_drawdown_limit: Option<crate::Decimal>,
    #[serde(
        rename = "trailingMaxDrawdownMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    trailing_max_drawdown_mode: Option<UpdateUserAutoLiqTrailingMaxDrawdownMode>,
    #[serde(
        rename = "dailyProfitAutoLiq",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    daily_profit_auto_liq: Option<crate::Decimal>,
    #[serde(
        rename = "weeklyProfitAutoLiq",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    weekly_profit_auto_liq: Option<crate::Decimal>,
    #[serde(
        rename = "doNotUnlock",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    do_not_unlock: Option<bool>,
}

impl UpdateUserAutoLiq {
    /// Returns wire field `accountId`.
    #[must_use]
    pub fn account_id(&self) -> &crate::AccountId {
        &self.account_id
    }

    /// Returns wire field `asOwner`.
    #[must_use]
    pub fn as_owner(&self) -> Option<&bool> {
        self.as_owner.as_ref()
    }

    /// Returns wire field `changesLocked`.
    #[must_use]
    pub fn changes_locked(&self) -> Option<&bool> {
        self.changes_locked.as_ref()
    }

    /// Returns wire field `marginPercentageAlert`.
    #[must_use]
    pub fn margin_percentage_alert(&self) -> Option<&crate::Decimal> {
        self.margin_percentage_alert.as_ref()
    }

    /// Returns wire field `dailyLossPercentageAlert`.
    #[must_use]
    pub fn daily_loss_percentage_alert(&self) -> Option<&crate::Decimal> {
        self.daily_loss_percentage_alert.as_ref()
    }

    /// Returns wire field `dailyLossAlert`.
    #[must_use]
    pub fn daily_loss_alert(&self) -> Option<&crate::Decimal> {
        self.daily_loss_alert.as_ref()
    }

    /// Returns wire field `marginPercentageLiqOnly`.
    #[must_use]
    pub fn margin_percentage_liq_only(&self) -> Option<&crate::Decimal> {
        self.margin_percentage_liq_only.as_ref()
    }

    /// Returns wire field `dailyLossPercentageLiqOnly`.
    #[must_use]
    pub fn daily_loss_percentage_liq_only(&self) -> Option<&crate::Decimal> {
        self.daily_loss_percentage_liq_only.as_ref()
    }

    /// Returns wire field `dailyLossLiqOnly`.
    #[must_use]
    pub fn daily_loss_liq_only(&self) -> Option<&crate::Decimal> {
        self.daily_loss_liq_only.as_ref()
    }

    /// Returns wire field `marginPercentageAutoLiq`.
    #[must_use]
    pub fn margin_percentage_auto_liq(&self) -> Option<&crate::Decimal> {
        self.margin_percentage_auto_liq.as_ref()
    }

    /// Returns wire field `dailyLossPercentageAutoLiq`.
    #[must_use]
    pub fn daily_loss_percentage_auto_liq(&self) -> Option<&crate::Decimal> {
        self.daily_loss_percentage_auto_liq.as_ref()
    }

    /// Returns wire field `dailyLossAutoLiq`.
    #[must_use]
    pub fn daily_loss_auto_liq(&self) -> Option<&crate::Decimal> {
        self.daily_loss_auto_liq.as_ref()
    }

    /// Returns wire field `weeklyLossAutoLiq`.
    #[must_use]
    pub fn weekly_loss_auto_liq(&self) -> Option<&crate::Decimal> {
        self.weekly_loss_auto_liq.as_ref()
    }

    /// Returns wire field `flattenTimestamp`.
    #[must_use]
    pub fn flatten_timestamp(&self) -> Option<&jiff::Timestamp> {
        self.flatten_timestamp.as_ref()
    }

    /// Returns wire field `trailingMaxDrawdown`.
    #[must_use]
    pub fn trailing_max_drawdown(&self) -> Option<&crate::Decimal> {
        self.trailing_max_drawdown.as_ref()
    }

    /// Returns wire field `trailingMaxDrawdownLimit`.
    #[must_use]
    pub fn trailing_max_drawdown_limit(&self) -> Option<&crate::Decimal> {
        self.trailing_max_drawdown_limit.as_ref()
    }

    /// Returns wire field `trailingMaxDrawdownMode`.
    #[must_use]
    pub fn trailing_max_drawdown_mode(&self) -> Option<&UpdateUserAutoLiqTrailingMaxDrawdownMode> {
        self.trailing_max_drawdown_mode.as_ref()
    }

    /// Returns wire field `dailyProfitAutoLiq`.
    #[must_use]
    pub fn daily_profit_auto_liq(&self) -> Option<&crate::Decimal> {
        self.daily_profit_auto_liq.as_ref()
    }

    /// Returns wire field `weeklyProfitAutoLiq`.
    #[must_use]
    pub fn weekly_profit_auto_liq(&self) -> Option<&crate::Decimal> {
        self.weekly_profit_auto_liq.as_ref()
    }

    /// Returns wire field `doNotUnlock`.
    #[must_use]
    pub fn do_not_unlock(&self) -> Option<&bool> {
        self.do_not_unlock.as_ref()
    }

    /// Starts a builder for [`UpdateUserAutoLiq`].
    pub fn builder() -> UpdateUserAutoLiqBuilder {
        UpdateUserAutoLiqBuilder::default()
    }
}

/// Builder for [`UpdateUserAutoLiq`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct UpdateUserAutoLiqBuilder {
    account_id: Option<crate::AccountId>,
    as_owner: Option<bool>,
    changes_locked: Option<bool>,
    margin_percentage_alert: Option<crate::Decimal>,
    daily_loss_percentage_alert: Option<crate::Decimal>,
    daily_loss_alert: Option<crate::Decimal>,
    margin_percentage_liq_only: Option<crate::Decimal>,
    daily_loss_percentage_liq_only: Option<crate::Decimal>,
    daily_loss_liq_only: Option<crate::Decimal>,
    margin_percentage_auto_liq: Option<crate::Decimal>,
    daily_loss_percentage_auto_liq: Option<crate::Decimal>,
    daily_loss_auto_liq: Option<crate::Decimal>,
    weekly_loss_auto_liq: Option<crate::Decimal>,
    flatten_timestamp: Option<jiff::Timestamp>,
    trailing_max_drawdown: Option<crate::Decimal>,
    trailing_max_drawdown_limit: Option<crate::Decimal>,
    trailing_max_drawdown_mode: Option<UpdateUserAutoLiqTrailingMaxDrawdownMode>,
    daily_profit_auto_liq: Option<crate::Decimal>,
    weekly_profit_auto_liq: Option<crate::Decimal>,
    do_not_unlock: Option<bool>,
}

impl UpdateUserAutoLiqBuilder {
    /// Sets wire field `accountId`.
    pub fn account_id(mut self, value: crate::AccountId) -> Self {
        self.account_id = Some(value);
        self
    }

    /// Sets wire field `asOwner`.
    pub fn as_owner(mut self, value: bool) -> Self {
        self.as_owner = Some(value);
        self
    }

    /// Sets wire field `changesLocked`.
    pub fn changes_locked(mut self, value: bool) -> Self {
        self.changes_locked = Some(value);
        self
    }

    /// Sets wire field `marginPercentageAlert`.
    pub fn margin_percentage_alert(mut self, value: crate::Decimal) -> Self {
        self.margin_percentage_alert = Some(value);
        self
    }

    /// Sets wire field `dailyLossPercentageAlert`.
    pub fn daily_loss_percentage_alert(mut self, value: crate::Decimal) -> Self {
        self.daily_loss_percentage_alert = Some(value);
        self
    }

    /// Sets wire field `dailyLossAlert`.
    pub fn daily_loss_alert(mut self, value: crate::Decimal) -> Self {
        self.daily_loss_alert = Some(value);
        self
    }

    /// Sets wire field `marginPercentageLiqOnly`.
    pub fn margin_percentage_liq_only(mut self, value: crate::Decimal) -> Self {
        self.margin_percentage_liq_only = Some(value);
        self
    }

    /// Sets wire field `dailyLossPercentageLiqOnly`.
    pub fn daily_loss_percentage_liq_only(mut self, value: crate::Decimal) -> Self {
        self.daily_loss_percentage_liq_only = Some(value);
        self
    }

    /// Sets wire field `dailyLossLiqOnly`.
    pub fn daily_loss_liq_only(mut self, value: crate::Decimal) -> Self {
        self.daily_loss_liq_only = Some(value);
        self
    }

    /// Sets wire field `marginPercentageAutoLiq`.
    pub fn margin_percentage_auto_liq(mut self, value: crate::Decimal) -> Self {
        self.margin_percentage_auto_liq = Some(value);
        self
    }

    /// Sets wire field `dailyLossPercentageAutoLiq`.
    pub fn daily_loss_percentage_auto_liq(mut self, value: crate::Decimal) -> Self {
        self.daily_loss_percentage_auto_liq = Some(value);
        self
    }

    /// Sets wire field `dailyLossAutoLiq`.
    pub fn daily_loss_auto_liq(mut self, value: crate::Decimal) -> Self {
        self.daily_loss_auto_liq = Some(value);
        self
    }

    /// Sets wire field `weeklyLossAutoLiq`.
    pub fn weekly_loss_auto_liq(mut self, value: crate::Decimal) -> Self {
        self.weekly_loss_auto_liq = Some(value);
        self
    }

    /// Sets wire field `flattenTimestamp`.
    pub fn flatten_timestamp(mut self, value: jiff::Timestamp) -> Self {
        self.flatten_timestamp = Some(value);
        self
    }

    /// Sets wire field `trailingMaxDrawdown`.
    pub fn trailing_max_drawdown(mut self, value: crate::Decimal) -> Self {
        self.trailing_max_drawdown = Some(value);
        self
    }

    /// Sets wire field `trailingMaxDrawdownLimit`.
    pub fn trailing_max_drawdown_limit(mut self, value: crate::Decimal) -> Self {
        self.trailing_max_drawdown_limit = Some(value);
        self
    }

    /// Sets wire field `trailingMaxDrawdownMode`.
    pub fn trailing_max_drawdown_mode(
        mut self,
        value: UpdateUserAutoLiqTrailingMaxDrawdownMode,
    ) -> Self {
        self.trailing_max_drawdown_mode = Some(value);
        self
    }

    /// Sets wire field `dailyProfitAutoLiq`.
    pub fn daily_profit_auto_liq(mut self, value: crate::Decimal) -> Self {
        self.daily_profit_auto_liq = Some(value);
        self
    }

    /// Sets wire field `weeklyProfitAutoLiq`.
    pub fn weekly_profit_auto_liq(mut self, value: crate::Decimal) -> Self {
        self.weekly_profit_auto_liq = Some(value);
        self
    }

    /// Sets wire field `doNotUnlock`.
    pub fn do_not_unlock(mut self, value: bool) -> Self {
        self.do_not_unlock = Some(value);
        self
    }

    /// Validates required fields and builds [`UpdateUserAutoLiq`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<UpdateUserAutoLiq, crate::api::current::BuildError> {
        let account_id = self
            .account_id
            .ok_or(crate::api::current::BuildError::missing("accountId"))?;
        Ok(UpdateUserAutoLiq {
            account_id,
            as_owner: self.as_owner,
            changes_locked: self.changes_locked,
            margin_percentage_alert: self.margin_percentage_alert,
            daily_loss_percentage_alert: self.daily_loss_percentage_alert,
            daily_loss_alert: self.daily_loss_alert,
            margin_percentage_liq_only: self.margin_percentage_liq_only,
            daily_loss_percentage_liq_only: self.daily_loss_percentage_liq_only,
            daily_loss_liq_only: self.daily_loss_liq_only,
            margin_percentage_auto_liq: self.margin_percentage_auto_liq,
            daily_loss_percentage_auto_liq: self.daily_loss_percentage_auto_liq,
            daily_loss_auto_liq: self.daily_loss_auto_liq,
            weekly_loss_auto_liq: self.weekly_loss_auto_liq,
            flatten_timestamp: self.flatten_timestamp,
            trailing_max_drawdown: self.trailing_max_drawdown,
            trailing_max_drawdown_limit: self.trailing_max_drawdown_limit,
            trailing_max_drawdown_mode: self.trailing_max_drawdown_mode,
            daily_profit_auto_liq: self.daily_profit_auto_liq,
            weekly_profit_auto_liq: self.weekly_profit_auto_liq,
            do_not_unlock: self.do_not_unlock,
        })
    }
}

impl crate::api::current::support::CurrentRequest for UpdateUserAutoLiq {
    fn validate_current(&self) -> Result<(), crate::Error> {
        Ok(())
    }
}

/// Current wire model `UpdateUserAutoLiqItem`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct UpdateUserAutoLiqItem {
    #[serde(rename = "accountId")]
    account_id: crate::AccountId,
    #[serde(rename = "asOwner", default, skip_serializing_if = "Option::is_none")]
    as_owner: Option<bool>,
    #[serde(
        rename = "changesLocked",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    changes_locked: Option<bool>,
    #[serde(
        rename = "marginPercentageAlert",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    margin_percentage_alert: Option<crate::Decimal>,
    #[serde(
        rename = "dailyLossPercentageAlert",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    daily_loss_percentage_alert: Option<crate::Decimal>,
    #[serde(
        rename = "dailyLossAlert",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    daily_loss_alert: Option<crate::Decimal>,
    #[serde(
        rename = "marginPercentageLiqOnly",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    margin_percentage_liq_only: Option<crate::Decimal>,
    #[serde(
        rename = "dailyLossPercentageLiqOnly",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    daily_loss_percentage_liq_only: Option<crate::Decimal>,
    #[serde(
        rename = "dailyLossLiqOnly",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    daily_loss_liq_only: Option<crate::Decimal>,
    #[serde(
        rename = "marginPercentageAutoLiq",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    margin_percentage_auto_liq: Option<crate::Decimal>,
    #[serde(
        rename = "dailyLossPercentageAutoLiq",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    daily_loss_percentage_auto_liq: Option<crate::Decimal>,
    #[serde(
        rename = "dailyLossAutoLiq",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    daily_loss_auto_liq: Option<crate::Decimal>,
    #[serde(
        rename = "weeklyLossAutoLiq",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    weekly_loss_auto_liq: Option<crate::Decimal>,
    #[serde(
        rename = "flattenTimestamp",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    flatten_timestamp: Option<jiff::Timestamp>,
    #[serde(
        rename = "trailingMaxDrawdown",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    trailing_max_drawdown: Option<crate::Decimal>,
    #[serde(
        rename = "trailingMaxDrawdownLimit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    trailing_max_drawdown_limit: Option<crate::Decimal>,
    #[serde(
        rename = "trailingMaxDrawdownMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    trailing_max_drawdown_mode: Option<UpdateUserAutoLiqItemTrailingMaxDrawdownMode>,
    #[serde(
        rename = "dailyProfitAutoLiq",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    daily_profit_auto_liq: Option<crate::Decimal>,
    #[serde(
        rename = "weeklyProfitAutoLiq",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    weekly_profit_auto_liq: Option<crate::Decimal>,
    #[serde(
        rename = "doNotUnlock",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    do_not_unlock: Option<bool>,
}

impl UpdateUserAutoLiqItem {
    /// Returns wire field `accountId`.
    #[must_use]
    pub fn account_id(&self) -> &crate::AccountId {
        &self.account_id
    }

    /// Returns wire field `asOwner`.
    #[must_use]
    pub fn as_owner(&self) -> Option<&bool> {
        self.as_owner.as_ref()
    }

    /// Returns wire field `changesLocked`.
    #[must_use]
    pub fn changes_locked(&self) -> Option<&bool> {
        self.changes_locked.as_ref()
    }

    /// Returns wire field `marginPercentageAlert`.
    #[must_use]
    pub fn margin_percentage_alert(&self) -> Option<&crate::Decimal> {
        self.margin_percentage_alert.as_ref()
    }

    /// Returns wire field `dailyLossPercentageAlert`.
    #[must_use]
    pub fn daily_loss_percentage_alert(&self) -> Option<&crate::Decimal> {
        self.daily_loss_percentage_alert.as_ref()
    }

    /// Returns wire field `dailyLossAlert`.
    #[must_use]
    pub fn daily_loss_alert(&self) -> Option<&crate::Decimal> {
        self.daily_loss_alert.as_ref()
    }

    /// Returns wire field `marginPercentageLiqOnly`.
    #[must_use]
    pub fn margin_percentage_liq_only(&self) -> Option<&crate::Decimal> {
        self.margin_percentage_liq_only.as_ref()
    }

    /// Returns wire field `dailyLossPercentageLiqOnly`.
    #[must_use]
    pub fn daily_loss_percentage_liq_only(&self) -> Option<&crate::Decimal> {
        self.daily_loss_percentage_liq_only.as_ref()
    }

    /// Returns wire field `dailyLossLiqOnly`.
    #[must_use]
    pub fn daily_loss_liq_only(&self) -> Option<&crate::Decimal> {
        self.daily_loss_liq_only.as_ref()
    }

    /// Returns wire field `marginPercentageAutoLiq`.
    #[must_use]
    pub fn margin_percentage_auto_liq(&self) -> Option<&crate::Decimal> {
        self.margin_percentage_auto_liq.as_ref()
    }

    /// Returns wire field `dailyLossPercentageAutoLiq`.
    #[must_use]
    pub fn daily_loss_percentage_auto_liq(&self) -> Option<&crate::Decimal> {
        self.daily_loss_percentage_auto_liq.as_ref()
    }

    /// Returns wire field `dailyLossAutoLiq`.
    #[must_use]
    pub fn daily_loss_auto_liq(&self) -> Option<&crate::Decimal> {
        self.daily_loss_auto_liq.as_ref()
    }

    /// Returns wire field `weeklyLossAutoLiq`.
    #[must_use]
    pub fn weekly_loss_auto_liq(&self) -> Option<&crate::Decimal> {
        self.weekly_loss_auto_liq.as_ref()
    }

    /// Returns wire field `flattenTimestamp`.
    #[must_use]
    pub fn flatten_timestamp(&self) -> Option<&jiff::Timestamp> {
        self.flatten_timestamp.as_ref()
    }

    /// Returns wire field `trailingMaxDrawdown`.
    #[must_use]
    pub fn trailing_max_drawdown(&self) -> Option<&crate::Decimal> {
        self.trailing_max_drawdown.as_ref()
    }

    /// Returns wire field `trailingMaxDrawdownLimit`.
    #[must_use]
    pub fn trailing_max_drawdown_limit(&self) -> Option<&crate::Decimal> {
        self.trailing_max_drawdown_limit.as_ref()
    }

    /// Returns wire field `trailingMaxDrawdownMode`.
    #[must_use]
    pub fn trailing_max_drawdown_mode(
        &self,
    ) -> Option<&UpdateUserAutoLiqItemTrailingMaxDrawdownMode> {
        self.trailing_max_drawdown_mode.as_ref()
    }

    /// Returns wire field `dailyProfitAutoLiq`.
    #[must_use]
    pub fn daily_profit_auto_liq(&self) -> Option<&crate::Decimal> {
        self.daily_profit_auto_liq.as_ref()
    }

    /// Returns wire field `weeklyProfitAutoLiq`.
    #[must_use]
    pub fn weekly_profit_auto_liq(&self) -> Option<&crate::Decimal> {
        self.weekly_profit_auto_liq.as_ref()
    }

    /// Returns wire field `doNotUnlock`.
    #[must_use]
    pub fn do_not_unlock(&self) -> Option<&bool> {
        self.do_not_unlock.as_ref()
    }

    /// Starts a builder for [`UpdateUserAutoLiqItem`].
    pub fn builder() -> UpdateUserAutoLiqItemBuilder {
        UpdateUserAutoLiqItemBuilder::default()
    }
}

/// Builder for [`UpdateUserAutoLiqItem`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct UpdateUserAutoLiqItemBuilder {
    account_id: Option<crate::AccountId>,
    as_owner: Option<bool>,
    changes_locked: Option<bool>,
    margin_percentage_alert: Option<crate::Decimal>,
    daily_loss_percentage_alert: Option<crate::Decimal>,
    daily_loss_alert: Option<crate::Decimal>,
    margin_percentage_liq_only: Option<crate::Decimal>,
    daily_loss_percentage_liq_only: Option<crate::Decimal>,
    daily_loss_liq_only: Option<crate::Decimal>,
    margin_percentage_auto_liq: Option<crate::Decimal>,
    daily_loss_percentage_auto_liq: Option<crate::Decimal>,
    daily_loss_auto_liq: Option<crate::Decimal>,
    weekly_loss_auto_liq: Option<crate::Decimal>,
    flatten_timestamp: Option<jiff::Timestamp>,
    trailing_max_drawdown: Option<crate::Decimal>,
    trailing_max_drawdown_limit: Option<crate::Decimal>,
    trailing_max_drawdown_mode: Option<UpdateUserAutoLiqItemTrailingMaxDrawdownMode>,
    daily_profit_auto_liq: Option<crate::Decimal>,
    weekly_profit_auto_liq: Option<crate::Decimal>,
    do_not_unlock: Option<bool>,
}

impl UpdateUserAutoLiqItemBuilder {
    /// Sets wire field `accountId`.
    pub fn account_id(mut self, value: crate::AccountId) -> Self {
        self.account_id = Some(value);
        self
    }

    /// Sets wire field `asOwner`.
    pub fn as_owner(mut self, value: bool) -> Self {
        self.as_owner = Some(value);
        self
    }

    /// Sets wire field `changesLocked`.
    pub fn changes_locked(mut self, value: bool) -> Self {
        self.changes_locked = Some(value);
        self
    }

    /// Sets wire field `marginPercentageAlert`.
    pub fn margin_percentage_alert(mut self, value: crate::Decimal) -> Self {
        self.margin_percentage_alert = Some(value);
        self
    }

    /// Sets wire field `dailyLossPercentageAlert`.
    pub fn daily_loss_percentage_alert(mut self, value: crate::Decimal) -> Self {
        self.daily_loss_percentage_alert = Some(value);
        self
    }

    /// Sets wire field `dailyLossAlert`.
    pub fn daily_loss_alert(mut self, value: crate::Decimal) -> Self {
        self.daily_loss_alert = Some(value);
        self
    }

    /// Sets wire field `marginPercentageLiqOnly`.
    pub fn margin_percentage_liq_only(mut self, value: crate::Decimal) -> Self {
        self.margin_percentage_liq_only = Some(value);
        self
    }

    /// Sets wire field `dailyLossPercentageLiqOnly`.
    pub fn daily_loss_percentage_liq_only(mut self, value: crate::Decimal) -> Self {
        self.daily_loss_percentage_liq_only = Some(value);
        self
    }

    /// Sets wire field `dailyLossLiqOnly`.
    pub fn daily_loss_liq_only(mut self, value: crate::Decimal) -> Self {
        self.daily_loss_liq_only = Some(value);
        self
    }

    /// Sets wire field `marginPercentageAutoLiq`.
    pub fn margin_percentage_auto_liq(mut self, value: crate::Decimal) -> Self {
        self.margin_percentage_auto_liq = Some(value);
        self
    }

    /// Sets wire field `dailyLossPercentageAutoLiq`.
    pub fn daily_loss_percentage_auto_liq(mut self, value: crate::Decimal) -> Self {
        self.daily_loss_percentage_auto_liq = Some(value);
        self
    }

    /// Sets wire field `dailyLossAutoLiq`.
    pub fn daily_loss_auto_liq(mut self, value: crate::Decimal) -> Self {
        self.daily_loss_auto_liq = Some(value);
        self
    }

    /// Sets wire field `weeklyLossAutoLiq`.
    pub fn weekly_loss_auto_liq(mut self, value: crate::Decimal) -> Self {
        self.weekly_loss_auto_liq = Some(value);
        self
    }

    /// Sets wire field `flattenTimestamp`.
    pub fn flatten_timestamp(mut self, value: jiff::Timestamp) -> Self {
        self.flatten_timestamp = Some(value);
        self
    }

    /// Sets wire field `trailingMaxDrawdown`.
    pub fn trailing_max_drawdown(mut self, value: crate::Decimal) -> Self {
        self.trailing_max_drawdown = Some(value);
        self
    }

    /// Sets wire field `trailingMaxDrawdownLimit`.
    pub fn trailing_max_drawdown_limit(mut self, value: crate::Decimal) -> Self {
        self.trailing_max_drawdown_limit = Some(value);
        self
    }

    /// Sets wire field `trailingMaxDrawdownMode`.
    pub fn trailing_max_drawdown_mode(
        mut self,
        value: UpdateUserAutoLiqItemTrailingMaxDrawdownMode,
    ) -> Self {
        self.trailing_max_drawdown_mode = Some(value);
        self
    }

    /// Sets wire field `dailyProfitAutoLiq`.
    pub fn daily_profit_auto_liq(mut self, value: crate::Decimal) -> Self {
        self.daily_profit_auto_liq = Some(value);
        self
    }

    /// Sets wire field `weeklyProfitAutoLiq`.
    pub fn weekly_profit_auto_liq(mut self, value: crate::Decimal) -> Self {
        self.weekly_profit_auto_liq = Some(value);
        self
    }

    /// Sets wire field `doNotUnlock`.
    pub fn do_not_unlock(mut self, value: bool) -> Self {
        self.do_not_unlock = Some(value);
        self
    }

    /// Validates required fields and builds [`UpdateUserAutoLiqItem`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<UpdateUserAutoLiqItem, crate::api::current::BuildError> {
        let account_id = self
            .account_id
            .ok_or(crate::api::current::BuildError::missing("accountId"))?;
        Ok(UpdateUserAutoLiqItem {
            account_id,
            as_owner: self.as_owner,
            changes_locked: self.changes_locked,
            margin_percentage_alert: self.margin_percentage_alert,
            daily_loss_percentage_alert: self.daily_loss_percentage_alert,
            daily_loss_alert: self.daily_loss_alert,
            margin_percentage_liq_only: self.margin_percentage_liq_only,
            daily_loss_percentage_liq_only: self.daily_loss_percentage_liq_only,
            daily_loss_liq_only: self.daily_loss_liq_only,
            margin_percentage_auto_liq: self.margin_percentage_auto_liq,
            daily_loss_percentage_auto_liq: self.daily_loss_percentage_auto_liq,
            daily_loss_auto_liq: self.daily_loss_auto_liq,
            weekly_loss_auto_liq: self.weekly_loss_auto_liq,
            flatten_timestamp: self.flatten_timestamp,
            trailing_max_drawdown: self.trailing_max_drawdown,
            trailing_max_drawdown_limit: self.trailing_max_drawdown_limit,
            trailing_max_drawdown_mode: self.trailing_max_drawdown_mode,
            daily_profit_auto_liq: self.daily_profit_auto_liq,
            weekly_profit_auto_liq: self.weekly_profit_auto_liq,
            do_not_unlock: self.do_not_unlock,
        })
    }
}

/// Current wire model `UpdateUserAutoLiqItemResponse`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct UpdateUserAutoLiqItemResponse {
    #[serde(
        rename = "userAccountAutoLiq",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    user_account_auto_liq: Option<super::users::UserAccountAutoLiq>,
    #[serde(
        rename = "permissionedAccountAutoLiq",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    permissioned_account_auto_liq: Option<PermissionedAccountAutoLiq>,
}

impl UpdateUserAutoLiqItemResponse {
    /// Returns wire field `userAccountAutoLiq`.
    #[must_use]
    pub fn user_account_auto_liq(&self) -> Option<&super::users::UserAccountAutoLiq> {
        self.user_account_auto_liq.as_ref()
    }

    /// Returns wire field `permissionedAccountAutoLiq`.
    #[must_use]
    pub fn permissioned_account_auto_liq(&self) -> Option<&PermissionedAccountAutoLiq> {
        self.permissioned_account_auto_liq.as_ref()
    }

    /// Starts a builder for [`UpdateUserAutoLiqItemResponse`].
    pub fn builder() -> UpdateUserAutoLiqItemResponseBuilder {
        UpdateUserAutoLiqItemResponseBuilder::default()
    }
}

/// Builder for [`UpdateUserAutoLiqItemResponse`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct UpdateUserAutoLiqItemResponseBuilder {
    user_account_auto_liq: Option<super::users::UserAccountAutoLiq>,
    permissioned_account_auto_liq: Option<PermissionedAccountAutoLiq>,
}

impl UpdateUserAutoLiqItemResponseBuilder {
    /// Sets wire field `userAccountAutoLiq`.
    pub fn user_account_auto_liq(mut self, value: super::users::UserAccountAutoLiq) -> Self {
        self.user_account_auto_liq = Some(value);
        self
    }

    /// Sets wire field `permissionedAccountAutoLiq`.
    pub fn permissioned_account_auto_liq(mut self, value: PermissionedAccountAutoLiq) -> Self {
        self.permissioned_account_auto_liq = Some(value);
        self
    }

    /// Validates required fields and builds [`UpdateUserAutoLiqItemResponse`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<UpdateUserAutoLiqItemResponse, crate::api::current::BuildError> {
        Ok(UpdateUserAutoLiqItemResponse {
            user_account_auto_liq: self.user_account_auto_liq,
            permissioned_account_auto_liq: self.permissioned_account_auto_liq,
        })
    }
}

/// Current provider values for `UpdateUserAutoLiqItemTrailingMaxDrawdownMode`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum UpdateUserAutoLiqItemTrailingMaxDrawdownMode {
    /// Provider value `EOD`.
    Eod,
    /// Provider value `RealTime`.
    RealTime,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl UpdateUserAutoLiqItemTrailingMaxDrawdownMode {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Eod => "EOD",
            Self::RealTime => "RealTime",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for UpdateUserAutoLiqItemTrailingMaxDrawdownMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if matches!(self, Self::Unknown(_)) {
            return Err(serde::ser::Error::custom(
                "undocumented enum values cannot be sent",
            ));
        }
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for UpdateUserAutoLiqItemTrailingMaxDrawdownMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "EOD" => Self::Eod,
            "RealTime" => Self::RealTime,
            _ => Self::Unknown(value),
        })
    }
}

/// Current wire model `UpdateUserAutoLiqResponse`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct UpdateUserAutoLiqResponse {
    #[serde(rename = "errorText", default, skip_serializing_if = "Option::is_none")]
    error_text: Option<String>,
    #[serde(
        rename = "userAccountAutoLiq",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    user_account_auto_liq: Option<super::users::UserAccountAutoLiq>,
    #[serde(
        rename = "permissionedAccountAutoLiq",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    permissioned_account_auto_liq: Option<PermissionedAccountAutoLiq>,
}

impl UpdateUserAutoLiqResponse {
    /// Returns wire field `errorText`.
    #[must_use]
    pub fn error_text(&self) -> Option<&str> {
        self.error_text.as_deref()
    }

    /// Returns wire field `userAccountAutoLiq`.
    #[must_use]
    pub fn user_account_auto_liq(&self) -> Option<&super::users::UserAccountAutoLiq> {
        self.user_account_auto_liq.as_ref()
    }

    /// Returns wire field `permissionedAccountAutoLiq`.
    #[must_use]
    pub fn permissioned_account_auto_liq(&self) -> Option<&PermissionedAccountAutoLiq> {
        self.permissioned_account_auto_liq.as_ref()
    }

    /// Starts a builder for [`UpdateUserAutoLiqResponse`].
    pub fn builder() -> UpdateUserAutoLiqResponseBuilder {
        UpdateUserAutoLiqResponseBuilder::default()
    }
}

/// Builder for [`UpdateUserAutoLiqResponse`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct UpdateUserAutoLiqResponseBuilder {
    error_text: Option<String>,
    user_account_auto_liq: Option<super::users::UserAccountAutoLiq>,
    permissioned_account_auto_liq: Option<PermissionedAccountAutoLiq>,
}

impl UpdateUserAutoLiqResponseBuilder {
    /// Sets wire field `errorText`.
    pub fn error_text(mut self, value: impl Into<String>) -> Self {
        self.error_text = Some(value.into());
        self
    }

    /// Sets wire field `userAccountAutoLiq`.
    pub fn user_account_auto_liq(mut self, value: super::users::UserAccountAutoLiq) -> Self {
        self.user_account_auto_liq = Some(value);
        self
    }

    /// Sets wire field `permissionedAccountAutoLiq`.
    pub fn permissioned_account_auto_liq(mut self, value: PermissionedAccountAutoLiq) -> Self {
        self.permissioned_account_auto_liq = Some(value);
        self
    }

    /// Validates required fields and builds [`UpdateUserAutoLiqResponse`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<UpdateUserAutoLiqResponse, crate::api::current::BuildError> {
        Ok(UpdateUserAutoLiqResponse {
            error_text: self.error_text,
            user_account_auto_liq: self.user_account_auto_liq,
            permissioned_account_auto_liq: self.permissioned_account_auto_liq,
        })
    }
}

/// Current provider values for `UpdateUserAutoLiqTrailingMaxDrawdownMode`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum UpdateUserAutoLiqTrailingMaxDrawdownMode {
    /// Provider value `EOD`.
    Eod,
    /// Provider value `RealTime`.
    RealTime,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl UpdateUserAutoLiqTrailingMaxDrawdownMode {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Eod => "EOD",
            Self::RealTime => "RealTime",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for UpdateUserAutoLiqTrailingMaxDrawdownMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if matches!(self, Self::Unknown(_)) {
            return Err(serde::ser::Error::custom(
                "undocumented enum values cannot be sent",
            ));
        }
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for UpdateUserAutoLiqTrailingMaxDrawdownMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "EOD" => Self::Eod,
            "RealTime" => Self::RealTime,
            _ => Self::Unknown(value),
        })
    }
}

/// Current wire model `UpdateUserAutoLiqs`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct UpdateUserAutoLiqs {
    #[serde(rename = "items")]
    items: Vec<UpdateUserAutoLiqItem>,
}

impl UpdateUserAutoLiqs {
    /// Returns wire field `items`.
    #[must_use]
    pub fn items(&self) -> &[UpdateUserAutoLiqItem] {
        &self.items
    }

    /// Starts a builder for [`UpdateUserAutoLiqs`].
    pub fn builder() -> UpdateUserAutoLiqsBuilder {
        UpdateUserAutoLiqsBuilder::default()
    }
}

/// Builder for [`UpdateUserAutoLiqs`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct UpdateUserAutoLiqsBuilder {
    items: Option<Vec<UpdateUserAutoLiqItem>>,
}

impl UpdateUserAutoLiqsBuilder {
    /// Sets wire field `items`.
    pub fn items(mut self, value: Vec<UpdateUserAutoLiqItem>) -> Self {
        self.items = Some(value);
        self
    }

    /// Validates required fields and builds [`UpdateUserAutoLiqs`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<UpdateUserAutoLiqs, crate::api::current::BuildError> {
        let items = self
            .items
            .ok_or(crate::api::current::BuildError::missing("items"))?;
        if items.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "items",
                "must not be empty",
            ));
        }
        Ok(UpdateUserAutoLiqs { items })
    }
}

impl crate::api::current::support::CurrentRequest for UpdateUserAutoLiqs {
    fn validate_current(&self) -> Result<(), crate::Error> {
        if self.items.is_empty() {
            return Err(crate::Error::InvalidRequest {
                field: "items",
                reason: "must not be empty",
            });
        }
        Ok(())
    }
}

/// Current wire model `UpdateUserAutoLiqsResponse`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct UpdateUserAutoLiqsResponse {
    #[serde(rename = "errorText", default, skip_serializing_if = "Option::is_none")]
    error_text: Option<String>,
    #[serde(rename = "userAutoLiqs")]
    user_auto_liqs: Vec<UpdateUserAutoLiqItemResponse>,
}

impl UpdateUserAutoLiqsResponse {
    /// Returns wire field `errorText`.
    #[must_use]
    pub fn error_text(&self) -> Option<&str> {
        self.error_text.as_deref()
    }

    /// Returns wire field `userAutoLiqs`.
    #[must_use]
    pub fn user_auto_liqs(&self) -> &[UpdateUserAutoLiqItemResponse] {
        &self.user_auto_liqs
    }

    /// Starts a builder for [`UpdateUserAutoLiqsResponse`].
    pub fn builder() -> UpdateUserAutoLiqsResponseBuilder {
        UpdateUserAutoLiqsResponseBuilder::default()
    }
}

/// Builder for [`UpdateUserAutoLiqsResponse`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct UpdateUserAutoLiqsResponseBuilder {
    error_text: Option<String>,
    user_auto_liqs: Option<Vec<UpdateUserAutoLiqItemResponse>>,
}

impl UpdateUserAutoLiqsResponseBuilder {
    /// Sets wire field `errorText`.
    pub fn error_text(mut self, value: impl Into<String>) -> Self {
        self.error_text = Some(value.into());
        self
    }

    /// Sets wire field `userAutoLiqs`.
    pub fn user_auto_liqs(mut self, value: Vec<UpdateUserAutoLiqItemResponse>) -> Self {
        self.user_auto_liqs = Some(value);
        self
    }

    /// Validates required fields and builds [`UpdateUserAutoLiqsResponse`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<UpdateUserAutoLiqsResponse, crate::api::current::BuildError> {
        let user_auto_liqs = self
            .user_auto_liqs
            .ok_or(crate::api::current::BuildError::missing("userAutoLiqs"))?;
        Ok(UpdateUserAutoLiqsResponse {
            error_text: self.error_text,
            user_auto_liqs,
        })
    }
}

/// Current wire model `UserAccountPositionLimit`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct UserAccountPositionLimit {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    id: Option<super::ids::UserAccountPositionLimitId>,
    #[serde(
        rename = "contractId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    contract_id: Option<crate::ContractId>,
    #[serde(rename = "productId", default, skip_serializing_if = "Option::is_none")]
    product_id: Option<super::ids::ProductId>,
    #[serde(
        rename = "exchangeId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    exchange_id: Option<super::ids::ExchangeId>,
    #[serde(
        rename = "productType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    product_type: Option<UserAccountPositionLimitProductType>,
    #[serde(
        rename = "riskDiscountContractGroupId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    risk_discount_contract_group_id: Option<super::ids::ContractGroupId>,
    #[serde(
        rename = "productVerificationStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    product_verification_status: Option<UserAccountPositionLimitProductVerificationStatus>,
    #[serde(
        rename = "contractGroupId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    contract_group_id: Option<super::ids::ContractGroupId>,
    #[serde(
        rename = "fungibleProductId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    fungible_product_id: Option<super::ids::FungibleProductId>,
    #[serde(rename = "active")]
    active: bool,
    #[serde(
        rename = "riskTimePeriodId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    risk_time_period_id: Option<super::ids::RiskTimePeriodId>,
    #[serde(rename = "totalBy")]
    total_by: UserAccountPositionLimitTotalBy,
    #[serde(
        rename = "shortLimit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    short_limit: Option<i64>,
    #[serde(rename = "longLimit", default, skip_serializing_if = "Option::is_none")]
    long_limit: Option<i64>,
    #[serde(
        rename = "exposedLimit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    exposed_limit: Option<i64>,
    #[serde(
        rename = "fungibleExposedLimit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    fungible_exposed_limit: Option<i64>,
    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    description: Option<String>,
    #[serde(rename = "accountId")]
    account_id: crate::AccountId,
}

impl UserAccountPositionLimit {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> Option<&super::ids::UserAccountPositionLimitId> {
        self.id.as_ref()
    }

    /// Returns wire field `contractId`.
    #[must_use]
    pub fn contract_id(&self) -> Option<&crate::ContractId> {
        self.contract_id.as_ref()
    }

    /// Returns wire field `productId`.
    #[must_use]
    pub fn product_id(&self) -> Option<&super::ids::ProductId> {
        self.product_id.as_ref()
    }

    /// Returns wire field `exchangeId`.
    #[must_use]
    pub fn exchange_id(&self) -> Option<&super::ids::ExchangeId> {
        self.exchange_id.as_ref()
    }

    /// Returns wire field `productType`.
    #[must_use]
    pub fn product_type(&self) -> Option<&UserAccountPositionLimitProductType> {
        self.product_type.as_ref()
    }

    /// Returns wire field `riskDiscountContractGroupId`.
    #[must_use]
    pub fn risk_discount_contract_group_id(&self) -> Option<&super::ids::ContractGroupId> {
        self.risk_discount_contract_group_id.as_ref()
    }

    /// Returns wire field `productVerificationStatus`.
    #[must_use]
    pub fn product_verification_status(
        &self,
    ) -> Option<&UserAccountPositionLimitProductVerificationStatus> {
        self.product_verification_status.as_ref()
    }

    /// Returns wire field `contractGroupId`.
    #[must_use]
    pub fn contract_group_id(&self) -> Option<&super::ids::ContractGroupId> {
        self.contract_group_id.as_ref()
    }

    /// Returns wire field `fungibleProductId`.
    #[must_use]
    pub fn fungible_product_id(&self) -> Option<&super::ids::FungibleProductId> {
        self.fungible_product_id.as_ref()
    }

    /// Returns wire field `active`.
    #[must_use]
    pub fn active(&self) -> &bool {
        &self.active
    }

    /// Returns wire field `riskTimePeriodId`.
    #[must_use]
    pub fn risk_time_period_id(&self) -> Option<&super::ids::RiskTimePeriodId> {
        self.risk_time_period_id.as_ref()
    }

    /// Returns wire field `totalBy`.
    #[must_use]
    pub fn total_by(&self) -> &UserAccountPositionLimitTotalBy {
        &self.total_by
    }

    /// Returns wire field `shortLimit`.
    #[must_use]
    pub fn short_limit(&self) -> Option<&i64> {
        self.short_limit.as_ref()
    }

    /// Returns wire field `longLimit`.
    #[must_use]
    pub fn long_limit(&self) -> Option<&i64> {
        self.long_limit.as_ref()
    }

    /// Returns wire field `exposedLimit`.
    #[must_use]
    pub fn exposed_limit(&self) -> Option<&i64> {
        self.exposed_limit.as_ref()
    }

    /// Returns wire field `fungibleExposedLimit`.
    #[must_use]
    pub fn fungible_exposed_limit(&self) -> Option<&i64> {
        self.fungible_exposed_limit.as_ref()
    }

    /// Returns wire field `description`.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Returns wire field `accountId`.
    #[must_use]
    pub fn account_id(&self) -> &crate::AccountId {
        &self.account_id
    }

    /// Starts a builder for [`UserAccountPositionLimit`].
    pub fn builder() -> UserAccountPositionLimitBuilder {
        UserAccountPositionLimitBuilder::default()
    }
}

/// Builder for [`UserAccountPositionLimit`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct UserAccountPositionLimitBuilder {
    id: Option<super::ids::UserAccountPositionLimitId>,
    contract_id: Option<crate::ContractId>,
    product_id: Option<super::ids::ProductId>,
    exchange_id: Option<super::ids::ExchangeId>,
    product_type: Option<UserAccountPositionLimitProductType>,
    risk_discount_contract_group_id: Option<super::ids::ContractGroupId>,
    product_verification_status: Option<UserAccountPositionLimitProductVerificationStatus>,
    contract_group_id: Option<super::ids::ContractGroupId>,
    fungible_product_id: Option<super::ids::FungibleProductId>,
    active: Option<bool>,
    risk_time_period_id: Option<super::ids::RiskTimePeriodId>,
    total_by: Option<UserAccountPositionLimitTotalBy>,
    short_limit: Option<i64>,
    long_limit: Option<i64>,
    exposed_limit: Option<i64>,
    fungible_exposed_limit: Option<i64>,
    description: Option<String>,
    account_id: Option<crate::AccountId>,
}

impl UserAccountPositionLimitBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::UserAccountPositionLimitId) -> Self {
        self.id = Some(value);
        self
    }

    /// Sets wire field `contractId`.
    pub fn contract_id(mut self, value: crate::ContractId) -> Self {
        self.contract_id = Some(value);
        self
    }

    /// Sets wire field `productId`.
    pub fn product_id(mut self, value: super::ids::ProductId) -> Self {
        self.product_id = Some(value);
        self
    }

    /// Sets wire field `exchangeId`.
    pub fn exchange_id(mut self, value: super::ids::ExchangeId) -> Self {
        self.exchange_id = Some(value);
        self
    }

    /// Sets wire field `productType`.
    pub fn product_type(mut self, value: UserAccountPositionLimitProductType) -> Self {
        self.product_type = Some(value);
        self
    }

    /// Sets wire field `riskDiscountContractGroupId`.
    pub fn risk_discount_contract_group_id(mut self, value: super::ids::ContractGroupId) -> Self {
        self.risk_discount_contract_group_id = Some(value);
        self
    }

    /// Sets wire field `productVerificationStatus`.
    pub fn product_verification_status(
        mut self,
        value: UserAccountPositionLimitProductVerificationStatus,
    ) -> Self {
        self.product_verification_status = Some(value);
        self
    }

    /// Sets wire field `contractGroupId`.
    pub fn contract_group_id(mut self, value: super::ids::ContractGroupId) -> Self {
        self.contract_group_id = Some(value);
        self
    }

    /// Sets wire field `fungibleProductId`.
    pub fn fungible_product_id(mut self, value: super::ids::FungibleProductId) -> Self {
        self.fungible_product_id = Some(value);
        self
    }

    /// Sets wire field `active`.
    pub fn active(mut self, value: bool) -> Self {
        self.active = Some(value);
        self
    }

    /// Sets wire field `riskTimePeriodId`.
    pub fn risk_time_period_id(mut self, value: super::ids::RiskTimePeriodId) -> Self {
        self.risk_time_period_id = Some(value);
        self
    }

    /// Sets wire field `totalBy`.
    pub fn total_by(mut self, value: UserAccountPositionLimitTotalBy) -> Self {
        self.total_by = Some(value);
        self
    }

    /// Sets wire field `shortLimit`.
    pub fn short_limit(mut self, value: i64) -> Self {
        self.short_limit = Some(value);
        self
    }

    /// Sets wire field `longLimit`.
    pub fn long_limit(mut self, value: i64) -> Self {
        self.long_limit = Some(value);
        self
    }

    /// Sets wire field `exposedLimit`.
    pub fn exposed_limit(mut self, value: i64) -> Self {
        self.exposed_limit = Some(value);
        self
    }

    /// Sets wire field `fungibleExposedLimit`.
    pub fn fungible_exposed_limit(mut self, value: i64) -> Self {
        self.fungible_exposed_limit = Some(value);
        self
    }

    /// Sets wire field `description`.
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Sets wire field `accountId`.
    pub fn account_id(mut self, value: crate::AccountId) -> Self {
        self.account_id = Some(value);
        self
    }

    /// Validates required fields and builds [`UserAccountPositionLimit`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<UserAccountPositionLimit, crate::api::current::BuildError> {
        let active = self
            .active
            .ok_or(crate::api::current::BuildError::missing("active"))?;
        let total_by = self
            .total_by
            .ok_or(crate::api::current::BuildError::missing("totalBy"))?;
        let account_id = self
            .account_id
            .ok_or(crate::api::current::BuildError::missing("accountId"))?;
        Ok(UserAccountPositionLimit {
            id: self.id,
            contract_id: self.contract_id,
            product_id: self.product_id,
            exchange_id: self.exchange_id,
            product_type: self.product_type,
            risk_discount_contract_group_id: self.risk_discount_contract_group_id,
            product_verification_status: self.product_verification_status,
            contract_group_id: self.contract_group_id,
            fungible_product_id: self.fungible_product_id,
            active,
            risk_time_period_id: self.risk_time_period_id,
            total_by,
            short_limit: self.short_limit,
            long_limit: self.long_limit,
            exposed_limit: self.exposed_limit,
            fungible_exposed_limit: self.fungible_exposed_limit,
            description: self.description,
            account_id,
        })
    }
}

impl crate::api::current::support::CurrentRequest for UserAccountPositionLimit {
    fn validate_current(&self) -> Result<(), crate::Error> {
        Ok(())
    }
}

/// Current provider values for `UserAccountPositionLimitProductType`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum UserAccountPositionLimitProductType {
    /// Provider value `CommonStock`.
    CommonStock,
    /// Provider value `Continuous`.
    Continuous,
    /// Provider value `Cryptocurrency`.
    Cryptocurrency,
    /// Provider value `Futures`.
    Futures,
    /// Provider value `MarketInternals`.
    MarketInternals,
    /// Provider value `Options`.
    Options,
    /// Provider value `Spread`.
    Spread,
    /// Provider value `Swap`.
    Swap,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl UserAccountPositionLimitProductType {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::CommonStock => "CommonStock",
            Self::Continuous => "Continuous",
            Self::Cryptocurrency => "Cryptocurrency",
            Self::Futures => "Futures",
            Self::MarketInternals => "MarketInternals",
            Self::Options => "Options",
            Self::Spread => "Spread",
            Self::Swap => "Swap",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for UserAccountPositionLimitProductType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if matches!(self, Self::Unknown(_)) {
            return Err(serde::ser::Error::custom(
                "undocumented enum values cannot be sent",
            ));
        }
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for UserAccountPositionLimitProductType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "CommonStock" => Self::CommonStock,
            "Continuous" => Self::Continuous,
            "Cryptocurrency" => Self::Cryptocurrency,
            "Futures" => Self::Futures,
            "MarketInternals" => Self::MarketInternals,
            "Options" => Self::Options,
            "Spread" => Self::Spread,
            "Swap" => Self::Swap,
            _ => Self::Unknown(value),
        })
    }
}

/// Current provider values for `UserAccountPositionLimitProductVerificationStatus`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum UserAccountPositionLimitProductVerificationStatus {
    /// Provider value `Inactive`.
    Inactive,
    /// Provider value `Locked`.
    Locked,
    /// Provider value `ReadyForContracts`.
    ReadyForContracts,
    /// Provider value `ReadyToTrade`.
    ReadyToTrade,
    /// Provider value `Verified`.
    Verified,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl UserAccountPositionLimitProductVerificationStatus {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Inactive => "Inactive",
            Self::Locked => "Locked",
            Self::ReadyForContracts => "ReadyForContracts",
            Self::ReadyToTrade => "ReadyToTrade",
            Self::Verified => "Verified",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for UserAccountPositionLimitProductVerificationStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if matches!(self, Self::Unknown(_)) {
            return Err(serde::ser::Error::custom(
                "undocumented enum values cannot be sent",
            ));
        }
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for UserAccountPositionLimitProductVerificationStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "Inactive" => Self::Inactive,
            "Locked" => Self::Locked,
            "ReadyForContracts" => Self::ReadyForContracts,
            "ReadyToTrade" => Self::ReadyToTrade,
            "Verified" => Self::Verified,
            _ => Self::Unknown(value),
        })
    }
}

/// Current provider values for `UserAccountPositionLimitTotalBy`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum UserAccountPositionLimitTotalBy {
    /// Provider value `Contract`.
    Contract,
    /// Provider value `ContractGroup`.
    ContractGroup,
    /// Provider value `DiscountGroup`.
    DiscountGroup,
    /// Provider value `Exchange`.
    Exchange,
    /// Provider value `FungibleProduct`.
    FungibleProduct,
    /// Provider value `Overall`.
    Overall,
    /// Provider value `Product`.
    Product,
    /// Provider value `ProductType`.
    ProductType,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl UserAccountPositionLimitTotalBy {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Contract => "Contract",
            Self::ContractGroup => "ContractGroup",
            Self::DiscountGroup => "DiscountGroup",
            Self::Exchange => "Exchange",
            Self::FungibleProduct => "FungibleProduct",
            Self::Overall => "Overall",
            Self::Product => "Product",
            Self::ProductType => "ProductType",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for UserAccountPositionLimitTotalBy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if matches!(self, Self::Unknown(_)) {
            return Err(serde::ser::Error::custom(
                "undocumented enum values cannot be sent",
            ));
        }
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for UserAccountPositionLimitTotalBy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "Contract" => Self::Contract,
            "ContractGroup" => Self::ContractGroup,
            "DiscountGroup" => Self::DiscountGroup,
            "Exchange" => Self::Exchange,
            "FungibleProduct" => Self::FungibleProduct,
            "Overall" => Self::Overall,
            "Product" => Self::Product,
            "ProductType" => Self::ProductType,
            _ => Self::Unknown(value),
        })
    }
}

/// Current wire model `UserAccountRiskParameter`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct UserAccountRiskParameter {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    id: Option<super::ids::UserAccountRiskParameterId>,
    #[serde(
        rename = "contractId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    contract_id: Option<crate::ContractId>,
    #[serde(rename = "productId", default, skip_serializing_if = "Option::is_none")]
    product_id: Option<super::ids::ProductId>,
    #[serde(
        rename = "exchangeId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    exchange_id: Option<super::ids::ExchangeId>,
    #[serde(
        rename = "productType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    product_type: Option<UserAccountRiskParameterProductType>,
    #[serde(
        rename = "riskDiscountContractGroupId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    risk_discount_contract_group_id: Option<super::ids::ContractGroupId>,
    #[serde(
        rename = "productVerificationStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    product_verification_status: Option<UserAccountRiskParameterProductVerificationStatus>,
    #[serde(
        rename = "contractGroupId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    contract_group_id: Option<super::ids::ContractGroupId>,
    #[serde(
        rename = "fungibleProductId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    fungible_product_id: Option<super::ids::FungibleProductId>,
    #[serde(
        rename = "maxOpeningOrderQty",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    max_opening_order_qty: Option<i64>,
    #[serde(
        rename = "maxClosingOrderQty",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    max_closing_order_qty: Option<i64>,
    #[serde(
        rename = "fungibleMaxOpeningOrderQty",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    fungible_max_opening_order_qty: Option<i64>,
    #[serde(
        rename = "fungibleMaxClosingOrderQty",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    fungible_max_closing_order_qty: Option<i64>,
    #[serde(
        rename = "maxBackMonth",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    max_back_month: Option<i64>,
    #[serde(
        rename = "preExpirationDays",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pre_expiration_days: Option<i64>,
    #[serde(
        rename = "marginPercentage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    margin_percentage: Option<crate::Decimal>,
    #[serde(
        rename = "marginDollarValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    margin_dollar_value: Option<crate::Decimal>,
    #[serde(rename = "hardLimit", default, skip_serializing_if = "Option::is_none")]
    hard_limit: Option<bool>,
    #[serde(rename = "userAccountPositionLimitId")]
    user_account_position_limit_id: super::ids::UserAccountPositionLimitId,
}

impl UserAccountRiskParameter {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> Option<&super::ids::UserAccountRiskParameterId> {
        self.id.as_ref()
    }

    /// Returns wire field `contractId`.
    #[must_use]
    pub fn contract_id(&self) -> Option<&crate::ContractId> {
        self.contract_id.as_ref()
    }

    /// Returns wire field `productId`.
    #[must_use]
    pub fn product_id(&self) -> Option<&super::ids::ProductId> {
        self.product_id.as_ref()
    }

    /// Returns wire field `exchangeId`.
    #[must_use]
    pub fn exchange_id(&self) -> Option<&super::ids::ExchangeId> {
        self.exchange_id.as_ref()
    }

    /// Returns wire field `productType`.
    #[must_use]
    pub fn product_type(&self) -> Option<&UserAccountRiskParameterProductType> {
        self.product_type.as_ref()
    }

    /// Returns wire field `riskDiscountContractGroupId`.
    #[must_use]
    pub fn risk_discount_contract_group_id(&self) -> Option<&super::ids::ContractGroupId> {
        self.risk_discount_contract_group_id.as_ref()
    }

    /// Returns wire field `productVerificationStatus`.
    #[must_use]
    pub fn product_verification_status(
        &self,
    ) -> Option<&UserAccountRiskParameterProductVerificationStatus> {
        self.product_verification_status.as_ref()
    }

    /// Returns wire field `contractGroupId`.
    #[must_use]
    pub fn contract_group_id(&self) -> Option<&super::ids::ContractGroupId> {
        self.contract_group_id.as_ref()
    }

    /// Returns wire field `fungibleProductId`.
    #[must_use]
    pub fn fungible_product_id(&self) -> Option<&super::ids::FungibleProductId> {
        self.fungible_product_id.as_ref()
    }

    /// Returns wire field `maxOpeningOrderQty`.
    #[must_use]
    pub fn max_opening_order_qty(&self) -> Option<&i64> {
        self.max_opening_order_qty.as_ref()
    }

    /// Returns wire field `maxClosingOrderQty`.
    #[must_use]
    pub fn max_closing_order_qty(&self) -> Option<&i64> {
        self.max_closing_order_qty.as_ref()
    }

    /// Returns wire field `fungibleMaxOpeningOrderQty`.
    #[must_use]
    pub fn fungible_max_opening_order_qty(&self) -> Option<&i64> {
        self.fungible_max_opening_order_qty.as_ref()
    }

    /// Returns wire field `fungibleMaxClosingOrderQty`.
    #[must_use]
    pub fn fungible_max_closing_order_qty(&self) -> Option<&i64> {
        self.fungible_max_closing_order_qty.as_ref()
    }

    /// Returns wire field `maxBackMonth`.
    #[must_use]
    pub fn max_back_month(&self) -> Option<&i64> {
        self.max_back_month.as_ref()
    }

    /// Returns wire field `preExpirationDays`.
    #[must_use]
    pub fn pre_expiration_days(&self) -> Option<&i64> {
        self.pre_expiration_days.as_ref()
    }

    /// Returns wire field `marginPercentage`.
    #[must_use]
    pub fn margin_percentage(&self) -> Option<&crate::Decimal> {
        self.margin_percentage.as_ref()
    }

    /// Returns wire field `marginDollarValue`.
    #[must_use]
    pub fn margin_dollar_value(&self) -> Option<&crate::Decimal> {
        self.margin_dollar_value.as_ref()
    }

    /// Returns wire field `hardLimit`.
    #[must_use]
    pub fn hard_limit(&self) -> Option<&bool> {
        self.hard_limit.as_ref()
    }

    /// Returns wire field `userAccountPositionLimitId`.
    #[must_use]
    pub fn user_account_position_limit_id(&self) -> &super::ids::UserAccountPositionLimitId {
        &self.user_account_position_limit_id
    }

    /// Starts a builder for [`UserAccountRiskParameter`].
    pub fn builder() -> UserAccountRiskParameterBuilder {
        UserAccountRiskParameterBuilder::default()
    }
}

/// Builder for [`UserAccountRiskParameter`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct UserAccountRiskParameterBuilder {
    id: Option<super::ids::UserAccountRiskParameterId>,
    contract_id: Option<crate::ContractId>,
    product_id: Option<super::ids::ProductId>,
    exchange_id: Option<super::ids::ExchangeId>,
    product_type: Option<UserAccountRiskParameterProductType>,
    risk_discount_contract_group_id: Option<super::ids::ContractGroupId>,
    product_verification_status: Option<UserAccountRiskParameterProductVerificationStatus>,
    contract_group_id: Option<super::ids::ContractGroupId>,
    fungible_product_id: Option<super::ids::FungibleProductId>,
    max_opening_order_qty: Option<i64>,
    max_closing_order_qty: Option<i64>,
    fungible_max_opening_order_qty: Option<i64>,
    fungible_max_closing_order_qty: Option<i64>,
    max_back_month: Option<i64>,
    pre_expiration_days: Option<i64>,
    margin_percentage: Option<crate::Decimal>,
    margin_dollar_value: Option<crate::Decimal>,
    hard_limit: Option<bool>,
    user_account_position_limit_id: Option<super::ids::UserAccountPositionLimitId>,
}

impl UserAccountRiskParameterBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::UserAccountRiskParameterId) -> Self {
        self.id = Some(value);
        self
    }

    /// Sets wire field `contractId`.
    pub fn contract_id(mut self, value: crate::ContractId) -> Self {
        self.contract_id = Some(value);
        self
    }

    /// Sets wire field `productId`.
    pub fn product_id(mut self, value: super::ids::ProductId) -> Self {
        self.product_id = Some(value);
        self
    }

    /// Sets wire field `exchangeId`.
    pub fn exchange_id(mut self, value: super::ids::ExchangeId) -> Self {
        self.exchange_id = Some(value);
        self
    }

    /// Sets wire field `productType`.
    pub fn product_type(mut self, value: UserAccountRiskParameterProductType) -> Self {
        self.product_type = Some(value);
        self
    }

    /// Sets wire field `riskDiscountContractGroupId`.
    pub fn risk_discount_contract_group_id(mut self, value: super::ids::ContractGroupId) -> Self {
        self.risk_discount_contract_group_id = Some(value);
        self
    }

    /// Sets wire field `productVerificationStatus`.
    pub fn product_verification_status(
        mut self,
        value: UserAccountRiskParameterProductVerificationStatus,
    ) -> Self {
        self.product_verification_status = Some(value);
        self
    }

    /// Sets wire field `contractGroupId`.
    pub fn contract_group_id(mut self, value: super::ids::ContractGroupId) -> Self {
        self.contract_group_id = Some(value);
        self
    }

    /// Sets wire field `fungibleProductId`.
    pub fn fungible_product_id(mut self, value: super::ids::FungibleProductId) -> Self {
        self.fungible_product_id = Some(value);
        self
    }

    /// Sets wire field `maxOpeningOrderQty`.
    pub fn max_opening_order_qty(mut self, value: i64) -> Self {
        self.max_opening_order_qty = Some(value);
        self
    }

    /// Sets wire field `maxClosingOrderQty`.
    pub fn max_closing_order_qty(mut self, value: i64) -> Self {
        self.max_closing_order_qty = Some(value);
        self
    }

    /// Sets wire field `fungibleMaxOpeningOrderQty`.
    pub fn fungible_max_opening_order_qty(mut self, value: i64) -> Self {
        self.fungible_max_opening_order_qty = Some(value);
        self
    }

    /// Sets wire field `fungibleMaxClosingOrderQty`.
    pub fn fungible_max_closing_order_qty(mut self, value: i64) -> Self {
        self.fungible_max_closing_order_qty = Some(value);
        self
    }

    /// Sets wire field `maxBackMonth`.
    pub fn max_back_month(mut self, value: i64) -> Self {
        self.max_back_month = Some(value);
        self
    }

    /// Sets wire field `preExpirationDays`.
    pub fn pre_expiration_days(mut self, value: i64) -> Self {
        self.pre_expiration_days = Some(value);
        self
    }

    /// Sets wire field `marginPercentage`.
    pub fn margin_percentage(mut self, value: crate::Decimal) -> Self {
        self.margin_percentage = Some(value);
        self
    }

    /// Sets wire field `marginDollarValue`.
    pub fn margin_dollar_value(mut self, value: crate::Decimal) -> Self {
        self.margin_dollar_value = Some(value);
        self
    }

    /// Sets wire field `hardLimit`.
    pub fn hard_limit(mut self, value: bool) -> Self {
        self.hard_limit = Some(value);
        self
    }

    /// Sets wire field `userAccountPositionLimitId`.
    pub fn user_account_position_limit_id(
        mut self,
        value: super::ids::UserAccountPositionLimitId,
    ) -> Self {
        self.user_account_position_limit_id = Some(value);
        self
    }

    /// Validates required fields and builds [`UserAccountRiskParameter`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<UserAccountRiskParameter, crate::api::current::BuildError> {
        let user_account_position_limit_id =
            self.user_account_position_limit_id
                .ok_or(crate::api::current::BuildError::missing(
                    "userAccountPositionLimitId",
                ))?;
        Ok(UserAccountRiskParameter {
            id: self.id,
            contract_id: self.contract_id,
            product_id: self.product_id,
            exchange_id: self.exchange_id,
            product_type: self.product_type,
            risk_discount_contract_group_id: self.risk_discount_contract_group_id,
            product_verification_status: self.product_verification_status,
            contract_group_id: self.contract_group_id,
            fungible_product_id: self.fungible_product_id,
            max_opening_order_qty: self.max_opening_order_qty,
            max_closing_order_qty: self.max_closing_order_qty,
            fungible_max_opening_order_qty: self.fungible_max_opening_order_qty,
            fungible_max_closing_order_qty: self.fungible_max_closing_order_qty,
            max_back_month: self.max_back_month,
            pre_expiration_days: self.pre_expiration_days,
            margin_percentage: self.margin_percentage,
            margin_dollar_value: self.margin_dollar_value,
            hard_limit: self.hard_limit,
            user_account_position_limit_id,
        })
    }
}

impl crate::api::current::support::CurrentRequest for UserAccountRiskParameter {
    fn validate_current(&self) -> Result<(), crate::Error> {
        Ok(())
    }
}

/// Current provider values for `UserAccountRiskParameterProductType`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum UserAccountRiskParameterProductType {
    /// Provider value `CommonStock`.
    CommonStock,
    /// Provider value `Continuous`.
    Continuous,
    /// Provider value `Cryptocurrency`.
    Cryptocurrency,
    /// Provider value `Futures`.
    Futures,
    /// Provider value `MarketInternals`.
    MarketInternals,
    /// Provider value `Options`.
    Options,
    /// Provider value `Spread`.
    Spread,
    /// Provider value `Swap`.
    Swap,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl UserAccountRiskParameterProductType {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::CommonStock => "CommonStock",
            Self::Continuous => "Continuous",
            Self::Cryptocurrency => "Cryptocurrency",
            Self::Futures => "Futures",
            Self::MarketInternals => "MarketInternals",
            Self::Options => "Options",
            Self::Spread => "Spread",
            Self::Swap => "Swap",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for UserAccountRiskParameterProductType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if matches!(self, Self::Unknown(_)) {
            return Err(serde::ser::Error::custom(
                "undocumented enum values cannot be sent",
            ));
        }
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for UserAccountRiskParameterProductType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "CommonStock" => Self::CommonStock,
            "Continuous" => Self::Continuous,
            "Cryptocurrency" => Self::Cryptocurrency,
            "Futures" => Self::Futures,
            "MarketInternals" => Self::MarketInternals,
            "Options" => Self::Options,
            "Spread" => Self::Spread,
            "Swap" => Self::Swap,
            _ => Self::Unknown(value),
        })
    }
}

/// Current provider values for `UserAccountRiskParameterProductVerificationStatus`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum UserAccountRiskParameterProductVerificationStatus {
    /// Provider value `Inactive`.
    Inactive,
    /// Provider value `Locked`.
    Locked,
    /// Provider value `ReadyForContracts`.
    ReadyForContracts,
    /// Provider value `ReadyToTrade`.
    ReadyToTrade,
    /// Provider value `Verified`.
    Verified,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl UserAccountRiskParameterProductVerificationStatus {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Inactive => "Inactive",
            Self::Locked => "Locked",
            Self::ReadyForContracts => "ReadyForContracts",
            Self::ReadyToTrade => "ReadyToTrade",
            Self::Verified => "Verified",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for UserAccountRiskParameterProductVerificationStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if matches!(self, Self::Unknown(_)) {
            return Err(serde::ser::Error::custom(
                "undocumented enum values cannot be sent",
            ));
        }
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for UserAccountRiskParameterProductVerificationStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "Inactive" => Self::Inactive,
            "Locked" => Self::Locked,
            "ReadyForContracts" => Self::ReadyForContracts,
            "ReadyToTrade" => Self::ReadyToTrade,
            "Verified" => Self::Verified,
            _ => Self::Unknown(value),
        })
    }
}

/// Typed query parameters for `/accountRiskStatus/deps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct AccountRiskStatusDependentsQuery {
    #[serde(rename = "masterid")]
    masterid: super::ids::ProviderEntityId,
}

impl AccountRiskStatusDependentsQuery {
    /// Returns wire field `masterid`.
    #[must_use]
    pub fn masterid(&self) -> &super::ids::ProviderEntityId {
        &self.masterid
    }

    /// Starts a builder for [`AccountRiskStatusDependentsQuery`].
    pub fn builder() -> AccountRiskStatusDependentsQueryBuilder {
        AccountRiskStatusDependentsQueryBuilder::default()
    }
}

/// Builder for [`AccountRiskStatusDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct AccountRiskStatusDependentsQueryBuilder {
    masterid: Option<super::ids::ProviderEntityId>,
}

impl AccountRiskStatusDependentsQueryBuilder {
    /// Sets wire field `masterid`.
    pub fn masterid(mut self, value: super::ids::ProviderEntityId) -> Self {
        self.masterid = Some(value);
        self
    }

    /// Validates required fields and builds [`AccountRiskStatusDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(
        self,
    ) -> Result<AccountRiskStatusDependentsQuery, crate::api::current::BuildError> {
        let masterid = self
            .masterid
            .ok_or(crate::api::current::BuildError::missing("masterid"))?;
        Ok(AccountRiskStatusDependentsQuery { masterid })
    }
}

impl crate::api::current::support::CurrentQuery for AccountRiskStatusDependentsQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "masterid", &self.masterid)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /accountRiskStatus/deps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn account_risk_status_dependents(
        &self,
        query: &AccountRiskStatusDependentsQuery,
    ) -> Result<Vec<super::users::AccountRiskStatus>, crate::Error> {
        self.get_current("/accountRiskStatus/deps", query).await
    }
}

/// Typed query parameters for `/accountRiskStatus/item`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct AccountRiskStatusItemQuery {
    #[serde(rename = "id")]
    id: super::ids::AccountRiskStatusId,
}

impl AccountRiskStatusItemQuery {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> &super::ids::AccountRiskStatusId {
        &self.id
    }

    /// Starts a builder for [`AccountRiskStatusItemQuery`].
    pub fn builder() -> AccountRiskStatusItemQueryBuilder {
        AccountRiskStatusItemQueryBuilder::default()
    }
}

/// Builder for [`AccountRiskStatusItemQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct AccountRiskStatusItemQueryBuilder {
    id: Option<super::ids::AccountRiskStatusId>,
}

impl AccountRiskStatusItemQueryBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::AccountRiskStatusId) -> Self {
        self.id = Some(value);
        self
    }

    /// Validates required fields and builds [`AccountRiskStatusItemQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<AccountRiskStatusItemQuery, crate::api::current::BuildError> {
        let id = self
            .id
            .ok_or(crate::api::current::BuildError::missing("id"))?;
        Ok(AccountRiskStatusItemQuery { id })
    }
}

impl crate::api::current::support::CurrentQuery for AccountRiskStatusItemQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "id", &self.id)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /accountRiskStatus/item` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn account_risk_status_item(
        &self,
        query: &AccountRiskStatusItemQuery,
    ) -> Result<super::users::AccountRiskStatus, crate::Error> {
        self.get_current("/accountRiskStatus/item", query).await
    }
}

/// Typed query parameters for `/accountRiskStatus/items`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct AccountRiskStatusItemsQuery {
    #[serde(rename = "ids")]
    ids: Vec<super::ids::AccountRiskStatusId>,
}

impl AccountRiskStatusItemsQuery {
    /// Returns wire field `ids`.
    #[must_use]
    pub fn ids(&self) -> &[super::ids::AccountRiskStatusId] {
        &self.ids
    }

    /// Starts a builder for [`AccountRiskStatusItemsQuery`].
    pub fn builder() -> AccountRiskStatusItemsQueryBuilder {
        AccountRiskStatusItemsQueryBuilder::default()
    }
}

/// Builder for [`AccountRiskStatusItemsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct AccountRiskStatusItemsQueryBuilder {
    ids: Option<Vec<super::ids::AccountRiskStatusId>>,
}

impl AccountRiskStatusItemsQueryBuilder {
    /// Sets wire field `ids`.
    pub fn ids(mut self, value: Vec<super::ids::AccountRiskStatusId>) -> Self {
        self.ids = Some(value);
        self
    }

    /// Validates required fields and builds [`AccountRiskStatusItemsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<AccountRiskStatusItemsQuery, crate::api::current::BuildError> {
        let ids = self
            .ids
            .ok_or(crate::api::current::BuildError::missing("ids"))?;
        if ids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "ids",
                "must not be empty",
            ));
        }
        Ok(AccountRiskStatusItemsQuery { ids })
    }
}

impl crate::api::current::support::CurrentQuery for AccountRiskStatusItemsQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        if self.ids.is_empty() {
            return Err(crate::Error::InvalidRequest {
                field: "ids",
                reason: "must not be empty",
            });
        }
        let mut pairs = Vec::new();
        for value in &self.ids {
            crate::api::current::support::push_query_value(&mut pairs, "ids", value)?;
        }
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /accountRiskStatus/items` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn account_risk_status_items(
        &self,
        query: &AccountRiskStatusItemsQuery,
    ) -> Result<Vec<super::users::AccountRiskStatus>, crate::Error> {
        self.get_current("/accountRiskStatus/items", query).await
    }
}

/// Typed query parameters for `/accountRiskStatus/ldeps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct AccountRiskStatusLDependentsQuery {
    #[serde(rename = "masterids")]
    masterids: Vec<super::ids::ProviderEntityId>,
}

impl AccountRiskStatusLDependentsQuery {
    /// Returns wire field `masterids`.
    #[must_use]
    pub fn masterids(&self) -> &[super::ids::ProviderEntityId] {
        &self.masterids
    }

    /// Starts a builder for [`AccountRiskStatusLDependentsQuery`].
    pub fn builder() -> AccountRiskStatusLDependentsQueryBuilder {
        AccountRiskStatusLDependentsQueryBuilder::default()
    }
}

/// Builder for [`AccountRiskStatusLDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct AccountRiskStatusLDependentsQueryBuilder {
    masterids: Option<Vec<super::ids::ProviderEntityId>>,
}

impl AccountRiskStatusLDependentsQueryBuilder {
    /// Sets wire field `masterids`.
    pub fn masterids(mut self, value: Vec<super::ids::ProviderEntityId>) -> Self {
        self.masterids = Some(value);
        self
    }

    /// Validates required fields and builds [`AccountRiskStatusLDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(
        self,
    ) -> Result<AccountRiskStatusLDependentsQuery, crate::api::current::BuildError> {
        let masterids = self
            .masterids
            .ok_or(crate::api::current::BuildError::missing("masterids"))?;
        if masterids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "masterids",
                "must not be empty",
            ));
        }
        Ok(AccountRiskStatusLDependentsQuery { masterids })
    }
}

impl crate::api::current::support::CurrentQuery for AccountRiskStatusLDependentsQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        if self.masterids.is_empty() {
            return Err(crate::Error::InvalidRequest {
                field: "masterids",
                reason: "must not be empty",
            });
        }
        let mut pairs = Vec::new();
        for value in &self.masterids {
            crate::api::current::support::push_query_value(&mut pairs, "masterids", value)?;
        }
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /accountRiskStatus/ldeps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn account_risk_status_l_dependents(
        &self,
        query: &AccountRiskStatusLDependentsQuery,
    ) -> Result<Vec<super::users::AccountRiskStatus>, crate::Error> {
        self.get_current("/accountRiskStatus/ldeps", query).await
    }
}

impl crate::Client {
    /// Calls the current `GET /accountRiskStatus/list` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn account_risk_status_list(
        &self,
    ) -> Result<Vec<super::users::AccountRiskStatus>, crate::Error> {
        self.get_without_query("/accountRiskStatus/list").await
    }
}

/// Typed query parameters for `/contractMargin/deps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ContractMarginDependentsQuery {
    #[serde(rename = "masterid")]
    masterid: super::ids::ProviderEntityId,
}

impl ContractMarginDependentsQuery {
    /// Returns wire field `masterid`.
    #[must_use]
    pub fn masterid(&self) -> &super::ids::ProviderEntityId {
        &self.masterid
    }

    /// Starts a builder for [`ContractMarginDependentsQuery`].
    pub fn builder() -> ContractMarginDependentsQueryBuilder {
        ContractMarginDependentsQueryBuilder::default()
    }
}

/// Builder for [`ContractMarginDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ContractMarginDependentsQueryBuilder {
    masterid: Option<super::ids::ProviderEntityId>,
}

impl ContractMarginDependentsQueryBuilder {
    /// Sets wire field `masterid`.
    pub fn masterid(mut self, value: super::ids::ProviderEntityId) -> Self {
        self.masterid = Some(value);
        self
    }

    /// Validates required fields and builds [`ContractMarginDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ContractMarginDependentsQuery, crate::api::current::BuildError> {
        let masterid = self
            .masterid
            .ok_or(crate::api::current::BuildError::missing("masterid"))?;
        Ok(ContractMarginDependentsQuery { masterid })
    }
}

impl crate::api::current::support::CurrentQuery for ContractMarginDependentsQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "masterid", &self.masterid)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /contractMargin/deps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn contract_margin_dependents(
        &self,
        query: &ContractMarginDependentsQuery,
    ) -> Result<Vec<ContractMargin>, crate::Error> {
        self.get_current("/contractMargin/deps", query).await
    }
}

/// Typed query parameters for `/contractMargin/item`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ContractMarginItemQuery {
    #[serde(rename = "id")]
    id: super::ids::ContractMarginId,
}

impl ContractMarginItemQuery {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> &super::ids::ContractMarginId {
        &self.id
    }

    /// Starts a builder for [`ContractMarginItemQuery`].
    pub fn builder() -> ContractMarginItemQueryBuilder {
        ContractMarginItemQueryBuilder::default()
    }
}

/// Builder for [`ContractMarginItemQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ContractMarginItemQueryBuilder {
    id: Option<super::ids::ContractMarginId>,
}

impl ContractMarginItemQueryBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::ContractMarginId) -> Self {
        self.id = Some(value);
        self
    }

    /// Validates required fields and builds [`ContractMarginItemQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ContractMarginItemQuery, crate::api::current::BuildError> {
        let id = self
            .id
            .ok_or(crate::api::current::BuildError::missing("id"))?;
        Ok(ContractMarginItemQuery { id })
    }
}

impl crate::api::current::support::CurrentQuery for ContractMarginItemQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "id", &self.id)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /contractMargin/item` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn contract_margin_item(
        &self,
        query: &ContractMarginItemQuery,
    ) -> Result<ContractMargin, crate::Error> {
        self.get_current("/contractMargin/item", query).await
    }
}

/// Typed query parameters for `/contractMargin/items`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ContractMarginItemsQuery {
    #[serde(rename = "ids")]
    ids: Vec<super::ids::ContractMarginId>,
}

impl ContractMarginItemsQuery {
    /// Returns wire field `ids`.
    #[must_use]
    pub fn ids(&self) -> &[super::ids::ContractMarginId] {
        &self.ids
    }

    /// Starts a builder for [`ContractMarginItemsQuery`].
    pub fn builder() -> ContractMarginItemsQueryBuilder {
        ContractMarginItemsQueryBuilder::default()
    }
}

/// Builder for [`ContractMarginItemsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ContractMarginItemsQueryBuilder {
    ids: Option<Vec<super::ids::ContractMarginId>>,
}

impl ContractMarginItemsQueryBuilder {
    /// Sets wire field `ids`.
    pub fn ids(mut self, value: Vec<super::ids::ContractMarginId>) -> Self {
        self.ids = Some(value);
        self
    }

    /// Validates required fields and builds [`ContractMarginItemsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ContractMarginItemsQuery, crate::api::current::BuildError> {
        let ids = self
            .ids
            .ok_or(crate::api::current::BuildError::missing("ids"))?;
        if ids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "ids",
                "must not be empty",
            ));
        }
        Ok(ContractMarginItemsQuery { ids })
    }
}

impl crate::api::current::support::CurrentQuery for ContractMarginItemsQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        if self.ids.is_empty() {
            return Err(crate::Error::InvalidRequest {
                field: "ids",
                reason: "must not be empty",
            });
        }
        let mut pairs = Vec::new();
        for value in &self.ids {
            crate::api::current::support::push_query_value(&mut pairs, "ids", value)?;
        }
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /contractMargin/items` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn contract_margin_items(
        &self,
        query: &ContractMarginItemsQuery,
    ) -> Result<Vec<ContractMargin>, crate::Error> {
        self.get_current("/contractMargin/items", query).await
    }
}

/// Typed query parameters for `/contractMargin/ldeps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ContractMarginLDependentsQuery {
    #[serde(rename = "masterids")]
    masterids: Vec<super::ids::ProviderEntityId>,
}

impl ContractMarginLDependentsQuery {
    /// Returns wire field `masterids`.
    #[must_use]
    pub fn masterids(&self) -> &[super::ids::ProviderEntityId] {
        &self.masterids
    }

    /// Starts a builder for [`ContractMarginLDependentsQuery`].
    pub fn builder() -> ContractMarginLDependentsQueryBuilder {
        ContractMarginLDependentsQueryBuilder::default()
    }
}

/// Builder for [`ContractMarginLDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ContractMarginLDependentsQueryBuilder {
    masterids: Option<Vec<super::ids::ProviderEntityId>>,
}

impl ContractMarginLDependentsQueryBuilder {
    /// Sets wire field `masterids`.
    pub fn masterids(mut self, value: Vec<super::ids::ProviderEntityId>) -> Self {
        self.masterids = Some(value);
        self
    }

    /// Validates required fields and builds [`ContractMarginLDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ContractMarginLDependentsQuery, crate::api::current::BuildError> {
        let masterids = self
            .masterids
            .ok_or(crate::api::current::BuildError::missing("masterids"))?;
        if masterids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "masterids",
                "must not be empty",
            ));
        }
        Ok(ContractMarginLDependentsQuery { masterids })
    }
}

impl crate::api::current::support::CurrentQuery for ContractMarginLDependentsQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        if self.masterids.is_empty() {
            return Err(crate::Error::InvalidRequest {
                field: "masterids",
                reason: "must not be empty",
            });
        }
        let mut pairs = Vec::new();
        for value in &self.masterids {
            crate::api::current::support::push_query_value(&mut pairs, "masterids", value)?;
        }
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /contractMargin/ldeps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn contract_margin_l_dependents(
        &self,
        query: &ContractMarginLDependentsQuery,
    ) -> Result<Vec<ContractMargin>, crate::Error> {
        self.get_current("/contractMargin/ldeps", query).await
    }
}

/// Typed query parameters for `/permissionedAccountAutoLiq/deps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct PermissionedAccountAutoLiqDependentsQuery {
    #[serde(rename = "masterid")]
    masterid: super::ids::ProviderEntityId,
}

impl PermissionedAccountAutoLiqDependentsQuery {
    /// Returns wire field `masterid`.
    #[must_use]
    pub fn masterid(&self) -> &super::ids::ProviderEntityId {
        &self.masterid
    }

    /// Starts a builder for [`PermissionedAccountAutoLiqDependentsQuery`].
    pub fn builder() -> PermissionedAccountAutoLiqDependentsQueryBuilder {
        PermissionedAccountAutoLiqDependentsQueryBuilder::default()
    }
}

/// Builder for [`PermissionedAccountAutoLiqDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct PermissionedAccountAutoLiqDependentsQueryBuilder {
    masterid: Option<super::ids::ProviderEntityId>,
}

impl PermissionedAccountAutoLiqDependentsQueryBuilder {
    /// Sets wire field `masterid`.
    pub fn masterid(mut self, value: super::ids::ProviderEntityId) -> Self {
        self.masterid = Some(value);
        self
    }

    /// Validates required fields and builds [`PermissionedAccountAutoLiqDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(
        self,
    ) -> Result<PermissionedAccountAutoLiqDependentsQuery, crate::api::current::BuildError> {
        let masterid = self
            .masterid
            .ok_or(crate::api::current::BuildError::missing("masterid"))?;
        Ok(PermissionedAccountAutoLiqDependentsQuery { masterid })
    }
}

impl crate::api::current::support::CurrentQuery for PermissionedAccountAutoLiqDependentsQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "masterid", &self.masterid)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /permissionedAccountAutoLiq/deps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn permissioned_account_auto_liq_dependents(
        &self,
        query: &PermissionedAccountAutoLiqDependentsQuery,
    ) -> Result<Vec<PermissionedAccountAutoLiq>, crate::Error> {
        self.get_current("/permissionedAccountAutoLiq/deps", query)
            .await
    }
}

/// Typed query parameters for `/permissionedAccountAutoLiq/item`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct PermissionedAccountAutoLiqItemQuery {
    #[serde(rename = "id")]
    id: super::ids::PermissionedAccountAutoLiqId,
}

impl PermissionedAccountAutoLiqItemQuery {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> &super::ids::PermissionedAccountAutoLiqId {
        &self.id
    }

    /// Starts a builder for [`PermissionedAccountAutoLiqItemQuery`].
    pub fn builder() -> PermissionedAccountAutoLiqItemQueryBuilder {
        PermissionedAccountAutoLiqItemQueryBuilder::default()
    }
}

/// Builder for [`PermissionedAccountAutoLiqItemQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct PermissionedAccountAutoLiqItemQueryBuilder {
    id: Option<super::ids::PermissionedAccountAutoLiqId>,
}

impl PermissionedAccountAutoLiqItemQueryBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::PermissionedAccountAutoLiqId) -> Self {
        self.id = Some(value);
        self
    }

    /// Validates required fields and builds [`PermissionedAccountAutoLiqItemQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(
        self,
    ) -> Result<PermissionedAccountAutoLiqItemQuery, crate::api::current::BuildError> {
        let id = self
            .id
            .ok_or(crate::api::current::BuildError::missing("id"))?;
        Ok(PermissionedAccountAutoLiqItemQuery { id })
    }
}

impl crate::api::current::support::CurrentQuery for PermissionedAccountAutoLiqItemQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "id", &self.id)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /permissionedAccountAutoLiq/item` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn permissioned_account_auto_liq_item(
        &self,
        query: &PermissionedAccountAutoLiqItemQuery,
    ) -> Result<PermissionedAccountAutoLiq, crate::Error> {
        self.get_current("/permissionedAccountAutoLiq/item", query)
            .await
    }
}

/// Typed query parameters for `/permissionedAccountAutoLiq/items`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct PermissionedAccountAutoLiqItemsQuery {
    #[serde(rename = "ids")]
    ids: Vec<super::ids::PermissionedAccountAutoLiqId>,
}

impl PermissionedAccountAutoLiqItemsQuery {
    /// Returns wire field `ids`.
    #[must_use]
    pub fn ids(&self) -> &[super::ids::PermissionedAccountAutoLiqId] {
        &self.ids
    }

    /// Starts a builder for [`PermissionedAccountAutoLiqItemsQuery`].
    pub fn builder() -> PermissionedAccountAutoLiqItemsQueryBuilder {
        PermissionedAccountAutoLiqItemsQueryBuilder::default()
    }
}

/// Builder for [`PermissionedAccountAutoLiqItemsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct PermissionedAccountAutoLiqItemsQueryBuilder {
    ids: Option<Vec<super::ids::PermissionedAccountAutoLiqId>>,
}

impl PermissionedAccountAutoLiqItemsQueryBuilder {
    /// Sets wire field `ids`.
    pub fn ids(mut self, value: Vec<super::ids::PermissionedAccountAutoLiqId>) -> Self {
        self.ids = Some(value);
        self
    }

    /// Validates required fields and builds [`PermissionedAccountAutoLiqItemsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(
        self,
    ) -> Result<PermissionedAccountAutoLiqItemsQuery, crate::api::current::BuildError> {
        let ids = self
            .ids
            .ok_or(crate::api::current::BuildError::missing("ids"))?;
        if ids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "ids",
                "must not be empty",
            ));
        }
        Ok(PermissionedAccountAutoLiqItemsQuery { ids })
    }
}

impl crate::api::current::support::CurrentQuery for PermissionedAccountAutoLiqItemsQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        if self.ids.is_empty() {
            return Err(crate::Error::InvalidRequest {
                field: "ids",
                reason: "must not be empty",
            });
        }
        let mut pairs = Vec::new();
        for value in &self.ids {
            crate::api::current::support::push_query_value(&mut pairs, "ids", value)?;
        }
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /permissionedAccountAutoLiq/items` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn permissioned_account_auto_liq_items(
        &self,
        query: &PermissionedAccountAutoLiqItemsQuery,
    ) -> Result<Vec<PermissionedAccountAutoLiq>, crate::Error> {
        self.get_current("/permissionedAccountAutoLiq/items", query)
            .await
    }
}

/// Typed query parameters for `/permissionedAccountAutoLiq/ldeps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct PermissionedAccountAutoLiqLDependentsQuery {
    #[serde(rename = "masterids")]
    masterids: Vec<super::ids::ProviderEntityId>,
}

impl PermissionedAccountAutoLiqLDependentsQuery {
    /// Returns wire field `masterids`.
    #[must_use]
    pub fn masterids(&self) -> &[super::ids::ProviderEntityId] {
        &self.masterids
    }

    /// Starts a builder for [`PermissionedAccountAutoLiqLDependentsQuery`].
    pub fn builder() -> PermissionedAccountAutoLiqLDependentsQueryBuilder {
        PermissionedAccountAutoLiqLDependentsQueryBuilder::default()
    }
}

/// Builder for [`PermissionedAccountAutoLiqLDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct PermissionedAccountAutoLiqLDependentsQueryBuilder {
    masterids: Option<Vec<super::ids::ProviderEntityId>>,
}

impl PermissionedAccountAutoLiqLDependentsQueryBuilder {
    /// Sets wire field `masterids`.
    pub fn masterids(mut self, value: Vec<super::ids::ProviderEntityId>) -> Self {
        self.masterids = Some(value);
        self
    }

    /// Validates required fields and builds [`PermissionedAccountAutoLiqLDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(
        self,
    ) -> Result<PermissionedAccountAutoLiqLDependentsQuery, crate::api::current::BuildError> {
        let masterids = self
            .masterids
            .ok_or(crate::api::current::BuildError::missing("masterids"))?;
        if masterids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "masterids",
                "must not be empty",
            ));
        }
        Ok(PermissionedAccountAutoLiqLDependentsQuery { masterids })
    }
}

impl crate::api::current::support::CurrentQuery for PermissionedAccountAutoLiqLDependentsQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        if self.masterids.is_empty() {
            return Err(crate::Error::InvalidRequest {
                field: "masterids",
                reason: "must not be empty",
            });
        }
        let mut pairs = Vec::new();
        for value in &self.masterids {
            crate::api::current::support::push_query_value(&mut pairs, "masterids", value)?;
        }
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /permissionedAccountAutoLiq/ldeps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn permissioned_account_auto_liq_l_dependents(
        &self,
        query: &PermissionedAccountAutoLiqLDependentsQuery,
    ) -> Result<Vec<PermissionedAccountAutoLiq>, crate::Error> {
        self.get_current("/permissionedAccountAutoLiq/ldeps", query)
            .await
    }
}

impl crate::Client {
    /// Calls the current `GET /permissionedAccountAutoLiq/list` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn permissioned_account_auto_liq_list(
        &self,
    ) -> Result<Vec<PermissionedAccountAutoLiq>, crate::Error> {
        self.get_without_query("/permissionedAccountAutoLiq/list")
            .await
    }
}

/// Typed query parameters for `/productMargin/deps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ProductMarginDependentsQuery {
    #[serde(rename = "masterid")]
    masterid: super::ids::ProviderEntityId,
}

impl ProductMarginDependentsQuery {
    /// Returns wire field `masterid`.
    #[must_use]
    pub fn masterid(&self) -> &super::ids::ProviderEntityId {
        &self.masterid
    }

    /// Starts a builder for [`ProductMarginDependentsQuery`].
    pub fn builder() -> ProductMarginDependentsQueryBuilder {
        ProductMarginDependentsQueryBuilder::default()
    }
}

/// Builder for [`ProductMarginDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ProductMarginDependentsQueryBuilder {
    masterid: Option<super::ids::ProviderEntityId>,
}

impl ProductMarginDependentsQueryBuilder {
    /// Sets wire field `masterid`.
    pub fn masterid(mut self, value: super::ids::ProviderEntityId) -> Self {
        self.masterid = Some(value);
        self
    }

    /// Validates required fields and builds [`ProductMarginDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ProductMarginDependentsQuery, crate::api::current::BuildError> {
        let masterid = self
            .masterid
            .ok_or(crate::api::current::BuildError::missing("masterid"))?;
        Ok(ProductMarginDependentsQuery { masterid })
    }
}

impl crate::api::current::support::CurrentQuery for ProductMarginDependentsQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "masterid", &self.masterid)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /productMargin/deps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn product_margin_dependents(
        &self,
        query: &ProductMarginDependentsQuery,
    ) -> Result<Vec<ProductMargin>, crate::Error> {
        self.get_current("/productMargin/deps", query).await
    }
}

/// Typed query parameters for `/productMargin/item`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ProductMarginItemQuery {
    #[serde(rename = "id")]
    id: super::ids::ProductMarginId,
}

impl ProductMarginItemQuery {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> &super::ids::ProductMarginId {
        &self.id
    }

    /// Starts a builder for [`ProductMarginItemQuery`].
    pub fn builder() -> ProductMarginItemQueryBuilder {
        ProductMarginItemQueryBuilder::default()
    }
}

/// Builder for [`ProductMarginItemQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ProductMarginItemQueryBuilder {
    id: Option<super::ids::ProductMarginId>,
}

impl ProductMarginItemQueryBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::ProductMarginId) -> Self {
        self.id = Some(value);
        self
    }

    /// Validates required fields and builds [`ProductMarginItemQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ProductMarginItemQuery, crate::api::current::BuildError> {
        let id = self
            .id
            .ok_or(crate::api::current::BuildError::missing("id"))?;
        Ok(ProductMarginItemQuery { id })
    }
}

impl crate::api::current::support::CurrentQuery for ProductMarginItemQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "id", &self.id)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /productMargin/item` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn product_margin_item(
        &self,
        query: &ProductMarginItemQuery,
    ) -> Result<ProductMargin, crate::Error> {
        self.get_current("/productMargin/item", query).await
    }
}

/// Typed query parameters for `/productMargin/items`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ProductMarginItemsQuery {
    #[serde(rename = "ids")]
    ids: Vec<super::ids::ProductMarginId>,
}

impl ProductMarginItemsQuery {
    /// Returns wire field `ids`.
    #[must_use]
    pub fn ids(&self) -> &[super::ids::ProductMarginId] {
        &self.ids
    }

    /// Starts a builder for [`ProductMarginItemsQuery`].
    pub fn builder() -> ProductMarginItemsQueryBuilder {
        ProductMarginItemsQueryBuilder::default()
    }
}

/// Builder for [`ProductMarginItemsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ProductMarginItemsQueryBuilder {
    ids: Option<Vec<super::ids::ProductMarginId>>,
}

impl ProductMarginItemsQueryBuilder {
    /// Sets wire field `ids`.
    pub fn ids(mut self, value: Vec<super::ids::ProductMarginId>) -> Self {
        self.ids = Some(value);
        self
    }

    /// Validates required fields and builds [`ProductMarginItemsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ProductMarginItemsQuery, crate::api::current::BuildError> {
        let ids = self
            .ids
            .ok_or(crate::api::current::BuildError::missing("ids"))?;
        if ids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "ids",
                "must not be empty",
            ));
        }
        Ok(ProductMarginItemsQuery { ids })
    }
}

impl crate::api::current::support::CurrentQuery for ProductMarginItemsQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        if self.ids.is_empty() {
            return Err(crate::Error::InvalidRequest {
                field: "ids",
                reason: "must not be empty",
            });
        }
        let mut pairs = Vec::new();
        for value in &self.ids {
            crate::api::current::support::push_query_value(&mut pairs, "ids", value)?;
        }
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /productMargin/items` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn product_margin_items(
        &self,
        query: &ProductMarginItemsQuery,
    ) -> Result<Vec<ProductMargin>, crate::Error> {
        self.get_current("/productMargin/items", query).await
    }
}

/// Typed query parameters for `/productMargin/ldeps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ProductMarginLDependentsQuery {
    #[serde(rename = "masterids")]
    masterids: Vec<super::ids::ProviderEntityId>,
}

impl ProductMarginLDependentsQuery {
    /// Returns wire field `masterids`.
    #[must_use]
    pub fn masterids(&self) -> &[super::ids::ProviderEntityId] {
        &self.masterids
    }

    /// Starts a builder for [`ProductMarginLDependentsQuery`].
    pub fn builder() -> ProductMarginLDependentsQueryBuilder {
        ProductMarginLDependentsQueryBuilder::default()
    }
}

/// Builder for [`ProductMarginLDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ProductMarginLDependentsQueryBuilder {
    masterids: Option<Vec<super::ids::ProviderEntityId>>,
}

impl ProductMarginLDependentsQueryBuilder {
    /// Sets wire field `masterids`.
    pub fn masterids(mut self, value: Vec<super::ids::ProviderEntityId>) -> Self {
        self.masterids = Some(value);
        self
    }

    /// Validates required fields and builds [`ProductMarginLDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ProductMarginLDependentsQuery, crate::api::current::BuildError> {
        let masterids = self
            .masterids
            .ok_or(crate::api::current::BuildError::missing("masterids"))?;
        if masterids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "masterids",
                "must not be empty",
            ));
        }
        Ok(ProductMarginLDependentsQuery { masterids })
    }
}

impl crate::api::current::support::CurrentQuery for ProductMarginLDependentsQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        if self.masterids.is_empty() {
            return Err(crate::Error::InvalidRequest {
                field: "masterids",
                reason: "must not be empty",
            });
        }
        let mut pairs = Vec::new();
        for value in &self.masterids {
            crate::api::current::support::push_query_value(&mut pairs, "masterids", value)?;
        }
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /productMargin/ldeps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn product_margin_l_dependents(
        &self,
        query: &ProductMarginLDependentsQuery,
    ) -> Result<Vec<ProductMargin>, crate::Error> {
        self.get_current("/productMargin/ldeps", query).await
    }
}

impl crate::Client {
    /// Calls the current `GET /productMargin/list` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn product_margin_list(&self) -> Result<Vec<ProductMargin>, crate::Error> {
        self.get_without_query("/productMargin/list").await
    }
}

/// Typed query parameters for `/userAccountAutoLiq/deps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct UserAccountAutoLiqDependentsQuery {
    #[serde(rename = "masterid")]
    masterid: super::ids::ProviderEntityId,
}

impl UserAccountAutoLiqDependentsQuery {
    /// Returns wire field `masterid`.
    #[must_use]
    pub fn masterid(&self) -> &super::ids::ProviderEntityId {
        &self.masterid
    }

    /// Starts a builder for [`UserAccountAutoLiqDependentsQuery`].
    pub fn builder() -> UserAccountAutoLiqDependentsQueryBuilder {
        UserAccountAutoLiqDependentsQueryBuilder::default()
    }
}

/// Builder for [`UserAccountAutoLiqDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct UserAccountAutoLiqDependentsQueryBuilder {
    masterid: Option<super::ids::ProviderEntityId>,
}

impl UserAccountAutoLiqDependentsQueryBuilder {
    /// Sets wire field `masterid`.
    pub fn masterid(mut self, value: super::ids::ProviderEntityId) -> Self {
        self.masterid = Some(value);
        self
    }

    /// Validates required fields and builds [`UserAccountAutoLiqDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(
        self,
    ) -> Result<UserAccountAutoLiqDependentsQuery, crate::api::current::BuildError> {
        let masterid = self
            .masterid
            .ok_or(crate::api::current::BuildError::missing("masterid"))?;
        Ok(UserAccountAutoLiqDependentsQuery { masterid })
    }
}

impl crate::api::current::support::CurrentQuery for UserAccountAutoLiqDependentsQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "masterid", &self.masterid)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /userAccountAutoLiq/deps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn user_account_auto_liq_dependents(
        &self,
        query: &UserAccountAutoLiqDependentsQuery,
    ) -> Result<Vec<super::users::UserAccountAutoLiq>, crate::Error> {
        self.get_current("/userAccountAutoLiq/deps", query).await
    }
}

/// Typed query parameters for `/userAccountAutoLiq/item`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct UserAccountAutoLiqItemQuery {
    #[serde(rename = "id")]
    id: super::ids::UserAccountAutoLiqId,
}

impl UserAccountAutoLiqItemQuery {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> &super::ids::UserAccountAutoLiqId {
        &self.id
    }

    /// Starts a builder for [`UserAccountAutoLiqItemQuery`].
    pub fn builder() -> UserAccountAutoLiqItemQueryBuilder {
        UserAccountAutoLiqItemQueryBuilder::default()
    }
}

/// Builder for [`UserAccountAutoLiqItemQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct UserAccountAutoLiqItemQueryBuilder {
    id: Option<super::ids::UserAccountAutoLiqId>,
}

impl UserAccountAutoLiqItemQueryBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::UserAccountAutoLiqId) -> Self {
        self.id = Some(value);
        self
    }

    /// Validates required fields and builds [`UserAccountAutoLiqItemQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<UserAccountAutoLiqItemQuery, crate::api::current::BuildError> {
        let id = self
            .id
            .ok_or(crate::api::current::BuildError::missing("id"))?;
        Ok(UserAccountAutoLiqItemQuery { id })
    }
}

impl crate::api::current::support::CurrentQuery for UserAccountAutoLiqItemQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "id", &self.id)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /userAccountAutoLiq/item` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn user_account_auto_liq_item(
        &self,
        query: &UserAccountAutoLiqItemQuery,
    ) -> Result<super::users::UserAccountAutoLiq, crate::Error> {
        self.get_current("/userAccountAutoLiq/item", query).await
    }
}

/// Typed query parameters for `/userAccountAutoLiq/items`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct UserAccountAutoLiqItemsQuery {
    #[serde(rename = "ids")]
    ids: Vec<super::ids::UserAccountAutoLiqId>,
}

impl UserAccountAutoLiqItemsQuery {
    /// Returns wire field `ids`.
    #[must_use]
    pub fn ids(&self) -> &[super::ids::UserAccountAutoLiqId] {
        &self.ids
    }

    /// Starts a builder for [`UserAccountAutoLiqItemsQuery`].
    pub fn builder() -> UserAccountAutoLiqItemsQueryBuilder {
        UserAccountAutoLiqItemsQueryBuilder::default()
    }
}

/// Builder for [`UserAccountAutoLiqItemsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct UserAccountAutoLiqItemsQueryBuilder {
    ids: Option<Vec<super::ids::UserAccountAutoLiqId>>,
}

impl UserAccountAutoLiqItemsQueryBuilder {
    /// Sets wire field `ids`.
    pub fn ids(mut self, value: Vec<super::ids::UserAccountAutoLiqId>) -> Self {
        self.ids = Some(value);
        self
    }

    /// Validates required fields and builds [`UserAccountAutoLiqItemsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<UserAccountAutoLiqItemsQuery, crate::api::current::BuildError> {
        let ids = self
            .ids
            .ok_or(crate::api::current::BuildError::missing("ids"))?;
        if ids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "ids",
                "must not be empty",
            ));
        }
        Ok(UserAccountAutoLiqItemsQuery { ids })
    }
}

impl crate::api::current::support::CurrentQuery for UserAccountAutoLiqItemsQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        if self.ids.is_empty() {
            return Err(crate::Error::InvalidRequest {
                field: "ids",
                reason: "must not be empty",
            });
        }
        let mut pairs = Vec::new();
        for value in &self.ids {
            crate::api::current::support::push_query_value(&mut pairs, "ids", value)?;
        }
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /userAccountAutoLiq/items` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn user_account_auto_liq_items(
        &self,
        query: &UserAccountAutoLiqItemsQuery,
    ) -> Result<Vec<super::users::UserAccountAutoLiq>, crate::Error> {
        self.get_current("/userAccountAutoLiq/items", query).await
    }
}

/// Typed query parameters for `/userAccountAutoLiq/ldeps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct UserAccountAutoLiqLDependentsQuery {
    #[serde(rename = "masterids")]
    masterids: Vec<super::ids::ProviderEntityId>,
}

impl UserAccountAutoLiqLDependentsQuery {
    /// Returns wire field `masterids`.
    #[must_use]
    pub fn masterids(&self) -> &[super::ids::ProviderEntityId] {
        &self.masterids
    }

    /// Starts a builder for [`UserAccountAutoLiqLDependentsQuery`].
    pub fn builder() -> UserAccountAutoLiqLDependentsQueryBuilder {
        UserAccountAutoLiqLDependentsQueryBuilder::default()
    }
}

/// Builder for [`UserAccountAutoLiqLDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct UserAccountAutoLiqLDependentsQueryBuilder {
    masterids: Option<Vec<super::ids::ProviderEntityId>>,
}

impl UserAccountAutoLiqLDependentsQueryBuilder {
    /// Sets wire field `masterids`.
    pub fn masterids(mut self, value: Vec<super::ids::ProviderEntityId>) -> Self {
        self.masterids = Some(value);
        self
    }

    /// Validates required fields and builds [`UserAccountAutoLiqLDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(
        self,
    ) -> Result<UserAccountAutoLiqLDependentsQuery, crate::api::current::BuildError> {
        let masterids = self
            .masterids
            .ok_or(crate::api::current::BuildError::missing("masterids"))?;
        if masterids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "masterids",
                "must not be empty",
            ));
        }
        Ok(UserAccountAutoLiqLDependentsQuery { masterids })
    }
}

impl crate::api::current::support::CurrentQuery for UserAccountAutoLiqLDependentsQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        if self.masterids.is_empty() {
            return Err(crate::Error::InvalidRequest {
                field: "masterids",
                reason: "must not be empty",
            });
        }
        let mut pairs = Vec::new();
        for value in &self.masterids {
            crate::api::current::support::push_query_value(&mut pairs, "masterids", value)?;
        }
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /userAccountAutoLiq/ldeps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn user_account_auto_liq_l_dependents(
        &self,
        query: &UserAccountAutoLiqLDependentsQuery,
    ) -> Result<Vec<super::users::UserAccountAutoLiq>, crate::Error> {
        self.get_current("/userAccountAutoLiq/ldeps", query).await
    }
}

impl crate::Client {
    /// Calls the current `GET /userAccountAutoLiq/list` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn user_account_auto_liq_list(
        &self,
    ) -> Result<Vec<super::users::UserAccountAutoLiq>, crate::Error> {
        self.get_without_query("/userAccountAutoLiq/list").await
    }
}

/// Typed query parameters for `/userAccountPositionLimit/deps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct UserAccountPositionLimitDependentsQuery {
    #[serde(rename = "masterid")]
    masterid: super::ids::ProviderEntityId,
}

impl UserAccountPositionLimitDependentsQuery {
    /// Returns wire field `masterid`.
    #[must_use]
    pub fn masterid(&self) -> &super::ids::ProviderEntityId {
        &self.masterid
    }

    /// Starts a builder for [`UserAccountPositionLimitDependentsQuery`].
    pub fn builder() -> UserAccountPositionLimitDependentsQueryBuilder {
        UserAccountPositionLimitDependentsQueryBuilder::default()
    }
}

/// Builder for [`UserAccountPositionLimitDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct UserAccountPositionLimitDependentsQueryBuilder {
    masterid: Option<super::ids::ProviderEntityId>,
}

impl UserAccountPositionLimitDependentsQueryBuilder {
    /// Sets wire field `masterid`.
    pub fn masterid(mut self, value: super::ids::ProviderEntityId) -> Self {
        self.masterid = Some(value);
        self
    }

    /// Validates required fields and builds [`UserAccountPositionLimitDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(
        self,
    ) -> Result<UserAccountPositionLimitDependentsQuery, crate::api::current::BuildError> {
        let masterid = self
            .masterid
            .ok_or(crate::api::current::BuildError::missing("masterid"))?;
        Ok(UserAccountPositionLimitDependentsQuery { masterid })
    }
}

impl crate::api::current::support::CurrentQuery for UserAccountPositionLimitDependentsQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "masterid", &self.masterid)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /userAccountPositionLimit/deps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn user_account_position_limit_dependents(
        &self,
        query: &UserAccountPositionLimitDependentsQuery,
    ) -> Result<Vec<UserAccountPositionLimit>, crate::Error> {
        self.get_current("/userAccountPositionLimit/deps", query)
            .await
    }
}

/// Typed query parameters for `/userAccountPositionLimit/item`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct UserAccountPositionLimitItemQuery {
    #[serde(rename = "id")]
    id: super::ids::UserAccountPositionLimitId,
}

impl UserAccountPositionLimitItemQuery {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> &super::ids::UserAccountPositionLimitId {
        &self.id
    }

    /// Starts a builder for [`UserAccountPositionLimitItemQuery`].
    pub fn builder() -> UserAccountPositionLimitItemQueryBuilder {
        UserAccountPositionLimitItemQueryBuilder::default()
    }
}

/// Builder for [`UserAccountPositionLimitItemQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct UserAccountPositionLimitItemQueryBuilder {
    id: Option<super::ids::UserAccountPositionLimitId>,
}

impl UserAccountPositionLimitItemQueryBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::UserAccountPositionLimitId) -> Self {
        self.id = Some(value);
        self
    }

    /// Validates required fields and builds [`UserAccountPositionLimitItemQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(
        self,
    ) -> Result<UserAccountPositionLimitItemQuery, crate::api::current::BuildError> {
        let id = self
            .id
            .ok_or(crate::api::current::BuildError::missing("id"))?;
        Ok(UserAccountPositionLimitItemQuery { id })
    }
}

impl crate::api::current::support::CurrentQuery for UserAccountPositionLimitItemQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "id", &self.id)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /userAccountPositionLimit/item` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn user_account_position_limit_item(
        &self,
        query: &UserAccountPositionLimitItemQuery,
    ) -> Result<UserAccountPositionLimit, crate::Error> {
        self.get_current("/userAccountPositionLimit/item", query)
            .await
    }
}

/// Typed query parameters for `/userAccountPositionLimit/items`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct UserAccountPositionLimitItemsQuery {
    #[serde(rename = "ids")]
    ids: Vec<super::ids::UserAccountPositionLimitId>,
}

impl UserAccountPositionLimitItemsQuery {
    /// Returns wire field `ids`.
    #[must_use]
    pub fn ids(&self) -> &[super::ids::UserAccountPositionLimitId] {
        &self.ids
    }

    /// Starts a builder for [`UserAccountPositionLimitItemsQuery`].
    pub fn builder() -> UserAccountPositionLimitItemsQueryBuilder {
        UserAccountPositionLimitItemsQueryBuilder::default()
    }
}

/// Builder for [`UserAccountPositionLimitItemsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct UserAccountPositionLimitItemsQueryBuilder {
    ids: Option<Vec<super::ids::UserAccountPositionLimitId>>,
}

impl UserAccountPositionLimitItemsQueryBuilder {
    /// Sets wire field `ids`.
    pub fn ids(mut self, value: Vec<super::ids::UserAccountPositionLimitId>) -> Self {
        self.ids = Some(value);
        self
    }

    /// Validates required fields and builds [`UserAccountPositionLimitItemsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(
        self,
    ) -> Result<UserAccountPositionLimitItemsQuery, crate::api::current::BuildError> {
        let ids = self
            .ids
            .ok_or(crate::api::current::BuildError::missing("ids"))?;
        if ids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "ids",
                "must not be empty",
            ));
        }
        Ok(UserAccountPositionLimitItemsQuery { ids })
    }
}

impl crate::api::current::support::CurrentQuery for UserAccountPositionLimitItemsQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        if self.ids.is_empty() {
            return Err(crate::Error::InvalidRequest {
                field: "ids",
                reason: "must not be empty",
            });
        }
        let mut pairs = Vec::new();
        for value in &self.ids {
            crate::api::current::support::push_query_value(&mut pairs, "ids", value)?;
        }
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /userAccountPositionLimit/items` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn user_account_position_limit_items(
        &self,
        query: &UserAccountPositionLimitItemsQuery,
    ) -> Result<Vec<UserAccountPositionLimit>, crate::Error> {
        self.get_current("/userAccountPositionLimit/items", query)
            .await
    }
}

/// Typed query parameters for `/userAccountPositionLimit/ldeps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct UserAccountPositionLimitLDependentsQuery {
    #[serde(rename = "masterids")]
    masterids: Vec<super::ids::ProviderEntityId>,
}

impl UserAccountPositionLimitLDependentsQuery {
    /// Returns wire field `masterids`.
    #[must_use]
    pub fn masterids(&self) -> &[super::ids::ProviderEntityId] {
        &self.masterids
    }

    /// Starts a builder for [`UserAccountPositionLimitLDependentsQuery`].
    pub fn builder() -> UserAccountPositionLimitLDependentsQueryBuilder {
        UserAccountPositionLimitLDependentsQueryBuilder::default()
    }
}

/// Builder for [`UserAccountPositionLimitLDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct UserAccountPositionLimitLDependentsQueryBuilder {
    masterids: Option<Vec<super::ids::ProviderEntityId>>,
}

impl UserAccountPositionLimitLDependentsQueryBuilder {
    /// Sets wire field `masterids`.
    pub fn masterids(mut self, value: Vec<super::ids::ProviderEntityId>) -> Self {
        self.masterids = Some(value);
        self
    }

    /// Validates required fields and builds [`UserAccountPositionLimitLDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(
        self,
    ) -> Result<UserAccountPositionLimitLDependentsQuery, crate::api::current::BuildError> {
        let masterids = self
            .masterids
            .ok_or(crate::api::current::BuildError::missing("masterids"))?;
        if masterids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "masterids",
                "must not be empty",
            ));
        }
        Ok(UserAccountPositionLimitLDependentsQuery { masterids })
    }
}

impl crate::api::current::support::CurrentQuery for UserAccountPositionLimitLDependentsQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        if self.masterids.is_empty() {
            return Err(crate::Error::InvalidRequest {
                field: "masterids",
                reason: "must not be empty",
            });
        }
        let mut pairs = Vec::new();
        for value in &self.masterids {
            crate::api::current::support::push_query_value(&mut pairs, "masterids", value)?;
        }
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /userAccountPositionLimit/ldeps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn user_account_position_limit_l_dependents(
        &self,
        query: &UserAccountPositionLimitLDependentsQuery,
    ) -> Result<Vec<UserAccountPositionLimit>, crate::Error> {
        self.get_current("/userAccountPositionLimit/ldeps", query)
            .await
    }
}

/// Typed query parameters for `/userAccountRiskParameter/deps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct UserAccountRiskParameterDependentsQuery {
    #[serde(rename = "masterid")]
    masterid: super::ids::ProviderEntityId,
}

impl UserAccountRiskParameterDependentsQuery {
    /// Returns wire field `masterid`.
    #[must_use]
    pub fn masterid(&self) -> &super::ids::ProviderEntityId {
        &self.masterid
    }

    /// Starts a builder for [`UserAccountRiskParameterDependentsQuery`].
    pub fn builder() -> UserAccountRiskParameterDependentsQueryBuilder {
        UserAccountRiskParameterDependentsQueryBuilder::default()
    }
}

/// Builder for [`UserAccountRiskParameterDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct UserAccountRiskParameterDependentsQueryBuilder {
    masterid: Option<super::ids::ProviderEntityId>,
}

impl UserAccountRiskParameterDependentsQueryBuilder {
    /// Sets wire field `masterid`.
    pub fn masterid(mut self, value: super::ids::ProviderEntityId) -> Self {
        self.masterid = Some(value);
        self
    }

    /// Validates required fields and builds [`UserAccountRiskParameterDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(
        self,
    ) -> Result<UserAccountRiskParameterDependentsQuery, crate::api::current::BuildError> {
        let masterid = self
            .masterid
            .ok_or(crate::api::current::BuildError::missing("masterid"))?;
        Ok(UserAccountRiskParameterDependentsQuery { masterid })
    }
}

impl crate::api::current::support::CurrentQuery for UserAccountRiskParameterDependentsQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "masterid", &self.masterid)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /userAccountRiskParameter/deps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn user_account_risk_parameter_dependents(
        &self,
        query: &UserAccountRiskParameterDependentsQuery,
    ) -> Result<Vec<UserAccountRiskParameter>, crate::Error> {
        self.get_current("/userAccountRiskParameter/deps", query)
            .await
    }
}

/// Typed query parameters for `/userAccountRiskParameter/item`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct UserAccountRiskParameterItemQuery {
    #[serde(rename = "id")]
    id: super::ids::UserAccountRiskParameterId,
}

impl UserAccountRiskParameterItemQuery {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> &super::ids::UserAccountRiskParameterId {
        &self.id
    }

    /// Starts a builder for [`UserAccountRiskParameterItemQuery`].
    pub fn builder() -> UserAccountRiskParameterItemQueryBuilder {
        UserAccountRiskParameterItemQueryBuilder::default()
    }
}

/// Builder for [`UserAccountRiskParameterItemQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct UserAccountRiskParameterItemQueryBuilder {
    id: Option<super::ids::UserAccountRiskParameterId>,
}

impl UserAccountRiskParameterItemQueryBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::UserAccountRiskParameterId) -> Self {
        self.id = Some(value);
        self
    }

    /// Validates required fields and builds [`UserAccountRiskParameterItemQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(
        self,
    ) -> Result<UserAccountRiskParameterItemQuery, crate::api::current::BuildError> {
        let id = self
            .id
            .ok_or(crate::api::current::BuildError::missing("id"))?;
        Ok(UserAccountRiskParameterItemQuery { id })
    }
}

impl crate::api::current::support::CurrentQuery for UserAccountRiskParameterItemQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "id", &self.id)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /userAccountRiskParameter/item` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn user_account_risk_parameter_item(
        &self,
        query: &UserAccountRiskParameterItemQuery,
    ) -> Result<UserAccountRiskParameter, crate::Error> {
        self.get_current("/userAccountRiskParameter/item", query)
            .await
    }
}

/// Typed query parameters for `/userAccountRiskParameter/items`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct UserAccountRiskParameterItemsQuery {
    #[serde(rename = "ids")]
    ids: Vec<super::ids::UserAccountRiskParameterId>,
}

impl UserAccountRiskParameterItemsQuery {
    /// Returns wire field `ids`.
    #[must_use]
    pub fn ids(&self) -> &[super::ids::UserAccountRiskParameterId] {
        &self.ids
    }

    /// Starts a builder for [`UserAccountRiskParameterItemsQuery`].
    pub fn builder() -> UserAccountRiskParameterItemsQueryBuilder {
        UserAccountRiskParameterItemsQueryBuilder::default()
    }
}

/// Builder for [`UserAccountRiskParameterItemsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct UserAccountRiskParameterItemsQueryBuilder {
    ids: Option<Vec<super::ids::UserAccountRiskParameterId>>,
}

impl UserAccountRiskParameterItemsQueryBuilder {
    /// Sets wire field `ids`.
    pub fn ids(mut self, value: Vec<super::ids::UserAccountRiskParameterId>) -> Self {
        self.ids = Some(value);
        self
    }

    /// Validates required fields and builds [`UserAccountRiskParameterItemsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(
        self,
    ) -> Result<UserAccountRiskParameterItemsQuery, crate::api::current::BuildError> {
        let ids = self
            .ids
            .ok_or(crate::api::current::BuildError::missing("ids"))?;
        if ids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "ids",
                "must not be empty",
            ));
        }
        Ok(UserAccountRiskParameterItemsQuery { ids })
    }
}

impl crate::api::current::support::CurrentQuery for UserAccountRiskParameterItemsQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        if self.ids.is_empty() {
            return Err(crate::Error::InvalidRequest {
                field: "ids",
                reason: "must not be empty",
            });
        }
        let mut pairs = Vec::new();
        for value in &self.ids {
            crate::api::current::support::push_query_value(&mut pairs, "ids", value)?;
        }
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /userAccountRiskParameter/items` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn user_account_risk_parameter_items(
        &self,
        query: &UserAccountRiskParameterItemsQuery,
    ) -> Result<Vec<UserAccountRiskParameter>, crate::Error> {
        self.get_current("/userAccountRiskParameter/items", query)
            .await
    }
}

/// Typed query parameters for `/userAccountRiskParameter/ldeps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct UserAccountRiskParameterLDependentsQuery {
    #[serde(rename = "masterids")]
    masterids: Vec<super::ids::ProviderEntityId>,
}

impl UserAccountRiskParameterLDependentsQuery {
    /// Returns wire field `masterids`.
    #[must_use]
    pub fn masterids(&self) -> &[super::ids::ProviderEntityId] {
        &self.masterids
    }

    /// Starts a builder for [`UserAccountRiskParameterLDependentsQuery`].
    pub fn builder() -> UserAccountRiskParameterLDependentsQueryBuilder {
        UserAccountRiskParameterLDependentsQueryBuilder::default()
    }
}

/// Builder for [`UserAccountRiskParameterLDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct UserAccountRiskParameterLDependentsQueryBuilder {
    masterids: Option<Vec<super::ids::ProviderEntityId>>,
}

impl UserAccountRiskParameterLDependentsQueryBuilder {
    /// Sets wire field `masterids`.
    pub fn masterids(mut self, value: Vec<super::ids::ProviderEntityId>) -> Self {
        self.masterids = Some(value);
        self
    }

    /// Validates required fields and builds [`UserAccountRiskParameterLDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(
        self,
    ) -> Result<UserAccountRiskParameterLDependentsQuery, crate::api::current::BuildError> {
        let masterids = self
            .masterids
            .ok_or(crate::api::current::BuildError::missing("masterids"))?;
        if masterids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "masterids",
                "must not be empty",
            ));
        }
        Ok(UserAccountRiskParameterLDependentsQuery { masterids })
    }
}

impl crate::api::current::support::CurrentQuery for UserAccountRiskParameterLDependentsQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        if self.masterids.is_empty() {
            return Err(crate::Error::InvalidRequest {
                field: "masterids",
                reason: "must not be empty",
            });
        }
        let mut pairs = Vec::new();
        for value in &self.masterids {
            crate::api::current::support::push_query_value(&mut pairs, "masterids", value)?;
        }
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /userAccountRiskParameter/ldeps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn user_account_risk_parameter_l_dependents(
        &self,
        query: &UserAccountRiskParameterLDependentsQuery,
    ) -> Result<Vec<UserAccountRiskParameter>, crate::Error> {
        self.get_current("/userAccountRiskParameter/ldeps", query)
            .await
    }
}

// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary
// @generated
// Generator: tools/generate_openapi.py
// Source: https://partner.tradovate.com/openapi.json (snapshot 2026-08-21, sha256 37caeccf4b0913460a788fcaf4c902497059b8ffe6f6355512e6c08eaacde769)

// Provider wire fields remain schema-auditable even when they repeat
// their type name; wide schema-faithful builders remain one generated
// unit so regeneration and source review cannot drift field subsets.
#![allow(clippy::struct_field_names, clippy::too_many_lines)]

//! Current order, command, execution, and fill operations.

/// Current wire model `CancelOrder`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct CancelOrder {
    #[serde(rename = "orderId")]
    order_id: crate::OrderId,
    #[serde(rename = "clOrdId", default, skip_serializing_if = "Option::is_none")]
    cl_ord_id: Option<crate::ClientOrderId>,
    #[serde(
        rename = "activationTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    activation_time: Option<jiff::Timestamp>,
    #[serde(
        rename = "customTag50",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    custom_tag50: Option<String>,
    #[serde(
        rename = "isAutomated",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    is_automated: Option<bool>,
}

impl CancelOrder {
    /// Returns wire field `orderId`.
    #[must_use]
    pub fn order_id(&self) -> &crate::OrderId {
        &self.order_id
    }

    /// Returns wire field `clOrdId`.
    #[must_use]
    pub fn cl_ord_id(&self) -> Option<&crate::ClientOrderId> {
        self.cl_ord_id.as_ref()
    }

    /// Returns wire field `activationTime`.
    #[must_use]
    pub fn activation_time(&self) -> Option<&jiff::Timestamp> {
        self.activation_time.as_ref()
    }

    /// Returns wire field `customTag50`.
    #[must_use]
    pub fn custom_tag50(&self) -> Option<&str> {
        self.custom_tag50.as_deref()
    }

    /// Returns wire field `isAutomated`.
    #[must_use]
    pub fn is_automated(&self) -> Option<&bool> {
        self.is_automated.as_ref()
    }

    /// Starts a builder for [`CancelOrder`].
    pub fn builder() -> CancelOrderBuilder {
        CancelOrderBuilder::default()
    }
}

/// Builder for [`CancelOrder`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct CancelOrderBuilder {
    order_id: Option<crate::OrderId>,
    cl_ord_id: Option<crate::ClientOrderId>,
    activation_time: Option<jiff::Timestamp>,
    custom_tag50: Option<String>,
    is_automated: Option<bool>,
}

impl CancelOrderBuilder {
    /// Sets wire field `orderId`.
    pub fn order_id(mut self, value: crate::OrderId) -> Self {
        self.order_id = Some(value);
        self
    }

    /// Sets wire field `clOrdId`.
    pub fn cl_ord_id(mut self, value: crate::ClientOrderId) -> Self {
        self.cl_ord_id = Some(value);
        self
    }

    /// Sets wire field `activationTime`.
    pub fn activation_time(mut self, value: jiff::Timestamp) -> Self {
        self.activation_time = Some(value);
        self
    }

    /// Sets wire field `customTag50`.
    pub fn custom_tag50(mut self, value: impl Into<String>) -> Self {
        self.custom_tag50 = Some(value.into());
        self
    }

    /// Sets wire field `isAutomated`.
    pub fn is_automated(mut self, value: bool) -> Self {
        self.is_automated = Some(value);
        self
    }

    /// Validates required fields and builds [`CancelOrder`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<CancelOrder, crate::api::current::BuildError> {
        let order_id = self
            .order_id
            .ok_or(crate::api::current::BuildError::missing("orderId"))?;
        Ok(CancelOrder {
            order_id,
            cl_ord_id: self.cl_ord_id,
            activation_time: self.activation_time,
            custom_tag50: self.custom_tag50,
            is_automated: self.is_automated,
        })
    }
}

impl crate::api::current::support::CurrentRequest for CancelOrder {
    fn validate_current(&self) -> Result<(), crate::Error> {
        Ok(())
    }
}

/// Current wire model `CommandResult`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct CommandResult {
    #[serde(
        rename = "failureReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    failure_reason: Option<CommandResultFailureReason>,
    #[serde(
        rename = "failureText",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    failure_text: Option<String>,
    #[serde(rename = "commandId", default, skip_serializing_if = "Option::is_none")]
    command_id: Option<crate::CommandId>,
}

impl CommandResult {
    /// Returns wire field `failureReason`.
    #[must_use]
    pub fn failure_reason(&self) -> Option<&CommandResultFailureReason> {
        self.failure_reason.as_ref()
    }

    /// Returns wire field `failureText`.
    #[must_use]
    pub fn failure_text(&self) -> Option<&str> {
        self.failure_text.as_deref()
    }

    /// Returns wire field `commandId`.
    #[must_use]
    pub fn command_id(&self) -> Option<&crate::CommandId> {
        self.command_id.as_ref()
    }

    /// Starts a builder for [`CommandResult`].
    pub fn builder() -> CommandResultBuilder {
        CommandResultBuilder::default()
    }
}

/// Builder for [`CommandResult`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct CommandResultBuilder {
    failure_reason: Option<CommandResultFailureReason>,
    failure_text: Option<String>,
    command_id: Option<crate::CommandId>,
}

impl CommandResultBuilder {
    /// Sets wire field `failureReason`.
    pub fn failure_reason(mut self, value: CommandResultFailureReason) -> Self {
        self.failure_reason = Some(value);
        self
    }

    /// Sets wire field `failureText`.
    pub fn failure_text(mut self, value: impl Into<String>) -> Self {
        self.failure_text = Some(value.into());
        self
    }

    /// Sets wire field `commandId`.
    pub fn command_id(mut self, value: crate::CommandId) -> Self {
        self.command_id = Some(value);
        self
    }

    /// Validates required fields and builds [`CommandResult`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<CommandResult, crate::api::current::BuildError> {
        Ok(CommandResult {
            failure_reason: self.failure_reason,
            failure_text: self.failure_text,
            command_id: self.command_id,
        })
    }
}

/// Current provider values for `CommandResultFailureReason`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum CommandResultFailureReason {
    /// Provider value `AccountClosed`.
    AccountClosed,
    /// Provider value `AdvancedTrailingStopUnsupported`.
    AdvancedTrailingStopUnsupported,
    /// Provider value `AnotherCommandPending`.
    AnotherCommandPending,
    /// Provider value `BackMonthProhibited`.
    BackMonthProhibited,
    /// Provider value `ExecutionProviderNotConfigured`.
    ExecutionProviderNotConfigured,
    /// Provider value `ExecutionProviderUnavailable`.
    ExecutionProviderUnavailable,
    /// Provider value `InvalidContract`.
    InvalidContract,
    /// Provider value `InvalidPrice`.
    InvalidPrice,
    /// Provider value `KeyInformationDocumentRequired`.
    KeyInformationDocumentRequired,
    /// Provider value `LiquidationOnly`.
    LiquidationOnly,
    /// Provider value `LiquidationOnlyBeforeExpiration`.
    LiquidationOnlyBeforeExpiration,
    /// Provider value `MaxOrderQtyIsNotSpecified`.
    MaxOrderQtyIsNotSpecified,
    /// Provider value `MaxOrderQtyLimitReached`.
    MaxOrderQtyLimitReached,
    /// Provider value `MaxPosLimitMisconfigured`.
    MaxPosLimitMisconfigured,
    /// Provider value `MaxPosLimitReached`.
    MaxPosLimitReached,
    /// Provider value `MaxTotalPosLimitReached`.
    MaxTotalPosLimitReached,
    /// Provider value `MultipleAccountPlanRequired`.
    MultipleAccountPlanRequired,
    /// Provider value `NoQuote`.
    NoQuote,
    /// Provider value `NotEnoughLiquidity`.
    NotEnoughLiquidity,
    /// Provider value `OtherExecutionRelated`.
    OtherExecutionRelated,
    /// Provider value `ParentRejected`.
    ParentRejected,
    /// Provider value `RiskCheckTimeout`.
    RiskCheckTimeout,
    /// Provider value `SSFRiskDisclosureAcknowledgmentRequired`.
    SsfRiskDisclosureAcknowledgmentRequired,
    /// Provider value `SessionClosed`.
    SessionClosed,
    /// Provider value `Success`.
    Success,
    /// Provider value `TooLate`.
    TooLate,
    /// Provider value `TradingLocked`.
    TradingLocked,
    /// Provider value `TrailingStopNonOrderQtyModify`.
    TrailingStopNonOrderQtyModify,
    /// Provider value `Unauthorized`.
    Unauthorized,
    /// Provider value `UnknownReason`.
    UnknownReason,
    /// Provider value `Unsupported`.
    Unsupported,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl CommandResultFailureReason {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::AccountClosed => "AccountClosed",
            Self::AdvancedTrailingStopUnsupported => "AdvancedTrailingStopUnsupported",
            Self::AnotherCommandPending => "AnotherCommandPending",
            Self::BackMonthProhibited => "BackMonthProhibited",
            Self::ExecutionProviderNotConfigured => "ExecutionProviderNotConfigured",
            Self::ExecutionProviderUnavailable => "ExecutionProviderUnavailable",
            Self::InvalidContract => "InvalidContract",
            Self::InvalidPrice => "InvalidPrice",
            Self::KeyInformationDocumentRequired => "KeyInformationDocumentRequired",
            Self::LiquidationOnly => "LiquidationOnly",
            Self::LiquidationOnlyBeforeExpiration => "LiquidationOnlyBeforeExpiration",
            Self::MaxOrderQtyIsNotSpecified => "MaxOrderQtyIsNotSpecified",
            Self::MaxOrderQtyLimitReached => "MaxOrderQtyLimitReached",
            Self::MaxPosLimitMisconfigured => "MaxPosLimitMisconfigured",
            Self::MaxPosLimitReached => "MaxPosLimitReached",
            Self::MaxTotalPosLimitReached => "MaxTotalPosLimitReached",
            Self::MultipleAccountPlanRequired => "MultipleAccountPlanRequired",
            Self::NoQuote => "NoQuote",
            Self::NotEnoughLiquidity => "NotEnoughLiquidity",
            Self::OtherExecutionRelated => "OtherExecutionRelated",
            Self::ParentRejected => "ParentRejected",
            Self::RiskCheckTimeout => "RiskCheckTimeout",
            Self::SsfRiskDisclosureAcknowledgmentRequired => {
                "SSFRiskDisclosureAcknowledgmentRequired"
            }
            Self::SessionClosed => "SessionClosed",
            Self::Success => "Success",
            Self::TooLate => "TooLate",
            Self::TradingLocked => "TradingLocked",
            Self::TrailingStopNonOrderQtyModify => "TrailingStopNonOrderQtyModify",
            Self::Unauthorized => "Unauthorized",
            Self::UnknownReason => "UnknownReason",
            Self::Unsupported => "Unsupported",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for CommandResultFailureReason {
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

impl<'de> serde::Deserialize<'de> for CommandResultFailureReason {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "AccountClosed" => Self::AccountClosed,
            "AdvancedTrailingStopUnsupported" => Self::AdvancedTrailingStopUnsupported,
            "AnotherCommandPending" => Self::AnotherCommandPending,
            "BackMonthProhibited" => Self::BackMonthProhibited,
            "ExecutionProviderNotConfigured" => Self::ExecutionProviderNotConfigured,
            "ExecutionProviderUnavailable" => Self::ExecutionProviderUnavailable,
            "InvalidContract" => Self::InvalidContract,
            "InvalidPrice" => Self::InvalidPrice,
            "KeyInformationDocumentRequired" => Self::KeyInformationDocumentRequired,
            "LiquidationOnly" => Self::LiquidationOnly,
            "LiquidationOnlyBeforeExpiration" => Self::LiquidationOnlyBeforeExpiration,
            "MaxOrderQtyIsNotSpecified" => Self::MaxOrderQtyIsNotSpecified,
            "MaxOrderQtyLimitReached" => Self::MaxOrderQtyLimitReached,
            "MaxPosLimitMisconfigured" => Self::MaxPosLimitMisconfigured,
            "MaxPosLimitReached" => Self::MaxPosLimitReached,
            "MaxTotalPosLimitReached" => Self::MaxTotalPosLimitReached,
            "MultipleAccountPlanRequired" => Self::MultipleAccountPlanRequired,
            "NoQuote" => Self::NoQuote,
            "NotEnoughLiquidity" => Self::NotEnoughLiquidity,
            "OtherExecutionRelated" => Self::OtherExecutionRelated,
            "ParentRejected" => Self::ParentRejected,
            "RiskCheckTimeout" => Self::RiskCheckTimeout,
            "SSFRiskDisclosureAcknowledgmentRequired" => {
                Self::SsfRiskDisclosureAcknowledgmentRequired
            }
            "SessionClosed" => Self::SessionClosed,
            "Success" => Self::Success,
            "TooLate" => Self::TooLate,
            "TradingLocked" => Self::TradingLocked,
            "TrailingStopNonOrderQtyModify" => Self::TrailingStopNonOrderQtyModify,
            "Unauthorized" => Self::Unauthorized,
            "UnknownReason" => Self::UnknownReason,
            "Unsupported" => Self::Unsupported,
            _ => Self::Unknown(value),
        })
    }
}

/// Current wire model `DryRun`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct DryRun {
    #[serde(rename = "accountId")]
    account_id: crate::AccountId,
    #[serde(rename = "orders")]
    orders: Vec<DryRunOrder>,
    #[serde(
        rename = "extraPreTradeRisk",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    extra_pre_trade_risk: Option<ExtraPreTradeRisk>,
}

impl DryRun {
    /// Returns wire field `accountId`.
    #[must_use]
    pub fn account_id(&self) -> &crate::AccountId {
        &self.account_id
    }

    /// Returns wire field `orders`.
    #[must_use]
    pub fn orders(&self) -> &[DryRunOrder] {
        &self.orders
    }

    /// Returns wire field `extraPreTradeRisk`.
    #[must_use]
    pub fn extra_pre_trade_risk(&self) -> Option<&ExtraPreTradeRisk> {
        self.extra_pre_trade_risk.as_ref()
    }

    /// Starts a builder for [`DryRun`].
    pub fn builder() -> DryRunBuilder {
        DryRunBuilder::default()
    }
}

/// Builder for [`DryRun`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct DryRunBuilder {
    account_id: Option<crate::AccountId>,
    orders: Option<Vec<DryRunOrder>>,
    extra_pre_trade_risk: Option<ExtraPreTradeRisk>,
}

impl DryRunBuilder {
    /// Sets wire field `accountId`.
    pub fn account_id(mut self, value: crate::AccountId) -> Self {
        self.account_id = Some(value);
        self
    }

    /// Sets wire field `orders`.
    pub fn orders(mut self, value: Vec<DryRunOrder>) -> Self {
        self.orders = Some(value);
        self
    }

    /// Sets wire field `extraPreTradeRisk`.
    pub fn extra_pre_trade_risk(mut self, value: ExtraPreTradeRisk) -> Self {
        self.extra_pre_trade_risk = Some(value);
        self
    }

    /// Validates required fields and builds [`DryRun`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<DryRun, crate::api::current::BuildError> {
        let account_id = self
            .account_id
            .ok_or(crate::api::current::BuildError::missing("accountId"))?;
        let orders = self
            .orders
            .ok_or(crate::api::current::BuildError::missing("orders"))?;
        if orders.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "orders",
                "must not be empty",
            ));
        }
        Ok(DryRun {
            account_id,
            orders,
            extra_pre_trade_risk: self.extra_pre_trade_risk,
        })
    }
}

impl crate::api::current::support::CurrentRequest for DryRun {
    fn validate_current(&self) -> Result<(), crate::Error> {
        if self.orders.is_empty() {
            return Err(crate::Error::InvalidRequest {
                field: "orders",
                reason: "must not be empty",
            });
        }
        Ok(())
    }
}

/// Current wire model `DryRunOrder`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct DryRunOrder {
    #[serde(rename = "contractId")]
    contract_id: crate::ContractId,
    #[serde(rename = "action")]
    action: DryRunOrderAction,
    #[serde(rename = "orderQty")]
    order_qty: i64,
    #[serde(rename = "orderType")]
    order_type: DryRunOrderOrderType,
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "crate::decimal::option")]
    price: Option<crate::Decimal>,
    #[serde(rename = "stopPrice", default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "crate::decimal::option")]
    stop_price: Option<crate::Decimal>,
}

impl DryRunOrder {
    /// Returns wire field `contractId`.
    #[must_use]
    pub fn contract_id(&self) -> &crate::ContractId {
        &self.contract_id
    }

    /// Returns wire field `action`.
    #[must_use]
    pub fn action(&self) -> &DryRunOrderAction {
        &self.action
    }

    /// Returns wire field `orderQty`.
    #[must_use]
    pub fn order_qty(&self) -> &i64 {
        &self.order_qty
    }

    /// Returns wire field `orderType`.
    #[must_use]
    pub fn order_type(&self) -> &DryRunOrderOrderType {
        &self.order_type
    }

    /// Returns wire field `price`.
    #[must_use]
    pub fn price(&self) -> Option<&crate::Decimal> {
        self.price.as_ref()
    }

    /// Returns wire field `stopPrice`.
    #[must_use]
    pub fn stop_price(&self) -> Option<&crate::Decimal> {
        self.stop_price.as_ref()
    }

    /// Starts a builder for [`DryRunOrder`].
    pub fn builder() -> DryRunOrderBuilder {
        DryRunOrderBuilder::default()
    }
}

/// Builder for [`DryRunOrder`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct DryRunOrderBuilder {
    contract_id: Option<crate::ContractId>,
    action: Option<DryRunOrderAction>,
    order_qty: Option<i64>,
    order_type: Option<DryRunOrderOrderType>,
    price: Option<crate::Decimal>,
    stop_price: Option<crate::Decimal>,
}

impl DryRunOrderBuilder {
    /// Sets wire field `contractId`.
    pub fn contract_id(mut self, value: crate::ContractId) -> Self {
        self.contract_id = Some(value);
        self
    }

    /// Sets wire field `action`.
    pub fn action(mut self, value: DryRunOrderAction) -> Self {
        self.action = Some(value);
        self
    }

    /// Sets wire field `orderQty`.
    pub fn order_qty(mut self, value: i64) -> Self {
        self.order_qty = Some(value);
        self
    }

    /// Sets wire field `orderType`.
    pub fn order_type(mut self, value: DryRunOrderOrderType) -> Self {
        self.order_type = Some(value);
        self
    }

    /// Sets wire field `price`.
    pub fn price(mut self, value: crate::Decimal) -> Self {
        self.price = Some(value);
        self
    }

    /// Sets wire field `stopPrice`.
    pub fn stop_price(mut self, value: crate::Decimal) -> Self {
        self.stop_price = Some(value);
        self
    }

    /// Validates required fields and builds [`DryRunOrder`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<DryRunOrder, crate::api::current::BuildError> {
        let contract_id = self
            .contract_id
            .ok_or(crate::api::current::BuildError::missing("contractId"))?;
        let action = self
            .action
            .ok_or(crate::api::current::BuildError::missing("action"))?;
        let order_qty = self
            .order_qty
            .ok_or(crate::api::current::BuildError::missing("orderQty"))?;
        let order_type = self
            .order_type
            .ok_or(crate::api::current::BuildError::missing("orderType"))?;
        Ok(DryRunOrder {
            contract_id,
            action,
            order_qty,
            order_type,
            price: self.price,
            stop_price: self.stop_price,
        })
    }
}

/// Current provider values for `DryRunOrderAction`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum DryRunOrderAction {
    /// Provider value `Buy`.
    Buy,
    /// Provider value `Sell`.
    Sell,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl DryRunOrderAction {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Buy => "Buy",
            Self::Sell => "Sell",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for DryRunOrderAction {
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

impl<'de> serde::Deserialize<'de> for DryRunOrderAction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "Buy" => Self::Buy,
            "Sell" => Self::Sell,
            _ => Self::Unknown(value),
        })
    }
}

/// Current provider values for `DryRunOrderOrderType`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum DryRunOrderOrderType {
    /// Provider value `Limit`.
    Limit,
    /// Provider value `LimitIfTouched`.
    LimitIfTouched,
    /// Provider value `MIT`.
    Mit,
    /// Provider value `Market`.
    Market,
    /// Provider value `QTS`.
    Qts,
    /// Provider value `Stop`.
    Stop,
    /// Provider value `StopLimit`.
    StopLimit,
    /// Provider value `TrailingStop`.
    TrailingStop,
    /// Provider value `TrailingStopLimit`.
    TrailingStopLimit,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl DryRunOrderOrderType {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Limit => "Limit",
            Self::LimitIfTouched => "LimitIfTouched",
            Self::Mit => "MIT",
            Self::Market => "Market",
            Self::Qts => "QTS",
            Self::Stop => "Stop",
            Self::StopLimit => "StopLimit",
            Self::TrailingStop => "TrailingStop",
            Self::TrailingStopLimit => "TrailingStopLimit",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for DryRunOrderOrderType {
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

impl<'de> serde::Deserialize<'de> for DryRunOrderOrderType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "Limit" => Self::Limit,
            "LimitIfTouched" => Self::LimitIfTouched,
            "MIT" => Self::Mit,
            "Market" => Self::Market,
            "QTS" => Self::Qts,
            "Stop" => Self::Stop,
            "StopLimit" => Self::StopLimit,
            "TrailingStop" => Self::TrailingStop,
            "TrailingStopLimit" => Self::TrailingStopLimit,
            _ => Self::Unknown(value),
        })
    }
}

/// Current wire model `DryRunResponse`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct DryRunResponse {
    #[serde(rename = "errorText", default, skip_serializing_if = "Option::is_none")]
    error_text: Option<String>,
    #[serde(
        rename = "rejectReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    reject_reason: Option<DryRunResponseRejectReason>,
    #[serde(rename = "comment", default, skip_serializing_if = "Option::is_none")]
    comment: Option<String>,
    #[serde(rename = "details", default, skip_serializing_if = "Option::is_none")]
    details: Option<RiskEvaluationDetails>,
    #[serde(rename = "fees", default, skip_serializing_if = "Option::is_none")]
    fees: Option<EstimatedFillFee>,
}

impl DryRunResponse {
    /// Returns wire field `errorText`.
    #[must_use]
    pub fn error_text(&self) -> Option<&str> {
        self.error_text.as_deref()
    }

    /// Returns wire field `rejectReason`.
    #[must_use]
    pub fn reject_reason(&self) -> Option<&DryRunResponseRejectReason> {
        self.reject_reason.as_ref()
    }

    /// Returns wire field `comment`.
    #[must_use]
    pub fn comment(&self) -> Option<&str> {
        self.comment.as_deref()
    }

    /// Returns wire field `details`.
    #[must_use]
    pub fn details(&self) -> Option<&RiskEvaluationDetails> {
        self.details.as_ref()
    }

    /// Returns wire field `fees`.
    #[must_use]
    pub fn fees(&self) -> Option<&EstimatedFillFee> {
        self.fees.as_ref()
    }

    /// Starts a builder for [`DryRunResponse`].
    pub fn builder() -> DryRunResponseBuilder {
        DryRunResponseBuilder::default()
    }
}

/// Builder for [`DryRunResponse`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct DryRunResponseBuilder {
    error_text: Option<String>,
    reject_reason: Option<DryRunResponseRejectReason>,
    comment: Option<String>,
    details: Option<RiskEvaluationDetails>,
    fees: Option<EstimatedFillFee>,
}

impl DryRunResponseBuilder {
    /// Sets wire field `errorText`.
    pub fn error_text(mut self, value: impl Into<String>) -> Self {
        self.error_text = Some(value.into());
        self
    }

    /// Sets wire field `rejectReason`.
    pub fn reject_reason(mut self, value: DryRunResponseRejectReason) -> Self {
        self.reject_reason = Some(value);
        self
    }

    /// Sets wire field `comment`.
    pub fn comment(mut self, value: impl Into<String>) -> Self {
        self.comment = Some(value.into());
        self
    }

    /// Sets wire field `details`.
    pub fn details(mut self, value: RiskEvaluationDetails) -> Self {
        self.details = Some(value);
        self
    }

    /// Sets wire field `fees`.
    pub fn fees(mut self, value: EstimatedFillFee) -> Self {
        self.fees = Some(value);
        self
    }

    /// Validates required fields and builds [`DryRunResponse`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<DryRunResponse, crate::api::current::BuildError> {
        Ok(DryRunResponse {
            error_text: self.error_text,
            reject_reason: self.reject_reason,
            comment: self.comment,
            details: self.details,
            fees: self.fees,
        })
    }
}

/// Current provider values for `DryRunResponseRejectReason`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum DryRunResponseRejectReason {
    /// Provider value `AccountClosed`.
    AccountClosed,
    /// Provider value `AdvancedTrailingStopUnsupported`.
    AdvancedTrailingStopUnsupported,
    /// Provider value `AnotherCommandPending`.
    AnotherCommandPending,
    /// Provider value `BackMonthProhibited`.
    BackMonthProhibited,
    /// Provider value `ExecutionProviderNotConfigured`.
    ExecutionProviderNotConfigured,
    /// Provider value `ExecutionProviderUnavailable`.
    ExecutionProviderUnavailable,
    /// Provider value `InvalidContract`.
    InvalidContract,
    /// Provider value `InvalidPrice`.
    InvalidPrice,
    /// Provider value `KeyInformationDocumentRequired`.
    KeyInformationDocumentRequired,
    /// Provider value `LiquidationOnly`.
    LiquidationOnly,
    /// Provider value `LiquidationOnlyBeforeExpiration`.
    LiquidationOnlyBeforeExpiration,
    /// Provider value `MaxOrderQtyIsNotSpecified`.
    MaxOrderQtyIsNotSpecified,
    /// Provider value `MaxOrderQtyLimitReached`.
    MaxOrderQtyLimitReached,
    /// Provider value `MaxPosLimitMisconfigured`.
    MaxPosLimitMisconfigured,
    /// Provider value `MaxPosLimitReached`.
    MaxPosLimitReached,
    /// Provider value `MaxTotalPosLimitReached`.
    MaxTotalPosLimitReached,
    /// Provider value `MultipleAccountPlanRequired`.
    MultipleAccountPlanRequired,
    /// Provider value `NoQuote`.
    NoQuote,
    /// Provider value `NotEnoughLiquidity`.
    NotEnoughLiquidity,
    /// Provider value `OtherExecutionRelated`.
    OtherExecutionRelated,
    /// Provider value `ParentRejected`.
    ParentRejected,
    /// Provider value `RiskCheckTimeout`.
    RiskCheckTimeout,
    /// Provider value `SSFRiskDisclosureAcknowledgmentRequired`.
    SsfRiskDisclosureAcknowledgmentRequired,
    /// Provider value `SessionClosed`.
    SessionClosed,
    /// Provider value `Success`.
    Success,
    /// Provider value `TooLate`.
    TooLate,
    /// Provider value `TradingLocked`.
    TradingLocked,
    /// Provider value `TrailingStopNonOrderQtyModify`.
    TrailingStopNonOrderQtyModify,
    /// Provider value `Unauthorized`.
    Unauthorized,
    /// Provider value `UnknownReason`.
    UnknownReason,
    /// Provider value `Unsupported`.
    Unsupported,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl DryRunResponseRejectReason {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::AccountClosed => "AccountClosed",
            Self::AdvancedTrailingStopUnsupported => "AdvancedTrailingStopUnsupported",
            Self::AnotherCommandPending => "AnotherCommandPending",
            Self::BackMonthProhibited => "BackMonthProhibited",
            Self::ExecutionProviderNotConfigured => "ExecutionProviderNotConfigured",
            Self::ExecutionProviderUnavailable => "ExecutionProviderUnavailable",
            Self::InvalidContract => "InvalidContract",
            Self::InvalidPrice => "InvalidPrice",
            Self::KeyInformationDocumentRequired => "KeyInformationDocumentRequired",
            Self::LiquidationOnly => "LiquidationOnly",
            Self::LiquidationOnlyBeforeExpiration => "LiquidationOnlyBeforeExpiration",
            Self::MaxOrderQtyIsNotSpecified => "MaxOrderQtyIsNotSpecified",
            Self::MaxOrderQtyLimitReached => "MaxOrderQtyLimitReached",
            Self::MaxPosLimitMisconfigured => "MaxPosLimitMisconfigured",
            Self::MaxPosLimitReached => "MaxPosLimitReached",
            Self::MaxTotalPosLimitReached => "MaxTotalPosLimitReached",
            Self::MultipleAccountPlanRequired => "MultipleAccountPlanRequired",
            Self::NoQuote => "NoQuote",
            Self::NotEnoughLiquidity => "NotEnoughLiquidity",
            Self::OtherExecutionRelated => "OtherExecutionRelated",
            Self::ParentRejected => "ParentRejected",
            Self::RiskCheckTimeout => "RiskCheckTimeout",
            Self::SsfRiskDisclosureAcknowledgmentRequired => {
                "SSFRiskDisclosureAcknowledgmentRequired"
            }
            Self::SessionClosed => "SessionClosed",
            Self::Success => "Success",
            Self::TooLate => "TooLate",
            Self::TradingLocked => "TradingLocked",
            Self::TrailingStopNonOrderQtyModify => "TrailingStopNonOrderQtyModify",
            Self::Unauthorized => "Unauthorized",
            Self::UnknownReason => "UnknownReason",
            Self::Unsupported => "Unsupported",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for DryRunResponseRejectReason {
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

impl<'de> serde::Deserialize<'de> for DryRunResponseRejectReason {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "AccountClosed" => Self::AccountClosed,
            "AdvancedTrailingStopUnsupported" => Self::AdvancedTrailingStopUnsupported,
            "AnotherCommandPending" => Self::AnotherCommandPending,
            "BackMonthProhibited" => Self::BackMonthProhibited,
            "ExecutionProviderNotConfigured" => Self::ExecutionProviderNotConfigured,
            "ExecutionProviderUnavailable" => Self::ExecutionProviderUnavailable,
            "InvalidContract" => Self::InvalidContract,
            "InvalidPrice" => Self::InvalidPrice,
            "KeyInformationDocumentRequired" => Self::KeyInformationDocumentRequired,
            "LiquidationOnly" => Self::LiquidationOnly,
            "LiquidationOnlyBeforeExpiration" => Self::LiquidationOnlyBeforeExpiration,
            "MaxOrderQtyIsNotSpecified" => Self::MaxOrderQtyIsNotSpecified,
            "MaxOrderQtyLimitReached" => Self::MaxOrderQtyLimitReached,
            "MaxPosLimitMisconfigured" => Self::MaxPosLimitMisconfigured,
            "MaxPosLimitReached" => Self::MaxPosLimitReached,
            "MaxTotalPosLimitReached" => Self::MaxTotalPosLimitReached,
            "MultipleAccountPlanRequired" => Self::MultipleAccountPlanRequired,
            "NoQuote" => Self::NoQuote,
            "NotEnoughLiquidity" => Self::NotEnoughLiquidity,
            "OtherExecutionRelated" => Self::OtherExecutionRelated,
            "ParentRejected" => Self::ParentRejected,
            "RiskCheckTimeout" => Self::RiskCheckTimeout,
            "SSFRiskDisclosureAcknowledgmentRequired" => {
                Self::SsfRiskDisclosureAcknowledgmentRequired
            }
            "SessionClosed" => Self::SessionClosed,
            "Success" => Self::Success,
            "TooLate" => Self::TooLate,
            "TradingLocked" => Self::TradingLocked,
            "TrailingStopNonOrderQtyModify" => Self::TrailingStopNonOrderQtyModify,
            "Unauthorized" => Self::Unauthorized,
            "UnknownReason" => Self::UnknownReason,
            "Unsupported" => Self::Unsupported,
            _ => Self::Unknown(value),
        })
    }
}

/// Current wire model `EstimatedFillFee`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct EstimatedFillFee {
    #[serde(
        rename = "clearingFee",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    clearing_fee: Option<crate::Decimal>,
    #[serde(
        rename = "clearingCurrencyId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    clearing_currency_id: Option<super::ids::CurrencyId>,
    #[serde(
        rename = "exchangeFee",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    exchange_fee: Option<crate::Decimal>,
    #[serde(
        rename = "exchangeCurrencyId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    exchange_currency_id: Option<super::ids::CurrencyId>,
    #[serde(rename = "nfaFee", default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "crate::decimal::option")]
    nfa_fee: Option<crate::Decimal>,
    #[serde(
        rename = "nfaCurrencyId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    nfa_currency_id: Option<super::ids::CurrencyId>,
    #[serde(
        rename = "brokerageFee",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    brokerage_fee: Option<crate::Decimal>,
    #[serde(
        rename = "brokerageCurrencyId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    brokerage_currency_id: Option<super::ids::CurrencyId>,
    #[serde(rename = "ipFee", default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "crate::decimal::option")]
    ip_fee: Option<crate::Decimal>,
    #[serde(
        rename = "ipCurrencyId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    ip_currency_id: Option<super::ids::CurrencyId>,
    #[serde(
        rename = "commission",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    commission: Option<crate::Decimal>,
    #[serde(
        rename = "commissionCurrencyId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    commission_currency_id: Option<super::ids::CurrencyId>,
    #[serde(
        rename = "orderRoutingFee",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    order_routing_fee: Option<crate::Decimal>,
    #[serde(
        rename = "orderRoutingCurrencyId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    order_routing_currency_id: Option<super::ids::CurrencyId>,
    #[serde(
        rename = "commissionNotionalValueBPS",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    commission_notional_value_bps: Option<crate::Decimal>,
    #[serde(
        rename = "exchangeFeeNotionalValueBPS",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    exchange_fee_notional_value_bps: Option<crate::Decimal>,
}

impl EstimatedFillFee {
    /// Returns wire field `clearingFee`.
    #[must_use]
    pub fn clearing_fee(&self) -> Option<&crate::Decimal> {
        self.clearing_fee.as_ref()
    }

    /// Returns wire field `clearingCurrencyId`.
    #[must_use]
    pub fn clearing_currency_id(&self) -> Option<&super::ids::CurrencyId> {
        self.clearing_currency_id.as_ref()
    }

    /// Returns wire field `exchangeFee`.
    #[must_use]
    pub fn exchange_fee(&self) -> Option<&crate::Decimal> {
        self.exchange_fee.as_ref()
    }

    /// Returns wire field `exchangeCurrencyId`.
    #[must_use]
    pub fn exchange_currency_id(&self) -> Option<&super::ids::CurrencyId> {
        self.exchange_currency_id.as_ref()
    }

    /// Returns wire field `nfaFee`.
    #[must_use]
    pub fn nfa_fee(&self) -> Option<&crate::Decimal> {
        self.nfa_fee.as_ref()
    }

    /// Returns wire field `nfaCurrencyId`.
    #[must_use]
    pub fn nfa_currency_id(&self) -> Option<&super::ids::CurrencyId> {
        self.nfa_currency_id.as_ref()
    }

    /// Returns wire field `brokerageFee`.
    #[must_use]
    pub fn brokerage_fee(&self) -> Option<&crate::Decimal> {
        self.brokerage_fee.as_ref()
    }

    /// Returns wire field `brokerageCurrencyId`.
    #[must_use]
    pub fn brokerage_currency_id(&self) -> Option<&super::ids::CurrencyId> {
        self.brokerage_currency_id.as_ref()
    }

    /// Returns wire field `ipFee`.
    #[must_use]
    pub fn ip_fee(&self) -> Option<&crate::Decimal> {
        self.ip_fee.as_ref()
    }

    /// Returns wire field `ipCurrencyId`.
    #[must_use]
    pub fn ip_currency_id(&self) -> Option<&super::ids::CurrencyId> {
        self.ip_currency_id.as_ref()
    }

    /// Returns wire field `commission`.
    #[must_use]
    pub fn commission(&self) -> Option<&crate::Decimal> {
        self.commission.as_ref()
    }

    /// Returns wire field `commissionCurrencyId`.
    #[must_use]
    pub fn commission_currency_id(&self) -> Option<&super::ids::CurrencyId> {
        self.commission_currency_id.as_ref()
    }

    /// Returns wire field `orderRoutingFee`.
    #[must_use]
    pub fn order_routing_fee(&self) -> Option<&crate::Decimal> {
        self.order_routing_fee.as_ref()
    }

    /// Returns wire field `orderRoutingCurrencyId`.
    #[must_use]
    pub fn order_routing_currency_id(&self) -> Option<&super::ids::CurrencyId> {
        self.order_routing_currency_id.as_ref()
    }

    /// Returns wire field `commissionNotionalValueBPS`.
    #[must_use]
    pub fn commission_notional_value_bps(&self) -> Option<&crate::Decimal> {
        self.commission_notional_value_bps.as_ref()
    }

    /// Returns wire field `exchangeFeeNotionalValueBPS`.
    #[must_use]
    pub fn exchange_fee_notional_value_bps(&self) -> Option<&crate::Decimal> {
        self.exchange_fee_notional_value_bps.as_ref()
    }

    /// Starts a builder for [`EstimatedFillFee`].
    pub fn builder() -> EstimatedFillFeeBuilder {
        EstimatedFillFeeBuilder::default()
    }
}

/// Builder for [`EstimatedFillFee`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct EstimatedFillFeeBuilder {
    clearing_fee: Option<crate::Decimal>,
    clearing_currency_id: Option<super::ids::CurrencyId>,
    exchange_fee: Option<crate::Decimal>,
    exchange_currency_id: Option<super::ids::CurrencyId>,
    nfa_fee: Option<crate::Decimal>,
    nfa_currency_id: Option<super::ids::CurrencyId>,
    brokerage_fee: Option<crate::Decimal>,
    brokerage_currency_id: Option<super::ids::CurrencyId>,
    ip_fee: Option<crate::Decimal>,
    ip_currency_id: Option<super::ids::CurrencyId>,
    commission: Option<crate::Decimal>,
    commission_currency_id: Option<super::ids::CurrencyId>,
    order_routing_fee: Option<crate::Decimal>,
    order_routing_currency_id: Option<super::ids::CurrencyId>,
    commission_notional_value_bps: Option<crate::Decimal>,
    exchange_fee_notional_value_bps: Option<crate::Decimal>,
}

impl EstimatedFillFeeBuilder {
    /// Sets wire field `clearingFee`.
    pub fn clearing_fee(mut self, value: crate::Decimal) -> Self {
        self.clearing_fee = Some(value);
        self
    }

    /// Sets wire field `clearingCurrencyId`.
    pub fn clearing_currency_id(mut self, value: super::ids::CurrencyId) -> Self {
        self.clearing_currency_id = Some(value);
        self
    }

    /// Sets wire field `exchangeFee`.
    pub fn exchange_fee(mut self, value: crate::Decimal) -> Self {
        self.exchange_fee = Some(value);
        self
    }

    /// Sets wire field `exchangeCurrencyId`.
    pub fn exchange_currency_id(mut self, value: super::ids::CurrencyId) -> Self {
        self.exchange_currency_id = Some(value);
        self
    }

    /// Sets wire field `nfaFee`.
    pub fn nfa_fee(mut self, value: crate::Decimal) -> Self {
        self.nfa_fee = Some(value);
        self
    }

    /// Sets wire field `nfaCurrencyId`.
    pub fn nfa_currency_id(mut self, value: super::ids::CurrencyId) -> Self {
        self.nfa_currency_id = Some(value);
        self
    }

    /// Sets wire field `brokerageFee`.
    pub fn brokerage_fee(mut self, value: crate::Decimal) -> Self {
        self.brokerage_fee = Some(value);
        self
    }

    /// Sets wire field `brokerageCurrencyId`.
    pub fn brokerage_currency_id(mut self, value: super::ids::CurrencyId) -> Self {
        self.brokerage_currency_id = Some(value);
        self
    }

    /// Sets wire field `ipFee`.
    pub fn ip_fee(mut self, value: crate::Decimal) -> Self {
        self.ip_fee = Some(value);
        self
    }

    /// Sets wire field `ipCurrencyId`.
    pub fn ip_currency_id(mut self, value: super::ids::CurrencyId) -> Self {
        self.ip_currency_id = Some(value);
        self
    }

    /// Sets wire field `commission`.
    pub fn commission(mut self, value: crate::Decimal) -> Self {
        self.commission = Some(value);
        self
    }

    /// Sets wire field `commissionCurrencyId`.
    pub fn commission_currency_id(mut self, value: super::ids::CurrencyId) -> Self {
        self.commission_currency_id = Some(value);
        self
    }

    /// Sets wire field `orderRoutingFee`.
    pub fn order_routing_fee(mut self, value: crate::Decimal) -> Self {
        self.order_routing_fee = Some(value);
        self
    }

    /// Sets wire field `orderRoutingCurrencyId`.
    pub fn order_routing_currency_id(mut self, value: super::ids::CurrencyId) -> Self {
        self.order_routing_currency_id = Some(value);
        self
    }

    /// Sets wire field `commissionNotionalValueBPS`.
    pub fn commission_notional_value_bps(mut self, value: crate::Decimal) -> Self {
        self.commission_notional_value_bps = Some(value);
        self
    }

    /// Sets wire field `exchangeFeeNotionalValueBPS`.
    pub fn exchange_fee_notional_value_bps(mut self, value: crate::Decimal) -> Self {
        self.exchange_fee_notional_value_bps = Some(value);
        self
    }

    /// Validates required fields and builds [`EstimatedFillFee`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<EstimatedFillFee, crate::api::current::BuildError> {
        Ok(EstimatedFillFee {
            clearing_fee: self.clearing_fee,
            clearing_currency_id: self.clearing_currency_id,
            exchange_fee: self.exchange_fee,
            exchange_currency_id: self.exchange_currency_id,
            nfa_fee: self.nfa_fee,
            nfa_currency_id: self.nfa_currency_id,
            brokerage_fee: self.brokerage_fee,
            brokerage_currency_id: self.brokerage_currency_id,
            ip_fee: self.ip_fee,
            ip_currency_id: self.ip_currency_id,
            commission: self.commission,
            commission_currency_id: self.commission_currency_id,
            order_routing_fee: self.order_routing_fee,
            order_routing_currency_id: self.order_routing_currency_id,
            commission_notional_value_bps: self.commission_notional_value_bps,
            exchange_fee_notional_value_bps: self.exchange_fee_notional_value_bps,
        })
    }
}

/// Current wire model `ExtraPreTradeRisk`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ExtraPreTradeRisk {
    #[serde(
        rename = "maxExposedTotal",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    max_exposed_total: Option<i64>,
    #[serde(
        rename = "maxTradedVolumeTotal",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    max_traded_volume_total: Option<i64>,
    #[serde(rename = "products")]
    products: ExtraPreTradeRiskProducts,
    #[serde(rename = "contracts")]
    contracts: ExtraPreTradeRiskContracts,
}

impl ExtraPreTradeRisk {
    /// Returns wire field `maxExposedTotal`.
    #[must_use]
    pub fn max_exposed_total(&self) -> Option<&i64> {
        self.max_exposed_total.as_ref()
    }

    /// Returns wire field `maxTradedVolumeTotal`.
    #[must_use]
    pub fn max_traded_volume_total(&self) -> Option<&i64> {
        self.max_traded_volume_total.as_ref()
    }

    /// Returns wire field `products`.
    #[must_use]
    pub fn products(&self) -> &ExtraPreTradeRiskProducts {
        &self.products
    }

    /// Returns wire field `contracts`.
    #[must_use]
    pub fn contracts(&self) -> &ExtraPreTradeRiskContracts {
        &self.contracts
    }

    /// Starts a builder for [`ExtraPreTradeRisk`].
    pub fn builder() -> ExtraPreTradeRiskBuilder {
        ExtraPreTradeRiskBuilder::default()
    }
}

/// Builder for [`ExtraPreTradeRisk`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ExtraPreTradeRiskBuilder {
    max_exposed_total: Option<i64>,
    max_traded_volume_total: Option<i64>,
    products: Option<ExtraPreTradeRiskProducts>,
    contracts: Option<ExtraPreTradeRiskContracts>,
}

impl ExtraPreTradeRiskBuilder {
    /// Sets wire field `maxExposedTotal`.
    pub fn max_exposed_total(mut self, value: i64) -> Self {
        self.max_exposed_total = Some(value);
        self
    }

    /// Sets wire field `maxTradedVolumeTotal`.
    pub fn max_traded_volume_total(mut self, value: i64) -> Self {
        self.max_traded_volume_total = Some(value);
        self
    }

    /// Sets wire field `products`.
    pub fn products(mut self, value: ExtraPreTradeRiskProducts) -> Self {
        self.products = Some(value);
        self
    }

    /// Sets wire field `contracts`.
    pub fn contracts(mut self, value: ExtraPreTradeRiskContracts) -> Self {
        self.contracts = Some(value);
        self
    }

    /// Validates required fields and builds [`ExtraPreTradeRisk`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ExtraPreTradeRisk, crate::api::current::BuildError> {
        let products = self
            .products
            .ok_or(crate::api::current::BuildError::missing("products"))?;
        let contracts = self
            .contracts
            .ok_or(crate::api::current::BuildError::missing("contracts"))?;
        Ok(ExtraPreTradeRisk {
            max_exposed_total: self.max_exposed_total,
            max_traded_volume_total: self.max_traded_volume_total,
            products,
            contracts,
        })
    }
}

/// Documentation-blocked current wire placeholder `ExtraPreTradeRiskContracts`.
///
/// The pinned contract publishes no member grammar. Deserialization
/// therefore accepts only an empty object and fails closed on provider data.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct ExtraPreTradeRiskContracts {}

impl ExtraPreTradeRiskContracts {
    /// Starts a builder for [`ExtraPreTradeRiskContracts`].
    pub fn builder() -> ExtraPreTradeRiskContractsBuilder {
        ExtraPreTradeRiskContractsBuilder::default()
    }
}

/// Builder for [`ExtraPreTradeRiskContracts`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ExtraPreTradeRiskContractsBuilder {}

impl ExtraPreTradeRiskContractsBuilder {
    /// Validates required fields and builds [`ExtraPreTradeRiskContracts`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ExtraPreTradeRiskContracts, crate::api::current::BuildError> {
        Ok(ExtraPreTradeRiskContracts {})
    }
}

/// Documentation-blocked current wire placeholder `ExtraPreTradeRiskProducts`.
///
/// The pinned contract publishes no member grammar. Deserialization
/// therefore accepts only an empty object and fails closed on provider data.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct ExtraPreTradeRiskProducts {}

impl ExtraPreTradeRiskProducts {
    /// Starts a builder for [`ExtraPreTradeRiskProducts`].
    pub fn builder() -> ExtraPreTradeRiskProductsBuilder {
        ExtraPreTradeRiskProductsBuilder::default()
    }
}

/// Builder for [`ExtraPreTradeRiskProducts`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ExtraPreTradeRiskProductsBuilder {}

impl ExtraPreTradeRiskProductsBuilder {
    /// Validates required fields and builds [`ExtraPreTradeRiskProducts`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ExtraPreTradeRiskProducts, crate::api::current::BuildError> {
        Ok(ExtraPreTradeRiskProducts {})
    }
}

/// Current wire model `InterruptOrderStrategy`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct InterruptOrderStrategy {
    #[serde(rename = "orderStrategyId")]
    order_strategy_id: super::ids::OrderStrategyId,
}

impl InterruptOrderStrategy {
    /// Returns wire field `orderStrategyId`.
    #[must_use]
    pub fn order_strategy_id(&self) -> &super::ids::OrderStrategyId {
        &self.order_strategy_id
    }

    /// Starts a builder for [`InterruptOrderStrategy`].
    pub fn builder() -> InterruptOrderStrategyBuilder {
        InterruptOrderStrategyBuilder::default()
    }
}

/// Builder for [`InterruptOrderStrategy`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct InterruptOrderStrategyBuilder {
    order_strategy_id: Option<super::ids::OrderStrategyId>,
}

impl InterruptOrderStrategyBuilder {
    /// Sets wire field `orderStrategyId`.
    pub fn order_strategy_id(mut self, value: super::ids::OrderStrategyId) -> Self {
        self.order_strategy_id = Some(value);
        self
    }

    /// Validates required fields and builds [`InterruptOrderStrategy`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<InterruptOrderStrategy, crate::api::current::BuildError> {
        let order_strategy_id = self
            .order_strategy_id
            .ok_or(crate::api::current::BuildError::missing("orderStrategyId"))?;
        Ok(InterruptOrderStrategy { order_strategy_id })
    }
}

impl crate::api::current::support::CurrentRequest for InterruptOrderStrategy {
    fn validate_current(&self) -> Result<(), crate::Error> {
        Ok(())
    }
}

/// Current wire model `LiquidatePosition`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct LiquidatePosition {
    #[serde(rename = "accountId")]
    account_id: crate::AccountId,
    #[serde(rename = "contractId")]
    contract_id: crate::ContractId,
    #[serde(rename = "admin")]
    admin: bool,
    #[serde(
        rename = "customTag50",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    custom_tag50: Option<String>,
    #[serde(
        rename = "isAutomated",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    is_automated: Option<bool>,
}

impl LiquidatePosition {
    /// Returns wire field `accountId`.
    #[must_use]
    pub fn account_id(&self) -> &crate::AccountId {
        &self.account_id
    }

    /// Returns wire field `contractId`.
    #[must_use]
    pub fn contract_id(&self) -> &crate::ContractId {
        &self.contract_id
    }

    /// Returns wire field `admin`.
    #[must_use]
    pub fn admin(&self) -> &bool {
        &self.admin
    }

    /// Returns wire field `customTag50`.
    #[must_use]
    pub fn custom_tag50(&self) -> Option<&str> {
        self.custom_tag50.as_deref()
    }

    /// Returns wire field `isAutomated`.
    #[must_use]
    pub fn is_automated(&self) -> Option<&bool> {
        self.is_automated.as_ref()
    }

    /// Starts a builder for [`LiquidatePosition`].
    pub fn builder() -> LiquidatePositionBuilder {
        LiquidatePositionBuilder::default()
    }
}

/// Builder for [`LiquidatePosition`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct LiquidatePositionBuilder {
    account_id: Option<crate::AccountId>,
    contract_id: Option<crate::ContractId>,
    admin: Option<bool>,
    custom_tag50: Option<String>,
    is_automated: Option<bool>,
}

impl LiquidatePositionBuilder {
    /// Sets wire field `accountId`.
    pub fn account_id(mut self, value: crate::AccountId) -> Self {
        self.account_id = Some(value);
        self
    }

    /// Sets wire field `contractId`.
    pub fn contract_id(mut self, value: crate::ContractId) -> Self {
        self.contract_id = Some(value);
        self
    }

    /// Sets wire field `admin`.
    pub fn admin(mut self, value: bool) -> Self {
        self.admin = Some(value);
        self
    }

    /// Sets wire field `customTag50`.
    pub fn custom_tag50(mut self, value: impl Into<String>) -> Self {
        self.custom_tag50 = Some(value.into());
        self
    }

    /// Sets wire field `isAutomated`.
    pub fn is_automated(mut self, value: bool) -> Self {
        self.is_automated = Some(value);
        self
    }

    /// Validates required fields and builds [`LiquidatePosition`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<LiquidatePosition, crate::api::current::BuildError> {
        let account_id = self
            .account_id
            .ok_or(crate::api::current::BuildError::missing("accountId"))?;
        let contract_id = self
            .contract_id
            .ok_or(crate::api::current::BuildError::missing("contractId"))?;
        let admin = self
            .admin
            .ok_or(crate::api::current::BuildError::missing("admin"))?;
        Ok(LiquidatePosition {
            account_id,
            contract_id,
            admin,
            custom_tag50: self.custom_tag50,
            is_automated: self.is_automated,
        })
    }
}

impl crate::api::current::support::CurrentRequest for LiquidatePosition {
    fn validate_current(&self) -> Result<(), crate::Error> {
        Ok(())
    }
}

/// Current wire model `LiquidatePositions`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct LiquidatePositions {
    #[serde(rename = "positions")]
    positions: Vec<i64>,
    #[serde(rename = "admin")]
    admin: bool,
    #[serde(
        rename = "customTag50",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    custom_tag50: Option<String>,
    #[serde(
        rename = "isAutomated",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    is_automated: Option<bool>,
}

impl LiquidatePositions {
    /// Returns wire field `positions`.
    #[must_use]
    pub fn positions(&self) -> &[i64] {
        &self.positions
    }

    /// Returns wire field `admin`.
    #[must_use]
    pub fn admin(&self) -> &bool {
        &self.admin
    }

    /// Returns wire field `customTag50`.
    #[must_use]
    pub fn custom_tag50(&self) -> Option<&str> {
        self.custom_tag50.as_deref()
    }

    /// Returns wire field `isAutomated`.
    #[must_use]
    pub fn is_automated(&self) -> Option<&bool> {
        self.is_automated.as_ref()
    }

    /// Starts a builder for [`LiquidatePositions`].
    pub fn builder() -> LiquidatePositionsBuilder {
        LiquidatePositionsBuilder::default()
    }
}

/// Builder for [`LiquidatePositions`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct LiquidatePositionsBuilder {
    positions: Option<Vec<i64>>,
    admin: Option<bool>,
    custom_tag50: Option<String>,
    is_automated: Option<bool>,
}

impl LiquidatePositionsBuilder {
    /// Sets wire field `positions`.
    pub fn positions(mut self, value: Vec<i64>) -> Self {
        self.positions = Some(value);
        self
    }

    /// Sets wire field `admin`.
    pub fn admin(mut self, value: bool) -> Self {
        self.admin = Some(value);
        self
    }

    /// Sets wire field `customTag50`.
    pub fn custom_tag50(mut self, value: impl Into<String>) -> Self {
        self.custom_tag50 = Some(value.into());
        self
    }

    /// Sets wire field `isAutomated`.
    pub fn is_automated(mut self, value: bool) -> Self {
        self.is_automated = Some(value);
        self
    }

    /// Validates required fields and builds [`LiquidatePositions`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<LiquidatePositions, crate::api::current::BuildError> {
        let positions = self
            .positions
            .ok_or(crate::api::current::BuildError::missing("positions"))?;
        if positions.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "positions",
                "must not be empty",
            ));
        }
        let admin = self
            .admin
            .ok_or(crate::api::current::BuildError::missing("admin"))?;
        Ok(LiquidatePositions {
            positions,
            admin,
            custom_tag50: self.custom_tag50,
            is_automated: self.is_automated,
        })
    }
}

impl crate::api::current::support::CurrentRequest for LiquidatePositions {
    fn validate_current(&self) -> Result<(), crate::Error> {
        if self.positions.is_empty() {
            return Err(crate::Error::InvalidRequest {
                field: "positions",
                reason: "must not be empty",
            });
        }
        Ok(())
    }
}

/// Current wire model `ModifyOrder`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ModifyOrder {
    #[serde(rename = "orderId")]
    order_id: crate::OrderId,
    #[serde(rename = "clOrdId", default, skip_serializing_if = "Option::is_none")]
    cl_ord_id: Option<crate::ClientOrderId>,
    #[serde(rename = "orderQty")]
    order_qty: i64,
    #[serde(rename = "orderType")]
    order_type: ModifyOrderOrderType,
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "crate::decimal::option")]
    price: Option<crate::Decimal>,
    #[serde(rename = "stopPrice", default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "crate::decimal::option")]
    stop_price: Option<crate::Decimal>,
    #[serde(
        rename = "limitIfTouchedPrice",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    limit_if_touched_price: Option<crate::Decimal>,
    #[serde(rename = "maxShow", default, skip_serializing_if = "Option::is_none")]
    max_show: Option<i64>,
    #[serde(
        rename = "pegDifference",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    peg_difference: Option<crate::Decimal>,
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    time_in_force: Option<ModifyOrderTimeInForce>,
    #[serde(
        rename = "expireTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    expire_time: Option<jiff::Timestamp>,
    #[serde(rename = "text", default, skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(
        rename = "activationTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    activation_time: Option<jiff::Timestamp>,
    #[serde(
        rename = "customTag50",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    custom_tag50: Option<String>,
    #[serde(
        rename = "isAutomated",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    is_automated: Option<bool>,
}

impl ModifyOrder {
    /// Returns wire field `orderId`.
    #[must_use]
    pub fn order_id(&self) -> &crate::OrderId {
        &self.order_id
    }

    /// Returns wire field `clOrdId`.
    #[must_use]
    pub fn cl_ord_id(&self) -> Option<&crate::ClientOrderId> {
        self.cl_ord_id.as_ref()
    }

    /// Returns wire field `orderQty`.
    #[must_use]
    pub fn order_qty(&self) -> &i64 {
        &self.order_qty
    }

    /// Returns wire field `orderType`.
    #[must_use]
    pub fn order_type(&self) -> &ModifyOrderOrderType {
        &self.order_type
    }

    /// Returns wire field `price`.
    #[must_use]
    pub fn price(&self) -> Option<&crate::Decimal> {
        self.price.as_ref()
    }

    /// Returns wire field `stopPrice`.
    #[must_use]
    pub fn stop_price(&self) -> Option<&crate::Decimal> {
        self.stop_price.as_ref()
    }

    /// Returns wire field `limitIfTouchedPrice`.
    #[must_use]
    pub fn limit_if_touched_price(&self) -> Option<&crate::Decimal> {
        self.limit_if_touched_price.as_ref()
    }

    /// Returns wire field `maxShow`.
    #[must_use]
    pub fn max_show(&self) -> Option<&i64> {
        self.max_show.as_ref()
    }

    /// Returns wire field `pegDifference`.
    #[must_use]
    pub fn peg_difference(&self) -> Option<&crate::Decimal> {
        self.peg_difference.as_ref()
    }

    /// Returns wire field `timeInForce`.
    #[must_use]
    pub fn time_in_force(&self) -> Option<&ModifyOrderTimeInForce> {
        self.time_in_force.as_ref()
    }

    /// Returns wire field `expireTime`.
    #[must_use]
    pub fn expire_time(&self) -> Option<&jiff::Timestamp> {
        self.expire_time.as_ref()
    }

    /// Returns wire field `text`.
    #[must_use]
    pub fn text(&self) -> Option<&str> {
        self.text.as_deref()
    }

    /// Returns wire field `activationTime`.
    #[must_use]
    pub fn activation_time(&self) -> Option<&jiff::Timestamp> {
        self.activation_time.as_ref()
    }

    /// Returns wire field `customTag50`.
    #[must_use]
    pub fn custom_tag50(&self) -> Option<&str> {
        self.custom_tag50.as_deref()
    }

    /// Returns wire field `isAutomated`.
    #[must_use]
    pub fn is_automated(&self) -> Option<&bool> {
        self.is_automated.as_ref()
    }

    /// Starts a builder for [`ModifyOrder`].
    pub fn builder() -> ModifyOrderBuilder {
        ModifyOrderBuilder::default()
    }
}

/// Builder for [`ModifyOrder`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ModifyOrderBuilder {
    order_id: Option<crate::OrderId>,
    cl_ord_id: Option<crate::ClientOrderId>,
    order_qty: Option<i64>,
    order_type: Option<ModifyOrderOrderType>,
    price: Option<crate::Decimal>,
    stop_price: Option<crate::Decimal>,
    limit_if_touched_price: Option<crate::Decimal>,
    max_show: Option<i64>,
    peg_difference: Option<crate::Decimal>,
    time_in_force: Option<ModifyOrderTimeInForce>,
    expire_time: Option<jiff::Timestamp>,
    text: Option<String>,
    activation_time: Option<jiff::Timestamp>,
    custom_tag50: Option<String>,
    is_automated: Option<bool>,
}

impl ModifyOrderBuilder {
    /// Sets wire field `orderId`.
    pub fn order_id(mut self, value: crate::OrderId) -> Self {
        self.order_id = Some(value);
        self
    }

    /// Sets wire field `clOrdId`.
    pub fn cl_ord_id(mut self, value: crate::ClientOrderId) -> Self {
        self.cl_ord_id = Some(value);
        self
    }

    /// Sets wire field `orderQty`.
    pub fn order_qty(mut self, value: i64) -> Self {
        self.order_qty = Some(value);
        self
    }

    /// Sets wire field `orderType`.
    pub fn order_type(mut self, value: ModifyOrderOrderType) -> Self {
        self.order_type = Some(value);
        self
    }

    /// Sets wire field `price`.
    pub fn price(mut self, value: crate::Decimal) -> Self {
        self.price = Some(value);
        self
    }

    /// Sets wire field `stopPrice`.
    pub fn stop_price(mut self, value: crate::Decimal) -> Self {
        self.stop_price = Some(value);
        self
    }

    /// Sets wire field `limitIfTouchedPrice`.
    pub fn limit_if_touched_price(mut self, value: crate::Decimal) -> Self {
        self.limit_if_touched_price = Some(value);
        self
    }

    /// Sets wire field `maxShow`.
    pub fn max_show(mut self, value: i64) -> Self {
        self.max_show = Some(value);
        self
    }

    /// Sets wire field `pegDifference`.
    pub fn peg_difference(mut self, value: crate::Decimal) -> Self {
        self.peg_difference = Some(value);
        self
    }

    /// Sets wire field `timeInForce`.
    pub fn time_in_force(mut self, value: ModifyOrderTimeInForce) -> Self {
        self.time_in_force = Some(value);
        self
    }

    /// Sets wire field `expireTime`.
    pub fn expire_time(mut self, value: jiff::Timestamp) -> Self {
        self.expire_time = Some(value);
        self
    }

    /// Sets wire field `text`.
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Sets wire field `activationTime`.
    pub fn activation_time(mut self, value: jiff::Timestamp) -> Self {
        self.activation_time = Some(value);
        self
    }

    /// Sets wire field `customTag50`.
    pub fn custom_tag50(mut self, value: impl Into<String>) -> Self {
        self.custom_tag50 = Some(value.into());
        self
    }

    /// Sets wire field `isAutomated`.
    pub fn is_automated(mut self, value: bool) -> Self {
        self.is_automated = Some(value);
        self
    }

    /// Validates required fields and builds [`ModifyOrder`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ModifyOrder, crate::api::current::BuildError> {
        let order_id = self
            .order_id
            .ok_or(crate::api::current::BuildError::missing("orderId"))?;
        let order_qty = self
            .order_qty
            .ok_or(crate::api::current::BuildError::missing("orderQty"))?;
        let order_type = self
            .order_type
            .ok_or(crate::api::current::BuildError::missing("orderType"))?;
        Ok(ModifyOrder {
            order_id,
            cl_ord_id: self.cl_ord_id,
            order_qty,
            order_type,
            price: self.price,
            stop_price: self.stop_price,
            limit_if_touched_price: self.limit_if_touched_price,
            max_show: self.max_show,
            peg_difference: self.peg_difference,
            time_in_force: self.time_in_force,
            expire_time: self.expire_time,
            text: self.text,
            activation_time: self.activation_time,
            custom_tag50: self.custom_tag50,
            is_automated: self.is_automated,
        })
    }
}

impl crate::api::current::support::CurrentRequest for ModifyOrder {
    fn validate_current(&self) -> Result<(), crate::Error> {
        Ok(())
    }
}

/// Current provider values for `ModifyOrderOrderType`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum ModifyOrderOrderType {
    /// Provider value `Limit`.
    Limit,
    /// Provider value `LimitIfTouched`.
    LimitIfTouched,
    /// Provider value `MIT`.
    Mit,
    /// Provider value `Market`.
    Market,
    /// Provider value `QTS`.
    Qts,
    /// Provider value `Stop`.
    Stop,
    /// Provider value `StopLimit`.
    StopLimit,
    /// Provider value `TrailingStop`.
    TrailingStop,
    /// Provider value `TrailingStopLimit`.
    TrailingStopLimit,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl ModifyOrderOrderType {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Limit => "Limit",
            Self::LimitIfTouched => "LimitIfTouched",
            Self::Mit => "MIT",
            Self::Market => "Market",
            Self::Qts => "QTS",
            Self::Stop => "Stop",
            Self::StopLimit => "StopLimit",
            Self::TrailingStop => "TrailingStop",
            Self::TrailingStopLimit => "TrailingStopLimit",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for ModifyOrderOrderType {
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

impl<'de> serde::Deserialize<'de> for ModifyOrderOrderType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "Limit" => Self::Limit,
            "LimitIfTouched" => Self::LimitIfTouched,
            "MIT" => Self::Mit,
            "Market" => Self::Market,
            "QTS" => Self::Qts,
            "Stop" => Self::Stop,
            "StopLimit" => Self::StopLimit,
            "TrailingStop" => Self::TrailingStop,
            "TrailingStopLimit" => Self::TrailingStopLimit,
            _ => Self::Unknown(value),
        })
    }
}

/// Current wire model `ModifyOrderStrategy`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ModifyOrderStrategy {
    #[serde(rename = "orderStrategyId")]
    order_strategy_id: super::ids::OrderStrategyId,
    #[serde(rename = "command")]
    command: String,
    #[serde(
        rename = "customTag50",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    custom_tag50: Option<String>,
}

impl ModifyOrderStrategy {
    /// Returns wire field `orderStrategyId`.
    #[must_use]
    pub fn order_strategy_id(&self) -> &super::ids::OrderStrategyId {
        &self.order_strategy_id
    }

    /// Returns wire field `command`.
    #[must_use]
    pub fn command(&self) -> &str {
        &self.command
    }

    /// Returns wire field `customTag50`.
    #[must_use]
    pub fn custom_tag50(&self) -> Option<&str> {
        self.custom_tag50.as_deref()
    }

    /// Starts a builder for [`ModifyOrderStrategy`].
    pub fn builder() -> ModifyOrderStrategyBuilder {
        ModifyOrderStrategyBuilder::default()
    }
}

/// Builder for [`ModifyOrderStrategy`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ModifyOrderStrategyBuilder {
    order_strategy_id: Option<super::ids::OrderStrategyId>,
    command: Option<String>,
    custom_tag50: Option<String>,
}

impl ModifyOrderStrategyBuilder {
    /// Sets wire field `orderStrategyId`.
    pub fn order_strategy_id(mut self, value: super::ids::OrderStrategyId) -> Self {
        self.order_strategy_id = Some(value);
        self
    }

    /// Sets wire field `command`.
    pub fn command(mut self, value: impl Into<String>) -> Self {
        self.command = Some(value.into());
        self
    }

    /// Sets wire field `customTag50`.
    pub fn custom_tag50(mut self, value: impl Into<String>) -> Self {
        self.custom_tag50 = Some(value.into());
        self
    }

    /// Validates required fields and builds [`ModifyOrderStrategy`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ModifyOrderStrategy, crate::api::current::BuildError> {
        let order_strategy_id = self
            .order_strategy_id
            .ok_or(crate::api::current::BuildError::missing("orderStrategyId"))?;
        let command = self
            .command
            .ok_or(crate::api::current::BuildError::missing("command"))?;
        if command.is_empty() || command.trim() != command {
            return Err(crate::api::current::BuildError::invalid(
                "command",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        Ok(ModifyOrderStrategy {
            order_strategy_id,
            command,
            custom_tag50: self.custom_tag50,
        })
    }
}

impl crate::api::current::support::CurrentRequest for ModifyOrderStrategy {
    fn validate_current(&self) -> Result<(), crate::Error> {
        if self.command.is_empty() || self.command.trim() != self.command {
            return Err(crate::Error::InvalidRequest {
                field: "command",
                reason: "must be non-empty and have no surrounding whitespace",
            });
        }
        Ok(())
    }
}

/// Current provider values for `ModifyOrderTimeInForce`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum ModifyOrderTimeInForce {
    /// Provider value `Day`.
    Day,
    /// Provider value `FOK`.
    Fok,
    /// Provider value `GTC`.
    Gtc,
    /// Provider value `GTD`.
    Gtd,
    /// Provider value `IOC`.
    Ioc,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl ModifyOrderTimeInForce {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Day => "Day",
            Self::Fok => "FOK",
            Self::Gtc => "GTC",
            Self::Gtd => "GTD",
            Self::Ioc => "IOC",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for ModifyOrderTimeInForce {
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

impl<'de> serde::Deserialize<'de> for ModifyOrderTimeInForce {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "Day" => Self::Day,
            "FOK" => Self::Fok,
            "GTC" => Self::Gtc,
            "GTD" => Self::Gtd,
            "IOC" => Self::Ioc,
            _ => Self::Unknown(value),
        })
    }
}

/// Current wire model `OrderStrategyStatusResponse`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct OrderStrategyStatusResponse {
    #[serde(rename = "errorText", default, skip_serializing_if = "Option::is_none")]
    error_text: Option<String>,
    #[serde(
        rename = "failureReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    failure_reason: Option<OrderStrategyStatusResponseFailureReason>,
    #[serde(
        rename = "orderStrategy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    order_strategy: Option<super::users::OrderStrategy>,
}

impl OrderStrategyStatusResponse {
    /// Returns wire field `errorText`.
    #[must_use]
    pub fn error_text(&self) -> Option<&str> {
        self.error_text.as_deref()
    }

    /// Returns wire field `failureReason`.
    #[must_use]
    pub fn failure_reason(&self) -> Option<&OrderStrategyStatusResponseFailureReason> {
        self.failure_reason.as_ref()
    }

    /// Returns wire field `orderStrategy`.
    #[must_use]
    pub fn order_strategy(&self) -> Option<&super::users::OrderStrategy> {
        self.order_strategy.as_ref()
    }

    /// Starts a builder for [`OrderStrategyStatusResponse`].
    pub fn builder() -> OrderStrategyStatusResponseBuilder {
        OrderStrategyStatusResponseBuilder::default()
    }
}

/// Builder for [`OrderStrategyStatusResponse`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct OrderStrategyStatusResponseBuilder {
    error_text: Option<String>,
    failure_reason: Option<OrderStrategyStatusResponseFailureReason>,
    order_strategy: Option<super::users::OrderStrategy>,
}

impl OrderStrategyStatusResponseBuilder {
    /// Sets wire field `errorText`.
    pub fn error_text(mut self, value: impl Into<String>) -> Self {
        self.error_text = Some(value.into());
        self
    }

    /// Sets wire field `failureReason`.
    pub fn failure_reason(mut self, value: OrderStrategyStatusResponseFailureReason) -> Self {
        self.failure_reason = Some(value);
        self
    }

    /// Sets wire field `orderStrategy`.
    pub fn order_strategy(mut self, value: super::users::OrderStrategy) -> Self {
        self.order_strategy = Some(value);
        self
    }

    /// Validates required fields and builds [`OrderStrategyStatusResponse`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<OrderStrategyStatusResponse, crate::api::current::BuildError> {
        Ok(OrderStrategyStatusResponse {
            error_text: self.error_text,
            failure_reason: self.failure_reason,
            order_strategy: self.order_strategy,
        })
    }
}

/// Current provider values for `OrderStrategyStatusResponseFailureReason`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum OrderStrategyStatusResponseFailureReason {
    /// Provider value `AccountClosed`.
    AccountClosed,
    /// Provider value `AdvancedTrailingStopUnsupported`.
    AdvancedTrailingStopUnsupported,
    /// Provider value `AnotherCommandPending`.
    AnotherCommandPending,
    /// Provider value `BackMonthProhibited`.
    BackMonthProhibited,
    /// Provider value `ExecutionProviderNotConfigured`.
    ExecutionProviderNotConfigured,
    /// Provider value `ExecutionProviderUnavailable`.
    ExecutionProviderUnavailable,
    /// Provider value `InvalidContract`.
    InvalidContract,
    /// Provider value `InvalidPrice`.
    InvalidPrice,
    /// Provider value `KeyInformationDocumentRequired`.
    KeyInformationDocumentRequired,
    /// Provider value `LiquidationOnly`.
    LiquidationOnly,
    /// Provider value `LiquidationOnlyBeforeExpiration`.
    LiquidationOnlyBeforeExpiration,
    /// Provider value `MaxOrderQtyIsNotSpecified`.
    MaxOrderQtyIsNotSpecified,
    /// Provider value `MaxOrderQtyLimitReached`.
    MaxOrderQtyLimitReached,
    /// Provider value `MaxPosLimitMisconfigured`.
    MaxPosLimitMisconfigured,
    /// Provider value `MaxPosLimitReached`.
    MaxPosLimitReached,
    /// Provider value `MaxTotalPosLimitReached`.
    MaxTotalPosLimitReached,
    /// Provider value `MultipleAccountPlanRequired`.
    MultipleAccountPlanRequired,
    /// Provider value `NoQuote`.
    NoQuote,
    /// Provider value `NotEnoughLiquidity`.
    NotEnoughLiquidity,
    /// Provider value `OtherExecutionRelated`.
    OtherExecutionRelated,
    /// Provider value `ParentRejected`.
    ParentRejected,
    /// Provider value `RiskCheckTimeout`.
    RiskCheckTimeout,
    /// Provider value `SSFRiskDisclosureAcknowledgmentRequired`.
    SsfRiskDisclosureAcknowledgmentRequired,
    /// Provider value `SessionClosed`.
    SessionClosed,
    /// Provider value `Success`.
    Success,
    /// Provider value `TooLate`.
    TooLate,
    /// Provider value `TradingLocked`.
    TradingLocked,
    /// Provider value `TrailingStopNonOrderQtyModify`.
    TrailingStopNonOrderQtyModify,
    /// Provider value `Unauthorized`.
    Unauthorized,
    /// Provider value `UnknownReason`.
    UnknownReason,
    /// Provider value `Unsupported`.
    Unsupported,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl OrderStrategyStatusResponseFailureReason {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::AccountClosed => "AccountClosed",
            Self::AdvancedTrailingStopUnsupported => "AdvancedTrailingStopUnsupported",
            Self::AnotherCommandPending => "AnotherCommandPending",
            Self::BackMonthProhibited => "BackMonthProhibited",
            Self::ExecutionProviderNotConfigured => "ExecutionProviderNotConfigured",
            Self::ExecutionProviderUnavailable => "ExecutionProviderUnavailable",
            Self::InvalidContract => "InvalidContract",
            Self::InvalidPrice => "InvalidPrice",
            Self::KeyInformationDocumentRequired => "KeyInformationDocumentRequired",
            Self::LiquidationOnly => "LiquidationOnly",
            Self::LiquidationOnlyBeforeExpiration => "LiquidationOnlyBeforeExpiration",
            Self::MaxOrderQtyIsNotSpecified => "MaxOrderQtyIsNotSpecified",
            Self::MaxOrderQtyLimitReached => "MaxOrderQtyLimitReached",
            Self::MaxPosLimitMisconfigured => "MaxPosLimitMisconfigured",
            Self::MaxPosLimitReached => "MaxPosLimitReached",
            Self::MaxTotalPosLimitReached => "MaxTotalPosLimitReached",
            Self::MultipleAccountPlanRequired => "MultipleAccountPlanRequired",
            Self::NoQuote => "NoQuote",
            Self::NotEnoughLiquidity => "NotEnoughLiquidity",
            Self::OtherExecutionRelated => "OtherExecutionRelated",
            Self::ParentRejected => "ParentRejected",
            Self::RiskCheckTimeout => "RiskCheckTimeout",
            Self::SsfRiskDisclosureAcknowledgmentRequired => {
                "SSFRiskDisclosureAcknowledgmentRequired"
            }
            Self::SessionClosed => "SessionClosed",
            Self::Success => "Success",
            Self::TooLate => "TooLate",
            Self::TradingLocked => "TradingLocked",
            Self::TrailingStopNonOrderQtyModify => "TrailingStopNonOrderQtyModify",
            Self::Unauthorized => "Unauthorized",
            Self::UnknownReason => "UnknownReason",
            Self::Unsupported => "Unsupported",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for OrderStrategyStatusResponseFailureReason {
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

impl<'de> serde::Deserialize<'de> for OrderStrategyStatusResponseFailureReason {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "AccountClosed" => Self::AccountClosed,
            "AdvancedTrailingStopUnsupported" => Self::AdvancedTrailingStopUnsupported,
            "AnotherCommandPending" => Self::AnotherCommandPending,
            "BackMonthProhibited" => Self::BackMonthProhibited,
            "ExecutionProviderNotConfigured" => Self::ExecutionProviderNotConfigured,
            "ExecutionProviderUnavailable" => Self::ExecutionProviderUnavailable,
            "InvalidContract" => Self::InvalidContract,
            "InvalidPrice" => Self::InvalidPrice,
            "KeyInformationDocumentRequired" => Self::KeyInformationDocumentRequired,
            "LiquidationOnly" => Self::LiquidationOnly,
            "LiquidationOnlyBeforeExpiration" => Self::LiquidationOnlyBeforeExpiration,
            "MaxOrderQtyIsNotSpecified" => Self::MaxOrderQtyIsNotSpecified,
            "MaxOrderQtyLimitReached" => Self::MaxOrderQtyLimitReached,
            "MaxPosLimitMisconfigured" => Self::MaxPosLimitMisconfigured,
            "MaxPosLimitReached" => Self::MaxPosLimitReached,
            "MaxTotalPosLimitReached" => Self::MaxTotalPosLimitReached,
            "MultipleAccountPlanRequired" => Self::MultipleAccountPlanRequired,
            "NoQuote" => Self::NoQuote,
            "NotEnoughLiquidity" => Self::NotEnoughLiquidity,
            "OtherExecutionRelated" => Self::OtherExecutionRelated,
            "ParentRejected" => Self::ParentRejected,
            "RiskCheckTimeout" => Self::RiskCheckTimeout,
            "SSFRiskDisclosureAcknowledgmentRequired" => {
                Self::SsfRiskDisclosureAcknowledgmentRequired
            }
            "SessionClosed" => Self::SessionClosed,
            "Success" => Self::Success,
            "TooLate" => Self::TooLate,
            "TradingLocked" => Self::TradingLocked,
            "TrailingStopNonOrderQtyModify" => Self::TrailingStopNonOrderQtyModify,
            "Unauthorized" => Self::Unauthorized,
            "UnknownReason" => Self::UnknownReason,
            "Unsupported" => Self::Unsupported,
            _ => Self::Unknown(value),
        })
    }
}

/// Current wire model `PlaceOCO`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct PlaceOCO {
    #[serde(
        rename = "accountSpec",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    account_spec: Option<crate::AccountSpec>,
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    account_id: Option<crate::AccountId>,
    #[serde(rename = "clOrdId", default, skip_serializing_if = "Option::is_none")]
    cl_ord_id: Option<crate::ClientOrderId>,
    #[serde(rename = "action")]
    action: PlaceOcoAction,
    #[serde(rename = "symbol")]
    symbol: crate::Symbol,
    #[serde(rename = "orderQty")]
    order_qty: i64,
    #[serde(rename = "orderType")]
    order_type: PlaceOcoOrderType,
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "crate::decimal::option")]
    price: Option<crate::Decimal>,
    #[serde(rename = "stopPrice", default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "crate::decimal::option")]
    stop_price: Option<crate::Decimal>,
    #[serde(
        rename = "limitIfTouchedPrice",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    limit_if_touched_price: Option<crate::Decimal>,
    #[serde(rename = "maxShow", default, skip_serializing_if = "Option::is_none")]
    max_show: Option<i64>,
    #[serde(
        rename = "pegDifference",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    peg_difference: Option<crate::Decimal>,
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    time_in_force: Option<PlaceOcoTimeInForce>,
    #[serde(
        rename = "expireTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    expire_time: Option<jiff::Timestamp>,
    #[serde(rename = "text", default, skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(
        rename = "activationTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    activation_time: Option<jiff::Timestamp>,
    #[serde(
        rename = "customTag50",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    custom_tag50: Option<String>,
    #[serde(
        rename = "isAutomated",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    is_automated: Option<bool>,
    #[serde(rename = "other")]
    other: RestrainedOrderVersion,
}

impl PlaceOCO {
    /// Returns wire field `accountSpec`.
    #[must_use]
    pub fn account_spec(&self) -> Option<&crate::AccountSpec> {
        self.account_spec.as_ref()
    }

    /// Returns wire field `accountId`.
    #[must_use]
    pub fn account_id(&self) -> Option<&crate::AccountId> {
        self.account_id.as_ref()
    }

    /// Returns wire field `clOrdId`.
    #[must_use]
    pub fn cl_ord_id(&self) -> Option<&crate::ClientOrderId> {
        self.cl_ord_id.as_ref()
    }

    /// Returns wire field `action`.
    #[must_use]
    pub fn action(&self) -> &PlaceOcoAction {
        &self.action
    }

    /// Returns wire field `symbol`.
    #[must_use]
    pub fn symbol(&self) -> &crate::Symbol {
        &self.symbol
    }

    /// Returns wire field `orderQty`.
    #[must_use]
    pub fn order_qty(&self) -> &i64 {
        &self.order_qty
    }

    /// Returns wire field `orderType`.
    #[must_use]
    pub fn order_type(&self) -> &PlaceOcoOrderType {
        &self.order_type
    }

    /// Returns wire field `price`.
    #[must_use]
    pub fn price(&self) -> Option<&crate::Decimal> {
        self.price.as_ref()
    }

    /// Returns wire field `stopPrice`.
    #[must_use]
    pub fn stop_price(&self) -> Option<&crate::Decimal> {
        self.stop_price.as_ref()
    }

    /// Returns wire field `limitIfTouchedPrice`.
    #[must_use]
    pub fn limit_if_touched_price(&self) -> Option<&crate::Decimal> {
        self.limit_if_touched_price.as_ref()
    }

    /// Returns wire field `maxShow`.
    #[must_use]
    pub fn max_show(&self) -> Option<&i64> {
        self.max_show.as_ref()
    }

    /// Returns wire field `pegDifference`.
    #[must_use]
    pub fn peg_difference(&self) -> Option<&crate::Decimal> {
        self.peg_difference.as_ref()
    }

    /// Returns wire field `timeInForce`.
    #[must_use]
    pub fn time_in_force(&self) -> Option<&PlaceOcoTimeInForce> {
        self.time_in_force.as_ref()
    }

    /// Returns wire field `expireTime`.
    #[must_use]
    pub fn expire_time(&self) -> Option<&jiff::Timestamp> {
        self.expire_time.as_ref()
    }

    /// Returns wire field `text`.
    #[must_use]
    pub fn text(&self) -> Option<&str> {
        self.text.as_deref()
    }

    /// Returns wire field `activationTime`.
    #[must_use]
    pub fn activation_time(&self) -> Option<&jiff::Timestamp> {
        self.activation_time.as_ref()
    }

    /// Returns wire field `customTag50`.
    #[must_use]
    pub fn custom_tag50(&self) -> Option<&str> {
        self.custom_tag50.as_deref()
    }

    /// Returns wire field `isAutomated`.
    #[must_use]
    pub fn is_automated(&self) -> Option<&bool> {
        self.is_automated.as_ref()
    }

    /// Returns wire field `other`.
    #[must_use]
    pub fn other(&self) -> &RestrainedOrderVersion {
        &self.other
    }

    /// Starts a builder for [`PlaceOCO`].
    pub fn builder() -> PlaceOCOBuilder {
        PlaceOCOBuilder::default()
    }
}

/// Builder for [`PlaceOCO`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct PlaceOCOBuilder {
    account_spec: Option<crate::AccountSpec>,
    account_id: Option<crate::AccountId>,
    cl_ord_id: Option<crate::ClientOrderId>,
    action: Option<PlaceOcoAction>,
    symbol: Option<crate::Symbol>,
    order_qty: Option<i64>,
    order_type: Option<PlaceOcoOrderType>,
    price: Option<crate::Decimal>,
    stop_price: Option<crate::Decimal>,
    limit_if_touched_price: Option<crate::Decimal>,
    max_show: Option<i64>,
    peg_difference: Option<crate::Decimal>,
    time_in_force: Option<PlaceOcoTimeInForce>,
    expire_time: Option<jiff::Timestamp>,
    text: Option<String>,
    activation_time: Option<jiff::Timestamp>,
    custom_tag50: Option<String>,
    is_automated: Option<bool>,
    other: Option<RestrainedOrderVersion>,
}

impl PlaceOCOBuilder {
    /// Sets wire field `accountSpec`.
    pub fn account_spec(mut self, value: crate::AccountSpec) -> Self {
        self.account_spec = Some(value);
        self
    }

    /// Sets wire field `accountId`.
    pub fn account_id(mut self, value: crate::AccountId) -> Self {
        self.account_id = Some(value);
        self
    }

    /// Sets wire field `clOrdId`.
    pub fn cl_ord_id(mut self, value: crate::ClientOrderId) -> Self {
        self.cl_ord_id = Some(value);
        self
    }

    /// Sets wire field `action`.
    pub fn action(mut self, value: PlaceOcoAction) -> Self {
        self.action = Some(value);
        self
    }

    /// Sets wire field `symbol`.
    pub fn symbol(mut self, value: crate::Symbol) -> Self {
        self.symbol = Some(value);
        self
    }

    /// Sets wire field `orderQty`.
    pub fn order_qty(mut self, value: i64) -> Self {
        self.order_qty = Some(value);
        self
    }

    /// Sets wire field `orderType`.
    pub fn order_type(mut self, value: PlaceOcoOrderType) -> Self {
        self.order_type = Some(value);
        self
    }

    /// Sets wire field `price`.
    pub fn price(mut self, value: crate::Decimal) -> Self {
        self.price = Some(value);
        self
    }

    /// Sets wire field `stopPrice`.
    pub fn stop_price(mut self, value: crate::Decimal) -> Self {
        self.stop_price = Some(value);
        self
    }

    /// Sets wire field `limitIfTouchedPrice`.
    pub fn limit_if_touched_price(mut self, value: crate::Decimal) -> Self {
        self.limit_if_touched_price = Some(value);
        self
    }

    /// Sets wire field `maxShow`.
    pub fn max_show(mut self, value: i64) -> Self {
        self.max_show = Some(value);
        self
    }

    /// Sets wire field `pegDifference`.
    pub fn peg_difference(mut self, value: crate::Decimal) -> Self {
        self.peg_difference = Some(value);
        self
    }

    /// Sets wire field `timeInForce`.
    pub fn time_in_force(mut self, value: PlaceOcoTimeInForce) -> Self {
        self.time_in_force = Some(value);
        self
    }

    /// Sets wire field `expireTime`.
    pub fn expire_time(mut self, value: jiff::Timestamp) -> Self {
        self.expire_time = Some(value);
        self
    }

    /// Sets wire field `text`.
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Sets wire field `activationTime`.
    pub fn activation_time(mut self, value: jiff::Timestamp) -> Self {
        self.activation_time = Some(value);
        self
    }

    /// Sets wire field `customTag50`.
    pub fn custom_tag50(mut self, value: impl Into<String>) -> Self {
        self.custom_tag50 = Some(value.into());
        self
    }

    /// Sets wire field `isAutomated`.
    pub fn is_automated(mut self, value: bool) -> Self {
        self.is_automated = Some(value);
        self
    }

    /// Sets wire field `other`.
    pub fn other(mut self, value: RestrainedOrderVersion) -> Self {
        self.other = Some(value);
        self
    }

    /// Validates required fields and builds [`PlaceOCO`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<PlaceOCO, crate::api::current::BuildError> {
        let action = self
            .action
            .ok_or(crate::api::current::BuildError::missing("action"))?;
        let symbol = self
            .symbol
            .ok_or(crate::api::current::BuildError::missing("symbol"))?;
        let order_qty = self
            .order_qty
            .ok_or(crate::api::current::BuildError::missing("orderQty"))?;
        let order_type = self
            .order_type
            .ok_or(crate::api::current::BuildError::missing("orderType"))?;
        let other = self
            .other
            .ok_or(crate::api::current::BuildError::missing("other"))?;
        Ok(PlaceOCO {
            account_spec: self.account_spec,
            account_id: self.account_id,
            cl_ord_id: self.cl_ord_id,
            action,
            symbol,
            order_qty,
            order_type,
            price: self.price,
            stop_price: self.stop_price,
            limit_if_touched_price: self.limit_if_touched_price,
            max_show: self.max_show,
            peg_difference: self.peg_difference,
            time_in_force: self.time_in_force,
            expire_time: self.expire_time,
            text: self.text,
            activation_time: self.activation_time,
            custom_tag50: self.custom_tag50,
            is_automated: self.is_automated,
            other,
        })
    }
}

impl crate::api::current::support::CurrentRequest for PlaceOCO {
    fn validate_current(&self) -> Result<(), crate::Error> {
        Ok(())
    }
}

/// Current wire model `PlaceOSO`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct PlaceOSO {
    #[serde(
        rename = "accountSpec",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    account_spec: Option<crate::AccountSpec>,
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    account_id: Option<crate::AccountId>,
    #[serde(rename = "clOrdId", default, skip_serializing_if = "Option::is_none")]
    cl_ord_id: Option<crate::ClientOrderId>,
    #[serde(rename = "action")]
    action: PlaceOsoAction,
    #[serde(rename = "symbol")]
    symbol: crate::Symbol,
    #[serde(rename = "orderQty")]
    order_qty: i64,
    #[serde(rename = "orderType")]
    order_type: PlaceOsoOrderType,
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "crate::decimal::option")]
    price: Option<crate::Decimal>,
    #[serde(rename = "stopPrice", default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "crate::decimal::option")]
    stop_price: Option<crate::Decimal>,
    #[serde(
        rename = "limitIfTouchedPrice",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    limit_if_touched_price: Option<crate::Decimal>,
    #[serde(rename = "maxShow", default, skip_serializing_if = "Option::is_none")]
    max_show: Option<i64>,
    #[serde(
        rename = "pegDifference",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    peg_difference: Option<crate::Decimal>,
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    time_in_force: Option<PlaceOsoTimeInForce>,
    #[serde(
        rename = "expireTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    expire_time: Option<jiff::Timestamp>,
    #[serde(rename = "text", default, skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(
        rename = "activationTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    activation_time: Option<jiff::Timestamp>,
    #[serde(
        rename = "customTag50",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    custom_tag50: Option<String>,
    #[serde(
        rename = "isAutomated",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    is_automated: Option<bool>,
    #[serde(rename = "bracket1")]
    bracket1: RestrainedOrderVersion,
    #[serde(rename = "bracket2", default, skip_serializing_if = "Option::is_none")]
    bracket2: Option<RestrainedOrderVersion>,
}

impl PlaceOSO {
    /// Returns wire field `accountSpec`.
    #[must_use]
    pub fn account_spec(&self) -> Option<&crate::AccountSpec> {
        self.account_spec.as_ref()
    }

    /// Returns wire field `accountId`.
    #[must_use]
    pub fn account_id(&self) -> Option<&crate::AccountId> {
        self.account_id.as_ref()
    }

    /// Returns wire field `clOrdId`.
    #[must_use]
    pub fn cl_ord_id(&self) -> Option<&crate::ClientOrderId> {
        self.cl_ord_id.as_ref()
    }

    /// Returns wire field `action`.
    #[must_use]
    pub fn action(&self) -> &PlaceOsoAction {
        &self.action
    }

    /// Returns wire field `symbol`.
    #[must_use]
    pub fn symbol(&self) -> &crate::Symbol {
        &self.symbol
    }

    /// Returns wire field `orderQty`.
    #[must_use]
    pub fn order_qty(&self) -> &i64 {
        &self.order_qty
    }

    /// Returns wire field `orderType`.
    #[must_use]
    pub fn order_type(&self) -> &PlaceOsoOrderType {
        &self.order_type
    }

    /// Returns wire field `price`.
    #[must_use]
    pub fn price(&self) -> Option<&crate::Decimal> {
        self.price.as_ref()
    }

    /// Returns wire field `stopPrice`.
    #[must_use]
    pub fn stop_price(&self) -> Option<&crate::Decimal> {
        self.stop_price.as_ref()
    }

    /// Returns wire field `limitIfTouchedPrice`.
    #[must_use]
    pub fn limit_if_touched_price(&self) -> Option<&crate::Decimal> {
        self.limit_if_touched_price.as_ref()
    }

    /// Returns wire field `maxShow`.
    #[must_use]
    pub fn max_show(&self) -> Option<&i64> {
        self.max_show.as_ref()
    }

    /// Returns wire field `pegDifference`.
    #[must_use]
    pub fn peg_difference(&self) -> Option<&crate::Decimal> {
        self.peg_difference.as_ref()
    }

    /// Returns wire field `timeInForce`.
    #[must_use]
    pub fn time_in_force(&self) -> Option<&PlaceOsoTimeInForce> {
        self.time_in_force.as_ref()
    }

    /// Returns wire field `expireTime`.
    #[must_use]
    pub fn expire_time(&self) -> Option<&jiff::Timestamp> {
        self.expire_time.as_ref()
    }

    /// Returns wire field `text`.
    #[must_use]
    pub fn text(&self) -> Option<&str> {
        self.text.as_deref()
    }

    /// Returns wire field `activationTime`.
    #[must_use]
    pub fn activation_time(&self) -> Option<&jiff::Timestamp> {
        self.activation_time.as_ref()
    }

    /// Returns wire field `customTag50`.
    #[must_use]
    pub fn custom_tag50(&self) -> Option<&str> {
        self.custom_tag50.as_deref()
    }

    /// Returns wire field `isAutomated`.
    #[must_use]
    pub fn is_automated(&self) -> Option<&bool> {
        self.is_automated.as_ref()
    }

    /// Returns wire field `bracket1`.
    #[must_use]
    pub fn bracket1(&self) -> &RestrainedOrderVersion {
        &self.bracket1
    }

    /// Returns wire field `bracket2`.
    #[must_use]
    pub fn bracket2(&self) -> Option<&RestrainedOrderVersion> {
        self.bracket2.as_ref()
    }

    /// Starts a builder for [`PlaceOSO`].
    pub fn builder() -> PlaceOSOBuilder {
        PlaceOSOBuilder::default()
    }
}

/// Builder for [`PlaceOSO`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct PlaceOSOBuilder {
    account_spec: Option<crate::AccountSpec>,
    account_id: Option<crate::AccountId>,
    cl_ord_id: Option<crate::ClientOrderId>,
    action: Option<PlaceOsoAction>,
    symbol: Option<crate::Symbol>,
    order_qty: Option<i64>,
    order_type: Option<PlaceOsoOrderType>,
    price: Option<crate::Decimal>,
    stop_price: Option<crate::Decimal>,
    limit_if_touched_price: Option<crate::Decimal>,
    max_show: Option<i64>,
    peg_difference: Option<crate::Decimal>,
    time_in_force: Option<PlaceOsoTimeInForce>,
    expire_time: Option<jiff::Timestamp>,
    text: Option<String>,
    activation_time: Option<jiff::Timestamp>,
    custom_tag50: Option<String>,
    is_automated: Option<bool>,
    bracket1: Option<RestrainedOrderVersion>,
    bracket2: Option<RestrainedOrderVersion>,
}

impl PlaceOSOBuilder {
    /// Sets wire field `accountSpec`.
    pub fn account_spec(mut self, value: crate::AccountSpec) -> Self {
        self.account_spec = Some(value);
        self
    }

    /// Sets wire field `accountId`.
    pub fn account_id(mut self, value: crate::AccountId) -> Self {
        self.account_id = Some(value);
        self
    }

    /// Sets wire field `clOrdId`.
    pub fn cl_ord_id(mut self, value: crate::ClientOrderId) -> Self {
        self.cl_ord_id = Some(value);
        self
    }

    /// Sets wire field `action`.
    pub fn action(mut self, value: PlaceOsoAction) -> Self {
        self.action = Some(value);
        self
    }

    /// Sets wire field `symbol`.
    pub fn symbol(mut self, value: crate::Symbol) -> Self {
        self.symbol = Some(value);
        self
    }

    /// Sets wire field `orderQty`.
    pub fn order_qty(mut self, value: i64) -> Self {
        self.order_qty = Some(value);
        self
    }

    /// Sets wire field `orderType`.
    pub fn order_type(mut self, value: PlaceOsoOrderType) -> Self {
        self.order_type = Some(value);
        self
    }

    /// Sets wire field `price`.
    pub fn price(mut self, value: crate::Decimal) -> Self {
        self.price = Some(value);
        self
    }

    /// Sets wire field `stopPrice`.
    pub fn stop_price(mut self, value: crate::Decimal) -> Self {
        self.stop_price = Some(value);
        self
    }

    /// Sets wire field `limitIfTouchedPrice`.
    pub fn limit_if_touched_price(mut self, value: crate::Decimal) -> Self {
        self.limit_if_touched_price = Some(value);
        self
    }

    /// Sets wire field `maxShow`.
    pub fn max_show(mut self, value: i64) -> Self {
        self.max_show = Some(value);
        self
    }

    /// Sets wire field `pegDifference`.
    pub fn peg_difference(mut self, value: crate::Decimal) -> Self {
        self.peg_difference = Some(value);
        self
    }

    /// Sets wire field `timeInForce`.
    pub fn time_in_force(mut self, value: PlaceOsoTimeInForce) -> Self {
        self.time_in_force = Some(value);
        self
    }

    /// Sets wire field `expireTime`.
    pub fn expire_time(mut self, value: jiff::Timestamp) -> Self {
        self.expire_time = Some(value);
        self
    }

    /// Sets wire field `text`.
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Sets wire field `activationTime`.
    pub fn activation_time(mut self, value: jiff::Timestamp) -> Self {
        self.activation_time = Some(value);
        self
    }

    /// Sets wire field `customTag50`.
    pub fn custom_tag50(mut self, value: impl Into<String>) -> Self {
        self.custom_tag50 = Some(value.into());
        self
    }

    /// Sets wire field `isAutomated`.
    pub fn is_automated(mut self, value: bool) -> Self {
        self.is_automated = Some(value);
        self
    }

    /// Sets wire field `bracket1`.
    pub fn bracket1(mut self, value: RestrainedOrderVersion) -> Self {
        self.bracket1 = Some(value);
        self
    }

    /// Sets wire field `bracket2`.
    pub fn bracket2(mut self, value: RestrainedOrderVersion) -> Self {
        self.bracket2 = Some(value);
        self
    }

    /// Validates required fields and builds [`PlaceOSO`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<PlaceOSO, crate::api::current::BuildError> {
        let action = self
            .action
            .ok_or(crate::api::current::BuildError::missing("action"))?;
        let symbol = self
            .symbol
            .ok_or(crate::api::current::BuildError::missing("symbol"))?;
        let order_qty = self
            .order_qty
            .ok_or(crate::api::current::BuildError::missing("orderQty"))?;
        let order_type = self
            .order_type
            .ok_or(crate::api::current::BuildError::missing("orderType"))?;
        let bracket1 = self
            .bracket1
            .ok_or(crate::api::current::BuildError::missing("bracket1"))?;
        Ok(PlaceOSO {
            account_spec: self.account_spec,
            account_id: self.account_id,
            cl_ord_id: self.cl_ord_id,
            action,
            symbol,
            order_qty,
            order_type,
            price: self.price,
            stop_price: self.stop_price,
            limit_if_touched_price: self.limit_if_touched_price,
            max_show: self.max_show,
            peg_difference: self.peg_difference,
            time_in_force: self.time_in_force,
            expire_time: self.expire_time,
            text: self.text,
            activation_time: self.activation_time,
            custom_tag50: self.custom_tag50,
            is_automated: self.is_automated,
            bracket1,
            bracket2: self.bracket2,
        })
    }
}

impl crate::api::current::support::CurrentRequest for PlaceOSO {
    fn validate_current(&self) -> Result<(), crate::Error> {
        Ok(())
    }
}

/// Current provider values for `PlaceOcoAction`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum PlaceOcoAction {
    /// Provider value `Buy`.
    Buy,
    /// Provider value `Sell`.
    Sell,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl PlaceOcoAction {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Buy => "Buy",
            Self::Sell => "Sell",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for PlaceOcoAction {
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

impl<'de> serde::Deserialize<'de> for PlaceOcoAction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "Buy" => Self::Buy,
            "Sell" => Self::Sell,
            _ => Self::Unknown(value),
        })
    }
}

/// Current provider values for `PlaceOcoOrderType`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum PlaceOcoOrderType {
    /// Provider value `Limit`.
    Limit,
    /// Provider value `LimitIfTouched`.
    LimitIfTouched,
    /// Provider value `MIT`.
    Mit,
    /// Provider value `Market`.
    Market,
    /// Provider value `QTS`.
    Qts,
    /// Provider value `Stop`.
    Stop,
    /// Provider value `StopLimit`.
    StopLimit,
    /// Provider value `TrailingStop`.
    TrailingStop,
    /// Provider value `TrailingStopLimit`.
    TrailingStopLimit,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl PlaceOcoOrderType {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Limit => "Limit",
            Self::LimitIfTouched => "LimitIfTouched",
            Self::Mit => "MIT",
            Self::Market => "Market",
            Self::Qts => "QTS",
            Self::Stop => "Stop",
            Self::StopLimit => "StopLimit",
            Self::TrailingStop => "TrailingStop",
            Self::TrailingStopLimit => "TrailingStopLimit",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for PlaceOcoOrderType {
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

impl<'de> serde::Deserialize<'de> for PlaceOcoOrderType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "Limit" => Self::Limit,
            "LimitIfTouched" => Self::LimitIfTouched,
            "MIT" => Self::Mit,
            "Market" => Self::Market,
            "QTS" => Self::Qts,
            "Stop" => Self::Stop,
            "StopLimit" => Self::StopLimit,
            "TrailingStop" => Self::TrailingStop,
            "TrailingStopLimit" => Self::TrailingStopLimit,
            _ => Self::Unknown(value),
        })
    }
}

/// Current wire model `PlaceOcoResult`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct PlaceOcoResult {
    #[serde(
        rename = "failureReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    failure_reason: Option<PlaceOcoResultFailureReason>,
    #[serde(
        rename = "failureText",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    failure_text: Option<String>,
    #[serde(rename = "orderId", default, skip_serializing_if = "Option::is_none")]
    order_id: Option<crate::OrderId>,
    #[serde(rename = "ocoId", default, skip_serializing_if = "Option::is_none")]
    oco_id: Option<super::ids::OcoId>,
}

impl PlaceOcoResult {
    /// Returns wire field `failureReason`.
    #[must_use]
    pub fn failure_reason(&self) -> Option<&PlaceOcoResultFailureReason> {
        self.failure_reason.as_ref()
    }

    /// Returns wire field `failureText`.
    #[must_use]
    pub fn failure_text(&self) -> Option<&str> {
        self.failure_text.as_deref()
    }

    /// Returns wire field `orderId`.
    #[must_use]
    pub fn order_id(&self) -> Option<&crate::OrderId> {
        self.order_id.as_ref()
    }

    /// Returns wire field `ocoId`.
    #[must_use]
    pub fn oco_id(&self) -> Option<&super::ids::OcoId> {
        self.oco_id.as_ref()
    }

    /// Starts a builder for [`PlaceOcoResult`].
    pub fn builder() -> PlaceOcoResultBuilder {
        PlaceOcoResultBuilder::default()
    }
}

/// Builder for [`PlaceOcoResult`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct PlaceOcoResultBuilder {
    failure_reason: Option<PlaceOcoResultFailureReason>,
    failure_text: Option<String>,
    order_id: Option<crate::OrderId>,
    oco_id: Option<super::ids::OcoId>,
}

impl PlaceOcoResultBuilder {
    /// Sets wire field `failureReason`.
    pub fn failure_reason(mut self, value: PlaceOcoResultFailureReason) -> Self {
        self.failure_reason = Some(value);
        self
    }

    /// Sets wire field `failureText`.
    pub fn failure_text(mut self, value: impl Into<String>) -> Self {
        self.failure_text = Some(value.into());
        self
    }

    /// Sets wire field `orderId`.
    pub fn order_id(mut self, value: crate::OrderId) -> Self {
        self.order_id = Some(value);
        self
    }

    /// Sets wire field `ocoId`.
    pub fn oco_id(mut self, value: super::ids::OcoId) -> Self {
        self.oco_id = Some(value);
        self
    }

    /// Validates required fields and builds [`PlaceOcoResult`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<PlaceOcoResult, crate::api::current::BuildError> {
        Ok(PlaceOcoResult {
            failure_reason: self.failure_reason,
            failure_text: self.failure_text,
            order_id: self.order_id,
            oco_id: self.oco_id,
        })
    }
}

/// Current provider values for `PlaceOcoResultFailureReason`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum PlaceOcoResultFailureReason {
    /// Provider value `AccountClosed`.
    AccountClosed,
    /// Provider value `AdvancedTrailingStopUnsupported`.
    AdvancedTrailingStopUnsupported,
    /// Provider value `AnotherCommandPending`.
    AnotherCommandPending,
    /// Provider value `BackMonthProhibited`.
    BackMonthProhibited,
    /// Provider value `ExecutionProviderNotConfigured`.
    ExecutionProviderNotConfigured,
    /// Provider value `ExecutionProviderUnavailable`.
    ExecutionProviderUnavailable,
    /// Provider value `InvalidContract`.
    InvalidContract,
    /// Provider value `InvalidPrice`.
    InvalidPrice,
    /// Provider value `KeyInformationDocumentRequired`.
    KeyInformationDocumentRequired,
    /// Provider value `LiquidationOnly`.
    LiquidationOnly,
    /// Provider value `LiquidationOnlyBeforeExpiration`.
    LiquidationOnlyBeforeExpiration,
    /// Provider value `MaxOrderQtyIsNotSpecified`.
    MaxOrderQtyIsNotSpecified,
    /// Provider value `MaxOrderQtyLimitReached`.
    MaxOrderQtyLimitReached,
    /// Provider value `MaxPosLimitMisconfigured`.
    MaxPosLimitMisconfigured,
    /// Provider value `MaxPosLimitReached`.
    MaxPosLimitReached,
    /// Provider value `MaxTotalPosLimitReached`.
    MaxTotalPosLimitReached,
    /// Provider value `MultipleAccountPlanRequired`.
    MultipleAccountPlanRequired,
    /// Provider value `NoQuote`.
    NoQuote,
    /// Provider value `NotEnoughLiquidity`.
    NotEnoughLiquidity,
    /// Provider value `OtherExecutionRelated`.
    OtherExecutionRelated,
    /// Provider value `ParentRejected`.
    ParentRejected,
    /// Provider value `RiskCheckTimeout`.
    RiskCheckTimeout,
    /// Provider value `SSFRiskDisclosureAcknowledgmentRequired`.
    SsfRiskDisclosureAcknowledgmentRequired,
    /// Provider value `SessionClosed`.
    SessionClosed,
    /// Provider value `Success`.
    Success,
    /// Provider value `TooLate`.
    TooLate,
    /// Provider value `TradingLocked`.
    TradingLocked,
    /// Provider value `TrailingStopNonOrderQtyModify`.
    TrailingStopNonOrderQtyModify,
    /// Provider value `Unauthorized`.
    Unauthorized,
    /// Provider value `UnknownReason`.
    UnknownReason,
    /// Provider value `Unsupported`.
    Unsupported,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl PlaceOcoResultFailureReason {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::AccountClosed => "AccountClosed",
            Self::AdvancedTrailingStopUnsupported => "AdvancedTrailingStopUnsupported",
            Self::AnotherCommandPending => "AnotherCommandPending",
            Self::BackMonthProhibited => "BackMonthProhibited",
            Self::ExecutionProviderNotConfigured => "ExecutionProviderNotConfigured",
            Self::ExecutionProviderUnavailable => "ExecutionProviderUnavailable",
            Self::InvalidContract => "InvalidContract",
            Self::InvalidPrice => "InvalidPrice",
            Self::KeyInformationDocumentRequired => "KeyInformationDocumentRequired",
            Self::LiquidationOnly => "LiquidationOnly",
            Self::LiquidationOnlyBeforeExpiration => "LiquidationOnlyBeforeExpiration",
            Self::MaxOrderQtyIsNotSpecified => "MaxOrderQtyIsNotSpecified",
            Self::MaxOrderQtyLimitReached => "MaxOrderQtyLimitReached",
            Self::MaxPosLimitMisconfigured => "MaxPosLimitMisconfigured",
            Self::MaxPosLimitReached => "MaxPosLimitReached",
            Self::MaxTotalPosLimitReached => "MaxTotalPosLimitReached",
            Self::MultipleAccountPlanRequired => "MultipleAccountPlanRequired",
            Self::NoQuote => "NoQuote",
            Self::NotEnoughLiquidity => "NotEnoughLiquidity",
            Self::OtherExecutionRelated => "OtherExecutionRelated",
            Self::ParentRejected => "ParentRejected",
            Self::RiskCheckTimeout => "RiskCheckTimeout",
            Self::SsfRiskDisclosureAcknowledgmentRequired => {
                "SSFRiskDisclosureAcknowledgmentRequired"
            }
            Self::SessionClosed => "SessionClosed",
            Self::Success => "Success",
            Self::TooLate => "TooLate",
            Self::TradingLocked => "TradingLocked",
            Self::TrailingStopNonOrderQtyModify => "TrailingStopNonOrderQtyModify",
            Self::Unauthorized => "Unauthorized",
            Self::UnknownReason => "UnknownReason",
            Self::Unsupported => "Unsupported",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for PlaceOcoResultFailureReason {
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

impl<'de> serde::Deserialize<'de> for PlaceOcoResultFailureReason {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "AccountClosed" => Self::AccountClosed,
            "AdvancedTrailingStopUnsupported" => Self::AdvancedTrailingStopUnsupported,
            "AnotherCommandPending" => Self::AnotherCommandPending,
            "BackMonthProhibited" => Self::BackMonthProhibited,
            "ExecutionProviderNotConfigured" => Self::ExecutionProviderNotConfigured,
            "ExecutionProviderUnavailable" => Self::ExecutionProviderUnavailable,
            "InvalidContract" => Self::InvalidContract,
            "InvalidPrice" => Self::InvalidPrice,
            "KeyInformationDocumentRequired" => Self::KeyInformationDocumentRequired,
            "LiquidationOnly" => Self::LiquidationOnly,
            "LiquidationOnlyBeforeExpiration" => Self::LiquidationOnlyBeforeExpiration,
            "MaxOrderQtyIsNotSpecified" => Self::MaxOrderQtyIsNotSpecified,
            "MaxOrderQtyLimitReached" => Self::MaxOrderQtyLimitReached,
            "MaxPosLimitMisconfigured" => Self::MaxPosLimitMisconfigured,
            "MaxPosLimitReached" => Self::MaxPosLimitReached,
            "MaxTotalPosLimitReached" => Self::MaxTotalPosLimitReached,
            "MultipleAccountPlanRequired" => Self::MultipleAccountPlanRequired,
            "NoQuote" => Self::NoQuote,
            "NotEnoughLiquidity" => Self::NotEnoughLiquidity,
            "OtherExecutionRelated" => Self::OtherExecutionRelated,
            "ParentRejected" => Self::ParentRejected,
            "RiskCheckTimeout" => Self::RiskCheckTimeout,
            "SSFRiskDisclosureAcknowledgmentRequired" => {
                Self::SsfRiskDisclosureAcknowledgmentRequired
            }
            "SessionClosed" => Self::SessionClosed,
            "Success" => Self::Success,
            "TooLate" => Self::TooLate,
            "TradingLocked" => Self::TradingLocked,
            "TrailingStopNonOrderQtyModify" => Self::TrailingStopNonOrderQtyModify,
            "Unauthorized" => Self::Unauthorized,
            "UnknownReason" => Self::UnknownReason,
            "Unsupported" => Self::Unsupported,
            _ => Self::Unknown(value),
        })
    }
}

/// Current provider values for `PlaceOcoTimeInForce`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum PlaceOcoTimeInForce {
    /// Provider value `Day`.
    Day,
    /// Provider value `FOK`.
    Fok,
    /// Provider value `GTC`.
    Gtc,
    /// Provider value `GTD`.
    Gtd,
    /// Provider value `IOC`.
    Ioc,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl PlaceOcoTimeInForce {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Day => "Day",
            Self::Fok => "FOK",
            Self::Gtc => "GTC",
            Self::Gtd => "GTD",
            Self::Ioc => "IOC",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for PlaceOcoTimeInForce {
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

impl<'de> serde::Deserialize<'de> for PlaceOcoTimeInForce {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "Day" => Self::Day,
            "FOK" => Self::Fok,
            "GTC" => Self::Gtc,
            "GTD" => Self::Gtd,
            "IOC" => Self::Ioc,
            _ => Self::Unknown(value),
        })
    }
}

/// Current wire model `PlaceOrder`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct PlaceOrder {
    #[serde(
        rename = "accountSpec",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    account_spec: Option<crate::AccountSpec>,
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    account_id: Option<crate::AccountId>,
    #[serde(rename = "clOrdId", default, skip_serializing_if = "Option::is_none")]
    cl_ord_id: Option<crate::ClientOrderId>,
    #[serde(rename = "action")]
    action: PlaceOrderAction,
    #[serde(rename = "symbol")]
    symbol: crate::Symbol,
    #[serde(rename = "orderQty")]
    order_qty: i64,
    #[serde(rename = "orderType")]
    order_type: PlaceOrderOrderType,
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "crate::decimal::option")]
    price: Option<crate::Decimal>,
    #[serde(rename = "stopPrice", default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "crate::decimal::option")]
    stop_price: Option<crate::Decimal>,
    #[serde(
        rename = "limitIfTouchedPrice",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    limit_if_touched_price: Option<crate::Decimal>,
    #[serde(rename = "maxShow", default, skip_serializing_if = "Option::is_none")]
    max_show: Option<i64>,
    #[serde(
        rename = "pegDifference",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    peg_difference: Option<crate::Decimal>,
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    time_in_force: Option<PlaceOrderTimeInForce>,
    #[serde(
        rename = "expireTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    expire_time: Option<jiff::Timestamp>,
    #[serde(rename = "text", default, skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(
        rename = "activationTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    activation_time: Option<jiff::Timestamp>,
    #[serde(
        rename = "customTag50",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    custom_tag50: Option<String>,
    #[serde(
        rename = "isAutomated",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    is_automated: Option<bool>,
}

impl PlaceOrder {
    /// Returns wire field `accountSpec`.
    #[must_use]
    pub fn account_spec(&self) -> Option<&crate::AccountSpec> {
        self.account_spec.as_ref()
    }

    /// Returns wire field `accountId`.
    #[must_use]
    pub fn account_id(&self) -> Option<&crate::AccountId> {
        self.account_id.as_ref()
    }

    /// Returns wire field `clOrdId`.
    #[must_use]
    pub fn cl_ord_id(&self) -> Option<&crate::ClientOrderId> {
        self.cl_ord_id.as_ref()
    }

    /// Returns wire field `action`.
    #[must_use]
    pub fn action(&self) -> &PlaceOrderAction {
        &self.action
    }

    /// Returns wire field `symbol`.
    #[must_use]
    pub fn symbol(&self) -> &crate::Symbol {
        &self.symbol
    }

    /// Returns wire field `orderQty`.
    #[must_use]
    pub fn order_qty(&self) -> &i64 {
        &self.order_qty
    }

    /// Returns wire field `orderType`.
    #[must_use]
    pub fn order_type(&self) -> &PlaceOrderOrderType {
        &self.order_type
    }

    /// Returns wire field `price`.
    #[must_use]
    pub fn price(&self) -> Option<&crate::Decimal> {
        self.price.as_ref()
    }

    /// Returns wire field `stopPrice`.
    #[must_use]
    pub fn stop_price(&self) -> Option<&crate::Decimal> {
        self.stop_price.as_ref()
    }

    /// Returns wire field `limitIfTouchedPrice`.
    #[must_use]
    pub fn limit_if_touched_price(&self) -> Option<&crate::Decimal> {
        self.limit_if_touched_price.as_ref()
    }

    /// Returns wire field `maxShow`.
    #[must_use]
    pub fn max_show(&self) -> Option<&i64> {
        self.max_show.as_ref()
    }

    /// Returns wire field `pegDifference`.
    #[must_use]
    pub fn peg_difference(&self) -> Option<&crate::Decimal> {
        self.peg_difference.as_ref()
    }

    /// Returns wire field `timeInForce`.
    #[must_use]
    pub fn time_in_force(&self) -> Option<&PlaceOrderTimeInForce> {
        self.time_in_force.as_ref()
    }

    /// Returns wire field `expireTime`.
    #[must_use]
    pub fn expire_time(&self) -> Option<&jiff::Timestamp> {
        self.expire_time.as_ref()
    }

    /// Returns wire field `text`.
    #[must_use]
    pub fn text(&self) -> Option<&str> {
        self.text.as_deref()
    }

    /// Returns wire field `activationTime`.
    #[must_use]
    pub fn activation_time(&self) -> Option<&jiff::Timestamp> {
        self.activation_time.as_ref()
    }

    /// Returns wire field `customTag50`.
    #[must_use]
    pub fn custom_tag50(&self) -> Option<&str> {
        self.custom_tag50.as_deref()
    }

    /// Returns wire field `isAutomated`.
    #[must_use]
    pub fn is_automated(&self) -> Option<&bool> {
        self.is_automated.as_ref()
    }

    /// Starts a builder for [`PlaceOrder`].
    pub fn builder() -> PlaceOrderBuilder {
        PlaceOrderBuilder::default()
    }
}

/// Builder for [`PlaceOrder`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct PlaceOrderBuilder {
    account_spec: Option<crate::AccountSpec>,
    account_id: Option<crate::AccountId>,
    cl_ord_id: Option<crate::ClientOrderId>,
    action: Option<PlaceOrderAction>,
    symbol: Option<crate::Symbol>,
    order_qty: Option<i64>,
    order_type: Option<PlaceOrderOrderType>,
    price: Option<crate::Decimal>,
    stop_price: Option<crate::Decimal>,
    limit_if_touched_price: Option<crate::Decimal>,
    max_show: Option<i64>,
    peg_difference: Option<crate::Decimal>,
    time_in_force: Option<PlaceOrderTimeInForce>,
    expire_time: Option<jiff::Timestamp>,
    text: Option<String>,
    activation_time: Option<jiff::Timestamp>,
    custom_tag50: Option<String>,
    is_automated: Option<bool>,
}

impl PlaceOrderBuilder {
    /// Sets wire field `accountSpec`.
    pub fn account_spec(mut self, value: crate::AccountSpec) -> Self {
        self.account_spec = Some(value);
        self
    }

    /// Sets wire field `accountId`.
    pub fn account_id(mut self, value: crate::AccountId) -> Self {
        self.account_id = Some(value);
        self
    }

    /// Sets wire field `clOrdId`.
    pub fn cl_ord_id(mut self, value: crate::ClientOrderId) -> Self {
        self.cl_ord_id = Some(value);
        self
    }

    /// Sets wire field `action`.
    pub fn action(mut self, value: PlaceOrderAction) -> Self {
        self.action = Some(value);
        self
    }

    /// Sets wire field `symbol`.
    pub fn symbol(mut self, value: crate::Symbol) -> Self {
        self.symbol = Some(value);
        self
    }

    /// Sets wire field `orderQty`.
    pub fn order_qty(mut self, value: i64) -> Self {
        self.order_qty = Some(value);
        self
    }

    /// Sets wire field `orderType`.
    pub fn order_type(mut self, value: PlaceOrderOrderType) -> Self {
        self.order_type = Some(value);
        self
    }

    /// Sets wire field `price`.
    pub fn price(mut self, value: crate::Decimal) -> Self {
        self.price = Some(value);
        self
    }

    /// Sets wire field `stopPrice`.
    pub fn stop_price(mut self, value: crate::Decimal) -> Self {
        self.stop_price = Some(value);
        self
    }

    /// Sets wire field `limitIfTouchedPrice`.
    pub fn limit_if_touched_price(mut self, value: crate::Decimal) -> Self {
        self.limit_if_touched_price = Some(value);
        self
    }

    /// Sets wire field `maxShow`.
    pub fn max_show(mut self, value: i64) -> Self {
        self.max_show = Some(value);
        self
    }

    /// Sets wire field `pegDifference`.
    pub fn peg_difference(mut self, value: crate::Decimal) -> Self {
        self.peg_difference = Some(value);
        self
    }

    /// Sets wire field `timeInForce`.
    pub fn time_in_force(mut self, value: PlaceOrderTimeInForce) -> Self {
        self.time_in_force = Some(value);
        self
    }

    /// Sets wire field `expireTime`.
    pub fn expire_time(mut self, value: jiff::Timestamp) -> Self {
        self.expire_time = Some(value);
        self
    }

    /// Sets wire field `text`.
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Sets wire field `activationTime`.
    pub fn activation_time(mut self, value: jiff::Timestamp) -> Self {
        self.activation_time = Some(value);
        self
    }

    /// Sets wire field `customTag50`.
    pub fn custom_tag50(mut self, value: impl Into<String>) -> Self {
        self.custom_tag50 = Some(value.into());
        self
    }

    /// Sets wire field `isAutomated`.
    pub fn is_automated(mut self, value: bool) -> Self {
        self.is_automated = Some(value);
        self
    }

    /// Validates required fields and builds [`PlaceOrder`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<PlaceOrder, crate::api::current::BuildError> {
        let action = self
            .action
            .ok_or(crate::api::current::BuildError::missing("action"))?;
        let symbol = self
            .symbol
            .ok_or(crate::api::current::BuildError::missing("symbol"))?;
        let order_qty = self
            .order_qty
            .ok_or(crate::api::current::BuildError::missing("orderQty"))?;
        let order_type = self
            .order_type
            .ok_or(crate::api::current::BuildError::missing("orderType"))?;
        Ok(PlaceOrder {
            account_spec: self.account_spec,
            account_id: self.account_id,
            cl_ord_id: self.cl_ord_id,
            action,
            symbol,
            order_qty,
            order_type,
            price: self.price,
            stop_price: self.stop_price,
            limit_if_touched_price: self.limit_if_touched_price,
            max_show: self.max_show,
            peg_difference: self.peg_difference,
            time_in_force: self.time_in_force,
            expire_time: self.expire_time,
            text: self.text,
            activation_time: self.activation_time,
            custom_tag50: self.custom_tag50,
            is_automated: self.is_automated,
        })
    }
}

impl crate::api::current::support::CurrentRequest for PlaceOrder {
    fn validate_current(&self) -> Result<(), crate::Error> {
        Ok(())
    }
}

/// Current provider values for `PlaceOrderAction`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum PlaceOrderAction {
    /// Provider value `Buy`.
    Buy,
    /// Provider value `Sell`.
    Sell,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl PlaceOrderAction {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Buy => "Buy",
            Self::Sell => "Sell",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for PlaceOrderAction {
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

impl<'de> serde::Deserialize<'de> for PlaceOrderAction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "Buy" => Self::Buy,
            "Sell" => Self::Sell,
            _ => Self::Unknown(value),
        })
    }
}

/// Current provider values for `PlaceOrderOrderType`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum PlaceOrderOrderType {
    /// Provider value `Limit`.
    Limit,
    /// Provider value `LimitIfTouched`.
    LimitIfTouched,
    /// Provider value `MIT`.
    Mit,
    /// Provider value `Market`.
    Market,
    /// Provider value `QTS`.
    Qts,
    /// Provider value `Stop`.
    Stop,
    /// Provider value `StopLimit`.
    StopLimit,
    /// Provider value `TrailingStop`.
    TrailingStop,
    /// Provider value `TrailingStopLimit`.
    TrailingStopLimit,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl PlaceOrderOrderType {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Limit => "Limit",
            Self::LimitIfTouched => "LimitIfTouched",
            Self::Mit => "MIT",
            Self::Market => "Market",
            Self::Qts => "QTS",
            Self::Stop => "Stop",
            Self::StopLimit => "StopLimit",
            Self::TrailingStop => "TrailingStop",
            Self::TrailingStopLimit => "TrailingStopLimit",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for PlaceOrderOrderType {
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

impl<'de> serde::Deserialize<'de> for PlaceOrderOrderType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "Limit" => Self::Limit,
            "LimitIfTouched" => Self::LimitIfTouched,
            "MIT" => Self::Mit,
            "Market" => Self::Market,
            "QTS" => Self::Qts,
            "Stop" => Self::Stop,
            "StopLimit" => Self::StopLimit,
            "TrailingStop" => Self::TrailingStop,
            "TrailingStopLimit" => Self::TrailingStopLimit,
            _ => Self::Unknown(value),
        })
    }
}

/// Current wire model `PlaceOrderResult`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct PlaceOrderResult {
    #[serde(
        rename = "failureReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    failure_reason: Option<PlaceOrderResultFailureReason>,
    #[serde(
        rename = "failureText",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    failure_text: Option<String>,
    #[serde(rename = "orderId", default, skip_serializing_if = "Option::is_none")]
    order_id: Option<crate::OrderId>,
}

impl PlaceOrderResult {
    /// Returns wire field `failureReason`.
    #[must_use]
    pub fn failure_reason(&self) -> Option<&PlaceOrderResultFailureReason> {
        self.failure_reason.as_ref()
    }

    /// Returns wire field `failureText`.
    #[must_use]
    pub fn failure_text(&self) -> Option<&str> {
        self.failure_text.as_deref()
    }

    /// Returns wire field `orderId`.
    #[must_use]
    pub fn order_id(&self) -> Option<&crate::OrderId> {
        self.order_id.as_ref()
    }

    /// Starts a builder for [`PlaceOrderResult`].
    pub fn builder() -> PlaceOrderResultBuilder {
        PlaceOrderResultBuilder::default()
    }
}

/// Builder for [`PlaceOrderResult`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct PlaceOrderResultBuilder {
    failure_reason: Option<PlaceOrderResultFailureReason>,
    failure_text: Option<String>,
    order_id: Option<crate::OrderId>,
}

impl PlaceOrderResultBuilder {
    /// Sets wire field `failureReason`.
    pub fn failure_reason(mut self, value: PlaceOrderResultFailureReason) -> Self {
        self.failure_reason = Some(value);
        self
    }

    /// Sets wire field `failureText`.
    pub fn failure_text(mut self, value: impl Into<String>) -> Self {
        self.failure_text = Some(value.into());
        self
    }

    /// Sets wire field `orderId`.
    pub fn order_id(mut self, value: crate::OrderId) -> Self {
        self.order_id = Some(value);
        self
    }

    /// Validates required fields and builds [`PlaceOrderResult`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<PlaceOrderResult, crate::api::current::BuildError> {
        Ok(PlaceOrderResult {
            failure_reason: self.failure_reason,
            failure_text: self.failure_text,
            order_id: self.order_id,
        })
    }
}

/// Current provider values for `PlaceOrderResultFailureReason`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum PlaceOrderResultFailureReason {
    /// Provider value `AccountClosed`.
    AccountClosed,
    /// Provider value `AdvancedTrailingStopUnsupported`.
    AdvancedTrailingStopUnsupported,
    /// Provider value `AnotherCommandPending`.
    AnotherCommandPending,
    /// Provider value `BackMonthProhibited`.
    BackMonthProhibited,
    /// Provider value `ExecutionProviderNotConfigured`.
    ExecutionProviderNotConfigured,
    /// Provider value `ExecutionProviderUnavailable`.
    ExecutionProviderUnavailable,
    /// Provider value `InvalidContract`.
    InvalidContract,
    /// Provider value `InvalidPrice`.
    InvalidPrice,
    /// Provider value `KeyInformationDocumentRequired`.
    KeyInformationDocumentRequired,
    /// Provider value `LiquidationOnly`.
    LiquidationOnly,
    /// Provider value `LiquidationOnlyBeforeExpiration`.
    LiquidationOnlyBeforeExpiration,
    /// Provider value `MaxOrderQtyIsNotSpecified`.
    MaxOrderQtyIsNotSpecified,
    /// Provider value `MaxOrderQtyLimitReached`.
    MaxOrderQtyLimitReached,
    /// Provider value `MaxPosLimitMisconfigured`.
    MaxPosLimitMisconfigured,
    /// Provider value `MaxPosLimitReached`.
    MaxPosLimitReached,
    /// Provider value `MaxTotalPosLimitReached`.
    MaxTotalPosLimitReached,
    /// Provider value `MultipleAccountPlanRequired`.
    MultipleAccountPlanRequired,
    /// Provider value `NoQuote`.
    NoQuote,
    /// Provider value `NotEnoughLiquidity`.
    NotEnoughLiquidity,
    /// Provider value `OtherExecutionRelated`.
    OtherExecutionRelated,
    /// Provider value `ParentRejected`.
    ParentRejected,
    /// Provider value `RiskCheckTimeout`.
    RiskCheckTimeout,
    /// Provider value `SSFRiskDisclosureAcknowledgmentRequired`.
    SsfRiskDisclosureAcknowledgmentRequired,
    /// Provider value `SessionClosed`.
    SessionClosed,
    /// Provider value `Success`.
    Success,
    /// Provider value `TooLate`.
    TooLate,
    /// Provider value `TradingLocked`.
    TradingLocked,
    /// Provider value `TrailingStopNonOrderQtyModify`.
    TrailingStopNonOrderQtyModify,
    /// Provider value `Unauthorized`.
    Unauthorized,
    /// Provider value `UnknownReason`.
    UnknownReason,
    /// Provider value `Unsupported`.
    Unsupported,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl PlaceOrderResultFailureReason {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::AccountClosed => "AccountClosed",
            Self::AdvancedTrailingStopUnsupported => "AdvancedTrailingStopUnsupported",
            Self::AnotherCommandPending => "AnotherCommandPending",
            Self::BackMonthProhibited => "BackMonthProhibited",
            Self::ExecutionProviderNotConfigured => "ExecutionProviderNotConfigured",
            Self::ExecutionProviderUnavailable => "ExecutionProviderUnavailable",
            Self::InvalidContract => "InvalidContract",
            Self::InvalidPrice => "InvalidPrice",
            Self::KeyInformationDocumentRequired => "KeyInformationDocumentRequired",
            Self::LiquidationOnly => "LiquidationOnly",
            Self::LiquidationOnlyBeforeExpiration => "LiquidationOnlyBeforeExpiration",
            Self::MaxOrderQtyIsNotSpecified => "MaxOrderQtyIsNotSpecified",
            Self::MaxOrderQtyLimitReached => "MaxOrderQtyLimitReached",
            Self::MaxPosLimitMisconfigured => "MaxPosLimitMisconfigured",
            Self::MaxPosLimitReached => "MaxPosLimitReached",
            Self::MaxTotalPosLimitReached => "MaxTotalPosLimitReached",
            Self::MultipleAccountPlanRequired => "MultipleAccountPlanRequired",
            Self::NoQuote => "NoQuote",
            Self::NotEnoughLiquidity => "NotEnoughLiquidity",
            Self::OtherExecutionRelated => "OtherExecutionRelated",
            Self::ParentRejected => "ParentRejected",
            Self::RiskCheckTimeout => "RiskCheckTimeout",
            Self::SsfRiskDisclosureAcknowledgmentRequired => {
                "SSFRiskDisclosureAcknowledgmentRequired"
            }
            Self::SessionClosed => "SessionClosed",
            Self::Success => "Success",
            Self::TooLate => "TooLate",
            Self::TradingLocked => "TradingLocked",
            Self::TrailingStopNonOrderQtyModify => "TrailingStopNonOrderQtyModify",
            Self::Unauthorized => "Unauthorized",
            Self::UnknownReason => "UnknownReason",
            Self::Unsupported => "Unsupported",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for PlaceOrderResultFailureReason {
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

impl<'de> serde::Deserialize<'de> for PlaceOrderResultFailureReason {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "AccountClosed" => Self::AccountClosed,
            "AdvancedTrailingStopUnsupported" => Self::AdvancedTrailingStopUnsupported,
            "AnotherCommandPending" => Self::AnotherCommandPending,
            "BackMonthProhibited" => Self::BackMonthProhibited,
            "ExecutionProviderNotConfigured" => Self::ExecutionProviderNotConfigured,
            "ExecutionProviderUnavailable" => Self::ExecutionProviderUnavailable,
            "InvalidContract" => Self::InvalidContract,
            "InvalidPrice" => Self::InvalidPrice,
            "KeyInformationDocumentRequired" => Self::KeyInformationDocumentRequired,
            "LiquidationOnly" => Self::LiquidationOnly,
            "LiquidationOnlyBeforeExpiration" => Self::LiquidationOnlyBeforeExpiration,
            "MaxOrderQtyIsNotSpecified" => Self::MaxOrderQtyIsNotSpecified,
            "MaxOrderQtyLimitReached" => Self::MaxOrderQtyLimitReached,
            "MaxPosLimitMisconfigured" => Self::MaxPosLimitMisconfigured,
            "MaxPosLimitReached" => Self::MaxPosLimitReached,
            "MaxTotalPosLimitReached" => Self::MaxTotalPosLimitReached,
            "MultipleAccountPlanRequired" => Self::MultipleAccountPlanRequired,
            "NoQuote" => Self::NoQuote,
            "NotEnoughLiquidity" => Self::NotEnoughLiquidity,
            "OtherExecutionRelated" => Self::OtherExecutionRelated,
            "ParentRejected" => Self::ParentRejected,
            "RiskCheckTimeout" => Self::RiskCheckTimeout,
            "SSFRiskDisclosureAcknowledgmentRequired" => {
                Self::SsfRiskDisclosureAcknowledgmentRequired
            }
            "SessionClosed" => Self::SessionClosed,
            "Success" => Self::Success,
            "TooLate" => Self::TooLate,
            "TradingLocked" => Self::TradingLocked,
            "TrailingStopNonOrderQtyModify" => Self::TrailingStopNonOrderQtyModify,
            "Unauthorized" => Self::Unauthorized,
            "UnknownReason" => Self::UnknownReason,
            "Unsupported" => Self::Unsupported,
            _ => Self::Unknown(value),
        })
    }
}

/// Current provider values for `PlaceOrderTimeInForce`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum PlaceOrderTimeInForce {
    /// Provider value `Day`.
    Day,
    /// Provider value `FOK`.
    Fok,
    /// Provider value `GTC`.
    Gtc,
    /// Provider value `GTD`.
    Gtd,
    /// Provider value `IOC`.
    Ioc,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl PlaceOrderTimeInForce {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Day => "Day",
            Self::Fok => "FOK",
            Self::Gtc => "GTC",
            Self::Gtd => "GTD",
            Self::Ioc => "IOC",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for PlaceOrderTimeInForce {
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

impl<'de> serde::Deserialize<'de> for PlaceOrderTimeInForce {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "Day" => Self::Day,
            "FOK" => Self::Fok,
            "GTC" => Self::Gtc,
            "GTD" => Self::Gtd,
            "IOC" => Self::Ioc,
            _ => Self::Unknown(value),
        })
    }
}

/// Current provider values for `PlaceOsoAction`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum PlaceOsoAction {
    /// Provider value `Buy`.
    Buy,
    /// Provider value `Sell`.
    Sell,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl PlaceOsoAction {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Buy => "Buy",
            Self::Sell => "Sell",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for PlaceOsoAction {
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

impl<'de> serde::Deserialize<'de> for PlaceOsoAction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "Buy" => Self::Buy,
            "Sell" => Self::Sell,
            _ => Self::Unknown(value),
        })
    }
}

/// Current provider values for `PlaceOsoOrderType`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum PlaceOsoOrderType {
    /// Provider value `Limit`.
    Limit,
    /// Provider value `LimitIfTouched`.
    LimitIfTouched,
    /// Provider value `MIT`.
    Mit,
    /// Provider value `Market`.
    Market,
    /// Provider value `QTS`.
    Qts,
    /// Provider value `Stop`.
    Stop,
    /// Provider value `StopLimit`.
    StopLimit,
    /// Provider value `TrailingStop`.
    TrailingStop,
    /// Provider value `TrailingStopLimit`.
    TrailingStopLimit,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl PlaceOsoOrderType {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Limit => "Limit",
            Self::LimitIfTouched => "LimitIfTouched",
            Self::Mit => "MIT",
            Self::Market => "Market",
            Self::Qts => "QTS",
            Self::Stop => "Stop",
            Self::StopLimit => "StopLimit",
            Self::TrailingStop => "TrailingStop",
            Self::TrailingStopLimit => "TrailingStopLimit",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for PlaceOsoOrderType {
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

impl<'de> serde::Deserialize<'de> for PlaceOsoOrderType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "Limit" => Self::Limit,
            "LimitIfTouched" => Self::LimitIfTouched,
            "MIT" => Self::Mit,
            "Market" => Self::Market,
            "QTS" => Self::Qts,
            "Stop" => Self::Stop,
            "StopLimit" => Self::StopLimit,
            "TrailingStop" => Self::TrailingStop,
            "TrailingStopLimit" => Self::TrailingStopLimit,
            _ => Self::Unknown(value),
        })
    }
}

/// Current wire model `PlaceOsoResult`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct PlaceOsoResult {
    #[serde(
        rename = "failureReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    failure_reason: Option<PlaceOsoResultFailureReason>,
    #[serde(
        rename = "failureText",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    failure_text: Option<String>,
    #[serde(rename = "orderId", default, skip_serializing_if = "Option::is_none")]
    order_id: Option<crate::OrderId>,
    #[serde(rename = "oso1Id", default, skip_serializing_if = "Option::is_none")]
    oso1_id: Option<super::ids::Oso1Id>,
    #[serde(rename = "oso2Id", default, skip_serializing_if = "Option::is_none")]
    oso2_id: Option<super::ids::Oso2Id>,
}

impl PlaceOsoResult {
    /// Returns wire field `failureReason`.
    #[must_use]
    pub fn failure_reason(&self) -> Option<&PlaceOsoResultFailureReason> {
        self.failure_reason.as_ref()
    }

    /// Returns wire field `failureText`.
    #[must_use]
    pub fn failure_text(&self) -> Option<&str> {
        self.failure_text.as_deref()
    }

    /// Returns wire field `orderId`.
    #[must_use]
    pub fn order_id(&self) -> Option<&crate::OrderId> {
        self.order_id.as_ref()
    }

    /// Returns wire field `oso1Id`.
    #[must_use]
    pub fn oso1_id(&self) -> Option<&super::ids::Oso1Id> {
        self.oso1_id.as_ref()
    }

    /// Returns wire field `oso2Id`.
    #[must_use]
    pub fn oso2_id(&self) -> Option<&super::ids::Oso2Id> {
        self.oso2_id.as_ref()
    }

    /// Starts a builder for [`PlaceOsoResult`].
    pub fn builder() -> PlaceOsoResultBuilder {
        PlaceOsoResultBuilder::default()
    }
}

/// Builder for [`PlaceOsoResult`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct PlaceOsoResultBuilder {
    failure_reason: Option<PlaceOsoResultFailureReason>,
    failure_text: Option<String>,
    order_id: Option<crate::OrderId>,
    oso1_id: Option<super::ids::Oso1Id>,
    oso2_id: Option<super::ids::Oso2Id>,
}

impl PlaceOsoResultBuilder {
    /// Sets wire field `failureReason`.
    pub fn failure_reason(mut self, value: PlaceOsoResultFailureReason) -> Self {
        self.failure_reason = Some(value);
        self
    }

    /// Sets wire field `failureText`.
    pub fn failure_text(mut self, value: impl Into<String>) -> Self {
        self.failure_text = Some(value.into());
        self
    }

    /// Sets wire field `orderId`.
    pub fn order_id(mut self, value: crate::OrderId) -> Self {
        self.order_id = Some(value);
        self
    }

    /// Sets wire field `oso1Id`.
    pub fn oso1_id(mut self, value: super::ids::Oso1Id) -> Self {
        self.oso1_id = Some(value);
        self
    }

    /// Sets wire field `oso2Id`.
    pub fn oso2_id(mut self, value: super::ids::Oso2Id) -> Self {
        self.oso2_id = Some(value);
        self
    }

    /// Validates required fields and builds [`PlaceOsoResult`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<PlaceOsoResult, crate::api::current::BuildError> {
        Ok(PlaceOsoResult {
            failure_reason: self.failure_reason,
            failure_text: self.failure_text,
            order_id: self.order_id,
            oso1_id: self.oso1_id,
            oso2_id: self.oso2_id,
        })
    }
}

/// Current provider values for `PlaceOsoResultFailureReason`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum PlaceOsoResultFailureReason {
    /// Provider value `AccountClosed`.
    AccountClosed,
    /// Provider value `AdvancedTrailingStopUnsupported`.
    AdvancedTrailingStopUnsupported,
    /// Provider value `AnotherCommandPending`.
    AnotherCommandPending,
    /// Provider value `BackMonthProhibited`.
    BackMonthProhibited,
    /// Provider value `ExecutionProviderNotConfigured`.
    ExecutionProviderNotConfigured,
    /// Provider value `ExecutionProviderUnavailable`.
    ExecutionProviderUnavailable,
    /// Provider value `InvalidContract`.
    InvalidContract,
    /// Provider value `InvalidPrice`.
    InvalidPrice,
    /// Provider value `KeyInformationDocumentRequired`.
    KeyInformationDocumentRequired,
    /// Provider value `LiquidationOnly`.
    LiquidationOnly,
    /// Provider value `LiquidationOnlyBeforeExpiration`.
    LiquidationOnlyBeforeExpiration,
    /// Provider value `MaxOrderQtyIsNotSpecified`.
    MaxOrderQtyIsNotSpecified,
    /// Provider value `MaxOrderQtyLimitReached`.
    MaxOrderQtyLimitReached,
    /// Provider value `MaxPosLimitMisconfigured`.
    MaxPosLimitMisconfigured,
    /// Provider value `MaxPosLimitReached`.
    MaxPosLimitReached,
    /// Provider value `MaxTotalPosLimitReached`.
    MaxTotalPosLimitReached,
    /// Provider value `MultipleAccountPlanRequired`.
    MultipleAccountPlanRequired,
    /// Provider value `NoQuote`.
    NoQuote,
    /// Provider value `NotEnoughLiquidity`.
    NotEnoughLiquidity,
    /// Provider value `OtherExecutionRelated`.
    OtherExecutionRelated,
    /// Provider value `ParentRejected`.
    ParentRejected,
    /// Provider value `RiskCheckTimeout`.
    RiskCheckTimeout,
    /// Provider value `SSFRiskDisclosureAcknowledgmentRequired`.
    SsfRiskDisclosureAcknowledgmentRequired,
    /// Provider value `SessionClosed`.
    SessionClosed,
    /// Provider value `Success`.
    Success,
    /// Provider value `TooLate`.
    TooLate,
    /// Provider value `TradingLocked`.
    TradingLocked,
    /// Provider value `TrailingStopNonOrderQtyModify`.
    TrailingStopNonOrderQtyModify,
    /// Provider value `Unauthorized`.
    Unauthorized,
    /// Provider value `UnknownReason`.
    UnknownReason,
    /// Provider value `Unsupported`.
    Unsupported,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl PlaceOsoResultFailureReason {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::AccountClosed => "AccountClosed",
            Self::AdvancedTrailingStopUnsupported => "AdvancedTrailingStopUnsupported",
            Self::AnotherCommandPending => "AnotherCommandPending",
            Self::BackMonthProhibited => "BackMonthProhibited",
            Self::ExecutionProviderNotConfigured => "ExecutionProviderNotConfigured",
            Self::ExecutionProviderUnavailable => "ExecutionProviderUnavailable",
            Self::InvalidContract => "InvalidContract",
            Self::InvalidPrice => "InvalidPrice",
            Self::KeyInformationDocumentRequired => "KeyInformationDocumentRequired",
            Self::LiquidationOnly => "LiquidationOnly",
            Self::LiquidationOnlyBeforeExpiration => "LiquidationOnlyBeforeExpiration",
            Self::MaxOrderQtyIsNotSpecified => "MaxOrderQtyIsNotSpecified",
            Self::MaxOrderQtyLimitReached => "MaxOrderQtyLimitReached",
            Self::MaxPosLimitMisconfigured => "MaxPosLimitMisconfigured",
            Self::MaxPosLimitReached => "MaxPosLimitReached",
            Self::MaxTotalPosLimitReached => "MaxTotalPosLimitReached",
            Self::MultipleAccountPlanRequired => "MultipleAccountPlanRequired",
            Self::NoQuote => "NoQuote",
            Self::NotEnoughLiquidity => "NotEnoughLiquidity",
            Self::OtherExecutionRelated => "OtherExecutionRelated",
            Self::ParentRejected => "ParentRejected",
            Self::RiskCheckTimeout => "RiskCheckTimeout",
            Self::SsfRiskDisclosureAcknowledgmentRequired => {
                "SSFRiskDisclosureAcknowledgmentRequired"
            }
            Self::SessionClosed => "SessionClosed",
            Self::Success => "Success",
            Self::TooLate => "TooLate",
            Self::TradingLocked => "TradingLocked",
            Self::TrailingStopNonOrderQtyModify => "TrailingStopNonOrderQtyModify",
            Self::Unauthorized => "Unauthorized",
            Self::UnknownReason => "UnknownReason",
            Self::Unsupported => "Unsupported",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for PlaceOsoResultFailureReason {
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

impl<'de> serde::Deserialize<'de> for PlaceOsoResultFailureReason {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "AccountClosed" => Self::AccountClosed,
            "AdvancedTrailingStopUnsupported" => Self::AdvancedTrailingStopUnsupported,
            "AnotherCommandPending" => Self::AnotherCommandPending,
            "BackMonthProhibited" => Self::BackMonthProhibited,
            "ExecutionProviderNotConfigured" => Self::ExecutionProviderNotConfigured,
            "ExecutionProviderUnavailable" => Self::ExecutionProviderUnavailable,
            "InvalidContract" => Self::InvalidContract,
            "InvalidPrice" => Self::InvalidPrice,
            "KeyInformationDocumentRequired" => Self::KeyInformationDocumentRequired,
            "LiquidationOnly" => Self::LiquidationOnly,
            "LiquidationOnlyBeforeExpiration" => Self::LiquidationOnlyBeforeExpiration,
            "MaxOrderQtyIsNotSpecified" => Self::MaxOrderQtyIsNotSpecified,
            "MaxOrderQtyLimitReached" => Self::MaxOrderQtyLimitReached,
            "MaxPosLimitMisconfigured" => Self::MaxPosLimitMisconfigured,
            "MaxPosLimitReached" => Self::MaxPosLimitReached,
            "MaxTotalPosLimitReached" => Self::MaxTotalPosLimitReached,
            "MultipleAccountPlanRequired" => Self::MultipleAccountPlanRequired,
            "NoQuote" => Self::NoQuote,
            "NotEnoughLiquidity" => Self::NotEnoughLiquidity,
            "OtherExecutionRelated" => Self::OtherExecutionRelated,
            "ParentRejected" => Self::ParentRejected,
            "RiskCheckTimeout" => Self::RiskCheckTimeout,
            "SSFRiskDisclosureAcknowledgmentRequired" => {
                Self::SsfRiskDisclosureAcknowledgmentRequired
            }
            "SessionClosed" => Self::SessionClosed,
            "Success" => Self::Success,
            "TooLate" => Self::TooLate,
            "TradingLocked" => Self::TradingLocked,
            "TrailingStopNonOrderQtyModify" => Self::TrailingStopNonOrderQtyModify,
            "Unauthorized" => Self::Unauthorized,
            "UnknownReason" => Self::UnknownReason,
            "Unsupported" => Self::Unsupported,
            _ => Self::Unknown(value),
        })
    }
}

/// Current provider values for `PlaceOsoTimeInForce`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum PlaceOsoTimeInForce {
    /// Provider value `Day`.
    Day,
    /// Provider value `FOK`.
    Fok,
    /// Provider value `GTC`.
    Gtc,
    /// Provider value `GTD`.
    Gtd,
    /// Provider value `IOC`.
    Ioc,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl PlaceOsoTimeInForce {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Day => "Day",
            Self::Fok => "FOK",
            Self::Gtc => "GTC",
            Self::Gtd => "GTD",
            Self::Ioc => "IOC",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for PlaceOsoTimeInForce {
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

impl<'de> serde::Deserialize<'de> for PlaceOsoTimeInForce {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "Day" => Self::Day,
            "FOK" => Self::Fok,
            "GTC" => Self::Gtc,
            "GTD" => Self::Gtd,
            "IOC" => Self::Ioc,
            _ => Self::Unknown(value),
        })
    }
}

/// Current wire model `RestrainedOrderVersion`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct RestrainedOrderVersion {
    #[serde(rename = "action")]
    action: RestrainedOrderVersionAction,
    #[serde(rename = "clOrdId", default, skip_serializing_if = "Option::is_none")]
    cl_ord_id: Option<crate::ClientOrderId>,
    #[serde(rename = "orderType")]
    order_type: RestrainedOrderVersionOrderType,
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "crate::decimal::option")]
    price: Option<crate::Decimal>,
    #[serde(rename = "stopPrice", default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "crate::decimal::option")]
    stop_price: Option<crate::Decimal>,
    #[serde(
        rename = "limitIfTouchedPrice",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    limit_if_touched_price: Option<crate::Decimal>,
    #[serde(rename = "maxShow", default, skip_serializing_if = "Option::is_none")]
    max_show: Option<i64>,
    #[serde(
        rename = "pegDifference",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    peg_difference: Option<crate::Decimal>,
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    time_in_force: Option<RestrainedOrderVersionTimeInForce>,
    #[serde(
        rename = "expireTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    expire_time: Option<jiff::Timestamp>,
    #[serde(rename = "text", default, skip_serializing_if = "Option::is_none")]
    text: Option<String>,
}

impl RestrainedOrderVersion {
    /// Returns wire field `action`.
    #[must_use]
    pub fn action(&self) -> &RestrainedOrderVersionAction {
        &self.action
    }

    /// Returns wire field `clOrdId`.
    #[must_use]
    pub fn cl_ord_id(&self) -> Option<&crate::ClientOrderId> {
        self.cl_ord_id.as_ref()
    }

    /// Returns wire field `orderType`.
    #[must_use]
    pub fn order_type(&self) -> &RestrainedOrderVersionOrderType {
        &self.order_type
    }

    /// Returns wire field `price`.
    #[must_use]
    pub fn price(&self) -> Option<&crate::Decimal> {
        self.price.as_ref()
    }

    /// Returns wire field `stopPrice`.
    #[must_use]
    pub fn stop_price(&self) -> Option<&crate::Decimal> {
        self.stop_price.as_ref()
    }

    /// Returns wire field `limitIfTouchedPrice`.
    #[must_use]
    pub fn limit_if_touched_price(&self) -> Option<&crate::Decimal> {
        self.limit_if_touched_price.as_ref()
    }

    /// Returns wire field `maxShow`.
    #[must_use]
    pub fn max_show(&self) -> Option<&i64> {
        self.max_show.as_ref()
    }

    /// Returns wire field `pegDifference`.
    #[must_use]
    pub fn peg_difference(&self) -> Option<&crate::Decimal> {
        self.peg_difference.as_ref()
    }

    /// Returns wire field `timeInForce`.
    #[must_use]
    pub fn time_in_force(&self) -> Option<&RestrainedOrderVersionTimeInForce> {
        self.time_in_force.as_ref()
    }

    /// Returns wire field `expireTime`.
    #[must_use]
    pub fn expire_time(&self) -> Option<&jiff::Timestamp> {
        self.expire_time.as_ref()
    }

    /// Returns wire field `text`.
    #[must_use]
    pub fn text(&self) -> Option<&str> {
        self.text.as_deref()
    }

    /// Starts a builder for [`RestrainedOrderVersion`].
    pub fn builder() -> RestrainedOrderVersionBuilder {
        RestrainedOrderVersionBuilder::default()
    }
}

/// Builder for [`RestrainedOrderVersion`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct RestrainedOrderVersionBuilder {
    action: Option<RestrainedOrderVersionAction>,
    cl_ord_id: Option<crate::ClientOrderId>,
    order_type: Option<RestrainedOrderVersionOrderType>,
    price: Option<crate::Decimal>,
    stop_price: Option<crate::Decimal>,
    limit_if_touched_price: Option<crate::Decimal>,
    max_show: Option<i64>,
    peg_difference: Option<crate::Decimal>,
    time_in_force: Option<RestrainedOrderVersionTimeInForce>,
    expire_time: Option<jiff::Timestamp>,
    text: Option<String>,
}

impl RestrainedOrderVersionBuilder {
    /// Sets wire field `action`.
    pub fn action(mut self, value: RestrainedOrderVersionAction) -> Self {
        self.action = Some(value);
        self
    }

    /// Sets wire field `clOrdId`.
    pub fn cl_ord_id(mut self, value: crate::ClientOrderId) -> Self {
        self.cl_ord_id = Some(value);
        self
    }

    /// Sets wire field `orderType`.
    pub fn order_type(mut self, value: RestrainedOrderVersionOrderType) -> Self {
        self.order_type = Some(value);
        self
    }

    /// Sets wire field `price`.
    pub fn price(mut self, value: crate::Decimal) -> Self {
        self.price = Some(value);
        self
    }

    /// Sets wire field `stopPrice`.
    pub fn stop_price(mut self, value: crate::Decimal) -> Self {
        self.stop_price = Some(value);
        self
    }

    /// Sets wire field `limitIfTouchedPrice`.
    pub fn limit_if_touched_price(mut self, value: crate::Decimal) -> Self {
        self.limit_if_touched_price = Some(value);
        self
    }

    /// Sets wire field `maxShow`.
    pub fn max_show(mut self, value: i64) -> Self {
        self.max_show = Some(value);
        self
    }

    /// Sets wire field `pegDifference`.
    pub fn peg_difference(mut self, value: crate::Decimal) -> Self {
        self.peg_difference = Some(value);
        self
    }

    /// Sets wire field `timeInForce`.
    pub fn time_in_force(mut self, value: RestrainedOrderVersionTimeInForce) -> Self {
        self.time_in_force = Some(value);
        self
    }

    /// Sets wire field `expireTime`.
    pub fn expire_time(mut self, value: jiff::Timestamp) -> Self {
        self.expire_time = Some(value);
        self
    }

    /// Sets wire field `text`.
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Validates required fields and builds [`RestrainedOrderVersion`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<RestrainedOrderVersion, crate::api::current::BuildError> {
        let action = self
            .action
            .ok_or(crate::api::current::BuildError::missing("action"))?;
        let order_type = self
            .order_type
            .ok_or(crate::api::current::BuildError::missing("orderType"))?;
        Ok(RestrainedOrderVersion {
            action,
            cl_ord_id: self.cl_ord_id,
            order_type,
            price: self.price,
            stop_price: self.stop_price,
            limit_if_touched_price: self.limit_if_touched_price,
            max_show: self.max_show,
            peg_difference: self.peg_difference,
            time_in_force: self.time_in_force,
            expire_time: self.expire_time,
            text: self.text,
        })
    }
}

/// Current provider values for `RestrainedOrderVersionAction`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum RestrainedOrderVersionAction {
    /// Provider value `Buy`.
    Buy,
    /// Provider value `Sell`.
    Sell,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl RestrainedOrderVersionAction {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Buy => "Buy",
            Self::Sell => "Sell",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for RestrainedOrderVersionAction {
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

impl<'de> serde::Deserialize<'de> for RestrainedOrderVersionAction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "Buy" => Self::Buy,
            "Sell" => Self::Sell,
            _ => Self::Unknown(value),
        })
    }
}

/// Current provider values for `RestrainedOrderVersionOrderType`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum RestrainedOrderVersionOrderType {
    /// Provider value `Limit`.
    Limit,
    /// Provider value `LimitIfTouched`.
    LimitIfTouched,
    /// Provider value `MIT`.
    Mit,
    /// Provider value `Market`.
    Market,
    /// Provider value `QTS`.
    Qts,
    /// Provider value `Stop`.
    Stop,
    /// Provider value `StopLimit`.
    StopLimit,
    /// Provider value `TrailingStop`.
    TrailingStop,
    /// Provider value `TrailingStopLimit`.
    TrailingStopLimit,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl RestrainedOrderVersionOrderType {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Limit => "Limit",
            Self::LimitIfTouched => "LimitIfTouched",
            Self::Mit => "MIT",
            Self::Market => "Market",
            Self::Qts => "QTS",
            Self::Stop => "Stop",
            Self::StopLimit => "StopLimit",
            Self::TrailingStop => "TrailingStop",
            Self::TrailingStopLimit => "TrailingStopLimit",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for RestrainedOrderVersionOrderType {
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

impl<'de> serde::Deserialize<'de> for RestrainedOrderVersionOrderType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "Limit" => Self::Limit,
            "LimitIfTouched" => Self::LimitIfTouched,
            "MIT" => Self::Mit,
            "Market" => Self::Market,
            "QTS" => Self::Qts,
            "Stop" => Self::Stop,
            "StopLimit" => Self::StopLimit,
            "TrailingStop" => Self::TrailingStop,
            "TrailingStopLimit" => Self::TrailingStopLimit,
            _ => Self::Unknown(value),
        })
    }
}

/// Current provider values for `RestrainedOrderVersionTimeInForce`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum RestrainedOrderVersionTimeInForce {
    /// Provider value `Day`.
    Day,
    /// Provider value `FOK`.
    Fok,
    /// Provider value `GTC`.
    Gtc,
    /// Provider value `GTD`.
    Gtd,
    /// Provider value `IOC`.
    Ioc,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl RestrainedOrderVersionTimeInForce {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Day => "Day",
            Self::Fok => "FOK",
            Self::Gtc => "GTC",
            Self::Gtd => "GTD",
            Self::Ioc => "IOC",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for RestrainedOrderVersionTimeInForce {
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

impl<'de> serde::Deserialize<'de> for RestrainedOrderVersionTimeInForce {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "Day" => Self::Day,
            "FOK" => Self::Fok,
            "GTC" => Self::Gtc,
            "GTD" => Self::Gtd,
            "IOC" => Self::Ioc,
            _ => Self::Unknown(value),
        })
    }
}

/// Current wire model `RiskEvaluationDetails`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct RiskEvaluationDetails {
    #[serde(rename = "beforeInitialMarginReq")]
    #[serde(with = "crate::decimal")]
    before_initial_margin_req: crate::Decimal,
    #[serde(rename = "beforeInitialFullMarginReq")]
    #[serde(with = "crate::decimal")]
    before_initial_full_margin_req: crate::Decimal,
    #[serde(rename = "beforeOpenCollateralReq")]
    #[serde(with = "crate::decimal")]
    before_open_collateral_req: crate::Decimal,
    #[serde(rename = "beforeNetPos")]
    before_net_pos: i64,
    #[serde(rename = "isExitOrder")]
    is_exit_order: bool,
    #[serde(rename = "initialMarginReq")]
    #[serde(with = "crate::decimal")]
    initial_margin_req: crate::Decimal,
    #[serde(rename = "initialFullMarginReq")]
    #[serde(with = "crate::decimal")]
    initial_full_margin_req: crate::Decimal,
    #[serde(rename = "openCollateralReq")]
    #[serde(with = "crate::decimal")]
    open_collateral_req: crate::Decimal,
    #[serde(rename = "totalCash")]
    #[serde(with = "crate::decimal")]
    total_cash: crate::Decimal,
    #[serde(rename = "futuresOpenPnL")]
    #[serde(with = "crate::decimal")]
    futures_open_pn_l: crate::Decimal,
    #[serde(rename = "optionsOpenPnL")]
    #[serde(with = "crate::decimal")]
    options_open_pn_l: crate::Decimal,
    #[serde(rename = "riskCredit")]
    #[serde(with = "crate::decimal")]
    risk_credit: crate::Decimal,
    #[serde(rename = "netLiqValue")]
    #[serde(with = "crate::decimal")]
    net_liq_value: crate::Decimal,
    #[serde(rename = "futureCommAndFees")]
    #[serde(with = "crate::decimal")]
    future_comm_and_fees: crate::Decimal,
    #[serde(rename = "totalCashRequired")]
    #[serde(with = "crate::decimal")]
    total_cash_required: crate::Decimal,
    #[serde(rename = "excess")]
    #[serde(with = "crate::decimal")]
    excess: crate::Decimal,
    #[serde(rename = "hypoLongPos")]
    hypo_long_pos: i64,
    #[serde(rename = "hypoShortPos")]
    hypo_short_pos: i64,
}

impl RiskEvaluationDetails {
    /// Returns wire field `beforeInitialMarginReq`.
    #[must_use]
    pub fn before_initial_margin_req(&self) -> &crate::Decimal {
        &self.before_initial_margin_req
    }

    /// Returns wire field `beforeInitialFullMarginReq`.
    #[must_use]
    pub fn before_initial_full_margin_req(&self) -> &crate::Decimal {
        &self.before_initial_full_margin_req
    }

    /// Returns wire field `beforeOpenCollateralReq`.
    #[must_use]
    pub fn before_open_collateral_req(&self) -> &crate::Decimal {
        &self.before_open_collateral_req
    }

    /// Returns wire field `beforeNetPos`.
    #[must_use]
    pub fn before_net_pos(&self) -> &i64 {
        &self.before_net_pos
    }

    /// Returns wire field `isExitOrder`.
    #[must_use]
    pub fn is_exit_order(&self) -> &bool {
        &self.is_exit_order
    }

    /// Returns wire field `initialMarginReq`.
    #[must_use]
    pub fn initial_margin_req(&self) -> &crate::Decimal {
        &self.initial_margin_req
    }

    /// Returns wire field `initialFullMarginReq`.
    #[must_use]
    pub fn initial_full_margin_req(&self) -> &crate::Decimal {
        &self.initial_full_margin_req
    }

    /// Returns wire field `openCollateralReq`.
    #[must_use]
    pub fn open_collateral_req(&self) -> &crate::Decimal {
        &self.open_collateral_req
    }

    /// Returns wire field `totalCash`.
    #[must_use]
    pub fn total_cash(&self) -> &crate::Decimal {
        &self.total_cash
    }

    /// Returns wire field `futuresOpenPnL`.
    #[must_use]
    pub fn futures_open_pn_l(&self) -> &crate::Decimal {
        &self.futures_open_pn_l
    }

    /// Returns wire field `optionsOpenPnL`.
    #[must_use]
    pub fn options_open_pn_l(&self) -> &crate::Decimal {
        &self.options_open_pn_l
    }

    /// Returns wire field `riskCredit`.
    #[must_use]
    pub fn risk_credit(&self) -> &crate::Decimal {
        &self.risk_credit
    }

    /// Returns wire field `netLiqValue`.
    #[must_use]
    pub fn net_liq_value(&self) -> &crate::Decimal {
        &self.net_liq_value
    }

    /// Returns wire field `futureCommAndFees`.
    #[must_use]
    pub fn future_comm_and_fees(&self) -> &crate::Decimal {
        &self.future_comm_and_fees
    }

    /// Returns wire field `totalCashRequired`.
    #[must_use]
    pub fn total_cash_required(&self) -> &crate::Decimal {
        &self.total_cash_required
    }

    /// Returns wire field `excess`.
    #[must_use]
    pub fn excess(&self) -> &crate::Decimal {
        &self.excess
    }

    /// Returns wire field `hypoLongPos`.
    #[must_use]
    pub fn hypo_long_pos(&self) -> &i64 {
        &self.hypo_long_pos
    }

    /// Returns wire field `hypoShortPos`.
    #[must_use]
    pub fn hypo_short_pos(&self) -> &i64 {
        &self.hypo_short_pos
    }

    /// Starts a builder for [`RiskEvaluationDetails`].
    pub fn builder() -> RiskEvaluationDetailsBuilder {
        RiskEvaluationDetailsBuilder::default()
    }
}

/// Builder for [`RiskEvaluationDetails`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct RiskEvaluationDetailsBuilder {
    before_initial_margin_req: Option<crate::Decimal>,
    before_initial_full_margin_req: Option<crate::Decimal>,
    before_open_collateral_req: Option<crate::Decimal>,
    before_net_pos: Option<i64>,
    is_exit_order: Option<bool>,
    initial_margin_req: Option<crate::Decimal>,
    initial_full_margin_req: Option<crate::Decimal>,
    open_collateral_req: Option<crate::Decimal>,
    total_cash: Option<crate::Decimal>,
    futures_open_pn_l: Option<crate::Decimal>,
    options_open_pn_l: Option<crate::Decimal>,
    risk_credit: Option<crate::Decimal>,
    net_liq_value: Option<crate::Decimal>,
    future_comm_and_fees: Option<crate::Decimal>,
    total_cash_required: Option<crate::Decimal>,
    excess: Option<crate::Decimal>,
    hypo_long_pos: Option<i64>,
    hypo_short_pos: Option<i64>,
}

impl RiskEvaluationDetailsBuilder {
    /// Sets wire field `beforeInitialMarginReq`.
    pub fn before_initial_margin_req(mut self, value: crate::Decimal) -> Self {
        self.before_initial_margin_req = Some(value);
        self
    }

    /// Sets wire field `beforeInitialFullMarginReq`.
    pub fn before_initial_full_margin_req(mut self, value: crate::Decimal) -> Self {
        self.before_initial_full_margin_req = Some(value);
        self
    }

    /// Sets wire field `beforeOpenCollateralReq`.
    pub fn before_open_collateral_req(mut self, value: crate::Decimal) -> Self {
        self.before_open_collateral_req = Some(value);
        self
    }

    /// Sets wire field `beforeNetPos`.
    pub fn before_net_pos(mut self, value: i64) -> Self {
        self.before_net_pos = Some(value);
        self
    }

    /// Sets wire field `isExitOrder`.
    pub fn is_exit_order(mut self, value: bool) -> Self {
        self.is_exit_order = Some(value);
        self
    }

    /// Sets wire field `initialMarginReq`.
    pub fn initial_margin_req(mut self, value: crate::Decimal) -> Self {
        self.initial_margin_req = Some(value);
        self
    }

    /// Sets wire field `initialFullMarginReq`.
    pub fn initial_full_margin_req(mut self, value: crate::Decimal) -> Self {
        self.initial_full_margin_req = Some(value);
        self
    }

    /// Sets wire field `openCollateralReq`.
    pub fn open_collateral_req(mut self, value: crate::Decimal) -> Self {
        self.open_collateral_req = Some(value);
        self
    }

    /// Sets wire field `totalCash`.
    pub fn total_cash(mut self, value: crate::Decimal) -> Self {
        self.total_cash = Some(value);
        self
    }

    /// Sets wire field `futuresOpenPnL`.
    pub fn futures_open_pn_l(mut self, value: crate::Decimal) -> Self {
        self.futures_open_pn_l = Some(value);
        self
    }

    /// Sets wire field `optionsOpenPnL`.
    pub fn options_open_pn_l(mut self, value: crate::Decimal) -> Self {
        self.options_open_pn_l = Some(value);
        self
    }

    /// Sets wire field `riskCredit`.
    pub fn risk_credit(mut self, value: crate::Decimal) -> Self {
        self.risk_credit = Some(value);
        self
    }

    /// Sets wire field `netLiqValue`.
    pub fn net_liq_value(mut self, value: crate::Decimal) -> Self {
        self.net_liq_value = Some(value);
        self
    }

    /// Sets wire field `futureCommAndFees`.
    pub fn future_comm_and_fees(mut self, value: crate::Decimal) -> Self {
        self.future_comm_and_fees = Some(value);
        self
    }

    /// Sets wire field `totalCashRequired`.
    pub fn total_cash_required(mut self, value: crate::Decimal) -> Self {
        self.total_cash_required = Some(value);
        self
    }

    /// Sets wire field `excess`.
    pub fn excess(mut self, value: crate::Decimal) -> Self {
        self.excess = Some(value);
        self
    }

    /// Sets wire field `hypoLongPos`.
    pub fn hypo_long_pos(mut self, value: i64) -> Self {
        self.hypo_long_pos = Some(value);
        self
    }

    /// Sets wire field `hypoShortPos`.
    pub fn hypo_short_pos(mut self, value: i64) -> Self {
        self.hypo_short_pos = Some(value);
        self
    }

    /// Validates required fields and builds [`RiskEvaluationDetails`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<RiskEvaluationDetails, crate::api::current::BuildError> {
        let before_initial_margin_req =
            self.before_initial_margin_req
                .ok_or(crate::api::current::BuildError::missing(
                    "beforeInitialMarginReq",
                ))?;
        let before_initial_full_margin_req =
            self.before_initial_full_margin_req
                .ok_or(crate::api::current::BuildError::missing(
                    "beforeInitialFullMarginReq",
                ))?;
        let before_open_collateral_req =
            self.before_open_collateral_req
                .ok_or(crate::api::current::BuildError::missing(
                    "beforeOpenCollateralReq",
                ))?;
        let before_net_pos = self
            .before_net_pos
            .ok_or(crate::api::current::BuildError::missing("beforeNetPos"))?;
        let is_exit_order = self
            .is_exit_order
            .ok_or(crate::api::current::BuildError::missing("isExitOrder"))?;
        let initial_margin_req = self
            .initial_margin_req
            .ok_or(crate::api::current::BuildError::missing("initialMarginReq"))?;
        let initial_full_margin_req =
            self.initial_full_margin_req
                .ok_or(crate::api::current::BuildError::missing(
                    "initialFullMarginReq",
                ))?;
        let open_collateral_req =
            self.open_collateral_req
                .ok_or(crate::api::current::BuildError::missing(
                    "openCollateralReq",
                ))?;
        let total_cash = self
            .total_cash
            .ok_or(crate::api::current::BuildError::missing("totalCash"))?;
        let futures_open_pn_l = self
            .futures_open_pn_l
            .ok_or(crate::api::current::BuildError::missing("futuresOpenPnL"))?;
        let options_open_pn_l = self
            .options_open_pn_l
            .ok_or(crate::api::current::BuildError::missing("optionsOpenPnL"))?;
        let risk_credit = self
            .risk_credit
            .ok_or(crate::api::current::BuildError::missing("riskCredit"))?;
        let net_liq_value = self
            .net_liq_value
            .ok_or(crate::api::current::BuildError::missing("netLiqValue"))?;
        let future_comm_and_fees =
            self.future_comm_and_fees
                .ok_or(crate::api::current::BuildError::missing(
                    "futureCommAndFees",
                ))?;
        let total_cash_required =
            self.total_cash_required
                .ok_or(crate::api::current::BuildError::missing(
                    "totalCashRequired",
                ))?;
        let excess = self
            .excess
            .ok_or(crate::api::current::BuildError::missing("excess"))?;
        let hypo_long_pos = self
            .hypo_long_pos
            .ok_or(crate::api::current::BuildError::missing("hypoLongPos"))?;
        let hypo_short_pos = self
            .hypo_short_pos
            .ok_or(crate::api::current::BuildError::missing("hypoShortPos"))?;
        Ok(RiskEvaluationDetails {
            before_initial_margin_req,
            before_initial_full_margin_req,
            before_open_collateral_req,
            before_net_pos,
            is_exit_order,
            initial_margin_req,
            initial_full_margin_req,
            open_collateral_req,
            total_cash,
            futures_open_pn_l,
            options_open_pn_l,
            risk_credit,
            net_liq_value,
            future_comm_and_fees,
            total_cash_required,
            excess,
            hypo_long_pos,
            hypo_short_pos,
        })
    }
}

/// Current wire model `StartOrderStrategy`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct StartOrderStrategy {
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    account_id: Option<crate::AccountId>,
    #[serde(
        rename = "accountSpec",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    account_spec: Option<crate::AccountSpec>,
    #[serde(rename = "symbol")]
    symbol: crate::Symbol,
    #[serde(rename = "orderStrategyTypeId")]
    order_strategy_type_id: super::ids::OrderStrategyTypeId,
    #[serde(rename = "action")]
    action: StartOrderStrategyAction,
    #[serde(rename = "params", default, skip_serializing_if = "Option::is_none")]
    params: Option<String>,
    #[serde(rename = "uuid", default, skip_serializing_if = "Option::is_none")]
    uuid: Option<String>,
    #[serde(
        rename = "customTag50",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    custom_tag50: Option<String>,
    #[serde(
        rename = "isAutomated",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    is_automated: Option<bool>,
}

impl StartOrderStrategy {
    /// Returns wire field `accountId`.
    #[must_use]
    pub fn account_id(&self) -> Option<&crate::AccountId> {
        self.account_id.as_ref()
    }

    /// Returns wire field `accountSpec`.
    #[must_use]
    pub fn account_spec(&self) -> Option<&crate::AccountSpec> {
        self.account_spec.as_ref()
    }

    /// Returns wire field `symbol`.
    #[must_use]
    pub fn symbol(&self) -> &crate::Symbol {
        &self.symbol
    }

    /// Returns wire field `orderStrategyTypeId`.
    #[must_use]
    pub fn order_strategy_type_id(&self) -> &super::ids::OrderStrategyTypeId {
        &self.order_strategy_type_id
    }

    /// Returns wire field `action`.
    #[must_use]
    pub fn action(&self) -> &StartOrderStrategyAction {
        &self.action
    }

    /// Returns wire field `params`.
    #[must_use]
    pub fn params(&self) -> Option<&str> {
        self.params.as_deref()
    }

    /// Returns wire field `uuid`.
    #[must_use]
    pub fn uuid(&self) -> Option<&str> {
        self.uuid.as_deref()
    }

    /// Returns wire field `customTag50`.
    #[must_use]
    pub fn custom_tag50(&self) -> Option<&str> {
        self.custom_tag50.as_deref()
    }

    /// Returns wire field `isAutomated`.
    #[must_use]
    pub fn is_automated(&self) -> Option<&bool> {
        self.is_automated.as_ref()
    }

    /// Starts a builder for [`StartOrderStrategy`].
    pub fn builder() -> StartOrderStrategyBuilder {
        StartOrderStrategyBuilder::default()
    }
}

/// Builder for [`StartOrderStrategy`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct StartOrderStrategyBuilder {
    account_id: Option<crate::AccountId>,
    account_spec: Option<crate::AccountSpec>,
    symbol: Option<crate::Symbol>,
    order_strategy_type_id: Option<super::ids::OrderStrategyTypeId>,
    action: Option<StartOrderStrategyAction>,
    params: Option<String>,
    uuid: Option<String>,
    custom_tag50: Option<String>,
    is_automated: Option<bool>,
}

impl StartOrderStrategyBuilder {
    /// Sets wire field `accountId`.
    pub fn account_id(mut self, value: crate::AccountId) -> Self {
        self.account_id = Some(value);
        self
    }

    /// Sets wire field `accountSpec`.
    pub fn account_spec(mut self, value: crate::AccountSpec) -> Self {
        self.account_spec = Some(value);
        self
    }

    /// Sets wire field `symbol`.
    pub fn symbol(mut self, value: crate::Symbol) -> Self {
        self.symbol = Some(value);
        self
    }

    /// Sets wire field `orderStrategyTypeId`.
    pub fn order_strategy_type_id(mut self, value: super::ids::OrderStrategyTypeId) -> Self {
        self.order_strategy_type_id = Some(value);
        self
    }

    /// Sets wire field `action`.
    pub fn action(mut self, value: StartOrderStrategyAction) -> Self {
        self.action = Some(value);
        self
    }

    /// Sets wire field `params`.
    pub fn params(mut self, value: impl Into<String>) -> Self {
        self.params = Some(value.into());
        self
    }

    /// Sets wire field `uuid`.
    pub fn uuid(mut self, value: impl Into<String>) -> Self {
        self.uuid = Some(value.into());
        self
    }

    /// Sets wire field `customTag50`.
    pub fn custom_tag50(mut self, value: impl Into<String>) -> Self {
        self.custom_tag50 = Some(value.into());
        self
    }

    /// Sets wire field `isAutomated`.
    pub fn is_automated(mut self, value: bool) -> Self {
        self.is_automated = Some(value);
        self
    }

    /// Validates required fields and builds [`StartOrderStrategy`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<StartOrderStrategy, crate::api::current::BuildError> {
        let symbol = self
            .symbol
            .ok_or(crate::api::current::BuildError::missing("symbol"))?;
        let order_strategy_type_id =
            self.order_strategy_type_id
                .ok_or(crate::api::current::BuildError::missing(
                    "orderStrategyTypeId",
                ))?;
        let action = self
            .action
            .ok_or(crate::api::current::BuildError::missing("action"))?;
        Ok(StartOrderStrategy {
            account_id: self.account_id,
            account_spec: self.account_spec,
            symbol,
            order_strategy_type_id,
            action,
            params: self.params,
            uuid: self.uuid,
            custom_tag50: self.custom_tag50,
            is_automated: self.is_automated,
        })
    }
}

impl crate::api::current::support::CurrentRequest for StartOrderStrategy {
    fn validate_current(&self) -> Result<(), crate::Error> {
        Ok(())
    }
}

/// Current provider values for `StartOrderStrategyAction`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum StartOrderStrategyAction {
    /// Provider value `Buy`.
    Buy,
    /// Provider value `Sell`.
    Sell,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl StartOrderStrategyAction {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Buy => "Buy",
            Self::Sell => "Sell",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for StartOrderStrategyAction {
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

impl<'de> serde::Deserialize<'de> for StartOrderStrategyAction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "Buy" => Self::Buy,
            "Sell" => Self::Sell,
            _ => Self::Unknown(value),
        })
    }
}

/// Typed query parameters for `/command/deps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct CommandDependentsQuery {
    #[serde(rename = "masterid")]
    masterid: super::ids::ProviderEntityId,
}

impl CommandDependentsQuery {
    /// Returns wire field `masterid`.
    #[must_use]
    pub fn masterid(&self) -> &super::ids::ProviderEntityId {
        &self.masterid
    }

    /// Starts a builder for [`CommandDependentsQuery`].
    pub fn builder() -> CommandDependentsQueryBuilder {
        CommandDependentsQueryBuilder::default()
    }
}

/// Builder for [`CommandDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct CommandDependentsQueryBuilder {
    masterid: Option<super::ids::ProviderEntityId>,
}

impl CommandDependentsQueryBuilder {
    /// Sets wire field `masterid`.
    pub fn masterid(mut self, value: super::ids::ProviderEntityId) -> Self {
        self.masterid = Some(value);
        self
    }

    /// Validates required fields and builds [`CommandDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<CommandDependentsQuery, crate::api::current::BuildError> {
        let masterid = self
            .masterid
            .ok_or(crate::api::current::BuildError::missing("masterid"))?;
        Ok(CommandDependentsQuery { masterid })
    }
}

impl crate::api::current::support::CurrentQuery for CommandDependentsQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "masterid", &self.masterid)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /command/deps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn command_dependents(
        &self,
        query: &CommandDependentsQuery,
    ) -> Result<Vec<super::users::Command>, crate::Error> {
        self.get_current("/command/deps", query).await
    }
}

/// Typed query parameters for `/command/item`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct CommandItemQuery {
    #[serde(rename = "id")]
    id: crate::CommandId,
}

impl CommandItemQuery {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> &crate::CommandId {
        &self.id
    }

    /// Starts a builder for [`CommandItemQuery`].
    pub fn builder() -> CommandItemQueryBuilder {
        CommandItemQueryBuilder::default()
    }
}

/// Builder for [`CommandItemQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct CommandItemQueryBuilder {
    id: Option<crate::CommandId>,
}

impl CommandItemQueryBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: crate::CommandId) -> Self {
        self.id = Some(value);
        self
    }

    /// Validates required fields and builds [`CommandItemQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<CommandItemQuery, crate::api::current::BuildError> {
        let id = self
            .id
            .ok_or(crate::api::current::BuildError::missing("id"))?;
        Ok(CommandItemQuery { id })
    }
}

impl crate::api::current::support::CurrentQuery for CommandItemQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "id", &self.id)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /command/item` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn command_item(
        &self,
        query: &CommandItemQuery,
    ) -> Result<super::users::Command, crate::Error> {
        self.get_current("/command/item", query).await
    }
}

/// Typed query parameters for `/command/items`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct CommandItemsQuery {
    #[serde(rename = "ids")]
    ids: Vec<crate::CommandId>,
}

impl CommandItemsQuery {
    /// Returns wire field `ids`.
    #[must_use]
    pub fn ids(&self) -> &[crate::CommandId] {
        &self.ids
    }

    /// Starts a builder for [`CommandItemsQuery`].
    pub fn builder() -> CommandItemsQueryBuilder {
        CommandItemsQueryBuilder::default()
    }
}

/// Builder for [`CommandItemsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct CommandItemsQueryBuilder {
    ids: Option<Vec<crate::CommandId>>,
}

impl CommandItemsQueryBuilder {
    /// Sets wire field `ids`.
    pub fn ids(mut self, value: Vec<crate::CommandId>) -> Self {
        self.ids = Some(value);
        self
    }

    /// Validates required fields and builds [`CommandItemsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<CommandItemsQuery, crate::api::current::BuildError> {
        let ids = self
            .ids
            .ok_or(crate::api::current::BuildError::missing("ids"))?;
        if ids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "ids",
                "must not be empty",
            ));
        }
        Ok(CommandItemsQuery { ids })
    }
}

impl crate::api::current::support::CurrentQuery for CommandItemsQuery {
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
    /// Calls the current `GET /command/items` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn command_items(
        &self,
        query: &CommandItemsQuery,
    ) -> Result<Vec<super::users::Command>, crate::Error> {
        self.get_current("/command/items", query).await
    }
}

/// Typed query parameters for `/command/ldeps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct CommandLDependentsQuery {
    #[serde(rename = "masterids")]
    masterids: Vec<super::ids::ProviderEntityId>,
}

impl CommandLDependentsQuery {
    /// Returns wire field `masterids`.
    #[must_use]
    pub fn masterids(&self) -> &[super::ids::ProviderEntityId] {
        &self.masterids
    }

    /// Starts a builder for [`CommandLDependentsQuery`].
    pub fn builder() -> CommandLDependentsQueryBuilder {
        CommandLDependentsQueryBuilder::default()
    }
}

/// Builder for [`CommandLDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct CommandLDependentsQueryBuilder {
    masterids: Option<Vec<super::ids::ProviderEntityId>>,
}

impl CommandLDependentsQueryBuilder {
    /// Sets wire field `masterids`.
    pub fn masterids(mut self, value: Vec<super::ids::ProviderEntityId>) -> Self {
        self.masterids = Some(value);
        self
    }

    /// Validates required fields and builds [`CommandLDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<CommandLDependentsQuery, crate::api::current::BuildError> {
        let masterids = self
            .masterids
            .ok_or(crate::api::current::BuildError::missing("masterids"))?;
        if masterids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "masterids",
                "must not be empty",
            ));
        }
        Ok(CommandLDependentsQuery { masterids })
    }
}

impl crate::api::current::support::CurrentQuery for CommandLDependentsQuery {
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
    /// Calls the current `GET /command/ldeps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn command_l_dependents(
        &self,
        query: &CommandLDependentsQuery,
    ) -> Result<Vec<super::users::Command>, crate::Error> {
        self.get_current("/command/ldeps", query).await
    }
}

impl crate::Client {
    /// Calls the current `GET /command/list` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn command_list(&self) -> Result<Vec<super::users::Command>, crate::Error> {
        self.get_without_query("/command/list").await
    }
}

/// Typed query parameters for `/commandReport/deps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct CommandReportDependentsQuery {
    #[serde(rename = "masterid")]
    masterid: super::ids::ProviderEntityId,
}

impl CommandReportDependentsQuery {
    /// Returns wire field `masterid`.
    #[must_use]
    pub fn masterid(&self) -> &super::ids::ProviderEntityId {
        &self.masterid
    }

    /// Starts a builder for [`CommandReportDependentsQuery`].
    pub fn builder() -> CommandReportDependentsQueryBuilder {
        CommandReportDependentsQueryBuilder::default()
    }
}

/// Builder for [`CommandReportDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct CommandReportDependentsQueryBuilder {
    masterid: Option<super::ids::ProviderEntityId>,
}

impl CommandReportDependentsQueryBuilder {
    /// Sets wire field `masterid`.
    pub fn masterid(mut self, value: super::ids::ProviderEntityId) -> Self {
        self.masterid = Some(value);
        self
    }

    /// Validates required fields and builds [`CommandReportDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<CommandReportDependentsQuery, crate::api::current::BuildError> {
        let masterid = self
            .masterid
            .ok_or(crate::api::current::BuildError::missing("masterid"))?;
        Ok(CommandReportDependentsQuery { masterid })
    }
}

impl crate::api::current::support::CurrentQuery for CommandReportDependentsQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "masterid", &self.masterid)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /commandReport/deps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn command_report_dependents(
        &self,
        query: &CommandReportDependentsQuery,
    ) -> Result<Vec<super::users::CommandReport>, crate::Error> {
        self.get_current("/commandReport/deps", query).await
    }
}

/// Typed query parameters for `/commandReport/item`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct CommandReportItemQuery {
    #[serde(rename = "id")]
    id: super::ids::CommandReportId,
}

impl CommandReportItemQuery {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> &super::ids::CommandReportId {
        &self.id
    }

    /// Starts a builder for [`CommandReportItemQuery`].
    pub fn builder() -> CommandReportItemQueryBuilder {
        CommandReportItemQueryBuilder::default()
    }
}

/// Builder for [`CommandReportItemQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct CommandReportItemQueryBuilder {
    id: Option<super::ids::CommandReportId>,
}

impl CommandReportItemQueryBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::CommandReportId) -> Self {
        self.id = Some(value);
        self
    }

    /// Validates required fields and builds [`CommandReportItemQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<CommandReportItemQuery, crate::api::current::BuildError> {
        let id = self
            .id
            .ok_or(crate::api::current::BuildError::missing("id"))?;
        Ok(CommandReportItemQuery { id })
    }
}

impl crate::api::current::support::CurrentQuery for CommandReportItemQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "id", &self.id)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /commandReport/item` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn command_report_item(
        &self,
        query: &CommandReportItemQuery,
    ) -> Result<super::users::CommandReport, crate::Error> {
        self.get_current("/commandReport/item", query).await
    }
}

/// Typed query parameters for `/commandReport/items`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct CommandReportItemsQuery {
    #[serde(rename = "ids")]
    ids: Vec<super::ids::CommandReportId>,
}

impl CommandReportItemsQuery {
    /// Returns wire field `ids`.
    #[must_use]
    pub fn ids(&self) -> &[super::ids::CommandReportId] {
        &self.ids
    }

    /// Starts a builder for [`CommandReportItemsQuery`].
    pub fn builder() -> CommandReportItemsQueryBuilder {
        CommandReportItemsQueryBuilder::default()
    }
}

/// Builder for [`CommandReportItemsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct CommandReportItemsQueryBuilder {
    ids: Option<Vec<super::ids::CommandReportId>>,
}

impl CommandReportItemsQueryBuilder {
    /// Sets wire field `ids`.
    pub fn ids(mut self, value: Vec<super::ids::CommandReportId>) -> Self {
        self.ids = Some(value);
        self
    }

    /// Validates required fields and builds [`CommandReportItemsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<CommandReportItemsQuery, crate::api::current::BuildError> {
        let ids = self
            .ids
            .ok_or(crate::api::current::BuildError::missing("ids"))?;
        if ids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "ids",
                "must not be empty",
            ));
        }
        Ok(CommandReportItemsQuery { ids })
    }
}

impl crate::api::current::support::CurrentQuery for CommandReportItemsQuery {
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
    /// Calls the current `GET /commandReport/items` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn command_report_items(
        &self,
        query: &CommandReportItemsQuery,
    ) -> Result<Vec<super::users::CommandReport>, crate::Error> {
        self.get_current("/commandReport/items", query).await
    }
}

/// Typed query parameters for `/commandReport/ldeps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct CommandReportLDependentsQuery {
    #[serde(rename = "masterids")]
    masterids: Vec<super::ids::ProviderEntityId>,
}

impl CommandReportLDependentsQuery {
    /// Returns wire field `masterids`.
    #[must_use]
    pub fn masterids(&self) -> &[super::ids::ProviderEntityId] {
        &self.masterids
    }

    /// Starts a builder for [`CommandReportLDependentsQuery`].
    pub fn builder() -> CommandReportLDependentsQueryBuilder {
        CommandReportLDependentsQueryBuilder::default()
    }
}

/// Builder for [`CommandReportLDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct CommandReportLDependentsQueryBuilder {
    masterids: Option<Vec<super::ids::ProviderEntityId>>,
}

impl CommandReportLDependentsQueryBuilder {
    /// Sets wire field `masterids`.
    pub fn masterids(mut self, value: Vec<super::ids::ProviderEntityId>) -> Self {
        self.masterids = Some(value);
        self
    }

    /// Validates required fields and builds [`CommandReportLDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<CommandReportLDependentsQuery, crate::api::current::BuildError> {
        let masterids = self
            .masterids
            .ok_or(crate::api::current::BuildError::missing("masterids"))?;
        if masterids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "masterids",
                "must not be empty",
            ));
        }
        Ok(CommandReportLDependentsQuery { masterids })
    }
}

impl crate::api::current::support::CurrentQuery for CommandReportLDependentsQuery {
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
    /// Calls the current `GET /commandReport/ldeps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn command_report_l_dependents(
        &self,
        query: &CommandReportLDependentsQuery,
    ) -> Result<Vec<super::users::CommandReport>, crate::Error> {
        self.get_current("/commandReport/ldeps", query).await
    }
}

impl crate::Client {
    /// Calls the current `GET /commandReport/list` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn command_report_list(
        &self,
    ) -> Result<Vec<super::users::CommandReport>, crate::Error> {
        self.get_without_query("/commandReport/list").await
    }
}

/// Typed query parameters for `/executionReport/deps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ExecutionReportDependentsQuery {
    #[serde(rename = "masterid")]
    masterid: super::ids::ProviderEntityId,
}

impl ExecutionReportDependentsQuery {
    /// Returns wire field `masterid`.
    #[must_use]
    pub fn masterid(&self) -> &super::ids::ProviderEntityId {
        &self.masterid
    }

    /// Starts a builder for [`ExecutionReportDependentsQuery`].
    pub fn builder() -> ExecutionReportDependentsQueryBuilder {
        ExecutionReportDependentsQueryBuilder::default()
    }
}

/// Builder for [`ExecutionReportDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ExecutionReportDependentsQueryBuilder {
    masterid: Option<super::ids::ProviderEntityId>,
}

impl ExecutionReportDependentsQueryBuilder {
    /// Sets wire field `masterid`.
    pub fn masterid(mut self, value: super::ids::ProviderEntityId) -> Self {
        self.masterid = Some(value);
        self
    }

    /// Validates required fields and builds [`ExecutionReportDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ExecutionReportDependentsQuery, crate::api::current::BuildError> {
        let masterid = self
            .masterid
            .ok_or(crate::api::current::BuildError::missing("masterid"))?;
        Ok(ExecutionReportDependentsQuery { masterid })
    }
}

impl crate::api::current::support::CurrentQuery for ExecutionReportDependentsQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "masterid", &self.masterid)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /executionReport/deps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn execution_report_dependents(
        &self,
        query: &ExecutionReportDependentsQuery,
    ) -> Result<Vec<super::users::ExecutionReport>, crate::Error> {
        self.get_current("/executionReport/deps", query).await
    }
}

/// Typed query parameters for `/executionReport/item`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ExecutionReportItemQuery {
    #[serde(rename = "id")]
    id: super::ids::ExecutionReportId,
}

impl ExecutionReportItemQuery {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> &super::ids::ExecutionReportId {
        &self.id
    }

    /// Starts a builder for [`ExecutionReportItemQuery`].
    pub fn builder() -> ExecutionReportItemQueryBuilder {
        ExecutionReportItemQueryBuilder::default()
    }
}

/// Builder for [`ExecutionReportItemQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ExecutionReportItemQueryBuilder {
    id: Option<super::ids::ExecutionReportId>,
}

impl ExecutionReportItemQueryBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::ExecutionReportId) -> Self {
        self.id = Some(value);
        self
    }

    /// Validates required fields and builds [`ExecutionReportItemQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ExecutionReportItemQuery, crate::api::current::BuildError> {
        let id = self
            .id
            .ok_or(crate::api::current::BuildError::missing("id"))?;
        Ok(ExecutionReportItemQuery { id })
    }
}

impl crate::api::current::support::CurrentQuery for ExecutionReportItemQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "id", &self.id)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /executionReport/item` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn execution_report_item(
        &self,
        query: &ExecutionReportItemQuery,
    ) -> Result<super::users::ExecutionReport, crate::Error> {
        self.get_current("/executionReport/item", query).await
    }
}

/// Typed query parameters for `/executionReport/items`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ExecutionReportItemsQuery {
    #[serde(rename = "ids")]
    ids: Vec<super::ids::ExecutionReportId>,
}

impl ExecutionReportItemsQuery {
    /// Returns wire field `ids`.
    #[must_use]
    pub fn ids(&self) -> &[super::ids::ExecutionReportId] {
        &self.ids
    }

    /// Starts a builder for [`ExecutionReportItemsQuery`].
    pub fn builder() -> ExecutionReportItemsQueryBuilder {
        ExecutionReportItemsQueryBuilder::default()
    }
}

/// Builder for [`ExecutionReportItemsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ExecutionReportItemsQueryBuilder {
    ids: Option<Vec<super::ids::ExecutionReportId>>,
}

impl ExecutionReportItemsQueryBuilder {
    /// Sets wire field `ids`.
    pub fn ids(mut self, value: Vec<super::ids::ExecutionReportId>) -> Self {
        self.ids = Some(value);
        self
    }

    /// Validates required fields and builds [`ExecutionReportItemsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ExecutionReportItemsQuery, crate::api::current::BuildError> {
        let ids = self
            .ids
            .ok_or(crate::api::current::BuildError::missing("ids"))?;
        if ids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "ids",
                "must not be empty",
            ));
        }
        Ok(ExecutionReportItemsQuery { ids })
    }
}

impl crate::api::current::support::CurrentQuery for ExecutionReportItemsQuery {
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
    /// Calls the current `GET /executionReport/items` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn execution_report_items(
        &self,
        query: &ExecutionReportItemsQuery,
    ) -> Result<Vec<super::users::ExecutionReport>, crate::Error> {
        self.get_current("/executionReport/items", query).await
    }
}

/// Typed query parameters for `/executionReport/ldeps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ExecutionReportLDependentsQuery {
    #[serde(rename = "masterids")]
    masterids: Vec<super::ids::ProviderEntityId>,
}

impl ExecutionReportLDependentsQuery {
    /// Returns wire field `masterids`.
    #[must_use]
    pub fn masterids(&self) -> &[super::ids::ProviderEntityId] {
        &self.masterids
    }

    /// Starts a builder for [`ExecutionReportLDependentsQuery`].
    pub fn builder() -> ExecutionReportLDependentsQueryBuilder {
        ExecutionReportLDependentsQueryBuilder::default()
    }
}

/// Builder for [`ExecutionReportLDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ExecutionReportLDependentsQueryBuilder {
    masterids: Option<Vec<super::ids::ProviderEntityId>>,
}

impl ExecutionReportLDependentsQueryBuilder {
    /// Sets wire field `masterids`.
    pub fn masterids(mut self, value: Vec<super::ids::ProviderEntityId>) -> Self {
        self.masterids = Some(value);
        self
    }

    /// Validates required fields and builds [`ExecutionReportLDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ExecutionReportLDependentsQuery, crate::api::current::BuildError> {
        let masterids = self
            .masterids
            .ok_or(crate::api::current::BuildError::missing("masterids"))?;
        if masterids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "masterids",
                "must not be empty",
            ));
        }
        Ok(ExecutionReportLDependentsQuery { masterids })
    }
}

impl crate::api::current::support::CurrentQuery for ExecutionReportLDependentsQuery {
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
    /// Calls the current `GET /executionReport/ldeps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn execution_report_l_dependents(
        &self,
        query: &ExecutionReportLDependentsQuery,
    ) -> Result<Vec<super::users::ExecutionReport>, crate::Error> {
        self.get_current("/executionReport/ldeps", query).await
    }
}

impl crate::Client {
    /// Calls the current `GET /executionReport/list` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn execution_report_list(
        &self,
    ) -> Result<Vec<super::users::ExecutionReport>, crate::Error> {
        self.get_without_query("/executionReport/list").await
    }
}

/// Typed query parameters for `/fill/deps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct FillDependentsQuery {
    #[serde(rename = "masterid")]
    masterid: super::ids::ProviderEntityId,
}

impl FillDependentsQuery {
    /// Returns wire field `masterid`.
    #[must_use]
    pub fn masterid(&self) -> &super::ids::ProviderEntityId {
        &self.masterid
    }

    /// Starts a builder for [`FillDependentsQuery`].
    pub fn builder() -> FillDependentsQueryBuilder {
        FillDependentsQueryBuilder::default()
    }
}

/// Builder for [`FillDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct FillDependentsQueryBuilder {
    masterid: Option<super::ids::ProviderEntityId>,
}

impl FillDependentsQueryBuilder {
    /// Sets wire field `masterid`.
    pub fn masterid(mut self, value: super::ids::ProviderEntityId) -> Self {
        self.masterid = Some(value);
        self
    }

    /// Validates required fields and builds [`FillDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<FillDependentsQuery, crate::api::current::BuildError> {
        let masterid = self
            .masterid
            .ok_or(crate::api::current::BuildError::missing("masterid"))?;
        Ok(FillDependentsQuery { masterid })
    }
}

impl crate::api::current::support::CurrentQuery for FillDependentsQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "masterid", &self.masterid)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /fill/deps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn fill_dependents(
        &self,
        query: &FillDependentsQuery,
    ) -> Result<Vec<super::users::Fill>, crate::Error> {
        self.get_current("/fill/deps", query).await
    }
}

/// Typed query parameters for `/fillFee/deps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct FillFeeDependentsQuery {
    #[serde(rename = "masterid")]
    masterid: super::ids::ProviderEntityId,
}

impl FillFeeDependentsQuery {
    /// Returns wire field `masterid`.
    #[must_use]
    pub fn masterid(&self) -> &super::ids::ProviderEntityId {
        &self.masterid
    }

    /// Starts a builder for [`FillFeeDependentsQuery`].
    pub fn builder() -> FillFeeDependentsQueryBuilder {
        FillFeeDependentsQueryBuilder::default()
    }
}

/// Builder for [`FillFeeDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct FillFeeDependentsQueryBuilder {
    masterid: Option<super::ids::ProviderEntityId>,
}

impl FillFeeDependentsQueryBuilder {
    /// Sets wire field `masterid`.
    pub fn masterid(mut self, value: super::ids::ProviderEntityId) -> Self {
        self.masterid = Some(value);
        self
    }

    /// Validates required fields and builds [`FillFeeDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<FillFeeDependentsQuery, crate::api::current::BuildError> {
        let masterid = self
            .masterid
            .ok_or(crate::api::current::BuildError::missing("masterid"))?;
        Ok(FillFeeDependentsQuery { masterid })
    }
}

impl crate::api::current::support::CurrentQuery for FillFeeDependentsQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "masterid", &self.masterid)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /fillFee/deps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn fill_fee_dependents(
        &self,
        query: &FillFeeDependentsQuery,
    ) -> Result<Vec<super::users::FillFee>, crate::Error> {
        self.get_current("/fillFee/deps", query).await
    }
}

/// Typed query parameters for `/fillFee/item`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct FillFeeItemQuery {
    #[serde(rename = "id")]
    id: super::ids::FillFeeId,
}

impl FillFeeItemQuery {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> &super::ids::FillFeeId {
        &self.id
    }

    /// Starts a builder for [`FillFeeItemQuery`].
    pub fn builder() -> FillFeeItemQueryBuilder {
        FillFeeItemQueryBuilder::default()
    }
}

/// Builder for [`FillFeeItemQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct FillFeeItemQueryBuilder {
    id: Option<super::ids::FillFeeId>,
}

impl FillFeeItemQueryBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::FillFeeId) -> Self {
        self.id = Some(value);
        self
    }

    /// Validates required fields and builds [`FillFeeItemQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<FillFeeItemQuery, crate::api::current::BuildError> {
        let id = self
            .id
            .ok_or(crate::api::current::BuildError::missing("id"))?;
        Ok(FillFeeItemQuery { id })
    }
}

impl crate::api::current::support::CurrentQuery for FillFeeItemQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "id", &self.id)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /fillFee/item` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn fill_fee_item(
        &self,
        query: &FillFeeItemQuery,
    ) -> Result<super::users::FillFee, crate::Error> {
        self.get_current("/fillFee/item", query).await
    }
}

/// Typed query parameters for `/fillFee/items`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct FillFeeItemsQuery {
    #[serde(rename = "ids")]
    ids: Vec<super::ids::FillFeeId>,
}

impl FillFeeItemsQuery {
    /// Returns wire field `ids`.
    #[must_use]
    pub fn ids(&self) -> &[super::ids::FillFeeId] {
        &self.ids
    }

    /// Starts a builder for [`FillFeeItemsQuery`].
    pub fn builder() -> FillFeeItemsQueryBuilder {
        FillFeeItemsQueryBuilder::default()
    }
}

/// Builder for [`FillFeeItemsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct FillFeeItemsQueryBuilder {
    ids: Option<Vec<super::ids::FillFeeId>>,
}

impl FillFeeItemsQueryBuilder {
    /// Sets wire field `ids`.
    pub fn ids(mut self, value: Vec<super::ids::FillFeeId>) -> Self {
        self.ids = Some(value);
        self
    }

    /// Validates required fields and builds [`FillFeeItemsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<FillFeeItemsQuery, crate::api::current::BuildError> {
        let ids = self
            .ids
            .ok_or(crate::api::current::BuildError::missing("ids"))?;
        if ids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "ids",
                "must not be empty",
            ));
        }
        Ok(FillFeeItemsQuery { ids })
    }
}

impl crate::api::current::support::CurrentQuery for FillFeeItemsQuery {
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
    /// Calls the current `GET /fillFee/items` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn fill_fee_items(
        &self,
        query: &FillFeeItemsQuery,
    ) -> Result<Vec<super::users::FillFee>, crate::Error> {
        self.get_current("/fillFee/items", query).await
    }
}

/// Typed query parameters for `/fillFee/ldeps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct FillFeeLDependentsQuery {
    #[serde(rename = "masterids")]
    masterids: Vec<super::ids::ProviderEntityId>,
}

impl FillFeeLDependentsQuery {
    /// Returns wire field `masterids`.
    #[must_use]
    pub fn masterids(&self) -> &[super::ids::ProviderEntityId] {
        &self.masterids
    }

    /// Starts a builder for [`FillFeeLDependentsQuery`].
    pub fn builder() -> FillFeeLDependentsQueryBuilder {
        FillFeeLDependentsQueryBuilder::default()
    }
}

/// Builder for [`FillFeeLDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct FillFeeLDependentsQueryBuilder {
    masterids: Option<Vec<super::ids::ProviderEntityId>>,
}

impl FillFeeLDependentsQueryBuilder {
    /// Sets wire field `masterids`.
    pub fn masterids(mut self, value: Vec<super::ids::ProviderEntityId>) -> Self {
        self.masterids = Some(value);
        self
    }

    /// Validates required fields and builds [`FillFeeLDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<FillFeeLDependentsQuery, crate::api::current::BuildError> {
        let masterids = self
            .masterids
            .ok_or(crate::api::current::BuildError::missing("masterids"))?;
        if masterids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "masterids",
                "must not be empty",
            ));
        }
        Ok(FillFeeLDependentsQuery { masterids })
    }
}

impl crate::api::current::support::CurrentQuery for FillFeeLDependentsQuery {
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
    /// Calls the current `GET /fillFee/ldeps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn fill_fee_l_dependents(
        &self,
        query: &FillFeeLDependentsQuery,
    ) -> Result<Vec<super::users::FillFee>, crate::Error> {
        self.get_current("/fillFee/ldeps", query).await
    }
}

impl crate::Client {
    /// Calls the current `GET /fillFee/list` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn fill_fee_list(&self) -> Result<Vec<super::users::FillFee>, crate::Error> {
        self.get_without_query("/fillFee/list").await
    }
}

/// Typed query parameters for `/fill/item`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct FillItemQuery {
    #[serde(rename = "id")]
    id: super::ids::FillId,
}

impl FillItemQuery {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> &super::ids::FillId {
        &self.id
    }

    /// Starts a builder for [`FillItemQuery`].
    pub fn builder() -> FillItemQueryBuilder {
        FillItemQueryBuilder::default()
    }
}

/// Builder for [`FillItemQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct FillItemQueryBuilder {
    id: Option<super::ids::FillId>,
}

impl FillItemQueryBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::FillId) -> Self {
        self.id = Some(value);
        self
    }

    /// Validates required fields and builds [`FillItemQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<FillItemQuery, crate::api::current::BuildError> {
        let id = self
            .id
            .ok_or(crate::api::current::BuildError::missing("id"))?;
        Ok(FillItemQuery { id })
    }
}

impl crate::api::current::support::CurrentQuery for FillItemQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "id", &self.id)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /fill/item` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn fill_item(
        &self,
        query: &FillItemQuery,
    ) -> Result<super::users::Fill, crate::Error> {
        self.get_current("/fill/item", query).await
    }
}

/// Typed query parameters for `/fill/items`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct FillItemsQuery {
    #[serde(rename = "ids")]
    ids: Vec<super::ids::FillId>,
}

impl FillItemsQuery {
    /// Returns wire field `ids`.
    #[must_use]
    pub fn ids(&self) -> &[super::ids::FillId] {
        &self.ids
    }

    /// Starts a builder for [`FillItemsQuery`].
    pub fn builder() -> FillItemsQueryBuilder {
        FillItemsQueryBuilder::default()
    }
}

/// Builder for [`FillItemsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct FillItemsQueryBuilder {
    ids: Option<Vec<super::ids::FillId>>,
}

impl FillItemsQueryBuilder {
    /// Sets wire field `ids`.
    pub fn ids(mut self, value: Vec<super::ids::FillId>) -> Self {
        self.ids = Some(value);
        self
    }

    /// Validates required fields and builds [`FillItemsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<FillItemsQuery, crate::api::current::BuildError> {
        let ids = self
            .ids
            .ok_or(crate::api::current::BuildError::missing("ids"))?;
        if ids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "ids",
                "must not be empty",
            ));
        }
        Ok(FillItemsQuery { ids })
    }
}

impl crate::api::current::support::CurrentQuery for FillItemsQuery {
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
    /// Calls the current `GET /fill/items` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn fill_items(
        &self,
        query: &FillItemsQuery,
    ) -> Result<Vec<super::users::Fill>, crate::Error> {
        self.get_current("/fill/items", query).await
    }
}

/// Typed query parameters for `/fill/ldeps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct FillLDependentsQuery {
    #[serde(rename = "masterids")]
    masterids: Vec<super::ids::ProviderEntityId>,
}

impl FillLDependentsQuery {
    /// Returns wire field `masterids`.
    #[must_use]
    pub fn masterids(&self) -> &[super::ids::ProviderEntityId] {
        &self.masterids
    }

    /// Starts a builder for [`FillLDependentsQuery`].
    pub fn builder() -> FillLDependentsQueryBuilder {
        FillLDependentsQueryBuilder::default()
    }
}

/// Builder for [`FillLDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct FillLDependentsQueryBuilder {
    masterids: Option<Vec<super::ids::ProviderEntityId>>,
}

impl FillLDependentsQueryBuilder {
    /// Sets wire field `masterids`.
    pub fn masterids(mut self, value: Vec<super::ids::ProviderEntityId>) -> Self {
        self.masterids = Some(value);
        self
    }

    /// Validates required fields and builds [`FillLDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<FillLDependentsQuery, crate::api::current::BuildError> {
        let masterids = self
            .masterids
            .ok_or(crate::api::current::BuildError::missing("masterids"))?;
        if masterids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "masterids",
                "must not be empty",
            ));
        }
        Ok(FillLDependentsQuery { masterids })
    }
}

impl crate::api::current::support::CurrentQuery for FillLDependentsQuery {
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
    /// Calls the current `GET /fill/ldeps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn fill_l_dependents(
        &self,
        query: &FillLDependentsQuery,
    ) -> Result<Vec<super::users::Fill>, crate::Error> {
        self.get_current("/fill/ldeps", query).await
    }
}

impl crate::Client {
    /// Calls the current `GET /fill/list` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn fill_list(&self) -> Result<Vec<super::users::Fill>, crate::Error> {
        self.get_without_query("/fill/list").await
    }
}

/// Typed query parameters for `/order/deps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct OrderDependentsQuery {
    #[serde(rename = "masterid")]
    masterid: super::ids::ProviderEntityId,
}

impl OrderDependentsQuery {
    /// Returns wire field `masterid`.
    #[must_use]
    pub fn masterid(&self) -> &super::ids::ProviderEntityId {
        &self.masterid
    }

    /// Starts a builder for [`OrderDependentsQuery`].
    pub fn builder() -> OrderDependentsQueryBuilder {
        OrderDependentsQueryBuilder::default()
    }
}

/// Builder for [`OrderDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct OrderDependentsQueryBuilder {
    masterid: Option<super::ids::ProviderEntityId>,
}

impl OrderDependentsQueryBuilder {
    /// Sets wire field `masterid`.
    pub fn masterid(mut self, value: super::ids::ProviderEntityId) -> Self {
        self.masterid = Some(value);
        self
    }

    /// Validates required fields and builds [`OrderDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<OrderDependentsQuery, crate::api::current::BuildError> {
        let masterid = self
            .masterid
            .ok_or(crate::api::current::BuildError::missing("masterid"))?;
        Ok(OrderDependentsQuery { masterid })
    }
}

impl crate::api::current::support::CurrentQuery for OrderDependentsQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "masterid", &self.masterid)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /order/deps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn order_dependents(
        &self,
        query: &OrderDependentsQuery,
    ) -> Result<Vec<super::users::Order>, crate::Error> {
        self.get_current("/order/deps", query).await
    }
}

/// Typed query parameters for `/order/item`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct OrderItemQuery {
    #[serde(rename = "id")]
    id: crate::OrderId,
}

impl OrderItemQuery {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> &crate::OrderId {
        &self.id
    }

    /// Starts a builder for [`OrderItemQuery`].
    pub fn builder() -> OrderItemQueryBuilder {
        OrderItemQueryBuilder::default()
    }
}

/// Builder for [`OrderItemQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct OrderItemQueryBuilder {
    id: Option<crate::OrderId>,
}

impl OrderItemQueryBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: crate::OrderId) -> Self {
        self.id = Some(value);
        self
    }

    /// Validates required fields and builds [`OrderItemQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<OrderItemQuery, crate::api::current::BuildError> {
        let id = self
            .id
            .ok_or(crate::api::current::BuildError::missing("id"))?;
        Ok(OrderItemQuery { id })
    }
}

impl crate::api::current::support::CurrentQuery for OrderItemQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "id", &self.id)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /order/item` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn order_item(
        &self,
        query: &OrderItemQuery,
    ) -> Result<super::users::Order, crate::Error> {
        self.get_current("/order/item", query).await
    }
}

/// Typed query parameters for `/order/items`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct OrderItemsQuery {
    #[serde(rename = "ids")]
    ids: Vec<crate::OrderId>,
}

impl OrderItemsQuery {
    /// Returns wire field `ids`.
    #[must_use]
    pub fn ids(&self) -> &[crate::OrderId] {
        &self.ids
    }

    /// Starts a builder for [`OrderItemsQuery`].
    pub fn builder() -> OrderItemsQueryBuilder {
        OrderItemsQueryBuilder::default()
    }
}

/// Builder for [`OrderItemsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct OrderItemsQueryBuilder {
    ids: Option<Vec<crate::OrderId>>,
}

impl OrderItemsQueryBuilder {
    /// Sets wire field `ids`.
    pub fn ids(mut self, value: Vec<crate::OrderId>) -> Self {
        self.ids = Some(value);
        self
    }

    /// Validates required fields and builds [`OrderItemsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<OrderItemsQuery, crate::api::current::BuildError> {
        let ids = self
            .ids
            .ok_or(crate::api::current::BuildError::missing("ids"))?;
        if ids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "ids",
                "must not be empty",
            ));
        }
        Ok(OrderItemsQuery { ids })
    }
}

impl crate::api::current::support::CurrentQuery for OrderItemsQuery {
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
    /// Calls the current `GET /order/items` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn order_items(
        &self,
        query: &OrderItemsQuery,
    ) -> Result<Vec<super::users::Order>, crate::Error> {
        self.get_current("/order/items", query).await
    }
}

/// Typed query parameters for `/order/ldeps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct OrderLDependentsQuery {
    #[serde(rename = "masterids")]
    masterids: Vec<super::ids::ProviderEntityId>,
}

impl OrderLDependentsQuery {
    /// Returns wire field `masterids`.
    #[must_use]
    pub fn masterids(&self) -> &[super::ids::ProviderEntityId] {
        &self.masterids
    }

    /// Starts a builder for [`OrderLDependentsQuery`].
    pub fn builder() -> OrderLDependentsQueryBuilder {
        OrderLDependentsQueryBuilder::default()
    }
}

/// Builder for [`OrderLDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct OrderLDependentsQueryBuilder {
    masterids: Option<Vec<super::ids::ProviderEntityId>>,
}

impl OrderLDependentsQueryBuilder {
    /// Sets wire field `masterids`.
    pub fn masterids(mut self, value: Vec<super::ids::ProviderEntityId>) -> Self {
        self.masterids = Some(value);
        self
    }

    /// Validates required fields and builds [`OrderLDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<OrderLDependentsQuery, crate::api::current::BuildError> {
        let masterids = self
            .masterids
            .ok_or(crate::api::current::BuildError::missing("masterids"))?;
        if masterids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "masterids",
                "must not be empty",
            ));
        }
        Ok(OrderLDependentsQuery { masterids })
    }
}

impl crate::api::current::support::CurrentQuery for OrderLDependentsQuery {
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
    /// Calls the current `GET /order/ldeps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn order_l_dependents(
        &self,
        query: &OrderLDependentsQuery,
    ) -> Result<Vec<super::users::Order>, crate::Error> {
        self.get_current("/order/ldeps", query).await
    }
}

impl crate::Client {
    /// Calls the current `GET /order/list` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn order_list(&self) -> Result<Vec<super::users::Order>, crate::Error> {
        self.get_without_query("/order/list").await
    }
}

/// Typed query parameters for `/orderStrategy/deps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct OrderStrategyDependentsQuery {
    #[serde(rename = "masterid")]
    masterid: super::ids::ProviderEntityId,
}

impl OrderStrategyDependentsQuery {
    /// Returns wire field `masterid`.
    #[must_use]
    pub fn masterid(&self) -> &super::ids::ProviderEntityId {
        &self.masterid
    }

    /// Starts a builder for [`OrderStrategyDependentsQuery`].
    pub fn builder() -> OrderStrategyDependentsQueryBuilder {
        OrderStrategyDependentsQueryBuilder::default()
    }
}

/// Builder for [`OrderStrategyDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct OrderStrategyDependentsQueryBuilder {
    masterid: Option<super::ids::ProviderEntityId>,
}

impl OrderStrategyDependentsQueryBuilder {
    /// Sets wire field `masterid`.
    pub fn masterid(mut self, value: super::ids::ProviderEntityId) -> Self {
        self.masterid = Some(value);
        self
    }

    /// Validates required fields and builds [`OrderStrategyDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<OrderStrategyDependentsQuery, crate::api::current::BuildError> {
        let masterid = self
            .masterid
            .ok_or(crate::api::current::BuildError::missing("masterid"))?;
        Ok(OrderStrategyDependentsQuery { masterid })
    }
}

impl crate::api::current::support::CurrentQuery for OrderStrategyDependentsQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "masterid", &self.masterid)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /orderStrategy/deps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn order_strategy_dependents(
        &self,
        query: &OrderStrategyDependentsQuery,
    ) -> Result<Vec<super::users::OrderStrategy>, crate::Error> {
        self.get_current("/orderStrategy/deps", query).await
    }
}

/// Typed query parameters for `/orderStrategy/item`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct OrderStrategyItemQuery {
    #[serde(rename = "id")]
    id: super::ids::OrderStrategyId,
}

impl OrderStrategyItemQuery {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> &super::ids::OrderStrategyId {
        &self.id
    }

    /// Starts a builder for [`OrderStrategyItemQuery`].
    pub fn builder() -> OrderStrategyItemQueryBuilder {
        OrderStrategyItemQueryBuilder::default()
    }
}

/// Builder for [`OrderStrategyItemQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct OrderStrategyItemQueryBuilder {
    id: Option<super::ids::OrderStrategyId>,
}

impl OrderStrategyItemQueryBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::OrderStrategyId) -> Self {
        self.id = Some(value);
        self
    }

    /// Validates required fields and builds [`OrderStrategyItemQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<OrderStrategyItemQuery, crate::api::current::BuildError> {
        let id = self
            .id
            .ok_or(crate::api::current::BuildError::missing("id"))?;
        Ok(OrderStrategyItemQuery { id })
    }
}

impl crate::api::current::support::CurrentQuery for OrderStrategyItemQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "id", &self.id)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /orderStrategy/item` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn order_strategy_item(
        &self,
        query: &OrderStrategyItemQuery,
    ) -> Result<super::users::OrderStrategy, crate::Error> {
        self.get_current("/orderStrategy/item", query).await
    }
}

/// Typed query parameters for `/orderStrategy/items`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct OrderStrategyItemsQuery {
    #[serde(rename = "ids")]
    ids: Vec<super::ids::OrderStrategyId>,
}

impl OrderStrategyItemsQuery {
    /// Returns wire field `ids`.
    #[must_use]
    pub fn ids(&self) -> &[super::ids::OrderStrategyId] {
        &self.ids
    }

    /// Starts a builder for [`OrderStrategyItemsQuery`].
    pub fn builder() -> OrderStrategyItemsQueryBuilder {
        OrderStrategyItemsQueryBuilder::default()
    }
}

/// Builder for [`OrderStrategyItemsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct OrderStrategyItemsQueryBuilder {
    ids: Option<Vec<super::ids::OrderStrategyId>>,
}

impl OrderStrategyItemsQueryBuilder {
    /// Sets wire field `ids`.
    pub fn ids(mut self, value: Vec<super::ids::OrderStrategyId>) -> Self {
        self.ids = Some(value);
        self
    }

    /// Validates required fields and builds [`OrderStrategyItemsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<OrderStrategyItemsQuery, crate::api::current::BuildError> {
        let ids = self
            .ids
            .ok_or(crate::api::current::BuildError::missing("ids"))?;
        if ids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "ids",
                "must not be empty",
            ));
        }
        Ok(OrderStrategyItemsQuery { ids })
    }
}

impl crate::api::current::support::CurrentQuery for OrderStrategyItemsQuery {
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
    /// Calls the current `GET /orderStrategy/items` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn order_strategy_items(
        &self,
        query: &OrderStrategyItemsQuery,
    ) -> Result<Vec<super::users::OrderStrategy>, crate::Error> {
        self.get_current("/orderStrategy/items", query).await
    }
}

/// Typed query parameters for `/orderStrategy/ldeps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct OrderStrategyLDependentsQuery {
    #[serde(rename = "masterids")]
    masterids: Vec<super::ids::ProviderEntityId>,
}

impl OrderStrategyLDependentsQuery {
    /// Returns wire field `masterids`.
    #[must_use]
    pub fn masterids(&self) -> &[super::ids::ProviderEntityId] {
        &self.masterids
    }

    /// Starts a builder for [`OrderStrategyLDependentsQuery`].
    pub fn builder() -> OrderStrategyLDependentsQueryBuilder {
        OrderStrategyLDependentsQueryBuilder::default()
    }
}

/// Builder for [`OrderStrategyLDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct OrderStrategyLDependentsQueryBuilder {
    masterids: Option<Vec<super::ids::ProviderEntityId>>,
}

impl OrderStrategyLDependentsQueryBuilder {
    /// Sets wire field `masterids`.
    pub fn masterids(mut self, value: Vec<super::ids::ProviderEntityId>) -> Self {
        self.masterids = Some(value);
        self
    }

    /// Validates required fields and builds [`OrderStrategyLDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<OrderStrategyLDependentsQuery, crate::api::current::BuildError> {
        let masterids = self
            .masterids
            .ok_or(crate::api::current::BuildError::missing("masterids"))?;
        if masterids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "masterids",
                "must not be empty",
            ));
        }
        Ok(OrderStrategyLDependentsQuery { masterids })
    }
}

impl crate::api::current::support::CurrentQuery for OrderStrategyLDependentsQuery {
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
    /// Calls the current `GET /orderStrategy/ldeps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn order_strategy_l_dependents(
        &self,
        query: &OrderStrategyLDependentsQuery,
    ) -> Result<Vec<super::users::OrderStrategy>, crate::Error> {
        self.get_current("/orderStrategy/ldeps", query).await
    }
}

/// Typed query parameters for `/orderStrategyLink/deps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct OrderStrategyLinkDependentsQuery {
    #[serde(rename = "masterid")]
    masterid: super::ids::ProviderEntityId,
}

impl OrderStrategyLinkDependentsQuery {
    /// Returns wire field `masterid`.
    #[must_use]
    pub fn masterid(&self) -> &super::ids::ProviderEntityId {
        &self.masterid
    }

    /// Starts a builder for [`OrderStrategyLinkDependentsQuery`].
    pub fn builder() -> OrderStrategyLinkDependentsQueryBuilder {
        OrderStrategyLinkDependentsQueryBuilder::default()
    }
}

/// Builder for [`OrderStrategyLinkDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct OrderStrategyLinkDependentsQueryBuilder {
    masterid: Option<super::ids::ProviderEntityId>,
}

impl OrderStrategyLinkDependentsQueryBuilder {
    /// Sets wire field `masterid`.
    pub fn masterid(mut self, value: super::ids::ProviderEntityId) -> Self {
        self.masterid = Some(value);
        self
    }

    /// Validates required fields and builds [`OrderStrategyLinkDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(
        self,
    ) -> Result<OrderStrategyLinkDependentsQuery, crate::api::current::BuildError> {
        let masterid = self
            .masterid
            .ok_or(crate::api::current::BuildError::missing("masterid"))?;
        Ok(OrderStrategyLinkDependentsQuery { masterid })
    }
}

impl crate::api::current::support::CurrentQuery for OrderStrategyLinkDependentsQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "masterid", &self.masterid)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /orderStrategyLink/deps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn order_strategy_link_dependents(
        &self,
        query: &OrderStrategyLinkDependentsQuery,
    ) -> Result<Vec<super::users::OrderStrategyLink>, crate::Error> {
        self.get_current("/orderStrategyLink/deps", query).await
    }
}

/// Typed query parameters for `/orderStrategyLink/item`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct OrderStrategyLinkItemQuery {
    #[serde(rename = "id")]
    id: super::ids::OrderStrategyLinkId,
}

impl OrderStrategyLinkItemQuery {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> &super::ids::OrderStrategyLinkId {
        &self.id
    }

    /// Starts a builder for [`OrderStrategyLinkItemQuery`].
    pub fn builder() -> OrderStrategyLinkItemQueryBuilder {
        OrderStrategyLinkItemQueryBuilder::default()
    }
}

/// Builder for [`OrderStrategyLinkItemQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct OrderStrategyLinkItemQueryBuilder {
    id: Option<super::ids::OrderStrategyLinkId>,
}

impl OrderStrategyLinkItemQueryBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::OrderStrategyLinkId) -> Self {
        self.id = Some(value);
        self
    }

    /// Validates required fields and builds [`OrderStrategyLinkItemQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<OrderStrategyLinkItemQuery, crate::api::current::BuildError> {
        let id = self
            .id
            .ok_or(crate::api::current::BuildError::missing("id"))?;
        Ok(OrderStrategyLinkItemQuery { id })
    }
}

impl crate::api::current::support::CurrentQuery for OrderStrategyLinkItemQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "id", &self.id)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /orderStrategyLink/item` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn order_strategy_link_item(
        &self,
        query: &OrderStrategyLinkItemQuery,
    ) -> Result<super::users::OrderStrategyLink, crate::Error> {
        self.get_current("/orderStrategyLink/item", query).await
    }
}

/// Typed query parameters for `/orderStrategyLink/items`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct OrderStrategyLinkItemsQuery {
    #[serde(rename = "ids")]
    ids: Vec<super::ids::OrderStrategyLinkId>,
}

impl OrderStrategyLinkItemsQuery {
    /// Returns wire field `ids`.
    #[must_use]
    pub fn ids(&self) -> &[super::ids::OrderStrategyLinkId] {
        &self.ids
    }

    /// Starts a builder for [`OrderStrategyLinkItemsQuery`].
    pub fn builder() -> OrderStrategyLinkItemsQueryBuilder {
        OrderStrategyLinkItemsQueryBuilder::default()
    }
}

/// Builder for [`OrderStrategyLinkItemsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct OrderStrategyLinkItemsQueryBuilder {
    ids: Option<Vec<super::ids::OrderStrategyLinkId>>,
}

impl OrderStrategyLinkItemsQueryBuilder {
    /// Sets wire field `ids`.
    pub fn ids(mut self, value: Vec<super::ids::OrderStrategyLinkId>) -> Self {
        self.ids = Some(value);
        self
    }

    /// Validates required fields and builds [`OrderStrategyLinkItemsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<OrderStrategyLinkItemsQuery, crate::api::current::BuildError> {
        let ids = self
            .ids
            .ok_or(crate::api::current::BuildError::missing("ids"))?;
        if ids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "ids",
                "must not be empty",
            ));
        }
        Ok(OrderStrategyLinkItemsQuery { ids })
    }
}

impl crate::api::current::support::CurrentQuery for OrderStrategyLinkItemsQuery {
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
    /// Calls the current `GET /orderStrategyLink/items` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn order_strategy_link_items(
        &self,
        query: &OrderStrategyLinkItemsQuery,
    ) -> Result<Vec<super::users::OrderStrategyLink>, crate::Error> {
        self.get_current("/orderStrategyLink/items", query).await
    }
}

/// Typed query parameters for `/orderStrategyLink/ldeps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct OrderStrategyLinkLDependentsQuery {
    #[serde(rename = "masterids")]
    masterids: Vec<super::ids::ProviderEntityId>,
}

impl OrderStrategyLinkLDependentsQuery {
    /// Returns wire field `masterids`.
    #[must_use]
    pub fn masterids(&self) -> &[super::ids::ProviderEntityId] {
        &self.masterids
    }

    /// Starts a builder for [`OrderStrategyLinkLDependentsQuery`].
    pub fn builder() -> OrderStrategyLinkLDependentsQueryBuilder {
        OrderStrategyLinkLDependentsQueryBuilder::default()
    }
}

/// Builder for [`OrderStrategyLinkLDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct OrderStrategyLinkLDependentsQueryBuilder {
    masterids: Option<Vec<super::ids::ProviderEntityId>>,
}

impl OrderStrategyLinkLDependentsQueryBuilder {
    /// Sets wire field `masterids`.
    pub fn masterids(mut self, value: Vec<super::ids::ProviderEntityId>) -> Self {
        self.masterids = Some(value);
        self
    }

    /// Validates required fields and builds [`OrderStrategyLinkLDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(
        self,
    ) -> Result<OrderStrategyLinkLDependentsQuery, crate::api::current::BuildError> {
        let masterids = self
            .masterids
            .ok_or(crate::api::current::BuildError::missing("masterids"))?;
        if masterids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "masterids",
                "must not be empty",
            ));
        }
        Ok(OrderStrategyLinkLDependentsQuery { masterids })
    }
}

impl crate::api::current::support::CurrentQuery for OrderStrategyLinkLDependentsQuery {
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
    /// Calls the current `GET /orderStrategyLink/ldeps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn order_strategy_link_l_dependents(
        &self,
        query: &OrderStrategyLinkLDependentsQuery,
    ) -> Result<Vec<super::users::OrderStrategyLink>, crate::Error> {
        self.get_current("/orderStrategyLink/ldeps", query).await
    }
}

impl crate::Client {
    /// Calls the current `GET /orderStrategyLink/list` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn order_strategy_link_list(
        &self,
    ) -> Result<Vec<super::users::OrderStrategyLink>, crate::Error> {
        self.get_without_query("/orderStrategyLink/list").await
    }
}

impl crate::Client {
    /// Calls the current `GET /orderStrategy/list` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn order_strategy_list(
        &self,
    ) -> Result<Vec<super::users::OrderStrategy>, crate::Error> {
        self.get_without_query("/orderStrategy/list").await
    }
}

/// Typed query parameters for `/orderVersion/deps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct OrderVersionDependentsQuery {
    #[serde(rename = "masterid")]
    masterid: super::ids::ProviderEntityId,
}

impl OrderVersionDependentsQuery {
    /// Returns wire field `masterid`.
    #[must_use]
    pub fn masterid(&self) -> &super::ids::ProviderEntityId {
        &self.masterid
    }

    /// Starts a builder for [`OrderVersionDependentsQuery`].
    pub fn builder() -> OrderVersionDependentsQueryBuilder {
        OrderVersionDependentsQueryBuilder::default()
    }
}

/// Builder for [`OrderVersionDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct OrderVersionDependentsQueryBuilder {
    masterid: Option<super::ids::ProviderEntityId>,
}

impl OrderVersionDependentsQueryBuilder {
    /// Sets wire field `masterid`.
    pub fn masterid(mut self, value: super::ids::ProviderEntityId) -> Self {
        self.masterid = Some(value);
        self
    }

    /// Validates required fields and builds [`OrderVersionDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<OrderVersionDependentsQuery, crate::api::current::BuildError> {
        let masterid = self
            .masterid
            .ok_or(crate::api::current::BuildError::missing("masterid"))?;
        Ok(OrderVersionDependentsQuery { masterid })
    }
}

impl crate::api::current::support::CurrentQuery for OrderVersionDependentsQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "masterid", &self.masterid)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /orderVersion/deps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn order_version_dependents(
        &self,
        query: &OrderVersionDependentsQuery,
    ) -> Result<Vec<super::users::OrderVersion>, crate::Error> {
        self.get_current("/orderVersion/deps", query).await
    }
}

/// Typed query parameters for `/orderVersion/item`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct OrderVersionItemQuery {
    #[serde(rename = "id")]
    id: super::ids::OrderVersionId,
}

impl OrderVersionItemQuery {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> &super::ids::OrderVersionId {
        &self.id
    }

    /// Starts a builder for [`OrderVersionItemQuery`].
    pub fn builder() -> OrderVersionItemQueryBuilder {
        OrderVersionItemQueryBuilder::default()
    }
}

/// Builder for [`OrderVersionItemQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct OrderVersionItemQueryBuilder {
    id: Option<super::ids::OrderVersionId>,
}

impl OrderVersionItemQueryBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::OrderVersionId) -> Self {
        self.id = Some(value);
        self
    }

    /// Validates required fields and builds [`OrderVersionItemQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<OrderVersionItemQuery, crate::api::current::BuildError> {
        let id = self
            .id
            .ok_or(crate::api::current::BuildError::missing("id"))?;
        Ok(OrderVersionItemQuery { id })
    }
}

impl crate::api::current::support::CurrentQuery for OrderVersionItemQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "id", &self.id)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /orderVersion/item` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn order_version_item(
        &self,
        query: &OrderVersionItemQuery,
    ) -> Result<super::users::OrderVersion, crate::Error> {
        self.get_current("/orderVersion/item", query).await
    }
}

/// Typed query parameters for `/orderVersion/items`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct OrderVersionItemsQuery {
    #[serde(rename = "ids")]
    ids: Vec<super::ids::OrderVersionId>,
}

impl OrderVersionItemsQuery {
    /// Returns wire field `ids`.
    #[must_use]
    pub fn ids(&self) -> &[super::ids::OrderVersionId] {
        &self.ids
    }

    /// Starts a builder for [`OrderVersionItemsQuery`].
    pub fn builder() -> OrderVersionItemsQueryBuilder {
        OrderVersionItemsQueryBuilder::default()
    }
}

/// Builder for [`OrderVersionItemsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct OrderVersionItemsQueryBuilder {
    ids: Option<Vec<super::ids::OrderVersionId>>,
}

impl OrderVersionItemsQueryBuilder {
    /// Sets wire field `ids`.
    pub fn ids(mut self, value: Vec<super::ids::OrderVersionId>) -> Self {
        self.ids = Some(value);
        self
    }

    /// Validates required fields and builds [`OrderVersionItemsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<OrderVersionItemsQuery, crate::api::current::BuildError> {
        let ids = self
            .ids
            .ok_or(crate::api::current::BuildError::missing("ids"))?;
        if ids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "ids",
                "must not be empty",
            ));
        }
        Ok(OrderVersionItemsQuery { ids })
    }
}

impl crate::api::current::support::CurrentQuery for OrderVersionItemsQuery {
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
    /// Calls the current `GET /orderVersion/items` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn order_version_items(
        &self,
        query: &OrderVersionItemsQuery,
    ) -> Result<Vec<super::users::OrderVersion>, crate::Error> {
        self.get_current("/orderVersion/items", query).await
    }
}

/// Typed query parameters for `/orderVersion/ldeps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct OrderVersionLDependentsQuery {
    #[serde(rename = "masterids")]
    masterids: Vec<super::ids::ProviderEntityId>,
}

impl OrderVersionLDependentsQuery {
    /// Returns wire field `masterids`.
    #[must_use]
    pub fn masterids(&self) -> &[super::ids::ProviderEntityId] {
        &self.masterids
    }

    /// Starts a builder for [`OrderVersionLDependentsQuery`].
    pub fn builder() -> OrderVersionLDependentsQueryBuilder {
        OrderVersionLDependentsQueryBuilder::default()
    }
}

/// Builder for [`OrderVersionLDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct OrderVersionLDependentsQueryBuilder {
    masterids: Option<Vec<super::ids::ProviderEntityId>>,
}

impl OrderVersionLDependentsQueryBuilder {
    /// Sets wire field `masterids`.
    pub fn masterids(mut self, value: Vec<super::ids::ProviderEntityId>) -> Self {
        self.masterids = Some(value);
        self
    }

    /// Validates required fields and builds [`OrderVersionLDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<OrderVersionLDependentsQuery, crate::api::current::BuildError> {
        let masterids = self
            .masterids
            .ok_or(crate::api::current::BuildError::missing("masterids"))?;
        if masterids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "masterids",
                "must not be empty",
            ));
        }
        Ok(OrderVersionLDependentsQuery { masterids })
    }
}

impl crate::api::current::support::CurrentQuery for OrderVersionLDependentsQuery {
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
    /// Calls the current `GET /orderVersion/ldeps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn order_version_l_dependents(
        &self,
        query: &OrderVersionLDependentsQuery,
    ) -> Result<Vec<super::users::OrderVersion>, crate::Error> {
        self.get_current("/orderVersion/ldeps", query).await
    }
}

impl crate::Client {
    /// Calls the current `GET /orderVersion/list` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn order_version_list(
        &self,
    ) -> Result<Vec<super::users::OrderVersion>, crate::Error> {
        self.get_without_query("/orderVersion/list").await
    }
}

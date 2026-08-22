// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0
// @generated
// Generator: tools/generate_openapi.py
// Source: https://partner.tradovate.com/openapi.json (snapshot 2026-08-21, sha256 37caeccf4b0913460a788fcaf4c902497059b8ffe6f6355512e6c08eaacde769)

// Provider wire fields remain schema-auditable even when they repeat
// their type name; wide schema-faithful builders remain one generated
// unit so regeneration and source review cannot drift field subsets.
#![allow(clippy::struct_field_names, clippy::too_many_lines)]

//! Current account, balance, margin, and permission operations.

/// Current wire model `CashBalanceLog`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct CashBalanceLog {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    id: Option<super::ids::CashBalanceLogId>,
    #[serde(rename = "accountId")]
    account_id: crate::AccountId,
    #[serde(rename = "timestamp")]
    timestamp: jiff::Timestamp,
    #[serde(rename = "tradeDate")]
    trade_date: super::users::TradeDate,
    #[serde(rename = "currencyId")]
    currency_id: super::ids::CurrencyId,
    #[serde(rename = "amount")]
    #[serde(with = "crate::decimal")]
    amount: crate::Decimal,
    #[serde(
        rename = "realizedPnL",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    realized_pn_l: Option<crate::Decimal>,
    #[serde(
        rename = "weekRealizedPnL",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    week_realized_pn_l: Option<crate::Decimal>,
    #[serde(rename = "cashChangeType")]
    cash_change_type: CashBalanceLogCashChangeType,
    #[serde(
        rename = "fillPairId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    fill_pair_id: Option<super::ids::FillPairId>,
    #[serde(rename = "fillId", default, skip_serializing_if = "Option::is_none")]
    fill_id: Option<super::ids::FillId>,
    #[serde(
        rename = "fundTransactionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    fund_transaction_id: Option<super::ids::FundTransactionId>,
    #[serde(rename = "comment", default, skip_serializing_if = "Option::is_none")]
    comment: Option<String>,
    #[serde(rename = "delta")]
    #[serde(with = "crate::decimal")]
    delta: crate::Decimal,
    #[serde(rename = "senderId", default, skip_serializing_if = "Option::is_none")]
    sender_id: Option<super::ids::SenderId>,
}

impl CashBalanceLog {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> Option<&super::ids::CashBalanceLogId> {
        self.id.as_ref()
    }

    /// Returns wire field `accountId`.
    #[must_use]
    pub fn account_id(&self) -> &crate::AccountId {
        &self.account_id
    }

    /// Returns wire field `timestamp`.
    #[must_use]
    pub fn timestamp(&self) -> &jiff::Timestamp {
        &self.timestamp
    }

    /// Returns wire field `tradeDate`.
    #[must_use]
    pub fn trade_date(&self) -> &super::users::TradeDate {
        &self.trade_date
    }

    /// Returns wire field `currencyId`.
    #[must_use]
    pub fn currency_id(&self) -> &super::ids::CurrencyId {
        &self.currency_id
    }

    /// Returns wire field `amount`.
    #[must_use]
    pub fn amount(&self) -> &crate::Decimal {
        &self.amount
    }

    /// Returns wire field `realizedPnL`.
    #[must_use]
    pub fn realized_pn_l(&self) -> Option<&crate::Decimal> {
        self.realized_pn_l.as_ref()
    }

    /// Returns wire field `weekRealizedPnL`.
    #[must_use]
    pub fn week_realized_pn_l(&self) -> Option<&crate::Decimal> {
        self.week_realized_pn_l.as_ref()
    }

    /// Returns wire field `cashChangeType`.
    #[must_use]
    pub fn cash_change_type(&self) -> &CashBalanceLogCashChangeType {
        &self.cash_change_type
    }

    /// Returns wire field `fillPairId`.
    #[must_use]
    pub fn fill_pair_id(&self) -> Option<&super::ids::FillPairId> {
        self.fill_pair_id.as_ref()
    }

    /// Returns wire field `fillId`.
    #[must_use]
    pub fn fill_id(&self) -> Option<&super::ids::FillId> {
        self.fill_id.as_ref()
    }

    /// Returns wire field `fundTransactionId`.
    #[must_use]
    pub fn fund_transaction_id(&self) -> Option<&super::ids::FundTransactionId> {
        self.fund_transaction_id.as_ref()
    }

    /// Returns wire field `comment`.
    #[must_use]
    pub fn comment(&self) -> Option<&str> {
        self.comment.as_deref()
    }

    /// Returns wire field `delta`.
    #[must_use]
    pub fn delta(&self) -> &crate::Decimal {
        &self.delta
    }

    /// Returns wire field `senderId`.
    #[must_use]
    pub fn sender_id(&self) -> Option<&super::ids::SenderId> {
        self.sender_id.as_ref()
    }

    /// Starts a builder for [`CashBalanceLog`].
    pub fn builder() -> CashBalanceLogBuilder {
        CashBalanceLogBuilder::default()
    }
}

/// Builder for [`CashBalanceLog`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct CashBalanceLogBuilder {
    id: Option<super::ids::CashBalanceLogId>,
    account_id: Option<crate::AccountId>,
    timestamp: Option<jiff::Timestamp>,
    trade_date: Option<super::users::TradeDate>,
    currency_id: Option<super::ids::CurrencyId>,
    amount: Option<crate::Decimal>,
    realized_pn_l: Option<crate::Decimal>,
    week_realized_pn_l: Option<crate::Decimal>,
    cash_change_type: Option<CashBalanceLogCashChangeType>,
    fill_pair_id: Option<super::ids::FillPairId>,
    fill_id: Option<super::ids::FillId>,
    fund_transaction_id: Option<super::ids::FundTransactionId>,
    comment: Option<String>,
    delta: Option<crate::Decimal>,
    sender_id: Option<super::ids::SenderId>,
}

impl CashBalanceLogBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::CashBalanceLogId) -> Self {
        self.id = Some(value);
        self
    }

    /// Sets wire field `accountId`.
    pub fn account_id(mut self, value: crate::AccountId) -> Self {
        self.account_id = Some(value);
        self
    }

    /// Sets wire field `timestamp`.
    pub fn timestamp(mut self, value: jiff::Timestamp) -> Self {
        self.timestamp = Some(value);
        self
    }

    /// Sets wire field `tradeDate`.
    pub fn trade_date(mut self, value: super::users::TradeDate) -> Self {
        self.trade_date = Some(value);
        self
    }

    /// Sets wire field `currencyId`.
    pub fn currency_id(mut self, value: super::ids::CurrencyId) -> Self {
        self.currency_id = Some(value);
        self
    }

    /// Sets wire field `amount`.
    pub fn amount(mut self, value: crate::Decimal) -> Self {
        self.amount = Some(value);
        self
    }

    /// Sets wire field `realizedPnL`.
    pub fn realized_pn_l(mut self, value: crate::Decimal) -> Self {
        self.realized_pn_l = Some(value);
        self
    }

    /// Sets wire field `weekRealizedPnL`.
    pub fn week_realized_pn_l(mut self, value: crate::Decimal) -> Self {
        self.week_realized_pn_l = Some(value);
        self
    }

    /// Sets wire field `cashChangeType`.
    pub fn cash_change_type(mut self, value: CashBalanceLogCashChangeType) -> Self {
        self.cash_change_type = Some(value);
        self
    }

    /// Sets wire field `fillPairId`.
    pub fn fill_pair_id(mut self, value: super::ids::FillPairId) -> Self {
        self.fill_pair_id = Some(value);
        self
    }

    /// Sets wire field `fillId`.
    pub fn fill_id(mut self, value: super::ids::FillId) -> Self {
        self.fill_id = Some(value);
        self
    }

    /// Sets wire field `fundTransactionId`.
    pub fn fund_transaction_id(mut self, value: super::ids::FundTransactionId) -> Self {
        self.fund_transaction_id = Some(value);
        self
    }

    /// Sets wire field `comment`.
    pub fn comment(mut self, value: impl Into<String>) -> Self {
        self.comment = Some(value.into());
        self
    }

    /// Sets wire field `delta`.
    pub fn delta(mut self, value: crate::Decimal) -> Self {
        self.delta = Some(value);
        self
    }

    /// Sets wire field `senderId`.
    pub fn sender_id(mut self, value: super::ids::SenderId) -> Self {
        self.sender_id = Some(value);
        self
    }

    /// Validates required fields and builds [`CashBalanceLog`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<CashBalanceLog, crate::api::current::BuildError> {
        let account_id = self
            .account_id
            .ok_or(crate::api::current::BuildError::missing("accountId"))?;
        let timestamp = self
            .timestamp
            .ok_or(crate::api::current::BuildError::missing("timestamp"))?;
        let trade_date = self
            .trade_date
            .ok_or(crate::api::current::BuildError::missing("tradeDate"))?;
        let currency_id = self
            .currency_id
            .ok_or(crate::api::current::BuildError::missing("currencyId"))?;
        let amount = self
            .amount
            .ok_or(crate::api::current::BuildError::missing("amount"))?;
        let cash_change_type = self
            .cash_change_type
            .ok_or(crate::api::current::BuildError::missing("cashChangeType"))?;
        let delta = self
            .delta
            .ok_or(crate::api::current::BuildError::missing("delta"))?;
        Ok(CashBalanceLog {
            id: self.id,
            account_id,
            timestamp,
            trade_date,
            currency_id,
            amount,
            realized_pn_l: self.realized_pn_l,
            week_realized_pn_l: self.week_realized_pn_l,
            cash_change_type,
            fill_pair_id: self.fill_pair_id,
            fill_id: self.fill_id,
            fund_transaction_id: self.fund_transaction_id,
            comment: self.comment,
            delta,
            sender_id: self.sender_id,
        })
    }
}

/// Current provider values for `CashBalanceLogCashChangeType`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum CashBalanceLogCashChangeType {
    /// Provider value `AccountClosureFee`.
    AccountClosureFee,
    /// Provider value `AddUserFee`.
    AddUserFee,
    /// Provider value `AutomaticReconciliation`.
    AutomaticReconciliation,
    /// Provider value `BrokerageFee`.
    BrokerageFee,
    /// Provider value `CancelledPairedTrade`.
    CancelledPairedTrade,
    /// Provider value `CashSettlement`.
    CashSettlement,
    /// Provider value `ChallengePayout`.
    ChallengePayout,
    /// Provider value `ClearingFee`.
    ClearingFee,
    /// Provider value `Commission`.
    Commission,
    /// Provider value `Courtesy`.
    Courtesy,
    /// Provider value `CurrencyConversion`.
    CurrencyConversion,
    /// Provider value `CurrencyConversionFee`.
    CurrencyConversionFee,
    /// Provider value `Debit`.
    Debit,
    /// Provider value `DepositFee`.
    DepositFee,
    /// Provider value `DeskFee`.
    DeskFee,
    /// Provider value `DormantFee`.
    DormantFee,
    /// Provider value `EntitlementSubscription`.
    EntitlementSubscription,
    /// Provider value `Escheatment`.
    Escheatment,
    /// Provider value `ExchangeFee`.
    ExchangeFee,
    /// Provider value `FundTransaction`.
    FundTransaction,
    /// Provider value `FundTransactionFee`.
    FundTransactionFee,
    /// Provider value `FundingRate`.
    FundingRate,
    /// Provider value `GoodwillCredit`.
    GoodwillCredit,
    /// Provider value `IPFee`.
    IpFee,
    /// Provider value `InactivityFee`.
    InactivityFee,
    /// Provider value `LiquidationFee`.
    LiquidationFee,
    /// Provider value `LiquidationFee2`.
    LiquidationFee2,
    /// Provider value `ManualAdjustment`.
    ManualAdjustment,
    /// Provider value `MarketDataSubscription`.
    MarketDataSubscription,
    /// Provider value `NewSession`.
    NewSession,
    /// Provider value `NfaFee`.
    NfaFee,
    /// Provider value `NsfCheckFee`.
    NsfCheckFee,
    /// Provider value `OptionsTrade`.
    OptionsTrade,
    /// Provider value `OrderRoutingFee`.
    OrderRoutingFee,
    /// Provider value `PROMO`.
    Promo,
    /// Provider value `RithmicFee`.
    RithmicFee,
    /// Provider value `SeatLeasePayment`.
    SeatLeasePayment,
    /// Provider value `StopPaymentFee`.
    StopPaymentFee,
    /// Provider value `SwapTrade`.
    SwapTrade,
    /// Provider value `ThirdPartyFee`.
    ThirdPartyFee,
    /// Provider value `TradePaired`.
    TradePaired,
    /// Provider value `TradovateSubscription`.
    TradovateSubscription,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl CashBalanceLogCashChangeType {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::AccountClosureFee => "AccountClosureFee",
            Self::AddUserFee => "AddUserFee",
            Self::AutomaticReconciliation => "AutomaticReconciliation",
            Self::BrokerageFee => "BrokerageFee",
            Self::CancelledPairedTrade => "CancelledPairedTrade",
            Self::CashSettlement => "CashSettlement",
            Self::ChallengePayout => "ChallengePayout",
            Self::ClearingFee => "ClearingFee",
            Self::Commission => "Commission",
            Self::Courtesy => "Courtesy",
            Self::CurrencyConversion => "CurrencyConversion",
            Self::CurrencyConversionFee => "CurrencyConversionFee",
            Self::Debit => "Debit",
            Self::DepositFee => "DepositFee",
            Self::DeskFee => "DeskFee",
            Self::DormantFee => "DormantFee",
            Self::EntitlementSubscription => "EntitlementSubscription",
            Self::Escheatment => "Escheatment",
            Self::ExchangeFee => "ExchangeFee",
            Self::FundTransaction => "FundTransaction",
            Self::FundTransactionFee => "FundTransactionFee",
            Self::FundingRate => "FundingRate",
            Self::GoodwillCredit => "GoodwillCredit",
            Self::IpFee => "IPFee",
            Self::InactivityFee => "InactivityFee",
            Self::LiquidationFee => "LiquidationFee",
            Self::LiquidationFee2 => "LiquidationFee2",
            Self::ManualAdjustment => "ManualAdjustment",
            Self::MarketDataSubscription => "MarketDataSubscription",
            Self::NewSession => "NewSession",
            Self::NfaFee => "NfaFee",
            Self::NsfCheckFee => "NsfCheckFee",
            Self::OptionsTrade => "OptionsTrade",
            Self::OrderRoutingFee => "OrderRoutingFee",
            Self::Promo => "PROMO",
            Self::RithmicFee => "RithmicFee",
            Self::SeatLeasePayment => "SeatLeasePayment",
            Self::StopPaymentFee => "StopPaymentFee",
            Self::SwapTrade => "SwapTrade",
            Self::ThirdPartyFee => "ThirdPartyFee",
            Self::TradePaired => "TradePaired",
            Self::TradovateSubscription => "TradovateSubscription",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for CashBalanceLogCashChangeType {
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

impl<'de> serde::Deserialize<'de> for CashBalanceLogCashChangeType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "AccountClosureFee" => Self::AccountClosureFee,
            "AddUserFee" => Self::AddUserFee,
            "AutomaticReconciliation" => Self::AutomaticReconciliation,
            "BrokerageFee" => Self::BrokerageFee,
            "CancelledPairedTrade" => Self::CancelledPairedTrade,
            "CashSettlement" => Self::CashSettlement,
            "ChallengePayout" => Self::ChallengePayout,
            "ClearingFee" => Self::ClearingFee,
            "Commission" => Self::Commission,
            "Courtesy" => Self::Courtesy,
            "CurrencyConversion" => Self::CurrencyConversion,
            "CurrencyConversionFee" => Self::CurrencyConversionFee,
            "Debit" => Self::Debit,
            "DepositFee" => Self::DepositFee,
            "DeskFee" => Self::DeskFee,
            "DormantFee" => Self::DormantFee,
            "EntitlementSubscription" => Self::EntitlementSubscription,
            "Escheatment" => Self::Escheatment,
            "ExchangeFee" => Self::ExchangeFee,
            "FundTransaction" => Self::FundTransaction,
            "FundTransactionFee" => Self::FundTransactionFee,
            "FundingRate" => Self::FundingRate,
            "GoodwillCredit" => Self::GoodwillCredit,
            "IPFee" => Self::IpFee,
            "InactivityFee" => Self::InactivityFee,
            "LiquidationFee" => Self::LiquidationFee,
            "LiquidationFee2" => Self::LiquidationFee2,
            "ManualAdjustment" => Self::ManualAdjustment,
            "MarketDataSubscription" => Self::MarketDataSubscription,
            "NewSession" => Self::NewSession,
            "NfaFee" => Self::NfaFee,
            "NsfCheckFee" => Self::NsfCheckFee,
            "OptionsTrade" => Self::OptionsTrade,
            "OrderRoutingFee" => Self::OrderRoutingFee,
            "PROMO" => Self::Promo,
            "RithmicFee" => Self::RithmicFee,
            "SeatLeasePayment" => Self::SeatLeasePayment,
            "StopPaymentFee" => Self::StopPaymentFee,
            "SwapTrade" => Self::SwapTrade,
            "ThirdPartyFee" => Self::ThirdPartyFee,
            "TradePaired" => Self::TradePaired,
            "TradovateSubscription" => Self::TradovateSubscription,
            _ => Self::Unknown(value),
        })
    }
}

/// Current wire model `CashBalanceSnapshot`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct CashBalanceSnapshot {
    #[serde(rename = "errorText", default, skip_serializing_if = "Option::is_none")]
    error_text: Option<String>,
    #[serde(
        rename = "totalCashValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    total_cash_value: Option<crate::Decimal>,
    #[serde(rename = "totalPnL", default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "crate::decimal::option")]
    total_pn_l: Option<crate::Decimal>,
    #[serde(
        rename = "initialMargin",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    initial_margin: Option<crate::Decimal>,
    #[serde(
        rename = "maintenanceMargin",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    maintenance_margin: Option<crate::Decimal>,
    #[serde(rename = "netLiq", default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "crate::decimal::option")]
    net_liq: Option<crate::Decimal>,
    #[serde(rename = "openPnL", default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "crate::decimal::option")]
    open_pn_l: Option<crate::Decimal>,
    #[serde(
        rename = "realizedPnL",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    realized_pn_l: Option<crate::Decimal>,
    #[serde(
        rename = "weekRealizedPnL",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    week_realized_pn_l: Option<crate::Decimal>,
    #[serde(
        rename = "withdrawalRejectReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    withdrawal_reject_reason: Option<CashBalanceSnapshotWithdrawalRejectReason>,
    #[serde(
        rename = "currencyCashAvailWithdrawalUSD",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    currency_cash_avail_withdrawal_usd: Option<crate::Decimal>,
    #[serde(rename = "netLiqSOD", default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "crate::decimal::option")]
    net_liq_sod: Option<crate::Decimal>,
    #[serde(
        rename = "totalCashValueSOD",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    total_cash_value_sod: Option<crate::Decimal>,
    #[serde(rename = "cashUSD", default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "crate::decimal::option")]
    cash_usd: Option<crate::Decimal>,
    #[serde(
        rename = "cashSODUSD",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    cash_sodusd: Option<crate::Decimal>,
    #[serde(
        rename = "fullInitialMargin",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    full_initial_margin: Option<crate::Decimal>,
    #[serde(
        rename = "fullInitialMarginSOD",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    full_initial_margin_sod: Option<crate::Decimal>,
    #[serde(
        rename = "autoLiqLevel",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    auto_liq_level: Option<crate::Decimal>,
}

impl CashBalanceSnapshot {
    /// Returns wire field `errorText`.
    #[must_use]
    pub fn error_text(&self) -> Option<&str> {
        self.error_text.as_deref()
    }

    /// Returns wire field `totalCashValue`.
    #[must_use]
    pub fn total_cash_value(&self) -> Option<&crate::Decimal> {
        self.total_cash_value.as_ref()
    }

    /// Returns wire field `totalPnL`.
    #[must_use]
    pub fn total_pn_l(&self) -> Option<&crate::Decimal> {
        self.total_pn_l.as_ref()
    }

    /// Returns wire field `initialMargin`.
    #[must_use]
    pub fn initial_margin(&self) -> Option<&crate::Decimal> {
        self.initial_margin.as_ref()
    }

    /// Returns wire field `maintenanceMargin`.
    #[must_use]
    pub fn maintenance_margin(&self) -> Option<&crate::Decimal> {
        self.maintenance_margin.as_ref()
    }

    /// Returns wire field `netLiq`.
    #[must_use]
    pub fn net_liq(&self) -> Option<&crate::Decimal> {
        self.net_liq.as_ref()
    }

    /// Returns wire field `openPnL`.
    #[must_use]
    pub fn open_pn_l(&self) -> Option<&crate::Decimal> {
        self.open_pn_l.as_ref()
    }

    /// Returns wire field `realizedPnL`.
    #[must_use]
    pub fn realized_pn_l(&self) -> Option<&crate::Decimal> {
        self.realized_pn_l.as_ref()
    }

    /// Returns wire field `weekRealizedPnL`.
    #[must_use]
    pub fn week_realized_pn_l(&self) -> Option<&crate::Decimal> {
        self.week_realized_pn_l.as_ref()
    }

    /// Returns wire field `withdrawalRejectReason`.
    #[must_use]
    pub fn withdrawal_reject_reason(&self) -> Option<&CashBalanceSnapshotWithdrawalRejectReason> {
        self.withdrawal_reject_reason.as_ref()
    }

    /// Returns wire field `currencyCashAvailWithdrawalUSD`.
    #[must_use]
    pub fn currency_cash_avail_withdrawal_usd(&self) -> Option<&crate::Decimal> {
        self.currency_cash_avail_withdrawal_usd.as_ref()
    }

    /// Returns wire field `netLiqSOD`.
    #[must_use]
    pub fn net_liq_sod(&self) -> Option<&crate::Decimal> {
        self.net_liq_sod.as_ref()
    }

    /// Returns wire field `totalCashValueSOD`.
    #[must_use]
    pub fn total_cash_value_sod(&self) -> Option<&crate::Decimal> {
        self.total_cash_value_sod.as_ref()
    }

    /// Returns wire field `cashUSD`.
    #[must_use]
    pub fn cash_usd(&self) -> Option<&crate::Decimal> {
        self.cash_usd.as_ref()
    }

    /// Returns wire field `cashSODUSD`.
    #[must_use]
    pub fn cash_sodusd(&self) -> Option<&crate::Decimal> {
        self.cash_sodusd.as_ref()
    }

    /// Returns wire field `fullInitialMargin`.
    #[must_use]
    pub fn full_initial_margin(&self) -> Option<&crate::Decimal> {
        self.full_initial_margin.as_ref()
    }

    /// Returns wire field `fullInitialMarginSOD`.
    #[must_use]
    pub fn full_initial_margin_sod(&self) -> Option<&crate::Decimal> {
        self.full_initial_margin_sod.as_ref()
    }

    /// Returns wire field `autoLiqLevel`.
    #[must_use]
    pub fn auto_liq_level(&self) -> Option<&crate::Decimal> {
        self.auto_liq_level.as_ref()
    }

    /// Starts a builder for [`CashBalanceSnapshot`].
    pub fn builder() -> CashBalanceSnapshotBuilder {
        CashBalanceSnapshotBuilder::default()
    }
}

/// Builder for [`CashBalanceSnapshot`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct CashBalanceSnapshotBuilder {
    error_text: Option<String>,
    total_cash_value: Option<crate::Decimal>,
    total_pn_l: Option<crate::Decimal>,
    initial_margin: Option<crate::Decimal>,
    maintenance_margin: Option<crate::Decimal>,
    net_liq: Option<crate::Decimal>,
    open_pn_l: Option<crate::Decimal>,
    realized_pn_l: Option<crate::Decimal>,
    week_realized_pn_l: Option<crate::Decimal>,
    withdrawal_reject_reason: Option<CashBalanceSnapshotWithdrawalRejectReason>,
    currency_cash_avail_withdrawal_usd: Option<crate::Decimal>,
    net_liq_sod: Option<crate::Decimal>,
    total_cash_value_sod: Option<crate::Decimal>,
    cash_usd: Option<crate::Decimal>,
    cash_sodusd: Option<crate::Decimal>,
    full_initial_margin: Option<crate::Decimal>,
    full_initial_margin_sod: Option<crate::Decimal>,
    auto_liq_level: Option<crate::Decimal>,
}

impl CashBalanceSnapshotBuilder {
    /// Sets wire field `errorText`.
    pub fn error_text(mut self, value: impl Into<String>) -> Self {
        self.error_text = Some(value.into());
        self
    }

    /// Sets wire field `totalCashValue`.
    pub fn total_cash_value(mut self, value: crate::Decimal) -> Self {
        self.total_cash_value = Some(value);
        self
    }

    /// Sets wire field `totalPnL`.
    pub fn total_pn_l(mut self, value: crate::Decimal) -> Self {
        self.total_pn_l = Some(value);
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

    /// Sets wire field `netLiq`.
    pub fn net_liq(mut self, value: crate::Decimal) -> Self {
        self.net_liq = Some(value);
        self
    }

    /// Sets wire field `openPnL`.
    pub fn open_pn_l(mut self, value: crate::Decimal) -> Self {
        self.open_pn_l = Some(value);
        self
    }

    /// Sets wire field `realizedPnL`.
    pub fn realized_pn_l(mut self, value: crate::Decimal) -> Self {
        self.realized_pn_l = Some(value);
        self
    }

    /// Sets wire field `weekRealizedPnL`.
    pub fn week_realized_pn_l(mut self, value: crate::Decimal) -> Self {
        self.week_realized_pn_l = Some(value);
        self
    }

    /// Sets wire field `withdrawalRejectReason`.
    pub fn withdrawal_reject_reason(
        mut self,
        value: CashBalanceSnapshotWithdrawalRejectReason,
    ) -> Self {
        self.withdrawal_reject_reason = Some(value);
        self
    }

    /// Sets wire field `currencyCashAvailWithdrawalUSD`.
    pub fn currency_cash_avail_withdrawal_usd(mut self, value: crate::Decimal) -> Self {
        self.currency_cash_avail_withdrawal_usd = Some(value);
        self
    }

    /// Sets wire field `netLiqSOD`.
    pub fn net_liq_sod(mut self, value: crate::Decimal) -> Self {
        self.net_liq_sod = Some(value);
        self
    }

    /// Sets wire field `totalCashValueSOD`.
    pub fn total_cash_value_sod(mut self, value: crate::Decimal) -> Self {
        self.total_cash_value_sod = Some(value);
        self
    }

    /// Sets wire field `cashUSD`.
    pub fn cash_usd(mut self, value: crate::Decimal) -> Self {
        self.cash_usd = Some(value);
        self
    }

    /// Sets wire field `cashSODUSD`.
    pub fn cash_sodusd(mut self, value: crate::Decimal) -> Self {
        self.cash_sodusd = Some(value);
        self
    }

    /// Sets wire field `fullInitialMargin`.
    pub fn full_initial_margin(mut self, value: crate::Decimal) -> Self {
        self.full_initial_margin = Some(value);
        self
    }

    /// Sets wire field `fullInitialMarginSOD`.
    pub fn full_initial_margin_sod(mut self, value: crate::Decimal) -> Self {
        self.full_initial_margin_sod = Some(value);
        self
    }

    /// Sets wire field `autoLiqLevel`.
    pub fn auto_liq_level(mut self, value: crate::Decimal) -> Self {
        self.auto_liq_level = Some(value);
        self
    }

    /// Validates required fields and builds [`CashBalanceSnapshot`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<CashBalanceSnapshot, crate::api::current::BuildError> {
        Ok(CashBalanceSnapshot {
            error_text: self.error_text,
            total_cash_value: self.total_cash_value,
            total_pn_l: self.total_pn_l,
            initial_margin: self.initial_margin,
            maintenance_margin: self.maintenance_margin,
            net_liq: self.net_liq,
            open_pn_l: self.open_pn_l,
            realized_pn_l: self.realized_pn_l,
            week_realized_pn_l: self.week_realized_pn_l,
            withdrawal_reject_reason: self.withdrawal_reject_reason,
            currency_cash_avail_withdrawal_usd: self.currency_cash_avail_withdrawal_usd,
            net_liq_sod: self.net_liq_sod,
            total_cash_value_sod: self.total_cash_value_sod,
            cash_usd: self.cash_usd,
            cash_sodusd: self.cash_sodusd,
            full_initial_margin: self.full_initial_margin,
            full_initial_margin_sod: self.full_initial_margin_sod,
            auto_liq_level: self.auto_liq_level,
        })
    }
}

/// Current provider values for `CashBalanceSnapshotWithdrawalRejectReason`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum CashBalanceSnapshotWithdrawalRejectReason {
    /// Provider value `NoData`.
    NoData,
    /// Provider value `PendingContactInfoChange`.
    PendingContactInfoChange,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl CashBalanceSnapshotWithdrawalRejectReason {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::NoData => "NoData",
            Self::PendingContactInfoChange => "PendingContactInfoChange",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for CashBalanceSnapshotWithdrawalRejectReason {
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

impl<'de> serde::Deserialize<'de> for CashBalanceSnapshotWithdrawalRejectReason {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "NoData" => Self::NoData,
            "PendingContactInfoChange" => Self::PendingContactInfoChange,
            _ => Self::Unknown(value),
        })
    }
}

/// Current wire model `ChangeDemoBalance`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ChangeDemoBalance {
    #[serde(rename = "accountId")]
    account_id: crate::AccountId,
    #[serde(rename = "cashChange")]
    #[serde(with = "crate::decimal")]
    cash_change: crate::Decimal,
    #[serde(rename = "comment", default, skip_serializing_if = "Option::is_none")]
    comment: Option<String>,
}

impl ChangeDemoBalance {
    /// Returns wire field `accountId`.
    #[must_use]
    pub fn account_id(&self) -> &crate::AccountId {
        &self.account_id
    }

    /// Returns wire field `cashChange`.
    #[must_use]
    pub fn cash_change(&self) -> &crate::Decimal {
        &self.cash_change
    }

    /// Returns wire field `comment`.
    #[must_use]
    pub fn comment(&self) -> Option<&str> {
        self.comment.as_deref()
    }

    /// Starts a builder for [`ChangeDemoBalance`].
    pub fn builder() -> ChangeDemoBalanceBuilder {
        ChangeDemoBalanceBuilder::default()
    }
}

/// Builder for [`ChangeDemoBalance`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ChangeDemoBalanceBuilder {
    account_id: Option<crate::AccountId>,
    cash_change: Option<crate::Decimal>,
    comment: Option<String>,
}

impl ChangeDemoBalanceBuilder {
    /// Sets wire field `accountId`.
    pub fn account_id(mut self, value: crate::AccountId) -> Self {
        self.account_id = Some(value);
        self
    }

    /// Sets wire field `cashChange`.
    pub fn cash_change(mut self, value: crate::Decimal) -> Self {
        self.cash_change = Some(value);
        self
    }

    /// Sets wire field `comment`.
    pub fn comment(mut self, value: impl Into<String>) -> Self {
        self.comment = Some(value.into());
        self
    }

    /// Validates required fields and builds [`ChangeDemoBalance`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ChangeDemoBalance, crate::api::current::BuildError> {
        let account_id = self
            .account_id
            .ok_or(crate::api::current::BuildError::missing("accountId"))?;
        let cash_change = self
            .cash_change
            .ok_or(crate::api::current::BuildError::missing("cashChange"))?;
        Ok(ChangeDemoBalance {
            account_id,
            cash_change,
            comment: self.comment,
        })
    }
}

impl crate::api::current::support::CurrentRequest for ChangeDemoBalance {
    fn validate_current(&self) -> Result<(), crate::Error> {
        Ok(())
    }
}

/// Current wire model `GetCashBalanceSnapshot`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct GetCashBalanceSnapshot {
    #[serde(rename = "accountId")]
    account_id: crate::AccountId,
}

impl GetCashBalanceSnapshot {
    /// Returns wire field `accountId`.
    #[must_use]
    pub fn account_id(&self) -> &crate::AccountId {
        &self.account_id
    }

    /// Starts a builder for [`GetCashBalanceSnapshot`].
    pub fn builder() -> GetCashBalanceSnapshotBuilder {
        GetCashBalanceSnapshotBuilder::default()
    }
}

/// Builder for [`GetCashBalanceSnapshot`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct GetCashBalanceSnapshotBuilder {
    account_id: Option<crate::AccountId>,
}

impl GetCashBalanceSnapshotBuilder {
    /// Sets wire field `accountId`.
    pub fn account_id(mut self, value: crate::AccountId) -> Self {
        self.account_id = Some(value);
        self
    }

    /// Validates required fields and builds [`GetCashBalanceSnapshot`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<GetCashBalanceSnapshot, crate::api::current::BuildError> {
        let account_id = self
            .account_id
            .ok_or(crate::api::current::BuildError::missing("accountId"))?;
        Ok(GetCashBalanceSnapshot { account_id })
    }
}

impl crate::api::current::support::CurrentRequest for GetCashBalanceSnapshot {
    fn validate_current(&self) -> Result<(), crate::Error> {
        Ok(())
    }
}

/// Current wire model `ResetDemoAccountState`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ResetDemoAccountState {
    #[serde(rename = "accountIds")]
    account_ids: Vec<crate::AccountId>,
    #[serde(rename = "resetTradeDate")]
    reset_trade_date: super::users::TradeDate,
}

impl ResetDemoAccountState {
    /// Returns wire field `accountIds`.
    #[must_use]
    pub fn account_ids(&self) -> &[crate::AccountId] {
        &self.account_ids
    }

    /// Returns wire field `resetTradeDate`.
    #[must_use]
    pub fn reset_trade_date(&self) -> &super::users::TradeDate {
        &self.reset_trade_date
    }

    /// Starts a builder for [`ResetDemoAccountState`].
    pub fn builder() -> ResetDemoAccountStateBuilder {
        ResetDemoAccountStateBuilder::default()
    }
}

/// Builder for [`ResetDemoAccountState`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ResetDemoAccountStateBuilder {
    account_ids: Option<Vec<crate::AccountId>>,
    reset_trade_date: Option<super::users::TradeDate>,
}

impl ResetDemoAccountStateBuilder {
    /// Sets wire field `accountIds`.
    pub fn account_ids(mut self, value: Vec<crate::AccountId>) -> Self {
        self.account_ids = Some(value);
        self
    }

    /// Sets wire field `resetTradeDate`.
    pub fn reset_trade_date(mut self, value: super::users::TradeDate) -> Self {
        self.reset_trade_date = Some(value);
        self
    }

    /// Validates required fields and builds [`ResetDemoAccountState`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ResetDemoAccountState, crate::api::current::BuildError> {
        let account_ids = self
            .account_ids
            .ok_or(crate::api::current::BuildError::missing("accountIds"))?;
        if account_ids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "accountIds",
                "must not be empty",
            ));
        }
        let reset_trade_date = self
            .reset_trade_date
            .ok_or(crate::api::current::BuildError::missing("resetTradeDate"))?;
        Ok(ResetDemoAccountState {
            account_ids,
            reset_trade_date,
        })
    }
}

impl crate::api::current::support::CurrentRequest for ResetDemoAccountState {
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

/// Typed query parameters for `/account/deps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct AccountDependentsQuery {
    #[serde(rename = "masterid")]
    masterid: super::ids::ProviderEntityId,
}

impl AccountDependentsQuery {
    /// Returns wire field `masterid`.
    #[must_use]
    pub fn masterid(&self) -> &super::ids::ProviderEntityId {
        &self.masterid
    }

    /// Starts a builder for [`AccountDependentsQuery`].
    pub fn builder() -> AccountDependentsQueryBuilder {
        AccountDependentsQueryBuilder::default()
    }
}

/// Builder for [`AccountDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct AccountDependentsQueryBuilder {
    masterid: Option<super::ids::ProviderEntityId>,
}

impl AccountDependentsQueryBuilder {
    /// Sets wire field `masterid`.
    pub fn masterid(mut self, value: super::ids::ProviderEntityId) -> Self {
        self.masterid = Some(value);
        self
    }

    /// Validates required fields and builds [`AccountDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<AccountDependentsQuery, crate::api::current::BuildError> {
        let masterid = self
            .masterid
            .ok_or(crate::api::current::BuildError::missing("masterid"))?;
        Ok(AccountDependentsQuery { masterid })
    }
}

impl crate::api::current::support::CurrentQuery for AccountDependentsQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "masterid", &self.masterid)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /account/deps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn account_dependents(
        &self,
        query: &AccountDependentsQuery,
    ) -> Result<Vec<super::users::Account>, crate::Error> {
        self.get_current("/account/deps", query).await
    }
}

/// Typed query parameters for `/account/find`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct AccountFindQuery {
    #[serde(rename = "name")]
    name: String,
}

impl AccountFindQuery {
    /// Returns wire field `name`.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Starts a builder for [`AccountFindQuery`].
    pub fn builder() -> AccountFindQueryBuilder {
        AccountFindQueryBuilder::default()
    }
}

/// Builder for [`AccountFindQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct AccountFindQueryBuilder {
    name: Option<String>,
}

impl AccountFindQueryBuilder {
    /// Sets wire field `name`.
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Validates required fields and builds [`AccountFindQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<AccountFindQuery, crate::api::current::BuildError> {
        let name = self
            .name
            .ok_or(crate::api::current::BuildError::missing("name"))?;
        if name.is_empty() || name.trim() != name {
            return Err(crate::api::current::BuildError::invalid(
                "name",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        Ok(AccountFindQuery { name })
    }
}

impl crate::api::current::support::CurrentQuery for AccountFindQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        if self.name.is_empty() || self.name.trim() != self.name {
            return Err(crate::Error::InvalidRequest {
                field: "name",
                reason: "must be non-empty and have no surrounding whitespace",
            });
        }
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "name", &self.name)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /account/find` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn account_find(
        &self,
        query: &AccountFindQuery,
    ) -> Result<super::users::Account, crate::Error> {
        self.get_current("/account/find", query).await
    }
}

/// Typed query parameters for `/account/item`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct AccountItemQuery {
    #[serde(rename = "id")]
    id: crate::AccountId,
}

impl AccountItemQuery {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> &crate::AccountId {
        &self.id
    }

    /// Starts a builder for [`AccountItemQuery`].
    pub fn builder() -> AccountItemQueryBuilder {
        AccountItemQueryBuilder::default()
    }
}

/// Builder for [`AccountItemQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct AccountItemQueryBuilder {
    id: Option<crate::AccountId>,
}

impl AccountItemQueryBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: crate::AccountId) -> Self {
        self.id = Some(value);
        self
    }

    /// Validates required fields and builds [`AccountItemQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<AccountItemQuery, crate::api::current::BuildError> {
        let id = self
            .id
            .ok_or(crate::api::current::BuildError::missing("id"))?;
        Ok(AccountItemQuery { id })
    }
}

impl crate::api::current::support::CurrentQuery for AccountItemQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "id", &self.id)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /account/item` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn account_item(
        &self,
        query: &AccountItemQuery,
    ) -> Result<super::users::Account, crate::Error> {
        self.get_current("/account/item", query).await
    }
}

/// Typed query parameters for `/account/items`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct AccountItemsQuery {
    #[serde(rename = "ids")]
    ids: Vec<crate::AccountId>,
}

impl AccountItemsQuery {
    /// Returns wire field `ids`.
    #[must_use]
    pub fn ids(&self) -> &[crate::AccountId] {
        &self.ids
    }

    /// Starts a builder for [`AccountItemsQuery`].
    pub fn builder() -> AccountItemsQueryBuilder {
        AccountItemsQueryBuilder::default()
    }
}

/// Builder for [`AccountItemsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct AccountItemsQueryBuilder {
    ids: Option<Vec<crate::AccountId>>,
}

impl AccountItemsQueryBuilder {
    /// Sets wire field `ids`.
    pub fn ids(mut self, value: Vec<crate::AccountId>) -> Self {
        self.ids = Some(value);
        self
    }

    /// Validates required fields and builds [`AccountItemsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<AccountItemsQuery, crate::api::current::BuildError> {
        let ids = self
            .ids
            .ok_or(crate::api::current::BuildError::missing("ids"))?;
        if ids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "ids",
                "must not be empty",
            ));
        }
        Ok(AccountItemsQuery { ids })
    }
}

impl crate::api::current::support::CurrentQuery for AccountItemsQuery {
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
    /// Calls the current `GET /account/items` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn account_items(
        &self,
        query: &AccountItemsQuery,
    ) -> Result<Vec<super::users::Account>, crate::Error> {
        self.get_current("/account/items", query).await
    }
}

/// Typed query parameters for `/account/ldeps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct AccountLDependentsQuery {
    #[serde(rename = "masterids")]
    masterids: Vec<super::ids::ProviderEntityId>,
}

impl AccountLDependentsQuery {
    /// Returns wire field `masterids`.
    #[must_use]
    pub fn masterids(&self) -> &[super::ids::ProviderEntityId] {
        &self.masterids
    }

    /// Starts a builder for [`AccountLDependentsQuery`].
    pub fn builder() -> AccountLDependentsQueryBuilder {
        AccountLDependentsQueryBuilder::default()
    }
}

/// Builder for [`AccountLDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct AccountLDependentsQueryBuilder {
    masterids: Option<Vec<super::ids::ProviderEntityId>>,
}

impl AccountLDependentsQueryBuilder {
    /// Sets wire field `masterids`.
    pub fn masterids(mut self, value: Vec<super::ids::ProviderEntityId>) -> Self {
        self.masterids = Some(value);
        self
    }

    /// Validates required fields and builds [`AccountLDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<AccountLDependentsQuery, crate::api::current::BuildError> {
        let masterids = self
            .masterids
            .ok_or(crate::api::current::BuildError::missing("masterids"))?;
        if masterids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "masterids",
                "must not be empty",
            ));
        }
        Ok(AccountLDependentsQuery { masterids })
    }
}

impl crate::api::current::support::CurrentQuery for AccountLDependentsQuery {
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
    /// Calls the current `GET /account/ldeps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn account_l_dependents(
        &self,
        query: &AccountLDependentsQuery,
    ) -> Result<Vec<super::users::Account>, crate::Error> {
        self.get_current("/account/ldeps", query).await
    }
}

impl crate::Client {
    /// Calls the current `GET /account/list` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn account_list(&self) -> Result<Vec<super::users::Account>, crate::Error> {
        self.get_without_query("/account/list").await
    }
}

/// Typed query parameters for `/account/suggest`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct AccountSuggestQuery {
    #[serde(rename = "t")]
    t: String,
    #[serde(rename = "l")]
    l: i64,
}

impl AccountSuggestQuery {
    /// Returns wire field `t`.
    #[must_use]
    pub fn t(&self) -> &str {
        &self.t
    }

    /// Returns wire field `l`.
    #[must_use]
    pub fn l(&self) -> &i64 {
        &self.l
    }

    /// Starts a builder for [`AccountSuggestQuery`].
    pub fn builder() -> AccountSuggestQueryBuilder {
        AccountSuggestQueryBuilder::default()
    }
}

/// Builder for [`AccountSuggestQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct AccountSuggestQueryBuilder {
    t: Option<String>,
    l: Option<i64>,
}

impl AccountSuggestQueryBuilder {
    /// Sets wire field `t`.
    pub fn t(mut self, value: impl Into<String>) -> Self {
        self.t = Some(value.into());
        self
    }

    /// Sets wire field `l`.
    pub fn l(mut self, value: i64) -> Self {
        self.l = Some(value);
        self
    }

    /// Validates required fields and builds [`AccountSuggestQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<AccountSuggestQuery, crate::api::current::BuildError> {
        let t = self
            .t
            .ok_or(crate::api::current::BuildError::missing("t"))?;
        if t.is_empty() || t.trim() != t {
            return Err(crate::api::current::BuildError::invalid(
                "t",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        let l = self
            .l
            .ok_or(crate::api::current::BuildError::missing("l"))?;
        Ok(AccountSuggestQuery { t, l })
    }
}

impl crate::api::current::support::CurrentQuery for AccountSuggestQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        if self.t.is_empty() || self.t.trim() != self.t {
            return Err(crate::Error::InvalidRequest {
                field: "t",
                reason: "must be non-empty and have no surrounding whitespace",
            });
        }
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "t", &self.t)?;
        crate::api::current::support::push_query_value(&mut pairs, "l", &self.l)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /account/suggest` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn account_suggest(
        &self,
        query: &AccountSuggestQuery,
    ) -> Result<Vec<super::users::Account>, crate::Error> {
        self.get_current("/account/suggest", query).await
    }
}

/// Typed query parameters for `/cashBalance/deps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct CashBalanceDependentsQuery {
    #[serde(rename = "masterid")]
    masterid: super::ids::ProviderEntityId,
}

impl CashBalanceDependentsQuery {
    /// Returns wire field `masterid`.
    #[must_use]
    pub fn masterid(&self) -> &super::ids::ProviderEntityId {
        &self.masterid
    }

    /// Starts a builder for [`CashBalanceDependentsQuery`].
    pub fn builder() -> CashBalanceDependentsQueryBuilder {
        CashBalanceDependentsQueryBuilder::default()
    }
}

/// Builder for [`CashBalanceDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct CashBalanceDependentsQueryBuilder {
    masterid: Option<super::ids::ProviderEntityId>,
}

impl CashBalanceDependentsQueryBuilder {
    /// Sets wire field `masterid`.
    pub fn masterid(mut self, value: super::ids::ProviderEntityId) -> Self {
        self.masterid = Some(value);
        self
    }

    /// Validates required fields and builds [`CashBalanceDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<CashBalanceDependentsQuery, crate::api::current::BuildError> {
        let masterid = self
            .masterid
            .ok_or(crate::api::current::BuildError::missing("masterid"))?;
        Ok(CashBalanceDependentsQuery { masterid })
    }
}

impl crate::api::current::support::CurrentQuery for CashBalanceDependentsQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "masterid", &self.masterid)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /cashBalance/deps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn cash_balance_dependents(
        &self,
        query: &CashBalanceDependentsQuery,
    ) -> Result<Vec<super::users::CashBalance>, crate::Error> {
        self.get_current("/cashBalance/deps", query).await
    }
}

/// Typed query parameters for `/cashBalance/item`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct CashBalanceItemQuery {
    #[serde(rename = "id")]
    id: super::ids::CashBalanceId,
}

impl CashBalanceItemQuery {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> &super::ids::CashBalanceId {
        &self.id
    }

    /// Starts a builder for [`CashBalanceItemQuery`].
    pub fn builder() -> CashBalanceItemQueryBuilder {
        CashBalanceItemQueryBuilder::default()
    }
}

/// Builder for [`CashBalanceItemQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct CashBalanceItemQueryBuilder {
    id: Option<super::ids::CashBalanceId>,
}

impl CashBalanceItemQueryBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::CashBalanceId) -> Self {
        self.id = Some(value);
        self
    }

    /// Validates required fields and builds [`CashBalanceItemQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<CashBalanceItemQuery, crate::api::current::BuildError> {
        let id = self
            .id
            .ok_or(crate::api::current::BuildError::missing("id"))?;
        Ok(CashBalanceItemQuery { id })
    }
}

impl crate::api::current::support::CurrentQuery for CashBalanceItemQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "id", &self.id)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /cashBalance/item` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn cash_balance_item(
        &self,
        query: &CashBalanceItemQuery,
    ) -> Result<super::users::CashBalance, crate::Error> {
        self.get_current("/cashBalance/item", query).await
    }
}

/// Typed query parameters for `/cashBalance/items`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct CashBalanceItemsQuery {
    #[serde(rename = "ids")]
    ids: Vec<super::ids::CashBalanceId>,
}

impl CashBalanceItemsQuery {
    /// Returns wire field `ids`.
    #[must_use]
    pub fn ids(&self) -> &[super::ids::CashBalanceId] {
        &self.ids
    }

    /// Starts a builder for [`CashBalanceItemsQuery`].
    pub fn builder() -> CashBalanceItemsQueryBuilder {
        CashBalanceItemsQueryBuilder::default()
    }
}

/// Builder for [`CashBalanceItemsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct CashBalanceItemsQueryBuilder {
    ids: Option<Vec<super::ids::CashBalanceId>>,
}

impl CashBalanceItemsQueryBuilder {
    /// Sets wire field `ids`.
    pub fn ids(mut self, value: Vec<super::ids::CashBalanceId>) -> Self {
        self.ids = Some(value);
        self
    }

    /// Validates required fields and builds [`CashBalanceItemsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<CashBalanceItemsQuery, crate::api::current::BuildError> {
        let ids = self
            .ids
            .ok_or(crate::api::current::BuildError::missing("ids"))?;
        if ids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "ids",
                "must not be empty",
            ));
        }
        Ok(CashBalanceItemsQuery { ids })
    }
}

impl crate::api::current::support::CurrentQuery for CashBalanceItemsQuery {
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
    /// Calls the current `GET /cashBalance/items` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn cash_balance_items(
        &self,
        query: &CashBalanceItemsQuery,
    ) -> Result<Vec<super::users::CashBalance>, crate::Error> {
        self.get_current("/cashBalance/items", query).await
    }
}

/// Typed query parameters for `/cashBalance/ldeps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct CashBalanceLDependentsQuery {
    #[serde(rename = "masterids")]
    masterids: Vec<super::ids::ProviderEntityId>,
}

impl CashBalanceLDependentsQuery {
    /// Returns wire field `masterids`.
    #[must_use]
    pub fn masterids(&self) -> &[super::ids::ProviderEntityId] {
        &self.masterids
    }

    /// Starts a builder for [`CashBalanceLDependentsQuery`].
    pub fn builder() -> CashBalanceLDependentsQueryBuilder {
        CashBalanceLDependentsQueryBuilder::default()
    }
}

/// Builder for [`CashBalanceLDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct CashBalanceLDependentsQueryBuilder {
    masterids: Option<Vec<super::ids::ProviderEntityId>>,
}

impl CashBalanceLDependentsQueryBuilder {
    /// Sets wire field `masterids`.
    pub fn masterids(mut self, value: Vec<super::ids::ProviderEntityId>) -> Self {
        self.masterids = Some(value);
        self
    }

    /// Validates required fields and builds [`CashBalanceLDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<CashBalanceLDependentsQuery, crate::api::current::BuildError> {
        let masterids = self
            .masterids
            .ok_or(crate::api::current::BuildError::missing("masterids"))?;
        if masterids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "masterids",
                "must not be empty",
            ));
        }
        Ok(CashBalanceLDependentsQuery { masterids })
    }
}

impl crate::api::current::support::CurrentQuery for CashBalanceLDependentsQuery {
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
    /// Calls the current `GET /cashBalance/ldeps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn cash_balance_l_dependents(
        &self,
        query: &CashBalanceLDependentsQuery,
    ) -> Result<Vec<super::users::CashBalance>, crate::Error> {
        self.get_current("/cashBalance/ldeps", query).await
    }
}

impl crate::Client {
    /// Calls the current `GET /cashBalance/list` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn cash_balance_list(&self) -> Result<Vec<super::users::CashBalance>, crate::Error> {
        self.get_without_query("/cashBalance/list").await
    }
}

/// Typed query parameters for `/cashBalanceLog/deps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct CashBalanceLogDependentsQuery {
    #[serde(rename = "masterid")]
    masterid: super::ids::ProviderEntityId,
}

impl CashBalanceLogDependentsQuery {
    /// Returns wire field `masterid`.
    #[must_use]
    pub fn masterid(&self) -> &super::ids::ProviderEntityId {
        &self.masterid
    }

    /// Starts a builder for [`CashBalanceLogDependentsQuery`].
    pub fn builder() -> CashBalanceLogDependentsQueryBuilder {
        CashBalanceLogDependentsQueryBuilder::default()
    }
}

/// Builder for [`CashBalanceLogDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct CashBalanceLogDependentsQueryBuilder {
    masterid: Option<super::ids::ProviderEntityId>,
}

impl CashBalanceLogDependentsQueryBuilder {
    /// Sets wire field `masterid`.
    pub fn masterid(mut self, value: super::ids::ProviderEntityId) -> Self {
        self.masterid = Some(value);
        self
    }

    /// Validates required fields and builds [`CashBalanceLogDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<CashBalanceLogDependentsQuery, crate::api::current::BuildError> {
        let masterid = self
            .masterid
            .ok_or(crate::api::current::BuildError::missing("masterid"))?;
        Ok(CashBalanceLogDependentsQuery { masterid })
    }
}

impl crate::api::current::support::CurrentQuery for CashBalanceLogDependentsQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "masterid", &self.masterid)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /cashBalanceLog/deps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn cash_balance_log_dependents(
        &self,
        query: &CashBalanceLogDependentsQuery,
    ) -> Result<Vec<CashBalanceLog>, crate::Error> {
        self.get_current("/cashBalanceLog/deps", query).await
    }
}

/// Typed query parameters for `/cashBalanceLog/item`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct CashBalanceLogItemQuery {
    #[serde(rename = "id")]
    id: super::ids::CashBalanceLogId,
}

impl CashBalanceLogItemQuery {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> &super::ids::CashBalanceLogId {
        &self.id
    }

    /// Starts a builder for [`CashBalanceLogItemQuery`].
    pub fn builder() -> CashBalanceLogItemQueryBuilder {
        CashBalanceLogItemQueryBuilder::default()
    }
}

/// Builder for [`CashBalanceLogItemQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct CashBalanceLogItemQueryBuilder {
    id: Option<super::ids::CashBalanceLogId>,
}

impl CashBalanceLogItemQueryBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::CashBalanceLogId) -> Self {
        self.id = Some(value);
        self
    }

    /// Validates required fields and builds [`CashBalanceLogItemQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<CashBalanceLogItemQuery, crate::api::current::BuildError> {
        let id = self
            .id
            .ok_or(crate::api::current::BuildError::missing("id"))?;
        Ok(CashBalanceLogItemQuery { id })
    }
}

impl crate::api::current::support::CurrentQuery for CashBalanceLogItemQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "id", &self.id)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /cashBalanceLog/item` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn cash_balance_log_item(
        &self,
        query: &CashBalanceLogItemQuery,
    ) -> Result<CashBalanceLog, crate::Error> {
        self.get_current("/cashBalanceLog/item", query).await
    }
}

/// Typed query parameters for `/cashBalanceLog/items`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct CashBalanceLogItemsQuery {
    #[serde(rename = "ids")]
    ids: Vec<super::ids::CashBalanceLogId>,
}

impl CashBalanceLogItemsQuery {
    /// Returns wire field `ids`.
    #[must_use]
    pub fn ids(&self) -> &[super::ids::CashBalanceLogId] {
        &self.ids
    }

    /// Starts a builder for [`CashBalanceLogItemsQuery`].
    pub fn builder() -> CashBalanceLogItemsQueryBuilder {
        CashBalanceLogItemsQueryBuilder::default()
    }
}

/// Builder for [`CashBalanceLogItemsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct CashBalanceLogItemsQueryBuilder {
    ids: Option<Vec<super::ids::CashBalanceLogId>>,
}

impl CashBalanceLogItemsQueryBuilder {
    /// Sets wire field `ids`.
    pub fn ids(mut self, value: Vec<super::ids::CashBalanceLogId>) -> Self {
        self.ids = Some(value);
        self
    }

    /// Validates required fields and builds [`CashBalanceLogItemsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<CashBalanceLogItemsQuery, crate::api::current::BuildError> {
        let ids = self
            .ids
            .ok_or(crate::api::current::BuildError::missing("ids"))?;
        if ids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "ids",
                "must not be empty",
            ));
        }
        Ok(CashBalanceLogItemsQuery { ids })
    }
}

impl crate::api::current::support::CurrentQuery for CashBalanceLogItemsQuery {
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
    /// Calls the current `GET /cashBalanceLog/items` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn cash_balance_log_items(
        &self,
        query: &CashBalanceLogItemsQuery,
    ) -> Result<Vec<CashBalanceLog>, crate::Error> {
        self.get_current("/cashBalanceLog/items", query).await
    }
}

/// Typed query parameters for `/cashBalanceLog/ldeps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct CashBalanceLogLDependentsQuery {
    #[serde(rename = "masterids")]
    masterids: Vec<super::ids::ProviderEntityId>,
}

impl CashBalanceLogLDependentsQuery {
    /// Returns wire field `masterids`.
    #[must_use]
    pub fn masterids(&self) -> &[super::ids::ProviderEntityId] {
        &self.masterids
    }

    /// Starts a builder for [`CashBalanceLogLDependentsQuery`].
    pub fn builder() -> CashBalanceLogLDependentsQueryBuilder {
        CashBalanceLogLDependentsQueryBuilder::default()
    }
}

/// Builder for [`CashBalanceLogLDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct CashBalanceLogLDependentsQueryBuilder {
    masterids: Option<Vec<super::ids::ProviderEntityId>>,
}

impl CashBalanceLogLDependentsQueryBuilder {
    /// Sets wire field `masterids`.
    pub fn masterids(mut self, value: Vec<super::ids::ProviderEntityId>) -> Self {
        self.masterids = Some(value);
        self
    }

    /// Validates required fields and builds [`CashBalanceLogLDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<CashBalanceLogLDependentsQuery, crate::api::current::BuildError> {
        let masterids = self
            .masterids
            .ok_or(crate::api::current::BuildError::missing("masterids"))?;
        if masterids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "masterids",
                "must not be empty",
            ));
        }
        Ok(CashBalanceLogLDependentsQuery { masterids })
    }
}

impl crate::api::current::support::CurrentQuery for CashBalanceLogLDependentsQuery {
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
    /// Calls the current `GET /cashBalanceLog/ldeps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn cash_balance_log_l_dependents(
        &self,
        query: &CashBalanceLogLDependentsQuery,
    ) -> Result<Vec<CashBalanceLog>, crate::Error> {
        self.get_current("/cashBalanceLog/ldeps", query).await
    }
}

impl crate::Client {
    /// Calls the current `POST /cashBalance/getcashbalancesnapshot` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn cash_balance_get_cash_balance_snapshot(
        &self,
        request: &GetCashBalanceSnapshot,
    ) -> Result<CashBalanceSnapshot, crate::Error> {
        crate::api::current::support::CurrentRequest::validate_current(request)?;
        self.post_query("/cashBalance/getcashbalancesnapshot", request)
            .await
    }
}

/// Typed query parameters for `/marginSnapshot/deps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct MarginSnapshotDependentsQuery {
    #[serde(rename = "masterid")]
    masterid: super::ids::ProviderEntityId,
}

impl MarginSnapshotDependentsQuery {
    /// Returns wire field `masterid`.
    #[must_use]
    pub fn masterid(&self) -> &super::ids::ProviderEntityId {
        &self.masterid
    }

    /// Starts a builder for [`MarginSnapshotDependentsQuery`].
    pub fn builder() -> MarginSnapshotDependentsQueryBuilder {
        MarginSnapshotDependentsQueryBuilder::default()
    }
}

/// Builder for [`MarginSnapshotDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct MarginSnapshotDependentsQueryBuilder {
    masterid: Option<super::ids::ProviderEntityId>,
}

impl MarginSnapshotDependentsQueryBuilder {
    /// Sets wire field `masterid`.
    pub fn masterid(mut self, value: super::ids::ProviderEntityId) -> Self {
        self.masterid = Some(value);
        self
    }

    /// Validates required fields and builds [`MarginSnapshotDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<MarginSnapshotDependentsQuery, crate::api::current::BuildError> {
        let masterid = self
            .masterid
            .ok_or(crate::api::current::BuildError::missing("masterid"))?;
        Ok(MarginSnapshotDependentsQuery { masterid })
    }
}

impl crate::api::current::support::CurrentQuery for MarginSnapshotDependentsQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "masterid", &self.masterid)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /marginSnapshot/deps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn margin_snapshot_dependents(
        &self,
        query: &MarginSnapshotDependentsQuery,
    ) -> Result<Vec<super::users::MarginSnapshot>, crate::Error> {
        self.get_current("/marginSnapshot/deps", query).await
    }
}

/// Typed query parameters for `/marginSnapshot/item`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct MarginSnapshotItemQuery {
    #[serde(rename = "id")]
    id: super::ids::MarginSnapshotId,
}

impl MarginSnapshotItemQuery {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> &super::ids::MarginSnapshotId {
        &self.id
    }

    /// Starts a builder for [`MarginSnapshotItemQuery`].
    pub fn builder() -> MarginSnapshotItemQueryBuilder {
        MarginSnapshotItemQueryBuilder::default()
    }
}

/// Builder for [`MarginSnapshotItemQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct MarginSnapshotItemQueryBuilder {
    id: Option<super::ids::MarginSnapshotId>,
}

impl MarginSnapshotItemQueryBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::MarginSnapshotId) -> Self {
        self.id = Some(value);
        self
    }

    /// Validates required fields and builds [`MarginSnapshotItemQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<MarginSnapshotItemQuery, crate::api::current::BuildError> {
        let id = self
            .id
            .ok_or(crate::api::current::BuildError::missing("id"))?;
        Ok(MarginSnapshotItemQuery { id })
    }
}

impl crate::api::current::support::CurrentQuery for MarginSnapshotItemQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "id", &self.id)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /marginSnapshot/item` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn margin_snapshot_item(
        &self,
        query: &MarginSnapshotItemQuery,
    ) -> Result<super::users::MarginSnapshot, crate::Error> {
        self.get_current("/marginSnapshot/item", query).await
    }
}

/// Typed query parameters for `/marginSnapshot/items`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct MarginSnapshotItemsQuery {
    #[serde(rename = "ids")]
    ids: Vec<super::ids::MarginSnapshotId>,
}

impl MarginSnapshotItemsQuery {
    /// Returns wire field `ids`.
    #[must_use]
    pub fn ids(&self) -> &[super::ids::MarginSnapshotId] {
        &self.ids
    }

    /// Starts a builder for [`MarginSnapshotItemsQuery`].
    pub fn builder() -> MarginSnapshotItemsQueryBuilder {
        MarginSnapshotItemsQueryBuilder::default()
    }
}

/// Builder for [`MarginSnapshotItemsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct MarginSnapshotItemsQueryBuilder {
    ids: Option<Vec<super::ids::MarginSnapshotId>>,
}

impl MarginSnapshotItemsQueryBuilder {
    /// Sets wire field `ids`.
    pub fn ids(mut self, value: Vec<super::ids::MarginSnapshotId>) -> Self {
        self.ids = Some(value);
        self
    }

    /// Validates required fields and builds [`MarginSnapshotItemsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<MarginSnapshotItemsQuery, crate::api::current::BuildError> {
        let ids = self
            .ids
            .ok_or(crate::api::current::BuildError::missing("ids"))?;
        if ids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "ids",
                "must not be empty",
            ));
        }
        Ok(MarginSnapshotItemsQuery { ids })
    }
}

impl crate::api::current::support::CurrentQuery for MarginSnapshotItemsQuery {
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
    /// Calls the current `GET /marginSnapshot/items` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn margin_snapshot_items(
        &self,
        query: &MarginSnapshotItemsQuery,
    ) -> Result<Vec<super::users::MarginSnapshot>, crate::Error> {
        self.get_current("/marginSnapshot/items", query).await
    }
}

/// Typed query parameters for `/marginSnapshot/ldeps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct MarginSnapshotLDependentsQuery {
    #[serde(rename = "masterids")]
    masterids: Vec<super::ids::ProviderEntityId>,
}

impl MarginSnapshotLDependentsQuery {
    /// Returns wire field `masterids`.
    #[must_use]
    pub fn masterids(&self) -> &[super::ids::ProviderEntityId] {
        &self.masterids
    }

    /// Starts a builder for [`MarginSnapshotLDependentsQuery`].
    pub fn builder() -> MarginSnapshotLDependentsQueryBuilder {
        MarginSnapshotLDependentsQueryBuilder::default()
    }
}

/// Builder for [`MarginSnapshotLDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct MarginSnapshotLDependentsQueryBuilder {
    masterids: Option<Vec<super::ids::ProviderEntityId>>,
}

impl MarginSnapshotLDependentsQueryBuilder {
    /// Sets wire field `masterids`.
    pub fn masterids(mut self, value: Vec<super::ids::ProviderEntityId>) -> Self {
        self.masterids = Some(value);
        self
    }

    /// Validates required fields and builds [`MarginSnapshotLDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<MarginSnapshotLDependentsQuery, crate::api::current::BuildError> {
        let masterids = self
            .masterids
            .ok_or(crate::api::current::BuildError::missing("masterids"))?;
        if masterids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "masterids",
                "must not be empty",
            ));
        }
        Ok(MarginSnapshotLDependentsQuery { masterids })
    }
}

impl crate::api::current::support::CurrentQuery for MarginSnapshotLDependentsQuery {
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
    /// Calls the current `GET /marginSnapshot/ldeps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn margin_snapshot_l_dependents(
        &self,
        query: &MarginSnapshotLDependentsQuery,
    ) -> Result<Vec<super::users::MarginSnapshot>, crate::Error> {
        self.get_current("/marginSnapshot/ldeps", query).await
    }
}

impl crate::Client {
    /// Calls the current `GET /marginSnapshot/list` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn margin_snapshot_list(
        &self,
    ) -> Result<Vec<super::users::MarginSnapshot>, crate::Error> {
        self.get_without_query("/marginSnapshot/list").await
    }
}

/// Typed query parameters for `/tradingPermission/deps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct TradingPermissionDependentsQuery {
    #[serde(rename = "masterid")]
    masterid: super::ids::ProviderEntityId,
}

impl TradingPermissionDependentsQuery {
    /// Returns wire field `masterid`.
    #[must_use]
    pub fn masterid(&self) -> &super::ids::ProviderEntityId {
        &self.masterid
    }

    /// Starts a builder for [`TradingPermissionDependentsQuery`].
    pub fn builder() -> TradingPermissionDependentsQueryBuilder {
        TradingPermissionDependentsQueryBuilder::default()
    }
}

/// Builder for [`TradingPermissionDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct TradingPermissionDependentsQueryBuilder {
    masterid: Option<super::ids::ProviderEntityId>,
}

impl TradingPermissionDependentsQueryBuilder {
    /// Sets wire field `masterid`.
    pub fn masterid(mut self, value: super::ids::ProviderEntityId) -> Self {
        self.masterid = Some(value);
        self
    }

    /// Validates required fields and builds [`TradingPermissionDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(
        self,
    ) -> Result<TradingPermissionDependentsQuery, crate::api::current::BuildError> {
        let masterid = self
            .masterid
            .ok_or(crate::api::current::BuildError::missing("masterid"))?;
        Ok(TradingPermissionDependentsQuery { masterid })
    }
}

impl crate::api::current::support::CurrentQuery for TradingPermissionDependentsQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "masterid", &self.masterid)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /tradingPermission/deps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn trading_permission_dependents(
        &self,
        query: &TradingPermissionDependentsQuery,
    ) -> Result<Vec<super::users::TradingPermission>, crate::Error> {
        self.get_current("/tradingPermission/deps", query).await
    }
}

/// Typed query parameters for `/tradingPermission/item`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct TradingPermissionItemQuery {
    #[serde(rename = "id")]
    id: super::ids::TradingPermissionId,
}

impl TradingPermissionItemQuery {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> &super::ids::TradingPermissionId {
        &self.id
    }

    /// Starts a builder for [`TradingPermissionItemQuery`].
    pub fn builder() -> TradingPermissionItemQueryBuilder {
        TradingPermissionItemQueryBuilder::default()
    }
}

/// Builder for [`TradingPermissionItemQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct TradingPermissionItemQueryBuilder {
    id: Option<super::ids::TradingPermissionId>,
}

impl TradingPermissionItemQueryBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::TradingPermissionId) -> Self {
        self.id = Some(value);
        self
    }

    /// Validates required fields and builds [`TradingPermissionItemQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<TradingPermissionItemQuery, crate::api::current::BuildError> {
        let id = self
            .id
            .ok_or(crate::api::current::BuildError::missing("id"))?;
        Ok(TradingPermissionItemQuery { id })
    }
}

impl crate::api::current::support::CurrentQuery for TradingPermissionItemQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "id", &self.id)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /tradingPermission/item` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn trading_permission_item(
        &self,
        query: &TradingPermissionItemQuery,
    ) -> Result<super::users::TradingPermission, crate::Error> {
        self.get_current("/tradingPermission/item", query).await
    }
}

/// Typed query parameters for `/tradingPermission/items`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct TradingPermissionItemsQuery {
    #[serde(rename = "ids")]
    ids: Vec<super::ids::TradingPermissionId>,
}

impl TradingPermissionItemsQuery {
    /// Returns wire field `ids`.
    #[must_use]
    pub fn ids(&self) -> &[super::ids::TradingPermissionId] {
        &self.ids
    }

    /// Starts a builder for [`TradingPermissionItemsQuery`].
    pub fn builder() -> TradingPermissionItemsQueryBuilder {
        TradingPermissionItemsQueryBuilder::default()
    }
}

/// Builder for [`TradingPermissionItemsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct TradingPermissionItemsQueryBuilder {
    ids: Option<Vec<super::ids::TradingPermissionId>>,
}

impl TradingPermissionItemsQueryBuilder {
    /// Sets wire field `ids`.
    pub fn ids(mut self, value: Vec<super::ids::TradingPermissionId>) -> Self {
        self.ids = Some(value);
        self
    }

    /// Validates required fields and builds [`TradingPermissionItemsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<TradingPermissionItemsQuery, crate::api::current::BuildError> {
        let ids = self
            .ids
            .ok_or(crate::api::current::BuildError::missing("ids"))?;
        if ids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "ids",
                "must not be empty",
            ));
        }
        Ok(TradingPermissionItemsQuery { ids })
    }
}

impl crate::api::current::support::CurrentQuery for TradingPermissionItemsQuery {
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
    /// Calls the current `GET /tradingPermission/items` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn trading_permission_items(
        &self,
        query: &TradingPermissionItemsQuery,
    ) -> Result<Vec<super::users::TradingPermission>, crate::Error> {
        self.get_current("/tradingPermission/items", query).await
    }
}

/// Typed query parameters for `/tradingPermission/ldeps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct TradingPermissionLDependentsQuery {
    #[serde(rename = "masterids")]
    masterids: Vec<super::ids::ProviderEntityId>,
}

impl TradingPermissionLDependentsQuery {
    /// Returns wire field `masterids`.
    #[must_use]
    pub fn masterids(&self) -> &[super::ids::ProviderEntityId] {
        &self.masterids
    }

    /// Starts a builder for [`TradingPermissionLDependentsQuery`].
    pub fn builder() -> TradingPermissionLDependentsQueryBuilder {
        TradingPermissionLDependentsQueryBuilder::default()
    }
}

/// Builder for [`TradingPermissionLDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct TradingPermissionLDependentsQueryBuilder {
    masterids: Option<Vec<super::ids::ProviderEntityId>>,
}

impl TradingPermissionLDependentsQueryBuilder {
    /// Sets wire field `masterids`.
    pub fn masterids(mut self, value: Vec<super::ids::ProviderEntityId>) -> Self {
        self.masterids = Some(value);
        self
    }

    /// Validates required fields and builds [`TradingPermissionLDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(
        self,
    ) -> Result<TradingPermissionLDependentsQuery, crate::api::current::BuildError> {
        let masterids = self
            .masterids
            .ok_or(crate::api::current::BuildError::missing("masterids"))?;
        if masterids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "masterids",
                "must not be empty",
            ));
        }
        Ok(TradingPermissionLDependentsQuery { masterids })
    }
}

impl crate::api::current::support::CurrentQuery for TradingPermissionLDependentsQuery {
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
    /// Calls the current `GET /tradingPermission/ldeps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn trading_permission_l_dependents(
        &self,
        query: &TradingPermissionLDependentsQuery,
    ) -> Result<Vec<super::users::TradingPermission>, crate::Error> {
        self.get_current("/tradingPermission/ldeps", query).await
    }
}

impl crate::Client {
    /// Calls the current `GET /tradingPermission/list` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn trading_permission_list(
        &self,
    ) -> Result<Vec<super::users::TradingPermission>, crate::Error> {
        self.get_without_query("/tradingPermission/list").await
    }
}

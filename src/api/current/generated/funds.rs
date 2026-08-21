// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary
// @generated
// Generator: tools/generate_openapi.py
// Source: https://partner.tradovate.com/openapi.json (snapshot 2026-08-21, sha256 37caeccf4b0913460a788fcaf4c902497059b8ffe6f6355512e6c08eaacde769)

// Provider wire fields remain schema-auditable even when they repeat
// their type name; wide schema-faithful builders remain one generated
// unit so regeneration and source review cannot drift field subsets.
#![allow(clippy::struct_field_names, clippy::too_many_lines)]

//! Current fund-adjustment operations.

/// Current wire model `AdjustCash`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct AdjustCash {
    #[serde(rename = "accountId")]
    account_id: crate::AccountId,
    #[serde(rename = "cashChange")]
    #[serde(with = "crate::decimal")]
    cash_change: crate::Decimal,
    #[serde(rename = "cashChangeType")]
    cash_change_type: AdjustCashCashChangeType,
    #[serde(rename = "currencyId")]
    currency_id: super::ids::CurrencyId,
    #[serde(rename = "comment", default, skip_serializing_if = "Option::is_none")]
    comment: Option<String>,
    #[serde(rename = "senderId", default, skip_serializing_if = "Option::is_none")]
    sender_id: Option<super::ids::SenderId>,
}

impl AdjustCash {
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

    /// Returns wire field `cashChangeType`.
    #[must_use]
    pub fn cash_change_type(&self) -> &AdjustCashCashChangeType {
        &self.cash_change_type
    }

    /// Returns wire field `currencyId`.
    #[must_use]
    pub fn currency_id(&self) -> &super::ids::CurrencyId {
        &self.currency_id
    }

    /// Returns wire field `comment`.
    #[must_use]
    pub fn comment(&self) -> Option<&str> {
        self.comment.as_deref()
    }

    /// Returns wire field `senderId`.
    #[must_use]
    pub fn sender_id(&self) -> Option<&super::ids::SenderId> {
        self.sender_id.as_ref()
    }

    /// Starts a builder for [`AdjustCash`].
    pub fn builder() -> AdjustCashBuilder {
        AdjustCashBuilder::default()
    }
}

/// Builder for [`AdjustCash`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct AdjustCashBuilder {
    account_id: Option<crate::AccountId>,
    cash_change: Option<crate::Decimal>,
    cash_change_type: Option<AdjustCashCashChangeType>,
    currency_id: Option<super::ids::CurrencyId>,
    comment: Option<String>,
    sender_id: Option<super::ids::SenderId>,
}

impl AdjustCashBuilder {
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

    /// Sets wire field `cashChangeType`.
    pub fn cash_change_type(mut self, value: AdjustCashCashChangeType) -> Self {
        self.cash_change_type = Some(value);
        self
    }

    /// Sets wire field `currencyId`.
    pub fn currency_id(mut self, value: super::ids::CurrencyId) -> Self {
        self.currency_id = Some(value);
        self
    }

    /// Sets wire field `comment`.
    pub fn comment(mut self, value: impl Into<String>) -> Self {
        self.comment = Some(value.into());
        self
    }

    /// Sets wire field `senderId`.
    pub fn sender_id(mut self, value: super::ids::SenderId) -> Self {
        self.sender_id = Some(value);
        self
    }

    /// Validates required fields and builds [`AdjustCash`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<AdjustCash, crate::api::current::BuildError> {
        let account_id = self
            .account_id
            .ok_or(crate::api::current::BuildError::missing("accountId"))?;
        let cash_change = self
            .cash_change
            .ok_or(crate::api::current::BuildError::missing("cashChange"))?;
        let cash_change_type = self
            .cash_change_type
            .ok_or(crate::api::current::BuildError::missing("cashChangeType"))?;
        let currency_id = self
            .currency_id
            .ok_or(crate::api::current::BuildError::missing("currencyId"))?;
        Ok(AdjustCash {
            account_id,
            cash_change,
            cash_change_type,
            currency_id,
            comment: self.comment,
            sender_id: self.sender_id,
        })
    }
}

impl crate::api::current::support::CurrentRequest for AdjustCash {
    fn validate_current(&self) -> Result<(), crate::Error> {
        Ok(())
    }
}

/// Current provider values for `AdjustCashCashChangeType`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum AdjustCashCashChangeType {
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

impl AdjustCashCashChangeType {
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

impl serde::Serialize for AdjustCashCashChangeType {
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

impl<'de> serde::Deserialize<'de> for AdjustCashCashChangeType {
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

// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0
// @generated
// Generator: tools/generate_openapi.py
// Source: https://partner.tradovate.com/openapi.json (snapshot 2026-08-21, sha256 37caeccf4b0913460a788fcaf4c902497059b8ffe6f6355512e6c08eaacde769)

// Provider wire fields remain schema-auditable even when they repeat
// their type name; wide schema-faithful builders remain one generated
// unit so regeneration and source review cannot drift field subsets.
#![allow(clippy::struct_field_names, clippy::too_many_lines)]

//! Current user, subscription, and contact operations.

/// Current wire model `AcceptTradingPermission`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct AcceptTradingPermission {
    #[serde(rename = "tradingPermissionId")]
    trading_permission_id: super::ids::TradingPermissionId,
}

impl AcceptTradingPermission {
    /// Returns wire field `tradingPermissionId`.
    #[must_use]
    pub fn trading_permission_id(&self) -> &super::ids::TradingPermissionId {
        &self.trading_permission_id
    }

    /// Starts a builder for [`AcceptTradingPermission`].
    pub fn builder() -> AcceptTradingPermissionBuilder {
        AcceptTradingPermissionBuilder::default()
    }
}

/// Builder for [`AcceptTradingPermission`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct AcceptTradingPermissionBuilder {
    trading_permission_id: Option<super::ids::TradingPermissionId>,
}

impl AcceptTradingPermissionBuilder {
    /// Sets wire field `tradingPermissionId`.
    pub fn trading_permission_id(mut self, value: super::ids::TradingPermissionId) -> Self {
        self.trading_permission_id = Some(value);
        self
    }

    /// Validates required fields and builds [`AcceptTradingPermission`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<AcceptTradingPermission, crate::api::current::BuildError> {
        let trading_permission_id =
            self.trading_permission_id
                .ok_or(crate::api::current::BuildError::missing(
                    "tradingPermissionId",
                ))?;
        Ok(AcceptTradingPermission {
            trading_permission_id,
        })
    }
}

impl crate::api::current::support::CurrentRequest for AcceptTradingPermission {
    fn validate_current(&self) -> Result<(), crate::Error> {
        Ok(())
    }
}

/// Current wire model `AccessTokenResponse`.
#[derive(Clone, Debug, serde::Deserialize)]
#[non_exhaustive]
pub struct AccessTokenResponse {
    #[serde(rename = "errorText", default, skip_serializing_if = "Option::is_none")]
    error_text: Option<String>,
    #[serde(rename = "hibpHint", default, skip_serializing_if = "Option::is_none")]
    hibp_hint: Option<AccessTokenResponseHibpHint>,
    #[serde(
        rename = "accessToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    access_token: Option<crate::api::current::SecretValue>,
    #[serde(
        rename = "expirationTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    expiration_time: Option<jiff::Timestamp>,
    #[serde(
        rename = "passwordExpirationTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    password_expiration_time: Option<jiff::Timestamp>,
    #[serde(
        rename = "userStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    user_status: Option<AccessTokenResponseUserStatus>,
    #[serde(rename = "userId", default, skip_serializing_if = "Option::is_none")]
    user_id: Option<crate::UserId>,
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    name: Option<crate::api::current::SecretValue>,
    #[serde(rename = "hasLive", default, skip_serializing_if = "Option::is_none")]
    has_live: Option<bool>,
    #[serde(
        rename = "hasSimPlus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    has_sim_plus: Option<bool>,
    #[serde(rename = "showKIDs", default, skip_serializing_if = "Option::is_none")]
    show_ki_ds: Option<bool>,
}

impl AccessTokenResponse {
    /// Returns wire field `errorText`.
    #[must_use]
    pub fn error_text(&self) -> Option<&str> {
        self.error_text.as_deref()
    }

    /// Returns wire field `hibpHint`.
    #[must_use]
    pub fn hibp_hint(&self) -> Option<&AccessTokenResponseHibpHint> {
        self.hibp_hint.as_ref()
    }

    /// Reports whether secret field `accessToken` is present.
    #[must_use]
    pub const fn has_access_token(&self) -> bool {
        self.access_token.is_some()
    }

    pub(crate) fn access_token_secret(&self) -> Option<&crate::api::current::SecretValue> {
        self.access_token.as_ref()
    }

    /// Returns wire field `expirationTime`.
    #[must_use]
    pub fn expiration_time(&self) -> Option<&jiff::Timestamp> {
        self.expiration_time.as_ref()
    }

    /// Returns wire field `passwordExpirationTime`.
    #[must_use]
    pub fn password_expiration_time(&self) -> Option<&jiff::Timestamp> {
        self.password_expiration_time.as_ref()
    }

    /// Returns wire field `userStatus`.
    #[must_use]
    pub fn user_status(&self) -> Option<&AccessTokenResponseUserStatus> {
        self.user_status.as_ref()
    }

    /// Returns wire field `userId`.
    #[must_use]
    pub fn user_id(&self) -> Option<&crate::UserId> {
        self.user_id.as_ref()
    }

    /// Reports whether secret field `name` is present.
    #[must_use]
    pub const fn has_name(&self) -> bool {
        self.name.is_some()
    }

    /// Returns wire field `hasLive`.
    #[must_use]
    pub fn has_live(&self) -> Option<&bool> {
        self.has_live.as_ref()
    }

    /// Returns wire field `hasSimPlus`.
    #[must_use]
    pub fn has_sim_plus(&self) -> Option<&bool> {
        self.has_sim_plus.as_ref()
    }

    /// Returns wire field `showKIDs`.
    #[must_use]
    pub fn show_ki_ds(&self) -> Option<&bool> {
        self.show_ki_ds.as_ref()
    }
}

/// Current provider values for `AccessTokenResponseHibpHint`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum AccessTokenResponseHibpHint {
    /// Provider value `EmailAndPasswordCompromised`.
    EmailAndPasswordCompromised,
    /// Provider value `PasswordCompromised`.
    PasswordCompromised,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl AccessTokenResponseHibpHint {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::EmailAndPasswordCompromised => "EmailAndPasswordCompromised",
            Self::PasswordCompromised => "PasswordCompromised",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for AccessTokenResponseHibpHint {
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

impl<'de> serde::Deserialize<'de> for AccessTokenResponseHibpHint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "EmailAndPasswordCompromised" => Self::EmailAndPasswordCompromised,
            "PasswordCompromised" => Self::PasswordCompromised,
            _ => Self::Unknown(value),
        })
    }
}

/// Current provider values for `AccessTokenResponseUserStatus`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum AccessTokenResponseUserStatus {
    /// Provider value `Active`.
    Active,
    /// Provider value `Closed`.
    Closed,
    /// Provider value `Initiated`.
    Initiated,
    /// Provider value `TemporaryLocked`.
    TemporaryLocked,
    /// Provider value `UnconfirmedEmail`.
    UnconfirmedEmail,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl AccessTokenResponseUserStatus {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Active => "Active",
            Self::Closed => "Closed",
            Self::Initiated => "Initiated",
            Self::TemporaryLocked => "TemporaryLocked",
            Self::UnconfirmedEmail => "UnconfirmedEmail",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for AccessTokenResponseUserStatus {
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

impl<'de> serde::Deserialize<'de> for AccessTokenResponseUserStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "Active" => Self::Active,
            "Closed" => Self::Closed,
            "Initiated" => Self::Initiated,
            "TemporaryLocked" => Self::TemporaryLocked,
            "UnconfirmedEmail" => Self::UnconfirmedEmail,
            _ => Self::Unknown(value),
        })
    }
}

/// Current wire model `Account`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct Account {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    id: Option<crate::AccountId>,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "userId")]
    user_id: crate::UserId,
    #[serde(rename = "accountType")]
    account_type: AccountAccountType,
    #[serde(
        rename = "restricted",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    restricted: Option<bool>,
    #[serde(rename = "closed", default, skip_serializing_if = "Option::is_none")]
    closed: Option<bool>,
    #[serde(rename = "clearingHouseId")]
    clearing_house_id: super::ids::ClearingHouseId,
    #[serde(rename = "riskCategoryId")]
    risk_category_id: super::ids::RiskCategoryId,
    #[serde(rename = "autoLiqProfileId")]
    auto_liq_profile_id: super::ids::AutoLiqProfileId,
    #[serde(rename = "marginAccountType")]
    margin_account_type: AccountMarginAccountType,
    #[serde(rename = "legalStatus")]
    legal_status: AccountLegalStatus,
    #[serde(rename = "timestamp")]
    timestamp: jiff::Timestamp,
    #[serde(
        rename = "evaluationSize",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    evaluation_size: Option<crate::Decimal>,
    #[serde(rename = "readonly", default, skip_serializing_if = "Option::is_none")]
    readonly: Option<bool>,
    #[serde(rename = "ccEmail", default, skip_serializing_if = "Option::is_none")]
    cc_email: Option<String>,
    #[serde(
        rename = "futuresDisabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    futures_disabled: Option<bool>,
    #[serde(
        rename = "swapEnabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    swap_enabled: Option<bool>,
    #[serde(
        rename = "ssfRiskDisclosureAcknowledgment",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    ssf_risk_disclosure_acknowledgment: Option<jiff::Timestamp>,
    #[serde(
        rename = "spotMarginEnabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    spot_margin_enabled: Option<bool>,
}

impl Account {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> Option<&crate::AccountId> {
        self.id.as_ref()
    }

    /// Returns wire field `name`.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns wire field `userId`.
    #[must_use]
    pub fn user_id(&self) -> &crate::UserId {
        &self.user_id
    }

    /// Returns wire field `accountType`.
    #[must_use]
    pub fn account_type(&self) -> &AccountAccountType {
        &self.account_type
    }

    /// Returns wire field `restricted`.
    #[must_use]
    pub fn restricted(&self) -> Option<&bool> {
        self.restricted.as_ref()
    }

    /// Returns wire field `closed`.
    #[must_use]
    pub fn closed(&self) -> Option<&bool> {
        self.closed.as_ref()
    }

    /// Returns wire field `clearingHouseId`.
    #[must_use]
    pub fn clearing_house_id(&self) -> &super::ids::ClearingHouseId {
        &self.clearing_house_id
    }

    /// Returns wire field `riskCategoryId`.
    #[must_use]
    pub fn risk_category_id(&self) -> &super::ids::RiskCategoryId {
        &self.risk_category_id
    }

    /// Returns wire field `autoLiqProfileId`.
    #[must_use]
    pub fn auto_liq_profile_id(&self) -> &super::ids::AutoLiqProfileId {
        &self.auto_liq_profile_id
    }

    /// Returns wire field `marginAccountType`.
    #[must_use]
    pub fn margin_account_type(&self) -> &AccountMarginAccountType {
        &self.margin_account_type
    }

    /// Returns wire field `legalStatus`.
    #[must_use]
    pub fn legal_status(&self) -> &AccountLegalStatus {
        &self.legal_status
    }

    /// Returns wire field `timestamp`.
    #[must_use]
    pub fn timestamp(&self) -> &jiff::Timestamp {
        &self.timestamp
    }

    /// Returns wire field `evaluationSize`.
    #[must_use]
    pub fn evaluation_size(&self) -> Option<&crate::Decimal> {
        self.evaluation_size.as_ref()
    }

    /// Returns wire field `readonly`.
    #[must_use]
    pub fn readonly(&self) -> Option<&bool> {
        self.readonly.as_ref()
    }

    /// Returns wire field `ccEmail`.
    #[must_use]
    pub fn cc_email(&self) -> Option<&str> {
        self.cc_email.as_deref()
    }

    /// Returns wire field `futuresDisabled`.
    #[must_use]
    pub fn futures_disabled(&self) -> Option<&bool> {
        self.futures_disabled.as_ref()
    }

    /// Returns wire field `swapEnabled`.
    #[must_use]
    pub fn swap_enabled(&self) -> Option<&bool> {
        self.swap_enabled.as_ref()
    }

    /// Returns wire field `ssfRiskDisclosureAcknowledgment`.
    #[must_use]
    pub fn ssf_risk_disclosure_acknowledgment(&self) -> Option<&jiff::Timestamp> {
        self.ssf_risk_disclosure_acknowledgment.as_ref()
    }

    /// Returns wire field `spotMarginEnabled`.
    #[must_use]
    pub fn spot_margin_enabled(&self) -> Option<&bool> {
        self.spot_margin_enabled.as_ref()
    }

    /// Starts a builder for [`Account`].
    pub fn builder() -> AccountBuilder {
        AccountBuilder::default()
    }
}

/// Builder for [`Account`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct AccountBuilder {
    id: Option<crate::AccountId>,
    name: Option<String>,
    user_id: Option<crate::UserId>,
    account_type: Option<AccountAccountType>,
    restricted: Option<bool>,
    closed: Option<bool>,
    clearing_house_id: Option<super::ids::ClearingHouseId>,
    risk_category_id: Option<super::ids::RiskCategoryId>,
    auto_liq_profile_id: Option<super::ids::AutoLiqProfileId>,
    margin_account_type: Option<AccountMarginAccountType>,
    legal_status: Option<AccountLegalStatus>,
    timestamp: Option<jiff::Timestamp>,
    evaluation_size: Option<crate::Decimal>,
    readonly: Option<bool>,
    cc_email: Option<String>,
    futures_disabled: Option<bool>,
    swap_enabled: Option<bool>,
    ssf_risk_disclosure_acknowledgment: Option<jiff::Timestamp>,
    spot_margin_enabled: Option<bool>,
}

impl AccountBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: crate::AccountId) -> Self {
        self.id = Some(value);
        self
    }

    /// Sets wire field `name`.
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Sets wire field `userId`.
    pub fn user_id(mut self, value: crate::UserId) -> Self {
        self.user_id = Some(value);
        self
    }

    /// Sets wire field `accountType`.
    pub fn account_type(mut self, value: AccountAccountType) -> Self {
        self.account_type = Some(value);
        self
    }

    /// Sets wire field `restricted`.
    pub fn restricted(mut self, value: bool) -> Self {
        self.restricted = Some(value);
        self
    }

    /// Sets wire field `closed`.
    pub fn closed(mut self, value: bool) -> Self {
        self.closed = Some(value);
        self
    }

    /// Sets wire field `clearingHouseId`.
    pub fn clearing_house_id(mut self, value: super::ids::ClearingHouseId) -> Self {
        self.clearing_house_id = Some(value);
        self
    }

    /// Sets wire field `riskCategoryId`.
    pub fn risk_category_id(mut self, value: super::ids::RiskCategoryId) -> Self {
        self.risk_category_id = Some(value);
        self
    }

    /// Sets wire field `autoLiqProfileId`.
    pub fn auto_liq_profile_id(mut self, value: super::ids::AutoLiqProfileId) -> Self {
        self.auto_liq_profile_id = Some(value);
        self
    }

    /// Sets wire field `marginAccountType`.
    pub fn margin_account_type(mut self, value: AccountMarginAccountType) -> Self {
        self.margin_account_type = Some(value);
        self
    }

    /// Sets wire field `legalStatus`.
    pub fn legal_status(mut self, value: AccountLegalStatus) -> Self {
        self.legal_status = Some(value);
        self
    }

    /// Sets wire field `timestamp`.
    pub fn timestamp(mut self, value: jiff::Timestamp) -> Self {
        self.timestamp = Some(value);
        self
    }

    /// Sets wire field `evaluationSize`.
    pub fn evaluation_size(mut self, value: crate::Decimal) -> Self {
        self.evaluation_size = Some(value);
        self
    }

    /// Sets wire field `readonly`.
    pub fn readonly(mut self, value: bool) -> Self {
        self.readonly = Some(value);
        self
    }

    /// Sets wire field `ccEmail`.
    pub fn cc_email(mut self, value: impl Into<String>) -> Self {
        self.cc_email = Some(value.into());
        self
    }

    /// Sets wire field `futuresDisabled`.
    pub fn futures_disabled(mut self, value: bool) -> Self {
        self.futures_disabled = Some(value);
        self
    }

    /// Sets wire field `swapEnabled`.
    pub fn swap_enabled(mut self, value: bool) -> Self {
        self.swap_enabled = Some(value);
        self
    }

    /// Sets wire field `ssfRiskDisclosureAcknowledgment`.
    pub fn ssf_risk_disclosure_acknowledgment(mut self, value: jiff::Timestamp) -> Self {
        self.ssf_risk_disclosure_acknowledgment = Some(value);
        self
    }

    /// Sets wire field `spotMarginEnabled`.
    pub fn spot_margin_enabled(mut self, value: bool) -> Self {
        self.spot_margin_enabled = Some(value);
        self
    }

    /// Validates required fields and builds [`Account`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<Account, crate::api::current::BuildError> {
        let name = self
            .name
            .ok_or(crate::api::current::BuildError::missing("name"))?;
        let user_id = self
            .user_id
            .ok_or(crate::api::current::BuildError::missing("userId"))?;
        let account_type = self
            .account_type
            .ok_or(crate::api::current::BuildError::missing("accountType"))?;
        let clearing_house_id = self
            .clearing_house_id
            .ok_or(crate::api::current::BuildError::missing("clearingHouseId"))?;
        let risk_category_id = self
            .risk_category_id
            .ok_or(crate::api::current::BuildError::missing("riskCategoryId"))?;
        let auto_liq_profile_id = self
            .auto_liq_profile_id
            .ok_or(crate::api::current::BuildError::missing("autoLiqProfileId"))?;
        let margin_account_type =
            self.margin_account_type
                .ok_or(crate::api::current::BuildError::missing(
                    "marginAccountType",
                ))?;
        let legal_status = self
            .legal_status
            .ok_or(crate::api::current::BuildError::missing("legalStatus"))?;
        let timestamp = self
            .timestamp
            .ok_or(crate::api::current::BuildError::missing("timestamp"))?;
        Ok(Account {
            id: self.id,
            name,
            user_id,
            account_type,
            restricted: self.restricted,
            closed: self.closed,
            clearing_house_id,
            risk_category_id,
            auto_liq_profile_id,
            margin_account_type,
            legal_status,
            timestamp,
            evaluation_size: self.evaluation_size,
            readonly: self.readonly,
            cc_email: self.cc_email,
            futures_disabled: self.futures_disabled,
            swap_enabled: self.swap_enabled,
            ssf_risk_disclosure_acknowledgment: self.ssf_risk_disclosure_acknowledgment,
            spot_margin_enabled: self.spot_margin_enabled,
        })
    }
}

/// Current provider values for `AccountAccountType`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum AccountAccountType {
    /// Provider value `Customer`.
    Customer,
    /// Provider value `Employee`.
    Employee,
    /// Provider value `Giveup`.
    Giveup,
    /// Provider value `House`.
    House,
    /// Provider value `Omnibus`.
    Omnibus,
    /// Provider value `Wash`.
    Wash,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl AccountAccountType {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Customer => "Customer",
            Self::Employee => "Employee",
            Self::Giveup => "Giveup",
            Self::House => "House",
            Self::Omnibus => "Omnibus",
            Self::Wash => "Wash",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for AccountAccountType {
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

impl<'de> serde::Deserialize<'de> for AccountAccountType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "Customer" => Self::Customer,
            "Employee" => Self::Employee,
            "Giveup" => Self::Giveup,
            "House" => Self::House,
            "Omnibus" => Self::Omnibus,
            "Wash" => Self::Wash,
            _ => Self::Unknown(value),
        })
    }
}

/// Current provider values for `AccountLegalStatus`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum AccountLegalStatus {
    /// Provider value `Corporation`.
    Corporation,
    /// Provider value `GP`.
    Gp,
    /// Provider value `IRA`.
    Ira,
    /// Provider value `Individual`.
    Individual,
    /// Provider value `Joint`.
    Joint,
    /// Provider value `LLC`.
    Llc,
    /// Provider value `LLP`.
    Llp,
    /// Provider value `LP`.
    Lp,
    /// Provider value `PTR`.
    Ptr,
    /// Provider value `Trust`.
    Trust,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl AccountLegalStatus {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Corporation => "Corporation",
            Self::Gp => "GP",
            Self::Ira => "IRA",
            Self::Individual => "Individual",
            Self::Joint => "Joint",
            Self::Llc => "LLC",
            Self::Llp => "LLP",
            Self::Lp => "LP",
            Self::Ptr => "PTR",
            Self::Trust => "Trust",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for AccountLegalStatus {
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

impl<'de> serde::Deserialize<'de> for AccountLegalStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "Corporation" => Self::Corporation,
            "GP" => Self::Gp,
            "IRA" => Self::Ira,
            "Individual" => Self::Individual,
            "Joint" => Self::Joint,
            "LLC" => Self::Llc,
            "LLP" => Self::Llp,
            "LP" => Self::Lp,
            "PTR" => Self::Ptr,
            "Trust" => Self::Trust,
            _ => Self::Unknown(value),
        })
    }
}

/// Current provider values for `AccountMarginAccountType`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum AccountMarginAccountType {
    /// Provider value `Hedger`.
    Hedger,
    /// Provider value `Speculator`.
    Speculator,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl AccountMarginAccountType {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Hedger => "Hedger",
            Self::Speculator => "Speculator",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for AccountMarginAccountType {
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

impl<'de> serde::Deserialize<'de> for AccountMarginAccountType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "Hedger" => Self::Hedger,
            "Speculator" => Self::Speculator,
            _ => Self::Unknown(value),
        })
    }
}

/// Current wire model `AccountRiskStatus`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct AccountRiskStatus {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    id: Option<super::ids::AccountRiskStatusId>,
    #[serde(
        rename = "adminAction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    admin_action: Option<AccountRiskStatusAdminAction>,
    #[serde(
        rename = "adminTimestamp",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    admin_timestamp: Option<jiff::Timestamp>,
    #[serde(
        rename = "liquidateOnly",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    liquidate_only: Option<jiff::Timestamp>,
    #[serde(
        rename = "userTriggeredLiqOnly",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    user_triggered_liq_only: Option<bool>,
    #[serde(rename = "maxNetLiq", default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "crate::decimal::option")]
    max_net_liq: Option<crate::Decimal>,
    #[serde(rename = "minNetLiq", default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "crate::decimal::option")]
    min_net_liq: Option<crate::Decimal>,
}

impl AccountRiskStatus {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> Option<&super::ids::AccountRiskStatusId> {
        self.id.as_ref()
    }

    /// Returns wire field `adminAction`.
    #[must_use]
    pub fn admin_action(&self) -> Option<&AccountRiskStatusAdminAction> {
        self.admin_action.as_ref()
    }

    /// Returns wire field `adminTimestamp`.
    #[must_use]
    pub fn admin_timestamp(&self) -> Option<&jiff::Timestamp> {
        self.admin_timestamp.as_ref()
    }

    /// Returns wire field `liquidateOnly`.
    #[must_use]
    pub fn liquidate_only(&self) -> Option<&jiff::Timestamp> {
        self.liquidate_only.as_ref()
    }

    /// Returns wire field `userTriggeredLiqOnly`.
    #[must_use]
    pub fn user_triggered_liq_only(&self) -> Option<&bool> {
        self.user_triggered_liq_only.as_ref()
    }

    /// Returns wire field `maxNetLiq`.
    #[must_use]
    pub fn max_net_liq(&self) -> Option<&crate::Decimal> {
        self.max_net_liq.as_ref()
    }

    /// Returns wire field `minNetLiq`.
    #[must_use]
    pub fn min_net_liq(&self) -> Option<&crate::Decimal> {
        self.min_net_liq.as_ref()
    }

    /// Starts a builder for [`AccountRiskStatus`].
    pub fn builder() -> AccountRiskStatusBuilder {
        AccountRiskStatusBuilder::default()
    }
}

/// Builder for [`AccountRiskStatus`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct AccountRiskStatusBuilder {
    id: Option<super::ids::AccountRiskStatusId>,
    admin_action: Option<AccountRiskStatusAdminAction>,
    admin_timestamp: Option<jiff::Timestamp>,
    liquidate_only: Option<jiff::Timestamp>,
    user_triggered_liq_only: Option<bool>,
    max_net_liq: Option<crate::Decimal>,
    min_net_liq: Option<crate::Decimal>,
}

impl AccountRiskStatusBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::AccountRiskStatusId) -> Self {
        self.id = Some(value);
        self
    }

    /// Sets wire field `adminAction`.
    pub fn admin_action(mut self, value: AccountRiskStatusAdminAction) -> Self {
        self.admin_action = Some(value);
        self
    }

    /// Sets wire field `adminTimestamp`.
    pub fn admin_timestamp(mut self, value: jiff::Timestamp) -> Self {
        self.admin_timestamp = Some(value);
        self
    }

    /// Sets wire field `liquidateOnly`.
    pub fn liquidate_only(mut self, value: jiff::Timestamp) -> Self {
        self.liquidate_only = Some(value);
        self
    }

    /// Sets wire field `userTriggeredLiqOnly`.
    pub fn user_triggered_liq_only(mut self, value: bool) -> Self {
        self.user_triggered_liq_only = Some(value);
        self
    }

    /// Sets wire field `maxNetLiq`.
    pub fn max_net_liq(mut self, value: crate::Decimal) -> Self {
        self.max_net_liq = Some(value);
        self
    }

    /// Sets wire field `minNetLiq`.
    pub fn min_net_liq(mut self, value: crate::Decimal) -> Self {
        self.min_net_liq = Some(value);
        self
    }

    /// Validates required fields and builds [`AccountRiskStatus`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<AccountRiskStatus, crate::api::current::BuildError> {
        Ok(AccountRiskStatus {
            id: self.id,
            admin_action: self.admin_action,
            admin_timestamp: self.admin_timestamp,
            liquidate_only: self.liquidate_only,
            user_triggered_liq_only: self.user_triggered_liq_only,
            max_net_liq: self.max_net_liq,
            min_net_liq: self.min_net_liq,
        })
    }
}

/// Current provider values for `AccountRiskStatusAdminAction`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum AccountRiskStatusAdminAction {
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

impl AccountRiskStatusAdminAction {
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

impl serde::Serialize for AccountRiskStatusAdminAction {
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

impl<'de> serde::Deserialize<'de> for AccountRiskStatusAdminAction {
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

/// Current wire model `AddEntitlementSubscription`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct AddEntitlementSubscription {
    #[serde(rename = "entitlementId")]
    entitlement_id: super::ids::EntitlementId,
    #[serde(
        rename = "creditCardId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    credit_card_id: Option<super::ids::CreditCardId>,
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    account_id: Option<crate::AccountId>,
    #[serde(rename = "userId", default, skip_serializing_if = "Option::is_none")]
    user_id: Option<crate::UserId>,
    #[serde(
        rename = "nonProSigned",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    non_pro_signed: Option<bool>,
}

impl AddEntitlementSubscription {
    /// Returns wire field `entitlementId`.
    #[must_use]
    pub fn entitlement_id(&self) -> &super::ids::EntitlementId {
        &self.entitlement_id
    }

    /// Returns wire field `creditCardId`.
    #[must_use]
    pub fn credit_card_id(&self) -> Option<&super::ids::CreditCardId> {
        self.credit_card_id.as_ref()
    }

    /// Returns wire field `accountId`.
    #[must_use]
    pub fn account_id(&self) -> Option<&crate::AccountId> {
        self.account_id.as_ref()
    }

    /// Returns wire field `userId`.
    #[must_use]
    pub fn user_id(&self) -> Option<&crate::UserId> {
        self.user_id.as_ref()
    }

    /// Returns wire field `nonProSigned`.
    #[must_use]
    pub fn non_pro_signed(&self) -> Option<&bool> {
        self.non_pro_signed.as_ref()
    }

    /// Starts a builder for [`AddEntitlementSubscription`].
    pub fn builder() -> AddEntitlementSubscriptionBuilder {
        AddEntitlementSubscriptionBuilder::default()
    }
}

/// Builder for [`AddEntitlementSubscription`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct AddEntitlementSubscriptionBuilder {
    entitlement_id: Option<super::ids::EntitlementId>,
    credit_card_id: Option<super::ids::CreditCardId>,
    account_id: Option<crate::AccountId>,
    user_id: Option<crate::UserId>,
    non_pro_signed: Option<bool>,
}

impl AddEntitlementSubscriptionBuilder {
    /// Sets wire field `entitlementId`.
    pub fn entitlement_id(mut self, value: super::ids::EntitlementId) -> Self {
        self.entitlement_id = Some(value);
        self
    }

    /// Sets wire field `creditCardId`.
    pub fn credit_card_id(mut self, value: super::ids::CreditCardId) -> Self {
        self.credit_card_id = Some(value);
        self
    }

    /// Sets wire field `accountId`.
    pub fn account_id(mut self, value: crate::AccountId) -> Self {
        self.account_id = Some(value);
        self
    }

    /// Sets wire field `userId`.
    pub fn user_id(mut self, value: crate::UserId) -> Self {
        self.user_id = Some(value);
        self
    }

    /// Sets wire field `nonProSigned`.
    pub fn non_pro_signed(mut self, value: bool) -> Self {
        self.non_pro_signed = Some(value);
        self
    }

    /// Validates required fields and builds [`AddEntitlementSubscription`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<AddEntitlementSubscription, crate::api::current::BuildError> {
        let entitlement_id = self
            .entitlement_id
            .ok_or(crate::api::current::BuildError::missing("entitlementId"))?;
        Ok(AddEntitlementSubscription {
            entitlement_id,
            credit_card_id: self.credit_card_id,
            account_id: self.account_id,
            user_id: self.user_id,
            non_pro_signed: self.non_pro_signed,
        })
    }
}

impl crate::api::current::support::CurrentRequest for AddEntitlementSubscription {
    fn validate_current(&self) -> Result<(), crate::Error> {
        Ok(())
    }
}

/// Current wire model `AddMarketDataSubscription`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct AddMarketDataSubscription {
    #[serde(rename = "marketDataSubscriptionPlanIds")]
    market_data_subscription_plan_ids: Vec<super::ids::MarketDataSubscriptionPlanId>,
    #[serde(rename = "year")]
    year: i64,
    #[serde(rename = "month")]
    month: i64,
    #[serde(
        rename = "creditCardId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    credit_card_id: Option<super::ids::CreditCardId>,
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    account_id: Option<crate::AccountId>,
    #[serde(rename = "userId", default, skip_serializing_if = "Option::is_none")]
    user_id: Option<crate::UserId>,
}

impl AddMarketDataSubscription {
    /// Returns wire field `marketDataSubscriptionPlanIds`.
    #[must_use]
    pub fn market_data_subscription_plan_ids(&self) -> &[super::ids::MarketDataSubscriptionPlanId] {
        &self.market_data_subscription_plan_ids
    }

    /// Returns wire field `year`.
    #[must_use]
    pub fn year(&self) -> &i64 {
        &self.year
    }

    /// Returns wire field `month`.
    #[must_use]
    pub fn month(&self) -> &i64 {
        &self.month
    }

    /// Returns wire field `creditCardId`.
    #[must_use]
    pub fn credit_card_id(&self) -> Option<&super::ids::CreditCardId> {
        self.credit_card_id.as_ref()
    }

    /// Returns wire field `accountId`.
    #[must_use]
    pub fn account_id(&self) -> Option<&crate::AccountId> {
        self.account_id.as_ref()
    }

    /// Returns wire field `userId`.
    #[must_use]
    pub fn user_id(&self) -> Option<&crate::UserId> {
        self.user_id.as_ref()
    }

    /// Starts a builder for [`AddMarketDataSubscription`].
    pub fn builder() -> AddMarketDataSubscriptionBuilder {
        AddMarketDataSubscriptionBuilder::default()
    }
}

/// Builder for [`AddMarketDataSubscription`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct AddMarketDataSubscriptionBuilder {
    market_data_subscription_plan_ids: Option<Vec<super::ids::MarketDataSubscriptionPlanId>>,
    year: Option<i64>,
    month: Option<i64>,
    credit_card_id: Option<super::ids::CreditCardId>,
    account_id: Option<crate::AccountId>,
    user_id: Option<crate::UserId>,
}

impl AddMarketDataSubscriptionBuilder {
    /// Sets wire field `marketDataSubscriptionPlanIds`.
    pub fn market_data_subscription_plan_ids(
        mut self,
        value: Vec<super::ids::MarketDataSubscriptionPlanId>,
    ) -> Self {
        self.market_data_subscription_plan_ids = Some(value);
        self
    }

    /// Sets wire field `year`.
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    /// Sets wire field `month`.
    pub fn month(mut self, value: i64) -> Self {
        self.month = Some(value);
        self
    }

    /// Sets wire field `creditCardId`.
    pub fn credit_card_id(mut self, value: super::ids::CreditCardId) -> Self {
        self.credit_card_id = Some(value);
        self
    }

    /// Sets wire field `accountId`.
    pub fn account_id(mut self, value: crate::AccountId) -> Self {
        self.account_id = Some(value);
        self
    }

    /// Sets wire field `userId`.
    pub fn user_id(mut self, value: crate::UserId) -> Self {
        self.user_id = Some(value);
        self
    }

    /// Validates required fields and builds [`AddMarketDataSubscription`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<AddMarketDataSubscription, crate::api::current::BuildError> {
        let market_data_subscription_plan_ids = self.market_data_subscription_plan_ids.ok_or(
            crate::api::current::BuildError::missing("marketDataSubscriptionPlanIds"),
        )?;
        if market_data_subscription_plan_ids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "marketDataSubscriptionPlanIds",
                "must not be empty",
            ));
        }
        let year = self
            .year
            .ok_or(crate::api::current::BuildError::missing("year"))?;
        let month = self
            .month
            .ok_or(crate::api::current::BuildError::missing("month"))?;
        Ok(AddMarketDataSubscription {
            market_data_subscription_plan_ids,
            year,
            month,
            credit_card_id: self.credit_card_id,
            account_id: self.account_id,
            user_id: self.user_id,
        })
    }
}

impl crate::api::current::support::CurrentRequest for AddMarketDataSubscription {
    fn validate_current(&self) -> Result<(), crate::Error> {
        if self.market_data_subscription_plan_ids.is_empty() {
            return Err(crate::Error::InvalidRequest {
                field: "marketDataSubscriptionPlanIds",
                reason: "must not be empty",
            });
        }
        Ok(())
    }
}

/// Current wire model `AddTradovateSubscription`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct AddTradovateSubscription {
    #[serde(rename = "tradovateSubscriptionPlanId")]
    tradovate_subscription_plan_id: super::ids::TradovateSubscriptionPlanId,
    #[serde(
        rename = "creditCardId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    credit_card_id: Option<super::ids::CreditCardId>,
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    account_id: Option<crate::AccountId>,
    #[serde(rename = "userId", default, skip_serializing_if = "Option::is_none")]
    user_id: Option<crate::UserId>,
}

impl AddTradovateSubscription {
    /// Returns wire field `tradovateSubscriptionPlanId`.
    #[must_use]
    pub fn tradovate_subscription_plan_id(&self) -> &super::ids::TradovateSubscriptionPlanId {
        &self.tradovate_subscription_plan_id
    }

    /// Returns wire field `creditCardId`.
    #[must_use]
    pub fn credit_card_id(&self) -> Option<&super::ids::CreditCardId> {
        self.credit_card_id.as_ref()
    }

    /// Returns wire field `accountId`.
    #[must_use]
    pub fn account_id(&self) -> Option<&crate::AccountId> {
        self.account_id.as_ref()
    }

    /// Returns wire field `userId`.
    #[must_use]
    pub fn user_id(&self) -> Option<&crate::UserId> {
        self.user_id.as_ref()
    }

    /// Starts a builder for [`AddTradovateSubscription`].
    pub fn builder() -> AddTradovateSubscriptionBuilder {
        AddTradovateSubscriptionBuilder::default()
    }
}

/// Builder for [`AddTradovateSubscription`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct AddTradovateSubscriptionBuilder {
    tradovate_subscription_plan_id: Option<super::ids::TradovateSubscriptionPlanId>,
    credit_card_id: Option<super::ids::CreditCardId>,
    account_id: Option<crate::AccountId>,
    user_id: Option<crate::UserId>,
}

impl AddTradovateSubscriptionBuilder {
    /// Sets wire field `tradovateSubscriptionPlanId`.
    pub fn tradovate_subscription_plan_id(
        mut self,
        value: super::ids::TradovateSubscriptionPlanId,
    ) -> Self {
        self.tradovate_subscription_plan_id = Some(value);
        self
    }

    /// Sets wire field `creditCardId`.
    pub fn credit_card_id(mut self, value: super::ids::CreditCardId) -> Self {
        self.credit_card_id = Some(value);
        self
    }

    /// Sets wire field `accountId`.
    pub fn account_id(mut self, value: crate::AccountId) -> Self {
        self.account_id = Some(value);
        self
    }

    /// Sets wire field `userId`.
    pub fn user_id(mut self, value: crate::UserId) -> Self {
        self.user_id = Some(value);
        self
    }

    /// Validates required fields and builds [`AddTradovateSubscription`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<AddTradovateSubscription, crate::api::current::BuildError> {
        let tradovate_subscription_plan_id =
            self.tradovate_subscription_plan_id
                .ok_or(crate::api::current::BuildError::missing(
                    "tradovateSubscriptionPlanId",
                ))?;
        Ok(AddTradovateSubscription {
            tradovate_subscription_plan_id,
            credit_card_id: self.credit_card_id,
            account_id: self.account_id,
            user_id: self.user_id,
        })
    }
}

impl crate::api::current::support::CurrentRequest for AddTradovateSubscription {
    fn validate_current(&self) -> Result<(), crate::Error> {
        Ok(())
    }
}

/// Current wire model `AnnualReview`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct AnnualReview {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    id: Option<super::ids::AnnualReviewId>,
    #[serde(rename = "userId")]
    user_id: crate::UserId,
    #[serde(
        rename = "firstEmail",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    first_email: Option<String>,
    #[serde(
        rename = "secondEmail",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    second_email: Option<String>,
    #[serde(
        rename = "jointFirstEmail",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    joint_first_email: Option<String>,
    #[serde(
        rename = "jointSecondEmail",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    joint_second_email: Option<String>,
    #[serde(
        rename = "firstEmailSent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    first_email_sent: Option<jiff::Timestamp>,
    #[serde(
        rename = "secondEmailSent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    second_email_sent: Option<jiff::Timestamp>,
    #[serde(rename = "finished", default, skip_serializing_if = "Option::is_none")]
    finished: Option<jiff::Timestamp>,
    #[serde(
        rename = "jointFinished",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    joint_finished: Option<jiff::Timestamp>,
    #[serde(rename = "riskDisclosureNeeded")]
    risk_disclosure_needed: bool,
    #[serde(
        rename = "identityCheckResult",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    identity_check_result: Option<AnnualReviewIdentityCheckResult>,
    #[serde(
        rename = "jointIdentityCheckResult",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    joint_identity_check_result: Option<AnnualReviewJointIdentityCheckResult>,
    #[serde(rename = "archived")]
    archived: bool,
    #[serde(
        rename = "contactInfoId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    contact_info_id: Option<super::ids::ContactInfoId>,
    #[serde(rename = "status")]
    status: AnnualReviewStatus,
}

impl AnnualReview {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> Option<&super::ids::AnnualReviewId> {
        self.id.as_ref()
    }

    /// Returns wire field `userId`.
    #[must_use]
    pub fn user_id(&self) -> &crate::UserId {
        &self.user_id
    }

    /// Returns wire field `firstEmail`.
    #[must_use]
    pub fn first_email(&self) -> Option<&str> {
        self.first_email.as_deref()
    }

    /// Returns wire field `secondEmail`.
    #[must_use]
    pub fn second_email(&self) -> Option<&str> {
        self.second_email.as_deref()
    }

    /// Returns wire field `jointFirstEmail`.
    #[must_use]
    pub fn joint_first_email(&self) -> Option<&str> {
        self.joint_first_email.as_deref()
    }

    /// Returns wire field `jointSecondEmail`.
    #[must_use]
    pub fn joint_second_email(&self) -> Option<&str> {
        self.joint_second_email.as_deref()
    }

    /// Returns wire field `firstEmailSent`.
    #[must_use]
    pub fn first_email_sent(&self) -> Option<&jiff::Timestamp> {
        self.first_email_sent.as_ref()
    }

    /// Returns wire field `secondEmailSent`.
    #[must_use]
    pub fn second_email_sent(&self) -> Option<&jiff::Timestamp> {
        self.second_email_sent.as_ref()
    }

    /// Returns wire field `finished`.
    #[must_use]
    pub fn finished(&self) -> Option<&jiff::Timestamp> {
        self.finished.as_ref()
    }

    /// Returns wire field `jointFinished`.
    #[must_use]
    pub fn joint_finished(&self) -> Option<&jiff::Timestamp> {
        self.joint_finished.as_ref()
    }

    /// Returns wire field `riskDisclosureNeeded`.
    #[must_use]
    pub fn risk_disclosure_needed(&self) -> &bool {
        &self.risk_disclosure_needed
    }

    /// Returns wire field `identityCheckResult`.
    #[must_use]
    pub fn identity_check_result(&self) -> Option<&AnnualReviewIdentityCheckResult> {
        self.identity_check_result.as_ref()
    }

    /// Returns wire field `jointIdentityCheckResult`.
    #[must_use]
    pub fn joint_identity_check_result(&self) -> Option<&AnnualReviewJointIdentityCheckResult> {
        self.joint_identity_check_result.as_ref()
    }

    /// Returns wire field `archived`.
    #[must_use]
    pub fn archived(&self) -> &bool {
        &self.archived
    }

    /// Returns wire field `contactInfoId`.
    #[must_use]
    pub fn contact_info_id(&self) -> Option<&super::ids::ContactInfoId> {
        self.contact_info_id.as_ref()
    }

    /// Returns wire field `status`.
    #[must_use]
    pub fn status(&self) -> &AnnualReviewStatus {
        &self.status
    }

    /// Starts a builder for [`AnnualReview`].
    pub fn builder() -> AnnualReviewBuilder {
        AnnualReviewBuilder::default()
    }
}

/// Builder for [`AnnualReview`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct AnnualReviewBuilder {
    id: Option<super::ids::AnnualReviewId>,
    user_id: Option<crate::UserId>,
    first_email: Option<String>,
    second_email: Option<String>,
    joint_first_email: Option<String>,
    joint_second_email: Option<String>,
    first_email_sent: Option<jiff::Timestamp>,
    second_email_sent: Option<jiff::Timestamp>,
    finished: Option<jiff::Timestamp>,
    joint_finished: Option<jiff::Timestamp>,
    risk_disclosure_needed: Option<bool>,
    identity_check_result: Option<AnnualReviewIdentityCheckResult>,
    joint_identity_check_result: Option<AnnualReviewJointIdentityCheckResult>,
    archived: Option<bool>,
    contact_info_id: Option<super::ids::ContactInfoId>,
    status: Option<AnnualReviewStatus>,
}

impl AnnualReviewBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::AnnualReviewId) -> Self {
        self.id = Some(value);
        self
    }

    /// Sets wire field `userId`.
    pub fn user_id(mut self, value: crate::UserId) -> Self {
        self.user_id = Some(value);
        self
    }

    /// Sets wire field `firstEmail`.
    pub fn first_email(mut self, value: impl Into<String>) -> Self {
        self.first_email = Some(value.into());
        self
    }

    /// Sets wire field `secondEmail`.
    pub fn second_email(mut self, value: impl Into<String>) -> Self {
        self.second_email = Some(value.into());
        self
    }

    /// Sets wire field `jointFirstEmail`.
    pub fn joint_first_email(mut self, value: impl Into<String>) -> Self {
        self.joint_first_email = Some(value.into());
        self
    }

    /// Sets wire field `jointSecondEmail`.
    pub fn joint_second_email(mut self, value: impl Into<String>) -> Self {
        self.joint_second_email = Some(value.into());
        self
    }

    /// Sets wire field `firstEmailSent`.
    pub fn first_email_sent(mut self, value: jiff::Timestamp) -> Self {
        self.first_email_sent = Some(value);
        self
    }

    /// Sets wire field `secondEmailSent`.
    pub fn second_email_sent(mut self, value: jiff::Timestamp) -> Self {
        self.second_email_sent = Some(value);
        self
    }

    /// Sets wire field `finished`.
    pub fn finished(mut self, value: jiff::Timestamp) -> Self {
        self.finished = Some(value);
        self
    }

    /// Sets wire field `jointFinished`.
    pub fn joint_finished(mut self, value: jiff::Timestamp) -> Self {
        self.joint_finished = Some(value);
        self
    }

    /// Sets wire field `riskDisclosureNeeded`.
    pub fn risk_disclosure_needed(mut self, value: bool) -> Self {
        self.risk_disclosure_needed = Some(value);
        self
    }

    /// Sets wire field `identityCheckResult`.
    pub fn identity_check_result(mut self, value: AnnualReviewIdentityCheckResult) -> Self {
        self.identity_check_result = Some(value);
        self
    }

    /// Sets wire field `jointIdentityCheckResult`.
    pub fn joint_identity_check_result(
        mut self,
        value: AnnualReviewJointIdentityCheckResult,
    ) -> Self {
        self.joint_identity_check_result = Some(value);
        self
    }

    /// Sets wire field `archived`.
    pub fn archived(mut self, value: bool) -> Self {
        self.archived = Some(value);
        self
    }

    /// Sets wire field `contactInfoId`.
    pub fn contact_info_id(mut self, value: super::ids::ContactInfoId) -> Self {
        self.contact_info_id = Some(value);
        self
    }

    /// Sets wire field `status`.
    pub fn status(mut self, value: AnnualReviewStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Validates required fields and builds [`AnnualReview`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<AnnualReview, crate::api::current::BuildError> {
        let user_id = self
            .user_id
            .ok_or(crate::api::current::BuildError::missing("userId"))?;
        let risk_disclosure_needed =
            self.risk_disclosure_needed
                .ok_or(crate::api::current::BuildError::missing(
                    "riskDisclosureNeeded",
                ))?;
        let archived = self
            .archived
            .ok_or(crate::api::current::BuildError::missing("archived"))?;
        let status = self
            .status
            .ok_or(crate::api::current::BuildError::missing("status"))?;
        Ok(AnnualReview {
            id: self.id,
            user_id,
            first_email: self.first_email,
            second_email: self.second_email,
            joint_first_email: self.joint_first_email,
            joint_second_email: self.joint_second_email,
            first_email_sent: self.first_email_sent,
            second_email_sent: self.second_email_sent,
            finished: self.finished,
            joint_finished: self.joint_finished,
            risk_disclosure_needed,
            identity_check_result: self.identity_check_result,
            joint_identity_check_result: self.joint_identity_check_result,
            archived,
            contact_info_id: self.contact_info_id,
            status,
        })
    }
}

/// Current provider values for `AnnualReviewIdentityCheckResult`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum AnnualReviewIdentityCheckResult {
    /// Provider value `Fail`.
    Fail,
    /// Provider value `Pass`.
    Pass,
    /// Provider value `ReviewNeeded`.
    ReviewNeeded,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl AnnualReviewIdentityCheckResult {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Fail => "Fail",
            Self::Pass => "Pass",
            Self::ReviewNeeded => "ReviewNeeded",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for AnnualReviewIdentityCheckResult {
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

impl<'de> serde::Deserialize<'de> for AnnualReviewIdentityCheckResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "Fail" => Self::Fail,
            "Pass" => Self::Pass,
            "ReviewNeeded" => Self::ReviewNeeded,
            _ => Self::Unknown(value),
        })
    }
}

/// Current provider values for `AnnualReviewJointIdentityCheckResult`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum AnnualReviewJointIdentityCheckResult {
    /// Provider value `Fail`.
    Fail,
    /// Provider value `Pass`.
    Pass,
    /// Provider value `ReviewNeeded`.
    ReviewNeeded,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl AnnualReviewJointIdentityCheckResult {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Fail => "Fail",
            Self::Pass => "Pass",
            Self::ReviewNeeded => "ReviewNeeded",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for AnnualReviewJointIdentityCheckResult {
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

impl<'de> serde::Deserialize<'de> for AnnualReviewJointIdentityCheckResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "Fail" => Self::Fail,
            "Pass" => Self::Pass,
            "ReviewNeeded" => Self::ReviewNeeded,
            _ => Self::Unknown(value),
        })
    }
}

/// Current provider values for `AnnualReviewStatus`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum AnnualReviewStatus {
    /// Provider value `Closed`.
    Closed,
    /// Provider value `Open`.
    Open,
    /// Provider value `Processed`.
    Processed,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl AnnualReviewStatus {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Closed => "Closed",
            Self::Open => "Open",
            Self::Processed => "Processed",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for AnnualReviewStatus {
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

impl<'de> serde::Deserialize<'de> for AnnualReviewStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "Closed" => Self::Closed,
            "Open" => Self::Open,
            "Processed" => Self::Processed,
            _ => Self::Unknown(value),
        })
    }
}

/// Current wire model `CancelEverything`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct CancelEverything {
    #[serde(rename = "userIds")]
    user_ids: Vec<crate::UserId>,
    #[serde(
        rename = "tradovateSubscriptions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    tradovate_subscriptions: Option<bool>,
    #[serde(
        rename = "userPlugins",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    user_plugins: Option<bool>,
    #[serde(
        rename = "marketDataSubscriptions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    market_data_subscriptions: Option<bool>,
    #[serde(
        rename = "tradingPermissions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    trading_permissions: Option<bool>,
}

impl CancelEverything {
    /// Returns wire field `userIds`.
    #[must_use]
    pub fn user_ids(&self) -> &[crate::UserId] {
        &self.user_ids
    }

    /// Returns wire field `tradovateSubscriptions`.
    #[must_use]
    pub fn tradovate_subscriptions(&self) -> Option<&bool> {
        self.tradovate_subscriptions.as_ref()
    }

    /// Returns wire field `userPlugins`.
    #[must_use]
    pub fn user_plugins(&self) -> Option<&bool> {
        self.user_plugins.as_ref()
    }

    /// Returns wire field `marketDataSubscriptions`.
    #[must_use]
    pub fn market_data_subscriptions(&self) -> Option<&bool> {
        self.market_data_subscriptions.as_ref()
    }

    /// Returns wire field `tradingPermissions`.
    #[must_use]
    pub fn trading_permissions(&self) -> Option<&bool> {
        self.trading_permissions.as_ref()
    }

    /// Starts a builder for [`CancelEverything`].
    pub fn builder() -> CancelEverythingBuilder {
        CancelEverythingBuilder::default()
    }
}

/// Builder for [`CancelEverything`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct CancelEverythingBuilder {
    user_ids: Option<Vec<crate::UserId>>,
    tradovate_subscriptions: Option<bool>,
    user_plugins: Option<bool>,
    market_data_subscriptions: Option<bool>,
    trading_permissions: Option<bool>,
}

impl CancelEverythingBuilder {
    /// Sets wire field `userIds`.
    pub fn user_ids(mut self, value: Vec<crate::UserId>) -> Self {
        self.user_ids = Some(value);
        self
    }

    /// Sets wire field `tradovateSubscriptions`.
    pub fn tradovate_subscriptions(mut self, value: bool) -> Self {
        self.tradovate_subscriptions = Some(value);
        self
    }

    /// Sets wire field `userPlugins`.
    pub fn user_plugins(mut self, value: bool) -> Self {
        self.user_plugins = Some(value);
        self
    }

    /// Sets wire field `marketDataSubscriptions`.
    pub fn market_data_subscriptions(mut self, value: bool) -> Self {
        self.market_data_subscriptions = Some(value);
        self
    }

    /// Sets wire field `tradingPermissions`.
    pub fn trading_permissions(mut self, value: bool) -> Self {
        self.trading_permissions = Some(value);
        self
    }

    /// Validates required fields and builds [`CancelEverything`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<CancelEverything, crate::api::current::BuildError> {
        let user_ids = self
            .user_ids
            .ok_or(crate::api::current::BuildError::missing("userIds"))?;
        if user_ids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "userIds",
                "must not be empty",
            ));
        }
        Ok(CancelEverything {
            user_ids,
            tradovate_subscriptions: self.tradovate_subscriptions,
            user_plugins: self.user_plugins,
            market_data_subscriptions: self.market_data_subscriptions,
            trading_permissions: self.trading_permissions,
        })
    }
}

impl crate::api::current::support::CurrentRequest for CancelEverything {
    fn validate_current(&self) -> Result<(), crate::Error> {
        if self.user_ids.is_empty() {
            return Err(crate::Error::InvalidRequest {
                field: "userIds",
                reason: "must not be empty",
            });
        }
        Ok(())
    }
}

/// Current wire model `CancelEverythingResponse`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct CancelEverythingResponse {
    #[serde(rename = "tradovateSubscriptionIds")]
    tradovate_subscription_ids: Vec<super::ids::TradovateSubscriptionId>,
    #[serde(rename = "userPluginIds")]
    user_plugin_ids: Vec<super::ids::UserPluginId>,
    #[serde(rename = "marketDataSubscriptionIds")]
    market_data_subscription_ids: Vec<super::ids::MarketDataSubscriptionId>,
    #[serde(rename = "tradingPermissionIds")]
    trading_permission_ids: Vec<super::ids::TradingPermissionId>,
}

impl CancelEverythingResponse {
    /// Returns wire field `tradovateSubscriptionIds`.
    #[must_use]
    pub fn tradovate_subscription_ids(&self) -> &[super::ids::TradovateSubscriptionId] {
        &self.tradovate_subscription_ids
    }

    /// Returns wire field `userPluginIds`.
    #[must_use]
    pub fn user_plugin_ids(&self) -> &[super::ids::UserPluginId] {
        &self.user_plugin_ids
    }

    /// Returns wire field `marketDataSubscriptionIds`.
    #[must_use]
    pub fn market_data_subscription_ids(&self) -> &[super::ids::MarketDataSubscriptionId] {
        &self.market_data_subscription_ids
    }

    /// Returns wire field `tradingPermissionIds`.
    #[must_use]
    pub fn trading_permission_ids(&self) -> &[super::ids::TradingPermissionId] {
        &self.trading_permission_ids
    }

    /// Starts a builder for [`CancelEverythingResponse`].
    pub fn builder() -> CancelEverythingResponseBuilder {
        CancelEverythingResponseBuilder::default()
    }
}

/// Builder for [`CancelEverythingResponse`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct CancelEverythingResponseBuilder {
    tradovate_subscription_ids: Option<Vec<super::ids::TradovateSubscriptionId>>,
    user_plugin_ids: Option<Vec<super::ids::UserPluginId>>,
    market_data_subscription_ids: Option<Vec<super::ids::MarketDataSubscriptionId>>,
    trading_permission_ids: Option<Vec<super::ids::TradingPermissionId>>,
}

impl CancelEverythingResponseBuilder {
    /// Sets wire field `tradovateSubscriptionIds`.
    pub fn tradovate_subscription_ids(
        mut self,
        value: Vec<super::ids::TradovateSubscriptionId>,
    ) -> Self {
        self.tradovate_subscription_ids = Some(value);
        self
    }

    /// Sets wire field `userPluginIds`.
    pub fn user_plugin_ids(mut self, value: Vec<super::ids::UserPluginId>) -> Self {
        self.user_plugin_ids = Some(value);
        self
    }

    /// Sets wire field `marketDataSubscriptionIds`.
    pub fn market_data_subscription_ids(
        mut self,
        value: Vec<super::ids::MarketDataSubscriptionId>,
    ) -> Self {
        self.market_data_subscription_ids = Some(value);
        self
    }

    /// Sets wire field `tradingPermissionIds`.
    pub fn trading_permission_ids(mut self, value: Vec<super::ids::TradingPermissionId>) -> Self {
        self.trading_permission_ids = Some(value);
        self
    }

    /// Validates required fields and builds [`CancelEverythingResponse`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<CancelEverythingResponse, crate::api::current::BuildError> {
        let tradovate_subscription_ids =
            self.tradovate_subscription_ids
                .ok_or(crate::api::current::BuildError::missing(
                    "tradovateSubscriptionIds",
                ))?;
        let user_plugin_ids = self
            .user_plugin_ids
            .ok_or(crate::api::current::BuildError::missing("userPluginIds"))?;
        let market_data_subscription_ids =
            self.market_data_subscription_ids
                .ok_or(crate::api::current::BuildError::missing(
                    "marketDataSubscriptionIds",
                ))?;
        let trading_permission_ids =
            self.trading_permission_ids
                .ok_or(crate::api::current::BuildError::missing(
                    "tradingPermissionIds",
                ))?;
        Ok(CancelEverythingResponse {
            tradovate_subscription_ids,
            user_plugin_ids,
            market_data_subscription_ids,
            trading_permission_ids,
        })
    }
}

/// Current wire model `CancelTradovateSubscription`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct CancelTradovateSubscription {
    #[serde(rename = "tradovateSubscriptionId")]
    tradovate_subscription_id: super::ids::TradovateSubscriptionId,
    #[serde(
        rename = "cancelReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    cancel_reason: Option<String>,
    #[serde(rename = "expire", default, skip_serializing_if = "Option::is_none")]
    expire: Option<bool>,
}

impl CancelTradovateSubscription {
    /// Returns wire field `tradovateSubscriptionId`.
    #[must_use]
    pub fn tradovate_subscription_id(&self) -> &super::ids::TradovateSubscriptionId {
        &self.tradovate_subscription_id
    }

    /// Returns wire field `cancelReason`.
    #[must_use]
    pub fn cancel_reason(&self) -> Option<&str> {
        self.cancel_reason.as_deref()
    }

    /// Returns wire field `expire`.
    #[must_use]
    pub fn expire(&self) -> Option<&bool> {
        self.expire.as_ref()
    }

    /// Starts a builder for [`CancelTradovateSubscription`].
    pub fn builder() -> CancelTradovateSubscriptionBuilder {
        CancelTradovateSubscriptionBuilder::default()
    }
}

/// Builder for [`CancelTradovateSubscription`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct CancelTradovateSubscriptionBuilder {
    tradovate_subscription_id: Option<super::ids::TradovateSubscriptionId>,
    cancel_reason: Option<String>,
    expire: Option<bool>,
}

impl CancelTradovateSubscriptionBuilder {
    /// Sets wire field `tradovateSubscriptionId`.
    pub fn tradovate_subscription_id(mut self, value: super::ids::TradovateSubscriptionId) -> Self {
        self.tradovate_subscription_id = Some(value);
        self
    }

    /// Sets wire field `cancelReason`.
    pub fn cancel_reason(mut self, value: impl Into<String>) -> Self {
        self.cancel_reason = Some(value.into());
        self
    }

    /// Sets wire field `expire`.
    pub fn expire(mut self, value: bool) -> Self {
        self.expire = Some(value);
        self
    }

    /// Validates required fields and builds [`CancelTradovateSubscription`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<CancelTradovateSubscription, crate::api::current::BuildError> {
        let tradovate_subscription_id =
            self.tradovate_subscription_id
                .ok_or(crate::api::current::BuildError::missing(
                    "tradovateSubscriptionId",
                ))?;
        Ok(CancelTradovateSubscription {
            tradovate_subscription_id,
            cancel_reason: self.cancel_reason,
            expire: self.expire,
        })
    }
}

impl crate::api::current::support::CurrentRequest for CancelTradovateSubscription {
    fn validate_current(&self) -> Result<(), crate::Error> {
        Ok(())
    }
}

/// Current wire model `CashBalance`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct CashBalance {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    id: Option<super::ids::CashBalanceId>,
    #[serde(rename = "accountId")]
    account_id: crate::AccountId,
    #[serde(rename = "timestamp")]
    timestamp: jiff::Timestamp,
    #[serde(rename = "tradeDate")]
    trade_date: TradeDate,
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
    #[serde(rename = "amountSOD", default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "crate::decimal::option")]
    amount_sod: Option<crate::Decimal>,
}

impl CashBalance {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> Option<&super::ids::CashBalanceId> {
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
    pub fn trade_date(&self) -> &TradeDate {
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

    /// Returns wire field `amountSOD`.
    #[must_use]
    pub fn amount_sod(&self) -> Option<&crate::Decimal> {
        self.amount_sod.as_ref()
    }

    /// Starts a builder for [`CashBalance`].
    pub fn builder() -> CashBalanceBuilder {
        CashBalanceBuilder::default()
    }
}

/// Builder for [`CashBalance`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct CashBalanceBuilder {
    id: Option<super::ids::CashBalanceId>,
    account_id: Option<crate::AccountId>,
    timestamp: Option<jiff::Timestamp>,
    trade_date: Option<TradeDate>,
    currency_id: Option<super::ids::CurrencyId>,
    amount: Option<crate::Decimal>,
    realized_pn_l: Option<crate::Decimal>,
    week_realized_pn_l: Option<crate::Decimal>,
    amount_sod: Option<crate::Decimal>,
}

impl CashBalanceBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::CashBalanceId) -> Self {
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
    pub fn trade_date(mut self, value: TradeDate) -> Self {
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

    /// Sets wire field `amountSOD`.
    pub fn amount_sod(mut self, value: crate::Decimal) -> Self {
        self.amount_sod = Some(value);
        self
    }

    /// Validates required fields and builds [`CashBalance`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<CashBalance, crate::api::current::BuildError> {
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
        Ok(CashBalance {
            id: self.id,
            account_id,
            timestamp,
            trade_date,
            currency_id,
            amount,
            realized_pn_l: self.realized_pn_l,
            week_realized_pn_l: self.week_realized_pn_l,
            amount_sod: self.amount_sod,
        })
    }
}

/// Current wire model `ChangePluginPermission`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ChangePluginPermission {
    #[serde(rename = "userId", default, skip_serializing_if = "Option::is_none")]
    user_id: Option<crate::UserId>,
    #[serde(rename = "pluginName")]
    plugin_name: String,
    #[serde(rename = "approval")]
    approval: bool,
}

impl ChangePluginPermission {
    /// Returns wire field `userId`.
    #[must_use]
    pub fn user_id(&self) -> Option<&crate::UserId> {
        self.user_id.as_ref()
    }

    /// Returns wire field `pluginName`.
    #[must_use]
    pub fn plugin_name(&self) -> &str {
        &self.plugin_name
    }

    /// Returns wire field `approval`.
    #[must_use]
    pub fn approval(&self) -> &bool {
        &self.approval
    }

    /// Starts a builder for [`ChangePluginPermission`].
    pub fn builder() -> ChangePluginPermissionBuilder {
        ChangePluginPermissionBuilder::default()
    }
}

/// Builder for [`ChangePluginPermission`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ChangePluginPermissionBuilder {
    user_id: Option<crate::UserId>,
    plugin_name: Option<String>,
    approval: Option<bool>,
}

impl ChangePluginPermissionBuilder {
    /// Sets wire field `userId`.
    pub fn user_id(mut self, value: crate::UserId) -> Self {
        self.user_id = Some(value);
        self
    }

    /// Sets wire field `pluginName`.
    pub fn plugin_name(mut self, value: impl Into<String>) -> Self {
        self.plugin_name = Some(value.into());
        self
    }

    /// Sets wire field `approval`.
    pub fn approval(mut self, value: bool) -> Self {
        self.approval = Some(value);
        self
    }

    /// Validates required fields and builds [`ChangePluginPermission`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ChangePluginPermission, crate::api::current::BuildError> {
        let plugin_name = self
            .plugin_name
            .ok_or(crate::api::current::BuildError::missing("pluginName"))?;
        if plugin_name.is_empty() || plugin_name.trim() != plugin_name {
            return Err(crate::api::current::BuildError::invalid(
                "pluginName",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        let approval = self
            .approval
            .ok_or(crate::api::current::BuildError::missing("approval"))?;
        Ok(ChangePluginPermission {
            user_id: self.user_id,
            plugin_name,
            approval,
        })
    }
}

impl crate::api::current::support::CurrentRequest for ChangePluginPermission {
    fn validate_current(&self) -> Result<(), crate::Error> {
        if self.plugin_name.is_empty() || self.plugin_name.trim() != self.plugin_name {
            return Err(crate::Error::InvalidRequest {
                field: "pluginName",
                reason: "must be non-empty and have no surrounding whitespace",
            });
        }
        Ok(())
    }
}

/// Current wire model `CheckDuplicate`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct CheckDuplicate {
    #[serde(rename = "userId", default, skip_serializing_if = "Option::is_none")]
    user_id: Option<crate::UserId>,
    #[serde(
        rename = "taxIdentifier",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    tax_identifier: Option<crate::api::current::SecretValue>,
    #[serde(
        rename = "nationalId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    national_id: Option<crate::api::current::SecretValue>,
    #[serde(
        rename = "countryCode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    country_code: Option<String>,
    #[serde(
        rename = "organizationId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    organization_id: Option<super::ids::OrganizationId>,
}

impl CheckDuplicate {
    /// Returns wire field `userId`.
    #[must_use]
    pub fn user_id(&self) -> Option<&crate::UserId> {
        self.user_id.as_ref()
    }

    /// Reports whether secret field `taxIdentifier` is present.
    #[must_use]
    pub const fn has_tax_identifier(&self) -> bool {
        self.tax_identifier.is_some()
    }

    /// Reports whether secret field `nationalId` is present.
    #[must_use]
    pub const fn has_national_id(&self) -> bool {
        self.national_id.is_some()
    }

    /// Returns wire field `countryCode`.
    #[must_use]
    pub fn country_code(&self) -> Option<&str> {
        self.country_code.as_deref()
    }

    /// Returns wire field `organizationId`.
    #[must_use]
    pub fn organization_id(&self) -> Option<&super::ids::OrganizationId> {
        self.organization_id.as_ref()
    }

    /// Starts a builder for [`CheckDuplicate`].
    pub fn builder() -> CheckDuplicateBuilder {
        CheckDuplicateBuilder::default()
    }
}

/// Builder for [`CheckDuplicate`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct CheckDuplicateBuilder {
    user_id: Option<crate::UserId>,
    tax_identifier: Option<crate::api::current::SecretValue>,
    national_id: Option<crate::api::current::SecretValue>,
    country_code: Option<String>,
    organization_id: Option<super::ids::OrganizationId>,
}

impl CheckDuplicateBuilder {
    /// Sets wire field `userId`.
    pub fn user_id(mut self, value: crate::UserId) -> Self {
        self.user_id = Some(value);
        self
    }

    /// Sets wire field `taxIdentifier`.
    pub fn tax_identifier(mut self, value: crate::api::current::SecretValue) -> Self {
        self.tax_identifier = Some(value);
        self
    }

    /// Sets wire field `nationalId`.
    pub fn national_id(mut self, value: crate::api::current::SecretValue) -> Self {
        self.national_id = Some(value);
        self
    }

    /// Sets wire field `countryCode`.
    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    /// Sets wire field `organizationId`.
    pub fn organization_id(mut self, value: super::ids::OrganizationId) -> Self {
        self.organization_id = Some(value);
        self
    }

    /// Validates required fields and builds [`CheckDuplicate`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<CheckDuplicate, crate::api::current::BuildError> {
        Ok(CheckDuplicate {
            user_id: self.user_id,
            tax_identifier: self.tax_identifier,
            national_id: self.national_id,
            country_code: self.country_code,
            organization_id: self.organization_id,
        })
    }
}

impl crate::api::current::support::CurrentRequest for CheckDuplicate {
    fn validate_current(&self) -> Result<(), crate::Error> {
        Ok(())
    }
}

/// Current wire model `Command`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct Command {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    id: Option<crate::CommandId>,
    #[serde(rename = "orderId")]
    order_id: crate::OrderId,
    #[serde(rename = "timestamp")]
    timestamp: jiff::Timestamp,
    #[serde(rename = "clOrdId", default, skip_serializing_if = "Option::is_none")]
    cl_ord_id: Option<crate::ClientOrderId>,
    #[serde(rename = "commandType")]
    command_type: CommandCommandType,
    #[serde(rename = "commandStatus")]
    command_status: CommandCommandStatus,
    #[serde(rename = "senderId", default, skip_serializing_if = "Option::is_none")]
    sender_id: Option<super::ids::SenderId>,
    #[serde(
        rename = "userSessionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    user_session_id: Option<super::ids::UserSessionId>,
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

impl Command {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> Option<&crate::CommandId> {
        self.id.as_ref()
    }

    /// Returns wire field `orderId`.
    #[must_use]
    pub fn order_id(&self) -> &crate::OrderId {
        &self.order_id
    }

    /// Returns wire field `timestamp`.
    #[must_use]
    pub fn timestamp(&self) -> &jiff::Timestamp {
        &self.timestamp
    }

    /// Returns wire field `clOrdId`.
    #[must_use]
    pub fn cl_ord_id(&self) -> Option<&crate::ClientOrderId> {
        self.cl_ord_id.as_ref()
    }

    /// Returns wire field `commandType`.
    #[must_use]
    pub fn command_type(&self) -> &CommandCommandType {
        &self.command_type
    }

    /// Returns wire field `commandStatus`.
    #[must_use]
    pub fn command_status(&self) -> &CommandCommandStatus {
        &self.command_status
    }

    /// Returns wire field `senderId`.
    #[must_use]
    pub fn sender_id(&self) -> Option<&super::ids::SenderId> {
        self.sender_id.as_ref()
    }

    /// Returns wire field `userSessionId`.
    #[must_use]
    pub fn user_session_id(&self) -> Option<&super::ids::UserSessionId> {
        self.user_session_id.as_ref()
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

    /// Starts a builder for [`Command`].
    pub fn builder() -> CommandBuilder {
        CommandBuilder::default()
    }
}

/// Builder for [`Command`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct CommandBuilder {
    id: Option<crate::CommandId>,
    order_id: Option<crate::OrderId>,
    timestamp: Option<jiff::Timestamp>,
    cl_ord_id: Option<crate::ClientOrderId>,
    command_type: Option<CommandCommandType>,
    command_status: Option<CommandCommandStatus>,
    sender_id: Option<super::ids::SenderId>,
    user_session_id: Option<super::ids::UserSessionId>,
    activation_time: Option<jiff::Timestamp>,
    custom_tag50: Option<String>,
    is_automated: Option<bool>,
}

impl CommandBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: crate::CommandId) -> Self {
        self.id = Some(value);
        self
    }

    /// Sets wire field `orderId`.
    pub fn order_id(mut self, value: crate::OrderId) -> Self {
        self.order_id = Some(value);
        self
    }

    /// Sets wire field `timestamp`.
    pub fn timestamp(mut self, value: jiff::Timestamp) -> Self {
        self.timestamp = Some(value);
        self
    }

    /// Sets wire field `clOrdId`.
    pub fn cl_ord_id(mut self, value: crate::ClientOrderId) -> Self {
        self.cl_ord_id = Some(value);
        self
    }

    /// Sets wire field `commandType`.
    pub fn command_type(mut self, value: CommandCommandType) -> Self {
        self.command_type = Some(value);
        self
    }

    /// Sets wire field `commandStatus`.
    pub fn command_status(mut self, value: CommandCommandStatus) -> Self {
        self.command_status = Some(value);
        self
    }

    /// Sets wire field `senderId`.
    pub fn sender_id(mut self, value: super::ids::SenderId) -> Self {
        self.sender_id = Some(value);
        self
    }

    /// Sets wire field `userSessionId`.
    pub fn user_session_id(mut self, value: super::ids::UserSessionId) -> Self {
        self.user_session_id = Some(value);
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

    /// Validates required fields and builds [`Command`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<Command, crate::api::current::BuildError> {
        let order_id = self
            .order_id
            .ok_or(crate::api::current::BuildError::missing("orderId"))?;
        let timestamp = self
            .timestamp
            .ok_or(crate::api::current::BuildError::missing("timestamp"))?;
        let command_type = self
            .command_type
            .ok_or(crate::api::current::BuildError::missing("commandType"))?;
        let command_status = self
            .command_status
            .ok_or(crate::api::current::BuildError::missing("commandStatus"))?;
        Ok(Command {
            id: self.id,
            order_id,
            timestamp,
            cl_ord_id: self.cl_ord_id,
            command_type,
            command_status,
            sender_id: self.sender_id,
            user_session_id: self.user_session_id,
            activation_time: self.activation_time,
            custom_tag50: self.custom_tag50,
            is_automated: self.is_automated,
        })
    }
}

/// Current provider values for `CommandCommandStatus`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum CommandCommandStatus {
    /// Provider value `AtExecution`.
    AtExecution,
    /// Provider value `ExecutionRejected`.
    ExecutionRejected,
    /// Provider value `ExecutionStopped`.
    ExecutionStopped,
    /// Provider value `ExecutionSuspended`.
    ExecutionSuspended,
    /// Provider value `OnHold`.
    OnHold,
    /// Provider value `Pending`.
    Pending,
    /// Provider value `PendingExecution`.
    PendingExecution,
    /// Provider value `Replaced`.
    Replaced,
    /// Provider value `RiskPassed`.
    RiskPassed,
    /// Provider value `RiskRejected`.
    RiskRejected,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl CommandCommandStatus {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::AtExecution => "AtExecution",
            Self::ExecutionRejected => "ExecutionRejected",
            Self::ExecutionStopped => "ExecutionStopped",
            Self::ExecutionSuspended => "ExecutionSuspended",
            Self::OnHold => "OnHold",
            Self::Pending => "Pending",
            Self::PendingExecution => "PendingExecution",
            Self::Replaced => "Replaced",
            Self::RiskPassed => "RiskPassed",
            Self::RiskRejected => "RiskRejected",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for CommandCommandStatus {
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

impl<'de> serde::Deserialize<'de> for CommandCommandStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "AtExecution" => Self::AtExecution,
            "ExecutionRejected" => Self::ExecutionRejected,
            "ExecutionStopped" => Self::ExecutionStopped,
            "ExecutionSuspended" => Self::ExecutionSuspended,
            "OnHold" => Self::OnHold,
            "Pending" => Self::Pending,
            "PendingExecution" => Self::PendingExecution,
            "Replaced" => Self::Replaced,
            "RiskPassed" => Self::RiskPassed,
            "RiskRejected" => Self::RiskRejected,
            _ => Self::Unknown(value),
        })
    }
}

/// Current provider values for `CommandCommandType`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum CommandCommandType {
    /// Provider value `Cancel`.
    Cancel,
    /// Provider value `Modify`.
    Modify,
    /// Provider value `New`.
    New,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl CommandCommandType {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Cancel => "Cancel",
            Self::Modify => "Modify",
            Self::New => "New",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for CommandCommandType {
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

impl<'de> serde::Deserialize<'de> for CommandCommandType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "Cancel" => Self::Cancel,
            "Modify" => Self::Modify,
            "New" => Self::New,
            _ => Self::Unknown(value),
        })
    }
}

/// Current wire model `CommandReport`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct CommandReport {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    id: Option<super::ids::CommandReportId>,
    #[serde(rename = "commandId")]
    command_id: crate::CommandId,
    #[serde(rename = "timestamp")]
    timestamp: jiff::Timestamp,
    #[serde(rename = "commandStatus")]
    command_status: CommandReportCommandStatus,
    #[serde(
        rename = "rejectReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    reject_reason: Option<CommandReportRejectReason>,
    #[serde(rename = "text", default, skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(rename = "ordStatus", default, skip_serializing_if = "Option::is_none")]
    ord_status: Option<CommandReportOrdStatus>,
}

impl CommandReport {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> Option<&super::ids::CommandReportId> {
        self.id.as_ref()
    }

    /// Returns wire field `commandId`.
    #[must_use]
    pub fn command_id(&self) -> &crate::CommandId {
        &self.command_id
    }

    /// Returns wire field `timestamp`.
    #[must_use]
    pub fn timestamp(&self) -> &jiff::Timestamp {
        &self.timestamp
    }

    /// Returns wire field `commandStatus`.
    #[must_use]
    pub fn command_status(&self) -> &CommandReportCommandStatus {
        &self.command_status
    }

    /// Returns wire field `rejectReason`.
    #[must_use]
    pub fn reject_reason(&self) -> Option<&CommandReportRejectReason> {
        self.reject_reason.as_ref()
    }

    /// Returns wire field `text`.
    #[must_use]
    pub fn text(&self) -> Option<&str> {
        self.text.as_deref()
    }

    /// Returns wire field `ordStatus`.
    #[must_use]
    pub fn ord_status(&self) -> Option<&CommandReportOrdStatus> {
        self.ord_status.as_ref()
    }

    /// Starts a builder for [`CommandReport`].
    pub fn builder() -> CommandReportBuilder {
        CommandReportBuilder::default()
    }
}

/// Builder for [`CommandReport`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct CommandReportBuilder {
    id: Option<super::ids::CommandReportId>,
    command_id: Option<crate::CommandId>,
    timestamp: Option<jiff::Timestamp>,
    command_status: Option<CommandReportCommandStatus>,
    reject_reason: Option<CommandReportRejectReason>,
    text: Option<String>,
    ord_status: Option<CommandReportOrdStatus>,
}

impl CommandReportBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::CommandReportId) -> Self {
        self.id = Some(value);
        self
    }

    /// Sets wire field `commandId`.
    pub fn command_id(mut self, value: crate::CommandId) -> Self {
        self.command_id = Some(value);
        self
    }

    /// Sets wire field `timestamp`.
    pub fn timestamp(mut self, value: jiff::Timestamp) -> Self {
        self.timestamp = Some(value);
        self
    }

    /// Sets wire field `commandStatus`.
    pub fn command_status(mut self, value: CommandReportCommandStatus) -> Self {
        self.command_status = Some(value);
        self
    }

    /// Sets wire field `rejectReason`.
    pub fn reject_reason(mut self, value: CommandReportRejectReason) -> Self {
        self.reject_reason = Some(value);
        self
    }

    /// Sets wire field `text`.
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Sets wire field `ordStatus`.
    pub fn ord_status(mut self, value: CommandReportOrdStatus) -> Self {
        self.ord_status = Some(value);
        self
    }

    /// Validates required fields and builds [`CommandReport`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<CommandReport, crate::api::current::BuildError> {
        let command_id = self
            .command_id
            .ok_or(crate::api::current::BuildError::missing("commandId"))?;
        let timestamp = self
            .timestamp
            .ok_or(crate::api::current::BuildError::missing("timestamp"))?;
        let command_status = self
            .command_status
            .ok_or(crate::api::current::BuildError::missing("commandStatus"))?;
        Ok(CommandReport {
            id: self.id,
            command_id,
            timestamp,
            command_status,
            reject_reason: self.reject_reason,
            text: self.text,
            ord_status: self.ord_status,
        })
    }
}

/// Current provider values for `CommandReportCommandStatus`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum CommandReportCommandStatus {
    /// Provider value `AtExecution`.
    AtExecution,
    /// Provider value `ExecutionRejected`.
    ExecutionRejected,
    /// Provider value `ExecutionStopped`.
    ExecutionStopped,
    /// Provider value `ExecutionSuspended`.
    ExecutionSuspended,
    /// Provider value `OnHold`.
    OnHold,
    /// Provider value `Pending`.
    Pending,
    /// Provider value `PendingExecution`.
    PendingExecution,
    /// Provider value `Replaced`.
    Replaced,
    /// Provider value `RiskPassed`.
    RiskPassed,
    /// Provider value `RiskRejected`.
    RiskRejected,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl CommandReportCommandStatus {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::AtExecution => "AtExecution",
            Self::ExecutionRejected => "ExecutionRejected",
            Self::ExecutionStopped => "ExecutionStopped",
            Self::ExecutionSuspended => "ExecutionSuspended",
            Self::OnHold => "OnHold",
            Self::Pending => "Pending",
            Self::PendingExecution => "PendingExecution",
            Self::Replaced => "Replaced",
            Self::RiskPassed => "RiskPassed",
            Self::RiskRejected => "RiskRejected",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for CommandReportCommandStatus {
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

impl<'de> serde::Deserialize<'de> for CommandReportCommandStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "AtExecution" => Self::AtExecution,
            "ExecutionRejected" => Self::ExecutionRejected,
            "ExecutionStopped" => Self::ExecutionStopped,
            "ExecutionSuspended" => Self::ExecutionSuspended,
            "OnHold" => Self::OnHold,
            "Pending" => Self::Pending,
            "PendingExecution" => Self::PendingExecution,
            "Replaced" => Self::Replaced,
            "RiskPassed" => Self::RiskPassed,
            "RiskRejected" => Self::RiskRejected,
            _ => Self::Unknown(value),
        })
    }
}

/// Current provider values for `CommandReportOrdStatus`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum CommandReportOrdStatus {
    /// Provider value `Canceled`.
    Canceled,
    /// Provider value `Completed`.
    Completed,
    /// Provider value `Expired`.
    Expired,
    /// Provider value `Filled`.
    Filled,
    /// Provider value `PendingCancel`.
    PendingCancel,
    /// Provider value `PendingNew`.
    PendingNew,
    /// Provider value `PendingReplace`.
    PendingReplace,
    /// Provider value `Rejected`.
    Rejected,
    /// Provider value `Suspended`.
    Suspended,
    /// Provider value `Unknown`.
    Unknown2,
    /// Provider value `Working`.
    Working,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl CommandReportOrdStatus {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Canceled => "Canceled",
            Self::Completed => "Completed",
            Self::Expired => "Expired",
            Self::Filled => "Filled",
            Self::PendingCancel => "PendingCancel",
            Self::PendingNew => "PendingNew",
            Self::PendingReplace => "PendingReplace",
            Self::Rejected => "Rejected",
            Self::Suspended => "Suspended",
            Self::Unknown2 => "Unknown",
            Self::Working => "Working",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for CommandReportOrdStatus {
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

impl<'de> serde::Deserialize<'de> for CommandReportOrdStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "Canceled" => Self::Canceled,
            "Completed" => Self::Completed,
            "Expired" => Self::Expired,
            "Filled" => Self::Filled,
            "PendingCancel" => Self::PendingCancel,
            "PendingNew" => Self::PendingNew,
            "PendingReplace" => Self::PendingReplace,
            "Rejected" => Self::Rejected,
            "Suspended" => Self::Suspended,
            "Unknown" => Self::Unknown2,
            "Working" => Self::Working,
            _ => Self::Unknown(value),
        })
    }
}

/// Current provider values for `CommandReportRejectReason`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum CommandReportRejectReason {
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

impl CommandReportRejectReason {
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

impl serde::Serialize for CommandReportRejectReason {
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

impl<'de> serde::Deserialize<'de> for CommandReportRejectReason {
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

/// Current wire model `ContactInfo`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ContactInfo {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    id: Option<super::ids::ContactInfoId>,
    #[serde(rename = "userId")]
    user_id: crate::UserId,
    #[serde(rename = "firstName")]
    first_name: String,
    #[serde(rename = "lastName")]
    last_name: String,
    #[serde(rename = "streetAddress1")]
    street_address1: String,
    #[serde(
        rename = "streetAddress2",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    street_address2: Option<String>,
    #[serde(rename = "city")]
    city: String,
    #[serde(rename = "state", default, skip_serializing_if = "Option::is_none")]
    state: Option<String>,
    #[serde(rename = "postCode", default, skip_serializing_if = "Option::is_none")]
    post_code: Option<String>,
    #[serde(rename = "country")]
    country: String,
    #[serde(rename = "phone")]
    phone: String,
    #[serde(
        rename = "mailingIsDifferent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    mailing_is_different: Option<bool>,
    #[serde(
        rename = "mailingStreetAddress1",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    mailing_street_address1: Option<String>,
    #[serde(
        rename = "mailingStreetAddress2",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    mailing_street_address2: Option<String>,
    #[serde(
        rename = "mailingCity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    mailing_city: Option<String>,
    #[serde(
        rename = "mailingState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    mailing_state: Option<String>,
    #[serde(
        rename = "mailingPostCode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    mailing_post_code: Option<String>,
    #[serde(
        rename = "mailingCountry",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    mailing_country: Option<String>,
    #[serde(
        rename = "approvedId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    approved_id: Option<super::ids::ApprovedId>,
    #[serde(
        rename = "jointFirstName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    joint_first_name: Option<String>,
    #[serde(
        rename = "jointLastName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    joint_last_name: Option<String>,
    #[serde(
        rename = "iraCustodianName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    ira_custodian_name: Option<String>,
}

impl ContactInfo {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> Option<&super::ids::ContactInfoId> {
        self.id.as_ref()
    }

    /// Returns wire field `userId`.
    #[must_use]
    pub fn user_id(&self) -> &crate::UserId {
        &self.user_id
    }

    /// Returns wire field `firstName`.
    #[must_use]
    pub fn first_name(&self) -> &str {
        &self.first_name
    }

    /// Returns wire field `lastName`.
    #[must_use]
    pub fn last_name(&self) -> &str {
        &self.last_name
    }

    /// Returns wire field `streetAddress1`.
    #[must_use]
    pub fn street_address1(&self) -> &str {
        &self.street_address1
    }

    /// Returns wire field `streetAddress2`.
    #[must_use]
    pub fn street_address2(&self) -> Option<&str> {
        self.street_address2.as_deref()
    }

    /// Returns wire field `city`.
    #[must_use]
    pub fn city(&self) -> &str {
        &self.city
    }

    /// Returns wire field `state`.
    #[must_use]
    pub fn state(&self) -> Option<&str> {
        self.state.as_deref()
    }

    /// Returns wire field `postCode`.
    #[must_use]
    pub fn post_code(&self) -> Option<&str> {
        self.post_code.as_deref()
    }

    /// Returns wire field `country`.
    #[must_use]
    pub fn country(&self) -> &str {
        &self.country
    }

    /// Returns wire field `phone`.
    #[must_use]
    pub fn phone(&self) -> &str {
        &self.phone
    }

    /// Returns wire field `mailingIsDifferent`.
    #[must_use]
    pub fn mailing_is_different(&self) -> Option<&bool> {
        self.mailing_is_different.as_ref()
    }

    /// Returns wire field `mailingStreetAddress1`.
    #[must_use]
    pub fn mailing_street_address1(&self) -> Option<&str> {
        self.mailing_street_address1.as_deref()
    }

    /// Returns wire field `mailingStreetAddress2`.
    #[must_use]
    pub fn mailing_street_address2(&self) -> Option<&str> {
        self.mailing_street_address2.as_deref()
    }

    /// Returns wire field `mailingCity`.
    #[must_use]
    pub fn mailing_city(&self) -> Option<&str> {
        self.mailing_city.as_deref()
    }

    /// Returns wire field `mailingState`.
    #[must_use]
    pub fn mailing_state(&self) -> Option<&str> {
        self.mailing_state.as_deref()
    }

    /// Returns wire field `mailingPostCode`.
    #[must_use]
    pub fn mailing_post_code(&self) -> Option<&str> {
        self.mailing_post_code.as_deref()
    }

    /// Returns wire field `mailingCountry`.
    #[must_use]
    pub fn mailing_country(&self) -> Option<&str> {
        self.mailing_country.as_deref()
    }

    /// Returns wire field `approvedId`.
    #[must_use]
    pub fn approved_id(&self) -> Option<&super::ids::ApprovedId> {
        self.approved_id.as_ref()
    }

    /// Returns wire field `jointFirstName`.
    #[must_use]
    pub fn joint_first_name(&self) -> Option<&str> {
        self.joint_first_name.as_deref()
    }

    /// Returns wire field `jointLastName`.
    #[must_use]
    pub fn joint_last_name(&self) -> Option<&str> {
        self.joint_last_name.as_deref()
    }

    /// Returns wire field `iraCustodianName`.
    #[must_use]
    pub fn ira_custodian_name(&self) -> Option<&str> {
        self.ira_custodian_name.as_deref()
    }

    /// Starts a builder for [`ContactInfo`].
    pub fn builder() -> ContactInfoBuilder {
        ContactInfoBuilder::default()
    }
}

/// Builder for [`ContactInfo`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ContactInfoBuilder {
    id: Option<super::ids::ContactInfoId>,
    user_id: Option<crate::UserId>,
    first_name: Option<String>,
    last_name: Option<String>,
    street_address1: Option<String>,
    street_address2: Option<String>,
    city: Option<String>,
    state: Option<String>,
    post_code: Option<String>,
    country: Option<String>,
    phone: Option<String>,
    mailing_is_different: Option<bool>,
    mailing_street_address1: Option<String>,
    mailing_street_address2: Option<String>,
    mailing_city: Option<String>,
    mailing_state: Option<String>,
    mailing_post_code: Option<String>,
    mailing_country: Option<String>,
    approved_id: Option<super::ids::ApprovedId>,
    joint_first_name: Option<String>,
    joint_last_name: Option<String>,
    ira_custodian_name: Option<String>,
}

impl ContactInfoBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::ContactInfoId) -> Self {
        self.id = Some(value);
        self
    }

    /// Sets wire field `userId`.
    pub fn user_id(mut self, value: crate::UserId) -> Self {
        self.user_id = Some(value);
        self
    }

    /// Sets wire field `firstName`.
    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    /// Sets wire field `lastName`.
    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
        self
    }

    /// Sets wire field `streetAddress1`.
    pub fn street_address1(mut self, value: impl Into<String>) -> Self {
        self.street_address1 = Some(value.into());
        self
    }

    /// Sets wire field `streetAddress2`.
    pub fn street_address2(mut self, value: impl Into<String>) -> Self {
        self.street_address2 = Some(value.into());
        self
    }

    /// Sets wire field `city`.
    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    /// Sets wire field `state`.
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    /// Sets wire field `postCode`.
    pub fn post_code(mut self, value: impl Into<String>) -> Self {
        self.post_code = Some(value.into());
        self
    }

    /// Sets wire field `country`.
    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    /// Sets wire field `phone`.
    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    /// Sets wire field `mailingIsDifferent`.
    pub fn mailing_is_different(mut self, value: bool) -> Self {
        self.mailing_is_different = Some(value);
        self
    }

    /// Sets wire field `mailingStreetAddress1`.
    pub fn mailing_street_address1(mut self, value: impl Into<String>) -> Self {
        self.mailing_street_address1 = Some(value.into());
        self
    }

    /// Sets wire field `mailingStreetAddress2`.
    pub fn mailing_street_address2(mut self, value: impl Into<String>) -> Self {
        self.mailing_street_address2 = Some(value.into());
        self
    }

    /// Sets wire field `mailingCity`.
    pub fn mailing_city(mut self, value: impl Into<String>) -> Self {
        self.mailing_city = Some(value.into());
        self
    }

    /// Sets wire field `mailingState`.
    pub fn mailing_state(mut self, value: impl Into<String>) -> Self {
        self.mailing_state = Some(value.into());
        self
    }

    /// Sets wire field `mailingPostCode`.
    pub fn mailing_post_code(mut self, value: impl Into<String>) -> Self {
        self.mailing_post_code = Some(value.into());
        self
    }

    /// Sets wire field `mailingCountry`.
    pub fn mailing_country(mut self, value: impl Into<String>) -> Self {
        self.mailing_country = Some(value.into());
        self
    }

    /// Sets wire field `approvedId`.
    pub fn approved_id(mut self, value: super::ids::ApprovedId) -> Self {
        self.approved_id = Some(value);
        self
    }

    /// Sets wire field `jointFirstName`.
    pub fn joint_first_name(mut self, value: impl Into<String>) -> Self {
        self.joint_first_name = Some(value.into());
        self
    }

    /// Sets wire field `jointLastName`.
    pub fn joint_last_name(mut self, value: impl Into<String>) -> Self {
        self.joint_last_name = Some(value.into());
        self
    }

    /// Sets wire field `iraCustodianName`.
    pub fn ira_custodian_name(mut self, value: impl Into<String>) -> Self {
        self.ira_custodian_name = Some(value.into());
        self
    }

    /// Validates required fields and builds [`ContactInfo`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ContactInfo, crate::api::current::BuildError> {
        let user_id = self
            .user_id
            .ok_or(crate::api::current::BuildError::missing("userId"))?;
        let first_name = self
            .first_name
            .ok_or(crate::api::current::BuildError::missing("firstName"))?;
        let last_name = self
            .last_name
            .ok_or(crate::api::current::BuildError::missing("lastName"))?;
        let street_address1 = self
            .street_address1
            .ok_or(crate::api::current::BuildError::missing("streetAddress1"))?;
        let city = self
            .city
            .ok_or(crate::api::current::BuildError::missing("city"))?;
        let country = self
            .country
            .ok_or(crate::api::current::BuildError::missing("country"))?;
        let phone = self
            .phone
            .ok_or(crate::api::current::BuildError::missing("phone"))?;
        Ok(ContactInfo {
            id: self.id,
            user_id,
            first_name,
            last_name,
            street_address1,
            street_address2: self.street_address2,
            city,
            state: self.state,
            post_code: self.post_code,
            country,
            phone,
            mailing_is_different: self.mailing_is_different,
            mailing_street_address1: self.mailing_street_address1,
            mailing_street_address2: self.mailing_street_address2,
            mailing_city: self.mailing_city,
            mailing_state: self.mailing_state,
            mailing_post_code: self.mailing_post_code,
            mailing_country: self.mailing_country,
            approved_id: self.approved_id,
            joint_first_name: self.joint_first_name,
            joint_last_name: self.joint_last_name,
            ira_custodian_name: self.ira_custodian_name,
        })
    }
}

/// Current wire model `Contract`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct Contract {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    id: Option<crate::ContractId>,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "contractMaturityId")]
    contract_maturity_id: crate::ContractMaturityId,
    #[serde(rename = "timestamp")]
    timestamp: jiff::Timestamp,
}

impl Contract {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> Option<&crate::ContractId> {
        self.id.as_ref()
    }

    /// Returns wire field `name`.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns wire field `contractMaturityId`.
    #[must_use]
    pub fn contract_maturity_id(&self) -> &crate::ContractMaturityId {
        &self.contract_maturity_id
    }

    /// Returns wire field `timestamp`.
    #[must_use]
    pub fn timestamp(&self) -> &jiff::Timestamp {
        &self.timestamp
    }

    /// Starts a builder for [`Contract`].
    pub fn builder() -> ContractBuilder {
        ContractBuilder::default()
    }
}

/// Builder for [`Contract`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ContractBuilder {
    id: Option<crate::ContractId>,
    name: Option<String>,
    contract_maturity_id: Option<crate::ContractMaturityId>,
    timestamp: Option<jiff::Timestamp>,
}

impl ContractBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: crate::ContractId) -> Self {
        self.id = Some(value);
        self
    }

    /// Sets wire field `name`.
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Sets wire field `contractMaturityId`.
    pub fn contract_maturity_id(mut self, value: crate::ContractMaturityId) -> Self {
        self.contract_maturity_id = Some(value);
        self
    }

    /// Sets wire field `timestamp`.
    pub fn timestamp(mut self, value: jiff::Timestamp) -> Self {
        self.timestamp = Some(value);
        self
    }

    /// Validates required fields and builds [`Contract`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<Contract, crate::api::current::BuildError> {
        let name = self
            .name
            .ok_or(crate::api::current::BuildError::missing("name"))?;
        let contract_maturity_id =
            self.contract_maturity_id
                .ok_or(crate::api::current::BuildError::missing(
                    "contractMaturityId",
                ))?;
        let timestamp = self
            .timestamp
            .ok_or(crate::api::current::BuildError::missing("timestamp"))?;
        Ok(Contract {
            id: self.id,
            name,
            contract_maturity_id,
            timestamp,
        })
    }
}

/// Current wire model `ContractGroup`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ContractGroup {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    id: Option<super::ids::ContractGroupId>,
    #[serde(rename = "name")]
    name: String,
}

impl ContractGroup {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> Option<&super::ids::ContractGroupId> {
        self.id.as_ref()
    }

    /// Returns wire field `name`.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Starts a builder for [`ContractGroup`].
    pub fn builder() -> ContractGroupBuilder {
        ContractGroupBuilder::default()
    }
}

/// Builder for [`ContractGroup`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ContractGroupBuilder {
    id: Option<super::ids::ContractGroupId>,
    name: Option<String>,
}

impl ContractGroupBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::ContractGroupId) -> Self {
        self.id = Some(value);
        self
    }

    /// Sets wire field `name`.
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Validates required fields and builds [`ContractGroup`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ContractGroup, crate::api::current::BuildError> {
        let name = self
            .name
            .ok_or(crate::api::current::BuildError::missing("name"))?;
        Ok(ContractGroup { id: self.id, name })
    }
}

/// Current wire model `ContractMaturity`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ContractMaturity {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    id: Option<crate::ContractMaturityId>,
    #[serde(rename = "productId")]
    product_id: super::ids::ProductId,
    #[serde(rename = "expirationMonth")]
    expiration_month: i64,
    #[serde(rename = "expirationDate")]
    expiration_date: jiff::Timestamp,
    #[serde(
        rename = "firstIntentDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    first_intent_date: Option<jiff::Timestamp>,
    #[serde(
        rename = "underlyingId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    underlying_id: Option<crate::ContractMaturityId>,
    #[serde(rename = "isFront")]
    is_front: bool,
    #[serde(
        rename = "kalshiEventId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    kalshi_event_id: Option<super::ids::KalshiEventId>,
}

impl ContractMaturity {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> Option<&crate::ContractMaturityId> {
        self.id.as_ref()
    }

    /// Returns wire field `productId`.
    #[must_use]
    pub fn product_id(&self) -> &super::ids::ProductId {
        &self.product_id
    }

    /// Returns wire field `expirationMonth`.
    #[must_use]
    pub fn expiration_month(&self) -> &i64 {
        &self.expiration_month
    }

    /// Returns wire field `expirationDate`.
    #[must_use]
    pub fn expiration_date(&self) -> &jiff::Timestamp {
        &self.expiration_date
    }

    /// Returns wire field `firstIntentDate`.
    #[must_use]
    pub fn first_intent_date(&self) -> Option<&jiff::Timestamp> {
        self.first_intent_date.as_ref()
    }

    /// Returns wire field `underlyingId`.
    #[must_use]
    pub fn underlying_id(&self) -> Option<&crate::ContractMaturityId> {
        self.underlying_id.as_ref()
    }

    /// Returns wire field `isFront`.
    #[must_use]
    pub fn is_front(&self) -> &bool {
        &self.is_front
    }

    /// Returns wire field `kalshiEventId`.
    #[must_use]
    pub fn kalshi_event_id(&self) -> Option<&super::ids::KalshiEventId> {
        self.kalshi_event_id.as_ref()
    }

    /// Starts a builder for [`ContractMaturity`].
    pub fn builder() -> ContractMaturityBuilder {
        ContractMaturityBuilder::default()
    }
}

/// Builder for [`ContractMaturity`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ContractMaturityBuilder {
    id: Option<crate::ContractMaturityId>,
    product_id: Option<super::ids::ProductId>,
    expiration_month: Option<i64>,
    expiration_date: Option<jiff::Timestamp>,
    first_intent_date: Option<jiff::Timestamp>,
    underlying_id: Option<crate::ContractMaturityId>,
    is_front: Option<bool>,
    kalshi_event_id: Option<super::ids::KalshiEventId>,
}

impl ContractMaturityBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: crate::ContractMaturityId) -> Self {
        self.id = Some(value);
        self
    }

    /// Sets wire field `productId`.
    pub fn product_id(mut self, value: super::ids::ProductId) -> Self {
        self.product_id = Some(value);
        self
    }

    /// Sets wire field `expirationMonth`.
    pub fn expiration_month(mut self, value: i64) -> Self {
        self.expiration_month = Some(value);
        self
    }

    /// Sets wire field `expirationDate`.
    pub fn expiration_date(mut self, value: jiff::Timestamp) -> Self {
        self.expiration_date = Some(value);
        self
    }

    /// Sets wire field `firstIntentDate`.
    pub fn first_intent_date(mut self, value: jiff::Timestamp) -> Self {
        self.first_intent_date = Some(value);
        self
    }

    /// Sets wire field `underlyingId`.
    pub fn underlying_id(mut self, value: crate::ContractMaturityId) -> Self {
        self.underlying_id = Some(value);
        self
    }

    /// Sets wire field `isFront`.
    pub fn is_front(mut self, value: bool) -> Self {
        self.is_front = Some(value);
        self
    }

    /// Sets wire field `kalshiEventId`.
    pub fn kalshi_event_id(mut self, value: super::ids::KalshiEventId) -> Self {
        self.kalshi_event_id = Some(value);
        self
    }

    /// Validates required fields and builds [`ContractMaturity`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ContractMaturity, crate::api::current::BuildError> {
        let product_id = self
            .product_id
            .ok_or(crate::api::current::BuildError::missing("productId"))?;
        let expiration_month = self
            .expiration_month
            .ok_or(crate::api::current::BuildError::missing("expirationMonth"))?;
        let expiration_date = self
            .expiration_date
            .ok_or(crate::api::current::BuildError::missing("expirationDate"))?;
        let is_front = self
            .is_front
            .ok_or(crate::api::current::BuildError::missing("isFront"))?;
        Ok(ContractMaturity {
            id: self.id,
            product_id,
            expiration_month,
            expiration_date,
            first_intent_date: self.first_intent_date,
            underlying_id: self.underlying_id,
            is_front,
            kalshi_event_id: self.kalshi_event_id,
        })
    }
}

/// Current wire model `CreateEvaluationAccountResponse`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct CreateEvaluationAccountResponse {
    #[serde(rename = "errorText", default, skip_serializing_if = "Option::is_none")]
    error_text: Option<String>,
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    account_id: Option<crate::AccountId>,
    #[serde(
        rename = "tradingPermissionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    trading_permission_id: Option<super::ids::TradingPermissionId>,
}

impl CreateEvaluationAccountResponse {
    /// Returns wire field `errorText`.
    #[must_use]
    pub fn error_text(&self) -> Option<&str> {
        self.error_text.as_deref()
    }

    /// Returns wire field `accountId`.
    #[must_use]
    pub fn account_id(&self) -> Option<&crate::AccountId> {
        self.account_id.as_ref()
    }

    /// Returns wire field `tradingPermissionId`.
    #[must_use]
    pub fn trading_permission_id(&self) -> Option<&super::ids::TradingPermissionId> {
        self.trading_permission_id.as_ref()
    }

    /// Starts a builder for [`CreateEvaluationAccountResponse`].
    pub fn builder() -> CreateEvaluationAccountResponseBuilder {
        CreateEvaluationAccountResponseBuilder::default()
    }
}

/// Builder for [`CreateEvaluationAccountResponse`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct CreateEvaluationAccountResponseBuilder {
    error_text: Option<String>,
    account_id: Option<crate::AccountId>,
    trading_permission_id: Option<super::ids::TradingPermissionId>,
}

impl CreateEvaluationAccountResponseBuilder {
    /// Sets wire field `errorText`.
    pub fn error_text(mut self, value: impl Into<String>) -> Self {
        self.error_text = Some(value.into());
        self
    }

    /// Sets wire field `accountId`.
    pub fn account_id(mut self, value: crate::AccountId) -> Self {
        self.account_id = Some(value);
        self
    }

    /// Sets wire field `tradingPermissionId`.
    pub fn trading_permission_id(mut self, value: super::ids::TradingPermissionId) -> Self {
        self.trading_permission_id = Some(value);
        self
    }

    /// Validates required fields and builds [`CreateEvaluationAccountResponse`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<CreateEvaluationAccountResponse, crate::api::current::BuildError> {
        Ok(CreateEvaluationAccountResponse {
            error_text: self.error_text,
            account_id: self.account_id,
            trading_permission_id: self.trading_permission_id,
        })
    }
}

/// Current wire model `CreateEvaluationAccounts`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct CreateEvaluationAccounts {
    #[serde(rename = "accounts")]
    accounts: Vec<EvaluationAccount>,
}

impl CreateEvaluationAccounts {
    /// Returns wire field `accounts`.
    #[must_use]
    pub fn accounts(&self) -> &[EvaluationAccount] {
        &self.accounts
    }

    /// Starts a builder for [`CreateEvaluationAccounts`].
    pub fn builder() -> CreateEvaluationAccountsBuilder {
        CreateEvaluationAccountsBuilder::default()
    }
}

/// Builder for [`CreateEvaluationAccounts`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct CreateEvaluationAccountsBuilder {
    accounts: Option<Vec<EvaluationAccount>>,
}

impl CreateEvaluationAccountsBuilder {
    /// Sets wire field `accounts`.
    pub fn accounts(mut self, value: Vec<EvaluationAccount>) -> Self {
        self.accounts = Some(value);
        self
    }

    /// Validates required fields and builds [`CreateEvaluationAccounts`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<CreateEvaluationAccounts, crate::api::current::BuildError> {
        let accounts = self
            .accounts
            .ok_or(crate::api::current::BuildError::missing("accounts"))?;
        if accounts.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "accounts",
                "must not be empty",
            ));
        }
        Ok(CreateEvaluationAccounts { accounts })
    }
}

impl crate::api::current::support::CurrentRequest for CreateEvaluationAccounts {
    fn validate_current(&self) -> Result<(), crate::Error> {
        if self.accounts.is_empty() {
            return Err(crate::Error::InvalidRequest {
                field: "accounts",
                reason: "must not be empty",
            });
        }
        Ok(())
    }
}

/// Current wire model `CreateEvaluationAccountsResponse`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct CreateEvaluationAccountsResponse {
    #[serde(rename = "errorText", default, skip_serializing_if = "Option::is_none")]
    error_text: Option<String>,
    #[serde(rename = "results")]
    results: Vec<CreateEvaluationAccountResponse>,
}

impl CreateEvaluationAccountsResponse {
    /// Returns wire field `errorText`.
    #[must_use]
    pub fn error_text(&self) -> Option<&str> {
        self.error_text.as_deref()
    }

    /// Returns wire field `results`.
    #[must_use]
    pub fn results(&self) -> &[CreateEvaluationAccountResponse] {
        &self.results
    }

    /// Starts a builder for [`CreateEvaluationAccountsResponse`].
    pub fn builder() -> CreateEvaluationAccountsResponseBuilder {
        CreateEvaluationAccountsResponseBuilder::default()
    }
}

/// Builder for [`CreateEvaluationAccountsResponse`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct CreateEvaluationAccountsResponseBuilder {
    error_text: Option<String>,
    results: Option<Vec<CreateEvaluationAccountResponse>>,
}

impl CreateEvaluationAccountsResponseBuilder {
    /// Sets wire field `errorText`.
    pub fn error_text(mut self, value: impl Into<String>) -> Self {
        self.error_text = Some(value.into());
        self
    }

    /// Sets wire field `results`.
    pub fn results(mut self, value: Vec<CreateEvaluationAccountResponse>) -> Self {
        self.results = Some(value);
        self
    }

    /// Validates required fields and builds [`CreateEvaluationAccountsResponse`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(
        self,
    ) -> Result<CreateEvaluationAccountsResponse, crate::api::current::BuildError> {
        let results = self
            .results
            .ok_or(crate::api::current::BuildError::missing("results"))?;
        Ok(CreateEvaluationAccountsResponse {
            error_text: self.error_text,
            results,
        })
    }
}

/// Current wire model `CreateEvaluationUserResponse`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct CreateEvaluationUserResponse {
    #[serde(rename = "errorText", default, skip_serializing_if = "Option::is_none")]
    error_text: Option<String>,
    #[serde(rename = "userId", default, skip_serializing_if = "Option::is_none")]
    user_id: Option<crate::UserId>,
}

impl CreateEvaluationUserResponse {
    /// Returns wire field `errorText`.
    #[must_use]
    pub fn error_text(&self) -> Option<&str> {
        self.error_text.as_deref()
    }

    /// Returns wire field `userId`.
    #[must_use]
    pub fn user_id(&self) -> Option<&crate::UserId> {
        self.user_id.as_ref()
    }

    /// Starts a builder for [`CreateEvaluationUserResponse`].
    pub fn builder() -> CreateEvaluationUserResponseBuilder {
        CreateEvaluationUserResponseBuilder::default()
    }
}

/// Builder for [`CreateEvaluationUserResponse`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct CreateEvaluationUserResponseBuilder {
    error_text: Option<String>,
    user_id: Option<crate::UserId>,
}

impl CreateEvaluationUserResponseBuilder {
    /// Sets wire field `errorText`.
    pub fn error_text(mut self, value: impl Into<String>) -> Self {
        self.error_text = Some(value.into());
        self
    }

    /// Sets wire field `userId`.
    pub fn user_id(mut self, value: crate::UserId) -> Self {
        self.user_id = Some(value);
        self
    }

    /// Validates required fields and builds [`CreateEvaluationUserResponse`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<CreateEvaluationUserResponse, crate::api::current::BuildError> {
        Ok(CreateEvaluationUserResponse {
            error_text: self.error_text,
            user_id: self.user_id,
        })
    }
}

/// Current wire model `CreateEvaluationUsers`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct CreateEvaluationUsers {
    #[serde(rename = "users")]
    users: Vec<EvaluationUser>,
}

impl CreateEvaluationUsers {
    /// Returns wire field `users`.
    #[must_use]
    pub fn users(&self) -> &[EvaluationUser] {
        &self.users
    }

    /// Starts a builder for [`CreateEvaluationUsers`].
    pub fn builder() -> CreateEvaluationUsersBuilder {
        CreateEvaluationUsersBuilder::default()
    }
}

/// Builder for [`CreateEvaluationUsers`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct CreateEvaluationUsersBuilder {
    users: Option<Vec<EvaluationUser>>,
}

impl CreateEvaluationUsersBuilder {
    /// Sets wire field `users`.
    pub fn users(mut self, value: Vec<EvaluationUser>) -> Self {
        self.users = Some(value);
        self
    }

    /// Validates required fields and builds [`CreateEvaluationUsers`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<CreateEvaluationUsers, crate::api::current::BuildError> {
        let users = self
            .users
            .ok_or(crate::api::current::BuildError::missing("users"))?;
        if users.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "users",
                "must not be empty",
            ));
        }
        Ok(CreateEvaluationUsers { users })
    }
}

impl crate::api::current::support::CurrentRequest for CreateEvaluationUsers {
    fn validate_current(&self) -> Result<(), crate::Error> {
        if self.users.is_empty() {
            return Err(crate::Error::InvalidRequest {
                field: "users",
                reason: "must not be empty",
            });
        }
        Ok(())
    }
}

/// Current wire model `CreateEvaluationUsersResponse`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct CreateEvaluationUsersResponse {
    #[serde(rename = "errorText", default, skip_serializing_if = "Option::is_none")]
    error_text: Option<String>,
    #[serde(rename = "results")]
    results: Vec<CreateEvaluationUserResponse>,
}

impl CreateEvaluationUsersResponse {
    /// Returns wire field `errorText`.
    #[must_use]
    pub fn error_text(&self) -> Option<&str> {
        self.error_text.as_deref()
    }

    /// Returns wire field `results`.
    #[must_use]
    pub fn results(&self) -> &[CreateEvaluationUserResponse] {
        &self.results
    }

    /// Starts a builder for [`CreateEvaluationUsersResponse`].
    pub fn builder() -> CreateEvaluationUsersResponseBuilder {
        CreateEvaluationUsersResponseBuilder::default()
    }
}

/// Builder for [`CreateEvaluationUsersResponse`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct CreateEvaluationUsersResponseBuilder {
    error_text: Option<String>,
    results: Option<Vec<CreateEvaluationUserResponse>>,
}

impl CreateEvaluationUsersResponseBuilder {
    /// Sets wire field `errorText`.
    pub fn error_text(mut self, value: impl Into<String>) -> Self {
        self.error_text = Some(value.into());
        self
    }

    /// Sets wire field `results`.
    pub fn results(mut self, value: Vec<CreateEvaluationUserResponse>) -> Self {
        self.results = Some(value);
        self
    }

    /// Validates required fields and builds [`CreateEvaluationUsersResponse`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<CreateEvaluationUsersResponse, crate::api::current::BuildError> {
        let results = self
            .results
            .ok_or(crate::api::current::BuildError::missing("results"))?;
        Ok(CreateEvaluationUsersResponse {
            error_text: self.error_text,
            results,
        })
    }
}

/// Current wire model `CreatePartnerSubAccountRequest`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct CreatePartnerSubAccountRequest {
    #[serde(rename = "ctaUserId")]
    cta_user_id: crate::UserId,
    #[serde(rename = "riskCategoryId")]
    risk_category_id: super::ids::RiskCategoryId,
    #[serde(
        rename = "autoLiqProfileId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    auto_liq_profile_id: Option<super::ids::AutoLiqProfileId>,
    #[serde(
        rename = "traderReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    trader_reason: Option<String>,
    #[serde(rename = "marginType")]
    margin_type: CreatePartnerSubAccountRequestMarginType,
    #[serde(rename = "transferAmount")]
    #[serde(with = "crate::decimal")]
    transfer_amount: crate::Decimal,
    #[serde(rename = "authorizedIndividual")]
    authorized_individual: bool,
    #[serde(rename = "firstName")]
    first_name: String,
    #[serde(rename = "lastName")]
    last_name: String,
    #[serde(rename = "country")]
    country: String,
    #[serde(rename = "state")]
    state: String,
    #[serde(rename = "streetAddress1")]
    street_address1: String,
    #[serde(
        rename = "streetAddress2",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    street_address2: Option<String>,
    #[serde(rename = "city")]
    city: String,
    #[serde(rename = "zipCode")]
    zip_code: String,
    #[serde(rename = "phone")]
    phone: String,
    #[serde(rename = "citizenship")]
    citizenship: String,
    #[serde(
        rename = "taxIdentifier",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    tax_identifier: Option<crate::api::current::SecretValue>,
    #[serde(
        rename = "nationalId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    national_id: Option<crate::api::current::SecretValue>,
    #[serde(rename = "birthDate")]
    birth_date: TradeDate,
    #[serde(
        rename = "pOAFormDocId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    p_oa_form_doc_id: Option<super::ids::DocumentId>,
    #[serde(
        rename = "governmentDocId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    government_doc_id: Option<super::ids::DocumentId>,
    #[serde(
        rename = "addressDocId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    address_doc_id: Option<super::ids::DocumentId>,
}

impl CreatePartnerSubAccountRequest {
    /// Returns wire field `ctaUserId`.
    #[must_use]
    pub fn cta_user_id(&self) -> &crate::UserId {
        &self.cta_user_id
    }

    /// Returns wire field `riskCategoryId`.
    #[must_use]
    pub fn risk_category_id(&self) -> &super::ids::RiskCategoryId {
        &self.risk_category_id
    }

    /// Returns wire field `autoLiqProfileId`.
    #[must_use]
    pub fn auto_liq_profile_id(&self) -> Option<&super::ids::AutoLiqProfileId> {
        self.auto_liq_profile_id.as_ref()
    }

    /// Returns wire field `traderReason`.
    #[must_use]
    pub fn trader_reason(&self) -> Option<&str> {
        self.trader_reason.as_deref()
    }

    /// Returns wire field `marginType`.
    #[must_use]
    pub fn margin_type(&self) -> &CreatePartnerSubAccountRequestMarginType {
        &self.margin_type
    }

    /// Returns wire field `transferAmount`.
    #[must_use]
    pub fn transfer_amount(&self) -> &crate::Decimal {
        &self.transfer_amount
    }

    /// Returns wire field `authorizedIndividual`.
    #[must_use]
    pub fn authorized_individual(&self) -> &bool {
        &self.authorized_individual
    }

    /// Returns wire field `firstName`.
    #[must_use]
    pub fn first_name(&self) -> &str {
        &self.first_name
    }

    /// Returns wire field `lastName`.
    #[must_use]
    pub fn last_name(&self) -> &str {
        &self.last_name
    }

    /// Returns wire field `country`.
    #[must_use]
    pub fn country(&self) -> &str {
        &self.country
    }

    /// Returns wire field `state`.
    #[must_use]
    pub fn state(&self) -> &str {
        &self.state
    }

    /// Returns wire field `streetAddress1`.
    #[must_use]
    pub fn street_address1(&self) -> &str {
        &self.street_address1
    }

    /// Returns wire field `streetAddress2`.
    #[must_use]
    pub fn street_address2(&self) -> Option<&str> {
        self.street_address2.as_deref()
    }

    /// Returns wire field `city`.
    #[must_use]
    pub fn city(&self) -> &str {
        &self.city
    }

    /// Returns wire field `zipCode`.
    #[must_use]
    pub fn zip_code(&self) -> &str {
        &self.zip_code
    }

    /// Returns wire field `phone`.
    #[must_use]
    pub fn phone(&self) -> &str {
        &self.phone
    }

    /// Returns wire field `citizenship`.
    #[must_use]
    pub fn citizenship(&self) -> &str {
        &self.citizenship
    }

    /// Reports whether secret field `taxIdentifier` is present.
    #[must_use]
    pub const fn has_tax_identifier(&self) -> bool {
        self.tax_identifier.is_some()
    }

    pub(crate) fn tax_identifier_secret(&self) -> Option<&crate::api::current::SecretValue> {
        self.tax_identifier.as_ref()
    }

    /// Reports whether secret field `nationalId` is present.
    #[must_use]
    pub const fn has_national_id(&self) -> bool {
        self.national_id.is_some()
    }

    pub(crate) fn national_id_secret(&self) -> Option<&crate::api::current::SecretValue> {
        self.national_id.as_ref()
    }

    /// Returns wire field `birthDate`.
    #[must_use]
    pub fn birth_date(&self) -> &TradeDate {
        &self.birth_date
    }

    /// Returns wire field `pOAFormDocId`.
    #[must_use]
    pub fn p_oa_form_doc_id(&self) -> Option<&super::ids::DocumentId> {
        self.p_oa_form_doc_id.as_ref()
    }

    /// Returns wire field `governmentDocId`.
    #[must_use]
    pub fn government_doc_id(&self) -> Option<&super::ids::DocumentId> {
        self.government_doc_id.as_ref()
    }

    /// Returns wire field `addressDocId`.
    #[must_use]
    pub fn address_doc_id(&self) -> Option<&super::ids::DocumentId> {
        self.address_doc_id.as_ref()
    }

    /// Starts a builder for [`CreatePartnerSubAccountRequest`].
    pub fn builder() -> CreatePartnerSubAccountRequestBuilder {
        CreatePartnerSubAccountRequestBuilder::default()
    }
}

/// Builder for [`CreatePartnerSubAccountRequest`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct CreatePartnerSubAccountRequestBuilder {
    cta_user_id: Option<crate::UserId>,
    risk_category_id: Option<super::ids::RiskCategoryId>,
    auto_liq_profile_id: Option<super::ids::AutoLiqProfileId>,
    trader_reason: Option<String>,
    margin_type: Option<CreatePartnerSubAccountRequestMarginType>,
    transfer_amount: Option<crate::Decimal>,
    authorized_individual: Option<bool>,
    first_name: Option<String>,
    last_name: Option<String>,
    country: Option<String>,
    state: Option<String>,
    street_address1: Option<String>,
    street_address2: Option<String>,
    city: Option<String>,
    zip_code: Option<String>,
    phone: Option<String>,
    citizenship: Option<String>,
    tax_identifier: Option<crate::api::current::SecretValue>,
    national_id: Option<crate::api::current::SecretValue>,
    birth_date: Option<TradeDate>,
    p_oa_form_doc_id: Option<super::ids::DocumentId>,
    government_doc_id: Option<super::ids::DocumentId>,
    address_doc_id: Option<super::ids::DocumentId>,
}

impl CreatePartnerSubAccountRequestBuilder {
    /// Sets wire field `ctaUserId`.
    pub fn cta_user_id(mut self, value: crate::UserId) -> Self {
        self.cta_user_id = Some(value);
        self
    }

    /// Sets wire field `riskCategoryId`.
    pub fn risk_category_id(mut self, value: super::ids::RiskCategoryId) -> Self {
        self.risk_category_id = Some(value);
        self
    }

    /// Sets wire field `autoLiqProfileId`.
    pub fn auto_liq_profile_id(mut self, value: super::ids::AutoLiqProfileId) -> Self {
        self.auto_liq_profile_id = Some(value);
        self
    }

    /// Sets wire field `traderReason`.
    pub fn trader_reason(mut self, value: impl Into<String>) -> Self {
        self.trader_reason = Some(value.into());
        self
    }

    /// Sets wire field `marginType`.
    pub fn margin_type(mut self, value: CreatePartnerSubAccountRequestMarginType) -> Self {
        self.margin_type = Some(value);
        self
    }

    /// Sets wire field `transferAmount`.
    pub fn transfer_amount(mut self, value: crate::Decimal) -> Self {
        self.transfer_amount = Some(value);
        self
    }

    /// Sets wire field `authorizedIndividual`.
    pub fn authorized_individual(mut self, value: bool) -> Self {
        self.authorized_individual = Some(value);
        self
    }

    /// Sets wire field `firstName`.
    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    /// Sets wire field `lastName`.
    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
        self
    }

    /// Sets wire field `country`.
    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    /// Sets wire field `state`.
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    /// Sets wire field `streetAddress1`.
    pub fn street_address1(mut self, value: impl Into<String>) -> Self {
        self.street_address1 = Some(value.into());
        self
    }

    /// Sets wire field `streetAddress2`.
    pub fn street_address2(mut self, value: impl Into<String>) -> Self {
        self.street_address2 = Some(value.into());
        self
    }

    /// Sets wire field `city`.
    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    /// Sets wire field `zipCode`.
    pub fn zip_code(mut self, value: impl Into<String>) -> Self {
        self.zip_code = Some(value.into());
        self
    }

    /// Sets wire field `phone`.
    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    /// Sets wire field `citizenship`.
    pub fn citizenship(mut self, value: impl Into<String>) -> Self {
        self.citizenship = Some(value.into());
        self
    }

    /// Sets wire field `taxIdentifier`.
    pub fn tax_identifier(mut self, value: crate::api::current::SecretValue) -> Self {
        self.tax_identifier = Some(value);
        self
    }

    /// Sets wire field `nationalId`.
    pub fn national_id(mut self, value: crate::api::current::SecretValue) -> Self {
        self.national_id = Some(value);
        self
    }

    /// Sets wire field `birthDate`.
    pub fn birth_date(mut self, value: TradeDate) -> Self {
        self.birth_date = Some(value);
        self
    }

    /// Sets wire field `pOAFormDocId`.
    pub fn p_oa_form_doc_id(mut self, value: super::ids::DocumentId) -> Self {
        self.p_oa_form_doc_id = Some(value);
        self
    }

    /// Sets wire field `governmentDocId`.
    pub fn government_doc_id(mut self, value: super::ids::DocumentId) -> Self {
        self.government_doc_id = Some(value);
        self
    }

    /// Sets wire field `addressDocId`.
    pub fn address_doc_id(mut self, value: super::ids::DocumentId) -> Self {
        self.address_doc_id = Some(value);
        self
    }

    /// Validates required fields and builds [`CreatePartnerSubAccountRequest`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<CreatePartnerSubAccountRequest, crate::api::current::BuildError> {
        let cta_user_id = self
            .cta_user_id
            .ok_or(crate::api::current::BuildError::missing("ctaUserId"))?;
        let risk_category_id = self
            .risk_category_id
            .ok_or(crate::api::current::BuildError::missing("riskCategoryId"))?;
        let margin_type = self
            .margin_type
            .ok_or(crate::api::current::BuildError::missing("marginType"))?;
        let transfer_amount = self
            .transfer_amount
            .ok_or(crate::api::current::BuildError::missing("transferAmount"))?;
        let authorized_individual =
            self.authorized_individual
                .ok_or(crate::api::current::BuildError::missing(
                    "authorizedIndividual",
                ))?;
        let first_name = self
            .first_name
            .ok_or(crate::api::current::BuildError::missing("firstName"))?;
        if first_name.is_empty() || first_name.trim() != first_name {
            return Err(crate::api::current::BuildError::invalid(
                "firstName",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        let last_name = self
            .last_name
            .ok_or(crate::api::current::BuildError::missing("lastName"))?;
        if last_name.is_empty() || last_name.trim() != last_name {
            return Err(crate::api::current::BuildError::invalid(
                "lastName",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        let country = self
            .country
            .ok_or(crate::api::current::BuildError::missing("country"))?;
        if country.is_empty() || country.trim() != country {
            return Err(crate::api::current::BuildError::invalid(
                "country",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        let state = self
            .state
            .ok_or(crate::api::current::BuildError::missing("state"))?;
        if state.is_empty() || state.trim() != state {
            return Err(crate::api::current::BuildError::invalid(
                "state",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        let street_address1 = self
            .street_address1
            .ok_or(crate::api::current::BuildError::missing("streetAddress1"))?;
        if street_address1.is_empty() || street_address1.trim() != street_address1 {
            return Err(crate::api::current::BuildError::invalid(
                "streetAddress1",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        let city = self
            .city
            .ok_or(crate::api::current::BuildError::missing("city"))?;
        if city.is_empty() || city.trim() != city {
            return Err(crate::api::current::BuildError::invalid(
                "city",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        let zip_code = self
            .zip_code
            .ok_or(crate::api::current::BuildError::missing("zipCode"))?;
        if zip_code.is_empty() || zip_code.trim() != zip_code {
            return Err(crate::api::current::BuildError::invalid(
                "zipCode",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        let phone = self
            .phone
            .ok_or(crate::api::current::BuildError::missing("phone"))?;
        if phone.is_empty() || phone.trim() != phone {
            return Err(crate::api::current::BuildError::invalid(
                "phone",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        let citizenship = self
            .citizenship
            .ok_or(crate::api::current::BuildError::missing("citizenship"))?;
        if citizenship.is_empty() || citizenship.trim() != citizenship {
            return Err(crate::api::current::BuildError::invalid(
                "citizenship",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        let birth_date = self
            .birth_date
            .ok_or(crate::api::current::BuildError::missing("birthDate"))?;
        Ok(CreatePartnerSubAccountRequest {
            cta_user_id,
            risk_category_id,
            auto_liq_profile_id: self.auto_liq_profile_id,
            trader_reason: self.trader_reason,
            margin_type,
            transfer_amount,
            authorized_individual,
            first_name,
            last_name,
            country,
            state,
            street_address1,
            street_address2: self.street_address2,
            city,
            zip_code,
            phone,
            citizenship,
            tax_identifier: self.tax_identifier,
            national_id: self.national_id,
            birth_date,
            p_oa_form_doc_id: self.p_oa_form_doc_id,
            government_doc_id: self.government_doc_id,
            address_doc_id: self.address_doc_id,
        })
    }
}

impl crate::api::current::support::CurrentRequest for CreatePartnerSubAccountRequest {
    fn validate_current(&self) -> Result<(), crate::Error> {
        if self.citizenship.is_empty() || self.citizenship.trim() != self.citizenship {
            return Err(crate::Error::InvalidRequest {
                field: "citizenship",
                reason: "must be non-empty and have no surrounding whitespace",
            });
        }
        if self.city.is_empty() || self.city.trim() != self.city {
            return Err(crate::Error::InvalidRequest {
                field: "city",
                reason: "must be non-empty and have no surrounding whitespace",
            });
        }
        if self.country.is_empty() || self.country.trim() != self.country {
            return Err(crate::Error::InvalidRequest {
                field: "country",
                reason: "must be non-empty and have no surrounding whitespace",
            });
        }
        if self.first_name.is_empty() || self.first_name.trim() != self.first_name {
            return Err(crate::Error::InvalidRequest {
                field: "firstName",
                reason: "must be non-empty and have no surrounding whitespace",
            });
        }
        if self.last_name.is_empty() || self.last_name.trim() != self.last_name {
            return Err(crate::Error::InvalidRequest {
                field: "lastName",
                reason: "must be non-empty and have no surrounding whitespace",
            });
        }
        if self.phone.is_empty() || self.phone.trim() != self.phone {
            return Err(crate::Error::InvalidRequest {
                field: "phone",
                reason: "must be non-empty and have no surrounding whitespace",
            });
        }
        if self.state.is_empty() || self.state.trim() != self.state {
            return Err(crate::Error::InvalidRequest {
                field: "state",
                reason: "must be non-empty and have no surrounding whitespace",
            });
        }
        if self.street_address1.is_empty() || self.street_address1.trim() != self.street_address1 {
            return Err(crate::Error::InvalidRequest {
                field: "streetAddress1",
                reason: "must be non-empty and have no surrounding whitespace",
            });
        }
        if self.zip_code.is_empty() || self.zip_code.trim() != self.zip_code {
            return Err(crate::Error::InvalidRequest {
                field: "zipCode",
                reason: "must be non-empty and have no surrounding whitespace",
            });
        }
        Ok(())
    }
}

/// Current provider values for `CreatePartnerSubAccountRequestMarginType`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum CreatePartnerSubAccountRequestMarginType {
    /// Provider value `Hedger`.
    Hedger,
    /// Provider value `Speculator`.
    Speculator,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl CreatePartnerSubAccountRequestMarginType {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Hedger => "Hedger",
            Self::Speculator => "Speculator",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for CreatePartnerSubAccountRequestMarginType {
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

impl<'de> serde::Deserialize<'de> for CreatePartnerSubAccountRequestMarginType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "Hedger" => Self::Hedger,
            "Speculator" => Self::Speculator,
            _ => Self::Unknown(value),
        })
    }
}

/// Current wire model `CreatePartnerSubAccountRequestResponse`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct CreatePartnerSubAccountRequestResponse {
    #[serde(rename = "errorText", default, skip_serializing_if = "Option::is_none")]
    error_text: Option<String>,
    #[serde(rename = "requestId", default, skip_serializing_if = "Option::is_none")]
    request_id: Option<super::ids::SubAccountRequestId>,
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    status: Option<CreatePartnerSubAccountRequestResponseStatus>,
}

impl CreatePartnerSubAccountRequestResponse {
    /// Returns wire field `errorText`.
    #[must_use]
    pub fn error_text(&self) -> Option<&str> {
        self.error_text.as_deref()
    }

    /// Returns wire field `requestId`.
    #[must_use]
    pub fn request_id(&self) -> Option<&super::ids::SubAccountRequestId> {
        self.request_id.as_ref()
    }

    /// Returns wire field `status`.
    #[must_use]
    pub fn status(&self) -> Option<&CreatePartnerSubAccountRequestResponseStatus> {
        self.status.as_ref()
    }

    /// Starts a builder for [`CreatePartnerSubAccountRequestResponse`].
    pub fn builder() -> CreatePartnerSubAccountRequestResponseBuilder {
        CreatePartnerSubAccountRequestResponseBuilder::default()
    }
}

/// Builder for [`CreatePartnerSubAccountRequestResponse`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct CreatePartnerSubAccountRequestResponseBuilder {
    error_text: Option<String>,
    request_id: Option<super::ids::SubAccountRequestId>,
    status: Option<CreatePartnerSubAccountRequestResponseStatus>,
}

impl CreatePartnerSubAccountRequestResponseBuilder {
    /// Sets wire field `errorText`.
    pub fn error_text(mut self, value: impl Into<String>) -> Self {
        self.error_text = Some(value.into());
        self
    }

    /// Sets wire field `requestId`.
    pub fn request_id(mut self, value: super::ids::SubAccountRequestId) -> Self {
        self.request_id = Some(value);
        self
    }

    /// Sets wire field `status`.
    pub fn status(mut self, value: CreatePartnerSubAccountRequestResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Validates required fields and builds [`CreatePartnerSubAccountRequestResponse`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(
        self,
    ) -> Result<CreatePartnerSubAccountRequestResponse, crate::api::current::BuildError> {
        Ok(CreatePartnerSubAccountRequestResponse {
            error_text: self.error_text,
            request_id: self.request_id,
            status: self.status,
        })
    }
}

/// Current provider values for `CreatePartnerSubAccountRequestResponseStatus`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum CreatePartnerSubAccountRequestResponseStatus {
    /// Provider value `Approved`.
    Approved,
    /// Provider value `Denied`.
    Denied,
    /// Provider value `InAMLReview`.
    InAmlReview,
    /// Provider value `InReview`.
    InReview,
    /// Provider value `Pending`.
    Pending,
    /// Provider value `Preapproved`.
    Preapproved,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl CreatePartnerSubAccountRequestResponseStatus {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Approved => "Approved",
            Self::Denied => "Denied",
            Self::InAmlReview => "InAMLReview",
            Self::InReview => "InReview",
            Self::Pending => "Pending",
            Self::Preapproved => "Preapproved",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for CreatePartnerSubAccountRequestResponseStatus {
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

impl<'de> serde::Deserialize<'de> for CreatePartnerSubAccountRequestResponseStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "Approved" => Self::Approved,
            "Denied" => Self::Denied,
            "InAMLReview" => Self::InAmlReview,
            "InReview" => Self::InReview,
            "Pending" => Self::Pending,
            "Preapproved" => Self::Preapproved,
            _ => Self::Unknown(value),
        })
    }
}

/// Current wire model `CreateTradingPermission`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct CreateTradingPermission {
    #[serde(rename = "accountId")]
    account_id: crate::AccountId,
    #[serde(rename = "userId")]
    user_id: crate::UserId,
}

impl CreateTradingPermission {
    /// Returns wire field `accountId`.
    #[must_use]
    pub fn account_id(&self) -> &crate::AccountId {
        &self.account_id
    }

    /// Returns wire field `userId`.
    #[must_use]
    pub fn user_id(&self) -> &crate::UserId {
        &self.user_id
    }

    /// Starts a builder for [`CreateTradingPermission`].
    pub fn builder() -> CreateTradingPermissionBuilder {
        CreateTradingPermissionBuilder::default()
    }
}

/// Builder for [`CreateTradingPermission`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct CreateTradingPermissionBuilder {
    account_id: Option<crate::AccountId>,
    user_id: Option<crate::UserId>,
}

impl CreateTradingPermissionBuilder {
    /// Sets wire field `accountId`.
    pub fn account_id(mut self, value: crate::AccountId) -> Self {
        self.account_id = Some(value);
        self
    }

    /// Sets wire field `userId`.
    pub fn user_id(mut self, value: crate::UserId) -> Self {
        self.user_id = Some(value);
        self
    }

    /// Validates required fields and builds [`CreateTradingPermission`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<CreateTradingPermission, crate::api::current::BuildError> {
        let account_id = self
            .account_id
            .ok_or(crate::api::current::BuildError::missing("accountId"))?;
        let user_id = self
            .user_id
            .ok_or(crate::api::current::BuildError::missing("userId"))?;
        Ok(CreateTradingPermission {
            account_id,
            user_id,
        })
    }
}

impl crate::api::current::support::CurrentRequest for CreateTradingPermission {
    fn validate_current(&self) -> Result<(), crate::Error> {
        Ok(())
    }
}

/// Current wire model `Currency`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct Currency {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    id: Option<super::ids::CurrencyId>,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    symbol: Option<crate::Symbol>,
}

impl Currency {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> Option<&super::ids::CurrencyId> {
        self.id.as_ref()
    }

    /// Returns wire field `name`.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns wire field `symbol`.
    #[must_use]
    pub fn symbol(&self) -> Option<&crate::Symbol> {
        self.symbol.as_ref()
    }

    /// Starts a builder for [`Currency`].
    pub fn builder() -> CurrencyBuilder {
        CurrencyBuilder::default()
    }
}

/// Builder for [`Currency`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct CurrencyBuilder {
    id: Option<super::ids::CurrencyId>,
    name: Option<String>,
    symbol: Option<crate::Symbol>,
}

impl CurrencyBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::CurrencyId) -> Self {
        self.id = Some(value);
        self
    }

    /// Sets wire field `name`.
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Sets wire field `symbol`.
    pub fn symbol(mut self, value: crate::Symbol) -> Self {
        self.symbol = Some(value);
        self
    }

    /// Validates required fields and builds [`Currency`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<Currency, crate::api::current::BuildError> {
        let name = self
            .name
            .ok_or(crate::api::current::BuildError::missing("name"))?;
        Ok(Currency {
            id: self.id,
            name,
            symbol: self.symbol,
        })
    }
}

/// Current wire model `EntitlementSubscriptionResponse`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct EntitlementSubscriptionResponse {
    #[serde(rename = "errorText", default, skip_serializing_if = "Option::is_none")]
    error_text: Option<String>,
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    error_code: Option<EntitlementSubscriptionResponseErrorCode>,
    #[serde(
        rename = "entitlementSubscription",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    entitlement_subscription: Option<UserPlugin>,
}

impl EntitlementSubscriptionResponse {
    /// Returns wire field `errorText`.
    #[must_use]
    pub fn error_text(&self) -> Option<&str> {
        self.error_text.as_deref()
    }

    /// Returns wire field `errorCode`.
    #[must_use]
    pub fn error_code(&self) -> Option<&EntitlementSubscriptionResponseErrorCode> {
        self.error_code.as_ref()
    }

    /// Returns wire field `entitlementSubscription`.
    #[must_use]
    pub fn entitlement_subscription(&self) -> Option<&UserPlugin> {
        self.entitlement_subscription.as_ref()
    }

    /// Starts a builder for [`EntitlementSubscriptionResponse`].
    pub fn builder() -> EntitlementSubscriptionResponseBuilder {
        EntitlementSubscriptionResponseBuilder::default()
    }
}

/// Builder for [`EntitlementSubscriptionResponse`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct EntitlementSubscriptionResponseBuilder {
    error_text: Option<String>,
    error_code: Option<EntitlementSubscriptionResponseErrorCode>,
    entitlement_subscription: Option<UserPlugin>,
}

impl EntitlementSubscriptionResponseBuilder {
    /// Sets wire field `errorText`.
    pub fn error_text(mut self, value: impl Into<String>) -> Self {
        self.error_text = Some(value.into());
        self
    }

    /// Sets wire field `errorCode`.
    pub fn error_code(mut self, value: EntitlementSubscriptionResponseErrorCode) -> Self {
        self.error_code = Some(value);
        self
    }

    /// Sets wire field `entitlementSubscription`.
    pub fn entitlement_subscription(mut self, value: UserPlugin) -> Self {
        self.entitlement_subscription = Some(value);
        self
    }

    /// Validates required fields and builds [`EntitlementSubscriptionResponse`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<EntitlementSubscriptionResponse, crate::api::current::BuildError> {
        Ok(EntitlementSubscriptionResponse {
            error_text: self.error_text,
            error_code: self.error_code,
            entitlement_subscription: self.entitlement_subscription,
        })
    }
}

/// Current provider values for `EntitlementSubscriptionResponseErrorCode`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum EntitlementSubscriptionResponseErrorCode {
    /// Provider value `ConflictWithExisting`.
    ConflictWithExisting,
    /// Provider value `DowngradeNotAllowed`.
    DowngradeNotAllowed,
    /// Provider value `IncompatibleCMEMarketDataSubscriptionPlans`.
    IncompatibleCmeMarketDataSubscriptionPlans,
    /// Provider value `IncorrectPaymentMethod`.
    IncorrectPaymentMethod,
    /// Provider value `InsufficientFunds`.
    InsufficientFunds,
    /// Provider value `PaymentProviderError`.
    PaymentProviderError,
    /// Provider value `PlanDiscontinued`.
    PlanDiscontinued,
    /// Provider value `SingleTrialOnly`.
    SingleTrialOnly,
    /// Provider value `Success`.
    Success,
    /// Provider value `UnknownError`.
    UnknownError,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl EntitlementSubscriptionResponseErrorCode {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::ConflictWithExisting => "ConflictWithExisting",
            Self::DowngradeNotAllowed => "DowngradeNotAllowed",
            Self::IncompatibleCmeMarketDataSubscriptionPlans => {
                "IncompatibleCMEMarketDataSubscriptionPlans"
            }
            Self::IncorrectPaymentMethod => "IncorrectPaymentMethod",
            Self::InsufficientFunds => "InsufficientFunds",
            Self::PaymentProviderError => "PaymentProviderError",
            Self::PlanDiscontinued => "PlanDiscontinued",
            Self::SingleTrialOnly => "SingleTrialOnly",
            Self::Success => "Success",
            Self::UnknownError => "UnknownError",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for EntitlementSubscriptionResponseErrorCode {
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

impl<'de> serde::Deserialize<'de> for EntitlementSubscriptionResponseErrorCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "ConflictWithExisting" => Self::ConflictWithExisting,
            "DowngradeNotAllowed" => Self::DowngradeNotAllowed,
            "IncompatibleCMEMarketDataSubscriptionPlans" => {
                Self::IncompatibleCmeMarketDataSubscriptionPlans
            }
            "IncorrectPaymentMethod" => Self::IncorrectPaymentMethod,
            "InsufficientFunds" => Self::InsufficientFunds,
            "PaymentProviderError" => Self::PaymentProviderError,
            "PlanDiscontinued" => Self::PlanDiscontinued,
            "SingleTrialOnly" => Self::SingleTrialOnly,
            "Success" => Self::Success,
            "UnknownError" => Self::UnknownError,
            _ => Self::Unknown(value),
        })
    }
}

/// Current wire model `EvaluationAccount`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct EvaluationAccount {
    #[serde(rename = "userId")]
    user_id: crate::UserId,
    #[serde(rename = "templateAccountId")]
    template_account_id: crate::AccountId,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "initialBalance")]
    #[serde(with = "crate::decimal")]
    initial_balance: crate::Decimal,
    #[serde(
        rename = "preTradeRisk",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pre_trade_risk: Option<Vec<PreTradeRisk>>,
    #[serde(
        rename = "postTradeRisk",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    post_trade_risk: Option<PostTradeRisk>,
}

impl EvaluationAccount {
    /// Returns wire field `userId`.
    #[must_use]
    pub fn user_id(&self) -> &crate::UserId {
        &self.user_id
    }

    /// Returns wire field `templateAccountId`.
    #[must_use]
    pub fn template_account_id(&self) -> &crate::AccountId {
        &self.template_account_id
    }

    /// Returns wire field `name`.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns wire field `initialBalance`.
    #[must_use]
    pub fn initial_balance(&self) -> &crate::Decimal {
        &self.initial_balance
    }

    /// Returns wire field `preTradeRisk`.
    #[must_use]
    pub fn pre_trade_risk(&self) -> Option<&[PreTradeRisk]> {
        self.pre_trade_risk.as_deref()
    }

    /// Returns wire field `postTradeRisk`.
    #[must_use]
    pub fn post_trade_risk(&self) -> Option<&PostTradeRisk> {
        self.post_trade_risk.as_ref()
    }

    /// Starts a builder for [`EvaluationAccount`].
    pub fn builder() -> EvaluationAccountBuilder {
        EvaluationAccountBuilder::default()
    }
}

/// Builder for [`EvaluationAccount`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct EvaluationAccountBuilder {
    user_id: Option<crate::UserId>,
    template_account_id: Option<crate::AccountId>,
    name: Option<String>,
    initial_balance: Option<crate::Decimal>,
    pre_trade_risk: Option<Vec<PreTradeRisk>>,
    post_trade_risk: Option<PostTradeRisk>,
}

impl EvaluationAccountBuilder {
    /// Sets wire field `userId`.
    pub fn user_id(mut self, value: crate::UserId) -> Self {
        self.user_id = Some(value);
        self
    }

    /// Sets wire field `templateAccountId`.
    pub fn template_account_id(mut self, value: crate::AccountId) -> Self {
        self.template_account_id = Some(value);
        self
    }

    /// Sets wire field `name`.
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Sets wire field `initialBalance`.
    pub fn initial_balance(mut self, value: crate::Decimal) -> Self {
        self.initial_balance = Some(value);
        self
    }

    /// Sets wire field `preTradeRisk`.
    pub fn pre_trade_risk(mut self, value: Vec<PreTradeRisk>) -> Self {
        self.pre_trade_risk = Some(value);
        self
    }

    /// Sets wire field `postTradeRisk`.
    pub fn post_trade_risk(mut self, value: PostTradeRisk) -> Self {
        self.post_trade_risk = Some(value);
        self
    }

    /// Validates required fields and builds [`EvaluationAccount`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<EvaluationAccount, crate::api::current::BuildError> {
        let user_id = self
            .user_id
            .ok_or(crate::api::current::BuildError::missing("userId"))?;
        let template_account_id =
            self.template_account_id
                .ok_or(crate::api::current::BuildError::missing(
                    "templateAccountId",
                ))?;
        let name = self
            .name
            .ok_or(crate::api::current::BuildError::missing("name"))?;
        let initial_balance = self
            .initial_balance
            .ok_or(crate::api::current::BuildError::missing("initialBalance"))?;
        Ok(EvaluationAccount {
            user_id,
            template_account_id,
            name,
            initial_balance,
            pre_trade_risk: self.pre_trade_risk,
            post_trade_risk: self.post_trade_risk,
        })
    }
}

/// Current wire model `EvaluationUser`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct EvaluationUser {
    #[serde(rename = "name")]
    name: crate::api::current::SecretValue,
    #[serde(rename = "email")]
    email: String,
    #[serde(rename = "password")]
    password: crate::api::current::SecretValue,
    #[serde(rename = "firstName")]
    first_name: String,
    #[serde(rename = "lastName")]
    last_name: String,
    #[serde(
        rename = "tradovateSubscriptionPlanId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    tradovate_subscription_plan_id: Option<super::ids::TradovateSubscriptionPlanId>,
    #[serde(
        rename = "entitlementIds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    entitlement_ids: Option<Vec<super::ids::EntitlementId>>,
}

impl EvaluationUser {
    /// Reports whether secret field `name` is present.
    #[must_use]
    pub const fn has_name(&self) -> bool {
        true
    }

    pub(crate) fn name_secret(&self) -> &crate::api::current::SecretValue {
        &self.name
    }

    /// Returns wire field `email`.
    #[must_use]
    pub fn email(&self) -> &str {
        &self.email
    }

    /// Reports whether secret field `password` is present.
    #[must_use]
    pub const fn has_password(&self) -> bool {
        true
    }

    /// Returns wire field `firstName`.
    #[must_use]
    pub fn first_name(&self) -> &str {
        &self.first_name
    }

    /// Returns wire field `lastName`.
    #[must_use]
    pub fn last_name(&self) -> &str {
        &self.last_name
    }

    /// Returns wire field `tradovateSubscriptionPlanId`.
    #[must_use]
    pub fn tradovate_subscription_plan_id(
        &self,
    ) -> Option<&super::ids::TradovateSubscriptionPlanId> {
        self.tradovate_subscription_plan_id.as_ref()
    }

    /// Returns wire field `entitlementIds`.
    #[must_use]
    pub fn entitlement_ids(&self) -> Option<&[super::ids::EntitlementId]> {
        self.entitlement_ids.as_deref()
    }

    /// Starts a builder for [`EvaluationUser`].
    pub fn builder() -> EvaluationUserBuilder {
        EvaluationUserBuilder::default()
    }
}

/// Builder for [`EvaluationUser`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct EvaluationUserBuilder {
    name: Option<crate::api::current::SecretValue>,
    email: Option<String>,
    password: Option<crate::api::current::SecretValue>,
    first_name: Option<String>,
    last_name: Option<String>,
    tradovate_subscription_plan_id: Option<super::ids::TradovateSubscriptionPlanId>,
    entitlement_ids: Option<Vec<super::ids::EntitlementId>>,
}

impl EvaluationUserBuilder {
    /// Sets wire field `name`.
    pub fn name(mut self, value: crate::api::current::SecretValue) -> Self {
        self.name = Some(value);
        self
    }

    /// Sets wire field `email`.
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    /// Sets wire field `password`.
    pub fn password(mut self, value: crate::api::current::SecretValue) -> Self {
        self.password = Some(value);
        self
    }

    /// Sets wire field `firstName`.
    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    /// Sets wire field `lastName`.
    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
        self
    }

    /// Sets wire field `tradovateSubscriptionPlanId`.
    pub fn tradovate_subscription_plan_id(
        mut self,
        value: super::ids::TradovateSubscriptionPlanId,
    ) -> Self {
        self.tradovate_subscription_plan_id = Some(value);
        self
    }

    /// Sets wire field `entitlementIds`.
    pub fn entitlement_ids(mut self, value: Vec<super::ids::EntitlementId>) -> Self {
        self.entitlement_ids = Some(value);
        self
    }

    /// Validates required fields and builds [`EvaluationUser`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<EvaluationUser, crate::api::current::BuildError> {
        let name = self
            .name
            .ok_or(crate::api::current::BuildError::missing("name"))?;
        let email = self
            .email
            .ok_or(crate::api::current::BuildError::missing("email"))?;
        let password = self
            .password
            .ok_or(crate::api::current::BuildError::missing("password"))?;
        let first_name = self
            .first_name
            .ok_or(crate::api::current::BuildError::missing("firstName"))?;
        let last_name = self
            .last_name
            .ok_or(crate::api::current::BuildError::missing("lastName"))?;
        Ok(EvaluationUser {
            name,
            email,
            password,
            first_name,
            last_name,
            tradovate_subscription_plan_id: self.tradovate_subscription_plan_id,
            entitlement_ids: self.entitlement_ids,
        })
    }
}

/// Current wire model `Exchange`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct Exchange {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    id: Option<super::ids::ExchangeId>,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "micCode", default, skip_serializing_if = "Option::is_none")]
    mic_code: Option<String>,
}

impl Exchange {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> Option<&super::ids::ExchangeId> {
        self.id.as_ref()
    }

    /// Returns wire field `name`.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns wire field `micCode`.
    #[must_use]
    pub fn mic_code(&self) -> Option<&str> {
        self.mic_code.as_deref()
    }

    /// Starts a builder for [`Exchange`].
    pub fn builder() -> ExchangeBuilder {
        ExchangeBuilder::default()
    }
}

/// Builder for [`Exchange`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ExchangeBuilder {
    id: Option<super::ids::ExchangeId>,
    name: Option<String>,
    mic_code: Option<String>,
}

impl ExchangeBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::ExchangeId) -> Self {
        self.id = Some(value);
        self
    }

    /// Sets wire field `name`.
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Sets wire field `micCode`.
    pub fn mic_code(mut self, value: impl Into<String>) -> Self {
        self.mic_code = Some(value.into());
        self
    }

    /// Validates required fields and builds [`Exchange`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<Exchange, crate::api::current::BuildError> {
        let name = self
            .name
            .ok_or(crate::api::current::BuildError::missing("name"))?;
        Ok(Exchange {
            id: self.id,
            name,
            mic_code: self.mic_code,
        })
    }
}

/// Current wire model `ExecutionReport`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ExecutionReport {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    id: Option<super::ids::ExecutionReportId>,
    #[serde(rename = "commandId")]
    command_id: crate::CommandId,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "accountId")]
    account_id: crate::AccountId,
    #[serde(rename = "contractId")]
    contract_id: crate::ContractId,
    #[serde(rename = "timestamp")]
    timestamp: jiff::Timestamp,
    #[serde(rename = "tradeDate", default, skip_serializing_if = "Option::is_none")]
    trade_date: Option<TradeDate>,
    #[serde(rename = "orderId")]
    order_id: crate::OrderId,
    #[serde(rename = "execType")]
    exec_type: ExecutionReportExecType,
    #[serde(rename = "execRefId", default, skip_serializing_if = "Option::is_none")]
    exec_ref_id: Option<String>,
    #[serde(rename = "ordStatus", default, skip_serializing_if = "Option::is_none")]
    ord_status: Option<ExecutionReportOrdStatus>,
    #[serde(rename = "action")]
    action: ExecutionReportAction,
    #[serde(rename = "cumQty", default, skip_serializing_if = "Option::is_none")]
    cum_qty: Option<i64>,
    #[serde(rename = "avgPx", default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "crate::decimal::option")]
    avg_px: Option<crate::Decimal>,
    #[serde(rename = "lastQty", default, skip_serializing_if = "Option::is_none")]
    last_qty: Option<i64>,
    #[serde(rename = "lastPx", default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "crate::decimal::option")]
    last_px: Option<crate::Decimal>,
    #[serde(
        rename = "rejectReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    reject_reason: Option<ExecutionReportRejectReason>,
    #[serde(rename = "text", default, skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(
        rename = "exchangeOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    exchange_order_id: Option<String>,
}

impl ExecutionReport {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> Option<&super::ids::ExecutionReportId> {
        self.id.as_ref()
    }

    /// Returns wire field `commandId`.
    #[must_use]
    pub fn command_id(&self) -> &crate::CommandId {
        &self.command_id
    }

    /// Returns wire field `name`.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

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

    /// Returns wire field `timestamp`.
    #[must_use]
    pub fn timestamp(&self) -> &jiff::Timestamp {
        &self.timestamp
    }

    /// Returns wire field `tradeDate`.
    #[must_use]
    pub fn trade_date(&self) -> Option<&TradeDate> {
        self.trade_date.as_ref()
    }

    /// Returns wire field `orderId`.
    #[must_use]
    pub fn order_id(&self) -> &crate::OrderId {
        &self.order_id
    }

    /// Returns wire field `execType`.
    #[must_use]
    pub fn exec_type(&self) -> &ExecutionReportExecType {
        &self.exec_type
    }

    /// Returns wire field `execRefId`.
    #[must_use]
    pub fn exec_ref_id(&self) -> Option<&str> {
        self.exec_ref_id.as_deref()
    }

    /// Returns wire field `ordStatus`.
    #[must_use]
    pub fn ord_status(&self) -> Option<&ExecutionReportOrdStatus> {
        self.ord_status.as_ref()
    }

    /// Returns wire field `action`.
    #[must_use]
    pub fn action(&self) -> &ExecutionReportAction {
        &self.action
    }

    /// Returns wire field `cumQty`.
    #[must_use]
    pub fn cum_qty(&self) -> Option<&i64> {
        self.cum_qty.as_ref()
    }

    /// Returns wire field `avgPx`.
    #[must_use]
    pub fn avg_px(&self) -> Option<&crate::Decimal> {
        self.avg_px.as_ref()
    }

    /// Returns wire field `lastQty`.
    #[must_use]
    pub fn last_qty(&self) -> Option<&i64> {
        self.last_qty.as_ref()
    }

    /// Returns wire field `lastPx`.
    #[must_use]
    pub fn last_px(&self) -> Option<&crate::Decimal> {
        self.last_px.as_ref()
    }

    /// Returns wire field `rejectReason`.
    #[must_use]
    pub fn reject_reason(&self) -> Option<&ExecutionReportRejectReason> {
        self.reject_reason.as_ref()
    }

    /// Returns wire field `text`.
    #[must_use]
    pub fn text(&self) -> Option<&str> {
        self.text.as_deref()
    }

    /// Returns wire field `exchangeOrderId`.
    #[must_use]
    pub fn exchange_order_id(&self) -> Option<&str> {
        self.exchange_order_id.as_deref()
    }

    /// Starts a builder for [`ExecutionReport`].
    pub fn builder() -> ExecutionReportBuilder {
        ExecutionReportBuilder::default()
    }
}

/// Builder for [`ExecutionReport`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ExecutionReportBuilder {
    id: Option<super::ids::ExecutionReportId>,
    command_id: Option<crate::CommandId>,
    name: Option<String>,
    account_id: Option<crate::AccountId>,
    contract_id: Option<crate::ContractId>,
    timestamp: Option<jiff::Timestamp>,
    trade_date: Option<TradeDate>,
    order_id: Option<crate::OrderId>,
    exec_type: Option<ExecutionReportExecType>,
    exec_ref_id: Option<String>,
    ord_status: Option<ExecutionReportOrdStatus>,
    action: Option<ExecutionReportAction>,
    cum_qty: Option<i64>,
    avg_px: Option<crate::Decimal>,
    last_qty: Option<i64>,
    last_px: Option<crate::Decimal>,
    reject_reason: Option<ExecutionReportRejectReason>,
    text: Option<String>,
    exchange_order_id: Option<String>,
}

impl ExecutionReportBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::ExecutionReportId) -> Self {
        self.id = Some(value);
        self
    }

    /// Sets wire field `commandId`.
    pub fn command_id(mut self, value: crate::CommandId) -> Self {
        self.command_id = Some(value);
        self
    }

    /// Sets wire field `name`.
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

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

    /// Sets wire field `timestamp`.
    pub fn timestamp(mut self, value: jiff::Timestamp) -> Self {
        self.timestamp = Some(value);
        self
    }

    /// Sets wire field `tradeDate`.
    pub fn trade_date(mut self, value: TradeDate) -> Self {
        self.trade_date = Some(value);
        self
    }

    /// Sets wire field `orderId`.
    pub fn order_id(mut self, value: crate::OrderId) -> Self {
        self.order_id = Some(value);
        self
    }

    /// Sets wire field `execType`.
    pub fn exec_type(mut self, value: ExecutionReportExecType) -> Self {
        self.exec_type = Some(value);
        self
    }

    /// Sets wire field `execRefId`.
    pub fn exec_ref_id(mut self, value: impl Into<String>) -> Self {
        self.exec_ref_id = Some(value.into());
        self
    }

    /// Sets wire field `ordStatus`.
    pub fn ord_status(mut self, value: ExecutionReportOrdStatus) -> Self {
        self.ord_status = Some(value);
        self
    }

    /// Sets wire field `action`.
    pub fn action(mut self, value: ExecutionReportAction) -> Self {
        self.action = Some(value);
        self
    }

    /// Sets wire field `cumQty`.
    pub fn cum_qty(mut self, value: i64) -> Self {
        self.cum_qty = Some(value);
        self
    }

    /// Sets wire field `avgPx`.
    pub fn avg_px(mut self, value: crate::Decimal) -> Self {
        self.avg_px = Some(value);
        self
    }

    /// Sets wire field `lastQty`.
    pub fn last_qty(mut self, value: i64) -> Self {
        self.last_qty = Some(value);
        self
    }

    /// Sets wire field `lastPx`.
    pub fn last_px(mut self, value: crate::Decimal) -> Self {
        self.last_px = Some(value);
        self
    }

    /// Sets wire field `rejectReason`.
    pub fn reject_reason(mut self, value: ExecutionReportRejectReason) -> Self {
        self.reject_reason = Some(value);
        self
    }

    /// Sets wire field `text`.
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Sets wire field `exchangeOrderId`.
    pub fn exchange_order_id(mut self, value: impl Into<String>) -> Self {
        self.exchange_order_id = Some(value.into());
        self
    }

    /// Validates required fields and builds [`ExecutionReport`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ExecutionReport, crate::api::current::BuildError> {
        let command_id = self
            .command_id
            .ok_or(crate::api::current::BuildError::missing("commandId"))?;
        let name = self
            .name
            .ok_or(crate::api::current::BuildError::missing("name"))?;
        let account_id = self
            .account_id
            .ok_or(crate::api::current::BuildError::missing("accountId"))?;
        let contract_id = self
            .contract_id
            .ok_or(crate::api::current::BuildError::missing("contractId"))?;
        let timestamp = self
            .timestamp
            .ok_or(crate::api::current::BuildError::missing("timestamp"))?;
        let order_id = self
            .order_id
            .ok_or(crate::api::current::BuildError::missing("orderId"))?;
        let exec_type = self
            .exec_type
            .ok_or(crate::api::current::BuildError::missing("execType"))?;
        let action = self
            .action
            .ok_or(crate::api::current::BuildError::missing("action"))?;
        Ok(ExecutionReport {
            id: self.id,
            command_id,
            name,
            account_id,
            contract_id,
            timestamp,
            trade_date: self.trade_date,
            order_id,
            exec_type,
            exec_ref_id: self.exec_ref_id,
            ord_status: self.ord_status,
            action,
            cum_qty: self.cum_qty,
            avg_px: self.avg_px,
            last_qty: self.last_qty,
            last_px: self.last_px,
            reject_reason: self.reject_reason,
            text: self.text,
            exchange_order_id: self.exchange_order_id,
        })
    }
}

/// Current provider values for `ExecutionReportAction`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum ExecutionReportAction {
    /// Provider value `Buy`.
    Buy,
    /// Provider value `Sell`.
    Sell,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl ExecutionReportAction {
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

impl serde::Serialize for ExecutionReportAction {
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

impl<'de> serde::Deserialize<'de> for ExecutionReportAction {
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

/// Current provider values for `ExecutionReportExecType`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum ExecutionReportExecType {
    /// Provider value `Canceled`.
    Canceled,
    /// Provider value `Completed`.
    Completed,
    /// Provider value `DoneForDay`.
    DoneForDay,
    /// Provider value `Expired`.
    Expired,
    /// Provider value `New`.
    New,
    /// Provider value `OrderStatus`.
    OrderStatus,
    /// Provider value `PendingCancel`.
    PendingCancel,
    /// Provider value `PendingNew`.
    PendingNew,
    /// Provider value `PendingReplace`.
    PendingReplace,
    /// Provider value `Rejected`.
    Rejected,
    /// Provider value `Replaced`.
    Replaced,
    /// Provider value `Stopped`.
    Stopped,
    /// Provider value `Suspended`.
    Suspended,
    /// Provider value `Trade`.
    Trade,
    /// Provider value `TradeCancel`.
    TradeCancel,
    /// Provider value `TradeCorrect`.
    TradeCorrect,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl ExecutionReportExecType {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Canceled => "Canceled",
            Self::Completed => "Completed",
            Self::DoneForDay => "DoneForDay",
            Self::Expired => "Expired",
            Self::New => "New",
            Self::OrderStatus => "OrderStatus",
            Self::PendingCancel => "PendingCancel",
            Self::PendingNew => "PendingNew",
            Self::PendingReplace => "PendingReplace",
            Self::Rejected => "Rejected",
            Self::Replaced => "Replaced",
            Self::Stopped => "Stopped",
            Self::Suspended => "Suspended",
            Self::Trade => "Trade",
            Self::TradeCancel => "TradeCancel",
            Self::TradeCorrect => "TradeCorrect",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for ExecutionReportExecType {
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

impl<'de> serde::Deserialize<'de> for ExecutionReportExecType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "Canceled" => Self::Canceled,
            "Completed" => Self::Completed,
            "DoneForDay" => Self::DoneForDay,
            "Expired" => Self::Expired,
            "New" => Self::New,
            "OrderStatus" => Self::OrderStatus,
            "PendingCancel" => Self::PendingCancel,
            "PendingNew" => Self::PendingNew,
            "PendingReplace" => Self::PendingReplace,
            "Rejected" => Self::Rejected,
            "Replaced" => Self::Replaced,
            "Stopped" => Self::Stopped,
            "Suspended" => Self::Suspended,
            "Trade" => Self::Trade,
            "TradeCancel" => Self::TradeCancel,
            "TradeCorrect" => Self::TradeCorrect,
            _ => Self::Unknown(value),
        })
    }
}

/// Current provider values for `ExecutionReportOrdStatus`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum ExecutionReportOrdStatus {
    /// Provider value `Canceled`.
    Canceled,
    /// Provider value `Completed`.
    Completed,
    /// Provider value `Expired`.
    Expired,
    /// Provider value `Filled`.
    Filled,
    /// Provider value `PendingCancel`.
    PendingCancel,
    /// Provider value `PendingNew`.
    PendingNew,
    /// Provider value `PendingReplace`.
    PendingReplace,
    /// Provider value `Rejected`.
    Rejected,
    /// Provider value `Suspended`.
    Suspended,
    /// Provider value `Unknown`.
    Unknown2,
    /// Provider value `Working`.
    Working,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl ExecutionReportOrdStatus {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Canceled => "Canceled",
            Self::Completed => "Completed",
            Self::Expired => "Expired",
            Self::Filled => "Filled",
            Self::PendingCancel => "PendingCancel",
            Self::PendingNew => "PendingNew",
            Self::PendingReplace => "PendingReplace",
            Self::Rejected => "Rejected",
            Self::Suspended => "Suspended",
            Self::Unknown2 => "Unknown",
            Self::Working => "Working",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for ExecutionReportOrdStatus {
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

impl<'de> serde::Deserialize<'de> for ExecutionReportOrdStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "Canceled" => Self::Canceled,
            "Completed" => Self::Completed,
            "Expired" => Self::Expired,
            "Filled" => Self::Filled,
            "PendingCancel" => Self::PendingCancel,
            "PendingNew" => Self::PendingNew,
            "PendingReplace" => Self::PendingReplace,
            "Rejected" => Self::Rejected,
            "Suspended" => Self::Suspended,
            "Unknown" => Self::Unknown2,
            "Working" => Self::Working,
            _ => Self::Unknown(value),
        })
    }
}

/// Current provider values for `ExecutionReportRejectReason`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum ExecutionReportRejectReason {
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

impl ExecutionReportRejectReason {
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

impl serde::Serialize for ExecutionReportRejectReason {
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

impl<'de> serde::Deserialize<'de> for ExecutionReportRejectReason {
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

/// Current wire model `ExpireUserLockout`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ExpireUserLockout {
    #[serde(rename = "userId")]
    user_id: crate::UserId,
    #[serde(rename = "notes", default, skip_serializing_if = "Option::is_none")]
    notes: Option<String>,
}

impl ExpireUserLockout {
    /// Returns wire field `userId`.
    #[must_use]
    pub fn user_id(&self) -> &crate::UserId {
        &self.user_id
    }

    /// Returns wire field `notes`.
    #[must_use]
    pub fn notes(&self) -> Option<&str> {
        self.notes.as_deref()
    }

    /// Starts a builder for [`ExpireUserLockout`].
    pub fn builder() -> ExpireUserLockoutBuilder {
        ExpireUserLockoutBuilder::default()
    }
}

/// Builder for [`ExpireUserLockout`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ExpireUserLockoutBuilder {
    user_id: Option<crate::UserId>,
    notes: Option<String>,
}

impl ExpireUserLockoutBuilder {
    /// Sets wire field `userId`.
    pub fn user_id(mut self, value: crate::UserId) -> Self {
        self.user_id = Some(value);
        self
    }

    /// Sets wire field `notes`.
    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    /// Validates required fields and builds [`ExpireUserLockout`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ExpireUserLockout, crate::api::current::BuildError> {
        let user_id = self
            .user_id
            .ok_or(crate::api::current::BuildError::missing("userId"))?;
        Ok(ExpireUserLockout {
            user_id,
            notes: self.notes,
        })
    }
}

impl crate::api::current::support::CurrentRequest for ExpireUserLockout {
    fn validate_current(&self) -> Result<(), crate::Error> {
        Ok(())
    }
}

/// Current wire model `Fill`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct Fill {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    id: Option<super::ids::FillId>,
    #[serde(rename = "orderId")]
    order_id: crate::OrderId,
    #[serde(rename = "contractId")]
    contract_id: crate::ContractId,
    #[serde(rename = "timestamp")]
    timestamp: jiff::Timestamp,
    #[serde(rename = "tradeDate")]
    trade_date: TradeDate,
    #[serde(rename = "action")]
    action: FillAction,
    #[serde(rename = "qty")]
    qty: i64,
    #[serde(rename = "price")]
    #[serde(with = "crate::decimal")]
    price: crate::Decimal,
    #[serde(rename = "active")]
    active: bool,
    #[serde(rename = "finallyPaired")]
    finally_paired: i64,
}

impl Fill {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> Option<&super::ids::FillId> {
        self.id.as_ref()
    }

    /// Returns wire field `orderId`.
    #[must_use]
    pub fn order_id(&self) -> &crate::OrderId {
        &self.order_id
    }

    /// Returns wire field `contractId`.
    #[must_use]
    pub fn contract_id(&self) -> &crate::ContractId {
        &self.contract_id
    }

    /// Returns wire field `timestamp`.
    #[must_use]
    pub fn timestamp(&self) -> &jiff::Timestamp {
        &self.timestamp
    }

    /// Returns wire field `tradeDate`.
    #[must_use]
    pub fn trade_date(&self) -> &TradeDate {
        &self.trade_date
    }

    /// Returns wire field `action`.
    #[must_use]
    pub fn action(&self) -> &FillAction {
        &self.action
    }

    /// Returns wire field `qty`.
    #[must_use]
    pub fn qty(&self) -> &i64 {
        &self.qty
    }

    /// Returns wire field `price`.
    #[must_use]
    pub fn price(&self) -> &crate::Decimal {
        &self.price
    }

    /// Returns wire field `active`.
    #[must_use]
    pub fn active(&self) -> &bool {
        &self.active
    }

    /// Returns wire field `finallyPaired`.
    #[must_use]
    pub fn finally_paired(&self) -> &i64 {
        &self.finally_paired
    }

    /// Starts a builder for [`Fill`].
    pub fn builder() -> FillBuilder {
        FillBuilder::default()
    }
}

/// Builder for [`Fill`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct FillBuilder {
    id: Option<super::ids::FillId>,
    order_id: Option<crate::OrderId>,
    contract_id: Option<crate::ContractId>,
    timestamp: Option<jiff::Timestamp>,
    trade_date: Option<TradeDate>,
    action: Option<FillAction>,
    qty: Option<i64>,
    price: Option<crate::Decimal>,
    active: Option<bool>,
    finally_paired: Option<i64>,
}

impl FillBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::FillId) -> Self {
        self.id = Some(value);
        self
    }

    /// Sets wire field `orderId`.
    pub fn order_id(mut self, value: crate::OrderId) -> Self {
        self.order_id = Some(value);
        self
    }

    /// Sets wire field `contractId`.
    pub fn contract_id(mut self, value: crate::ContractId) -> Self {
        self.contract_id = Some(value);
        self
    }

    /// Sets wire field `timestamp`.
    pub fn timestamp(mut self, value: jiff::Timestamp) -> Self {
        self.timestamp = Some(value);
        self
    }

    /// Sets wire field `tradeDate`.
    pub fn trade_date(mut self, value: TradeDate) -> Self {
        self.trade_date = Some(value);
        self
    }

    /// Sets wire field `action`.
    pub fn action(mut self, value: FillAction) -> Self {
        self.action = Some(value);
        self
    }

    /// Sets wire field `qty`.
    pub fn qty(mut self, value: i64) -> Self {
        self.qty = Some(value);
        self
    }

    /// Sets wire field `price`.
    pub fn price(mut self, value: crate::Decimal) -> Self {
        self.price = Some(value);
        self
    }

    /// Sets wire field `active`.
    pub fn active(mut self, value: bool) -> Self {
        self.active = Some(value);
        self
    }

    /// Sets wire field `finallyPaired`.
    pub fn finally_paired(mut self, value: i64) -> Self {
        self.finally_paired = Some(value);
        self
    }

    /// Validates required fields and builds [`Fill`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<Fill, crate::api::current::BuildError> {
        let order_id = self
            .order_id
            .ok_or(crate::api::current::BuildError::missing("orderId"))?;
        let contract_id = self
            .contract_id
            .ok_or(crate::api::current::BuildError::missing("contractId"))?;
        let timestamp = self
            .timestamp
            .ok_or(crate::api::current::BuildError::missing("timestamp"))?;
        let trade_date = self
            .trade_date
            .ok_or(crate::api::current::BuildError::missing("tradeDate"))?;
        let action = self
            .action
            .ok_or(crate::api::current::BuildError::missing("action"))?;
        let qty = self
            .qty
            .ok_or(crate::api::current::BuildError::missing("qty"))?;
        let price = self
            .price
            .ok_or(crate::api::current::BuildError::missing("price"))?;
        let active = self
            .active
            .ok_or(crate::api::current::BuildError::missing("active"))?;
        let finally_paired = self
            .finally_paired
            .ok_or(crate::api::current::BuildError::missing("finallyPaired"))?;
        Ok(Fill {
            id: self.id,
            order_id,
            contract_id,
            timestamp,
            trade_date,
            action,
            qty,
            price,
            active,
            finally_paired,
        })
    }
}

/// Current provider values for `FillAction`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum FillAction {
    /// Provider value `Buy`.
    Buy,
    /// Provider value `Sell`.
    Sell,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl FillAction {
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

impl serde::Serialize for FillAction {
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

impl<'de> serde::Deserialize<'de> for FillAction {
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

/// Current wire model `FillFee`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct FillFee {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    id: Option<super::ids::FillFeeId>,
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
}

impl FillFee {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> Option<&super::ids::FillFeeId> {
        self.id.as_ref()
    }

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

    /// Starts a builder for [`FillFee`].
    pub fn builder() -> FillFeeBuilder {
        FillFeeBuilder::default()
    }
}

/// Builder for [`FillFee`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct FillFeeBuilder {
    id: Option<super::ids::FillFeeId>,
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
}

impl FillFeeBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::FillFeeId) -> Self {
        self.id = Some(value);
        self
    }

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

    /// Validates required fields and builds [`FillFee`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<FillFee, crate::api::current::BuildError> {
        Ok(FillFee {
            id: self.id,
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
        })
    }
}

/// Current wire model `FillPair`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct FillPair {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    id: Option<super::ids::FillPairId>,
    #[serde(rename = "positionId")]
    position_id: crate::PositionId,
    #[serde(rename = "buyFillId")]
    buy_fill_id: super::ids::FillId,
    #[serde(rename = "sellFillId")]
    sell_fill_id: super::ids::FillId,
    #[serde(rename = "qty")]
    qty: i64,
    #[serde(rename = "buyPrice")]
    #[serde(with = "crate::decimal")]
    buy_price: crate::Decimal,
    #[serde(rename = "sellPrice")]
    #[serde(with = "crate::decimal")]
    sell_price: crate::Decimal,
    #[serde(rename = "active")]
    active: bool,
}

impl FillPair {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> Option<&super::ids::FillPairId> {
        self.id.as_ref()
    }

    /// Returns wire field `positionId`.
    #[must_use]
    pub fn position_id(&self) -> &crate::PositionId {
        &self.position_id
    }

    /// Returns wire field `buyFillId`.
    #[must_use]
    pub fn buy_fill_id(&self) -> &super::ids::FillId {
        &self.buy_fill_id
    }

    /// Returns wire field `sellFillId`.
    #[must_use]
    pub fn sell_fill_id(&self) -> &super::ids::FillId {
        &self.sell_fill_id
    }

    /// Returns wire field `qty`.
    #[must_use]
    pub fn qty(&self) -> &i64 {
        &self.qty
    }

    /// Returns wire field `buyPrice`.
    #[must_use]
    pub fn buy_price(&self) -> &crate::Decimal {
        &self.buy_price
    }

    /// Returns wire field `sellPrice`.
    #[must_use]
    pub fn sell_price(&self) -> &crate::Decimal {
        &self.sell_price
    }

    /// Returns wire field `active`.
    #[must_use]
    pub fn active(&self) -> &bool {
        &self.active
    }

    /// Starts a builder for [`FillPair`].
    pub fn builder() -> FillPairBuilder {
        FillPairBuilder::default()
    }
}

/// Builder for [`FillPair`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct FillPairBuilder {
    id: Option<super::ids::FillPairId>,
    position_id: Option<crate::PositionId>,
    buy_fill_id: Option<super::ids::FillId>,
    sell_fill_id: Option<super::ids::FillId>,
    qty: Option<i64>,
    buy_price: Option<crate::Decimal>,
    sell_price: Option<crate::Decimal>,
    active: Option<bool>,
}

impl FillPairBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::FillPairId) -> Self {
        self.id = Some(value);
        self
    }

    /// Sets wire field `positionId`.
    pub fn position_id(mut self, value: crate::PositionId) -> Self {
        self.position_id = Some(value);
        self
    }

    /// Sets wire field `buyFillId`.
    pub fn buy_fill_id(mut self, value: super::ids::FillId) -> Self {
        self.buy_fill_id = Some(value);
        self
    }

    /// Sets wire field `sellFillId`.
    pub fn sell_fill_id(mut self, value: super::ids::FillId) -> Self {
        self.sell_fill_id = Some(value);
        self
    }

    /// Sets wire field `qty`.
    pub fn qty(mut self, value: i64) -> Self {
        self.qty = Some(value);
        self
    }

    /// Sets wire field `buyPrice`.
    pub fn buy_price(mut self, value: crate::Decimal) -> Self {
        self.buy_price = Some(value);
        self
    }

    /// Sets wire field `sellPrice`.
    pub fn sell_price(mut self, value: crate::Decimal) -> Self {
        self.sell_price = Some(value);
        self
    }

    /// Sets wire field `active`.
    pub fn active(mut self, value: bool) -> Self {
        self.active = Some(value);
        self
    }

    /// Validates required fields and builds [`FillPair`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<FillPair, crate::api::current::BuildError> {
        let position_id = self
            .position_id
            .ok_or(crate::api::current::BuildError::missing("positionId"))?;
        let buy_fill_id = self
            .buy_fill_id
            .ok_or(crate::api::current::BuildError::missing("buyFillId"))?;
        let sell_fill_id = self
            .sell_fill_id
            .ok_or(crate::api::current::BuildError::missing("sellFillId"))?;
        let qty = self
            .qty
            .ok_or(crate::api::current::BuildError::missing("qty"))?;
        let buy_price = self
            .buy_price
            .ok_or(crate::api::current::BuildError::missing("buyPrice"))?;
        let sell_price = self
            .sell_price
            .ok_or(crate::api::current::BuildError::missing("sellPrice"))?;
        let active = self
            .active
            .ok_or(crate::api::current::BuildError::missing("active"))?;
        Ok(FillPair {
            id: self.id,
            position_id,
            buy_fill_id,
            sell_fill_id,
            qty,
            buy_price,
            sell_price,
            active,
        })
    }
}

/// Current wire model `GetAccountTradingPermissions`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct GetAccountTradingPermissions {
    #[serde(rename = "accountId")]
    account_id: crate::AccountId,
}

impl GetAccountTradingPermissions {
    /// Returns wire field `accountId`.
    #[must_use]
    pub fn account_id(&self) -> &crate::AccountId {
        &self.account_id
    }

    /// Starts a builder for [`GetAccountTradingPermissions`].
    pub fn builder() -> GetAccountTradingPermissionsBuilder {
        GetAccountTradingPermissionsBuilder::default()
    }
}

/// Builder for [`GetAccountTradingPermissions`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct GetAccountTradingPermissionsBuilder {
    account_id: Option<crate::AccountId>,
}

impl GetAccountTradingPermissionsBuilder {
    /// Sets wire field `accountId`.
    pub fn account_id(mut self, value: crate::AccountId) -> Self {
        self.account_id = Some(value);
        self
    }

    /// Validates required fields and builds [`GetAccountTradingPermissions`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<GetAccountTradingPermissions, crate::api::current::BuildError> {
        let account_id = self
            .account_id
            .ok_or(crate::api::current::BuildError::missing("accountId"))?;
        Ok(GetAccountTradingPermissions { account_id })
    }
}

impl crate::api::current::support::CurrentRequest for GetAccountTradingPermissions {
    fn validate_current(&self) -> Result<(), crate::Error> {
        Ok(())
    }
}

/// Current wire model `GetPartnerSubAccountRequestStatus`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct GetPartnerSubAccountRequestStatus {
    #[serde(rename = "subAccountRequestId")]
    sub_account_request_id: super::ids::SubAccountRequestId,
}

impl GetPartnerSubAccountRequestStatus {
    /// Returns wire field `subAccountRequestId`.
    #[must_use]
    pub fn sub_account_request_id(&self) -> &super::ids::SubAccountRequestId {
        &self.sub_account_request_id
    }

    /// Starts a builder for [`GetPartnerSubAccountRequestStatus`].
    pub fn builder() -> GetPartnerSubAccountRequestStatusBuilder {
        GetPartnerSubAccountRequestStatusBuilder::default()
    }
}

/// Builder for [`GetPartnerSubAccountRequestStatus`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct GetPartnerSubAccountRequestStatusBuilder {
    sub_account_request_id: Option<super::ids::SubAccountRequestId>,
}

impl GetPartnerSubAccountRequestStatusBuilder {
    /// Sets wire field `subAccountRequestId`.
    pub fn sub_account_request_id(mut self, value: super::ids::SubAccountRequestId) -> Self {
        self.sub_account_request_id = Some(value);
        self
    }

    /// Validates required fields and builds [`GetPartnerSubAccountRequestStatus`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(
        self,
    ) -> Result<GetPartnerSubAccountRequestStatus, crate::api::current::BuildError> {
        let sub_account_request_id =
            self.sub_account_request_id
                .ok_or(crate::api::current::BuildError::missing(
                    "subAccountRequestId",
                ))?;
        Ok(GetPartnerSubAccountRequestStatus {
            sub_account_request_id,
        })
    }
}

impl crate::api::current::support::CurrentRequest for GetPartnerSubAccountRequestStatus {
    fn validate_current(&self) -> Result<(), crate::Error> {
        Ok(())
    }
}

/// Current wire model `MarginSnapshot`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct MarginSnapshot {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    id: Option<super::ids::MarginSnapshotId>,
    #[serde(rename = "timestamp")]
    timestamp: jiff::Timestamp,
    #[serde(rename = "riskTimePeriodId")]
    risk_time_period_id: super::ids::RiskTimePeriodId,
    #[serde(rename = "initialMargin")]
    #[serde(with = "crate::decimal")]
    initial_margin: crate::Decimal,
    #[serde(rename = "maintenanceMargin")]
    #[serde(with = "crate::decimal")]
    maintenance_margin: crate::Decimal,
    #[serde(
        rename = "autoLiqLevel",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    auto_liq_level: Option<crate::Decimal>,
    #[serde(
        rename = "liqOnlyLevel",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    liq_only_level: Option<crate::Decimal>,
    #[serde(rename = "totalUsedMargin")]
    #[serde(with = "crate::decimal")]
    total_used_margin: crate::Decimal,
    #[serde(rename = "fullInitialMargin")]
    #[serde(with = "crate::decimal")]
    full_initial_margin: crate::Decimal,
    #[serde(rename = "positionMargin")]
    #[serde(with = "crate::decimal")]
    position_margin: crate::Decimal,
    #[serde(rename = "totalUsedFullMargin")]
    #[serde(with = "crate::decimal")]
    total_used_full_margin: crate::Decimal,
    #[serde(
        rename = "openCollateralReq",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    open_collateral_req: Option<crate::Decimal>,
}

impl MarginSnapshot {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> Option<&super::ids::MarginSnapshotId> {
        self.id.as_ref()
    }

    /// Returns wire field `timestamp`.
    #[must_use]
    pub fn timestamp(&self) -> &jiff::Timestamp {
        &self.timestamp
    }

    /// Returns wire field `riskTimePeriodId`.
    #[must_use]
    pub fn risk_time_period_id(&self) -> &super::ids::RiskTimePeriodId {
        &self.risk_time_period_id
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

    /// Returns wire field `autoLiqLevel`.
    #[must_use]
    pub fn auto_liq_level(&self) -> Option<&crate::Decimal> {
        self.auto_liq_level.as_ref()
    }

    /// Returns wire field `liqOnlyLevel`.
    #[must_use]
    pub fn liq_only_level(&self) -> Option<&crate::Decimal> {
        self.liq_only_level.as_ref()
    }

    /// Returns wire field `totalUsedMargin`.
    #[must_use]
    pub fn total_used_margin(&self) -> &crate::Decimal {
        &self.total_used_margin
    }

    /// Returns wire field `fullInitialMargin`.
    #[must_use]
    pub fn full_initial_margin(&self) -> &crate::Decimal {
        &self.full_initial_margin
    }

    /// Returns wire field `positionMargin`.
    #[must_use]
    pub fn position_margin(&self) -> &crate::Decimal {
        &self.position_margin
    }

    /// Returns wire field `totalUsedFullMargin`.
    #[must_use]
    pub fn total_used_full_margin(&self) -> &crate::Decimal {
        &self.total_used_full_margin
    }

    /// Returns wire field `openCollateralReq`.
    #[must_use]
    pub fn open_collateral_req(&self) -> Option<&crate::Decimal> {
        self.open_collateral_req.as_ref()
    }

    /// Starts a builder for [`MarginSnapshot`].
    pub fn builder() -> MarginSnapshotBuilder {
        MarginSnapshotBuilder::default()
    }
}

/// Builder for [`MarginSnapshot`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct MarginSnapshotBuilder {
    id: Option<super::ids::MarginSnapshotId>,
    timestamp: Option<jiff::Timestamp>,
    risk_time_period_id: Option<super::ids::RiskTimePeriodId>,
    initial_margin: Option<crate::Decimal>,
    maintenance_margin: Option<crate::Decimal>,
    auto_liq_level: Option<crate::Decimal>,
    liq_only_level: Option<crate::Decimal>,
    total_used_margin: Option<crate::Decimal>,
    full_initial_margin: Option<crate::Decimal>,
    position_margin: Option<crate::Decimal>,
    total_used_full_margin: Option<crate::Decimal>,
    open_collateral_req: Option<crate::Decimal>,
}

impl MarginSnapshotBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::MarginSnapshotId) -> Self {
        self.id = Some(value);
        self
    }

    /// Sets wire field `timestamp`.
    pub fn timestamp(mut self, value: jiff::Timestamp) -> Self {
        self.timestamp = Some(value);
        self
    }

    /// Sets wire field `riskTimePeriodId`.
    pub fn risk_time_period_id(mut self, value: super::ids::RiskTimePeriodId) -> Self {
        self.risk_time_period_id = Some(value);
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

    /// Sets wire field `autoLiqLevel`.
    pub fn auto_liq_level(mut self, value: crate::Decimal) -> Self {
        self.auto_liq_level = Some(value);
        self
    }

    /// Sets wire field `liqOnlyLevel`.
    pub fn liq_only_level(mut self, value: crate::Decimal) -> Self {
        self.liq_only_level = Some(value);
        self
    }

    /// Sets wire field `totalUsedMargin`.
    pub fn total_used_margin(mut self, value: crate::Decimal) -> Self {
        self.total_used_margin = Some(value);
        self
    }

    /// Sets wire field `fullInitialMargin`.
    pub fn full_initial_margin(mut self, value: crate::Decimal) -> Self {
        self.full_initial_margin = Some(value);
        self
    }

    /// Sets wire field `positionMargin`.
    pub fn position_margin(mut self, value: crate::Decimal) -> Self {
        self.position_margin = Some(value);
        self
    }

    /// Sets wire field `totalUsedFullMargin`.
    pub fn total_used_full_margin(mut self, value: crate::Decimal) -> Self {
        self.total_used_full_margin = Some(value);
        self
    }

    /// Sets wire field `openCollateralReq`.
    pub fn open_collateral_req(mut self, value: crate::Decimal) -> Self {
        self.open_collateral_req = Some(value);
        self
    }

    /// Validates required fields and builds [`MarginSnapshot`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<MarginSnapshot, crate::api::current::BuildError> {
        let timestamp = self
            .timestamp
            .ok_or(crate::api::current::BuildError::missing("timestamp"))?;
        let risk_time_period_id = self
            .risk_time_period_id
            .ok_or(crate::api::current::BuildError::missing("riskTimePeriodId"))?;
        let initial_margin = self
            .initial_margin
            .ok_or(crate::api::current::BuildError::missing("initialMargin"))?;
        let maintenance_margin =
            self.maintenance_margin
                .ok_or(crate::api::current::BuildError::missing(
                    "maintenanceMargin",
                ))?;
        let total_used_margin = self
            .total_used_margin
            .ok_or(crate::api::current::BuildError::missing("totalUsedMargin"))?;
        let full_initial_margin =
            self.full_initial_margin
                .ok_or(crate::api::current::BuildError::missing(
                    "fullInitialMargin",
                ))?;
        let position_margin = self
            .position_margin
            .ok_or(crate::api::current::BuildError::missing("positionMargin"))?;
        let total_used_full_margin =
            self.total_used_full_margin
                .ok_or(crate::api::current::BuildError::missing(
                    "totalUsedFullMargin",
                ))?;
        Ok(MarginSnapshot {
            id: self.id,
            timestamp,
            risk_time_period_id,
            initial_margin,
            maintenance_margin,
            auto_liq_level: self.auto_liq_level,
            liq_only_level: self.liq_only_level,
            total_used_margin,
            full_initial_margin,
            position_margin,
            total_used_full_margin,
            open_collateral_req: self.open_collateral_req,
        })
    }
}

/// Current wire model `MarketDataSubscription`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct MarketDataSubscription {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    id: Option<super::ids::MarketDataSubscriptionId>,
    #[serde(rename = "userId")]
    user_id: crate::UserId,
    #[serde(rename = "timestamp")]
    timestamp: jiff::Timestamp,
    #[serde(rename = "planPrice")]
    #[serde(with = "crate::decimal")]
    plan_price: crate::Decimal,
    #[serde(
        rename = "cashBalanceLogId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    cash_balance_log_id: Option<super::ids::CashBalanceLogId>,
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    account_id: Option<crate::AccountId>,
    #[serde(rename = "marketDataSubscriptionPlanId")]
    market_data_subscription_plan_id: super::ids::MarketDataSubscriptionPlanId,
    #[serde(rename = "year")]
    year: i64,
    #[serde(rename = "month")]
    month: i64,
    #[serde(rename = "expired")]
    expired: bool,
    #[serde(
        rename = "renewalCreditCardId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    renewal_credit_card_id: Option<super::ids::CreditCardId>,
    #[serde(
        rename = "renewalAccountId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    renewal_account_id: Option<crate::AccountId>,
}

impl MarketDataSubscription {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> Option<&super::ids::MarketDataSubscriptionId> {
        self.id.as_ref()
    }

    /// Returns wire field `userId`.
    #[must_use]
    pub fn user_id(&self) -> &crate::UserId {
        &self.user_id
    }

    /// Returns wire field `timestamp`.
    #[must_use]
    pub fn timestamp(&self) -> &jiff::Timestamp {
        &self.timestamp
    }

    /// Returns wire field `planPrice`.
    #[must_use]
    pub fn plan_price(&self) -> &crate::Decimal {
        &self.plan_price
    }

    /// Returns wire field `cashBalanceLogId`.
    #[must_use]
    pub fn cash_balance_log_id(&self) -> Option<&super::ids::CashBalanceLogId> {
        self.cash_balance_log_id.as_ref()
    }

    /// Returns wire field `accountId`.
    #[must_use]
    pub fn account_id(&self) -> Option<&crate::AccountId> {
        self.account_id.as_ref()
    }

    /// Returns wire field `marketDataSubscriptionPlanId`.
    #[must_use]
    pub fn market_data_subscription_plan_id(&self) -> &super::ids::MarketDataSubscriptionPlanId {
        &self.market_data_subscription_plan_id
    }

    /// Returns wire field `year`.
    #[must_use]
    pub fn year(&self) -> &i64 {
        &self.year
    }

    /// Returns wire field `month`.
    #[must_use]
    pub fn month(&self) -> &i64 {
        &self.month
    }

    /// Returns wire field `expired`.
    #[must_use]
    pub fn expired(&self) -> &bool {
        &self.expired
    }

    /// Returns wire field `renewalCreditCardId`.
    #[must_use]
    pub fn renewal_credit_card_id(&self) -> Option<&super::ids::CreditCardId> {
        self.renewal_credit_card_id.as_ref()
    }

    /// Returns wire field `renewalAccountId`.
    #[must_use]
    pub fn renewal_account_id(&self) -> Option<&crate::AccountId> {
        self.renewal_account_id.as_ref()
    }

    /// Starts a builder for [`MarketDataSubscription`].
    pub fn builder() -> MarketDataSubscriptionBuilder {
        MarketDataSubscriptionBuilder::default()
    }
}

/// Builder for [`MarketDataSubscription`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct MarketDataSubscriptionBuilder {
    id: Option<super::ids::MarketDataSubscriptionId>,
    user_id: Option<crate::UserId>,
    timestamp: Option<jiff::Timestamp>,
    plan_price: Option<crate::Decimal>,
    cash_balance_log_id: Option<super::ids::CashBalanceLogId>,
    account_id: Option<crate::AccountId>,
    market_data_subscription_plan_id: Option<super::ids::MarketDataSubscriptionPlanId>,
    year: Option<i64>,
    month: Option<i64>,
    expired: Option<bool>,
    renewal_credit_card_id: Option<super::ids::CreditCardId>,
    renewal_account_id: Option<crate::AccountId>,
}

impl MarketDataSubscriptionBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::MarketDataSubscriptionId) -> Self {
        self.id = Some(value);
        self
    }

    /// Sets wire field `userId`.
    pub fn user_id(mut self, value: crate::UserId) -> Self {
        self.user_id = Some(value);
        self
    }

    /// Sets wire field `timestamp`.
    pub fn timestamp(mut self, value: jiff::Timestamp) -> Self {
        self.timestamp = Some(value);
        self
    }

    /// Sets wire field `planPrice`.
    pub fn plan_price(mut self, value: crate::Decimal) -> Self {
        self.plan_price = Some(value);
        self
    }

    /// Sets wire field `cashBalanceLogId`.
    pub fn cash_balance_log_id(mut self, value: super::ids::CashBalanceLogId) -> Self {
        self.cash_balance_log_id = Some(value);
        self
    }

    /// Sets wire field `accountId`.
    pub fn account_id(mut self, value: crate::AccountId) -> Self {
        self.account_id = Some(value);
        self
    }

    /// Sets wire field `marketDataSubscriptionPlanId`.
    pub fn market_data_subscription_plan_id(
        mut self,
        value: super::ids::MarketDataSubscriptionPlanId,
    ) -> Self {
        self.market_data_subscription_plan_id = Some(value);
        self
    }

    /// Sets wire field `year`.
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    /// Sets wire field `month`.
    pub fn month(mut self, value: i64) -> Self {
        self.month = Some(value);
        self
    }

    /// Sets wire field `expired`.
    pub fn expired(mut self, value: bool) -> Self {
        self.expired = Some(value);
        self
    }

    /// Sets wire field `renewalCreditCardId`.
    pub fn renewal_credit_card_id(mut self, value: super::ids::CreditCardId) -> Self {
        self.renewal_credit_card_id = Some(value);
        self
    }

    /// Sets wire field `renewalAccountId`.
    pub fn renewal_account_id(mut self, value: crate::AccountId) -> Self {
        self.renewal_account_id = Some(value);
        self
    }

    /// Validates required fields and builds [`MarketDataSubscription`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<MarketDataSubscription, crate::api::current::BuildError> {
        let user_id = self
            .user_id
            .ok_or(crate::api::current::BuildError::missing("userId"))?;
        let timestamp = self
            .timestamp
            .ok_or(crate::api::current::BuildError::missing("timestamp"))?;
        let plan_price = self
            .plan_price
            .ok_or(crate::api::current::BuildError::missing("planPrice"))?;
        let market_data_subscription_plan_id = self.market_data_subscription_plan_id.ok_or(
            crate::api::current::BuildError::missing("marketDataSubscriptionPlanId"),
        )?;
        let year = self
            .year
            .ok_or(crate::api::current::BuildError::missing("year"))?;
        let month = self
            .month
            .ok_or(crate::api::current::BuildError::missing("month"))?;
        let expired = self
            .expired
            .ok_or(crate::api::current::BuildError::missing("expired"))?;
        Ok(MarketDataSubscription {
            id: self.id,
            user_id,
            timestamp,
            plan_price,
            cash_balance_log_id: self.cash_balance_log_id,
            account_id: self.account_id,
            market_data_subscription_plan_id,
            year,
            month,
            expired,
            renewal_credit_card_id: self.renewal_credit_card_id,
            renewal_account_id: self.renewal_account_id,
        })
    }
}

impl crate::api::current::support::CurrentRequest for MarketDataSubscription {
    fn validate_current(&self) -> Result<(), crate::Error> {
        Ok(())
    }
}

/// Current wire model `MarketDataSubscriptionResponse`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct MarketDataSubscriptionResponse {
    #[serde(rename = "errorText", default, skip_serializing_if = "Option::is_none")]
    error_text: Option<String>,
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    error_code: Option<MarketDataSubscriptionResponseErrorCode>,
    #[serde(
        rename = "marketDataSubscription",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    market_data_subscription: Option<MarketDataSubscription>,
}

impl MarketDataSubscriptionResponse {
    /// Returns wire field `errorText`.
    #[must_use]
    pub fn error_text(&self) -> Option<&str> {
        self.error_text.as_deref()
    }

    /// Returns wire field `errorCode`.
    #[must_use]
    pub fn error_code(&self) -> Option<&MarketDataSubscriptionResponseErrorCode> {
        self.error_code.as_ref()
    }

    /// Returns wire field `marketDataSubscription`.
    #[must_use]
    pub fn market_data_subscription(&self) -> Option<&MarketDataSubscription> {
        self.market_data_subscription.as_ref()
    }

    /// Starts a builder for [`MarketDataSubscriptionResponse`].
    pub fn builder() -> MarketDataSubscriptionResponseBuilder {
        MarketDataSubscriptionResponseBuilder::default()
    }
}

/// Builder for [`MarketDataSubscriptionResponse`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct MarketDataSubscriptionResponseBuilder {
    error_text: Option<String>,
    error_code: Option<MarketDataSubscriptionResponseErrorCode>,
    market_data_subscription: Option<MarketDataSubscription>,
}

impl MarketDataSubscriptionResponseBuilder {
    /// Sets wire field `errorText`.
    pub fn error_text(mut self, value: impl Into<String>) -> Self {
        self.error_text = Some(value.into());
        self
    }

    /// Sets wire field `errorCode`.
    pub fn error_code(mut self, value: MarketDataSubscriptionResponseErrorCode) -> Self {
        self.error_code = Some(value);
        self
    }

    /// Sets wire field `marketDataSubscription`.
    pub fn market_data_subscription(mut self, value: MarketDataSubscription) -> Self {
        self.market_data_subscription = Some(value);
        self
    }

    /// Validates required fields and builds [`MarketDataSubscriptionResponse`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<MarketDataSubscriptionResponse, crate::api::current::BuildError> {
        Ok(MarketDataSubscriptionResponse {
            error_text: self.error_text,
            error_code: self.error_code,
            market_data_subscription: self.market_data_subscription,
        })
    }
}

/// Current provider values for `MarketDataSubscriptionResponseErrorCode`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum MarketDataSubscriptionResponseErrorCode {
    /// Provider value `ConflictWithExisting`.
    ConflictWithExisting,
    /// Provider value `DowngradeNotAllowed`.
    DowngradeNotAllowed,
    /// Provider value `IncompatibleCMEMarketDataSubscriptionPlans`.
    IncompatibleCmeMarketDataSubscriptionPlans,
    /// Provider value `IncorrectPaymentMethod`.
    IncorrectPaymentMethod,
    /// Provider value `InsufficientFunds`.
    InsufficientFunds,
    /// Provider value `PaymentProviderError`.
    PaymentProviderError,
    /// Provider value `PlanDiscontinued`.
    PlanDiscontinued,
    /// Provider value `SingleTrialOnly`.
    SingleTrialOnly,
    /// Provider value `Success`.
    Success,
    /// Provider value `UnknownError`.
    UnknownError,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl MarketDataSubscriptionResponseErrorCode {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::ConflictWithExisting => "ConflictWithExisting",
            Self::DowngradeNotAllowed => "DowngradeNotAllowed",
            Self::IncompatibleCmeMarketDataSubscriptionPlans => {
                "IncompatibleCMEMarketDataSubscriptionPlans"
            }
            Self::IncorrectPaymentMethod => "IncorrectPaymentMethod",
            Self::InsufficientFunds => "InsufficientFunds",
            Self::PaymentProviderError => "PaymentProviderError",
            Self::PlanDiscontinued => "PlanDiscontinued",
            Self::SingleTrialOnly => "SingleTrialOnly",
            Self::Success => "Success",
            Self::UnknownError => "UnknownError",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for MarketDataSubscriptionResponseErrorCode {
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

impl<'de> serde::Deserialize<'de> for MarketDataSubscriptionResponseErrorCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "ConflictWithExisting" => Self::ConflictWithExisting,
            "DowngradeNotAllowed" => Self::DowngradeNotAllowed,
            "IncompatibleCMEMarketDataSubscriptionPlans" => {
                Self::IncompatibleCmeMarketDataSubscriptionPlans
            }
            "IncorrectPaymentMethod" => Self::IncorrectPaymentMethod,
            "InsufficientFunds" => Self::InsufficientFunds,
            "PaymentProviderError" => Self::PaymentProviderError,
            "PlanDiscontinued" => Self::PlanDiscontinued,
            "SingleTrialOnly" => Self::SingleTrialOnly,
            "Success" => Self::Success,
            "UnknownError" => Self::UnknownError,
            _ => Self::Unknown(value),
        })
    }
}

/// Current wire model `ModifyCredentials`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ModifyCredentials {
    #[serde(rename = "userId", default, skip_serializing_if = "Option::is_none")]
    user_id: Option<crate::UserId>,
    #[serde(rename = "name")]
    name: crate::api::current::SecretValue,
    #[serde(rename = "password")]
    password: crate::api::current::SecretValue,
    #[serde(rename = "currentPassword")]
    current_password: crate::api::current::SecretValue,
}

impl ModifyCredentials {
    /// Returns wire field `userId`.
    #[must_use]
    pub fn user_id(&self) -> Option<&crate::UserId> {
        self.user_id.as_ref()
    }

    /// Reports whether secret field `name` is present.
    #[must_use]
    pub const fn has_name(&self) -> bool {
        true
    }

    /// Reports whether secret field `password` is present.
    #[must_use]
    pub const fn has_password(&self) -> bool {
        true
    }

    /// Reports whether secret field `currentPassword` is present.
    #[must_use]
    pub const fn has_current_password(&self) -> bool {
        true
    }

    /// Starts a builder for [`ModifyCredentials`].
    pub fn builder() -> ModifyCredentialsBuilder {
        ModifyCredentialsBuilder::default()
    }
}

/// Builder for [`ModifyCredentials`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ModifyCredentialsBuilder {
    user_id: Option<crate::UserId>,
    name: Option<crate::api::current::SecretValue>,
    password: Option<crate::api::current::SecretValue>,
    current_password: Option<crate::api::current::SecretValue>,
}

impl ModifyCredentialsBuilder {
    /// Sets wire field `userId`.
    pub fn user_id(mut self, value: crate::UserId) -> Self {
        self.user_id = Some(value);
        self
    }

    /// Sets wire field `name`.
    pub fn name(mut self, value: crate::api::current::SecretValue) -> Self {
        self.name = Some(value);
        self
    }

    /// Sets wire field `password`.
    pub fn password(mut self, value: crate::api::current::SecretValue) -> Self {
        self.password = Some(value);
        self
    }

    /// Sets wire field `currentPassword`.
    pub fn current_password(mut self, value: crate::api::current::SecretValue) -> Self {
        self.current_password = Some(value);
        self
    }

    /// Validates required fields and builds [`ModifyCredentials`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ModifyCredentials, crate::api::current::BuildError> {
        let name = self
            .name
            .ok_or(crate::api::current::BuildError::missing("name"))?;
        let password = self
            .password
            .ok_or(crate::api::current::BuildError::missing("password"))?;
        let current_password = self
            .current_password
            .ok_or(crate::api::current::BuildError::missing("currentPassword"))?;
        Ok(ModifyCredentials {
            user_id: self.user_id,
            name,
            password,
            current_password,
        })
    }
}

impl crate::api::current::support::CurrentRequest for ModifyCredentials {
    fn validate_current(&self) -> Result<(), crate::Error> {
        Ok(())
    }
}

/// Current wire model `ModifyEmailAddress`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ModifyEmailAddress {
    #[serde(rename = "userId", default, skip_serializing_if = "Option::is_none")]
    user_id: Option<crate::UserId>,
    #[serde(rename = "email")]
    email: String,
}

impl ModifyEmailAddress {
    /// Returns wire field `userId`.
    #[must_use]
    pub fn user_id(&self) -> Option<&crate::UserId> {
        self.user_id.as_ref()
    }

    /// Returns wire field `email`.
    #[must_use]
    pub fn email(&self) -> &str {
        &self.email
    }

    /// Starts a builder for [`ModifyEmailAddress`].
    pub fn builder() -> ModifyEmailAddressBuilder {
        ModifyEmailAddressBuilder::default()
    }
}

/// Builder for [`ModifyEmailAddress`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ModifyEmailAddressBuilder {
    user_id: Option<crate::UserId>,
    email: Option<String>,
}

impl ModifyEmailAddressBuilder {
    /// Sets wire field `userId`.
    pub fn user_id(mut self, value: crate::UserId) -> Self {
        self.user_id = Some(value);
        self
    }

    /// Sets wire field `email`.
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    /// Validates required fields and builds [`ModifyEmailAddress`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ModifyEmailAddress, crate::api::current::BuildError> {
        let email = self
            .email
            .ok_or(crate::api::current::BuildError::missing("email"))?;
        if email.is_empty() || email.trim() != email {
            return Err(crate::api::current::BuildError::invalid(
                "email",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        Ok(ModifyEmailAddress {
            user_id: self.user_id,
            email,
        })
    }
}

impl crate::api::current::support::CurrentRequest for ModifyEmailAddress {
    fn validate_current(&self) -> Result<(), crate::Error> {
        if self.email.is_empty() || self.email.trim() != self.email {
            return Err(crate::Error::InvalidRequest {
                field: "email",
                reason: "must be non-empty and have no surrounding whitespace",
            });
        }
        Ok(())
    }
}

/// Current wire model `ModifyPassword`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ModifyPassword {
    #[serde(rename = "userId", default, skip_serializing_if = "Option::is_none")]
    user_id: Option<crate::UserId>,
    #[serde(rename = "password")]
    password: crate::api::current::SecretValue,
    #[serde(rename = "currentPassword")]
    current_password: crate::api::current::SecretValue,
}

impl ModifyPassword {
    /// Returns wire field `userId`.
    #[must_use]
    pub fn user_id(&self) -> Option<&crate::UserId> {
        self.user_id.as_ref()
    }

    /// Reports whether secret field `password` is present.
    #[must_use]
    pub const fn has_password(&self) -> bool {
        true
    }

    /// Reports whether secret field `currentPassword` is present.
    #[must_use]
    pub const fn has_current_password(&self) -> bool {
        true
    }

    /// Starts a builder for [`ModifyPassword`].
    pub fn builder() -> ModifyPasswordBuilder {
        ModifyPasswordBuilder::default()
    }
}

/// Builder for [`ModifyPassword`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ModifyPasswordBuilder {
    user_id: Option<crate::UserId>,
    password: Option<crate::api::current::SecretValue>,
    current_password: Option<crate::api::current::SecretValue>,
}

impl ModifyPasswordBuilder {
    /// Sets wire field `userId`.
    pub fn user_id(mut self, value: crate::UserId) -> Self {
        self.user_id = Some(value);
        self
    }

    /// Sets wire field `password`.
    pub fn password(mut self, value: crate::api::current::SecretValue) -> Self {
        self.password = Some(value);
        self
    }

    /// Sets wire field `currentPassword`.
    pub fn current_password(mut self, value: crate::api::current::SecretValue) -> Self {
        self.current_password = Some(value);
        self
    }

    /// Validates required fields and builds [`ModifyPassword`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ModifyPassword, crate::api::current::BuildError> {
        let password = self
            .password
            .ok_or(crate::api::current::BuildError::missing("password"))?;
        let current_password = self
            .current_password
            .ok_or(crate::api::current::BuildError::missing("currentPassword"))?;
        Ok(ModifyPassword {
            user_id: self.user_id,
            password,
            current_password,
        })
    }
}

impl crate::api::current::support::CurrentRequest for ModifyPassword {
    fn validate_current(&self) -> Result<(), crate::Error> {
        Ok(())
    }
}

/// Current wire model `OpenDemoAccount`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct OpenDemoAccount {
    #[serde(
        rename = "templateAccountId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    template_account_id: Option<crate::AccountId>,
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(
        rename = "initialBalance",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    initial_balance: Option<crate::Decimal>,
    #[serde(
        rename = "defaultAutoLiq",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    default_auto_liq: Option<PostTradeRisk>,
    #[serde(
        rename = "preTradeRisk",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pre_trade_risk: Option<Vec<PreTradeRisk>>,
}

impl OpenDemoAccount {
    /// Returns wire field `templateAccountId`.
    #[must_use]
    pub fn template_account_id(&self) -> Option<&crate::AccountId> {
        self.template_account_id.as_ref()
    }

    /// Returns wire field `name`.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Returns wire field `initialBalance`.
    #[must_use]
    pub fn initial_balance(&self) -> Option<&crate::Decimal> {
        self.initial_balance.as_ref()
    }

    /// Returns wire field `defaultAutoLiq`.
    #[must_use]
    pub fn default_auto_liq(&self) -> Option<&PostTradeRisk> {
        self.default_auto_liq.as_ref()
    }

    /// Returns wire field `preTradeRisk`.
    #[must_use]
    pub fn pre_trade_risk(&self) -> Option<&[PreTradeRisk]> {
        self.pre_trade_risk.as_deref()
    }

    /// Starts a builder for [`OpenDemoAccount`].
    pub fn builder() -> OpenDemoAccountBuilder {
        OpenDemoAccountBuilder::default()
    }
}

/// Builder for [`OpenDemoAccount`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct OpenDemoAccountBuilder {
    template_account_id: Option<crate::AccountId>,
    name: Option<String>,
    initial_balance: Option<crate::Decimal>,
    default_auto_liq: Option<PostTradeRisk>,
    pre_trade_risk: Option<Vec<PreTradeRisk>>,
}

impl OpenDemoAccountBuilder {
    /// Sets wire field `templateAccountId`.
    pub fn template_account_id(mut self, value: crate::AccountId) -> Self {
        self.template_account_id = Some(value);
        self
    }

    /// Sets wire field `name`.
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Sets wire field `initialBalance`.
    pub fn initial_balance(mut self, value: crate::Decimal) -> Self {
        self.initial_balance = Some(value);
        self
    }

    /// Sets wire field `defaultAutoLiq`.
    pub fn default_auto_liq(mut self, value: PostTradeRisk) -> Self {
        self.default_auto_liq = Some(value);
        self
    }

    /// Sets wire field `preTradeRisk`.
    pub fn pre_trade_risk(mut self, value: Vec<PreTradeRisk>) -> Self {
        self.pre_trade_risk = Some(value);
        self
    }

    /// Validates required fields and builds [`OpenDemoAccount`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<OpenDemoAccount, crate::api::current::BuildError> {
        Ok(OpenDemoAccount {
            template_account_id: self.template_account_id,
            name: self.name,
            initial_balance: self.initial_balance,
            default_auto_liq: self.default_auto_liq,
            pre_trade_risk: self.pre_trade_risk,
        })
    }
}

impl crate::api::current::support::CurrentRequest for OpenDemoAccount {
    fn validate_current(&self) -> Result<(), crate::Error> {
        Ok(())
    }
}

/// Current wire model `OpenDemoAccountResponse`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct OpenDemoAccountResponse {
    #[serde(rename = "errorText", default, skip_serializing_if = "Option::is_none")]
    error_text: Option<String>,
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    account_id: Option<crate::AccountId>,
}

impl OpenDemoAccountResponse {
    /// Returns wire field `errorText`.
    #[must_use]
    pub fn error_text(&self) -> Option<&str> {
        self.error_text.as_deref()
    }

    /// Returns wire field `accountId`.
    #[must_use]
    pub fn account_id(&self) -> Option<&crate::AccountId> {
        self.account_id.as_ref()
    }

    /// Starts a builder for [`OpenDemoAccountResponse`].
    pub fn builder() -> OpenDemoAccountResponseBuilder {
        OpenDemoAccountResponseBuilder::default()
    }
}

/// Builder for [`OpenDemoAccountResponse`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct OpenDemoAccountResponseBuilder {
    error_text: Option<String>,
    account_id: Option<crate::AccountId>,
}

impl OpenDemoAccountResponseBuilder {
    /// Sets wire field `errorText`.
    pub fn error_text(mut self, value: impl Into<String>) -> Self {
        self.error_text = Some(value.into());
        self
    }

    /// Sets wire field `accountId`.
    pub fn account_id(mut self, value: crate::AccountId) -> Self {
        self.account_id = Some(value);
        self
    }

    /// Validates required fields and builds [`OpenDemoAccountResponse`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<OpenDemoAccountResponse, crate::api::current::BuildError> {
        Ok(OpenDemoAccountResponse {
            error_text: self.error_text,
            account_id: self.account_id,
        })
    }
}

/// Current wire model `Order`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct Order {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    id: Option<crate::OrderId>,
    #[serde(rename = "accountId")]
    account_id: crate::AccountId,
    #[serde(
        rename = "contractId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    contract_id: Option<crate::ContractId>,
    #[serde(
        rename = "spreadDefinitionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    spread_definition_id: Option<super::ids::SpreadDefinitionId>,
    #[serde(rename = "timestamp")]
    timestamp: jiff::Timestamp,
    #[serde(rename = "action")]
    action: OrderAction,
    #[serde(rename = "ordStatus")]
    ord_status: OrderOrdStatus,
    #[serde(
        rename = "executionProviderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    execution_provider_id: Option<super::ids::ExecutionProviderId>,
    #[serde(rename = "ocoId", default, skip_serializing_if = "Option::is_none")]
    oco_id: Option<super::ids::OcoId>,
    #[serde(rename = "parentId", default, skip_serializing_if = "Option::is_none")]
    parent_id: Option<crate::OrderId>,
    #[serde(rename = "linkedId", default, skip_serializing_if = "Option::is_none")]
    linked_id: Option<crate::OrderId>,
    #[serde(rename = "admin")]
    admin: bool,
}

impl Order {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> Option<&crate::OrderId> {
        self.id.as_ref()
    }

    /// Returns wire field `accountId`.
    #[must_use]
    pub fn account_id(&self) -> &crate::AccountId {
        &self.account_id
    }

    /// Returns wire field `contractId`.
    #[must_use]
    pub fn contract_id(&self) -> Option<&crate::ContractId> {
        self.contract_id.as_ref()
    }

    /// Returns wire field `spreadDefinitionId`.
    #[must_use]
    pub fn spread_definition_id(&self) -> Option<&super::ids::SpreadDefinitionId> {
        self.spread_definition_id.as_ref()
    }

    /// Returns wire field `timestamp`.
    #[must_use]
    pub fn timestamp(&self) -> &jiff::Timestamp {
        &self.timestamp
    }

    /// Returns wire field `action`.
    #[must_use]
    pub fn action(&self) -> &OrderAction {
        &self.action
    }

    /// Returns wire field `ordStatus`.
    #[must_use]
    pub fn ord_status(&self) -> &OrderOrdStatus {
        &self.ord_status
    }

    /// Returns wire field `executionProviderId`.
    #[must_use]
    pub fn execution_provider_id(&self) -> Option<&super::ids::ExecutionProviderId> {
        self.execution_provider_id.as_ref()
    }

    /// Returns wire field `ocoId`.
    #[must_use]
    pub fn oco_id(&self) -> Option<&super::ids::OcoId> {
        self.oco_id.as_ref()
    }

    /// Returns wire field `parentId`.
    #[must_use]
    pub fn parent_id(&self) -> Option<&crate::OrderId> {
        self.parent_id.as_ref()
    }

    /// Returns wire field `linkedId`.
    #[must_use]
    pub fn linked_id(&self) -> Option<&crate::OrderId> {
        self.linked_id.as_ref()
    }

    /// Returns wire field `admin`.
    #[must_use]
    pub fn admin(&self) -> &bool {
        &self.admin
    }

    /// Starts a builder for [`Order`].
    pub fn builder() -> OrderBuilder {
        OrderBuilder::default()
    }
}

/// Builder for [`Order`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct OrderBuilder {
    id: Option<crate::OrderId>,
    account_id: Option<crate::AccountId>,
    contract_id: Option<crate::ContractId>,
    spread_definition_id: Option<super::ids::SpreadDefinitionId>,
    timestamp: Option<jiff::Timestamp>,
    action: Option<OrderAction>,
    ord_status: Option<OrderOrdStatus>,
    execution_provider_id: Option<super::ids::ExecutionProviderId>,
    oco_id: Option<super::ids::OcoId>,
    parent_id: Option<crate::OrderId>,
    linked_id: Option<crate::OrderId>,
    admin: Option<bool>,
}

impl OrderBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: crate::OrderId) -> Self {
        self.id = Some(value);
        self
    }

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

    /// Sets wire field `spreadDefinitionId`.
    pub fn spread_definition_id(mut self, value: super::ids::SpreadDefinitionId) -> Self {
        self.spread_definition_id = Some(value);
        self
    }

    /// Sets wire field `timestamp`.
    pub fn timestamp(mut self, value: jiff::Timestamp) -> Self {
        self.timestamp = Some(value);
        self
    }

    /// Sets wire field `action`.
    pub fn action(mut self, value: OrderAction) -> Self {
        self.action = Some(value);
        self
    }

    /// Sets wire field `ordStatus`.
    pub fn ord_status(mut self, value: OrderOrdStatus) -> Self {
        self.ord_status = Some(value);
        self
    }

    /// Sets wire field `executionProviderId`.
    pub fn execution_provider_id(mut self, value: super::ids::ExecutionProviderId) -> Self {
        self.execution_provider_id = Some(value);
        self
    }

    /// Sets wire field `ocoId`.
    pub fn oco_id(mut self, value: super::ids::OcoId) -> Self {
        self.oco_id = Some(value);
        self
    }

    /// Sets wire field `parentId`.
    pub fn parent_id(mut self, value: crate::OrderId) -> Self {
        self.parent_id = Some(value);
        self
    }

    /// Sets wire field `linkedId`.
    pub fn linked_id(mut self, value: crate::OrderId) -> Self {
        self.linked_id = Some(value);
        self
    }

    /// Sets wire field `admin`.
    pub fn admin(mut self, value: bool) -> Self {
        self.admin = Some(value);
        self
    }

    /// Validates required fields and builds [`Order`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<Order, crate::api::current::BuildError> {
        let account_id = self
            .account_id
            .ok_or(crate::api::current::BuildError::missing("accountId"))?;
        let timestamp = self
            .timestamp
            .ok_or(crate::api::current::BuildError::missing("timestamp"))?;
        let action = self
            .action
            .ok_or(crate::api::current::BuildError::missing("action"))?;
        let ord_status = self
            .ord_status
            .ok_or(crate::api::current::BuildError::missing("ordStatus"))?;
        let admin = self
            .admin
            .ok_or(crate::api::current::BuildError::missing("admin"))?;
        Ok(Order {
            id: self.id,
            account_id,
            contract_id: self.contract_id,
            spread_definition_id: self.spread_definition_id,
            timestamp,
            action,
            ord_status,
            execution_provider_id: self.execution_provider_id,
            oco_id: self.oco_id,
            parent_id: self.parent_id,
            linked_id: self.linked_id,
            admin,
        })
    }
}

/// Current provider values for `OrderAction`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum OrderAction {
    /// Provider value `Buy`.
    Buy,
    /// Provider value `Sell`.
    Sell,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl OrderAction {
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

impl serde::Serialize for OrderAction {
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

impl<'de> serde::Deserialize<'de> for OrderAction {
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

/// Current provider values for `OrderOrdStatus`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum OrderOrdStatus {
    /// Provider value `Canceled`.
    Canceled,
    /// Provider value `Completed`.
    Completed,
    /// Provider value `Expired`.
    Expired,
    /// Provider value `Filled`.
    Filled,
    /// Provider value `PendingCancel`.
    PendingCancel,
    /// Provider value `PendingNew`.
    PendingNew,
    /// Provider value `PendingReplace`.
    PendingReplace,
    /// Provider value `Rejected`.
    Rejected,
    /// Provider value `Suspended`.
    Suspended,
    /// Provider value `Unknown`.
    Unknown2,
    /// Provider value `Working`.
    Working,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl OrderOrdStatus {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Canceled => "Canceled",
            Self::Completed => "Completed",
            Self::Expired => "Expired",
            Self::Filled => "Filled",
            Self::PendingCancel => "PendingCancel",
            Self::PendingNew => "PendingNew",
            Self::PendingReplace => "PendingReplace",
            Self::Rejected => "Rejected",
            Self::Suspended => "Suspended",
            Self::Unknown2 => "Unknown",
            Self::Working => "Working",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for OrderOrdStatus {
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

impl<'de> serde::Deserialize<'de> for OrderOrdStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "Canceled" => Self::Canceled,
            "Completed" => Self::Completed,
            "Expired" => Self::Expired,
            "Filled" => Self::Filled,
            "PendingCancel" => Self::PendingCancel,
            "PendingNew" => Self::PendingNew,
            "PendingReplace" => Self::PendingReplace,
            "Rejected" => Self::Rejected,
            "Suspended" => Self::Suspended,
            "Unknown" => Self::Unknown2,
            "Working" => Self::Working,
            _ => Self::Unknown(value),
        })
    }
}

/// Current wire model `OrderStrategy`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct OrderStrategy {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    id: Option<super::ids::OrderStrategyId>,
    #[serde(rename = "accountId")]
    account_id: crate::AccountId,
    #[serde(rename = "timestamp")]
    timestamp: jiff::Timestamp,
    #[serde(rename = "contractId")]
    contract_id: crate::ContractId,
    #[serde(rename = "orderStrategyTypeId")]
    order_strategy_type_id: super::ids::OrderStrategyTypeId,
    #[serde(
        rename = "initiatorId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    initiator_id: Option<super::ids::InitiatorId>,
    #[serde(rename = "action")]
    action: OrderStrategyAction,
    #[serde(rename = "params", default, skip_serializing_if = "Option::is_none")]
    params: Option<String>,
    #[serde(rename = "uuid", default, skip_serializing_if = "Option::is_none")]
    uuid: Option<String>,
    #[serde(rename = "status")]
    status: OrderStrategyStatus,
    #[serde(
        rename = "failureMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    failure_message: Option<String>,
    #[serde(rename = "senderId", default, skip_serializing_if = "Option::is_none")]
    sender_id: Option<super::ids::SenderId>,
    #[serde(
        rename = "customTag50",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    custom_tag50: Option<String>,
    #[serde(
        rename = "userSessionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    user_session_id: Option<super::ids::UserSessionId>,
}

impl OrderStrategy {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> Option<&super::ids::OrderStrategyId> {
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

    /// Returns wire field `contractId`.
    #[must_use]
    pub fn contract_id(&self) -> &crate::ContractId {
        &self.contract_id
    }

    /// Returns wire field `orderStrategyTypeId`.
    #[must_use]
    pub fn order_strategy_type_id(&self) -> &super::ids::OrderStrategyTypeId {
        &self.order_strategy_type_id
    }

    /// Returns wire field `initiatorId`.
    #[must_use]
    pub fn initiator_id(&self) -> Option<&super::ids::InitiatorId> {
        self.initiator_id.as_ref()
    }

    /// Returns wire field `action`.
    #[must_use]
    pub fn action(&self) -> &OrderStrategyAction {
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

    /// Returns wire field `status`.
    #[must_use]
    pub fn status(&self) -> &OrderStrategyStatus {
        &self.status
    }

    /// Returns wire field `failureMessage`.
    #[must_use]
    pub fn failure_message(&self) -> Option<&str> {
        self.failure_message.as_deref()
    }

    /// Returns wire field `senderId`.
    #[must_use]
    pub fn sender_id(&self) -> Option<&super::ids::SenderId> {
        self.sender_id.as_ref()
    }

    /// Returns wire field `customTag50`.
    #[must_use]
    pub fn custom_tag50(&self) -> Option<&str> {
        self.custom_tag50.as_deref()
    }

    /// Returns wire field `userSessionId`.
    #[must_use]
    pub fn user_session_id(&self) -> Option<&super::ids::UserSessionId> {
        self.user_session_id.as_ref()
    }

    /// Starts a builder for [`OrderStrategy`].
    pub fn builder() -> OrderStrategyBuilder {
        OrderStrategyBuilder::default()
    }
}

/// Builder for [`OrderStrategy`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct OrderStrategyBuilder {
    id: Option<super::ids::OrderStrategyId>,
    account_id: Option<crate::AccountId>,
    timestamp: Option<jiff::Timestamp>,
    contract_id: Option<crate::ContractId>,
    order_strategy_type_id: Option<super::ids::OrderStrategyTypeId>,
    initiator_id: Option<super::ids::InitiatorId>,
    action: Option<OrderStrategyAction>,
    params: Option<String>,
    uuid: Option<String>,
    status: Option<OrderStrategyStatus>,
    failure_message: Option<String>,
    sender_id: Option<super::ids::SenderId>,
    custom_tag50: Option<String>,
    user_session_id: Option<super::ids::UserSessionId>,
}

impl OrderStrategyBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::OrderStrategyId) -> Self {
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

    /// Sets wire field `contractId`.
    pub fn contract_id(mut self, value: crate::ContractId) -> Self {
        self.contract_id = Some(value);
        self
    }

    /// Sets wire field `orderStrategyTypeId`.
    pub fn order_strategy_type_id(mut self, value: super::ids::OrderStrategyTypeId) -> Self {
        self.order_strategy_type_id = Some(value);
        self
    }

    /// Sets wire field `initiatorId`.
    pub fn initiator_id(mut self, value: super::ids::InitiatorId) -> Self {
        self.initiator_id = Some(value);
        self
    }

    /// Sets wire field `action`.
    pub fn action(mut self, value: OrderStrategyAction) -> Self {
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

    /// Sets wire field `status`.
    pub fn status(mut self, value: OrderStrategyStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Sets wire field `failureMessage`.
    pub fn failure_message(mut self, value: impl Into<String>) -> Self {
        self.failure_message = Some(value.into());
        self
    }

    /// Sets wire field `senderId`.
    pub fn sender_id(mut self, value: super::ids::SenderId) -> Self {
        self.sender_id = Some(value);
        self
    }

    /// Sets wire field `customTag50`.
    pub fn custom_tag50(mut self, value: impl Into<String>) -> Self {
        self.custom_tag50 = Some(value.into());
        self
    }

    /// Sets wire field `userSessionId`.
    pub fn user_session_id(mut self, value: super::ids::UserSessionId) -> Self {
        self.user_session_id = Some(value);
        self
    }

    /// Validates required fields and builds [`OrderStrategy`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<OrderStrategy, crate::api::current::BuildError> {
        let account_id = self
            .account_id
            .ok_or(crate::api::current::BuildError::missing("accountId"))?;
        let timestamp = self
            .timestamp
            .ok_or(crate::api::current::BuildError::missing("timestamp"))?;
        let contract_id = self
            .contract_id
            .ok_or(crate::api::current::BuildError::missing("contractId"))?;
        let order_strategy_type_id =
            self.order_strategy_type_id
                .ok_or(crate::api::current::BuildError::missing(
                    "orderStrategyTypeId",
                ))?;
        let action = self
            .action
            .ok_or(crate::api::current::BuildError::missing("action"))?;
        let status = self
            .status
            .ok_or(crate::api::current::BuildError::missing("status"))?;
        Ok(OrderStrategy {
            id: self.id,
            account_id,
            timestamp,
            contract_id,
            order_strategy_type_id,
            initiator_id: self.initiator_id,
            action,
            params: self.params,
            uuid: self.uuid,
            status,
            failure_message: self.failure_message,
            sender_id: self.sender_id,
            custom_tag50: self.custom_tag50,
            user_session_id: self.user_session_id,
        })
    }
}

/// Current provider values for `OrderStrategyAction`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum OrderStrategyAction {
    /// Provider value `Buy`.
    Buy,
    /// Provider value `Sell`.
    Sell,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl OrderStrategyAction {
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

impl serde::Serialize for OrderStrategyAction {
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

impl<'de> serde::Deserialize<'de> for OrderStrategyAction {
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

/// Current wire model `OrderStrategyLink`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct OrderStrategyLink {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    id: Option<super::ids::OrderStrategyLinkId>,
    #[serde(rename = "orderStrategyId")]
    order_strategy_id: super::ids::OrderStrategyId,
    #[serde(rename = "orderId")]
    order_id: crate::OrderId,
    #[serde(rename = "label")]
    label: String,
}

impl OrderStrategyLink {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> Option<&super::ids::OrderStrategyLinkId> {
        self.id.as_ref()
    }

    /// Returns wire field `orderStrategyId`.
    #[must_use]
    pub fn order_strategy_id(&self) -> &super::ids::OrderStrategyId {
        &self.order_strategy_id
    }

    /// Returns wire field `orderId`.
    #[must_use]
    pub fn order_id(&self) -> &crate::OrderId {
        &self.order_id
    }

    /// Returns wire field `label`.
    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Starts a builder for [`OrderStrategyLink`].
    pub fn builder() -> OrderStrategyLinkBuilder {
        OrderStrategyLinkBuilder::default()
    }
}

/// Builder for [`OrderStrategyLink`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct OrderStrategyLinkBuilder {
    id: Option<super::ids::OrderStrategyLinkId>,
    order_strategy_id: Option<super::ids::OrderStrategyId>,
    order_id: Option<crate::OrderId>,
    label: Option<String>,
}

impl OrderStrategyLinkBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::OrderStrategyLinkId) -> Self {
        self.id = Some(value);
        self
    }

    /// Sets wire field `orderStrategyId`.
    pub fn order_strategy_id(mut self, value: super::ids::OrderStrategyId) -> Self {
        self.order_strategy_id = Some(value);
        self
    }

    /// Sets wire field `orderId`.
    pub fn order_id(mut self, value: crate::OrderId) -> Self {
        self.order_id = Some(value);
        self
    }

    /// Sets wire field `label`.
    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    /// Validates required fields and builds [`OrderStrategyLink`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<OrderStrategyLink, crate::api::current::BuildError> {
        let order_strategy_id = self
            .order_strategy_id
            .ok_or(crate::api::current::BuildError::missing("orderStrategyId"))?;
        let order_id = self
            .order_id
            .ok_or(crate::api::current::BuildError::missing("orderId"))?;
        let label = self
            .label
            .ok_or(crate::api::current::BuildError::missing("label"))?;
        Ok(OrderStrategyLink {
            id: self.id,
            order_strategy_id,
            order_id,
            label,
        })
    }
}

/// Current provider values for `OrderStrategyStatus`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum OrderStrategyStatus {
    /// Provider value `ActiveStrategy`.
    ActiveStrategy,
    /// Provider value `ExecutionFailed`.
    ExecutionFailed,
    /// Provider value `ExecutionFinished`.
    ExecutionFinished,
    /// Provider value `ExecutionInterrupted`.
    ExecutionInterrupted,
    /// Provider value `InactiveStrategy`.
    InactiveStrategy,
    /// Provider value `NotEnoughLiquidity`.
    NotEnoughLiquidity,
    /// Provider value `StoppedByUser`.
    StoppedByUser,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl OrderStrategyStatus {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::ActiveStrategy => "ActiveStrategy",
            Self::ExecutionFailed => "ExecutionFailed",
            Self::ExecutionFinished => "ExecutionFinished",
            Self::ExecutionInterrupted => "ExecutionInterrupted",
            Self::InactiveStrategy => "InactiveStrategy",
            Self::NotEnoughLiquidity => "NotEnoughLiquidity",
            Self::StoppedByUser => "StoppedByUser",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for OrderStrategyStatus {
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

impl<'de> serde::Deserialize<'de> for OrderStrategyStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "ActiveStrategy" => Self::ActiveStrategy,
            "ExecutionFailed" => Self::ExecutionFailed,
            "ExecutionFinished" => Self::ExecutionFinished,
            "ExecutionInterrupted" => Self::ExecutionInterrupted,
            "InactiveStrategy" => Self::InactiveStrategy,
            "NotEnoughLiquidity" => Self::NotEnoughLiquidity,
            "StoppedByUser" => Self::StoppedByUser,
            _ => Self::Unknown(value),
        })
    }
}

/// Current wire model `OrderStrategyType`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct OrderStrategyType {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    id: Option<super::ids::OrderStrategyTypeId>,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "enabled")]
    enabled: bool,
}

impl OrderStrategyType {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> Option<&super::ids::OrderStrategyTypeId> {
        self.id.as_ref()
    }

    /// Returns wire field `name`.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns wire field `enabled`.
    #[must_use]
    pub fn enabled(&self) -> &bool {
        &self.enabled
    }

    /// Starts a builder for [`OrderStrategyType`].
    pub fn builder() -> OrderStrategyTypeBuilder {
        OrderStrategyTypeBuilder::default()
    }
}

/// Builder for [`OrderStrategyType`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct OrderStrategyTypeBuilder {
    id: Option<super::ids::OrderStrategyTypeId>,
    name: Option<String>,
    enabled: Option<bool>,
}

impl OrderStrategyTypeBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::OrderStrategyTypeId) -> Self {
        self.id = Some(value);
        self
    }

    /// Sets wire field `name`.
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Sets wire field `enabled`.
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    /// Validates required fields and builds [`OrderStrategyType`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<OrderStrategyType, crate::api::current::BuildError> {
        let name = self
            .name
            .ok_or(crate::api::current::BuildError::missing("name"))?;
        let enabled = self
            .enabled
            .ok_or(crate::api::current::BuildError::missing("enabled"))?;
        Ok(OrderStrategyType {
            id: self.id,
            name,
            enabled,
        })
    }
}

/// Current wire model `OrderVersion`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct OrderVersion {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    id: Option<super::ids::OrderVersionId>,
    #[serde(rename = "orderId")]
    order_id: crate::OrderId,
    #[serde(rename = "orderQty")]
    order_qty: i64,
    #[serde(rename = "orderType")]
    order_type: OrderVersionOrderType,
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
    time_in_force: Option<OrderVersionTimeInForce>,
    #[serde(
        rename = "expireTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    expire_time: Option<jiff::Timestamp>,
    #[serde(rename = "text", default, skip_serializing_if = "Option::is_none")]
    text: Option<String>,
}

impl OrderVersion {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> Option<&super::ids::OrderVersionId> {
        self.id.as_ref()
    }

    /// Returns wire field `orderId`.
    #[must_use]
    pub fn order_id(&self) -> &crate::OrderId {
        &self.order_id
    }

    /// Returns wire field `orderQty`.
    #[must_use]
    pub fn order_qty(&self) -> &i64 {
        &self.order_qty
    }

    /// Returns wire field `orderType`.
    #[must_use]
    pub fn order_type(&self) -> &OrderVersionOrderType {
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
    pub fn time_in_force(&self) -> Option<&OrderVersionTimeInForce> {
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

    /// Starts a builder for [`OrderVersion`].
    pub fn builder() -> OrderVersionBuilder {
        OrderVersionBuilder::default()
    }
}

/// Builder for [`OrderVersion`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct OrderVersionBuilder {
    id: Option<super::ids::OrderVersionId>,
    order_id: Option<crate::OrderId>,
    order_qty: Option<i64>,
    order_type: Option<OrderVersionOrderType>,
    price: Option<crate::Decimal>,
    stop_price: Option<crate::Decimal>,
    limit_if_touched_price: Option<crate::Decimal>,
    max_show: Option<i64>,
    peg_difference: Option<crate::Decimal>,
    time_in_force: Option<OrderVersionTimeInForce>,
    expire_time: Option<jiff::Timestamp>,
    text: Option<String>,
}

impl OrderVersionBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::OrderVersionId) -> Self {
        self.id = Some(value);
        self
    }

    /// Sets wire field `orderId`.
    pub fn order_id(mut self, value: crate::OrderId) -> Self {
        self.order_id = Some(value);
        self
    }

    /// Sets wire field `orderQty`.
    pub fn order_qty(mut self, value: i64) -> Self {
        self.order_qty = Some(value);
        self
    }

    /// Sets wire field `orderType`.
    pub fn order_type(mut self, value: OrderVersionOrderType) -> Self {
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
    pub fn time_in_force(mut self, value: OrderVersionTimeInForce) -> Self {
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

    /// Validates required fields and builds [`OrderVersion`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<OrderVersion, crate::api::current::BuildError> {
        let order_id = self
            .order_id
            .ok_or(crate::api::current::BuildError::missing("orderId"))?;
        let order_qty = self
            .order_qty
            .ok_or(crate::api::current::BuildError::missing("orderQty"))?;
        let order_type = self
            .order_type
            .ok_or(crate::api::current::BuildError::missing("orderType"))?;
        Ok(OrderVersion {
            id: self.id,
            order_id,
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
        })
    }
}

/// Current provider values for `OrderVersionOrderType`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum OrderVersionOrderType {
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

impl OrderVersionOrderType {
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

impl serde::Serialize for OrderVersionOrderType {
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

impl<'de> serde::Deserialize<'de> for OrderVersionOrderType {
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

/// Current provider values for `OrderVersionTimeInForce`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum OrderVersionTimeInForce {
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

impl OrderVersionTimeInForce {
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

impl serde::Serialize for OrderVersionTimeInForce {
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

impl<'de> serde::Deserialize<'de> for OrderVersionTimeInForce {
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

/// Current wire model `OrgWorkspaceTemplateResponse`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct OrgWorkspaceTemplateResponse {
    #[serde(rename = "errorText", default, skip_serializing_if = "Option::is_none")]
    error_text: Option<String>,
    #[serde(
        rename = "workspaceTemplate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    workspace_template: Option<WorkspaceTemplate>,
}

impl OrgWorkspaceTemplateResponse {
    /// Returns wire field `errorText`.
    #[must_use]
    pub fn error_text(&self) -> Option<&str> {
        self.error_text.as_deref()
    }

    /// Returns wire field `workspaceTemplate`.
    #[must_use]
    pub fn workspace_template(&self) -> Option<&WorkspaceTemplate> {
        self.workspace_template.as_ref()
    }

    /// Starts a builder for [`OrgWorkspaceTemplateResponse`].
    pub fn builder() -> OrgWorkspaceTemplateResponseBuilder {
        OrgWorkspaceTemplateResponseBuilder::default()
    }
}

/// Builder for [`OrgWorkspaceTemplateResponse`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct OrgWorkspaceTemplateResponseBuilder {
    error_text: Option<String>,
    workspace_template: Option<WorkspaceTemplate>,
}

impl OrgWorkspaceTemplateResponseBuilder {
    /// Sets wire field `errorText`.
    pub fn error_text(mut self, value: impl Into<String>) -> Self {
        self.error_text = Some(value.into());
        self
    }

    /// Sets wire field `workspaceTemplate`.
    pub fn workspace_template(mut self, value: WorkspaceTemplate) -> Self {
        self.workspace_template = Some(value);
        self
    }

    /// Validates required fields and builds [`OrgWorkspaceTemplateResponse`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<OrgWorkspaceTemplateResponse, crate::api::current::BuildError> {
        Ok(OrgWorkspaceTemplateResponse {
            error_text: self.error_text,
            workspace_template: self.workspace_template,
        })
    }
}

/// Current wire model `Organization`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct Organization {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    id: Option<super::ids::OrganizationId>,
    #[serde(rename = "name")]
    name: String,
}

impl Organization {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> Option<&super::ids::OrganizationId> {
        self.id.as_ref()
    }

    /// Returns wire field `name`.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Starts a builder for [`Organization`].
    pub fn builder() -> OrganizationBuilder {
        OrganizationBuilder::default()
    }
}

/// Builder for [`Organization`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct OrganizationBuilder {
    id: Option<super::ids::OrganizationId>,
    name: Option<String>,
}

impl OrganizationBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::OrganizationId) -> Self {
        self.id = Some(value);
        self
    }

    /// Sets wire field `name`.
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Validates required fields and builds [`Organization`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<Organization, crate::api::current::BuildError> {
        let name = self
            .name
            .ok_or(crate::api::current::BuildError::missing("name"))?;
        Ok(Organization { id: self.id, name })
    }
}

/// Current wire model `POAContact`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct POAContact {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    id: Option<super::ids::PoaContactId>,
    #[serde(rename = "timestamp")]
    timestamp: jiff::Timestamp,
    #[serde(rename = "userId")]
    user_id: crate::UserId,
    #[serde(rename = "firstName")]
    first_name: String,
    #[serde(rename = "lastName")]
    last_name: String,
    #[serde(rename = "country")]
    country: String,
    #[serde(rename = "state")]
    state: String,
    #[serde(rename = "streetAddress1")]
    street_address1: String,
    #[serde(
        rename = "streetAddress2",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    street_address2: Option<String>,
    #[serde(rename = "city")]
    city: String,
    #[serde(rename = "zipCode")]
    zip_code: String,
    #[serde(rename = "phone")]
    phone: String,
    #[serde(rename = "citizenship")]
    citizenship: String,
    #[serde(
        rename = "taxIdentifier",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    tax_identifier: Option<crate::api::current::SecretValue>,
    #[serde(
        rename = "nationalId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    national_id: Option<crate::api::current::SecretValue>,
    #[serde(rename = "birthDate")]
    birth_date: TradeDate,
    #[serde(rename = "organizationId")]
    organization_id: super::ids::OrganizationId,
}

impl POAContact {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> Option<&super::ids::PoaContactId> {
        self.id.as_ref()
    }

    /// Returns wire field `timestamp`.
    #[must_use]
    pub fn timestamp(&self) -> &jiff::Timestamp {
        &self.timestamp
    }

    /// Returns wire field `userId`.
    #[must_use]
    pub fn user_id(&self) -> &crate::UserId {
        &self.user_id
    }

    /// Returns wire field `firstName`.
    #[must_use]
    pub fn first_name(&self) -> &str {
        &self.first_name
    }

    /// Returns wire field `lastName`.
    #[must_use]
    pub fn last_name(&self) -> &str {
        &self.last_name
    }

    /// Returns wire field `country`.
    #[must_use]
    pub fn country(&self) -> &str {
        &self.country
    }

    /// Returns wire field `state`.
    #[must_use]
    pub fn state(&self) -> &str {
        &self.state
    }

    /// Returns wire field `streetAddress1`.
    #[must_use]
    pub fn street_address1(&self) -> &str {
        &self.street_address1
    }

    /// Returns wire field `streetAddress2`.
    #[must_use]
    pub fn street_address2(&self) -> Option<&str> {
        self.street_address2.as_deref()
    }

    /// Returns wire field `city`.
    #[must_use]
    pub fn city(&self) -> &str {
        &self.city
    }

    /// Returns wire field `zipCode`.
    #[must_use]
    pub fn zip_code(&self) -> &str {
        &self.zip_code
    }

    /// Returns wire field `phone`.
    #[must_use]
    pub fn phone(&self) -> &str {
        &self.phone
    }

    /// Returns wire field `citizenship`.
    #[must_use]
    pub fn citizenship(&self) -> &str {
        &self.citizenship
    }

    /// Reports whether secret field `taxIdentifier` is present.
    #[must_use]
    pub const fn has_tax_identifier(&self) -> bool {
        self.tax_identifier.is_some()
    }

    pub(crate) fn tax_identifier_secret(&self) -> Option<&crate::api::current::SecretValue> {
        self.tax_identifier.as_ref()
    }

    /// Reports whether secret field `nationalId` is present.
    #[must_use]
    pub const fn has_national_id(&self) -> bool {
        self.national_id.is_some()
    }

    pub(crate) fn national_id_secret(&self) -> Option<&crate::api::current::SecretValue> {
        self.national_id.as_ref()
    }

    /// Returns wire field `birthDate`.
    #[must_use]
    pub fn birth_date(&self) -> &TradeDate {
        &self.birth_date
    }

    /// Returns wire field `organizationId`.
    #[must_use]
    pub fn organization_id(&self) -> &super::ids::OrganizationId {
        &self.organization_id
    }

    /// Starts a builder for [`POAContact`].
    pub fn builder() -> POAContactBuilder {
        POAContactBuilder::default()
    }
}

/// Builder for [`POAContact`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct POAContactBuilder {
    id: Option<super::ids::PoaContactId>,
    timestamp: Option<jiff::Timestamp>,
    user_id: Option<crate::UserId>,
    first_name: Option<String>,
    last_name: Option<String>,
    country: Option<String>,
    state: Option<String>,
    street_address1: Option<String>,
    street_address2: Option<String>,
    city: Option<String>,
    zip_code: Option<String>,
    phone: Option<String>,
    citizenship: Option<String>,
    tax_identifier: Option<crate::api::current::SecretValue>,
    national_id: Option<crate::api::current::SecretValue>,
    birth_date: Option<TradeDate>,
    organization_id: Option<super::ids::OrganizationId>,
}

impl POAContactBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::PoaContactId) -> Self {
        self.id = Some(value);
        self
    }

    /// Sets wire field `timestamp`.
    pub fn timestamp(mut self, value: jiff::Timestamp) -> Self {
        self.timestamp = Some(value);
        self
    }

    /// Sets wire field `userId`.
    pub fn user_id(mut self, value: crate::UserId) -> Self {
        self.user_id = Some(value);
        self
    }

    /// Sets wire field `firstName`.
    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    /// Sets wire field `lastName`.
    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
        self
    }

    /// Sets wire field `country`.
    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    /// Sets wire field `state`.
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    /// Sets wire field `streetAddress1`.
    pub fn street_address1(mut self, value: impl Into<String>) -> Self {
        self.street_address1 = Some(value.into());
        self
    }

    /// Sets wire field `streetAddress2`.
    pub fn street_address2(mut self, value: impl Into<String>) -> Self {
        self.street_address2 = Some(value.into());
        self
    }

    /// Sets wire field `city`.
    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    /// Sets wire field `zipCode`.
    pub fn zip_code(mut self, value: impl Into<String>) -> Self {
        self.zip_code = Some(value.into());
        self
    }

    /// Sets wire field `phone`.
    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    /// Sets wire field `citizenship`.
    pub fn citizenship(mut self, value: impl Into<String>) -> Self {
        self.citizenship = Some(value.into());
        self
    }

    /// Sets wire field `taxIdentifier`.
    pub fn tax_identifier(mut self, value: crate::api::current::SecretValue) -> Self {
        self.tax_identifier = Some(value);
        self
    }

    /// Sets wire field `nationalId`.
    pub fn national_id(mut self, value: crate::api::current::SecretValue) -> Self {
        self.national_id = Some(value);
        self
    }

    /// Sets wire field `birthDate`.
    pub fn birth_date(mut self, value: TradeDate) -> Self {
        self.birth_date = Some(value);
        self
    }

    /// Sets wire field `organizationId`.
    pub fn organization_id(mut self, value: super::ids::OrganizationId) -> Self {
        self.organization_id = Some(value);
        self
    }

    /// Validates required fields and builds [`POAContact`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<POAContact, crate::api::current::BuildError> {
        let timestamp = self
            .timestamp
            .ok_or(crate::api::current::BuildError::missing("timestamp"))?;
        let user_id = self
            .user_id
            .ok_or(crate::api::current::BuildError::missing("userId"))?;
        let first_name = self
            .first_name
            .ok_or(crate::api::current::BuildError::missing("firstName"))?;
        if first_name.is_empty() || first_name.trim() != first_name {
            return Err(crate::api::current::BuildError::invalid(
                "firstName",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        let last_name = self
            .last_name
            .ok_or(crate::api::current::BuildError::missing("lastName"))?;
        if last_name.is_empty() || last_name.trim() != last_name {
            return Err(crate::api::current::BuildError::invalid(
                "lastName",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        let country = self
            .country
            .ok_or(crate::api::current::BuildError::missing("country"))?;
        if country.is_empty() || country.trim() != country {
            return Err(crate::api::current::BuildError::invalid(
                "country",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        let state = self
            .state
            .ok_or(crate::api::current::BuildError::missing("state"))?;
        if state.is_empty() || state.trim() != state {
            return Err(crate::api::current::BuildError::invalid(
                "state",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        let street_address1 = self
            .street_address1
            .ok_or(crate::api::current::BuildError::missing("streetAddress1"))?;
        if street_address1.is_empty() || street_address1.trim() != street_address1 {
            return Err(crate::api::current::BuildError::invalid(
                "streetAddress1",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        let city = self
            .city
            .ok_or(crate::api::current::BuildError::missing("city"))?;
        if city.is_empty() || city.trim() != city {
            return Err(crate::api::current::BuildError::invalid(
                "city",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        let zip_code = self
            .zip_code
            .ok_or(crate::api::current::BuildError::missing("zipCode"))?;
        if zip_code.is_empty() || zip_code.trim() != zip_code {
            return Err(crate::api::current::BuildError::invalid(
                "zipCode",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        let phone = self
            .phone
            .ok_or(crate::api::current::BuildError::missing("phone"))?;
        if phone.is_empty() || phone.trim() != phone {
            return Err(crate::api::current::BuildError::invalid(
                "phone",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        let citizenship = self
            .citizenship
            .ok_or(crate::api::current::BuildError::missing("citizenship"))?;
        if citizenship.is_empty() || citizenship.trim() != citizenship {
            return Err(crate::api::current::BuildError::invalid(
                "citizenship",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        let birth_date = self
            .birth_date
            .ok_or(crate::api::current::BuildError::missing("birthDate"))?;
        let organization_id = self
            .organization_id
            .ok_or(crate::api::current::BuildError::missing("organizationId"))?;
        Ok(POAContact {
            id: self.id,
            timestamp,
            user_id,
            first_name,
            last_name,
            country,
            state,
            street_address1,
            street_address2: self.street_address2,
            city,
            zip_code,
            phone,
            citizenship,
            tax_identifier: self.tax_identifier,
            national_id: self.national_id,
            birth_date,
            organization_id,
        })
    }
}

impl crate::api::current::support::CurrentRequest for POAContact {
    fn validate_current(&self) -> Result<(), crate::Error> {
        if self.citizenship.is_empty() || self.citizenship.trim() != self.citizenship {
            return Err(crate::Error::InvalidRequest {
                field: "citizenship",
                reason: "must be non-empty and have no surrounding whitespace",
            });
        }
        if self.city.is_empty() || self.city.trim() != self.city {
            return Err(crate::Error::InvalidRequest {
                field: "city",
                reason: "must be non-empty and have no surrounding whitespace",
            });
        }
        if self.country.is_empty() || self.country.trim() != self.country {
            return Err(crate::Error::InvalidRequest {
                field: "country",
                reason: "must be non-empty and have no surrounding whitespace",
            });
        }
        if self.first_name.is_empty() || self.first_name.trim() != self.first_name {
            return Err(crate::Error::InvalidRequest {
                field: "firstName",
                reason: "must be non-empty and have no surrounding whitespace",
            });
        }
        if self.last_name.is_empty() || self.last_name.trim() != self.last_name {
            return Err(crate::Error::InvalidRequest {
                field: "lastName",
                reason: "must be non-empty and have no surrounding whitespace",
            });
        }
        if self.phone.is_empty() || self.phone.trim() != self.phone {
            return Err(crate::Error::InvalidRequest {
                field: "phone",
                reason: "must be non-empty and have no surrounding whitespace",
            });
        }
        if self.state.is_empty() || self.state.trim() != self.state {
            return Err(crate::Error::InvalidRequest {
                field: "state",
                reason: "must be non-empty and have no surrounding whitespace",
            });
        }
        if self.street_address1.is_empty() || self.street_address1.trim() != self.street_address1 {
            return Err(crate::Error::InvalidRequest {
                field: "streetAddress1",
                reason: "must be non-empty and have no surrounding whitespace",
            });
        }
        if self.zip_code.is_empty() || self.zip_code.trim() != self.zip_code {
            return Err(crate::Error::InvalidRequest {
                field: "zipCode",
                reason: "must be non-empty and have no surrounding whitespace",
            });
        }
        Ok(())
    }
}

/// Current wire model `PartnerSubAccountRequestStatusResponse`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct PartnerSubAccountRequestStatusResponse {
    #[serde(rename = "errorText", default, skip_serializing_if = "Option::is_none")]
    error_text: Option<String>,
    #[serde(rename = "requestId", default, skip_serializing_if = "Option::is_none")]
    request_id: Option<super::ids::SubAccountRequestId>,
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    status: Option<PartnerSubAccountRequestStatusResponseStatus>,
    #[serde(rename = "message", default, skip_serializing_if = "Option::is_none")]
    message: Option<String>,
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    account_id: Option<crate::AccountId>,
}

impl PartnerSubAccountRequestStatusResponse {
    /// Returns wire field `errorText`.
    #[must_use]
    pub fn error_text(&self) -> Option<&str> {
        self.error_text.as_deref()
    }

    /// Returns wire field `requestId`.
    #[must_use]
    pub fn request_id(&self) -> Option<&super::ids::SubAccountRequestId> {
        self.request_id.as_ref()
    }

    /// Returns wire field `status`.
    #[must_use]
    pub fn status(&self) -> Option<&PartnerSubAccountRequestStatusResponseStatus> {
        self.status.as_ref()
    }

    /// Returns wire field `message`.
    #[must_use]
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }

    /// Returns wire field `accountId`.
    #[must_use]
    pub fn account_id(&self) -> Option<&crate::AccountId> {
        self.account_id.as_ref()
    }

    /// Starts a builder for [`PartnerSubAccountRequestStatusResponse`].
    pub fn builder() -> PartnerSubAccountRequestStatusResponseBuilder {
        PartnerSubAccountRequestStatusResponseBuilder::default()
    }
}

/// Builder for [`PartnerSubAccountRequestStatusResponse`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct PartnerSubAccountRequestStatusResponseBuilder {
    error_text: Option<String>,
    request_id: Option<super::ids::SubAccountRequestId>,
    status: Option<PartnerSubAccountRequestStatusResponseStatus>,
    message: Option<String>,
    account_id: Option<crate::AccountId>,
}

impl PartnerSubAccountRequestStatusResponseBuilder {
    /// Sets wire field `errorText`.
    pub fn error_text(mut self, value: impl Into<String>) -> Self {
        self.error_text = Some(value.into());
        self
    }

    /// Sets wire field `requestId`.
    pub fn request_id(mut self, value: super::ids::SubAccountRequestId) -> Self {
        self.request_id = Some(value);
        self
    }

    /// Sets wire field `status`.
    pub fn status(mut self, value: PartnerSubAccountRequestStatusResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Sets wire field `message`.
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Sets wire field `accountId`.
    pub fn account_id(mut self, value: crate::AccountId) -> Self {
        self.account_id = Some(value);
        self
    }

    /// Validates required fields and builds [`PartnerSubAccountRequestStatusResponse`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(
        self,
    ) -> Result<PartnerSubAccountRequestStatusResponse, crate::api::current::BuildError> {
        Ok(PartnerSubAccountRequestStatusResponse {
            error_text: self.error_text,
            request_id: self.request_id,
            status: self.status,
            message: self.message,
            account_id: self.account_id,
        })
    }
}

/// Current provider values for `PartnerSubAccountRequestStatusResponseStatus`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum PartnerSubAccountRequestStatusResponseStatus {
    /// Provider value `Approved`.
    Approved,
    /// Provider value `Denied`.
    Denied,
    /// Provider value `InAMLReview`.
    InAmlReview,
    /// Provider value `InReview`.
    InReview,
    /// Provider value `Pending`.
    Pending,
    /// Provider value `Preapproved`.
    Preapproved,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl PartnerSubAccountRequestStatusResponseStatus {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Approved => "Approved",
            Self::Denied => "Denied",
            Self::InAmlReview => "InAMLReview",
            Self::InReview => "InReview",
            Self::Pending => "Pending",
            Self::Preapproved => "Preapproved",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for PartnerSubAccountRequestStatusResponseStatus {
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

impl<'de> serde::Deserialize<'de> for PartnerSubAccountRequestStatusResponseStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "Approved" => Self::Approved,
            "Denied" => Self::Denied,
            "InAMLReview" => Self::InAmlReview,
            "InReview" => Self::InReview,
            "Pending" => Self::Pending,
            "Preapproved" => Self::Preapproved,
            _ => Self::Unknown(value),
        })
    }
}

/// Current wire model `Position`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct Position {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    id: Option<crate::PositionId>,
    #[serde(rename = "accountId")]
    account_id: crate::AccountId,
    #[serde(rename = "contractId")]
    contract_id: crate::ContractId,
    #[serde(rename = "timestamp")]
    timestamp: jiff::Timestamp,
    #[serde(rename = "tradeDate")]
    trade_date: TradeDate,
    #[serde(rename = "netPos")]
    net_pos: i64,
    #[serde(rename = "netPrice", default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "crate::decimal::option")]
    net_price: Option<crate::Decimal>,
    #[serde(rename = "bought")]
    bought: i64,
    #[serde(rename = "boughtValue")]
    #[serde(with = "crate::decimal")]
    bought_value: crate::Decimal,
    #[serde(rename = "sold")]
    sold: i64,
    #[serde(rename = "soldValue")]
    #[serde(with = "crate::decimal")]
    sold_value: crate::Decimal,
    #[serde(rename = "prevPos")]
    prev_pos: i64,
    #[serde(rename = "prevPrice", default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "crate::decimal::option")]
    prev_price: Option<crate::Decimal>,
}

impl Position {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> Option<&crate::PositionId> {
        self.id.as_ref()
    }

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

    /// Returns wire field `timestamp`.
    #[must_use]
    pub fn timestamp(&self) -> &jiff::Timestamp {
        &self.timestamp
    }

    /// Returns wire field `tradeDate`.
    #[must_use]
    pub fn trade_date(&self) -> &TradeDate {
        &self.trade_date
    }

    /// Returns wire field `netPos`.
    #[must_use]
    pub fn net_pos(&self) -> &i64 {
        &self.net_pos
    }

    /// Returns wire field `netPrice`.
    #[must_use]
    pub fn net_price(&self) -> Option<&crate::Decimal> {
        self.net_price.as_ref()
    }

    /// Returns wire field `bought`.
    #[must_use]
    pub fn bought(&self) -> &i64 {
        &self.bought
    }

    /// Returns wire field `boughtValue`.
    #[must_use]
    pub fn bought_value(&self) -> &crate::Decimal {
        &self.bought_value
    }

    /// Returns wire field `sold`.
    #[must_use]
    pub fn sold(&self) -> &i64 {
        &self.sold
    }

    /// Returns wire field `soldValue`.
    #[must_use]
    pub fn sold_value(&self) -> &crate::Decimal {
        &self.sold_value
    }

    /// Returns wire field `prevPos`.
    #[must_use]
    pub fn prev_pos(&self) -> &i64 {
        &self.prev_pos
    }

    /// Returns wire field `prevPrice`.
    #[must_use]
    pub fn prev_price(&self) -> Option<&crate::Decimal> {
        self.prev_price.as_ref()
    }

    /// Starts a builder for [`Position`].
    pub fn builder() -> PositionBuilder {
        PositionBuilder::default()
    }
}

/// Builder for [`Position`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct PositionBuilder {
    id: Option<crate::PositionId>,
    account_id: Option<crate::AccountId>,
    contract_id: Option<crate::ContractId>,
    timestamp: Option<jiff::Timestamp>,
    trade_date: Option<TradeDate>,
    net_pos: Option<i64>,
    net_price: Option<crate::Decimal>,
    bought: Option<i64>,
    bought_value: Option<crate::Decimal>,
    sold: Option<i64>,
    sold_value: Option<crate::Decimal>,
    prev_pos: Option<i64>,
    prev_price: Option<crate::Decimal>,
}

impl PositionBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: crate::PositionId) -> Self {
        self.id = Some(value);
        self
    }

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

    /// Sets wire field `timestamp`.
    pub fn timestamp(mut self, value: jiff::Timestamp) -> Self {
        self.timestamp = Some(value);
        self
    }

    /// Sets wire field `tradeDate`.
    pub fn trade_date(mut self, value: TradeDate) -> Self {
        self.trade_date = Some(value);
        self
    }

    /// Sets wire field `netPos`.
    pub fn net_pos(mut self, value: i64) -> Self {
        self.net_pos = Some(value);
        self
    }

    /// Sets wire field `netPrice`.
    pub fn net_price(mut self, value: crate::Decimal) -> Self {
        self.net_price = Some(value);
        self
    }

    /// Sets wire field `bought`.
    pub fn bought(mut self, value: i64) -> Self {
        self.bought = Some(value);
        self
    }

    /// Sets wire field `boughtValue`.
    pub fn bought_value(mut self, value: crate::Decimal) -> Self {
        self.bought_value = Some(value);
        self
    }

    /// Sets wire field `sold`.
    pub fn sold(mut self, value: i64) -> Self {
        self.sold = Some(value);
        self
    }

    /// Sets wire field `soldValue`.
    pub fn sold_value(mut self, value: crate::Decimal) -> Self {
        self.sold_value = Some(value);
        self
    }

    /// Sets wire field `prevPos`.
    pub fn prev_pos(mut self, value: i64) -> Self {
        self.prev_pos = Some(value);
        self
    }

    /// Sets wire field `prevPrice`.
    pub fn prev_price(mut self, value: crate::Decimal) -> Self {
        self.prev_price = Some(value);
        self
    }

    /// Validates required fields and builds [`Position`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<Position, crate::api::current::BuildError> {
        let account_id = self
            .account_id
            .ok_or(crate::api::current::BuildError::missing("accountId"))?;
        let contract_id = self
            .contract_id
            .ok_or(crate::api::current::BuildError::missing("contractId"))?;
        let timestamp = self
            .timestamp
            .ok_or(crate::api::current::BuildError::missing("timestamp"))?;
        let trade_date = self
            .trade_date
            .ok_or(crate::api::current::BuildError::missing("tradeDate"))?;
        let net_pos = self
            .net_pos
            .ok_or(crate::api::current::BuildError::missing("netPos"))?;
        let bought = self
            .bought
            .ok_or(crate::api::current::BuildError::missing("bought"))?;
        let bought_value = self
            .bought_value
            .ok_or(crate::api::current::BuildError::missing("boughtValue"))?;
        let sold = self
            .sold
            .ok_or(crate::api::current::BuildError::missing("sold"))?;
        let sold_value = self
            .sold_value
            .ok_or(crate::api::current::BuildError::missing("soldValue"))?;
        let prev_pos = self
            .prev_pos
            .ok_or(crate::api::current::BuildError::missing("prevPos"))?;
        Ok(Position {
            id: self.id,
            account_id,
            contract_id,
            timestamp,
            trade_date,
            net_pos,
            net_price: self.net_price,
            bought,
            bought_value,
            sold,
            sold_value,
            prev_pos,
            prev_price: self.prev_price,
        })
    }
}

/// Current wire model `PostTradeRisk`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct PostTradeRisk {
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
    trailing_max_drawdown_mode: Option<PostTradeRiskTrailingMaxDrawdownMode>,
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
    #[serde(
        rename = "changesLocked",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    changes_locked: Option<bool>,
}

impl PostTradeRisk {
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
    pub fn trailing_max_drawdown_mode(&self) -> Option<&PostTradeRiskTrailingMaxDrawdownMode> {
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

    /// Returns wire field `changesLocked`.
    #[must_use]
    pub fn changes_locked(&self) -> Option<&bool> {
        self.changes_locked.as_ref()
    }

    /// Starts a builder for [`PostTradeRisk`].
    pub fn builder() -> PostTradeRiskBuilder {
        PostTradeRiskBuilder::default()
    }
}

/// Builder for [`PostTradeRisk`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct PostTradeRiskBuilder {
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
    trailing_max_drawdown_mode: Option<PostTradeRiskTrailingMaxDrawdownMode>,
    daily_profit_auto_liq: Option<crate::Decimal>,
    weekly_profit_auto_liq: Option<crate::Decimal>,
    do_not_unlock: Option<bool>,
    changes_locked: Option<bool>,
}

impl PostTradeRiskBuilder {
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
        value: PostTradeRiskTrailingMaxDrawdownMode,
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

    /// Sets wire field `changesLocked`.
    pub fn changes_locked(mut self, value: bool) -> Self {
        self.changes_locked = Some(value);
        self
    }

    /// Validates required fields and builds [`PostTradeRisk`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<PostTradeRisk, crate::api::current::BuildError> {
        Ok(PostTradeRisk {
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
            changes_locked: self.changes_locked,
        })
    }
}

/// Current provider values for `PostTradeRiskTrailingMaxDrawdownMode`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum PostTradeRiskTrailingMaxDrawdownMode {
    /// Provider value `EOD`.
    Eod,
    /// Provider value `RealTime`.
    RealTime,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl PostTradeRiskTrailingMaxDrawdownMode {
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

impl serde::Serialize for PostTradeRiskTrailingMaxDrawdownMode {
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

impl<'de> serde::Deserialize<'de> for PostTradeRiskTrailingMaxDrawdownMode {
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

/// Current wire model `PreTradeRisk`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct PreTradeRisk {
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
    product_type: Option<PreTradeRiskProductType>,
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
    product_verification_status: Option<PreTradeRiskProductVerificationStatus>,
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
    total_by: PreTradeRiskTotalBy,
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
    #[serde(rename = "parameters")]
    parameters: Vec<PreTradeRiskParameter>,
}

impl PreTradeRisk {
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
    pub fn product_type(&self) -> Option<&PreTradeRiskProductType> {
        self.product_type.as_ref()
    }

    /// Returns wire field `riskDiscountContractGroupId`.
    #[must_use]
    pub fn risk_discount_contract_group_id(&self) -> Option<&super::ids::ContractGroupId> {
        self.risk_discount_contract_group_id.as_ref()
    }

    /// Returns wire field `productVerificationStatus`.
    #[must_use]
    pub fn product_verification_status(&self) -> Option<&PreTradeRiskProductVerificationStatus> {
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
    pub fn total_by(&self) -> &PreTradeRiskTotalBy {
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

    /// Returns wire field `parameters`.
    #[must_use]
    pub fn parameters(&self) -> &[PreTradeRiskParameter] {
        &self.parameters
    }

    /// Starts a builder for [`PreTradeRisk`].
    pub fn builder() -> PreTradeRiskBuilder {
        PreTradeRiskBuilder::default()
    }
}

/// Builder for [`PreTradeRisk`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct PreTradeRiskBuilder {
    contract_id: Option<crate::ContractId>,
    product_id: Option<super::ids::ProductId>,
    exchange_id: Option<super::ids::ExchangeId>,
    product_type: Option<PreTradeRiskProductType>,
    risk_discount_contract_group_id: Option<super::ids::ContractGroupId>,
    product_verification_status: Option<PreTradeRiskProductVerificationStatus>,
    contract_group_id: Option<super::ids::ContractGroupId>,
    fungible_product_id: Option<super::ids::FungibleProductId>,
    active: Option<bool>,
    risk_time_period_id: Option<super::ids::RiskTimePeriodId>,
    total_by: Option<PreTradeRiskTotalBy>,
    short_limit: Option<i64>,
    long_limit: Option<i64>,
    exposed_limit: Option<i64>,
    fungible_exposed_limit: Option<i64>,
    description: Option<String>,
    parameters: Option<Vec<PreTradeRiskParameter>>,
}

impl PreTradeRiskBuilder {
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
    pub fn product_type(mut self, value: PreTradeRiskProductType) -> Self {
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
        value: PreTradeRiskProductVerificationStatus,
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
    pub fn total_by(mut self, value: PreTradeRiskTotalBy) -> Self {
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

    /// Sets wire field `parameters`.
    pub fn parameters(mut self, value: Vec<PreTradeRiskParameter>) -> Self {
        self.parameters = Some(value);
        self
    }

    /// Validates required fields and builds [`PreTradeRisk`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<PreTradeRisk, crate::api::current::BuildError> {
        let active = self
            .active
            .ok_or(crate::api::current::BuildError::missing("active"))?;
        let total_by = self
            .total_by
            .ok_or(crate::api::current::BuildError::missing("totalBy"))?;
        let parameters = self
            .parameters
            .ok_or(crate::api::current::BuildError::missing("parameters"))?;
        Ok(PreTradeRisk {
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
            parameters,
        })
    }
}

/// Current wire model `PreTradeRiskParameter`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct PreTradeRiskParameter {
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
    product_type: Option<PreTradeRiskParameterProductType>,
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
    product_verification_status: Option<PreTradeRiskParameterProductVerificationStatus>,
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
}

impl PreTradeRiskParameter {
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
    pub fn product_type(&self) -> Option<&PreTradeRiskParameterProductType> {
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
    ) -> Option<&PreTradeRiskParameterProductVerificationStatus> {
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

    /// Starts a builder for [`PreTradeRiskParameter`].
    pub fn builder() -> PreTradeRiskParameterBuilder {
        PreTradeRiskParameterBuilder::default()
    }
}

/// Builder for [`PreTradeRiskParameter`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct PreTradeRiskParameterBuilder {
    contract_id: Option<crate::ContractId>,
    product_id: Option<super::ids::ProductId>,
    exchange_id: Option<super::ids::ExchangeId>,
    product_type: Option<PreTradeRiskParameterProductType>,
    risk_discount_contract_group_id: Option<super::ids::ContractGroupId>,
    product_verification_status: Option<PreTradeRiskParameterProductVerificationStatus>,
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
}

impl PreTradeRiskParameterBuilder {
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
    pub fn product_type(mut self, value: PreTradeRiskParameterProductType) -> Self {
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
        value: PreTradeRiskParameterProductVerificationStatus,
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

    /// Validates required fields and builds [`PreTradeRiskParameter`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<PreTradeRiskParameter, crate::api::current::BuildError> {
        Ok(PreTradeRiskParameter {
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
        })
    }
}

/// Current provider values for `PreTradeRiskParameterProductType`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum PreTradeRiskParameterProductType {
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

impl PreTradeRiskParameterProductType {
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

impl serde::Serialize for PreTradeRiskParameterProductType {
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

impl<'de> serde::Deserialize<'de> for PreTradeRiskParameterProductType {
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

/// Current provider values for `PreTradeRiskParameterProductVerificationStatus`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum PreTradeRiskParameterProductVerificationStatus {
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

impl PreTradeRiskParameterProductVerificationStatus {
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

impl serde::Serialize for PreTradeRiskParameterProductVerificationStatus {
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

impl<'de> serde::Deserialize<'de> for PreTradeRiskParameterProductVerificationStatus {
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

/// Current provider values for `PreTradeRiskProductType`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum PreTradeRiskProductType {
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

impl PreTradeRiskProductType {
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

impl serde::Serialize for PreTradeRiskProductType {
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

impl<'de> serde::Deserialize<'de> for PreTradeRiskProductType {
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

/// Current provider values for `PreTradeRiskProductVerificationStatus`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum PreTradeRiskProductVerificationStatus {
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

impl PreTradeRiskProductVerificationStatus {
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

impl serde::Serialize for PreTradeRiskProductVerificationStatus {
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

impl<'de> serde::Deserialize<'de> for PreTradeRiskProductVerificationStatus {
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

/// Current provider values for `PreTradeRiskTotalBy`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum PreTradeRiskTotalBy {
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

impl PreTradeRiskTotalBy {
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

impl serde::Serialize for PreTradeRiskTotalBy {
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

impl<'de> serde::Deserialize<'de> for PreTradeRiskTotalBy {
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

/// Current wire model `Product`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct Product {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    id: Option<super::ids::ProductId>,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "currencyId")]
    currency_id: super::ids::CurrencyId,
    #[serde(rename = "productType")]
    product_type: ProductProductType,
    #[serde(rename = "description")]
    description: String,
    #[serde(rename = "exchangeId")]
    exchange_id: super::ids::ExchangeId,
    #[serde(rename = "contractGroupId")]
    contract_group_id: super::ids::ContractGroupId,
    #[serde(
        rename = "riskDiscountContractGroupId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    risk_discount_contract_group_id: Option<super::ids::ContractGroupId>,
    #[serde(rename = "status")]
    status: ProductStatus,
    #[serde(rename = "months", default, skip_serializing_if = "Option::is_none")]
    months: Option<String>,
    #[serde(rename = "isSecured", default, skip_serializing_if = "Option::is_none")]
    is_secured: Option<bool>,
    #[serde(rename = "valuePerPoint")]
    #[serde(with = "crate::decimal")]
    value_per_point: crate::Decimal,
    #[serde(rename = "priceFormatType")]
    price_format_type: ProductPriceFormatType,
    #[serde(rename = "priceFormat")]
    price_format: i64,
    #[serde(rename = "tickSize")]
    #[serde(with = "crate::decimal")]
    tick_size: crate::Decimal,
    #[serde(
        rename = "postTradeCategoryId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    post_trade_category_id: Option<super::ids::PostTradeCategoryId>,
}

impl Product {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> Option<&super::ids::ProductId> {
        self.id.as_ref()
    }

    /// Returns wire field `name`.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns wire field `currencyId`.
    #[must_use]
    pub fn currency_id(&self) -> &super::ids::CurrencyId {
        &self.currency_id
    }

    /// Returns wire field `productType`.
    #[must_use]
    pub fn product_type(&self) -> &ProductProductType {
        &self.product_type
    }

    /// Returns wire field `description`.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns wire field `exchangeId`.
    #[must_use]
    pub fn exchange_id(&self) -> &super::ids::ExchangeId {
        &self.exchange_id
    }

    /// Returns wire field `contractGroupId`.
    #[must_use]
    pub fn contract_group_id(&self) -> &super::ids::ContractGroupId {
        &self.contract_group_id
    }

    /// Returns wire field `riskDiscountContractGroupId`.
    #[must_use]
    pub fn risk_discount_contract_group_id(&self) -> Option<&super::ids::ContractGroupId> {
        self.risk_discount_contract_group_id.as_ref()
    }

    /// Returns wire field `status`.
    #[must_use]
    pub fn status(&self) -> &ProductStatus {
        &self.status
    }

    /// Returns wire field `months`.
    #[must_use]
    pub fn months(&self) -> Option<&str> {
        self.months.as_deref()
    }

    /// Returns wire field `isSecured`.
    #[must_use]
    pub fn is_secured(&self) -> Option<&bool> {
        self.is_secured.as_ref()
    }

    /// Returns wire field `valuePerPoint`.
    #[must_use]
    pub fn value_per_point(&self) -> &crate::Decimal {
        &self.value_per_point
    }

    /// Returns wire field `priceFormatType`.
    #[must_use]
    pub fn price_format_type(&self) -> &ProductPriceFormatType {
        &self.price_format_type
    }

    /// Returns wire field `priceFormat`.
    #[must_use]
    pub fn price_format(&self) -> &i64 {
        &self.price_format
    }

    /// Returns wire field `tickSize`.
    #[must_use]
    pub fn tick_size(&self) -> &crate::Decimal {
        &self.tick_size
    }

    /// Returns wire field `postTradeCategoryId`.
    #[must_use]
    pub fn post_trade_category_id(&self) -> Option<&super::ids::PostTradeCategoryId> {
        self.post_trade_category_id.as_ref()
    }

    /// Starts a builder for [`Product`].
    pub fn builder() -> ProductBuilder {
        ProductBuilder::default()
    }
}

/// Builder for [`Product`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ProductBuilder {
    id: Option<super::ids::ProductId>,
    name: Option<String>,
    currency_id: Option<super::ids::CurrencyId>,
    product_type: Option<ProductProductType>,
    description: Option<String>,
    exchange_id: Option<super::ids::ExchangeId>,
    contract_group_id: Option<super::ids::ContractGroupId>,
    risk_discount_contract_group_id: Option<super::ids::ContractGroupId>,
    status: Option<ProductStatus>,
    months: Option<String>,
    is_secured: Option<bool>,
    value_per_point: Option<crate::Decimal>,
    price_format_type: Option<ProductPriceFormatType>,
    price_format: Option<i64>,
    tick_size: Option<crate::Decimal>,
    post_trade_category_id: Option<super::ids::PostTradeCategoryId>,
}

impl ProductBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::ProductId) -> Self {
        self.id = Some(value);
        self
    }

    /// Sets wire field `name`.
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Sets wire field `currencyId`.
    pub fn currency_id(mut self, value: super::ids::CurrencyId) -> Self {
        self.currency_id = Some(value);
        self
    }

    /// Sets wire field `productType`.
    pub fn product_type(mut self, value: ProductProductType) -> Self {
        self.product_type = Some(value);
        self
    }

    /// Sets wire field `description`.
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Sets wire field `exchangeId`.
    pub fn exchange_id(mut self, value: super::ids::ExchangeId) -> Self {
        self.exchange_id = Some(value);
        self
    }

    /// Sets wire field `contractGroupId`.
    pub fn contract_group_id(mut self, value: super::ids::ContractGroupId) -> Self {
        self.contract_group_id = Some(value);
        self
    }

    /// Sets wire field `riskDiscountContractGroupId`.
    pub fn risk_discount_contract_group_id(mut self, value: super::ids::ContractGroupId) -> Self {
        self.risk_discount_contract_group_id = Some(value);
        self
    }

    /// Sets wire field `status`.
    pub fn status(mut self, value: ProductStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Sets wire field `months`.
    pub fn months(mut self, value: impl Into<String>) -> Self {
        self.months = Some(value.into());
        self
    }

    /// Sets wire field `isSecured`.
    pub fn is_secured(mut self, value: bool) -> Self {
        self.is_secured = Some(value);
        self
    }

    /// Sets wire field `valuePerPoint`.
    pub fn value_per_point(mut self, value: crate::Decimal) -> Self {
        self.value_per_point = Some(value);
        self
    }

    /// Sets wire field `priceFormatType`.
    pub fn price_format_type(mut self, value: ProductPriceFormatType) -> Self {
        self.price_format_type = Some(value);
        self
    }

    /// Sets wire field `priceFormat`.
    pub fn price_format(mut self, value: i64) -> Self {
        self.price_format = Some(value);
        self
    }

    /// Sets wire field `tickSize`.
    pub fn tick_size(mut self, value: crate::Decimal) -> Self {
        self.tick_size = Some(value);
        self
    }

    /// Sets wire field `postTradeCategoryId`.
    pub fn post_trade_category_id(mut self, value: super::ids::PostTradeCategoryId) -> Self {
        self.post_trade_category_id = Some(value);
        self
    }

    /// Validates required fields and builds [`Product`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<Product, crate::api::current::BuildError> {
        let name = self
            .name
            .ok_or(crate::api::current::BuildError::missing("name"))?;
        let currency_id = self
            .currency_id
            .ok_or(crate::api::current::BuildError::missing("currencyId"))?;
        let product_type = self
            .product_type
            .ok_or(crate::api::current::BuildError::missing("productType"))?;
        let description = self
            .description
            .ok_or(crate::api::current::BuildError::missing("description"))?;
        let exchange_id = self
            .exchange_id
            .ok_or(crate::api::current::BuildError::missing("exchangeId"))?;
        let contract_group_id = self
            .contract_group_id
            .ok_or(crate::api::current::BuildError::missing("contractGroupId"))?;
        let status = self
            .status
            .ok_or(crate::api::current::BuildError::missing("status"))?;
        let value_per_point = self
            .value_per_point
            .ok_or(crate::api::current::BuildError::missing("valuePerPoint"))?;
        let price_format_type = self
            .price_format_type
            .ok_or(crate::api::current::BuildError::missing("priceFormatType"))?;
        let price_format = self
            .price_format
            .ok_or(crate::api::current::BuildError::missing("priceFormat"))?;
        let tick_size = self
            .tick_size
            .ok_or(crate::api::current::BuildError::missing("tickSize"))?;
        Ok(Product {
            id: self.id,
            name,
            currency_id,
            product_type,
            description,
            exchange_id,
            contract_group_id,
            risk_discount_contract_group_id: self.risk_discount_contract_group_id,
            status,
            months: self.months,
            is_secured: self.is_secured,
            value_per_point,
            price_format_type,
            price_format,
            tick_size,
            post_trade_category_id: self.post_trade_category_id,
        })
    }
}

/// Current provider values for `ProductPriceFormatType`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum ProductPriceFormatType {
    /// Provider value `Decimal`.
    Decimal,
    /// Provider value `Fractional`.
    Fractional,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl ProductPriceFormatType {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Decimal => "Decimal",
            Self::Fractional => "Fractional",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for ProductPriceFormatType {
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

impl<'de> serde::Deserialize<'de> for ProductPriceFormatType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "Decimal" => Self::Decimal,
            "Fractional" => Self::Fractional,
            _ => Self::Unknown(value),
        })
    }
}

/// Current provider values for `ProductProductType`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum ProductProductType {
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

impl ProductProductType {
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

impl serde::Serialize for ProductProductType {
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

impl<'de> serde::Deserialize<'de> for ProductProductType {
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

/// Current provider values for `ProductStatus`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum ProductStatus {
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

impl ProductStatus {
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

impl serde::Serialize for ProductStatus {
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

impl<'de> serde::Deserialize<'de> for ProductStatus {
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

/// Current wire model `Property`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct Property {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    id: Option<super::ids::PropertyId>,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "propertyType")]
    property_type: PropertyPropertyType,
    #[serde(
        rename = "enumOptions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    enum_options: Option<String>,
    #[serde(
        rename = "defaultValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    default_value: Option<String>,
}

impl Property {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> Option<&super::ids::PropertyId> {
        self.id.as_ref()
    }

    /// Returns wire field `name`.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns wire field `propertyType`.
    #[must_use]
    pub fn property_type(&self) -> &PropertyPropertyType {
        &self.property_type
    }

    /// Returns wire field `enumOptions`.
    #[must_use]
    pub fn enum_options(&self) -> Option<&str> {
        self.enum_options.as_deref()
    }

    /// Returns wire field `defaultValue`.
    #[must_use]
    pub fn default_value(&self) -> Option<&str> {
        self.default_value.as_deref()
    }

    /// Starts a builder for [`Property`].
    pub fn builder() -> PropertyBuilder {
        PropertyBuilder::default()
    }
}

/// Builder for [`Property`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct PropertyBuilder {
    id: Option<super::ids::PropertyId>,
    name: Option<String>,
    property_type: Option<PropertyPropertyType>,
    enum_options: Option<String>,
    default_value: Option<String>,
}

impl PropertyBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::PropertyId) -> Self {
        self.id = Some(value);
        self
    }

    /// Sets wire field `name`.
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Sets wire field `propertyType`.
    pub fn property_type(mut self, value: PropertyPropertyType) -> Self {
        self.property_type = Some(value);
        self
    }

    /// Sets wire field `enumOptions`.
    pub fn enum_options(mut self, value: impl Into<String>) -> Self {
        self.enum_options = Some(value.into());
        self
    }

    /// Sets wire field `defaultValue`.
    pub fn default_value(mut self, value: impl Into<String>) -> Self {
        self.default_value = Some(value.into());
        self
    }

    /// Validates required fields and builds [`Property`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<Property, crate::api::current::BuildError> {
        let name = self
            .name
            .ok_or(crate::api::current::BuildError::missing("name"))?;
        let property_type = self
            .property_type
            .ok_or(crate::api::current::BuildError::missing("propertyType"))?;
        Ok(Property {
            id: self.id,
            name,
            property_type,
            enum_options: self.enum_options,
            default_value: self.default_value,
        })
    }
}

/// Current provider values for `PropertyPropertyType`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum PropertyPropertyType {
    /// Provider value `Boolean`.
    Boolean,
    /// Provider value `Enum`.
    Enum,
    /// Provider value `Integer`.
    Integer,
    /// Provider value `String`.
    String,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl PropertyPropertyType {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Boolean => "Boolean",
            Self::Enum => "Enum",
            Self::Integer => "Integer",
            Self::String => "String",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for PropertyPropertyType {
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

impl<'de> serde::Deserialize<'de> for PropertyPropertyType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "Boolean" => Self::Boolean,
            "Enum" => Self::Enum,
            "Integer" => Self::Integer,
            "String" => Self::String,
            _ => Self::Unknown(value),
        })
    }
}

/// Current wire model `RequestTradingPermission`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct RequestTradingPermission {
    #[serde(rename = "accountId")]
    account_id: crate::AccountId,
    #[serde(rename = "ctaContact")]
    cta_contact: String,
    #[serde(rename = "ctaEmail")]
    cta_email: String,
}

impl RequestTradingPermission {
    /// Returns wire field `accountId`.
    #[must_use]
    pub fn account_id(&self) -> &crate::AccountId {
        &self.account_id
    }

    /// Returns wire field `ctaContact`.
    #[must_use]
    pub fn cta_contact(&self) -> &str {
        &self.cta_contact
    }

    /// Returns wire field `ctaEmail`.
    #[must_use]
    pub fn cta_email(&self) -> &str {
        &self.cta_email
    }

    /// Starts a builder for [`RequestTradingPermission`].
    pub fn builder() -> RequestTradingPermissionBuilder {
        RequestTradingPermissionBuilder::default()
    }
}

/// Builder for [`RequestTradingPermission`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct RequestTradingPermissionBuilder {
    account_id: Option<crate::AccountId>,
    cta_contact: Option<String>,
    cta_email: Option<String>,
}

impl RequestTradingPermissionBuilder {
    /// Sets wire field `accountId`.
    pub fn account_id(mut self, value: crate::AccountId) -> Self {
        self.account_id = Some(value);
        self
    }

    /// Sets wire field `ctaContact`.
    pub fn cta_contact(mut self, value: impl Into<String>) -> Self {
        self.cta_contact = Some(value.into());
        self
    }

    /// Sets wire field `ctaEmail`.
    pub fn cta_email(mut self, value: impl Into<String>) -> Self {
        self.cta_email = Some(value.into());
        self
    }

    /// Validates required fields and builds [`RequestTradingPermission`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<RequestTradingPermission, crate::api::current::BuildError> {
        let account_id = self
            .account_id
            .ok_or(crate::api::current::BuildError::missing("accountId"))?;
        let cta_contact = self
            .cta_contact
            .ok_or(crate::api::current::BuildError::missing("ctaContact"))?;
        if cta_contact.is_empty() || cta_contact.trim() != cta_contact {
            return Err(crate::api::current::BuildError::invalid(
                "ctaContact",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        let cta_email = self
            .cta_email
            .ok_or(crate::api::current::BuildError::missing("ctaEmail"))?;
        if cta_email.is_empty() || cta_email.trim() != cta_email {
            return Err(crate::api::current::BuildError::invalid(
                "ctaEmail",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        Ok(RequestTradingPermission {
            account_id,
            cta_contact,
            cta_email,
        })
    }
}

impl crate::api::current::support::CurrentRequest for RequestTradingPermission {
    fn validate_current(&self) -> Result<(), crate::Error> {
        if self.cta_contact.is_empty() || self.cta_contact.trim() != self.cta_contact {
            return Err(crate::Error::InvalidRequest {
                field: "ctaContact",
                reason: "must be non-empty and have no surrounding whitespace",
            });
        }
        if self.cta_email.is_empty() || self.cta_email.trim() != self.cta_email {
            return Err(crate::Error::InvalidRequest {
                field: "ctaEmail",
                reason: "must be non-empty and have no surrounding whitespace",
            });
        }
        Ok(())
    }
}

/// Current wire model `RevokeTradingPermission`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct RevokeTradingPermission {
    #[serde(rename = "tradingPermissionId")]
    trading_permission_id: super::ids::TradingPermissionId,
}

impl RevokeTradingPermission {
    /// Returns wire field `tradingPermissionId`.
    #[must_use]
    pub fn trading_permission_id(&self) -> &super::ids::TradingPermissionId {
        &self.trading_permission_id
    }

    /// Starts a builder for [`RevokeTradingPermission`].
    pub fn builder() -> RevokeTradingPermissionBuilder {
        RevokeTradingPermissionBuilder::default()
    }
}

/// Builder for [`RevokeTradingPermission`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct RevokeTradingPermissionBuilder {
    trading_permission_id: Option<super::ids::TradingPermissionId>,
}

impl RevokeTradingPermissionBuilder {
    /// Sets wire field `tradingPermissionId`.
    pub fn trading_permission_id(mut self, value: super::ids::TradingPermissionId) -> Self {
        self.trading_permission_id = Some(value);
        self
    }

    /// Validates required fields and builds [`RevokeTradingPermission`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<RevokeTradingPermission, crate::api::current::BuildError> {
        let trading_permission_id =
            self.trading_permission_id
                .ok_or(crate::api::current::BuildError::missing(
                    "tradingPermissionId",
                ))?;
        Ok(RevokeTradingPermission {
            trading_permission_id,
        })
    }
}

impl crate::api::current::support::CurrentRequest for RevokeTradingPermission {
    fn validate_current(&self) -> Result<(), crate::Error> {
        Ok(())
    }
}

/// Current wire model `RevokeTradingPermissions`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct RevokeTradingPermissions {
    #[serde(rename = "tradingPermissionIds")]
    trading_permission_ids: Vec<super::ids::TradingPermissionId>,
}

impl RevokeTradingPermissions {
    /// Returns wire field `tradingPermissionIds`.
    #[must_use]
    pub fn trading_permission_ids(&self) -> &[super::ids::TradingPermissionId] {
        &self.trading_permission_ids
    }

    /// Starts a builder for [`RevokeTradingPermissions`].
    pub fn builder() -> RevokeTradingPermissionsBuilder {
        RevokeTradingPermissionsBuilder::default()
    }
}

/// Builder for [`RevokeTradingPermissions`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct RevokeTradingPermissionsBuilder {
    trading_permission_ids: Option<Vec<super::ids::TradingPermissionId>>,
}

impl RevokeTradingPermissionsBuilder {
    /// Sets wire field `tradingPermissionIds`.
    pub fn trading_permission_ids(mut self, value: Vec<super::ids::TradingPermissionId>) -> Self {
        self.trading_permission_ids = Some(value);
        self
    }

    /// Validates required fields and builds [`RevokeTradingPermissions`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<RevokeTradingPermissions, crate::api::current::BuildError> {
        let trading_permission_ids =
            self.trading_permission_ids
                .ok_or(crate::api::current::BuildError::missing(
                    "tradingPermissionIds",
                ))?;
        if trading_permission_ids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "tradingPermissionIds",
                "must not be empty",
            ));
        }
        Ok(RevokeTradingPermissions {
            trading_permission_ids,
        })
    }
}

impl crate::api::current::support::CurrentRequest for RevokeTradingPermissions {
    fn validate_current(&self) -> Result<(), crate::Error> {
        if self.trading_permission_ids.is_empty() {
            return Err(crate::Error::InvalidRequest {
                field: "tradingPermissionIds",
                reason: "must not be empty",
            });
        }
        Ok(())
    }
}

/// Current wire model `SecondMarketDataSubscription`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct SecondMarketDataSubscription {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    id: Option<super::ids::SecondMarketDataSubscriptionId>,
    #[serde(rename = "userId")]
    user_id: crate::UserId,
    #[serde(rename = "timestamp")]
    timestamp: jiff::Timestamp,
    #[serde(rename = "year")]
    year: i64,
    #[serde(rename = "month")]
    month: i64,
    #[serde(
        rename = "cancelledRenewal",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    cancelled_renewal: Option<bool>,
    #[serde(
        rename = "cancellationTimestamp",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    cancellation_timestamp: Option<jiff::Timestamp>,
}

impl SecondMarketDataSubscription {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> Option<&super::ids::SecondMarketDataSubscriptionId> {
        self.id.as_ref()
    }

    /// Returns wire field `userId`.
    #[must_use]
    pub fn user_id(&self) -> &crate::UserId {
        &self.user_id
    }

    /// Returns wire field `timestamp`.
    #[must_use]
    pub fn timestamp(&self) -> &jiff::Timestamp {
        &self.timestamp
    }

    /// Returns wire field `year`.
    #[must_use]
    pub fn year(&self) -> &i64 {
        &self.year
    }

    /// Returns wire field `month`.
    #[must_use]
    pub fn month(&self) -> &i64 {
        &self.month
    }

    /// Returns wire field `cancelledRenewal`.
    #[must_use]
    pub fn cancelled_renewal(&self) -> Option<&bool> {
        self.cancelled_renewal.as_ref()
    }

    /// Returns wire field `cancellationTimestamp`.
    #[must_use]
    pub fn cancellation_timestamp(&self) -> Option<&jiff::Timestamp> {
        self.cancellation_timestamp.as_ref()
    }

    /// Starts a builder for [`SecondMarketDataSubscription`].
    pub fn builder() -> SecondMarketDataSubscriptionBuilder {
        SecondMarketDataSubscriptionBuilder::default()
    }
}

/// Builder for [`SecondMarketDataSubscription`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct SecondMarketDataSubscriptionBuilder {
    id: Option<super::ids::SecondMarketDataSubscriptionId>,
    user_id: Option<crate::UserId>,
    timestamp: Option<jiff::Timestamp>,
    year: Option<i64>,
    month: Option<i64>,
    cancelled_renewal: Option<bool>,
    cancellation_timestamp: Option<jiff::Timestamp>,
}

impl SecondMarketDataSubscriptionBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::SecondMarketDataSubscriptionId) -> Self {
        self.id = Some(value);
        self
    }

    /// Sets wire field `userId`.
    pub fn user_id(mut self, value: crate::UserId) -> Self {
        self.user_id = Some(value);
        self
    }

    /// Sets wire field `timestamp`.
    pub fn timestamp(mut self, value: jiff::Timestamp) -> Self {
        self.timestamp = Some(value);
        self
    }

    /// Sets wire field `year`.
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    /// Sets wire field `month`.
    pub fn month(mut self, value: i64) -> Self {
        self.month = Some(value);
        self
    }

    /// Sets wire field `cancelledRenewal`.
    pub fn cancelled_renewal(mut self, value: bool) -> Self {
        self.cancelled_renewal = Some(value);
        self
    }

    /// Sets wire field `cancellationTimestamp`.
    pub fn cancellation_timestamp(mut self, value: jiff::Timestamp) -> Self {
        self.cancellation_timestamp = Some(value);
        self
    }

    /// Validates required fields and builds [`SecondMarketDataSubscription`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<SecondMarketDataSubscription, crate::api::current::BuildError> {
        let user_id = self
            .user_id
            .ok_or(crate::api::current::BuildError::missing("userId"))?;
        let timestamp = self
            .timestamp
            .ok_or(crate::api::current::BuildError::missing("timestamp"))?;
        let year = self
            .year
            .ok_or(crate::api::current::BuildError::missing("year"))?;
        let month = self
            .month
            .ok_or(crate::api::current::BuildError::missing("month"))?;
        Ok(SecondMarketDataSubscription {
            id: self.id,
            user_id,
            timestamp,
            year,
            month,
            cancelled_renewal: self.cancelled_renewal,
            cancellation_timestamp: self.cancellation_timestamp,
        })
    }
}

/// Current wire model `ShardingExpression`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ShardingExpression {
    #[serde(rename = "expressionType")]
    expression_type: String,
    #[serde(rename = "divisor")]
    divisor: i64,
    #[serde(rename = "remainder")]
    remainder: i64,
}

impl ShardingExpression {
    /// Returns wire field `expressionType`.
    #[must_use]
    pub fn expression_type(&self) -> &str {
        &self.expression_type
    }

    /// Returns wire field `divisor`.
    #[must_use]
    pub fn divisor(&self) -> &i64 {
        &self.divisor
    }

    /// Returns wire field `remainder`.
    #[must_use]
    pub fn remainder(&self) -> &i64 {
        &self.remainder
    }

    /// Starts a builder for [`ShardingExpression`].
    pub fn builder() -> ShardingExpressionBuilder {
        ShardingExpressionBuilder::default()
    }
}

/// Builder for [`ShardingExpression`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ShardingExpressionBuilder {
    expression_type: Option<String>,
    divisor: Option<i64>,
    remainder: Option<i64>,
}

impl ShardingExpressionBuilder {
    /// Sets wire field `expressionType`.
    pub fn expression_type(mut self, value: impl Into<String>) -> Self {
        self.expression_type = Some(value.into());
        self
    }

    /// Sets wire field `divisor`.
    pub fn divisor(mut self, value: i64) -> Self {
        self.divisor = Some(value);
        self
    }

    /// Sets wire field `remainder`.
    pub fn remainder(mut self, value: i64) -> Self {
        self.remainder = Some(value);
        self
    }

    /// Validates required fields and builds [`ShardingExpression`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ShardingExpression, crate::api::current::BuildError> {
        let expression_type = self
            .expression_type
            .ok_or(crate::api::current::BuildError::missing("expressionType"))?;
        let divisor = self
            .divisor
            .ok_or(crate::api::current::BuildError::missing("divisor"))?;
        let remainder = self
            .remainder
            .ok_or(crate::api::current::BuildError::missing("remainder"))?;
        Ok(ShardingExpression {
            expression_type,
            divisor,
            remainder,
        })
    }
}

/// Current wire model `SignUpOrganizationMember`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct SignUpOrganizationMember {
    #[serde(rename = "name")]
    name: crate::api::current::SecretValue,
    #[serde(rename = "email")]
    email: String,
    #[serde(rename = "password")]
    password: crate::api::current::SecretValue,
    #[serde(rename = "firstName")]
    first_name: String,
    #[serde(rename = "lastName")]
    last_name: String,
    #[serde(
        rename = "tradovateSubscriptionPlanId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    tradovate_subscription_plan_id: Option<super::ids::TradovateSubscriptionPlanId>,
    #[serde(
        rename = "entitlementIds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    entitlement_ids: Option<Vec<super::ids::EntitlementId>>,
    #[serde(
        rename = "originOrganization",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    origin_organization: Option<String>,
}

impl SignUpOrganizationMember {
    /// Reports whether secret field `name` is present.
    #[must_use]
    pub const fn has_name(&self) -> bool {
        true
    }

    pub(crate) fn name_secret(&self) -> &crate::api::current::SecretValue {
        &self.name
    }

    /// Returns wire field `email`.
    #[must_use]
    pub fn email(&self) -> &str {
        &self.email
    }

    /// Reports whether secret field `password` is present.
    #[must_use]
    pub const fn has_password(&self) -> bool {
        true
    }

    /// Returns wire field `firstName`.
    #[must_use]
    pub fn first_name(&self) -> &str {
        &self.first_name
    }

    /// Returns wire field `lastName`.
    #[must_use]
    pub fn last_name(&self) -> &str {
        &self.last_name
    }

    /// Returns wire field `tradovateSubscriptionPlanId`.
    #[must_use]
    pub fn tradovate_subscription_plan_id(
        &self,
    ) -> Option<&super::ids::TradovateSubscriptionPlanId> {
        self.tradovate_subscription_plan_id.as_ref()
    }

    /// Returns wire field `entitlementIds`.
    #[must_use]
    pub fn entitlement_ids(&self) -> Option<&[super::ids::EntitlementId]> {
        self.entitlement_ids.as_deref()
    }

    /// Returns wire field `originOrganization`.
    #[must_use]
    pub fn origin_organization(&self) -> Option<&str> {
        self.origin_organization.as_deref()
    }

    /// Starts a builder for [`SignUpOrganizationMember`].
    pub fn builder() -> SignUpOrganizationMemberBuilder {
        SignUpOrganizationMemberBuilder::default()
    }
}

/// Builder for [`SignUpOrganizationMember`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct SignUpOrganizationMemberBuilder {
    name: Option<crate::api::current::SecretValue>,
    email: Option<String>,
    password: Option<crate::api::current::SecretValue>,
    first_name: Option<String>,
    last_name: Option<String>,
    tradovate_subscription_plan_id: Option<super::ids::TradovateSubscriptionPlanId>,
    entitlement_ids: Option<Vec<super::ids::EntitlementId>>,
    origin_organization: Option<String>,
}

impl SignUpOrganizationMemberBuilder {
    /// Sets wire field `name`.
    pub fn name(mut self, value: crate::api::current::SecretValue) -> Self {
        self.name = Some(value);
        self
    }

    /// Sets wire field `email`.
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    /// Sets wire field `password`.
    pub fn password(mut self, value: crate::api::current::SecretValue) -> Self {
        self.password = Some(value);
        self
    }

    /// Sets wire field `firstName`.
    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    /// Sets wire field `lastName`.
    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
        self
    }

    /// Sets wire field `tradovateSubscriptionPlanId`.
    pub fn tradovate_subscription_plan_id(
        mut self,
        value: super::ids::TradovateSubscriptionPlanId,
    ) -> Self {
        self.tradovate_subscription_plan_id = Some(value);
        self
    }

    /// Sets wire field `entitlementIds`.
    pub fn entitlement_ids(mut self, value: Vec<super::ids::EntitlementId>) -> Self {
        self.entitlement_ids = Some(value);
        self
    }

    /// Sets wire field `originOrganization`.
    pub fn origin_organization(mut self, value: impl Into<String>) -> Self {
        self.origin_organization = Some(value.into());
        self
    }

    /// Validates required fields and builds [`SignUpOrganizationMember`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<SignUpOrganizationMember, crate::api::current::BuildError> {
        let name = self
            .name
            .ok_or(crate::api::current::BuildError::missing("name"))?;
        let email = self
            .email
            .ok_or(crate::api::current::BuildError::missing("email"))?;
        if email.is_empty() || email.trim() != email {
            return Err(crate::api::current::BuildError::invalid(
                "email",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        let password = self
            .password
            .ok_or(crate::api::current::BuildError::missing("password"))?;
        let first_name = self
            .first_name
            .ok_or(crate::api::current::BuildError::missing("firstName"))?;
        if first_name.is_empty() || first_name.trim() != first_name {
            return Err(crate::api::current::BuildError::invalid(
                "firstName",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        let last_name = self
            .last_name
            .ok_or(crate::api::current::BuildError::missing("lastName"))?;
        if last_name.is_empty() || last_name.trim() != last_name {
            return Err(crate::api::current::BuildError::invalid(
                "lastName",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        Ok(SignUpOrganizationMember {
            name,
            email,
            password,
            first_name,
            last_name,
            tradovate_subscription_plan_id: self.tradovate_subscription_plan_id,
            entitlement_ids: self.entitlement_ids,
            origin_organization: self.origin_organization,
        })
    }
}

impl crate::api::current::support::CurrentRequest for SignUpOrganizationMember {
    fn validate_current(&self) -> Result<(), crate::Error> {
        if self.email.is_empty() || self.email.trim() != self.email {
            return Err(crate::Error::InvalidRequest {
                field: "email",
                reason: "must be non-empty and have no surrounding whitespace",
            });
        }
        if self.first_name.is_empty() || self.first_name.trim() != self.first_name {
            return Err(crate::Error::InvalidRequest {
                field: "firstName",
                reason: "must be non-empty and have no surrounding whitespace",
            });
        }
        if self.last_name.is_empty() || self.last_name.trim() != self.last_name {
            return Err(crate::Error::InvalidRequest {
                field: "lastName",
                reason: "must be non-empty and have no surrounding whitespace",
            });
        }
        Ok(())
    }
}

/// Current wire model `SignUpResponse`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct SignUpResponse {
    #[serde(rename = "errorText", default, skip_serializing_if = "Option::is_none")]
    error_text: Option<String>,
    #[serde(rename = "errorCode")]
    error_code: SignUpResponseErrorCode,
    #[serde(rename = "userId", default, skip_serializing_if = "Option::is_none")]
    user_id: Option<crate::UserId>,
    #[serde(rename = "emailVerified")]
    email_verified: bool,
}

impl SignUpResponse {
    /// Returns wire field `errorText`.
    #[must_use]
    pub fn error_text(&self) -> Option<&str> {
        self.error_text.as_deref()
    }

    /// Returns wire field `errorCode`.
    #[must_use]
    pub fn error_code(&self) -> &SignUpResponseErrorCode {
        &self.error_code
    }

    /// Returns wire field `userId`.
    #[must_use]
    pub fn user_id(&self) -> Option<&crate::UserId> {
        self.user_id.as_ref()
    }

    /// Returns wire field `emailVerified`.
    #[must_use]
    pub fn email_verified(&self) -> &bool {
        &self.email_verified
    }

    /// Starts a builder for [`SignUpResponse`].
    pub fn builder() -> SignUpResponseBuilder {
        SignUpResponseBuilder::default()
    }
}

/// Builder for [`SignUpResponse`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct SignUpResponseBuilder {
    error_text: Option<String>,
    error_code: Option<SignUpResponseErrorCode>,
    user_id: Option<crate::UserId>,
    email_verified: Option<bool>,
}

impl SignUpResponseBuilder {
    /// Sets wire field `errorText`.
    pub fn error_text(mut self, value: impl Into<String>) -> Self {
        self.error_text = Some(value.into());
        self
    }

    /// Sets wire field `errorCode`.
    pub fn error_code(mut self, value: SignUpResponseErrorCode) -> Self {
        self.error_code = Some(value);
        self
    }

    /// Sets wire field `userId`.
    pub fn user_id(mut self, value: crate::UserId) -> Self {
        self.user_id = Some(value);
        self
    }

    /// Sets wire field `emailVerified`.
    pub fn email_verified(mut self, value: bool) -> Self {
        self.email_verified = Some(value);
        self
    }

    /// Validates required fields and builds [`SignUpResponse`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<SignUpResponse, crate::api::current::BuildError> {
        let error_code = self
            .error_code
            .ok_or(crate::api::current::BuildError::missing("errorCode"))?;
        let email_verified = self
            .email_verified
            .ok_or(crate::api::current::BuildError::missing("emailVerified"))?;
        Ok(SignUpResponse {
            error_text: self.error_text,
            error_code,
            user_id: self.user_id,
            email_verified,
        })
    }
}

/// Current provider values for `SignUpResponseErrorCode`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum SignUpResponseErrorCode {
    /// Provider value `DataError`.
    DataError,
    /// Provider value `EmailAlreadyRegistered`.
    EmailAlreadyRegistered,
    /// Provider value `EmailAndPasswordCompromised`.
    EmailAndPasswordCompromised,
    /// Provider value `EmailPolicyFailed`.
    EmailPolicyFailed,
    /// Provider value `FailedRecaptcha`.
    FailedRecaptcha,
    /// Provider value `PasswordCompromised`.
    PasswordCompromised,
    /// Provider value `Success`.
    Success,
    /// Provider value `UnknownError`.
    UnknownError,
    /// Provider value `UserAlreadyExists`.
    UserAlreadyExists,
    /// Provider value `WeakPassword`.
    WeakPassword,
    /// Provider value `WrongChallenge`.
    WrongChallenge,
    /// Provider value `WrongChallengeOrigin`.
    WrongChallengeOrigin,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl SignUpResponseErrorCode {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::DataError => "DataError",
            Self::EmailAlreadyRegistered => "EmailAlreadyRegistered",
            Self::EmailAndPasswordCompromised => "EmailAndPasswordCompromised",
            Self::EmailPolicyFailed => "EmailPolicyFailed",
            Self::FailedRecaptcha => "FailedRecaptcha",
            Self::PasswordCompromised => "PasswordCompromised",
            Self::Success => "Success",
            Self::UnknownError => "UnknownError",
            Self::UserAlreadyExists => "UserAlreadyExists",
            Self::WeakPassword => "WeakPassword",
            Self::WrongChallenge => "WrongChallenge",
            Self::WrongChallengeOrigin => "WrongChallengeOrigin",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for SignUpResponseErrorCode {
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

impl<'de> serde::Deserialize<'de> for SignUpResponseErrorCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "DataError" => Self::DataError,
            "EmailAlreadyRegistered" => Self::EmailAlreadyRegistered,
            "EmailAndPasswordCompromised" => Self::EmailAndPasswordCompromised,
            "EmailPolicyFailed" => Self::EmailPolicyFailed,
            "FailedRecaptcha" => Self::FailedRecaptcha,
            "PasswordCompromised" => Self::PasswordCompromised,
            "Success" => Self::Success,
            "UnknownError" => Self::UnknownError,
            "UserAlreadyExists" => Self::UserAlreadyExists,
            "WeakPassword" => Self::WeakPassword,
            "WrongChallenge" => Self::WrongChallenge,
            "WrongChallengeOrigin" => Self::WrongChallengeOrigin,
            _ => Self::Unknown(value),
        })
    }
}

/// Current wire model `SimpleResponse`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct SimpleResponse {
    #[serde(rename = "errorText", default, skip_serializing_if = "Option::is_none")]
    error_text: Option<String>,
    #[serde(rename = "ok")]
    ok: bool,
}

impl SimpleResponse {
    /// Returns wire field `errorText`.
    #[must_use]
    pub fn error_text(&self) -> Option<&str> {
        self.error_text.as_deref()
    }

    /// Returns wire field `ok`.
    #[must_use]
    pub fn ok(&self) -> &bool {
        &self.ok
    }

    /// Starts a builder for [`SimpleResponse`].
    pub fn builder() -> SimpleResponseBuilder {
        SimpleResponseBuilder::default()
    }
}

/// Builder for [`SimpleResponse`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct SimpleResponseBuilder {
    error_text: Option<String>,
    ok: Option<bool>,
}

impl SimpleResponseBuilder {
    /// Sets wire field `errorText`.
    pub fn error_text(mut self, value: impl Into<String>) -> Self {
        self.error_text = Some(value.into());
        self
    }

    /// Sets wire field `ok`.
    pub fn ok(mut self, value: bool) -> Self {
        self.ok = Some(value);
        self
    }

    /// Validates required fields and builds [`SimpleResponse`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<SimpleResponse, crate::api::current::BuildError> {
        let ok = self
            .ok
            .ok_or(crate::api::current::BuildError::missing("ok"))?;
        Ok(SimpleResponse {
            error_text: self.error_text,
            ok,
        })
    }
}

/// Current wire model `SpreadDefinition`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct SpreadDefinition {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    id: Option<super::ids::SpreadDefinitionId>,
    #[serde(rename = "timestamp")]
    timestamp: jiff::Timestamp,
    #[serde(rename = "spreadType")]
    spread_type: SpreadDefinitionSpreadType,
    #[serde(rename = "uds")]
    uds: bool,
}

impl SpreadDefinition {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> Option<&super::ids::SpreadDefinitionId> {
        self.id.as_ref()
    }

    /// Returns wire field `timestamp`.
    #[must_use]
    pub fn timestamp(&self) -> &jiff::Timestamp {
        &self.timestamp
    }

    /// Returns wire field `spreadType`.
    #[must_use]
    pub fn spread_type(&self) -> &SpreadDefinitionSpreadType {
        &self.spread_type
    }

    /// Returns wire field `uds`.
    #[must_use]
    pub fn uds(&self) -> &bool {
        &self.uds
    }

    /// Starts a builder for [`SpreadDefinition`].
    pub fn builder() -> SpreadDefinitionBuilder {
        SpreadDefinitionBuilder::default()
    }
}

/// Builder for [`SpreadDefinition`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct SpreadDefinitionBuilder {
    id: Option<super::ids::SpreadDefinitionId>,
    timestamp: Option<jiff::Timestamp>,
    spread_type: Option<SpreadDefinitionSpreadType>,
    uds: Option<bool>,
}

impl SpreadDefinitionBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::SpreadDefinitionId) -> Self {
        self.id = Some(value);
        self
    }

    /// Sets wire field `timestamp`.
    pub fn timestamp(mut self, value: jiff::Timestamp) -> Self {
        self.timestamp = Some(value);
        self
    }

    /// Sets wire field `spreadType`.
    pub fn spread_type(mut self, value: SpreadDefinitionSpreadType) -> Self {
        self.spread_type = Some(value);
        self
    }

    /// Sets wire field `uds`.
    pub fn uds(mut self, value: bool) -> Self {
        self.uds = Some(value);
        self
    }

    /// Validates required fields and builds [`SpreadDefinition`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<SpreadDefinition, crate::api::current::BuildError> {
        let timestamp = self
            .timestamp
            .ok_or(crate::api::current::BuildError::missing("timestamp"))?;
        let spread_type = self
            .spread_type
            .ok_or(crate::api::current::BuildError::missing("spreadType"))?;
        let uds = self
            .uds
            .ok_or(crate::api::current::BuildError::missing("uds"))?;
        Ok(SpreadDefinition {
            id: self.id,
            timestamp,
            spread_type,
            uds,
        })
    }
}

/// Current provider values for `SpreadDefinitionSpreadType`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum SpreadDefinitionSpreadType {
    /// Provider value `Bundle`.
    Bundle,
    /// Provider value `BundleSpread`.
    BundleSpread,
    /// Provider value `Butterfly`.
    Butterfly,
    /// Provider value `CalendarSpread`.
    CalendarSpread,
    /// Provider value `Condor`.
    Condor,
    /// Provider value `Crack`.
    Crack,
    /// Provider value `DoubleButterfly`.
    DoubleButterfly,
    /// Provider value `General`.
    General,
    /// Provider value `IntercommoditySpread`.
    IntercommoditySpread,
    /// Provider value `LaggedIntercommoditySpread`.
    LaggedIntercommoditySpread,
    /// Provider value `Pack`.
    Pack,
    /// Provider value `PackButterfly`.
    PackButterfly,
    /// Provider value `PackSpread`.
    PackSpread,
    /// Provider value `ReducedTickCalendarSpread`.
    ReducedTickCalendarSpread,
    /// Provider value `ReverseIntercommoditySpread`.
    ReverseIntercommoditySpread,
    /// Provider value `ReverseSpread`.
    ReverseSpread,
    /// Provider value `Strip`.
    Strip,
    /// Provider value `TreasuryIntercommoditySpread`.
    TreasuryIntercommoditySpread,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl SpreadDefinitionSpreadType {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Bundle => "Bundle",
            Self::BundleSpread => "BundleSpread",
            Self::Butterfly => "Butterfly",
            Self::CalendarSpread => "CalendarSpread",
            Self::Condor => "Condor",
            Self::Crack => "Crack",
            Self::DoubleButterfly => "DoubleButterfly",
            Self::General => "General",
            Self::IntercommoditySpread => "IntercommoditySpread",
            Self::LaggedIntercommoditySpread => "LaggedIntercommoditySpread",
            Self::Pack => "Pack",
            Self::PackButterfly => "PackButterfly",
            Self::PackSpread => "PackSpread",
            Self::ReducedTickCalendarSpread => "ReducedTickCalendarSpread",
            Self::ReverseIntercommoditySpread => "ReverseIntercommoditySpread",
            Self::ReverseSpread => "ReverseSpread",
            Self::Strip => "Strip",
            Self::TreasuryIntercommoditySpread => "TreasuryIntercommoditySpread",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for SpreadDefinitionSpreadType {
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

impl<'de> serde::Deserialize<'de> for SpreadDefinitionSpreadType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "Bundle" => Self::Bundle,
            "BundleSpread" => Self::BundleSpread,
            "Butterfly" => Self::Butterfly,
            "CalendarSpread" => Self::CalendarSpread,
            "Condor" => Self::Condor,
            "Crack" => Self::Crack,
            "DoubleButterfly" => Self::DoubleButterfly,
            "General" => Self::General,
            "IntercommoditySpread" => Self::IntercommoditySpread,
            "LaggedIntercommoditySpread" => Self::LaggedIntercommoditySpread,
            "Pack" => Self::Pack,
            "PackButterfly" => Self::PackButterfly,
            "PackSpread" => Self::PackSpread,
            "ReducedTickCalendarSpread" => Self::ReducedTickCalendarSpread,
            "ReverseIntercommoditySpread" => Self::ReverseIntercommoditySpread,
            "ReverseSpread" => Self::ReverseSpread,
            "Strip" => Self::Strip,
            "TreasuryIntercommoditySpread" => Self::TreasuryIntercommoditySpread,
            _ => Self::Unknown(value),
        })
    }
}

/// Current wire model `SubmitCustomerApplicationDocument`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct SubmitCustomerApplicationDocument {
    #[serde(rename = "process")]
    process: SubmitCustomerApplicationDocumentProcess,
    #[serde(rename = "documentType")]
    document_type: SubmitCustomerApplicationDocumentDocumentType,
    #[serde(rename = "filename")]
    filename: String,
    #[serde(rename = "base64data")]
    base64data: crate::api::current::SecretValue,
    #[serde(rename = "userId", default, skip_serializing_if = "Option::is_none")]
    user_id: Option<crate::UserId>,
    #[serde(
        rename = "personType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    person_type: Option<SubmitCustomerApplicationDocumentPersonType>,
    #[serde(
        rename = "customerApplicationId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    customer_application_id: Option<super::ids::CustomerApplicationId>,
}

impl SubmitCustomerApplicationDocument {
    /// Returns wire field `process`.
    #[must_use]
    pub fn process(&self) -> &SubmitCustomerApplicationDocumentProcess {
        &self.process
    }

    /// Returns wire field `documentType`.
    #[must_use]
    pub fn document_type(&self) -> &SubmitCustomerApplicationDocumentDocumentType {
        &self.document_type
    }

    /// Returns wire field `filename`.
    #[must_use]
    pub fn filename(&self) -> &str {
        &self.filename
    }

    /// Reports whether secret field `base64data` is present.
    #[must_use]
    pub const fn has_base64data(&self) -> bool {
        true
    }

    pub(crate) fn base64data_secret(&self) -> &crate::api::current::SecretValue {
        &self.base64data
    }

    /// Returns wire field `userId`.
    #[must_use]
    pub fn user_id(&self) -> Option<&crate::UserId> {
        self.user_id.as_ref()
    }

    /// Returns wire field `personType`.
    #[must_use]
    pub fn person_type(&self) -> Option<&SubmitCustomerApplicationDocumentPersonType> {
        self.person_type.as_ref()
    }

    /// Returns wire field `customerApplicationId`.
    #[must_use]
    pub fn customer_application_id(&self) -> Option<&super::ids::CustomerApplicationId> {
        self.customer_application_id.as_ref()
    }

    /// Starts a builder for [`SubmitCustomerApplicationDocument`].
    pub fn builder() -> SubmitCustomerApplicationDocumentBuilder {
        SubmitCustomerApplicationDocumentBuilder::default()
    }
}

/// Builder for [`SubmitCustomerApplicationDocument`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct SubmitCustomerApplicationDocumentBuilder {
    process: Option<SubmitCustomerApplicationDocumentProcess>,
    document_type: Option<SubmitCustomerApplicationDocumentDocumentType>,
    filename: Option<String>,
    base64data: Option<crate::api::current::SecretValue>,
    user_id: Option<crate::UserId>,
    person_type: Option<SubmitCustomerApplicationDocumentPersonType>,
    customer_application_id: Option<super::ids::CustomerApplicationId>,
}

impl SubmitCustomerApplicationDocumentBuilder {
    /// Sets wire field `process`.
    pub fn process(mut self, value: SubmitCustomerApplicationDocumentProcess) -> Self {
        self.process = Some(value);
        self
    }

    /// Sets wire field `documentType`.
    pub fn document_type(mut self, value: SubmitCustomerApplicationDocumentDocumentType) -> Self {
        self.document_type = Some(value);
        self
    }

    /// Sets wire field `filename`.
    pub fn filename(mut self, value: impl Into<String>) -> Self {
        self.filename = Some(value.into());
        self
    }

    /// Sets wire field `base64data`.
    pub fn base64data(mut self, value: crate::api::current::SecretValue) -> Self {
        self.base64data = Some(value);
        self
    }

    /// Sets wire field `userId`.
    pub fn user_id(mut self, value: crate::UserId) -> Self {
        self.user_id = Some(value);
        self
    }

    /// Sets wire field `personType`.
    pub fn person_type(mut self, value: SubmitCustomerApplicationDocumentPersonType) -> Self {
        self.person_type = Some(value);
        self
    }

    /// Sets wire field `customerApplicationId`.
    pub fn customer_application_id(mut self, value: super::ids::CustomerApplicationId) -> Self {
        self.customer_application_id = Some(value);
        self
    }

    /// Validates required fields and builds [`SubmitCustomerApplicationDocument`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(
        self,
    ) -> Result<SubmitCustomerApplicationDocument, crate::api::current::BuildError> {
        let process = self
            .process
            .ok_or(crate::api::current::BuildError::missing("process"))?;
        let document_type = self
            .document_type
            .ok_or(crate::api::current::BuildError::missing("documentType"))?;
        let filename = self
            .filename
            .ok_or(crate::api::current::BuildError::missing("filename"))?;
        if filename.is_empty() || filename.trim() != filename {
            return Err(crate::api::current::BuildError::invalid(
                "filename",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        let base64data = self
            .base64data
            .ok_or(crate::api::current::BuildError::missing("base64data"))?;
        Ok(SubmitCustomerApplicationDocument {
            process,
            document_type,
            filename,
            base64data,
            user_id: self.user_id,
            person_type: self.person_type,
            customer_application_id: self.customer_application_id,
        })
    }
}

impl crate::api::current::support::CurrentRequest for SubmitCustomerApplicationDocument {
    fn validate_current(&self) -> Result<(), crate::Error> {
        if self.filename.is_empty() || self.filename.trim() != self.filename {
            return Err(crate::Error::InvalidRequest {
                field: "filename",
                reason: "must be non-empty and have no surrounding whitespace",
            });
        }
        Ok(())
    }
}

/// Current provider values for `SubmitCustomerApplicationDocumentDocumentType`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum SubmitCustomerApplicationDocumentDocumentType {
    /// Provider value `AMLBankStatement`.
    AmlBankStatement,
    /// Provider value `AMLOwnFundsLetter`.
    AmlOwnFundsLetter,
    /// Provider value `AccountReactivation`.
    AccountReactivation,
    /// Provider value `AchOther`.
    AchOther,
    /// Provider value `AdditionalRiskDisclosure`.
    AdditionalRiskDisclosure,
    /// Provider value `AddressChangeVerification`.
    AddressChangeVerification,
    /// Provider value `AddressClarificationDocVsApplication`.
    AddressClarificationDocVsApplication,
    /// Provider value `AddressClarificationDocVsConfirmedAV`.
    AddressClarificationDocVsConfirmedAv,
    /// Provider value `AddressVerificationFAFTA`.
    AddressVerificationFafta,
    /// Provider value `AddressVerificationLLC`.
    AddressVerificationLlc,
    /// Provider value `AddressVerificationManagingMember`.
    AddressVerificationManagingMember,
    /// Provider value `AmendmentApprovingFuturesTrading`.
    AmendmentApprovingFuturesTrading,
    /// Provider value `AnnualIncomeAmount`.
    AnnualIncomeAmount,
    /// Provider value `ApplicationAndIDNameMismatch`.
    ApplicationAndIdNameMismatch,
    /// Provider value `ArticlesOfOrganization`.
    ArticlesOfOrganization,
    /// Provider value `BankSupportingDocument`.
    BankSupportingDocument,
    /// Provider value `BankruptcyDischargePaperwork`.
    BankruptcyDischargePaperwork,
    /// Provider value `BeneficialOwnerCertification`.
    BeneficialOwnerCertification,
    /// Provider value `CertificateOfGoodStanding`.
    CertificateOfGoodStanding,
    /// Provider value `CompanyFinancials`.
    CompanyFinancials,
    /// Provider value `ConfirmAccountInterest`.
    ConfirmAccountInterest,
    /// Provider value `ConfirmAffiliations`.
    ConfirmAffiliations,
    /// Provider value `ConfirmFullName`.
    ConfirmFullName,
    /// Provider value `ConfirmIRACustodian`.
    ConfirmIraCustodian,
    /// Provider value `ConfirmationOfDeposits`.
    ConfirmationOfDeposits,
    /// Provider value `CorporateBylaws`.
    CorporateBylaws,
    /// Provider value `CurrentBankStatement`.
    CurrentBankStatement,
    /// Provider value `DrivingLicense`.
    DrivingLicense,
    /// Provider value `EIN`.
    Ein,
    /// Provider value `EVS`.
    Evs,
    /// Provider value `EmployeeAttestationLetter`.
    EmployeeAttestationLetter,
    /// Provider value `EmploymentDetails`.
    EmploymentDetails,
    /// Provider value `EntityWebsite`.
    EntityWebsite,
    /// Provider value `FullTimeTrader`.
    FullTimeTrader,
    /// Provider value `HedgeClarification`.
    HedgeClarification,
    /// Provider value `HighRiskApplicant`.
    HighRiskApplicant,
    /// Provider value `HomemakerIncome`.
    HomemakerIncome,
    /// Provider value `IRACustody`.
    IraCustody,
    /// Provider value `Id`.
    Id,
    /// Provider value `InvestmentType`.
    InvestmentType,
    /// Provider value `LetterOfAwareness`.
    LetterOfAwareness,
    /// Provider value `LiquidNetWorthAmount`.
    LiquidNetWorthAmount,
    /// Provider value `ManagingMemberId`.
    ManagingMemberId,
    /// Provider value `NFARegistrationConfirmation`.
    NfaRegistrationConfirmation,
    /// Provider value `NatureOfEntity`.
    NatureOfEntity,
    /// Provider value `NatureOfSelfEmployment`.
    NatureOfSelfEmployment,
    /// Provider value `NetWorthAmount`.
    NetWorthAmount,
    /// Provider value `OfferingCircular`.
    OfferingCircular,
    /// Provider value `OperatingAgreement`.
    OperatingAgreement,
    /// Provider value `Other`.
    Other,
    /// Provider value `OtherText`.
    OtherText,
    /// Provider value `OwnFunds`.
    OwnFunds,
    /// Provider value `POAForm`.
    PoaForm,
    /// Provider value `POBox`.
    PoBox,
    /// Provider value `PartnershipDocuments`.
    PartnershipDocuments,
    /// Provider value `Passport`.
    Passport,
    /// Provider value `Professional`.
    Professional,
    /// Provider value `ResidentialAddressConfirmation`.
    ResidentialAddressConfirmation,
    /// Provider value `RetirementIncome`.
    RetirementIncome,
    /// Provider value `SSNMiskey`.
    SsnMiskey,
    /// Provider value `ShareholderRegister`.
    ShareholderRegister,
    /// Provider value `SocialSecurityCard`.
    SocialSecurityCard,
    /// Provider value `SourceOfIncome`.
    SourceOfIncome,
    /// Provider value `SourceOfRiskCapital`.
    SourceOfRiskCapital,
    /// Provider value `StatementOfGoodStanding`.
    StatementOfGoodStanding,
    /// Provider value `StudentIncome`.
    StudentIncome,
    /// Provider value `ThirdPartyFunds`.
    ThirdPartyFunds,
    /// Provider value `TradingAuthorityPOA`.
    TradingAuthorityPoa,
    /// Provider value `TrustDocuments`.
    TrustDocuments,
    /// Provider value `UnderstandingOfFuturesTrading`.
    UnderstandingOfFuturesTrading,
    /// Provider value `UnemployedIncome`.
    UnemployedIncome,
    /// Provider value `UtilityBill`.
    UtilityBill,
    /// Provider value `VerifyEmailAnotherName`.
    VerifyEmailAnotherName,
    /// Provider value `VerifyEmailOwnership`.
    VerifyEmailOwnership,
    /// Provider value `VerifyEmailUsernameEntityWording`.
    VerifyEmailUsernameEntityWording,
    /// Provider value `VerifyEmailUsernameFinancialWording`.
    VerifyEmailUsernameFinancialWording,
    /// Provider value `VerifyUsernameAnotherName`.
    VerifyUsernameAnotherName,
    /// Provider value `Visa`.
    Visa,
    /// Provider value `W8Clarification`.
    W8Clarification,
    /// Provider value `W8Incomplete`.
    W8Incomplete,
    /// Provider value `W9`.
    W9,
    /// Provider value `YearsOfTradingExperience`.
    YearsOfTradingExperience,
    /// Provider value `eSignBeneficialOwnerCertification`.
    ESignBeneficialOwnerCertification,
    /// Provider value `eSignEmployeeAttestationLetter`.
    ESignEmployeeAttestationLetter,
    /// Provider value `eSignMarketDataAgreement`.
    ESignMarketDataAgreement,
    /// Provider value `eSignW9`.
    ESignW9,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl SubmitCustomerApplicationDocumentDocumentType {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::AmlBankStatement => "AMLBankStatement",
            Self::AmlOwnFundsLetter => "AMLOwnFundsLetter",
            Self::AccountReactivation => "AccountReactivation",
            Self::AchOther => "AchOther",
            Self::AdditionalRiskDisclosure => "AdditionalRiskDisclosure",
            Self::AddressChangeVerification => "AddressChangeVerification",
            Self::AddressClarificationDocVsApplication => "AddressClarificationDocVsApplication",
            Self::AddressClarificationDocVsConfirmedAv => "AddressClarificationDocVsConfirmedAV",
            Self::AddressVerificationFafta => "AddressVerificationFAFTA",
            Self::AddressVerificationLlc => "AddressVerificationLLC",
            Self::AddressVerificationManagingMember => "AddressVerificationManagingMember",
            Self::AmendmentApprovingFuturesTrading => "AmendmentApprovingFuturesTrading",
            Self::AnnualIncomeAmount => "AnnualIncomeAmount",
            Self::ApplicationAndIdNameMismatch => "ApplicationAndIDNameMismatch",
            Self::ArticlesOfOrganization => "ArticlesOfOrganization",
            Self::BankSupportingDocument => "BankSupportingDocument",
            Self::BankruptcyDischargePaperwork => "BankruptcyDischargePaperwork",
            Self::BeneficialOwnerCertification => "BeneficialOwnerCertification",
            Self::CertificateOfGoodStanding => "CertificateOfGoodStanding",
            Self::CompanyFinancials => "CompanyFinancials",
            Self::ConfirmAccountInterest => "ConfirmAccountInterest",
            Self::ConfirmAffiliations => "ConfirmAffiliations",
            Self::ConfirmFullName => "ConfirmFullName",
            Self::ConfirmIraCustodian => "ConfirmIRACustodian",
            Self::ConfirmationOfDeposits => "ConfirmationOfDeposits",
            Self::CorporateBylaws => "CorporateBylaws",
            Self::CurrentBankStatement => "CurrentBankStatement",
            Self::DrivingLicense => "DrivingLicense",
            Self::Ein => "EIN",
            Self::Evs => "EVS",
            Self::EmployeeAttestationLetter => "EmployeeAttestationLetter",
            Self::EmploymentDetails => "EmploymentDetails",
            Self::EntityWebsite => "EntityWebsite",
            Self::FullTimeTrader => "FullTimeTrader",
            Self::HedgeClarification => "HedgeClarification",
            Self::HighRiskApplicant => "HighRiskApplicant",
            Self::HomemakerIncome => "HomemakerIncome",
            Self::IraCustody => "IRACustody",
            Self::Id => "Id",
            Self::InvestmentType => "InvestmentType",
            Self::LetterOfAwareness => "LetterOfAwareness",
            Self::LiquidNetWorthAmount => "LiquidNetWorthAmount",
            Self::ManagingMemberId => "ManagingMemberId",
            Self::NfaRegistrationConfirmation => "NFARegistrationConfirmation",
            Self::NatureOfEntity => "NatureOfEntity",
            Self::NatureOfSelfEmployment => "NatureOfSelfEmployment",
            Self::NetWorthAmount => "NetWorthAmount",
            Self::OfferingCircular => "OfferingCircular",
            Self::OperatingAgreement => "OperatingAgreement",
            Self::Other => "Other",
            Self::OtherText => "OtherText",
            Self::OwnFunds => "OwnFunds",
            Self::PoaForm => "POAForm",
            Self::PoBox => "POBox",
            Self::PartnershipDocuments => "PartnershipDocuments",
            Self::Passport => "Passport",
            Self::Professional => "Professional",
            Self::ResidentialAddressConfirmation => "ResidentialAddressConfirmation",
            Self::RetirementIncome => "RetirementIncome",
            Self::SsnMiskey => "SSNMiskey",
            Self::ShareholderRegister => "ShareholderRegister",
            Self::SocialSecurityCard => "SocialSecurityCard",
            Self::SourceOfIncome => "SourceOfIncome",
            Self::SourceOfRiskCapital => "SourceOfRiskCapital",
            Self::StatementOfGoodStanding => "StatementOfGoodStanding",
            Self::StudentIncome => "StudentIncome",
            Self::ThirdPartyFunds => "ThirdPartyFunds",
            Self::TradingAuthorityPoa => "TradingAuthorityPOA",
            Self::TrustDocuments => "TrustDocuments",
            Self::UnderstandingOfFuturesTrading => "UnderstandingOfFuturesTrading",
            Self::UnemployedIncome => "UnemployedIncome",
            Self::UtilityBill => "UtilityBill",
            Self::VerifyEmailAnotherName => "VerifyEmailAnotherName",
            Self::VerifyEmailOwnership => "VerifyEmailOwnership",
            Self::VerifyEmailUsernameEntityWording => "VerifyEmailUsernameEntityWording",
            Self::VerifyEmailUsernameFinancialWording => "VerifyEmailUsernameFinancialWording",
            Self::VerifyUsernameAnotherName => "VerifyUsernameAnotherName",
            Self::Visa => "Visa",
            Self::W8Clarification => "W8Clarification",
            Self::W8Incomplete => "W8Incomplete",
            Self::W9 => "W9",
            Self::YearsOfTradingExperience => "YearsOfTradingExperience",
            Self::ESignBeneficialOwnerCertification => "eSignBeneficialOwnerCertification",
            Self::ESignEmployeeAttestationLetter => "eSignEmployeeAttestationLetter",
            Self::ESignMarketDataAgreement => "eSignMarketDataAgreement",
            Self::ESignW9 => "eSignW9",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for SubmitCustomerApplicationDocumentDocumentType {
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

impl<'de> serde::Deserialize<'de> for SubmitCustomerApplicationDocumentDocumentType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "AMLBankStatement" => Self::AmlBankStatement,
            "AMLOwnFundsLetter" => Self::AmlOwnFundsLetter,
            "AccountReactivation" => Self::AccountReactivation,
            "AchOther" => Self::AchOther,
            "AdditionalRiskDisclosure" => Self::AdditionalRiskDisclosure,
            "AddressChangeVerification" => Self::AddressChangeVerification,
            "AddressClarificationDocVsApplication" => Self::AddressClarificationDocVsApplication,
            "AddressClarificationDocVsConfirmedAV" => Self::AddressClarificationDocVsConfirmedAv,
            "AddressVerificationFAFTA" => Self::AddressVerificationFafta,
            "AddressVerificationLLC" => Self::AddressVerificationLlc,
            "AddressVerificationManagingMember" => Self::AddressVerificationManagingMember,
            "AmendmentApprovingFuturesTrading" => Self::AmendmentApprovingFuturesTrading,
            "AnnualIncomeAmount" => Self::AnnualIncomeAmount,
            "ApplicationAndIDNameMismatch" => Self::ApplicationAndIdNameMismatch,
            "ArticlesOfOrganization" => Self::ArticlesOfOrganization,
            "BankSupportingDocument" => Self::BankSupportingDocument,
            "BankruptcyDischargePaperwork" => Self::BankruptcyDischargePaperwork,
            "BeneficialOwnerCertification" => Self::BeneficialOwnerCertification,
            "CertificateOfGoodStanding" => Self::CertificateOfGoodStanding,
            "CompanyFinancials" => Self::CompanyFinancials,
            "ConfirmAccountInterest" => Self::ConfirmAccountInterest,
            "ConfirmAffiliations" => Self::ConfirmAffiliations,
            "ConfirmFullName" => Self::ConfirmFullName,
            "ConfirmIRACustodian" => Self::ConfirmIraCustodian,
            "ConfirmationOfDeposits" => Self::ConfirmationOfDeposits,
            "CorporateBylaws" => Self::CorporateBylaws,
            "CurrentBankStatement" => Self::CurrentBankStatement,
            "DrivingLicense" => Self::DrivingLicense,
            "EIN" => Self::Ein,
            "EVS" => Self::Evs,
            "EmployeeAttestationLetter" => Self::EmployeeAttestationLetter,
            "EmploymentDetails" => Self::EmploymentDetails,
            "EntityWebsite" => Self::EntityWebsite,
            "FullTimeTrader" => Self::FullTimeTrader,
            "HedgeClarification" => Self::HedgeClarification,
            "HighRiskApplicant" => Self::HighRiskApplicant,
            "HomemakerIncome" => Self::HomemakerIncome,
            "IRACustody" => Self::IraCustody,
            "Id" => Self::Id,
            "InvestmentType" => Self::InvestmentType,
            "LetterOfAwareness" => Self::LetterOfAwareness,
            "LiquidNetWorthAmount" => Self::LiquidNetWorthAmount,
            "ManagingMemberId" => Self::ManagingMemberId,
            "NFARegistrationConfirmation" => Self::NfaRegistrationConfirmation,
            "NatureOfEntity" => Self::NatureOfEntity,
            "NatureOfSelfEmployment" => Self::NatureOfSelfEmployment,
            "NetWorthAmount" => Self::NetWorthAmount,
            "OfferingCircular" => Self::OfferingCircular,
            "OperatingAgreement" => Self::OperatingAgreement,
            "Other" => Self::Other,
            "OtherText" => Self::OtherText,
            "OwnFunds" => Self::OwnFunds,
            "POAForm" => Self::PoaForm,
            "POBox" => Self::PoBox,
            "PartnershipDocuments" => Self::PartnershipDocuments,
            "Passport" => Self::Passport,
            "Professional" => Self::Professional,
            "ResidentialAddressConfirmation" => Self::ResidentialAddressConfirmation,
            "RetirementIncome" => Self::RetirementIncome,
            "SSNMiskey" => Self::SsnMiskey,
            "ShareholderRegister" => Self::ShareholderRegister,
            "SocialSecurityCard" => Self::SocialSecurityCard,
            "SourceOfIncome" => Self::SourceOfIncome,
            "SourceOfRiskCapital" => Self::SourceOfRiskCapital,
            "StatementOfGoodStanding" => Self::StatementOfGoodStanding,
            "StudentIncome" => Self::StudentIncome,
            "ThirdPartyFunds" => Self::ThirdPartyFunds,
            "TradingAuthorityPOA" => Self::TradingAuthorityPoa,
            "TrustDocuments" => Self::TrustDocuments,
            "UnderstandingOfFuturesTrading" => Self::UnderstandingOfFuturesTrading,
            "UnemployedIncome" => Self::UnemployedIncome,
            "UtilityBill" => Self::UtilityBill,
            "VerifyEmailAnotherName" => Self::VerifyEmailAnotherName,
            "VerifyEmailOwnership" => Self::VerifyEmailOwnership,
            "VerifyEmailUsernameEntityWording" => Self::VerifyEmailUsernameEntityWording,
            "VerifyEmailUsernameFinancialWording" => Self::VerifyEmailUsernameFinancialWording,
            "VerifyUsernameAnotherName" => Self::VerifyUsernameAnotherName,
            "Visa" => Self::Visa,
            "W8Clarification" => Self::W8Clarification,
            "W8Incomplete" => Self::W8Incomplete,
            "W9" => Self::W9,
            "YearsOfTradingExperience" => Self::YearsOfTradingExperience,
            "eSignBeneficialOwnerCertification" => Self::ESignBeneficialOwnerCertification,
            "eSignEmployeeAttestationLetter" => Self::ESignEmployeeAttestationLetter,
            "eSignMarketDataAgreement" => Self::ESignMarketDataAgreement,
            "eSignW9" => Self::ESignW9,
            _ => Self::Unknown(value),
        })
    }
}

/// Current provider values for `SubmitCustomerApplicationDocumentPersonType`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum SubmitCustomerApplicationDocumentPersonType {
    /// Provider value `Applicant`.
    Applicant,
    /// Provider value `EntityOpener`.
    EntityOpener,
    /// Provider value `EntityResponsiblePerson`.
    EntityResponsiblePerson,
    /// Provider value `EntityStakeholder1`.
    EntityStakeholder1,
    /// Provider value `EntityStakeholder2`.
    EntityStakeholder2,
    /// Provider value `JointApplicant`.
    JointApplicant,
    /// Provider value `JointThirdPartyPerson`.
    JointThirdPartyPerson,
    /// Provider value `SubAccount`.
    SubAccount,
    /// Provider value `ThirdPartyPerson`.
    ThirdPartyPerson,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl SubmitCustomerApplicationDocumentPersonType {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Applicant => "Applicant",
            Self::EntityOpener => "EntityOpener",
            Self::EntityResponsiblePerson => "EntityResponsiblePerson",
            Self::EntityStakeholder1 => "EntityStakeholder1",
            Self::EntityStakeholder2 => "EntityStakeholder2",
            Self::JointApplicant => "JointApplicant",
            Self::JointThirdPartyPerson => "JointThirdPartyPerson",
            Self::SubAccount => "SubAccount",
            Self::ThirdPartyPerson => "ThirdPartyPerson",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for SubmitCustomerApplicationDocumentPersonType {
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

impl<'de> serde::Deserialize<'de> for SubmitCustomerApplicationDocumentPersonType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "Applicant" => Self::Applicant,
            "EntityOpener" => Self::EntityOpener,
            "EntityResponsiblePerson" => Self::EntityResponsiblePerson,
            "EntityStakeholder1" => Self::EntityStakeholder1,
            "EntityStakeholder2" => Self::EntityStakeholder2,
            "JointApplicant" => Self::JointApplicant,
            "JointThirdPartyPerson" => Self::JointThirdPartyPerson,
            "SubAccount" => Self::SubAccount,
            "ThirdPartyPerson" => Self::ThirdPartyPerson,
            _ => Self::Unknown(value),
        })
    }
}

/// Current provider values for `SubmitCustomerApplicationDocumentProcess`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum SubmitCustomerApplicationDocumentProcess {
    /// Provider value `AML_FPV`.
    AMLFPV,
    /// Provider value `Bankruptcy`.
    Bankruptcy,
    /// Provider value `Compliance`.
    Compliance,
    /// Provider value `Identity`.
    Identity,
    /// Provider value `NFA`.
    Nfa,
    /// Provider value `SubAccountRequest`.
    SubAccountRequest,
    /// Provider value `Watchlist`.
    Watchlist,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl SubmitCustomerApplicationDocumentProcess {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::AMLFPV => "AML_FPV",
            Self::Bankruptcy => "Bankruptcy",
            Self::Compliance => "Compliance",
            Self::Identity => "Identity",
            Self::Nfa => "NFA",
            Self::SubAccountRequest => "SubAccountRequest",
            Self::Watchlist => "Watchlist",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for SubmitCustomerApplicationDocumentProcess {
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

impl<'de> serde::Deserialize<'de> for SubmitCustomerApplicationDocumentProcess {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "AML_FPV" => Self::AMLFPV,
            "Bankruptcy" => Self::Bankruptcy,
            "Compliance" => Self::Compliance,
            "Identity" => Self::Identity,
            "NFA" => Self::Nfa,
            "SubAccountRequest" => Self::SubAccountRequest,
            "Watchlist" => Self::Watchlist,
            _ => Self::Unknown(value),
        })
    }
}

/// Current wire model `SubmitCustomerApplicationDocumentResponse`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct SubmitCustomerApplicationDocumentResponse {
    #[serde(rename = "errorText", default, skip_serializing_if = "Option::is_none")]
    error_text: Option<String>,
    #[serde(rename = "ok")]
    ok: bool,
    #[serde(
        rename = "documentId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    document_id: Option<super::ids::DocumentId>,
}

impl SubmitCustomerApplicationDocumentResponse {
    /// Returns wire field `errorText`.
    #[must_use]
    pub fn error_text(&self) -> Option<&str> {
        self.error_text.as_deref()
    }

    /// Returns wire field `ok`.
    #[must_use]
    pub fn ok(&self) -> &bool {
        &self.ok
    }

    /// Returns wire field `documentId`.
    #[must_use]
    pub fn document_id(&self) -> Option<&super::ids::DocumentId> {
        self.document_id.as_ref()
    }

    /// Starts a builder for [`SubmitCustomerApplicationDocumentResponse`].
    pub fn builder() -> SubmitCustomerApplicationDocumentResponseBuilder {
        SubmitCustomerApplicationDocumentResponseBuilder::default()
    }
}

/// Builder for [`SubmitCustomerApplicationDocumentResponse`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct SubmitCustomerApplicationDocumentResponseBuilder {
    error_text: Option<String>,
    ok: Option<bool>,
    document_id: Option<super::ids::DocumentId>,
}

impl SubmitCustomerApplicationDocumentResponseBuilder {
    /// Sets wire field `errorText`.
    pub fn error_text(mut self, value: impl Into<String>) -> Self {
        self.error_text = Some(value.into());
        self
    }

    /// Sets wire field `ok`.
    pub fn ok(mut self, value: bool) -> Self {
        self.ok = Some(value);
        self
    }

    /// Sets wire field `documentId`.
    pub fn document_id(mut self, value: super::ids::DocumentId) -> Self {
        self.document_id = Some(value);
        self
    }

    /// Validates required fields and builds [`SubmitCustomerApplicationDocumentResponse`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(
        self,
    ) -> Result<SubmitCustomerApplicationDocumentResponse, crate::api::current::BuildError> {
        let ok = self
            .ok
            .ok_or(crate::api::current::BuildError::missing("ok"))?;
        Ok(SubmitCustomerApplicationDocumentResponse {
            error_text: self.error_text,
            ok,
            document_id: self.document_id,
        })
    }
}

/// Current wire model `SubmitPartnerSubAccountDocument`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct SubmitPartnerSubAccountDocument {
    #[serde(rename = "subAccountRequestId")]
    sub_account_request_id: super::ids::SubAccountRequestId,
    #[serde(rename = "documentType")]
    document_type: SubmitPartnerSubAccountDocumentDocumentType,
    #[serde(rename = "filename")]
    filename: String,
    #[serde(rename = "base64data")]
    base64data: crate::api::current::SecretValue,
}

impl SubmitPartnerSubAccountDocument {
    /// Returns wire field `subAccountRequestId`.
    #[must_use]
    pub fn sub_account_request_id(&self) -> &super::ids::SubAccountRequestId {
        &self.sub_account_request_id
    }

    /// Returns wire field `documentType`.
    #[must_use]
    pub fn document_type(&self) -> &SubmitPartnerSubAccountDocumentDocumentType {
        &self.document_type
    }

    /// Returns wire field `filename`.
    #[must_use]
    pub fn filename(&self) -> &str {
        &self.filename
    }

    /// Reports whether secret field `base64data` is present.
    #[must_use]
    pub const fn has_base64data(&self) -> bool {
        true
    }

    pub(crate) fn base64data_secret(&self) -> &crate::api::current::SecretValue {
        &self.base64data
    }

    /// Starts a builder for [`SubmitPartnerSubAccountDocument`].
    pub fn builder() -> SubmitPartnerSubAccountDocumentBuilder {
        SubmitPartnerSubAccountDocumentBuilder::default()
    }
}

/// Builder for [`SubmitPartnerSubAccountDocument`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct SubmitPartnerSubAccountDocumentBuilder {
    sub_account_request_id: Option<super::ids::SubAccountRequestId>,
    document_type: Option<SubmitPartnerSubAccountDocumentDocumentType>,
    filename: Option<String>,
    base64data: Option<crate::api::current::SecretValue>,
}

impl SubmitPartnerSubAccountDocumentBuilder {
    /// Sets wire field `subAccountRequestId`.
    pub fn sub_account_request_id(mut self, value: super::ids::SubAccountRequestId) -> Self {
        self.sub_account_request_id = Some(value);
        self
    }

    /// Sets wire field `documentType`.
    pub fn document_type(mut self, value: SubmitPartnerSubAccountDocumentDocumentType) -> Self {
        self.document_type = Some(value);
        self
    }

    /// Sets wire field `filename`.
    pub fn filename(mut self, value: impl Into<String>) -> Self {
        self.filename = Some(value.into());
        self
    }

    /// Sets wire field `base64data`.
    pub fn base64data(mut self, value: crate::api::current::SecretValue) -> Self {
        self.base64data = Some(value);
        self
    }

    /// Validates required fields and builds [`SubmitPartnerSubAccountDocument`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<SubmitPartnerSubAccountDocument, crate::api::current::BuildError> {
        let sub_account_request_id =
            self.sub_account_request_id
                .ok_or(crate::api::current::BuildError::missing(
                    "subAccountRequestId",
                ))?;
        let document_type = self
            .document_type
            .ok_or(crate::api::current::BuildError::missing("documentType"))?;
        let filename = self
            .filename
            .ok_or(crate::api::current::BuildError::missing("filename"))?;
        if filename.is_empty() || filename.trim() != filename {
            return Err(crate::api::current::BuildError::invalid(
                "filename",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        let base64data = self
            .base64data
            .ok_or(crate::api::current::BuildError::missing("base64data"))?;
        Ok(SubmitPartnerSubAccountDocument {
            sub_account_request_id,
            document_type,
            filename,
            base64data,
        })
    }
}

impl crate::api::current::support::CurrentRequest for SubmitPartnerSubAccountDocument {
    fn validate_current(&self) -> Result<(), crate::Error> {
        if self.filename.is_empty() || self.filename.trim() != self.filename {
            return Err(crate::Error::InvalidRequest {
                field: "filename",
                reason: "must be non-empty and have no surrounding whitespace",
            });
        }
        Ok(())
    }
}

/// Current provider values for `SubmitPartnerSubAccountDocumentDocumentType`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum SubmitPartnerSubAccountDocumentDocumentType {
    /// Provider value `AMLBankStatement`.
    AmlBankStatement,
    /// Provider value `AMLOwnFundsLetter`.
    AmlOwnFundsLetter,
    /// Provider value `AccountReactivation`.
    AccountReactivation,
    /// Provider value `AchOther`.
    AchOther,
    /// Provider value `AdditionalRiskDisclosure`.
    AdditionalRiskDisclosure,
    /// Provider value `AddressChangeVerification`.
    AddressChangeVerification,
    /// Provider value `AddressClarificationDocVsApplication`.
    AddressClarificationDocVsApplication,
    /// Provider value `AddressClarificationDocVsConfirmedAV`.
    AddressClarificationDocVsConfirmedAv,
    /// Provider value `AddressVerificationFAFTA`.
    AddressVerificationFafta,
    /// Provider value `AddressVerificationLLC`.
    AddressVerificationLlc,
    /// Provider value `AddressVerificationManagingMember`.
    AddressVerificationManagingMember,
    /// Provider value `AmendmentApprovingFuturesTrading`.
    AmendmentApprovingFuturesTrading,
    /// Provider value `AnnualIncomeAmount`.
    AnnualIncomeAmount,
    /// Provider value `ApplicationAndIDNameMismatch`.
    ApplicationAndIdNameMismatch,
    /// Provider value `ArticlesOfOrganization`.
    ArticlesOfOrganization,
    /// Provider value `BankSupportingDocument`.
    BankSupportingDocument,
    /// Provider value `BankruptcyDischargePaperwork`.
    BankruptcyDischargePaperwork,
    /// Provider value `BeneficialOwnerCertification`.
    BeneficialOwnerCertification,
    /// Provider value `CertificateOfGoodStanding`.
    CertificateOfGoodStanding,
    /// Provider value `CompanyFinancials`.
    CompanyFinancials,
    /// Provider value `ConfirmAccountInterest`.
    ConfirmAccountInterest,
    /// Provider value `ConfirmAffiliations`.
    ConfirmAffiliations,
    /// Provider value `ConfirmFullName`.
    ConfirmFullName,
    /// Provider value `ConfirmIRACustodian`.
    ConfirmIraCustodian,
    /// Provider value `ConfirmationOfDeposits`.
    ConfirmationOfDeposits,
    /// Provider value `CorporateBylaws`.
    CorporateBylaws,
    /// Provider value `CurrentBankStatement`.
    CurrentBankStatement,
    /// Provider value `DrivingLicense`.
    DrivingLicense,
    /// Provider value `EIN`.
    Ein,
    /// Provider value `EVS`.
    Evs,
    /// Provider value `EmployeeAttestationLetter`.
    EmployeeAttestationLetter,
    /// Provider value `EmploymentDetails`.
    EmploymentDetails,
    /// Provider value `EntityWebsite`.
    EntityWebsite,
    /// Provider value `FullTimeTrader`.
    FullTimeTrader,
    /// Provider value `HedgeClarification`.
    HedgeClarification,
    /// Provider value `HighRiskApplicant`.
    HighRiskApplicant,
    /// Provider value `HomemakerIncome`.
    HomemakerIncome,
    /// Provider value `IRACustody`.
    IraCustody,
    /// Provider value `Id`.
    Id,
    /// Provider value `InvestmentType`.
    InvestmentType,
    /// Provider value `LetterOfAwareness`.
    LetterOfAwareness,
    /// Provider value `LiquidNetWorthAmount`.
    LiquidNetWorthAmount,
    /// Provider value `ManagingMemberId`.
    ManagingMemberId,
    /// Provider value `NFARegistrationConfirmation`.
    NfaRegistrationConfirmation,
    /// Provider value `NatureOfEntity`.
    NatureOfEntity,
    /// Provider value `NatureOfSelfEmployment`.
    NatureOfSelfEmployment,
    /// Provider value `NetWorthAmount`.
    NetWorthAmount,
    /// Provider value `OfferingCircular`.
    OfferingCircular,
    /// Provider value `OperatingAgreement`.
    OperatingAgreement,
    /// Provider value `Other`.
    Other,
    /// Provider value `OtherText`.
    OtherText,
    /// Provider value `OwnFunds`.
    OwnFunds,
    /// Provider value `POAForm`.
    PoaForm,
    /// Provider value `POBox`.
    PoBox,
    /// Provider value `PartnershipDocuments`.
    PartnershipDocuments,
    /// Provider value `Passport`.
    Passport,
    /// Provider value `Professional`.
    Professional,
    /// Provider value `ResidentialAddressConfirmation`.
    ResidentialAddressConfirmation,
    /// Provider value `RetirementIncome`.
    RetirementIncome,
    /// Provider value `SSNMiskey`.
    SsnMiskey,
    /// Provider value `ShareholderRegister`.
    ShareholderRegister,
    /// Provider value `SocialSecurityCard`.
    SocialSecurityCard,
    /// Provider value `SourceOfIncome`.
    SourceOfIncome,
    /// Provider value `SourceOfRiskCapital`.
    SourceOfRiskCapital,
    /// Provider value `StatementOfGoodStanding`.
    StatementOfGoodStanding,
    /// Provider value `StudentIncome`.
    StudentIncome,
    /// Provider value `ThirdPartyFunds`.
    ThirdPartyFunds,
    /// Provider value `TradingAuthorityPOA`.
    TradingAuthorityPoa,
    /// Provider value `TrustDocuments`.
    TrustDocuments,
    /// Provider value `UnderstandingOfFuturesTrading`.
    UnderstandingOfFuturesTrading,
    /// Provider value `UnemployedIncome`.
    UnemployedIncome,
    /// Provider value `UtilityBill`.
    UtilityBill,
    /// Provider value `VerifyEmailAnotherName`.
    VerifyEmailAnotherName,
    /// Provider value `VerifyEmailOwnership`.
    VerifyEmailOwnership,
    /// Provider value `VerifyEmailUsernameEntityWording`.
    VerifyEmailUsernameEntityWording,
    /// Provider value `VerifyEmailUsernameFinancialWording`.
    VerifyEmailUsernameFinancialWording,
    /// Provider value `VerifyUsernameAnotherName`.
    VerifyUsernameAnotherName,
    /// Provider value `Visa`.
    Visa,
    /// Provider value `W8Clarification`.
    W8Clarification,
    /// Provider value `W8Incomplete`.
    W8Incomplete,
    /// Provider value `W9`.
    W9,
    /// Provider value `YearsOfTradingExperience`.
    YearsOfTradingExperience,
    /// Provider value `eSignBeneficialOwnerCertification`.
    ESignBeneficialOwnerCertification,
    /// Provider value `eSignEmployeeAttestationLetter`.
    ESignEmployeeAttestationLetter,
    /// Provider value `eSignMarketDataAgreement`.
    ESignMarketDataAgreement,
    /// Provider value `eSignW9`.
    ESignW9,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl SubmitPartnerSubAccountDocumentDocumentType {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::AmlBankStatement => "AMLBankStatement",
            Self::AmlOwnFundsLetter => "AMLOwnFundsLetter",
            Self::AccountReactivation => "AccountReactivation",
            Self::AchOther => "AchOther",
            Self::AdditionalRiskDisclosure => "AdditionalRiskDisclosure",
            Self::AddressChangeVerification => "AddressChangeVerification",
            Self::AddressClarificationDocVsApplication => "AddressClarificationDocVsApplication",
            Self::AddressClarificationDocVsConfirmedAv => "AddressClarificationDocVsConfirmedAV",
            Self::AddressVerificationFafta => "AddressVerificationFAFTA",
            Self::AddressVerificationLlc => "AddressVerificationLLC",
            Self::AddressVerificationManagingMember => "AddressVerificationManagingMember",
            Self::AmendmentApprovingFuturesTrading => "AmendmentApprovingFuturesTrading",
            Self::AnnualIncomeAmount => "AnnualIncomeAmount",
            Self::ApplicationAndIdNameMismatch => "ApplicationAndIDNameMismatch",
            Self::ArticlesOfOrganization => "ArticlesOfOrganization",
            Self::BankSupportingDocument => "BankSupportingDocument",
            Self::BankruptcyDischargePaperwork => "BankruptcyDischargePaperwork",
            Self::BeneficialOwnerCertification => "BeneficialOwnerCertification",
            Self::CertificateOfGoodStanding => "CertificateOfGoodStanding",
            Self::CompanyFinancials => "CompanyFinancials",
            Self::ConfirmAccountInterest => "ConfirmAccountInterest",
            Self::ConfirmAffiliations => "ConfirmAffiliations",
            Self::ConfirmFullName => "ConfirmFullName",
            Self::ConfirmIraCustodian => "ConfirmIRACustodian",
            Self::ConfirmationOfDeposits => "ConfirmationOfDeposits",
            Self::CorporateBylaws => "CorporateBylaws",
            Self::CurrentBankStatement => "CurrentBankStatement",
            Self::DrivingLicense => "DrivingLicense",
            Self::Ein => "EIN",
            Self::Evs => "EVS",
            Self::EmployeeAttestationLetter => "EmployeeAttestationLetter",
            Self::EmploymentDetails => "EmploymentDetails",
            Self::EntityWebsite => "EntityWebsite",
            Self::FullTimeTrader => "FullTimeTrader",
            Self::HedgeClarification => "HedgeClarification",
            Self::HighRiskApplicant => "HighRiskApplicant",
            Self::HomemakerIncome => "HomemakerIncome",
            Self::IraCustody => "IRACustody",
            Self::Id => "Id",
            Self::InvestmentType => "InvestmentType",
            Self::LetterOfAwareness => "LetterOfAwareness",
            Self::LiquidNetWorthAmount => "LiquidNetWorthAmount",
            Self::ManagingMemberId => "ManagingMemberId",
            Self::NfaRegistrationConfirmation => "NFARegistrationConfirmation",
            Self::NatureOfEntity => "NatureOfEntity",
            Self::NatureOfSelfEmployment => "NatureOfSelfEmployment",
            Self::NetWorthAmount => "NetWorthAmount",
            Self::OfferingCircular => "OfferingCircular",
            Self::OperatingAgreement => "OperatingAgreement",
            Self::Other => "Other",
            Self::OtherText => "OtherText",
            Self::OwnFunds => "OwnFunds",
            Self::PoaForm => "POAForm",
            Self::PoBox => "POBox",
            Self::PartnershipDocuments => "PartnershipDocuments",
            Self::Passport => "Passport",
            Self::Professional => "Professional",
            Self::ResidentialAddressConfirmation => "ResidentialAddressConfirmation",
            Self::RetirementIncome => "RetirementIncome",
            Self::SsnMiskey => "SSNMiskey",
            Self::ShareholderRegister => "ShareholderRegister",
            Self::SocialSecurityCard => "SocialSecurityCard",
            Self::SourceOfIncome => "SourceOfIncome",
            Self::SourceOfRiskCapital => "SourceOfRiskCapital",
            Self::StatementOfGoodStanding => "StatementOfGoodStanding",
            Self::StudentIncome => "StudentIncome",
            Self::ThirdPartyFunds => "ThirdPartyFunds",
            Self::TradingAuthorityPoa => "TradingAuthorityPOA",
            Self::TrustDocuments => "TrustDocuments",
            Self::UnderstandingOfFuturesTrading => "UnderstandingOfFuturesTrading",
            Self::UnemployedIncome => "UnemployedIncome",
            Self::UtilityBill => "UtilityBill",
            Self::VerifyEmailAnotherName => "VerifyEmailAnotherName",
            Self::VerifyEmailOwnership => "VerifyEmailOwnership",
            Self::VerifyEmailUsernameEntityWording => "VerifyEmailUsernameEntityWording",
            Self::VerifyEmailUsernameFinancialWording => "VerifyEmailUsernameFinancialWording",
            Self::VerifyUsernameAnotherName => "VerifyUsernameAnotherName",
            Self::Visa => "Visa",
            Self::W8Clarification => "W8Clarification",
            Self::W8Incomplete => "W8Incomplete",
            Self::W9 => "W9",
            Self::YearsOfTradingExperience => "YearsOfTradingExperience",
            Self::ESignBeneficialOwnerCertification => "eSignBeneficialOwnerCertification",
            Self::ESignEmployeeAttestationLetter => "eSignEmployeeAttestationLetter",
            Self::ESignMarketDataAgreement => "eSignMarketDataAgreement",
            Self::ESignW9 => "eSignW9",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for SubmitPartnerSubAccountDocumentDocumentType {
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

impl<'de> serde::Deserialize<'de> for SubmitPartnerSubAccountDocumentDocumentType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "AMLBankStatement" => Self::AmlBankStatement,
            "AMLOwnFundsLetter" => Self::AmlOwnFundsLetter,
            "AccountReactivation" => Self::AccountReactivation,
            "AchOther" => Self::AchOther,
            "AdditionalRiskDisclosure" => Self::AdditionalRiskDisclosure,
            "AddressChangeVerification" => Self::AddressChangeVerification,
            "AddressClarificationDocVsApplication" => Self::AddressClarificationDocVsApplication,
            "AddressClarificationDocVsConfirmedAV" => Self::AddressClarificationDocVsConfirmedAv,
            "AddressVerificationFAFTA" => Self::AddressVerificationFafta,
            "AddressVerificationLLC" => Self::AddressVerificationLlc,
            "AddressVerificationManagingMember" => Self::AddressVerificationManagingMember,
            "AmendmentApprovingFuturesTrading" => Self::AmendmentApprovingFuturesTrading,
            "AnnualIncomeAmount" => Self::AnnualIncomeAmount,
            "ApplicationAndIDNameMismatch" => Self::ApplicationAndIdNameMismatch,
            "ArticlesOfOrganization" => Self::ArticlesOfOrganization,
            "BankSupportingDocument" => Self::BankSupportingDocument,
            "BankruptcyDischargePaperwork" => Self::BankruptcyDischargePaperwork,
            "BeneficialOwnerCertification" => Self::BeneficialOwnerCertification,
            "CertificateOfGoodStanding" => Self::CertificateOfGoodStanding,
            "CompanyFinancials" => Self::CompanyFinancials,
            "ConfirmAccountInterest" => Self::ConfirmAccountInterest,
            "ConfirmAffiliations" => Self::ConfirmAffiliations,
            "ConfirmFullName" => Self::ConfirmFullName,
            "ConfirmIRACustodian" => Self::ConfirmIraCustodian,
            "ConfirmationOfDeposits" => Self::ConfirmationOfDeposits,
            "CorporateBylaws" => Self::CorporateBylaws,
            "CurrentBankStatement" => Self::CurrentBankStatement,
            "DrivingLicense" => Self::DrivingLicense,
            "EIN" => Self::Ein,
            "EVS" => Self::Evs,
            "EmployeeAttestationLetter" => Self::EmployeeAttestationLetter,
            "EmploymentDetails" => Self::EmploymentDetails,
            "EntityWebsite" => Self::EntityWebsite,
            "FullTimeTrader" => Self::FullTimeTrader,
            "HedgeClarification" => Self::HedgeClarification,
            "HighRiskApplicant" => Self::HighRiskApplicant,
            "HomemakerIncome" => Self::HomemakerIncome,
            "IRACustody" => Self::IraCustody,
            "Id" => Self::Id,
            "InvestmentType" => Self::InvestmentType,
            "LetterOfAwareness" => Self::LetterOfAwareness,
            "LiquidNetWorthAmount" => Self::LiquidNetWorthAmount,
            "ManagingMemberId" => Self::ManagingMemberId,
            "NFARegistrationConfirmation" => Self::NfaRegistrationConfirmation,
            "NatureOfEntity" => Self::NatureOfEntity,
            "NatureOfSelfEmployment" => Self::NatureOfSelfEmployment,
            "NetWorthAmount" => Self::NetWorthAmount,
            "OfferingCircular" => Self::OfferingCircular,
            "OperatingAgreement" => Self::OperatingAgreement,
            "Other" => Self::Other,
            "OtherText" => Self::OtherText,
            "OwnFunds" => Self::OwnFunds,
            "POAForm" => Self::PoaForm,
            "POBox" => Self::PoBox,
            "PartnershipDocuments" => Self::PartnershipDocuments,
            "Passport" => Self::Passport,
            "Professional" => Self::Professional,
            "ResidentialAddressConfirmation" => Self::ResidentialAddressConfirmation,
            "RetirementIncome" => Self::RetirementIncome,
            "SSNMiskey" => Self::SsnMiskey,
            "ShareholderRegister" => Self::ShareholderRegister,
            "SocialSecurityCard" => Self::SocialSecurityCard,
            "SourceOfIncome" => Self::SourceOfIncome,
            "SourceOfRiskCapital" => Self::SourceOfRiskCapital,
            "StatementOfGoodStanding" => Self::StatementOfGoodStanding,
            "StudentIncome" => Self::StudentIncome,
            "ThirdPartyFunds" => Self::ThirdPartyFunds,
            "TradingAuthorityPOA" => Self::TradingAuthorityPoa,
            "TrustDocuments" => Self::TrustDocuments,
            "UnderstandingOfFuturesTrading" => Self::UnderstandingOfFuturesTrading,
            "UnemployedIncome" => Self::UnemployedIncome,
            "UtilityBill" => Self::UtilityBill,
            "VerifyEmailAnotherName" => Self::VerifyEmailAnotherName,
            "VerifyEmailOwnership" => Self::VerifyEmailOwnership,
            "VerifyEmailUsernameEntityWording" => Self::VerifyEmailUsernameEntityWording,
            "VerifyEmailUsernameFinancialWording" => Self::VerifyEmailUsernameFinancialWording,
            "VerifyUsernameAnotherName" => Self::VerifyUsernameAnotherName,
            "Visa" => Self::Visa,
            "W8Clarification" => Self::W8Clarification,
            "W8Incomplete" => Self::W8Incomplete,
            "W9" => Self::W9,
            "YearsOfTradingExperience" => Self::YearsOfTradingExperience,
            "eSignBeneficialOwnerCertification" => Self::ESignBeneficialOwnerCertification,
            "eSignEmployeeAttestationLetter" => Self::ESignEmployeeAttestationLetter,
            "eSignMarketDataAgreement" => Self::ESignMarketDataAgreement,
            "eSignW9" => Self::ESignW9,
            _ => Self::Unknown(value),
        })
    }
}

/// Current wire model `SubmitPartnerSubAccountDocumentResponse`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct SubmitPartnerSubAccountDocumentResponse {
    #[serde(rename = "errorText", default, skip_serializing_if = "Option::is_none")]
    error_text: Option<String>,
    #[serde(
        rename = "documentId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    document_id: Option<super::ids::DocumentId>,
}

impl SubmitPartnerSubAccountDocumentResponse {
    /// Returns wire field `errorText`.
    #[must_use]
    pub fn error_text(&self) -> Option<&str> {
        self.error_text.as_deref()
    }

    /// Returns wire field `documentId`.
    #[must_use]
    pub fn document_id(&self) -> Option<&super::ids::DocumentId> {
        self.document_id.as_ref()
    }

    /// Starts a builder for [`SubmitPartnerSubAccountDocumentResponse`].
    pub fn builder() -> SubmitPartnerSubAccountDocumentResponseBuilder {
        SubmitPartnerSubAccountDocumentResponseBuilder::default()
    }
}

/// Builder for [`SubmitPartnerSubAccountDocumentResponse`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct SubmitPartnerSubAccountDocumentResponseBuilder {
    error_text: Option<String>,
    document_id: Option<super::ids::DocumentId>,
}

impl SubmitPartnerSubAccountDocumentResponseBuilder {
    /// Sets wire field `errorText`.
    pub fn error_text(mut self, value: impl Into<String>) -> Self {
        self.error_text = Some(value.into());
        self
    }

    /// Sets wire field `documentId`.
    pub fn document_id(mut self, value: super::ids::DocumentId) -> Self {
        self.document_id = Some(value);
        self
    }

    /// Validates required fields and builds [`SubmitPartnerSubAccountDocumentResponse`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(
        self,
    ) -> Result<SubmitPartnerSubAccountDocumentResponse, crate::api::current::BuildError> {
        Ok(SubmitPartnerSubAccountDocumentResponse {
            error_text: self.error_text,
            document_id: self.document_id,
        })
    }
}

/// Current wire model `SyncMessage`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct SyncMessage {
    #[serde(rename = "users")]
    users: Vec<User>,
    #[serde(
        rename = "userProperties",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    user_properties: Option<Vec<UserProperty>>,
    #[serde(
        rename = "properties",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    properties: Option<Vec<Property>>,
    #[serde(rename = "accounts", default, skip_serializing_if = "Option::is_none")]
    accounts: Option<Vec<Account>>,
    #[serde(
        rename = "accountRiskStatuses",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    account_risk_statuses: Option<Vec<AccountRiskStatus>>,
    #[serde(
        rename = "marginSnapshots",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    margin_snapshots: Option<Vec<MarginSnapshot>>,
    #[serde(
        rename = "userAccountAutoLiqs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    user_account_auto_liqs: Option<Vec<UserAccountAutoLiq>>,
    #[serde(
        rename = "cashBalances",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    cash_balances: Option<Vec<CashBalance>>,
    #[serde(
        rename = "currencies",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    currencies: Option<Vec<Currency>>,
    #[serde(rename = "positions", default, skip_serializing_if = "Option::is_none")]
    positions: Option<Vec<Position>>,
    #[serde(rename = "fillPairs", default, skip_serializing_if = "Option::is_none")]
    fill_pairs: Option<Vec<FillPair>>,
    #[serde(rename = "orders", default, skip_serializing_if = "Option::is_none")]
    orders: Option<Vec<Order>>,
    #[serde(rename = "contracts", default, skip_serializing_if = "Option::is_none")]
    contracts: Option<Vec<Contract>>,
    #[serde(
        rename = "contractMaturities",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    contract_maturities: Option<Vec<ContractMaturity>>,
    #[serde(rename = "products", default, skip_serializing_if = "Option::is_none")]
    products: Option<Vec<Product>>,
    #[serde(rename = "exchanges", default, skip_serializing_if = "Option::is_none")]
    exchanges: Option<Vec<Exchange>>,
    #[serde(
        rename = "spreadDefinitions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    spread_definitions: Option<Vec<SpreadDefinition>>,
    #[serde(rename = "commands", default, skip_serializing_if = "Option::is_none")]
    commands: Option<Vec<Command>>,
    #[serde(
        rename = "commandReports",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    command_reports: Option<Vec<CommandReport>>,
    #[serde(
        rename = "executionReports",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    execution_reports: Option<Vec<ExecutionReport>>,
    #[serde(
        rename = "orderVersions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    order_versions: Option<Vec<OrderVersion>>,
    #[serde(rename = "fills", default, skip_serializing_if = "Option::is_none")]
    fills: Option<Vec<Fill>>,
    #[serde(rename = "fillFees", default, skip_serializing_if = "Option::is_none")]
    fill_fees: Option<Vec<FillFee>>,
    #[serde(
        rename = "orderStrategies",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    order_strategies: Option<Vec<OrderStrategy>>,
    #[serde(
        rename = "orderStrategyLinks",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    order_strategy_links: Option<Vec<OrderStrategyLink>>,
    #[serde(
        rename = "userPlugins",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    user_plugins: Option<Vec<UserPlugin>>,
    #[serde(
        rename = "annualReviews",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    annual_reviews: Option<Vec<AnnualReview>>,
    #[serde(
        rename = "userReadStatuses",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    user_read_statuses: Option<Vec<UserReadStatus>>,
    #[serde(
        rename = "userPromoCodes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    user_promo_codes: Option<Vec<UserPromoCode>>,
    #[serde(rename = "contractGroups")]
    contract_groups: Vec<ContractGroup>,
    #[serde(
        rename = "orderStrategyTypes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    order_strategy_types: Option<Vec<OrderStrategyType>>,
}

impl SyncMessage {
    /// Returns wire field `users`.
    #[must_use]
    pub fn users(&self) -> &[User] {
        &self.users
    }

    /// Returns wire field `userProperties`.
    #[must_use]
    pub fn user_properties(&self) -> Option<&[UserProperty]> {
        self.user_properties.as_deref()
    }

    /// Returns wire field `properties`.
    #[must_use]
    pub fn properties(&self) -> Option<&[Property]> {
        self.properties.as_deref()
    }

    /// Returns wire field `accounts`.
    #[must_use]
    pub fn accounts(&self) -> Option<&[Account]> {
        self.accounts.as_deref()
    }

    /// Returns wire field `accountRiskStatuses`.
    #[must_use]
    pub fn account_risk_statuses(&self) -> Option<&[AccountRiskStatus]> {
        self.account_risk_statuses.as_deref()
    }

    /// Returns wire field `marginSnapshots`.
    #[must_use]
    pub fn margin_snapshots(&self) -> Option<&[MarginSnapshot]> {
        self.margin_snapshots.as_deref()
    }

    /// Returns wire field `userAccountAutoLiqs`.
    #[must_use]
    pub fn user_account_auto_liqs(&self) -> Option<&[UserAccountAutoLiq]> {
        self.user_account_auto_liqs.as_deref()
    }

    /// Returns wire field `cashBalances`.
    #[must_use]
    pub fn cash_balances(&self) -> Option<&[CashBalance]> {
        self.cash_balances.as_deref()
    }

    /// Returns wire field `currencies`.
    #[must_use]
    pub fn currencies(&self) -> Option<&[Currency]> {
        self.currencies.as_deref()
    }

    /// Returns wire field `positions`.
    #[must_use]
    pub fn positions(&self) -> Option<&[Position]> {
        self.positions.as_deref()
    }

    /// Returns wire field `fillPairs`.
    #[must_use]
    pub fn fill_pairs(&self) -> Option<&[FillPair]> {
        self.fill_pairs.as_deref()
    }

    /// Returns wire field `orders`.
    #[must_use]
    pub fn orders(&self) -> Option<&[Order]> {
        self.orders.as_deref()
    }

    /// Returns wire field `contracts`.
    #[must_use]
    pub fn contracts(&self) -> Option<&[Contract]> {
        self.contracts.as_deref()
    }

    /// Returns wire field `contractMaturities`.
    #[must_use]
    pub fn contract_maturities(&self) -> Option<&[ContractMaturity]> {
        self.contract_maturities.as_deref()
    }

    /// Returns wire field `products`.
    #[must_use]
    pub fn products(&self) -> Option<&[Product]> {
        self.products.as_deref()
    }

    /// Returns wire field `exchanges`.
    #[must_use]
    pub fn exchanges(&self) -> Option<&[Exchange]> {
        self.exchanges.as_deref()
    }

    /// Returns wire field `spreadDefinitions`.
    #[must_use]
    pub fn spread_definitions(&self) -> Option<&[SpreadDefinition]> {
        self.spread_definitions.as_deref()
    }

    /// Returns wire field `commands`.
    #[must_use]
    pub fn commands(&self) -> Option<&[Command]> {
        self.commands.as_deref()
    }

    /// Returns wire field `commandReports`.
    #[must_use]
    pub fn command_reports(&self) -> Option<&[CommandReport]> {
        self.command_reports.as_deref()
    }

    /// Returns wire field `executionReports`.
    #[must_use]
    pub fn execution_reports(&self) -> Option<&[ExecutionReport]> {
        self.execution_reports.as_deref()
    }

    /// Returns wire field `orderVersions`.
    #[must_use]
    pub fn order_versions(&self) -> Option<&[OrderVersion]> {
        self.order_versions.as_deref()
    }

    /// Returns wire field `fills`.
    #[must_use]
    pub fn fills(&self) -> Option<&[Fill]> {
        self.fills.as_deref()
    }

    /// Returns wire field `fillFees`.
    #[must_use]
    pub fn fill_fees(&self) -> Option<&[FillFee]> {
        self.fill_fees.as_deref()
    }

    /// Returns wire field `orderStrategies`.
    #[must_use]
    pub fn order_strategies(&self) -> Option<&[OrderStrategy]> {
        self.order_strategies.as_deref()
    }

    /// Returns wire field `orderStrategyLinks`.
    #[must_use]
    pub fn order_strategy_links(&self) -> Option<&[OrderStrategyLink]> {
        self.order_strategy_links.as_deref()
    }

    /// Returns wire field `userPlugins`.
    #[must_use]
    pub fn user_plugins(&self) -> Option<&[UserPlugin]> {
        self.user_plugins.as_deref()
    }

    /// Returns wire field `annualReviews`.
    #[must_use]
    pub fn annual_reviews(&self) -> Option<&[AnnualReview]> {
        self.annual_reviews.as_deref()
    }

    /// Returns wire field `userReadStatuses`.
    #[must_use]
    pub fn user_read_statuses(&self) -> Option<&[UserReadStatus]> {
        self.user_read_statuses.as_deref()
    }

    /// Returns wire field `userPromoCodes`.
    #[must_use]
    pub fn user_promo_codes(&self) -> Option<&[UserPromoCode]> {
        self.user_promo_codes.as_deref()
    }

    /// Returns wire field `contractGroups`.
    #[must_use]
    pub fn contract_groups(&self) -> &[ContractGroup] {
        &self.contract_groups
    }

    /// Returns wire field `orderStrategyTypes`.
    #[must_use]
    pub fn order_strategy_types(&self) -> Option<&[OrderStrategyType]> {
        self.order_strategy_types.as_deref()
    }

    /// Starts a builder for [`SyncMessage`].
    pub fn builder() -> SyncMessageBuilder {
        SyncMessageBuilder::default()
    }
}

/// Builder for [`SyncMessage`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct SyncMessageBuilder {
    users: Option<Vec<User>>,
    user_properties: Option<Vec<UserProperty>>,
    properties: Option<Vec<Property>>,
    accounts: Option<Vec<Account>>,
    account_risk_statuses: Option<Vec<AccountRiskStatus>>,
    margin_snapshots: Option<Vec<MarginSnapshot>>,
    user_account_auto_liqs: Option<Vec<UserAccountAutoLiq>>,
    cash_balances: Option<Vec<CashBalance>>,
    currencies: Option<Vec<Currency>>,
    positions: Option<Vec<Position>>,
    fill_pairs: Option<Vec<FillPair>>,
    orders: Option<Vec<Order>>,
    contracts: Option<Vec<Contract>>,
    contract_maturities: Option<Vec<ContractMaturity>>,
    products: Option<Vec<Product>>,
    exchanges: Option<Vec<Exchange>>,
    spread_definitions: Option<Vec<SpreadDefinition>>,
    commands: Option<Vec<Command>>,
    command_reports: Option<Vec<CommandReport>>,
    execution_reports: Option<Vec<ExecutionReport>>,
    order_versions: Option<Vec<OrderVersion>>,
    fills: Option<Vec<Fill>>,
    fill_fees: Option<Vec<FillFee>>,
    order_strategies: Option<Vec<OrderStrategy>>,
    order_strategy_links: Option<Vec<OrderStrategyLink>>,
    user_plugins: Option<Vec<UserPlugin>>,
    annual_reviews: Option<Vec<AnnualReview>>,
    user_read_statuses: Option<Vec<UserReadStatus>>,
    user_promo_codes: Option<Vec<UserPromoCode>>,
    contract_groups: Option<Vec<ContractGroup>>,
    order_strategy_types: Option<Vec<OrderStrategyType>>,
}

impl SyncMessageBuilder {
    /// Sets wire field `users`.
    pub fn users(mut self, value: Vec<User>) -> Self {
        self.users = Some(value);
        self
    }

    /// Sets wire field `userProperties`.
    pub fn user_properties(mut self, value: Vec<UserProperty>) -> Self {
        self.user_properties = Some(value);
        self
    }

    /// Sets wire field `properties`.
    pub fn properties(mut self, value: Vec<Property>) -> Self {
        self.properties = Some(value);
        self
    }

    /// Sets wire field `accounts`.
    pub fn accounts(mut self, value: Vec<Account>) -> Self {
        self.accounts = Some(value);
        self
    }

    /// Sets wire field `accountRiskStatuses`.
    pub fn account_risk_statuses(mut self, value: Vec<AccountRiskStatus>) -> Self {
        self.account_risk_statuses = Some(value);
        self
    }

    /// Sets wire field `marginSnapshots`.
    pub fn margin_snapshots(mut self, value: Vec<MarginSnapshot>) -> Self {
        self.margin_snapshots = Some(value);
        self
    }

    /// Sets wire field `userAccountAutoLiqs`.
    pub fn user_account_auto_liqs(mut self, value: Vec<UserAccountAutoLiq>) -> Self {
        self.user_account_auto_liqs = Some(value);
        self
    }

    /// Sets wire field `cashBalances`.
    pub fn cash_balances(mut self, value: Vec<CashBalance>) -> Self {
        self.cash_balances = Some(value);
        self
    }

    /// Sets wire field `currencies`.
    pub fn currencies(mut self, value: Vec<Currency>) -> Self {
        self.currencies = Some(value);
        self
    }

    /// Sets wire field `positions`.
    pub fn positions(mut self, value: Vec<Position>) -> Self {
        self.positions = Some(value);
        self
    }

    /// Sets wire field `fillPairs`.
    pub fn fill_pairs(mut self, value: Vec<FillPair>) -> Self {
        self.fill_pairs = Some(value);
        self
    }

    /// Sets wire field `orders`.
    pub fn orders(mut self, value: Vec<Order>) -> Self {
        self.orders = Some(value);
        self
    }

    /// Sets wire field `contracts`.
    pub fn contracts(mut self, value: Vec<Contract>) -> Self {
        self.contracts = Some(value);
        self
    }

    /// Sets wire field `contractMaturities`.
    pub fn contract_maturities(mut self, value: Vec<ContractMaturity>) -> Self {
        self.contract_maturities = Some(value);
        self
    }

    /// Sets wire field `products`.
    pub fn products(mut self, value: Vec<Product>) -> Self {
        self.products = Some(value);
        self
    }

    /// Sets wire field `exchanges`.
    pub fn exchanges(mut self, value: Vec<Exchange>) -> Self {
        self.exchanges = Some(value);
        self
    }

    /// Sets wire field `spreadDefinitions`.
    pub fn spread_definitions(mut self, value: Vec<SpreadDefinition>) -> Self {
        self.spread_definitions = Some(value);
        self
    }

    /// Sets wire field `commands`.
    pub fn commands(mut self, value: Vec<Command>) -> Self {
        self.commands = Some(value);
        self
    }

    /// Sets wire field `commandReports`.
    pub fn command_reports(mut self, value: Vec<CommandReport>) -> Self {
        self.command_reports = Some(value);
        self
    }

    /// Sets wire field `executionReports`.
    pub fn execution_reports(mut self, value: Vec<ExecutionReport>) -> Self {
        self.execution_reports = Some(value);
        self
    }

    /// Sets wire field `orderVersions`.
    pub fn order_versions(mut self, value: Vec<OrderVersion>) -> Self {
        self.order_versions = Some(value);
        self
    }

    /// Sets wire field `fills`.
    pub fn fills(mut self, value: Vec<Fill>) -> Self {
        self.fills = Some(value);
        self
    }

    /// Sets wire field `fillFees`.
    pub fn fill_fees(mut self, value: Vec<FillFee>) -> Self {
        self.fill_fees = Some(value);
        self
    }

    /// Sets wire field `orderStrategies`.
    pub fn order_strategies(mut self, value: Vec<OrderStrategy>) -> Self {
        self.order_strategies = Some(value);
        self
    }

    /// Sets wire field `orderStrategyLinks`.
    pub fn order_strategy_links(mut self, value: Vec<OrderStrategyLink>) -> Self {
        self.order_strategy_links = Some(value);
        self
    }

    /// Sets wire field `userPlugins`.
    pub fn user_plugins(mut self, value: Vec<UserPlugin>) -> Self {
        self.user_plugins = Some(value);
        self
    }

    /// Sets wire field `annualReviews`.
    pub fn annual_reviews(mut self, value: Vec<AnnualReview>) -> Self {
        self.annual_reviews = Some(value);
        self
    }

    /// Sets wire field `userReadStatuses`.
    pub fn user_read_statuses(mut self, value: Vec<UserReadStatus>) -> Self {
        self.user_read_statuses = Some(value);
        self
    }

    /// Sets wire field `userPromoCodes`.
    pub fn user_promo_codes(mut self, value: Vec<UserPromoCode>) -> Self {
        self.user_promo_codes = Some(value);
        self
    }

    /// Sets wire field `contractGroups`.
    pub fn contract_groups(mut self, value: Vec<ContractGroup>) -> Self {
        self.contract_groups = Some(value);
        self
    }

    /// Sets wire field `orderStrategyTypes`.
    pub fn order_strategy_types(mut self, value: Vec<OrderStrategyType>) -> Self {
        self.order_strategy_types = Some(value);
        self
    }

    /// Validates required fields and builds [`SyncMessage`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<SyncMessage, crate::api::current::BuildError> {
        let users = self
            .users
            .ok_or(crate::api::current::BuildError::missing("users"))?;
        let contract_groups = self
            .contract_groups
            .ok_or(crate::api::current::BuildError::missing("contractGroups"))?;
        Ok(SyncMessage {
            users,
            user_properties: self.user_properties,
            properties: self.properties,
            accounts: self.accounts,
            account_risk_statuses: self.account_risk_statuses,
            margin_snapshots: self.margin_snapshots,
            user_account_auto_liqs: self.user_account_auto_liqs,
            cash_balances: self.cash_balances,
            currencies: self.currencies,
            positions: self.positions,
            fill_pairs: self.fill_pairs,
            orders: self.orders,
            contracts: self.contracts,
            contract_maturities: self.contract_maturities,
            products: self.products,
            exchanges: self.exchanges,
            spread_definitions: self.spread_definitions,
            commands: self.commands,
            command_reports: self.command_reports,
            execution_reports: self.execution_reports,
            order_versions: self.order_versions,
            fills: self.fills,
            fill_fees: self.fill_fees,
            order_strategies: self.order_strategies,
            order_strategy_links: self.order_strategy_links,
            user_plugins: self.user_plugins,
            annual_reviews: self.annual_reviews,
            user_read_statuses: self.user_read_statuses,
            user_promo_codes: self.user_promo_codes,
            contract_groups,
            order_strategy_types: self.order_strategy_types,
        })
    }
}

/// Current wire model `SyncRequest`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct SyncRequest {
    #[serde(rename = "users", default, skip_serializing_if = "Option::is_none")]
    users: Option<Vec<i64>>,
    #[serde(rename = "accounts", default, skip_serializing_if = "Option::is_none")]
    accounts: Option<Vec<i64>>,
    #[serde(
        rename = "splitResponses",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    split_responses: Option<bool>,
    #[serde(
        rename = "cutoffTimestamp",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    cutoff_timestamp: Option<jiff::Timestamp>,
    #[serde(
        rename = "entityTypes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    entity_types: Option<Vec<String>>,
    #[serde(
        rename = "shardingExpression",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    sharding_expression: Option<ShardingExpression>,
    #[serde(
        rename = "fullOrgSnapshot",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    full_org_snapshot: Option<bool>,
}

impl SyncRequest {
    /// Returns wire field `users`.
    #[must_use]
    pub fn users(&self) -> Option<&[i64]> {
        self.users.as_deref()
    }

    /// Returns wire field `accounts`.
    #[must_use]
    pub fn accounts(&self) -> Option<&[i64]> {
        self.accounts.as_deref()
    }

    /// Returns wire field `splitResponses`.
    #[must_use]
    pub fn split_responses(&self) -> Option<&bool> {
        self.split_responses.as_ref()
    }

    /// Returns wire field `cutoffTimestamp`.
    #[must_use]
    pub fn cutoff_timestamp(&self) -> Option<&jiff::Timestamp> {
        self.cutoff_timestamp.as_ref()
    }

    /// Returns wire field `entityTypes`.
    #[must_use]
    pub fn entity_types(&self) -> Option<&[String]> {
        self.entity_types.as_deref()
    }

    /// Returns wire field `shardingExpression`.
    #[must_use]
    pub fn sharding_expression(&self) -> Option<&ShardingExpression> {
        self.sharding_expression.as_ref()
    }

    /// Returns wire field `fullOrgSnapshot`.
    #[must_use]
    pub fn full_org_snapshot(&self) -> Option<&bool> {
        self.full_org_snapshot.as_ref()
    }

    /// Starts a builder for [`SyncRequest`].
    pub fn builder() -> SyncRequestBuilder {
        SyncRequestBuilder::default()
    }
}

/// Builder for [`SyncRequest`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct SyncRequestBuilder {
    users: Option<Vec<i64>>,
    accounts: Option<Vec<i64>>,
    split_responses: Option<bool>,
    cutoff_timestamp: Option<jiff::Timestamp>,
    entity_types: Option<Vec<String>>,
    sharding_expression: Option<ShardingExpression>,
    full_org_snapshot: Option<bool>,
}

impl SyncRequestBuilder {
    /// Sets wire field `users`.
    pub fn users(mut self, value: Vec<i64>) -> Self {
        self.users = Some(value);
        self
    }

    /// Sets wire field `accounts`.
    pub fn accounts(mut self, value: Vec<i64>) -> Self {
        self.accounts = Some(value);
        self
    }

    /// Sets wire field `splitResponses`.
    pub fn split_responses(mut self, value: bool) -> Self {
        self.split_responses = Some(value);
        self
    }

    /// Sets wire field `cutoffTimestamp`.
    pub fn cutoff_timestamp(mut self, value: jiff::Timestamp) -> Self {
        self.cutoff_timestamp = Some(value);
        self
    }

    /// Sets wire field `entityTypes`.
    pub fn entity_types(mut self, value: Vec<String>) -> Self {
        self.entity_types = Some(value);
        self
    }

    /// Sets wire field `shardingExpression`.
    pub fn sharding_expression(mut self, value: ShardingExpression) -> Self {
        self.sharding_expression = Some(value);
        self
    }

    /// Sets wire field `fullOrgSnapshot`.
    pub fn full_org_snapshot(mut self, value: bool) -> Self {
        self.full_org_snapshot = Some(value);
        self
    }

    /// Validates required fields and builds [`SyncRequest`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<SyncRequest, crate::api::current::BuildError> {
        Ok(SyncRequest {
            users: self.users,
            accounts: self.accounts,
            split_responses: self.split_responses,
            cutoff_timestamp: self.cutoff_timestamp,
            entity_types: self.entity_types,
            sharding_expression: self.sharding_expression,
            full_org_snapshot: self.full_org_snapshot,
        })
    }
}

impl crate::api::current::support::CurrentRequest for SyncRequest {
    fn validate_current(&self) -> Result<(), crate::Error> {
        Ok(())
    }
}

/// Current wire model `TradeDate`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct TradeDate {
    #[serde(rename = "year")]
    year: i64,
    #[serde(rename = "month")]
    month: i64,
    #[serde(rename = "day")]
    day: i64,
}

impl TradeDate {
    /// Returns wire field `year`.
    #[must_use]
    pub fn year(&self) -> &i64 {
        &self.year
    }

    /// Returns wire field `month`.
    #[must_use]
    pub fn month(&self) -> &i64 {
        &self.month
    }

    /// Returns wire field `day`.
    #[must_use]
    pub fn day(&self) -> &i64 {
        &self.day
    }

    /// Starts a builder for [`TradeDate`].
    pub fn builder() -> TradeDateBuilder {
        TradeDateBuilder::default()
    }
}

/// Builder for [`TradeDate`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct TradeDateBuilder {
    year: Option<i64>,
    month: Option<i64>,
    day: Option<i64>,
}

impl TradeDateBuilder {
    /// Sets wire field `year`.
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    /// Sets wire field `month`.
    pub fn month(mut self, value: i64) -> Self {
        self.month = Some(value);
        self
    }

    /// Sets wire field `day`.
    pub fn day(mut self, value: i64) -> Self {
        self.day = Some(value);
        self
    }

    /// Validates required fields and builds [`TradeDate`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<TradeDate, crate::api::current::BuildError> {
        let year = self
            .year
            .ok_or(crate::api::current::BuildError::missing("year"))?;
        let month = self
            .month
            .ok_or(crate::api::current::BuildError::missing("month"))?;
        let day = self
            .day
            .ok_or(crate::api::current::BuildError::missing("day"))?;
        Ok(TradeDate { year, month, day })
    }
}

/// Current wire model `TradingPermission`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct TradingPermission {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    id: Option<super::ids::TradingPermissionId>,
    #[serde(rename = "userId")]
    user_id: crate::UserId,
    #[serde(rename = "accountId")]
    account_id: crate::AccountId,
    #[serde(rename = "accountHolderContact")]
    account_holder_contact: String,
    #[serde(rename = "accountHolderEmail")]
    account_holder_email: String,
    #[serde(rename = "ctaContact")]
    cta_contact: String,
    #[serde(rename = "ctaEmail")]
    cta_email: String,
    #[serde(rename = "status")]
    status: TradingPermissionStatus,
    #[serde(rename = "updated", default, skip_serializing_if = "Option::is_none")]
    updated: Option<jiff::Timestamp>,
    #[serde(
        rename = "approvedById",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    approved_by_id: Option<super::ids::ApprovedById>,
}

impl TradingPermission {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> Option<&super::ids::TradingPermissionId> {
        self.id.as_ref()
    }

    /// Returns wire field `userId`.
    #[must_use]
    pub fn user_id(&self) -> &crate::UserId {
        &self.user_id
    }

    /// Returns wire field `accountId`.
    #[must_use]
    pub fn account_id(&self) -> &crate::AccountId {
        &self.account_id
    }

    /// Returns wire field `accountHolderContact`.
    #[must_use]
    pub fn account_holder_contact(&self) -> &str {
        &self.account_holder_contact
    }

    /// Returns wire field `accountHolderEmail`.
    #[must_use]
    pub fn account_holder_email(&self) -> &str {
        &self.account_holder_email
    }

    /// Returns wire field `ctaContact`.
    #[must_use]
    pub fn cta_contact(&self) -> &str {
        &self.cta_contact
    }

    /// Returns wire field `ctaEmail`.
    #[must_use]
    pub fn cta_email(&self) -> &str {
        &self.cta_email
    }

    /// Returns wire field `status`.
    #[must_use]
    pub fn status(&self) -> &TradingPermissionStatus {
        &self.status
    }

    /// Returns wire field `updated`.
    #[must_use]
    pub fn updated(&self) -> Option<&jiff::Timestamp> {
        self.updated.as_ref()
    }

    /// Returns wire field `approvedById`.
    #[must_use]
    pub fn approved_by_id(&self) -> Option<&super::ids::ApprovedById> {
        self.approved_by_id.as_ref()
    }

    /// Starts a builder for [`TradingPermission`].
    pub fn builder() -> TradingPermissionBuilder {
        TradingPermissionBuilder::default()
    }
}

/// Builder for [`TradingPermission`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct TradingPermissionBuilder {
    id: Option<super::ids::TradingPermissionId>,
    user_id: Option<crate::UserId>,
    account_id: Option<crate::AccountId>,
    account_holder_contact: Option<String>,
    account_holder_email: Option<String>,
    cta_contact: Option<String>,
    cta_email: Option<String>,
    status: Option<TradingPermissionStatus>,
    updated: Option<jiff::Timestamp>,
    approved_by_id: Option<super::ids::ApprovedById>,
}

impl TradingPermissionBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::TradingPermissionId) -> Self {
        self.id = Some(value);
        self
    }

    /// Sets wire field `userId`.
    pub fn user_id(mut self, value: crate::UserId) -> Self {
        self.user_id = Some(value);
        self
    }

    /// Sets wire field `accountId`.
    pub fn account_id(mut self, value: crate::AccountId) -> Self {
        self.account_id = Some(value);
        self
    }

    /// Sets wire field `accountHolderContact`.
    pub fn account_holder_contact(mut self, value: impl Into<String>) -> Self {
        self.account_holder_contact = Some(value.into());
        self
    }

    /// Sets wire field `accountHolderEmail`.
    pub fn account_holder_email(mut self, value: impl Into<String>) -> Self {
        self.account_holder_email = Some(value.into());
        self
    }

    /// Sets wire field `ctaContact`.
    pub fn cta_contact(mut self, value: impl Into<String>) -> Self {
        self.cta_contact = Some(value.into());
        self
    }

    /// Sets wire field `ctaEmail`.
    pub fn cta_email(mut self, value: impl Into<String>) -> Self {
        self.cta_email = Some(value.into());
        self
    }

    /// Sets wire field `status`.
    pub fn status(mut self, value: TradingPermissionStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Sets wire field `updated`.
    pub fn updated(mut self, value: jiff::Timestamp) -> Self {
        self.updated = Some(value);
        self
    }

    /// Sets wire field `approvedById`.
    pub fn approved_by_id(mut self, value: super::ids::ApprovedById) -> Self {
        self.approved_by_id = Some(value);
        self
    }

    /// Validates required fields and builds [`TradingPermission`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<TradingPermission, crate::api::current::BuildError> {
        let user_id = self
            .user_id
            .ok_or(crate::api::current::BuildError::missing("userId"))?;
        let account_id = self
            .account_id
            .ok_or(crate::api::current::BuildError::missing("accountId"))?;
        let account_holder_contact =
            self.account_holder_contact
                .ok_or(crate::api::current::BuildError::missing(
                    "accountHolderContact",
                ))?;
        let account_holder_email =
            self.account_holder_email
                .ok_or(crate::api::current::BuildError::missing(
                    "accountHolderEmail",
                ))?;
        let cta_contact = self
            .cta_contact
            .ok_or(crate::api::current::BuildError::missing("ctaContact"))?;
        let cta_email = self
            .cta_email
            .ok_or(crate::api::current::BuildError::missing("ctaEmail"))?;
        let status = self
            .status
            .ok_or(crate::api::current::BuildError::missing("status"))?;
        Ok(TradingPermission {
            id: self.id,
            user_id,
            account_id,
            account_holder_contact,
            account_holder_email,
            cta_contact,
            cta_email,
            status,
            updated: self.updated,
            approved_by_id: self.approved_by_id,
        })
    }
}

/// Current wire model `TradingPermissionResponse`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct TradingPermissionResponse {
    #[serde(rename = "errorText", default, skip_serializing_if = "Option::is_none")]
    error_text: Option<String>,
    #[serde(
        rename = "tradingPermission",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    trading_permission: Option<TradingPermission>,
}

impl TradingPermissionResponse {
    /// Returns wire field `errorText`.
    #[must_use]
    pub fn error_text(&self) -> Option<&str> {
        self.error_text.as_deref()
    }

    /// Returns wire field `tradingPermission`.
    #[must_use]
    pub fn trading_permission(&self) -> Option<&TradingPermission> {
        self.trading_permission.as_ref()
    }

    /// Starts a builder for [`TradingPermissionResponse`].
    pub fn builder() -> TradingPermissionResponseBuilder {
        TradingPermissionResponseBuilder::default()
    }
}

/// Builder for [`TradingPermissionResponse`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct TradingPermissionResponseBuilder {
    error_text: Option<String>,
    trading_permission: Option<TradingPermission>,
}

impl TradingPermissionResponseBuilder {
    /// Sets wire field `errorText`.
    pub fn error_text(mut self, value: impl Into<String>) -> Self {
        self.error_text = Some(value.into());
        self
    }

    /// Sets wire field `tradingPermission`.
    pub fn trading_permission(mut self, value: TradingPermission) -> Self {
        self.trading_permission = Some(value);
        self
    }

    /// Validates required fields and builds [`TradingPermissionResponse`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<TradingPermissionResponse, crate::api::current::BuildError> {
        Ok(TradingPermissionResponse {
            error_text: self.error_text,
            trading_permission: self.trading_permission,
        })
    }
}

/// Current provider values for `TradingPermissionStatus`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum TradingPermissionStatus {
    /// Provider value `Accepted`.
    Accepted,
    /// Provider value `Approved`.
    Approved,
    /// Provider value `Declined`.
    Declined,
    /// Provider value `Requested`.
    Requested,
    /// Provider value `Revoked`.
    Revoked,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl TradingPermissionStatus {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Accepted => "Accepted",
            Self::Approved => "Approved",
            Self::Declined => "Declined",
            Self::Requested => "Requested",
            Self::Revoked => "Revoked",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for TradingPermissionStatus {
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

impl<'de> serde::Deserialize<'de> for TradingPermissionStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "Accepted" => Self::Accepted,
            "Approved" => Self::Approved,
            "Declined" => Self::Declined,
            "Requested" => Self::Requested,
            "Revoked" => Self::Revoked,
            _ => Self::Unknown(value),
        })
    }
}

/// Current wire model `TradingPermissionsResponse`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct TradingPermissionsResponse {
    #[serde(rename = "tradingPermissions")]
    trading_permissions: Vec<TradingPermission>,
}

impl TradingPermissionsResponse {
    /// Returns wire field `tradingPermissions`.
    #[must_use]
    pub fn trading_permissions(&self) -> &[TradingPermission] {
        &self.trading_permissions
    }

    /// Starts a builder for [`TradingPermissionsResponse`].
    pub fn builder() -> TradingPermissionsResponseBuilder {
        TradingPermissionsResponseBuilder::default()
    }
}

/// Builder for [`TradingPermissionsResponse`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct TradingPermissionsResponseBuilder {
    trading_permissions: Option<Vec<TradingPermission>>,
}

impl TradingPermissionsResponseBuilder {
    /// Sets wire field `tradingPermissions`.
    pub fn trading_permissions(mut self, value: Vec<TradingPermission>) -> Self {
        self.trading_permissions = Some(value);
        self
    }

    /// Validates required fields and builds [`TradingPermissionsResponse`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<TradingPermissionsResponse, crate::api::current::BuildError> {
        let trading_permissions =
            self.trading_permissions
                .ok_or(crate::api::current::BuildError::missing(
                    "tradingPermissions",
                ))?;
        Ok(TradingPermissionsResponse {
            trading_permissions,
        })
    }
}

/// Current wire model `TradovateSubscription`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct TradovateSubscription {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    id: Option<super::ids::TradovateSubscriptionId>,
    #[serde(rename = "userId")]
    user_id: crate::UserId,
    #[serde(rename = "timestamp")]
    timestamp: jiff::Timestamp,
    #[serde(rename = "planPrice")]
    #[serde(with = "crate::decimal")]
    plan_price: crate::Decimal,
    #[serde(
        rename = "cashBalanceLogId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    cash_balance_log_id: Option<super::ids::CashBalanceLogId>,
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    account_id: Option<crate::AccountId>,
    #[serde(rename = "tradovateSubscriptionPlanId")]
    tradovate_subscription_plan_id: super::ids::TradovateSubscriptionPlanId,
    #[serde(rename = "startDate")]
    start_date: TradeDate,
    #[serde(rename = "expirationDate")]
    expiration_date: TradeDate,
    #[serde(rename = "paidAmount")]
    #[serde(with = "crate::decimal")]
    paid_amount: crate::Decimal,
    #[serde(
        rename = "cancelledRenewal",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    cancelled_renewal: Option<bool>,
    #[serde(
        rename = "cancelReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    cancel_reason: Option<String>,
}

impl TradovateSubscription {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> Option<&super::ids::TradovateSubscriptionId> {
        self.id.as_ref()
    }

    /// Returns wire field `userId`.
    #[must_use]
    pub fn user_id(&self) -> &crate::UserId {
        &self.user_id
    }

    /// Returns wire field `timestamp`.
    #[must_use]
    pub fn timestamp(&self) -> &jiff::Timestamp {
        &self.timestamp
    }

    /// Returns wire field `planPrice`.
    #[must_use]
    pub fn plan_price(&self) -> &crate::Decimal {
        &self.plan_price
    }

    /// Returns wire field `cashBalanceLogId`.
    #[must_use]
    pub fn cash_balance_log_id(&self) -> Option<&super::ids::CashBalanceLogId> {
        self.cash_balance_log_id.as_ref()
    }

    /// Returns wire field `accountId`.
    #[must_use]
    pub fn account_id(&self) -> Option<&crate::AccountId> {
        self.account_id.as_ref()
    }

    /// Returns wire field `tradovateSubscriptionPlanId`.
    #[must_use]
    pub fn tradovate_subscription_plan_id(&self) -> &super::ids::TradovateSubscriptionPlanId {
        &self.tradovate_subscription_plan_id
    }

    /// Returns wire field `startDate`.
    #[must_use]
    pub fn start_date(&self) -> &TradeDate {
        &self.start_date
    }

    /// Returns wire field `expirationDate`.
    #[must_use]
    pub fn expiration_date(&self) -> &TradeDate {
        &self.expiration_date
    }

    /// Returns wire field `paidAmount`.
    #[must_use]
    pub fn paid_amount(&self) -> &crate::Decimal {
        &self.paid_amount
    }

    /// Returns wire field `cancelledRenewal`.
    #[must_use]
    pub fn cancelled_renewal(&self) -> Option<&bool> {
        self.cancelled_renewal.as_ref()
    }

    /// Returns wire field `cancelReason`.
    #[must_use]
    pub fn cancel_reason(&self) -> Option<&str> {
        self.cancel_reason.as_deref()
    }

    /// Starts a builder for [`TradovateSubscription`].
    pub fn builder() -> TradovateSubscriptionBuilder {
        TradovateSubscriptionBuilder::default()
    }
}

/// Builder for [`TradovateSubscription`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct TradovateSubscriptionBuilder {
    id: Option<super::ids::TradovateSubscriptionId>,
    user_id: Option<crate::UserId>,
    timestamp: Option<jiff::Timestamp>,
    plan_price: Option<crate::Decimal>,
    cash_balance_log_id: Option<super::ids::CashBalanceLogId>,
    account_id: Option<crate::AccountId>,
    tradovate_subscription_plan_id: Option<super::ids::TradovateSubscriptionPlanId>,
    start_date: Option<TradeDate>,
    expiration_date: Option<TradeDate>,
    paid_amount: Option<crate::Decimal>,
    cancelled_renewal: Option<bool>,
    cancel_reason: Option<String>,
}

impl TradovateSubscriptionBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::TradovateSubscriptionId) -> Self {
        self.id = Some(value);
        self
    }

    /// Sets wire field `userId`.
    pub fn user_id(mut self, value: crate::UserId) -> Self {
        self.user_id = Some(value);
        self
    }

    /// Sets wire field `timestamp`.
    pub fn timestamp(mut self, value: jiff::Timestamp) -> Self {
        self.timestamp = Some(value);
        self
    }

    /// Sets wire field `planPrice`.
    pub fn plan_price(mut self, value: crate::Decimal) -> Self {
        self.plan_price = Some(value);
        self
    }

    /// Sets wire field `cashBalanceLogId`.
    pub fn cash_balance_log_id(mut self, value: super::ids::CashBalanceLogId) -> Self {
        self.cash_balance_log_id = Some(value);
        self
    }

    /// Sets wire field `accountId`.
    pub fn account_id(mut self, value: crate::AccountId) -> Self {
        self.account_id = Some(value);
        self
    }

    /// Sets wire field `tradovateSubscriptionPlanId`.
    pub fn tradovate_subscription_plan_id(
        mut self,
        value: super::ids::TradovateSubscriptionPlanId,
    ) -> Self {
        self.tradovate_subscription_plan_id = Some(value);
        self
    }

    /// Sets wire field `startDate`.
    pub fn start_date(mut self, value: TradeDate) -> Self {
        self.start_date = Some(value);
        self
    }

    /// Sets wire field `expirationDate`.
    pub fn expiration_date(mut self, value: TradeDate) -> Self {
        self.expiration_date = Some(value);
        self
    }

    /// Sets wire field `paidAmount`.
    pub fn paid_amount(mut self, value: crate::Decimal) -> Self {
        self.paid_amount = Some(value);
        self
    }

    /// Sets wire field `cancelledRenewal`.
    pub fn cancelled_renewal(mut self, value: bool) -> Self {
        self.cancelled_renewal = Some(value);
        self
    }

    /// Sets wire field `cancelReason`.
    pub fn cancel_reason(mut self, value: impl Into<String>) -> Self {
        self.cancel_reason = Some(value.into());
        self
    }

    /// Validates required fields and builds [`TradovateSubscription`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<TradovateSubscription, crate::api::current::BuildError> {
        let user_id = self
            .user_id
            .ok_or(crate::api::current::BuildError::missing("userId"))?;
        let timestamp = self
            .timestamp
            .ok_or(crate::api::current::BuildError::missing("timestamp"))?;
        let plan_price = self
            .plan_price
            .ok_or(crate::api::current::BuildError::missing("planPrice"))?;
        let tradovate_subscription_plan_id =
            self.tradovate_subscription_plan_id
                .ok_or(crate::api::current::BuildError::missing(
                    "tradovateSubscriptionPlanId",
                ))?;
        let start_date = self
            .start_date
            .ok_or(crate::api::current::BuildError::missing("startDate"))?;
        let expiration_date = self
            .expiration_date
            .ok_or(crate::api::current::BuildError::missing("expirationDate"))?;
        let paid_amount = self
            .paid_amount
            .ok_or(crate::api::current::BuildError::missing("paidAmount"))?;
        Ok(TradovateSubscription {
            id: self.id,
            user_id,
            timestamp,
            plan_price,
            cash_balance_log_id: self.cash_balance_log_id,
            account_id: self.account_id,
            tradovate_subscription_plan_id,
            start_date,
            expiration_date,
            paid_amount,
            cancelled_renewal: self.cancelled_renewal,
            cancel_reason: self.cancel_reason,
        })
    }
}

impl crate::api::current::support::CurrentRequest for TradovateSubscription {
    fn validate_current(&self) -> Result<(), crate::Error> {
        Ok(())
    }
}

/// Current wire model `TradovateSubscriptionResponse`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct TradovateSubscriptionResponse {
    #[serde(rename = "errorText", default, skip_serializing_if = "Option::is_none")]
    error_text: Option<String>,
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    error_code: Option<TradovateSubscriptionResponseErrorCode>,
    #[serde(
        rename = "tradovateSubscription",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    tradovate_subscription: Option<TradovateSubscription>,
}

impl TradovateSubscriptionResponse {
    /// Returns wire field `errorText`.
    #[must_use]
    pub fn error_text(&self) -> Option<&str> {
        self.error_text.as_deref()
    }

    /// Returns wire field `errorCode`.
    #[must_use]
    pub fn error_code(&self) -> Option<&TradovateSubscriptionResponseErrorCode> {
        self.error_code.as_ref()
    }

    /// Returns wire field `tradovateSubscription`.
    #[must_use]
    pub fn tradovate_subscription(&self) -> Option<&TradovateSubscription> {
        self.tradovate_subscription.as_ref()
    }

    /// Starts a builder for [`TradovateSubscriptionResponse`].
    pub fn builder() -> TradovateSubscriptionResponseBuilder {
        TradovateSubscriptionResponseBuilder::default()
    }
}

/// Builder for [`TradovateSubscriptionResponse`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct TradovateSubscriptionResponseBuilder {
    error_text: Option<String>,
    error_code: Option<TradovateSubscriptionResponseErrorCode>,
    tradovate_subscription: Option<TradovateSubscription>,
}

impl TradovateSubscriptionResponseBuilder {
    /// Sets wire field `errorText`.
    pub fn error_text(mut self, value: impl Into<String>) -> Self {
        self.error_text = Some(value.into());
        self
    }

    /// Sets wire field `errorCode`.
    pub fn error_code(mut self, value: TradovateSubscriptionResponseErrorCode) -> Self {
        self.error_code = Some(value);
        self
    }

    /// Sets wire field `tradovateSubscription`.
    pub fn tradovate_subscription(mut self, value: TradovateSubscription) -> Self {
        self.tradovate_subscription = Some(value);
        self
    }

    /// Validates required fields and builds [`TradovateSubscriptionResponse`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<TradovateSubscriptionResponse, crate::api::current::BuildError> {
        Ok(TradovateSubscriptionResponse {
            error_text: self.error_text,
            error_code: self.error_code,
            tradovate_subscription: self.tradovate_subscription,
        })
    }
}

/// Current provider values for `TradovateSubscriptionResponseErrorCode`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum TradovateSubscriptionResponseErrorCode {
    /// Provider value `ConflictWithExisting`.
    ConflictWithExisting,
    /// Provider value `DowngradeNotAllowed`.
    DowngradeNotAllowed,
    /// Provider value `IncompatibleCMEMarketDataSubscriptionPlans`.
    IncompatibleCmeMarketDataSubscriptionPlans,
    /// Provider value `IncorrectPaymentMethod`.
    IncorrectPaymentMethod,
    /// Provider value `InsufficientFunds`.
    InsufficientFunds,
    /// Provider value `PaymentProviderError`.
    PaymentProviderError,
    /// Provider value `PlanDiscontinued`.
    PlanDiscontinued,
    /// Provider value `SingleTrialOnly`.
    SingleTrialOnly,
    /// Provider value `Success`.
    Success,
    /// Provider value `UnknownError`.
    UnknownError,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl TradovateSubscriptionResponseErrorCode {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::ConflictWithExisting => "ConflictWithExisting",
            Self::DowngradeNotAllowed => "DowngradeNotAllowed",
            Self::IncompatibleCmeMarketDataSubscriptionPlans => {
                "IncompatibleCMEMarketDataSubscriptionPlans"
            }
            Self::IncorrectPaymentMethod => "IncorrectPaymentMethod",
            Self::InsufficientFunds => "InsufficientFunds",
            Self::PaymentProviderError => "PaymentProviderError",
            Self::PlanDiscontinued => "PlanDiscontinued",
            Self::SingleTrialOnly => "SingleTrialOnly",
            Self::Success => "Success",
            Self::UnknownError => "UnknownError",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for TradovateSubscriptionResponseErrorCode {
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

impl<'de> serde::Deserialize<'de> for TradovateSubscriptionResponseErrorCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "ConflictWithExisting" => Self::ConflictWithExisting,
            "DowngradeNotAllowed" => Self::DowngradeNotAllowed,
            "IncompatibleCMEMarketDataSubscriptionPlans" => {
                Self::IncompatibleCmeMarketDataSubscriptionPlans
            }
            "IncorrectPaymentMethod" => Self::IncorrectPaymentMethod,
            "InsufficientFunds" => Self::InsufficientFunds,
            "PaymentProviderError" => Self::PaymentProviderError,
            "PlanDiscontinued" => Self::PlanDiscontinued,
            "SingleTrialOnly" => Self::SingleTrialOnly,
            "Success" => Self::Success,
            "UnknownError" => Self::UnknownError,
            _ => Self::Unknown(value),
        })
    }
}

/// Current wire model `UpdateContactCountry`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct UpdateContactCountry {
    #[serde(rename = "country")]
    country: String,
}

impl UpdateContactCountry {
    /// Returns wire field `country`.
    #[must_use]
    pub fn country(&self) -> &str {
        &self.country
    }

    /// Starts a builder for [`UpdateContactCountry`].
    pub fn builder() -> UpdateContactCountryBuilder {
        UpdateContactCountryBuilder::default()
    }
}

/// Builder for [`UpdateContactCountry`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct UpdateContactCountryBuilder {
    country: Option<String>,
}

impl UpdateContactCountryBuilder {
    /// Sets wire field `country`.
    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    /// Validates required fields and builds [`UpdateContactCountry`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<UpdateContactCountry, crate::api::current::BuildError> {
        let country = self
            .country
            .ok_or(crate::api::current::BuildError::missing("country"))?;
        if country.is_empty() || country.trim() != country {
            return Err(crate::api::current::BuildError::invalid(
                "country",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        Ok(UpdateContactCountry { country })
    }
}

impl crate::api::current::support::CurrentRequest for UpdateContactCountry {
    fn validate_current(&self) -> Result<(), crate::Error> {
        if self.country.is_empty() || self.country.trim() != self.country {
            return Err(crate::Error::InvalidRequest {
                field: "country",
                reason: "must be non-empty and have no surrounding whitespace",
            });
        }
        Ok(())
    }
}

/// Current wire model `UpdateContactInfo`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct UpdateContactInfo {
    #[serde(rename = "userId")]
    user_id: crate::UserId,
    #[serde(rename = "firstName")]
    first_name: String,
    #[serde(rename = "lastName")]
    last_name: String,
    #[serde(rename = "streetAddress1")]
    street_address1: String,
    #[serde(
        rename = "streetAddress2",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    street_address2: Option<String>,
    #[serde(rename = "city")]
    city: String,
    #[serde(rename = "state", default, skip_serializing_if = "Option::is_none")]
    state: Option<String>,
    #[serde(rename = "postCode", default, skip_serializing_if = "Option::is_none")]
    post_code: Option<String>,
    #[serde(rename = "country")]
    country: String,
    #[serde(rename = "phone")]
    phone: String,
    #[serde(
        rename = "mailingIsDifferent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    mailing_is_different: Option<bool>,
    #[serde(
        rename = "mailingStreetAddress1",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    mailing_street_address1: Option<String>,
    #[serde(
        rename = "mailingStreetAddress2",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    mailing_street_address2: Option<String>,
    #[serde(
        rename = "mailingCity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    mailing_city: Option<String>,
    #[serde(
        rename = "mailingState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    mailing_state: Option<String>,
    #[serde(
        rename = "mailingPostCode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    mailing_post_code: Option<String>,
    #[serde(
        rename = "mailingCountry",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    mailing_country: Option<String>,
    #[serde(
        rename = "approvedId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    approved_id: Option<super::ids::ApprovedId>,
    #[serde(
        rename = "jointFirstName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    joint_first_name: Option<String>,
    #[serde(
        rename = "jointLastName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    joint_last_name: Option<String>,
}

impl UpdateContactInfo {
    /// Returns wire field `userId`.
    #[must_use]
    pub fn user_id(&self) -> &crate::UserId {
        &self.user_id
    }

    /// Returns wire field `firstName`.
    #[must_use]
    pub fn first_name(&self) -> &str {
        &self.first_name
    }

    /// Returns wire field `lastName`.
    #[must_use]
    pub fn last_name(&self) -> &str {
        &self.last_name
    }

    /// Returns wire field `streetAddress1`.
    #[must_use]
    pub fn street_address1(&self) -> &str {
        &self.street_address1
    }

    /// Returns wire field `streetAddress2`.
    #[must_use]
    pub fn street_address2(&self) -> Option<&str> {
        self.street_address2.as_deref()
    }

    /// Returns wire field `city`.
    #[must_use]
    pub fn city(&self) -> &str {
        &self.city
    }

    /// Returns wire field `state`.
    #[must_use]
    pub fn state(&self) -> Option<&str> {
        self.state.as_deref()
    }

    /// Returns wire field `postCode`.
    #[must_use]
    pub fn post_code(&self) -> Option<&str> {
        self.post_code.as_deref()
    }

    /// Returns wire field `country`.
    #[must_use]
    pub fn country(&self) -> &str {
        &self.country
    }

    /// Returns wire field `phone`.
    #[must_use]
    pub fn phone(&self) -> &str {
        &self.phone
    }

    /// Returns wire field `mailingIsDifferent`.
    #[must_use]
    pub fn mailing_is_different(&self) -> Option<&bool> {
        self.mailing_is_different.as_ref()
    }

    /// Returns wire field `mailingStreetAddress1`.
    #[must_use]
    pub fn mailing_street_address1(&self) -> Option<&str> {
        self.mailing_street_address1.as_deref()
    }

    /// Returns wire field `mailingStreetAddress2`.
    #[must_use]
    pub fn mailing_street_address2(&self) -> Option<&str> {
        self.mailing_street_address2.as_deref()
    }

    /// Returns wire field `mailingCity`.
    #[must_use]
    pub fn mailing_city(&self) -> Option<&str> {
        self.mailing_city.as_deref()
    }

    /// Returns wire field `mailingState`.
    #[must_use]
    pub fn mailing_state(&self) -> Option<&str> {
        self.mailing_state.as_deref()
    }

    /// Returns wire field `mailingPostCode`.
    #[must_use]
    pub fn mailing_post_code(&self) -> Option<&str> {
        self.mailing_post_code.as_deref()
    }

    /// Returns wire field `mailingCountry`.
    #[must_use]
    pub fn mailing_country(&self) -> Option<&str> {
        self.mailing_country.as_deref()
    }

    /// Returns wire field `approvedId`.
    #[must_use]
    pub fn approved_id(&self) -> Option<&super::ids::ApprovedId> {
        self.approved_id.as_ref()
    }

    /// Returns wire field `jointFirstName`.
    #[must_use]
    pub fn joint_first_name(&self) -> Option<&str> {
        self.joint_first_name.as_deref()
    }

    /// Returns wire field `jointLastName`.
    #[must_use]
    pub fn joint_last_name(&self) -> Option<&str> {
        self.joint_last_name.as_deref()
    }

    /// Starts a builder for [`UpdateContactInfo`].
    pub fn builder() -> UpdateContactInfoBuilder {
        UpdateContactInfoBuilder::default()
    }
}

/// Builder for [`UpdateContactInfo`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct UpdateContactInfoBuilder {
    user_id: Option<crate::UserId>,
    first_name: Option<String>,
    last_name: Option<String>,
    street_address1: Option<String>,
    street_address2: Option<String>,
    city: Option<String>,
    state: Option<String>,
    post_code: Option<String>,
    country: Option<String>,
    phone: Option<String>,
    mailing_is_different: Option<bool>,
    mailing_street_address1: Option<String>,
    mailing_street_address2: Option<String>,
    mailing_city: Option<String>,
    mailing_state: Option<String>,
    mailing_post_code: Option<String>,
    mailing_country: Option<String>,
    approved_id: Option<super::ids::ApprovedId>,
    joint_first_name: Option<String>,
    joint_last_name: Option<String>,
}

impl UpdateContactInfoBuilder {
    /// Sets wire field `userId`.
    pub fn user_id(mut self, value: crate::UserId) -> Self {
        self.user_id = Some(value);
        self
    }

    /// Sets wire field `firstName`.
    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    /// Sets wire field `lastName`.
    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
        self
    }

    /// Sets wire field `streetAddress1`.
    pub fn street_address1(mut self, value: impl Into<String>) -> Self {
        self.street_address1 = Some(value.into());
        self
    }

    /// Sets wire field `streetAddress2`.
    pub fn street_address2(mut self, value: impl Into<String>) -> Self {
        self.street_address2 = Some(value.into());
        self
    }

    /// Sets wire field `city`.
    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    /// Sets wire field `state`.
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    /// Sets wire field `postCode`.
    pub fn post_code(mut self, value: impl Into<String>) -> Self {
        self.post_code = Some(value.into());
        self
    }

    /// Sets wire field `country`.
    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    /// Sets wire field `phone`.
    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    /// Sets wire field `mailingIsDifferent`.
    pub fn mailing_is_different(mut self, value: bool) -> Self {
        self.mailing_is_different = Some(value);
        self
    }

    /// Sets wire field `mailingStreetAddress1`.
    pub fn mailing_street_address1(mut self, value: impl Into<String>) -> Self {
        self.mailing_street_address1 = Some(value.into());
        self
    }

    /// Sets wire field `mailingStreetAddress2`.
    pub fn mailing_street_address2(mut self, value: impl Into<String>) -> Self {
        self.mailing_street_address2 = Some(value.into());
        self
    }

    /// Sets wire field `mailingCity`.
    pub fn mailing_city(mut self, value: impl Into<String>) -> Self {
        self.mailing_city = Some(value.into());
        self
    }

    /// Sets wire field `mailingState`.
    pub fn mailing_state(mut self, value: impl Into<String>) -> Self {
        self.mailing_state = Some(value.into());
        self
    }

    /// Sets wire field `mailingPostCode`.
    pub fn mailing_post_code(mut self, value: impl Into<String>) -> Self {
        self.mailing_post_code = Some(value.into());
        self
    }

    /// Sets wire field `mailingCountry`.
    pub fn mailing_country(mut self, value: impl Into<String>) -> Self {
        self.mailing_country = Some(value.into());
        self
    }

    /// Sets wire field `approvedId`.
    pub fn approved_id(mut self, value: super::ids::ApprovedId) -> Self {
        self.approved_id = Some(value);
        self
    }

    /// Sets wire field `jointFirstName`.
    pub fn joint_first_name(mut self, value: impl Into<String>) -> Self {
        self.joint_first_name = Some(value.into());
        self
    }

    /// Sets wire field `jointLastName`.
    pub fn joint_last_name(mut self, value: impl Into<String>) -> Self {
        self.joint_last_name = Some(value.into());
        self
    }

    /// Validates required fields and builds [`UpdateContactInfo`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<UpdateContactInfo, crate::api::current::BuildError> {
        let user_id = self
            .user_id
            .ok_or(crate::api::current::BuildError::missing("userId"))?;
        let first_name = self
            .first_name
            .ok_or(crate::api::current::BuildError::missing("firstName"))?;
        if first_name.is_empty() || first_name.trim() != first_name {
            return Err(crate::api::current::BuildError::invalid(
                "firstName",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        let last_name = self
            .last_name
            .ok_or(crate::api::current::BuildError::missing("lastName"))?;
        if last_name.is_empty() || last_name.trim() != last_name {
            return Err(crate::api::current::BuildError::invalid(
                "lastName",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        let street_address1 = self
            .street_address1
            .ok_or(crate::api::current::BuildError::missing("streetAddress1"))?;
        if street_address1.is_empty() || street_address1.trim() != street_address1 {
            return Err(crate::api::current::BuildError::invalid(
                "streetAddress1",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        let city = self
            .city
            .ok_or(crate::api::current::BuildError::missing("city"))?;
        if city.is_empty() || city.trim() != city {
            return Err(crate::api::current::BuildError::invalid(
                "city",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        let country = self
            .country
            .ok_or(crate::api::current::BuildError::missing("country"))?;
        if country.is_empty() || country.trim() != country {
            return Err(crate::api::current::BuildError::invalid(
                "country",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        let phone = self
            .phone
            .ok_or(crate::api::current::BuildError::missing("phone"))?;
        if phone.is_empty() || phone.trim() != phone {
            return Err(crate::api::current::BuildError::invalid(
                "phone",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        Ok(UpdateContactInfo {
            user_id,
            first_name,
            last_name,
            street_address1,
            street_address2: self.street_address2,
            city,
            state: self.state,
            post_code: self.post_code,
            country,
            phone,
            mailing_is_different: self.mailing_is_different,
            mailing_street_address1: self.mailing_street_address1,
            mailing_street_address2: self.mailing_street_address2,
            mailing_city: self.mailing_city,
            mailing_state: self.mailing_state,
            mailing_post_code: self.mailing_post_code,
            mailing_country: self.mailing_country,
            approved_id: self.approved_id,
            joint_first_name: self.joint_first_name,
            joint_last_name: self.joint_last_name,
        })
    }
}

impl crate::api::current::support::CurrentRequest for UpdateContactInfo {
    fn validate_current(&self) -> Result<(), crate::Error> {
        if self.city.is_empty() || self.city.trim() != self.city {
            return Err(crate::Error::InvalidRequest {
                field: "city",
                reason: "must be non-empty and have no surrounding whitespace",
            });
        }
        if self.country.is_empty() || self.country.trim() != self.country {
            return Err(crate::Error::InvalidRequest {
                field: "country",
                reason: "must be non-empty and have no surrounding whitespace",
            });
        }
        if self.first_name.is_empty() || self.first_name.trim() != self.first_name {
            return Err(crate::Error::InvalidRequest {
                field: "firstName",
                reason: "must be non-empty and have no surrounding whitespace",
            });
        }
        if self.last_name.is_empty() || self.last_name.trim() != self.last_name {
            return Err(crate::Error::InvalidRequest {
                field: "lastName",
                reason: "must be non-empty and have no surrounding whitespace",
            });
        }
        if self.phone.is_empty() || self.phone.trim() != self.phone {
            return Err(crate::Error::InvalidRequest {
                field: "phone",
                reason: "must be non-empty and have no surrounding whitespace",
            });
        }
        if self.street_address1.is_empty() || self.street_address1.trim() != self.street_address1 {
            return Err(crate::Error::InvalidRequest {
                field: "streetAddress1",
                reason: "must be non-empty and have no surrounding whitespace",
            });
        }
        Ok(())
    }
}

/// Current wire model `UpdateContactInfoName`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct UpdateContactInfoName {
    #[serde(rename = "firstName")]
    first_name: String,
    #[serde(rename = "lastName")]
    last_name: String,
    #[serde(rename = "country")]
    country: String,
    #[serde(rename = "phone")]
    phone: String,
}

impl UpdateContactInfoName {
    /// Returns wire field `firstName`.
    #[must_use]
    pub fn first_name(&self) -> &str {
        &self.first_name
    }

    /// Returns wire field `lastName`.
    #[must_use]
    pub fn last_name(&self) -> &str {
        &self.last_name
    }

    /// Returns wire field `country`.
    #[must_use]
    pub fn country(&self) -> &str {
        &self.country
    }

    /// Returns wire field `phone`.
    #[must_use]
    pub fn phone(&self) -> &str {
        &self.phone
    }

    /// Starts a builder for [`UpdateContactInfoName`].
    pub fn builder() -> UpdateContactInfoNameBuilder {
        UpdateContactInfoNameBuilder::default()
    }
}

/// Builder for [`UpdateContactInfoName`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct UpdateContactInfoNameBuilder {
    first_name: Option<String>,
    last_name: Option<String>,
    country: Option<String>,
    phone: Option<String>,
}

impl UpdateContactInfoNameBuilder {
    /// Sets wire field `firstName`.
    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    /// Sets wire field `lastName`.
    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
        self
    }

    /// Sets wire field `country`.
    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    /// Sets wire field `phone`.
    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    /// Validates required fields and builds [`UpdateContactInfoName`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<UpdateContactInfoName, crate::api::current::BuildError> {
        let first_name = self
            .first_name
            .ok_or(crate::api::current::BuildError::missing("firstName"))?;
        if first_name.is_empty() || first_name.trim() != first_name {
            return Err(crate::api::current::BuildError::invalid(
                "firstName",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        let last_name = self
            .last_name
            .ok_or(crate::api::current::BuildError::missing("lastName"))?;
        if last_name.is_empty() || last_name.trim() != last_name {
            return Err(crate::api::current::BuildError::invalid(
                "lastName",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        let country = self
            .country
            .ok_or(crate::api::current::BuildError::missing("country"))?;
        if country.is_empty() || country.trim() != country {
            return Err(crate::api::current::BuildError::invalid(
                "country",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        let phone = self
            .phone
            .ok_or(crate::api::current::BuildError::missing("phone"))?;
        if phone.is_empty() || phone.trim() != phone {
            return Err(crate::api::current::BuildError::invalid(
                "phone",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        Ok(UpdateContactInfoName {
            first_name,
            last_name,
            country,
            phone,
        })
    }
}

impl crate::api::current::support::CurrentRequest for UpdateContactInfoName {
    fn validate_current(&self) -> Result<(), crate::Error> {
        if self.country.is_empty() || self.country.trim() != self.country {
            return Err(crate::Error::InvalidRequest {
                field: "country",
                reason: "must be non-empty and have no surrounding whitespace",
            });
        }
        if self.first_name.is_empty() || self.first_name.trim() != self.first_name {
            return Err(crate::Error::InvalidRequest {
                field: "firstName",
                reason: "must be non-empty and have no surrounding whitespace",
            });
        }
        if self.last_name.is_empty() || self.last_name.trim() != self.last_name {
            return Err(crate::Error::InvalidRequest {
                field: "lastName",
                reason: "must be non-empty and have no surrounding whitespace",
            });
        }
        if self.phone.is_empty() || self.phone.trim() != self.phone {
            return Err(crate::Error::InvalidRequest {
                field: "phone",
                reason: "must be non-empty and have no surrounding whitespace",
            });
        }
        Ok(())
    }
}

/// Current wire model `UpdateContactInfoResponse`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct UpdateContactInfoResponse {
    #[serde(rename = "errorText", default, skip_serializing_if = "Option::is_none")]
    error_text: Option<String>,
    #[serde(
        rename = "contactInfo",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    contact_info: Option<ContactInfo>,
}

impl UpdateContactInfoResponse {
    /// Returns wire field `errorText`.
    #[must_use]
    pub fn error_text(&self) -> Option<&str> {
        self.error_text.as_deref()
    }

    /// Returns wire field `contactInfo`.
    #[must_use]
    pub fn contact_info(&self) -> Option<&ContactInfo> {
        self.contact_info.as_ref()
    }

    /// Starts a builder for [`UpdateContactInfoResponse`].
    pub fn builder() -> UpdateContactInfoResponseBuilder {
        UpdateContactInfoResponseBuilder::default()
    }
}

/// Builder for [`UpdateContactInfoResponse`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct UpdateContactInfoResponseBuilder {
    error_text: Option<String>,
    contact_info: Option<ContactInfo>,
}

impl UpdateContactInfoResponseBuilder {
    /// Sets wire field `errorText`.
    pub fn error_text(mut self, value: impl Into<String>) -> Self {
        self.error_text = Some(value.into());
        self
    }

    /// Sets wire field `contactInfo`.
    pub fn contact_info(mut self, value: ContactInfo) -> Self {
        self.contact_info = Some(value);
        self
    }

    /// Validates required fields and builds [`UpdateContactInfoResponse`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<UpdateContactInfoResponse, crate::api::current::BuildError> {
        Ok(UpdateContactInfoResponse {
            error_text: self.error_text,
            contact_info: self.contact_info,
        })
    }
}

/// Current wire model `User`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct User {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    id: Option<crate::UserId>,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "timestamp")]
    timestamp: jiff::Timestamp,
    #[serde(rename = "email")]
    email: String,
    #[serde(rename = "status")]
    status: UserStatus,
    #[serde(rename = "professional")]
    professional: bool,
    #[serde(
        rename = "organizationId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    organization_id: Option<super::ids::OrganizationId>,
    #[serde(
        rename = "introducingPartnerId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    introducing_partner_id: Option<super::ids::IntroducingPartnerId>,
}

impl User {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> Option<&crate::UserId> {
        self.id.as_ref()
    }

    /// Returns wire field `name`.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns wire field `timestamp`.
    #[must_use]
    pub fn timestamp(&self) -> &jiff::Timestamp {
        &self.timestamp
    }

    /// Returns wire field `email`.
    #[must_use]
    pub fn email(&self) -> &str {
        &self.email
    }

    /// Returns wire field `status`.
    #[must_use]
    pub fn status(&self) -> &UserStatus {
        &self.status
    }

    /// Returns wire field `professional`.
    #[must_use]
    pub fn professional(&self) -> &bool {
        &self.professional
    }

    /// Returns wire field `organizationId`.
    #[must_use]
    pub fn organization_id(&self) -> Option<&super::ids::OrganizationId> {
        self.organization_id.as_ref()
    }

    /// Returns wire field `introducingPartnerId`.
    #[must_use]
    pub fn introducing_partner_id(&self) -> Option<&super::ids::IntroducingPartnerId> {
        self.introducing_partner_id.as_ref()
    }

    /// Starts a builder for [`User`].
    pub fn builder() -> UserBuilder {
        UserBuilder::default()
    }
}

/// Builder for [`User`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct UserBuilder {
    id: Option<crate::UserId>,
    name: Option<String>,
    timestamp: Option<jiff::Timestamp>,
    email: Option<String>,
    status: Option<UserStatus>,
    professional: Option<bool>,
    organization_id: Option<super::ids::OrganizationId>,
    introducing_partner_id: Option<super::ids::IntroducingPartnerId>,
}

impl UserBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: crate::UserId) -> Self {
        self.id = Some(value);
        self
    }

    /// Sets wire field `name`.
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Sets wire field `timestamp`.
    pub fn timestamp(mut self, value: jiff::Timestamp) -> Self {
        self.timestamp = Some(value);
        self
    }

    /// Sets wire field `email`.
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    /// Sets wire field `status`.
    pub fn status(mut self, value: UserStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Sets wire field `professional`.
    pub fn professional(mut self, value: bool) -> Self {
        self.professional = Some(value);
        self
    }

    /// Sets wire field `organizationId`.
    pub fn organization_id(mut self, value: super::ids::OrganizationId) -> Self {
        self.organization_id = Some(value);
        self
    }

    /// Sets wire field `introducingPartnerId`.
    pub fn introducing_partner_id(mut self, value: super::ids::IntroducingPartnerId) -> Self {
        self.introducing_partner_id = Some(value);
        self
    }

    /// Validates required fields and builds [`User`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<User, crate::api::current::BuildError> {
        let name = self
            .name
            .ok_or(crate::api::current::BuildError::missing("name"))?;
        let timestamp = self
            .timestamp
            .ok_or(crate::api::current::BuildError::missing("timestamp"))?;
        let email = self
            .email
            .ok_or(crate::api::current::BuildError::missing("email"))?;
        let status = self
            .status
            .ok_or(crate::api::current::BuildError::missing("status"))?;
        let professional = self
            .professional
            .ok_or(crate::api::current::BuildError::missing("professional"))?;
        Ok(User {
            id: self.id,
            name,
            timestamp,
            email,
            status,
            professional,
            organization_id: self.organization_id,
            introducing_partner_id: self.introducing_partner_id,
        })
    }
}

/// Current wire model `UserAccountAutoLiq`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct UserAccountAutoLiq {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    id: Option<super::ids::UserAccountAutoLiqId>,
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
    trailing_max_drawdown_mode: Option<UserAccountAutoLiqTrailingMaxDrawdownMode>,
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

impl UserAccountAutoLiq {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> Option<&super::ids::UserAccountAutoLiqId> {
        self.id.as_ref()
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
    pub fn trailing_max_drawdown_mode(&self) -> Option<&UserAccountAutoLiqTrailingMaxDrawdownMode> {
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

    /// Starts a builder for [`UserAccountAutoLiq`].
    pub fn builder() -> UserAccountAutoLiqBuilder {
        UserAccountAutoLiqBuilder::default()
    }
}

/// Builder for [`UserAccountAutoLiq`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct UserAccountAutoLiqBuilder {
    id: Option<super::ids::UserAccountAutoLiqId>,
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
    trailing_max_drawdown_mode: Option<UserAccountAutoLiqTrailingMaxDrawdownMode>,
    daily_profit_auto_liq: Option<crate::Decimal>,
    weekly_profit_auto_liq: Option<crate::Decimal>,
    do_not_unlock: Option<bool>,
}

impl UserAccountAutoLiqBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::UserAccountAutoLiqId) -> Self {
        self.id = Some(value);
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
        value: UserAccountAutoLiqTrailingMaxDrawdownMode,
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

    /// Validates required fields and builds [`UserAccountAutoLiq`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<UserAccountAutoLiq, crate::api::current::BuildError> {
        Ok(UserAccountAutoLiq {
            id: self.id,
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

impl crate::api::current::support::CurrentRequest for UserAccountAutoLiq {
    fn validate_current(&self) -> Result<(), crate::Error> {
        Ok(())
    }
}

/// Current provider values for `UserAccountAutoLiqTrailingMaxDrawdownMode`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum UserAccountAutoLiqTrailingMaxDrawdownMode {
    /// Provider value `EOD`.
    Eod,
    /// Provider value `RealTime`.
    RealTime,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl UserAccountAutoLiqTrailingMaxDrawdownMode {
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

impl serde::Serialize for UserAccountAutoLiqTrailingMaxDrawdownMode {
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

impl<'de> serde::Deserialize<'de> for UserAccountAutoLiqTrailingMaxDrawdownMode {
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

/// Current wire model `UserPlugin`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct UserPlugin {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    id: Option<super::ids::UserPluginId>,
    #[serde(rename = "userId")]
    user_id: crate::UserId,
    #[serde(rename = "timestamp")]
    timestamp: jiff::Timestamp,
    #[serde(rename = "planPrice")]
    #[serde(with = "crate::decimal")]
    plan_price: crate::Decimal,
    #[serde(
        rename = "cashBalanceLogId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    cash_balance_log_id: Option<super::ids::CashBalanceLogId>,
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    account_id: Option<crate::AccountId>,
    #[serde(rename = "pluginName")]
    plugin_name: String,
    #[serde(rename = "approval")]
    approval: bool,
    #[serde(
        rename = "entitlementId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    entitlement_id: Option<super::ids::EntitlementId>,
    #[serde(rename = "startDate")]
    start_date: TradeDate,
    #[serde(
        rename = "expirationDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    expiration_date: Option<TradeDate>,
    #[serde(rename = "paidAmount")]
    #[serde(with = "crate::decimal")]
    paid_amount: crate::Decimal,
    #[serde(
        rename = "autorenewal",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    autorenewal: Option<bool>,
    #[serde(
        rename = "planCategories",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    plan_categories: Option<String>,
    #[serde(rename = "rebate", default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "crate::decimal::option")]
    rebate: Option<crate::Decimal>,
}

impl UserPlugin {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> Option<&super::ids::UserPluginId> {
        self.id.as_ref()
    }

    /// Returns wire field `userId`.
    #[must_use]
    pub fn user_id(&self) -> &crate::UserId {
        &self.user_id
    }

    /// Returns wire field `timestamp`.
    #[must_use]
    pub fn timestamp(&self) -> &jiff::Timestamp {
        &self.timestamp
    }

    /// Returns wire field `planPrice`.
    #[must_use]
    pub fn plan_price(&self) -> &crate::Decimal {
        &self.plan_price
    }

    /// Returns wire field `cashBalanceLogId`.
    #[must_use]
    pub fn cash_balance_log_id(&self) -> Option<&super::ids::CashBalanceLogId> {
        self.cash_balance_log_id.as_ref()
    }

    /// Returns wire field `accountId`.
    #[must_use]
    pub fn account_id(&self) -> Option<&crate::AccountId> {
        self.account_id.as_ref()
    }

    /// Returns wire field `pluginName`.
    #[must_use]
    pub fn plugin_name(&self) -> &str {
        &self.plugin_name
    }

    /// Returns wire field `approval`.
    #[must_use]
    pub fn approval(&self) -> &bool {
        &self.approval
    }

    /// Returns wire field `entitlementId`.
    #[must_use]
    pub fn entitlement_id(&self) -> Option<&super::ids::EntitlementId> {
        self.entitlement_id.as_ref()
    }

    /// Returns wire field `startDate`.
    #[must_use]
    pub fn start_date(&self) -> &TradeDate {
        &self.start_date
    }

    /// Returns wire field `expirationDate`.
    #[must_use]
    pub fn expiration_date(&self) -> Option<&TradeDate> {
        self.expiration_date.as_ref()
    }

    /// Returns wire field `paidAmount`.
    #[must_use]
    pub fn paid_amount(&self) -> &crate::Decimal {
        &self.paid_amount
    }

    /// Returns wire field `autorenewal`.
    #[must_use]
    pub fn autorenewal(&self) -> Option<&bool> {
        self.autorenewal.as_ref()
    }

    /// Returns wire field `planCategories`.
    #[must_use]
    pub fn plan_categories(&self) -> Option<&str> {
        self.plan_categories.as_deref()
    }

    /// Returns wire field `rebate`.
    #[must_use]
    pub fn rebate(&self) -> Option<&crate::Decimal> {
        self.rebate.as_ref()
    }

    /// Starts a builder for [`UserPlugin`].
    pub fn builder() -> UserPluginBuilder {
        UserPluginBuilder::default()
    }
}

/// Builder for [`UserPlugin`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct UserPluginBuilder {
    id: Option<super::ids::UserPluginId>,
    user_id: Option<crate::UserId>,
    timestamp: Option<jiff::Timestamp>,
    plan_price: Option<crate::Decimal>,
    cash_balance_log_id: Option<super::ids::CashBalanceLogId>,
    account_id: Option<crate::AccountId>,
    plugin_name: Option<String>,
    approval: Option<bool>,
    entitlement_id: Option<super::ids::EntitlementId>,
    start_date: Option<TradeDate>,
    expiration_date: Option<TradeDate>,
    paid_amount: Option<crate::Decimal>,
    autorenewal: Option<bool>,
    plan_categories: Option<String>,
    rebate: Option<crate::Decimal>,
}

impl UserPluginBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::UserPluginId) -> Self {
        self.id = Some(value);
        self
    }

    /// Sets wire field `userId`.
    pub fn user_id(mut self, value: crate::UserId) -> Self {
        self.user_id = Some(value);
        self
    }

    /// Sets wire field `timestamp`.
    pub fn timestamp(mut self, value: jiff::Timestamp) -> Self {
        self.timestamp = Some(value);
        self
    }

    /// Sets wire field `planPrice`.
    pub fn plan_price(mut self, value: crate::Decimal) -> Self {
        self.plan_price = Some(value);
        self
    }

    /// Sets wire field `cashBalanceLogId`.
    pub fn cash_balance_log_id(mut self, value: super::ids::CashBalanceLogId) -> Self {
        self.cash_balance_log_id = Some(value);
        self
    }

    /// Sets wire field `accountId`.
    pub fn account_id(mut self, value: crate::AccountId) -> Self {
        self.account_id = Some(value);
        self
    }

    /// Sets wire field `pluginName`.
    pub fn plugin_name(mut self, value: impl Into<String>) -> Self {
        self.plugin_name = Some(value.into());
        self
    }

    /// Sets wire field `approval`.
    pub fn approval(mut self, value: bool) -> Self {
        self.approval = Some(value);
        self
    }

    /// Sets wire field `entitlementId`.
    pub fn entitlement_id(mut self, value: super::ids::EntitlementId) -> Self {
        self.entitlement_id = Some(value);
        self
    }

    /// Sets wire field `startDate`.
    pub fn start_date(mut self, value: TradeDate) -> Self {
        self.start_date = Some(value);
        self
    }

    /// Sets wire field `expirationDate`.
    pub fn expiration_date(mut self, value: TradeDate) -> Self {
        self.expiration_date = Some(value);
        self
    }

    /// Sets wire field `paidAmount`.
    pub fn paid_amount(mut self, value: crate::Decimal) -> Self {
        self.paid_amount = Some(value);
        self
    }

    /// Sets wire field `autorenewal`.
    pub fn autorenewal(mut self, value: bool) -> Self {
        self.autorenewal = Some(value);
        self
    }

    /// Sets wire field `planCategories`.
    pub fn plan_categories(mut self, value: impl Into<String>) -> Self {
        self.plan_categories = Some(value.into());
        self
    }

    /// Sets wire field `rebate`.
    pub fn rebate(mut self, value: crate::Decimal) -> Self {
        self.rebate = Some(value);
        self
    }

    /// Validates required fields and builds [`UserPlugin`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<UserPlugin, crate::api::current::BuildError> {
        let user_id = self
            .user_id
            .ok_or(crate::api::current::BuildError::missing("userId"))?;
        let timestamp = self
            .timestamp
            .ok_or(crate::api::current::BuildError::missing("timestamp"))?;
        let plan_price = self
            .plan_price
            .ok_or(crate::api::current::BuildError::missing("planPrice"))?;
        let plugin_name = self
            .plugin_name
            .ok_or(crate::api::current::BuildError::missing("pluginName"))?;
        if plugin_name.is_empty() || plugin_name.trim() != plugin_name {
            return Err(crate::api::current::BuildError::invalid(
                "pluginName",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        let approval = self
            .approval
            .ok_or(crate::api::current::BuildError::missing("approval"))?;
        let start_date = self
            .start_date
            .ok_or(crate::api::current::BuildError::missing("startDate"))?;
        let paid_amount = self
            .paid_amount
            .ok_or(crate::api::current::BuildError::missing("paidAmount"))?;
        Ok(UserPlugin {
            id: self.id,
            user_id,
            timestamp,
            plan_price,
            cash_balance_log_id: self.cash_balance_log_id,
            account_id: self.account_id,
            plugin_name,
            approval,
            entitlement_id: self.entitlement_id,
            start_date,
            expiration_date: self.expiration_date,
            paid_amount,
            autorenewal: self.autorenewal,
            plan_categories: self.plan_categories,
            rebate: self.rebate,
        })
    }
}

impl crate::api::current::support::CurrentRequest for UserPlugin {
    fn validate_current(&self) -> Result<(), crate::Error> {
        if self.plugin_name.is_empty() || self.plugin_name.trim() != self.plugin_name {
            return Err(crate::Error::InvalidRequest {
                field: "pluginName",
                reason: "must be non-empty and have no surrounding whitespace",
            });
        }
        Ok(())
    }
}

/// Current wire model `UserPromoCode`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct UserPromoCode {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    id: Option<super::ids::UserPromoCodeId>,
    #[serde(rename = "userId")]
    user_id: crate::UserId,
    #[serde(rename = "promoCodeId")]
    promo_code_id: super::ids::PromoCodeId,
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    account_id: Option<crate::AccountId>,
    #[serde(rename = "source")]
    source: UserPromoCodeSource,
    #[serde(rename = "comments", default, skip_serializing_if = "Option::is_none")]
    comments: Option<String>,
}

impl UserPromoCode {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> Option<&super::ids::UserPromoCodeId> {
        self.id.as_ref()
    }

    /// Returns wire field `userId`.
    #[must_use]
    pub fn user_id(&self) -> &crate::UserId {
        &self.user_id
    }

    /// Returns wire field `promoCodeId`.
    #[must_use]
    pub fn promo_code_id(&self) -> &super::ids::PromoCodeId {
        &self.promo_code_id
    }

    /// Returns wire field `accountId`.
    #[must_use]
    pub fn account_id(&self) -> Option<&crate::AccountId> {
        self.account_id.as_ref()
    }

    /// Returns wire field `source`.
    #[must_use]
    pub fn source(&self) -> &UserPromoCodeSource {
        &self.source
    }

    /// Returns wire field `comments`.
    #[must_use]
    pub fn comments(&self) -> Option<&str> {
        self.comments.as_deref()
    }

    /// Starts a builder for [`UserPromoCode`].
    pub fn builder() -> UserPromoCodeBuilder {
        UserPromoCodeBuilder::default()
    }
}

/// Builder for [`UserPromoCode`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct UserPromoCodeBuilder {
    id: Option<super::ids::UserPromoCodeId>,
    user_id: Option<crate::UserId>,
    promo_code_id: Option<super::ids::PromoCodeId>,
    account_id: Option<crate::AccountId>,
    source: Option<UserPromoCodeSource>,
    comments: Option<String>,
}

impl UserPromoCodeBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::UserPromoCodeId) -> Self {
        self.id = Some(value);
        self
    }

    /// Sets wire field `userId`.
    pub fn user_id(mut self, value: crate::UserId) -> Self {
        self.user_id = Some(value);
        self
    }

    /// Sets wire field `promoCodeId`.
    pub fn promo_code_id(mut self, value: super::ids::PromoCodeId) -> Self {
        self.promo_code_id = Some(value);
        self
    }

    /// Sets wire field `accountId`.
    pub fn account_id(mut self, value: crate::AccountId) -> Self {
        self.account_id = Some(value);
        self
    }

    /// Sets wire field `source`.
    pub fn source(mut self, value: UserPromoCodeSource) -> Self {
        self.source = Some(value);
        self
    }

    /// Sets wire field `comments`.
    pub fn comments(mut self, value: impl Into<String>) -> Self {
        self.comments = Some(value.into());
        self
    }

    /// Validates required fields and builds [`UserPromoCode`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<UserPromoCode, crate::api::current::BuildError> {
        let user_id = self
            .user_id
            .ok_or(crate::api::current::BuildError::missing("userId"))?;
        let promo_code_id = self
            .promo_code_id
            .ok_or(crate::api::current::BuildError::missing("promoCodeId"))?;
        let source = self
            .source
            .ok_or(crate::api::current::BuildError::missing("source"))?;
        Ok(UserPromoCode {
            id: self.id,
            user_id,
            promo_code_id,
            account_id: self.account_id,
            source,
            comments: self.comments,
        })
    }
}

/// Current provider values for `UserPromoCodeSource`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum UserPromoCodeSource {
    /// Provider value `Admin`.
    Admin,
    /// Provider value `Input`.
    Input,
    /// Provider value `URL`.
    Url,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl UserPromoCodeSource {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Admin => "Admin",
            Self::Input => "Input",
            Self::Url => "URL",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for UserPromoCodeSource {
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

impl<'de> serde::Deserialize<'de> for UserPromoCodeSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "Admin" => Self::Admin,
            "Input" => Self::Input,
            "URL" => Self::Url,
            _ => Self::Unknown(value),
        })
    }
}

/// Current wire model `UserProperty`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct UserProperty {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    id: Option<super::ids::UserPropertyId>,
    #[serde(rename = "userId")]
    user_id: crate::UserId,
    #[serde(rename = "propertyId")]
    property_id: super::ids::PropertyId,
    #[serde(rename = "value", default, skip_serializing_if = "Option::is_none")]
    value: Option<String>,
}

impl UserProperty {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> Option<&super::ids::UserPropertyId> {
        self.id.as_ref()
    }

    /// Returns wire field `userId`.
    #[must_use]
    pub fn user_id(&self) -> &crate::UserId {
        &self.user_id
    }

    /// Returns wire field `propertyId`.
    #[must_use]
    pub fn property_id(&self) -> &super::ids::PropertyId {
        &self.property_id
    }

    /// Returns wire field `value`.
    #[must_use]
    pub fn value(&self) -> Option<&str> {
        self.value.as_deref()
    }

    /// Starts a builder for [`UserProperty`].
    pub fn builder() -> UserPropertyBuilder {
        UserPropertyBuilder::default()
    }
}

/// Builder for [`UserProperty`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct UserPropertyBuilder {
    id: Option<super::ids::UserPropertyId>,
    user_id: Option<crate::UserId>,
    property_id: Option<super::ids::PropertyId>,
    value: Option<String>,
}

impl UserPropertyBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::UserPropertyId) -> Self {
        self.id = Some(value);
        self
    }

    /// Sets wire field `userId`.
    pub fn user_id(mut self, value: crate::UserId) -> Self {
        self.user_id = Some(value);
        self
    }

    /// Sets wire field `propertyId`.
    pub fn property_id(mut self, value: super::ids::PropertyId) -> Self {
        self.property_id = Some(value);
        self
    }

    /// Sets wire field `value`.
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Validates required fields and builds [`UserProperty`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<UserProperty, crate::api::current::BuildError> {
        let user_id = self
            .user_id
            .ok_or(crate::api::current::BuildError::missing("userId"))?;
        let property_id = self
            .property_id
            .ok_or(crate::api::current::BuildError::missing("propertyId"))?;
        Ok(UserProperty {
            id: self.id,
            user_id,
            property_id,
            value: self.value,
        })
    }
}

/// Current wire model `UserReadStatus`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct UserReadStatus {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    id: Option<super::ids::UserReadStatusId>,
    #[serde(
        rename = "newsStoryId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    news_story_id: Option<super::ids::NewsStoryId>,
}

impl UserReadStatus {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> Option<&super::ids::UserReadStatusId> {
        self.id.as_ref()
    }

    /// Returns wire field `newsStoryId`.
    #[must_use]
    pub fn news_story_id(&self) -> Option<&super::ids::NewsStoryId> {
        self.news_story_id.as_ref()
    }

    /// Starts a builder for [`UserReadStatus`].
    pub fn builder() -> UserReadStatusBuilder {
        UserReadStatusBuilder::default()
    }
}

/// Builder for [`UserReadStatus`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct UserReadStatusBuilder {
    id: Option<super::ids::UserReadStatusId>,
    news_story_id: Option<super::ids::NewsStoryId>,
}

impl UserReadStatusBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::UserReadStatusId) -> Self {
        self.id = Some(value);
        self
    }

    /// Sets wire field `newsStoryId`.
    pub fn news_story_id(mut self, value: super::ids::NewsStoryId) -> Self {
        self.news_story_id = Some(value);
        self
    }

    /// Validates required fields and builds [`UserReadStatus`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<UserReadStatus, crate::api::current::BuildError> {
        Ok(UserReadStatus {
            id: self.id,
            news_story_id: self.news_story_id,
        })
    }
}

/// Current wire model `UserSession`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct UserSession {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    id: Option<super::ids::UserSessionId>,
    #[serde(rename = "userId")]
    user_id: crate::UserId,
    #[serde(rename = "startTime")]
    start_time: jiff::Timestamp,
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    end_time: Option<jiff::Timestamp>,
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    ip_address: Option<String>,
    #[serde(rename = "appId", default, skip_serializing_if = "Option::is_none")]
    app_id: Option<String>,
    #[serde(
        rename = "appVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    app_version: Option<String>,
    #[serde(rename = "clientAppId")]
    client_app_id: super::ids::ClientAppId,
}

impl UserSession {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> Option<&super::ids::UserSessionId> {
        self.id.as_ref()
    }

    /// Returns wire field `userId`.
    #[must_use]
    pub fn user_id(&self) -> &crate::UserId {
        &self.user_id
    }

    /// Returns wire field `startTime`.
    #[must_use]
    pub fn start_time(&self) -> &jiff::Timestamp {
        &self.start_time
    }

    /// Returns wire field `endTime`.
    #[must_use]
    pub fn end_time(&self) -> Option<&jiff::Timestamp> {
        self.end_time.as_ref()
    }

    /// Returns wire field `ipAddress`.
    #[must_use]
    pub fn ip_address(&self) -> Option<&str> {
        self.ip_address.as_deref()
    }

    /// Returns wire field `appId`.
    #[must_use]
    pub fn app_id(&self) -> Option<&str> {
        self.app_id.as_deref()
    }

    /// Returns wire field `appVersion`.
    #[must_use]
    pub fn app_version(&self) -> Option<&str> {
        self.app_version.as_deref()
    }

    /// Returns wire field `clientAppId`.
    #[must_use]
    pub fn client_app_id(&self) -> &super::ids::ClientAppId {
        &self.client_app_id
    }

    /// Starts a builder for [`UserSession`].
    pub fn builder() -> UserSessionBuilder {
        UserSessionBuilder::default()
    }
}

/// Builder for [`UserSession`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct UserSessionBuilder {
    id: Option<super::ids::UserSessionId>,
    user_id: Option<crate::UserId>,
    start_time: Option<jiff::Timestamp>,
    end_time: Option<jiff::Timestamp>,
    ip_address: Option<String>,
    app_id: Option<String>,
    app_version: Option<String>,
    client_app_id: Option<super::ids::ClientAppId>,
}

impl UserSessionBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::UserSessionId) -> Self {
        self.id = Some(value);
        self
    }

    /// Sets wire field `userId`.
    pub fn user_id(mut self, value: crate::UserId) -> Self {
        self.user_id = Some(value);
        self
    }

    /// Sets wire field `startTime`.
    pub fn start_time(mut self, value: jiff::Timestamp) -> Self {
        self.start_time = Some(value);
        self
    }

    /// Sets wire field `endTime`.
    pub fn end_time(mut self, value: jiff::Timestamp) -> Self {
        self.end_time = Some(value);
        self
    }

    /// Sets wire field `ipAddress`.
    pub fn ip_address(mut self, value: impl Into<String>) -> Self {
        self.ip_address = Some(value.into());
        self
    }

    /// Sets wire field `appId`.
    pub fn app_id(mut self, value: impl Into<String>) -> Self {
        self.app_id = Some(value.into());
        self
    }

    /// Sets wire field `appVersion`.
    pub fn app_version(mut self, value: impl Into<String>) -> Self {
        self.app_version = Some(value.into());
        self
    }

    /// Sets wire field `clientAppId`.
    pub fn client_app_id(mut self, value: super::ids::ClientAppId) -> Self {
        self.client_app_id = Some(value);
        self
    }

    /// Validates required fields and builds [`UserSession`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<UserSession, crate::api::current::BuildError> {
        let user_id = self
            .user_id
            .ok_or(crate::api::current::BuildError::missing("userId"))?;
        let start_time = self
            .start_time
            .ok_or(crate::api::current::BuildError::missing("startTime"))?;
        let client_app_id = self
            .client_app_id
            .ok_or(crate::api::current::BuildError::missing("clientAppId"))?;
        Ok(UserSession {
            id: self.id,
            user_id,
            start_time,
            end_time: self.end_time,
            ip_address: self.ip_address,
            app_id: self.app_id,
            app_version: self.app_version,
            client_app_id,
        })
    }
}

/// Current wire model `UserSessionStats`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct UserSessionStats {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    id: Option<super::ids::UserSessionStatsId>,
    #[serde(rename = "lastSessionTime")]
    last_session_time: jiff::Timestamp,
    #[serde(rename = "failedPasswords")]
    failed_passwords: i64,
}

impl UserSessionStats {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> Option<&super::ids::UserSessionStatsId> {
        self.id.as_ref()
    }

    /// Returns wire field `lastSessionTime`.
    #[must_use]
    pub fn last_session_time(&self) -> &jiff::Timestamp {
        &self.last_session_time
    }

    /// Returns wire field `failedPasswords`.
    #[must_use]
    pub fn failed_passwords(&self) -> &i64 {
        &self.failed_passwords
    }

    /// Starts a builder for [`UserSessionStats`].
    pub fn builder() -> UserSessionStatsBuilder {
        UserSessionStatsBuilder::default()
    }
}

/// Builder for [`UserSessionStats`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct UserSessionStatsBuilder {
    id: Option<super::ids::UserSessionStatsId>,
    last_session_time: Option<jiff::Timestamp>,
    failed_passwords: Option<i64>,
}

impl UserSessionStatsBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::UserSessionStatsId) -> Self {
        self.id = Some(value);
        self
    }

    /// Sets wire field `lastSessionTime`.
    pub fn last_session_time(mut self, value: jiff::Timestamp) -> Self {
        self.last_session_time = Some(value);
        self
    }

    /// Sets wire field `failedPasswords`.
    pub fn failed_passwords(mut self, value: i64) -> Self {
        self.failed_passwords = Some(value);
        self
    }

    /// Validates required fields and builds [`UserSessionStats`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<UserSessionStats, crate::api::current::BuildError> {
        let last_session_time = self
            .last_session_time
            .ok_or(crate::api::current::BuildError::missing("lastSessionTime"))?;
        let failed_passwords = self
            .failed_passwords
            .ok_or(crate::api::current::BuildError::missing("failedPasswords"))?;
        Ok(UserSessionStats {
            id: self.id,
            last_session_time,
            failed_passwords,
        })
    }
}

/// Current provider values for `UserStatus`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum UserStatus {
    /// Provider value `Active`.
    Active,
    /// Provider value `Closed`.
    Closed,
    /// Provider value `Initiated`.
    Initiated,
    /// Provider value `TemporaryLocked`.
    TemporaryLocked,
    /// Provider value `UnconfirmedEmail`.
    UnconfirmedEmail,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl UserStatus {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Active => "Active",
            Self::Closed => "Closed",
            Self::Initiated => "Initiated",
            Self::TemporaryLocked => "TemporaryLocked",
            Self::UnconfirmedEmail => "UnconfirmedEmail",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for UserStatus {
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

impl<'de> serde::Deserialize<'de> for UserStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "Active" => Self::Active,
            "Closed" => Self::Closed,
            "Initiated" => Self::Initiated,
            "TemporaryLocked" => Self::TemporaryLocked,
            "UnconfirmedEmail" => Self::UnconfirmedEmail,
            _ => Self::Unknown(value),
        })
    }
}

/// Current wire model `UserStatusMessage`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct UserStatusMessage {
    #[serde(rename = "errorText", default, skip_serializing_if = "Option::is_none")]
    error_text: Option<String>,
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    status: Option<UserStatusMessageStatus>,
}

impl UserStatusMessage {
    /// Returns wire field `errorText`.
    #[must_use]
    pub fn error_text(&self) -> Option<&str> {
        self.error_text.as_deref()
    }

    /// Returns wire field `status`.
    #[must_use]
    pub fn status(&self) -> Option<&UserStatusMessageStatus> {
        self.status.as_ref()
    }

    /// Starts a builder for [`UserStatusMessage`].
    pub fn builder() -> UserStatusMessageBuilder {
        UserStatusMessageBuilder::default()
    }
}

/// Builder for [`UserStatusMessage`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct UserStatusMessageBuilder {
    error_text: Option<String>,
    status: Option<UserStatusMessageStatus>,
}

impl UserStatusMessageBuilder {
    /// Sets wire field `errorText`.
    pub fn error_text(mut self, value: impl Into<String>) -> Self {
        self.error_text = Some(value.into());
        self
    }

    /// Sets wire field `status`.
    pub fn status(mut self, value: UserStatusMessageStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Validates required fields and builds [`UserStatusMessage`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<UserStatusMessage, crate::api::current::BuildError> {
        Ok(UserStatusMessage {
            error_text: self.error_text,
            status: self.status,
        })
    }
}

/// Current provider values for `UserStatusMessageStatus`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum UserStatusMessageStatus {
    /// Provider value `Active`.
    Active,
    /// Provider value `Closed`.
    Closed,
    /// Provider value `Initiated`.
    Initiated,
    /// Provider value `TemporaryLocked`.
    TemporaryLocked,
    /// Provider value `UnconfirmedEmail`.
    UnconfirmedEmail,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl UserStatusMessageStatus {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Active => "Active",
            Self::Closed => "Closed",
            Self::Initiated => "Initiated",
            Self::TemporaryLocked => "TemporaryLocked",
            Self::UnconfirmedEmail => "UnconfirmedEmail",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for UserStatusMessageStatus {
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

impl<'de> serde::Deserialize<'de> for UserStatusMessageStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "Active" => Self::Active,
            "Closed" => Self::Closed,
            "Initiated" => Self::Initiated,
            "TemporaryLocked" => Self::TemporaryLocked,
            "UnconfirmedEmail" => Self::UnconfirmedEmail,
            _ => Self::Unknown(value),
        })
    }
}

/// Current wire model `WorkspaceTemplate`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct WorkspaceTemplate {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    id: Option<super::ids::WorkspaceTemplateId>,
    #[serde(rename = "name")]
    name: String,
}

impl WorkspaceTemplate {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> Option<&super::ids::WorkspaceTemplateId> {
        self.id.as_ref()
    }

    /// Returns wire field `name`.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Starts a builder for [`WorkspaceTemplate`].
    pub fn builder() -> WorkspaceTemplateBuilder {
        WorkspaceTemplateBuilder::default()
    }
}

/// Builder for [`WorkspaceTemplate`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct WorkspaceTemplateBuilder {
    id: Option<super::ids::WorkspaceTemplateId>,
    name: Option<String>,
}

impl WorkspaceTemplateBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::WorkspaceTemplateId) -> Self {
        self.id = Some(value);
        self
    }

    /// Sets wire field `name`.
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Validates required fields and builds [`WorkspaceTemplate`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<WorkspaceTemplate, crate::api::current::BuildError> {
        let name = self
            .name
            .ok_or(crate::api::current::BuildError::missing("name"))?;
        if name.is_empty() || name.trim() != name {
            return Err(crate::api::current::BuildError::invalid(
                "name",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        Ok(WorkspaceTemplate { id: self.id, name })
    }
}

impl crate::api::current::support::CurrentRequest for WorkspaceTemplate {
    fn validate_current(&self) -> Result<(), crate::Error> {
        if self.name.is_empty() || self.name.trim() != self.name {
            return Err(crate::Error::InvalidRequest {
                field: "name",
                reason: "must be non-empty and have no surrounding whitespace",
            });
        }
        Ok(())
    }
}

impl crate::Client {
    /// Calls the current `POST /customerApplication/checkduplicate` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn customer_application_check_duplicate(
        &self,
        request: &CheckDuplicate,
    ) -> Result<SimpleResponse, crate::Error> {
        crate::api::current::support::CurrentRequest::validate_current(request)?;
        self.post_query("/customerApplication/checkduplicate", request)
            .await
    }
}

/// Typed query parameters for `/contactInfo/deps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ContactInfoDependentsQuery {
    #[serde(rename = "masterid")]
    masterid: super::ids::ProviderEntityId,
}

impl ContactInfoDependentsQuery {
    /// Returns wire field `masterid`.
    #[must_use]
    pub fn masterid(&self) -> &super::ids::ProviderEntityId {
        &self.masterid
    }

    /// Starts a builder for [`ContactInfoDependentsQuery`].
    pub fn builder() -> ContactInfoDependentsQueryBuilder {
        ContactInfoDependentsQueryBuilder::default()
    }
}

/// Builder for [`ContactInfoDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ContactInfoDependentsQueryBuilder {
    masterid: Option<super::ids::ProviderEntityId>,
}

impl ContactInfoDependentsQueryBuilder {
    /// Sets wire field `masterid`.
    pub fn masterid(mut self, value: super::ids::ProviderEntityId) -> Self {
        self.masterid = Some(value);
        self
    }

    /// Validates required fields and builds [`ContactInfoDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ContactInfoDependentsQuery, crate::api::current::BuildError> {
        let masterid = self
            .masterid
            .ok_or(crate::api::current::BuildError::missing("masterid"))?;
        Ok(ContactInfoDependentsQuery { masterid })
    }
}

impl crate::api::current::support::CurrentQuery for ContactInfoDependentsQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "masterid", &self.masterid)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /contactInfo/deps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn contact_info_dependents(
        &self,
        query: &ContactInfoDependentsQuery,
    ) -> Result<Vec<ContactInfo>, crate::Error> {
        self.get_current("/contactInfo/deps", query).await
    }
}

/// Typed query parameters for `/contactInfo/item`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ContactInfoItemQuery {
    #[serde(rename = "id")]
    id: super::ids::ContactInfoId,
}

impl ContactInfoItemQuery {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> &super::ids::ContactInfoId {
        &self.id
    }

    /// Starts a builder for [`ContactInfoItemQuery`].
    pub fn builder() -> ContactInfoItemQueryBuilder {
        ContactInfoItemQueryBuilder::default()
    }
}

/// Builder for [`ContactInfoItemQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ContactInfoItemQueryBuilder {
    id: Option<super::ids::ContactInfoId>,
}

impl ContactInfoItemQueryBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::ContactInfoId) -> Self {
        self.id = Some(value);
        self
    }

    /// Validates required fields and builds [`ContactInfoItemQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ContactInfoItemQuery, crate::api::current::BuildError> {
        let id = self
            .id
            .ok_or(crate::api::current::BuildError::missing("id"))?;
        Ok(ContactInfoItemQuery { id })
    }
}

impl crate::api::current::support::CurrentQuery for ContactInfoItemQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "id", &self.id)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /contactInfo/item` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn contact_info_item(
        &self,
        query: &ContactInfoItemQuery,
    ) -> Result<ContactInfo, crate::Error> {
        self.get_current("/contactInfo/item", query).await
    }
}

/// Typed query parameters for `/contactInfo/items`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ContactInfoItemsQuery {
    #[serde(rename = "ids")]
    ids: Vec<super::ids::ContactInfoId>,
}

impl ContactInfoItemsQuery {
    /// Returns wire field `ids`.
    #[must_use]
    pub fn ids(&self) -> &[super::ids::ContactInfoId] {
        &self.ids
    }

    /// Starts a builder for [`ContactInfoItemsQuery`].
    pub fn builder() -> ContactInfoItemsQueryBuilder {
        ContactInfoItemsQueryBuilder::default()
    }
}

/// Builder for [`ContactInfoItemsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ContactInfoItemsQueryBuilder {
    ids: Option<Vec<super::ids::ContactInfoId>>,
}

impl ContactInfoItemsQueryBuilder {
    /// Sets wire field `ids`.
    pub fn ids(mut self, value: Vec<super::ids::ContactInfoId>) -> Self {
        self.ids = Some(value);
        self
    }

    /// Validates required fields and builds [`ContactInfoItemsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ContactInfoItemsQuery, crate::api::current::BuildError> {
        let ids = self
            .ids
            .ok_or(crate::api::current::BuildError::missing("ids"))?;
        if ids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "ids",
                "must not be empty",
            ));
        }
        Ok(ContactInfoItemsQuery { ids })
    }
}

impl crate::api::current::support::CurrentQuery for ContactInfoItemsQuery {
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
    /// Calls the current `GET /contactInfo/items` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn contact_info_items(
        &self,
        query: &ContactInfoItemsQuery,
    ) -> Result<Vec<ContactInfo>, crate::Error> {
        self.get_current("/contactInfo/items", query).await
    }
}

/// Typed query parameters for `/contactInfo/ldeps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ContactInfoLDependentsQuery {
    #[serde(rename = "masterids")]
    masterids: Vec<super::ids::ProviderEntityId>,
}

impl ContactInfoLDependentsQuery {
    /// Returns wire field `masterids`.
    #[must_use]
    pub fn masterids(&self) -> &[super::ids::ProviderEntityId] {
        &self.masterids
    }

    /// Starts a builder for [`ContactInfoLDependentsQuery`].
    pub fn builder() -> ContactInfoLDependentsQueryBuilder {
        ContactInfoLDependentsQueryBuilder::default()
    }
}

/// Builder for [`ContactInfoLDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ContactInfoLDependentsQueryBuilder {
    masterids: Option<Vec<super::ids::ProviderEntityId>>,
}

impl ContactInfoLDependentsQueryBuilder {
    /// Sets wire field `masterids`.
    pub fn masterids(mut self, value: Vec<super::ids::ProviderEntityId>) -> Self {
        self.masterids = Some(value);
        self
    }

    /// Validates required fields and builds [`ContactInfoLDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ContactInfoLDependentsQuery, crate::api::current::BuildError> {
        let masterids = self
            .masterids
            .ok_or(crate::api::current::BuildError::missing("masterids"))?;
        if masterids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "masterids",
                "must not be empty",
            ));
        }
        Ok(ContactInfoLDependentsQuery { masterids })
    }
}

impl crate::api::current::support::CurrentQuery for ContactInfoLDependentsQuery {
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
    /// Calls the current `GET /contactInfo/ldeps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn contact_info_l_dependents(
        &self,
        query: &ContactInfoLDependentsQuery,
    ) -> Result<Vec<ContactInfo>, crate::Error> {
        self.get_current("/contactInfo/ldeps", query).await
    }
}

impl crate::Client {
    /// Calls the current `POST /user/getaccounttradingpermissions` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn user_get_account_trading_permissions(
        &self,
        request: &GetAccountTradingPermissions,
    ) -> Result<TradingPermissionsResponse, crate::Error> {
        crate::api::current::support::CurrentRequest::validate_current(request)?;
        self.post_query("/user/getaccounttradingpermissions", request)
            .await
    }
}

impl crate::Client {
    /// Calls the current `GET /organization/getorgworkspacetemplate` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn organization_get_org_workspace_template(
        &self,
    ) -> Result<OrgWorkspaceTemplateResponse, crate::Error> {
        self.get_without_query("/organization/getorgworkspacetemplate")
            .await
    }
}

impl crate::Client {
    /// Calls the current `POST /customerApplication/getpartnersubaccountrequeststatus` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn customer_application_get_partner_subaccount_request_status(
        &self,
        request: &GetPartnerSubAccountRequestStatus,
    ) -> Result<PartnerSubAccountRequestStatusResponse, crate::Error> {
        crate::api::current::support::CurrentRequest::validate_current(request)?;
        self.post_query(
            "/customerApplication/getpartnersubaccountrequeststatus",
            request,
        )
        .await
    }
}

/// Typed query parameters for `/marketDataSubscription/deps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct MarketDataSubscriptionDependentsQuery {
    #[serde(rename = "masterid")]
    masterid: super::ids::ProviderEntityId,
}

impl MarketDataSubscriptionDependentsQuery {
    /// Returns wire field `masterid`.
    #[must_use]
    pub fn masterid(&self) -> &super::ids::ProviderEntityId {
        &self.masterid
    }

    /// Starts a builder for [`MarketDataSubscriptionDependentsQuery`].
    pub fn builder() -> MarketDataSubscriptionDependentsQueryBuilder {
        MarketDataSubscriptionDependentsQueryBuilder::default()
    }
}

/// Builder for [`MarketDataSubscriptionDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct MarketDataSubscriptionDependentsQueryBuilder {
    masterid: Option<super::ids::ProviderEntityId>,
}

impl MarketDataSubscriptionDependentsQueryBuilder {
    /// Sets wire field `masterid`.
    pub fn masterid(mut self, value: super::ids::ProviderEntityId) -> Self {
        self.masterid = Some(value);
        self
    }

    /// Validates required fields and builds [`MarketDataSubscriptionDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(
        self,
    ) -> Result<MarketDataSubscriptionDependentsQuery, crate::api::current::BuildError> {
        let masterid = self
            .masterid
            .ok_or(crate::api::current::BuildError::missing("masterid"))?;
        Ok(MarketDataSubscriptionDependentsQuery { masterid })
    }
}

impl crate::api::current::support::CurrentQuery for MarketDataSubscriptionDependentsQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "masterid", &self.masterid)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /marketDataSubscription/deps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn market_data_subscription_dependents(
        &self,
        query: &MarketDataSubscriptionDependentsQuery,
    ) -> Result<Vec<MarketDataSubscription>, crate::Error> {
        self.get_current("/marketDataSubscription/deps", query)
            .await
    }
}

/// Typed query parameters for `/marketDataSubscription/item`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct MarketDataSubscriptionItemQuery {
    #[serde(rename = "id")]
    id: super::ids::MarketDataSubscriptionId,
}

impl MarketDataSubscriptionItemQuery {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> &super::ids::MarketDataSubscriptionId {
        &self.id
    }

    /// Starts a builder for [`MarketDataSubscriptionItemQuery`].
    pub fn builder() -> MarketDataSubscriptionItemQueryBuilder {
        MarketDataSubscriptionItemQueryBuilder::default()
    }
}

/// Builder for [`MarketDataSubscriptionItemQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct MarketDataSubscriptionItemQueryBuilder {
    id: Option<super::ids::MarketDataSubscriptionId>,
}

impl MarketDataSubscriptionItemQueryBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::MarketDataSubscriptionId) -> Self {
        self.id = Some(value);
        self
    }

    /// Validates required fields and builds [`MarketDataSubscriptionItemQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<MarketDataSubscriptionItemQuery, crate::api::current::BuildError> {
        let id = self
            .id
            .ok_or(crate::api::current::BuildError::missing("id"))?;
        Ok(MarketDataSubscriptionItemQuery { id })
    }
}

impl crate::api::current::support::CurrentQuery for MarketDataSubscriptionItemQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "id", &self.id)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /marketDataSubscription/item` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn market_data_subscription_item(
        &self,
        query: &MarketDataSubscriptionItemQuery,
    ) -> Result<MarketDataSubscription, crate::Error> {
        self.get_current("/marketDataSubscription/item", query)
            .await
    }
}

/// Typed query parameters for `/marketDataSubscription/items`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct MarketDataSubscriptionItemsQuery {
    #[serde(rename = "ids")]
    ids: Vec<super::ids::MarketDataSubscriptionId>,
}

impl MarketDataSubscriptionItemsQuery {
    /// Returns wire field `ids`.
    #[must_use]
    pub fn ids(&self) -> &[super::ids::MarketDataSubscriptionId] {
        &self.ids
    }

    /// Starts a builder for [`MarketDataSubscriptionItemsQuery`].
    pub fn builder() -> MarketDataSubscriptionItemsQueryBuilder {
        MarketDataSubscriptionItemsQueryBuilder::default()
    }
}

/// Builder for [`MarketDataSubscriptionItemsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct MarketDataSubscriptionItemsQueryBuilder {
    ids: Option<Vec<super::ids::MarketDataSubscriptionId>>,
}

impl MarketDataSubscriptionItemsQueryBuilder {
    /// Sets wire field `ids`.
    pub fn ids(mut self, value: Vec<super::ids::MarketDataSubscriptionId>) -> Self {
        self.ids = Some(value);
        self
    }

    /// Validates required fields and builds [`MarketDataSubscriptionItemsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(
        self,
    ) -> Result<MarketDataSubscriptionItemsQuery, crate::api::current::BuildError> {
        let ids = self
            .ids
            .ok_or(crate::api::current::BuildError::missing("ids"))?;
        if ids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "ids",
                "must not be empty",
            ));
        }
        Ok(MarketDataSubscriptionItemsQuery { ids })
    }
}

impl crate::api::current::support::CurrentQuery for MarketDataSubscriptionItemsQuery {
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
    /// Calls the current `GET /marketDataSubscription/items` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn market_data_subscription_items(
        &self,
        query: &MarketDataSubscriptionItemsQuery,
    ) -> Result<Vec<MarketDataSubscription>, crate::Error> {
        self.get_current("/marketDataSubscription/items", query)
            .await
    }
}

/// Typed query parameters for `/marketDataSubscription/ldeps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct MarketDataSubscriptionLDependentsQuery {
    #[serde(rename = "masterids")]
    masterids: Vec<super::ids::ProviderEntityId>,
}

impl MarketDataSubscriptionLDependentsQuery {
    /// Returns wire field `masterids`.
    #[must_use]
    pub fn masterids(&self) -> &[super::ids::ProviderEntityId] {
        &self.masterids
    }

    /// Starts a builder for [`MarketDataSubscriptionLDependentsQuery`].
    pub fn builder() -> MarketDataSubscriptionLDependentsQueryBuilder {
        MarketDataSubscriptionLDependentsQueryBuilder::default()
    }
}

/// Builder for [`MarketDataSubscriptionLDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct MarketDataSubscriptionLDependentsQueryBuilder {
    masterids: Option<Vec<super::ids::ProviderEntityId>>,
}

impl MarketDataSubscriptionLDependentsQueryBuilder {
    /// Sets wire field `masterids`.
    pub fn masterids(mut self, value: Vec<super::ids::ProviderEntityId>) -> Self {
        self.masterids = Some(value);
        self
    }

    /// Validates required fields and builds [`MarketDataSubscriptionLDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(
        self,
    ) -> Result<MarketDataSubscriptionLDependentsQuery, crate::api::current::BuildError> {
        let masterids = self
            .masterids
            .ok_or(crate::api::current::BuildError::missing("masterids"))?;
        if masterids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "masterids",
                "must not be empty",
            ));
        }
        Ok(MarketDataSubscriptionLDependentsQuery { masterids })
    }
}

impl crate::api::current::support::CurrentQuery for MarketDataSubscriptionLDependentsQuery {
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
    /// Calls the current `GET /marketDataSubscription/ldeps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn market_data_subscription_l_dependents(
        &self,
        query: &MarketDataSubscriptionLDependentsQuery,
    ) -> Result<Vec<MarketDataSubscription>, crate::Error> {
        self.get_current("/marketDataSubscription/ldeps", query)
            .await
    }
}

/// Typed query parameters for `/organization/find`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct OrganizationFindQuery {
    #[serde(rename = "name")]
    name: String,
}

impl OrganizationFindQuery {
    /// Returns wire field `name`.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Starts a builder for [`OrganizationFindQuery`].
    pub fn builder() -> OrganizationFindQueryBuilder {
        OrganizationFindQueryBuilder::default()
    }
}

/// Builder for [`OrganizationFindQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct OrganizationFindQueryBuilder {
    name: Option<String>,
}

impl OrganizationFindQueryBuilder {
    /// Sets wire field `name`.
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Validates required fields and builds [`OrganizationFindQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<OrganizationFindQuery, crate::api::current::BuildError> {
        let name = self
            .name
            .ok_or(crate::api::current::BuildError::missing("name"))?;
        if name.is_empty() || name.trim() != name {
            return Err(crate::api::current::BuildError::invalid(
                "name",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        Ok(OrganizationFindQuery { name })
    }
}

impl crate::api::current::support::CurrentQuery for OrganizationFindQuery {
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
    /// Calls the current `GET /organization/find` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn organization_find(
        &self,
        query: &OrganizationFindQuery,
    ) -> Result<Organization, crate::Error> {
        self.get_current("/organization/find", query).await
    }
}

/// Typed query parameters for `/organization/item`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct OrganizationItemQuery {
    #[serde(rename = "id")]
    id: super::ids::OrganizationId,
}

impl OrganizationItemQuery {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> &super::ids::OrganizationId {
        &self.id
    }

    /// Starts a builder for [`OrganizationItemQuery`].
    pub fn builder() -> OrganizationItemQueryBuilder {
        OrganizationItemQueryBuilder::default()
    }
}

/// Builder for [`OrganizationItemQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct OrganizationItemQueryBuilder {
    id: Option<super::ids::OrganizationId>,
}

impl OrganizationItemQueryBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::OrganizationId) -> Self {
        self.id = Some(value);
        self
    }

    /// Validates required fields and builds [`OrganizationItemQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<OrganizationItemQuery, crate::api::current::BuildError> {
        let id = self
            .id
            .ok_or(crate::api::current::BuildError::missing("id"))?;
        Ok(OrganizationItemQuery { id })
    }
}

impl crate::api::current::support::CurrentQuery for OrganizationItemQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "id", &self.id)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /organization/item` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn organization_item(
        &self,
        query: &OrganizationItemQuery,
    ) -> Result<Organization, crate::Error> {
        self.get_current("/organization/item", query).await
    }
}

/// Typed query parameters for `/organization/items`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct OrganizationItemsQuery {
    #[serde(rename = "ids")]
    ids: Vec<super::ids::OrganizationId>,
}

impl OrganizationItemsQuery {
    /// Returns wire field `ids`.
    #[must_use]
    pub fn ids(&self) -> &[super::ids::OrganizationId] {
        &self.ids
    }

    /// Starts a builder for [`OrganizationItemsQuery`].
    pub fn builder() -> OrganizationItemsQueryBuilder {
        OrganizationItemsQueryBuilder::default()
    }
}

/// Builder for [`OrganizationItemsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct OrganizationItemsQueryBuilder {
    ids: Option<Vec<super::ids::OrganizationId>>,
}

impl OrganizationItemsQueryBuilder {
    /// Sets wire field `ids`.
    pub fn ids(mut self, value: Vec<super::ids::OrganizationId>) -> Self {
        self.ids = Some(value);
        self
    }

    /// Validates required fields and builds [`OrganizationItemsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<OrganizationItemsQuery, crate::api::current::BuildError> {
        let ids = self
            .ids
            .ok_or(crate::api::current::BuildError::missing("ids"))?;
        if ids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "ids",
                "must not be empty",
            ));
        }
        Ok(OrganizationItemsQuery { ids })
    }
}

impl crate::api::current::support::CurrentQuery for OrganizationItemsQuery {
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
    /// Calls the current `GET /organization/items` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn organization_items(
        &self,
        query: &OrganizationItemsQuery,
    ) -> Result<Vec<Organization>, crate::Error> {
        self.get_current("/organization/items", query).await
    }
}

impl crate::Client {
    /// Calls the current `GET /organization/list` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn organization_list(&self) -> Result<Vec<Organization>, crate::Error> {
        self.get_without_query("/organization/list").await
    }
}

/// Typed query parameters for `/organization/suggest`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct OrganizationSuggestQuery {
    #[serde(rename = "t")]
    t: String,
    #[serde(rename = "l")]
    l: i64,
}

impl OrganizationSuggestQuery {
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

    /// Starts a builder for [`OrganizationSuggestQuery`].
    pub fn builder() -> OrganizationSuggestQueryBuilder {
        OrganizationSuggestQueryBuilder::default()
    }
}

/// Builder for [`OrganizationSuggestQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct OrganizationSuggestQueryBuilder {
    t: Option<String>,
    l: Option<i64>,
}

impl OrganizationSuggestQueryBuilder {
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

    /// Validates required fields and builds [`OrganizationSuggestQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<OrganizationSuggestQuery, crate::api::current::BuildError> {
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
        Ok(OrganizationSuggestQuery { t, l })
    }
}

impl crate::api::current::support::CurrentQuery for OrganizationSuggestQuery {
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
    /// Calls the current `GET /organization/suggest` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn organization_suggest(
        &self,
        query: &OrganizationSuggestQuery,
    ) -> Result<Vec<Organization>, crate::Error> {
        self.get_current("/organization/suggest", query).await
    }
}

/// Typed query parameters for `/secondMarketDataSubscription/deps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct SecondMarketDataSubscriptionDependentsQuery {
    #[serde(rename = "masterid")]
    masterid: super::ids::ProviderEntityId,
}

impl SecondMarketDataSubscriptionDependentsQuery {
    /// Returns wire field `masterid`.
    #[must_use]
    pub fn masterid(&self) -> &super::ids::ProviderEntityId {
        &self.masterid
    }

    /// Starts a builder for [`SecondMarketDataSubscriptionDependentsQuery`].
    pub fn builder() -> SecondMarketDataSubscriptionDependentsQueryBuilder {
        SecondMarketDataSubscriptionDependentsQueryBuilder::default()
    }
}

/// Builder for [`SecondMarketDataSubscriptionDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct SecondMarketDataSubscriptionDependentsQueryBuilder {
    masterid: Option<super::ids::ProviderEntityId>,
}

impl SecondMarketDataSubscriptionDependentsQueryBuilder {
    /// Sets wire field `masterid`.
    pub fn masterid(mut self, value: super::ids::ProviderEntityId) -> Self {
        self.masterid = Some(value);
        self
    }

    /// Validates required fields and builds [`SecondMarketDataSubscriptionDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(
        self,
    ) -> Result<SecondMarketDataSubscriptionDependentsQuery, crate::api::current::BuildError> {
        let masterid = self
            .masterid
            .ok_or(crate::api::current::BuildError::missing("masterid"))?;
        Ok(SecondMarketDataSubscriptionDependentsQuery { masterid })
    }
}

impl crate::api::current::support::CurrentQuery for SecondMarketDataSubscriptionDependentsQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "masterid", &self.masterid)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /secondMarketDataSubscription/deps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn second_market_data_subscription_dependents(
        &self,
        query: &SecondMarketDataSubscriptionDependentsQuery,
    ) -> Result<Vec<SecondMarketDataSubscription>, crate::Error> {
        self.get_current("/secondMarketDataSubscription/deps", query)
            .await
    }
}

/// Typed query parameters for `/secondMarketDataSubscription/item`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct SecondMarketDataSubscriptionItemQuery {
    #[serde(rename = "id")]
    id: super::ids::SecondMarketDataSubscriptionId,
}

impl SecondMarketDataSubscriptionItemQuery {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> &super::ids::SecondMarketDataSubscriptionId {
        &self.id
    }

    /// Starts a builder for [`SecondMarketDataSubscriptionItemQuery`].
    pub fn builder() -> SecondMarketDataSubscriptionItemQueryBuilder {
        SecondMarketDataSubscriptionItemQueryBuilder::default()
    }
}

/// Builder for [`SecondMarketDataSubscriptionItemQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct SecondMarketDataSubscriptionItemQueryBuilder {
    id: Option<super::ids::SecondMarketDataSubscriptionId>,
}

impl SecondMarketDataSubscriptionItemQueryBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::SecondMarketDataSubscriptionId) -> Self {
        self.id = Some(value);
        self
    }

    /// Validates required fields and builds [`SecondMarketDataSubscriptionItemQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(
        self,
    ) -> Result<SecondMarketDataSubscriptionItemQuery, crate::api::current::BuildError> {
        let id = self
            .id
            .ok_or(crate::api::current::BuildError::missing("id"))?;
        Ok(SecondMarketDataSubscriptionItemQuery { id })
    }
}

impl crate::api::current::support::CurrentQuery for SecondMarketDataSubscriptionItemQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "id", &self.id)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /secondMarketDataSubscription/item` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn second_market_data_subscription_item(
        &self,
        query: &SecondMarketDataSubscriptionItemQuery,
    ) -> Result<SecondMarketDataSubscription, crate::Error> {
        self.get_current("/secondMarketDataSubscription/item", query)
            .await
    }
}

/// Typed query parameters for `/secondMarketDataSubscription/items`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct SecondMarketDataSubscriptionItemsQuery {
    #[serde(rename = "ids")]
    ids: Vec<super::ids::SecondMarketDataSubscriptionId>,
}

impl SecondMarketDataSubscriptionItemsQuery {
    /// Returns wire field `ids`.
    #[must_use]
    pub fn ids(&self) -> &[super::ids::SecondMarketDataSubscriptionId] {
        &self.ids
    }

    /// Starts a builder for [`SecondMarketDataSubscriptionItemsQuery`].
    pub fn builder() -> SecondMarketDataSubscriptionItemsQueryBuilder {
        SecondMarketDataSubscriptionItemsQueryBuilder::default()
    }
}

/// Builder for [`SecondMarketDataSubscriptionItemsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct SecondMarketDataSubscriptionItemsQueryBuilder {
    ids: Option<Vec<super::ids::SecondMarketDataSubscriptionId>>,
}

impl SecondMarketDataSubscriptionItemsQueryBuilder {
    /// Sets wire field `ids`.
    pub fn ids(mut self, value: Vec<super::ids::SecondMarketDataSubscriptionId>) -> Self {
        self.ids = Some(value);
        self
    }

    /// Validates required fields and builds [`SecondMarketDataSubscriptionItemsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(
        self,
    ) -> Result<SecondMarketDataSubscriptionItemsQuery, crate::api::current::BuildError> {
        let ids = self
            .ids
            .ok_or(crate::api::current::BuildError::missing("ids"))?;
        if ids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "ids",
                "must not be empty",
            ));
        }
        Ok(SecondMarketDataSubscriptionItemsQuery { ids })
    }
}

impl crate::api::current::support::CurrentQuery for SecondMarketDataSubscriptionItemsQuery {
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
    /// Calls the current `GET /secondMarketDataSubscription/items` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn second_market_data_subscription_items(
        &self,
        query: &SecondMarketDataSubscriptionItemsQuery,
    ) -> Result<Vec<SecondMarketDataSubscription>, crate::Error> {
        self.get_current("/secondMarketDataSubscription/items", query)
            .await
    }
}

/// Typed query parameters for `/secondMarketDataSubscription/ldeps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct SecondMarketDataSubscriptionLDependentsQuery {
    #[serde(rename = "masterids")]
    masterids: Vec<super::ids::ProviderEntityId>,
}

impl SecondMarketDataSubscriptionLDependentsQuery {
    /// Returns wire field `masterids`.
    #[must_use]
    pub fn masterids(&self) -> &[super::ids::ProviderEntityId] {
        &self.masterids
    }

    /// Starts a builder for [`SecondMarketDataSubscriptionLDependentsQuery`].
    pub fn builder() -> SecondMarketDataSubscriptionLDependentsQueryBuilder {
        SecondMarketDataSubscriptionLDependentsQueryBuilder::default()
    }
}

/// Builder for [`SecondMarketDataSubscriptionLDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct SecondMarketDataSubscriptionLDependentsQueryBuilder {
    masterids: Option<Vec<super::ids::ProviderEntityId>>,
}

impl SecondMarketDataSubscriptionLDependentsQueryBuilder {
    /// Sets wire field `masterids`.
    pub fn masterids(mut self, value: Vec<super::ids::ProviderEntityId>) -> Self {
        self.masterids = Some(value);
        self
    }

    /// Validates required fields and builds [`SecondMarketDataSubscriptionLDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(
        self,
    ) -> Result<SecondMarketDataSubscriptionLDependentsQuery, crate::api::current::BuildError> {
        let masterids = self
            .masterids
            .ok_or(crate::api::current::BuildError::missing("masterids"))?;
        if masterids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "masterids",
                "must not be empty",
            ));
        }
        Ok(SecondMarketDataSubscriptionLDependentsQuery { masterids })
    }
}

impl crate::api::current::support::CurrentQuery for SecondMarketDataSubscriptionLDependentsQuery {
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
    /// Calls the current `GET /secondMarketDataSubscription/ldeps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn second_market_data_subscription_l_dependents(
        &self,
        query: &SecondMarketDataSubscriptionLDependentsQuery,
    ) -> Result<Vec<SecondMarketDataSubscription>, crate::Error> {
        self.get_current("/secondMarketDataSubscription/ldeps", query)
            .await
    }
}

impl crate::Client {
    /// Calls the current `GET /secondMarketDataSubscription/list` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn second_market_data_subscription_list(
        &self,
    ) -> Result<Vec<SecondMarketDataSubscription>, crate::Error> {
        self.get_without_query("/secondMarketDataSubscription/list")
            .await
    }
}

/// Typed query parameters for `/tradovateSubscription/deps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct TradovateSubscriptionDependentsQuery {
    #[serde(rename = "masterid")]
    masterid: super::ids::ProviderEntityId,
}

impl TradovateSubscriptionDependentsQuery {
    /// Returns wire field `masterid`.
    #[must_use]
    pub fn masterid(&self) -> &super::ids::ProviderEntityId {
        &self.masterid
    }

    /// Starts a builder for [`TradovateSubscriptionDependentsQuery`].
    pub fn builder() -> TradovateSubscriptionDependentsQueryBuilder {
        TradovateSubscriptionDependentsQueryBuilder::default()
    }
}

/// Builder for [`TradovateSubscriptionDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct TradovateSubscriptionDependentsQueryBuilder {
    masterid: Option<super::ids::ProviderEntityId>,
}

impl TradovateSubscriptionDependentsQueryBuilder {
    /// Sets wire field `masterid`.
    pub fn masterid(mut self, value: super::ids::ProviderEntityId) -> Self {
        self.masterid = Some(value);
        self
    }

    /// Validates required fields and builds [`TradovateSubscriptionDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(
        self,
    ) -> Result<TradovateSubscriptionDependentsQuery, crate::api::current::BuildError> {
        let masterid = self
            .masterid
            .ok_or(crate::api::current::BuildError::missing("masterid"))?;
        Ok(TradovateSubscriptionDependentsQuery { masterid })
    }
}

impl crate::api::current::support::CurrentQuery for TradovateSubscriptionDependentsQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "masterid", &self.masterid)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /tradovateSubscription/deps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn tradovate_subscription_dependents(
        &self,
        query: &TradovateSubscriptionDependentsQuery,
    ) -> Result<Vec<TradovateSubscription>, crate::Error> {
        self.get_current("/tradovateSubscription/deps", query).await
    }
}

/// Typed query parameters for `/tradovateSubscription/item`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct TradovateSubscriptionItemQuery {
    #[serde(rename = "id")]
    id: super::ids::TradovateSubscriptionId,
}

impl TradovateSubscriptionItemQuery {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> &super::ids::TradovateSubscriptionId {
        &self.id
    }

    /// Starts a builder for [`TradovateSubscriptionItemQuery`].
    pub fn builder() -> TradovateSubscriptionItemQueryBuilder {
        TradovateSubscriptionItemQueryBuilder::default()
    }
}

/// Builder for [`TradovateSubscriptionItemQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct TradovateSubscriptionItemQueryBuilder {
    id: Option<super::ids::TradovateSubscriptionId>,
}

impl TradovateSubscriptionItemQueryBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::TradovateSubscriptionId) -> Self {
        self.id = Some(value);
        self
    }

    /// Validates required fields and builds [`TradovateSubscriptionItemQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<TradovateSubscriptionItemQuery, crate::api::current::BuildError> {
        let id = self
            .id
            .ok_or(crate::api::current::BuildError::missing("id"))?;
        Ok(TradovateSubscriptionItemQuery { id })
    }
}

impl crate::api::current::support::CurrentQuery for TradovateSubscriptionItemQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "id", &self.id)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /tradovateSubscription/item` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn tradovate_subscription_item(
        &self,
        query: &TradovateSubscriptionItemQuery,
    ) -> Result<TradovateSubscription, crate::Error> {
        self.get_current("/tradovateSubscription/item", query).await
    }
}

/// Typed query parameters for `/tradovateSubscription/items`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct TradovateSubscriptionItemsQuery {
    #[serde(rename = "ids")]
    ids: Vec<super::ids::TradovateSubscriptionId>,
}

impl TradovateSubscriptionItemsQuery {
    /// Returns wire field `ids`.
    #[must_use]
    pub fn ids(&self) -> &[super::ids::TradovateSubscriptionId] {
        &self.ids
    }

    /// Starts a builder for [`TradovateSubscriptionItemsQuery`].
    pub fn builder() -> TradovateSubscriptionItemsQueryBuilder {
        TradovateSubscriptionItemsQueryBuilder::default()
    }
}

/// Builder for [`TradovateSubscriptionItemsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct TradovateSubscriptionItemsQueryBuilder {
    ids: Option<Vec<super::ids::TradovateSubscriptionId>>,
}

impl TradovateSubscriptionItemsQueryBuilder {
    /// Sets wire field `ids`.
    pub fn ids(mut self, value: Vec<super::ids::TradovateSubscriptionId>) -> Self {
        self.ids = Some(value);
        self
    }

    /// Validates required fields and builds [`TradovateSubscriptionItemsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<TradovateSubscriptionItemsQuery, crate::api::current::BuildError> {
        let ids = self
            .ids
            .ok_or(crate::api::current::BuildError::missing("ids"))?;
        if ids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "ids",
                "must not be empty",
            ));
        }
        Ok(TradovateSubscriptionItemsQuery { ids })
    }
}

impl crate::api::current::support::CurrentQuery for TradovateSubscriptionItemsQuery {
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
    /// Calls the current `GET /tradovateSubscription/items` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn tradovate_subscription_items(
        &self,
        query: &TradovateSubscriptionItemsQuery,
    ) -> Result<Vec<TradovateSubscription>, crate::Error> {
        self.get_current("/tradovateSubscription/items", query)
            .await
    }
}

/// Typed query parameters for `/tradovateSubscription/ldeps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct TradovateSubscriptionLDependentsQuery {
    #[serde(rename = "masterids")]
    masterids: Vec<super::ids::ProviderEntityId>,
}

impl TradovateSubscriptionLDependentsQuery {
    /// Returns wire field `masterids`.
    #[must_use]
    pub fn masterids(&self) -> &[super::ids::ProviderEntityId] {
        &self.masterids
    }

    /// Starts a builder for [`TradovateSubscriptionLDependentsQuery`].
    pub fn builder() -> TradovateSubscriptionLDependentsQueryBuilder {
        TradovateSubscriptionLDependentsQueryBuilder::default()
    }
}

/// Builder for [`TradovateSubscriptionLDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct TradovateSubscriptionLDependentsQueryBuilder {
    masterids: Option<Vec<super::ids::ProviderEntityId>>,
}

impl TradovateSubscriptionLDependentsQueryBuilder {
    /// Sets wire field `masterids`.
    pub fn masterids(mut self, value: Vec<super::ids::ProviderEntityId>) -> Self {
        self.masterids = Some(value);
        self
    }

    /// Validates required fields and builds [`TradovateSubscriptionLDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(
        self,
    ) -> Result<TradovateSubscriptionLDependentsQuery, crate::api::current::BuildError> {
        let masterids = self
            .masterids
            .ok_or(crate::api::current::BuildError::missing("masterids"))?;
        if masterids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "masterids",
                "must not be empty",
            ));
        }
        Ok(TradovateSubscriptionLDependentsQuery { masterids })
    }
}

impl crate::api::current::support::CurrentQuery for TradovateSubscriptionLDependentsQuery {
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
    /// Calls the current `GET /tradovateSubscription/ldeps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn tradovate_subscription_l_dependents(
        &self,
        query: &TradovateSubscriptionLDependentsQuery,
    ) -> Result<Vec<TradovateSubscription>, crate::Error> {
        self.get_current("/tradovateSubscription/ldeps", query)
            .await
    }
}

/// Typed query parameters for `/user/find`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct UserFindQuery {
    #[serde(rename = "name")]
    name: String,
}

impl UserFindQuery {
    /// Returns wire field `name`.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Starts a builder for [`UserFindQuery`].
    pub fn builder() -> UserFindQueryBuilder {
        UserFindQueryBuilder::default()
    }
}

/// Builder for [`UserFindQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct UserFindQueryBuilder {
    name: Option<String>,
}

impl UserFindQueryBuilder {
    /// Sets wire field `name`.
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Validates required fields and builds [`UserFindQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<UserFindQuery, crate::api::current::BuildError> {
        let name = self
            .name
            .ok_or(crate::api::current::BuildError::missing("name"))?;
        if name.is_empty() || name.trim() != name {
            return Err(crate::api::current::BuildError::invalid(
                "name",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        Ok(UserFindQuery { name })
    }
}

impl crate::api::current::support::CurrentQuery for UserFindQuery {
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
    /// Calls the current `GET /user/find` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn user_find(&self, query: &UserFindQuery) -> Result<User, crate::Error> {
        self.get_current("/user/find", query).await
    }
}

/// Typed query parameters for `/user/item`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct UserItemQuery {
    #[serde(rename = "id")]
    id: crate::UserId,
}

impl UserItemQuery {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> &crate::UserId {
        &self.id
    }

    /// Starts a builder for [`UserItemQuery`].
    pub fn builder() -> UserItemQueryBuilder {
        UserItemQueryBuilder::default()
    }
}

/// Builder for [`UserItemQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct UserItemQueryBuilder {
    id: Option<crate::UserId>,
}

impl UserItemQueryBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: crate::UserId) -> Self {
        self.id = Some(value);
        self
    }

    /// Validates required fields and builds [`UserItemQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<UserItemQuery, crate::api::current::BuildError> {
        let id = self
            .id
            .ok_or(crate::api::current::BuildError::missing("id"))?;
        Ok(UserItemQuery { id })
    }
}

impl crate::api::current::support::CurrentQuery for UserItemQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "id", &self.id)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /user/item` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn user_item(&self, query: &UserItemQuery) -> Result<User, crate::Error> {
        self.get_current("/user/item", query).await
    }
}

/// Typed query parameters for `/user/items`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct UserItemsQuery {
    #[serde(rename = "ids")]
    ids: Vec<crate::UserId>,
}

impl UserItemsQuery {
    /// Returns wire field `ids`.
    #[must_use]
    pub fn ids(&self) -> &[crate::UserId] {
        &self.ids
    }

    /// Starts a builder for [`UserItemsQuery`].
    pub fn builder() -> UserItemsQueryBuilder {
        UserItemsQueryBuilder::default()
    }
}

/// Builder for [`UserItemsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct UserItemsQueryBuilder {
    ids: Option<Vec<crate::UserId>>,
}

impl UserItemsQueryBuilder {
    /// Sets wire field `ids`.
    pub fn ids(mut self, value: Vec<crate::UserId>) -> Self {
        self.ids = Some(value);
        self
    }

    /// Validates required fields and builds [`UserItemsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<UserItemsQuery, crate::api::current::BuildError> {
        let ids = self
            .ids
            .ok_or(crate::api::current::BuildError::missing("ids"))?;
        if ids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "ids",
                "must not be empty",
            ));
        }
        Ok(UserItemsQuery { ids })
    }
}

impl crate::api::current::support::CurrentQuery for UserItemsQuery {
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
    /// Calls the current `GET /user/items` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn user_items(&self, query: &UserItemsQuery) -> Result<Vec<User>, crate::Error> {
        self.get_current("/user/items", query).await
    }
}

impl crate::Client {
    /// Calls the current `GET /user/list` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn user_list(&self) -> Result<Vec<User>, crate::Error> {
        self.get_without_query("/user/list").await
    }
}

/// Typed query parameters for `/userPlugin/deps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct UserPluginDependentsQuery {
    #[serde(rename = "masterid")]
    masterid: super::ids::ProviderEntityId,
}

impl UserPluginDependentsQuery {
    /// Returns wire field `masterid`.
    #[must_use]
    pub fn masterid(&self) -> &super::ids::ProviderEntityId {
        &self.masterid
    }

    /// Starts a builder for [`UserPluginDependentsQuery`].
    pub fn builder() -> UserPluginDependentsQueryBuilder {
        UserPluginDependentsQueryBuilder::default()
    }
}

/// Builder for [`UserPluginDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct UserPluginDependentsQueryBuilder {
    masterid: Option<super::ids::ProviderEntityId>,
}

impl UserPluginDependentsQueryBuilder {
    /// Sets wire field `masterid`.
    pub fn masterid(mut self, value: super::ids::ProviderEntityId) -> Self {
        self.masterid = Some(value);
        self
    }

    /// Validates required fields and builds [`UserPluginDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<UserPluginDependentsQuery, crate::api::current::BuildError> {
        let masterid = self
            .masterid
            .ok_or(crate::api::current::BuildError::missing("masterid"))?;
        Ok(UserPluginDependentsQuery { masterid })
    }
}

impl crate::api::current::support::CurrentQuery for UserPluginDependentsQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "masterid", &self.masterid)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /userPlugin/deps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn user_plugin_dependents(
        &self,
        query: &UserPluginDependentsQuery,
    ) -> Result<Vec<UserPlugin>, crate::Error> {
        self.get_current("/userPlugin/deps", query).await
    }
}

/// Typed query parameters for `/userPlugin/item`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct UserPluginItemQuery {
    #[serde(rename = "id")]
    id: super::ids::UserPluginId,
}

impl UserPluginItemQuery {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> &super::ids::UserPluginId {
        &self.id
    }

    /// Starts a builder for [`UserPluginItemQuery`].
    pub fn builder() -> UserPluginItemQueryBuilder {
        UserPluginItemQueryBuilder::default()
    }
}

/// Builder for [`UserPluginItemQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct UserPluginItemQueryBuilder {
    id: Option<super::ids::UserPluginId>,
}

impl UserPluginItemQueryBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::UserPluginId) -> Self {
        self.id = Some(value);
        self
    }

    /// Validates required fields and builds [`UserPluginItemQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<UserPluginItemQuery, crate::api::current::BuildError> {
        let id = self
            .id
            .ok_or(crate::api::current::BuildError::missing("id"))?;
        Ok(UserPluginItemQuery { id })
    }
}

impl crate::api::current::support::CurrentQuery for UserPluginItemQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "id", &self.id)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /userPlugin/item` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn user_plugin_item(
        &self,
        query: &UserPluginItemQuery,
    ) -> Result<UserPlugin, crate::Error> {
        self.get_current("/userPlugin/item", query).await
    }
}

/// Typed query parameters for `/userPlugin/items`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct UserPluginItemsQuery {
    #[serde(rename = "ids")]
    ids: Vec<super::ids::UserPluginId>,
}

impl UserPluginItemsQuery {
    /// Returns wire field `ids`.
    #[must_use]
    pub fn ids(&self) -> &[super::ids::UserPluginId] {
        &self.ids
    }

    /// Starts a builder for [`UserPluginItemsQuery`].
    pub fn builder() -> UserPluginItemsQueryBuilder {
        UserPluginItemsQueryBuilder::default()
    }
}

/// Builder for [`UserPluginItemsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct UserPluginItemsQueryBuilder {
    ids: Option<Vec<super::ids::UserPluginId>>,
}

impl UserPluginItemsQueryBuilder {
    /// Sets wire field `ids`.
    pub fn ids(mut self, value: Vec<super::ids::UserPluginId>) -> Self {
        self.ids = Some(value);
        self
    }

    /// Validates required fields and builds [`UserPluginItemsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<UserPluginItemsQuery, crate::api::current::BuildError> {
        let ids = self
            .ids
            .ok_or(crate::api::current::BuildError::missing("ids"))?;
        if ids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "ids",
                "must not be empty",
            ));
        }
        Ok(UserPluginItemsQuery { ids })
    }
}

impl crate::api::current::support::CurrentQuery for UserPluginItemsQuery {
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
    /// Calls the current `GET /userPlugin/items` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn user_plugin_items(
        &self,
        query: &UserPluginItemsQuery,
    ) -> Result<Vec<UserPlugin>, crate::Error> {
        self.get_current("/userPlugin/items", query).await
    }
}

/// Typed query parameters for `/userPlugin/ldeps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct UserPluginLDependentsQuery {
    #[serde(rename = "masterids")]
    masterids: Vec<super::ids::ProviderEntityId>,
}

impl UserPluginLDependentsQuery {
    /// Returns wire field `masterids`.
    #[must_use]
    pub fn masterids(&self) -> &[super::ids::ProviderEntityId] {
        &self.masterids
    }

    /// Starts a builder for [`UserPluginLDependentsQuery`].
    pub fn builder() -> UserPluginLDependentsQueryBuilder {
        UserPluginLDependentsQueryBuilder::default()
    }
}

/// Builder for [`UserPluginLDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct UserPluginLDependentsQueryBuilder {
    masterids: Option<Vec<super::ids::ProviderEntityId>>,
}

impl UserPluginLDependentsQueryBuilder {
    /// Sets wire field `masterids`.
    pub fn masterids(mut self, value: Vec<super::ids::ProviderEntityId>) -> Self {
        self.masterids = Some(value);
        self
    }

    /// Validates required fields and builds [`UserPluginLDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<UserPluginLDependentsQuery, crate::api::current::BuildError> {
        let masterids = self
            .masterids
            .ok_or(crate::api::current::BuildError::missing("masterids"))?;
        if masterids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "masterids",
                "must not be empty",
            ));
        }
        Ok(UserPluginLDependentsQuery { masterids })
    }
}

impl crate::api::current::support::CurrentQuery for UserPluginLDependentsQuery {
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
    /// Calls the current `GET /userPlugin/ldeps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn user_plugin_l_dependents(
        &self,
        query: &UserPluginLDependentsQuery,
    ) -> Result<Vec<UserPlugin>, crate::Error> {
        self.get_current("/userPlugin/ldeps", query).await
    }
}

/// Typed query parameters for `/userProperty/deps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct UserPropertyDependentsQuery {
    #[serde(rename = "masterid")]
    masterid: super::ids::ProviderEntityId,
}

impl UserPropertyDependentsQuery {
    /// Returns wire field `masterid`.
    #[must_use]
    pub fn masterid(&self) -> &super::ids::ProviderEntityId {
        &self.masterid
    }

    /// Starts a builder for [`UserPropertyDependentsQuery`].
    pub fn builder() -> UserPropertyDependentsQueryBuilder {
        UserPropertyDependentsQueryBuilder::default()
    }
}

/// Builder for [`UserPropertyDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct UserPropertyDependentsQueryBuilder {
    masterid: Option<super::ids::ProviderEntityId>,
}

impl UserPropertyDependentsQueryBuilder {
    /// Sets wire field `masterid`.
    pub fn masterid(mut self, value: super::ids::ProviderEntityId) -> Self {
        self.masterid = Some(value);
        self
    }

    /// Validates required fields and builds [`UserPropertyDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<UserPropertyDependentsQuery, crate::api::current::BuildError> {
        let masterid = self
            .masterid
            .ok_or(crate::api::current::BuildError::missing("masterid"))?;
        Ok(UserPropertyDependentsQuery { masterid })
    }
}

impl crate::api::current::support::CurrentQuery for UserPropertyDependentsQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "masterid", &self.masterid)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /userProperty/deps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn user_property_dependents(
        &self,
        query: &UserPropertyDependentsQuery,
    ) -> Result<Vec<UserProperty>, crate::Error> {
        self.get_current("/userProperty/deps", query).await
    }
}

/// Typed query parameters for `/userProperty/item`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct UserPropertyItemQuery {
    #[serde(rename = "id")]
    id: super::ids::UserPropertyId,
}

impl UserPropertyItemQuery {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> &super::ids::UserPropertyId {
        &self.id
    }

    /// Starts a builder for [`UserPropertyItemQuery`].
    pub fn builder() -> UserPropertyItemQueryBuilder {
        UserPropertyItemQueryBuilder::default()
    }
}

/// Builder for [`UserPropertyItemQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct UserPropertyItemQueryBuilder {
    id: Option<super::ids::UserPropertyId>,
}

impl UserPropertyItemQueryBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::UserPropertyId) -> Self {
        self.id = Some(value);
        self
    }

    /// Validates required fields and builds [`UserPropertyItemQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<UserPropertyItemQuery, crate::api::current::BuildError> {
        let id = self
            .id
            .ok_or(crate::api::current::BuildError::missing("id"))?;
        Ok(UserPropertyItemQuery { id })
    }
}

impl crate::api::current::support::CurrentQuery for UserPropertyItemQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "id", &self.id)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /userProperty/item` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn user_property_item(
        &self,
        query: &UserPropertyItemQuery,
    ) -> Result<UserProperty, crate::Error> {
        self.get_current("/userProperty/item", query).await
    }
}

/// Typed query parameters for `/userProperty/items`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct UserPropertyItemsQuery {
    #[serde(rename = "ids")]
    ids: Vec<super::ids::UserPropertyId>,
}

impl UserPropertyItemsQuery {
    /// Returns wire field `ids`.
    #[must_use]
    pub fn ids(&self) -> &[super::ids::UserPropertyId] {
        &self.ids
    }

    /// Starts a builder for [`UserPropertyItemsQuery`].
    pub fn builder() -> UserPropertyItemsQueryBuilder {
        UserPropertyItemsQueryBuilder::default()
    }
}

/// Builder for [`UserPropertyItemsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct UserPropertyItemsQueryBuilder {
    ids: Option<Vec<super::ids::UserPropertyId>>,
}

impl UserPropertyItemsQueryBuilder {
    /// Sets wire field `ids`.
    pub fn ids(mut self, value: Vec<super::ids::UserPropertyId>) -> Self {
        self.ids = Some(value);
        self
    }

    /// Validates required fields and builds [`UserPropertyItemsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<UserPropertyItemsQuery, crate::api::current::BuildError> {
        let ids = self
            .ids
            .ok_or(crate::api::current::BuildError::missing("ids"))?;
        if ids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "ids",
                "must not be empty",
            ));
        }
        Ok(UserPropertyItemsQuery { ids })
    }
}

impl crate::api::current::support::CurrentQuery for UserPropertyItemsQuery {
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
    /// Calls the current `GET /userProperty/items` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn user_property_items(
        &self,
        query: &UserPropertyItemsQuery,
    ) -> Result<Vec<UserProperty>, crate::Error> {
        self.get_current("/userProperty/items", query).await
    }
}

/// Typed query parameters for `/userProperty/ldeps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct UserPropertyLDependentsQuery {
    #[serde(rename = "masterids")]
    masterids: Vec<super::ids::ProviderEntityId>,
}

impl UserPropertyLDependentsQuery {
    /// Returns wire field `masterids`.
    #[must_use]
    pub fn masterids(&self) -> &[super::ids::ProviderEntityId] {
        &self.masterids
    }

    /// Starts a builder for [`UserPropertyLDependentsQuery`].
    pub fn builder() -> UserPropertyLDependentsQueryBuilder {
        UserPropertyLDependentsQueryBuilder::default()
    }
}

/// Builder for [`UserPropertyLDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct UserPropertyLDependentsQueryBuilder {
    masterids: Option<Vec<super::ids::ProviderEntityId>>,
}

impl UserPropertyLDependentsQueryBuilder {
    /// Sets wire field `masterids`.
    pub fn masterids(mut self, value: Vec<super::ids::ProviderEntityId>) -> Self {
        self.masterids = Some(value);
        self
    }

    /// Validates required fields and builds [`UserPropertyLDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<UserPropertyLDependentsQuery, crate::api::current::BuildError> {
        let masterids = self
            .masterids
            .ok_or(crate::api::current::BuildError::missing("masterids"))?;
        if masterids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "masterids",
                "must not be empty",
            ));
        }
        Ok(UserPropertyLDependentsQuery { masterids })
    }
}

impl crate::api::current::support::CurrentQuery for UserPropertyLDependentsQuery {
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
    /// Calls the current `GET /userProperty/ldeps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn user_property_l_dependents(
        &self,
        query: &UserPropertyLDependentsQuery,
    ) -> Result<Vec<UserProperty>, crate::Error> {
        self.get_current("/userProperty/ldeps", query).await
    }
}

/// Typed query parameters for `/userSession/item`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct UserSessionItemQuery {
    #[serde(rename = "id")]
    id: super::ids::UserSessionId,
}

impl UserSessionItemQuery {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> &super::ids::UserSessionId {
        &self.id
    }

    /// Starts a builder for [`UserSessionItemQuery`].
    pub fn builder() -> UserSessionItemQueryBuilder {
        UserSessionItemQueryBuilder::default()
    }
}

/// Builder for [`UserSessionItemQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct UserSessionItemQueryBuilder {
    id: Option<super::ids::UserSessionId>,
}

impl UserSessionItemQueryBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::UserSessionId) -> Self {
        self.id = Some(value);
        self
    }

    /// Validates required fields and builds [`UserSessionItemQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<UserSessionItemQuery, crate::api::current::BuildError> {
        let id = self
            .id
            .ok_or(crate::api::current::BuildError::missing("id"))?;
        Ok(UserSessionItemQuery { id })
    }
}

impl crate::api::current::support::CurrentQuery for UserSessionItemQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "id", &self.id)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /userSession/item` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn user_session_item(
        &self,
        query: &UserSessionItemQuery,
    ) -> Result<UserSession, crate::Error> {
        self.get_current("/userSession/item", query).await
    }
}

/// Typed query parameters for `/userSession/items`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct UserSessionItemsQuery {
    #[serde(rename = "ids")]
    ids: Vec<super::ids::UserSessionId>,
}

impl UserSessionItemsQuery {
    /// Returns wire field `ids`.
    #[must_use]
    pub fn ids(&self) -> &[super::ids::UserSessionId] {
        &self.ids
    }

    /// Starts a builder for [`UserSessionItemsQuery`].
    pub fn builder() -> UserSessionItemsQueryBuilder {
        UserSessionItemsQueryBuilder::default()
    }
}

/// Builder for [`UserSessionItemsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct UserSessionItemsQueryBuilder {
    ids: Option<Vec<super::ids::UserSessionId>>,
}

impl UserSessionItemsQueryBuilder {
    /// Sets wire field `ids`.
    pub fn ids(mut self, value: Vec<super::ids::UserSessionId>) -> Self {
        self.ids = Some(value);
        self
    }

    /// Validates required fields and builds [`UserSessionItemsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<UserSessionItemsQuery, crate::api::current::BuildError> {
        let ids = self
            .ids
            .ok_or(crate::api::current::BuildError::missing("ids"))?;
        if ids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "ids",
                "must not be empty",
            ));
        }
        Ok(UserSessionItemsQuery { ids })
    }
}

impl crate::api::current::support::CurrentQuery for UserSessionItemsQuery {
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
    /// Calls the current `GET /userSession/items` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn user_session_items(
        &self,
        query: &UserSessionItemsQuery,
    ) -> Result<Vec<UserSession>, crate::Error> {
        self.get_current("/userSession/items", query).await
    }
}

/// Typed query parameters for `/userSessionStats/deps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct UserSessionStatsDependentsQuery {
    #[serde(rename = "masterid")]
    masterid: super::ids::ProviderEntityId,
}

impl UserSessionStatsDependentsQuery {
    /// Returns wire field `masterid`.
    #[must_use]
    pub fn masterid(&self) -> &super::ids::ProviderEntityId {
        &self.masterid
    }

    /// Starts a builder for [`UserSessionStatsDependentsQuery`].
    pub fn builder() -> UserSessionStatsDependentsQueryBuilder {
        UserSessionStatsDependentsQueryBuilder::default()
    }
}

/// Builder for [`UserSessionStatsDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct UserSessionStatsDependentsQueryBuilder {
    masterid: Option<super::ids::ProviderEntityId>,
}

impl UserSessionStatsDependentsQueryBuilder {
    /// Sets wire field `masterid`.
    pub fn masterid(mut self, value: super::ids::ProviderEntityId) -> Self {
        self.masterid = Some(value);
        self
    }

    /// Validates required fields and builds [`UserSessionStatsDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<UserSessionStatsDependentsQuery, crate::api::current::BuildError> {
        let masterid = self
            .masterid
            .ok_or(crate::api::current::BuildError::missing("masterid"))?;
        Ok(UserSessionStatsDependentsQuery { masterid })
    }
}

impl crate::api::current::support::CurrentQuery for UserSessionStatsDependentsQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "masterid", &self.masterid)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /userSessionStats/deps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn user_session_stats_dependents(
        &self,
        query: &UserSessionStatsDependentsQuery,
    ) -> Result<Vec<UserSessionStats>, crate::Error> {
        self.get_current("/userSessionStats/deps", query).await
    }
}

/// Typed query parameters for `/userSessionStats/item`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct UserSessionStatsItemQuery {
    #[serde(rename = "id")]
    id: super::ids::UserSessionStatsId,
}

impl UserSessionStatsItemQuery {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> &super::ids::UserSessionStatsId {
        &self.id
    }

    /// Starts a builder for [`UserSessionStatsItemQuery`].
    pub fn builder() -> UserSessionStatsItemQueryBuilder {
        UserSessionStatsItemQueryBuilder::default()
    }
}

/// Builder for [`UserSessionStatsItemQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct UserSessionStatsItemQueryBuilder {
    id: Option<super::ids::UserSessionStatsId>,
}

impl UserSessionStatsItemQueryBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::UserSessionStatsId) -> Self {
        self.id = Some(value);
        self
    }

    /// Validates required fields and builds [`UserSessionStatsItemQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<UserSessionStatsItemQuery, crate::api::current::BuildError> {
        let id = self
            .id
            .ok_or(crate::api::current::BuildError::missing("id"))?;
        Ok(UserSessionStatsItemQuery { id })
    }
}

impl crate::api::current::support::CurrentQuery for UserSessionStatsItemQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "id", &self.id)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /userSessionStats/item` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn user_session_stats_item(
        &self,
        query: &UserSessionStatsItemQuery,
    ) -> Result<UserSessionStats, crate::Error> {
        self.get_current("/userSessionStats/item", query).await
    }
}

/// Typed query parameters for `/userSessionStats/items`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct UserSessionStatsItemsQuery {
    #[serde(rename = "ids")]
    ids: Vec<super::ids::UserSessionStatsId>,
}

impl UserSessionStatsItemsQuery {
    /// Returns wire field `ids`.
    #[must_use]
    pub fn ids(&self) -> &[super::ids::UserSessionStatsId] {
        &self.ids
    }

    /// Starts a builder for [`UserSessionStatsItemsQuery`].
    pub fn builder() -> UserSessionStatsItemsQueryBuilder {
        UserSessionStatsItemsQueryBuilder::default()
    }
}

/// Builder for [`UserSessionStatsItemsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct UserSessionStatsItemsQueryBuilder {
    ids: Option<Vec<super::ids::UserSessionStatsId>>,
}

impl UserSessionStatsItemsQueryBuilder {
    /// Sets wire field `ids`.
    pub fn ids(mut self, value: Vec<super::ids::UserSessionStatsId>) -> Self {
        self.ids = Some(value);
        self
    }

    /// Validates required fields and builds [`UserSessionStatsItemsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<UserSessionStatsItemsQuery, crate::api::current::BuildError> {
        let ids = self
            .ids
            .ok_or(crate::api::current::BuildError::missing("ids"))?;
        if ids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "ids",
                "must not be empty",
            ));
        }
        Ok(UserSessionStatsItemsQuery { ids })
    }
}

impl crate::api::current::support::CurrentQuery for UserSessionStatsItemsQuery {
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
    /// Calls the current `GET /userSessionStats/items` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn user_session_stats_items(
        &self,
        query: &UserSessionStatsItemsQuery,
    ) -> Result<Vec<UserSessionStats>, crate::Error> {
        self.get_current("/userSessionStats/items", query).await
    }
}

/// Typed query parameters for `/userSessionStats/ldeps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct UserSessionStatsLDependentsQuery {
    #[serde(rename = "masterids")]
    masterids: Vec<super::ids::ProviderEntityId>,
}

impl UserSessionStatsLDependentsQuery {
    /// Returns wire field `masterids`.
    #[must_use]
    pub fn masterids(&self) -> &[super::ids::ProviderEntityId] {
        &self.masterids
    }

    /// Starts a builder for [`UserSessionStatsLDependentsQuery`].
    pub fn builder() -> UserSessionStatsLDependentsQueryBuilder {
        UserSessionStatsLDependentsQueryBuilder::default()
    }
}

/// Builder for [`UserSessionStatsLDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct UserSessionStatsLDependentsQueryBuilder {
    masterids: Option<Vec<super::ids::ProviderEntityId>>,
}

impl UserSessionStatsLDependentsQueryBuilder {
    /// Sets wire field `masterids`.
    pub fn masterids(mut self, value: Vec<super::ids::ProviderEntityId>) -> Self {
        self.masterids = Some(value);
        self
    }

    /// Validates required fields and builds [`UserSessionStatsLDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(
        self,
    ) -> Result<UserSessionStatsLDependentsQuery, crate::api::current::BuildError> {
        let masterids = self
            .masterids
            .ok_or(crate::api::current::BuildError::missing("masterids"))?;
        if masterids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "masterids",
                "must not be empty",
            ));
        }
        Ok(UserSessionStatsLDependentsQuery { masterids })
    }
}

impl crate::api::current::support::CurrentQuery for UserSessionStatsLDependentsQuery {
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
    /// Calls the current `GET /userSessionStats/ldeps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn user_session_stats_l_dependents(
        &self,
        query: &UserSessionStatsLDependentsQuery,
    ) -> Result<Vec<UserSessionStats>, crate::Error> {
        self.get_current("/userSessionStats/ldeps", query).await
    }
}

/// Typed query parameters for `/user/suggest`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct UserSuggestQuery {
    #[serde(rename = "t")]
    t: String,
    #[serde(rename = "l")]
    l: i64,
}

impl UserSuggestQuery {
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

    /// Starts a builder for [`UserSuggestQuery`].
    pub fn builder() -> UserSuggestQueryBuilder {
        UserSuggestQueryBuilder::default()
    }
}

/// Builder for [`UserSuggestQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct UserSuggestQueryBuilder {
    t: Option<String>,
    l: Option<i64>,
}

impl UserSuggestQueryBuilder {
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

    /// Validates required fields and builds [`UserSuggestQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<UserSuggestQuery, crate::api::current::BuildError> {
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
        Ok(UserSuggestQuery { t, l })
    }
}

impl crate::api::current::support::CurrentQuery for UserSuggestQuery {
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
    /// Calls the current `GET /user/suggest` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn user_suggest(&self, query: &UserSuggestQuery) -> Result<Vec<User>, crate::Error> {
        self.get_current("/user/suggest", query).await
    }
}

/// Typed query parameters for `/workspaceTemplate/find`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct WorkspaceTemplateFindQuery {
    #[serde(rename = "name")]
    name: String,
}

impl WorkspaceTemplateFindQuery {
    /// Returns wire field `name`.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Starts a builder for [`WorkspaceTemplateFindQuery`].
    pub fn builder() -> WorkspaceTemplateFindQueryBuilder {
        WorkspaceTemplateFindQueryBuilder::default()
    }
}

/// Builder for [`WorkspaceTemplateFindQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct WorkspaceTemplateFindQueryBuilder {
    name: Option<String>,
}

impl WorkspaceTemplateFindQueryBuilder {
    /// Sets wire field `name`.
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Validates required fields and builds [`WorkspaceTemplateFindQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<WorkspaceTemplateFindQuery, crate::api::current::BuildError> {
        let name = self
            .name
            .ok_or(crate::api::current::BuildError::missing("name"))?;
        if name.is_empty() || name.trim() != name {
            return Err(crate::api::current::BuildError::invalid(
                "name",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        Ok(WorkspaceTemplateFindQuery { name })
    }
}

impl crate::api::current::support::CurrentQuery for WorkspaceTemplateFindQuery {
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
    /// Calls the current `GET /workspaceTemplate/find` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn workspace_template_find(
        &self,
        query: &WorkspaceTemplateFindQuery,
    ) -> Result<WorkspaceTemplate, crate::Error> {
        self.get_current("/workspaceTemplate/find", query).await
    }
}

/// Typed query parameters for `/workspaceTemplate/finds`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct WorkspaceTemplateFindsQuery {
    #[serde(rename = "names")]
    names: Vec<String>,
}

impl WorkspaceTemplateFindsQuery {
    /// Returns wire field `names`.
    #[must_use]
    pub fn names(&self) -> &[String] {
        &self.names
    }

    /// Starts a builder for [`WorkspaceTemplateFindsQuery`].
    pub fn builder() -> WorkspaceTemplateFindsQueryBuilder {
        WorkspaceTemplateFindsQueryBuilder::default()
    }
}

/// Builder for [`WorkspaceTemplateFindsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct WorkspaceTemplateFindsQueryBuilder {
    names: Option<Vec<String>>,
}

impl WorkspaceTemplateFindsQueryBuilder {
    /// Sets wire field `names`.
    pub fn names(mut self, value: Vec<String>) -> Self {
        self.names = Some(value);
        self
    }

    /// Validates required fields and builds [`WorkspaceTemplateFindsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<WorkspaceTemplateFindsQuery, crate::api::current::BuildError> {
        let names = self
            .names
            .ok_or(crate::api::current::BuildError::missing("names"))?;
        if names.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "names",
                "must not be empty",
            ));
        }
        Ok(WorkspaceTemplateFindsQuery { names })
    }
}

impl crate::api::current::support::CurrentQuery for WorkspaceTemplateFindsQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        if self.names.is_empty() {
            return Err(crate::Error::InvalidRequest {
                field: "names",
                reason: "must not be empty",
            });
        }
        let mut pairs = Vec::new();
        for value in &self.names {
            crate::api::current::support::push_query_value(&mut pairs, "names", value)?;
        }
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /workspaceTemplate/finds` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn workspace_template_finds(
        &self,
        query: &WorkspaceTemplateFindsQuery,
    ) -> Result<Vec<WorkspaceTemplate>, crate::Error> {
        self.get_current("/workspaceTemplate/finds", query).await
    }
}

/// Typed query parameters for `/workspaceTemplate/item`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct WorkspaceTemplateItemQuery {
    #[serde(rename = "id")]
    id: super::ids::WorkspaceTemplateId,
}

impl WorkspaceTemplateItemQuery {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> &super::ids::WorkspaceTemplateId {
        &self.id
    }

    /// Starts a builder for [`WorkspaceTemplateItemQuery`].
    pub fn builder() -> WorkspaceTemplateItemQueryBuilder {
        WorkspaceTemplateItemQueryBuilder::default()
    }
}

/// Builder for [`WorkspaceTemplateItemQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct WorkspaceTemplateItemQueryBuilder {
    id: Option<super::ids::WorkspaceTemplateId>,
}

impl WorkspaceTemplateItemQueryBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::WorkspaceTemplateId) -> Self {
        self.id = Some(value);
        self
    }

    /// Validates required fields and builds [`WorkspaceTemplateItemQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<WorkspaceTemplateItemQuery, crate::api::current::BuildError> {
        let id = self
            .id
            .ok_or(crate::api::current::BuildError::missing("id"))?;
        Ok(WorkspaceTemplateItemQuery { id })
    }
}

impl crate::api::current::support::CurrentQuery for WorkspaceTemplateItemQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "id", &self.id)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /workspaceTemplate/item` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn workspace_template_item(
        &self,
        query: &WorkspaceTemplateItemQuery,
    ) -> Result<WorkspaceTemplate, crate::Error> {
        self.get_current("/workspaceTemplate/item", query).await
    }
}

/// Typed query parameters for `/workspaceTemplate/items`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct WorkspaceTemplateItemsQuery {
    #[serde(rename = "ids")]
    ids: Vec<super::ids::WorkspaceTemplateId>,
}

impl WorkspaceTemplateItemsQuery {
    /// Returns wire field `ids`.
    #[must_use]
    pub fn ids(&self) -> &[super::ids::WorkspaceTemplateId] {
        &self.ids
    }

    /// Starts a builder for [`WorkspaceTemplateItemsQuery`].
    pub fn builder() -> WorkspaceTemplateItemsQueryBuilder {
        WorkspaceTemplateItemsQueryBuilder::default()
    }
}

/// Builder for [`WorkspaceTemplateItemsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct WorkspaceTemplateItemsQueryBuilder {
    ids: Option<Vec<super::ids::WorkspaceTemplateId>>,
}

impl WorkspaceTemplateItemsQueryBuilder {
    /// Sets wire field `ids`.
    pub fn ids(mut self, value: Vec<super::ids::WorkspaceTemplateId>) -> Self {
        self.ids = Some(value);
        self
    }

    /// Validates required fields and builds [`WorkspaceTemplateItemsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<WorkspaceTemplateItemsQuery, crate::api::current::BuildError> {
        let ids = self
            .ids
            .ok_or(crate::api::current::BuildError::missing("ids"))?;
        if ids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "ids",
                "must not be empty",
            ));
        }
        Ok(WorkspaceTemplateItemsQuery { ids })
    }
}

impl crate::api::current::support::CurrentQuery for WorkspaceTemplateItemsQuery {
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
    /// Calls the current `GET /workspaceTemplate/items` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn workspace_template_items(
        &self,
        query: &WorkspaceTemplateItemsQuery,
    ) -> Result<Vec<WorkspaceTemplate>, crate::Error> {
        self.get_current("/workspaceTemplate/items", query).await
    }
}

impl crate::Client {
    /// Calls the current `GET /workspaceTemplate/list` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn workspace_template_list(&self) -> Result<Vec<WorkspaceTemplate>, crate::Error> {
        self.get_without_query("/workspaceTemplate/list").await
    }
}

/// Typed query parameters for `/workspaceTemplate/suggest`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct WorkspaceTemplateSuggestQuery {
    #[serde(rename = "t")]
    t: String,
    #[serde(rename = "l")]
    l: i64,
}

impl WorkspaceTemplateSuggestQuery {
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

    /// Starts a builder for [`WorkspaceTemplateSuggestQuery`].
    pub fn builder() -> WorkspaceTemplateSuggestQueryBuilder {
        WorkspaceTemplateSuggestQueryBuilder::default()
    }
}

/// Builder for [`WorkspaceTemplateSuggestQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct WorkspaceTemplateSuggestQueryBuilder {
    t: Option<String>,
    l: Option<i64>,
}

impl WorkspaceTemplateSuggestQueryBuilder {
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

    /// Validates required fields and builds [`WorkspaceTemplateSuggestQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<WorkspaceTemplateSuggestQuery, crate::api::current::BuildError> {
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
        Ok(WorkspaceTemplateSuggestQuery { t, l })
    }
}

impl crate::api::current::support::CurrentQuery for WorkspaceTemplateSuggestQuery {
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
    /// Calls the current `GET /workspaceTemplate/suggest` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn workspace_template_suggest(
        &self,
        query: &WorkspaceTemplateSuggestQuery,
    ) -> Result<Vec<WorkspaceTemplate>, crate::Error> {
        self.get_current("/workspaceTemplate/suggest", query).await
    }
}

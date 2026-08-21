// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary
// @generated
// Generator: tools/generate_openapi.py
// Source: https://partner.tradovate.com/openapi.json (snapshot 2026-08-21, sha256 37caeccf4b0913460a788fcaf4c902497059b8ffe6f6355512e6c08eaacde769)

// Provider wire fields remain schema-auditable even when they repeat
// their type name; wide schema-faithful builders remain one generated
// unit so regeneration and source review cannot drift field subsets.
#![allow(clippy::struct_field_names, clippy::too_many_lines)]

//! Current fee and subscription-plan operations.

/// Current wire model `MarketDataSubscriptionExchangeScope`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct MarketDataSubscriptionExchangeScope {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    id: Option<super::ids::ExchangeScopeId>,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "bundleOf", default, skip_serializing_if = "Option::is_none")]
    bundle_of: Option<String>,
}

impl MarketDataSubscriptionExchangeScope {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> Option<&super::ids::ExchangeScopeId> {
        self.id.as_ref()
    }

    /// Returns wire field `name`.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns wire field `bundleOf`.
    #[must_use]
    pub fn bundle_of(&self) -> Option<&str> {
        self.bundle_of.as_deref()
    }

    /// Starts a builder for [`MarketDataSubscriptionExchangeScope`].
    pub fn builder() -> MarketDataSubscriptionExchangeScopeBuilder {
        MarketDataSubscriptionExchangeScopeBuilder::default()
    }
}

/// Builder for [`MarketDataSubscriptionExchangeScope`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct MarketDataSubscriptionExchangeScopeBuilder {
    id: Option<super::ids::ExchangeScopeId>,
    name: Option<String>,
    bundle_of: Option<String>,
}

impl MarketDataSubscriptionExchangeScopeBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::ExchangeScopeId) -> Self {
        self.id = Some(value);
        self
    }

    /// Sets wire field `name`.
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Sets wire field `bundleOf`.
    pub fn bundle_of(mut self, value: impl Into<String>) -> Self {
        self.bundle_of = Some(value.into());
        self
    }

    /// Validates required fields and builds [`MarketDataSubscriptionExchangeScope`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(
        self,
    ) -> Result<MarketDataSubscriptionExchangeScope, crate::api::current::BuildError> {
        let name = self
            .name
            .ok_or(crate::api::current::BuildError::missing("name"))?;
        Ok(MarketDataSubscriptionExchangeScope {
            id: self.id,
            name,
            bundle_of: self.bundle_of,
        })
    }
}

/// Current wire model `MarketDataSubscriptionPlan`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct MarketDataSubscriptionPlan {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    id: Option<super::ids::MarketDataSubscriptionPlanId>,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "title")]
    title: String,
    #[serde(rename = "price")]
    #[serde(with = "crate::decimal")]
    price: crate::Decimal,
    #[serde(rename = "startDate", default, skip_serializing_if = "Option::is_none")]
    start_date: Option<super::users::TradeDate>,
    #[serde(
        rename = "discontinuedDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    discontinued_date: Option<super::users::TradeDate>,
    #[serde(rename = "exchangeScopeId")]
    exchange_scope_id: super::ids::ExchangeScopeId,
    #[serde(rename = "dataType")]
    data_type: MarketDataSubscriptionPlanDataType,
    #[serde(rename = "professional")]
    professional: MarketDataSubscriptionPlanProfessional,
    #[serde(rename = "tooltip", default, skip_serializing_if = "Option::is_none")]
    tooltip: Option<String>,
}

impl MarketDataSubscriptionPlan {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> Option<&super::ids::MarketDataSubscriptionPlanId> {
        self.id.as_ref()
    }

    /// Returns wire field `name`.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns wire field `title`.
    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns wire field `price`.
    #[must_use]
    pub fn price(&self) -> &crate::Decimal {
        &self.price
    }

    /// Returns wire field `startDate`.
    #[must_use]
    pub fn start_date(&self) -> Option<&super::users::TradeDate> {
        self.start_date.as_ref()
    }

    /// Returns wire field `discontinuedDate`.
    #[must_use]
    pub fn discontinued_date(&self) -> Option<&super::users::TradeDate> {
        self.discontinued_date.as_ref()
    }

    /// Returns wire field `exchangeScopeId`.
    #[must_use]
    pub fn exchange_scope_id(&self) -> &super::ids::ExchangeScopeId {
        &self.exchange_scope_id
    }

    /// Returns wire field `dataType`.
    #[must_use]
    pub fn data_type(&self) -> &MarketDataSubscriptionPlanDataType {
        &self.data_type
    }

    /// Returns wire field `professional`.
    #[must_use]
    pub fn professional(&self) -> &MarketDataSubscriptionPlanProfessional {
        &self.professional
    }

    /// Returns wire field `tooltip`.
    #[must_use]
    pub fn tooltip(&self) -> Option<&str> {
        self.tooltip.as_deref()
    }

    /// Starts a builder for [`MarketDataSubscriptionPlan`].
    pub fn builder() -> MarketDataSubscriptionPlanBuilder {
        MarketDataSubscriptionPlanBuilder::default()
    }
}

/// Builder for [`MarketDataSubscriptionPlan`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct MarketDataSubscriptionPlanBuilder {
    id: Option<super::ids::MarketDataSubscriptionPlanId>,
    name: Option<String>,
    title: Option<String>,
    price: Option<crate::Decimal>,
    start_date: Option<super::users::TradeDate>,
    discontinued_date: Option<super::users::TradeDate>,
    exchange_scope_id: Option<super::ids::ExchangeScopeId>,
    data_type: Option<MarketDataSubscriptionPlanDataType>,
    professional: Option<MarketDataSubscriptionPlanProfessional>,
    tooltip: Option<String>,
}

impl MarketDataSubscriptionPlanBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::MarketDataSubscriptionPlanId) -> Self {
        self.id = Some(value);
        self
    }

    /// Sets wire field `name`.
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Sets wire field `title`.
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Sets wire field `price`.
    pub fn price(mut self, value: crate::Decimal) -> Self {
        self.price = Some(value);
        self
    }

    /// Sets wire field `startDate`.
    pub fn start_date(mut self, value: super::users::TradeDate) -> Self {
        self.start_date = Some(value);
        self
    }

    /// Sets wire field `discontinuedDate`.
    pub fn discontinued_date(mut self, value: super::users::TradeDate) -> Self {
        self.discontinued_date = Some(value);
        self
    }

    /// Sets wire field `exchangeScopeId`.
    pub fn exchange_scope_id(mut self, value: super::ids::ExchangeScopeId) -> Self {
        self.exchange_scope_id = Some(value);
        self
    }

    /// Sets wire field `dataType`.
    pub fn data_type(mut self, value: MarketDataSubscriptionPlanDataType) -> Self {
        self.data_type = Some(value);
        self
    }

    /// Sets wire field `professional`.
    pub fn professional(mut self, value: MarketDataSubscriptionPlanProfessional) -> Self {
        self.professional = Some(value);
        self
    }

    /// Sets wire field `tooltip`.
    pub fn tooltip(mut self, value: impl Into<String>) -> Self {
        self.tooltip = Some(value.into());
        self
    }

    /// Validates required fields and builds [`MarketDataSubscriptionPlan`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<MarketDataSubscriptionPlan, crate::api::current::BuildError> {
        let name = self
            .name
            .ok_or(crate::api::current::BuildError::missing("name"))?;
        let title = self
            .title
            .ok_or(crate::api::current::BuildError::missing("title"))?;
        let price = self
            .price
            .ok_or(crate::api::current::BuildError::missing("price"))?;
        let exchange_scope_id = self
            .exchange_scope_id
            .ok_or(crate::api::current::BuildError::missing("exchangeScopeId"))?;
        let data_type = self
            .data_type
            .ok_or(crate::api::current::BuildError::missing("dataType"))?;
        let professional = self
            .professional
            .ok_or(crate::api::current::BuildError::missing("professional"))?;
        Ok(MarketDataSubscriptionPlan {
            id: self.id,
            name,
            title,
            price,
            start_date: self.start_date,
            discontinued_date: self.discontinued_date,
            exchange_scope_id,
            data_type,
            professional,
            tooltip: self.tooltip,
        })
    }
}

/// Current provider values for `MarketDataSubscriptionPlanDataType`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum MarketDataSubscriptionPlanDataType {
    /// Provider value `DOM`.
    Dom,
    /// Provider value `Top`.
    Top,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl MarketDataSubscriptionPlanDataType {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Dom => "DOM",
            Self::Top => "Top",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for MarketDataSubscriptionPlanDataType {
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

impl<'de> serde::Deserialize<'de> for MarketDataSubscriptionPlanDataType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "DOM" => Self::Dom,
            "Top" => Self::Top,
            _ => Self::Unknown(value),
        })
    }
}

/// Current provider values for `MarketDataSubscriptionPlanProfessional`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum MarketDataSubscriptionPlanProfessional {
    /// Provider value `Either`.
    Either,
    /// Provider value `NonProfessional`.
    NonProfessional,
    /// Provider value `Professional`.
    Professional,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl MarketDataSubscriptionPlanProfessional {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Either => "Either",
            Self::NonProfessional => "NonProfessional",
            Self::Professional => "Professional",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for MarketDataSubscriptionPlanProfessional {
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

impl<'de> serde::Deserialize<'de> for MarketDataSubscriptionPlanProfessional {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "Either" => Self::Either,
            "NonProfessional" => Self::NonProfessional,
            "Professional" => Self::Professional,
            _ => Self::Unknown(value),
        })
    }
}

/// Current wire model `TradovateSubscriptionPlan`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct TradovateSubscriptionPlan {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    id: Option<super::ids::TradovateSubscriptionPlanId>,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "title")]
    title: String,
    #[serde(rename = "price")]
    #[serde(with = "crate::decimal")]
    price: crate::Decimal,
    #[serde(rename = "startDate", default, skip_serializing_if = "Option::is_none")]
    start_date: Option<super::users::TradeDate>,
    #[serde(
        rename = "discontinuedDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    discontinued_date: Option<super::users::TradeDate>,
    #[serde(rename = "category")]
    category: String,
    #[serde(rename = "trial")]
    trial: bool,
    #[serde(rename = "duration")]
    duration: i64,
    #[serde(rename = "durationUnits")]
    duration_units: TradovateSubscriptionPlanDurationUnits,
    #[serde(
        rename = "riskCategoryId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    risk_category_id: Option<super::ids::RiskCategoryId>,
    #[serde(
        rename = "multipleAccounts",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    multiple_accounts: Option<bool>,
    #[serde(
        rename = "organizationId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    organization_id: Option<super::ids::OrganizationId>,
    #[serde(
        rename = "replaySessions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    replay_sessions: Option<i64>,
    #[serde(rename = "footnote", default, skip_serializing_if = "Option::is_none")]
    footnote: Option<String>,
    #[serde(rename = "simOnly", default, skip_serializing_if = "Option::is_none")]
    sim_only: Option<bool>,
}

impl TradovateSubscriptionPlan {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> Option<&super::ids::TradovateSubscriptionPlanId> {
        self.id.as_ref()
    }

    /// Returns wire field `name`.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns wire field `title`.
    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns wire field `price`.
    #[must_use]
    pub fn price(&self) -> &crate::Decimal {
        &self.price
    }

    /// Returns wire field `startDate`.
    #[must_use]
    pub fn start_date(&self) -> Option<&super::users::TradeDate> {
        self.start_date.as_ref()
    }

    /// Returns wire field `discontinuedDate`.
    #[must_use]
    pub fn discontinued_date(&self) -> Option<&super::users::TradeDate> {
        self.discontinued_date.as_ref()
    }

    /// Returns wire field `category`.
    #[must_use]
    pub fn category(&self) -> &str {
        &self.category
    }

    /// Returns wire field `trial`.
    #[must_use]
    pub fn trial(&self) -> &bool {
        &self.trial
    }

    /// Returns wire field `duration`.
    #[must_use]
    pub fn duration(&self) -> &i64 {
        &self.duration
    }

    /// Returns wire field `durationUnits`.
    #[must_use]
    pub fn duration_units(&self) -> &TradovateSubscriptionPlanDurationUnits {
        &self.duration_units
    }

    /// Returns wire field `riskCategoryId`.
    #[must_use]
    pub fn risk_category_id(&self) -> Option<&super::ids::RiskCategoryId> {
        self.risk_category_id.as_ref()
    }

    /// Returns wire field `multipleAccounts`.
    #[must_use]
    pub fn multiple_accounts(&self) -> Option<&bool> {
        self.multiple_accounts.as_ref()
    }

    /// Returns wire field `organizationId`.
    #[must_use]
    pub fn organization_id(&self) -> Option<&super::ids::OrganizationId> {
        self.organization_id.as_ref()
    }

    /// Returns wire field `replaySessions`.
    #[must_use]
    pub fn replay_sessions(&self) -> Option<&i64> {
        self.replay_sessions.as_ref()
    }

    /// Returns wire field `footnote`.
    #[must_use]
    pub fn footnote(&self) -> Option<&str> {
        self.footnote.as_deref()
    }

    /// Returns wire field `simOnly`.
    #[must_use]
    pub fn sim_only(&self) -> Option<&bool> {
        self.sim_only.as_ref()
    }

    /// Starts a builder for [`TradovateSubscriptionPlan`].
    pub fn builder() -> TradovateSubscriptionPlanBuilder {
        TradovateSubscriptionPlanBuilder::default()
    }
}

/// Builder for [`TradovateSubscriptionPlan`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct TradovateSubscriptionPlanBuilder {
    id: Option<super::ids::TradovateSubscriptionPlanId>,
    name: Option<String>,
    title: Option<String>,
    price: Option<crate::Decimal>,
    start_date: Option<super::users::TradeDate>,
    discontinued_date: Option<super::users::TradeDate>,
    category: Option<String>,
    trial: Option<bool>,
    duration: Option<i64>,
    duration_units: Option<TradovateSubscriptionPlanDurationUnits>,
    risk_category_id: Option<super::ids::RiskCategoryId>,
    multiple_accounts: Option<bool>,
    organization_id: Option<super::ids::OrganizationId>,
    replay_sessions: Option<i64>,
    footnote: Option<String>,
    sim_only: Option<bool>,
}

impl TradovateSubscriptionPlanBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::TradovateSubscriptionPlanId) -> Self {
        self.id = Some(value);
        self
    }

    /// Sets wire field `name`.
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Sets wire field `title`.
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Sets wire field `price`.
    pub fn price(mut self, value: crate::Decimal) -> Self {
        self.price = Some(value);
        self
    }

    /// Sets wire field `startDate`.
    pub fn start_date(mut self, value: super::users::TradeDate) -> Self {
        self.start_date = Some(value);
        self
    }

    /// Sets wire field `discontinuedDate`.
    pub fn discontinued_date(mut self, value: super::users::TradeDate) -> Self {
        self.discontinued_date = Some(value);
        self
    }

    /// Sets wire field `category`.
    pub fn category(mut self, value: impl Into<String>) -> Self {
        self.category = Some(value.into());
        self
    }

    /// Sets wire field `trial`.
    pub fn trial(mut self, value: bool) -> Self {
        self.trial = Some(value);
        self
    }

    /// Sets wire field `duration`.
    pub fn duration(mut self, value: i64) -> Self {
        self.duration = Some(value);
        self
    }

    /// Sets wire field `durationUnits`.
    pub fn duration_units(mut self, value: TradovateSubscriptionPlanDurationUnits) -> Self {
        self.duration_units = Some(value);
        self
    }

    /// Sets wire field `riskCategoryId`.
    pub fn risk_category_id(mut self, value: super::ids::RiskCategoryId) -> Self {
        self.risk_category_id = Some(value);
        self
    }

    /// Sets wire field `multipleAccounts`.
    pub fn multiple_accounts(mut self, value: bool) -> Self {
        self.multiple_accounts = Some(value);
        self
    }

    /// Sets wire field `organizationId`.
    pub fn organization_id(mut self, value: super::ids::OrganizationId) -> Self {
        self.organization_id = Some(value);
        self
    }

    /// Sets wire field `replaySessions`.
    pub fn replay_sessions(mut self, value: i64) -> Self {
        self.replay_sessions = Some(value);
        self
    }

    /// Sets wire field `footnote`.
    pub fn footnote(mut self, value: impl Into<String>) -> Self {
        self.footnote = Some(value.into());
        self
    }

    /// Sets wire field `simOnly`.
    pub fn sim_only(mut self, value: bool) -> Self {
        self.sim_only = Some(value);
        self
    }

    /// Validates required fields and builds [`TradovateSubscriptionPlan`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<TradovateSubscriptionPlan, crate::api::current::BuildError> {
        let name = self
            .name
            .ok_or(crate::api::current::BuildError::missing("name"))?;
        let title = self
            .title
            .ok_or(crate::api::current::BuildError::missing("title"))?;
        let price = self
            .price
            .ok_or(crate::api::current::BuildError::missing("price"))?;
        let category = self
            .category
            .ok_or(crate::api::current::BuildError::missing("category"))?;
        let trial = self
            .trial
            .ok_or(crate::api::current::BuildError::missing("trial"))?;
        let duration = self
            .duration
            .ok_or(crate::api::current::BuildError::missing("duration"))?;
        let duration_units = self
            .duration_units
            .ok_or(crate::api::current::BuildError::missing("durationUnits"))?;
        Ok(TradovateSubscriptionPlan {
            id: self.id,
            name,
            title,
            price,
            start_date: self.start_date,
            discontinued_date: self.discontinued_date,
            category,
            trial,
            duration,
            duration_units,
            risk_category_id: self.risk_category_id,
            multiple_accounts: self.multiple_accounts,
            organization_id: self.organization_id,
            replay_sessions: self.replay_sessions,
            footnote: self.footnote,
            sim_only: self.sim_only,
        })
    }
}

/// Current provider values for `TradovateSubscriptionPlanDurationUnits`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum TradovateSubscriptionPlanDurationUnits {
    /// Provider value `Lifetime`.
    Lifetime,
    /// Provider value `Month`.
    Month,
    /// Provider value `Quarter`.
    Quarter,
    /// Provider value `Week`.
    Week,
    /// Provider value `Year`.
    Year,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl TradovateSubscriptionPlanDurationUnits {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Lifetime => "Lifetime",
            Self::Month => "Month",
            Self::Quarter => "Quarter",
            Self::Week => "Week",
            Self::Year => "Year",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for TradovateSubscriptionPlanDurationUnits {
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

impl<'de> serde::Deserialize<'de> for TradovateSubscriptionPlanDurationUnits {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "Lifetime" => Self::Lifetime,
            "Month" => Self::Month,
            "Quarter" => Self::Quarter,
            "Week" => Self::Week,
            "Year" => Self::Year,
            _ => Self::Unknown(value),
        })
    }
}

/// Typed query parameters for `/marketDataSubscriptionExchangeScope/find`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct MarketDataSubscriptionExchangeScopeFindQuery {
    #[serde(rename = "name")]
    name: String,
}

impl MarketDataSubscriptionExchangeScopeFindQuery {
    /// Returns wire field `name`.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Starts a builder for [`MarketDataSubscriptionExchangeScopeFindQuery`].
    pub fn builder() -> MarketDataSubscriptionExchangeScopeFindQueryBuilder {
        MarketDataSubscriptionExchangeScopeFindQueryBuilder::default()
    }
}

/// Builder for [`MarketDataSubscriptionExchangeScopeFindQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct MarketDataSubscriptionExchangeScopeFindQueryBuilder {
    name: Option<String>,
}

impl MarketDataSubscriptionExchangeScopeFindQueryBuilder {
    /// Sets wire field `name`.
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Validates required fields and builds [`MarketDataSubscriptionExchangeScopeFindQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(
        self,
    ) -> Result<MarketDataSubscriptionExchangeScopeFindQuery, crate::api::current::BuildError> {
        let name = self
            .name
            .ok_or(crate::api::current::BuildError::missing("name"))?;
        if name.is_empty() || name.trim() != name {
            return Err(crate::api::current::BuildError::invalid(
                "name",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        Ok(MarketDataSubscriptionExchangeScopeFindQuery { name })
    }
}

impl crate::api::current::support::CurrentQuery for MarketDataSubscriptionExchangeScopeFindQuery {
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
    /// Calls the current `GET /marketDataSubscriptionExchangeScope/find` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn market_data_subscription_exchange_scope_find(
        &self,
        query: &MarketDataSubscriptionExchangeScopeFindQuery,
    ) -> Result<MarketDataSubscriptionExchangeScope, crate::Error> {
        self.get_current("/marketDataSubscriptionExchangeScope/find", query)
            .await
    }
}

/// Typed query parameters for `/marketDataSubscriptionExchangeScope/item`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct MarketDataSubscriptionExchangeScopeItemQuery {
    #[serde(rename = "id")]
    id: super::ids::ExchangeScopeId,
}

impl MarketDataSubscriptionExchangeScopeItemQuery {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> &super::ids::ExchangeScopeId {
        &self.id
    }

    /// Starts a builder for [`MarketDataSubscriptionExchangeScopeItemQuery`].
    pub fn builder() -> MarketDataSubscriptionExchangeScopeItemQueryBuilder {
        MarketDataSubscriptionExchangeScopeItemQueryBuilder::default()
    }
}

/// Builder for [`MarketDataSubscriptionExchangeScopeItemQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct MarketDataSubscriptionExchangeScopeItemQueryBuilder {
    id: Option<super::ids::ExchangeScopeId>,
}

impl MarketDataSubscriptionExchangeScopeItemQueryBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::ExchangeScopeId) -> Self {
        self.id = Some(value);
        self
    }

    /// Validates required fields and builds [`MarketDataSubscriptionExchangeScopeItemQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(
        self,
    ) -> Result<MarketDataSubscriptionExchangeScopeItemQuery, crate::api::current::BuildError> {
        let id = self
            .id
            .ok_or(crate::api::current::BuildError::missing("id"))?;
        Ok(MarketDataSubscriptionExchangeScopeItemQuery { id })
    }
}

impl crate::api::current::support::CurrentQuery for MarketDataSubscriptionExchangeScopeItemQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "id", &self.id)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /marketDataSubscriptionExchangeScope/item` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn market_data_subscription_exchange_scope_item(
        &self,
        query: &MarketDataSubscriptionExchangeScopeItemQuery,
    ) -> Result<MarketDataSubscriptionExchangeScope, crate::Error> {
        self.get_current("/marketDataSubscriptionExchangeScope/item", query)
            .await
    }
}

/// Typed query parameters for `/marketDataSubscriptionExchangeScope/items`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct MarketDataSubscriptionExchangeScopeItemsQuery {
    #[serde(rename = "ids")]
    ids: Vec<super::ids::MarketDataSubscriptionExchangeScopeId>,
}

impl MarketDataSubscriptionExchangeScopeItemsQuery {
    /// Returns wire field `ids`.
    #[must_use]
    pub fn ids(&self) -> &[super::ids::MarketDataSubscriptionExchangeScopeId] {
        &self.ids
    }

    /// Starts a builder for [`MarketDataSubscriptionExchangeScopeItemsQuery`].
    pub fn builder() -> MarketDataSubscriptionExchangeScopeItemsQueryBuilder {
        MarketDataSubscriptionExchangeScopeItemsQueryBuilder::default()
    }
}

/// Builder for [`MarketDataSubscriptionExchangeScopeItemsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct MarketDataSubscriptionExchangeScopeItemsQueryBuilder {
    ids: Option<Vec<super::ids::MarketDataSubscriptionExchangeScopeId>>,
}

impl MarketDataSubscriptionExchangeScopeItemsQueryBuilder {
    /// Sets wire field `ids`.
    pub fn ids(mut self, value: Vec<super::ids::MarketDataSubscriptionExchangeScopeId>) -> Self {
        self.ids = Some(value);
        self
    }

    /// Validates required fields and builds [`MarketDataSubscriptionExchangeScopeItemsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(
        self,
    ) -> Result<MarketDataSubscriptionExchangeScopeItemsQuery, crate::api::current::BuildError>
    {
        let ids = self
            .ids
            .ok_or(crate::api::current::BuildError::missing("ids"))?;
        if ids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "ids",
                "must not be empty",
            ));
        }
        Ok(MarketDataSubscriptionExchangeScopeItemsQuery { ids })
    }
}

impl crate::api::current::support::CurrentQuery for MarketDataSubscriptionExchangeScopeItemsQuery {
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
    /// Calls the current `GET /marketDataSubscriptionExchangeScope/items` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn market_data_subscription_exchange_scope_items(
        &self,
        query: &MarketDataSubscriptionExchangeScopeItemsQuery,
    ) -> Result<Vec<MarketDataSubscriptionExchangeScope>, crate::Error> {
        self.get_current("/marketDataSubscriptionExchangeScope/items", query)
            .await
    }
}

impl crate::Client {
    /// Calls the current `GET /marketDataSubscriptionExchangeScope/list` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn market_data_subscription_exchange_scope_list(
        &self,
    ) -> Result<Vec<MarketDataSubscriptionExchangeScope>, crate::Error> {
        self.get_without_query("/marketDataSubscriptionExchangeScope/list")
            .await
    }
}

/// Typed query parameters for `/marketDataSubscriptionExchangeScope/suggest`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct MarketDataSubscriptionExchangeScopeSuggestQuery {
    #[serde(rename = "t")]
    t: String,
    #[serde(rename = "l")]
    l: i64,
}

impl MarketDataSubscriptionExchangeScopeSuggestQuery {
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

    /// Starts a builder for [`MarketDataSubscriptionExchangeScopeSuggestQuery`].
    pub fn builder() -> MarketDataSubscriptionExchangeScopeSuggestQueryBuilder {
        MarketDataSubscriptionExchangeScopeSuggestQueryBuilder::default()
    }
}

/// Builder for [`MarketDataSubscriptionExchangeScopeSuggestQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct MarketDataSubscriptionExchangeScopeSuggestQueryBuilder {
    t: Option<String>,
    l: Option<i64>,
}

impl MarketDataSubscriptionExchangeScopeSuggestQueryBuilder {
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

    /// Validates required fields and builds [`MarketDataSubscriptionExchangeScopeSuggestQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(
        self,
    ) -> Result<MarketDataSubscriptionExchangeScopeSuggestQuery, crate::api::current::BuildError>
    {
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
        Ok(MarketDataSubscriptionExchangeScopeSuggestQuery { t, l })
    }
}

impl crate::api::current::support::CurrentQuery
    for MarketDataSubscriptionExchangeScopeSuggestQuery
{
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
    /// Calls the current `GET /marketDataSubscriptionExchangeScope/suggest` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn market_data_subscription_exchange_scope_suggest(
        &self,
        query: &MarketDataSubscriptionExchangeScopeSuggestQuery,
    ) -> Result<Vec<MarketDataSubscriptionExchangeScope>, crate::Error> {
        self.get_current("/marketDataSubscriptionExchangeScope/suggest", query)
            .await
    }
}

/// Typed query parameters for `/marketDataSubscriptionPlan/find`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct MarketDataSubscriptionPlanFindQuery {
    #[serde(rename = "name")]
    name: String,
}

impl MarketDataSubscriptionPlanFindQuery {
    /// Returns wire field `name`.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Starts a builder for [`MarketDataSubscriptionPlanFindQuery`].
    pub fn builder() -> MarketDataSubscriptionPlanFindQueryBuilder {
        MarketDataSubscriptionPlanFindQueryBuilder::default()
    }
}

/// Builder for [`MarketDataSubscriptionPlanFindQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct MarketDataSubscriptionPlanFindQueryBuilder {
    name: Option<String>,
}

impl MarketDataSubscriptionPlanFindQueryBuilder {
    /// Sets wire field `name`.
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Validates required fields and builds [`MarketDataSubscriptionPlanFindQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(
        self,
    ) -> Result<MarketDataSubscriptionPlanFindQuery, crate::api::current::BuildError> {
        let name = self
            .name
            .ok_or(crate::api::current::BuildError::missing("name"))?;
        if name.is_empty() || name.trim() != name {
            return Err(crate::api::current::BuildError::invalid(
                "name",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        Ok(MarketDataSubscriptionPlanFindQuery { name })
    }
}

impl crate::api::current::support::CurrentQuery for MarketDataSubscriptionPlanFindQuery {
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
    /// Calls the current `GET /marketDataSubscriptionPlan/find` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn market_data_subscription_plan_find(
        &self,
        query: &MarketDataSubscriptionPlanFindQuery,
    ) -> Result<MarketDataSubscriptionPlan, crate::Error> {
        self.get_current("/marketDataSubscriptionPlan/find", query)
            .await
    }
}

/// Typed query parameters for `/marketDataSubscriptionPlan/item`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct MarketDataSubscriptionPlanItemQuery {
    #[serde(rename = "id")]
    id: super::ids::MarketDataSubscriptionPlanId,
}

impl MarketDataSubscriptionPlanItemQuery {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> &super::ids::MarketDataSubscriptionPlanId {
        &self.id
    }

    /// Starts a builder for [`MarketDataSubscriptionPlanItemQuery`].
    pub fn builder() -> MarketDataSubscriptionPlanItemQueryBuilder {
        MarketDataSubscriptionPlanItemQueryBuilder::default()
    }
}

/// Builder for [`MarketDataSubscriptionPlanItemQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct MarketDataSubscriptionPlanItemQueryBuilder {
    id: Option<super::ids::MarketDataSubscriptionPlanId>,
}

impl MarketDataSubscriptionPlanItemQueryBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::MarketDataSubscriptionPlanId) -> Self {
        self.id = Some(value);
        self
    }

    /// Validates required fields and builds [`MarketDataSubscriptionPlanItemQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(
        self,
    ) -> Result<MarketDataSubscriptionPlanItemQuery, crate::api::current::BuildError> {
        let id = self
            .id
            .ok_or(crate::api::current::BuildError::missing("id"))?;
        Ok(MarketDataSubscriptionPlanItemQuery { id })
    }
}

impl crate::api::current::support::CurrentQuery for MarketDataSubscriptionPlanItemQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "id", &self.id)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /marketDataSubscriptionPlan/item` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn market_data_subscription_plan_item(
        &self,
        query: &MarketDataSubscriptionPlanItemQuery,
    ) -> Result<MarketDataSubscriptionPlan, crate::Error> {
        self.get_current("/marketDataSubscriptionPlan/item", query)
            .await
    }
}

/// Typed query parameters for `/marketDataSubscriptionPlan/items`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct MarketDataSubscriptionPlanItemsQuery {
    #[serde(rename = "ids")]
    ids: Vec<super::ids::MarketDataSubscriptionPlanId>,
}

impl MarketDataSubscriptionPlanItemsQuery {
    /// Returns wire field `ids`.
    #[must_use]
    pub fn ids(&self) -> &[super::ids::MarketDataSubscriptionPlanId] {
        &self.ids
    }

    /// Starts a builder for [`MarketDataSubscriptionPlanItemsQuery`].
    pub fn builder() -> MarketDataSubscriptionPlanItemsQueryBuilder {
        MarketDataSubscriptionPlanItemsQueryBuilder::default()
    }
}

/// Builder for [`MarketDataSubscriptionPlanItemsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct MarketDataSubscriptionPlanItemsQueryBuilder {
    ids: Option<Vec<super::ids::MarketDataSubscriptionPlanId>>,
}

impl MarketDataSubscriptionPlanItemsQueryBuilder {
    /// Sets wire field `ids`.
    pub fn ids(mut self, value: Vec<super::ids::MarketDataSubscriptionPlanId>) -> Self {
        self.ids = Some(value);
        self
    }

    /// Validates required fields and builds [`MarketDataSubscriptionPlanItemsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(
        self,
    ) -> Result<MarketDataSubscriptionPlanItemsQuery, crate::api::current::BuildError> {
        let ids = self
            .ids
            .ok_or(crate::api::current::BuildError::missing("ids"))?;
        if ids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "ids",
                "must not be empty",
            ));
        }
        Ok(MarketDataSubscriptionPlanItemsQuery { ids })
    }
}

impl crate::api::current::support::CurrentQuery for MarketDataSubscriptionPlanItemsQuery {
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
    /// Calls the current `GET /marketDataSubscriptionPlan/items` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn market_data_subscription_plan_items(
        &self,
        query: &MarketDataSubscriptionPlanItemsQuery,
    ) -> Result<Vec<MarketDataSubscriptionPlan>, crate::Error> {
        self.get_current("/marketDataSubscriptionPlan/items", query)
            .await
    }
}

impl crate::Client {
    /// Calls the current `GET /marketDataSubscriptionPlan/list` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn market_data_subscription_plan_list(
        &self,
    ) -> Result<Vec<MarketDataSubscriptionPlan>, crate::Error> {
        self.get_without_query("/marketDataSubscriptionPlan/list")
            .await
    }
}

/// Typed query parameters for `/marketDataSubscriptionPlan/suggest`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct MarketDataSubscriptionPlanSuggestQuery {
    #[serde(rename = "t")]
    t: String,
    #[serde(rename = "l")]
    l: i64,
}

impl MarketDataSubscriptionPlanSuggestQuery {
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

    /// Starts a builder for [`MarketDataSubscriptionPlanSuggestQuery`].
    pub fn builder() -> MarketDataSubscriptionPlanSuggestQueryBuilder {
        MarketDataSubscriptionPlanSuggestQueryBuilder::default()
    }
}

/// Builder for [`MarketDataSubscriptionPlanSuggestQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct MarketDataSubscriptionPlanSuggestQueryBuilder {
    t: Option<String>,
    l: Option<i64>,
}

impl MarketDataSubscriptionPlanSuggestQueryBuilder {
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

    /// Validates required fields and builds [`MarketDataSubscriptionPlanSuggestQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(
        self,
    ) -> Result<MarketDataSubscriptionPlanSuggestQuery, crate::api::current::BuildError> {
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
        Ok(MarketDataSubscriptionPlanSuggestQuery { t, l })
    }
}

impl crate::api::current::support::CurrentQuery for MarketDataSubscriptionPlanSuggestQuery {
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
    /// Calls the current `GET /marketDataSubscriptionPlan/suggest` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn market_data_subscription_plan_suggest(
        &self,
        query: &MarketDataSubscriptionPlanSuggestQuery,
    ) -> Result<Vec<MarketDataSubscriptionPlan>, crate::Error> {
        self.get_current("/marketDataSubscriptionPlan/suggest", query)
            .await
    }
}

/// Typed query parameters for `/tradovateSubscriptionPlan/find`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct TradovateSubscriptionPlanFindQuery {
    #[serde(rename = "name")]
    name: String,
}

impl TradovateSubscriptionPlanFindQuery {
    /// Returns wire field `name`.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Starts a builder for [`TradovateSubscriptionPlanFindQuery`].
    pub fn builder() -> TradovateSubscriptionPlanFindQueryBuilder {
        TradovateSubscriptionPlanFindQueryBuilder::default()
    }
}

/// Builder for [`TradovateSubscriptionPlanFindQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct TradovateSubscriptionPlanFindQueryBuilder {
    name: Option<String>,
}

impl TradovateSubscriptionPlanFindQueryBuilder {
    /// Sets wire field `name`.
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Validates required fields and builds [`TradovateSubscriptionPlanFindQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(
        self,
    ) -> Result<TradovateSubscriptionPlanFindQuery, crate::api::current::BuildError> {
        let name = self
            .name
            .ok_or(crate::api::current::BuildError::missing("name"))?;
        if name.is_empty() || name.trim() != name {
            return Err(crate::api::current::BuildError::invalid(
                "name",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        Ok(TradovateSubscriptionPlanFindQuery { name })
    }
}

impl crate::api::current::support::CurrentQuery for TradovateSubscriptionPlanFindQuery {
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
    /// Calls the current `GET /tradovateSubscriptionPlan/find` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn tradovate_subscription_plan_find(
        &self,
        query: &TradovateSubscriptionPlanFindQuery,
    ) -> Result<TradovateSubscriptionPlan, crate::Error> {
        self.get_current("/tradovateSubscriptionPlan/find", query)
            .await
    }
}

/// Typed query parameters for `/tradovateSubscriptionPlan/item`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct TradovateSubscriptionPlanItemQuery {
    #[serde(rename = "id")]
    id: super::ids::TradovateSubscriptionPlanId,
}

impl TradovateSubscriptionPlanItemQuery {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> &super::ids::TradovateSubscriptionPlanId {
        &self.id
    }

    /// Starts a builder for [`TradovateSubscriptionPlanItemQuery`].
    pub fn builder() -> TradovateSubscriptionPlanItemQueryBuilder {
        TradovateSubscriptionPlanItemQueryBuilder::default()
    }
}

/// Builder for [`TradovateSubscriptionPlanItemQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct TradovateSubscriptionPlanItemQueryBuilder {
    id: Option<super::ids::TradovateSubscriptionPlanId>,
}

impl TradovateSubscriptionPlanItemQueryBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::TradovateSubscriptionPlanId) -> Self {
        self.id = Some(value);
        self
    }

    /// Validates required fields and builds [`TradovateSubscriptionPlanItemQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(
        self,
    ) -> Result<TradovateSubscriptionPlanItemQuery, crate::api::current::BuildError> {
        let id = self
            .id
            .ok_or(crate::api::current::BuildError::missing("id"))?;
        Ok(TradovateSubscriptionPlanItemQuery { id })
    }
}

impl crate::api::current::support::CurrentQuery for TradovateSubscriptionPlanItemQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "id", &self.id)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /tradovateSubscriptionPlan/item` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn tradovate_subscription_plan_item(
        &self,
        query: &TradovateSubscriptionPlanItemQuery,
    ) -> Result<TradovateSubscriptionPlan, crate::Error> {
        self.get_current("/tradovateSubscriptionPlan/item", query)
            .await
    }
}

/// Typed query parameters for `/tradovateSubscriptionPlan/items`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct TradovateSubscriptionPlanItemsQuery {
    #[serde(rename = "ids")]
    ids: Vec<super::ids::TradovateSubscriptionPlanId>,
}

impl TradovateSubscriptionPlanItemsQuery {
    /// Returns wire field `ids`.
    #[must_use]
    pub fn ids(&self) -> &[super::ids::TradovateSubscriptionPlanId] {
        &self.ids
    }

    /// Starts a builder for [`TradovateSubscriptionPlanItemsQuery`].
    pub fn builder() -> TradovateSubscriptionPlanItemsQueryBuilder {
        TradovateSubscriptionPlanItemsQueryBuilder::default()
    }
}

/// Builder for [`TradovateSubscriptionPlanItemsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct TradovateSubscriptionPlanItemsQueryBuilder {
    ids: Option<Vec<super::ids::TradovateSubscriptionPlanId>>,
}

impl TradovateSubscriptionPlanItemsQueryBuilder {
    /// Sets wire field `ids`.
    pub fn ids(mut self, value: Vec<super::ids::TradovateSubscriptionPlanId>) -> Self {
        self.ids = Some(value);
        self
    }

    /// Validates required fields and builds [`TradovateSubscriptionPlanItemsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(
        self,
    ) -> Result<TradovateSubscriptionPlanItemsQuery, crate::api::current::BuildError> {
        let ids = self
            .ids
            .ok_or(crate::api::current::BuildError::missing("ids"))?;
        if ids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "ids",
                "must not be empty",
            ));
        }
        Ok(TradovateSubscriptionPlanItemsQuery { ids })
    }
}

impl crate::api::current::support::CurrentQuery for TradovateSubscriptionPlanItemsQuery {
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
    /// Calls the current `GET /tradovateSubscriptionPlan/items` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn tradovate_subscription_plan_items(
        &self,
        query: &TradovateSubscriptionPlanItemsQuery,
    ) -> Result<Vec<TradovateSubscriptionPlan>, crate::Error> {
        self.get_current("/tradovateSubscriptionPlan/items", query)
            .await
    }
}

impl crate::Client {
    /// Calls the current `GET /tradovateSubscriptionPlan/list` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn tradovate_subscription_plan_list(
        &self,
    ) -> Result<Vec<TradovateSubscriptionPlan>, crate::Error> {
        self.get_without_query("/tradovateSubscriptionPlan/list")
            .await
    }
}

/// Typed query parameters for `/tradovateSubscriptionPlan/suggest`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct TradovateSubscriptionPlanSuggestQuery {
    #[serde(rename = "t")]
    t: String,
    #[serde(rename = "l")]
    l: i64,
}

impl TradovateSubscriptionPlanSuggestQuery {
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

    /// Starts a builder for [`TradovateSubscriptionPlanSuggestQuery`].
    pub fn builder() -> TradovateSubscriptionPlanSuggestQueryBuilder {
        TradovateSubscriptionPlanSuggestQueryBuilder::default()
    }
}

/// Builder for [`TradovateSubscriptionPlanSuggestQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct TradovateSubscriptionPlanSuggestQueryBuilder {
    t: Option<String>,
    l: Option<i64>,
}

impl TradovateSubscriptionPlanSuggestQueryBuilder {
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

    /// Validates required fields and builds [`TradovateSubscriptionPlanSuggestQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(
        self,
    ) -> Result<TradovateSubscriptionPlanSuggestQuery, crate::api::current::BuildError> {
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
        Ok(TradovateSubscriptionPlanSuggestQuery { t, l })
    }
}

impl crate::api::current::support::CurrentQuery for TradovateSubscriptionPlanSuggestQuery {
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
    /// Calls the current `GET /tradovateSubscriptionPlan/suggest` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn tradovate_subscription_plan_suggest(
        &self,
        query: &TradovateSubscriptionPlanSuggestQuery,
    ) -> Result<Vec<TradovateSubscriptionPlan>, crate::Error> {
        self.get_current("/tradovateSubscriptionPlan/suggest", query)
            .await
    }
}

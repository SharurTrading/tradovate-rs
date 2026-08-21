// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary
// @generated
// Generator: tools/generate_openapi.py
// Source: https://partner.tradovate.com/openapi.json (snapshot 2026-08-21, sha256 37caeccf4b0913460a788fcaf4c902497059b8ffe6f6355512e6c08eaacde769)

// Provider wire fields remain schema-auditable even when they repeat
// their type name; wide schema-faithful builders remain one generated
// unit so regeneration and source review cannot drift field subsets.
#![allow(clippy::struct_field_names, clippy::too_many_lines)]

//! Current configuration and entitlement operations.

/// Current wire model `AdminAlert`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct AdminAlert {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    id: Option<super::ids::AdminAlertId>,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "timestamp")]
    timestamp: jiff::Timestamp,
}

impl AdminAlert {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> Option<&super::ids::AdminAlertId> {
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

    /// Starts a builder for [`AdminAlert`].
    pub fn builder() -> AdminAlertBuilder {
        AdminAlertBuilder::default()
    }
}

/// Builder for [`AdminAlert`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct AdminAlertBuilder {
    id: Option<super::ids::AdminAlertId>,
    name: Option<String>,
    timestamp: Option<jiff::Timestamp>,
}

impl AdminAlertBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::AdminAlertId) -> Self {
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

    /// Validates required fields and builds [`AdminAlert`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<AdminAlert, crate::api::current::BuildError> {
        let name = self
            .name
            .ok_or(crate::api::current::BuildError::missing("name"))?;
        let timestamp = self
            .timestamp
            .ok_or(crate::api::current::BuildError::missing("timestamp"))?;
        Ok(AdminAlert {
            id: self.id,
            name,
            timestamp,
        })
    }
}

/// Current wire model `ClearingHouse`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ClearingHouse {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    id: Option<super::ids::ClearingHouseId>,
    #[serde(rename = "name")]
    name: String,
}

impl ClearingHouse {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> Option<&super::ids::ClearingHouseId> {
        self.id.as_ref()
    }

    /// Returns wire field `name`.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Starts a builder for [`ClearingHouse`].
    pub fn builder() -> ClearingHouseBuilder {
        ClearingHouseBuilder::default()
    }
}

/// Builder for [`ClearingHouse`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ClearingHouseBuilder {
    id: Option<super::ids::ClearingHouseId>,
    name: Option<String>,
}

impl ClearingHouseBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::ClearingHouseId) -> Self {
        self.id = Some(value);
        self
    }

    /// Sets wire field `name`.
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Validates required fields and builds [`ClearingHouse`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ClearingHouse, crate::api::current::BuildError> {
        let name = self
            .name
            .ok_or(crate::api::current::BuildError::missing("name"))?;
        Ok(ClearingHouse { id: self.id, name })
    }
}

/// Current wire model `Entitlement`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct Entitlement {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    id: Option<super::ids::EntitlementId>,
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
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "duration", default, skip_serializing_if = "Option::is_none")]
    duration: Option<i64>,
    #[serde(
        rename = "durationUnits",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    duration_units: Option<EntitlementDurationUnits>,
    #[serde(
        rename = "autorenewal",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    autorenewal: Option<bool>,
    #[serde(
        rename = "legalEntityType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    legal_entity_type: Option<EntitlementLegalEntityType>,
    #[serde(
        rename = "requiredNonProCertification",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    required_non_pro_certification: Option<bool>,
    #[serde(
        rename = "isUpgradable",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    is_upgradable: Option<bool>,
    #[serde(
        rename = "freeForFirstTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    free_for_first_time: Option<bool>,
    #[serde(
        rename = "minDaysBetweenUse",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    min_days_between_use: Option<i64>,
    #[serde(
        rename = "purchaseType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    purchase_type: Option<EntitlementPurchaseType>,
    #[serde(
        rename = "rebateContractsThreshold",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    rebate_contracts_threshold: Option<i64>,
    #[serde(rename = "useLimit", default, skip_serializing_if = "Option::is_none")]
    use_limit: Option<i64>,
}

impl Entitlement {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> Option<&super::ids::EntitlementId> {
        self.id.as_ref()
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

    /// Returns wire field `name`.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns wire field `duration`.
    #[must_use]
    pub fn duration(&self) -> Option<&i64> {
        self.duration.as_ref()
    }

    /// Returns wire field `durationUnits`.
    #[must_use]
    pub fn duration_units(&self) -> Option<&EntitlementDurationUnits> {
        self.duration_units.as_ref()
    }

    /// Returns wire field `autorenewal`.
    #[must_use]
    pub fn autorenewal(&self) -> Option<&bool> {
        self.autorenewal.as_ref()
    }

    /// Returns wire field `legalEntityType`.
    #[must_use]
    pub fn legal_entity_type(&self) -> Option<&EntitlementLegalEntityType> {
        self.legal_entity_type.as_ref()
    }

    /// Returns wire field `requiredNonProCertification`.
    #[must_use]
    pub fn required_non_pro_certification(&self) -> Option<&bool> {
        self.required_non_pro_certification.as_ref()
    }

    /// Returns wire field `isUpgradable`.
    #[must_use]
    pub fn is_upgradable(&self) -> Option<&bool> {
        self.is_upgradable.as_ref()
    }

    /// Returns wire field `freeForFirstTime`.
    #[must_use]
    pub fn free_for_first_time(&self) -> Option<&bool> {
        self.free_for_first_time.as_ref()
    }

    /// Returns wire field `minDaysBetweenUse`.
    #[must_use]
    pub fn min_days_between_use(&self) -> Option<&i64> {
        self.min_days_between_use.as_ref()
    }

    /// Returns wire field `purchaseType`.
    #[must_use]
    pub fn purchase_type(&self) -> Option<&EntitlementPurchaseType> {
        self.purchase_type.as_ref()
    }

    /// Returns wire field `rebateContractsThreshold`.
    #[must_use]
    pub fn rebate_contracts_threshold(&self) -> Option<&i64> {
        self.rebate_contracts_threshold.as_ref()
    }

    /// Returns wire field `useLimit`.
    #[must_use]
    pub fn use_limit(&self) -> Option<&i64> {
        self.use_limit.as_ref()
    }

    /// Starts a builder for [`Entitlement`].
    pub fn builder() -> EntitlementBuilder {
        EntitlementBuilder::default()
    }
}

/// Builder for [`Entitlement`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct EntitlementBuilder {
    id: Option<super::ids::EntitlementId>,
    title: Option<String>,
    price: Option<crate::Decimal>,
    start_date: Option<super::users::TradeDate>,
    discontinued_date: Option<super::users::TradeDate>,
    name: Option<String>,
    duration: Option<i64>,
    duration_units: Option<EntitlementDurationUnits>,
    autorenewal: Option<bool>,
    legal_entity_type: Option<EntitlementLegalEntityType>,
    required_non_pro_certification: Option<bool>,
    is_upgradable: Option<bool>,
    free_for_first_time: Option<bool>,
    min_days_between_use: Option<i64>,
    purchase_type: Option<EntitlementPurchaseType>,
    rebate_contracts_threshold: Option<i64>,
    use_limit: Option<i64>,
}

impl EntitlementBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::EntitlementId) -> Self {
        self.id = Some(value);
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

    /// Sets wire field `name`.
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Sets wire field `duration`.
    pub fn duration(mut self, value: i64) -> Self {
        self.duration = Some(value);
        self
    }

    /// Sets wire field `durationUnits`.
    pub fn duration_units(mut self, value: EntitlementDurationUnits) -> Self {
        self.duration_units = Some(value);
        self
    }

    /// Sets wire field `autorenewal`.
    pub fn autorenewal(mut self, value: bool) -> Self {
        self.autorenewal = Some(value);
        self
    }

    /// Sets wire field `legalEntityType`.
    pub fn legal_entity_type(mut self, value: EntitlementLegalEntityType) -> Self {
        self.legal_entity_type = Some(value);
        self
    }

    /// Sets wire field `requiredNonProCertification`.
    pub fn required_non_pro_certification(mut self, value: bool) -> Self {
        self.required_non_pro_certification = Some(value);
        self
    }

    /// Sets wire field `isUpgradable`.
    pub fn is_upgradable(mut self, value: bool) -> Self {
        self.is_upgradable = Some(value);
        self
    }

    /// Sets wire field `freeForFirstTime`.
    pub fn free_for_first_time(mut self, value: bool) -> Self {
        self.free_for_first_time = Some(value);
        self
    }

    /// Sets wire field `minDaysBetweenUse`.
    pub fn min_days_between_use(mut self, value: i64) -> Self {
        self.min_days_between_use = Some(value);
        self
    }

    /// Sets wire field `purchaseType`.
    pub fn purchase_type(mut self, value: EntitlementPurchaseType) -> Self {
        self.purchase_type = Some(value);
        self
    }

    /// Sets wire field `rebateContractsThreshold`.
    pub fn rebate_contracts_threshold(mut self, value: i64) -> Self {
        self.rebate_contracts_threshold = Some(value);
        self
    }

    /// Sets wire field `useLimit`.
    pub fn use_limit(mut self, value: i64) -> Self {
        self.use_limit = Some(value);
        self
    }

    /// Validates required fields and builds [`Entitlement`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<Entitlement, crate::api::current::BuildError> {
        let title = self
            .title
            .ok_or(crate::api::current::BuildError::missing("title"))?;
        let price = self
            .price
            .ok_or(crate::api::current::BuildError::missing("price"))?;
        let name = self
            .name
            .ok_or(crate::api::current::BuildError::missing("name"))?;
        Ok(Entitlement {
            id: self.id,
            title,
            price,
            start_date: self.start_date,
            discontinued_date: self.discontinued_date,
            name,
            duration: self.duration,
            duration_units: self.duration_units,
            autorenewal: self.autorenewal,
            legal_entity_type: self.legal_entity_type,
            required_non_pro_certification: self.required_non_pro_certification,
            is_upgradable: self.is_upgradable,
            free_for_first_time: self.free_for_first_time,
            min_days_between_use: self.min_days_between_use,
            purchase_type: self.purchase_type,
            rebate_contracts_threshold: self.rebate_contracts_threshold,
            use_limit: self.use_limit,
        })
    }
}

/// Current provider values for `EntitlementDurationUnits`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum EntitlementDurationUnits {
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

impl EntitlementDurationUnits {
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

impl serde::Serialize for EntitlementDurationUnits {
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

impl<'de> serde::Deserialize<'de> for EntitlementDurationUnits {
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

/// Current provider values for `EntitlementLegalEntityType`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum EntitlementLegalEntityType {
    /// Provider value `NTCore`.
    NtCore,
    /// Provider value `SimPlusBeta`.
    SimPlusBeta,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl EntitlementLegalEntityType {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::NtCore => "NTCore",
            Self::SimPlusBeta => "SimPlusBeta",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for EntitlementLegalEntityType {
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

impl<'de> serde::Deserialize<'de> for EntitlementLegalEntityType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "NTCore" => Self::NtCore,
            "SimPlusBeta" => Self::SimPlusBeta,
            _ => Self::Unknown(value),
        })
    }
}

/// Current provider values for `EntitlementPurchaseType`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum EntitlementPurchaseType {
    /// Provider value `Any`.
    Any,
    /// Provider value `CreditCard`.
    CreditCard,
    /// Provider value `LiveAccount`.
    LiveAccount,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl EntitlementPurchaseType {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Any => "Any",
            Self::CreditCard => "CreditCard",
            Self::LiveAccount => "LiveAccount",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for EntitlementPurchaseType {
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

impl<'de> serde::Deserialize<'de> for EntitlementPurchaseType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "Any" => Self::Any,
            "CreditCard" => Self::CreditCard,
            "LiveAccount" => Self::LiveAccount,
            _ => Self::Unknown(value),
        })
    }
}

/// Typed query parameters for `/adminAlert/find`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct AdminAlertFindQuery {
    #[serde(rename = "name")]
    name: String,
}

impl AdminAlertFindQuery {
    /// Returns wire field `name`.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Starts a builder for [`AdminAlertFindQuery`].
    pub fn builder() -> AdminAlertFindQueryBuilder {
        AdminAlertFindQueryBuilder::default()
    }
}

/// Builder for [`AdminAlertFindQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct AdminAlertFindQueryBuilder {
    name: Option<String>,
}

impl AdminAlertFindQueryBuilder {
    /// Sets wire field `name`.
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Validates required fields and builds [`AdminAlertFindQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<AdminAlertFindQuery, crate::api::current::BuildError> {
        let name = self
            .name
            .ok_or(crate::api::current::BuildError::missing("name"))?;
        if name.is_empty() || name.trim() != name {
            return Err(crate::api::current::BuildError::invalid(
                "name",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        Ok(AdminAlertFindQuery { name })
    }
}

impl crate::api::current::support::CurrentQuery for AdminAlertFindQuery {
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
    /// Calls the current `GET /adminAlert/find` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn admin_alert_find(
        &self,
        query: &AdminAlertFindQuery,
    ) -> Result<AdminAlert, crate::Error> {
        self.get_current("/adminAlert/find", query).await
    }
}

/// Typed query parameters for `/adminAlert/item`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct AdminAlertItemQuery {
    #[serde(rename = "id")]
    id: super::ids::AdminAlertId,
}

impl AdminAlertItemQuery {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> &super::ids::AdminAlertId {
        &self.id
    }

    /// Starts a builder for [`AdminAlertItemQuery`].
    pub fn builder() -> AdminAlertItemQueryBuilder {
        AdminAlertItemQueryBuilder::default()
    }
}

/// Builder for [`AdminAlertItemQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct AdminAlertItemQueryBuilder {
    id: Option<super::ids::AdminAlertId>,
}

impl AdminAlertItemQueryBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::AdminAlertId) -> Self {
        self.id = Some(value);
        self
    }

    /// Validates required fields and builds [`AdminAlertItemQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<AdminAlertItemQuery, crate::api::current::BuildError> {
        let id = self
            .id
            .ok_or(crate::api::current::BuildError::missing("id"))?;
        Ok(AdminAlertItemQuery { id })
    }
}

impl crate::api::current::support::CurrentQuery for AdminAlertItemQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "id", &self.id)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /adminAlert/item` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn admin_alert_item(
        &self,
        query: &AdminAlertItemQuery,
    ) -> Result<AdminAlert, crate::Error> {
        self.get_current("/adminAlert/item", query).await
    }
}

/// Typed query parameters for `/adminAlert/items`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct AdminAlertItemsQuery {
    #[serde(rename = "ids")]
    ids: Vec<super::ids::AdminAlertId>,
}

impl AdminAlertItemsQuery {
    /// Returns wire field `ids`.
    #[must_use]
    pub fn ids(&self) -> &[super::ids::AdminAlertId] {
        &self.ids
    }

    /// Starts a builder for [`AdminAlertItemsQuery`].
    pub fn builder() -> AdminAlertItemsQueryBuilder {
        AdminAlertItemsQueryBuilder::default()
    }
}

/// Builder for [`AdminAlertItemsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct AdminAlertItemsQueryBuilder {
    ids: Option<Vec<super::ids::AdminAlertId>>,
}

impl AdminAlertItemsQueryBuilder {
    /// Sets wire field `ids`.
    pub fn ids(mut self, value: Vec<super::ids::AdminAlertId>) -> Self {
        self.ids = Some(value);
        self
    }

    /// Validates required fields and builds [`AdminAlertItemsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<AdminAlertItemsQuery, crate::api::current::BuildError> {
        let ids = self
            .ids
            .ok_or(crate::api::current::BuildError::missing("ids"))?;
        if ids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "ids",
                "must not be empty",
            ));
        }
        Ok(AdminAlertItemsQuery { ids })
    }
}

impl crate::api::current::support::CurrentQuery for AdminAlertItemsQuery {
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
    /// Calls the current `GET /adminAlert/items` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn admin_alert_items(
        &self,
        query: &AdminAlertItemsQuery,
    ) -> Result<Vec<AdminAlert>, crate::Error> {
        self.get_current("/adminAlert/items", query).await
    }
}

impl crate::Client {
    /// Calls the current `GET /adminAlert/list` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn admin_alert_list(&self) -> Result<Vec<AdminAlert>, crate::Error> {
        self.get_without_query("/adminAlert/list").await
    }
}

/// Typed query parameters for `/adminAlert/suggest`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct AdminAlertSuggestQuery {
    #[serde(rename = "t")]
    t: String,
    #[serde(rename = "l")]
    l: i64,
}

impl AdminAlertSuggestQuery {
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

    /// Starts a builder for [`AdminAlertSuggestQuery`].
    pub fn builder() -> AdminAlertSuggestQueryBuilder {
        AdminAlertSuggestQueryBuilder::default()
    }
}

/// Builder for [`AdminAlertSuggestQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct AdminAlertSuggestQueryBuilder {
    t: Option<String>,
    l: Option<i64>,
}

impl AdminAlertSuggestQueryBuilder {
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

    /// Validates required fields and builds [`AdminAlertSuggestQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<AdminAlertSuggestQuery, crate::api::current::BuildError> {
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
        Ok(AdminAlertSuggestQuery { t, l })
    }
}

impl crate::api::current::support::CurrentQuery for AdminAlertSuggestQuery {
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
    /// Calls the current `GET /adminAlert/suggest` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn admin_alert_suggest(
        &self,
        query: &AdminAlertSuggestQuery,
    ) -> Result<Vec<AdminAlert>, crate::Error> {
        self.get_current("/adminAlert/suggest", query).await
    }
}

/// Typed query parameters for `/clearingHouse/find`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ClearingHouseFindQuery {
    #[serde(rename = "name")]
    name: String,
}

impl ClearingHouseFindQuery {
    /// Returns wire field `name`.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Starts a builder for [`ClearingHouseFindQuery`].
    pub fn builder() -> ClearingHouseFindQueryBuilder {
        ClearingHouseFindQueryBuilder::default()
    }
}

/// Builder for [`ClearingHouseFindQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ClearingHouseFindQueryBuilder {
    name: Option<String>,
}

impl ClearingHouseFindQueryBuilder {
    /// Sets wire field `name`.
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Validates required fields and builds [`ClearingHouseFindQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ClearingHouseFindQuery, crate::api::current::BuildError> {
        let name = self
            .name
            .ok_or(crate::api::current::BuildError::missing("name"))?;
        if name.is_empty() || name.trim() != name {
            return Err(crate::api::current::BuildError::invalid(
                "name",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        Ok(ClearingHouseFindQuery { name })
    }
}

impl crate::api::current::support::CurrentQuery for ClearingHouseFindQuery {
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
    /// Calls the current `GET /clearingHouse/find` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn clearing_house_find(
        &self,
        query: &ClearingHouseFindQuery,
    ) -> Result<ClearingHouse, crate::Error> {
        self.get_current("/clearingHouse/find", query).await
    }
}

/// Typed query parameters for `/clearingHouse/item`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ClearingHouseItemQuery {
    #[serde(rename = "id")]
    id: super::ids::ClearingHouseId,
}

impl ClearingHouseItemQuery {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> &super::ids::ClearingHouseId {
        &self.id
    }

    /// Starts a builder for [`ClearingHouseItemQuery`].
    pub fn builder() -> ClearingHouseItemQueryBuilder {
        ClearingHouseItemQueryBuilder::default()
    }
}

/// Builder for [`ClearingHouseItemQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ClearingHouseItemQueryBuilder {
    id: Option<super::ids::ClearingHouseId>,
}

impl ClearingHouseItemQueryBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::ClearingHouseId) -> Self {
        self.id = Some(value);
        self
    }

    /// Validates required fields and builds [`ClearingHouseItemQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ClearingHouseItemQuery, crate::api::current::BuildError> {
        let id = self
            .id
            .ok_or(crate::api::current::BuildError::missing("id"))?;
        Ok(ClearingHouseItemQuery { id })
    }
}

impl crate::api::current::support::CurrentQuery for ClearingHouseItemQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "id", &self.id)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /clearingHouse/item` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn clearing_house_item(
        &self,
        query: &ClearingHouseItemQuery,
    ) -> Result<ClearingHouse, crate::Error> {
        self.get_current("/clearingHouse/item", query).await
    }
}

/// Typed query parameters for `/clearingHouse/items`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ClearingHouseItemsQuery {
    #[serde(rename = "ids")]
    ids: Vec<super::ids::ClearingHouseId>,
}

impl ClearingHouseItemsQuery {
    /// Returns wire field `ids`.
    #[must_use]
    pub fn ids(&self) -> &[super::ids::ClearingHouseId] {
        &self.ids
    }

    /// Starts a builder for [`ClearingHouseItemsQuery`].
    pub fn builder() -> ClearingHouseItemsQueryBuilder {
        ClearingHouseItemsQueryBuilder::default()
    }
}

/// Builder for [`ClearingHouseItemsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ClearingHouseItemsQueryBuilder {
    ids: Option<Vec<super::ids::ClearingHouseId>>,
}

impl ClearingHouseItemsQueryBuilder {
    /// Sets wire field `ids`.
    pub fn ids(mut self, value: Vec<super::ids::ClearingHouseId>) -> Self {
        self.ids = Some(value);
        self
    }

    /// Validates required fields and builds [`ClearingHouseItemsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ClearingHouseItemsQuery, crate::api::current::BuildError> {
        let ids = self
            .ids
            .ok_or(crate::api::current::BuildError::missing("ids"))?;
        if ids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "ids",
                "must not be empty",
            ));
        }
        Ok(ClearingHouseItemsQuery { ids })
    }
}

impl crate::api::current::support::CurrentQuery for ClearingHouseItemsQuery {
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
    /// Calls the current `GET /clearingHouse/items` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn clearing_house_items(
        &self,
        query: &ClearingHouseItemsQuery,
    ) -> Result<Vec<ClearingHouse>, crate::Error> {
        self.get_current("/clearingHouse/items", query).await
    }
}

impl crate::Client {
    /// Calls the current `GET /clearingHouse/list` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn clearing_house_list(&self) -> Result<Vec<ClearingHouse>, crate::Error> {
        self.get_without_query("/clearingHouse/list").await
    }
}

/// Typed query parameters for `/clearingHouse/suggest`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ClearingHouseSuggestQuery {
    #[serde(rename = "t")]
    t: String,
    #[serde(rename = "l")]
    l: i64,
}

impl ClearingHouseSuggestQuery {
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

    /// Starts a builder for [`ClearingHouseSuggestQuery`].
    pub fn builder() -> ClearingHouseSuggestQueryBuilder {
        ClearingHouseSuggestQueryBuilder::default()
    }
}

/// Builder for [`ClearingHouseSuggestQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ClearingHouseSuggestQueryBuilder {
    t: Option<String>,
    l: Option<i64>,
}

impl ClearingHouseSuggestQueryBuilder {
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

    /// Validates required fields and builds [`ClearingHouseSuggestQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ClearingHouseSuggestQuery, crate::api::current::BuildError> {
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
        Ok(ClearingHouseSuggestQuery { t, l })
    }
}

impl crate::api::current::support::CurrentQuery for ClearingHouseSuggestQuery {
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
    /// Calls the current `GET /clearingHouse/suggest` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn clearing_house_suggest(
        &self,
        query: &ClearingHouseSuggestQuery,
    ) -> Result<Vec<ClearingHouse>, crate::Error> {
        self.get_current("/clearingHouse/suggest", query).await
    }
}

/// Typed query parameters for `/entitlement/item`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct EntitlementItemQuery {
    #[serde(rename = "id")]
    id: super::ids::EntitlementId,
}

impl EntitlementItemQuery {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> &super::ids::EntitlementId {
        &self.id
    }

    /// Starts a builder for [`EntitlementItemQuery`].
    pub fn builder() -> EntitlementItemQueryBuilder {
        EntitlementItemQueryBuilder::default()
    }
}

/// Builder for [`EntitlementItemQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct EntitlementItemQueryBuilder {
    id: Option<super::ids::EntitlementId>,
}

impl EntitlementItemQueryBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::EntitlementId) -> Self {
        self.id = Some(value);
        self
    }

    /// Validates required fields and builds [`EntitlementItemQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<EntitlementItemQuery, crate::api::current::BuildError> {
        let id = self
            .id
            .ok_or(crate::api::current::BuildError::missing("id"))?;
        Ok(EntitlementItemQuery { id })
    }
}

impl crate::api::current::support::CurrentQuery for EntitlementItemQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "id", &self.id)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /entitlement/item` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn entitlement_item(
        &self,
        query: &EntitlementItemQuery,
    ) -> Result<Entitlement, crate::Error> {
        self.get_current("/entitlement/item", query).await
    }
}

/// Typed query parameters for `/entitlement/items`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct EntitlementItemsQuery {
    #[serde(rename = "ids")]
    ids: Vec<super::ids::EntitlementId>,
}

impl EntitlementItemsQuery {
    /// Returns wire field `ids`.
    #[must_use]
    pub fn ids(&self) -> &[super::ids::EntitlementId] {
        &self.ids
    }

    /// Starts a builder for [`EntitlementItemsQuery`].
    pub fn builder() -> EntitlementItemsQueryBuilder {
        EntitlementItemsQueryBuilder::default()
    }
}

/// Builder for [`EntitlementItemsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct EntitlementItemsQueryBuilder {
    ids: Option<Vec<super::ids::EntitlementId>>,
}

impl EntitlementItemsQueryBuilder {
    /// Sets wire field `ids`.
    pub fn ids(mut self, value: Vec<super::ids::EntitlementId>) -> Self {
        self.ids = Some(value);
        self
    }

    /// Validates required fields and builds [`EntitlementItemsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<EntitlementItemsQuery, crate::api::current::BuildError> {
        let ids = self
            .ids
            .ok_or(crate::api::current::BuildError::missing("ids"))?;
        if ids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "ids",
                "must not be empty",
            ));
        }
        Ok(EntitlementItemsQuery { ids })
    }
}

impl crate::api::current::support::CurrentQuery for EntitlementItemsQuery {
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
    /// Calls the current `GET /entitlement/items` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn entitlement_items(
        &self,
        query: &EntitlementItemsQuery,
    ) -> Result<Vec<Entitlement>, crate::Error> {
        self.get_current("/entitlement/items", query).await
    }
}

impl crate::Client {
    /// Calls the current `GET /entitlement/list` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn entitlement_list(&self) -> Result<Vec<Entitlement>, crate::Error> {
        self.get_without_query("/entitlement/list").await
    }
}

/// Typed query parameters for `/orderStrategyType/find`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct OrderStrategyTypeFindQuery {
    #[serde(rename = "name")]
    name: String,
}

impl OrderStrategyTypeFindQuery {
    /// Returns wire field `name`.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Starts a builder for [`OrderStrategyTypeFindQuery`].
    pub fn builder() -> OrderStrategyTypeFindQueryBuilder {
        OrderStrategyTypeFindQueryBuilder::default()
    }
}

/// Builder for [`OrderStrategyTypeFindQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct OrderStrategyTypeFindQueryBuilder {
    name: Option<String>,
}

impl OrderStrategyTypeFindQueryBuilder {
    /// Sets wire field `name`.
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Validates required fields and builds [`OrderStrategyTypeFindQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<OrderStrategyTypeFindQuery, crate::api::current::BuildError> {
        let name = self
            .name
            .ok_or(crate::api::current::BuildError::missing("name"))?;
        if name.is_empty() || name.trim() != name {
            return Err(crate::api::current::BuildError::invalid(
                "name",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        Ok(OrderStrategyTypeFindQuery { name })
    }
}

impl crate::api::current::support::CurrentQuery for OrderStrategyTypeFindQuery {
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
    /// Calls the current `GET /orderStrategyType/find` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn order_strategy_type_find(
        &self,
        query: &OrderStrategyTypeFindQuery,
    ) -> Result<super::users::OrderStrategyType, crate::Error> {
        self.get_current("/orderStrategyType/find", query).await
    }
}

/// Typed query parameters for `/orderStrategyType/item`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct OrderStrategyTypeItemQuery {
    #[serde(rename = "id")]
    id: super::ids::OrderStrategyTypeId,
}

impl OrderStrategyTypeItemQuery {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> &super::ids::OrderStrategyTypeId {
        &self.id
    }

    /// Starts a builder for [`OrderStrategyTypeItemQuery`].
    pub fn builder() -> OrderStrategyTypeItemQueryBuilder {
        OrderStrategyTypeItemQueryBuilder::default()
    }
}

/// Builder for [`OrderStrategyTypeItemQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct OrderStrategyTypeItemQueryBuilder {
    id: Option<super::ids::OrderStrategyTypeId>,
}

impl OrderStrategyTypeItemQueryBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::OrderStrategyTypeId) -> Self {
        self.id = Some(value);
        self
    }

    /// Validates required fields and builds [`OrderStrategyTypeItemQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<OrderStrategyTypeItemQuery, crate::api::current::BuildError> {
        let id = self
            .id
            .ok_or(crate::api::current::BuildError::missing("id"))?;
        Ok(OrderStrategyTypeItemQuery { id })
    }
}

impl crate::api::current::support::CurrentQuery for OrderStrategyTypeItemQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "id", &self.id)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /orderStrategyType/item` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn order_strategy_type_item(
        &self,
        query: &OrderStrategyTypeItemQuery,
    ) -> Result<super::users::OrderStrategyType, crate::Error> {
        self.get_current("/orderStrategyType/item", query).await
    }
}

/// Typed query parameters for `/orderStrategyType/items`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct OrderStrategyTypeItemsQuery {
    #[serde(rename = "ids")]
    ids: Vec<super::ids::OrderStrategyTypeId>,
}

impl OrderStrategyTypeItemsQuery {
    /// Returns wire field `ids`.
    #[must_use]
    pub fn ids(&self) -> &[super::ids::OrderStrategyTypeId] {
        &self.ids
    }

    /// Starts a builder for [`OrderStrategyTypeItemsQuery`].
    pub fn builder() -> OrderStrategyTypeItemsQueryBuilder {
        OrderStrategyTypeItemsQueryBuilder::default()
    }
}

/// Builder for [`OrderStrategyTypeItemsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct OrderStrategyTypeItemsQueryBuilder {
    ids: Option<Vec<super::ids::OrderStrategyTypeId>>,
}

impl OrderStrategyTypeItemsQueryBuilder {
    /// Sets wire field `ids`.
    pub fn ids(mut self, value: Vec<super::ids::OrderStrategyTypeId>) -> Self {
        self.ids = Some(value);
        self
    }

    /// Validates required fields and builds [`OrderStrategyTypeItemsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<OrderStrategyTypeItemsQuery, crate::api::current::BuildError> {
        let ids = self
            .ids
            .ok_or(crate::api::current::BuildError::missing("ids"))?;
        if ids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "ids",
                "must not be empty",
            ));
        }
        Ok(OrderStrategyTypeItemsQuery { ids })
    }
}

impl crate::api::current::support::CurrentQuery for OrderStrategyTypeItemsQuery {
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
    /// Calls the current `GET /orderStrategyType/items` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn order_strategy_type_items(
        &self,
        query: &OrderStrategyTypeItemsQuery,
    ) -> Result<Vec<super::users::OrderStrategyType>, crate::Error> {
        self.get_current("/orderStrategyType/items", query).await
    }
}

impl crate::Client {
    /// Calls the current `GET /orderStrategyType/list` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn order_strategy_type_list(
        &self,
    ) -> Result<Vec<super::users::OrderStrategyType>, crate::Error> {
        self.get_without_query("/orderStrategyType/list").await
    }
}

/// Typed query parameters for `/orderStrategyType/suggest`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct OrderStrategyTypeSuggestQuery {
    #[serde(rename = "t")]
    t: String,
    #[serde(rename = "l")]
    l: i64,
}

impl OrderStrategyTypeSuggestQuery {
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

    /// Starts a builder for [`OrderStrategyTypeSuggestQuery`].
    pub fn builder() -> OrderStrategyTypeSuggestQueryBuilder {
        OrderStrategyTypeSuggestQueryBuilder::default()
    }
}

/// Builder for [`OrderStrategyTypeSuggestQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct OrderStrategyTypeSuggestQueryBuilder {
    t: Option<String>,
    l: Option<i64>,
}

impl OrderStrategyTypeSuggestQueryBuilder {
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

    /// Validates required fields and builds [`OrderStrategyTypeSuggestQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<OrderStrategyTypeSuggestQuery, crate::api::current::BuildError> {
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
        Ok(OrderStrategyTypeSuggestQuery { t, l })
    }
}

impl crate::api::current::support::CurrentQuery for OrderStrategyTypeSuggestQuery {
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
    /// Calls the current `GET /orderStrategyType/suggest` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn order_strategy_type_suggest(
        &self,
        query: &OrderStrategyTypeSuggestQuery,
    ) -> Result<Vec<super::users::OrderStrategyType>, crate::Error> {
        self.get_current("/orderStrategyType/suggest", query).await
    }
}

/// Typed query parameters for `/property/find`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct PropertyFindQuery {
    #[serde(rename = "name")]
    name: String,
}

impl PropertyFindQuery {
    /// Returns wire field `name`.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Starts a builder for [`PropertyFindQuery`].
    pub fn builder() -> PropertyFindQueryBuilder {
        PropertyFindQueryBuilder::default()
    }
}

/// Builder for [`PropertyFindQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct PropertyFindQueryBuilder {
    name: Option<String>,
}

impl PropertyFindQueryBuilder {
    /// Sets wire field `name`.
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Validates required fields and builds [`PropertyFindQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<PropertyFindQuery, crate::api::current::BuildError> {
        let name = self
            .name
            .ok_or(crate::api::current::BuildError::missing("name"))?;
        if name.is_empty() || name.trim() != name {
            return Err(crate::api::current::BuildError::invalid(
                "name",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        Ok(PropertyFindQuery { name })
    }
}

impl crate::api::current::support::CurrentQuery for PropertyFindQuery {
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
    /// Calls the current `GET /property/find` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn property_find(
        &self,
        query: &PropertyFindQuery,
    ) -> Result<super::users::Property, crate::Error> {
        self.get_current("/property/find", query).await
    }
}

/// Typed query parameters for `/property/item`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct PropertyItemQuery {
    #[serde(rename = "id")]
    id: super::ids::PropertyId,
}

impl PropertyItemQuery {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> &super::ids::PropertyId {
        &self.id
    }

    /// Starts a builder for [`PropertyItemQuery`].
    pub fn builder() -> PropertyItemQueryBuilder {
        PropertyItemQueryBuilder::default()
    }
}

/// Builder for [`PropertyItemQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct PropertyItemQueryBuilder {
    id: Option<super::ids::PropertyId>,
}

impl PropertyItemQueryBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::PropertyId) -> Self {
        self.id = Some(value);
        self
    }

    /// Validates required fields and builds [`PropertyItemQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<PropertyItemQuery, crate::api::current::BuildError> {
        let id = self
            .id
            .ok_or(crate::api::current::BuildError::missing("id"))?;
        Ok(PropertyItemQuery { id })
    }
}

impl crate::api::current::support::CurrentQuery for PropertyItemQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "id", &self.id)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /property/item` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn property_item(
        &self,
        query: &PropertyItemQuery,
    ) -> Result<super::users::Property, crate::Error> {
        self.get_current("/property/item", query).await
    }
}

/// Typed query parameters for `/property/items`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct PropertyItemsQuery {
    #[serde(rename = "ids")]
    ids: Vec<super::ids::PropertyId>,
}

impl PropertyItemsQuery {
    /// Returns wire field `ids`.
    #[must_use]
    pub fn ids(&self) -> &[super::ids::PropertyId] {
        &self.ids
    }

    /// Starts a builder for [`PropertyItemsQuery`].
    pub fn builder() -> PropertyItemsQueryBuilder {
        PropertyItemsQueryBuilder::default()
    }
}

/// Builder for [`PropertyItemsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct PropertyItemsQueryBuilder {
    ids: Option<Vec<super::ids::PropertyId>>,
}

impl PropertyItemsQueryBuilder {
    /// Sets wire field `ids`.
    pub fn ids(mut self, value: Vec<super::ids::PropertyId>) -> Self {
        self.ids = Some(value);
        self
    }

    /// Validates required fields and builds [`PropertyItemsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<PropertyItemsQuery, crate::api::current::BuildError> {
        let ids = self
            .ids
            .ok_or(crate::api::current::BuildError::missing("ids"))?;
        if ids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "ids",
                "must not be empty",
            ));
        }
        Ok(PropertyItemsQuery { ids })
    }
}

impl crate::api::current::support::CurrentQuery for PropertyItemsQuery {
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
    /// Calls the current `GET /property/items` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn property_items(
        &self,
        query: &PropertyItemsQuery,
    ) -> Result<Vec<super::users::Property>, crate::Error> {
        self.get_current("/property/items", query).await
    }
}

impl crate::Client {
    /// Calls the current `GET /property/list` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn property_list(&self) -> Result<Vec<super::users::Property>, crate::Error> {
        self.get_without_query("/property/list").await
    }
}

/// Typed query parameters for `/property/suggest`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct PropertySuggestQuery {
    #[serde(rename = "t")]
    t: String,
    #[serde(rename = "l")]
    l: i64,
}

impl PropertySuggestQuery {
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

    /// Starts a builder for [`PropertySuggestQuery`].
    pub fn builder() -> PropertySuggestQueryBuilder {
        PropertySuggestQueryBuilder::default()
    }
}

/// Builder for [`PropertySuggestQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct PropertySuggestQueryBuilder {
    t: Option<String>,
    l: Option<i64>,
}

impl PropertySuggestQueryBuilder {
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

    /// Validates required fields and builds [`PropertySuggestQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<PropertySuggestQuery, crate::api::current::BuildError> {
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
        Ok(PropertySuggestQuery { t, l })
    }
}

impl crate::api::current::support::CurrentQuery for PropertySuggestQuery {
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
    /// Calls the current `GET /property/suggest` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn property_suggest(
        &self,
        query: &PropertySuggestQuery,
    ) -> Result<Vec<super::users::Property>, crate::Error> {
        self.get_current("/property/suggest", query).await
    }
}

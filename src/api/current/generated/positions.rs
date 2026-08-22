// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0
// @generated
// Generator: tools/generate_openapi.py
// Source: https://partner.tradovate.com/openapi.json (snapshot 2026-08-21, sha256 37caeccf4b0913460a788fcaf4c902497059b8ffe6f6355512e6c08eaacde769)

// Provider wire fields remain schema-auditable even when they repeat
// their type name; wide schema-faithful builders remain one generated
// unit so regeneration and source review cannot drift field subsets.
#![allow(clippy::struct_field_names, clippy::too_many_lines)]

//! Current position and fill-pair operations.

/// Typed query parameters for `/fillPair/deps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct FillPairDependentsQuery {
    #[serde(rename = "masterid")]
    masterid: super::ids::ProviderEntityId,
}

impl FillPairDependentsQuery {
    /// Returns wire field `masterid`.
    #[must_use]
    pub fn masterid(&self) -> &super::ids::ProviderEntityId {
        &self.masterid
    }

    /// Starts a builder for [`FillPairDependentsQuery`].
    pub fn builder() -> FillPairDependentsQueryBuilder {
        FillPairDependentsQueryBuilder::default()
    }
}

/// Builder for [`FillPairDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct FillPairDependentsQueryBuilder {
    masterid: Option<super::ids::ProviderEntityId>,
}

impl FillPairDependentsQueryBuilder {
    /// Sets wire field `masterid`.
    pub fn masterid(mut self, value: super::ids::ProviderEntityId) -> Self {
        self.masterid = Some(value);
        self
    }

    /// Validates required fields and builds [`FillPairDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<FillPairDependentsQuery, crate::api::current::BuildError> {
        let masterid = self
            .masterid
            .ok_or(crate::api::current::BuildError::missing("masterid"))?;
        Ok(FillPairDependentsQuery { masterid })
    }
}

impl crate::api::current::support::CurrentQuery for FillPairDependentsQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "masterid", &self.masterid)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /fillPair/deps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn fill_pair_dependents(
        &self,
        query: &FillPairDependentsQuery,
    ) -> Result<Vec<super::users::FillPair>, crate::Error> {
        self.get_current("/fillPair/deps", query).await
    }
}

/// Typed query parameters for `/fillPair/item`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct FillPairItemQuery {
    #[serde(rename = "id")]
    id: super::ids::FillPairId,
}

impl FillPairItemQuery {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> &super::ids::FillPairId {
        &self.id
    }

    /// Starts a builder for [`FillPairItemQuery`].
    pub fn builder() -> FillPairItemQueryBuilder {
        FillPairItemQueryBuilder::default()
    }
}

/// Builder for [`FillPairItemQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct FillPairItemQueryBuilder {
    id: Option<super::ids::FillPairId>,
}

impl FillPairItemQueryBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::FillPairId) -> Self {
        self.id = Some(value);
        self
    }

    /// Validates required fields and builds [`FillPairItemQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<FillPairItemQuery, crate::api::current::BuildError> {
        let id = self
            .id
            .ok_or(crate::api::current::BuildError::missing("id"))?;
        Ok(FillPairItemQuery { id })
    }
}

impl crate::api::current::support::CurrentQuery for FillPairItemQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "id", &self.id)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /fillPair/item` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn fill_pair_item(
        &self,
        query: &FillPairItemQuery,
    ) -> Result<super::users::FillPair, crate::Error> {
        self.get_current("/fillPair/item", query).await
    }
}

/// Typed query parameters for `/fillPair/items`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct FillPairItemsQuery {
    #[serde(rename = "ids")]
    ids: Vec<super::ids::FillPairId>,
}

impl FillPairItemsQuery {
    /// Returns wire field `ids`.
    #[must_use]
    pub fn ids(&self) -> &[super::ids::FillPairId] {
        &self.ids
    }

    /// Starts a builder for [`FillPairItemsQuery`].
    pub fn builder() -> FillPairItemsQueryBuilder {
        FillPairItemsQueryBuilder::default()
    }
}

/// Builder for [`FillPairItemsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct FillPairItemsQueryBuilder {
    ids: Option<Vec<super::ids::FillPairId>>,
}

impl FillPairItemsQueryBuilder {
    /// Sets wire field `ids`.
    pub fn ids(mut self, value: Vec<super::ids::FillPairId>) -> Self {
        self.ids = Some(value);
        self
    }

    /// Validates required fields and builds [`FillPairItemsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<FillPairItemsQuery, crate::api::current::BuildError> {
        let ids = self
            .ids
            .ok_or(crate::api::current::BuildError::missing("ids"))?;
        if ids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "ids",
                "must not be empty",
            ));
        }
        Ok(FillPairItemsQuery { ids })
    }
}

impl crate::api::current::support::CurrentQuery for FillPairItemsQuery {
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
    /// Calls the current `GET /fillPair/items` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn fill_pair_items(
        &self,
        query: &FillPairItemsQuery,
    ) -> Result<Vec<super::users::FillPair>, crate::Error> {
        self.get_current("/fillPair/items", query).await
    }
}

/// Typed query parameters for `/fillPair/ldeps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct FillPairLDependentsQuery {
    #[serde(rename = "masterids")]
    masterids: Vec<super::ids::ProviderEntityId>,
}

impl FillPairLDependentsQuery {
    /// Returns wire field `masterids`.
    #[must_use]
    pub fn masterids(&self) -> &[super::ids::ProviderEntityId] {
        &self.masterids
    }

    /// Starts a builder for [`FillPairLDependentsQuery`].
    pub fn builder() -> FillPairLDependentsQueryBuilder {
        FillPairLDependentsQueryBuilder::default()
    }
}

/// Builder for [`FillPairLDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct FillPairLDependentsQueryBuilder {
    masterids: Option<Vec<super::ids::ProviderEntityId>>,
}

impl FillPairLDependentsQueryBuilder {
    /// Sets wire field `masterids`.
    pub fn masterids(mut self, value: Vec<super::ids::ProviderEntityId>) -> Self {
        self.masterids = Some(value);
        self
    }

    /// Validates required fields and builds [`FillPairLDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<FillPairLDependentsQuery, crate::api::current::BuildError> {
        let masterids = self
            .masterids
            .ok_or(crate::api::current::BuildError::missing("masterids"))?;
        if masterids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "masterids",
                "must not be empty",
            ));
        }
        Ok(FillPairLDependentsQuery { masterids })
    }
}

impl crate::api::current::support::CurrentQuery for FillPairLDependentsQuery {
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
    /// Calls the current `GET /fillPair/ldeps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn fill_pair_l_dependents(
        &self,
        query: &FillPairLDependentsQuery,
    ) -> Result<Vec<super::users::FillPair>, crate::Error> {
        self.get_current("/fillPair/ldeps", query).await
    }
}

impl crate::Client {
    /// Calls the current `GET /fillPair/list` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn fill_pair_list(&self) -> Result<Vec<super::users::FillPair>, crate::Error> {
        self.get_without_query("/fillPair/list").await
    }
}

/// Typed query parameters for `/position/deps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct PositionDependentsQuery {
    #[serde(rename = "masterid")]
    masterid: super::ids::ProviderEntityId,
}

impl PositionDependentsQuery {
    /// Returns wire field `masterid`.
    #[must_use]
    pub fn masterid(&self) -> &super::ids::ProviderEntityId {
        &self.masterid
    }

    /// Starts a builder for [`PositionDependentsQuery`].
    pub fn builder() -> PositionDependentsQueryBuilder {
        PositionDependentsQueryBuilder::default()
    }
}

/// Builder for [`PositionDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct PositionDependentsQueryBuilder {
    masterid: Option<super::ids::ProviderEntityId>,
}

impl PositionDependentsQueryBuilder {
    /// Sets wire field `masterid`.
    pub fn masterid(mut self, value: super::ids::ProviderEntityId) -> Self {
        self.masterid = Some(value);
        self
    }

    /// Validates required fields and builds [`PositionDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<PositionDependentsQuery, crate::api::current::BuildError> {
        let masterid = self
            .masterid
            .ok_or(crate::api::current::BuildError::missing("masterid"))?;
        Ok(PositionDependentsQuery { masterid })
    }
}

impl crate::api::current::support::CurrentQuery for PositionDependentsQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "masterid", &self.masterid)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /position/deps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn position_dependents(
        &self,
        query: &PositionDependentsQuery,
    ) -> Result<Vec<super::users::Position>, crate::Error> {
        self.get_current("/position/deps", query).await
    }
}

/// Typed query parameters for `/position/find`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct PositionFindQuery {
    #[serde(rename = "name")]
    name: String,
}

impl PositionFindQuery {
    /// Returns wire field `name`.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Starts a builder for [`PositionFindQuery`].
    pub fn builder() -> PositionFindQueryBuilder {
        PositionFindQueryBuilder::default()
    }
}

/// Builder for [`PositionFindQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct PositionFindQueryBuilder {
    name: Option<String>,
}

impl PositionFindQueryBuilder {
    /// Sets wire field `name`.
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Validates required fields and builds [`PositionFindQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<PositionFindQuery, crate::api::current::BuildError> {
        let name = self
            .name
            .ok_or(crate::api::current::BuildError::missing("name"))?;
        if name.is_empty() || name.trim() != name {
            return Err(crate::api::current::BuildError::invalid(
                "name",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        Ok(PositionFindQuery { name })
    }
}

impl crate::api::current::support::CurrentQuery for PositionFindQuery {
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
    /// Calls the current `GET /position/find` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn position_find(
        &self,
        query: &PositionFindQuery,
    ) -> Result<super::users::Position, crate::Error> {
        self.get_current("/position/find", query).await
    }
}

/// Typed query parameters for `/position/item`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct PositionItemQuery {
    #[serde(rename = "id")]
    id: crate::PositionId,
}

impl PositionItemQuery {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> &crate::PositionId {
        &self.id
    }

    /// Starts a builder for [`PositionItemQuery`].
    pub fn builder() -> PositionItemQueryBuilder {
        PositionItemQueryBuilder::default()
    }
}

/// Builder for [`PositionItemQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct PositionItemQueryBuilder {
    id: Option<crate::PositionId>,
}

impl PositionItemQueryBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: crate::PositionId) -> Self {
        self.id = Some(value);
        self
    }

    /// Validates required fields and builds [`PositionItemQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<PositionItemQuery, crate::api::current::BuildError> {
        let id = self
            .id
            .ok_or(crate::api::current::BuildError::missing("id"))?;
        Ok(PositionItemQuery { id })
    }
}

impl crate::api::current::support::CurrentQuery for PositionItemQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "id", &self.id)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /position/item` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn position_item(
        &self,
        query: &PositionItemQuery,
    ) -> Result<super::users::Position, crate::Error> {
        self.get_current("/position/item", query).await
    }
}

/// Typed query parameters for `/position/items`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct PositionItemsQuery {
    #[serde(rename = "ids")]
    ids: Vec<crate::PositionId>,
}

impl PositionItemsQuery {
    /// Returns wire field `ids`.
    #[must_use]
    pub fn ids(&self) -> &[crate::PositionId] {
        &self.ids
    }

    /// Starts a builder for [`PositionItemsQuery`].
    pub fn builder() -> PositionItemsQueryBuilder {
        PositionItemsQueryBuilder::default()
    }
}

/// Builder for [`PositionItemsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct PositionItemsQueryBuilder {
    ids: Option<Vec<crate::PositionId>>,
}

impl PositionItemsQueryBuilder {
    /// Sets wire field `ids`.
    pub fn ids(mut self, value: Vec<crate::PositionId>) -> Self {
        self.ids = Some(value);
        self
    }

    /// Validates required fields and builds [`PositionItemsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<PositionItemsQuery, crate::api::current::BuildError> {
        let ids = self
            .ids
            .ok_or(crate::api::current::BuildError::missing("ids"))?;
        if ids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "ids",
                "must not be empty",
            ));
        }
        Ok(PositionItemsQuery { ids })
    }
}

impl crate::api::current::support::CurrentQuery for PositionItemsQuery {
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
    /// Calls the current `GET /position/items` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn position_items(
        &self,
        query: &PositionItemsQuery,
    ) -> Result<Vec<super::users::Position>, crate::Error> {
        self.get_current("/position/items", query).await
    }
}

/// Typed query parameters for `/position/ldeps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct PositionLDependentsQuery {
    #[serde(rename = "masterids")]
    masterids: Vec<super::ids::ProviderEntityId>,
}

impl PositionLDependentsQuery {
    /// Returns wire field `masterids`.
    #[must_use]
    pub fn masterids(&self) -> &[super::ids::ProviderEntityId] {
        &self.masterids
    }

    /// Starts a builder for [`PositionLDependentsQuery`].
    pub fn builder() -> PositionLDependentsQueryBuilder {
        PositionLDependentsQueryBuilder::default()
    }
}

/// Builder for [`PositionLDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct PositionLDependentsQueryBuilder {
    masterids: Option<Vec<super::ids::ProviderEntityId>>,
}

impl PositionLDependentsQueryBuilder {
    /// Sets wire field `masterids`.
    pub fn masterids(mut self, value: Vec<super::ids::ProviderEntityId>) -> Self {
        self.masterids = Some(value);
        self
    }

    /// Validates required fields and builds [`PositionLDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<PositionLDependentsQuery, crate::api::current::BuildError> {
        let masterids = self
            .masterids
            .ok_or(crate::api::current::BuildError::missing("masterids"))?;
        if masterids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "masterids",
                "must not be empty",
            ));
        }
        Ok(PositionLDependentsQuery { masterids })
    }
}

impl crate::api::current::support::CurrentQuery for PositionLDependentsQuery {
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
    /// Calls the current `GET /position/ldeps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn position_l_dependents(
        &self,
        query: &PositionLDependentsQuery,
    ) -> Result<Vec<super::users::Position>, crate::Error> {
        self.get_current("/position/ldeps", query).await
    }
}

impl crate::Client {
    /// Calls the current `GET /position/list` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn position_list(&self) -> Result<Vec<super::users::Position>, crate::Error> {
        self.get_without_query("/position/list").await
    }
}

// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary
// @generated
// Generator: tools/generate_openapi.py
// Source: https://partner.tradovate.com/openapi.json (snapshot 2026-08-21, sha256 37caeccf4b0913460a788fcaf4c902497059b8ffe6f6355512e6c08eaacde769)

// Provider wire fields remain schema-auditable even when they repeat
// their type name; wide schema-faithful builders remain one generated
// unit so regeneration and source review cannot drift field subsets.
#![allow(clippy::struct_field_names, clippy::too_many_lines)]

//! Current contract-library operations and wire models.

/// Current wire model `CurrencyRate`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct CurrencyRate {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    id: Option<super::ids::CurrencyRateId>,
    #[serde(rename = "timestamp")]
    timestamp: jiff::Timestamp,
    #[serde(rename = "rate")]
    #[serde(with = "crate::decimal")]
    rate: crate::Decimal,
}

impl CurrencyRate {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> Option<&super::ids::CurrencyRateId> {
        self.id.as_ref()
    }

    /// Returns wire field `timestamp`.
    #[must_use]
    pub fn timestamp(&self) -> &jiff::Timestamp {
        &self.timestamp
    }

    /// Returns wire field `rate`.
    #[must_use]
    pub fn rate(&self) -> &crate::Decimal {
        &self.rate
    }

    /// Starts a builder for [`CurrencyRate`].
    pub fn builder() -> CurrencyRateBuilder {
        CurrencyRateBuilder::default()
    }
}

/// Builder for [`CurrencyRate`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct CurrencyRateBuilder {
    id: Option<super::ids::CurrencyRateId>,
    timestamp: Option<jiff::Timestamp>,
    rate: Option<crate::Decimal>,
}

impl CurrencyRateBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::CurrencyRateId) -> Self {
        self.id = Some(value);
        self
    }

    /// Sets wire field `timestamp`.
    pub fn timestamp(mut self, value: jiff::Timestamp) -> Self {
        self.timestamp = Some(value);
        self
    }

    /// Sets wire field `rate`.
    pub fn rate(mut self, value: crate::Decimal) -> Self {
        self.rate = Some(value);
        self
    }

    /// Validates required fields and builds [`CurrencyRate`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<CurrencyRate, crate::api::current::BuildError> {
        let timestamp = self
            .timestamp
            .ok_or(crate::api::current::BuildError::missing("timestamp"))?;
        let rate = self
            .rate
            .ok_or(crate::api::current::BuildError::missing("rate"))?;
        Ok(CurrencyRate {
            id: self.id,
            timestamp,
            rate,
        })
    }
}

/// Current wire model `GetProductFeeParams`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct GetProductFeeParams {
    #[serde(rename = "productIds")]
    product_ids: Vec<super::ids::ProductId>,
}

impl GetProductFeeParams {
    /// Returns wire field `productIds`.
    #[must_use]
    pub fn product_ids(&self) -> &[super::ids::ProductId] {
        &self.product_ids
    }

    /// Starts a builder for [`GetProductFeeParams`].
    pub fn builder() -> GetProductFeeParamsBuilder {
        GetProductFeeParamsBuilder::default()
    }
}

/// Builder for [`GetProductFeeParams`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct GetProductFeeParamsBuilder {
    product_ids: Option<Vec<super::ids::ProductId>>,
}

impl GetProductFeeParamsBuilder {
    /// Sets wire field `productIds`.
    pub fn product_ids(mut self, value: Vec<super::ids::ProductId>) -> Self {
        self.product_ids = Some(value);
        self
    }

    /// Validates required fields and builds [`GetProductFeeParams`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<GetProductFeeParams, crate::api::current::BuildError> {
        let product_ids = self
            .product_ids
            .ok_or(crate::api::current::BuildError::missing("productIds"))?;
        if product_ids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "productIds",
                "must not be empty",
            ));
        }
        Ok(GetProductFeeParams { product_ids })
    }
}

impl crate::api::current::support::CurrentRequest for GetProductFeeParams {
    fn validate_current(&self) -> Result<(), crate::Error> {
        if self.product_ids.is_empty() {
            return Err(crate::Error::InvalidRequest {
                field: "productIds",
                reason: "must not be empty",
            });
        }
        Ok(())
    }
}

/// Current wire model `ProductFeeParams`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ProductFeeParams {
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
    #[serde(rename = "productId")]
    product_id: super::ids::ProductId,
    #[serde(rename = "dayMargin", default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "crate::decimal::option")]
    day_margin: Option<crate::Decimal>,
    #[serde(
        rename = "nightMargin",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    night_margin: Option<crate::Decimal>,
    #[serde(
        rename = "fullMargin",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    full_margin: Option<super::risks::ProductMargin>,
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

impl ProductFeeParams {
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

    /// Returns wire field `productId`.
    #[must_use]
    pub fn product_id(&self) -> &super::ids::ProductId {
        &self.product_id
    }

    /// Returns wire field `dayMargin`.
    #[must_use]
    pub fn day_margin(&self) -> Option<&crate::Decimal> {
        self.day_margin.as_ref()
    }

    /// Returns wire field `nightMargin`.
    #[must_use]
    pub fn night_margin(&self) -> Option<&crate::Decimal> {
        self.night_margin.as_ref()
    }

    /// Returns wire field `fullMargin`.
    #[must_use]
    pub fn full_margin(&self) -> Option<&super::risks::ProductMargin> {
        self.full_margin.as_ref()
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

    /// Starts a builder for [`ProductFeeParams`].
    pub fn builder() -> ProductFeeParamsBuilder {
        ProductFeeParamsBuilder::default()
    }
}

/// Builder for [`ProductFeeParams`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ProductFeeParamsBuilder {
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
    product_id: Option<super::ids::ProductId>,
    day_margin: Option<crate::Decimal>,
    night_margin: Option<crate::Decimal>,
    full_margin: Option<super::risks::ProductMargin>,
    commission_notional_value_bps: Option<crate::Decimal>,
    exchange_fee_notional_value_bps: Option<crate::Decimal>,
}

impl ProductFeeParamsBuilder {
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

    /// Sets wire field `productId`.
    pub fn product_id(mut self, value: super::ids::ProductId) -> Self {
        self.product_id = Some(value);
        self
    }

    /// Sets wire field `dayMargin`.
    pub fn day_margin(mut self, value: crate::Decimal) -> Self {
        self.day_margin = Some(value);
        self
    }

    /// Sets wire field `nightMargin`.
    pub fn night_margin(mut self, value: crate::Decimal) -> Self {
        self.night_margin = Some(value);
        self
    }

    /// Sets wire field `fullMargin`.
    pub fn full_margin(mut self, value: super::risks::ProductMargin) -> Self {
        self.full_margin = Some(value);
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

    /// Validates required fields and builds [`ProductFeeParams`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ProductFeeParams, crate::api::current::BuildError> {
        let product_id = self
            .product_id
            .ok_or(crate::api::current::BuildError::missing("productId"))?;
        Ok(ProductFeeParams {
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
            product_id,
            day_margin: self.day_margin,
            night_margin: self.night_margin,
            full_margin: self.full_margin,
            commission_notional_value_bps: self.commission_notional_value_bps,
            exchange_fee_notional_value_bps: self.exchange_fee_notional_value_bps,
        })
    }
}

/// Current wire model `ProductFeeParamsResponse`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ProductFeeParamsResponse {
    #[serde(rename = "params")]
    params: Vec<ProductFeeParams>,
}

impl ProductFeeParamsResponse {
    /// Returns wire field `params`.
    #[must_use]
    pub fn params(&self) -> &[ProductFeeParams] {
        &self.params
    }

    /// Starts a builder for [`ProductFeeParamsResponse`].
    pub fn builder() -> ProductFeeParamsResponseBuilder {
        ProductFeeParamsResponseBuilder::default()
    }
}

/// Builder for [`ProductFeeParamsResponse`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ProductFeeParamsResponseBuilder {
    params: Option<Vec<ProductFeeParams>>,
}

impl ProductFeeParamsResponseBuilder {
    /// Sets wire field `params`.
    pub fn params(mut self, value: Vec<ProductFeeParams>) -> Self {
        self.params = Some(value);
        self
    }

    /// Validates required fields and builds [`ProductFeeParamsResponse`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ProductFeeParamsResponse, crate::api::current::BuildError> {
        let params = self
            .params
            .ok_or(crate::api::current::BuildError::missing("params"))?;
        Ok(ProductFeeParamsResponse { params })
    }
}

/// Current wire model `ProductSession`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ProductSession {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    id: Option<super::ids::ProductSessionId>,
    #[serde(rename = "openTime")]
    open_time: TradeTime,
    #[serde(rename = "startTime")]
    start_time: TradeTime,
    #[serde(rename = "stopTime")]
    stop_time: TradeTime,
    #[serde(rename = "closeTime")]
    close_time: TradeTime,
    #[serde(
        rename = "sundayOpenTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    sunday_open_time: Option<TradeTime>,
    #[serde(rename = "allDay", default, skip_serializing_if = "Option::is_none")]
    all_day: Option<bool>,
}

impl ProductSession {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> Option<&super::ids::ProductSessionId> {
        self.id.as_ref()
    }

    /// Returns wire field `openTime`.
    #[must_use]
    pub fn open_time(&self) -> &TradeTime {
        &self.open_time
    }

    /// Returns wire field `startTime`.
    #[must_use]
    pub fn start_time(&self) -> &TradeTime {
        &self.start_time
    }

    /// Returns wire field `stopTime`.
    #[must_use]
    pub fn stop_time(&self) -> &TradeTime {
        &self.stop_time
    }

    /// Returns wire field `closeTime`.
    #[must_use]
    pub fn close_time(&self) -> &TradeTime {
        &self.close_time
    }

    /// Returns wire field `sundayOpenTime`.
    #[must_use]
    pub fn sunday_open_time(&self) -> Option<&TradeTime> {
        self.sunday_open_time.as_ref()
    }

    /// Returns wire field `allDay`.
    #[must_use]
    pub fn all_day(&self) -> Option<&bool> {
        self.all_day.as_ref()
    }

    /// Starts a builder for [`ProductSession`].
    pub fn builder() -> ProductSessionBuilder {
        ProductSessionBuilder::default()
    }
}

/// Builder for [`ProductSession`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ProductSessionBuilder {
    id: Option<super::ids::ProductSessionId>,
    open_time: Option<TradeTime>,
    start_time: Option<TradeTime>,
    stop_time: Option<TradeTime>,
    close_time: Option<TradeTime>,
    sunday_open_time: Option<TradeTime>,
    all_day: Option<bool>,
}

impl ProductSessionBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::ProductSessionId) -> Self {
        self.id = Some(value);
        self
    }

    /// Sets wire field `openTime`.
    pub fn open_time(mut self, value: TradeTime) -> Self {
        self.open_time = Some(value);
        self
    }

    /// Sets wire field `startTime`.
    pub fn start_time(mut self, value: TradeTime) -> Self {
        self.start_time = Some(value);
        self
    }

    /// Sets wire field `stopTime`.
    pub fn stop_time(mut self, value: TradeTime) -> Self {
        self.stop_time = Some(value);
        self
    }

    /// Sets wire field `closeTime`.
    pub fn close_time(mut self, value: TradeTime) -> Self {
        self.close_time = Some(value);
        self
    }

    /// Sets wire field `sundayOpenTime`.
    pub fn sunday_open_time(mut self, value: TradeTime) -> Self {
        self.sunday_open_time = Some(value);
        self
    }

    /// Sets wire field `allDay`.
    pub fn all_day(mut self, value: bool) -> Self {
        self.all_day = Some(value);
        self
    }

    /// Validates required fields and builds [`ProductSession`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ProductSession, crate::api::current::BuildError> {
        let open_time = self
            .open_time
            .ok_or(crate::api::current::BuildError::missing("openTime"))?;
        let start_time = self
            .start_time
            .ok_or(crate::api::current::BuildError::missing("startTime"))?;
        let stop_time = self
            .stop_time
            .ok_or(crate::api::current::BuildError::missing("stopTime"))?;
        let close_time = self
            .close_time
            .ok_or(crate::api::current::BuildError::missing("closeTime"))?;
        Ok(ProductSession {
            id: self.id,
            open_time,
            start_time,
            stop_time,
            close_time,
            sunday_open_time: self.sunday_open_time,
            all_day: self.all_day,
        })
    }
}

/// Current wire model `RollContract`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct RollContract {
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "forward")]
    forward: bool,
    #[serde(rename = "ifExpired", default, skip_serializing_if = "Option::is_none")]
    if_expired: Option<bool>,
}

impl RollContract {
    /// Returns wire field `name`.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns wire field `forward`.
    #[must_use]
    pub fn forward(&self) -> &bool {
        &self.forward
    }

    /// Returns wire field `ifExpired`.
    #[must_use]
    pub fn if_expired(&self) -> Option<&bool> {
        self.if_expired.as_ref()
    }

    /// Starts a builder for [`RollContract`].
    pub fn builder() -> RollContractBuilder {
        RollContractBuilder::default()
    }
}

/// Builder for [`RollContract`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct RollContractBuilder {
    name: Option<String>,
    forward: Option<bool>,
    if_expired: Option<bool>,
}

impl RollContractBuilder {
    /// Sets wire field `name`.
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Sets wire field `forward`.
    pub fn forward(mut self, value: bool) -> Self {
        self.forward = Some(value);
        self
    }

    /// Sets wire field `ifExpired`.
    pub fn if_expired(mut self, value: bool) -> Self {
        self.if_expired = Some(value);
        self
    }

    /// Validates required fields and builds [`RollContract`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<RollContract, crate::api::current::BuildError> {
        let name = self
            .name
            .ok_or(crate::api::current::BuildError::missing("name"))?;
        if name.is_empty() || name.trim() != name {
            return Err(crate::api::current::BuildError::invalid(
                "name",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        let forward = self
            .forward
            .ok_or(crate::api::current::BuildError::missing("forward"))?;
        Ok(RollContract {
            name,
            forward,
            if_expired: self.if_expired,
        })
    }
}

impl crate::api::current::support::CurrentRequest for RollContract {
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

/// Current wire model `RollContractBase`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct RollContractBase {
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "forward")]
    forward: bool,
    #[serde(rename = "ifExpired", default, skip_serializing_if = "Option::is_none")]
    if_expired: Option<bool>,
}

impl RollContractBase {
    /// Returns wire field `name`.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns wire field `forward`.
    #[must_use]
    pub fn forward(&self) -> &bool {
        &self.forward
    }

    /// Returns wire field `ifExpired`.
    #[must_use]
    pub fn if_expired(&self) -> Option<&bool> {
        self.if_expired.as_ref()
    }

    /// Starts a builder for [`RollContractBase`].
    pub fn builder() -> RollContractBaseBuilder {
        RollContractBaseBuilder::default()
    }
}

/// Builder for [`RollContractBase`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct RollContractBaseBuilder {
    name: Option<String>,
    forward: Option<bool>,
    if_expired: Option<bool>,
}

impl RollContractBaseBuilder {
    /// Sets wire field `name`.
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Sets wire field `forward`.
    pub fn forward(mut self, value: bool) -> Self {
        self.forward = Some(value);
        self
    }

    /// Sets wire field `ifExpired`.
    pub fn if_expired(mut self, value: bool) -> Self {
        self.if_expired = Some(value);
        self
    }

    /// Validates required fields and builds [`RollContractBase`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<RollContractBase, crate::api::current::BuildError> {
        let name = self
            .name
            .ok_or(crate::api::current::BuildError::missing("name"))?;
        let forward = self
            .forward
            .ok_or(crate::api::current::BuildError::missing("forward"))?;
        Ok(RollContractBase {
            name,
            forward,
            if_expired: self.if_expired,
        })
    }
}

/// Current wire model `RollContractResponse`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct RollContractResponse {
    #[serde(rename = "errorText", default, skip_serializing_if = "Option::is_none")]
    error_text: Option<String>,
    #[serde(rename = "contract", default, skip_serializing_if = "Option::is_none")]
    contract: Option<super::users::Contract>,
}

impl RollContractResponse {
    /// Returns wire field `errorText`.
    #[must_use]
    pub fn error_text(&self) -> Option<&str> {
        self.error_text.as_deref()
    }

    /// Returns wire field `contract`.
    #[must_use]
    pub fn contract(&self) -> Option<&super::users::Contract> {
        self.contract.as_ref()
    }

    /// Starts a builder for [`RollContractResponse`].
    pub fn builder() -> RollContractResponseBuilder {
        RollContractResponseBuilder::default()
    }
}

/// Builder for [`RollContractResponse`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct RollContractResponseBuilder {
    error_text: Option<String>,
    contract: Option<super::users::Contract>,
}

impl RollContractResponseBuilder {
    /// Sets wire field `errorText`.
    pub fn error_text(mut self, value: impl Into<String>) -> Self {
        self.error_text = Some(value.into());
        self
    }

    /// Sets wire field `contract`.
    pub fn contract(mut self, value: super::users::Contract) -> Self {
        self.contract = Some(value);
        self
    }

    /// Validates required fields and builds [`RollContractResponse`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<RollContractResponse, crate::api::current::BuildError> {
        Ok(RollContractResponse {
            error_text: self.error_text,
            contract: self.contract,
        })
    }
}

/// Current wire model `RollContracts`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct RollContracts {
    #[serde(rename = "rollContracts")]
    roll_contracts: Vec<RollContractBase>,
}

impl RollContracts {
    /// Returns wire field `rollContracts`.
    #[must_use]
    pub fn roll_contracts(&self) -> &[RollContractBase] {
        &self.roll_contracts
    }

    /// Starts a builder for [`RollContracts`].
    pub fn builder() -> RollContractsBuilder {
        RollContractsBuilder::default()
    }
}

/// Builder for [`RollContracts`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct RollContractsBuilder {
    roll_contracts: Option<Vec<RollContractBase>>,
}

impl RollContractsBuilder {
    /// Sets wire field `rollContracts`.
    pub fn roll_contracts(mut self, value: Vec<RollContractBase>) -> Self {
        self.roll_contracts = Some(value);
        self
    }

    /// Validates required fields and builds [`RollContracts`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<RollContracts, crate::api::current::BuildError> {
        let roll_contracts = self
            .roll_contracts
            .ok_or(crate::api::current::BuildError::missing("rollContracts"))?;
        if roll_contracts.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "rollContracts",
                "must not be empty",
            ));
        }
        Ok(RollContracts { roll_contracts })
    }
}

impl crate::api::current::support::CurrentRequest for RollContracts {
    fn validate_current(&self) -> Result<(), crate::Error> {
        if self.roll_contracts.is_empty() {
            return Err(crate::Error::InvalidRequest {
                field: "rollContracts",
                reason: "must not be empty",
            });
        }
        Ok(())
    }
}

/// Current wire model `RollContractsResponse`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct RollContractsResponse {
    #[serde(rename = "errorText", default, skip_serializing_if = "Option::is_none")]
    error_text: Option<String>,
    #[serde(rename = "contracts")]
    contracts: RollContractsResponseContracts,
}

impl RollContractsResponse {
    /// Returns wire field `errorText`.
    #[must_use]
    pub fn error_text(&self) -> Option<&str> {
        self.error_text.as_deref()
    }

    /// Returns wire field `contracts`.
    #[must_use]
    pub fn contracts(&self) -> &RollContractsResponseContracts {
        &self.contracts
    }

    /// Starts a builder for [`RollContractsResponse`].
    pub fn builder() -> RollContractsResponseBuilder {
        RollContractsResponseBuilder::default()
    }
}

/// Builder for [`RollContractsResponse`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct RollContractsResponseBuilder {
    error_text: Option<String>,
    contracts: Option<RollContractsResponseContracts>,
}

impl RollContractsResponseBuilder {
    /// Sets wire field `errorText`.
    pub fn error_text(mut self, value: impl Into<String>) -> Self {
        self.error_text = Some(value.into());
        self
    }

    /// Sets wire field `contracts`.
    pub fn contracts(mut self, value: RollContractsResponseContracts) -> Self {
        self.contracts = Some(value);
        self
    }

    /// Validates required fields and builds [`RollContractsResponse`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<RollContractsResponse, crate::api::current::BuildError> {
        let contracts = self
            .contracts
            .ok_or(crate::api::current::BuildError::missing("contracts"))?;
        Ok(RollContractsResponse {
            error_text: self.error_text,
            contracts,
        })
    }
}

/// Documentation-blocked current wire placeholder `RollContractsResponseContracts`.
///
/// The pinned contract publishes no member grammar. Deserialization
/// therefore accepts only an empty object and fails closed on provider data.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct RollContractsResponseContracts {}

impl RollContractsResponseContracts {
    /// Starts a builder for [`RollContractsResponseContracts`].
    pub fn builder() -> RollContractsResponseContractsBuilder {
        RollContractsResponseContractsBuilder::default()
    }
}

/// Builder for [`RollContractsResponseContracts`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct RollContractsResponseContractsBuilder {}

impl RollContractsResponseContractsBuilder {
    /// Validates required fields and builds [`RollContractsResponseContracts`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<RollContractsResponseContracts, crate::api::current::BuildError> {
        Ok(RollContractsResponseContracts {})
    }
}

/// Current wire model `TradeTime`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct TradeTime {
    #[serde(rename = "hour")]
    hour: i64,
    #[serde(rename = "minute")]
    minute: i64,
}

impl TradeTime {
    /// Returns wire field `hour`.
    #[must_use]
    pub fn hour(&self) -> &i64 {
        &self.hour
    }

    /// Returns wire field `minute`.
    #[must_use]
    pub fn minute(&self) -> &i64 {
        &self.minute
    }

    /// Starts a builder for [`TradeTime`].
    pub fn builder() -> TradeTimeBuilder {
        TradeTimeBuilder::default()
    }
}

/// Builder for [`TradeTime`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct TradeTimeBuilder {
    hour: Option<i64>,
    minute: Option<i64>,
}

impl TradeTimeBuilder {
    /// Sets wire field `hour`.
    pub fn hour(mut self, value: i64) -> Self {
        self.hour = Some(value);
        self
    }

    /// Sets wire field `minute`.
    pub fn minute(mut self, value: i64) -> Self {
        self.minute = Some(value);
        self
    }

    /// Validates required fields and builds [`TradeTime`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<TradeTime, crate::api::current::BuildError> {
        let hour = self
            .hour
            .ok_or(crate::api::current::BuildError::missing("hour"))?;
        let minute = self
            .minute
            .ok_or(crate::api::current::BuildError::missing("minute"))?;
        Ok(TradeTime { hour, minute })
    }
}

/// Typed query parameters for `/contract/deps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ContractDependentsQuery {
    #[serde(rename = "masterid")]
    masterid: super::ids::ProviderEntityId,
}

impl ContractDependentsQuery {
    /// Returns wire field `masterid`.
    #[must_use]
    pub fn masterid(&self) -> &super::ids::ProviderEntityId {
        &self.masterid
    }

    /// Starts a builder for [`ContractDependentsQuery`].
    pub fn builder() -> ContractDependentsQueryBuilder {
        ContractDependentsQueryBuilder::default()
    }
}

/// Builder for [`ContractDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ContractDependentsQueryBuilder {
    masterid: Option<super::ids::ProviderEntityId>,
}

impl ContractDependentsQueryBuilder {
    /// Sets wire field `masterid`.
    pub fn masterid(mut self, value: super::ids::ProviderEntityId) -> Self {
        self.masterid = Some(value);
        self
    }

    /// Validates required fields and builds [`ContractDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ContractDependentsQuery, crate::api::current::BuildError> {
        let masterid = self
            .masterid
            .ok_or(crate::api::current::BuildError::missing("masterid"))?;
        Ok(ContractDependentsQuery { masterid })
    }
}

impl crate::api::current::support::CurrentQuery for ContractDependentsQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "masterid", &self.masterid)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /contract/deps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn contract_dependents(
        &self,
        query: &ContractDependentsQuery,
    ) -> Result<Vec<super::users::Contract>, crate::Error> {
        self.get_current("/contract/deps", query).await
    }
}

/// Typed query parameters for `/contract/find`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ContractFindQuery {
    #[serde(rename = "name")]
    name: String,
}

impl ContractFindQuery {
    /// Returns wire field `name`.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Starts a builder for [`ContractFindQuery`].
    pub fn builder() -> ContractFindQueryBuilder {
        ContractFindQueryBuilder::default()
    }
}

/// Builder for [`ContractFindQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ContractFindQueryBuilder {
    name: Option<String>,
}

impl ContractFindQueryBuilder {
    /// Sets wire field `name`.
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Validates required fields and builds [`ContractFindQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ContractFindQuery, crate::api::current::BuildError> {
        let name = self
            .name
            .ok_or(crate::api::current::BuildError::missing("name"))?;
        if name.is_empty() || name.trim() != name {
            return Err(crate::api::current::BuildError::invalid(
                "name",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        Ok(ContractFindQuery { name })
    }
}

impl crate::api::current::support::CurrentQuery for ContractFindQuery {
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
    /// Calls the current `GET /contract/find` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn contract_find(
        &self,
        query: &ContractFindQuery,
    ) -> Result<super::users::Contract, crate::Error> {
        self.get_current("/contract/find", query).await
    }
}

/// Typed query parameters for `/contractGroup/find`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ContractGroupFindQuery {
    #[serde(rename = "name")]
    name: String,
}

impl ContractGroupFindQuery {
    /// Returns wire field `name`.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Starts a builder for [`ContractGroupFindQuery`].
    pub fn builder() -> ContractGroupFindQueryBuilder {
        ContractGroupFindQueryBuilder::default()
    }
}

/// Builder for [`ContractGroupFindQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ContractGroupFindQueryBuilder {
    name: Option<String>,
}

impl ContractGroupFindQueryBuilder {
    /// Sets wire field `name`.
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Validates required fields and builds [`ContractGroupFindQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ContractGroupFindQuery, crate::api::current::BuildError> {
        let name = self
            .name
            .ok_or(crate::api::current::BuildError::missing("name"))?;
        if name.is_empty() || name.trim() != name {
            return Err(crate::api::current::BuildError::invalid(
                "name",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        Ok(ContractGroupFindQuery { name })
    }
}

impl crate::api::current::support::CurrentQuery for ContractGroupFindQuery {
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
    /// Calls the current `GET /contractGroup/find` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn contract_group_find(
        &self,
        query: &ContractGroupFindQuery,
    ) -> Result<super::users::ContractGroup, crate::Error> {
        self.get_current("/contractGroup/find", query).await
    }
}

/// Typed query parameters for `/contractGroup/item`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ContractGroupItemQuery {
    #[serde(rename = "id")]
    id: super::ids::ContractGroupId,
}

impl ContractGroupItemQuery {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> &super::ids::ContractGroupId {
        &self.id
    }

    /// Starts a builder for [`ContractGroupItemQuery`].
    pub fn builder() -> ContractGroupItemQueryBuilder {
        ContractGroupItemQueryBuilder::default()
    }
}

/// Builder for [`ContractGroupItemQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ContractGroupItemQueryBuilder {
    id: Option<super::ids::ContractGroupId>,
}

impl ContractGroupItemQueryBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::ContractGroupId) -> Self {
        self.id = Some(value);
        self
    }

    /// Validates required fields and builds [`ContractGroupItemQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ContractGroupItemQuery, crate::api::current::BuildError> {
        let id = self
            .id
            .ok_or(crate::api::current::BuildError::missing("id"))?;
        Ok(ContractGroupItemQuery { id })
    }
}

impl crate::api::current::support::CurrentQuery for ContractGroupItemQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "id", &self.id)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /contractGroup/item` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn contract_group_item(
        &self,
        query: &ContractGroupItemQuery,
    ) -> Result<super::users::ContractGroup, crate::Error> {
        self.get_current("/contractGroup/item", query).await
    }
}

/// Typed query parameters for `/contractGroup/items`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ContractGroupItemsQuery {
    #[serde(rename = "ids")]
    ids: Vec<super::ids::ContractGroupId>,
}

impl ContractGroupItemsQuery {
    /// Returns wire field `ids`.
    #[must_use]
    pub fn ids(&self) -> &[super::ids::ContractGroupId] {
        &self.ids
    }

    /// Starts a builder for [`ContractGroupItemsQuery`].
    pub fn builder() -> ContractGroupItemsQueryBuilder {
        ContractGroupItemsQueryBuilder::default()
    }
}

/// Builder for [`ContractGroupItemsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ContractGroupItemsQueryBuilder {
    ids: Option<Vec<super::ids::ContractGroupId>>,
}

impl ContractGroupItemsQueryBuilder {
    /// Sets wire field `ids`.
    pub fn ids(mut self, value: Vec<super::ids::ContractGroupId>) -> Self {
        self.ids = Some(value);
        self
    }

    /// Validates required fields and builds [`ContractGroupItemsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ContractGroupItemsQuery, crate::api::current::BuildError> {
        let ids = self
            .ids
            .ok_or(crate::api::current::BuildError::missing("ids"))?;
        if ids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "ids",
                "must not be empty",
            ));
        }
        Ok(ContractGroupItemsQuery { ids })
    }
}

impl crate::api::current::support::CurrentQuery for ContractGroupItemsQuery {
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
    /// Calls the current `GET /contractGroup/items` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn contract_group_items(
        &self,
        query: &ContractGroupItemsQuery,
    ) -> Result<Vec<super::users::ContractGroup>, crate::Error> {
        self.get_current("/contractGroup/items", query).await
    }
}

impl crate::Client {
    /// Calls the current `GET /contractGroup/list` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn contract_group_list(
        &self,
    ) -> Result<Vec<super::users::ContractGroup>, crate::Error> {
        self.get_without_query("/contractGroup/list").await
    }
}

/// Typed query parameters for `/contractGroup/suggest`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ContractGroupSuggestQuery {
    #[serde(rename = "t")]
    t: String,
    #[serde(rename = "l")]
    l: i64,
}

impl ContractGroupSuggestQuery {
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

    /// Starts a builder for [`ContractGroupSuggestQuery`].
    pub fn builder() -> ContractGroupSuggestQueryBuilder {
        ContractGroupSuggestQueryBuilder::default()
    }
}

/// Builder for [`ContractGroupSuggestQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ContractGroupSuggestQueryBuilder {
    t: Option<String>,
    l: Option<i64>,
}

impl ContractGroupSuggestQueryBuilder {
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

    /// Validates required fields and builds [`ContractGroupSuggestQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ContractGroupSuggestQuery, crate::api::current::BuildError> {
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
        Ok(ContractGroupSuggestQuery { t, l })
    }
}

impl crate::api::current::support::CurrentQuery for ContractGroupSuggestQuery {
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
    /// Calls the current `GET /contractGroup/suggest` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn contract_group_suggest(
        &self,
        query: &ContractGroupSuggestQuery,
    ) -> Result<Vec<super::users::ContractGroup>, crate::Error> {
        self.get_current("/contractGroup/suggest", query).await
    }
}

/// Typed query parameters for `/contract/item`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ContractItemQuery {
    #[serde(rename = "id")]
    id: crate::ContractId,
}

impl ContractItemQuery {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> &crate::ContractId {
        &self.id
    }

    /// Starts a builder for [`ContractItemQuery`].
    pub fn builder() -> ContractItemQueryBuilder {
        ContractItemQueryBuilder::default()
    }
}

/// Builder for [`ContractItemQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ContractItemQueryBuilder {
    id: Option<crate::ContractId>,
}

impl ContractItemQueryBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: crate::ContractId) -> Self {
        self.id = Some(value);
        self
    }

    /// Validates required fields and builds [`ContractItemQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ContractItemQuery, crate::api::current::BuildError> {
        let id = self
            .id
            .ok_or(crate::api::current::BuildError::missing("id"))?;
        Ok(ContractItemQuery { id })
    }
}

impl crate::api::current::support::CurrentQuery for ContractItemQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "id", &self.id)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /contract/item` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn contract_item(
        &self,
        query: &ContractItemQuery,
    ) -> Result<super::users::Contract, crate::Error> {
        self.get_current("/contract/item", query).await
    }
}

/// Typed query parameters for `/contract/items`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ContractItemsQuery {
    #[serde(rename = "ids")]
    ids: Vec<crate::ContractId>,
}

impl ContractItemsQuery {
    /// Returns wire field `ids`.
    #[must_use]
    pub fn ids(&self) -> &[crate::ContractId] {
        &self.ids
    }

    /// Starts a builder for [`ContractItemsQuery`].
    pub fn builder() -> ContractItemsQueryBuilder {
        ContractItemsQueryBuilder::default()
    }
}

/// Builder for [`ContractItemsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ContractItemsQueryBuilder {
    ids: Option<Vec<crate::ContractId>>,
}

impl ContractItemsQueryBuilder {
    /// Sets wire field `ids`.
    pub fn ids(mut self, value: Vec<crate::ContractId>) -> Self {
        self.ids = Some(value);
        self
    }

    /// Validates required fields and builds [`ContractItemsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ContractItemsQuery, crate::api::current::BuildError> {
        let ids = self
            .ids
            .ok_or(crate::api::current::BuildError::missing("ids"))?;
        if ids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "ids",
                "must not be empty",
            ));
        }
        Ok(ContractItemsQuery { ids })
    }
}

impl crate::api::current::support::CurrentQuery for ContractItemsQuery {
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
    /// Calls the current `GET /contract/items` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn contract_items(
        &self,
        query: &ContractItemsQuery,
    ) -> Result<Vec<super::users::Contract>, crate::Error> {
        self.get_current("/contract/items", query).await
    }
}

/// Typed query parameters for `/contract/ldeps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ContractLDependentsQuery {
    #[serde(rename = "masterids")]
    masterids: Vec<super::ids::ProviderEntityId>,
}

impl ContractLDependentsQuery {
    /// Returns wire field `masterids`.
    #[must_use]
    pub fn masterids(&self) -> &[super::ids::ProviderEntityId] {
        &self.masterids
    }

    /// Starts a builder for [`ContractLDependentsQuery`].
    pub fn builder() -> ContractLDependentsQueryBuilder {
        ContractLDependentsQueryBuilder::default()
    }
}

/// Builder for [`ContractLDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ContractLDependentsQueryBuilder {
    masterids: Option<Vec<super::ids::ProviderEntityId>>,
}

impl ContractLDependentsQueryBuilder {
    /// Sets wire field `masterids`.
    pub fn masterids(mut self, value: Vec<super::ids::ProviderEntityId>) -> Self {
        self.masterids = Some(value);
        self
    }

    /// Validates required fields and builds [`ContractLDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ContractLDependentsQuery, crate::api::current::BuildError> {
        let masterids = self
            .masterids
            .ok_or(crate::api::current::BuildError::missing("masterids"))?;
        if masterids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "masterids",
                "must not be empty",
            ));
        }
        Ok(ContractLDependentsQuery { masterids })
    }
}

impl crate::api::current::support::CurrentQuery for ContractLDependentsQuery {
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
    /// Calls the current `GET /contract/ldeps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn contract_l_dependents(
        &self,
        query: &ContractLDependentsQuery,
    ) -> Result<Vec<super::users::Contract>, crate::Error> {
        self.get_current("/contract/ldeps", query).await
    }
}

/// Typed query parameters for `/contractMaturity/deps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ContractMaturityDependentsQuery {
    #[serde(rename = "masterid")]
    masterid: super::ids::ProviderEntityId,
}

impl ContractMaturityDependentsQuery {
    /// Returns wire field `masterid`.
    #[must_use]
    pub fn masterid(&self) -> &super::ids::ProviderEntityId {
        &self.masterid
    }

    /// Starts a builder for [`ContractMaturityDependentsQuery`].
    pub fn builder() -> ContractMaturityDependentsQueryBuilder {
        ContractMaturityDependentsQueryBuilder::default()
    }
}

/// Builder for [`ContractMaturityDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ContractMaturityDependentsQueryBuilder {
    masterid: Option<super::ids::ProviderEntityId>,
}

impl ContractMaturityDependentsQueryBuilder {
    /// Sets wire field `masterid`.
    pub fn masterid(mut self, value: super::ids::ProviderEntityId) -> Self {
        self.masterid = Some(value);
        self
    }

    /// Validates required fields and builds [`ContractMaturityDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ContractMaturityDependentsQuery, crate::api::current::BuildError> {
        let masterid = self
            .masterid
            .ok_or(crate::api::current::BuildError::missing("masterid"))?;
        Ok(ContractMaturityDependentsQuery { masterid })
    }
}

impl crate::api::current::support::CurrentQuery for ContractMaturityDependentsQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "masterid", &self.masterid)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /contractMaturity/deps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn contract_maturity_dependents(
        &self,
        query: &ContractMaturityDependentsQuery,
    ) -> Result<Vec<super::users::ContractMaturity>, crate::Error> {
        self.get_current("/contractMaturity/deps", query).await
    }
}

/// Typed query parameters for `/contractMaturity/item`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ContractMaturityItemQuery {
    #[serde(rename = "id")]
    id: crate::ContractMaturityId,
}

impl ContractMaturityItemQuery {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> &crate::ContractMaturityId {
        &self.id
    }

    /// Starts a builder for [`ContractMaturityItemQuery`].
    pub fn builder() -> ContractMaturityItemQueryBuilder {
        ContractMaturityItemQueryBuilder::default()
    }
}

/// Builder for [`ContractMaturityItemQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ContractMaturityItemQueryBuilder {
    id: Option<crate::ContractMaturityId>,
}

impl ContractMaturityItemQueryBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: crate::ContractMaturityId) -> Self {
        self.id = Some(value);
        self
    }

    /// Validates required fields and builds [`ContractMaturityItemQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ContractMaturityItemQuery, crate::api::current::BuildError> {
        let id = self
            .id
            .ok_or(crate::api::current::BuildError::missing("id"))?;
        Ok(ContractMaturityItemQuery { id })
    }
}

impl crate::api::current::support::CurrentQuery for ContractMaturityItemQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "id", &self.id)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /contractMaturity/item` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn contract_maturity_item(
        &self,
        query: &ContractMaturityItemQuery,
    ) -> Result<super::users::ContractMaturity, crate::Error> {
        self.get_current("/contractMaturity/item", query).await
    }
}

/// Typed query parameters for `/contractMaturity/items`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ContractMaturityItemsQuery {
    #[serde(rename = "ids")]
    ids: Vec<crate::ContractMaturityId>,
}

impl ContractMaturityItemsQuery {
    /// Returns wire field `ids`.
    #[must_use]
    pub fn ids(&self) -> &[crate::ContractMaturityId] {
        &self.ids
    }

    /// Starts a builder for [`ContractMaturityItemsQuery`].
    pub fn builder() -> ContractMaturityItemsQueryBuilder {
        ContractMaturityItemsQueryBuilder::default()
    }
}

/// Builder for [`ContractMaturityItemsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ContractMaturityItemsQueryBuilder {
    ids: Option<Vec<crate::ContractMaturityId>>,
}

impl ContractMaturityItemsQueryBuilder {
    /// Sets wire field `ids`.
    pub fn ids(mut self, value: Vec<crate::ContractMaturityId>) -> Self {
        self.ids = Some(value);
        self
    }

    /// Validates required fields and builds [`ContractMaturityItemsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ContractMaturityItemsQuery, crate::api::current::BuildError> {
        let ids = self
            .ids
            .ok_or(crate::api::current::BuildError::missing("ids"))?;
        if ids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "ids",
                "must not be empty",
            ));
        }
        Ok(ContractMaturityItemsQuery { ids })
    }
}

impl crate::api::current::support::CurrentQuery for ContractMaturityItemsQuery {
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
    /// Calls the current `GET /contractMaturity/items` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn contract_maturity_items(
        &self,
        query: &ContractMaturityItemsQuery,
    ) -> Result<Vec<super::users::ContractMaturity>, crate::Error> {
        self.get_current("/contractMaturity/items", query).await
    }
}

/// Typed query parameters for `/contractMaturity/ldeps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ContractMaturityLDependentsQuery {
    #[serde(rename = "masterids")]
    masterids: Vec<super::ids::ProviderEntityId>,
}

impl ContractMaturityLDependentsQuery {
    /// Returns wire field `masterids`.
    #[must_use]
    pub fn masterids(&self) -> &[super::ids::ProviderEntityId] {
        &self.masterids
    }

    /// Starts a builder for [`ContractMaturityLDependentsQuery`].
    pub fn builder() -> ContractMaturityLDependentsQueryBuilder {
        ContractMaturityLDependentsQueryBuilder::default()
    }
}

/// Builder for [`ContractMaturityLDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ContractMaturityLDependentsQueryBuilder {
    masterids: Option<Vec<super::ids::ProviderEntityId>>,
}

impl ContractMaturityLDependentsQueryBuilder {
    /// Sets wire field `masterids`.
    pub fn masterids(mut self, value: Vec<super::ids::ProviderEntityId>) -> Self {
        self.masterids = Some(value);
        self
    }

    /// Validates required fields and builds [`ContractMaturityLDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(
        self,
    ) -> Result<ContractMaturityLDependentsQuery, crate::api::current::BuildError> {
        let masterids = self
            .masterids
            .ok_or(crate::api::current::BuildError::missing("masterids"))?;
        if masterids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "masterids",
                "must not be empty",
            ));
        }
        Ok(ContractMaturityLDependentsQuery { masterids })
    }
}

impl crate::api::current::support::CurrentQuery for ContractMaturityLDependentsQuery {
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
    /// Calls the current `GET /contractMaturity/ldeps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn contract_maturity_l_dependents(
        &self,
        query: &ContractMaturityLDependentsQuery,
    ) -> Result<Vec<super::users::ContractMaturity>, crate::Error> {
        self.get_current("/contractMaturity/ldeps", query).await
    }
}

/// Typed query parameters for `/contract/suggest`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ContractSuggestQuery {
    #[serde(rename = "t")]
    t: String,
    #[serde(rename = "l")]
    l: i64,
}

impl ContractSuggestQuery {
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

    /// Starts a builder for [`ContractSuggestQuery`].
    pub fn builder() -> ContractSuggestQueryBuilder {
        ContractSuggestQueryBuilder::default()
    }
}

/// Builder for [`ContractSuggestQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ContractSuggestQueryBuilder {
    t: Option<String>,
    l: Option<i64>,
}

impl ContractSuggestQueryBuilder {
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

    /// Validates required fields and builds [`ContractSuggestQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ContractSuggestQuery, crate::api::current::BuildError> {
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
        Ok(ContractSuggestQuery { t, l })
    }
}

impl crate::api::current::support::CurrentQuery for ContractSuggestQuery {
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
    /// Calls the current `GET /contract/suggest` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn contract_suggest(
        &self,
        query: &ContractSuggestQuery,
    ) -> Result<Vec<super::users::Contract>, crate::Error> {
        self.get_current("/contract/suggest", query).await
    }
}

/// Typed query parameters for `/currency/find`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct CurrencyFindQuery {
    #[serde(rename = "name")]
    name: String,
}

impl CurrencyFindQuery {
    /// Returns wire field `name`.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Starts a builder for [`CurrencyFindQuery`].
    pub fn builder() -> CurrencyFindQueryBuilder {
        CurrencyFindQueryBuilder::default()
    }
}

/// Builder for [`CurrencyFindQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct CurrencyFindQueryBuilder {
    name: Option<String>,
}

impl CurrencyFindQueryBuilder {
    /// Sets wire field `name`.
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Validates required fields and builds [`CurrencyFindQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<CurrencyFindQuery, crate::api::current::BuildError> {
        let name = self
            .name
            .ok_or(crate::api::current::BuildError::missing("name"))?;
        if name.is_empty() || name.trim() != name {
            return Err(crate::api::current::BuildError::invalid(
                "name",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        Ok(CurrencyFindQuery { name })
    }
}

impl crate::api::current::support::CurrentQuery for CurrencyFindQuery {
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
    /// Calls the current `GET /currency/find` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn currency_find(
        &self,
        query: &CurrencyFindQuery,
    ) -> Result<super::users::Currency, crate::Error> {
        self.get_current("/currency/find", query).await
    }
}

/// Typed query parameters for `/currency/item`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct CurrencyItemQuery {
    #[serde(rename = "id")]
    id: super::ids::CurrencyId,
}

impl CurrencyItemQuery {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> &super::ids::CurrencyId {
        &self.id
    }

    /// Starts a builder for [`CurrencyItemQuery`].
    pub fn builder() -> CurrencyItemQueryBuilder {
        CurrencyItemQueryBuilder::default()
    }
}

/// Builder for [`CurrencyItemQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct CurrencyItemQueryBuilder {
    id: Option<super::ids::CurrencyId>,
}

impl CurrencyItemQueryBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::CurrencyId) -> Self {
        self.id = Some(value);
        self
    }

    /// Validates required fields and builds [`CurrencyItemQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<CurrencyItemQuery, crate::api::current::BuildError> {
        let id = self
            .id
            .ok_or(crate::api::current::BuildError::missing("id"))?;
        Ok(CurrencyItemQuery { id })
    }
}

impl crate::api::current::support::CurrentQuery for CurrencyItemQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "id", &self.id)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /currency/item` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn currency_item(
        &self,
        query: &CurrencyItemQuery,
    ) -> Result<super::users::Currency, crate::Error> {
        self.get_current("/currency/item", query).await
    }
}

/// Typed query parameters for `/currency/items`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct CurrencyItemsQuery {
    #[serde(rename = "ids")]
    ids: Vec<super::ids::CurrencyId>,
}

impl CurrencyItemsQuery {
    /// Returns wire field `ids`.
    #[must_use]
    pub fn ids(&self) -> &[super::ids::CurrencyId] {
        &self.ids
    }

    /// Starts a builder for [`CurrencyItemsQuery`].
    pub fn builder() -> CurrencyItemsQueryBuilder {
        CurrencyItemsQueryBuilder::default()
    }
}

/// Builder for [`CurrencyItemsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct CurrencyItemsQueryBuilder {
    ids: Option<Vec<super::ids::CurrencyId>>,
}

impl CurrencyItemsQueryBuilder {
    /// Sets wire field `ids`.
    pub fn ids(mut self, value: Vec<super::ids::CurrencyId>) -> Self {
        self.ids = Some(value);
        self
    }

    /// Validates required fields and builds [`CurrencyItemsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<CurrencyItemsQuery, crate::api::current::BuildError> {
        let ids = self
            .ids
            .ok_or(crate::api::current::BuildError::missing("ids"))?;
        if ids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "ids",
                "must not be empty",
            ));
        }
        Ok(CurrencyItemsQuery { ids })
    }
}

impl crate::api::current::support::CurrentQuery for CurrencyItemsQuery {
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
    /// Calls the current `GET /currency/items` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn currency_items(
        &self,
        query: &CurrencyItemsQuery,
    ) -> Result<Vec<super::users::Currency>, crate::Error> {
        self.get_current("/currency/items", query).await
    }
}

impl crate::Client {
    /// Calls the current `GET /currency/list` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn currency_list(&self) -> Result<Vec<super::users::Currency>, crate::Error> {
        self.get_without_query("/currency/list").await
    }
}

/// Typed query parameters for `/currencyRate/deps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct CurrencyRateDependentsQuery {
    #[serde(rename = "masterid")]
    masterid: super::ids::ProviderEntityId,
}

impl CurrencyRateDependentsQuery {
    /// Returns wire field `masterid`.
    #[must_use]
    pub fn masterid(&self) -> &super::ids::ProviderEntityId {
        &self.masterid
    }

    /// Starts a builder for [`CurrencyRateDependentsQuery`].
    pub fn builder() -> CurrencyRateDependentsQueryBuilder {
        CurrencyRateDependentsQueryBuilder::default()
    }
}

/// Builder for [`CurrencyRateDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct CurrencyRateDependentsQueryBuilder {
    masterid: Option<super::ids::ProviderEntityId>,
}

impl CurrencyRateDependentsQueryBuilder {
    /// Sets wire field `masterid`.
    pub fn masterid(mut self, value: super::ids::ProviderEntityId) -> Self {
        self.masterid = Some(value);
        self
    }

    /// Validates required fields and builds [`CurrencyRateDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<CurrencyRateDependentsQuery, crate::api::current::BuildError> {
        let masterid = self
            .masterid
            .ok_or(crate::api::current::BuildError::missing("masterid"))?;
        Ok(CurrencyRateDependentsQuery { masterid })
    }
}

impl crate::api::current::support::CurrentQuery for CurrencyRateDependentsQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "masterid", &self.masterid)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /currencyRate/deps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn currency_rate_dependents(
        &self,
        query: &CurrencyRateDependentsQuery,
    ) -> Result<Vec<CurrencyRate>, crate::Error> {
        self.get_current("/currencyRate/deps", query).await
    }
}

/// Typed query parameters for `/currencyRate/item`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct CurrencyRateItemQuery {
    #[serde(rename = "id")]
    id: super::ids::CurrencyRateId,
}

impl CurrencyRateItemQuery {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> &super::ids::CurrencyRateId {
        &self.id
    }

    /// Starts a builder for [`CurrencyRateItemQuery`].
    pub fn builder() -> CurrencyRateItemQueryBuilder {
        CurrencyRateItemQueryBuilder::default()
    }
}

/// Builder for [`CurrencyRateItemQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct CurrencyRateItemQueryBuilder {
    id: Option<super::ids::CurrencyRateId>,
}

impl CurrencyRateItemQueryBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::CurrencyRateId) -> Self {
        self.id = Some(value);
        self
    }

    /// Validates required fields and builds [`CurrencyRateItemQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<CurrencyRateItemQuery, crate::api::current::BuildError> {
        let id = self
            .id
            .ok_or(crate::api::current::BuildError::missing("id"))?;
        Ok(CurrencyRateItemQuery { id })
    }
}

impl crate::api::current::support::CurrentQuery for CurrencyRateItemQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "id", &self.id)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /currencyRate/item` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn currency_rate_item(
        &self,
        query: &CurrencyRateItemQuery,
    ) -> Result<CurrencyRate, crate::Error> {
        self.get_current("/currencyRate/item", query).await
    }
}

/// Typed query parameters for `/currencyRate/items`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct CurrencyRateItemsQuery {
    #[serde(rename = "ids")]
    ids: Vec<super::ids::CurrencyRateId>,
}

impl CurrencyRateItemsQuery {
    /// Returns wire field `ids`.
    #[must_use]
    pub fn ids(&self) -> &[super::ids::CurrencyRateId] {
        &self.ids
    }

    /// Starts a builder for [`CurrencyRateItemsQuery`].
    pub fn builder() -> CurrencyRateItemsQueryBuilder {
        CurrencyRateItemsQueryBuilder::default()
    }
}

/// Builder for [`CurrencyRateItemsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct CurrencyRateItemsQueryBuilder {
    ids: Option<Vec<super::ids::CurrencyRateId>>,
}

impl CurrencyRateItemsQueryBuilder {
    /// Sets wire field `ids`.
    pub fn ids(mut self, value: Vec<super::ids::CurrencyRateId>) -> Self {
        self.ids = Some(value);
        self
    }

    /// Validates required fields and builds [`CurrencyRateItemsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<CurrencyRateItemsQuery, crate::api::current::BuildError> {
        let ids = self
            .ids
            .ok_or(crate::api::current::BuildError::missing("ids"))?;
        if ids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "ids",
                "must not be empty",
            ));
        }
        Ok(CurrencyRateItemsQuery { ids })
    }
}

impl crate::api::current::support::CurrentQuery for CurrencyRateItemsQuery {
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
    /// Calls the current `GET /currencyRate/items` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn currency_rate_items(
        &self,
        query: &CurrencyRateItemsQuery,
    ) -> Result<Vec<CurrencyRate>, crate::Error> {
        self.get_current("/currencyRate/items", query).await
    }
}

/// Typed query parameters for `/currencyRate/ldeps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct CurrencyRateLDependentsQuery {
    #[serde(rename = "masterids")]
    masterids: Vec<super::ids::ProviderEntityId>,
}

impl CurrencyRateLDependentsQuery {
    /// Returns wire field `masterids`.
    #[must_use]
    pub fn masterids(&self) -> &[super::ids::ProviderEntityId] {
        &self.masterids
    }

    /// Starts a builder for [`CurrencyRateLDependentsQuery`].
    pub fn builder() -> CurrencyRateLDependentsQueryBuilder {
        CurrencyRateLDependentsQueryBuilder::default()
    }
}

/// Builder for [`CurrencyRateLDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct CurrencyRateLDependentsQueryBuilder {
    masterids: Option<Vec<super::ids::ProviderEntityId>>,
}

impl CurrencyRateLDependentsQueryBuilder {
    /// Sets wire field `masterids`.
    pub fn masterids(mut self, value: Vec<super::ids::ProviderEntityId>) -> Self {
        self.masterids = Some(value);
        self
    }

    /// Validates required fields and builds [`CurrencyRateLDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<CurrencyRateLDependentsQuery, crate::api::current::BuildError> {
        let masterids = self
            .masterids
            .ok_or(crate::api::current::BuildError::missing("masterids"))?;
        if masterids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "masterids",
                "must not be empty",
            ));
        }
        Ok(CurrencyRateLDependentsQuery { masterids })
    }
}

impl crate::api::current::support::CurrentQuery for CurrencyRateLDependentsQuery {
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
    /// Calls the current `GET /currencyRate/ldeps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn currency_rate_l_dependents(
        &self,
        query: &CurrencyRateLDependentsQuery,
    ) -> Result<Vec<CurrencyRate>, crate::Error> {
        self.get_current("/currencyRate/ldeps", query).await
    }
}

impl crate::Client {
    /// Calls the current `GET /currencyRate/list` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn currency_rate_list(&self) -> Result<Vec<CurrencyRate>, crate::Error> {
        self.get_without_query("/currencyRate/list").await
    }
}

/// Typed query parameters for `/currency/suggest`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct CurrencySuggestQuery {
    #[serde(rename = "t")]
    t: String,
    #[serde(rename = "l")]
    l: i64,
}

impl CurrencySuggestQuery {
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

    /// Starts a builder for [`CurrencySuggestQuery`].
    pub fn builder() -> CurrencySuggestQueryBuilder {
        CurrencySuggestQueryBuilder::default()
    }
}

/// Builder for [`CurrencySuggestQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct CurrencySuggestQueryBuilder {
    t: Option<String>,
    l: Option<i64>,
}

impl CurrencySuggestQueryBuilder {
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

    /// Validates required fields and builds [`CurrencySuggestQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<CurrencySuggestQuery, crate::api::current::BuildError> {
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
        Ok(CurrencySuggestQuery { t, l })
    }
}

impl crate::api::current::support::CurrentQuery for CurrencySuggestQuery {
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
    /// Calls the current `GET /currency/suggest` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn currency_suggest(
        &self,
        query: &CurrencySuggestQuery,
    ) -> Result<Vec<super::users::Currency>, crate::Error> {
        self.get_current("/currency/suggest", query).await
    }
}

/// Typed query parameters for `/exchange/find`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ExchangeFindQuery {
    #[serde(rename = "name")]
    name: String,
}

impl ExchangeFindQuery {
    /// Returns wire field `name`.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Starts a builder for [`ExchangeFindQuery`].
    pub fn builder() -> ExchangeFindQueryBuilder {
        ExchangeFindQueryBuilder::default()
    }
}

/// Builder for [`ExchangeFindQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ExchangeFindQueryBuilder {
    name: Option<String>,
}

impl ExchangeFindQueryBuilder {
    /// Sets wire field `name`.
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Validates required fields and builds [`ExchangeFindQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ExchangeFindQuery, crate::api::current::BuildError> {
        let name = self
            .name
            .ok_or(crate::api::current::BuildError::missing("name"))?;
        if name.is_empty() || name.trim() != name {
            return Err(crate::api::current::BuildError::invalid(
                "name",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        Ok(ExchangeFindQuery { name })
    }
}

impl crate::api::current::support::CurrentQuery for ExchangeFindQuery {
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
    /// Calls the current `GET /exchange/find` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn exchange_find(
        &self,
        query: &ExchangeFindQuery,
    ) -> Result<super::users::Exchange, crate::Error> {
        self.get_current("/exchange/find", query).await
    }
}

/// Typed query parameters for `/exchange/item`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ExchangeItemQuery {
    #[serde(rename = "id")]
    id: super::ids::ExchangeId,
}

impl ExchangeItemQuery {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> &super::ids::ExchangeId {
        &self.id
    }

    /// Starts a builder for [`ExchangeItemQuery`].
    pub fn builder() -> ExchangeItemQueryBuilder {
        ExchangeItemQueryBuilder::default()
    }
}

/// Builder for [`ExchangeItemQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ExchangeItemQueryBuilder {
    id: Option<super::ids::ExchangeId>,
}

impl ExchangeItemQueryBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::ExchangeId) -> Self {
        self.id = Some(value);
        self
    }

    /// Validates required fields and builds [`ExchangeItemQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ExchangeItemQuery, crate::api::current::BuildError> {
        let id = self
            .id
            .ok_or(crate::api::current::BuildError::missing("id"))?;
        Ok(ExchangeItemQuery { id })
    }
}

impl crate::api::current::support::CurrentQuery for ExchangeItemQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "id", &self.id)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /exchange/item` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn exchange_item(
        &self,
        query: &ExchangeItemQuery,
    ) -> Result<super::users::Exchange, crate::Error> {
        self.get_current("/exchange/item", query).await
    }
}

/// Typed query parameters for `/exchange/items`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ExchangeItemsQuery {
    #[serde(rename = "ids")]
    ids: Vec<super::ids::ExchangeId>,
}

impl ExchangeItemsQuery {
    /// Returns wire field `ids`.
    #[must_use]
    pub fn ids(&self) -> &[super::ids::ExchangeId] {
        &self.ids
    }

    /// Starts a builder for [`ExchangeItemsQuery`].
    pub fn builder() -> ExchangeItemsQueryBuilder {
        ExchangeItemsQueryBuilder::default()
    }
}

/// Builder for [`ExchangeItemsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ExchangeItemsQueryBuilder {
    ids: Option<Vec<super::ids::ExchangeId>>,
}

impl ExchangeItemsQueryBuilder {
    /// Sets wire field `ids`.
    pub fn ids(mut self, value: Vec<super::ids::ExchangeId>) -> Self {
        self.ids = Some(value);
        self
    }

    /// Validates required fields and builds [`ExchangeItemsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ExchangeItemsQuery, crate::api::current::BuildError> {
        let ids = self
            .ids
            .ok_or(crate::api::current::BuildError::missing("ids"))?;
        if ids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "ids",
                "must not be empty",
            ));
        }
        Ok(ExchangeItemsQuery { ids })
    }
}

impl crate::api::current::support::CurrentQuery for ExchangeItemsQuery {
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
    /// Calls the current `GET /exchange/items` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn exchange_items(
        &self,
        query: &ExchangeItemsQuery,
    ) -> Result<Vec<super::users::Exchange>, crate::Error> {
        self.get_current("/exchange/items", query).await
    }
}

impl crate::Client {
    /// Calls the current `GET /exchange/list` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn exchange_list(&self) -> Result<Vec<super::users::Exchange>, crate::Error> {
        self.get_without_query("/exchange/list").await
    }
}

/// Typed query parameters for `/exchange/suggest`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ExchangeSuggestQuery {
    #[serde(rename = "t")]
    t: String,
    #[serde(rename = "l")]
    l: i64,
}

impl ExchangeSuggestQuery {
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

    /// Starts a builder for [`ExchangeSuggestQuery`].
    pub fn builder() -> ExchangeSuggestQueryBuilder {
        ExchangeSuggestQueryBuilder::default()
    }
}

/// Builder for [`ExchangeSuggestQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ExchangeSuggestQueryBuilder {
    t: Option<String>,
    l: Option<i64>,
}

impl ExchangeSuggestQueryBuilder {
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

    /// Validates required fields and builds [`ExchangeSuggestQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ExchangeSuggestQuery, crate::api::current::BuildError> {
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
        Ok(ExchangeSuggestQuery { t, l })
    }
}

impl crate::api::current::support::CurrentQuery for ExchangeSuggestQuery {
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
    /// Calls the current `GET /exchange/suggest` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn exchange_suggest(
        &self,
        query: &ExchangeSuggestQuery,
    ) -> Result<Vec<super::users::Exchange>, crate::Error> {
        self.get_current("/exchange/suggest", query).await
    }
}

impl crate::Client {
    /// Calls the current `POST /contract/getproductfeeparams` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn contract_get_product_fee_params(
        &self,
        request: &GetProductFeeParams,
    ) -> Result<ProductFeeParamsResponse, crate::Error> {
        crate::api::current::support::CurrentRequest::validate_current(request)?;
        self.post_query("/contract/getproductfeeparams", request)
            .await
    }
}

/// Typed query parameters for `/product/deps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ProductDependentsQuery {
    #[serde(rename = "masterid")]
    masterid: super::ids::ProviderEntityId,
}

impl ProductDependentsQuery {
    /// Returns wire field `masterid`.
    #[must_use]
    pub fn masterid(&self) -> &super::ids::ProviderEntityId {
        &self.masterid
    }

    /// Starts a builder for [`ProductDependentsQuery`].
    pub fn builder() -> ProductDependentsQueryBuilder {
        ProductDependentsQueryBuilder::default()
    }
}

/// Builder for [`ProductDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ProductDependentsQueryBuilder {
    masterid: Option<super::ids::ProviderEntityId>,
}

impl ProductDependentsQueryBuilder {
    /// Sets wire field `masterid`.
    pub fn masterid(mut self, value: super::ids::ProviderEntityId) -> Self {
        self.masterid = Some(value);
        self
    }

    /// Validates required fields and builds [`ProductDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ProductDependentsQuery, crate::api::current::BuildError> {
        let masterid = self
            .masterid
            .ok_or(crate::api::current::BuildError::missing("masterid"))?;
        Ok(ProductDependentsQuery { masterid })
    }
}

impl crate::api::current::support::CurrentQuery for ProductDependentsQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "masterid", &self.masterid)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /product/deps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn product_dependents(
        &self,
        query: &ProductDependentsQuery,
    ) -> Result<Vec<super::users::Product>, crate::Error> {
        self.get_current("/product/deps", query).await
    }
}

/// Typed query parameters for `/product/find`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ProductFindQuery {
    #[serde(rename = "name")]
    name: String,
}

impl ProductFindQuery {
    /// Returns wire field `name`.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Starts a builder for [`ProductFindQuery`].
    pub fn builder() -> ProductFindQueryBuilder {
        ProductFindQueryBuilder::default()
    }
}

/// Builder for [`ProductFindQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ProductFindQueryBuilder {
    name: Option<String>,
}

impl ProductFindQueryBuilder {
    /// Sets wire field `name`.
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Validates required fields and builds [`ProductFindQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ProductFindQuery, crate::api::current::BuildError> {
        let name = self
            .name
            .ok_or(crate::api::current::BuildError::missing("name"))?;
        if name.is_empty() || name.trim() != name {
            return Err(crate::api::current::BuildError::invalid(
                "name",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        Ok(ProductFindQuery { name })
    }
}

impl crate::api::current::support::CurrentQuery for ProductFindQuery {
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
    /// Calls the current `GET /product/find` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn product_find(
        &self,
        query: &ProductFindQuery,
    ) -> Result<super::users::Product, crate::Error> {
        self.get_current("/product/find", query).await
    }
}

/// Typed query parameters for `/product/item`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ProductItemQuery {
    #[serde(rename = "id")]
    id: super::ids::ProductId,
}

impl ProductItemQuery {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> &super::ids::ProductId {
        &self.id
    }

    /// Starts a builder for [`ProductItemQuery`].
    pub fn builder() -> ProductItemQueryBuilder {
        ProductItemQueryBuilder::default()
    }
}

/// Builder for [`ProductItemQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ProductItemQueryBuilder {
    id: Option<super::ids::ProductId>,
}

impl ProductItemQueryBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::ProductId) -> Self {
        self.id = Some(value);
        self
    }

    /// Validates required fields and builds [`ProductItemQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ProductItemQuery, crate::api::current::BuildError> {
        let id = self
            .id
            .ok_or(crate::api::current::BuildError::missing("id"))?;
        Ok(ProductItemQuery { id })
    }
}

impl crate::api::current::support::CurrentQuery for ProductItemQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "id", &self.id)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /product/item` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn product_item(
        &self,
        query: &ProductItemQuery,
    ) -> Result<super::users::Product, crate::Error> {
        self.get_current("/product/item", query).await
    }
}

/// Typed query parameters for `/product/items`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ProductItemsQuery {
    #[serde(rename = "ids")]
    ids: Vec<super::ids::ProductId>,
}

impl ProductItemsQuery {
    /// Returns wire field `ids`.
    #[must_use]
    pub fn ids(&self) -> &[super::ids::ProductId] {
        &self.ids
    }

    /// Starts a builder for [`ProductItemsQuery`].
    pub fn builder() -> ProductItemsQueryBuilder {
        ProductItemsQueryBuilder::default()
    }
}

/// Builder for [`ProductItemsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ProductItemsQueryBuilder {
    ids: Option<Vec<super::ids::ProductId>>,
}

impl ProductItemsQueryBuilder {
    /// Sets wire field `ids`.
    pub fn ids(mut self, value: Vec<super::ids::ProductId>) -> Self {
        self.ids = Some(value);
        self
    }

    /// Validates required fields and builds [`ProductItemsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ProductItemsQuery, crate::api::current::BuildError> {
        let ids = self
            .ids
            .ok_or(crate::api::current::BuildError::missing("ids"))?;
        if ids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "ids",
                "must not be empty",
            ));
        }
        Ok(ProductItemsQuery { ids })
    }
}

impl crate::api::current::support::CurrentQuery for ProductItemsQuery {
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
    /// Calls the current `GET /product/items` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn product_items(
        &self,
        query: &ProductItemsQuery,
    ) -> Result<Vec<super::users::Product>, crate::Error> {
        self.get_current("/product/items", query).await
    }
}

/// Typed query parameters for `/product/ldeps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ProductLDependentsQuery {
    #[serde(rename = "masterids")]
    masterids: Vec<super::ids::ProviderEntityId>,
}

impl ProductLDependentsQuery {
    /// Returns wire field `masterids`.
    #[must_use]
    pub fn masterids(&self) -> &[super::ids::ProviderEntityId] {
        &self.masterids
    }

    /// Starts a builder for [`ProductLDependentsQuery`].
    pub fn builder() -> ProductLDependentsQueryBuilder {
        ProductLDependentsQueryBuilder::default()
    }
}

/// Builder for [`ProductLDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ProductLDependentsQueryBuilder {
    masterids: Option<Vec<super::ids::ProviderEntityId>>,
}

impl ProductLDependentsQueryBuilder {
    /// Sets wire field `masterids`.
    pub fn masterids(mut self, value: Vec<super::ids::ProviderEntityId>) -> Self {
        self.masterids = Some(value);
        self
    }

    /// Validates required fields and builds [`ProductLDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ProductLDependentsQuery, crate::api::current::BuildError> {
        let masterids = self
            .masterids
            .ok_or(crate::api::current::BuildError::missing("masterids"))?;
        if masterids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "masterids",
                "must not be empty",
            ));
        }
        Ok(ProductLDependentsQuery { masterids })
    }
}

impl crate::api::current::support::CurrentQuery for ProductLDependentsQuery {
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
    /// Calls the current `GET /product/ldeps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn product_l_dependents(
        &self,
        query: &ProductLDependentsQuery,
    ) -> Result<Vec<super::users::Product>, crate::Error> {
        self.get_current("/product/ldeps", query).await
    }
}

impl crate::Client {
    /// Calls the current `GET /product/list` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn product_list(&self) -> Result<Vec<super::users::Product>, crate::Error> {
        self.get_without_query("/product/list").await
    }
}

/// Typed query parameters for `/productSession/deps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ProductSessionDependentsQuery {
    #[serde(rename = "masterid")]
    masterid: super::ids::ProviderEntityId,
}

impl ProductSessionDependentsQuery {
    /// Returns wire field `masterid`.
    #[must_use]
    pub fn masterid(&self) -> &super::ids::ProviderEntityId {
        &self.masterid
    }

    /// Starts a builder for [`ProductSessionDependentsQuery`].
    pub fn builder() -> ProductSessionDependentsQueryBuilder {
        ProductSessionDependentsQueryBuilder::default()
    }
}

/// Builder for [`ProductSessionDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ProductSessionDependentsQueryBuilder {
    masterid: Option<super::ids::ProviderEntityId>,
}

impl ProductSessionDependentsQueryBuilder {
    /// Sets wire field `masterid`.
    pub fn masterid(mut self, value: super::ids::ProviderEntityId) -> Self {
        self.masterid = Some(value);
        self
    }

    /// Validates required fields and builds [`ProductSessionDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ProductSessionDependentsQuery, crate::api::current::BuildError> {
        let masterid = self
            .masterid
            .ok_or(crate::api::current::BuildError::missing("masterid"))?;
        Ok(ProductSessionDependentsQuery { masterid })
    }
}

impl crate::api::current::support::CurrentQuery for ProductSessionDependentsQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "masterid", &self.masterid)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /productSession/deps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn product_session_dependents(
        &self,
        query: &ProductSessionDependentsQuery,
    ) -> Result<Vec<ProductSession>, crate::Error> {
        self.get_current("/productSession/deps", query).await
    }
}

/// Typed query parameters for `/productSession/item`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ProductSessionItemQuery {
    #[serde(rename = "id")]
    id: super::ids::ProductSessionId,
}

impl ProductSessionItemQuery {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> &super::ids::ProductSessionId {
        &self.id
    }

    /// Starts a builder for [`ProductSessionItemQuery`].
    pub fn builder() -> ProductSessionItemQueryBuilder {
        ProductSessionItemQueryBuilder::default()
    }
}

/// Builder for [`ProductSessionItemQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ProductSessionItemQueryBuilder {
    id: Option<super::ids::ProductSessionId>,
}

impl ProductSessionItemQueryBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::ProductSessionId) -> Self {
        self.id = Some(value);
        self
    }

    /// Validates required fields and builds [`ProductSessionItemQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ProductSessionItemQuery, crate::api::current::BuildError> {
        let id = self
            .id
            .ok_or(crate::api::current::BuildError::missing("id"))?;
        Ok(ProductSessionItemQuery { id })
    }
}

impl crate::api::current::support::CurrentQuery for ProductSessionItemQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "id", &self.id)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /productSession/item` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn product_session_item(
        &self,
        query: &ProductSessionItemQuery,
    ) -> Result<ProductSession, crate::Error> {
        self.get_current("/productSession/item", query).await
    }
}

/// Typed query parameters for `/productSession/items`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ProductSessionItemsQuery {
    #[serde(rename = "ids")]
    ids: Vec<super::ids::ProductSessionId>,
}

impl ProductSessionItemsQuery {
    /// Returns wire field `ids`.
    #[must_use]
    pub fn ids(&self) -> &[super::ids::ProductSessionId] {
        &self.ids
    }

    /// Starts a builder for [`ProductSessionItemsQuery`].
    pub fn builder() -> ProductSessionItemsQueryBuilder {
        ProductSessionItemsQueryBuilder::default()
    }
}

/// Builder for [`ProductSessionItemsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ProductSessionItemsQueryBuilder {
    ids: Option<Vec<super::ids::ProductSessionId>>,
}

impl ProductSessionItemsQueryBuilder {
    /// Sets wire field `ids`.
    pub fn ids(mut self, value: Vec<super::ids::ProductSessionId>) -> Self {
        self.ids = Some(value);
        self
    }

    /// Validates required fields and builds [`ProductSessionItemsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ProductSessionItemsQuery, crate::api::current::BuildError> {
        let ids = self
            .ids
            .ok_or(crate::api::current::BuildError::missing("ids"))?;
        if ids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "ids",
                "must not be empty",
            ));
        }
        Ok(ProductSessionItemsQuery { ids })
    }
}

impl crate::api::current::support::CurrentQuery for ProductSessionItemsQuery {
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
    /// Calls the current `GET /productSession/items` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn product_session_items(
        &self,
        query: &ProductSessionItemsQuery,
    ) -> Result<Vec<ProductSession>, crate::Error> {
        self.get_current("/productSession/items", query).await
    }
}

/// Typed query parameters for `/productSession/ldeps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ProductSessionLDependentsQuery {
    #[serde(rename = "masterids")]
    masterids: Vec<super::ids::ProviderEntityId>,
}

impl ProductSessionLDependentsQuery {
    /// Returns wire field `masterids`.
    #[must_use]
    pub fn masterids(&self) -> &[super::ids::ProviderEntityId] {
        &self.masterids
    }

    /// Starts a builder for [`ProductSessionLDependentsQuery`].
    pub fn builder() -> ProductSessionLDependentsQueryBuilder {
        ProductSessionLDependentsQueryBuilder::default()
    }
}

/// Builder for [`ProductSessionLDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ProductSessionLDependentsQueryBuilder {
    masterids: Option<Vec<super::ids::ProviderEntityId>>,
}

impl ProductSessionLDependentsQueryBuilder {
    /// Sets wire field `masterids`.
    pub fn masterids(mut self, value: Vec<super::ids::ProviderEntityId>) -> Self {
        self.masterids = Some(value);
        self
    }

    /// Validates required fields and builds [`ProductSessionLDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ProductSessionLDependentsQuery, crate::api::current::BuildError> {
        let masterids = self
            .masterids
            .ok_or(crate::api::current::BuildError::missing("masterids"))?;
        if masterids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "masterids",
                "must not be empty",
            ));
        }
        Ok(ProductSessionLDependentsQuery { masterids })
    }
}

impl crate::api::current::support::CurrentQuery for ProductSessionLDependentsQuery {
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
    /// Calls the current `GET /productSession/ldeps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn product_session_l_dependents(
        &self,
        query: &ProductSessionLDependentsQuery,
    ) -> Result<Vec<ProductSession>, crate::Error> {
        self.get_current("/productSession/ldeps", query).await
    }
}

/// Typed query parameters for `/product/suggest`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ProductSuggestQuery {
    #[serde(rename = "t")]
    t: String,
    #[serde(rename = "l")]
    l: i64,
}

impl ProductSuggestQuery {
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

    /// Starts a builder for [`ProductSuggestQuery`].
    pub fn builder() -> ProductSuggestQueryBuilder {
        ProductSuggestQueryBuilder::default()
    }
}

/// Builder for [`ProductSuggestQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ProductSuggestQueryBuilder {
    t: Option<String>,
    l: Option<i64>,
}

impl ProductSuggestQueryBuilder {
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

    /// Validates required fields and builds [`ProductSuggestQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ProductSuggestQuery, crate::api::current::BuildError> {
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
        Ok(ProductSuggestQuery { t, l })
    }
}

impl crate::api::current::support::CurrentQuery for ProductSuggestQuery {
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
    /// Calls the current `GET /product/suggest` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn product_suggest(
        &self,
        query: &ProductSuggestQuery,
    ) -> Result<Vec<super::users::Product>, crate::Error> {
        self.get_current("/product/suggest", query).await
    }
}

impl crate::Client {
    /// Calls the current `POST /contract/rollcontract` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn contract_roll_contract(
        &self,
        request: &RollContract,
    ) -> Result<RollContractResponse, crate::Error> {
        crate::api::current::support::CurrentRequest::validate_current(request)?;
        self.post_query("/contract/rollcontract", request).await
    }
}

/// Typed query parameters for `/spreadDefinition/item`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct SpreadDefinitionItemQuery {
    #[serde(rename = "id")]
    id: super::ids::SpreadDefinitionId,
}

impl SpreadDefinitionItemQuery {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> &super::ids::SpreadDefinitionId {
        &self.id
    }

    /// Starts a builder for [`SpreadDefinitionItemQuery`].
    pub fn builder() -> SpreadDefinitionItemQueryBuilder {
        SpreadDefinitionItemQueryBuilder::default()
    }
}

/// Builder for [`SpreadDefinitionItemQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct SpreadDefinitionItemQueryBuilder {
    id: Option<super::ids::SpreadDefinitionId>,
}

impl SpreadDefinitionItemQueryBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::SpreadDefinitionId) -> Self {
        self.id = Some(value);
        self
    }

    /// Validates required fields and builds [`SpreadDefinitionItemQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<SpreadDefinitionItemQuery, crate::api::current::BuildError> {
        let id = self
            .id
            .ok_or(crate::api::current::BuildError::missing("id"))?;
        Ok(SpreadDefinitionItemQuery { id })
    }
}

impl crate::api::current::support::CurrentQuery for SpreadDefinitionItemQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "id", &self.id)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /spreadDefinition/item` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn spread_definition_item(
        &self,
        query: &SpreadDefinitionItemQuery,
    ) -> Result<super::users::SpreadDefinition, crate::Error> {
        self.get_current("/spreadDefinition/item", query).await
    }
}

/// Typed query parameters for `/spreadDefinition/items`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct SpreadDefinitionItemsQuery {
    #[serde(rename = "ids")]
    ids: Vec<super::ids::SpreadDefinitionId>,
}

impl SpreadDefinitionItemsQuery {
    /// Returns wire field `ids`.
    #[must_use]
    pub fn ids(&self) -> &[super::ids::SpreadDefinitionId] {
        &self.ids
    }

    /// Starts a builder for [`SpreadDefinitionItemsQuery`].
    pub fn builder() -> SpreadDefinitionItemsQueryBuilder {
        SpreadDefinitionItemsQueryBuilder::default()
    }
}

/// Builder for [`SpreadDefinitionItemsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct SpreadDefinitionItemsQueryBuilder {
    ids: Option<Vec<super::ids::SpreadDefinitionId>>,
}

impl SpreadDefinitionItemsQueryBuilder {
    /// Sets wire field `ids`.
    pub fn ids(mut self, value: Vec<super::ids::SpreadDefinitionId>) -> Self {
        self.ids = Some(value);
        self
    }

    /// Validates required fields and builds [`SpreadDefinitionItemsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<SpreadDefinitionItemsQuery, crate::api::current::BuildError> {
        let ids = self
            .ids
            .ok_or(crate::api::current::BuildError::missing("ids"))?;
        if ids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "ids",
                "must not be empty",
            ));
        }
        Ok(SpreadDefinitionItemsQuery { ids })
    }
}

impl crate::api::current::support::CurrentQuery for SpreadDefinitionItemsQuery {
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
    /// Calls the current `GET /spreadDefinition/items` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn spread_definition_items(
        &self,
        query: &SpreadDefinitionItemsQuery,
    ) -> Result<Vec<super::users::SpreadDefinition>, crate::Error> {
        self.get_current("/spreadDefinition/items", query).await
    }
}

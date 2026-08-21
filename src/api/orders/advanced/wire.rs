// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Private current OCO/OSO wire contracts.

use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use super::{AttachedOrder, PlaceOco, PlaceOso};
use crate::api::orders::failure::{OrderFailureReason, deserialize_optional_non_null};
use crate::api::orders::wire::has_nonempty_text;
use crate::api::orders::{OrderQuantity, OrderSide, OrderType, TimeInForce};
use crate::client::{DocumentedMutationResponse, MutationOutcome};
use crate::{
    AccountId, Decimal, OrderId,
    api::current::ids::{OcoId, Oso1Id, Oso2Id},
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct PlaceOcoWire<'a> {
    account_id: AccountId,
    #[serde(skip_serializing_if = "Option::is_none")]
    cl_ord_id: Option<&'a str>,
    action: OrderSide,
    symbol: &'a str,
    order_qty: OrderQuantity,
    order_type: OrderType,
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "crate::decimal::option"
    )]
    price: Option<Decimal>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "crate::decimal::option"
    )]
    stop_price: Option<Decimal>,
    time_in_force: TimeInForce,
    #[serde(skip_serializing_if = "Option::is_none")]
    expire_time: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    activation_time: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_tag50: Option<&'a str>,
    is_automated: bool,
    other: AttachedOrderWire<'a>,
}

impl<'a> From<&'a PlaceOco> for PlaceOcoWire<'a> {
    fn from(request: &'a PlaceOco) -> Self {
        let (price, stop_price) = advanced_primary_prices(&request.primary);
        Self {
            account_id: request.primary.account_id,
            cl_ord_id: request
                .primary
                .client_order_id
                .as_ref()
                .map(crate::ClientOrderId::as_str),
            action: request.primary.side,
            symbol: request.primary.symbol.as_str(),
            order_qty: request.primary.quantity,
            order_type: request.primary.order_type,
            price,
            stop_price,
            time_in_force: request.primary.time_in_force,
            expire_time: request.primary.expire_time,
            activation_time: request.activation_time,
            custom_tag50: request
                .custom_tag50
                .as_ref()
                .map(super::CustomTag50::as_str),
            is_automated: request.primary.origin.is_automated(),
            other: AttachedOrderWire::from(&request.other),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct PlaceOsoWire<'a> {
    account_id: AccountId,
    #[serde(skip_serializing_if = "Option::is_none")]
    cl_ord_id: Option<&'a str>,
    action: OrderSide,
    symbol: &'a str,
    order_qty: OrderQuantity,
    order_type: OrderType,
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "crate::decimal::option"
    )]
    price: Option<Decimal>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "crate::decimal::option"
    )]
    stop_price: Option<Decimal>,
    time_in_force: TimeInForce,
    #[serde(skip_serializing_if = "Option::is_none")]
    expire_time: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    activation_time: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_tag50: Option<&'a str>,
    is_automated: bool,
    bracket1: AttachedOrderWire<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bracket2: Option<AttachedOrderWire<'a>>,
}

impl<'a> From<&'a PlaceOso> for PlaceOsoWire<'a> {
    fn from(request: &'a PlaceOso) -> Self {
        let (price, stop_price) = advanced_primary_prices(&request.primary);
        Self {
            account_id: request.primary.account_id,
            cl_ord_id: request
                .primary
                .client_order_id
                .as_ref()
                .map(crate::ClientOrderId::as_str),
            action: request.primary.side,
            symbol: request.primary.symbol.as_str(),
            order_qty: request.primary.quantity,
            order_type: request.primary.order_type,
            price,
            stop_price,
            time_in_force: request.primary.time_in_force,
            expire_time: request.primary.expire_time,
            activation_time: request.activation_time,
            custom_tag50: request
                .custom_tag50
                .as_ref()
                .map(super::CustomTag50::as_str),
            is_automated: request.primary.origin.is_automated(),
            bracket1: AttachedOrderWire::from(&request.first_bracket),
            bracket2: request.second_bracket.as_ref().map(AttachedOrderWire::from),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AttachedOrderWire<'a> {
    action: OrderSide,
    #[serde(skip_serializing_if = "Option::is_none")]
    cl_ord_id: Option<&'a str>,
    order_type: OrderType,
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "crate::decimal::option"
    )]
    price: Option<Decimal>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "crate::decimal::option"
    )]
    stop_price: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_in_force: Option<TimeInForce>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expire_time: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<&'a str>,
}

impl<'a> From<&'a AttachedOrder> for AttachedOrderWire<'a> {
    fn from(order: &'a AttachedOrder) -> Self {
        Self {
            action: order.action,
            cl_ord_id: order
                .client_order_id
                .as_ref()
                .map(crate::ClientOrderId::as_str),
            order_type: order.order_type,
            price: order.price,
            stop_price: order.stop_price,
            time_in_force: order.time_in_force,
            expire_time: order.expire_time,
            text: order.text.as_ref().map(super::OrderText::as_str),
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct OcoResponse {
    #[serde(default, deserialize_with = "deserialize_optional_non_null")]
    pub(super) failure_reason: Option<OrderFailureReason>,
    #[serde(default, deserialize_with = "deserialize_optional_non_null")]
    pub(super) failure_text: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_non_null")]
    pub(super) order_id: Option<OrderId>,
    #[serde(default, deserialize_with = "deserialize_optional_non_null")]
    pub(super) oco_id: Option<OcoId>,
}

impl DocumentedMutationResponse for OcoResponse {
    fn mutation_outcome(&self) -> MutationOutcome {
        classify_completion(
            self.failure_reason.as_ref(),
            self.failure_text.as_deref(),
            self.order_id.is_some() && self.oco_id.is_some(),
            self.order_id.is_some() || self.oco_id.is_some(),
        )
    }

    fn has_success_evidence(&self) -> bool {
        self.failure_reason
            .as_ref()
            .is_some_and(OrderFailureReason::is_success)
            || self.order_id.is_some()
            || self.oco_id.is_some()
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct OsoResponse {
    #[serde(default, deserialize_with = "deserialize_optional_non_null")]
    pub(super) failure_reason: Option<OrderFailureReason>,
    #[serde(default, deserialize_with = "deserialize_optional_non_null")]
    pub(super) failure_text: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_non_null")]
    pub(super) order_id: Option<OrderId>,
    #[serde(default, deserialize_with = "deserialize_optional_non_null")]
    pub(super) oso1_id: Option<Oso1Id>,
    #[serde(default, deserialize_with = "deserialize_optional_non_null")]
    pub(super) oso2_id: Option<Oso2Id>,
}

impl DocumentedMutationResponse for OsoResponse {
    fn mutation_outcome(&self) -> MutationOutcome {
        classify_completion(
            self.failure_reason.as_ref(),
            self.failure_text.as_deref(),
            self.order_id.is_some() && self.oso1_id.is_some(),
            self.order_id.is_some() || self.oso1_id.is_some() || self.oso2_id.is_some(),
        )
    }

    fn has_success_evidence(&self) -> bool {
        self.failure_reason
            .as_ref()
            .is_some_and(OrderFailureReason::is_success)
            || self.order_id.is_some()
            || self.oso1_id.is_some()
            || self.oso2_id.is_some()
    }
}

fn advanced_primary_prices(
    order: &crate::api::orders::PlaceOrder,
) -> (Option<Decimal>, Option<Decimal>) {
    if matches!(order.order_type, OrderType::Stop) {
        (order.stop_price, None)
    } else {
        (order.price, order.stop_price)
    }
}

fn classify_completion(
    reason: Option<&OrderFailureReason>,
    failure_text: Option<&str>,
    complete_identifiers: bool,
    any_identifier: bool,
) -> MutationOutcome {
    let has_success_evidence = reason.is_some_and(OrderFailureReason::is_success) || any_identifier;
    if has_nonempty_text(failure_text) && has_success_evidence {
        return MutationOutcome::Ambiguous;
    }
    match (reason, complete_identifiers, any_identifier) {
        (None | Some(OrderFailureReason::Success), true, _) => MutationOutcome::Success,
        (Some(reason), false, false) if reason.is_known_rejection() => MutationOutcome::Rejected,
        _ => MutationOutcome::Ambiguous,
    }
}

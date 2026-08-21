// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Private order wire models.

use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use super::{CancelOrder, OrderQuantity, OrderSide, OrderType, PlaceOrder, TimeInForce};
use crate::api::orders::failure::{OrderFailureReason, deserialize_optional_non_null};
use crate::client::{DocumentedMutationResponse, MutationOutcome};
use crate::{AccountId, ClientOrderId, CommandId, Decimal, Error, OrderId};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct PlaceOrderWire<'a> {
    account_id: AccountId,
    symbol: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    cl_ord_id: Option<&'a str>,
    action: OrderSide,
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
    text: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    activation_time: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_tag50: Option<&'a str>,
    is_automated: bool,
}

impl<'a> From<&'a PlaceOrder> for PlaceOrderWire<'a> {
    fn from(order: &'a PlaceOrder) -> Self {
        // Current Partner place-order documentation (pinned 2026-08-21)
        // explicitly puts a single Stop trigger in `price`. Preserve the
        // compatible builder's `stop_price` input while translating only at
        // the provider boundary.
        let (price, stop_price) = if matches!(order.order_type, OrderType::Stop) {
            (order.stop_price, None)
        } else {
            (order.price, order.stop_price)
        };
        Self {
            account_id: order.account_id,
            symbol: order.symbol.as_str(),
            cl_ord_id: order.client_order_id.as_ref().map(ClientOrderId::as_str),
            action: order.side,
            order_qty: order.quantity,
            order_type: order.order_type,
            price,
            stop_price,
            time_in_force: order.time_in_force,
            expire_time: order.expire_time,
            text: order.text.as_ref().map(super::OrderText::as_str),
            activation_time: order.activation_time,
            custom_tag50: order.custom_tag50.as_ref().map(super::CustomTag50::as_str),
            is_automated: order.origin.is_automated(),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct CancelOrderWire<'a> {
    pub(super) order_id: OrderId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) cl_ord_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) activation_time: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) custom_tag50: Option<&'a str>,
    pub(super) is_automated: bool,
}

impl<'a> From<&'a CancelOrder> for CancelOrderWire<'a> {
    fn from(command: &'a CancelOrder) -> Self {
        Self {
            order_id: command.order_id,
            cl_ord_id: command.client_order_id.as_ref().map(ClientOrderId::as_str),
            activation_time: command.activation_time,
            custom_tag50: command
                .custom_tag50
                .as_ref()
                .map(super::CustomTag50::as_str),
            is_automated: command.origin.is_automated(),
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct PlacementResponse {
    #[serde(default, deserialize_with = "deserialize_optional_non_null")]
    pub(super) failure_reason: Option<OrderFailureReason>,
    #[serde(default, deserialize_with = "deserialize_optional_non_null")]
    pub(super) failure_text: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_non_null")]
    pub(super) order_id: Option<OrderId>,
}

impl DocumentedMutationResponse for PlacementResponse {
    fn mutation_outcome(&self) -> MutationOutcome {
        match classify_outcome(
            self.failure_reason.as_ref(),
            self.failure_text.as_deref(),
            self.order_id,
        ) {
            WireOutcome::Accepted(_) => MutationOutcome::Success,
            WireOutcome::Rejected(_) => MutationOutcome::Rejected,
            WireOutcome::Ambiguous => MutationOutcome::Ambiguous,
        }
    }

    fn has_success_evidence(&self) -> bool {
        self.order_id.is_some()
            || self
                .failure_reason
                .as_ref()
                .is_some_and(OrderFailureReason::is_success)
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct CommandResponse {
    #[serde(default, deserialize_with = "deserialize_optional_non_null")]
    pub(super) failure_reason: Option<OrderFailureReason>,
    #[serde(default, deserialize_with = "deserialize_optional_non_null")]
    pub(super) failure_text: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_non_null")]
    pub(super) command_id: Option<CommandId>,
}

impl DocumentedMutationResponse for CommandResponse {
    fn mutation_outcome(&self) -> MutationOutcome {
        match classify_outcome(
            self.failure_reason.as_ref(),
            self.failure_text.as_deref(),
            self.command_id,
        ) {
            WireOutcome::Accepted(_) => MutationOutcome::Success,
            WireOutcome::Rejected(_) => MutationOutcome::Rejected,
            WireOutcome::Ambiguous => MutationOutcome::Ambiguous,
        }
    }

    fn has_success_evidence(&self) -> bool {
        self.command_id.is_some()
            || self
                .failure_reason
                .as_ref()
                .is_some_and(OrderFailureReason::is_success)
    }
}

pub(super) fn validate_prices(
    order_type: OrderType,
    price: Option<Decimal>,
    stop_price: Option<Decimal>,
) -> Result<(), Error> {
    if price.is_some_and(|value| value <= Decimal::ZERO)
        || stop_price.is_some_and(|value| value <= Decimal::ZERO)
    {
        return Err(Error::InvalidRequest {
            field: "price",
            reason: "prices must be positive",
        });
    }
    let valid = match order_type {
        OrderType::Market => price.is_none() && stop_price.is_none(),
        OrderType::Limit => price.is_some() && stop_price.is_none(),
        OrderType::Stop => price.is_none() && stop_price.is_some(),
        OrderType::StopLimit => price.is_some() && stop_price.is_some(),
    };
    if valid {
        Ok(())
    } else {
        Err(Error::InvalidRequest {
            field: "price",
            reason: "price fields do not match order_type",
        })
    }
}

pub(super) enum WireOutcome<T> {
    Accepted(T),
    Rejected(OrderFailureReason),
    Ambiguous,
}

pub(super) fn classify_outcome<T>(
    reason: Option<&OrderFailureReason>,
    failure_text: Option<&str>,
    id: Option<T>,
) -> WireOutcome<T> {
    let has_success_evidence = reason.is_some_and(OrderFailureReason::is_success) || id.is_some();
    if has_nonempty_text(failure_text) && has_success_evidence {
        return WireOutcome::Ambiguous;
    }
    match (reason, id) {
        (None | Some(OrderFailureReason::Success), Some(id)) => WireOutcome::Accepted(id),
        (Some(reason), None) if reason.is_known_rejection() => {
            WireOutcome::Rejected(reason.clone())
        }
        _ => WireOutcome::Ambiguous,
    }
}

pub(super) fn has_nonempty_text(value: Option<&str>) -> bool {
    value.is_some_and(|value| !value.is_empty())
}

// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Private order wire models.

use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use super::{OrderQuantity, OrderSide, OrderType, PlaceOrder, TimeInForce};
use crate::client::MutationWireResponse;
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
    is_automated: bool,
}

impl<'a> From<&'a PlaceOrder> for PlaceOrderWire<'a> {
    fn from(order: &'a PlaceOrder) -> Self {
        Self {
            account_id: order.account_id,
            symbol: order.symbol.as_str(),
            cl_ord_id: order.client_order_id.as_ref().map(ClientOrderId::as_str),
            action: order.side,
            order_qty: order.quantity,
            order_type: order.order_type,
            price: order.price,
            stop_price: order.stop_price,
            time_in_force: order.time_in_force,
            expire_time: order.expire_time,
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
    pub(super) is_automated: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct PlacementResponse {
    pub(super) failure_reason: Option<String>,
    pub(super) order_id: Option<OrderId>,
}

impl MutationWireResponse for PlacementResponse {
    fn has_success_evidence(&self) -> bool {
        self.order_id.is_some() || self.failure_reason.as_deref() == Some("Success")
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct CommandResponse {
    pub(super) failure_reason: Option<String>,
    pub(super) command_id: Option<CommandId>,
}

impl MutationWireResponse for CommandResponse {
    fn has_success_evidence(&self) -> bool {
        self.command_id.is_some() || self.failure_reason.as_deref() == Some("Success")
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
    Rejected,
    Ambiguous,
}

pub(super) fn classify_outcome<T>(reason: Option<&str>, id: Option<T>) -> WireOutcome<T> {
    let success = reason.is_none_or(|reason| reason == "Success");
    match (success, id) {
        (true, Some(id)) => WireOutcome::Accepted(id),
        (false, None) => WireOutcome::Rejected,
        (true, None) | (false, Some(_)) => WireOutcome::Ambiguous,
    }
}

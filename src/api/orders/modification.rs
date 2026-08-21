// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Validated modification of an existing provider order.

use jiff::Timestamp;
use serde::Serialize;

use super::wire::{CommandResponse, WireOutcome, classify_outcome, validate_prices};
use super::{CustomTag50, OrderOrigin, OrderQuantity, OrderText, OrderType, TimeInForce};
use crate::{Client, ClientOrderId, CommandId, Decimal, Error, OrderId};

const MODIFY_ORDER_ENDPOINT: &str = "/order/modifyorder";

/// A validated replacement request for one explicit provider order.
///
/// The current endpoint does not accept an account selector. The [`OrderId`]
/// is therefore the only route identity Tradovate permits for this command.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifyOrder {
    order_id: OrderId,
    #[serde(skip_serializing_if = "Option::is_none")]
    cl_ord_id: Option<ClientOrderId>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    time_in_force: Option<TimeInForce>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expire_time: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<OrderText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    activation_time: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_tag50: Option<CustomTag50>,
    is_automated: bool,
}

impl ModifyOrder {
    /// Replaces an order with market-order terms.
    #[must_use]
    pub const fn market(order_id: OrderId, quantity: OrderQuantity, origin: OrderOrigin) -> Self {
        Self::from_parts(order_id, quantity, origin, OrderType::Market, None, None)
    }

    /// Replaces an order with limit-order terms.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidRequest`] when `price` is not positive.
    pub fn limit(
        order_id: OrderId,
        quantity: OrderQuantity,
        origin: OrderOrigin,
        price: Decimal,
    ) -> Result<Self, Error> {
        validate_prices(OrderType::Limit, Some(price), None)?;
        Ok(Self::from_parts(
            order_id,
            quantity,
            origin,
            OrderType::Limit,
            Some(price),
            None,
        ))
    }

    const fn from_parts(
        order_id: OrderId,
        order_qty: OrderQuantity,
        origin: OrderOrigin,
        order_type: OrderType,
        price: Option<Decimal>,
        stop_price: Option<Decimal>,
    ) -> Self {
        Self {
            order_id,
            cl_ord_id: None,
            order_qty,
            order_type,
            price,
            stop_price,
            time_in_force: None,
            expire_time: None,
            text: None,
            activation_time: None,
            custom_tag50: None,
            is_automated: origin.is_automated(),
        }
    }

    /// Sets a caller-owned command correlation identifier.
    #[must_use]
    pub fn with_client_order_id(mut self, value: ClientOrderId) -> Self {
        self.cl_ord_id = Some(value);
        self
    }

    /// Sets a non-GTD time-in-force and clears any prior expiration.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidRequest`] for [`TimeInForce::GoodTillDate`]; use
    /// [`Self::good_till_date`] so an expiration is always supplied.
    pub fn with_time_in_force(mut self, value: TimeInForce) -> Result<Self, Error> {
        if matches!(value, TimeInForce::GoodTillDate) {
            return Err(Error::InvalidRequest {
                field: "time_in_force",
                reason: "GTD requires good_till_date with an expiration",
            });
        }
        self.time_in_force = Some(value);
        self.expire_time = None;
        Ok(self)
    }

    /// Sets GTD together with its required expiration.
    #[must_use]
    pub const fn good_till_date(mut self, expiration: Timestamp) -> Self {
        self.time_in_force = Some(TimeInForce::GoodTillDate);
        self.expire_time = Some(expiration);
        self
    }

    /// Adds a bounded operator annotation.
    #[must_use]
    pub fn with_text(mut self, value: OrderText) -> Self {
        self.text = Some(value);
        self
    }

    /// Delays activation until the supplied provider timestamp.
    #[must_use]
    pub const fn with_activation_time(mut self, value: Timestamp) -> Self {
        self.activation_time = Some(value);
        self
    }

    /// Adds a bounded provider correlation tag.
    #[must_use]
    pub fn with_custom_tag(mut self, value: CustomTag50) -> Self {
        self.custom_tag50 = Some(value);
        self
    }
}

impl Client {
    /// Sends one validated order modification without automatic retry.
    ///
    /// Success requires a command identifier and either an absent failure
    /// reason or the current `Success` reason. Any contradiction remains
    /// mutation-ambiguous.
    ///
    /// # Errors
    ///
    /// Returns a typed rejection, transport ambiguity, reconciliation latch,
    /// penalty, authentication, request-bound, encoding, or decoding failure.
    pub async fn modify_order(&self, request: &ModifyOrder) -> Result<CommandId, Error> {
        let response = self
            .post_mutation::<CommandResponse, _>(MODIFY_ORDER_ENDPOINT, request)
            .await?;
        match classify_outcome(
            response.value().failure_reason.as_ref(),
            response.value().failure_text.as_deref(),
            response.value().command_id,
        ) {
            WireOutcome::Accepted(command_id) => {
                response.resolve();
                Ok(command_id)
            }
            WireOutcome::Rejected(reason) => {
                response.resolve();
                Err(Error::OrderRejected {
                    endpoint: MODIFY_ORDER_ENDPOINT,
                    reason,
                })
            }
            WireOutcome::Ambiguous => Err(Error::AmbiguousMutation {
                endpoint: MODIFY_ORDER_ENDPOINT,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modification_requires_gtd_expiration_atomically() {
        let order_id = OrderId::new(11).unwrap_or_else(|error| panic!("{error}"));
        let quantity = OrderQuantity::new(1).unwrap_or_else(|error| panic!("{error}"));
        let request = ModifyOrder::market(order_id, quantity, OrderOrigin::Automated);
        assert!(
            request
                .with_time_in_force(TimeInForce::GoodTillDate)
                .is_err()
        );
    }

    #[test]
    fn modification_always_serializes_explicit_automation_origin() {
        let order_id = OrderId::new(11).unwrap_or_else(|error| panic!("{error}"));
        let quantity = OrderQuantity::new(1).unwrap_or_else(|error| panic!("{error}"));
        let request = ModifyOrder::market(order_id, quantity, OrderOrigin::Manual);
        let encoded = serde_json::to_value(request).unwrap_or_else(|error| panic!("{error}"));
        assert_eq!(
            encoded
                .get("isAutomated")
                .and_then(serde_json::Value::as_bool),
            Some(false)
        );
    }
}

// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Typed OCO and OSO order relationships.

#[path = "advanced/client.rs"]
mod client;
#[path = "advanced/wire.rs"]
mod wire;

use jiff::Timestamp;

use super::{
    ClientOrderId, CustomTag50, Decimal, Error, OrderSide, OrderText, OrderType, PlaceOrder,
    TimeInForce,
};
use crate::{
    OrderId,
    api::current::ids::{OcoId, Oso1Id, Oso2Id},
};

/// The linked order in an OCO or an OSO bracket.
///
/// Quantity is inherited from the parent request because the current
/// `RestrainedOrderVersion` schema exposes no quantity field.
#[derive(Clone, Debug)]
pub struct AttachedOrder {
    action: OrderSide,
    client_order_id: Option<ClientOrderId>,
    order_type: OrderType,
    price: Option<Decimal>,
    stop_price: Option<Decimal>,
    time_in_force: Option<TimeInForce>,
    expire_time: Option<Timestamp>,
    text: Option<OrderText>,
}

impl AttachedOrder {
    /// Creates an attached market order.
    #[must_use]
    pub const fn market(action: OrderSide) -> Self {
        Self::from_parts(action, OrderType::Market, None, None)
    }

    /// Creates an attached limit order.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidRequest`] when `price` is not positive.
    pub fn limit(action: OrderSide, price: Decimal) -> Result<Self, Error> {
        validate_advanced_prices(OrderType::Limit, Some(price), None)?;
        Ok(Self::from_parts(
            action,
            OrderType::Limit,
            Some(price),
            None,
        ))
    }

    /// Creates an attached stop order.
    ///
    /// The current Partner OCO/OSO pages explicitly encode a stop trigger in
    /// the wire `price` field, unlike the standalone order schema's
    /// `stopPrice` convention.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidRequest`] when `trigger_price` is not positive.
    pub fn stop(action: OrderSide, trigger_price: Decimal) -> Result<Self, Error> {
        validate_advanced_prices(OrderType::Stop, Some(trigger_price), None)?;
        Ok(Self::from_parts(
            action,
            OrderType::Stop,
            Some(trigger_price),
            None,
        ))
    }

    /// Creates an attached stop-limit order.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidRequest`] when either exact price is not
    /// positive.
    pub fn stop_limit(
        action: OrderSide,
        limit_price: Decimal,
        stop_price: Decimal,
    ) -> Result<Self, Error> {
        validate_advanced_prices(OrderType::StopLimit, Some(limit_price), Some(stop_price))?;
        Ok(Self::from_parts(
            action,
            OrderType::StopLimit,
            Some(limit_price),
            Some(stop_price),
        ))
    }

    const fn from_parts(
        action: OrderSide,
        order_type: OrderType,
        price: Option<Decimal>,
        stop_price: Option<Decimal>,
    ) -> Self {
        Self {
            action,
            client_order_id: None,
            order_type,
            price,
            stop_price,
            time_in_force: None,
            expire_time: None,
            text: None,
        }
    }

    /// Sets a caller-owned correlation identifier for this attached order.
    #[must_use]
    pub fn with_client_order_id(mut self, value: ClientOrderId) -> Self {
        self.client_order_id = Some(value);
        self
    }

    /// Sets a non-GTD time-in-force.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidRequest`] for [`TimeInForce::GoodTillDate`]; use
    /// [`Self::good_till_date`] to supply the required expiration atomically.
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
}

/// A validated order-cancels-order request.
#[derive(Clone, Debug)]
pub struct PlaceOco {
    primary: PlaceOrder,
    other: AttachedOrder,
    activation_time: Option<Timestamp>,
    custom_tag50: Option<CustomTag50>,
}

impl PlaceOco {
    /// Links a fully validated, explicitly routed parent order to one attached
    /// order of the same quantity.
    #[must_use]
    pub const fn new(primary: PlaceOrder, other: AttachedOrder) -> Self {
        Self {
            primary,
            other,
            activation_time: None,
            custom_tag50: None,
        }
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

/// A validated order-sends-order request with one required and one optional
/// attached bracket.
#[derive(Clone, Debug)]
pub struct PlaceOso {
    primary: PlaceOrder,
    first_bracket: AttachedOrder,
    second_bracket: Option<AttachedOrder>,
    activation_time: Option<Timestamp>,
    custom_tag50: Option<CustomTag50>,
}

impl PlaceOso {
    /// Links a fully validated, explicitly routed parent order to its first
    /// attached bracket using combinations established by the current OSO
    /// documentation.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidRequest`] for `Stop` or `StopLimit` terms because
    /// the pinned OSO contract documents only Market and Limit combinations.
    pub fn new(primary: PlaceOrder, first_bracket: AttachedOrder) -> Result<Self, Error> {
        validate_oso_type(primary.order_type, "order_type")?;
        validate_oso_type(first_bracket.order_type, "bracket1.order_type")?;
        Ok(Self {
            primary,
            first_bracket,
            second_bracket: None,
            activation_time: None,
            custom_tag50: None,
        })
    }

    /// Adds a second bracket. Tradovate links the two brackets as an OCO.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidRequest`] for `Stop` or `StopLimit` terms because
    /// their OSO bracket field grammar is absent from the pinned contract.
    pub fn with_second_bracket(mut self, value: AttachedOrder) -> Result<Self, Error> {
        validate_oso_type(value.order_type, "bracket2.order_type")?;
        self.second_bracket = Some(value);
        Ok(self)
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

    pub(super) const fn has_second_bracket(&self) -> bool {
        self.second_bracket.is_some()
    }
}

fn validate_oso_type(order_type: OrderType, field: &'static str) -> Result<(), Error> {
    if matches!(order_type, OrderType::Market | OrderType::Limit) {
        Ok(())
    } else {
        Err(Error::InvalidRequest {
            field,
            reason: "current OSO documentation does not define Stop or StopLimit field grammar",
        })
    }
}

/// Provider acceptance of both linked OCO orders.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OcoPlacement {
    order_id: OrderId,
    oco_id: OcoId,
}

impl OcoPlacement {
    /// Returns the accepted parent order identifier.
    #[must_use]
    pub const fn order_id(self) -> OrderId {
        self.order_id
    }

    /// Returns the provider OCO relationship identifier.
    #[must_use]
    pub const fn oco_id(self) -> OcoId {
        self.oco_id
    }
}

/// Provider acceptance of an OSO parent and its attached brackets.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OsoPlacement {
    parent: OrderId,
    first_bracket: Oso1Id,
    second_bracket: Option<Oso2Id>,
}

impl OsoPlacement {
    /// Returns the accepted parent order identifier.
    #[must_use]
    pub const fn order_id(self) -> OrderId {
        self.parent
    }

    /// Returns the first attached bracket identifier.
    #[must_use]
    pub const fn first_bracket_id(self) -> Oso1Id {
        self.first_bracket
    }

    /// Returns the second bracket identifier when one was requested.
    #[must_use]
    pub const fn second_bracket_id(self) -> Option<Oso2Id> {
        self.second_bracket
    }
}

fn validate_advanced_prices(
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
        OrderType::Limit | OrderType::Stop => price.is_some() && stop_price.is_none(),
        OrderType::StopLimit => price.is_some() && stop_price.is_some(),
    };
    if valid {
        Ok(())
    } else {
        Err(Error::InvalidRequest {
            field: "price",
            reason: "price fields do not match the current OCO/OSO order type",
        })
    }
}

#[cfg(test)]
#[path = "advanced/tests.rs"]
mod tests;

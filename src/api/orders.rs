// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Validated order commands and order queries.

#[path = "orders/models.rs"]
mod models;
#[path = "orders/wire.rs"]
mod wire;

use jiff::Timestamp;
use serde::{Deserialize, Deserializer, Serialize, de};

use crate::{AccountId, Client, ClientOrderId, CommandId, Decimal, Error, OrderId, Symbol};
use wire::{
    CancelOrderWire, CommandResponse, PlaceOrderWire, PlacementResponse, WireOutcome,
    classify_outcome, validate_prices,
};

pub use models::{Order, OrderStatus};

const LIST_ORDERS: &str = "/order/list";
const PLACE_ORDER: &str = "/order/placeorder";
const CANCEL_ORDER: &str = "/order/cancelorder";

/// Positive integral Tradovate futures order quantity.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(transparent)]
pub struct OrderQuantity(u32);

impl OrderQuantity {
    /// Creates a quantity accepted by Tradovate's signed 32-bit wire field.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidRequest`] for zero or values above `i32::MAX`.
    pub const fn new(value: u32) -> Result<Self, Error> {
        if value == 0 {
            Err(Error::InvalidRequest {
                field: "order_qty",
                reason: "must be positive",
            })
        } else if value > i32::MAX as u32 {
            Err(Error::InvalidRequest {
                field: "order_qty",
                reason: "exceeds Tradovate's signed 32-bit wire limit",
            })
        } else {
            Ok(Self(value))
        }
    }

    /// Returns the integral number of contracts.
    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }
}

impl<'de> Deserialize<'de> for OrderQuantity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        Self::new(value).map_err(de::Error::custom)
    }
}

/// Buy or sell direction.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
pub enum OrderSide {
    /// Buy.
    Buy,
    /// Sell.
    Sell,
}

/// Supported high-level order type.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[non_exhaustive]
pub enum OrderType {
    /// Market order.
    Market,
    /// Limit order.
    Limit,
    /// Stop-market order.
    Stop,
    /// Stop-limit order.
    StopLimit,
}

/// Provider time-in-force instruction.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[non_exhaustive]
pub enum TimeInForce {
    /// Day order.
    Day,
    /// Fill or kill.
    #[serde(rename = "FOK")]
    FillOrKill,
    /// Good until cancelled.
    #[serde(rename = "GTC")]
    GoodTillCancelled,
    /// Good until the specified expiration time.
    #[serde(rename = "GTD")]
    GoodTillDate,
    /// Immediate or cancel.
    #[serde(rename = "IOC")]
    ImmediateOrCancel,
}

/// Explicit origin required by Tradovate automated-trading guidance.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OrderOrigin {
    /// Automated order; serializes `isAutomated: true`.
    Automated,
    /// Manually initiated order; serializes `isAutomated: false`.
    Manual,
}

impl OrderOrigin {
    const fn is_automated(self) -> bool {
        matches!(self, Self::Automated)
    }
}

/// Validated order-placement request.
#[derive(Clone, Debug)]
pub struct PlaceOrder {
    account_id: AccountId,
    symbol: Symbol,
    client_order_id: Option<ClientOrderId>,
    side: OrderSide,
    quantity: OrderQuantity,
    order_type: OrderType,
    price: Option<Decimal>,
    stop_price: Option<Decimal>,
    time_in_force: TimeInForce,
    expire_time: Option<Timestamp>,
    origin: OrderOrigin,
}

impl PlaceOrder {
    /// Starts a builder with route, instrument, side, quantity, and origin.
    pub fn builder(
        account_id: AccountId,
        symbol: Symbol,
        side: OrderSide,
        quantity: OrderQuantity,
        origin: OrderOrigin,
    ) -> PlaceOrderBuilder {
        PlaceOrderBuilder {
            account_id,
            symbol,
            client_order_id: None,
            side,
            quantity,
            order_type: None,
            price: None,
            stop_price: None,
            time_in_force: TimeInForce::Day,
            expire_time: None,
            origin,
        }
    }
}

/// Builder that prevents invalid price/order-type combinations.
#[must_use = "an order builder does nothing until build is called"]
#[derive(Clone, Debug)]
pub struct PlaceOrderBuilder {
    account_id: AccountId,
    symbol: Symbol,
    client_order_id: Option<ClientOrderId>,
    side: OrderSide,
    quantity: OrderQuantity,
    order_type: Option<OrderType>,
    price: Option<Decimal>,
    stop_price: Option<Decimal>,
    time_in_force: TimeInForce,
    expire_time: Option<Timestamp>,
    origin: OrderOrigin,
}

impl PlaceOrderBuilder {
    /// Sets the required order type.
    pub const fn order_type(mut self, value: OrderType) -> Self {
        self.order_type = Some(value);
        self
    }

    /// Sets a caller-owned idempotency/correlation identifier.
    pub fn client_order_id(mut self, value: ClientOrderId) -> Self {
        self.client_order_id = Some(value);
        self
    }

    /// Sets the limit price.
    pub const fn price(mut self, value: Decimal) -> Self {
        self.price = Some(value);
        self
    }

    /// Sets the stop trigger price.
    pub const fn stop_price(mut self, value: Decimal) -> Self {
        self.stop_price = Some(value);
        self
    }

    /// Sets time in force.
    pub const fn time_in_force(mut self, value: TimeInForce) -> Self {
        self.time_in_force = value;
        self
    }

    /// Sets a good-till-date expiration time.
    pub const fn expire_time(mut self, value: Timestamp) -> Self {
        self.expire_time = Some(value);
        self
    }

    /// Validates and constructs the placement request.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidRequest`] for a missing order type, non-positive
    /// price, inconsistent price fields, or inconsistent GTD expiration.
    pub fn build(self) -> Result<PlaceOrder, Error> {
        let order_type = self.order_type.ok_or(Error::InvalidRequest {
            field: "order_type",
            reason: "is required",
        })?;
        validate_prices(order_type, self.price, self.stop_price)?;
        let is_gtd = matches!(self.time_in_force, TimeInForce::GoodTillDate);
        if is_gtd != self.expire_time.is_some() {
            return Err(Error::InvalidRequest {
                field: "expire_time",
                reason: "must be present exactly when time_in_force is GTD",
            });
        }
        Ok(PlaceOrder {
            account_id: self.account_id,
            symbol: self.symbol,
            client_order_id: self.client_order_id,
            side: self.side,
            quantity: self.quantity,
            order_type,
            price: self.price,
            stop_price: self.stop_price,
            time_in_force: self.time_in_force,
            expire_time: self.expire_time,
            origin: self.origin,
        })
    }
}

/// Validated order-cancellation request.
#[derive(Clone, Debug)]
pub struct CancelOrder {
    order_id: OrderId,
    client_order_id: Option<ClientOrderId>,
    origin: OrderOrigin,
}

impl CancelOrder {
    /// Creates a cancellation for an explicit provider order.
    #[must_use]
    pub const fn new(order_id: OrderId, origin: OrderOrigin) -> Self {
        Self {
            order_id,
            client_order_id: None,
            origin,
        }
    }

    /// Sets a caller-owned command correlation identifier.
    #[must_use]
    pub fn with_client_order_id(mut self, value: ClientOrderId) -> Self {
        self.client_order_id = Some(value);
        self
    }
}

/// Provider acceptance of a placement command; this is not a fill.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OrderPlacement {
    order_id: OrderId,
}

impl OrderPlacement {
    /// Returns the accepted provider order identifier.
    #[must_use]
    pub const fn order_id(self) -> OrderId {
        self.order_id
    }
}

impl Client {
    /// Lists orders visible to the authenticated user.
    ///
    /// # Errors
    ///
    /// Returns a typed authentication, transport, provider, bound, or decoding
    /// failure.
    pub async fn list_orders(&self) -> Result<Vec<Order>, Error> {
        self.get_without_query(LIST_ORDERS).await
    }

    /// Places one validated order without automatic retry.
    ///
    /// A transport loss becomes [`Error::AmbiguousMutation`]. Callers must
    /// reconcile by `ClientOrderId`, order state, and user synchronization.
    ///
    /// # Errors
    ///
    /// Returns a typed provider rejection, transport ambiguity, penalty,
    /// authentication, bound, encoding, or decoding failure.
    pub async fn place_order(&self, order: &PlaceOrder) -> Result<OrderPlacement, Error> {
        let response = self
            .post_mutation::<PlacementResponse, _>(PLACE_ORDER, &PlaceOrderWire::from(order))
            .await?;
        match classify_outcome(
            response.value().failure_reason.as_deref(),
            response.value().order_id,
        ) {
            WireOutcome::Accepted(order_id) => {
                response.resolve();
                Ok(OrderPlacement { order_id })
            }
            WireOutcome::Rejected => {
                response.resolve();
                Err(Error::Business {
                    endpoint: PLACE_ORDER,
                })
            }
            WireOutcome::Ambiguous => Err(Error::AmbiguousMutation {
                endpoint: PLACE_ORDER,
            }),
        }
    }

    /// Cancels one explicit order without automatic retry.
    ///
    /// # Errors
    ///
    /// Returns a typed provider rejection, transport ambiguity, penalty,
    /// authentication, bound, encoding, or decoding failure.
    pub async fn cancel_order(&self, command: &CancelOrder) -> Result<CommandId, Error> {
        let body = CancelOrderWire {
            order_id: command.order_id,
            cl_ord_id: command.client_order_id.as_ref().map(ClientOrderId::as_str),
            is_automated: command.origin.is_automated(),
        };
        let response = self
            .post_mutation::<CommandResponse, _>(CANCEL_ORDER, &body)
            .await?;
        match classify_outcome(
            response.value().failure_reason.as_deref(),
            response.value().command_id,
        ) {
            WireOutcome::Accepted(command_id) => {
                response.resolve();
                Ok(command_id)
            }
            WireOutcome::Rejected => {
                response.resolve();
                Err(Error::Business {
                    endpoint: CANCEL_ORDER,
                })
            }
            WireOutcome::Ambiguous => Err(Error::AmbiguousMutation {
                endpoint: CANCEL_ORDER,
            }),
        }
    }
}

#[cfg(test)]
#[path = "orders/tests.rs"]
mod tests;

// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Core order query, placement, and cancellation methods.

use super::wire::{
    CancelOrderWire, CommandResponse, PlaceOrderWire, PlacementResponse, WireOutcome,
    classify_outcome,
};
use super::{CancelOrder, OrderPlacement, OrderType, PlaceOrder};
use crate::{Client, CommandId, Error};

const PLACE_ORDER: &str = "/order/placeorder";
const CANCEL_ORDER: &str = "/order/cancelorder";

impl Client {
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
        validate_standalone_type(order.order_type)?;
        let response = self
            .post_mutation::<PlacementResponse, _>(PLACE_ORDER, &PlaceOrderWire::from(order))
            .await?;
        match classify_outcome(
            response.value().failure_reason.as_ref(),
            response.value().failure_text.as_deref(),
            response.value().order_id,
        ) {
            WireOutcome::Accepted(order_id) => {
                response.resolve();
                Ok(OrderPlacement { order_id })
            }
            WireOutcome::Rejected(reason) => {
                response.resolve();
                Err(Error::OrderRejected {
                    endpoint: PLACE_ORDER,
                    reason,
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
        let body = CancelOrderWire::from(command);
        let response = self
            .post_mutation::<CommandResponse, _>(CANCEL_ORDER, &body)
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
                    endpoint: CANCEL_ORDER,
                    reason,
                })
            }
            WireOutcome::Ambiguous => Err(Error::AmbiguousMutation {
                endpoint: CANCEL_ORDER,
            }),
        }
    }
}

fn validate_standalone_type(order_type: OrderType) -> Result<(), Error> {
    if matches!(order_type, OrderType::StopLimit) {
        Err(Error::InvalidRequest {
            field: "order_type",
            reason: "current standalone StopLimit field grammar is not documented",
        })
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standalone_type_matrix_withholds_only_undocumented_stop_limit() {
        assert!(validate_standalone_type(OrderType::Market).is_ok());
        assert!(validate_standalone_type(OrderType::Limit).is_ok());
        assert!(validate_standalone_type(OrderType::Stop).is_ok());
        assert!(validate_standalone_type(OrderType::StopLimit).is_err());
    }
}

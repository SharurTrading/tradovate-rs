// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Explicit position-liquidation commands.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use super::wire::{PlacementResponse, WireOutcome, classify_outcome};
use super::{CustomTag50, OrderOrigin, OrderPlacement};
use crate::api::orders::failure::deserialize_optional_non_null;
use crate::client::{DocumentedMutationResponse, MutationOutcome};
use crate::{AccountId, Client, ContractId, Error, PositionId};

const LIQUIDATE_POSITION_ENDPOINT: &str = "/order/liquidateposition";
const LIQUIDATE_POSITIONS_ENDPOINT: &str = "/order/liquidatepositions";
const MAX_LIQUIDATION_POSITIONS: usize = 100;

/// Whether Tradovate should execute a liquidation as an administrative action.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LiquidationAuthority {
    /// Normal account-holder action (`admin: false`).
    AccountHolder,
    /// Explicit administrative action (`admin: true`).
    Administrator,
}

impl LiquidationAuthority {
    const fn is_admin(self) -> bool {
        matches!(self, Self::Administrator)
    }
}

/// Cancels open orders for one explicit account/contract route and requests
/// that Tradovate close the corresponding position.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LiquidatePosition {
    account_id: AccountId,
    contract_id: ContractId,
    admin: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_tag50: Option<CustomTag50>,
    is_automated: bool,
}

impl LiquidatePosition {
    /// Creates a liquidation with explicit route, authority, and origin.
    #[must_use]
    pub const fn new(
        account_id: AccountId,
        contract_id: ContractId,
        authority: LiquidationAuthority,
        origin: OrderOrigin,
    ) -> Self {
        Self {
            account_id,
            contract_id,
            admin: authority.is_admin(),
            custom_tag50: None,
            is_automated: origin.is_automated(),
        }
    }

    /// Adds a bounded provider correlation tag.
    #[must_use]
    pub fn with_custom_tag(mut self, value: CustomTag50) -> Self {
        self.custom_tag50 = Some(value);
        self
    }
}

/// A bounded, duplicate-free batch of explicit provider positions to close.
///
/// The current batch endpoint exposes no account field, so this type cannot
/// claim account routing the provider does not accept. Each validated
/// [`PositionId`] is the endpoint's documented identity.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LiquidatePositions {
    positions: Box<[PositionId]>,
    admin: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_tag50: Option<CustomTag50>,
    is_automated: bool,
}

impl LiquidatePositions {
    /// Validates and owns a batch liquidation.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidRequest`] for an empty, duplicate-containing,
    /// or larger-than-100 position batch.
    pub fn new(
        positions: Vec<PositionId>,
        authority: LiquidationAuthority,
        origin: OrderOrigin,
    ) -> Result<Self, Error> {
        if positions.is_empty() || positions.len() > MAX_LIQUIDATION_POSITIONS {
            return Err(Error::InvalidRequest {
                field: "positions",
                reason: "must contain between one and 100 position identifiers",
            });
        }
        let unique = positions.iter().copied().collect::<BTreeSet<_>>();
        if unique.len() != positions.len() {
            return Err(Error::InvalidRequest {
                field: "positions",
                reason: "must not contain duplicate position identifiers",
            });
        }
        Ok(Self {
            positions: positions.into_boxed_slice(),
            admin: authority.is_admin(),
            custom_tag50: None,
            is_automated: origin.is_automated(),
        })
    }

    /// Returns the exact provider positions selected by the caller.
    #[must_use]
    pub fn positions(&self) -> &[PositionId] {
        &self.positions
    }

    /// Adds a bounded provider correlation tag.
    #[must_use]
    pub fn with_custom_tag(mut self, value: CustomTag50) -> Self {
        self.custom_tag50 = Some(value);
        self
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SimpleResponse {
    #[serde(default, deserialize_with = "deserialize_optional_non_null")]
    error_text: Option<String>,
    ok: bool,
}

impl DocumentedMutationResponse for SimpleResponse {
    fn mutation_outcome(&self) -> MutationOutcome {
        match (self.ok, has_error_text(self.error_text.as_deref())) {
            (true, false) => MutationOutcome::Success,
            (false, _) => MutationOutcome::Rejected,
            (true, true) => MutationOutcome::Ambiguous,
        }
    }

    fn has_success_evidence(&self) -> bool {
        self.ok
    }
}

impl Client {
    /// Requests cancellation and closure for one explicit account position,
    /// without automatic retry.
    ///
    /// # Errors
    ///
    /// Returns a typed rejection, ambiguity, reconciliation latch, penalty,
    /// authentication, request-bound, encoding, or decoding failure.
    pub async fn liquidate_position(
        &self,
        request: &LiquidatePosition,
    ) -> Result<OrderPlacement, Error> {
        let response = self
            .post_mutation::<PlacementResponse, _>(LIQUIDATE_POSITION_ENDPOINT, request)
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
                    endpoint: LIQUIDATE_POSITION_ENDPOINT,
                    reason,
                })
            }
            WireOutcome::Ambiguous => Err(Error::AmbiguousMutation {
                endpoint: LIQUIDATE_POSITION_ENDPOINT,
            }),
        }
    }

    /// Requests a bounded batch liquidation without automatic retry.
    ///
    /// # Errors
    ///
    /// Returns a typed rejection, ambiguity, reconciliation latch, penalty,
    /// authentication, request-bound, encoding, or decoding failure.
    pub async fn liquidate_positions(&self, request: &LiquidatePositions) -> Result<(), Error> {
        let response = self
            .post_mutation::<SimpleResponse, _>(LIQUIDATE_POSITIONS_ENDPOINT, request)
            .await?;
        match response.value().mutation_outcome() {
            MutationOutcome::Success => {
                response.resolve();
                Ok(())
            }
            MutationOutcome::Rejected => {
                response.resolve();
                Err(Error::Business {
                    endpoint: LIQUIDATE_POSITIONS_ENDPOINT,
                })
            }
            MutationOutcome::Ambiguous => Err(Error::AmbiguousMutation {
                endpoint: LIQUIDATE_POSITIONS_ENDPOINT,
            }),
        }
    }
}

fn has_error_text(value: Option<&str>) -> bool {
    value.is_some_and(|value| !value.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn batch_rejects_duplicate_position_routes() {
        let position = PositionId::new(7).unwrap_or_else(|error| panic!("{error}"));
        let result = LiquidatePositions::new(
            vec![position, position],
            LiquidationAuthority::AccountHolder,
            OrderOrigin::Automated,
        );
        assert!(result.is_err());
    }

    #[test]
    fn simple_response_contradiction_is_ambiguous() {
        let response = SimpleResponse {
            error_text: Some("rejected".to_owned()),
            ok: true,
        };
        assert_eq!(response.mutation_outcome(), MutationOutcome::Ambiguous);
        assert!(response.has_success_evidence());
    }

    #[test]
    fn null_simple_response_control_is_malformed() {
        assert!(serde_json::from_str::<SimpleResponse>(r#"{"errorText":null,"ok":true}"#).is_err());
    }
}

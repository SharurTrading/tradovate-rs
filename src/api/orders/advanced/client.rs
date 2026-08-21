// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! OCO and OSO mutation methods.

use super::wire::{OcoResponse, OsoResponse, PlaceOcoWire, PlaceOsoWire};
use super::{OcoPlacement, OsoPlacement, PlaceOco, PlaceOso};
use crate::client::{DocumentedMutationResponse, MutationOutcome};
use crate::{Client, Error};

const PLACE_OCO_ENDPOINT: &str = "/order/placeoco";
const PLACE_OSO_ENDPOINT: &str = "/order/placeoso";

impl Client {
    /// Places a validated OCO relationship without automatic retry.
    ///
    /// Success requires the parent [`crate::OrderId`] and OCO relationship
    /// identifier, with either an absent failure reason or the current
    /// `Success` reason. Partial or contradictory evidence remains ambiguous
    /// and latches reconciliation.
    ///
    /// # Errors
    ///
    /// Returns a typed rejection, ambiguity, reconciliation latch, penalty,
    /// authentication, request-bound, encoding, or decoding failure.
    pub async fn place_oco(&self, request: &PlaceOco) -> Result<OcoPlacement, Error> {
        let response = self
            .post_mutation::<OcoResponse, _>(PLACE_OCO_ENDPOINT, &PlaceOcoWire::from(request))
            .await?;
        match response.value().mutation_outcome() {
            MutationOutcome::Success => {
                let value = response.value();
                let (Some(order_id), Some(oco_id)) = (value.order_id, value.oco_id) else {
                    return Err(Error::AmbiguousMutation {
                        endpoint: PLACE_OCO_ENDPOINT,
                    });
                };
                response.resolve();
                Ok(OcoPlacement { order_id, oco_id })
            }
            MutationOutcome::Rejected => {
                let Some(reason) = response
                    .value()
                    .failure_reason
                    .as_ref()
                    .filter(|reason| reason.is_known_rejection())
                    .cloned()
                else {
                    return Err(Error::AmbiguousMutation {
                        endpoint: PLACE_OCO_ENDPOINT,
                    });
                };
                response.resolve();
                Err(Error::OrderRejected {
                    endpoint: PLACE_OCO_ENDPOINT,
                    reason,
                })
            }
            MutationOutcome::Ambiguous => Err(Error::AmbiguousMutation {
                endpoint: PLACE_OCO_ENDPOINT,
            }),
        }
    }

    /// Places a validated OSO relationship without automatic retry.
    ///
    /// Success requires parent and first-bracket identifiers, plus a
    /// second-bracket identifier exactly when the request included a second
    /// bracket, with either an absent failure reason or `Success`.
    ///
    /// # Errors
    ///
    /// Returns a typed rejection, ambiguity, reconciliation latch, penalty,
    /// authentication, request-bound, encoding, or decoding failure.
    pub async fn place_oso(&self, request: &PlaceOso) -> Result<OsoPlacement, Error> {
        let response = self
            .post_mutation::<OsoResponse, _>(PLACE_OSO_ENDPOINT, &PlaceOsoWire::from(request))
            .await?;
        match response.value().mutation_outcome() {
            MutationOutcome::Success => {
                let value = response.value();
                let bracket_count_matches = request.has_second_bracket() == value.oso2_id.is_some();
                let (Some(order_id), Some(first_bracket_id), true) =
                    (value.order_id, value.oso1_id, bracket_count_matches)
                else {
                    return Err(Error::AmbiguousMutation {
                        endpoint: PLACE_OSO_ENDPOINT,
                    });
                };
                let second_bracket_id = value.oso2_id;
                response.resolve();
                Ok(OsoPlacement {
                    parent: order_id,
                    first_bracket: first_bracket_id,
                    second_bracket: second_bracket_id,
                })
            }
            MutationOutcome::Rejected => {
                let Some(reason) = response
                    .value()
                    .failure_reason
                    .as_ref()
                    .filter(|reason| reason.is_known_rejection())
                    .cloned()
                else {
                    return Err(Error::AmbiguousMutation {
                        endpoint: PLACE_OSO_ENDPOINT,
                    });
                };
                response.resolve();
                Err(Error::OrderRejected {
                    endpoint: PLACE_OSO_ENDPOINT,
                    reason,
                })
            }
            MutationOutcome::Ambiguous => Err(Error::AmbiguousMutation {
                endpoint: PLACE_OSO_ENDPOINT,
            }),
        }
    }
}

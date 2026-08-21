// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Multi-bracket strategy mutation methods.

use super::{
    INTERRUPT_STRATEGY_ENDPOINT, InterruptStrategy, MutationOutcome, OrderStrategyId,
    OrderStrategyReceipt, START_STRATEGY_ENDPOINT, StartMultiBracketStrategy, StartStrategyWire,
    StrategyResponse, receipt_from_strategy, start_identity_matches,
};
use crate::{Client, Error, client::DocumentedMutationResponse};

impl Client {
    /// Starts the current documented type-2 multi-bracket strategy without
    /// automatic retry.
    ///
    /// The nested `params` wire string is produced only from bounded typed
    /// values; callers cannot inject raw JSON.
    ///
    /// # Errors
    ///
    /// Returns a typed rejection, ambiguity, reconciliation latch, penalty,
    /// authentication, request-bound, encoding, or decoding failure.
    pub async fn start_order_strategy(
        &self,
        request: &StartMultiBracketStrategy,
    ) -> Result<OrderStrategyReceipt, Error> {
        let body = StartStrategyWire::new(request)?;
        let response = self
            .post_mutation::<StrategyResponse, _>(START_STRATEGY_ENDPOINT, &body)
            .await?;
        match response.value().mutation_outcome() {
            MutationOutcome::Success => {
                let receipt = response
                    .value()
                    .order_strategy
                    .as_ref()
                    .filter(|strategy| start_identity_matches(request, strategy))
                    .and_then(receipt_from_strategy);
                if let Some(receipt) = receipt {
                    response.resolve();
                    Ok(receipt)
                } else {
                    Err(Error::AmbiguousMutation {
                        endpoint: START_STRATEGY_ENDPOINT,
                    })
                }
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
                        endpoint: START_STRATEGY_ENDPOINT,
                    });
                };
                response.resolve();
                Err(Error::OrderRejected {
                    endpoint: START_STRATEGY_ENDPOINT,
                    reason,
                })
            }
            MutationOutcome::Ambiguous => Err(Error::AmbiguousMutation {
                endpoint: START_STRATEGY_ENDPOINT,
            }),
        }
    }

    /// Interrupts one explicit provider strategy without automatic retry.
    ///
    /// # Errors
    ///
    /// Returns a typed rejection, ambiguity, reconciliation latch, penalty,
    /// authentication, request-bound, encoding, or decoding failure.
    pub async fn interrupt_order_strategy(
        &self,
        order_strategy_id: OrderStrategyId,
    ) -> Result<OrderStrategyReceipt, Error> {
        let response = self
            .post_mutation::<StrategyResponse, _>(
                INTERRUPT_STRATEGY_ENDPOINT,
                &InterruptStrategy { order_strategy_id },
            )
            .await?;
        match response.value().mutation_outcome() {
            MutationOutcome::Success => {
                let receipt = response
                    .value()
                    .order_strategy
                    .as_ref()
                    .and_then(receipt_from_strategy)
                    .filter(|receipt| receipt.id == order_strategy_id);
                if let Some(receipt) = receipt {
                    response.resolve();
                    Ok(receipt)
                } else {
                    Err(Error::AmbiguousMutation {
                        endpoint: INTERRUPT_STRATEGY_ENDPOINT,
                    })
                }
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
                        endpoint: INTERRUPT_STRATEGY_ENDPOINT,
                    });
                };
                response.resolve();
                Err(Error::OrderRejected {
                    endpoint: INTERRUPT_STRATEGY_ENDPOINT,
                    reason,
                })
            }
            MutationOutcome::Ambiguous => Err(Error::AmbiguousMutation {
                endpoint: INTERRUPT_STRATEGY_ENDPOINT,
            }),
        }
    }
}

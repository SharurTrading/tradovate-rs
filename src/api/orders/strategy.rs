// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Current typed multi-bracket strategy commands.

#[path = "strategy/client.rs"]
mod client;

use serde::{Deserialize, Serialize};

use super::documentation::CurrentDocumentationGap;
use super::{CustomTag50, OrderOrigin, OrderQuantity, OrderSide, StrategyInstanceId};
use crate::api::current::ids::{OrderStrategyId, OrderStrategyTypeId};
pub use crate::api::current::users::OrderStrategyStatus;
use crate::api::current::users::{OrderStrategy as CurrentOrderStrategy, OrderStrategyAction};
use crate::api::orders::failure::{OrderFailureReason, deserialize_optional_non_null};
use crate::api::orders::wire::has_nonempty_text;
use crate::client::{DocumentedMutationResponse, MutationOutcome};
use crate::{AccountId, Decimal, Error, Symbol};

const START_STRATEGY_ENDPOINT: &str = "/orderStrategy/startorderstrategy";
const INTERRUPT_STRATEGY_ENDPOINT: &str = "/orderStrategy/interruptorderstrategy";
const MODIFY_STRATEGY_ENDPOINT: &str = "/orderStrategy/modifyorderstrategy";
const MULTI_BRACKET_STRATEGY_TYPE_ID: i64 = 2;
const MAX_STRATEGY_BRACKETS: usize = 64;

/// The current Partner contract exposes `command` only as an unconstrained
/// string and publishes no command grammar. The mutation is deliberately not
/// callable until that current grammar is documented.
pub const MODIFY_ORDER_STRATEGY_DOCUMENTATION_GAP: CurrentDocumentationGap =
    CurrentDocumentationGap::new(
        MODIFY_STRATEGY_ENDPOINT,
        "command",
        "the current Partner contract publishes no structured modification command grammar",
    );

/// One exact, bounded bracket in the documented current multi-bracket params.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiBracket {
    qty: OrderQuantity,
    #[serde(with = "crate::decimal")]
    profit_target: Decimal,
    #[serde(with = "crate::decimal")]
    stop_loss: Decimal,
    trailing_stop: bool,
}

impl MultiBracket {
    /// Creates a bracket using exact provider offsets.
    ///
    /// Tradovate's current example permits signed offsets and does not publish
    /// a sign or non-zero invariant, so this constructor preserves the caller's
    /// exact decimals without guessing one.
    #[must_use]
    pub const fn new(
        quantity: OrderQuantity,
        profit_target: Decimal,
        stop_loss: Decimal,
        trailing_stop: bool,
    ) -> Self {
        Self {
            qty: quantity,
            profit_target,
            stop_loss,
            trailing_stop,
        }
    }
}

/// Structured params for Tradovate's currently documented type-2
/// multi-bracket strategy.
#[derive(Clone, Debug)]
pub struct MultiBracketParams {
    entry_quantity: OrderQuantity,
    brackets: Box<[MultiBracket]>,
}

impl MultiBracketParams {
    /// Validates and owns one current multi-bracket parameter set.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidRequest`] unless there are one to 64 brackets.
    /// The library bound prevents an unbounded nested JSON string while still
    /// permitting substantially more brackets than the official example.
    pub fn new(entry_quantity: OrderQuantity, brackets: Vec<MultiBracket>) -> Result<Self, Error> {
        if brackets.is_empty() || brackets.len() > MAX_STRATEGY_BRACKETS {
            return Err(Error::InvalidRequest {
                field: "brackets",
                reason: "must contain between one and 64 strategy brackets",
            });
        }
        Ok(Self {
            entry_quantity,
            brackets: brackets.into_boxed_slice(),
        })
    }

    /// Returns the bounded bracket set.
    #[must_use]
    pub fn brackets(&self) -> &[MultiBracket] {
        &self.brackets
    }
}

/// A validated request for the only strategy type currently documented by the
/// Partner API: type 2, multi-bracket.
#[derive(Clone, Debug)]
pub struct StartMultiBracketStrategy {
    account_id: AccountId,
    symbol: Symbol,
    action: OrderSide,
    params: MultiBracketParams,
    origin: OrderOrigin,
    instance_id: Option<StrategyInstanceId>,
    custom_tag50: Option<CustomTag50>,
}

impl StartMultiBracketStrategy {
    /// Creates a strategy with explicit account, symbol, action, params, and
    /// automated/manual origin.
    #[must_use]
    pub const fn new(
        account_id: AccountId,
        symbol: Symbol,
        action: OrderSide,
        params: MultiBracketParams,
        origin: OrderOrigin,
    ) -> Self {
        Self {
            account_id,
            symbol,
            action,
            params,
            origin,
            instance_id: None,
            custom_tag50: None,
        }
    }

    /// Adds a bounded caller-owned strategy instance identifier.
    #[must_use]
    pub fn with_instance_id(mut self, value: StrategyInstanceId) -> Self {
        self.instance_id = Some(value);
        self
    }

    /// Adds a bounded provider correlation tag.
    #[must_use]
    pub fn with_custom_tag(mut self, value: CustomTag50) -> Self {
        self.custom_tag50 = Some(value);
        self
    }
}

/// Provider acceptance evidence for a strategy command.
#[derive(Clone, Debug)]
pub struct OrderStrategyReceipt {
    id: OrderStrategyId,
    status: OrderStrategyStatus,
    failure_message: Option<String>,
    strategy: CurrentOrderStrategy,
}

impl OrderStrategyReceipt {
    /// Returns the provider strategy identifier.
    #[must_use]
    pub const fn id(&self) -> OrderStrategyId {
        self.id
    }

    /// Returns the provider status that completed the command response.
    #[must_use]
    pub const fn status(&self) -> &OrderStrategyStatus {
        &self.status
    }

    /// Returns provider strategy failure context for terminal strategy states.
    #[must_use]
    pub fn failure_message(&self) -> Option<&str> {
        self.failure_message.as_deref()
    }

    /// Returns the complete current Partner strategy entity received as
    /// acceptance evidence.
    #[must_use]
    pub const fn strategy(&self) -> &CurrentOrderStrategy {
        &self.strategy
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct EntryVersion {
    order_qty: OrderQuantity,
    order_type: &'static str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ParamsWire<'a> {
    entry_version: EntryVersion,
    brackets: &'a [MultiBracket],
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct StartStrategyWire<'a> {
    account_id: AccountId,
    symbol: &'a str,
    order_strategy_type_id: OrderStrategyTypeId,
    action: OrderSide,
    params: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    uuid: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_tag50: Option<&'a str>,
    is_automated: bool,
}

impl<'a> StartStrategyWire<'a> {
    fn new(request: &'a StartMultiBracketStrategy) -> Result<Self, Error> {
        let params = ParamsWire {
            entry_version: EntryVersion {
                order_qty: request.params.entry_quantity,
                order_type: "Market",
            },
            brackets: &request.params.brackets,
        };
        let params = serde_json::to_string(&params).map_err(|source| Error::Encode {
            endpoint: START_STRATEGY_ENDPOINT,
            source,
        })?;
        let order_strategy_type_id = OrderStrategyTypeId::new(MULTI_BRACKET_STRATEGY_TYPE_ID)
            .map_err(|_| Error::InvalidRequest {
                field: "order_strategy_type_id",
                reason: "current multi-bracket strategy type ID must be positive",
            })?;
        Ok(Self {
            account_id: request.account_id,
            symbol: request.symbol.as_str(),
            order_strategy_type_id,
            action: request.action,
            params,
            uuid: request.instance_id.as_ref().map(StrategyInstanceId::as_str),
            custom_tag50: request.custom_tag50.as_ref().map(CustomTag50::as_str),
            is_automated: request.origin.is_automated(),
        })
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct InterruptStrategy {
    order_strategy_id: OrderStrategyId,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct StrategyResponse {
    #[serde(default, deserialize_with = "deserialize_optional_non_null")]
    error_text: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_non_null")]
    failure_reason: Option<OrderFailureReason>,
    #[serde(default, deserialize_with = "deserialize_optional_non_null")]
    order_strategy: Option<CurrentOrderStrategy>,
}

impl DocumentedMutationResponse for StrategyResponse {
    fn mutation_outcome(&self) -> MutationOutcome {
        let complete = self.order_strategy.as_ref().is_some_and(|strategy| {
            strategy.id().is_some() && is_current_status(strategy.status())
        });
        let any_evidence = self.order_strategy.is_some();
        let has_failure_text = has_nonempty_text(self.error_text.as_deref());
        let has_success_evidence = self
            .failure_reason
            .as_ref()
            .is_some_and(OrderFailureReason::is_success)
            || any_evidence;
        if has_failure_text && has_success_evidence {
            return MutationOutcome::Ambiguous;
        }
        match (self.failure_reason.as_ref(), complete, any_evidence) {
            (None | Some(OrderFailureReason::Success), true, _) => MutationOutcome::Success,
            (Some(reason), false, false) if reason.is_known_rejection() => {
                MutationOutcome::Rejected
            }
            _ => MutationOutcome::Ambiguous,
        }
    }

    fn has_success_evidence(&self) -> bool {
        self.failure_reason
            .as_ref()
            .is_some_and(OrderFailureReason::is_success)
            || self.order_strategy.is_some()
    }
}

fn receipt_from_strategy(strategy: &CurrentOrderStrategy) -> Option<OrderStrategyReceipt> {
    let id = strategy.id().copied()?;
    let status = strategy.status();
    is_current_status(status).then(|| OrderStrategyReceipt {
        id,
        status: status.clone(),
        failure_message: strategy.failure_message().map(str::to_owned),
        strategy: strategy.clone(),
    })
}

fn start_identity_matches(
    request: &StartMultiBracketStrategy,
    strategy: &CurrentOrderStrategy,
) -> bool {
    let account_matches = strategy.account_id() == &request.account_id;
    let type_matches = strategy.order_strategy_type_id().get() == MULTI_BRACKET_STRATEGY_TYPE_ID;
    let action_matches = matches!(
        (request.action, strategy.action()),
        (OrderSide::Buy, OrderStrategyAction::Buy) | (OrderSide::Sell, OrderStrategyAction::Sell)
    );
    let uuid_matches = match (request.instance_id.as_ref(), strategy.uuid()) {
        (Some(expected), Some(actual)) => expected.as_str() == actual,
        (Some(_), None) => false,
        (None, _) => true,
    };
    account_matches && type_matches && action_matches && uuid_matches
}

fn is_current_status(status: &OrderStrategyStatus) -> bool {
    !matches!(status, OrderStrategyStatus::Unknown(_))
}

#[cfg(test)]
#[path = "strategy/tests.rs"]
mod tests;

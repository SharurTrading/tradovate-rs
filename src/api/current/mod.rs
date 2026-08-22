// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Complete typed coverage of the pinned current Tradovate Partner REST API.
//!
//! The checked-in generated modules are derived only from the official current
//! Partner `OpenAPI` snapshot. Handwritten execution policy remains in the
//! client layer so generated operations cannot bypass authentication, bounds,
//! rate admission, provider-control validation, or mutation ambiguity fencing.

mod demo_balance;
mod generated;
mod mutations;
pub(crate) mod support;

pub use generated::{
    OPERATIONS, accounting, alerts, authentication, configuration, contracts, fees, funds, ids,
    orders, positions, risks, users,
};
pub use mutations::customer;
pub use mutations::risk_control::{
    CreateUserAccountPositionLimitRequest, CreateUserAccountRiskParameterRequest,
    PartnerAdminAutoLiqAction, SetAdminAutoLiqActionRequest, SetAdminAutoLiqActionRequestBuilder,
    UpdateUserAccountPositionLimitRequest, UpdateUserAccountRiskParameterRequest,
};
pub use mutations::{
    CreateMarketDataSubscriptionRequest, CreatePoaContactRequest,
    CreateTradovateSubscriptionRequest, CreateUserPluginRequest,
    UpdateMarketDataSubscriptionRequest, UpdatePoaContactRequest, UpdateUserPluginRequest,
};
pub use support::{
    BuildError, DocumentedAcknowledgement, HttpMethod, Operation, OperationClass, OperationSurface,
    ResponseContract, SchemaGap, SecretValue,
};

/// SHA-256 of the pinned official current Partner `OpenAPI` JSON.
pub const SPEC_SHA256: &str = "37caeccf4b0913460a788fcaf4c902497059b8ffe6f6355512e6c08eaacde769";

/// Date on which the official current Partner contract was pinned.
pub const SPEC_SNAPSHOT_DATE: &str = "2026-08-21";

/// Current component schemas that cannot be represented losslessly without
/// inventing a value grammar or exposing raw JSON.
pub const SCHEMA_GAPS: &[SchemaGap] = &[
    SchemaGap {
        schema: "ExtraPreTradeRiskProducts",
        operations: &["/order/dryrun"],
        reason: "the current component is an object with no value schema",
    },
    SchemaGap {
        schema: "ExtraPreTradeRiskContracts",
        operations: &["/order/dryrun"],
        reason: "the current component is an object with no value schema",
    },
    SchemaGap {
        schema: "RollContractsResponseContracts",
        operations: &["/contract/rollcontracts"],
        reason: "the current response map has no key or value schema",
    },
];

#[cfg(test)]
mod tests;

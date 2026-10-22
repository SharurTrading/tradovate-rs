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
pub(crate) mod spec;
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
pub use spec::{SCHEMA_GAPS, SPEC_SHA256, SPEC_SNAPSHOT_DATE};
pub use support::{
    BuildError, DocumentedAcknowledgement, HttpMethod, Operation, OperationClass, OperationSurface,
    ResponseContract, SchemaGap, SecretValue,
};

#[cfg(test)]
mod tests;

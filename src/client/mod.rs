// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! REST client facade and construction.

mod authentication;
mod builder;
mod execute;
mod handle;
mod json;
mod mutation;

pub use builder::ClientBuilder;
pub use handle::Client;

pub(crate) use execute::ControlWireResponse;
pub(crate) use handle::allocate_client_instance_id;
pub(crate) use json::encode_bounded_json;
pub(crate) use mutation::{
    DocumentedMutationResponse, MutationAssessment, MutationGate, MutationOutcome,
};

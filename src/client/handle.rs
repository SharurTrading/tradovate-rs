// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! The cloneable REST client handle and its process-local instance identity.

use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};

use crate::{ConfigError, EndpointSet, auth::TokenStore, rate_limit::RateGovernor};

use super::{ClientBuilder, MutationGate};

/// Cloneable Tradovate REST client sharing one authenticated session.
///
/// Clones share the HTTP connection pool and revision-fenced token state. The
/// crate never creates a Tokio runtime; asynchronous methods use the caller's
/// current runtime.
#[derive(Clone, Debug)]
pub struct Client {
    pub(crate) http: reqwest::Client,
    pub(crate) endpoints: EndpointSet,
    pub(crate) instance_id: u64,
    pub(crate) tokens: Arc<TokenStore>,
    pub(crate) rate_limits: Arc<RateGovernor>,
    pub(crate) mutation_gate: Arc<MutationGate>,
    pub(crate) max_request_bytes: usize,
    pub(crate) max_response_bytes: usize,
}

static NEXT_CLIENT_INSTANCE_ID: AtomicU64 = AtomicU64::new(1);

pub(crate) fn allocate_client_instance_id() -> Result<u64, ConfigError> {
    NEXT_CLIENT_INSTANCE_ID
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, checked_add_one)
        .map_err(|_| ConfigError::InvalidSetting {
            field: "client_instance_id",
            reason: "process-local identity space is exhausted",
        })
}

const fn checked_add_one(value: u64) -> Option<u64> {
    value.checked_add(1)
}

impl Client {
    /// Starts a builder for the selected Tradovate environment.
    pub fn builder(environment: crate::Environment) -> ClientBuilder {
        ClientBuilder::new(environment.endpoints())
    }

    /// Starts a builder with an explicitly validated endpoint set.
    pub fn builder_with_endpoints(endpoints: EndpointSet) -> ClientBuilder {
        ClientBuilder::new(endpoints)
    }
}

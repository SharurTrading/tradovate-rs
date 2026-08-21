// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Cancellation-safe lifecycle for money-moving REST requests.

use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use super::Client;
use crate::Error;

/// Endpoint wire responses expose provider-success evidence before a control
/// response can be treated as a definitive rejection.
pub(crate) trait MutationWireResponse {
    fn has_success_evidence(&self) -> bool;
}

/// Shared fail-closed state for all clones of one client.
#[derive(Debug, Default)]
pub(crate) struct MutationGate {
    reconciliation_required: AtomicBool,
}

impl MutationGate {
    pub(crate) fn ensure_available(&self, endpoint: &'static str) -> Result<(), Error> {
        if self.reconciliation_required.load(Ordering::Acquire) {
            Err(Error::MutationReconciliationRequired { endpoint })
        } else {
            Ok(())
        }
    }

    pub(crate) fn attempt(
        self: &Arc<Self>,
        endpoint: &'static str,
    ) -> Result<MutationAttempt, Error> {
        self.ensure_available(endpoint)?;
        Ok(MutationAttempt {
            gate: Arc::clone(self),
            endpoint,
            armed: false,
        })
    }

    fn acknowledge(&self) {
        self.reconciliation_required.store(false, Ordering::Release);
    }

    fn is_reconciliation_required(&self) -> bool {
        self.reconciliation_required.load(Ordering::Acquire)
    }
}

/// An admitted mutation that becomes ambiguous if dropped while armed.
#[derive(Debug)]
pub(crate) struct MutationAttempt {
    gate: Arc<MutationGate>,
    endpoint: &'static str,
    armed: bool,
}

impl MutationAttempt {
    /// Arms immediately before the HTTP send future is first polled.
    pub(crate) fn arm(&mut self) -> Result<(), Error> {
        self.gate.ensure_available(self.endpoint)?;
        self.armed = true;
        Ok(())
    }

    /// Records a provider-confirmed success or definitive rejection.
    pub(crate) fn resolve(mut self) {
        self.armed = false;
    }
}

impl Drop for MutationAttempt {
    fn drop(&mut self) {
        if self.armed {
            self.gate
                .reconciliation_required
                .store(true, Ordering::Release);
        }
    }
}

/// Decoded mutation data whose provider semantics still require validation.
pub(crate) struct MutationResponse<T> {
    value: T,
    attempt: MutationAttempt,
}

impl<T> MutationResponse<T> {
    pub(crate) const fn new(value: T, attempt: MutationAttempt) -> Self {
        Self { value, attempt }
    }

    pub(crate) const fn value(&self) -> &T {
        &self.value
    }

    pub(crate) fn resolve(self) -> T {
        let Self { value, attempt } = self;
        attempt.resolve();
        value
    }
}

impl Client {
    /// Reports whether a prior money-moving request has an uncertain outcome.
    ///
    /// While this is `true`, this client and all of its clones reject further
    /// mutations. Queries remain available for reconciliation.
    #[must_use]
    pub fn mutation_reconciliation_required(&self) -> bool {
        self.mutation_gate.is_reconciliation_required()
    }

    /// Confirms that the caller reconciled provider order and command state.
    ///
    /// Calling this method without first querying authoritative provider state
    /// can permit a duplicate order or cancellation. The library cannot verify
    /// that external reconciliation was completed.
    pub fn acknowledge_mutation_reconciliation(&self) {
        self.mutation_gate.acknowledge();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dropping_an_armed_attempt_latches_all_clones() {
        let gate = Arc::new(MutationGate::default());
        let mut attempt = gate
            .attempt("/order/placeorder")
            .unwrap_or_else(|error| panic!("attempt must start: {error}"));
        attempt
            .arm()
            .unwrap_or_else(|error| panic!("attempt must arm: {error}"));
        drop(attempt);

        assert!(gate.is_reconciliation_required());
        assert!(matches!(
            gate.attempt("/order/cancelorder"),
            Err(Error::MutationReconciliationRequired { .. })
        ));
    }

    #[test]
    fn definitive_resolution_does_not_latch() {
        let gate = Arc::new(MutationGate::default());
        let mut attempt = gate
            .attempt("/order/placeorder")
            .unwrap_or_else(|error| panic!("attempt must start: {error}"));
        attempt
            .arm()
            .unwrap_or_else(|error| panic!("attempt must arm: {error}"));
        attempt.resolve();
        assert!(!gate.is_reconciliation_required());
    }
}

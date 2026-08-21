// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Cancellation-safe lifecycle for money-moving REST requests.

use std::sync::{
    Arc,
    atomic::{AtomicU8, Ordering},
};

use super::Client;
use crate::Error;

/// Resolution category derived from one endpoint's documented response contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MutationOutcome {
    /// The response contains every field required to prove provider acceptance.
    Success,
    /// The response definitively proves that the provider rejected the request.
    Rejected,
    /// The response is contradictory or lacks evidence required for resolution.
    Ambiguous,
}

/// Request-aware interpretation of one decoded mutation response.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MutationAssessment {
    outcome: MutationOutcome,
    success_evidence: bool,
}

impl MutationAssessment {
    pub(crate) const fn success() -> Self {
        Self {
            outcome: MutationOutcome::Success,
            success_evidence: true,
        }
    }

    pub(crate) const fn rejected() -> Self {
        Self {
            outcome: MutationOutcome::Rejected,
            success_evidence: false,
        }
    }

    pub(crate) const fn ambiguous(success_evidence: bool) -> Self {
        Self {
            outcome: MutationOutcome::Ambiguous,
            success_evidence,
        }
    }

    pub(crate) const fn outcome(self) -> MutationOutcome {
        self.outcome
    }

    pub(crate) const fn has_success_evidence(self) -> bool {
        self.success_evidence
    }
}

/// A documented mutation response validates both control contradictions and
/// endpoint-specific completion evidence before its attempt may be resolved.
pub(crate) trait DocumentedMutationResponse {
    fn mutation_outcome(&self) -> MutationOutcome;

    fn has_success_evidence(&self) -> bool;
}

/// Shared fail-closed state for all clones of one client.
const AVAILABLE: u8 = 0;
const IN_FLIGHT: u8 = 1;
const RECONCILIATION_REQUIRED: u8 = 2;

#[derive(Debug, Default)]
pub(crate) struct MutationGate {
    state: AtomicU8,
}

impl MutationGate {
    pub(crate) fn ensure_available(&self, endpoint: &'static str) -> Result<(), Error> {
        match self.state.load(Ordering::Acquire) {
            AVAILABLE => Ok(()),
            IN_FLIGHT => Err(Error::MutationInProgress { endpoint }),
            _ => Err(Error::MutationReconciliationRequired { endpoint }),
        }
    }

    pub(crate) fn attempt(
        self: &Arc<Self>,
        endpoint: &'static str,
    ) -> Result<MutationAttempt, Error> {
        match self
            .state
            .compare_exchange(AVAILABLE, IN_FLIGHT, Ordering::AcqRel, Ordering::Acquire)
        {
            Ok(_) => Ok(MutationAttempt {
                gate: Arc::clone(self),
                endpoint,
                armed: false,
                released: false,
            }),
            Err(IN_FLIGHT) => Err(Error::MutationInProgress { endpoint }),
            Err(_) => Err(Error::MutationReconciliationRequired { endpoint }),
        }
    }

    fn acknowledge(&self) {
        let _ = self.state.compare_exchange(
            RECONCILIATION_REQUIRED,
            AVAILABLE,
            Ordering::AcqRel,
            Ordering::Acquire,
        );
    }

    fn is_reconciliation_required(&self) -> bool {
        self.state.load(Ordering::Acquire) == RECONCILIATION_REQUIRED
    }
}

/// An admitted mutation that becomes ambiguous if dropped while armed.
#[derive(Debug)]
pub(crate) struct MutationAttempt {
    gate: Arc<MutationGate>,
    endpoint: &'static str,
    armed: bool,
    released: bool,
}

impl MutationAttempt {
    /// Arms immediately before the HTTP send future is first polled.
    pub(crate) fn arm(&mut self) -> Result<(), Error> {
        if self.gate.state.load(Ordering::Acquire) != IN_FLIGHT || self.released {
            return Err(Error::MutationReconciliationRequired {
                endpoint: self.endpoint,
            });
        }
        self.armed = true;
        Ok(())
    }

    /// Records a provider-confirmed success or definitive rejection.
    pub(crate) fn resolve(mut self) {
        self.armed = false;
        self.released = true;
        self.gate.state.store(AVAILABLE, Ordering::Release);
    }
}

impl Drop for MutationAttempt {
    fn drop(&mut self) {
        if self.released {
            return;
        }
        let next = if self.armed {
            RECONCILIATION_REQUIRED
        } else {
            AVAILABLE
        };
        self.gate.state.store(next, Ordering::Release);
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

    #[test]
    fn only_one_clone_can_own_an_in_flight_mutation() {
        let gate = Arc::new(MutationGate::default());
        let first = gate
            .attempt("/order/placeorder")
            .unwrap_or_else(|error| panic!("first attempt must claim the gate: {error}"));
        assert!(matches!(
            gate.attempt("/order/cancelorder"),
            Err(Error::MutationInProgress { .. })
        ));
        drop(first);
        assert!(gate.attempt("/order/cancelorder").is_ok());
    }

    #[test]
    fn acknowledgement_cannot_clear_an_in_flight_claim() {
        let gate = Arc::new(MutationGate::default());
        let first = gate
            .attempt("/order/placeorder")
            .unwrap_or_else(|error| panic!("first attempt must claim the gate: {error}"));
        gate.acknowledge();
        assert!(matches!(
            gate.attempt("/order/cancelorder"),
            Err(Error::MutationInProgress { .. })
        ));
        drop(first);
    }
}

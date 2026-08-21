// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Cancellation-safe reservation for failed-response-only quotas.

use super::RateGovernor;

/// Reservation for a provider quota that counts failed responses only.
#[must_use]
pub(crate) struct FailedOnlyAdmission<'a> {
    governor: &'a RateGovernor,
    endpoint: &'static str,
    active: bool,
}

impl<'a> FailedOnlyAdmission<'a> {
    pub(super) const fn new(governor: &'a RateGovernor, endpoint: &'static str) -> Self {
        Self {
            governor,
            endpoint,
            active: true,
        }
    }

    pub(crate) fn succeed(mut self) {
        self.governor.finish_failed_only(self.endpoint, false);
        self.active = false;
    }

    pub(crate) fn release_unsent(mut self) {
        self.governor.finish_failed_only(self.endpoint, false);
        self.active = false;
    }
}

impl Drop for FailedOnlyAdmission<'_> {
    fn drop(&mut self) {
        if self.active {
            self.governor.finish_failed_only(self.endpoint, true);
        }
    }
}

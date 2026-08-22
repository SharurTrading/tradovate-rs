// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Cancellation-safe request admission and failed-response-only accounting.

use tokio::time::{Instant, sleep};

use super::{RateGovernor, window::Audience};
use crate::{AccountId, Error};

/// Admission spanning one request attempt.
///
/// All-request windows are committed atomically when this guard is created.
/// For endpoints whose quota counts failures only, dropping an active guard
/// conservatively records a failure. A fully validated success releases that
/// reservation; a definitive connection failure may release it as unsent.
#[must_use]
pub(crate) struct RateAdmission<'a> {
    governor: &'a RateGovernor,
    audience: Audience,
    endpoint: &'static str,
    account_id: Option<AccountId>,
    admitted_at: Instant,
    active: bool,
}

impl<'a> RateAdmission<'a> {
    const fn new(
        governor: &'a RateGovernor,
        audience: Audience,
        endpoint: &'static str,
        account_id: Option<AccountId>,
        admitted_at: Instant,
    ) -> Self {
        Self {
            governor,
            audience,
            endpoint,
            account_id,
            admitted_at,
            active: true,
        }
    }

    pub(crate) fn succeed(mut self) {
        self.governor.finish_failed_only(self.endpoint, false);
        self.active = false;
    }

    pub(crate) fn release_unsent(mut self) {
        self.governor.release_unsent(
            self.audience,
            self.endpoint,
            self.account_id,
            self.admitted_at,
        );
        self.active = false;
    }
}

impl Drop for RateAdmission<'_> {
    fn drop(&mut self) {
        if self.active {
            self.governor.finish_failed_only(self.endpoint, true);
        }
    }
}

impl RateGovernor {
    pub(crate) async fn begin_authenticated(&self, endpoint: &'static str) -> RateAdmission<'_> {
        self.begin(Audience::Authenticated, endpoint).await
    }

    pub(crate) async fn begin_anonymous_failed_only(
        &self,
        endpoint: &'static str,
    ) -> RateAdmission<'_> {
        self.begin(Audience::Anonymous, endpoint).await
    }

    async fn begin(&self, audience: Audience, endpoint: &'static str) -> RateAdmission<'_> {
        loop {
            let notified = self.changed.notified();
            tokio::pin!(notified);
            notified.as_mut().enable();
            match self.try_admit(audience, endpoint, true) {
                Ok(admitted_at) => {
                    return RateAdmission::new(self, audience, endpoint, None, admitted_at);
                }
                Err(retry_after) => {
                    tokio::select! {
                        () = sleep(retry_after) => {}
                        () = &mut notified => {}
                    }
                }
            }
        }
    }

    pub(crate) fn admit_immediate(
        &self,
        endpoint: &'static str,
    ) -> Result<RateAdmission<'_>, Error> {
        self.admit_immediate_inner(endpoint, None)
    }

    pub(crate) fn admit_immediate_for_account(
        &self,
        endpoint: &'static str,
        account_id: AccountId,
    ) -> Result<RateAdmission<'_>, Error> {
        self.admit_immediate_inner(endpoint, Some(account_id))
    }

    fn admit_immediate_inner(
        &self,
        endpoint: &'static str,
        account_id: Option<AccountId>,
    ) -> Result<RateAdmission<'_>, Error> {
        match self.try_admit_inner(Audience::Authenticated, endpoint, true, account_id) {
            Ok(admitted_at) => Ok(RateAdmission::new(
                self,
                Audience::Authenticated,
                endpoint,
                account_id,
                admitted_at,
            )),
            Err(retry_after) => Err(Error::LocalRateLimit {
                endpoint,
                retry_after,
            }),
        }
    }

    fn release_unsent(
        &self,
        audience: Audience,
        endpoint: &'static str,
        account_id: Option<AccountId>,
        admitted_at: Instant,
    ) {
        let mut state = self.state.lock();
        for window in state
            .windows
            .iter_mut()
            .filter(|window| window.applies(audience, endpoint))
        {
            if let Some(index) = window
                .admitted
                .iter()
                .position(|instant| *instant == admitted_at)
            {
                window.admitted.remove(index);
            }
        }
        if let Some(window) = state.failed_only.get_mut(endpoint) {
            window.reservations = window.reservations.saturating_sub(1);
        }
        if let Some(account_id) = account_id {
            state.account_windows.release(endpoint, account_id);
        }
        self.changed.notify_waiters();
    }

    fn finish_failed_only(&self, endpoint: &'static str, failed: bool) {
        let mut state = self.state.lock();
        let Some(window) = state.failed_only.get_mut(endpoint) else {
            return;
        };
        window.reservations = window.reservations.saturating_sub(1);
        if failed {
            window.failures.push_back(Instant::now());
        }
        self.changed.notify_waiters();
    }
}

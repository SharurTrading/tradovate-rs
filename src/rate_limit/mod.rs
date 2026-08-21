// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Shared, bounded rolling-window request admission.

mod admission;
mod cooldown;
mod window;

use std::{
    collections::{HashMap, HashSet},
    time::Duration,
};

use parking_lot::Mutex;
use tokio::{
    sync::Notify,
    time::{Instant, sleep},
};

use self::{
    admission::FailedOnlyAdmission,
    window::{Audience, FailedOnlyWindow, Window, tradovate_failed_only, tradovate_windows},
};
use crate::Error;

const HOUR: Duration = Duration::from_hours(1);

#[derive(Debug)]
struct State {
    windows: Vec<Window>,
    failed_only: HashMap<&'static str, FailedOnlyWindow>,
    endpoint_cooldowns: HashMap<&'static str, Instant>,
    endpoint_blocked: HashSet<&'static str>,
    global_cooldown: Option<Instant>,
    global_blocked: bool,
}

/// One process-local admission governor shared across every client clone.
#[derive(Debug)]
pub(crate) struct RateGovernor {
    state: Mutex<State>,
    changed: Notify,
}

impl RateGovernor {
    pub(crate) fn tradovate_defaults() -> Self {
        Self {
            state: Mutex::new(State {
                windows: tradovate_windows(),
                failed_only: tradovate_failed_only(),
                endpoint_cooldowns: HashMap::new(),
                endpoint_blocked: HashSet::new(),
                global_cooldown: None,
                global_blocked: false,
            }),
            changed: Notify::new(),
        }
    }

    pub(crate) async fn wait(&self, endpoint: &'static str) {
        loop {
            let retry_after = self.try_admit_authenticated(endpoint);
            if retry_after.is_zero() {
                return;
            }
            sleep(retry_after).await;
        }
    }

    pub(crate) fn try_admit_authenticated(&self, endpoint: &'static str) -> Duration {
        self.try_admit(Audience::Authenticated, endpoint, false)
    }

    pub(crate) async fn begin_anonymous_failed_only(
        &self,
        endpoint: &'static str,
    ) -> FailedOnlyAdmission<'_> {
        loop {
            let notified = self.changed.notified();
            tokio::pin!(notified);
            notified.as_mut().enable();
            let retry_after = self.try_admit(Audience::Anonymous, endpoint, true);
            if retry_after.is_zero() {
                return FailedOnlyAdmission::new(self, endpoint);
            }
            tokio::select! {
                () = sleep(retry_after) => {}
                () = &mut notified => {}
            }
        }
    }

    pub(crate) fn admit_immediate(&self, endpoint: &'static str) -> Result<(), Error> {
        let retry_after = self.try_admit(Audience::Authenticated, endpoint, false);
        if retry_after.is_zero() {
            Ok(())
        } else {
            Err(Error::LocalRateLimit {
                endpoint,
                retry_after,
            })
        }
    }

    fn try_admit(
        &self,
        audience: Audience,
        endpoint: &'static str,
        reserve_failed_only: bool,
    ) -> Duration {
        let now = Instant::now();
        let mut state = self.state.lock();
        let mut retry_after = if state.global_blocked {
            HOUR
        } else {
            Duration::ZERO
        };
        retry_after = retry_after.max(
            state
                .global_cooldown
                .filter(|deadline| *deadline > now)
                .map_or(Duration::ZERO, |deadline| deadline - now),
        );
        state.global_cooldown = state.global_cooldown.filter(|deadline| *deadline > now);
        if state.endpoint_blocked.contains(endpoint) {
            retry_after = retry_after.max(HOUR);
        }
        if let Some(deadline) = state.endpoint_cooldowns.get(endpoint).copied() {
            if deadline > now {
                retry_after = retry_after.max(deadline - now);
            } else {
                state.endpoint_cooldowns.remove(endpoint);
            }
        }
        for window in &mut state.windows {
            window.prune(now);
            if window.applies(audience, endpoint) && window.admitted.len() >= window.limit {
                retry_after = retry_after.max(window.retry_after(now));
            }
        }
        if reserve_failed_only && let Some(window) = state.failed_only.get_mut(endpoint) {
            window.prune(now);
            if window.failures.len().saturating_add(window.reservations) >= window.limit {
                retry_after = retry_after.max(window.retry_after(now));
            }
        }
        if retry_after.is_zero() {
            for window in state
                .windows
                .iter_mut()
                .filter(|window| window.applies(audience, endpoint))
            {
                window.admitted.push_back(now);
            }
            if reserve_failed_only && let Some(window) = state.failed_only.get_mut(endpoint) {
                window.reservations = window.reservations.saturating_add(1);
            }
        }
        retry_after
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

#[cfg(test)]
#[path = "tests.rs"]
mod tests;

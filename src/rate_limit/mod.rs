// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Shared, bounded rolling-window request admission.

mod account;
mod admission;
mod cooldown;
mod window;

pub(crate) use admission::RateAdmission;

use std::{
    collections::{HashMap, HashSet},
    time::Duration,
};

use parking_lot::Mutex;
use tokio::{sync::Notify, time::Instant};

use self::{
    account::AccountWindows,
    window::{Audience, FailedOnlyWindow, Window, tradovate_failed_only, tradovate_windows},
};
use crate::AccountId;

const HOUR: Duration = Duration::from_hours(1);

#[derive(Debug)]
struct State {
    windows: Vec<Window>,
    failed_only: HashMap<&'static str, FailedOnlyWindow>,
    endpoint_cooldowns: HashMap<&'static str, Instant>,
    endpoint_blocked: HashSet<&'static str>,
    account_windows: AccountWindows,
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
                account_windows: AccountWindows::default(),
                global_cooldown: None,
                global_blocked: false,
            }),
            changed: Notify::new(),
        }
    }

    pub(crate) fn try_admit_authenticated(&self, endpoint: &'static str) -> Duration {
        self.try_admit(Audience::Authenticated, endpoint, false)
            .err()
            .unwrap_or_default()
    }

    fn try_admit(
        &self,
        audience: Audience,
        endpoint: &'static str,
        reserve_failed_only: bool,
    ) -> Result<Instant, Duration> {
        self.try_admit_inner(audience, endpoint, reserve_failed_only, None)
    }

    fn try_admit_inner(
        &self,
        audience: Audience,
        endpoint: &'static str,
        reserve_failed_only: bool,
        account_id: Option<AccountId>,
    ) -> Result<Instant, Duration> {
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
        let account_reservation = account_id.and_then(|account_id| {
            match state.account_windows.evaluate(now, endpoint, account_id) {
                Ok(reservation) => Some(reservation),
                Err(account_retry) => {
                    retry_after = retry_after.max(account_retry);
                    None
                }
            }
        });
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
            if let Some(reservation) = account_reservation {
                state.account_windows.admit(reservation);
            }
            Ok(now)
        } else {
            Err(retry_after)
        }
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;

// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! The shared rolling-window admission governor and its internal state.

use std::{
    collections::{HashMap, HashSet},
    time::Duration,
};

use parking_lot::Mutex;
use tokio::{sync::Notify, time::Instant};

use super::account::AccountWindows;
use super::window::{Audience, FailedOnlyWindow, Window, tradovate_failed_only, tradovate_windows};
use crate::AccountId;

pub(crate) const HOUR: Duration = Duration::from_hours(1);

#[derive(Debug)]
pub(super) struct State {
    pub(super) windows: Vec<Window>,
    pub(super) failed_only: HashMap<&'static str, FailedOnlyWindow>,
    pub(super) endpoint_cooldowns: HashMap<&'static str, Instant>,
    pub(super) endpoint_blocked: HashSet<&'static str>,
    pub(super) account_windows: AccountWindows,
    pub(super) global_cooldown: Option<Instant>,
    pub(super) global_blocked: bool,
}

/// One process-local admission governor shared across every client clone.
#[derive(Debug)]
pub(crate) struct RateGovernor {
    pub(super) state: Mutex<State>,
    pub(super) changed: Notify,
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

    pub(super) fn try_admit(
        &self,
        audience: Audience,
        endpoint: &'static str,
        reserve_failed_only: bool,
    ) -> Result<Instant, Duration> {
        self.try_admit_inner(audience, endpoint, reserve_failed_only, None)
    }

    pub(super) fn try_admit_inner(
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

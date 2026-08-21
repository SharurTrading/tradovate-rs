// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Provider-declared cooldown and captcha-lockout installation.

use std::time::Duration;

use tokio::time::Instant;

use super::{HOUR, RateGovernor};

impl RateGovernor {
    pub(crate) fn apply_endpoint_cooldown(&self, endpoint: &'static str, duration: Duration) {
        self.apply_endpoint_cooldown_until(
            endpoint,
            std::time::Instant::now().checked_add(duration),
        );
    }

    pub(crate) fn apply_endpoint_cooldown_until(
        &self,
        endpoint: &'static str,
        deadline: Option<std::time::Instant>,
    ) {
        let Some(deadline) = deadline else {
            self.state.lock().endpoint_blocked.insert(endpoint);
            return;
        };
        let deadline = Instant::from_std(deadline);
        let mut state = self.state.lock();
        state
            .endpoint_cooldowns
            .entry(endpoint)
            .and_modify(|current| *current = (*current).max(deadline))
            .or_insert(deadline);
    }

    pub(crate) fn apply_captcha_lockout(&self, endpoint: &'static str, duration: Duration) {
        self.apply_endpoint_cooldown(endpoint, duration.max(HOUR));
    }

    pub(crate) fn apply_global_cooldown(&self, duration: Duration) {
        let Some(deadline) = Instant::now().checked_add(duration) else {
            self.state.lock().global_blocked = true;
            return;
        };
        let mut state = self.state.lock();
        state.global_cooldown = Some(
            state
                .global_cooldown
                .map_or(deadline, |old| old.max(deadline)),
        );
    }
}

// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Rolling-window state used by the shared rate governor.

use std::{
    collections::{HashMap, VecDeque},
    time::Duration,
};

use tokio::time::Instant;

const HOUR: Duration = Duration::from_hours(1);
const SECOND: Duration = Duration::from_secs(1);
pub(super) const ANONYMOUS_SECONDLY_LIMIT: usize = 100;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Audience {
    Anonymous,
    Authenticated,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Scope {
    Audience(Audience),
    Endpoint(&'static str),
}

#[derive(Debug)]
pub(super) struct Window {
    scope: Scope,
    pub(super) limit: usize,
    duration: Duration,
    pub(super) admitted: VecDeque<Instant>,
}

impl Window {
    pub(super) fn applies(&self, audience: Audience, endpoint: &'static str) -> bool {
        match self.scope {
            Scope::Audience(expected) => expected == audience,
            Scope::Endpoint(expected) => expected == endpoint,
        }
    }

    pub(super) fn prune(&mut self, now: Instant) {
        while self
            .admitted
            .front()
            .is_some_and(|instant| now.duration_since(*instant) >= self.duration)
        {
            self.admitted.pop_front();
        }
    }

    pub(super) fn retry_after(&self, now: Instant) -> Duration {
        self.admitted.front().map_or(Duration::ZERO, |oldest| {
            self.duration.saturating_sub(now.duration_since(*oldest))
        })
    }
}

#[derive(Debug)]
pub(super) struct FailedOnlyWindow {
    pub(super) limit: usize,
    pub(super) duration: Duration,
    pub(super) failures: VecDeque<Instant>,
    pub(super) reservations: usize,
}

impl FailedOnlyWindow {
    pub(super) fn prune(&mut self, now: Instant) {
        while self
            .failures
            .front()
            .is_some_and(|instant| now.duration_since(*instant) >= self.duration)
        {
            self.failures.pop_front();
        }
    }

    pub(super) fn retry_after(&self, now: Instant) -> Duration {
        self.failures.front().map_or(self.duration, |oldest| {
            self.duration.saturating_sub(now.duration_since(*oldest))
        })
    }
}

pub(super) fn window(scope: Scope, limit: usize, duration: Duration) -> Window {
    Window {
        scope,
        limit,
        duration,
        admitted: VecDeque::with_capacity(limit),
    }
}

pub(super) fn tradovate_windows() -> Vec<Window> {
    vec![
        window(Scope::Audience(Audience::Anonymous), 1_000, HOUR),
        window(
            Scope::Audience(Audience::Anonymous),
            ANONYMOUS_SECONDLY_LIMIT,
            SECOND,
        ),
        window(Scope::Audience(Audience::Authenticated), 5_000, HOUR),
        window(Scope::Endpoint("/auth/renewaccesstoken"), 15, HOUR),
        window(Scope::Endpoint("/user/syncrequest"), 300, HOUR),
        window(Scope::Endpoint("/order/dryrun"), 500, HOUR),
        window(
            Scope::Endpoint("/cashBalance/changedemobalance"),
            1_000,
            HOUR,
        ),
        window(
            Scope::Endpoint("/accountRiskStatus/switchriskcategory"),
            5_000,
            HOUR,
        ),
        // Current Partner operational rate-limit table, verified 2026-08-22.
        // These Partner application budgets count every admitted request.
        window(
            Scope::Endpoint("/customerApplication/createpartnersubaccountrequest"),
            250,
            HOUR,
        ),
        window(
            Scope::Endpoint("/customerApplication/submitcustomerapplicationdocument"),
            30,
            HOUR,
        ),
        window(
            Scope::Endpoint("/customerApplication/submitpartnersubaccountdocument"),
            750,
            HOUR,
        ),
    ]
}

pub(super) fn tradovate_failed_only() -> HashMap<&'static str, FailedOnlyWindow> {
    HashMap::from([
        failed_only("/auth/accesstokenrequest", 5),
        failed_only("/auth/me", 10),
        failed_only("/user/createevaluationusers", 100),
        failed_only("/user/createevaluationaccounts", 100),
        failed_only("/user/requesttradingpermission", 20),
    ])
}

fn failed_only(endpoint: &'static str, limit: usize) -> (&'static str, FailedOnlyWindow) {
    (
        endpoint,
        FailedOnlyWindow {
            limit,
            duration: HOUR,
            failures: VecDeque::with_capacity(limit),
            reservations: 0,
        },
    )
}

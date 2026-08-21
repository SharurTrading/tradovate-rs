// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

use std::collections::VecDeque;

use super::window::{ANONYMOUS_SECONDLY_LIMIT, Scope, window};
use super::*;

#[tokio::test]
async fn mutation_admission_never_waits() {
    let governor = RateGovernor {
        state: Mutex::new(State {
            windows: vec![window(Scope::Audience(Audience::Authenticated), 1, HOUR)],
            failed_only: HashMap::new(),
            endpoint_cooldowns: HashMap::new(),
            endpoint_blocked: HashSet::new(),
            global_cooldown: None,
            global_blocked: false,
        }),
        changed: Notify::new(),
    };
    assert!(governor.admit_immediate("/order/placeorder").is_ok());
    assert!(matches!(
        governor.admit_immediate("/order/placeorder"),
        Err(Error::LocalRateLimit { .. })
    ));
}

#[test]
fn anonymous_admission_enforces_the_official_per_second_budget() {
    let governor = RateGovernor::tradovate_defaults();
    for _ in 0..ANONYMOUS_SECONDLY_LIMIT {
        assert!(
            governor
                .try_admit(Audience::Anonymous, "/fixture", false)
                .is_zero()
        );
    }
    assert!(
        governor.try_admit(Audience::Anonymous, "/fixture", false) >= Duration::from_millis(900)
    );
}

#[tokio::test]
async fn successful_failed_only_attempt_releases_its_reservation() {
    let governor = failed_only_governor();
    governor
        .begin_anonymous_failed_only("/auth/accesstokenrequest")
        .await
        .succeed();

    let next = tokio::time::timeout(
        Duration::from_millis(50),
        governor.begin_anonymous_failed_only("/auth/accesstokenrequest"),
    )
    .await;
    let Ok(next) = next else {
        panic!("successful calls must not consume the failed-only quota");
    };
    next.release_unsent();
}

#[tokio::test]
async fn failed_only_reservation_blocks_concurrent_and_recorded_failures() {
    let governor = failed_only_governor();
    let admission = governor
        .begin_anonymous_failed_only("/auth/accesstokenrequest")
        .await;
    assert!(
        tokio::time::timeout(
            Duration::from_millis(10),
            governor.begin_anonymous_failed_only("/auth/accesstokenrequest"),
        )
        .await
        .is_err()
    );
    drop(admission);
    assert!(
        tokio::time::timeout(
            Duration::from_millis(10),
            governor.begin_anonymous_failed_only("/auth/accesstokenrequest"),
        )
        .await
        .is_err()
    );
}

fn failed_only_governor() -> RateGovernor {
    RateGovernor {
        state: Mutex::new(State {
            windows: Vec::new(),
            failed_only: HashMap::from([(
                "/auth/accesstokenrequest",
                FailedOnlyWindow {
                    limit: 1,
                    duration: HOUR,
                    failures: VecDeque::with_capacity(1),
                    reservations: 0,
                },
            )]),
            endpoint_cooldowns: HashMap::new(),
            endpoint_blocked: HashSet::new(),
            global_cooldown: None,
            global_blocked: false,
        }),
        changed: Notify::new(),
    }
}

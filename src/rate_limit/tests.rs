// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use std::collections::VecDeque;

use super::account::{AccountWindows, MAX_ACCOUNT_RATE_KEYS};
use super::window::{ANONYMOUS_SECONDLY_LIMIT, Scope, window};
use super::*;
use crate::Error;

#[tokio::test]
async fn mutation_admission_never_waits() {
    let governor = RateGovernor {
        state: Mutex::new(State {
            windows: vec![window(Scope::Audience(Audience::Authenticated), 1, HOUR)],
            failed_only: HashMap::new(),
            endpoint_cooldowns: HashMap::new(),
            endpoint_blocked: HashSet::new(),
            account_windows: AccountWindows::default(),
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
                .is_ok()
        );
    }
    let Err(retry_after) = governor.try_admit(Audience::Anonymous, "/fixture", false) else {
        panic!("the per-second budget must reject the next admission");
    };
    assert!(retry_after >= Duration::from_millis(900));
}

#[test]
fn current_all_request_endpoints_enforce_their_hourly_budgets() {
    const ENDPOINTS: [(&str, usize); 8] = [
        ("/auth/renewaccesstoken", 15),
        ("/user/syncrequest", 300),
        ("/order/dryrun", 500),
        ("/cashBalance/changedemobalance", 1_000),
        ("/accountRiskStatus/switchriskcategory", 5_000),
        ("/customerApplication/createpartnersubaccountrequest", 250),
        ("/customerApplication/submitcustomerapplicationdocument", 30),
        ("/customerApplication/submitpartnersubaccountdocument", 750),
    ];

    for (endpoint, limit) in ENDPOINTS {
        let governor = RateGovernor::tradovate_defaults();
        for _ in 0..limit {
            assert!(governor.admit_immediate(endpoint).is_ok());
        }
        let Err(Error::LocalRateLimit { retry_after, .. }) = governor.admit_immediate(endpoint)
        else {
            panic!("the documented endpoint budget must reject the next request");
        };
        assert!(retry_after >= Duration::from_mins(59));
    }
}

#[test]
fn current_partner_application_budgets_are_endpoint_scoped() {
    let governor = RateGovernor::tradovate_defaults();
    for _ in 0..750 {
        assert!(
            governor
                .admit_immediate("/customerApplication/submitpartnersubaccountdocument")
                .is_ok()
        );
    }

    assert!(
        governor
            .admit_immediate("/customerApplication/submitcustomerapplicationdocument")
            .is_ok()
    );
}

#[test]
fn current_failed_only_budgets_match_the_partner_operational_table() {
    const ENDPOINTS: [(&str, usize); 5] = [
        ("/auth/accesstokenrequest", 5),
        ("/auth/me", 10),
        ("/user/createevaluationusers", 100),
        ("/user/createevaluationaccounts", 100),
        ("/user/requesttradingpermission", 20),
    ];

    for (endpoint, limit) in ENDPOINTS {
        let governor = RateGovernor::tradovate_defaults();
        for _ in 0..limit {
            let admission = governor
                .admit_immediate(endpoint)
                .unwrap_or_else(|error| panic!("failure must fit its endpoint budget: {error}"));
            drop(admission);
        }
        assert!(matches!(
            governor.admit_immediate(endpoint),
            Err(Error::LocalRateLimit { retry_after, .. })
                if retry_after >= Duration::from_mins(59)
        ));
    }
}

#[test]
fn fully_validated_successes_do_not_consume_failed_only_budgets() {
    const ENDPOINTS: [&str; 4] = [
        "/auth/me",
        "/user/createevaluationusers",
        "/user/createevaluationaccounts",
        "/user/requesttradingpermission",
    ];

    for endpoint in ENDPOINTS {
        let governor = RateGovernor::tradovate_defaults();
        for _ in 0..101 {
            governor
                .admit_immediate(endpoint)
                .unwrap_or_else(|error| {
                    panic!("success reservation must remain available: {error}")
                })
                .succeed();
        }
    }
}

#[test]
fn account_scoped_mutations_are_independent_and_immediate() {
    let governor = RateGovernor::tradovate_defaults();
    let first = AccountId::new(1).unwrap_or_else(|error| panic!("fixture ID: {error}"));
    let second = AccountId::new(2).unwrap_or_else(|error| panic!("fixture ID: {error}"));

    assert!(
        governor
            .admit_immediate_for_account("/cashBalance/changedemobalance", first)
            .is_ok()
    );
    assert!(matches!(
        governor.admit_immediate_for_account("/cashBalance/changedemobalance", first),
        Err(Error::LocalRateLimit { retry_after, .. })
            if retry_after >= Duration::from_mins(59)
    ));
    assert!(
        governor
            .admit_immediate_for_account("/cashBalance/changedemobalance", second)
            .is_ok()
    );
}

#[test]
fn explicitly_unsent_admission_rolls_back_every_reserved_scope() {
    let governor = RateGovernor::tradovate_defaults();
    let account = AccountId::new(1).unwrap_or_else(|error| panic!("fixture ID: {error}"));

    governor
        .admit_immediate_for_account("/cashBalance/changedemobalance", account)
        .unwrap_or_else(|error| panic!("first admission must fit: {error}"))
        .release_unsent();

    assert!(
        governor
            .admit_immediate_for_account("/cashBalance/changedemobalance", account)
            .is_ok()
    );
}

#[test]
fn demo_balance_account_guards_also_reserve_the_aggregate_endpoint_budget() {
    let governor = RateGovernor::tradovate_defaults();
    for raw in 1..=1_000 {
        let account = AccountId::new(raw).unwrap_or_else(|error| panic!("fixture ID: {error}"));
        assert!(
            governor
                .admit_immediate_for_account("/cashBalance/changedemobalance", account)
                .is_ok()
        );
    }
    let overflow = AccountId::new(1_001).unwrap_or_else(|error| panic!("fixture ID: {error}"));
    assert!(matches!(
        governor.admit_immediate_for_account("/cashBalance/changedemobalance", overflow),
        Err(Error::LocalRateLimit { retry_after, .. })
            if retry_after >= Duration::from_mins(59)
    ));
}

#[test]
fn account_and_general_admission_reserve_atomically() {
    let account = AccountId::new(1).unwrap_or_else(|error| panic!("fixture ID: {error}"));
    let mut blocked_window = window(Scope::Audience(Audience::Authenticated), 1, HOUR);
    blocked_window.admitted.push_back(Instant::now());
    let governor = RateGovernor {
        state: Mutex::new(State {
            windows: vec![blocked_window],
            failed_only: HashMap::new(),
            endpoint_cooldowns: HashMap::new(),
            endpoint_blocked: HashSet::new(),
            account_windows: AccountWindows::default(),
            global_cooldown: None,
            global_blocked: false,
        }),
        changed: Notify::new(),
    };

    assert!(
        governor
            .admit_immediate_for_account("/cashBalance/changedemobalance", account)
            .is_err()
    );
    assert!(governor.state.lock().account_windows.is_empty());
}

#[test]
fn account_scoped_admission_memory_is_bounded() {
    let governor = RateGovernor {
        state: Mutex::new(State {
            windows: Vec::new(),
            failed_only: HashMap::new(),
            endpoint_cooldowns: HashMap::new(),
            endpoint_blocked: HashSet::new(),
            account_windows: AccountWindows::default(),
            global_cooldown: None,
            global_blocked: false,
        }),
        changed: Notify::new(),
    };

    for raw in 1..=MAX_ACCOUNT_RATE_KEYS {
        let raw = i64::try_from(raw).unwrap_or_else(|error| panic!("fixture ID: {error}"));
        let account = AccountId::new(raw).unwrap_or_else(|error| panic!("fixture ID: {error}"));
        assert!(
            governor
                .admit_immediate_for_account("/cashBalance/changedemobalance", account)
                .is_ok()
        );
    }
    let overflow = i64::try_from(MAX_ACCOUNT_RATE_KEYS + 1)
        .unwrap_or_else(|error| panic!("fixture ID: {error}"));
    let overflow = AccountId::new(overflow).unwrap_or_else(|error| panic!("fixture ID: {error}"));
    assert!(
        governor
            .admit_immediate_for_account("/cashBalance/changedemobalance", overflow)
            .is_err()
    );
    assert_eq!(
        governor.state.lock().account_windows.len(),
        MAX_ACCOUNT_RATE_KEYS
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
            account_windows: AccountWindows::default(),
            global_cooldown: None,
            global_blocked: false,
        }),
        changed: Notify::new(),
    }
}

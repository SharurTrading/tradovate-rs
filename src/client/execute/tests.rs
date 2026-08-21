// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

use super::*;

#[test]
fn detects_http_200_business_failure() {
    let rates = crate::rate_limit::RateGovernor::tradovate_defaults();
    let result = inspect_provider_control(
        br#"{"errorText":"denied"}"#,
        1,
        "/order/placeorder",
        &rates,
        false,
    );
    assert!(matches!(result, Err(Error::Business { .. })));
}

#[test]
fn penalty_debug_does_not_expose_ticket() {
    let rates = crate::rate_limit::RateGovernor::tradovate_defaults();
    let penalty = inspect_provider_control(
        br#"{"p-ticket":"secret-ticket","p-time":15,"p-captcha":false}"#,
        7,
        "/auth/accesstokenrequest",
        &rates,
        true,
    );
    let Err(Error::Penalty(penalty)) = penalty else {
        panic!("valid penalty must classify");
    };
    assert!(!format!("{penalty:?}").contains("secret-ticket"));
}

#[test]
fn malformed_reserved_control_fails_closed() {
    let rates = crate::rate_limit::RateGovernor::tradovate_defaults();
    for body in [
        br#"{"orderId":1,"p-ticket":"partial"}"#.as_slice(),
        br#"{"orderId":1,"errorText":17}"#.as_slice(),
        br#"{"orderId":1,"violations":false}"#.as_slice(),
    ] {
        assert!(matches!(
            inspect_provider_control(body, 1, "/order/placeorder", &rates, false),
            Err(Error::InvalidProviderControl { .. })
        ));
    }
}

#[test]
fn unsupported_penalty_retry_discards_the_ticket() {
    let rates = crate::rate_limit::RateGovernor::tradovate_defaults();
    let result = inspect_provider_control(
        br#"{"p-ticket":"secret-ticket","p-time":15}"#,
        7,
        "/order/placeorder",
        &rates,
        false,
    );
    assert!(matches!(
        result,
        Err(Error::ProviderPenalty {
            endpoint: "/order/placeorder",
            retry_after,
            captcha_required: false,
        }) if retry_after == Duration::from_secs(15)
    ));
}

#[test]
fn provider_429_never_shortens_the_official_one_hour_penalty() {
    let rates = crate::rate_limit::RateGovernor::tradovate_defaults();
    assert_eq!(apply_429_cooldown(&rates, None), Duration::from_hours(1));
    let rates = crate::rate_limit::RateGovernor::tradovate_defaults();
    assert_eq!(
        apply_429_cooldown(&rates, Some(Duration::from_secs(1))),
        Duration::from_hours(1)
    );
    let rates = crate::rate_limit::RateGovernor::tradovate_defaults();
    assert_eq!(
        apply_429_cooldown(&rates, Some(Duration::from_hours(2))),
        Duration::from_hours(2)
    );
}

#[test]
fn endpoint_specific_operations_cannot_shorten_a_global_429_stop() {
    let rates = crate::rate_limit::RateGovernor::tradovate_defaults();
    assert_eq!(
        apply_429_cooldown(&rates, Some(Duration::from_secs(1))),
        Duration::from_hours(1)
    );
}

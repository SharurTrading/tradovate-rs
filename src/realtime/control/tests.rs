// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

use super::*;

#[test]
fn normal_and_business_payloads_are_distinguished_without_diagnostics() {
    assert!(matches!(inspect(None), Ok(ResponseControl::Payload)));
    assert!(matches!(
        inspect(Some(raw(r"[1,2,3]").as_ref())),
        Ok(ResponseControl::Payload)
    ));
    assert!(matches!(
        inspect(Some(raw(r#"{"errorText":""}"#).as_ref())),
        Ok(ResponseControl::Payload)
    ));
    assert!(matches!(
        inspect(Some(raw(r#"{"failureReason":"Success"}"#).as_ref())),
        Ok(ResponseControl::Payload)
    ));
    assert!(matches!(
        inspect(Some(
            raw(r#"{"errorText":"account secret leaked"}"#).as_ref()
        )),
        Ok(ResponseControl::BusinessFailure { .. })
    ));
    assert!(matches!(
        inspect(Some(raw(r#"{"failureReason":"Rejected"}"#).as_ref())),
        Ok(ResponseControl::BusinessFailure { .. })
    ));
    assert!(matches!(
        inspect(Some(raw(r#"{"violations":[{"field":"price"}]}"#).as_ref())),
        Ok(ResponseControl::BusinessFailure {
            violation_count: Some(1)
        })
    ));
}

#[test]
fn valid_penalty_is_typed_and_ticket_is_always_redacted() {
    let data = raw(
        r#"{"p-ticket":"do-not-render-this-ticket","p-time":15,"p-captcha":true,"p-message":"slow"}"#,
    );
    let Ok(ResponseControl::Penalty(penalty)) = inspect(Some(data.as_ref())) else {
        panic!("valid penalty must classify");
    };
    let debug = format!("{penalty:?}");
    assert!(!debug.contains("do-not-render-this-ticket"));
    assert!(debug.contains("[REDACTED]"));
    let (ticket, retry_after, captcha_required) = penalty.into_parts();
    assert_eq!(retry_after, Duration::from_secs(15));
    assert!(captcha_required);
    drop(ticket);
}

#[test]
fn malformed_or_contradictory_controls_fail_closed() {
    let malformed = [
        r#"{"p-ticket":"ticket"}"#,
        r#"{"p-time":1}"#,
        r#"{"p-captcha":false}"#,
        r#"{"p-message":"slow"}"#,
        r#"{"p-ticket":"","p-time":1}"#,
        r#"{"p-ticket":7,"p-time":1}"#,
        r#"{"p-ticket":"ticket","p-time":-1}"#,
        r#"{"p-ticket":"ticket","p-time":1.0}"#,
        r#"{"p-ticket":"ticket","p-time":1e0}"#,
        r#"{"p-ticket":"ticket","p-time":1e999}"#,
        r#"{"p-ticket":"ticket","p-time":"1"}"#,
        r#"{"p-ticket":"ticket","p-time":1,"p-captcha":null}"#,
        r#"{"p-ticket":"ticket","p-time":1,"p-captcha":"false"}"#,
        r#"{"p-ticket":"one","p-ticket":"two","p-time":1}"#,
        r#"{"p-ticket":"ticket","p-time":1,"errorText":"denied"}"#,
        r#"{"errorText":7}"#,
        r#"{"failureReason":""}"#,
        r#"{"failureReason":false}"#,
        r#"{"violations":17}"#,
    ];
    for payload in malformed {
        assert!(matches!(
            inspect(Some(raw(payload).as_ref())),
            Err(ControlError::MalformedControl)
        ));
    }
}

fn raw(value: &str) -> Box<RawValue> {
    RawValue::from_string(value.to_owned())
        .unwrap_or_else(|error| panic!("fixture JSON must be valid: {error}"))
}

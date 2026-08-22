// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use super::*;
use crate::{
    UserId,
    api::current::{
        alerts::{Alert, AlertStatus},
        ids::{AlertId, AlertSignalId},
    },
    client::MutationOutcome,
};

#[test]
fn create_requires_a_new_id_and_exact_request_echo() {
    let request = CreateAlert::builder()
        .expression("last > 100")
        .trigger_limits(2)
        .message("threshold")
        .build()
        .unwrap_or_else(|error| panic!("create-alert fixture: {error}"));
    let accepted = response(Some(alert(7, "last > 100", 2, "threshold")), None);
    assert_eq!(
        assess_create(&accepted, &request).outcome(),
        MutationOutcome::Success
    );

    let mismatch = response(Some(alert(7, "last > 101", 2, "threshold")), None);
    let assessment = assess_create(&mismatch, &request);
    assert_eq!(assessment.outcome(), MutationOutcome::Ambiguous);
    assert!(assessment.has_success_evidence());
}

#[test]
fn known_rejection_resolves_but_an_alert_contradiction_does_not() {
    let request = CreateAlert::builder()
        .expression("last > 100")
        .build()
        .unwrap_or_else(|error| panic!("create-alert fixture: {error}"));
    let rejected = response(None, Some("denied"));
    assert_eq!(
        assess_create(&rejected, &request).outcome(),
        MutationOutcome::Rejected
    );

    let contradiction = response(Some(alert(9, "other", 1, "other")), Some("denied"));
    let assessment = assess_create(&contradiction, &request);
    assert_eq!(assessment.outcome(), MutationOutcome::Ambiguous);
    assert!(assessment.has_success_evidence());
}

#[test]
fn modify_requires_the_exact_existing_alert_id() {
    let request = ModifyAlert::builder()
        .alert_id(alert_id(7))
        .expression("last > 100")
        .trigger_limits(2)
        .message("threshold")
        .build()
        .unwrap_or_else(|error| panic!("modify-alert fixture: {error}"));
    let accepted = response(Some(alert(7, "last > 100", 2, "threshold")), None);
    assert_eq!(
        assess_modify(&accepted, &request).outcome(),
        MutationOutcome::Success
    );

    let wrong_id = response(Some(alert(8, "last > 100", 2, "threshold")), None);
    assert_eq!(
        assess_modify(&wrong_id, &request).outcome(),
        MutationOutcome::Ambiguous
    );
}

#[test]
fn mark_read_never_claims_success_without_signal_identity() {
    let request = MarkReadAlertSignal::builder()
        .alert_id(alert_id(7))
        .alert_signal_id(alert_signal_id(11))
        .build()
        .unwrap_or_else(|error| panic!("mark-read fixture: {error}"));
    let returned_parent = response(Some(alert(7, "last > 100", 2, "threshold")), None);
    let assessment = assess_mark_read(&returned_parent, &request);
    assert_eq!(assessment.outcome(), MutationOutcome::Ambiguous);
    assert!(assessment.has_success_evidence());
}

#[test]
fn serde_bypassed_empty_expression_is_revalidated() {
    let decoded = serde_json::from_str::<CreateAlert>(r#"{"expression":""}"#);
    let Ok(decoded) = decoded else {
        panic!("wire fixture should bypass builder validation");
    };
    assert!(decoded.validate_current().is_err());
}

fn response(alert: Option<Alert>, error: Option<&str>) -> AlertResponse {
    let builder = AlertResponse::builder();
    let builder = match alert {
        Some(alert) => builder.alert(alert),
        None => builder,
    };
    let builder = match error {
        Some(error) => builder.error_text(error),
        None => builder,
    };
    builder
        .build()
        .unwrap_or_else(|error| panic!("alert response fixture: {error}"))
}

fn alert(id: i64, expression: &str, trigger_limits: i64, message: &str) -> Alert {
    Alert::builder()
        .id(alert_id(id))
        .timestamp(timestamp())
        .user_id(user_id(3))
        .status(AlertStatus::Active)
        .expression(expression)
        .trigger_limits(trigger_limits)
        .message(message)
        .build()
        .unwrap_or_else(|error| panic!("alert fixture: {error}"))
}

fn timestamp() -> jiff::Timestamp {
    "2026-08-21T00:00:00Z"
        .parse()
        .unwrap_or_else(|error| panic!("timestamp fixture: {error}"))
}

fn user_id(value: i64) -> UserId {
    UserId::new(value).unwrap_or_else(|error| panic!("user ID fixture: {error}"))
}

fn alert_id(value: i64) -> AlertId {
    AlertId::new(value).unwrap_or_else(|error| panic!("alert ID fixture: {error}"))
}

fn alert_signal_id(value: i64) -> AlertSignalId {
    AlertSignalId::new(value).unwrap_or_else(|error| panic!("signal ID fixture: {error}"))
}

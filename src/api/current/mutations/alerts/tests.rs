// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use super::*;
use crate::{
    api::current::ids::{AdminAlertSignalId, AlertId},
    client::MutationOutcome,
};

#[test]
fn administrative_signal_response_must_echo_the_request_identity() {
    let id =
        AdminAlertSignalId::new(31).unwrap_or_else(|error| panic!("signal ID fixture: {error}"));
    let request = CompleteAlertSignal::builder()
        .admin_alert_signal_id(id)
        .build()
        .unwrap_or_else(|error| panic!("complete request fixture: {error}"));

    assert_eq!(
        assess_complete_alert_signal(&admin_response(31, None), &request).outcome(),
        MutationOutcome::Success
    );
    assert_eq!(
        assess_complete_alert_signal(&admin_response(32, None), &request).outcome(),
        MutationOutcome::Ambiguous
    );

    let contradiction = assess_complete_alert_signal(&admin_response(31, Some("denied")), &request);
    assert_eq!(contradiction.outcome(), MutationOutcome::Ambiguous);
    assert!(contradiction.has_success_evidence());
}

#[test]
fn alert_response_must_echo_the_request_identity() {
    let id = AlertId::new(41).unwrap_or_else(|error| panic!("alert ID fixture: {error}"));
    let request = DeleteAlert::builder()
        .alert_id(id)
        .build()
        .unwrap_or_else(|error| panic!("delete alert request fixture: {error}"));

    assert_eq!(
        assess_delete_alert(&alert_response(41), &request).outcome(),
        MutationOutcome::Success
    );
    assert_eq!(
        assess_delete_alert(&alert_response(42), &request).outcome(),
        MutationOutcome::Ambiguous
    );
}

fn admin_response(id: i64, error: Option<&str>) -> AdminAlertSignalResponse {
    let mut value = serde_json::json!({
        "adminAlertSignal": {
            "id": id,
            "timestamp": "2026-08-21T00:00:00Z",
            "adminAlertId": 1,
            "text": "Review account",
            "emailSent": false,
            "subjectId": 2
        }
    });
    if let Some(error) = error {
        value["errorText"] = serde_json::Value::String(error.to_owned());
    }
    serde_json::from_value(value)
        .unwrap_or_else(|error| panic!("admin alert response fixture: {error}"))
}

fn alert_response(id: i64) -> AlertResponse {
    serde_json::from_value(serde_json::json!({
        "alert": {
            "id": id,
            "timestamp": "2026-08-21T00:00:00Z",
            "userId": 7,
            "status": "Active",
            "expression": "ES > 1"
        }
    }))
    .unwrap_or_else(|error| panic!("alert response fixture: {error}"))
}

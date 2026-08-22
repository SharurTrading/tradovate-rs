// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use super::*;
use crate::{
    AccountId, Environment, UserId,
    api::current::{
        ids::{TradingPermissionId, TradovateSubscriptionId},
        users::{TradingPermission, TradingPermissionStatus},
    },
    client::MutationOutcome,
};

#[tokio::test]
async fn lockout_expiry_is_rejected_on_the_live_rest_environment() {
    let client = Client::builder(Environment::Live)
        .build()
        .unwrap_or_else(|error| panic!("client fixture: {error}"));
    let request = ExpireUserLockout::builder()
        .user_id(user_id(7))
        .build()
        .unwrap_or_else(|error| panic!("lockout request fixture: {error}"));

    let result = client.user_expire_user_lockout(&request).await;
    assert!(matches!(
        result,
        Err(Error::InvalidRequest {
            field: "environment",
            ..
        })
    ));
}

#[test]
fn nested_permission_identity_must_match_the_request() {
    let requested = TradingPermissionId::new(41)
        .unwrap_or_else(|error| panic!("permission ID fixture: {error}"));
    let request = AcceptTradingPermission::builder()
        .trading_permission_id(requested)
        .build()
        .unwrap_or_else(|error| panic!("accept request fixture: {error}"));

    let exact = permission_response(41, 7, 9, "CTA", "cta@example.com");
    assert_eq!(
        assess_accept_trading_permission(&exact, &request).outcome(),
        MutationOutcome::Success
    );

    let mismatch = permission_response(42, 7, 9, "CTA", "cta@example.com");
    assert_eq!(
        assess_accept_trading_permission(&mismatch, &request).outcome(),
        MutationOutcome::Ambiguous
    );
}

#[test]
fn trading_permission_creation_requires_both_echoed_identities() {
    let account_id = account_id(9);
    let user_id = user_id(7);
    let request = CreateTradingPermission::builder()
        .account_id(account_id)
        .user_id(user_id)
        .build()
        .unwrap_or_else(|error| panic!("create request fixture: {error}"));

    let exact = permission_response(41, 7, 9, "CTA", "cta@example.com");
    assert_eq!(
        assess_create_trading_permission(&exact, &request).outcome(),
        MutationOutcome::Success
    );

    let mismatch = permission_response(41, 8, 9, "CTA", "cta@example.com");
    assert_eq!(
        assess_create_trading_permission(&mismatch, &request).outcome(),
        MutationOutcome::Ambiguous
    );
}

#[test]
fn trading_permission_request_requires_all_echoed_contact_fields() {
    let request = RequestTradingPermission::builder()
        .account_id(account_id(9))
        .cta_contact("CTA")
        .cta_email("cta@example.com")
        .build()
        .unwrap_or_else(|error| panic!("permission request fixture: {error}"));

    let exact = permission_response(41, 7, 9, "CTA", "cta@example.com");
    assert_eq!(
        assess_request_trading_permission(&exact, &request).outcome(),
        MutationOutcome::Success
    );

    let mismatch = permission_response(41, 7, 9, "Other", "cta@example.com");
    assert_eq!(
        assess_request_trading_permission(&mismatch, &request).outcome(),
        MutationOutcome::Ambiguous
    );
}

#[test]
fn subscription_success_code_cannot_replace_the_echoed_identity() {
    let requested = TradovateSubscriptionId::new(51)
        .unwrap_or_else(|error| panic!("subscription ID fixture: {error}"));
    let request = CancelTradovateSubscription::builder()
        .tradovate_subscription_id(requested)
        .build()
        .unwrap_or_else(|error| panic!("cancel request fixture: {error}"));

    let exact = subscription_response(51, "Success");
    assert_eq!(
        assess_cancel_subscription(&exact, &request).outcome(),
        MutationOutcome::Success
    );

    let mismatch = subscription_response(52, "Success");
    let assessment = assess_cancel_subscription(&mismatch, &request);
    assert_eq!(assessment.outcome(), MutationOutcome::Ambiguous);
    assert!(assessment.has_success_evidence());

    let rejection = serde_json::from_str::<TradovateSubscriptionResponse>(
        r#"{"errorCode":"InsufficientFunds"}"#,
    )
    .unwrap_or_else(|error| panic!("subscription rejection fixture: {error}"));
    assert_eq!(
        assess_cancel_subscription(&rejection, &request).outcome(),
        MutationOutcome::Rejected
    );
}

fn permission_response(
    permission_id: i64,
    user: i64,
    account: i64,
    contact: &str,
    email: &str,
) -> TradingPermissionResponse {
    let permission = TradingPermission::builder()
        .id(TradingPermissionId::new(permission_id)
            .unwrap_or_else(|error| panic!("permission ID fixture: {error}")))
        .user_id(user_id(user))
        .account_id(account_id(account))
        .account_holder_contact("Holder")
        .account_holder_email("holder@example.com")
        .cta_contact(contact)
        .cta_email(email)
        .status(TradingPermissionStatus::Requested)
        .build()
        .unwrap_or_else(|error| panic!("permission fixture: {error}"));
    TradingPermissionResponse::builder()
        .trading_permission(permission)
        .build()
        .unwrap_or_else(|error| panic!("permission response fixture: {error}"))
}

fn subscription_response(id: i64, error_code: &str) -> TradovateSubscriptionResponse {
    serde_json::from_value(serde_json::json!({
        "errorCode": error_code,
        "tradovateSubscription": {
            "id": id,
            "userId": 7,
            "timestamp": "2026-08-21T00:00:00Z",
            "planPrice": 1.25,
            "tradovateSubscriptionPlanId": 3,
            "startDate": { "year": 2026, "month": 8, "day": 1 },
            "expirationDate": { "year": 2026, "month": 9, "day": 1 },
            "paidAmount": 1.25
        }
    }))
    .unwrap_or_else(|error| panic!("subscription response fixture: {error}"))
}

fn user_id(value: i64) -> UserId {
    UserId::new(value).unwrap_or_else(|error| panic!("user ID fixture: {error}"))
}

fn account_id(value: i64) -> AccountId {
    AccountId::new(value).unwrap_or_else(|error| panic!("account ID fixture: {error}"))
}

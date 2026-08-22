// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use std::str::FromStr;

use httpmock::prelude::*;
use jiff::Timestamp;
use serde_json::json;

use super::*;
use crate::{
    AccountId, EndpointSet, Environment, UserId,
    auth::{InstalledSession, SessionInfo},
};

#[test]
fn pinned_cash_change_and_comment_bounds_are_enforced() {
    for value in ["-1000000", "1000000"] {
        let request = request(1, value, None);
        assert!(matches!(
            validate_request(&request),
            Err(Error::InvalidRequest {
                field: "cashChange",
                ..
            })
        ));
    }
    assert!(validate_request(&request(1, "999999.999", Some(&"x".repeat(64)))).is_ok());
    assert!(matches!(
        validate_request(&request(1, "1", Some(&"x".repeat(65)))),
        Err(Error::InvalidRequest {
            field: "comment",
            ..
        })
    ));
}

#[tokio::test]
async fn live_environment_is_rejected_before_authentication() {
    let client = Client::builder(Environment::Live)
        .build()
        .unwrap_or_else(|error| panic!("fixture client: {error}"));
    let result = client
        .cash_balance_change_demo_balance(&request(1, "1", None))
        .await;
    assert!(matches!(
        result,
        Err(Error::InvalidRequest {
            field: "environment",
            ..
        })
    ));
}

#[tokio::test]
async fn unresolved_response_and_account_limit_are_shared_by_clones() {
    let server = MockServer::start_async().await;
    let mutation = server
        .mock_async(|when, then| {
            when.method(POST)
                .path("/v1/cashBalance/changedemobalance")
                .header("authorization", "Bearer synthetic-access-token");
            then.status(200).json_body(json!({ "netLiq": 10 }));
        })
        .await;
    let client = authenticated_client(&server);

    assert!(matches!(
        client
            .cash_balance_change_demo_balance(&request(1, "1000000", None))
            .await,
        Err(Error::InvalidRequest {
            field: "cashChange",
            ..
        })
    ));
    assert!(matches!(
        client
            .cash_balance_change_demo_balance(&request(1, "10.25", None))
            .await,
        Err(Error::AmbiguousMutation { .. })
    ));
    assert!(client.mutation_reconciliation_required());
    assert!(matches!(
        client
            .clone()
            .cash_balance_change_demo_balance(&request(2, "1", None))
            .await,
        Err(Error::MutationReconciliationRequired { .. })
    ));
    client.acknowledge_mutation_reconciliation();
    assert!(matches!(
        client
            .cash_balance_change_demo_balance(&request(1, "1", None))
            .await,
        Err(Error::LocalRateLimit { .. })
    ));
    assert!(matches!(
        client
            .cash_balance_change_demo_balance(&request(2, "1", None))
            .await,
        Err(Error::AmbiguousMutation { .. })
    ));
    assert!(client.mutation_reconciliation_required());
    mutation.assert_calls_async(2).await;
}

fn request(account: i64, change: &str, comment: Option<&str>) -> ChangeDemoBalance {
    let account = AccountId::new(account).unwrap_or_else(|error| panic!("fixture ID: {error}"));
    let change =
        Decimal::from_str(change).unwrap_or_else(|error| panic!("fixture decimal: {error}"));
    let builder = ChangeDemoBalance::builder()
        .account_id(account)
        .cash_change(change);
    let builder = match comment {
        Some(comment) => builder.comment(comment),
        None => builder,
    };
    builder
        .build()
        .unwrap_or_else(|error| panic!("fixture request: {error}"))
}

fn authenticated_client(server: &MockServer) -> Client {
    let base = server.base_url();
    let websocket = base.replacen("http://", "ws://", 1);
    let endpoints = EndpointSet::custom(
        &format!("{base}/v1"),
        &format!("{websocket}/v1/websocket"),
        &format!("{websocket}/v1/websocket"),
        &format!("{websocket}/v1/websocket"),
    )
    .unwrap_or_else(|error| panic!("fixture endpoints: {error}"));
    let client = Client::builder_with_endpoints(endpoints)
        .build()
        .unwrap_or_else(|error| panic!("fixture client: {error}"));
    let expires_at = "2035-01-01T00:00:00Z"
        .parse::<Timestamp>()
        .unwrap_or_else(|error| panic!("fixture timestamp: {error}"));
    let user_id = UserId::new(1).unwrap_or_else(|error| panic!("fixture user: {error}"));
    let session = InstalledSession::try_new(
        "synthetic-access-token".to_owned(),
        None,
        SessionInfo::new(user_id, expires_at, false),
    )
    .unwrap_or_else(|error| panic!("fixture session: {error}"));
    assert!(client.tokens.begin_authentication().commit(session).is_ok());
    client
}

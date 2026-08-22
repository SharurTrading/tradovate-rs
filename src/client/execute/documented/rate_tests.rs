// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Failed-response-only endpoint admission integration tests.

use std::time::Duration;

use httpmock::prelude::*;
use serde_json::json;

use super::tests::authenticated_client;
use crate::Error;

#[tokio::test]
async fn auth_me_success_releases_its_failed_only_reservation() {
    let server = MockServer::start_async().await;
    let identity = server
        .mock_async(|when, then| {
            when.method(GET)
                .path("/v1/auth/me")
                .header("authorization", "Bearer synthetic-access-token");
            then.status(200).json_body(json!({ "userId": 1 }));
        })
        .await;
    let client = authenticated_client(&server);

    for _ in 0..11 {
        assert!(client.auth_me().await.is_ok());
    }
    identity.assert_calls_async(11).await;
}

#[tokio::test]
async fn auth_me_failure_budget_blocks_before_an_eleventh_send() {
    let server = MockServer::start_async().await;
    let identity = server
        .mock_async(|when, then| {
            when.method(GET)
                .path("/v1/auth/me")
                .header("authorization", "Bearer synthetic-access-token");
            then.status(400);
        })
        .await;
    let client = authenticated_client(&server);

    for _ in 0..10 {
        assert!(matches!(
            client.auth_me().await,
            Err(Error::HttpStatus { status: 400, .. })
        ));
    }
    assert!(
        tokio::time::timeout(Duration::from_millis(20), client.auth_me())
            .await
            .is_err()
    );
    identity.assert_calls_async(10).await;
}

// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Fail-closed current authentication and credential-rotation contracts.

use std::time::Duration;

use httpmock::prelude::*;
use serde_json::json;
use tokio::time::timeout;
use tradovate_client::{
    Client, EndpointSet, Error, UserId,
    api::current::{SecretValue, authentication::OAuthToken, users::ModifyPassword},
};

fn fixture_client(server: &MockServer) -> Client {
    let base = server.base_url();
    let websocket = base.replacen("http://", "ws://", 1);
    let endpoints = EndpointSet::custom(
        &format!("{base}/v1"),
        &format!("{websocket}/v1/websocket"),
        &format!("{websocket}/v1/websocket"),
        &format!("{websocket}/v1/websocket"),
    )
    .unwrap_or_else(|error| panic!("fixture endpoints must be valid: {error}"));
    Client::builder_with_endpoints(endpoints)
        .build()
        .unwrap_or_else(|error| panic!("fixture client must build: {error}"))
}

fn secret(value: &str) -> SecretValue {
    SecretValue::new(value).unwrap_or_else(|error| panic!("fixture secret must validate: {error}"))
}

fn oauth_grant() -> OAuthToken {
    OAuthToken::builder()
        .grant_type("authorization_code")
        .code(secret("synthetic-authorization-code"))
        .client_id("synthetic-client")
        .client_secret(secret("synthetic-client-secret"))
        .build()
        .unwrap_or_else(|error| panic!("fixture OAuth grant must build: {error}"))
}

fn password_request(user_id: Option<UserId>) -> ModifyPassword {
    let builder = ModifyPassword::builder()
        .password(secret("synthetic-new-password"))
        .current_password(secret("synthetic-old-password"));
    let builder = match user_id {
        Some(user_id) => builder.user_id(user_id),
        None => builder,
    };
    builder
        .build()
        .unwrap_or_else(|error| panic!("fixture password request must build: {error}"))
}

async fn authenticated_client(server: &MockServer) -> Client {
    let exchange = server
        .mock_async(|when, then| {
            when.method(POST).path("/v1/auth/oauthtoken");
            then.status(200).json_body(json!({
                "access_token": "synthetic-oauth-access",
                "token_type": "Bearer",
                "expires_in": 3600
            }));
        })
        .await;
    let identity = server
        .mock_async(|when, then| {
            when.method(GET)
                .path("/v1/auth/me")
                .header("authorization", "Bearer synthetic-oauth-access");
            then.status(200).json_body(json!({ "userId": 7 }));
        })
        .await;
    let client = fixture_client(server);
    client
        .authenticate_oauth(&oauth_grant())
        .await
        .unwrap_or_else(|error| panic!("fixture client must authenticate: {error}"));
    exchange.assert_async().await;
    identity.assert_async().await;
    client
}

#[tokio::test]
async fn malformed_rotated_bearer_invalidates_and_latches() {
    let server = MockServer::start_async().await;
    let client = authenticated_client(&server).await;
    let change = server
        .mock_async(|when, then| {
            when.method(POST).path("/v1/user/modifypassword");
            then.status(200).json_body(json!({
                "accessToken": "invalid token with whitespace",
                "expirationTime": "2035-08-21T01:30:00Z",
                "userId": 7
            }));
        })
        .await;

    assert!(matches!(
        client.modify_password(&password_request(None)).await,
        Err(Error::AmbiguousMutation { .. })
    ));
    change.assert_async().await;
    assert!(client.session_info().is_none());
    assert!(client.mutation_reconciliation_required());
}

#[tokio::test]
async fn cross_user_rotation_invalidates_and_latches() {
    let server = MockServer::start_async().await;
    let client = authenticated_client(&server).await;
    let change = server
        .mock_async(|when, then| {
            when.method(POST).path("/v1/user/modifypassword");
            then.status(200).json_body(json!({
                "accessToken": "synthetic-cross-user-access",
                "expirationTime": "2035-08-21T01:30:00Z",
                "userId": 8
            }));
        })
        .await;

    assert!(matches!(
        client.modify_password(&password_request(None)).await,
        Err(Error::AmbiguousMutation { .. })
    ));
    change.assert_async().await;
    assert!(client.session_info().is_none());
    assert!(client.mutation_reconciliation_required());
}

#[tokio::test]
async fn explicit_cross_user_request_is_rejected_before_send() {
    let server = MockServer::start_async().await;
    let client = authenticated_client(&server).await;
    let change = server
        .mock_async(|when, then| {
            when.method(POST).path("/v1/user/modifypassword");
            then.status(500);
        })
        .await;
    let other_user = UserId::new(8)
        .unwrap_or_else(|error| panic!("fixture user identifier must validate: {error}"));

    assert!(matches!(
        client
            .modify_password(&password_request(Some(other_user)))
            .await,
        Err(Error::InvalidRequest {
            field: "userId",
            ..
        })
    ));
    change.assert_calls_async(0).await;
    assert_eq!(
        client.session_info().map(|session| session.user_id().get()),
        Some(7)
    );
    assert!(!client.mutation_reconciliation_required());
}

#[tokio::test]
async fn credential_success_mixed_with_failure_control_is_ambiguous() {
    let server = MockServer::start_async().await;
    let client = authenticated_client(&server).await;
    let change = server
        .mock_async(|when, then| {
            when.method(POST).path("/v1/user/modifypassword");
            then.status(200).json_body(json!({
                "accessToken": "synthetic-replacement-access",
                "expirationTime": "2035-08-21T01:30:00Z",
                "userId": 7,
                "errorText": "synthetic rejection"
            }));
        })
        .await;

    assert!(matches!(
        client.modify_password(&password_request(None)).await,
        Err(Error::AmbiguousMutation { .. })
    ));
    change.assert_async().await;
    assert!(client.session_info().is_none());
    assert!(client.mutation_reconciliation_required());
}

#[tokio::test]
async fn cancellation_after_rotation_send_invalidates_and_latches() {
    let server = MockServer::start_async().await;
    let client = authenticated_client(&server).await;
    let change = server
        .mock_async(|when, then| {
            when.method(POST).path("/v1/user/modifypassword");
            then.status(200)
                .delay(Duration::from_millis(500))
                .json_body(json!({
                    "accessToken": "synthetic-replacement-access",
                    "expirationTime": "2035-08-21T01:30:00Z",
                    "userId": 7
                }));
        })
        .await;
    {
        let request = password_request(None);
        let operation = client.modify_password(&request);
        tokio::pin!(operation);
        assert!(
            timeout(Duration::from_millis(50), &mut operation)
                .await
                .is_err()
        );
    }

    change.assert_async().await;
    assert!(client.session_info().is_none());
    assert!(client.mutation_reconciliation_required());
}

#[tokio::test]
async fn renewal_rejects_a_cross_user_session() {
    let server = MockServer::start_async().await;
    let client = authenticated_client(&server).await;
    let renewal = server
        .mock_async(|when, then| {
            when.method(GET)
                .path("/v1/auth/renewaccesstoken")
                .header("authorization", "Bearer synthetic-oauth-access");
            then.status(200).json_body(json!({
                "accessToken": "synthetic-cross-user-access",
                "expirationTime": "2035-08-21T01:30:00Z",
                "userId": 8
            }));
        })
        .await;

    assert!(matches!(
        client.renew_access_token().await,
        Err(Error::InvalidAuthenticationResponse { .. })
    ));
    renewal.assert_async().await;
    assert!(client.session_info().is_none());
}

#[tokio::test]
async fn oauth_token_mixed_with_error_description_is_rejected() {
    let server = MockServer::start_async().await;
    let exchange = server
        .mock_async(|when, then| {
            when.method(POST).path("/v1/auth/oauthtoken");
            then.status(200).json_body(json!({
                "access_token": "synthetic-oauth-access",
                "token_type": "Bearer",
                "expires_in": 3600,
                "error_description": "synthetic provider failure"
            }));
        })
        .await;
    let identity = server
        .mock_async(|when, then| {
            when.method(GET).path("/v1/auth/me");
            then.status(500);
        })
        .await;
    let client = fixture_client(&server);

    assert!(matches!(
        client.authenticate_oauth(&oauth_grant()).await,
        Err(Error::InvalidProviderControl { .. })
    ));
    exchange.assert_async().await;
    identity.assert_calls_async(0).await;
    assert!(client.session_info().is_none());
}

// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Current generated API and OAuth contract regressions.

use std::str::FromStr;

use httpmock::prelude::*;
use serde_json::json;
use tradovate_client::{
    Client, Decimal, EndpointSet, Error,
    api::current::{
        SecretValue,
        authentication::OAuthToken,
        users::{Account, AccountAccountType, ModifyPassword},
    },
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

fn oauth_grant() -> OAuthToken {
    let code = SecretValue::new("synthetic-authorization-code")
        .unwrap_or_else(|error| panic!("fixture OAuth code must validate: {error}"));
    let client_secret = SecretValue::new("synthetic-client-secret")
        .unwrap_or_else(|error| panic!("fixture OAuth secret must validate: {error}"));
    OAuthToken::builder()
        .grant_type("authorization_code")
        .code(code)
        .client_id("synthetic-client")
        .client_secret(client_secret)
        .build()
        .unwrap_or_else(|error| panic!("fixture OAuth grant must build: {error}"))
}

fn secret(value: &str) -> SecretValue {
    SecretValue::new(value).unwrap_or_else(|error| panic!("fixture secret must validate: {error}"))
}

#[tokio::test]
async fn oauth_exchange_verifies_identity_and_installs_the_bearer() {
    let server = MockServer::start_async().await;
    let exchange = server
        .mock_async(|when, then| {
            when.method(POST)
                .path("/v1/auth/oauthtoken")
                .json_body(json!({
                    "grant_type": "authorization_code",
                    "code": "synthetic-authorization-code",
                    "client_id": "synthetic-client",
                    "client_secret": "synthetic-client-secret"
                }));
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

    let client = fixture_client(&server);
    let session = client
        .authenticate_oauth(&oauth_grant())
        .await
        .unwrap_or_else(|error| panic!("OAuth fixture must authenticate: {error}"));

    exchange.assert_async().await;
    identity.assert_async().await;
    assert_eq!(session.user_id().get(), 7);
    assert_eq!(
        client.session_info().map(|info| info.user_id().get()),
        Some(7)
    );
}

#[tokio::test]
async fn oauth_success_mixed_with_failure_control_fails_closed() {
    let server = MockServer::start_async().await;
    let exchange = server
        .mock_async(|when, then| {
            when.method(POST).path("/v1/auth/oauthtoken");
            then.status(200).json_body(json!({
                "access_token": "synthetic-rotated-access",
                "expires_in": 3600,
                "error": "synthetic_failure"
            }));
        })
        .await;
    let client = fixture_client(&server);

    assert!(matches!(
        client.authenticate_oauth(&oauth_grant()).await,
        Err(Error::InvalidProviderControl { .. })
    ));
    exchange.assert_async().await;
    assert!(client.session_info().is_none());
}

#[tokio::test]
async fn password_change_replaces_the_session_at_the_mutation_boundary() {
    let server = MockServer::start_async().await;
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
    let change = server
        .mock_async(|when, then| {
            when.method(POST)
                .path("/v1/user/modifypassword")
                .header("authorization", "Bearer synthetic-oauth-access")
                .json_body(json!({
                    "password": "synthetic-new-password",
                    "currentPassword": "synthetic-old-password"
                }));
            then.status(200).json_body(json!({
                "accessToken": "synthetic-replacement-access",
                "expirationTime": "2035-08-21T01:30:00Z",
                "userId": 7
            }));
        })
        .await;
    let replacement = server
        .mock_async(|when, then| {
            when.method(GET)
                .path("/v1/auth/me")
                .header("authorization", "Bearer synthetic-replacement-access");
            then.status(200).json_body(json!({ "userId": 7 }));
        })
        .await;
    let client = fixture_client(&server);
    client
        .authenticate_oauth(&oauth_grant())
        .await
        .unwrap_or_else(|error| panic!("OAuth fixture must authenticate: {error}"));
    let request = ModifyPassword::builder()
        .password(secret("synthetic-new-password"))
        .current_password(secret("synthetic-old-password"))
        .build()
        .unwrap_or_else(|error| panic!("password change must build: {error}"));

    let session = client
        .modify_password(&request)
        .await
        .unwrap_or_else(|error| panic!("password change must rotate the session: {error}"));
    assert_eq!(session.user_id().get(), 7);
    client
        .auth_me()
        .await
        .unwrap_or_else(|error| panic!("replacement bearer must be installed: {error}"));
    exchange.assert_async().await;
    identity.assert_async().await;
    change.assert_async().await;
    replacement.assert_async().await;
}

#[test]
fn generated_financial_values_preserve_decimal_lexemes() {
    let account: Account = serde_json::from_str(
        r#"{
            "name":"SYNTHETIC",
            "userId":7,
            "accountType":"Customer",
            "clearingHouseId":1,
            "riskCategoryId":2,
            "autoLiqProfileId":3,
            "marginAccountType":"Speculator",
            "legalStatus":"Individual",
            "timestamp":"2026-08-21T00:00:00Z",
            "evaluationSize":12345678901234567890.12345678
        }"#,
    )
    .unwrap_or_else(|error| panic!("exact generated account must decode: {error}"));
    let expected = Decimal::from_str("12345678901234567890.12345678")
        .unwrap_or_else(|error| panic!("fixture decimal must parse: {error}"));
    assert_eq!(account.evaluation_size().copied(), Some(expected));
}

#[test]
fn unknown_enums_are_receive_only_and_secrets_are_redacted() {
    let value: AccountAccountType = serde_json::from_str(r#""ProviderAddedType""#)
        .unwrap_or_else(|error| panic!("unknown provider enum must decode: {error}"));
    assert_eq!(value.as_str(), "ProviderAddedType");
    assert!(serde_json::to_string(&value).is_err());

    let secret = SecretValue::new("must-not-appear")
        .unwrap_or_else(|error| panic!("fixture secret must validate: {error}"));
    assert_eq!(format!("{secret:?}"), "[REDACTED]");
}

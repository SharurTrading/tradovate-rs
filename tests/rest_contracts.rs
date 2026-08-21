// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Public REST contracts using synthetic loopback fixtures.

use std::{str::FromStr, time::Duration};

use httpmock::prelude::*;
use serde_json::json;
use tradovate_client::{
    AccountId, Client, ClientOrderId, Decimal, DeviceId, EndpointSet, Error, OrderId, Symbol,
    api::{OrderOrigin, OrderQuantity, OrderSide, OrderType, PlaceOrder},
    auth::Credentials,
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

fn credentials() -> Credentials {
    credentials_for("synthetic-user")
}

fn credentials_for(name: &str) -> Credentials {
    Credentials::builder(name, "synthetic-password")
        .app_id("synthetic-app")
        .app_version("1.0")
        .numeric_client_id(123)
        .secret("synthetic-secret")
        .device_id(
            DeviceId::new("synthetic-device")
                .unwrap_or_else(|error| panic!("fixture device ID must be valid: {error}")),
        )
        .build()
        .unwrap_or_else(|error| panic!("fixture credentials must be valid: {error}"))
}

async fn authenticated_client(server: &MockServer) -> Client {
    let login = server
        .mock_async(|when, then| {
            when.method(POST)
                .path("/v1/auth/accesstokenrequest")
                .json_body(json!({
                    "name": "synthetic-user",
                    "password": "synthetic-password",
                    "appId": "synthetic-app",
                    "appVersion": "1.0",
                    "cid": 123,
                    "sec": "synthetic-secret",
                    "deviceId": "synthetic-device",
                    "hibpCheck": true
                }));
            then.status(200).json_body(json!({
                "accessToken": "synthetic-access-token",
                "mdAccessToken": "synthetic-market-token",
                "expirationTime": "2035-08-21T01:30:00Z",
                "userId": 7,
            }));
        })
        .await;
    let client = fixture_client(server);
    client
        .authenticate(&credentials())
        .await
        .unwrap_or_else(|error| panic!("fixture authentication must succeed: {error}"));
    login.assert_async().await;
    client
}

fn limit_order() -> PlaceOrder {
    let account = AccountId::new(42).unwrap_or_else(|error| panic!("{error}"));
    let symbol = Symbol::new("ESZ6").unwrap_or_else(|error| panic!("{error}"));
    let quantity = OrderQuantity::new(2).unwrap_or_else(|error| panic!("{error}"));
    let price = Decimal::from_str("5000.25").unwrap_or_else(|error| panic!("{error}"));
    PlaceOrder::builder(
        account,
        symbol,
        OrderSide::Buy,
        quantity,
        OrderOrigin::Automated,
    )
    .client_order_id(
        ClientOrderId::new("synthetic-order-1").unwrap_or_else(|error| panic!("{error}")),
    )
    .order_type(OrderType::Limit)
    .price(price)
    .build()
    .unwrap_or_else(|error| panic!("fixture order must be valid: {error}"))
}

#[tokio::test]
async fn authentication_and_account_query_match_wire_contract() {
    let server = MockServer::start_async().await;
    let accounts = server
        .mock_async(|when, then| {
            when.method(GET)
                .path("/v1/account/list")
                .header("authorization", "Bearer synthetic-access-token");
            then.status(200)
                .header("content-type", "application/json")
                .body(
                    r#"[{"id":42,"name":"SYNTHETIC","userId":7,"accountType":"Customer","clearingHouseId":1,"riskCategoryId":2,"autoLiqProfileId":3,"marginAccountType":"Speculator","legalStatus":"Individual","timestamp":"2026-08-21T00:00:00Z","evaluationSize":123456789.123456789,"readonly":false}]"#,
                );
        })
        .await;
    let client = authenticated_client(&server).await;
    let result = client
        .account_list()
        .await
        .unwrap_or_else(|error| panic!("account query must succeed: {error}"));
    accounts.assert_async().await;
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id().map(|id| id.get()), Some(42));
    assert_eq!(
        result[0]
            .evaluation_size()
            .copied()
            .map(|value| value.scale()),
        Some(9)
    );
}

#[tokio::test]
async fn placement_uses_explicit_route_origin_and_exact_price() {
    let server = MockServer::start_async().await;
    let placement = server
        .mock_async(|when, then| {
            when.method(POST)
                .path("/v1/order/placeorder")
                .header("authorization", "Bearer synthetic-access-token")
                .body(
                    r#"{"accountId":42,"symbol":"ESZ6","clOrdId":"synthetic-order-1","action":"Buy","orderQty":2,"orderType":"Limit","price":5000.25,"timeInForce":"Day","isAutomated":true}"#,
                );
            then.status(200).json_body(json!({
                "failureReason": "Success",
                "orderId": 84
            }));
        })
        .await;
    let client = authenticated_client(&server).await;
    let request = limit_order();
    let result = client
        .place_order(&request)
        .await
        .unwrap_or_else(|error| panic!("placement must succeed: {error}"));
    placement.assert_async().await;
    let expected = OrderId::new(84).unwrap_or_else(|error| panic!("{error}"));
    assert_eq!(result.order_id(), expected);
}

#[tokio::test]
async fn cancelling_an_admitted_mutation_requires_reconciliation() {
    let server = MockServer::start_async().await;
    let placement = server
        .mock_async(|when, then| {
            when.method(POST).path("/v1/order/placeorder");
            then.status(200)
                .delay(Duration::from_millis(500))
                .json_body(json!({
                    "failureReason": "Success",
                    "orderId": 84
                }));
        })
        .await;
    let client = authenticated_client(&server).await;
    let request = limit_order();
    let pending = tokio::spawn({
        let client = client.clone();
        async move { client.place_order(&request).await }
    });

    let observed = tokio::time::timeout(Duration::from_secs(1), async {
        while placement.calls_async().await == 0 {
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await;
    assert!(observed.is_ok(), "fixture never observed the mutation");
    pending.abort();
    assert!(pending.await.is_err(), "cancelled task must not complete");

    assert!(client.mutation_reconciliation_required());
    let refused = client.place_order(&limit_order()).await;
    assert!(matches!(
        refused,
        Err(Error::MutationReconciliationRequired { .. })
    ));
    assert_eq!(placement.calls_async().await, 1);

    client.acknowledge_mutation_reconciliation();
    assert!(!client.mutation_reconciliation_required());
}

#[tokio::test]
async fn contradictory_success_response_latches_mutations() {
    let server = MockServer::start_async().await;
    let placement = server
        .mock_async(|when, then| {
            when.method(POST).path("/v1/order/placeorder");
            then.status(200)
                .json_body(json!({ "failureReason": "Success" }));
        })
        .await;
    let client = authenticated_client(&server).await;

    let result = client.place_order(&limit_order()).await;
    placement.assert_async().await;
    assert!(matches!(result, Err(Error::AmbiguousMutation { .. })));
    assert!(client.mutation_reconciliation_required());
}

#[tokio::test]
async fn mixed_success_and_failure_controls_are_ambiguous_and_latch_mutations() {
    let server = MockServer::start_async().await;
    let placement = server
        .mock_async(|when, then| {
            when.method(POST).path("/v1/order/placeorder");
            then.status(200).json_body(json!({
                "failureReason": "Success",
                "errorText": "synthetic rejection"
            }));
        })
        .await;
    let client = authenticated_client(&server).await;

    assert!(matches!(
        client.place_order(&limit_order()).await,
        Err(Error::AmbiguousMutation { .. })
    ));
    placement.assert_calls_async(1).await;
    assert!(client.mutation_reconciliation_required());
    assert!(matches!(
        client.place_order(&limit_order()).await,
        Err(Error::MutationReconciliationRequired { .. })
    ));
    assert_eq!(placement.calls_async().await, 1);
}

#[tokio::test]
async fn penalty_ticket_is_typed_and_redacted() {
    let server = MockServer::start_async().await;
    let penalty = server
        .mock_async(|when, then| {
            when.method(POST).path("/v1/auth/accesstokenrequest");
            then.status(200).json_body(json!({
                "p-ticket": "secret-penalty-ticket",
                "p-time": 15,
                "p-captcha": false,
                "p-message": "slow down"
            }));
        })
        .await;
    let client = fixture_client(&server);
    let result = client.authenticate(&credentials()).await;
    penalty.assert_async().await;
    match result {
        Err(Error::Penalty(ticket)) => {
            assert_eq!(ticket.wait(), Duration::from_secs(15));
            assert!(!format!("{ticket:?}").contains("secret-penalty-ticket"));
            let mismatched = client
                .authenticate_with_penalty(&credentials_for("different-user"), &ticket)
                .await;
            assert!(matches!(mismatched, Err(Error::InvalidRequest { .. })));
        }
        other => panic!("expected typed penalty, received {other:?}"),
    }
    assert_eq!(penalty.calls_async().await, 1);
}

#[tokio::test]
async fn penalty_retry_is_bound_to_the_originating_client_and_exact_body() {
    let server = MockServer::start_async().await;
    let initial = server
        .mock_async(|when, then| {
            when.method(POST)
                .path("/v1/auth/accesstokenrequest")
                .json_body(json!({
                    "name": "synthetic-user",
                    "password": "synthetic-password",
                    "appId": "synthetic-app",
                    "appVersion": "1.0",
                    "cid": 123,
                    "sec": "synthetic-secret",
                    "deviceId": "synthetic-device",
                    "hibpCheck": true
                }));
            then.status(200).json_body(json!({
                "p-ticket": "secret-penalty-ticket",
                "p-time": 0,
                "p-captcha": false
            }));
        })
        .await;
    let retry = server
        .mock_async(|when, then| {
            when.method(POST)
                .path("/v1/auth/accesstokenrequest")
                .json_body(json!({
                    "name": "synthetic-user",
                    "password": "synthetic-password",
                    "appId": "synthetic-app",
                    "appVersion": "1.0",
                    "cid": 123,
                    "sec": "synthetic-secret",
                    "deviceId": "synthetic-device",
                    "hibpCheck": true,
                    "p-ticket": "secret-penalty-ticket"
                }));
            then.status(200).json_body(json!({
                "accessToken": "synthetic-access-token",
                "expirationTime": "2035-08-21T01:30:00Z",
                "userId": 7
            }));
        })
        .await;
    let owner = fixture_client(&server);
    let ticket = match owner.authenticate(&credentials()).await {
        Err(Error::Penalty(ticket)) => ticket,
        other => panic!("expected typed penalty, received {other:?}"),
    };

    let unrelated = fixture_client(&server);
    assert!(matches!(
        unrelated
            .authenticate_with_penalty(&credentials(), &ticket)
            .await,
        Err(Error::InvalidRequest { .. })
    ));
    assert!(
        owner
            .clone()
            .authenticate_with_penalty(&credentials(), &ticket)
            .await
            .is_ok()
    );
    initial.assert_calls_async(1).await;
    retry.assert_calls_async(1).await;
}

#[tokio::test]
async fn unusable_renewal_response_fails_closed_after_transmission() {
    let server = MockServer::start_async().await;
    let renewal = server
        .mock_async(|when, then| {
            when.method(GET)
                .path("/v1/auth/renewaccesstoken")
                .header("authorization", "Bearer synthetic-access-token");
            then.status(200).json_body(json!({
                "accessToken": "rotated-access-token",
                "expirationTime": "2000-01-01T00:00:00Z",
                "userId": 7
            }));
        })
        .await;
    let client = authenticated_client(&server).await;

    assert!(matches!(
        client.renew_access_token().await,
        Err(Error::InvalidAuthenticationResponse { .. })
    ));
    renewal.assert_async().await;
    assert!(client.session_info().is_none());
}

#[tokio::test]
async fn renewal_success_mixed_with_failure_control_invalidates_the_basis_session() {
    let server = MockServer::start_async().await;
    let renewal = server
        .mock_async(|when, then| {
            when.method(GET)
                .path("/v1/auth/renewaccesstoken")
                .header("authorization", "Bearer synthetic-access-token");
            then.status(200).json_body(json!({
                "accessToken": "rotated-access-token",
                "expirationTime": "2035-08-21T01:30:00Z",
                "userId": 7,
                "errorText": "synthetic contradiction"
            }));
        })
        .await;
    let client = authenticated_client(&server).await;

    assert!(matches!(
        client.renew_access_token().await,
        Err(Error::InvalidProviderControl { .. })
    ));
    renewal.assert_async().await;
    assert!(client.session_info().is_none());
}

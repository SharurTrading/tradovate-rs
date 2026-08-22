// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Synthetic loopback tests for the handwritten current order endpoints.

use httpmock::prelude::*;
use jiff::Timestamp;
use serde_json::json;

use super::*;
use crate::api::current::orders::DryRunResponseRejectReason;
use crate::auth::{InstalledSession, SessionInfo};
use crate::{Client, ContractId, EndpointSet, PositionId, UserId};

fn authenticated_client(server: &MockServer) -> Client {
    let base = server.base_url();
    let websocket = base.replacen("http://", "ws://", 1);
    let endpoints = EndpointSet::custom(
        &format!("{base}/v1"),
        &format!("{websocket}/v1/websocket"),
        &format!("{websocket}/v1/websocket"),
        &format!("{websocket}/v1/websocket"),
    )
    .unwrap_or_else(|error| panic!("fixture endpoints must validate: {error}"));
    let client = Client::builder_with_endpoints(endpoints)
        .build()
        .unwrap_or_else(|error| panic!("fixture client must build: {error}"));
    let expires_at = "2035-01-01T00:00:00Z"
        .parse::<Timestamp>()
        .unwrap_or_else(|error| panic!("fixture timestamp must parse: {error}"));
    let user_id = UserId::new(1).unwrap_or_else(|error| panic!("{error}"));
    let session = InstalledSession::try_new(
        "synthetic-access-token".to_owned(),
        None,
        SessionInfo::new(user_id, expires_at, false),
    )
    .unwrap_or_else(|error| panic!("fixture session must validate: {error}"));
    let authentication = client.tokens.begin_authentication();
    assert!(authentication.commit(session).is_ok());
    client
}

fn route() -> (AccountId, Symbol, OrderQuantity) {
    let account = AccountId::new(42).unwrap_or_else(|error| panic!("{error}"));
    let symbol = Symbol::new("ESZ6").unwrap_or_else(|error| panic!("{error}"));
    let quantity = OrderQuantity::new(2).unwrap_or_else(|error| panic!("{error}"));
    (account, symbol, quantity)
}

fn market_order() -> PlaceOrder {
    let (account, symbol, quantity) = route();
    PlaceOrder::builder(
        account,
        symbol,
        OrderSide::Buy,
        quantity,
        OrderOrigin::Automated,
    )
    .order_type(OrderType::Market)
    .build()
    .unwrap_or_else(|error| panic!("{error}"))
}

#[tokio::test]
async fn dry_run_is_a_query_and_preserves_a_risk_rejection() {
    let server = MockServer::start_async().await;
    let endpoint = server
        .mock_async(|when, then| {
            when.method(POST)
                .path("/v1/order/dryrun")
                .header("authorization", "Bearer synthetic-access-token")
                .body_includes(r#""accountId":42"#)
                .body_includes(r#""orders":[{"contractId":7"#);
            then.status(200).json_body(json!({
                "rejectReason": "MaxPosLimitReached",
                "comment": "synthetic risk rejection"
            }));
        })
        .await;
    let client = authenticated_client(&server);
    let contract = ContractId::new(7).unwrap_or_else(|error| panic!("{error}"));
    let (_, _, quantity) = route();
    let request = DryRun::new(
        AccountId::new(42).unwrap_or_else(|error| panic!("{error}")),
        vec![DryRunOrder::market(contract, OrderSide::Buy, quantity)],
    )
    .unwrap_or_else(|error| panic!("{error}"));

    let response = client
        .dry_run(&request)
        .await
        .unwrap_or_else(|error| panic!("dry run must decode: {error}"));

    assert_eq!(
        response.reject_reason(),
        Some(&DryRunResponseRejectReason::MaxPosLimitReached)
    );
    endpoint.assert_async().await;
}

#[tokio::test]
async fn modify_accepts_explicit_success_with_command_id() {
    let server = MockServer::start_async().await;
    let endpoint = server
        .mock_async(|when, then| {
            when.method(POST)
                .path("/v1/order/modifyorder")
                .body_includes(r#""orderId":9"#)
                .body_includes(r#""isAutomated":true"#);
            then.status(200).json_body(json!({
                "failureReason": "Success",
                "commandId": 91
            }));
        })
        .await;
    let client = authenticated_client(&server);
    let order_id = OrderId::new(9).unwrap_or_else(|error| panic!("{error}"));
    let (_, _, quantity) = route();
    let request = ModifyOrder::market(order_id, quantity, OrderOrigin::Automated);

    let command = client
        .modify_order(&request)
        .await
        .unwrap_or_else(|error| panic!("modification must resolve: {error}"));

    assert_eq!(command.get(), 91);
    assert!(!client.mutation_reconciliation_required());
    endpoint.assert_async().await;
}

#[tokio::test]
async fn unknown_failure_reason_with_identifier_is_ambiguous_and_latches() {
    let server = MockServer::start_async().await;
    let endpoint = server
        .mock_async(|when, then| {
            when.method(POST).path("/v1/order/modifyorder");
            then.status(200).json_body(json!({
                "failureReason": "FutureReason",
                "commandId": 91
            }));
        })
        .await;
    let client = authenticated_client(&server);
    let order_id = OrderId::new(9).unwrap_or_else(|error| panic!("{error}"));
    let (_, _, quantity) = route();
    let request = ModifyOrder::market(order_id, quantity, OrderOrigin::Automated);

    assert!(matches!(
        client.modify_order(&request).await,
        Err(Error::AmbiguousMutation { .. })
    ));
    assert!(client.mutation_reconciliation_required());
    endpoint.assert_async().await;
}

#[tokio::test]
async fn documented_order_rejection_preserves_its_typed_reason() {
    let server = MockServer::start_async().await;
    let endpoint = server
        .mock_async(|when, then| {
            when.method(POST).path("/v1/order/modifyorder");
            then.status(200).json_body(json!({
                "failureReason": "TradingLocked"
            }));
        })
        .await;
    let client = authenticated_client(&server);
    let order_id = OrderId::new(9).unwrap_or_else(|error| panic!("{error}"));
    let (_, _, quantity) = route();
    let request = ModifyOrder::market(order_id, quantity, OrderOrigin::Automated);

    assert!(matches!(
        client.modify_order(&request).await,
        Err(Error::OrderRejected {
            reason: OrderFailureReason::TradingLocked,
            ..
        })
    ));
    assert!(!client.mutation_reconciliation_required());
    endpoint.assert_async().await;
}

#[tokio::test]
async fn oco_and_batch_liquidation_use_fenced_mutations() {
    let server = MockServer::start_async().await;
    let oco = server
        .mock_async(|when, then| {
            when.method(POST)
                .path("/v1/order/placeoco")
                .body_includes(r#""isAutomated":true"#)
                .body_includes(r#""other":{"action":"Sell""#);
            then.status(200).json_body(json!({
                "failureReason": "Success",
                "orderId": 101,
                "ocoId": 202
            }));
        })
        .await;
    let liquidation = server
        .mock_async(|when, then| {
            when.method(POST)
                .path("/v1/order/liquidatepositions")
                .body_includes(r#""positions":[301,302]"#)
                .body_includes(r#""admin":false"#);
            then.status(200).json_body(json!({ "ok": true }));
        })
        .await;
    let client = authenticated_client(&server);
    let other = AttachedOrder::market(OrderSide::Sell);
    let placement = client
        .place_oco(&PlaceOco::new(market_order(), other))
        .await
        .unwrap_or_else(|error| panic!("OCO must resolve: {error}"));
    assert_eq!(placement.order_id().get(), 101);
    assert_eq!(placement.oco_id().get(), 202);

    let positions = [301, 302]
        .into_iter()
        .map(|id| PositionId::new(id).unwrap_or_else(|error| panic!("{error}")))
        .collect();
    let request = LiquidatePositions::new(
        positions,
        LiquidationAuthority::AccountHolder,
        OrderOrigin::Automated,
    )
    .unwrap_or_else(|error| panic!("{error}"));
    client
        .liquidate_positions(&request)
        .await
        .unwrap_or_else(|error| panic!("liquidation must resolve: {error}"));

    oco.assert_async().await;
    liquidation.assert_async().await;
}

#[tokio::test]
async fn strategy_params_are_typed_before_the_provider_string_boundary() {
    let server = MockServer::start_async().await;
    let endpoint = server
        .mock_async(|when, then| {
            when.method(POST)
                .path("/v1/orderStrategy/startorderstrategy")
                .body_includes(r#""orderStrategyTypeId":2"#)
                .body_includes(r#""isAutomated":true"#)
                .body_includes(r#"\"entryVersion\""#);
            then.status(200).json_body(json!({
                "failureReason": "Success",
                "orderStrategy": {
                    "id": 501,
                    "accountId": 42,
                    "timestamp": "2026-08-21T01:02:03Z",
                    "contractId": 601,
                    "orderStrategyTypeId": 2,
                    "action": "Sell",
                    "status": "ActiveStrategy"
                }
            }));
        })
        .await;
    let client = authenticated_client(&server);
    let (account, symbol, quantity) = route();
    let profit = "-30"
        .parse::<Decimal>()
        .unwrap_or_else(|error| panic!("{error}"));
    let stop = "15"
        .parse::<Decimal>()
        .unwrap_or_else(|error| panic!("{error}"));
    let params = MultiBracketParams::new(
        quantity,
        vec![MultiBracket::new(quantity, profit, stop, false)],
    )
    .unwrap_or_else(|error| panic!("{error}"));
    let request = StartMultiBracketStrategy::new(
        account,
        symbol,
        OrderSide::Sell,
        params,
        OrderOrigin::Automated,
    );

    let receipt = client
        .start_order_strategy(&request)
        .await
        .unwrap_or_else(|error| panic!("strategy must resolve: {error}"));

    assert_eq!(receipt.id().get(), 501);
    assert_eq!(receipt.status(), &OrderStrategyStatus::ActiveStrategy);
    endpoint.assert_async().await;
}

#[tokio::test]
async fn single_liquidation_oso_and_interrupt_require_typed_evidence() {
    let server = MockServer::start_async().await;
    let liquidation = server
        .mock_async(|when, then| {
            when.method(POST)
                .path("/v1/order/liquidateposition")
                .body_includes(r#""accountId":42"#)
                .body_includes(r#""contractId":7"#)
                .body_includes(r#""isAutomated":true"#);
            then.status(200).json_body(json!({
                "failureReason": "Success",
                "orderId": 601
            }));
        })
        .await;
    let oso = server
        .mock_async(|when, then| {
            when.method(POST)
                .path("/v1/order/placeoso")
                .body_includes(r#""bracket1":{"action":"Sell""#)
                .body_excludes("bracket2");
            then.status(200).json_body(json!({
                "failureReason": "Success",
                "orderId": 701,
                "oso1Id": 702
            }));
        })
        .await;
    let interrupt = server
        .mock_async(|when, then| {
            when.method(POST)
                .path("/v1/orderStrategy/interruptorderstrategy")
                .json_body(json!({ "orderStrategyId": 801 }));
            then.status(200).json_body(json!({
                "failureReason": "Success",
                "orderStrategy": {
                    "id": 801,
                    "accountId": 42,
                    "timestamp": "2026-08-21T01:02:03Z",
                    "contractId": 601,
                    "orderStrategyTypeId": 2,
                    "action": "Sell",
                    "status": "StoppedByUser"
                }
            }));
        })
        .await;
    let client = authenticated_client(&server);
    let request = LiquidatePosition::new(
        AccountId::new(42).unwrap_or_else(|error| panic!("{error}")),
        ContractId::new(7).unwrap_or_else(|error| panic!("{error}")),
        LiquidationAuthority::AccountHolder,
        OrderOrigin::Automated,
    );
    let placement = client
        .liquidate_position(&request)
        .await
        .unwrap_or_else(|error| panic!("single liquidation must resolve: {error}"));
    assert_eq!(placement.order_id().get(), 601);

    let limit = "5050.25"
        .parse::<Decimal>()
        .unwrap_or_else(|error| panic!("{error}"));
    let bracket =
        AttachedOrder::limit(OrderSide::Sell, limit).unwrap_or_else(|error| panic!("{error}"));
    let request = PlaceOso::new(market_order(), bracket)
        .unwrap_or_else(|error| panic!("OSO fixture must validate: {error}"));
    let placement = client
        .place_oso(&request)
        .await
        .unwrap_or_else(|error| panic!("OSO must resolve: {error}"));
    assert_eq!(placement.order_id().get(), 701);
    assert_eq!(placement.first_bracket_id().get(), 702);
    assert!(placement.second_bracket_id().is_none());

    let strategy_id = OrderStrategyId::new(801).unwrap_or_else(|error| panic!("{error}"));
    let receipt = client
        .interrupt_order_strategy(strategy_id)
        .await
        .unwrap_or_else(|error| panic!("interrupt must resolve: {error}"));
    assert_eq!(receipt.id(), strategy_id);
    assert_eq!(receipt.status(), &OrderStrategyStatus::StoppedByUser);

    liquidation.assert_async().await;
    oso.assert_async().await;
    interrupt.assert_async().await;
}

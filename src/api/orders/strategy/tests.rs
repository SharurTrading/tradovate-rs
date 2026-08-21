// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

use super::*;
use crate::auth::{InstalledSession, SessionInfo};
use crate::{Client, EndpointSet, UserId};
use httpmock::prelude::*;
use jiff::Timestamp;
use serde_json::{Value, json};

fn request() -> StartMultiBracketStrategy {
    let account = AccountId::new(1).unwrap_or_else(|error| panic!("{error}"));
    let symbol = Symbol::new("MESZ6").unwrap_or_else(|error| panic!("{error}"));
    let quantity = OrderQuantity::new(1).unwrap_or_else(|error| panic!("{error}"));
    let profit = "-30"
        .parse::<Decimal>()
        .unwrap_or_else(|error| panic!("{error}"));
    let stop = "15.25"
        .parse::<Decimal>()
        .unwrap_or_else(|error| panic!("{error}"));
    let params = MultiBracketParams::new(
        quantity,
        vec![MultiBracket::new(quantity, profit, stop, false)],
    )
    .unwrap_or_else(|error| panic!("{error}"));
    StartMultiBracketStrategy::new(
        account,
        symbol,
        OrderSide::Sell,
        params,
        OrderOrigin::Automated,
    )
}

fn strategy_value() -> Value {
    json!({
        "id": 2,
        "accountId": 1,
        "timestamp": "2026-08-21T01:02:03Z",
        "contractId": 3,
        "orderStrategyTypeId": 2,
        "action": "Sell",
        "status": "ActiveStrategy"
    })
}

fn response_with_strategy(strategy: &Value) -> StrategyResponse {
    serde_json::from_value(json!({
        "failureReason": "Success",
        "orderStrategy": strategy
    }))
    .unwrap_or_else(|error| panic!("fixture response must decode: {error}"))
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

#[test]
fn typed_params_are_bounded_and_encoded_as_the_provider_string() {
    let request = request();
    let wire = StartStrategyWire::new(&request).unwrap_or_else(|error| panic!("{error}"));
    let encoded = serde_json::to_value(wire).unwrap_or_else(|error| panic!("{error}"));
    let params = encoded
        .get("params")
        .and_then(serde_json::Value::as_str)
        .unwrap_or_else(|| panic!("typed params string must be present"));

    assert!(params.contains(r#""orderType":"Market""#));
    assert!(params.contains(r#""profitTarget":-30"#));
    assert!(params.contains(r#""stopLoss":15.25"#));
    assert_eq!(
        encoded
            .get("isAutomated")
            .and_then(serde_json::Value::as_bool),
        Some(true)
    );
}

#[test]
fn undocumented_modify_command_is_metadata_only() {
    assert_eq!(
        MODIFY_ORDER_STRATEGY_DOCUMENTATION_GAP.endpoint(),
        "/orderStrategy/modifyorderstrategy"
    );
    assert_eq!(MODIFY_ORDER_STRATEGY_DOCUMENTATION_GAP.field(), "command");
    assert!(
        MODIFY_ORDER_STRATEGY_DOCUMENTATION_GAP
            .reason()
            .contains("no structured")
    );
}

#[test]
fn unknown_strategy_status_cannot_resolve_a_mutation() {
    let mut strategy = strategy_value();
    strategy["status"] = json!("FutureState");
    let response = response_with_strategy(&strategy);
    assert_eq!(response.mutation_outcome(), MutationOutcome::Ambiguous);
}

#[test]
fn every_pinned_required_strategy_field_must_decode() {
    for field in [
        "accountId",
        "timestamp",
        "contractId",
        "orderStrategyTypeId",
        "action",
        "status",
    ] {
        let mut strategy = strategy_value();
        let object = strategy
            .as_object_mut()
            .unwrap_or_else(|| panic!("fixture strategy must be an object"));
        object.remove(field);
        assert!(
            serde_json::from_value::<StrategyResponse>(json!({
                "orderStrategy": strategy
            }))
            .is_err(),
            "missing required field {field} must fail decoding"
        );
    }
}

#[test]
fn start_identity_requires_account_type_action_and_returned_uuid_match() {
    let mut request = request();
    request.instance_id =
        Some(StrategyInstanceId::new("expected-uuid").unwrap_or_else(|error| panic!("{error}")));

    let mut matching = strategy_value();
    matching["uuid"] = json!("expected-uuid");
    let response = response_with_strategy(&matching);
    let strategy = response
        .order_strategy
        .as_ref()
        .unwrap_or_else(|| panic!("fixture strategy must exist"));
    assert!(start_identity_matches(&request, strategy));

    let response = response_with_strategy(&strategy_value());
    let strategy = response
        .order_strategy
        .as_ref()
        .unwrap_or_else(|| panic!("fixture strategy must exist"));
    assert!(
        !start_identity_matches(&request, strategy),
        "an omitted response UUID cannot prove a caller-supplied instance identity"
    );

    for (field, mismatch) in [
        ("accountId", json!(99)),
        ("orderStrategyTypeId", json!(3)),
        ("action", json!("Buy")),
        ("uuid", json!("different-uuid")),
    ] {
        let mut value = strategy_value();
        value["uuid"] = json!("expected-uuid");
        value[field] = mismatch;
        let response = response_with_strategy(&value);
        let strategy = response
            .order_strategy
            .as_ref()
            .unwrap_or_else(|| panic!("fixture strategy must exist"));
        assert!(
            !start_identity_matches(&request, strategy),
            "mismatched {field} must not resolve"
        );
    }
}

#[test]
fn top_level_error_text_contradicts_success_but_nested_failure_state_does_not() {
    let mut response = json!({
        "failureReason": "Success",
        "orderStrategy": strategy_value()
    });
    response["errorText"] = json!("provider control failure");
    let response = serde_json::from_value::<StrategyResponse>(response)
        .unwrap_or_else(|error| panic!("fixture response must decode: {error}"));
    assert_eq!(response.mutation_outcome(), MutationOutcome::Ambiguous);

    let mut strategy = strategy_value();
    strategy["failureMessage"] = json!("strategy execution failed after acceptance");
    let response = response_with_strategy(&strategy);
    assert_eq!(response.mutation_outcome(), MutationOutcome::Success);
    let receipt = response
        .order_strategy
        .as_ref()
        .and_then(receipt_from_strategy)
        .unwrap_or_else(|| panic!("terminal strategy state must produce a receipt"));
    assert_eq!(
        receipt.failure_message(),
        Some("strategy execution failed after acceptance")
    );
}

#[test]
fn null_strategy_controls_are_malformed() {
    assert!(
        serde_json::from_value::<StrategyResponse>(json!({
            "failureReason": null,
            "orderStrategy": strategy_value()
        }))
        .is_err()
    );
    assert!(
        serde_json::from_value::<StrategyResponse>(json!({
            "errorText": null,
            "orderStrategy": strategy_value()
        }))
        .is_err()
    );
}

#[tokio::test]
async fn interrupt_id_mismatch_is_ambiguous_and_latches_reconciliation() {
    let server = MockServer::start_async().await;
    let endpoint = server
        .mock_async(|when, then| {
            when.method(POST)
                .path("/v1/orderStrategy/interruptorderstrategy")
                .json_body(json!({ "orderStrategyId": 801 }));
            then.status(200).json_body(json!({
                "orderStrategy": {
                    "id": 999,
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
    let requested = OrderStrategyId::new(801).unwrap_or_else(|error| panic!("{error}"));

    assert!(matches!(
        client.interrupt_order_strategy(requested).await,
        Err(Error::AmbiguousMutation { .. })
    ));
    assert!(client.mutation_reconciliation_required());
    endpoint.assert_async().await;
}

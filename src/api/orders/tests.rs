// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use std::str::FromStr;

use super::*;
use crate::api::orders::failure::OrderFailureReason;
use crate::api::orders::wire::{
    CommandResponse, PlaceOrderWire, PlacementResponse, WireOutcome, classify_outcome,
};
use crate::client::{DocumentedMutationResponse, MutationOutcome};

fn base_builder() -> PlaceOrderBuilder {
    let account_id = AccountId::new(1).unwrap_or_else(|error| panic!("{error}"));
    let symbol = Symbol::new("ESZ6").unwrap_or_else(|error| panic!("{error}"));
    let quantity = OrderQuantity::new(1).unwrap_or_else(|error| panic!("{error}"));
    PlaceOrder::builder(
        account_id,
        symbol,
        OrderSide::Buy,
        quantity,
        OrderOrigin::Automated,
    )
}

#[test]
fn market_order_rejects_prices() {
    let price = Decimal::from_str("5000.25").unwrap_or_else(|error| panic!("{error}"));
    let result = base_builder()
        .order_type(OrderType::Market)
        .price(price)
        .build();
    assert!(matches!(result, Err(Error::InvalidRequest { .. })));
}

#[test]
fn stop_limit_requires_both_exact_prices() {
    let price = Decimal::from_str("5000.25").unwrap_or_else(|error| panic!("{error}"));
    let stop = Decimal::from_str("5000.00").unwrap_or_else(|error| panic!("{error}"));
    let result = base_builder()
        .order_type(OrderType::StopLimit)
        .price(price)
        .stop_price(stop)
        .build();
    assert!(result.is_ok());
}

#[test]
fn wire_marks_automated_orders_explicitly() {
    let order = base_builder()
        .order_type(OrderType::Market)
        .build()
        .unwrap_or_else(|error| panic!("{error}"));
    let json = serde_json::to_value(PlaceOrderWire::from(&order));
    assert_eq!(
        json.as_ref()
            .ok()
            .and_then(|value| value.get("isAutomated"))
            .and_then(serde_json::Value::as_bool),
        Some(true)
    );
}

#[test]
fn stop_builder_translates_to_the_current_price_wire_field() {
    let trigger = Decimal::from_str("4999.75").unwrap_or_else(|error| panic!("{error}"));
    let order = base_builder()
        .order_type(OrderType::Stop)
        .stop_price(trigger)
        .build()
        .unwrap_or_else(|error| panic!("{error}"));
    let encoded = serde_json::to_string(&PlaceOrderWire::from(&order))
        .unwrap_or_else(|error| panic!("{error}"));
    assert!(encoded.contains(r#""price":4999.75"#));
    assert!(!encoded.contains("stopPrice"));
}

#[test]
fn quantity_deserialization_preserves_the_constructor_invariant() {
    assert!(serde_json::from_str::<OrderQuantity>("1").is_ok());
    assert!(serde_json::from_str::<OrderQuantity>("0").is_err());
    assert!(serde_json::from_str::<OrderQuantity>("2147483648").is_err());
}

#[test]
fn explicit_success_is_mutation_evidence_even_without_an_identifier() {
    let placement = PlacementResponse {
        failure_reason: Some(OrderFailureReason::Success),
        failure_text: None,
        order_id: None,
    };
    let cancellation = CommandResponse {
        failure_reason: Some(OrderFailureReason::Success),
        failure_text: None,
        command_id: None,
    };

    assert!(placement.has_success_evidence());
    assert!(cancellation.has_success_evidence());
    assert_eq!(placement.mutation_outcome(), MutationOutcome::Ambiguous);
    assert_eq!(cancellation.mutation_outcome(), MutationOutcome::Ambiguous);
    assert!(matches!(
        classify_outcome::<OrderId>(Some(&OrderFailureReason::Success), None, None),
        WireOutcome::Ambiguous
    ));
}

#[test]
fn cancellation_serializes_all_pinned_optional_fields() {
    let order_id = OrderId::new(17).unwrap_or_else(|error| panic!("{error}"));
    let client_order_id = ClientOrderId::new("cancel-17").unwrap_or_else(|error| panic!("{error}"));
    let activation_time = "2026-08-21T01:02:03Z"
        .parse::<jiff::Timestamp>()
        .unwrap_or_else(|error| panic!("{error}"));
    let custom_tag = CustomTag50::new("cancel-audit").unwrap_or_else(|error| panic!("{error}"));
    let command = CancelOrder::new(order_id, OrderOrigin::Automated)
        .with_client_order_id(client_order_id)
        .with_activation_time(activation_time)
        .with_custom_tag(custom_tag);

    let encoded = serde_json::to_value(crate::api::orders::wire::CancelOrderWire::from(&command))
        .unwrap_or_else(|error| panic!("{error}"));

    assert_eq!(
        encoded.get("activationTime"),
        Some(&serde_json::json!("2026-08-21T01:02:03Z"))
    );
    assert_eq!(
        encoded.get("customTag50"),
        Some(&serde_json::json!("cancel-audit"))
    );
}

#[test]
fn null_response_controls_are_rejected_as_malformed() {
    assert!(
        serde_json::from_str::<PlacementResponse>(r#"{"failureReason":null,"orderId":17}"#,)
            .is_err()
    );
    assert!(
        serde_json::from_str::<CommandResponse>(r#"{"failureText":null,"commandId":17}"#,).is_err()
    );
}

#[test]
fn order_result_classifier_preserves_absence_unknown_and_contradiction() {
    let id = OrderId::new(44).unwrap_or_else(|error| panic!("{error}"));
    assert!(matches!(
        classify_outcome(None, None, Some(id)),
        WireOutcome::Accepted(value) if value == id
    ));
    assert!(matches!(
        classify_outcome::<OrderId>(
            Some(&OrderFailureReason::Unknown("FutureReason".to_owned())),
            None,
            None,
        ),
        WireOutcome::Ambiguous
    ));
    assert!(matches!(
        classify_outcome::<OrderId>(Some(&OrderFailureReason::TradingLocked), None, None),
        WireOutcome::Rejected(OrderFailureReason::TradingLocked)
    ));
    assert!(matches!(
        classify_outcome(
            Some(&OrderFailureReason::Success),
            Some("contradictory failure"),
            Some(id),
        ),
        WireOutcome::Ambiguous
    ));
}

#[test]
fn placement_and_command_failure_text_are_decoded_and_ambiguous_with_ids() {
    let placement =
        serde_json::from_str::<PlacementResponse>(r#"{"orderId":5,"failureText":"contradiction"}"#)
            .unwrap_or_else(|error| panic!("fixture must decode: {error}"));
    let command =
        serde_json::from_str::<CommandResponse>(r#"{"commandId":6,"failureText":"contradiction"}"#)
            .unwrap_or_else(|error| panic!("fixture must decode: {error}"));

    assert_eq!(placement.mutation_outcome(), MutationOutcome::Ambiguous);
    assert_eq!(command.mutation_outcome(), MutationOutcome::Ambiguous);
    assert!(placement.has_success_evidence());
    assert!(command.has_success_evidence());
}

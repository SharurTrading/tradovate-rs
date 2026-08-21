// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

use std::str::FromStr;

use super::*;
use crate::client::MutationWireResponse;

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
fn unknown_response_status_preserves_provider_code() {
    let decoded = serde_json::from_str::<OrderStatus>(r#""FutureStatus""#);
    assert!(matches!(
        decoded,
        Ok(OrderStatus::Unknown(value)) if value == "FutureStatus"
    ));
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
        failure_reason: Some("Success".to_owned()),
        order_id: None,
    };
    let cancellation = CommandResponse {
        failure_reason: Some("Success".to_owned()),
        command_id: None,
    };

    assert!(placement.has_success_evidence());
    assert!(cancellation.has_success_evidence());
    assert!(matches!(
        classify_outcome::<OrderId>(Some("Success"), None),
        WireOutcome::Ambiguous
    ));
}

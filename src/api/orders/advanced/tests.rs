// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use super::*;
use crate::api::{
    ADVANCED_ORDER_TYPES_DOCUMENTATION_GAPS, STANDARD_ORDER_COMBINATIONS_DOCUMENTATION_GAPS,
};
use crate::api::{OrderOrigin, OrderQuantity};
use crate::{AccountId, Symbol};

fn primary(
    order_type: OrderType,
    price: Option<Decimal>,
    stop_price: Option<Decimal>,
) -> PlaceOrder {
    let account = AccountId::new(1).unwrap_or_else(|error| panic!("{error}"));
    let symbol = Symbol::new("ESZ6").unwrap_or_else(|error| panic!("{error}"));
    let quantity = OrderQuantity::new(1).unwrap_or_else(|error| panic!("{error}"));
    let mut builder = PlaceOrder::builder(
        account,
        symbol,
        OrderSide::Buy,
        quantity,
        OrderOrigin::Automated,
    )
    .order_type(order_type);
    if let Some(value) = price {
        builder = builder.price(value);
    }
    if let Some(value) = stop_price {
        builder = builder.stop_price(value);
    }
    builder.build().unwrap_or_else(|error| panic!("{error}"))
}

#[test]
fn advanced_stop_uses_the_current_documented_price_field() {
    let trigger = "4100.25"
        .parse::<Decimal>()
        .unwrap_or_else(|error| panic!("{error}"));
    let parent = primary(OrderType::Stop, None, Some(trigger));
    let linked =
        AttachedOrder::limit(OrderSide::Sell, trigger).unwrap_or_else(|error| panic!("{error}"));
    let request = PlaceOco::new(parent, linked);
    let encoded = serde_json::to_value(wire::PlaceOcoWire::from(&request))
        .unwrap_or_else(|error| panic!("{error}"));

    let encoded = encoded.to_string();
    assert!(encoded.contains(r#""price":4100.25"#));
    assert!(!encoded.contains("stopPrice"));
    assert!(encoded.contains(r#""isAutomated":true"#));
}

#[test]
fn oco_response_requires_both_identifiers() {
    let response = wire::OcoResponse {
        failure_reason: Some(crate::api::orders::failure::OrderFailureReason::Success),
        failure_text: None,
        order_id: crate::OrderId::new(2).ok(),
        oco_id: None,
    };
    assert_eq!(
        crate::client::DocumentedMutationResponse::mutation_outcome(&response),
        crate::client::MutationOutcome::Ambiguous
    );
}

#[test]
fn complete_ids_without_failure_reason_are_documented_success() {
    let response = wire::OcoResponse {
        failure_reason: None,
        failure_text: None,
        order_id: crate::OrderId::new(2).ok(),
        oco_id: crate::api::current::ids::OcoId::new(3).ok(),
    };
    assert_eq!(
        crate::client::DocumentedMutationResponse::mutation_outcome(&response),
        crate::client::MutationOutcome::Success
    );
}

#[test]
fn oco_failure_text_contradicts_complete_success_evidence() {
    let response = wire::OcoResponse {
        failure_reason: None,
        failure_text: Some("provider reported a failure".to_owned()),
        order_id: crate::OrderId::new(2).ok(),
        oco_id: crate::api::current::ids::OcoId::new(3).ok(),
    };
    assert_eq!(
        crate::client::DocumentedMutationResponse::mutation_outcome(&response),
        crate::client::MutationOutcome::Ambiguous
    );
}

#[test]
fn oso_failure_text_and_unknown_reason_cannot_resolve() {
    let contradiction = wire::OsoResponse {
        failure_reason: None,
        failure_text: Some("provider reported a failure".to_owned()),
        order_id: crate::OrderId::new(2).ok(),
        oso1_id: crate::api::current::ids::Oso1Id::new(3).ok(),
        oso2_id: None,
    };
    let unknown = wire::OsoResponse {
        failure_reason: Some(crate::api::orders::failure::OrderFailureReason::Unknown(
            "FutureReason".to_owned(),
        )),
        failure_text: None,
        order_id: None,
        oso1_id: None,
        oso2_id: None,
    };

    assert_eq!(
        crate::client::DocumentedMutationResponse::mutation_outcome(&contradiction),
        crate::client::MutationOutcome::Ambiguous
    );
    assert_eq!(
        crate::client::DocumentedMutationResponse::mutation_outcome(&unknown),
        crate::client::MutationOutcome::Ambiguous
    );
}

#[test]
fn advanced_gap_matrix_covers_parent_and_bracket_surfaces() {
    let fields = ADVANCED_ORDER_TYPES_DOCUMENTATION_GAPS
        .iter()
        .map(|gap| (gap.endpoint(), gap.field()))
        .collect::<Vec<_>>();
    assert!(fields.contains(&("/order/modifyorder", "orderType")));
    assert!(fields.contains(&("/order/dryrun", "orders[].orderType")));
    assert!(fields.contains(&("/order/placeoco", "other.orderType")));
    assert!(fields.contains(&("/order/placeoso", "bracket1.orderType")));
    assert!(fields.contains(&("/order/placeoso", "bracket2.orderType")));

    let standard_fields = STANDARD_ORDER_COMBINATIONS_DOCUMENTATION_GAPS
        .iter()
        .map(|gap| (gap.endpoint(), gap.field()))
        .collect::<Vec<_>>();
    assert!(standard_fields.contains(&("/order/placeorder", "orderType=StopLimit")));
    assert!(standard_fields.contains(&("/order/modifyorder", "orderType=Stop|StopLimit")));
    assert!(standard_fields.contains(&("/order/dryrun", "orders[].orderType=Stop|StopLimit")));
}

#[test]
fn oso_rejects_undocumented_stop_bracket_combinations() {
    let trigger = "4100.25"
        .parse::<Decimal>()
        .unwrap_or_else(|error| panic!("{error}"));
    let stop =
        AttachedOrder::stop(OrderSide::Sell, trigger).unwrap_or_else(|error| panic!("{error}"));

    assert!(PlaceOso::new(primary(OrderType::Market, None, None), stop).is_err());
}

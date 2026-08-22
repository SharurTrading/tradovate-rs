// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use super::*;
use crate::{
    Decimal, UserId,
    api::current::{
        ids::{
            MarketDataSubscriptionId, MarketDataSubscriptionPlanId, TradovateSubscriptionId,
            TradovateSubscriptionPlanId,
        },
        users::TradeDate,
    },
    client::MutationOutcome,
};

#[test]
fn market_data_create_and_update_states_are_disjoint() {
    assert!(CreateMarketDataSubscriptionRequest::new(market_data(None, 8)).is_ok());
    assert!(CreateMarketDataSubscriptionRequest::new(market_data(Some(market_id(7)), 8)).is_err());
    assert!(UpdateMarketDataSubscriptionRequest::new(market_data(None, 8)).is_err());
    assert!(UpdateMarketDataSubscriptionRequest::new(market_data(Some(market_id(7)), 8)).is_ok());
}

#[test]
fn wrapper_validation_rejects_a_builder_bypassed_billing_month() {
    let invalid = market_data(None, 13);
    assert!(CreateMarketDataSubscriptionRequest::new(invalid).is_err());
}

#[test]
fn market_data_create_requires_a_new_id_and_exact_stable_payload() {
    let request = CreateMarketDataSubscriptionRequest::new(market_data(None, 8));
    let Ok(request) = request else {
        panic!("market-data request fixture must validate");
    };
    let response = market_data(Some(market_id(9)), 8);
    assert_eq!(
        assess_market_data_create(&response, &request).outcome(),
        MutationOutcome::Success
    );

    let mismatch = market_data(Some(market_id(9)), 9);
    let assessment = assess_market_data_create(&mismatch, &request);
    assert_eq!(assessment.outcome(), MutationOutcome::Ambiguous);
    assert!(assessment.has_success_evidence());
}

#[test]
fn tradovate_create_rejects_reversed_dates() {
    let invalid = tradovate(None, date(2026, 9, 1), date(2026, 8, 31));
    assert!(CreateTradovateSubscriptionRequest::new(invalid).is_err());

    let request = CreateTradovateSubscriptionRequest::new(tradovate(
        None,
        date(2026, 8, 1),
        date(2026, 9, 1),
    ));
    let Ok(request) = request else {
        panic!("Tradovate subscription fixture must validate");
    };
    let response = tradovate(Some(tradovate_id(11)), date(2026, 8, 1), date(2026, 9, 1));
    assert_eq!(
        assess_tradovate_create(&response, &request).outcome(),
        MutationOutcome::Success
    );
}

fn market_data(id: Option<MarketDataSubscriptionId>, month: i64) -> MarketDataSubscription {
    let builder = MarketDataSubscription::builder()
        .user_id(user_id(3))
        .timestamp(timestamp())
        .plan_price(Decimal::new(1250, 2))
        .market_data_subscription_plan_id(market_plan_id(5))
        .year(2026)
        .month(month)
        .expired(false);
    let builder = match id {
        Some(id) => builder.id(id),
        None => builder,
    };
    builder
        .build()
        .unwrap_or_else(|error| panic!("market-data fixture: {error}"))
}

fn tradovate(
    id: Option<TradovateSubscriptionId>,
    start: TradeDate,
    expiration: TradeDate,
) -> TradovateSubscription {
    let builder = TradovateSubscription::builder()
        .user_id(user_id(3))
        .timestamp(timestamp())
        .plan_price(Decimal::new(2500, 2))
        .tradovate_subscription_plan_id(tradovate_plan_id(5))
        .start_date(start)
        .expiration_date(expiration)
        .paid_amount(Decimal::new(2500, 2));
    let builder = match id {
        Some(id) => builder.id(id),
        None => builder,
    };
    builder
        .build()
        .unwrap_or_else(|error| panic!("Tradovate subscription fixture: {error}"))
}

fn date(year: i64, month: i64, day: i64) -> TradeDate {
    TradeDate::builder()
        .year(year)
        .month(month)
        .day(day)
        .build()
        .unwrap_or_else(|error| panic!("date fixture: {error}"))
}

fn timestamp() -> jiff::Timestamp {
    "2026-08-21T00:00:00Z"
        .parse()
        .unwrap_or_else(|error| panic!("timestamp fixture: {error}"))
}

fn user_id(value: i64) -> UserId {
    UserId::new(value).unwrap_or_else(|error| panic!("user ID fixture: {error}"))
}

fn market_id(value: i64) -> MarketDataSubscriptionId {
    MarketDataSubscriptionId::new(value)
        .unwrap_or_else(|error| panic!("market-data ID fixture: {error}"))
}

fn market_plan_id(value: i64) -> MarketDataSubscriptionPlanId {
    MarketDataSubscriptionPlanId::new(value)
        .unwrap_or_else(|error| panic!("market-data plan fixture: {error}"))
}

fn tradovate_id(value: i64) -> TradovateSubscriptionId {
    TradovateSubscriptionId::new(value)
        .unwrap_or_else(|error| panic!("Tradovate ID fixture: {error}"))
}

fn tradovate_plan_id(value: i64) -> TradovateSubscriptionPlanId {
    TradovateSubscriptionPlanId::new(value)
        .unwrap_or_else(|error| panic!("Tradovate plan fixture: {error}"))
}

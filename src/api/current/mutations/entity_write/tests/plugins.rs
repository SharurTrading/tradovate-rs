// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

use super::*;
use crate::{
    Decimal, UserId,
    api::current::{ids::UserPluginId, users::TradeDate},
    client::MutationOutcome,
};

#[test]
fn plugin_create_and_update_states_are_disjoint() {
    assert!(CreateUserPluginRequest::new(plugin(None, "charts", None)).is_ok());
    assert!(CreateUserPluginRequest::new(plugin(Some(plugin_id(7)), "charts", None)).is_err());
    assert!(UpdateUserPluginRequest::new(plugin(None, "charts", None)).is_err());
    assert!(UpdateUserPluginRequest::new(plugin(Some(plugin_id(7)), "charts", None)).is_ok());
}

#[test]
fn wrapper_revalidates_a_serde_bypassed_plugin_name() {
    let decoded = serde_json::from_str::<UserPlugin>(
        r#"{
            "userId":3,
            "timestamp":"2026-08-21T00:00:00Z",
            "planPrice":12.50,
            "pluginName":" charts",
            "approval":true,
            "startDate":{"year":2026,"month":8,"day":1},
            "paidAmount":12.50
        }"#,
    );
    let Ok(decoded) = decoded else {
        panic!("wire fixture should bypass builder validation");
    };
    assert!(CreateUserPluginRequest::new(decoded).is_err());
}

#[test]
fn plugin_dates_are_calendar_valid_and_ordered() {
    let expiration = Some(date(2026, 7, 31));
    assert!(CreateUserPluginRequest::new(plugin(None, "charts", expiration)).is_err());

    let invalid_calendar = Some(date(2026, 2, 30));
    assert!(CreateUserPluginRequest::new(plugin(None, "charts", invalid_calendar)).is_err());
}

#[test]
fn plugin_create_requires_new_id_and_exact_stable_payload() {
    let request = CreateUserPluginRequest::new(plugin(None, "charts", None));
    let Ok(request) = request else {
        panic!("plugin request fixture must validate");
    };
    let response = plugin(Some(plugin_id(8)), "charts", None);
    assert_eq!(
        assess_create(&response, &request).outcome(),
        MutationOutcome::Success
    );

    let mismatch = plugin(Some(plugin_id(8)), "ladder", None);
    let assessment = assess_create(&mismatch, &request);
    assert_eq!(assessment.outcome(), MutationOutcome::Ambiguous);
    assert!(assessment.has_success_evidence());
}

fn plugin(id: Option<UserPluginId>, name: &str, expiration: Option<TradeDate>) -> UserPlugin {
    let builder = UserPlugin::builder()
        .user_id(user_id(3))
        .timestamp(timestamp())
        .plan_price(Decimal::new(1250, 2))
        .plugin_name(name)
        .approval(true)
        .start_date(date(2026, 8, 1))
        .paid_amount(Decimal::new(1250, 2));
    let builder = match id {
        Some(id) => builder.id(id),
        None => builder,
    };
    let builder = match expiration {
        Some(expiration) => builder.expiration_date(expiration),
        None => builder,
    };
    builder
        .build()
        .unwrap_or_else(|error| panic!("plugin fixture: {error}"))
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

fn plugin_id(value: i64) -> UserPluginId {
    UserPluginId::new(value).unwrap_or_else(|error| panic!("plugin ID fixture: {error}"))
}

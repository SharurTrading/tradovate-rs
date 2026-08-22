// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use super::*;
use crate::{
    UserId,
    api::current::{
        SecretValue,
        ids::{OrganizationId, PoaContactId},
        users::TradeDate,
    },
    client::MutationOutcome,
};

#[test]
fn poa_create_and_update_states_are_disjoint() {
    assert!(CreatePoaContactRequest::new(contact(None, "Ada", "Lovelace", "US", "tax-a")).is_ok());
    assert!(
        CreatePoaContactRequest::new(contact(Some(poa_id(7)), "Ada", "Lovelace", "US", "tax-a"))
            .is_err()
    );
    assert!(UpdatePoaContactRequest::new(contact(None, "Ada", "Lovelace", "US", "tax-a")).is_err());
}

#[test]
fn wrapper_revalidates_combined_names_and_country_codes() {
    let long_first = "A".repeat(31);
    let long_last = "B".repeat(30);
    let oversized = contact(None, &long_first, &long_last, "US", "tax-a");
    assert!(CreatePoaContactRequest::new(oversized).is_err());

    let invalid_country = contact(None, "Ada", "Lovelace", "USA", "tax-a");
    assert!(CreatePoaContactRequest::new(invalid_country).is_err());
}

#[test]
fn protected_ids_stay_redacted_and_participate_in_response_proof() {
    let request = CreatePoaContactRequest::new(contact(None, "Ada", "Lovelace", "US", "tax-a"));
    let Ok(request) = request else {
        panic!("POA request fixture must validate");
    };
    let debug = format!("{request:?}");
    assert!(!debug.contains("tax-a"));

    let response = contact(Some(poa_id(8)), "Ada", "Lovelace", "US", "tax-a");
    assert_eq!(
        assess_create(&response, &request).outcome(),
        MutationOutcome::Success
    );

    let mismatch = contact(Some(poa_id(8)), "Ada", "Lovelace", "US", "tax-b");
    let assessment = assess_create(&mismatch, &request);
    assert_eq!(assessment.outcome(), MutationOutcome::Ambiguous);
    assert!(assessment.has_success_evidence());
}

fn contact(
    id: Option<PoaContactId>,
    first_name: &str,
    last_name: &str,
    country: &str,
    tax_identifier: &str,
) -> POAContact {
    let builder = POAContact::builder()
        .timestamp(timestamp())
        .user_id(user_id(3))
        .first_name(first_name)
        .last_name(last_name)
        .country(country)
        .state("IL")
        .street_address1("141 Trading Street")
        .city("Chicago")
        .zip_code("60601")
        .phone("+13125550100")
        .citizenship("US")
        .tax_identifier(secret(tax_identifier))
        .national_id(secret("national-a"))
        .birth_date(date(1985, 12, 10))
        .organization_id(organization_id(4));
    let builder = match id {
        Some(id) => builder.id(id),
        None => builder,
    };
    builder
        .build()
        .unwrap_or_else(|error| panic!("POA contact fixture: {error}"))
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

fn secret(value: &str) -> SecretValue {
    SecretValue::new(value).unwrap_or_else(|error| panic!("secret fixture: {error}"))
}

fn user_id(value: i64) -> UserId {
    UserId::new(value).unwrap_or_else(|error| panic!("user ID fixture: {error}"))
}

fn organization_id(value: i64) -> OrganizationId {
    OrganizationId::new(value).unwrap_or_else(|error| panic!("organization ID fixture: {error}"))
}

fn poa_id(value: i64) -> PoaContactId {
    PoaContactId::new(value).unwrap_or_else(|error| panic!("POA ID fixture: {error}"))
}

// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use super::*;
use crate::{Environment, api::current::ids::WorkspaceTemplateId, client::MutationOutcome};

#[test]
fn workspace_create_and_update_require_exact_response_fields() {
    let create = template(None, "Desk");
    let created = template(Some(17), "Desk");
    assert_eq!(
        assess_workspace_create(&created, &create).outcome(),
        MutationOutcome::Success
    );
    assert_eq!(
        assess_workspace_create(&template(Some(17), "Other"), &create).outcome(),
        MutationOutcome::Ambiguous
    );

    let update = template(Some(17), "Renamed");
    assert_eq!(
        assess_workspace_update(&template(Some(17), "Renamed"), &update).outcome(),
        MutationOutcome::Success
    );
    assert_eq!(
        assess_workspace_update(&template(Some(18), "Renamed"), &update).outcome(),
        MutationOutcome::Ambiguous
    );
}

#[tokio::test]
async fn workspace_operation_specific_identity_rules_run_before_authentication() {
    let client = Client::builder(Environment::Demo)
        .build()
        .unwrap_or_else(|error| panic!("client fixture: {error}"));

    let invalid_create = client
        .workspace_template_create(&template(Some(17), "Desk"))
        .await;
    assert!(matches!(
        invalid_create,
        Err(Error::InvalidRequest { field: "id", .. })
    ));

    let invalid_update = client
        .workspace_template_update(&template(None, "Desk"))
        .await;
    assert!(matches!(
        invalid_update,
        Err(Error::InvalidRequest { field: "id", .. })
    ));

    let long_name = UpdateContactInfoName::builder()
        .first_name("a".repeat(31))
        .last_name("b".repeat(30))
        .country("AU")
        .phone("+61000000000")
        .build()
        .unwrap_or_else(|error| panic!("long name fixture: {error}"));
    let invalid_name = client
        .contact_info_update_contact_info_name(&long_name)
        .await;
    assert!(matches!(
        invalid_name,
        Err(Error::InvalidRequest {
            field: "firstName/lastName",
            ..
        })
    ));
}

#[test]
fn contact_completion_requires_every_echoed_field() {
    let country_request = UpdateContactCountry::builder()
        .country("AU")
        .build()
        .unwrap_or_else(|error| panic!("country request fixture: {error}"));
    let exact_country = contact_response("Kevin", "Monaghan", "AU", "+61000000000");
    assert_eq!(
        assess_contact_country(&exact_country, &country_request).outcome(),
        MutationOutcome::Success
    );

    let name_request = UpdateContactInfoName::builder()
        .first_name("Kevin")
        .last_name("Monaghan")
        .country("AU")
        .phone("+61000000000")
        .build()
        .unwrap_or_else(|error| panic!("name request fixture: {error}"));
    assert_eq!(
        assess_contact_name(&exact_country, &name_request).outcome(),
        MutationOutcome::Success
    );
    let mismatch = contact_response("Kevin", "Other", "AU", "+61000000000");
    assert_eq!(
        assess_contact_name(&mismatch, &name_request).outcome(),
        MutationOutcome::Ambiguous
    );
}

fn template(id: Option<i64>, name: &str) -> WorkspaceTemplate {
    let builder = WorkspaceTemplate::builder().name(name);
    let builder = match id {
        Some(id) => builder.id(WorkspaceTemplateId::new(id)
            .unwrap_or_else(|error| panic!("template ID fixture: {error}"))),
        None => builder,
    };
    builder
        .build()
        .unwrap_or_else(|error| panic!("workspace template fixture: {error}"))
}

fn contact_response(
    first_name: &str,
    last_name: &str,
    country: &str,
    phone: &str,
) -> UpdateContactInfoResponse {
    serde_json::from_value(serde_json::json!({
        "contactInfo": {
            "id": 1,
            "userId": 7,
            "firstName": first_name,
            "lastName": last_name,
            "streetAddress1": "1 Example Street",
            "city": "Sydney",
            "country": country,
            "phone": phone
        }
    }))
    .unwrap_or_else(|error| panic!("contact response fixture: {error}"))
}

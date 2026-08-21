// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

use serde::de::DeserializeOwned;
use serde_json::json;

use super::{accounts, batches, documents, profiles};
use crate::{
    AccountId, Client, Decimal, Environment, UserId,
    api::current::{SecretValue, users::*},
    client::MutationOutcome,
};

#[test]
fn evaluation_account_batches_are_bounded_to_one_hundred() {
    let account = EvaluationAccount::builder()
        .user_id(user_id(1))
        .template_account_id(account_id(1))
        .name("evaluation")
        .initial_balance(Decimal::ONE)
        .build()
        .unwrap_or_else(|error| panic!("fixture account: {error}"));
    let request = CreateEvaluationAccounts::builder()
        .accounts(vec![account; 101])
        .build()
        .unwrap_or_else(|error| panic!("fixture batch: {error}"));

    assert!(matches!(
        batches::validate_account_batch(&request),
        Err(crate::Error::InvalidRequest {
            field: "accounts",
            ..
        })
    ));
}

#[test]
fn evaluation_user_names_use_the_documented_combined_limit() {
    let user = EvaluationUser::builder()
        .name(secret("login"))
        .email("person@example.com")
        .password(secret("password"))
        .first_name("a".repeat(31))
        .last_name("b".repeat(30))
        .build()
        .unwrap_or_else(|error| panic!("fixture user: {error}"));
    let request = CreateEvaluationUsers::builder()
        .users(vec![user])
        .build()
        .unwrap_or_else(|error| panic!("fixture batch: {error}"));

    assert!(matches!(
        batches::validate_user_batch(&request),
        Err(crate::Error::InvalidRequest {
            field: "users.name",
            ..
        })
    ));
}

#[test]
fn mixed_complete_batch_items_are_definitive() {
    let request: CreateEvaluationAccounts = decode(json!({
        "accounts": [
            {"userId": 1, "templateAccountId": 11, "name": "one", "initialBalance": 1},
            {"userId": 2, "templateAccountId": 12, "name": "two", "initialBalance": 1}
        ]
    }));
    let request = batches::EvaluationAccountsRequest::try_from(request)
        .unwrap_or_else(|error| panic!("fixture request: {error}"));
    let response: CreateEvaluationAccountsResponse = decode(json!({
        "results": [
            {"accountId": 101, "tradingPermissionId": 201},
            {"errorText": "synthetic rejection"}
        ]
    }));

    let assessment = batches::assess_account_batch(&response, &request);
    assert_eq!(assessment.outcome(), MutationOutcome::Success);
    assert!(assessment.has_success_evidence());
}

#[test]
fn partial_batch_ids_are_ambiguous() {
    let request: CreateEvaluationAccounts = decode(json!({
        "accounts": [
            {"userId": 1, "templateAccountId": 11, "name": "one", "initialBalance": 1}
        ]
    }));
    let request = batches::EvaluationAccountsRequest::try_from(request)
        .unwrap_or_else(|error| panic!("fixture request: {error}"));
    let response: CreateEvaluationAccountsResponse = decode(json!({
        "results": [{"accountId": 101}]
    }));

    assert_eq!(
        batches::assess_account_batch(&response, &request).outcome(),
        MutationOutcome::Ambiguous
    );
}

#[test]
fn partial_batch_ids_cannot_be_downgraded_to_a_business_rejection() {
    let request: CreateEvaluationAccounts = decode(json!({
        "accounts": [
            {"userId": 1, "templateAccountId": 11, "name": "one", "initialBalance": 1}
        ]
    }));
    let request = batches::EvaluationAccountsRequest::try_from(request)
        .unwrap_or_else(|error| panic!("fixture request: {error}"));
    let response: CreateEvaluationAccountsResponse = decode(json!({
        "errorText": "synthetic batch error",
        "results": [{"accountId": 101, "errorText": "synthetic item error"}]
    }));

    let assessment = batches::assess_account_batch(&response, &request);
    assert_eq!(assessment.outcome(), MutationOutcome::Ambiguous);
    assert!(assessment.has_success_evidence());
}

#[test]
fn partner_identity_rules_are_enforced_without_exposing_secrets() {
    let invalid_us = partner_subaccount(&json!({"taxIdentifier": "12345678"}));
    assert!(matches!(
        accounts::validate_partner_subaccount(&invalid_us),
        Err(crate::Error::InvalidRequest {
            field: "taxIdentifier",
            ..
        })
    ));

    let invalid_non_us = partner_subaccount(&json!({
        "citizenship": "AU",
        "taxIdentifier": null,
        "nationalId": "!"
    }));
    assert!(matches!(
        accounts::validate_partner_subaccount(&invalid_non_us),
        Err(crate::Error::InvalidRequest {
            field: "nationalId",
            ..
        })
    ));
}

#[test]
fn partner_document_ids_are_all_or_none() {
    let partial = partner_subaccount(&json!({
        "taxIdentifier": "123456789",
        "pOAFormDocId": 10
    }));
    assert!(matches!(
        accounts::validate_partner_subaccount(&partial),
        Err(crate::Error::InvalidRequest {
            field: "documentIds",
            ..
        })
    ));
}

#[test]
fn document_data_uri_grammar_is_fail_closed() {
    assert!(documents::validate_data_uri("data:application/pdf;base64,SGVsbG8=", 128).is_ok());
    assert!(documents::validate_data_uri("data:image/png;base64,iVBORw0KGgo=", 128).is_ok());
    assert!(documents::validate_data_uri("data:text/html;base64,SGVsbG8=", 128).is_err());
    assert!(documents::validate_data_uri("data:application/pdf;base64,%%%", 128).is_err());
    assert!(documents::validate_data_uri("data:application/pdf,SGVsbG8=", 128).is_err());
}

#[test]
fn document_success_and_error_evidence_cannot_coexist() {
    let request: SubmitCustomerApplicationDocument = decode(json!({
        "process": "Identity",
        "documentType": "Id",
        "filename": "identity.pdf",
        "base64data": "data:application/pdf;base64,SGVsbG8="
    }));
    let request = documents::CustomerApplicationDocumentRequest::try_from(request)
        .unwrap_or_else(|error| panic!("fixture request: {error}"));
    let response: SubmitCustomerApplicationDocumentResponse = decode(json!({
        "ok": true,
        "documentId": 9,
        "errorText": "synthetic contradiction"
    }));

    let assessment = documents::assess_customer_document(&response, &request);
    assert_eq!(assessment.outcome(), MutationOutcome::Ambiguous);
    assert!(assessment.has_success_evidence());
}

#[tokio::test]
async fn production_document_uploads_reject_demo_before_authentication() {
    let request: SubmitCustomerApplicationDocument = decode(json!({
        "process": "Identity",
        "documentType": "Id",
        "filename": "identity.pdf",
        "base64data": "data:application/pdf;base64,SGVsbG8="
    }));
    let request = documents::CustomerApplicationDocumentRequest::try_from(request)
        .unwrap_or_else(|error| panic!("fixture request: {error}"));
    let client = Client::builder(Environment::Demo)
        .build()
        .unwrap_or_else(|error| panic!("fixture client: {error}"));

    assert!(matches!(
        client
            .customer_application_submit_customer_application_document(&request)
            .await,
        Err(crate::Error::InvalidRequest {
            field: "environment",
            ..
        })
    ));
}

#[test]
fn organization_member_combined_names_are_bounded() {
    let request = SignUpOrganizationMember::builder()
        .name(secret("login"))
        .email("person@example.com")
        .password(secret("password"))
        .first_name("a".repeat(40))
        .last_name("b".repeat(21))
        .build()
        .unwrap_or_else(|error| panic!("fixture member: {error}"));

    assert!(profiles::validate_organization_member(&request).is_err());
}

fn partner_subaccount(overrides: &serde_json::Value) -> CreatePartnerSubAccountRequest {
    let mut value = json!({
        "ctaUserId": 1,
        "riskCategoryId": 2,
        "marginType": "Speculator",
        "transferAmount": 100,
        "authorizedIndividual": true,
        "firstName": "Ada",
        "lastName": "Lovelace",
        "country": "US",
        "state": "NSW",
        "streetAddress1": "1 Example Street",
        "city": "Sydney",
        "zipCode": "2000",
        "phone": "+61000000000",
        "citizenship": "US",
        "taxIdentifier": "123456789",
        "birthDate": {"year": 1990, "month": 1, "day": 2}
    });
    if let (Some(target), Some(source)) = (value.as_object_mut(), overrides.as_object()) {
        for (key, value) in source {
            if value.is_null() {
                target.remove(key);
            } else {
                target.insert(key.clone(), value.clone());
            }
        }
    }
    decode(value)
}

fn decode<T: DeserializeOwned>(value: serde_json::Value) -> T {
    serde_json::from_value(value).unwrap_or_else(|error| panic!("fixture must decode: {error}"))
}

fn secret(value: &str) -> SecretValue {
    SecretValue::new(value).unwrap_or_else(|error| panic!("fixture secret: {error}"))
}

fn user_id(value: i64) -> UserId {
    UserId::new(value).unwrap_or_else(|error| panic!("fixture user: {error}"))
}

fn account_id(value: i64) -> AccountId {
    AccountId::new(value).unwrap_or_else(|error| panic!("fixture account: {error}"))
}

// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

use std::collections::BTreeSet;

use super::*;

#[test]
fn manifest_is_an_exhaustive_unique_current_contract() {
    assert_eq!(OPERATIONS.len(), 350);
    let unique = OPERATIONS
        .iter()
        .map(|operation| (operation.method(), operation.path()))
        .collect::<BTreeSet<_>>();
    assert_eq!(unique.len(), 350);
    assert_eq!(
        OPERATIONS
            .iter()
            .filter(|operation| operation.class() == OperationClass::Query)
            .count(),
        271
    );
    assert_eq!(
        OPERATIONS
            .iter()
            .filter(|operation| operation.class() == OperationClass::Mutation)
            .count(),
        73
    );
    assert_eq!(
        OPERATIONS
            .iter()
            .filter(|operation| operation.class() == OperationClass::Lifecycle)
            .count(),
        6
    );
    assert_eq!(
        OPERATIONS
            .iter()
            .filter(|operation| operation.surface() == OperationSurface::Generated)
            .count(),
        263
    );
    assert_eq!(
        OPERATIONS
            .iter()
            .filter(|operation| operation.surface() == OperationSurface::Specialized)
            .count(),
        71
    );
    assert_eq!(
        OPERATIONS
            .iter()
            .filter(|operation| operation.surface() == OperationSurface::Modeled)
            .count(),
        0
    );
    assert_eq!(
        OPERATIONS
            .iter()
            .filter(|operation| { operation.surface() == OperationSurface::DocumentationBlocked })
            .count(),
        16
    );
    assert_eq!(
        OPERATIONS
            .iter()
            .filter(|operation| { operation.response_contract() == ResponseContract::Unspecified })
            .count(),
        11
    );
    assert_eq!(
        OPERATIONS
            .iter()
            .filter(|operation| { operation.response_contract() == ResponseContract::Incomplete })
            .count(),
        1
    );
    assert_eq!(SCHEMA_GAPS.len(), 3);
}

#[test]
fn empty_generated_secrets_are_rejected() {
    assert!(SecretValue::new("").is_err());
}

#[test]
fn subaccount_request_identity_flows_into_follow_up_operations() {
    let decoded = serde_json::from_str::<users::CreatePartnerSubAccountRequestResponse>(
        r#"{"requestId":42}"#,
    );
    let Ok(response) = decoded else {
        panic!("a documented positive requestId must decode");
    };
    let Some(request_id) = response.request_id().copied() else {
        panic!("the response must retain requestId");
    };

    assert!(
        users::GetPartnerSubAccountRequestStatus::builder()
            .sub_account_request_id(request_id)
            .build()
            .is_ok()
    );

    let status = serde_json::from_str::<users::PartnerSubAccountRequestStatusResponse>(
        r#"{"requestId":42}"#,
    );
    assert!(status.is_ok_and(|value| value.request_id().copied() == Some(request_id)));
}

#[test]
fn undocumented_object_members_fail_closed() {
    assert!(
        serde_json::from_str::<contracts::RollContractsResponseContracts>(r#"{"ESH7":17}"#)
            .is_err()
    );
    assert!(serde_json::from_str::<contracts::RollContractsResponseContracts>("{}").is_ok());
}

#[test]
fn generated_credential_names_are_redacted() {
    let decoded = serde_json::from_str::<users::EvaluationUser>(
        r#"{"name":"login-secret","email":"a@example.com","password":"password-secret","firstName":"A","lastName":"B"}"#,
    );
    let Ok(user) = decoded else {
        panic!("documented evaluation user must decode");
    };
    let debug = format!("{user:?}");
    assert!(!debug.contains("login-secret"));
    assert!(!debug.contains("password-secret"));
}

// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use super::*;
use crate::api::current::{ids::RiskCategoryId, users::SimpleResponse};

#[test]
fn other_reason_is_required_and_bounded() {
    let account_id = account(1);
    let missing = SetAdminAutoLiqActionRequest::builder()
        .account_id(account_id)
        .admin_action(PartnerAdminAutoLiqAction::Normal)
        .admin_action_reason_code(SetAdminAutoLiqActionAdminActionReasonCode::Other)
        .build();
    assert!(matches!(
        missing,
        Err(Error::InvalidRequest {
            field: "adminActionReason",
            ..
        })
    ));

    let oversized = SetAdminAutoLiqActionRequest::builder()
        .account_id(account_id)
        .admin_action(PartnerAdminAutoLiqAction::Normal)
        .admin_action_reason_code(SetAdminAutoLiqActionAdminActionReasonCode::Other)
        .admin_action_reason("x".repeat(MAX_ADMIN_REASON_CHARS + 1))
        .build();
    assert!(oversized.is_err());
}

#[test]
fn switch_accounts_must_be_unique() {
    let request = SwitchRiskCategory::builder()
        .account_ids(vec![account(7), account(7)])
        .risk_category_id(risk_category(3))
        .build();
    let Ok(request) = request else {
        panic!("generated request fixture must build");
    };
    assert!(validate_switch_accounts(&request).is_err());
}

#[test]
fn successful_switch_acknowledgement_stays_ambiguous() {
    let response = SimpleResponse::builder().ok(true).build();
    let request = SwitchRiskCategory::builder()
        .account_ids(vec![account(1)])
        .risk_category_id(risk_category(2))
        .build();
    let (Ok(response), Ok(request)) = (response, request) else {
        panic!("documented fixtures must build");
    };
    assert_eq!(
        assess_switch_risk_category(&response, &request).outcome(),
        MutationOutcome::Ambiguous
    );
}

fn account(value: i64) -> AccountId {
    AccountId::new(value).unwrap_or_else(|error| panic!("account fixture: {error}"))
}

fn risk_category(value: i64) -> RiskCategoryId {
    RiskCategoryId::new(value).unwrap_or_else(|error| panic!("risk-category fixture: {error}"))
}

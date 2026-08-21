// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Partner subaccount and simulation-account creation mutations.

use crate::api::current::{
    SecretValue,
    users::{
        CreatePartnerSubAccountRequest, CreatePartnerSubAccountRequestResponse,
        CreatePartnerSubAccountRequestResponseStatus, OpenDemoAccount, OpenDemoAccountResponse,
    },
};
use crate::{Client, Error, client::MutationAssessment};

use super::validation::{
    has_error, require_live, validate_combined_names, validate_country_code, validate_person_name,
    validate_required_text,
};

/// Locally validated production Partner subaccount request.
///
/// Construct this type with [`TryFrom<CreatePartnerSubAccountRequest>`] after
/// using the generated schema builder. It additionally enforces the current
/// prose-only identity, name, country, and document-set invariants.
#[derive(Clone, Debug, serde::Serialize)]
#[serde(transparent)]
pub struct PartnerSubAccountRequest(CreatePartnerSubAccountRequest);

impl TryFrom<CreatePartnerSubAccountRequest> for PartnerSubAccountRequest {
    type Error = Error;

    fn try_from(request: CreatePartnerSubAccountRequest) -> Result<Self, Self::Error> {
        validate_partner_subaccount(&request)?;
        Ok(Self(request))
    }
}

impl crate::api::current::support::CurrentRequest for PartnerSubAccountRequest {
    fn validate_current(&self) -> Result<(), Error> {
        validate_partner_subaccount(&self.0)
    }
}

impl Client {
    /// Creates a live Partner subaccount application request.
    ///
    /// The pinned production-only contract is validated locally, including its
    /// country-specific identity grammar, all-or-none document IDs, and
    /// combined-name ceiling. A returned positive request ID is required before
    /// completion is reported.
    ///
    /// # Errors
    ///
    /// Returns local validation, authentication, rate, transport, provider,
    /// decoding, or ambiguous-mutation errors.
    pub async fn customer_application_create_partner_subaccount_request(
        &self,
        request: &PartnerSubAccountRequest,
    ) -> Result<CreatePartnerSubAccountRequestResponse, Error> {
        require_live(self)?;
        self.post_reviewed_mutation(
            "/customerApplication/createpartnersubaccountrequest",
            request,
            assess_partner_subaccount,
        )
        .await
    }

    /// Opens one simulation account using the current Partner request model.
    ///
    /// # Errors
    ///
    /// Returns local validation, authentication, rate, transport, provider,
    /// decoding, or ambiguous-mutation errors. A new account ID with no
    /// contradictory error text is required for definitive completion.
    pub async fn user_open_demo_account(
        &self,
        request: &OpenDemoAccount,
    ) -> Result<OpenDemoAccountResponse, Error> {
        if let Some(name) = request.name() {
            validate_required_text(name, "name")?;
        }
        self.post_reviewed_mutation("/user/opendemoaccount", request, assess_open_demo_account)
            .await
    }
}

pub(super) fn validate_partner_subaccount(
    request: &CreatePartnerSubAccountRequest,
) -> Result<(), Error> {
    validate_person_name(request.first_name(), "firstName")?;
    validate_person_name(request.last_name(), "lastName")?;
    validate_combined_names(request.first_name(), request.last_name(), "name")?;
    validate_country_code(request.country(), "country")?;
    validate_country_code(request.citizenship(), "citizenship")?;
    validate_required_text(request.state(), "state")?;
    validate_required_text(request.street_address1(), "streetAddress1")?;
    validate_required_text(request.city(), "city")?;
    validate_required_text(request.zip_code(), "zipCode")?;
    validate_required_text(request.phone(), "phone")?;

    let tax_identifier = request.tax_identifier_secret().map(SecretValue::expose);
    let national_id = request.national_id_secret().map(SecretValue::expose);
    if request.citizenship().eq_ignore_ascii_case("US") {
        validate_tax_identifier(tax_identifier)?;
        if let Some(national_id) = national_id {
            validate_national_id(national_id)?;
        }
    } else {
        let national_id = national_id.ok_or(Error::InvalidRequest {
            field: "nationalId",
            reason: "is required for non-US citizenship",
        })?;
        validate_national_id(national_id)?;
        if let Some(tax_identifier) = tax_identifier {
            validate_tax_identifier(Some(tax_identifier))?;
        }
    }

    let document_count = [
        request.p_oa_form_doc_id().is_some(),
        request.government_doc_id().is_some(),
        request.address_doc_id().is_some(),
    ]
    .into_iter()
    .filter(|present| *present)
    .count();
    if !matches!(document_count, 0 | 3) {
        return Err(Error::InvalidRequest {
            field: "documentIds",
            reason: "pOAFormDocId, governmentDocId, and addressDocId must be supplied together",
        });
    }
    Ok(())
}

fn validate_tax_identifier(value: Option<&str>) -> Result<(), Error> {
    let value = value.ok_or(Error::InvalidRequest {
        field: "taxIdentifier",
        reason: "is required for US citizenship",
    })?;
    if value.len() != 9 || !value.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(Error::InvalidRequest {
            field: "taxIdentifier",
            reason: "must contain exactly nine digits",
        });
    }
    Ok(())
}

fn validate_national_id(value: &str) -> Result<(), Error> {
    if !(2..=64).contains(&value.len()) || !value.bytes().all(|byte| byte.is_ascii_alphanumeric()) {
        return Err(Error::InvalidRequest {
            field: "nationalId",
            reason: "must contain between two and 64 ASCII letters or digits",
        });
    }
    Ok(())
}

pub(super) fn assess_partner_subaccount(
    response: &CreatePartnerSubAccountRequestResponse,
    _: &PartnerSubAccountRequest,
) -> MutationAssessment {
    let has_request_id = response.request_id().is_some();
    let unknown_status = matches!(
        response.status(),
        Some(CreatePartnerSubAccountRequestResponseStatus::Unknown(_))
    );
    if has_error(response.error_text()) {
        return if has_request_id {
            MutationAssessment::ambiguous(true)
        } else {
            MutationAssessment::rejected()
        };
    }
    if has_request_id && !unknown_status {
        MutationAssessment::success()
    } else {
        MutationAssessment::ambiguous(has_request_id)
    }
}

pub(super) fn assess_open_demo_account(
    response: &OpenDemoAccountResponse,
    _: &OpenDemoAccount,
) -> MutationAssessment {
    let has_account_id = response.account_id().is_some();
    if has_error(response.error_text()) {
        return if has_account_id {
            MutationAssessment::ambiguous(true)
        } else {
            MutationAssessment::rejected()
        };
    }
    if has_account_id {
        MutationAssessment::success()
    } else {
        MutationAssessment::ambiguous(false)
    }
}

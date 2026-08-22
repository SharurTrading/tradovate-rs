// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Organization-member and contact-profile mutations.

use crate::api::current::users::{
    ModifyEmailAddress, SignUpOrganizationMember, SignUpResponse, SignUpResponseErrorCode,
    UpdateContactInfo, UpdateContactInfoResponse, UserStatusMessage,
};
use crate::{Client, Error, client::MutationAssessment};

use super::validation::{has_error, validate_combined_names, validate_required_text};

/// Locally validated organization-member creation request.
#[derive(Clone, Debug, serde::Serialize)]
#[serde(transparent)]
pub struct OrganizationMemberRequest(SignUpOrganizationMember);

impl TryFrom<SignUpOrganizationMember> for OrganizationMemberRequest {
    type Error = Error;

    fn try_from(request: SignUpOrganizationMember) -> Result<Self, Self::Error> {
        validate_organization_member(&request)?;
        Ok(Self(request))
    }
}

impl crate::api::current::support::CurrentRequest for OrganizationMemberRequest {
    fn validate_current(&self) -> Result<(), Error> {
        validate_organization_member(&self.0)
    }
}

/// Locally validated full contact-information update.
#[derive(Clone, Debug, serde::Serialize)]
#[serde(transparent)]
pub struct ContactInfoUpdateRequest(UpdateContactInfo);

impl TryFrom<UpdateContactInfo> for ContactInfoUpdateRequest {
    type Error = Error;

    fn try_from(request: UpdateContactInfo) -> Result<Self, Self::Error> {
        validate_contact_info(&request)?;
        Ok(Self(request))
    }
}

impl crate::api::current::support::CurrentRequest for ContactInfoUpdateRequest {
    fn validate_current(&self) -> Result<(), Error> {
        validate_contact_info(&self.0)
    }
}

impl Client {
    /// Requests an email-address change for the selected user.
    ///
    /// The current response exposes only account status and does not echo the
    /// user or email. A transmitted 2xx therefore remains ambiguous and must be
    /// reconciled through authoritative user state.
    ///
    /// # Errors
    ///
    /// Returns local validation, authentication, rate, transport, provider,
    /// decoding, or ambiguous-mutation errors. This method never treats account
    /// status alone as proof that the requested email was installed.
    pub async fn user_modify_email_address(
        &self,
        request: &ModifyEmailAddress,
    ) -> Result<UserStatusMessage, Error> {
        validate_required_text(request.email(), "email")?;
        self.post_reviewed_mutation("/user/modifyemailaddress", request, assess_modify_email)
            .await
    }

    /// Creates one organization member through the current Partner endpoint.
    ///
    /// # Errors
    ///
    /// Returns local validation, authentication, rate, transport, provider,
    /// decoding, or ambiguous-mutation errors. Completion requires the known
    /// `Success` code, a new user ID, and no contradictory error text.
    pub async fn user_signup_organization_member(
        &self,
        request: &OrganizationMemberRequest,
    ) -> Result<SignUpResponse, Error> {
        self.post_reviewed_mutation(
            "/user/signuporganizationmember",
            request,
            assess_organization_member,
        )
        .await
    }

    /// Replaces the specified user's current contact information.
    ///
    /// # Errors
    ///
    /// Returns local validation, authentication, rate, transport, provider,
    /// decoding, or ambiguous-mutation errors. Every requested field must be
    /// echoed by an identified contact entity for definitive completion.
    pub async fn contact_info_update_contact_info(
        &self,
        request: &ContactInfoUpdateRequest,
    ) -> Result<UpdateContactInfoResponse, Error> {
        self.post_reviewed_mutation(
            "/contactInfo/updatecontactinfo",
            request,
            assess_contact_info,
        )
        .await
    }
}

pub(super) fn validate_organization_member(
    request: &SignUpOrganizationMember,
) -> Result<(), Error> {
    validate_required_text(request.name_secret().expose(), "name")?;
    validate_required_text(request.email(), "email")?;
    validate_required_text(request.first_name(), "firstName")?;
    validate_required_text(request.last_name(), "lastName")?;
    validate_combined_names(request.first_name(), request.last_name(), "name")
}

pub(super) fn validate_contact_info(request: &UpdateContactInfo) -> Result<(), Error> {
    validate_combined_names(request.first_name(), request.last_name(), "name")?;
    validate_combined_names(
        request.joint_first_name().unwrap_or_default(),
        request.joint_last_name().unwrap_or_default(),
        "jointName",
    )
}

pub(super) fn assess_modify_email(
    response: &UserStatusMessage,
    _: &ModifyEmailAddress,
) -> MutationAssessment {
    let has_status = response.status().is_some();
    if has_error(response.error_text()) && !has_status {
        MutationAssessment::rejected()
    } else {
        MutationAssessment::ambiguous(has_status)
    }
}

pub(super) fn assess_organization_member(
    response: &SignUpResponse,
    _: &OrganizationMemberRequest,
) -> MutationAssessment {
    let has_user_id = response.user_id().is_some();
    let success = matches!(response.error_code(), SignUpResponseErrorCode::Success);
    let unknown = matches!(response.error_code(), SignUpResponseErrorCode::Unknown(_));
    if has_error(response.error_text()) {
        return if success || has_user_id {
            MutationAssessment::ambiguous(true)
        } else if unknown {
            MutationAssessment::ambiguous(false)
        } else {
            MutationAssessment::rejected()
        };
    }
    match (success, has_user_id, unknown) {
        (true, true, false) => MutationAssessment::success(),
        (false, false, false) => MutationAssessment::rejected(),
        _ => MutationAssessment::ambiguous(success || has_user_id),
    }
}

pub(super) fn assess_contact_info(
    response: &UpdateContactInfoResponse,
    request: &ContactInfoUpdateRequest,
) -> MutationAssessment {
    let request = &request.0;
    let contact = response.contact_info();
    let exact = contact.is_some_and(|contact| {
        contact.id().is_some()
            && contact.user_id() == request.user_id()
            && contact.first_name() == request.first_name()
            && contact.last_name() == request.last_name()
            && contact.street_address1() == request.street_address1()
            && contact.city() == request.city()
            && contact.country() == request.country()
            && contact.phone() == request.phone()
            && optional_matches(request.street_address2(), contact.street_address2())
            && optional_matches(request.state(), contact.state())
            && optional_matches(request.post_code(), contact.post_code())
            && optional_matches(
                request.mailing_street_address1(),
                contact.mailing_street_address1(),
            )
            && optional_matches(
                request.mailing_street_address2(),
                contact.mailing_street_address2(),
            )
            && optional_matches(request.mailing_city(), contact.mailing_city())
            && optional_matches(request.mailing_state(), contact.mailing_state())
            && optional_matches(request.mailing_post_code(), contact.mailing_post_code())
            && optional_matches(request.mailing_country(), contact.mailing_country())
            && optional_matches(request.joint_first_name(), contact.joint_first_name())
            && optional_matches(request.joint_last_name(), contact.joint_last_name())
            && request
                .mailing_is_different()
                .is_none_or(|expected| contact.mailing_is_different() == Some(expected))
            && request
                .approved_id()
                .is_none_or(|expected| contact.approved_id() == Some(expected))
    });
    if has_error(response.error_text()) {
        return if contact.is_some() {
            MutationAssessment::ambiguous(true)
        } else {
            MutationAssessment::rejected()
        };
    }
    if exact {
        MutationAssessment::success()
    } else {
        MutationAssessment::ambiguous(false)
    }
}

fn optional_matches<T: PartialEq + ?Sized>(expected: Option<&T>, actual: Option<&T>) -> bool {
    expected.is_none_or(|expected| actual == Some(expected))
}

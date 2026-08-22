// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Workspace-template and contact-information mutations.

use crate::{
    Client, Error,
    api::current::users::{
        UpdateContactCountry, UpdateContactInfoName, UpdateContactInfoResponse, WorkspaceTemplate,
    },
    client::MutationAssessment,
};

use super::common::exact_entity;

impl Client {
    /// Creates a named workspace template.
    ///
    /// # Errors
    ///
    /// Returns local validation, authentication, rate, transport, provider,
    /// decoding, or ambiguous-mutation errors. A create request must not carry
    /// an ID, and success requires the response to assign an ID and echo the
    /// exact name.
    pub async fn workspace_template_create(
        &self,
        request: &WorkspaceTemplate,
    ) -> Result<WorkspaceTemplate, Error> {
        if request.id().is_some() {
            return Err(Error::InvalidRequest {
                field: "id",
                reason: "must be absent when creating a workspace template",
            });
        }
        self.post_reviewed_mutation(
            "/workspaceTemplate/create",
            request,
            assess_workspace_create,
        )
        .await
    }

    /// Updates an existing named workspace template.
    ///
    /// # Errors
    ///
    /// Returns local validation, authentication, rate, transport, provider,
    /// decoding, or ambiguous-mutation errors. An update request must carry an
    /// ID, and the response must echo both that ID and the exact name.
    pub async fn workspace_template_update(
        &self,
        request: &WorkspaceTemplate,
    ) -> Result<WorkspaceTemplate, Error> {
        if request.id().is_none() {
            return Err(Error::InvalidRequest {
                field: "id",
                reason: "is required when updating a workspace template",
            });
        }
        self.post_reviewed_mutation(
            "/workspaceTemplate/update",
            request,
            assess_workspace_update,
        )
        .await
    }

    /// Updates the current user's contact country.
    ///
    /// # Errors
    ///
    /// Returns local validation, authentication, rate, transport, provider,
    /// decoding, or ambiguous-mutation errors. The returned contact record must
    /// echo the exact requested country before the mutation is resolved.
    pub async fn contact_info_update_contact_country(
        &self,
        request: &UpdateContactCountry,
    ) -> Result<UpdateContactInfoResponse, Error> {
        self.post_reviewed_mutation(
            "/contactInfo/updatecontactcountry",
            request,
            assess_contact_country,
        )
        .await
    }

    /// Updates the current user's contact name, country, and phone.
    ///
    /// # Errors
    ///
    /// Returns local validation, authentication, rate, transport, provider,
    /// decoding, or ambiguous-mutation errors. The combined first and last
    /// names may not exceed the current documented 60-character limit. Every
    /// field echoed by the returned contact record must match exactly.
    pub async fn contact_info_update_contact_info_name(
        &self,
        request: &UpdateContactInfoName,
    ) -> Result<UpdateContactInfoResponse, Error> {
        let combined_name_chars = request
            .first_name()
            .chars()
            .count()
            .saturating_add(request.last_name().chars().count());
        if combined_name_chars > 60 {
            return Err(Error::InvalidRequest {
                field: "firstName/lastName",
                reason: "combined length must not exceed 60 characters",
            });
        }
        self.post_reviewed_mutation(
            "/contactInfo/updatecontactinfoname",
            request,
            assess_contact_name,
        )
        .await
    }
}

fn assess_workspace_create(
    response: &WorkspaceTemplate,
    request: &WorkspaceTemplate,
) -> MutationAssessment {
    if response.id().is_some() && response.name() == request.name() {
        MutationAssessment::success()
    } else {
        MutationAssessment::ambiguous(false)
    }
}

fn assess_workspace_update(
    response: &WorkspaceTemplate,
    request: &WorkspaceTemplate,
) -> MutationAssessment {
    let exact_id = response
        .id()
        .zip(request.id())
        .is_some_and(|(response_id, request_id)| response_id == request_id);
    let exact = exact_id && response.name() == request.name();
    if exact {
        MutationAssessment::success()
    } else {
        MutationAssessment::ambiguous(false)
    }
}

fn assess_contact_country(
    response: &UpdateContactInfoResponse,
    request: &UpdateContactCountry,
) -> MutationAssessment {
    let exact = response
        .contact_info()
        .is_some_and(|contact| contact.country() == request.country());
    exact_entity(response.error_text(), exact)
}

fn assess_contact_name(
    response: &UpdateContactInfoResponse,
    request: &UpdateContactInfoName,
) -> MutationAssessment {
    let exact = response.contact_info().is_some_and(|contact| {
        contact.first_name() == request.first_name()
            && contact.last_name() == request.last_name()
            && contact.country() == request.country()
            && contact.phone() == request.phone()
    });
    exact_entity(response.error_text(), exact)
}

#[cfg(test)]
#[path = "workspace/tests.rs"]
mod tests;

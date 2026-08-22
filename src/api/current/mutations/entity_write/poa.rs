// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Secret-safe power-of-attorney contact entity writes.

use serde::Serialize;

use crate::{
    Client, Error,
    api::current::{SecretValue, support::CurrentRequest, users::POAContact},
    client::MutationAssessment,
};

use super::validation::{
    combined_names, country_code, optional_text, required_text, same_date, trade_date,
};

/// A POA contact create request with no caller-supplied entity ID.
#[derive(Clone, Debug, Serialize)]
#[serde(transparent)]
pub struct CreatePoaContactRequest(POAContact);

impl CreatePoaContactRequest {
    /// Validates a generated POA contact for create semantics.
    ///
    /// Secret tax and national identifiers remain redacted and have no public
    /// accessor on this wrapper.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidRequest`] for an existing ID, malformed contact
    /// field, invalid country code or date, or a combined name over 60 characters.
    pub fn new(value: POAContact) -> Result<Self, Error> {
        validate_contact(&value, false)?;
        Ok(Self(value))
    }

    /// Returns the validated entity; its secret fields remain inaccessible.
    #[must_use]
    pub const fn entity(&self) -> &POAContact {
        &self.0
    }
}

impl TryFrom<POAContact> for CreatePoaContactRequest {
    type Error = Error;

    fn try_from(value: POAContact) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl CurrentRequest for CreatePoaContactRequest {
    fn validate_current(&self) -> Result<(), Error> {
        validate_contact(&self.0, false)
    }
}

/// A POA contact update request that requires its entity ID.
#[derive(Clone, Debug, Serialize)]
#[serde(transparent)]
pub struct UpdatePoaContactRequest(POAContact);

impl UpdatePoaContactRequest {
    /// Validates a generated POA contact for update semantics.
    ///
    /// Secret tax and national identifiers remain redacted and have no public
    /// accessor on this wrapper.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidRequest`] for a missing ID, malformed contact
    /// field, invalid country code or date, or a combined name over 60 characters.
    pub fn new(value: POAContact) -> Result<Self, Error> {
        validate_contact(&value, true)?;
        Ok(Self(value))
    }

    /// Returns the validated entity; its secret fields remain inaccessible.
    #[must_use]
    pub const fn entity(&self) -> &POAContact {
        &self.0
    }
}

impl TryFrom<POAContact> for UpdatePoaContactRequest {
    type Error = Error;

    fn try_from(value: POAContact) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl CurrentRequest for UpdatePoaContactRequest {
    fn validate_current(&self) -> Result<(), Error> {
        validate_contact(&self.0, true)
    }
}

impl Client {
    /// Creates one current POA contact.
    ///
    /// # Errors
    ///
    /// Returns validation, authentication, rate, transport, provider-control,
    /// decoding, or ambiguity errors. Success requires a new response ID and
    /// exact stable user, organization, contact, date, and protected-ID values.
    pub async fn poa_contact_create(
        &self,
        request: &CreatePoaContactRequest,
    ) -> Result<POAContact, Error> {
        self.post_reviewed_mutation("/pOAContact/create", request, assess_create)
            .await
    }

    /// Updates one current POA contact.
    ///
    /// # Errors
    ///
    /// Returns validation, authentication, rate, transport, provider-control,
    /// decoding, or ambiguity errors. Success requires the exact response ID
    /// and exact stable user, organization, contact, date, and protected-ID values.
    pub async fn poa_contact_update(
        &self,
        request: &UpdatePoaContactRequest,
    ) -> Result<POAContact, Error> {
        self.post_reviewed_mutation("/pOAContact/update", request, assess_update)
            .await
    }
}

fn validate_contact(value: &POAContact, update: bool) -> Result<(), Error> {
    value.validate_current()?;
    match (update, value.id().is_some()) {
        (false, true) => {
            return Err(Error::InvalidRequest {
                field: "id",
                reason: "must be absent when creating a POA contact",
            });
        }
        (true, false) => {
            return Err(Error::InvalidRequest {
                field: "id",
                reason: "is required when updating a POA contact",
            });
        }
        _ => {}
    }

    required_text(value.first_name(), "firstName")?;
    required_text(value.last_name(), "lastName")?;
    combined_names(value.first_name(), value.last_name())?;
    country_code(value.country(), "country")?;
    country_code(value.citizenship(), "citizenship")?;
    required_text(value.state(), "state")?;
    required_text(value.street_address1(), "streetAddress1")?;
    optional_text(value.street_address2(), "streetAddress2")?;
    required_text(value.city(), "city")?;
    required_text(value.zip_code(), "zipCode")?;
    required_text(value.phone(), "phone")?;
    trade_date(value.birth_date(), "birthDate")
}

fn assess_create(response: &POAContact, request: &CreatePoaContactRequest) -> MutationAssessment {
    assess(
        response.id().is_some(),
        same_payload(response, request.entity()),
    )
}

fn assess_update(response: &POAContact, request: &UpdatePoaContactRequest) -> MutationAssessment {
    let exact_id = response.id() == request.entity().id();
    assess(
        response.id().is_some(),
        exact_id && same_payload(response, request.entity()),
    )
}

fn assess(has_id: bool, exact: bool) -> MutationAssessment {
    if has_id && exact {
        MutationAssessment::success()
    } else {
        MutationAssessment::ambiguous(has_id)
    }
}

fn same_payload(left: &POAContact, right: &POAContact) -> bool {
    left.user_id() == right.user_id()
        && left.first_name() == right.first_name()
        && left.last_name() == right.last_name()
        && left.country() == right.country()
        && left.state() == right.state()
        && left.street_address1() == right.street_address1()
        && optional_matches(right.street_address2(), left.street_address2())
        && left.city() == right.city()
        && left.zip_code() == right.zip_code()
        && left.phone() == right.phone()
        && left.citizenship() == right.citizenship()
        && optional_secret_matches(right.tax_identifier_secret(), left.tax_identifier_secret())
        && optional_secret_matches(right.national_id_secret(), left.national_id_secret())
        && same_date(left.birth_date(), right.birth_date())
        && left.organization_id() == right.organization_id()
}

fn optional_secret_matches(
    requested: Option<&SecretValue>,
    returned: Option<&SecretValue>,
) -> bool {
    requested.is_none_or(|value| {
        returned.is_some_and(|other| other.expose().as_bytes() == value.expose().as_bytes())
    })
}

fn optional_matches<T: PartialEq + ?Sized>(requested: Option<&T>, returned: Option<&T>) -> bool {
    requested.is_none_or(|value| returned == Some(value))
}

#[cfg(test)]
#[path = "tests/poa.rs"]
mod tests;

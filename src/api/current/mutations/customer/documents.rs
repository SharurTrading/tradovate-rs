// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Bounded, secret-safe production document uploads.

use crate::api::current::users::{
    SubmitCustomerApplicationDocument, SubmitCustomerApplicationDocumentResponse,
    SubmitPartnerSubAccountDocument, SubmitPartnerSubAccountDocumentResponse,
};
use crate::{Client, Error, client::MutationAssessment};

use super::validation::{has_error, require_live};

const MAX_CUSTOMER_DATA_URI_CHARS: usize = 8_388_608;
const MAX_PARTNER_REQUEST_BYTES: usize = 5 * 1024 * 1024;
// The data URI is ASCII and every other wire field is bounded. Reserving 1 KiB
// makes the complete serialized JSON body strictly smaller than 5 MiB without
// ever materializing a second, non-secret copy merely to measure it.
const PARTNER_JSON_ENVELOPE_ALLOWANCE: usize = 1024;
const MAX_PARTNER_DATA_URI_BYTES: usize =
    MAX_PARTNER_REQUEST_BYTES - PARTNER_JSON_ENVELOPE_ALLOWANCE;

/// Validated production customer-application document upload.
#[derive(Clone, Debug, serde::Serialize)]
#[serde(transparent)]
pub struct CustomerApplicationDocumentRequest(SubmitCustomerApplicationDocument);

impl TryFrom<SubmitCustomerApplicationDocument> for CustomerApplicationDocumentRequest {
    type Error = Error;

    fn try_from(request: SubmitCustomerApplicationDocument) -> Result<Self, Self::Error> {
        validate_customer_document(&request)?;
        Ok(Self(request))
    }
}

impl crate::api::current::support::CurrentRequest for CustomerApplicationDocumentRequest {
    fn validate_current(&self) -> Result<(), Error> {
        validate_customer_document(&self.0)
    }
}

/// Validated production Partner subaccount document upload.
#[derive(Clone, Debug, serde::Serialize)]
#[serde(transparent)]
pub struct PartnerSubAccountDocumentRequest(SubmitPartnerSubAccountDocument);

impl TryFrom<SubmitPartnerSubAccountDocument> for PartnerSubAccountDocumentRequest {
    type Error = Error;

    fn try_from(request: SubmitPartnerSubAccountDocument) -> Result<Self, Self::Error> {
        validate_partner_document(&request)?;
        Ok(Self(request))
    }
}

impl crate::api::current::support::CurrentRequest for PartnerSubAccountDocumentRequest {
    fn validate_current(&self) -> Result<(), Error> {
        validate_partner_document(&self.0)
    }
}

impl Client {
    /// Uploads one production customer-application document.
    ///
    /// The data is accepted only as a supported, syntactically valid base64
    /// data URI no longer than 8,388,608 characters. Its secret wrapper has no
    /// public getter and is never included in diagnostic output.
    ///
    /// # Errors
    ///
    /// Returns local validation, authentication, rate, transport, provider,
    /// decoding, or ambiguous-mutation errors. Success requires both `ok: true`
    /// and a new document ID with no contradictory error text.
    pub async fn customer_application_submit_customer_application_document(
        &self,
        request: &CustomerApplicationDocumentRequest,
    ) -> Result<SubmitCustomerApplicationDocumentResponse, Error> {
        require_live(self)?;
        self.post_reviewed_mutation(
            "/customerApplication/submitcustomerapplicationdocument",
            request,
            assess_customer_document,
        )
        .await
    }

    /// Uploads one production document for an existing subaccount request.
    ///
    /// Validation reserves bounded JSON-envelope headroom so the complete body
    /// remains below the documented five-MiB ceiling.
    ///
    /// # Errors
    ///
    /// Returns local validation, authentication, rate, transport, provider,
    /// decoding, or ambiguous-mutation errors. A new document ID with no
    /// contradictory error text is required for definitive completion.
    pub async fn customer_application_submit_partner_subaccount_document(
        &self,
        request: &PartnerSubAccountDocumentRequest,
    ) -> Result<SubmitPartnerSubAccountDocumentResponse, Error> {
        require_live(self)?;
        self.post_reviewed_mutation(
            "/customerApplication/submitpartnersubaccountdocument",
            request,
            assess_partner_document,
        )
        .await
    }
}

pub(super) fn validate_customer_document(
    request: &SubmitCustomerApplicationDocument,
) -> Result<(), Error> {
    validate_filename(request.filename())?;
    validate_data_uri(
        request.base64data_secret().expose(),
        MAX_CUSTOMER_DATA_URI_CHARS,
    )
}

pub(super) fn validate_partner_document(
    request: &SubmitPartnerSubAccountDocument,
) -> Result<(), Error> {
    validate_filename(request.filename())?;
    validate_data_uri(
        request.base64data_secret().expose(),
        MAX_PARTNER_DATA_URI_BYTES,
    )
}

fn validate_filename(filename: &str) -> Result<(), Error> {
    let length = filename.chars().count();
    if !(5..=64).contains(&length) {
        return Err(Error::InvalidRequest {
            field: "filename",
            reason: "must contain between five and 64 characters",
        });
    }
    if filename.trim() != filename || filename.chars().any(char::is_control) {
        return Err(Error::InvalidRequest {
            field: "filename",
            reason: "must not be padded or contain control characters",
        });
    }
    Ok(())
}

pub(super) fn validate_data_uri(value: &str, max_bytes: usize) -> Result<(), Error> {
    if value.len() > max_bytes {
        return Err(Error::InvalidRequest {
            field: "base64data",
            reason: "exceeds the documented endpoint size ceiling",
        });
    }
    let Some((mime, payload)) = value
        .strip_prefix("data:")
        .and_then(|value| value.split_once(";base64,"))
    else {
        return Err(Error::InvalidRequest {
            field: "base64data",
            reason: "must use the data:<mime>;base64,<payload> grammar",
        });
    };
    if !supported_mime(mime) {
        return Err(Error::InvalidRequest {
            field: "base64data",
            reason: "uses an unsupported MIME type",
        });
    }
    validate_base64(payload)
}

fn supported_mime(mime: &str) -> bool {
    if matches!(mime, "application/pdf" | "text/plain") {
        return true;
    }
    mime.strip_prefix("image/").is_some_and(|subtype| {
        !subtype.is_empty()
            && subtype
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'+' | b'-'))
    })
}

fn validate_base64(payload: &str) -> Result<(), Error> {
    if payload.is_empty() || !payload.is_ascii() {
        return Err(Error::InvalidRequest {
            field: "base64data",
            reason: "must contain a non-empty ASCII base64 payload",
        });
    }
    let padding = payload
        .bytes()
        .rev()
        .take_while(|byte| *byte == b'=')
        .count();
    let content_length = payload.len().saturating_sub(padding);
    let alphabet = payload.as_bytes()[..content_length]
        .iter()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'+' | b'/'));
    let padding_valid = padding <= 2
        && !payload.as_bytes()[..content_length].contains(&b'=')
        && if padding == 0 {
            !payload.len().wrapping_sub(1).is_multiple_of(4)
        } else {
            payload.len().is_multiple_of(4)
        };
    if !alphabet || !padding_valid {
        return Err(Error::InvalidRequest {
            field: "base64data",
            reason: "contains invalid base64 syntax",
        });
    }
    Ok(())
}

pub(super) fn assess_customer_document(
    response: &SubmitCustomerApplicationDocumentResponse,
    _: &CustomerApplicationDocumentRequest,
) -> MutationAssessment {
    let accepted = *response.ok();
    let has_id = response.document_id().is_some();
    if has_error(response.error_text()) {
        return if accepted || has_id {
            MutationAssessment::ambiguous(true)
        } else {
            MutationAssessment::rejected()
        };
    }
    match (accepted, has_id) {
        (true, true) => MutationAssessment::success(),
        (false, false) => MutationAssessment::rejected(),
        _ => MutationAssessment::ambiguous(accepted || has_id),
    }
}

pub(super) fn assess_partner_document(
    response: &SubmitPartnerSubAccountDocumentResponse,
    _: &PartnerSubAccountDocumentRequest,
) -> MutationAssessment {
    let has_id = response.document_id().is_some();
    if has_error(response.error_text()) {
        return if has_id {
            MutationAssessment::ambiguous(true)
        } else {
            MutationAssessment::rejected()
        };
    }
    if has_id {
        MutationAssessment::success()
    } else {
        MutationAssessment::ambiguous(false)
    }
}

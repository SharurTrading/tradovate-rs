// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Secret-safe classification of real-time response control payloads.

use std::{fmt, time::Duration};

use secrecy::{ExposeSecret, SecretString, zeroize::Zeroizing};
use serde::Deserialize;
use serde_json::value::RawValue;
use thiserror::Error;

/// Structural failures while classifying controls or constructing an exact retry.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Error)]
pub(super) enum ControlError {
    /// A response used a reserved control field with an invalid or contradictory shape.
    #[error("real-time response contained a malformed provider control envelope")]
    MalformedControl,
}

/// Provider control classification for a successful HTTP-style response.
#[derive(Debug)]
pub(super) enum ResponseControl {
    /// No provider control was present; retain the response's normal payload.
    Payload,
    /// The provider returned a business-level rejection without exposing its text.
    BusinessFailure {
        /// Count of structured violations when that control caused the failure.
        violation_count: Option<usize>,
    },
    /// The provider returned a validated delayed-retry control.
    Penalty(PenaltyControl),
}

/// A validated provider penalty whose opaque ticket never has a public accessor.
pub(super) struct PenaltyControl {
    ticket: SecretString,
    retry_after: Duration,
    captcha_required: bool,
}

impl PenaltyControl {
    pub(super) fn into_parts(self) -> (String, Duration, bool) {
        (
            self.ticket.expose_secret().to_owned(),
            self.retry_after,
            self.captcha_required,
        )
    }
}

impl fmt::Debug for PenaltyControl {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PenaltyControl")
            .field("ticket", &"[REDACTED]")
            .field("retry_after", &self.retry_after)
            .field("captcha_required", &self.captcha_required)
            .finish_non_exhaustive()
    }
}

/// Classifies optional response `d` without retaining provider diagnostic text.
pub(super) fn inspect(data: Option<&RawValue>) -> Result<ResponseControl, ControlError> {
    let Some(data) = data else {
        return Ok(ResponseControl::Payload);
    };
    if !data.get().trim_start().starts_with('{') {
        return Ok(ResponseControl::Payload);
    }
    let fields = serde_json::from_str::<ControlFields>(data.get())
        .map_err(|_| ControlError::MalformedControl)?;
    classify(&fields)
}

fn classify(fields: &ControlFields) -> Result<ResponseControl, ControlError> {
    let business_fields_present = fields.error_text.is_some()
        || fields.failure_reason.is_some()
        || fields.violations.is_some();
    let penalty_fields_present = fields.penalty_ticket.is_some()
        || fields.penalty_time.is_some()
        || fields.captcha.is_some()
        || fields.penalty_message.is_some();
    let error_text = parse_optional_secret_string(fields.error_text.as_deref())?;
    let failure_reason = parse_optional_secret_string(fields.failure_reason.as_deref())?;
    let business_failure = error_text.as_ref().is_some_and(|text| !text.is_empty())
        || failure_reason
            .as_ref()
            .is_some_and(|reason| !reason.is_empty() && reason.as_str() != "Success");
    let violation_count = fields
        .violations
        .as_deref()
        .map(|raw| serde_json::from_str::<Vec<serde::de::IgnoredAny>>(raw.get()))
        .transpose()
        .map_err(|_| ControlError::MalformedControl)?
        .map(|violations| violations.len());

    if failure_reason
        .as_ref()
        .is_some_and(|reason| reason.is_empty())
    {
        return Err(ControlError::MalformedControl);
    }
    if penalty_fields_present {
        if business_fields_present {
            return Err(ControlError::MalformedControl);
        }
        return parse_penalty(fields);
    }
    if business_failure || violation_count.is_some_and(|count| count != 0) {
        Ok(ResponseControl::BusinessFailure { violation_count })
    } else {
        Ok(ResponseControl::Payload)
    }
}

fn parse_penalty(fields: &ControlFields) -> Result<ResponseControl, ControlError> {
    let ticket = fields
        .penalty_ticket
        .as_deref()
        .and_then(parse_secret_string)
        .filter(|ticket| !ticket.expose_secret().is_empty())
        .ok_or(ControlError::MalformedControl)?;
    let seconds = fields
        .penalty_time
        .as_deref()
        .and_then(|raw| serde_json::from_str::<u64>(raw.get()).ok())
        .ok_or(ControlError::MalformedControl)?;
    let captcha_required = fields.captcha.as_deref().map_or(Ok(false), |raw| {
        serde_json::from_str::<bool>(raw.get()).map_err(|_| ControlError::MalformedControl)
    })?;
    if let Some(message) = fields.penalty_message.as_deref() {
        parse_optional_secret_string(Some(message))?;
    }
    Ok(ResponseControl::Penalty(PenaltyControl {
        ticket,
        retry_after: Duration::from_secs(seconds),
        captcha_required,
    }))
}

fn parse_optional_secret_string(
    raw: Option<&RawValue>,
) -> Result<Option<Zeroizing<String>>, ControlError> {
    raw.map(|raw| {
        serde_json::from_str::<String>(raw.get())
            .map(Zeroizing::new)
            .map_err(|_| ControlError::MalformedControl)
    })
    .transpose()
}

fn parse_secret_string(raw: &RawValue) -> Option<SecretString> {
    serde_json::from_str::<String>(raw.get())
        .ok()
        .map(SecretString::from)
}

#[derive(Deserialize)]
struct ControlFields {
    #[serde(rename = "errorText", default, deserialize_with = "present_raw")]
    error_text: Option<Box<RawValue>>,
    #[serde(rename = "failureReason", default, deserialize_with = "present_raw")]
    failure_reason: Option<Box<RawValue>>,
    #[serde(default, deserialize_with = "present_raw")]
    violations: Option<Box<RawValue>>,
    #[serde(rename = "p-ticket", default, deserialize_with = "present_raw")]
    penalty_ticket: Option<Box<RawValue>>,
    #[serde(rename = "p-time", default, deserialize_with = "present_raw")]
    penalty_time: Option<Box<RawValue>>,
    #[serde(rename = "p-captcha", default, deserialize_with = "present_raw")]
    captcha: Option<Box<RawValue>>,
    #[serde(rename = "p-message", default, deserialize_with = "present_raw")]
    penalty_message: Option<Box<RawValue>>,
}

fn present_raw<'de, D>(deserializer: D) -> Result<Option<Box<RawValue>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Box::<RawValue>::deserialize(deserializer).map(Some)
}

#[cfg(test)]
#[path = "control/tests.rs"]
mod tests;

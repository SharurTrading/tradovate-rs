// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Shared validation and evidence helpers for customer mutations.

use crate::{Client, Error, UserId};

pub(super) fn require_live(client: &Client) -> Result<(), Error> {
    if client.endpoints.permits_live_only_rest() {
        Ok(())
    } else {
        Err(Error::InvalidRequest {
            field: "environment",
            reason: "the current Partner operation is available only on live REST endpoints",
        })
    }
}

pub(super) fn effective_user(client: &Client, requested: Option<UserId>) -> Result<UserId, Error> {
    requested
        .or_else(|| client.session_info().map(|session| session.user_id()))
        .ok_or(Error::Unauthenticated)
}

pub(super) fn has_error(error: Option<&str>) -> bool {
    error.is_some_and(|value| !value.is_empty())
}

pub(super) fn validate_combined_names(
    first_name: &str,
    last_name: &str,
    field: &'static str,
) -> Result<(), Error> {
    let length = first_name
        .chars()
        .count()
        .checked_add(last_name.chars().count())
        .ok_or(Error::InvalidRequest {
            field,
            reason: "combined character count overflowed",
        })?;
    if length > 60 {
        return Err(Error::InvalidRequest {
            field,
            reason: "combined first and last name must not exceed 60 characters",
        });
    }
    Ok(())
}

pub(super) fn validate_required_text(value: &str, field: &'static str) -> Result<(), Error> {
    if value.is_empty() || value.trim() != value {
        return Err(Error::InvalidRequest {
            field,
            reason: "must be non-empty and have no surrounding whitespace",
        });
    }
    if value.chars().any(char::is_control) {
        return Err(Error::InvalidRequest {
            field,
            reason: "must not contain control characters",
        });
    }
    Ok(())
}

pub(super) fn validate_country_code(value: &str, field: &'static str) -> Result<(), Error> {
    if value.len() != 2 || !value.bytes().all(|byte| byte.is_ascii_alphabetic()) {
        return Err(Error::InvalidRequest {
            field,
            reason: "must be a two-letter ISO country code",
        });
    }
    Ok(())
}

pub(super) fn validate_person_name(value: &str, field: &'static str) -> Result<(), Error> {
    validate_required_text(value, field)?;
    if !value
        .chars()
        .all(|character| character.is_alphabetic() || matches!(character, ' ' | '-' | '\'' | '.'))
    {
        return Err(Error::InvalidRequest {
            field,
            reason: "contains a character outside the documented name grammar",
        });
    }
    Ok(())
}

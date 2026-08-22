// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Validated create and update states for current user-plugin entities.

use serde::Serialize;

use crate::{
    Client, Error,
    api::current::{support::CurrentRequest, users::UserPlugin},
    client::MutationAssessment,
};

use super::validation::{ordered_dates, same_date, trade_date};

/// A user-plugin create request with no caller-supplied entity ID.
#[derive(Clone, Debug, Serialize)]
#[serde(transparent)]
pub struct CreateUserPluginRequest(UserPlugin);

impl CreateUserPluginRequest {
    /// Validates a generated entity for create semantics.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidRequest`] for an existing ID, invalid plugin
    /// name, or invalid date range.
    pub fn new(value: UserPlugin) -> Result<Self, Error> {
        validate_plugin(&value, false)?;
        Ok(Self(value))
    }

    /// Returns the validated wire entity.
    #[must_use]
    pub const fn entity(&self) -> &UserPlugin {
        &self.0
    }
}

impl TryFrom<UserPlugin> for CreateUserPluginRequest {
    type Error = Error;

    fn try_from(value: UserPlugin) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl CurrentRequest for CreateUserPluginRequest {
    fn validate_current(&self) -> Result<(), Error> {
        validate_plugin(&self.0, false)
    }
}

/// A user-plugin update request that requires its entity ID.
#[derive(Clone, Debug, Serialize)]
#[serde(transparent)]
pub struct UpdateUserPluginRequest(UserPlugin);

impl UpdateUserPluginRequest {
    /// Validates a generated entity for update semantics.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidRequest`] for a missing ID, invalid plugin name,
    /// or invalid date range.
    pub fn new(value: UserPlugin) -> Result<Self, Error> {
        validate_plugin(&value, true)?;
        Ok(Self(value))
    }

    /// Returns the validated wire entity.
    #[must_use]
    pub const fn entity(&self) -> &UserPlugin {
        &self.0
    }
}

impl TryFrom<UserPlugin> for UpdateUserPluginRequest {
    type Error = Error;

    fn try_from(value: UserPlugin) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl CurrentRequest for UpdateUserPluginRequest {
    fn validate_current(&self) -> Result<(), Error> {
        validate_plugin(&self.0, true)
    }
}

impl Client {
    /// Creates one current user-plugin entity.
    ///
    /// # Errors
    ///
    /// Returns validation, authentication, rate, transport, provider-control,
    /// decoding, or ambiguity errors. Success requires a new response ID and
    /// exact stable plugin, user, account, date, and monetary values.
    pub async fn user_plugin_create(
        &self,
        request: &CreateUserPluginRequest,
    ) -> Result<UserPlugin, Error> {
        self.post_reviewed_mutation("/userPlugin/create", request, assess_create)
            .await
    }

    /// Updates one current user-plugin entity.
    ///
    /// # Errors
    ///
    /// Returns validation, authentication, rate, transport, provider-control,
    /// decoding, or ambiguity errors. Success requires the exact response ID
    /// and exact stable plugin, user, account, date, and monetary values.
    pub async fn user_plugin_update(
        &self,
        request: &UpdateUserPluginRequest,
    ) -> Result<UserPlugin, Error> {
        self.post_reviewed_mutation("/userPlugin/update", request, assess_update)
            .await
    }
}

fn validate_plugin(value: &UserPlugin, update: bool) -> Result<(), Error> {
    value.validate_current()?;
    match (update, value.id().is_some()) {
        (false, true) => {
            return Err(Error::InvalidRequest {
                field: "id",
                reason: "must be absent when creating a user plugin",
            });
        }
        (true, false) => {
            return Err(Error::InvalidRequest {
                field: "id",
                reason: "is required when updating a user plugin",
            });
        }
        _ => {}
    }
    match value.expiration_date() {
        Some(expiration) => ordered_dates(value.start_date(), expiration),
        None => trade_date(value.start_date(), "startDate"),
    }
}

fn assess_create(response: &UserPlugin, request: &CreateUserPluginRequest) -> MutationAssessment {
    assess(
        response.id().is_some(),
        same_payload(response, request.entity()),
    )
}

fn assess_update(response: &UserPlugin, request: &UpdateUserPluginRequest) -> MutationAssessment {
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

fn same_payload(left: &UserPlugin, right: &UserPlugin) -> bool {
    left.user_id() == right.user_id()
        && left.plan_price() == right.plan_price()
        && left.plugin_name() == right.plugin_name()
        && left.approval() == right.approval()
        && same_date(left.start_date(), right.start_date())
        && left.paid_amount() == right.paid_amount()
        && optional_matches(right.account_id(), left.account_id())
        && optional_matches(right.cash_balance_log_id(), left.cash_balance_log_id())
        && optional_matches(right.entitlement_id(), left.entitlement_id())
        && optional_date_matches(right.expiration_date(), left.expiration_date())
        && optional_matches(right.autorenewal(), left.autorenewal())
        && optional_matches(right.plan_categories(), left.plan_categories())
        && optional_matches(right.rebate(), left.rebate())
}

fn optional_date_matches(
    requested: Option<&crate::api::current::users::TradeDate>,
    returned: Option<&crate::api::current::users::TradeDate>,
) -> bool {
    requested.is_none_or(|value| returned.is_some_and(|other| same_date(value, other)))
}

fn optional_matches<T: PartialEq + ?Sized>(requested: Option<&T>, returned: Option<&T>) -> bool {
    requested.is_none_or(|value| returned == Some(value))
}

#[cfg(test)]
#[path = "tests/plugins.rs"]
mod tests;

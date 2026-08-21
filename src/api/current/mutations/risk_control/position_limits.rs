// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Validated create/update states for user-account position limits.

use serde::Serialize;

use crate::{
    Client, Error,
    api::current::{risks::UserAccountPositionLimit, support::CurrentRequest},
    client::MutationAssessment,
};

/// A position-limit create request that cannot carry an existing entity ID.
#[derive(Clone, Debug, Serialize)]
#[serde(transparent)]
pub struct CreateUserAccountPositionLimitRequest(UserAccountPositionLimit);

impl CreateUserAccountPositionLimitRequest {
    /// Validates a generated position-limit entity for create semantics.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidRequest`] when the entity already has an ID.
    pub fn new(value: UserAccountPositionLimit) -> Result<Self, Error> {
        if value.id().is_some() {
            return Err(Error::InvalidRequest {
                field: "id",
                reason: "must be absent when creating a user-account position limit",
            });
        }
        Ok(Self(value))
    }

    /// Returns the validated entity payload.
    #[must_use]
    pub const fn entity(&self) -> &UserAccountPositionLimit {
        &self.0
    }
}

impl TryFrom<UserAccountPositionLimit> for CreateUserAccountPositionLimitRequest {
    type Error = Error;

    fn try_from(value: UserAccountPositionLimit) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl CurrentRequest for CreateUserAccountPositionLimitRequest {
    fn validate_current(&self) -> Result<(), Error> {
        if self.0.id().is_some() {
            Err(Error::InvalidRequest {
                field: "id",
                reason: "must be absent when creating a user-account position limit",
            })
        } else {
            Ok(())
        }
    }
}

/// A position-limit update request that always carries its entity ID.
#[derive(Clone, Debug, Serialize)]
#[serde(transparent)]
pub struct UpdateUserAccountPositionLimitRequest(UserAccountPositionLimit);

impl UpdateUserAccountPositionLimitRequest {
    /// Validates a generated position-limit entity for update semantics.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidRequest`] when the entity has no ID.
    pub fn new(value: UserAccountPositionLimit) -> Result<Self, Error> {
        if value.id().is_none() {
            return Err(Error::InvalidRequest {
                field: "id",
                reason: "is required when updating a user-account position limit",
            });
        }
        Ok(Self(value))
    }

    /// Returns the validated entity payload.
    #[must_use]
    pub const fn entity(&self) -> &UserAccountPositionLimit {
        &self.0
    }
}

impl TryFrom<UserAccountPositionLimit> for UpdateUserAccountPositionLimitRequest {
    type Error = Error;

    fn try_from(value: UserAccountPositionLimit) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl CurrentRequest for UpdateUserAccountPositionLimitRequest {
    fn validate_current(&self) -> Result<(), Error> {
        if self.0.id().is_none() {
            Err(Error::InvalidRequest {
                field: "id",
                reason: "is required when updating a user-account position limit",
            })
        } else {
            Ok(())
        }
    }
}

impl Client {
    /// Creates one user-account position limit.
    ///
    /// # Errors
    ///
    /// Returns validation, authentication, rate, transport, provider-control,
    /// decoding, or ambiguity errors. Success requires an assigned response ID
    /// and an exact echo of every requested field.
    pub async fn user_account_position_limit_create(
        &self,
        request: &CreateUserAccountPositionLimitRequest,
    ) -> Result<UserAccountPositionLimit, Error> {
        self.post_reviewed_mutation("/userAccountPositionLimit/create", request, assess_create)
            .await
    }

    /// Updates one user-account position limit.
    ///
    /// # Errors
    ///
    /// Returns validation, authentication, rate, transport, provider-control,
    /// decoding, or ambiguity errors. Success requires the response to echo the
    /// exact requested ID and every requested field.
    pub async fn user_account_position_limit_update(
        &self,
        request: &UpdateUserAccountPositionLimitRequest,
    ) -> Result<UserAccountPositionLimit, Error> {
        self.post_reviewed_mutation("/userAccountPositionLimit/update", request, assess_update)
            .await
    }
}

fn assess_create(
    response: &UserAccountPositionLimit,
    request: &CreateUserAccountPositionLimitRequest,
) -> MutationAssessment {
    if response.id().is_some() && same_payload(response, request.entity()) {
        MutationAssessment::success()
    } else {
        MutationAssessment::ambiguous(response.id().is_some())
    }
}

fn assess_update(
    response: &UserAccountPositionLimit,
    request: &UpdateUserAccountPositionLimitRequest,
) -> MutationAssessment {
    let exact_id = response.id() == request.entity().id();
    if exact_id && same_payload(response, request.entity()) {
        MutationAssessment::success()
    } else {
        MutationAssessment::ambiguous(response.id().is_some())
    }
}

fn same_payload(left: &UserAccountPositionLimit, right: &UserAccountPositionLimit) -> bool {
    left.contract_id() == right.contract_id()
        && left.product_id() == right.product_id()
        && left.exchange_id() == right.exchange_id()
        && left.product_type() == right.product_type()
        && left.risk_discount_contract_group_id() == right.risk_discount_contract_group_id()
        && left.product_verification_status() == right.product_verification_status()
        && left.contract_group_id() == right.contract_group_id()
        && left.fungible_product_id() == right.fungible_product_id()
        && left.active() == right.active()
        && left.risk_time_period_id() == right.risk_time_period_id()
        && left.total_by() == right.total_by()
        && left.short_limit() == right.short_limit()
        && left.long_limit() == right.long_limit()
        && left.exposed_limit() == right.exposed_limit()
        && left.fungible_exposed_limit() == right.fungible_exposed_limit()
        && left.description() == right.description()
        && left.account_id() == right.account_id()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        AccountId,
        api::current::{ids::UserAccountPositionLimitId, risks::UserAccountPositionLimitTotalBy},
    };

    #[test]
    fn create_and_update_states_are_disjoint() {
        let without_id = fixture(None);
        assert!(CreateUserAccountPositionLimitRequest::new(without_id).is_ok());

        let with_id = fixture(Some(position_limit_id(8)));
        assert!(UpdateUserAccountPositionLimitRequest::new(with_id).is_ok());
    }

    #[test]
    fn create_assessment_requires_assigned_id_and_exact_payload() {
        let request = CreateUserAccountPositionLimitRequest::new(fixture(None));
        let Ok(request) = request else {
            panic!("create fixture must validate");
        };
        let response = fixture(Some(position_limit_id(9)));
        assert_eq!(
            assess_create(&response, &request).outcome(),
            crate::client::MutationOutcome::Success
        );
    }

    fn fixture(id: Option<UserAccountPositionLimitId>) -> UserAccountPositionLimit {
        let builder = UserAccountPositionLimit::builder()
            .active(true)
            .total_by(UserAccountPositionLimitTotalBy::Overall)
            .account_id(account(3));
        let builder = match id {
            Some(id) => builder.id(id),
            None => builder,
        };
        builder
            .build()
            .unwrap_or_else(|error| panic!("position-limit fixture: {error}"))
    }

    fn account(value: i64) -> AccountId {
        AccountId::new(value).unwrap_or_else(|error| panic!("account fixture: {error}"))
    }

    fn position_limit_id(value: i64) -> UserAccountPositionLimitId {
        UserAccountPositionLimitId::new(value)
            .unwrap_or_else(|error| panic!("position-limit ID fixture: {error}"))
    }
}

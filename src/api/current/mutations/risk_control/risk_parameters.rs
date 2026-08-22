// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Validated create/update states for user-account risk parameters.

use serde::Serialize;

use crate::{
    Client, Error,
    api::current::{risks::UserAccountRiskParameter, support::CurrentRequest},
    client::MutationAssessment,
};

/// A risk-parameter create request that cannot carry an existing entity ID.
#[derive(Clone, Debug, Serialize)]
#[serde(transparent)]
pub struct CreateUserAccountRiskParameterRequest(UserAccountRiskParameter);

impl CreateUserAccountRiskParameterRequest {
    /// Validates a generated risk-parameter entity for create semantics.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidRequest`] when the entity already has an ID.
    pub fn new(value: UserAccountRiskParameter) -> Result<Self, Error> {
        if value.id().is_some() {
            return Err(Error::InvalidRequest {
                field: "id",
                reason: "must be absent when creating a user-account risk parameter",
            });
        }
        Ok(Self(value))
    }

    /// Returns the validated entity payload.
    #[must_use]
    pub const fn entity(&self) -> &UserAccountRiskParameter {
        &self.0
    }
}

impl TryFrom<UserAccountRiskParameter> for CreateUserAccountRiskParameterRequest {
    type Error = Error;

    fn try_from(value: UserAccountRiskParameter) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl CurrentRequest for CreateUserAccountRiskParameterRequest {
    fn validate_current(&self) -> Result<(), Error> {
        if self.0.id().is_some() {
            Err(Error::InvalidRequest {
                field: "id",
                reason: "must be absent when creating a user-account risk parameter",
            })
        } else {
            Ok(())
        }
    }
}

/// A risk-parameter update request that always carries its entity ID.
#[derive(Clone, Debug, Serialize)]
#[serde(transparent)]
pub struct UpdateUserAccountRiskParameterRequest(UserAccountRiskParameter);

impl UpdateUserAccountRiskParameterRequest {
    /// Validates a generated risk-parameter entity for update semantics.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidRequest`] when the entity has no ID.
    pub fn new(value: UserAccountRiskParameter) -> Result<Self, Error> {
        if value.id().is_none() {
            return Err(Error::InvalidRequest {
                field: "id",
                reason: "is required when updating a user-account risk parameter",
            });
        }
        Ok(Self(value))
    }

    /// Returns the validated entity payload.
    #[must_use]
    pub const fn entity(&self) -> &UserAccountRiskParameter {
        &self.0
    }
}

impl TryFrom<UserAccountRiskParameter> for UpdateUserAccountRiskParameterRequest {
    type Error = Error;

    fn try_from(value: UserAccountRiskParameter) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl CurrentRequest for UpdateUserAccountRiskParameterRequest {
    fn validate_current(&self) -> Result<(), Error> {
        if self.0.id().is_none() {
            Err(Error::InvalidRequest {
                field: "id",
                reason: "is required when updating a user-account risk parameter",
            })
        } else {
            Ok(())
        }
    }
}

impl Client {
    /// Creates one user-account risk parameter.
    ///
    /// # Errors
    ///
    /// Returns validation, authentication, rate, transport, provider-control,
    /// decoding, or ambiguity errors. Success requires an assigned response ID
    /// and an exact echo of every requested field.
    pub async fn user_account_risk_parameter_create(
        &self,
        request: &CreateUserAccountRiskParameterRequest,
    ) -> Result<UserAccountRiskParameter, Error> {
        self.post_reviewed_mutation("/userAccountRiskParameter/create", request, assess_create)
            .await
    }

    /// Updates one user-account risk parameter.
    ///
    /// # Errors
    ///
    /// Returns validation, authentication, rate, transport, provider-control,
    /// decoding, or ambiguity errors. Success requires the response to echo the
    /// exact requested ID and every requested field.
    pub async fn user_account_risk_parameter_update(
        &self,
        request: &UpdateUserAccountRiskParameterRequest,
    ) -> Result<UserAccountRiskParameter, Error> {
        self.post_reviewed_mutation("/userAccountRiskParameter/update", request, assess_update)
            .await
    }
}

fn assess_create(
    response: &UserAccountRiskParameter,
    request: &CreateUserAccountRiskParameterRequest,
) -> MutationAssessment {
    if response.id().is_some() && same_payload(response, request.entity()) {
        MutationAssessment::success()
    } else {
        MutationAssessment::ambiguous(response.id().is_some())
    }
}

fn assess_update(
    response: &UserAccountRiskParameter,
    request: &UpdateUserAccountRiskParameterRequest,
) -> MutationAssessment {
    let exact_id = response.id() == request.entity().id();
    if exact_id && same_payload(response, request.entity()) {
        MutationAssessment::success()
    } else {
        MutationAssessment::ambiguous(response.id().is_some())
    }
}

fn same_payload(left: &UserAccountRiskParameter, right: &UserAccountRiskParameter) -> bool {
    left.contract_id() == right.contract_id()
        && left.product_id() == right.product_id()
        && left.exchange_id() == right.exchange_id()
        && left.product_type() == right.product_type()
        && left.risk_discount_contract_group_id() == right.risk_discount_contract_group_id()
        && left.product_verification_status() == right.product_verification_status()
        && left.contract_group_id() == right.contract_group_id()
        && left.fungible_product_id() == right.fungible_product_id()
        && left.max_opening_order_qty() == right.max_opening_order_qty()
        && left.max_closing_order_qty() == right.max_closing_order_qty()
        && left.fungible_max_opening_order_qty() == right.fungible_max_opening_order_qty()
        && left.fungible_max_closing_order_qty() == right.fungible_max_closing_order_qty()
        && left.max_back_month() == right.max_back_month()
        && left.pre_expiration_days() == right.pre_expiration_days()
        && left.margin_percentage() == right.margin_percentage()
        && left.margin_dollar_value() == right.margin_dollar_value()
        && left.hard_limit() == right.hard_limit()
        && left.user_account_position_limit_id() == right.user_account_position_limit_id()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::current::ids::{UserAccountPositionLimitId, UserAccountRiskParameterId};

    #[test]
    fn create_and_update_states_are_disjoint() {
        let without_id = fixture(None);
        assert!(CreateUserAccountRiskParameterRequest::new(without_id).is_ok());

        let with_id = fixture(Some(risk_parameter_id(12)));
        assert!(UpdateUserAccountRiskParameterRequest::new(with_id).is_ok());
    }

    #[test]
    fn update_assessment_requires_exact_id_and_payload() {
        let entity = fixture(Some(risk_parameter_id(5)));
        let request = UpdateUserAccountRiskParameterRequest::new(entity);
        let Ok(request) = request else {
            panic!("update fixture must validate");
        };
        let response = fixture(Some(risk_parameter_id(5)));
        assert_eq!(
            assess_update(&response, &request).outcome(),
            crate::client::MutationOutcome::Success
        );
    }

    fn fixture(id: Option<UserAccountRiskParameterId>) -> UserAccountRiskParameter {
        let builder = UserAccountRiskParameter::builder()
            .user_account_position_limit_id(position_limit_id(2));
        let builder = match id {
            Some(id) => builder.id(id),
            None => builder,
        };
        builder
            .build()
            .unwrap_or_else(|error| panic!("risk-parameter fixture: {error}"))
    }

    fn position_limit_id(value: i64) -> UserAccountPositionLimitId {
        UserAccountPositionLimitId::new(value)
            .unwrap_or_else(|error| panic!("position-limit ID fixture: {error}"))
    }

    fn risk_parameter_id(value: i64) -> UserAccountRiskParameterId {
        UserAccountRiskParameterId::new(value)
            .unwrap_or_else(|error| panic!("risk-parameter ID fixture: {error}"))
    }
}

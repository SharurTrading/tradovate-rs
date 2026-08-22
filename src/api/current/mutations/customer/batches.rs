// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Bounded Partner batch-creation mutations.

use std::collections::BTreeSet;

use crate::api::current::{
    ids::TradingPermissionId,
    users::{
        CreateEvaluationAccountResponse, CreateEvaluationAccounts,
        CreateEvaluationAccountsResponse, CreateEvaluationUserResponse, CreateEvaluationUsers,
        CreateEvaluationUsersResponse,
    },
};
use crate::{AccountId, Client, Error, UserId, client::MutationAssessment};

use super::validation::{has_error, validate_combined_names, validate_required_text};

const MAX_BATCH_ITEMS: usize = 100;

/// A locally bounded evaluation-account batch request.
#[derive(Clone, Debug, serde::Serialize)]
#[serde(transparent)]
pub struct EvaluationAccountsRequest(CreateEvaluationAccounts);

impl TryFrom<CreateEvaluationAccounts> for EvaluationAccountsRequest {
    type Error = Error;

    fn try_from(request: CreateEvaluationAccounts) -> Result<Self, Self::Error> {
        validate_account_batch(&request)?;
        Ok(Self(request))
    }
}

impl crate::api::current::support::CurrentRequest for EvaluationAccountsRequest {
    fn validate_current(&self) -> Result<(), Error> {
        validate_account_batch(&self.0)
    }
}

/// A locally bounded evaluation-user batch request.
#[derive(Clone, Debug, serde::Serialize)]
#[serde(transparent)]
pub struct EvaluationUsersRequest(CreateEvaluationUsers);

impl TryFrom<CreateEvaluationUsers> for EvaluationUsersRequest {
    type Error = Error;

    fn try_from(request: CreateEvaluationUsers) -> Result<Self, Self::Error> {
        validate_user_batch(&request)?;
        Ok(Self(request))
    }
}

impl crate::api::current::support::CurrentRequest for EvaluationUsersRequest {
    fn validate_current(&self) -> Result<(), Error> {
        validate_user_batch(&self.0)
    }
}

/// Definitive result for one evaluation-account input, in request order.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum EvaluationAccountCreation {
    /// The provider created both the account and its trading permission.
    Created {
        /// Newly assigned account identity.
        account_id: AccountId,
        /// Newly assigned trading-permission identity.
        trading_permission_id: TradingPermissionId,
    },
    /// The provider definitively rejected this input without creating IDs.
    Rejected,
}

/// Definitive per-input outcomes from one evaluation-account batch.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvaluationAccountBatchResult {
    items: Vec<EvaluationAccountCreation>,
}

impl EvaluationAccountBatchResult {
    /// Returns outcomes in the same position as their request inputs.
    #[must_use]
    pub fn items(&self) -> &[EvaluationAccountCreation] {
        &self.items
    }
}

/// Definitive result for one evaluation-user input, in request order.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum EvaluationUserCreation {
    /// The provider created the user with this identity.
    Created {
        /// Newly assigned user identity.
        user_id: UserId,
    },
    /// The provider definitively rejected this input without creating an ID.
    Rejected,
}

/// Definitive per-input outcomes from one evaluation-user batch.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvaluationUserBatchResult {
    items: Vec<EvaluationUserCreation>,
}

impl EvaluationUserBatchResult {
    /// Returns outcomes in the same position as their request inputs.
    #[must_use]
    pub fn items(&self) -> &[EvaluationUserCreation] {
        &self.items
    }
}

impl Client {
    /// Creates between one and 100 evaluation accounts in one Partner batch.
    ///
    /// Mixed item outcomes are returned explicitly and in request order. A
    /// response with the wrong cardinality, a partial ID pair, or contradictory
    /// success and error evidence is fenced as ambiguous.
    ///
    /// # Errors
    ///
    /// Returns local validation, authentication, rate, transport, provider,
    /// decoding, or ambiguous-mutation errors.
    pub async fn user_create_evaluation_accounts(
        &self,
        request: &EvaluationAccountsRequest,
    ) -> Result<EvaluationAccountBatchResult, Error> {
        let response = self
            .post_reviewed_mutation_response(
                "/user/createevaluationaccounts",
                request,
                &assess_account_batch,
            )
            .await?;
        let Some(result) = account_batch_result(response.value(), request) else {
            return Err(Error::AmbiguousMutation {
                endpoint: "/user/createevaluationaccounts",
            });
        };
        drop(response.resolve());
        Ok(result)
    }

    /// Creates between one and 100 organization users in one Partner batch.
    ///
    /// Mixed item outcomes are returned explicitly and in request order. A
    /// response with the wrong cardinality or contradictory per-item evidence
    /// is fenced as ambiguous.
    ///
    /// # Errors
    ///
    /// Returns local validation, authentication, rate, transport, provider,
    /// decoding, or ambiguous-mutation errors.
    pub async fn user_create_evaluation_users(
        &self,
        request: &EvaluationUsersRequest,
    ) -> Result<EvaluationUserBatchResult, Error> {
        let response = self
            .post_reviewed_mutation_response(
                "/user/createevaluationusers",
                request,
                &assess_user_batch,
            )
            .await?;
        let Some(result) = user_batch_result(response.value(), request) else {
            return Err(Error::AmbiguousMutation {
                endpoint: "/user/createevaluationusers",
            });
        };
        drop(response.resolve());
        Ok(result)
    }
}

pub(super) fn validate_account_batch(request: &CreateEvaluationAccounts) -> Result<(), Error> {
    validate_batch_len(request.accounts().len(), "accounts")?;
    for account in request.accounts() {
        validate_required_text(account.name(), "accounts.name")?;
    }
    Ok(())
}

pub(super) fn validate_user_batch(request: &CreateEvaluationUsers) -> Result<(), Error> {
    validate_batch_len(request.users().len(), "users")?;
    for user in request.users() {
        validate_required_text(user.name_secret().expose(), "users.name")?;
        validate_required_text(user.email(), "users.email")?;
        validate_required_text(user.first_name(), "users.firstName")?;
        validate_required_text(user.last_name(), "users.lastName")?;
        validate_combined_names(user.first_name(), user.last_name(), "users.name")?;
        if let Some(entitlements) = user.entitlement_ids() {
            let unique = entitlements.iter().collect::<BTreeSet<_>>();
            if unique.len() != entitlements.len() {
                return Err(Error::InvalidRequest {
                    field: "users.entitlementIds",
                    reason: "must not contain duplicate entitlement IDs",
                });
            }
        }
    }
    Ok(())
}

fn validate_batch_len(length: usize, field: &'static str) -> Result<(), Error> {
    if !(1..=MAX_BATCH_ITEMS).contains(&length) {
        return Err(Error::InvalidRequest {
            field,
            reason: "must contain between one and 100 items",
        });
    }
    Ok(())
}

pub(super) fn assess_account_batch(
    response: &CreateEvaluationAccountsResponse,
    request: &EvaluationAccountsRequest,
) -> MutationAssessment {
    assess_batch(
        response.error_text(),
        response.results(),
        request.0.accounts().len(),
        account_item,
        |item| item.account_id().is_some() || item.trading_permission_id().is_some(),
    )
}

pub(super) fn assess_user_batch(
    response: &CreateEvaluationUsersResponse,
    request: &EvaluationUsersRequest,
) -> MutationAssessment {
    assess_batch(
        response.error_text(),
        response.results(),
        request.0.users().len(),
        user_item,
        |item| item.user_id().is_some(),
    )
}

fn assess_batch<T, O>(
    error: Option<&str>,
    results: &[T],
    expected: usize,
    classify: impl Fn(&T) -> Option<O>,
    has_success_evidence: impl Fn(&T) -> bool,
) -> MutationAssessment {
    let any_success_evidence = results.iter().any(has_success_evidence);
    if has_error(error) {
        return if any_success_evidence {
            MutationAssessment::ambiguous(true)
        } else {
            MutationAssessment::rejected()
        };
    }
    if results.len() == expected && results.iter().all(|item| classify(item).is_some()) {
        MutationAssessment::success()
    } else {
        MutationAssessment::ambiguous(any_success_evidence)
    }
}

fn account_batch_result(
    response: &CreateEvaluationAccountsResponse,
    request: &EvaluationAccountsRequest,
) -> Option<EvaluationAccountBatchResult> {
    if has_error(response.error_text()) || response.results().len() != request.0.accounts().len() {
        return None;
    }
    response
        .results()
        .iter()
        .map(account_item)
        .collect::<Option<Vec<_>>>()
        .map(|items| EvaluationAccountBatchResult { items })
}

fn user_batch_result(
    response: &CreateEvaluationUsersResponse,
    request: &EvaluationUsersRequest,
) -> Option<EvaluationUserBatchResult> {
    if has_error(response.error_text()) || response.results().len() != request.0.users().len() {
        return None;
    }
    response
        .results()
        .iter()
        .map(user_item)
        .collect::<Option<Vec<_>>>()
        .map(|items| EvaluationUserBatchResult { items })
}

fn account_item(item: &CreateEvaluationAccountResponse) -> Option<EvaluationAccountCreation> {
    match (
        has_error(item.error_text()),
        item.account_id().copied(),
        item.trading_permission_id().copied(),
    ) {
        (false, Some(account_id), Some(trading_permission_id)) => {
            Some(EvaluationAccountCreation::Created {
                account_id,
                trading_permission_id,
            })
        }
        (true, None, None) => Some(EvaluationAccountCreation::Rejected),
        _ => None,
    }
}

fn user_item(item: &CreateEvaluationUserResponse) -> Option<EvaluationUserCreation> {
    match (has_error(item.error_text()), item.user_id().copied()) {
        (false, Some(user_id)) => Some(EvaluationUserCreation::Created { user_id }),
        (true, None) => Some(EvaluationUserCreation::Rejected),
        _ => None,
    }
}

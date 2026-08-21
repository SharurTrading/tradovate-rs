// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Fail-closed mutation transmission and response handling.

use std::time::Duration;

use serde::{Deserialize, de::DeserializeOwned};

use super::{apply_429_cooldown, definitive_rejection, inspect_provider_control, read_bounded};
use crate::{
    AccountId, Client, Error,
    auth::{RenewalAttempt, TokenSnapshot},
    client::{
        DocumentedMutationResponse, MutationAssessment, MutationOutcome,
        mutation::{MutationAttempt, MutationResponse},
    },
};

#[derive(Clone, Copy)]
enum MutationAdmission {
    General,
    Account(AccountId),
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct OrderFailureControl {
    failure_reason: Option<crate::api::OrderFailureReason>,
}

fn documented_order_failure_reason(
    endpoint: &'static str,
    body: &[u8],
) -> Option<crate::api::OrderFailureReason> {
    if !matches!(
        endpoint,
        "/order/placeorder"
            | "/order/cancelorder"
            | "/order/modifyorder"
            | "/order/liquidateposition"
            | "/order/placeoco"
            | "/order/placeoso"
            | "/orderStrategy/startorderstrategy"
            | "/orderStrategy/interruptorderstrategy"
    ) {
        return None;
    }
    serde_json::from_slice::<OrderFailureControl>(body)
        .ok()
        .and_then(|control| control.failure_reason)
}

impl Client {
    pub(super) async fn execute_mutation<T>(
        &self,
        request: reqwest::RequestBuilder,
        endpoint: &'static str,
        token: &TokenSnapshot,
    ) -> Result<MutationResponse<T>, Error>
    where
        T: DeserializeOwned + DocumentedMutationResponse,
    {
        self.execute_mutation_with_assessment(request, endpoint, token, |value: &T| {
            documented_assessment(value)
        })
        .await
    }

    pub(super) async fn execute_mutation_with_assessment<T, E>(
        &self,
        request: reqwest::RequestBuilder,
        endpoint: &'static str,
        token: &TokenSnapshot,
        assess: E,
    ) -> Result<MutationResponse<T>, Error>
    where
        T: DeserializeOwned,
        E: Fn(&T) -> MutationAssessment,
    {
        self.execute_mutation_inner(
            request,
            endpoint,
            token,
            None,
            MutationAdmission::General,
            assess,
        )
        .await
    }

    pub(super) async fn execute_account_scoped_mutation<T>(
        &self,
        request: reqwest::RequestBuilder,
        endpoint: &'static str,
        token: &TokenSnapshot,
        account_id: AccountId,
    ) -> Result<MutationResponse<T>, Error>
    where
        T: DeserializeOwned + DocumentedMutationResponse,
    {
        self.execute_mutation_inner(
            request,
            endpoint,
            token,
            None,
            MutationAdmission::Account(account_id),
            |value: &T| documented_assessment(value),
        )
        .await
    }

    pub(super) async fn execute_rotating_mutation<T>(
        &self,
        request: reqwest::RequestBuilder,
        endpoint: &'static str,
        token: &TokenSnapshot,
        rotation: &mut RenewalAttempt,
    ) -> Result<MutationResponse<T>, Error>
    where
        T: DeserializeOwned + DocumentedMutationResponse,
    {
        self.execute_mutation_inner(
            request,
            endpoint,
            token,
            Some(rotation),
            MutationAdmission::General,
            |value: &T| documented_assessment(value),
        )
        .await
    }

    async fn execute_mutation_inner<T, E>(
        &self,
        request: reqwest::RequestBuilder,
        endpoint: &'static str,
        token: &TokenSnapshot,
        rotation: Option<&mut RenewalAttempt>,
        admission: MutationAdmission,
        assess: E,
    ) -> Result<MutationResponse<T>, Error>
    where
        T: DeserializeOwned,
        E: Fn(&T) -> MutationAssessment,
    {
        if !self.tokens.is_current(token) {
            return Err(Error::Unauthenticated);
        }
        let mut attempt = self.mutation_gate.attempt(endpoint)?;
        let rate_admission = match admission {
            MutationAdmission::General => self.rate_limits.admit_immediate(endpoint)?,
            MutationAdmission::Account(account_id) => self
                .rate_limits
                .admit_immediate_for_account(endpoint, account_id)?,
        };
        if let Err(error) = arm_transmission(&mut attempt, rotation) {
            attempt.resolve();
            rate_admission.release_unsent();
            return Err(error);
        }
        let response = match request.send().await {
            Ok(response) => response,
            Err(source) if source.is_connect() || source.is_builder() => {
                attempt.resolve();
                rate_admission.release_unsent();
                return Err(Error::Transport { source });
            }
            Err(_) => return Err(Error::AmbiguousMutation { endpoint }),
        };
        let status = response.status();
        let retry_after = retry_after_header(&response);

        if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
            let retry_after = apply_429_cooldown(&self.rate_limits, retry_after);
            attempt.resolve();
            return Err(Error::ProviderRateLimit {
                endpoint,
                retry_after,
            });
        }
        if status == reqwest::StatusCode::UNAUTHORIZED {
            self.tokens.invalidate_if_current(token);
            attempt.resolve();
            return Err(Error::HttpStatus {
                endpoint,
                status: status.as_u16(),
            });
        }
        if !status.is_success() {
            if definitive_rejection(status) {
                attempt.resolve();
                return Err(Error::HttpStatus {
                    endpoint,
                    status: status.as_u16(),
                });
            }
            return Err(Error::AmbiguousMutation { endpoint });
        }

        let Ok(bytes) = read_bounded(response, endpoint, self.max_response_bytes).await else {
            return Err(Error::AmbiguousMutation { endpoint });
        };
        if let Err(error) =
            inspect_provider_control(&bytes, self.instance_id, endpoint, &self.rate_limits, false)
        {
            let has_success_evidence = serde_json::from_slice::<T>(&bytes)
                .is_ok_and(|value| assess(&value).has_success_evidence());
            if matches!(error, Error::Business { .. })
                && !has_success_evidence
                && let Some(reason) = documented_order_failure_reason(endpoint, &bytes)
            {
                if matches!(
                    reason,
                    crate::api::OrderFailureReason::Success
                        | crate::api::OrderFailureReason::Unknown(_)
                ) {
                    return Err(Error::AmbiguousMutation { endpoint });
                }
                attempt.resolve();
                return Err(Error::OrderRejected { endpoint, reason });
            }
            if matches!(
                error,
                Error::Business { .. }
                    | Error::Violations { .. }
                    | Error::Penalty(_)
                    | Error::ProviderPenalty { .. }
            ) && !has_success_evidence
            {
                attempt.resolve();
                return Err(error);
            }
            return Err(Error::AmbiguousMutation { endpoint });
        }
        let Ok(value) = serde_json::from_slice::<T>(&bytes) else {
            return Err(Error::AmbiguousMutation { endpoint });
        };
        let assessment = assess(&value);
        if assessment.outcome() == MutationOutcome::Success && assessment.has_success_evidence() {
            rate_admission.succeed();
        }
        Ok(MutationResponse::new(value, attempt))
    }
}

fn retry_after_header(response: &reqwest::Response) -> Option<Duration> {
    response
        .headers()
        .get(reqwest::header::RETRY_AFTER)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<u64>().ok())
        .map(Duration::from_secs)
}

fn arm_transmission(
    attempt: &mut MutationAttempt,
    rotation: Option<&mut RenewalAttempt>,
) -> Result<(), Error> {
    attempt.arm()?;
    if let Some(rotation) = rotation {
        rotation.arm()?;
    }
    Ok(())
}

fn documented_assessment<T>(value: &T) -> MutationAssessment
where
    T: DocumentedMutationResponse,
{
    match (value.mutation_outcome(), value.has_success_evidence()) {
        (MutationOutcome::Success, true) => MutationAssessment::success(),
        (MutationOutcome::Rejected, false) => MutationAssessment::rejected(),
        (_, success_evidence) => MutationAssessment::ambiguous(success_evidence),
    }
}

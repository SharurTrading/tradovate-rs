// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Safe execution hooks for generated documented REST capabilities.

use serde::{Serialize, de::DeserializeOwned};

use super::{finish_rate_admission, no_success_evidence};
use crate::{
    AccountId, Client, Error,
    api::current::support::{CurrentQuery, CurrentRequest},
    auth::{RenewalAttempt, TokenKind},
    client::{
        DocumentedMutationResponse, MutationAssessment, MutationOutcome, encode_bounded_json,
        mutation::MutationResponse,
    },
};

impl Client {
    /// Executes a current-API GET using its bounded repeated-key query encoder.
    ///
    /// Encoding finishes before rate admission so locally invalid queries do
    /// not consume provider capacity. Transport and response handling then use
    /// the same authenticated query policy as other GET operations.
    pub(crate) async fn get_current<T, Q>(
        &self,
        endpoint: &'static str,
        query: &Q,
    ) -> Result<T, Error>
    where
        T: DeserializeOwned,
        Q: CurrentQuery + ?Sized,
    {
        let pairs = query.encode_pairs()?;
        let admission = self.admit_query(endpoint).await;
        let token = match self.tokens.snapshot(crate::auth::TokenKind::Access) {
            Ok(token) => token,
            Err(error) => {
                admission.release_unsent();
                return Err(error);
            }
        };
        let request = self
            .http
            .get(self.endpoint_url(endpoint))
            .bearer_auth(token.expose())
            .query(&pairs);
        let result = self
            .execute_query(request, endpoint, Some(&token), false, no_success_evidence)
            .await;
        finish_rate_admission(admission, result)
    }

    /// Executes an authenticated POST that has query semantics.
    ///
    /// The request uses asynchronous query admission and the same token-freshness,
    /// bounded-response, provider-control, and decoding policy as authenticated
    /// GET queries.
    pub(crate) async fn post_query<T, B>(
        &self,
        endpoint: &'static str,
        body: &B,
    ) -> Result<T, Error>
    where
        T: DeserializeOwned,
        B: Serialize + CurrentRequest + ?Sized,
    {
        body.validate_current()?;
        let encoded = encode_bounded_json(body, endpoint, self.max_request_bytes)?;
        self.execute_post_query(endpoint, encoded).await
    }

    async fn execute_post_query<T>(&self, endpoint: &'static str, body: Vec<u8>) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let admission = self.admit_query(endpoint).await;
        let token = match self.tokens.snapshot(crate::auth::TokenKind::Access) {
            Ok(token) => token,
            Err(error) => {
                admission.release_unsent();
                return Err(error);
            }
        };
        let request = self
            .http
            .post(self.endpoint_url(endpoint))
            .bearer_auth(token.expose())
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(body);
        let result = self
            .execute_query(request, endpoint, Some(&token), false, no_success_evidence)
            .await;
        finish_rate_admission(admission, result)
    }

    /// Executes one documented money-moving mutation without automatic retry.
    ///
    /// Immediate rate admission and the shared reconciliation latch are owned by
    /// the underlying mutation executor. The endpoint response must then prove a
    /// complete success or definitive rejection; contradictory or incomplete
    /// evidence leaves the attempt armed and returns an ambiguous outcome.
    #[cfg(test)]
    pub(crate) async fn post_documented_mutation<T, B>(
        &self,
        endpoint: &'static str,
        body: &B,
    ) -> Result<T, Error>
    where
        T: DeserializeOwned + DocumentedMutationResponse,
        B: Serialize + ?Sized,
    {
        let response = self.post_mutation::<T, B>(endpoint, body).await?;
        resolve_documented_mutation(endpoint, response)
    }

    /// Executes an account-scoped mutation whose 2xx response cannot prove
    /// completion, atomically admitting it against all provider budgets.
    pub(crate) async fn post_account_scoped_unresolved_mutation<T, B>(
        &self,
        endpoint: &'static str,
        account_id: AccountId,
        body: &B,
    ) -> Result<T, Error>
    where
        T: DeserializeOwned + DocumentedMutationResponse,
        B: Serialize + CurrentRequest + ?Sized,
    {
        self.mutation_gate.ensure_available(endpoint)?;
        body.validate_current()?;
        let encoded = encode_bounded_json(body, endpoint, self.max_request_bytes)?;
        let token = self.tokens.snapshot(TokenKind::Access)?;
        let request = self
            .http
            .post(self.endpoint_url(endpoint))
            .bearer_auth(token.expose())
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(encoded);
        let response = self
            .execute_account_scoped_mutation::<T>(request, endpoint, &token, account_id)
            .await?;
        drop(response);
        Err(Error::AmbiguousMutation { endpoint })
    }

    /// Executes a mutation whose response cannot prove completion without
    /// inventing an endpoint-specific account rate window.
    pub(crate) async fn post_unresolved_mutation<T, B>(
        &self,
        endpoint: &'static str,
        body: &B,
    ) -> Result<T, Error>
    where
        T: DeserializeOwned + DocumentedMutationResponse,
        B: Serialize + CurrentRequest + ?Sized,
    {
        self.post_reviewed_mutation(endpoint, body, |response: &T, _| {
            MutationAssessment::ambiguous(response.has_success_evidence())
        })
        .await
    }

    /// Executes a mutation whose completion policy compares the exact request
    /// with the decoded response before resolving its RAII attempt.
    pub(crate) async fn post_reviewed_mutation<T, B, A>(
        &self,
        endpoint: &'static str,
        body: &B,
        assess: A,
    ) -> Result<T, Error>
    where
        T: DeserializeOwned,
        B: Serialize + CurrentRequest + ?Sized,
        A: Fn(&T, &B) -> MutationAssessment,
    {
        let response = self
            .post_reviewed_mutation_response(endpoint, body, &assess)
            .await?;
        let assessment = assess(response.value(), body);
        resolve_reviewed_mutation(endpoint, response, assessment)
    }

    /// Returns an armed decoded mutation response for endpoint-specific batch
    /// or reconciliation handling.
    pub(crate) async fn post_reviewed_mutation_response<T, B, A>(
        &self,
        endpoint: &'static str,
        body: &B,
        assess: &A,
    ) -> Result<MutationResponse<T>, Error>
    where
        T: DeserializeOwned,
        B: Serialize + CurrentRequest + ?Sized,
        A: Fn(&T, &B) -> MutationAssessment,
    {
        self.mutation_gate.ensure_available(endpoint)?;
        body.validate_current()?;
        let encoded = encode_bounded_json(body, endpoint, self.max_request_bytes)?;
        let token = self.tokens.snapshot(TokenKind::Access)?;
        let request = self
            .http
            .post(self.endpoint_url(endpoint))
            .bearer_auth(token.expose())
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(encoded);
        self.execute_mutation_with_assessment(request, endpoint, &token, |value| {
            assess(value, body)
        })
        .await
    }

    /// Executes a mutation that must atomically replace the bearer session.
    ///
    /// The renewal guard and mutation guard arm together at the final send
    /// boundary. Cancellation or an ambiguous response therefore invalidates
    /// the old bearer as well as latching mutation reconciliation.
    pub(crate) async fn post_session_rotation<T, B>(
        &self,
        endpoint: &'static str,
        body: &B,
        rotation: &mut RenewalAttempt,
    ) -> Result<MutationResponse<T>, Error>
    where
        T: DeserializeOwned + DocumentedMutationResponse,
        B: Serialize + ?Sized,
    {
        self.mutation_gate.ensure_available(endpoint)?;
        let encoded = encode_bounded_json(body, endpoint, self.max_request_bytes)?;
        let token = self.tokens.snapshot(TokenKind::Access)?;
        if !rotation.snapshot()?.has_same_revision(&token) {
            return Err(Error::SupersededAuthentication);
        }
        let request = self
            .http
            .post(self.endpoint_url(endpoint))
            .bearer_auth(token.expose())
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(encoded);
        self.execute_rotating_mutation(request, endpoint, &token, rotation)
            .await
    }
}

#[cfg(test)]
fn resolve_documented_mutation<T>(
    endpoint: &'static str,
    response: MutationResponse<T>,
) -> Result<T, Error>
where
    T: DocumentedMutationResponse,
{
    let outcome = response.value().mutation_outcome();
    let success_evidence = response.value().has_success_evidence();
    match (outcome, success_evidence) {
        (MutationOutcome::Success, true) => Ok(response.resolve()),
        (MutationOutcome::Rejected, false) => {
            response.resolve();
            Err(Error::Business { endpoint })
        }
        (MutationOutcome::Success, false)
        | (MutationOutcome::Rejected, true)
        | (MutationOutcome::Ambiguous, _) => Err(Error::AmbiguousMutation { endpoint }),
    }
}

fn resolve_reviewed_mutation<T>(
    endpoint: &'static str,
    response: MutationResponse<T>,
    assessment: MutationAssessment,
) -> Result<T, Error> {
    match (assessment.outcome(), assessment.has_success_evidence()) {
        (MutationOutcome::Success, true) => Ok(response.resolve()),
        (MutationOutcome::Rejected, false) => {
            response.resolve();
            Err(Error::Business { endpoint })
        }
        (MutationOutcome::Success, false)
        | (MutationOutcome::Rejected, true)
        | (MutationOutcome::Ambiguous, _) => Err(Error::AmbiguousMutation { endpoint }),
    }
}

#[cfg(test)]
#[path = "documented/tests.rs"]
mod tests;

#[cfg(test)]
#[path = "documented/rate_tests.rs"]
mod rate_tests;

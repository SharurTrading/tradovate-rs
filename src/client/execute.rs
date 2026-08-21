// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Bounded REST execution and provider-envelope classification.

mod documented;
mod mutation;

use std::time::Duration;

use futures_util::StreamExt;
use serde::{Serialize, de::DeserializeOwned};
use serde_json::value::RawValue;

use super::{
    Client, encode_bounded_json,
    mutation::{DocumentedMutationResponse, MutationResponse},
};
use crate::{
    Error, PenaltyTicket,
    auth::TokenSnapshot,
    provider_control::{self, ResponseControl},
    rate_limit::RateAdmission,
};

/// A response type can identify wire evidence that contradicts a provider
/// control envelope before normal payload decoding.
pub(crate) trait ControlWireResponse {
    fn has_success_evidence(body: &[u8]) -> bool;
}

impl Client {
    pub(crate) async fn get<T, Q>(&self, endpoint: &'static str, query: &Q) -> Result<T, Error>
    where
        T: DeserializeOwned,
        Q: Serialize + ?Sized,
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
            .get(self.endpoint_url(endpoint))
            .bearer_auth(token.expose())
            .query(query);
        let result = self
            .execute_query(request, endpoint, Some(&token), false, no_success_evidence)
            .await;
        finish_rate_admission(admission, result)
    }

    pub(crate) async fn get_without_query<T>(&self, endpoint: &'static str) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        self.get(endpoint, &()).await
    }

    pub(crate) async fn post_mutation<T, B>(
        &self,
        endpoint: &'static str,
        body: &B,
    ) -> Result<MutationResponse<T>, Error>
    where
        T: DeserializeOwned + DocumentedMutationResponse,
        B: Serialize + ?Sized,
    {
        self.mutation_gate.ensure_available(endpoint)?;
        let encoded = encode_bounded_json(body, endpoint, self.max_request_bytes)?;
        let token = self.tokens.snapshot(crate::auth::TokenKind::Access)?;
        let request = self
            .http
            .post(self.endpoint_url(endpoint))
            .bearer_auth(token.expose())
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(encoded);
        self.execute_mutation(request, endpoint, &token).await
    }

    pub(crate) async fn execute_public<T>(
        &self,
        request: reqwest::RequestBuilder,
        endpoint: &'static str,
    ) -> Result<T, Error>
    where
        T: DeserializeOwned + ControlWireResponse,
    {
        self.execute_query(request, endpoint, None, true, T::has_success_evidence)
            .await
    }

    /// Executes a public query whose penalty ticket cannot be safely retried.
    pub(crate) async fn execute_public_without_penalty_retry<T>(
        &self,
        request: reqwest::RequestBuilder,
        endpoint: &'static str,
    ) -> Result<T, Error>
    where
        T: DeserializeOwned + ControlWireResponse,
    {
        self.execute_query(request, endpoint, None, false, T::has_success_evidence)
            .await
    }

    pub(crate) async fn execute_authenticated_query<T>(
        &self,
        request: reqwest::RequestBuilder,
        endpoint: &'static str,
        token: &TokenSnapshot,
    ) -> Result<T, Error>
    where
        T: DeserializeOwned + ControlWireResponse,
    {
        self.execute_query(
            request,
            endpoint,
            Some(token),
            false,
            T::has_success_evidence,
        )
        .await
    }

    async fn execute_query<T>(
        &self,
        request: reqwest::RequestBuilder,
        endpoint: &'static str,
        token: Option<&TokenSnapshot>,
        retryable_penalty: bool,
        success_evidence: fn(&[u8]) -> bool,
    ) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        if token.is_some_and(|token| !self.tokens.is_current(token)) {
            return Err(Error::Unauthenticated);
        }
        let response = request
            .send()
            .await
            .map_err(|source| Error::Transport { source })?;
        let status = response.status();
        let retry_after = response
            .headers()
            .get(reqwest::header::RETRY_AFTER)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.parse::<u64>().ok())
            .map(Duration::from_secs);
        if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
            let retry_after = apply_429_cooldown(&self.rate_limits, retry_after);
            return Err(Error::ProviderRateLimit {
                endpoint,
                retry_after,
            });
        }
        if status == reqwest::StatusCode::UNAUTHORIZED {
            if let Some(token) = token {
                self.tokens.invalidate_if_current(token);
            }
            return Err(Error::HttpStatus {
                endpoint,
                status: status.as_u16(),
            });
        }
        if !status.is_success() {
            return Err(Error::HttpStatus {
                endpoint,
                status: status.as_u16(),
            });
        }
        let bytes = read_bounded(response, endpoint, self.max_response_bytes).await?;
        if let Err(error) = inspect_provider_control(
            &bytes,
            self.instance_id,
            endpoint,
            &self.rate_limits,
            retryable_penalty,
        ) {
            if success_evidence(&bytes) {
                return Err(Error::InvalidProviderControl { endpoint });
            }
            return Err(error);
        }
        serde_json::from_slice(&bytes).map_err(|source| Error::Decode { endpoint, source })
    }

    pub(crate) fn endpoint_url(&self, endpoint: &'static str) -> String {
        format!(
            "{}{}",
            self.endpoints.rest().as_str().trim_end_matches('/'),
            endpoint
        )
    }

    pub(crate) async fn admit_query(&self, endpoint: &'static str) -> RateAdmission<'_> {
        self.rate_limits.begin_authenticated(endpoint).await
    }
}

pub(super) fn finish_rate_admission<T>(
    admission: RateAdmission<'_>,
    result: Result<T, Error>,
) -> Result<T, Error> {
    match result {
        Ok(value) => {
            admission.succeed();
            Ok(value)
        }
        Err(Error::Unauthenticated) => {
            admission.release_unsent();
            Err(Error::Unauthenticated)
        }
        Err(Error::Transport { source }) if source.is_connect() || source.is_builder() => {
            admission.release_unsent();
            Err(Error::Transport { source })
        }
        Err(error) => Err(error),
    }
}

fn no_success_evidence(_body: &[u8]) -> bool {
    false
}

fn apply_429_cooldown(
    rate_limits: &crate::rate_limit::RateGovernor,
    retry_after: Option<Duration>,
) -> Duration {
    let retry_after = retry_after.unwrap_or_default().max(Duration::from_hours(1));
    rate_limits.apply_global_cooldown(retry_after);
    retry_after
}

fn definitive_rejection(status: reqwest::StatusCode) -> bool {
    matches!(
        status,
        reqwest::StatusCode::BAD_REQUEST
            | reqwest::StatusCode::FORBIDDEN
            | reqwest::StatusCode::NOT_FOUND
            | reqwest::StatusCode::METHOD_NOT_ALLOWED
            | reqwest::StatusCode::CONFLICT
            | reqwest::StatusCode::PAYLOAD_TOO_LARGE
            | reqwest::StatusCode::UNSUPPORTED_MEDIA_TYPE
            | reqwest::StatusCode::UNPROCESSABLE_ENTITY
            | reqwest::StatusCode::LOCKED
    )
}

async fn read_bounded(
    response: reqwest::Response,
    endpoint: &'static str,
    limit: usize,
) -> Result<Vec<u8>, Error> {
    if response
        .content_length()
        .is_some_and(|length| usize::try_from(length).map_or(true, |length| length > limit))
    {
        return Err(Error::ResponseTooLarge { endpoint, limit });
    }
    let mut body = Vec::new();
    let mut stream = response.bytes_stream();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|source| Error::Transport { source })?;
        let next_len = body
            .len()
            .checked_add(chunk.len())
            .ok_or(Error::ResponseTooLarge { endpoint, limit })?;
        if next_len > limit {
            return Err(Error::ResponseTooLarge { endpoint, limit });
        }
        body.extend_from_slice(&chunk);
    }
    Ok(body)
}

fn inspect_provider_control(
    body: &[u8],
    client_instance_id: u64,
    endpoint: &'static str,
    rate_limits: &crate::rate_limit::RateGovernor,
    retryable_penalty: bool,
) -> Result<(), Error> {
    let Ok(raw) = serde_json::from_slice::<Box<RawValue>>(body) else {
        return Ok(());
    };
    match provider_control::inspect(Some(raw.as_ref())) {
        Ok(ResponseControl::Payload) => Ok(()),
        Ok(ResponseControl::BusinessFailure {
            violation_count: Some(count),
        }) if count != 0 => Err(Error::Violations { endpoint, count }),
        Ok(ResponseControl::BusinessFailure { .. }) => Err(Error::Business { endpoint }),
        Ok(ResponseControl::Penalty(penalty)) => {
            let (ticket, retry_after, captcha_required) = penalty.into_parts();
            if retryable_penalty {
                let not_before = std::time::Instant::now().checked_add(retry_after);
                if captcha_required {
                    rate_limits.apply_captcha_lockout(endpoint, retry_after);
                } else {
                    rate_limits.apply_endpoint_cooldown_until(endpoint, not_before);
                }
                Err(Error::Penalty(PenaltyTicket::new(
                    ticket,
                    retry_after,
                    captcha_required,
                    client_instance_id,
                    endpoint,
                    not_before,
                )))
            } else {
                if captcha_required {
                    rate_limits.apply_captcha_lockout(endpoint, retry_after);
                } else {
                    rate_limits.apply_endpoint_cooldown(endpoint, retry_after);
                }
                drop(ticket);
                Err(Error::ProviderPenalty {
                    endpoint,
                    retry_after,
                    captcha_required,
                })
            }
        }
        Err(_) => Err(Error::InvalidProviderControl { endpoint }),
    }
}

#[cfg(test)]
#[path = "execute/tests.rs"]
mod tests;

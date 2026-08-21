// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Direct API-key authentication and token renewal.

#[path = "authentication/credential_changes.rs"]
mod credential_changes;
#[path = "authentication/validation.rs"]
mod validation;

use super::{Client, encode_bounded_json, execute::finish_rate_admission};
use crate::{
    Error, PenaltyTicket,
    api::current::authentication::{OAuthMeResponse, OAuthToken, OAuthTokenResponse},
    auth::{Credentials, InstalledSession, RenewalAttempt, SessionInfo, wire::AccessTokenResponse},
};
use secrecy::zeroize::Zeroizing;
use validation::{
    encode_request, into_session, renewal_is_definitive, validate_oauth_control,
    validate_oauth_grant,
};

const ACCESS_TOKEN_ENDPOINT: &str = "/auth/accesstokenrequest";
const RENEW_TOKEN_ENDPOINT: &str = "/auth/renewaccesstoken";
const OAUTH_TOKEN_ENDPOINT: &str = "/auth/oauthtoken";
const AUTH_ME_ENDPOINT: &str = "/auth/me";

impl Client {
    /// Authenticates using direct API-key credentials.
    ///
    /// Starting an attempt invalidates the prior local session. A delayed older
    /// response cannot replace a session installed by a newer attempt.
    ///
    /// # Errors
    ///
    /// Returns a typed transport, provider, decoding, penalty, or revision
    /// error. No credential or bearer-token value appears in the error.
    pub async fn authenticate(&self, credentials: &Credentials) -> Result<SessionInfo, Error> {
        self.authenticate_inner(credentials, None).await
    }

    /// Exchanges a current OAuth grant and installs its access token.
    ///
    /// The response token remains inside the single revision-fenced session
    /// store. Refresh and identity tokens are neither exposed nor persisted.
    /// The method verifies the new bearer with `/auth/me` before publishing a
    /// session and derives expiration from the documented `expires_in` value.
    ///
    /// # Errors
    ///
    /// Returns a typed input, transport, rate, provider-control, decoding, or
    /// authentication-response failure. The prior local session is invalidated
    /// when the OAuth attempt begins.
    pub async fn authenticate_oauth(&self, grant: &OAuthToken) -> Result<SessionInfo, Error> {
        validate_oauth_grant(grant)?;
        let body = encode_bounded_json(grant, OAUTH_TOKEN_ENDPOINT, self.max_request_bytes)?;
        let admission = self
            .rate_limits
            .begin_anonymous_failed_only(OAUTH_TOKEN_ENDPOINT)
            .await;
        let attempt = self.tokens.begin_authentication();
        let request = self
            .http
            .post(self.endpoint_url(OAUTH_TOKEN_ENDPOINT))
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(body);
        let response: OAuthTokenResponse = match self
            .execute_public_without_penalty_retry(request, OAUTH_TOKEN_ENDPOINT)
            .await
        {
            Ok(response) => response,
            Err(Error::Transport { source }) if source.is_connect() => {
                admission.release_unsent();
                return Err(Error::Transport { source });
            }
            Err(error) => return Err(error),
        };
        validate_oauth_control(&response)?;
        let token = response
            .access_token_secret()
            .ok_or(Error::InvalidAuthenticationResponse {
                reason: "OAuth response omitted access_token",
            })?;
        let expires_in = response
            .expires_in()
            .copied()
            .filter(|seconds| *seconds > 0)
            .and_then(|seconds| u64::try_from(seconds).ok())
            .ok_or(Error::InvalidAuthenticationResponse {
                reason: "OAuth response has an invalid expires_in",
            })?;
        if response
            .token_type()
            .is_some_and(|kind| !kind.eq_ignore_ascii_case("bearer"))
        {
            return Err(Error::InvalidAuthenticationResponse {
                reason: "OAuth response token_type is not Bearer",
            });
        }
        admission.succeed();

        let admission = self.rate_limits.begin_authenticated(AUTH_ME_ENDPOINT).await;
        let me_request = self
            .http
            .get(self.endpoint_url(AUTH_ME_ENDPOINT))
            .bearer_auth(token.expose());
        let result = self
            .execute_public_without_penalty_retry(me_request, AUTH_ME_ENDPOINT)
            .await;
        let me: OAuthMeResponse = finish_rate_admission(admission, result)?;
        let user_id = me
            .user_id()
            .copied()
            .ok_or(Error::InvalidAuthenticationResponse {
                reason: "OAuth identity response omitted userId",
            })?;
        let expires_at = jiff::Timestamp::now()
            .checked_add(std::time::Duration::from_secs(expires_in))
            .map_err(|_| Error::InvalidAuthenticationResponse {
                reason: "OAuth expiration exceeds the supported timestamp range",
            })?;
        let info = SessionInfo::new(user_id, expires_at, false);
        let session = InstalledSession::try_new(token.expose().to_owned(), None, info)?;
        attempt.commit(session)
    }

    /// Retries authentication with a provider-issued penalty ticket.
    ///
    /// The client enforces [`PenaltyTicket::wait`] against a monotonic
    /// deadline. Captcha challenges are rejected locally and require operator
    /// resolution. Tickets are bound to this client instance, this endpoint,
    /// the exact original request bytes, and one claim.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidRequest`] for captcha tickets, or the same typed
    /// failures as [`Client::authenticate`].
    pub async fn authenticate_with_penalty(
        &self,
        credentials: &Credentials,
        penalty: &PenaltyTicket,
    ) -> Result<SessionInfo, Error> {
        if penalty.captcha_required() {
            return Err(Error::InvalidRequest {
                field: "penalty_ticket",
                reason: "captcha-required tickets need operator resolution",
            });
        }
        self.authenticate_inner(credentials, Some(penalty)).await
    }

    async fn authenticate_inner(
        &self,
        credentials: &Credentials,
        penalty: Option<&PenaltyTicket>,
    ) -> Result<SessionInfo, Error> {
        let base_request =
            Zeroizing::new(encode_request(credentials, None, self.max_request_bytes)?);
        let admission = self
            .rate_limits
            .begin_anonymous_failed_only(ACCESS_TOKEN_ENDPOINT)
            .await;
        let attempt = self.tokens.begin_authentication();
        let mut penalty_claim = if let Some(penalty) = penalty {
            let Some(claim) = penalty.begin_claim_for_request(
                self.instance_id,
                ACCESS_TOKEN_ENDPOINT,
                &base_request,
            ) else {
                admission.release_unsent();
                return Err(Error::InvalidRequest {
                    field: "penalty_ticket",
                    reason: "ticket is early, mismatched, expired, or already used",
                });
            };
            Some(claim)
        } else {
            None
        };
        let body = match encode_request(
            credentials,
            penalty_claim
                .as_ref()
                .map(crate::error::PenaltyClaim::expose_ticket),
            self.max_request_bytes,
        ) {
            Ok(body) => body,
            Err(error) => {
                admission.release_unsent();
                return Err(error);
            }
        };
        let request = self
            .http
            .post(self.endpoint_url(ACCESS_TOKEN_ENDPOINT))
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(body);
        if let Some(claim) = &mut penalty_claim {
            claim.arm();
        }
        let response: AccessTokenResponse =
            match self.execute_public(request, ACCESS_TOKEN_ENDPOINT).await {
                Ok(response) => response,
                Err(Error::Transport { source }) if source.is_connect() => {
                    if let Some(claim) = penalty_claim.take() {
                        claim.release_unsent();
                    }
                    admission.release_unsent();
                    return Err(Error::Transport { source });
                }
                Err(Error::Penalty(mut ticket)) => {
                    ticket.bind_request(base_request.to_vec());
                    return Err(Error::Penalty(ticket));
                }
                Err(error) => return Err(error),
            };
        let session = into_session(response)?;
        admission.succeed();
        attempt.commit(session)
    }

    /// Renews the currently installed access token without creating a session.
    ///
    /// The renewal commits only if the token revision used by the request is
    /// still current.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Unauthenticated`] when no session exists, or a typed
    /// transport/provider/decoding error. A concurrent renewal returns
    /// [`Error::RenewalInProgress`]. Session replacement before transmission
    /// returns [`Error::Unauthenticated`]; replacement after the response but
    /// before commit returns [`Error::SupersededAuthentication`].
    pub async fn renew_access_token(&self) -> Result<SessionInfo, Error> {
        let mut attempt: RenewalAttempt = self.tokens.begin_renewal()?;
        let response_result = {
            let admission = self.admit_query(RENEW_TOKEN_ENDPOINT).await;
            let token = match attempt.snapshot() {
                Ok(token) => token,
                Err(error) => {
                    admission.release_unsent();
                    return Err(error);
                }
            };
            let request = self
                .http
                .get(self.endpoint_url(RENEW_TOKEN_ENDPOINT))
                .bearer_auth(token.expose());
            if let Err(error) = attempt.arm() {
                admission.release_unsent();
                return Err(error);
            }
            let token = match attempt.snapshot() {
                Ok(token) => token,
                Err(error) => {
                    admission.release_unsent();
                    return Err(error);
                }
            };
            let result = self
                .execute_authenticated_query(request, RENEW_TOKEN_ENDPOINT, token)
                .await;
            finish_rate_admission(admission, result)
        };
        let response: AccessTokenResponse = match response_result {
            Ok(response) => response,
            Err(error) if renewal_is_definitive(&error) => {
                attempt.retain();
                return Err(error);
            }
            Err(error) => return Err(error),
        };
        let session = into_session(response)?;
        if session.info().user_id() != attempt.user_id()? {
            return Err(Error::InvalidAuthenticationResponse {
                reason: "renewal response userId does not match the installed session",
            });
        }
        let info = session.info().clone();
        if attempt.commit(session) {
            Ok(info)
        } else {
            Err(Error::SupersededAuthentication)
        }
    }

    /// Returns non-secret metadata for the installed session.
    #[must_use]
    pub fn session_info(&self) -> Option<SessionInfo> {
        self.tokens.session_info()
    }
}

#[cfg(test)]
#[path = "authentication/tests.rs"]
mod tests;

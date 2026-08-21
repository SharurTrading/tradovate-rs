// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Direct API-key authentication and token renewal.

use super::Client;
use crate::{
    Error, PenaltyTicket,
    auth::{
        Credentials, InstalledSession, RenewalAttempt, SessionInfo,
        wire::{AccessTokenRequest, AccessTokenResponse},
    },
};
use secrecy::zeroize::Zeroizing;

const ACCESS_TOKEN_ENDPOINT: &str = "/auth/accesstokenrequest";
const RENEW_TOKEN_ENDPOINT: &str = "/auth/renewaccesstoken";

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
        let admission = self
            .rate_limits
            .begin_anonymous_failed_only(ACCESS_TOKEN_ENDPOINT)
            .await;
        let attempt = self.tokens.begin_authentication();
        let base_request = Zeroizing::new(match encode_request(credentials, None) {
            Ok(body) => body,
            Err(error) => {
                admission.release_unsent();
                return Err(error);
            }
        });
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
                Ok(response) => {
                    admission.succeed();
                    response
                }
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
        attempt.commit(into_session(response)?)
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
        self.admit_query(RENEW_TOKEN_ENDPOINT).await?;
        let response_result = {
            let token = attempt.snapshot()?;
            let request = self
                .http
                .get(self.endpoint_url(RENEW_TOKEN_ENDPOINT))
                .bearer_auth(token.expose());
            attempt.arm()?;
            let token = attempt.snapshot()?;
            self.execute_authenticated_query(request, RENEW_TOKEN_ENDPOINT, token)
                .await
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

fn renewal_is_definitive(error: &Error) -> bool {
    match error {
        Error::Business { .. }
        | Error::Violations { .. }
        | Error::Penalty(_)
        | Error::ProviderPenalty { .. }
        | Error::LocalRateLimit { .. }
        | Error::ProviderRateLimit { .. }
        | Error::Configuration(_)
        | Error::InvalidRequest { .. }
        | Error::RenewalInProgress
        | Error::MutationReconciliationRequired { .. }
        | Error::Unauthenticated
        | Error::Encode { .. }
        | Error::SupersededAuthentication => true,
        Error::HttpStatus { status, .. } => *status >= 400 && *status < 500 && *status != 401,
        Error::Transport { source } if source.is_connect() => true,
        Error::Transport { .. }
        | Error::ResponseTooLarge { .. }
        | Error::Decode { .. }
        | Error::InvalidProviderControl { .. }
        | Error::InvalidAuthenticationResponse { .. }
        | Error::AmbiguousMutation { .. } => false,
    }
}

fn encode_request(
    credentials: &Credentials,
    penalty_ticket: Option<&str>,
) -> Result<Vec<u8>, Error> {
    serde_json::to_vec(&AccessTokenRequest::new(credentials, penalty_ticket)).map_err(|source| {
        Error::Encode {
            endpoint: ACCESS_TOKEN_ENDPOINT,
            source,
        }
    })
}

fn into_session(response: AccessTokenResponse) -> Result<InstalledSession, Error> {
    let has_market_data = response.has_market_data || response.md_access_token.is_some();
    let info = SessionInfo::new(response.user_id, response.expiration_time, has_market_data);
    InstalledSession::try_new(response.access_token, response.md_access_token, info)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn connection_failure_is_a_definitive_pre_send_renewal_failure() {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await;
        let Ok(listener) = listener else {
            panic!("fixture listener must bind");
        };
        let Ok(address) = listener.local_addr() else {
            panic!("fixture listener must have an address");
        };
        drop(listener);

        let request = reqwest::Client::new().get(format!("http://{address}/v1/renew"));
        let Err(source) = request.send().await else {
            panic!("fixture endpoint must refuse the connection");
        };
        assert!(source.is_connect());
        assert!(renewal_is_definitive(&Error::Transport { source }));
    }
}

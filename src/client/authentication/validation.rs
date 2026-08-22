// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Authentication request and provider-response validation.

use crate::{
    Error,
    api::current::authentication::{OAuthMeResponse, OAuthToken, OAuthTokenResponse},
    auth::{
        Credentials, InstalledSession, SessionInfo,
        wire::{AccessTokenRequest, AccessTokenResponse},
    },
};

use super::{ACCESS_TOKEN_ENDPOINT, OAUTH_TOKEN_ENDPOINT};

pub(super) fn validate_oauth_control(response: &OAuthTokenResponse) -> Result<(), Error> {
    let malformed_error = response
        .error()
        .is_some_and(|value| value.is_empty() || value.trim() != value)
        || response
            .error_description()
            .is_some_and(|value| value.is_empty() || value.trim() != value);
    if malformed_error {
        return Err(Error::InvalidProviderControl {
            endpoint: OAUTH_TOKEN_ENDPOINT,
        });
    }
    let error = response.error().is_some() || response.error_description().is_some();
    let token = response.access_token_secret().is_some();
    match (error, token) {
        (true, true) => Err(Error::InvalidProviderControl {
            endpoint: OAUTH_TOKEN_ENDPOINT,
        }),
        (true, false) => Err(Error::Business {
            endpoint: OAUTH_TOKEN_ENDPOINT,
        }),
        (false, _) => Ok(()),
    }
}

pub(super) fn validate_oauth_grant(grant: &OAuthToken) -> Result<(), Error> {
    if grant.grant_type() != "authorization_code" {
        return Err(Error::InvalidRequest {
            field: "grant_type",
            reason: "only the documented authorization_code exchange is supported",
        });
    }
    if !grant.has_code() {
        return Err(Error::InvalidRequest {
            field: "code",
            reason: "is required for authorization_code",
        });
    }
    if grant
        .client_id()
        .is_none_or(|value| value.is_empty() || value.trim() != value)
    {
        return Err(Error::InvalidRequest {
            field: "client_id",
            reason: "is required and must not contain surrounding whitespace",
        });
    }
    if grant.has_refresh_token() || grant.has_assertion() {
        return Err(Error::InvalidRequest {
            field: "grant",
            reason: "contains fields from an unsupported OAuth grant flow",
        });
    }
    if grant.has_client_secret() && grant.has_http_auth() {
        return Err(Error::InvalidRequest {
            field: "client_authentication",
            reason: "client_secret and httpAuth are mutually exclusive",
        });
    }
    for (field, value) in [
        ("redirect_uri", grant.redirect_uri()),
        ("resource", grant.resource()),
    ] {
        if value.is_some_and(|value| value.is_empty() || value.trim() != value) {
            return Err(Error::InvalidRequest {
                field,
                reason: "must be non-empty and have no surrounding whitespace",
            });
        }
    }
    Ok(())
}

impl crate::client::ControlWireResponse for OAuthTokenResponse {
    fn has_success_evidence(body: &[u8]) -> bool {
        serde_json::from_slice::<Self>(body)
            .ok()
            .is_some_and(|response| response.access_token_secret().is_some())
    }
}

impl crate::client::ControlWireResponse for OAuthMeResponse {
    fn has_success_evidence(body: &[u8]) -> bool {
        serde_json::from_slice::<Self>(body)
            .ok()
            .is_some_and(|response| response.user_id().is_some())
    }
}

pub(super) fn renewal_is_definitive(error: &Error) -> bool {
    match error {
        Error::Business { .. }
        | Error::OrderRejected { .. }
        | Error::Violations { .. }
        | Error::Penalty(_)
        | Error::ProviderPenalty { .. }
        | Error::LocalRateLimit { .. }
        | Error::ProviderRateLimit { .. }
        | Error::Configuration(_)
        | Error::InvalidRequest { .. }
        | Error::RenewalInProgress
        | Error::MutationInProgress { .. }
        | Error::MutationReconciliationRequired { .. }
        | Error::Unauthenticated
        | Error::Encode { .. }
        | Error::RequestTooLarge { .. }
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

pub(super) fn encode_request(
    credentials: &Credentials,
    penalty_ticket: Option<&str>,
    limit: usize,
) -> Result<Vec<u8>, Error> {
    crate::client::encode_bounded_json(
        &AccessTokenRequest::new(credentials, penalty_ticket),
        ACCESS_TOKEN_ENDPOINT,
        limit,
    )
}

pub(super) fn into_session(response: AccessTokenResponse) -> Result<InstalledSession, Error> {
    // `mdAccessToken` is published in the pinned endpoint prose example even
    // though the adjacent component schema omits it. This narrow private
    // compatibility field is the only source of market-data capability.
    let has_market_data = response.md_access_token.is_some();
    let info = SessionInfo::new(response.user_id, response.expiration_time, has_market_data);
    InstalledSession::try_new(response.access_token, response.md_access_token, info)
}

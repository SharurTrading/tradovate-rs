// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Revision-fenced credential and password changes.

use serde::Serialize;

use super::renewal_is_definitive;
use crate::{
    Client, Error, UserId,
    api::current::users::{AccessTokenResponse, ModifyCredentials, ModifyPassword},
    auth::{InstalledSession, SessionInfo},
    client::{DocumentedMutationResponse, MutationOutcome},
};

const MODIFY_CREDENTIALS_ENDPOINT: &str = "/user/modifycredentials";
const MODIFY_PASSWORD_ENDPOINT: &str = "/user/modifypassword";

impl Client {
    /// Changes the current username/password pair and installs the returned bearer.
    ///
    /// The old session is preserved only for a definitive pre-application
    /// rejection. Cancellation or contradictory completion evidence invalidates
    /// it and requires fresh authentication plus provider reconciliation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, provider,
    /// transport, ambiguous-mutation, or invalid-session failure.
    pub async fn modify_credentials(
        &self,
        request: &ModifyCredentials,
    ) -> Result<SessionInfo, Error> {
        self.rotate_credentials(
            MODIFY_CREDENTIALS_ENDPOINT,
            request,
            request.user_id().copied(),
        )
        .await
    }

    /// Changes the current password and installs the returned bearer.
    ///
    /// # Errors
    ///
    /// Returns a typed authentication, rate, provider, transport,
    /// ambiguous-mutation, or invalid-session failure.
    pub async fn modify_password(&self, request: &ModifyPassword) -> Result<SessionInfo, Error> {
        self.rotate_credentials(
            MODIFY_PASSWORD_ENDPOINT,
            request,
            request.user_id().copied(),
        )
        .await
    }

    async fn rotate_credentials<B>(
        &self,
        endpoint: &'static str,
        request: &B,
        requested_user_id: Option<UserId>,
    ) -> Result<SessionInfo, Error>
    where
        B: Serialize + ?Sized,
    {
        let mut rotation = self.tokens.begin_renewal()?;
        let expected_user_id = rotation.user_id()?;
        if requested_user_id.is_some_and(|user_id| user_id != expected_user_id) {
            rotation.retain();
            return Err(Error::InvalidRequest {
                field: "userId",
                reason: "must match the authenticated session",
            });
        }
        let response = match self
            .post_session_rotation::<AccessTokenResponse, _>(endpoint, request, &mut rotation)
            .await
        {
            Ok(response) => response,
            Err(error) if renewal_is_definitive(&error) => {
                rotation.retain();
                return Err(error);
            }
            Err(error) => return Err(error),
        };
        let outcome = response.value().mutation_outcome();
        let success_evidence = response.value().has_success_evidence();
        match (outcome, success_evidence) {
            (MutationOutcome::Rejected, false) => {
                response.resolve();
                rotation.retain();
                return Err(Error::Business { endpoint });
            }
            (MutationOutcome::Success, true) => {}
            (MutationOutcome::Success, false)
            | (MutationOutcome::Rejected, true)
            | (MutationOutcome::Ambiguous, _) => {
                return Err(Error::AmbiguousMutation { endpoint });
            }
        }
        let session =
            rotated_session(response.value()).map_err(|_| Error::AmbiguousMutation { endpoint })?;
        if session.info().user_id() != expected_user_id {
            return Err(Error::AmbiguousMutation { endpoint });
        }
        let info = session.info().clone();
        if rotation.commit(session) {
            response.resolve();
            Ok(info)
        } else {
            Err(Error::SupersededAuthentication)
        }
    }
}

impl DocumentedMutationResponse for AccessTokenResponse {
    fn mutation_outcome(&self) -> MutationOutcome {
        if self.has_success_evidence() {
            MutationOutcome::Success
        } else {
            MutationOutcome::Ambiguous
        }
    }

    fn has_success_evidence(&self) -> bool {
        self.access_token_secret().is_some()
            && self.expiration_time().is_some()
            && self.user_id().is_some()
    }
}

fn rotated_session(response: &AccessTokenResponse) -> Result<InstalledSession, Error> {
    let token = response
        .access_token_secret()
        .ok_or(Error::InvalidAuthenticationResponse {
            reason: "credential-change response omitted accessToken",
        })?;
    let expires_at =
        response
            .expiration_time()
            .copied()
            .ok_or(Error::InvalidAuthenticationResponse {
                reason: "credential-change response omitted expirationTime",
            })?;
    let user_id = response
        .user_id()
        .copied()
        .ok_or(Error::InvalidAuthenticationResponse {
            reason: "credential-change response omitted userId",
        })?;
    let info = SessionInfo::new(user_id, expires_at, false);
    InstalledSession::try_new(token.expose().to_owned(), None, info)
}

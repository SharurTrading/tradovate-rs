// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Provider-verified adoption of an externally obtained access token.

use secrecy::zeroize::Zeroizing;

use super::{Client, RENEW_TOKEN_ENDPOINT, validation::into_session};
use crate::Error;
use crate::auth::{SessionInfo, is_valid_bearer_token, wire::AccessTokenResponse};
use crate::client::execute::finish_rate_admission;

impl Client {
    /// Adopts an externally obtained access token as this client's session.
    ///
    /// # Workaround status
    ///
    /// This is a **workaround, not an officially supported connection
    /// method**. Tradovate's documented authentication contracts are the
    /// API-key access-token request and the OAuth code exchange; nothing in
    /// the provider documentation promises that a web-application session
    /// token can be exchanged this way. It exists for accounts — such as
    /// prop-firm evaluation accounts — that cannot register an API key or an
    /// OAuth client and therefore have no documented programmatic access at
    /// all. The provider may change or reject this behavior at any time
    /// without notice; callers should treat any failure here as expected and
    /// migrate to a documented method when one becomes available.
    ///
    /// # Contract
    ///
    /// The caller extracts the web application's bearer token out-of-band
    /// (for example from an authenticated browser session) and hands only
    /// that token to this method; the crate never performs the browser
    /// delegation itself and accepts no other caller-supplied session
    /// metadata.
    ///
    /// Adoption is provider-verified rather than trusted: the external token
    /// is exchanged through [`Client::renew_access_token`]'s endpoint, and
    /// only the renewed session the provider returns — its access token,
    /// market-data token, user identifier, and expiration — is installed. A
    /// token the provider refuses installs nothing. Starting an adoption
    /// invalidates the prior local session, and a delayed older response
    /// cannot replace a session installed by a newer attempt, matching
    /// [`Client::authenticate`].
    ///
    /// The input is treated as a secret: it is zeroized after transmission,
    /// never logged, and never appears in errors. The installed session is
    /// renewed like any other and expires on the provider-declared schedule.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidRequest`] when the token has invalid
    /// bearer-token syntax, or a typed transport, rate, provider, decoding, or
    /// authentication-response failure from the renewal exchange. No
    /// credential or bearer-token value appears in the error.
    pub async fn adopt_session(
        &self,
        access_token: impl Into<String>,
    ) -> Result<SessionInfo, Error> {
        let token = Zeroizing::new(access_token.into());
        if !is_valid_bearer_token(token.as_str()) {
            return Err(Error::InvalidRequest {
                field: "access_token",
                reason: "must be a non-empty bearer token without whitespace",
            });
        }
        let admission = self.admit_query(RENEW_TOKEN_ENDPOINT).await;
        let attempt = self.tokens.begin_authentication();
        let request = self
            .http
            .get(self.endpoint_url(RENEW_TOKEN_ENDPOINT))
            .bearer_auth(token.as_str());
        let response: AccessTokenResponse = finish_rate_admission(
            admission,
            self.execute_public_without_penalty_retry(request, RENEW_TOKEN_ENDPOINT)
                .await,
        )?;
        let session = into_session(response)?;
        attempt.commit(session)
    }
}

#[cfg(test)]
#[path = "adoption/tests.rs"]
mod tests;

// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Revision-fenced bearer-session storage.

mod validation;

use std::{
    fmt,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

use jiff::Timestamp;
use parking_lot::Mutex;
use secrecy::{ExposeSecret, SecretString};

use crate::{Error, UserId};
use validation::is_valid_bearer_token;

/// Non-secret metadata for the installed Tradovate session.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SessionInfo {
    user_id: UserId,
    expires_at: Timestamp,
    market_data_access: bool,
}

impl SessionInfo {
    pub(crate) const fn new(
        user_id: UserId,
        expires_at: Timestamp,
        market_data_access: bool,
    ) -> Self {
        Self {
            user_id,
            expires_at,
            market_data_access,
        }
    }

    /// Returns the authenticated provider user identifier.
    #[must_use]
    pub const fn user_id(&self) -> UserId {
        self.user_id
    }

    /// Returns the server-declared token expiration instant.
    #[must_use]
    pub const fn expires_at(&self) -> Timestamp {
        self.expires_at
    }

    /// Reports whether this session advertises market-data access.
    #[must_use]
    pub const fn has_market_data_access(&self) -> bool {
        self.market_data_access
    }
}

pub(crate) struct InstalledSession {
    access_token: SecretString,
    market_data_token: Option<SecretString>,
    info: SessionInfo,
}

impl InstalledSession {
    pub(crate) fn try_new(
        access_token: String,
        market_data_token: Option<String>,
        info: SessionInfo,
    ) -> Result<Self, Error> {
        if !is_valid_bearer_token(&access_token) {
            return Err(Error::InvalidAuthenticationResponse {
                reason: "access token is empty or has invalid bearer-token syntax",
            });
        }
        if market_data_token
            .as_deref()
            .is_some_and(|token| !is_valid_bearer_token(token))
        {
            return Err(Error::InvalidAuthenticationResponse {
                reason: "market-data token is empty or has invalid bearer-token syntax",
            });
        }
        if info.expires_at <= Timestamp::now() {
            return Err(Error::InvalidAuthenticationResponse {
                reason: "token expiration is not in the future",
            });
        }
        Ok(Self {
            access_token: SecretString::from(access_token),
            market_data_token: market_data_token.map(SecretString::from),
            info,
        })
    }

    pub(crate) const fn info(&self) -> &SessionInfo {
        &self.info
    }

    fn is_expired(&self) -> bool {
        self.info.expires_at <= Timestamp::now()
    }

    fn has_valid_tokens(&self) -> bool {
        is_valid_bearer_token(self.access_token.expose_secret())
            && self
                .market_data_token
                .as_ref()
                .is_none_or(|token| is_valid_bearer_token(token.expose_secret()))
    }
}

impl fmt::Debug for InstalledSession {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("InstalledSession")
            .field("access_token", &"[REDACTED]")
            .field(
                "market_data_token",
                &self.market_data_token.as_ref().map(|_| "[REDACTED]"),
            )
            .field("info", &self.info)
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum TokenKind {
    Access,
    MarketData,
}

pub(crate) struct TokenSnapshot {
    token: SecretString,
    revision: u128,
    user_id: UserId,
}

impl TokenSnapshot {
    pub(crate) fn expose(&self) -> &str {
        self.token.expose_secret()
    }

    pub(crate) const fn has_same_revision(&self, other: &Self) -> bool {
        self.revision == other.revision
    }

    pub(crate) const fn user_id(&self) -> UserId {
        self.user_id
    }
}

impl fmt::Debug for TokenSnapshot {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TokenSnapshot")
            .field("token", &"[REDACTED]")
            .field("revision", &self.revision)
            .field("user_id", &self.user_id)
            .finish()
    }
}

#[derive(Debug, Default)]
struct State {
    revision: u128,
    session: Option<InstalledSession>,
}

#[derive(Debug, Default)]
pub(crate) struct TokenStore {
    state: Mutex<State>,
    renewal_in_flight: AtomicBool,
}

impl TokenStore {
    pub(crate) fn begin_authentication(self: &Arc<Self>) -> AuthAttempt {
        let revision = {
            let mut state = self.state.lock();
            state.revision = state.revision.wrapping_add(1);
            state.session = None;
            state.revision
        };
        AuthAttempt {
            store: Arc::clone(self),
            revision,
        }
    }

    pub(crate) fn snapshot(&self, kind: TokenKind) -> Result<TokenSnapshot, Error> {
        let mut state = self.state.lock();
        if invalidate_expired(&mut state) {
            return Err(Error::Unauthenticated);
        }
        if state
            .session
            .as_ref()
            .is_some_and(|session| !session.has_valid_tokens())
        {
            invalidate(&mut state);
            return Err(Error::InvalidAuthenticationResponse {
                reason: "installed session contains an invalid bearer token",
            });
        }
        let session = state.session.as_ref().ok_or(Error::Unauthenticated)?;
        let token = match kind {
            TokenKind::Access => &session.access_token,
            TokenKind::MarketData => session
                .market_data_token
                .as_ref()
                .unwrap_or(&session.access_token),
        };
        Ok(TokenSnapshot {
            token: SecretString::from(token.expose_secret().to_owned()),
            revision: state.revision,
            user_id: session.info.user_id,
        })
    }

    pub(crate) fn begin_renewal(self: &Arc<Self>) -> Result<RenewalAttempt, Error> {
        self.renewal_in_flight
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .map_err(|_| Error::RenewalInProgress)?;
        match self.snapshot(TokenKind::Access) {
            Ok(basis) => Ok(RenewalAttempt {
                store: Arc::clone(self),
                basis: Some(basis),
                armed: false,
            }),
            Err(error) => {
                self.renewal_in_flight.store(false, Ordering::Release);
                Err(error)
            }
        }
    }

    pub(crate) fn session_info(&self) -> Option<SessionInfo> {
        let mut state = self.state.lock();
        invalidate_expired(&mut state);
        state.session.as_ref().map(|session| session.info.clone())
    }

    pub(crate) fn is_current(&self, snapshot: &TokenSnapshot) -> bool {
        let mut state = self.state.lock();
        if invalidate_expired(&mut state) {
            return false;
        }
        if state
            .session
            .as_ref()
            .is_some_and(|session| !session.has_valid_tokens())
        {
            invalidate(&mut state);
            return false;
        }
        state.revision == snapshot.revision && state.session.is_some()
    }

    fn replace_if_current(&self, basis: &TokenSnapshot, session: InstalledSession) -> bool {
        let mut state = self.state.lock();
        if state.revision != basis.revision {
            return false;
        }
        state.revision = state.revision.wrapping_add(1);
        state.session = Some(session);
        true
    }

    pub(crate) fn invalidate_if_current(&self, basis: &TokenSnapshot) {
        let mut state = self.state.lock();
        if state.revision == basis.revision {
            invalidate(&mut state);
        }
    }
}

fn invalidate_expired(state: &mut State) -> bool {
    if state
        .session
        .as_ref()
        .is_some_and(InstalledSession::is_expired)
    {
        invalidate(state);
        true
    } else {
        false
    }
}

fn invalidate(state: &mut State) {
    state.revision = state.revision.wrapping_add(1);
    state.session = None;
}

/// Cancellation-safe ownership of one access-token renewal.
pub(crate) struct RenewalAttempt {
    store: Arc<TokenStore>,
    basis: Option<TokenSnapshot>,
    armed: bool,
}

impl RenewalAttempt {
    pub(crate) fn snapshot(&self) -> Result<&TokenSnapshot, Error> {
        self.basis.as_ref().ok_or(Error::SupersededAuthentication)
    }

    pub(crate) fn arm(&mut self) -> Result<(), Error> {
        if self.basis.is_none() {
            return Err(Error::SupersededAuthentication);
        }
        self.armed = true;
        Ok(())
    }

    pub(crate) fn user_id(&self) -> Result<UserId, Error> {
        self.snapshot().map(TokenSnapshot::user_id)
    }

    pub(crate) fn commit(mut self, session: InstalledSession) -> bool {
        let Some(basis) = self.basis.take() else {
            return false;
        };
        self.armed = false;
        self.store.replace_if_current(&basis, session)
    }

    pub(crate) fn retain(mut self) {
        self.basis = None;
        self.armed = false;
    }
}

impl fmt::Debug for RenewalAttempt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RenewalAttempt")
            .field("active", &self.basis.is_some())
            .field("armed", &self.armed)
            .finish_non_exhaustive()
    }
}

impl Drop for RenewalAttempt {
    fn drop(&mut self) {
        if self.armed
            && let Some(basis) = self.basis.take()
        {
            self.store.invalidate_if_current(&basis);
        }
        self.store.renewal_in_flight.store(false, Ordering::Release);
    }
}

pub(crate) struct AuthAttempt {
    store: Arc<TokenStore>,
    revision: u128,
}

impl AuthAttempt {
    pub(crate) fn commit(self, session: InstalledSession) -> Result<SessionInfo, Error> {
        let info = session.info.clone();
        let mut state = self.store.state.lock();
        if state.revision != self.revision {
            return Err(Error::SupersededAuthentication);
        }
        state.session = Some(session);
        Ok(info)
    }
}

impl fmt::Debug for AuthAttempt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AuthAttempt")
            .field("revision", &self.revision)
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
#[path = "token/tests.rs"]
mod tests;

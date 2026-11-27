// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Authentication credentials and session metadata.

mod credentials;
mod token;
mod wall_clock;

pub(crate) mod wire;

pub use credentials::{Credentials, CredentialsBuilder};
pub use token::SessionInfo;

pub(crate) use credentials::ApiClientId;
pub(crate) use token::{
    InstalledSession, RenewalAttempt, TokenKind, TokenSnapshot, TokenStore, is_valid_bearer_token,
};
pub(crate) use wall_clock::WallClock;

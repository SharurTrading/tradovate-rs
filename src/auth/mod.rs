// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Authentication credentials and session metadata.

mod credentials;
mod token;

pub(crate) mod wire;

pub use credentials::{Credentials, CredentialsBuilder};
pub use token::SessionInfo;

pub(crate) use credentials::ApiClientId;
pub(crate) use token::{InstalledSession, RenewalAttempt, TokenKind, TokenSnapshot, TokenStore};

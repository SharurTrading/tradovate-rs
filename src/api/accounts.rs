// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Account queries.

use jiff::Timestamp;
use serde::Deserialize;

use crate::{AccountId, AccountSpec, Client, Decimal, Error, UserId};

const LIST_ACCOUNTS: &str = "/account/list";

/// Tradovate account category.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum AccountKind {
    /// Customer account.
    Customer,
    /// Give-up account.
    Giveup,
    /// House account.
    House,
    /// Omnibus account.
    Omnibus,
    /// Wash account.
    Wash,
    /// Provider category added after this crate version.
    Unknown(String),
}

impl<'de> Deserialize<'de> for AccountKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "Customer" => Self::Customer,
            "Giveup" => Self::Giveup,
            "House" => Self::House,
            "Omnibus" => Self::Omnibus,
            "Wash" => Self::Wash,
            _ => Self::Unknown(value),
        })
    }
}

/// Provider-native account metadata.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Account {
    /// Provider account entity identifier.
    pub id: AccountId,
    /// Provider account specification used for display and compatibility.
    pub name: AccountSpec,
    /// Owning provider user identifier.
    pub user_id: UserId,
    /// Account category.
    pub account_type: AccountKind,
    /// Whether the provider marks the account active.
    pub active: bool,
    /// Last provider update time.
    pub timestamp: Timestamp,
    /// Optional evaluation size, decoded without floating-point conversion.
    #[serde(default, with = "crate::decimal::option")]
    pub evaluation_size: Option<Decimal>,
    /// Whether the provider marks the account read-only.
    #[serde(default, rename = "readonly")]
    pub read_only: bool,
}

impl Client {
    /// Lists accounts visible to the authenticated user.
    ///
    /// # Errors
    ///
    /// Returns a typed authentication, transport, provider, bound, or decoding
    /// failure.
    pub async fn list_accounts(&self) -> Result<Vec<Account>, Error> {
        self.get_without_query(LIST_ACCOUNTS).await
    }
}

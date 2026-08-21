// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Position queries.

use jiff::Timestamp;
use serde::Deserialize;

use crate::{AccountId, Client, ContractId, Decimal, Error, PositionId};

const LIST_POSITIONS: &str = "/position/list";

/// Provider-native open-position state.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Position {
    /// Provider position entity identifier.
    pub id: PositionId,
    /// Owning provider account.
    pub account_id: AccountId,
    /// Provider contract identifier.
    pub contract_id: ContractId,
    /// Last provider update time.
    pub timestamp: Timestamp,
    /// Signed integral futures position.
    #[serde(rename = "netPos")]
    pub net_position: i32,
    /// Average net price, when available.
    #[serde(default, with = "crate::decimal::option", rename = "netPrice")]
    pub net_price: Option<Decimal>,
    /// Session contracts bought.
    pub bought: u32,
    /// Exact aggregate bought value.
    #[serde(with = "crate::decimal")]
    pub bought_value: Decimal,
    /// Session contracts sold.
    pub sold: u32,
    /// Exact aggregate sold value.
    #[serde(with = "crate::decimal")]
    pub sold_value: Decimal,
    /// Position carried into the session.
    #[serde(rename = "prevPos")]
    pub previous_position: i32,
    /// Previous position price, when available.
    #[serde(default, with = "crate::decimal::option", rename = "prevPrice")]
    pub previous_price: Option<Decimal>,
}

impl Client {
    /// Lists positions visible to the authenticated user.
    ///
    /// # Errors
    ///
    /// Returns a typed authentication, transport, provider, bound, or decoding
    /// failure.
    pub async fn list_positions(&self) -> Result<Vec<Position>, Error> {
        self.get_without_query(LIST_POSITIONS).await
    }
}

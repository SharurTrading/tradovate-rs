// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Provider order response models.

use jiff::Timestamp;
use serde::Deserialize;

use super::OrderSide;
use crate::{AccountId, ContractId, OrderId};

/// Forward-compatible order status returned by Tradovate.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum OrderStatus {
    /// Cancelled.
    Canceled,
    /// Completed.
    Completed,
    /// Expired.
    Expired,
    /// Filled.
    Filled,
    /// Cancel requested.
    PendingCancel,
    /// New order pending.
    PendingNew,
    /// Replacement pending.
    PendingReplace,
    /// Rejected.
    Rejected,
    /// Suspended.
    Suspended,
    /// Working.
    Working,
    /// Provider status added after this crate version.
    Unknown(String),
}

impl<'de> Deserialize<'de> for OrderStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "Canceled" => Self::Canceled,
            "Completed" => Self::Completed,
            "Expired" => Self::Expired,
            "Filled" => Self::Filled,
            "PendingCancel" => Self::PendingCancel,
            "PendingNew" => Self::PendingNew,
            "PendingReplace" => Self::PendingReplace,
            "Rejected" => Self::Rejected,
            "Suspended" => Self::Suspended,
            "Working" => Self::Working,
            _ => Self::Unknown(value),
        })
    }
}

/// Provider-native order identity and status.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Order {
    /// Provider order identifier.
    pub id: OrderId,
    /// Owning provider account.
    pub account_id: AccountId,
    /// Provider contract identifier, when applicable.
    pub contract_id: Option<ContractId>,
    /// Creation time.
    pub timestamp: Timestamp,
    /// Buy or sell direction.
    pub action: OrderSide,
    /// Current provider status.
    #[serde(rename = "ordStatus")]
    pub order_status: OrderStatus,
    /// Whether this is an administrative order.
    pub admin: bool,
}

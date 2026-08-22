// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Typed failure reasons shared by current order mutations.

use serde::{Deserialize, Deserializer, de};

pub(super) fn deserialize_optional_non_null<'de, D, T>(
    deserializer: D,
) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    match Option::<T>::deserialize(deserializer)? {
        Some(value) => Ok(Some(value)),
        None => Err(de::Error::custom("field is optional but not nullable")),
    }
}

/// The closed set pinned in the current Partner order response schemas.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum OrderFailureReason {
    /// The account is closed.
    AccountClosed,
    /// The requested advanced trailing-stop form is unsupported.
    AdvancedTrailingStopUnsupported,
    /// Another command is already pending.
    AnotherCommandPending,
    /// Trading the requested back month is prohibited.
    BackMonthProhibited,
    /// No execution provider is configured.
    ExecutionProviderNotConfigured,
    /// The configured execution provider is unavailable.
    ExecutionProviderUnavailable,
    /// The contract is invalid for the command.
    InvalidContract,
    /// One or more prices are invalid.
    InvalidPrice,
    /// A required key-information document has not been accepted.
    KeyInformationDocumentRequired,
    /// The account or instrument is in liquidation-only mode.
    LiquidationOnly,
    /// Liquidation-only rules apply before expiration.
    LiquidationOnlyBeforeExpiration,
    /// Maximum order quantity is not configured.
    MaxOrderQtyIsNotSpecified,
    /// The maximum order quantity was reached.
    MaxOrderQtyLimitReached,
    /// Position-limit configuration is invalid.
    MaxPosLimitMisconfigured,
    /// The position limit was reached.
    MaxPosLimitReached,
    /// The total position limit was reached.
    MaxTotalPosLimitReached,
    /// The operation requires a multiple-account plan.
    MultipleAccountPlanRequired,
    /// No usable quote is available.
    NoQuote,
    /// The market lacks enough liquidity.
    NotEnoughLiquidity,
    /// Another documented execution-related rejection occurred.
    OtherExecutionRelated,
    /// The parent order was rejected.
    ParentRejected,
    /// Provider risk validation timed out.
    RiskCheckTimeout,
    /// The SSF risk disclosure must be acknowledged.
    SsfrRiskDisclosureAcknowledgmentRequired,
    /// The trading session is closed.
    SessionClosed,
    /// The provider explicitly reported success.
    Success,
    /// The command arrived too late to apply.
    TooLate,
    /// Trading is locked for the account or user.
    TradingLocked,
    /// Trailing-stop quantity modification is unsupported.
    TrailingStopNonOrderQtyModify,
    /// The caller is not authorized for the command.
    Unauthorized,
    /// The provider returned its documented generic unknown-reason code.
    UnknownReason,
    /// The requested operation is unsupported.
    Unsupported,
    /// A future provider value not present in the pinned current contract.
    Unknown(String),
}

impl OrderFailureReason {
    pub(super) const fn is_success(&self) -> bool {
        matches!(self, Self::Success)
    }

    pub(super) const fn is_known_rejection(&self) -> bool {
        !matches!(self, Self::Success | Self::Unknown(_))
    }
}

impl<'de> Deserialize<'de> for OrderFailureReason {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "AccountClosed" => Self::AccountClosed,
            "AdvancedTrailingStopUnsupported" => Self::AdvancedTrailingStopUnsupported,
            "AnotherCommandPending" => Self::AnotherCommandPending,
            "BackMonthProhibited" => Self::BackMonthProhibited,
            "ExecutionProviderNotConfigured" => Self::ExecutionProviderNotConfigured,
            "ExecutionProviderUnavailable" => Self::ExecutionProviderUnavailable,
            "InvalidContract" => Self::InvalidContract,
            "InvalidPrice" => Self::InvalidPrice,
            "KeyInformationDocumentRequired" => Self::KeyInformationDocumentRequired,
            "LiquidationOnly" => Self::LiquidationOnly,
            "LiquidationOnlyBeforeExpiration" => Self::LiquidationOnlyBeforeExpiration,
            "MaxOrderQtyIsNotSpecified" => Self::MaxOrderQtyIsNotSpecified,
            "MaxOrderQtyLimitReached" => Self::MaxOrderQtyLimitReached,
            "MaxPosLimitMisconfigured" => Self::MaxPosLimitMisconfigured,
            "MaxPosLimitReached" => Self::MaxPosLimitReached,
            "MaxTotalPosLimitReached" => Self::MaxTotalPosLimitReached,
            "MultipleAccountPlanRequired" => Self::MultipleAccountPlanRequired,
            "NoQuote" => Self::NoQuote,
            "NotEnoughLiquidity" => Self::NotEnoughLiquidity,
            "OtherExecutionRelated" => Self::OtherExecutionRelated,
            "ParentRejected" => Self::ParentRejected,
            "RiskCheckTimeout" => Self::RiskCheckTimeout,
            "SSFRiskDisclosureAcknowledgmentRequired" => {
                Self::SsfrRiskDisclosureAcknowledgmentRequired
            }
            "SessionClosed" => Self::SessionClosed,
            "Success" => Self::Success,
            "TooLate" => Self::TooLate,
            "TradingLocked" => Self::TradingLocked,
            "TrailingStopNonOrderQtyModify" => Self::TrailingStopNonOrderQtyModify,
            "Unauthorized" => Self::Unauthorized,
            "UnknownReason" => Self::UnknownReason,
            "Unsupported" => Self::Unsupported,
            _ => Self::Unknown(value),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_unknown_provider_reason_without_treating_it_as_rejection() {
        let decoded = serde_json::from_str::<OrderFailureReason>(r#""FutureFailure""#)
            .unwrap_or_else(|error| panic!("fixture must decode: {error}"));
        assert!(matches!(decoded, OrderFailureReason::Unknown(value) if value == "FutureFailure"));
    }
}

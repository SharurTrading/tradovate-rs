// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary
// @generated
// Generator: tools/generate_openapi.py
// Source: https://partner.tradovate.com/openapi.json (snapshot 2026-08-21, sha256 37caeccf4b0913460a788fcaf4c902497059b8ffe6f6355512e6c08eaacde769)

//! Validated identities used only by the current generated contract.

use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer, de};
use thiserror::Error;

/// A non-positive provider identity was decoded or constructed.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
#[error("provider identity must be positive")]
pub struct CurrentIdError;

macro_rules! current_id {
    ($name:ident, $label:literal) => {
        #[doc = concat!("A validated Tradovate ", $label, " identifier.")]
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(i64);

        impl $name {
            /// Creates an identity from a positive provider integer.
            ///
            /// # Errors
            ///
            /// Returns [`CurrentIdError`] when `value` is not positive.
            pub const fn new(value: i64) -> Result<Self, CurrentIdError> {
                if value > 0 {
                    Ok(Self(value))
                } else {
                    Err(CurrentIdError)
                }
            }

            /// Returns the provider integer.
            #[must_use]
            pub const fn get(self) -> i64 {
                self.0
            }
        }

        impl TryFrom<i64> for $name {
            type Error = CurrentIdError;
            fn try_from(value: i64) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(formatter)
            }
        }

        impl Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_i64(self.0)
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let value = i64::deserialize(deserializer)?;
                Self::new(value).map_err(de::Error::custom)
            }
        }
    };
}

current_id!(AccountRiskStatusId, "account risk status");
current_id!(AdminAlertId, "admin alert");
current_id!(AdminAlertSignalId, "admin alert signal");
current_id!(AlertId, "alert");
current_id!(AlertSignalId, "alert signal");
current_id!(AnnualReviewId, "annual review");
current_id!(ApprovedById, "approved by");
current_id!(ApprovedId, "approved");
current_id!(AutoLiqProfileId, "auto liq profile");
current_id!(CashBalanceId, "cash balance");
current_id!(CashBalanceLogId, "cash balance log");
current_id!(ClearingHouseId, "clearing house");
current_id!(ClientAppId, "client app");
current_id!(CommandReportId, "command report");
current_id!(ContactInfoId, "contact info");
current_id!(ContractGroupId, "contract group");
current_id!(ContractMarginId, "contract margin");
current_id!(CreditCardId, "credit card");
current_id!(CurrencyId, "currency");
current_id!(CurrencyRateId, "currency rate");
current_id!(CustomerApplicationId, "customer application");
current_id!(DocumentId, "document");
current_id!(EntitlementId, "entitlement");
current_id!(ExchangeId, "exchange");
current_id!(ExchangeScopeId, "exchange scope");
current_id!(ExecutionProviderId, "execution provider");
current_id!(ExecutionReportId, "execution report");
current_id!(FillFeeId, "fill fee");
current_id!(FillId, "fill");
current_id!(FillPairId, "fill pair");
current_id!(FundTransactionId, "fund transaction");
current_id!(FungibleProductId, "fungible product");
current_id!(InitiatorId, "initiator");
current_id!(IntroducingPartnerId, "introducing partner");
current_id!(KalshiEventId, "kalshi event");
current_id!(MarginSnapshotId, "margin snapshot");
current_id!(
    MarketDataSubscriptionExchangeScopeId,
    "market data subscription exchange scope"
);
current_id!(MarketDataSubscriptionId, "market data subscription");
current_id!(
    MarketDataSubscriptionPlanId,
    "market data subscription plan"
);
current_id!(NewsStoryId, "news story");
current_id!(OcoId, "oco");
current_id!(OrderStrategyId, "order strategy");
current_id!(OrderStrategyLinkId, "order strategy link");
current_id!(OrderStrategyTypeId, "order strategy type");
current_id!(OrderVersionId, "order version");
current_id!(OrganizationId, "organization");
current_id!(Oso1Id, "oso1");
current_id!(Oso2Id, "oso2");
current_id!(OwnedByAdminId, "owned by admin");
current_id!(
    PermissionedAccountAutoLiqId,
    "permissioned account auto liq"
);
current_id!(PoaContactId, "poa contact");
current_id!(PostTradeCategoryId, "post trade category");
current_id!(ProductId, "product");
current_id!(ProductMarginId, "product margin");
current_id!(ProductSessionId, "product session");
current_id!(PromoCodeId, "promo code");
current_id!(PropertyId, "property");
current_id!(ProviderEntityId, "provider entity");
current_id!(RiskCategoryId, "risk category");
current_id!(RiskTimePeriodId, "risk time period");
current_id!(
    SecondMarketDataSubscriptionId,
    "second market data subscription"
);
current_id!(SenderId, "sender");
current_id!(SpreadDefinitionId, "spread definition");
current_id!(SubAccountRequestId, "sub account request");
current_id!(SubjectId, "subject");
current_id!(TradingPermissionId, "trading permission");
current_id!(TradovateSubscriptionId, "tradovate subscription");
current_id!(TradovateSubscriptionPlanId, "tradovate subscription plan");
current_id!(UserAccountAutoLiqId, "user account auto liq");
current_id!(UserAccountPositionLimitId, "user account position limit");
current_id!(UserAccountRiskParameterId, "user account risk parameter");
current_id!(UserPluginId, "user plugin");
current_id!(UserPromoCodeId, "user promo code");
current_id!(UserPropertyId, "user property");
current_id!(UserReadStatusId, "user read status");
current_id!(UserSessionId, "user session");
current_id!(UserSessionStatsId, "user session stats");
current_id!(WorkspaceTemplateId, "workspace template");

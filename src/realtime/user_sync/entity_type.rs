// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Closed current entity-type and sharding grammars.

use serde::Serialize;

use crate::realtime::RealtimeError;

macro_rules! entity_types {
    ($($(#[$docs:meta])* $variant:ident => $wire_name:literal),+ $(,)?) => {
        /// A current entity family accepted by `user/syncrequest.entityTypes`.
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
        #[non_exhaustive]
        pub enum UserSyncEntityType {
            $($(#[$docs])* #[serde(rename = $wire_name)] $variant,)+
        }

        impl UserSyncEntityType {
            /// Every entity collection published by the pinned current
            /// `SyncMessage` contract, in contract order.
            pub const ALL: &'static [Self] = &[$(Self::$variant,)+];

            /// Returns the current provider entity-type name.
            #[must_use]
            pub const fn as_str(self) -> &'static str {
                match self {
                    $(Self::$variant => $wire_name,)+
                }
            }
        }
    };
}

entity_types! {
    /// User entities.
    User => "user",
    /// User-property entities.
    UserProperty => "userProperty",
    /// Property-definition entities.
    Property => "property",
    /// Account entities.
    Account => "account",
    /// Account-risk-status entities.
    AccountRiskStatus => "accountRiskStatus",
    /// Margin-snapshot entities.
    MarginSnapshot => "marginSnapshot",
    /// User-account-auto-liquidation entities.
    UserAccountAutoLiq => "userAccountAutoLiq",
    /// Cash-balance entities.
    CashBalance => "cashBalance",
    /// Currency entities.
    Currency => "currency",
    /// Position entities.
    Position => "position",
    /// Fill-pair entities.
    FillPair => "fillPair",
    /// Order entities.
    Order => "order",
    /// Contract entities.
    Contract => "contract",
    /// Contract-maturity entities.
    ContractMaturity => "contractMaturity",
    /// Product entities.
    Product => "product",
    /// Exchange entities.
    Exchange => "exchange",
    /// Spread-definition entities.
    SpreadDefinition => "spreadDefinition",
    /// Command entities.
    Command => "command",
    /// Command-report entities.
    CommandReport => "commandReport",
    /// Execution-report entities.
    ExecutionReport => "executionReport",
    /// Order-version entities.
    OrderVersion => "orderVersion",
    /// Fill entities.
    Fill => "fill",
    /// Fill-fee entities.
    FillFee => "fillFee",
    /// Order-strategy entities.
    OrderStrategy => "orderStrategy",
    /// Order-strategy-link entities.
    OrderStrategyLink => "orderStrategyLink",
    /// User-plugin entities.
    UserPlugin => "userPlugin",
    /// Annual-review entities.
    AnnualReview => "annualReview",
    /// User-read-status entities.
    UserReadStatus => "userReadStatus",
    /// User-promo-code entities.
    UserPromoCode => "userPromoCode",
    /// Contract-group entities.
    ContractGroup => "contractGroup",
    /// Order-strategy-type entities.
    OrderStrategyType => "orderStrategyType",
}

/// The current documented identity used by a user-socket shard modulus.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[non_exhaustive]
pub enum UserSyncShardBy {
    /// Route by `accountId % divisor`.
    #[serde(rename = "modAccountId")]
    AccountId,
    /// Route by `userId % divisor`.
    #[serde(rename = "modUserId")]
    UserId,
}

/// A validated current socket-sharding expression.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserSyncSharding {
    expression_type: UserSyncShardBy,
    divisor: i64,
    remainder: i64,
}

impl UserSyncSharding {
    /// Creates one shard of a modulus-partitioned user stream.
    ///
    /// # Errors
    ///
    /// Returns [`RealtimeError::InvalidConfiguration`] when `divisor` is not
    /// positive or `remainder` is outside `0..divisor`.
    pub const fn new(
        expression_type: UserSyncShardBy,
        divisor: i64,
        remainder: i64,
    ) -> Result<Self, RealtimeError> {
        if divisor <= 0 {
            return Err(RealtimeError::InvalidConfiguration {
                field: "user_sync.sharding.divisor",
                reason: "must be positive",
            });
        }
        if remainder < 0 || remainder >= divisor {
            return Err(RealtimeError::InvalidConfiguration {
                field: "user_sync.sharding.remainder",
                reason: "must be in 0..divisor",
            });
        }
        Ok(Self {
            expression_type,
            divisor,
            remainder,
        })
    }

    /// Returns the identity family used by the modulus.
    #[must_use]
    pub const fn expression_type(self) -> UserSyncShardBy {
        self.expression_type
    }

    /// Returns the total number of shards.
    #[must_use]
    pub const fn divisor(self) -> i64 {
        self.divisor
    }

    /// Returns this socket's shard remainder.
    #[must_use]
    pub const fn remainder(self) -> i64 {
        self.remainder
    }
}

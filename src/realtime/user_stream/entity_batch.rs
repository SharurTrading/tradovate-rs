// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Typed user-sync entity batches shared by bootstraps and `props` deltas.

use crate::api::current::alerts::AdminAlertSignal;
use crate::api::current::users::{
    Account, AccountRiskStatus, AnnualReview, CashBalance, Command, CommandReport, Contract,
    ContractGroup, ContractMaturity, Currency, Exchange, ExecutionReport, Fill, FillFee, FillPair,
    MarginSnapshot, Order, OrderStrategy, OrderStrategyLink, OrderStrategyType, OrderVersion,
    Position, Product, Property, SpreadDefinition, User, UserAccountAutoLiq, UserPlugin,
    UserPromoCode, UserProperty, UserReadStatus,
};

use crate::realtime::ProviderCode;

macro_rules! entity_batches {
    ($(
        $(#[$docs:meta])*
        $variant:ident($entity:ty) => $wire_name:literal
    ),+ $(,)?) => {
        /// A typed batch of one current user-stream entity kind.
        #[derive(Clone, Debug)]
        #[non_exhaustive]
        pub enum UserEntityBatch {
            $(
                $(#[$docs])*
                $variant(Box<[$entity]>),
            )+
            /// A bounded future entity family whose raw values are not exposed.
            Unsupported {
                /// Provider entity kind.
                entity_type: ProviderCode,
                /// Validated number of object entities discarded.
                item_count: usize,
            },
        }

        impl UserEntityBatch {
            /// Returns the current provider entity-type name.
            #[must_use]
            pub fn entity_type(&self) -> &str {
                match self {
                    $(Self::$variant(_) => $wire_name,)+
                    Self::Unsupported { entity_type, .. } => entity_type.as_str(),
                }
            }

            /// Returns the number of entities in this bounded batch.
            #[must_use]
            pub fn len(&self) -> usize {
                match self {
                    $(Self::$variant(values) => values.len(),)+
                    Self::Unsupported { item_count, .. } => *item_count,
                }
            }

            /// Returns whether this batch contains no entities.
            #[must_use]
            pub fn is_empty(&self) -> bool {
                self.len() == 0
            }
        }
    };
}

entity_batches! {
    /// User entities.
    Users(User) => "user",
    /// User-property entities.
    UserProperties(UserProperty) => "userProperty",
    /// Property-definition entities.
    Properties(Property) => "property",
    /// Account entities.
    Accounts(Account) => "account",
    /// Account-risk-status entities.
    AccountRiskStatuses(AccountRiskStatus) => "accountRiskStatus",
    /// Margin-snapshot entities.
    MarginSnapshots(MarginSnapshot) => "marginSnapshot",
    /// User-account-auto-liquidation entities.
    UserAccountAutoLiqs(UserAccountAutoLiq) => "userAccountAutoLiq",
    /// Cash-balance entities.
    CashBalances(CashBalance) => "cashBalance",
    /// Currency entities.
    Currencies(Currency) => "currency",
    /// Position entities.
    Positions(Position) => "position",
    /// Fill-pair entities.
    FillPairs(FillPair) => "fillPair",
    /// Order entities.
    Orders(Order) => "order",
    /// Contract entities.
    Contracts(Contract) => "contract",
    /// Contract-maturity entities.
    ContractMaturities(ContractMaturity) => "contractMaturity",
    /// Product entities.
    Products(Product) => "product",
    /// Exchange entities.
    Exchanges(Exchange) => "exchange",
    /// Spread-definition entities.
    SpreadDefinitions(SpreadDefinition) => "spreadDefinition",
    /// Command entities.
    Commands(Command) => "command",
    /// Command-report entities.
    CommandReports(CommandReport) => "commandReport",
    /// Execution-report entities.
    ExecutionReports(ExecutionReport) => "executionReport",
    /// Order-version entities.
    OrderVersions(OrderVersion) => "orderVersion",
    /// Fill entities.
    Fills(Fill) => "fill",
    /// Fill-fee entities.
    FillFees(FillFee) => "fillFee",
    /// Order-strategy entities.
    OrderStrategies(OrderStrategy) => "orderStrategy",
    /// Order-strategy-link entities.
    OrderStrategyLinks(OrderStrategyLink) => "orderStrategyLink",
    /// User-plugin entities.
    UserPlugins(UserPlugin) => "userPlugin",
    /// Annual-review entities.
    AnnualReviews(AnnualReview) => "annualReview",
    /// User-read-status entities.
    UserReadStatuses(UserReadStatus) => "userReadStatus",
    /// User-promo-code entities.
    UserPromoCodes(UserPromoCode) => "userPromoCode",
    /// Contract-group entities.
    ContractGroups(ContractGroup) => "contractGroup",
    /// Order-strategy-type entities.
    OrderStrategyTypes(OrderStrategyType) => "orderStrategyType",
    /// Cross-environment administrator-alert signature events.
    OtherEnvironmentAdminAlertSignals(AdminAlertSignal) => "OtherEnvAdminAlertSignal",
}

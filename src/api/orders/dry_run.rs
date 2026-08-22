// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Bounded pre-trade dry-run validation.

use serde::Serialize;

use super::documentation::CurrentDocumentationGap;
use super::{DryRunResponse, OrderQuantity, OrderSide, OrderType};
use crate::api::current::support::CurrentRequest;
use crate::{AccountId, Client, ContractId, Decimal, Error};

const DRY_RUN_ENDPOINT: &str = "/order/dryrun";
const MAX_DRY_RUN_ORDERS: usize = 4;

/// The current schema declares `extraPreTradeRisk.products` and `contracts` as
/// empty objects, so no keys or value contracts can be encoded without legacy
/// or guessed behavior.
pub const DRY_RUN_EXTRA_RISK_DOCUMENTATION_GAP: CurrentDocumentationGap =
    CurrentDocumentationGap::new(
        DRY_RUN_ENDPOINT,
        "extraPreTradeRisk",
        "product and contract map entries are undefined; documented total limits remain available",
    );

/// Documented aggregate pre-trade limits for a dry-run request.
///
/// The provider requires `products` and `contracts` objects but publishes no
/// entry grammar, so this type emits those objects empty and exposes only the
/// two schema-defined aggregate limits.
#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DryRunExtraRisk {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_exposed_total: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_traded_volume_total: Option<u64>,
    products: EmptyRiskMap,
    contracts: EmptyRiskMap,
}

impl DryRunExtraRisk {
    /// Validates aggregate limits without inventing per-product contracts.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidRequest`] when neither limit is present or a
    /// value exceeds the provider's signed 64-bit integer field.
    pub fn new(
        max_exposed_total: Option<u64>,
        max_traded_volume_total: Option<u64>,
    ) -> Result<Self, Error> {
        if max_exposed_total.is_none() && max_traded_volume_total.is_none() {
            return Err(Error::InvalidRequest {
                field: "extra_pre_trade_risk",
                reason: "at least one documented aggregate limit is required",
            });
        }
        if [max_exposed_total, max_traded_volume_total]
            .into_iter()
            .flatten()
            .any(|value| i64::try_from(value).is_err())
        {
            return Err(Error::InvalidRequest {
                field: "extra_pre_trade_risk",
                reason: "aggregate limits must fit the provider signed 64-bit field",
            });
        }
        Ok(Self {
            max_exposed_total,
            max_traded_volume_total,
            products: EmptyRiskMap {},
            contracts: EmptyRiskMap {},
        })
    }

    /// Returns the maximum aggregate exposed quantity.
    #[must_use]
    pub const fn max_exposed_total(self) -> Option<u64> {
        self.max_exposed_total
    }

    /// Returns the maximum aggregate traded volume.
    #[must_use]
    pub const fn max_traded_volume_total(self) -> Option<u64> {
        self.max_traded_volume_total
    }
}

#[derive(Clone, Copy, Debug, Serialize)]
struct EmptyRiskMap {}

/// One validated order in a pre-trade dry run.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DryRunOrder {
    contract_id: ContractId,
    action: OrderSide,
    order_qty: OrderQuantity,
    order_type: OrderType,
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "crate::decimal::option"
    )]
    price: Option<Decimal>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "crate::decimal::option"
    )]
    stop_price: Option<Decimal>,
}

impl DryRunOrder {
    /// Creates a market-order risk check.
    #[must_use]
    pub const fn market(
        contract_id: ContractId,
        action: OrderSide,
        quantity: OrderQuantity,
    ) -> Self {
        Self::from_parts(contract_id, action, quantity, OrderType::Market, None, None)
    }

    /// Creates a limit-order risk check.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidRequest`] when `price` is not positive.
    pub fn limit(
        contract_id: ContractId,
        action: OrderSide,
        quantity: OrderQuantity,
        price: Decimal,
    ) -> Result<Self, Error> {
        super::wire::validate_prices(OrderType::Limit, Some(price), None)?;
        Ok(Self::from_parts(
            contract_id,
            action,
            quantity,
            OrderType::Limit,
            Some(price),
            None,
        ))
    }

    const fn from_parts(
        contract_id: ContractId,
        action: OrderSide,
        order_qty: OrderQuantity,
        order_type: OrderType,
        price: Option<Decimal>,
        stop_price: Option<Decimal>,
    ) -> Self {
        Self {
            contract_id,
            action,
            order_qty,
            order_type,
            price,
            stop_price,
        }
    }
}

/// A bounded batch of one to four pre-trade checks for one explicit account.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DryRun {
    account_id: AccountId,
    orders: Box<[DryRunOrder]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    extra_pre_trade_risk: Option<DryRunExtraRisk>,
}

impl DryRun {
    /// Validates and owns a current dry-run request.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidRequest`] unless `orders` contains one to four
    /// entries, matching the current Partner endpoint limit.
    pub fn new(account_id: AccountId, orders: Vec<DryRunOrder>) -> Result<Self, Error> {
        if orders.is_empty() || orders.len() > MAX_DRY_RUN_ORDERS {
            return Err(Error::InvalidRequest {
                field: "orders",
                reason: "must contain between one and four dry-run orders",
            });
        }
        Ok(Self {
            account_id,
            orders: orders.into_boxed_slice(),
            extra_pre_trade_risk: None,
        })
    }

    /// Adds documented aggregate pre-trade limits.
    #[must_use]
    pub const fn with_extra_pre_trade_risk(mut self, value: DryRunExtraRisk) -> Self {
        self.extra_pre_trade_risk = Some(value);
        self
    }

    /// Returns the orders submitted for pre-trade evaluation.
    #[must_use]
    pub fn orders(&self) -> &[DryRunOrder] {
        &self.orders
    }
}

impl CurrentRequest for DryRun {
    fn validate_current(&self) -> Result<(), Error> {
        if self.orders.is_empty() || self.orders.len() > MAX_DRY_RUN_ORDERS {
            return Err(Error::InvalidRequest {
                field: "orders",
                reason: "must contain between one and four dry-run orders",
            });
        }
        for order in &self.orders {
            if !matches!(order.order_type, OrderType::Market | OrderType::Limit) {
                return Err(Error::InvalidRequest {
                    field: "orders[].order_type",
                    reason: "current dry-run Stop and StopLimit field grammar is not documented",
                });
            }
            super::wire::validate_prices(order.order_type, order.price, order.stop_price)?;
        }
        Ok(())
    }
}

impl Client {
    /// Validates a bounded batch against the account's pre-trade risk settings
    /// without placing an order.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure.
    pub async fn dry_run(&self, request: &DryRun) -> Result<DryRunResponse, Error> {
        self.post_query(DRY_RUN_ENDPOINT, request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ids() -> (AccountId, ContractId, OrderQuantity) {
        let account = AccountId::new(10).unwrap_or_else(|error| panic!("{error}"));
        let contract = ContractId::new(20).unwrap_or_else(|error| panic!("{error}"));
        let quantity = OrderQuantity::new(1).unwrap_or_else(|error| panic!("{error}"));
        (account, contract, quantity)
    }

    #[test]
    fn dry_run_is_bounded_to_current_four_order_limit() {
        let (account, contract, quantity) = ids();
        let order = DryRunOrder::market(contract, OrderSide::Buy, quantity);
        assert!(DryRun::new(account, Vec::new()).is_err());
        assert!(DryRun::new(account, vec![order; 5]).is_err());
    }

    #[test]
    fn dry_run_serializes_exact_prices_without_extra_risk_escape_hatch() {
        let (account, contract, quantity) = ids();
        let price = "5000.125"
            .parse::<Decimal>()
            .unwrap_or_else(|error| panic!("{error}"));
        let order = DryRunOrder::limit(contract, OrderSide::Buy, quantity, price)
            .unwrap_or_else(|error| panic!("{error}"));
        let request = DryRun::new(account, vec![order]).unwrap_or_else(|error| panic!("{error}"));
        let encoded = serde_json::to_string(&request).unwrap_or_else(|error| panic!("{error}"));

        assert!(encoded.contains(r#""price":5000.125"#));
        assert!(!encoded.contains("extraPreTradeRisk"));
        assert_eq!(
            DRY_RUN_EXTRA_RISK_DOCUMENTATION_GAP.field(),
            "extraPreTradeRisk"
        );
    }

    #[test]
    fn aggregate_extra_risk_serializes_with_empty_undocumented_maps() {
        let (account, contract, quantity) = ids();
        let order = DryRunOrder::market(contract, OrderSide::Buy, quantity);
        let extra = DryRunExtraRisk::new(Some(12), Some(34))
            .unwrap_or_else(|error| panic!("aggregate limits must validate: {error}"));
        let request = DryRun::new(account, vec![order])
            .unwrap_or_else(|error| panic!("dry run must validate: {error}"))
            .with_extra_pre_trade_risk(extra);
        let encoded = serde_json::to_value(request)
            .unwrap_or_else(|error| panic!("dry run must encode: {error}"));

        assert_eq!(encoded["extraPreTradeRisk"]["maxExposedTotal"], 12);
        assert_eq!(encoded["extraPreTradeRisk"]["maxTradedVolumeTotal"], 34);
        assert_eq!(
            encoded["extraPreTradeRisk"]["products"],
            serde_json::json!({})
        );
        assert_eq!(
            encoded["extraPreTradeRisk"]["contracts"],
            serde_json::json!({})
        );
    }
}

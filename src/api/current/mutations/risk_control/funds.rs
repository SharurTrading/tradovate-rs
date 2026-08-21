// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Demo-administrator cash adjustment with fail-closed reconciliation.

use crate::{
    Client, Decimal, Error,
    api::current::{accounting::CashBalanceSnapshot, funds::AdjustCash},
    client::MutationAssessment,
};

const MAX_COMMENT_CHARS: usize = 64;

impl Client {
    /// Adjusts one demo account's simulated cash balance as an administrator.
    ///
    /// The caller must have organization-administrator authority and the
    /// provider's `Funds:FullAccess` permission. There is no automatic retry or
    /// undo; reverse an adjustment with a separately authorized opposite delta.
    ///
    /// # Errors
    ///
    /// Returns a local error outside demo, when `cashChange` is not strictly
    /// between -100,000 and 100,000, or when `comment` exceeds 64 characters.
    /// It may also return authentication, rate, transport, provider-control,
    /// decoding, or ambiguity errors. The balance snapshot omits the account,
    /// currency, and exact delta, so apparent success remains latched until the
    /// account balance is authoritatively reconciled.
    pub async fn fund_transaction_adjust_cash(
        &self,
        request: &AdjustCash,
    ) -> Result<CashBalanceSnapshot, Error> {
        validate_adjust_cash(request)?;
        if !self.endpoints.permits_demo_only_rest() {
            return Err(Error::InvalidRequest {
                field: "environment",
                reason: "adjustCash requires the demo REST environment",
            });
        }
        self.post_reviewed_mutation("/fundTransaction/adjustcash", request, assess_adjust_cash)
            .await
    }
}

fn validate_adjust_cash(request: &AdjustCash) -> Result<(), Error> {
    let minimum = Decimal::from(-100_000_i64);
    let maximum = Decimal::from(100_000_i64);
    if request.cash_change() <= &minimum || request.cash_change() >= &maximum {
        return Err(Error::InvalidRequest {
            field: "cashChange",
            reason: "must be strictly between -100000 and 100000",
        });
    }
    if request
        .comment()
        .is_some_and(|comment| comment.chars().count() > MAX_COMMENT_CHARS)
    {
        return Err(Error::InvalidRequest {
            field: "comment",
            reason: "must not exceed 64 characters",
        });
    }
    Ok(())
}

fn assess_adjust_cash(response: &CashBalanceSnapshot, _: &AdjustCash) -> MutationAssessment {
    MutationAssessment::ambiguous(snapshot_has_financial_data(response))
}

fn snapshot_has_financial_data(response: &CashBalanceSnapshot) -> bool {
    response.total_cash_value().is_some()
        || response.total_pn_l().is_some()
        || response.initial_margin().is_some()
        || response.maintenance_margin().is_some()
        || response.net_liq().is_some()
        || response.open_pn_l().is_some()
        || response.realized_pn_l().is_some()
        || response.week_realized_pn_l().is_some()
        || response.currency_cash_avail_withdrawal_usd().is_some()
        || response.net_liq_sod().is_some()
        || response.total_cash_value_sod().is_some()
        || response.cash_usd().is_some()
        || response.cash_sodusd().is_some()
        || response.full_initial_margin().is_some()
        || response.full_initial_margin_sod().is_some()
        || response.auto_liq_level().is_some()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        AccountId,
        api::current::{funds::AdjustCashCashChangeType, ids::CurrencyId},
    };

    #[test]
    fn cash_change_bounds_are_strict() {
        let lower = request(Decimal::from(-100_000_i64), None);
        let upper = request(Decimal::from(100_000_i64), None);
        assert!(validate_adjust_cash(&lower).is_err());
        assert!(validate_adjust_cash(&upper).is_err());
    }

    #[test]
    fn comment_limit_counts_characters() {
        let accepted = request(Decimal::ONE, Some("£".repeat(MAX_COMMENT_CHARS)));
        let rejected = request(Decimal::ONE, Some("£".repeat(MAX_COMMENT_CHARS + 1)));
        assert!(validate_adjust_cash(&accepted).is_ok());
        assert!(validate_adjust_cash(&rejected).is_err());
    }

    fn request(cash_change: Decimal, comment: Option<String>) -> AdjustCash {
        let builder = AdjustCash::builder()
            .account_id(account(1))
            .cash_change(cash_change)
            .cash_change_type(AdjustCashCashChangeType::ManualAdjustment)
            .currency_id(currency(2));
        let builder = match comment {
            Some(comment) => builder.comment(comment),
            None => builder,
        };
        builder
            .build()
            .unwrap_or_else(|error| panic!("adjust-cash fixture: {error}"))
    }

    fn account(value: i64) -> AccountId {
        AccountId::new(value).unwrap_or_else(|error| panic!("account fixture: {error}"))
    }

    fn currency(value: i64) -> CurrencyId {
        CurrencyId::new(value).unwrap_or_else(|error| panic!("currency fixture: {error}"))
    }
}

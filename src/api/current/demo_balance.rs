// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Conservative account-scoped admission for demo cash-balance changes.

use super::accounting::{CashBalanceSnapshot, ChangeDemoBalance};
use crate::{
    Client, Decimal, Error,
    client::{DocumentedMutationResponse, MutationOutcome},
};

const ENDPOINT: &str = "/cashBalance/changedemobalance";
const MAX_COMMENT_CHARS: usize = 64;

// The current response has no account identity, change identifier, or exact
// delta evidence. It is useful for decoding provider-control contradictions,
// but can never resolve an admitted mutation by itself.
impl DocumentedMutationResponse for CashBalanceSnapshot {
    fn mutation_outcome(&self) -> MutationOutcome {
        MutationOutcome::Ambiguous
    }

    fn has_success_evidence(&self) -> bool {
        false
    }
}

impl Client {
    /// Changes one demo account's simulated cash balance.
    ///
    /// The current provider contract permits one change per account per hour
    /// for non-administrators. Session metadata does not prove administrator
    /// authority, so this client conservatively applies the account limit to
    /// every caller. Different accounts retain independent admission windows.
    ///
    /// # Errors
    ///
    /// Returns a local validation failure outside the demo environment, when
    /// `cashChange` is not strictly between -1,000,000 and 1,000,000, or when
    /// `comment` exceeds 64 characters. It can also return authentication,
    /// immediate rate-admission, transport, provider-control, bounded-response,
    /// decoding, business-rejection, or ambiguous-mutation failures. A 2xx
    /// balance snapshot cannot identify the changed account or prove the exact
    /// delta, so an apparently accepted change deliberately returns
    /// [`Error::AmbiguousMutation`] and requires authoritative reconciliation.
    pub async fn cash_balance_change_demo_balance(
        &self,
        request: &ChangeDemoBalance,
    ) -> Result<CashBalanceSnapshot, Error> {
        validate_request(request)?;
        if !self.endpoints.permits_demo_only_rest() {
            return Err(Error::InvalidRequest {
                field: "environment",
                reason: "changeDemoBalance requires the demo REST environment",
            });
        }
        self.post_account_scoped_unresolved_mutation(ENDPOINT, *request.account_id(), request)
            .await
    }
}

fn validate_request(request: &ChangeDemoBalance) -> Result<(), Error> {
    let cash_change = *request.cash_change();
    let minimum = Decimal::from(-1_000_000_i64);
    let maximum = Decimal::from(1_000_000_i64);
    if cash_change <= minimum || cash_change >= maximum {
        return Err(Error::InvalidRequest {
            field: "cashChange",
            reason: "must be strictly between -1000000 and 1000000",
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

#[cfg(test)]
#[path = "demo_balance/tests.rs"]
mod tests;

// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Account-scoped user auto-liquidation mutations.

use std::collections::HashSet;

use crate::{
    Client, Error,
    api::current::risks::{
        UpdateUserAutoLiq, UpdateUserAutoLiqResponse, UpdateUserAutoLiqs,
        UpdateUserAutoLiqsResponse,
    },
    client::{DocumentedMutationResponse, MutationAssessment, MutationOutcome},
};

const MAX_BATCH_ITEMS: usize = 10_000;

impl DocumentedMutationResponse for UpdateUserAutoLiqResponse {
    fn mutation_outcome(&self) -> MutationOutcome {
        MutationOutcome::Ambiguous
    }

    fn has_success_evidence(&self) -> bool {
        self.user_account_auto_liq().is_some() || self.permissioned_account_auto_liq().is_some()
    }
}

impl Client {
    /// Updates auto-liquidation controls for one account.
    ///
    /// # Errors
    ///
    /// Returns validation, authentication, rate, transport, provider-control,
    /// decoding, or ambiguity errors. Returned auto-liq entity IDs are not the
    /// request account ID, so apparent success remains latched until the account
    /// is reconciled through an authoritative query or user-stream update.
    pub async fn user_account_auto_liq_update_user_auto_liq(
        &self,
        request: &UpdateUserAutoLiq,
    ) -> Result<UpdateUserAutoLiqResponse, Error> {
        self.post_unresolved_mutation("/userAccountAutoLiq/updateuserautoliq", request)
            .await
    }

    /// Updates auto-liquidation controls for a bounded set of unique accounts.
    ///
    /// # Errors
    ///
    /// Returns a local error for an empty, duplicate, or over-10,000-item
    /// request, plus authentication, rate, transport, provider-control,
    /// decoding, or ambiguity errors. Even matching response cardinality does
    /// not correlate returned entity IDs to accounts, so success stays latched.
    pub async fn user_account_auto_liq_update_user_auto_liqs(
        &self,
        request: &UpdateUserAutoLiqs,
    ) -> Result<UpdateUserAutoLiqsResponse, Error> {
        validate_batch(request)?;
        self.post_reviewed_mutation(
            "/userAccountAutoLiq/updateuserautoliqs",
            request,
            assess_batch,
        )
        .await
    }
}

fn validate_batch(request: &UpdateUserAutoLiqs) -> Result<(), Error> {
    let items = request.items();
    if items.is_empty() || items.len() > MAX_BATCH_ITEMS {
        return Err(Error::InvalidRequest {
            field: "items",
            reason: "must contain between 1 and 10000 items",
        });
    }
    let mut accounts = HashSet::with_capacity(items.len());
    if !items
        .iter()
        .map(|item| *item.account_id())
        .all(|account| accounts.insert(account))
    {
        return Err(Error::InvalidRequest {
            field: "items",
            reason: "must target unique account identities",
        });
    }
    Ok(())
}

fn assess_batch(
    response: &UpdateUserAutoLiqsResponse,
    request: &UpdateUserAutoLiqs,
) -> MutationAssessment {
    let complete_cardinality = response.user_auto_liqs().len() == request.items().len()
        && response.user_auto_liqs().iter().all(|item| {
            item.user_account_auto_liq().is_some() || item.permissioned_account_auto_liq().is_some()
        });
    MutationAssessment::ambiguous(complete_cardinality)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AccountId;

    #[test]
    fn duplicate_batch_accounts_are_rejected() {
        let first = item(9);
        let second = item(9);
        let request = UpdateUserAutoLiqs::builder()
            .items(vec![first, second])
            .build();
        let Ok(request) = request else {
            panic!("batch fixture must build");
        };
        assert!(validate_batch(&request).is_err());
    }

    fn item(account_id: i64) -> crate::api::current::risks::UpdateUserAutoLiqItem {
        let account_id =
            AccountId::new(account_id).unwrap_or_else(|error| panic!("account fixture: {error}"));
        crate::api::current::risks::UpdateUserAutoLiqItem::builder()
            .account_id(account_id)
            .build()
            .unwrap_or_else(|error| panic!("auto-liq fixture: {error}"))
    }
}

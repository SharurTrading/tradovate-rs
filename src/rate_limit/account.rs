// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Bounded account-scoped cooldown admission.
//!
//! The current operation contract permits one demo-balance change per account
//! per hour for non-organization administrators. Session metadata does not
//! prove administrator status, so this guard remains conservative for every
//! caller and composes with the separate 1,000/hour endpoint window.

use std::{collections::HashMap, time::Duration};

use tokio::time::Instant;

use super::HOUR;
use crate::AccountId;

pub(super) const MAX_ACCOUNT_RATE_KEYS: usize = 4_096;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct AccountRateKey {
    endpoint: &'static str,
    account_id: AccountId,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct AccountReservation {
    key: AccountRateKey,
    deadline: Instant,
}

#[derive(Debug, Default)]
pub(super) struct AccountWindows {
    deadlines: HashMap<AccountRateKey, Instant>,
}

impl AccountWindows {
    pub(super) fn evaluate(
        &mut self,
        now: Instant,
        endpoint: &'static str,
        account_id: AccountId,
    ) -> Result<AccountReservation, Duration> {
        self.deadlines.retain(|_, deadline| *deadline > now);
        let key = AccountRateKey {
            endpoint,
            account_id,
        };
        if let Some(deadline) = self.deadlines.get(&key) {
            return Err(*deadline - now);
        }
        if self.deadlines.len() >= MAX_ACCOUNT_RATE_KEYS {
            return Err(self
                .deadlines
                .values()
                .map(|deadline| *deadline - now)
                .min()
                .unwrap_or(HOUR));
        }
        let Some(deadline) = now.checked_add(HOUR) else {
            return Err(HOUR);
        };
        Ok(AccountReservation { key, deadline })
    }

    pub(super) fn admit(&mut self, reservation: AccountReservation) {
        self.deadlines.insert(reservation.key, reservation.deadline);
    }

    pub(super) fn release(&mut self, endpoint: &'static str, account_id: AccountId) {
        self.deadlines.remove(&AccountRateKey {
            endpoint,
            account_id,
        });
    }

    #[cfg(test)]
    pub(super) fn is_empty(&self) -> bool {
        self.deadlines.is_empty()
    }

    #[cfg(test)]
    pub(super) fn len(&self) -> usize {
        self.deadlines.len()
    }
}

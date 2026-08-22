// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Cross-field validated user-sync request profiles and wire encoding.

use std::collections::BTreeSet;

use jiff::Timestamp;
use serde::Serialize;

use super::{UserSyncEntityType, UserSyncSharding};
use crate::{AccountId, UserId, realtime::RealtimeError};

const MAX_FILTER_IDS: usize = 4_096;

/// A validated unsplit current user-synchronization request.
///
/// The default explicitly subscribes to every entity family in the pinned
/// current `SyncMessage` schema. `splitResponses` is always sent as `false`;
/// multipart B2B readiness remains documentation-blocked because the current
/// documentation does not publish a completion marker.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserSyncConfig {
    users: Option<Box<[UserId]>>,
    accounts: Option<Box<[AccountId]>>,
    cutoff_timestamp: Option<Timestamp>,
    entity_types: Option<Box<[UserSyncEntityType]>>,
    sharding: Option<UserSyncSharding>,
    full_org_snapshot: Option<bool>,
}

impl UserSyncConfig {
    /// Creates an unsharded subscription for a non-empty set of entity types.
    ///
    /// # Errors
    ///
    /// Returns [`RealtimeError::InvalidConfiguration`] for an empty or
    /// duplicate entity list.
    pub fn new(entity_types: Vec<UserSyncEntityType>) -> Result<Self, RealtimeError> {
        validate_entity_types(&entity_types)?;
        Ok(Self {
            users: None,
            accounts: None,
            cutoff_timestamp: None,
            entity_types: Some(entity_types.into_boxed_slice()),
            sharding: None,
            full_org_snapshot: None,
        })
    }

    /// Creates the current documented user-ID filter profile.
    ///
    /// This profile omits `entityTypes`, as the provider forbids combining
    /// that field with `users`.
    ///
    /// # Errors
    ///
    /// Returns [`RealtimeError::InvalidConfiguration`] for an empty,
    /// duplicate, or oversized user list.
    pub fn for_users(users: Vec<UserId>) -> Result<Self, RealtimeError> {
        validate_ids(&users, "user_sync.users")?;
        Ok(Self {
            users: Some(users.into_boxed_slice()),
            accounts: None,
            cutoff_timestamp: None,
            entity_types: None,
            sharding: None,
            full_org_snapshot: None,
        })
    }

    /// Filters an unsharded request to a non-empty account set.
    ///
    /// # Errors
    ///
    /// Returns [`RealtimeError::InvalidConfiguration`] for an empty,
    /// duplicate, or oversized list, or when socket sharding is configured.
    pub fn accounts(mut self, accounts: Vec<AccountId>) -> Result<Self, RealtimeError> {
        if self.sharding.is_some() {
            return Err(conflict(
                "user_sync.accounts",
                "cannot be combined with sharding",
            ));
        }
        validate_ids(&accounts, "user_sync.accounts")?;
        self.accounts = Some(accounts.into_boxed_slice());
        Ok(self)
    }

    /// Partitions an entity-type subscription across socket shards.
    ///
    /// # Errors
    ///
    /// Returns [`RealtimeError::InvalidConfiguration`] when user or account
    /// filters are already configured, or when this is a user-filter profile.
    pub fn sharding(mut self, sharding: UserSyncSharding) -> Result<Self, RealtimeError> {
        if self.users.is_some() || self.accounts.is_some() || self.entity_types.is_none() {
            return Err(conflict(
                "user_sync.sharding",
                "requires entity types and cannot be combined with user/account filters",
            ));
        }
        self.sharding = Some(sharding);
        Ok(self)
    }

    /// Applies the current `cutoffTimestamp` field.
    #[must_use]
    pub fn cutoff_timestamp(mut self, cutoff: Timestamp) -> Self {
        self.cutoff_timestamp = Some(cutoff);
        self
    }

    /// Applies the current `fullOrgSnapshot` field.
    #[must_use]
    pub const fn full_org_snapshot(mut self, enabled: bool) -> Self {
        self.full_org_snapshot = Some(enabled);
        self
    }

    /// Returns the optional user-ID filter.
    #[must_use]
    pub fn users(&self) -> Option<&[UserId]> {
        self.users.as_deref()
    }

    /// Returns the optional account-ID filter.
    #[must_use]
    pub fn account_ids(&self) -> Option<&[AccountId]> {
        self.accounts.as_deref()
    }

    /// Returns the optional point-in-time cutoff.
    #[must_use]
    pub const fn cutoff(&self) -> Option<&Timestamp> {
        self.cutoff_timestamp.as_ref()
    }

    /// Returns the requested entity families, or `None` for a user filter.
    #[must_use]
    pub fn entity_types(&self) -> Option<&[UserSyncEntityType]> {
        self.entity_types.as_deref()
    }

    /// Returns the optional socket-sharding expression.
    #[must_use]
    pub const fn sharding_expression(&self) -> Option<UserSyncSharding> {
        self.sharding
    }

    /// Returns the optional full-organization snapshot flag.
    #[must_use]
    pub const fn full_org_snapshot_enabled(&self) -> Option<bool> {
        self.full_org_snapshot
    }

    pub(crate) fn validate(self) -> Result<Self, RealtimeError> {
        if let Some(users) = &self.users {
            validate_ids(users, "user_sync.users")?;
        }
        if let Some(accounts) = &self.accounts {
            validate_ids(accounts, "user_sync.accounts")?;
        }
        if let Some(entity_types) = &self.entity_types {
            validate_entity_types(entity_types)?;
        }
        if self.users.is_some() && (self.entity_types.is_some() || self.sharding.is_some()) {
            return Err(conflict(
                "user_sync.users",
                "cannot be combined with entity types or sharding",
            ));
        }
        if self.accounts.is_some() && self.sharding.is_some() {
            return Err(conflict(
                "user_sync.accounts",
                "cannot be combined with sharding",
            ));
        }
        Ok(self)
    }

    pub(crate) fn encode(&self) -> Result<String, RealtimeError> {
        serde_json::to_string(&WireUserSync {
            split_responses: false,
            users: self.users.as_deref(),
            accounts: self.accounts.as_deref(),
            cutoff_timestamp: self.cutoff_timestamp.as_ref(),
            entity_types: self.entity_types.as_deref(),
            sharding_expression: self.sharding.as_ref(),
            full_org_snapshot: self.full_org_snapshot,
        })
        .map_err(|_| RealtimeError::Protocol)
    }
}

impl Default for UserSyncConfig {
    fn default() -> Self {
        Self {
            users: None,
            accounts: None,
            cutoff_timestamp: None,
            entity_types: Some(UserSyncEntityType::ALL.to_vec().into_boxed_slice()),
            sharding: None,
            full_org_snapshot: None,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct WireUserSync<'a> {
    split_responses: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    users: Option<&'a [UserId]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accounts: Option<&'a [AccountId]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cutoff_timestamp: Option<&'a Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entity_types: Option<&'a [UserSyncEntityType]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sharding_expression: Option<&'a UserSyncSharding>,
    #[serde(skip_serializing_if = "Option::is_none")]
    full_org_snapshot: Option<bool>,
}

fn validate_entity_types(values: &[UserSyncEntityType]) -> Result<(), RealtimeError> {
    if values.is_empty() {
        return Err(conflict(
            "user_sync.entity_types",
            "must contain at least one entity type",
        ));
    }
    let unique = values.iter().copied().collect::<BTreeSet<_>>();
    if unique.len() != values.len() {
        return Err(conflict(
            "user_sync.entity_types",
            "must not contain duplicates",
        ));
    }
    Ok(())
}

fn validate_ids<T>(values: &[T], field: &'static str) -> Result<(), RealtimeError>
where
    T: Copy + Ord,
{
    if values.is_empty() {
        return Err(conflict(field, "must contain at least one ID"));
    }
    if values.len() > MAX_FILTER_IDS {
        return Err(conflict(field, "exceeds the 4096-ID safety maximum"));
    }
    let unique = values.iter().copied().collect::<BTreeSet<_>>();
    if unique.len() != values.len() {
        return Err(conflict(field, "must not contain duplicates"));
    }
    Ok(())
}

const fn conflict(field: &'static str, reason: &'static str) -> RealtimeError {
    RealtimeError::InvalidConfiguration { field, reason }
}

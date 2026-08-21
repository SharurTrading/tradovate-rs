// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Validated current `user/syncrequest` configuration.

mod config;
mod entity_type;

pub use config::UserSyncConfig;
pub use entity_type::{UserSyncEntityType, UserSyncShardBy, UserSyncSharding};

#[cfg(test)]
#[path = "user_sync/tests.rs"]
mod tests;

// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

use super::*;
use crate::{AccountId, UserId};
use jiff::Timestamp;

#[test]
fn default_explicitly_requests_every_current_entity_family_unsplit() {
    let config = UserSyncConfig::default();
    let encoded = config.encode();
    let Ok(encoded) = encoded else {
        panic!("default sync config must encode");
    };
    let value = serde_json::from_str::<serde_json::Value>(&encoded);
    let Ok(value) = value else {
        panic!("encoded sync config must be JSON");
    };
    assert_eq!(value["splitResponses"], false);
    assert_eq!(
        value["entityTypes"].as_array().map(Vec::len),
        Some(UserSyncEntityType::ALL.len())
    );
    assert_eq!(UserSyncEntityType::ALL.len(), 31);
}

#[test]
fn documented_filter_and_sharding_conflicts_fail_locally() {
    let user = UserId::new(1).unwrap_or_else(|error| panic!("fixture user ID: {error}"));
    let account = AccountId::new(2).unwrap_or_else(|error| panic!("fixture account ID: {error}"));
    let shard = UserSyncSharding::new(UserSyncShardBy::AccountId, 3, 1)
        .unwrap_or_else(|error| panic!("fixture shard: {error}"));

    let users = UserSyncConfig::for_users(vec![user])
        .unwrap_or_else(|error| panic!("fixture user filter: {error}"));
    assert!(users.clone().sharding(shard).is_err());

    let accounts = UserSyncConfig::default()
        .accounts(vec![account])
        .unwrap_or_else(|error| panic!("fixture account filter: {error}"));
    assert!(accounts.sharding(shard).is_err());
}

#[test]
fn sharding_uses_the_closed_current_expression_grammar() {
    let shard = UserSyncSharding::new(UserSyncShardBy::UserId, 4, 3)
        .unwrap_or_else(|error| panic!("fixture shard: {error}"));
    let config = UserSyncConfig::new(vec![UserSyncEntityType::Order])
        .and_then(|config| config.sharding(shard));
    let Ok(config) = config else {
        panic!("documented sharding profile must validate");
    };
    let encoded = config.encode();
    let Ok(encoded) = encoded else {
        panic!("sharded sync config must encode");
    };
    assert!(encoded.contains(r#""expressionType":"modUserId""#));
    assert!(UserSyncSharding::new(UserSyncShardBy::UserId, 0, 0).is_err());
    assert!(UserSyncSharding::new(UserSyncShardBy::UserId, 4, 4).is_err());
}

#[test]
fn duplicate_filters_and_entity_types_are_rejected() {
    let user = UserId::new(1).unwrap_or_else(|error| panic!("fixture user ID: {error}"));
    assert!(UserSyncConfig::for_users(vec![user, user]).is_err());
    assert!(
        UserSyncConfig::new(vec![UserSyncEntityType::Order, UserSyncEntityType::Order]).is_err()
    );
}

#[test]
fn typed_filters_cutoff_and_full_org_fields_encode_without_raw_input() {
    let user = UserId::new(11).unwrap_or_else(|error| panic!("fixture user ID: {error}"));
    let account = AccountId::new(22).unwrap_or_else(|error| panic!("fixture account ID: {error}"));
    let cutoff = "2026-08-21T00:00:00Z"
        .parse::<Timestamp>()
        .unwrap_or_else(|error| panic!("fixture cutoff: {error}"));
    let config = UserSyncConfig::for_users(vec![user])
        .and_then(|config| config.accounts(vec![account]))
        .map(|config| config.cutoff_timestamp(cutoff).full_org_snapshot(false));
    let Ok(config) = config else {
        panic!("documented filtered profile must validate");
    };
    let encoded = config.encode();
    let Ok(encoded) = encoded else {
        panic!("documented filtered profile must encode");
    };
    assert!(encoded.contains(r#""users":[11]"#));
    assert!(encoded.contains(r#""accounts":[22]"#));
    assert!(encoded.contains(r#""cutoffTimestamp":"2026-08-21T00:00:00Z""#));
    assert!(encoded.contains(r#""fullOrgSnapshot":false"#));
    assert!(!encoded.contains("entityTypes"));
    assert!(!encoded.contains("shardingExpression"));
}

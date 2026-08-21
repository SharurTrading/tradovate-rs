// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

use super::*;

#[test]
fn one_oversized_query_value_is_rejected_before_insertion() {
    let mut pairs = Vec::new();
    let value = "x".repeat(MAX_QUERY_SOURCE_BYTES + 1);
    let result = push_query_value(&mut pairs, "name", &value);
    assert!(matches!(result, Err(crate::Error::InvalidRequest { .. })));
    assert!(pairs.is_empty());
}

#[test]
fn aggregate_query_size_is_bounded() {
    let mut pairs = Vec::new();
    assert!(push_query_value(&mut pairs, "name", &"x".repeat(10_000)).is_ok());
    assert!(push_query_value(&mut pairs, "name", &"y".repeat(10_000)).is_ok());
    assert!(push_query_value(&mut pairs, "name", &"z".repeat(1_000)).is_err());
    assert_eq!(pairs.len(), 2);
}

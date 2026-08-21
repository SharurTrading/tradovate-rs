// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

use super::*;

#[test]
fn encoding_stops_at_the_configured_ceiling() {
    let payload = "x".repeat(128);
    let error = encode_bounded_json(&payload, "/fixture", 16);
    assert!(matches!(
        error,
        Err(Error::RequestTooLarge {
            endpoint: "/fixture",
            limit: 16
        })
    ));
}

#[test]
fn valid_json_within_the_ceiling_is_returned() {
    let encoded = encode_bounded_json(&[1_u8, 2, 3], "/fixture", 32);
    assert!(matches!(
        encoded.as_deref(),
        Ok(bytes) if bytes == b"[1,2,3]"
    ));
}

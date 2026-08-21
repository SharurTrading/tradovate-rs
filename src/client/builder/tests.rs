// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

use super::*;

fn builder() -> ClientBuilder {
    ClientBuilder::new(crate::Environment::Demo.endpoints())
}

#[test]
fn zero_and_excessive_response_limits_are_rejected() {
    assert!(builder().max_response_bytes(0).build().is_err());
    assert!(
        builder()
            .max_response_bytes(HARD_MAX_RESPONSE_BYTES + 1)
            .build()
            .is_err()
    );
}

#[test]
fn excessive_timeouts_are_rejected() {
    assert!(
        builder()
            .request_timeout(HARD_MAX_REQUEST_TIMEOUT + Duration::from_secs(1))
            .build()
            .is_err()
    );
    assert!(
        builder()
            .connect_timeout(HARD_MAX_CONNECT_TIMEOUT + Duration::from_secs(1))
            .build()
            .is_err()
    );
}

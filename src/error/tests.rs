// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

use super::*;

#[test]
fn penalty_retry_is_bound_and_single_use() {
    let mut ticket = PenaltyTicket::new(
        "synthetic-ticket".to_owned(),
        Duration::ZERO,
        false,
        7,
        "/auth/accesstokenrequest",
        Some(Instant::now()),
    );
    ticket.bind_request(b"exact-request".to_vec());
    assert!(
        ticket
            .begin_claim_for_request(7, "/auth/accesstokenrequest", b"different-request")
            .is_none()
    );
    assert!(
        ticket
            .begin_claim_for_request(8, "/auth/accesstokenrequest", b"exact-request")
            .is_none()
    );
    assert!(
        ticket
            .begin_claim_for_request(7, "/different", b"exact-request")
            .is_none()
    );
    let claim = ticket.begin_claim_for_request(7, "/auth/accesstokenrequest", b"exact-request");
    assert!(claim.is_some());
    drop(claim);
    let mut claim = ticket
        .begin_claim_for_request(7, "/auth/accesstokenrequest", b"exact-request")
        .unwrap_or_else(|| panic!("cancelled pre-send claim must roll back"));
    claim.arm();
    drop(claim);
    assert!(
        ticket
            .begin_claim_for_request(7, "/auth/accesstokenrequest", b"exact-request")
            .is_none()
    );
}

#[test]
fn penalty_retry_cannot_be_claimed_before_its_monotonic_deadline() {
    let mut ticket = PenaltyTicket::new(
        "synthetic-ticket".to_owned(),
        Duration::from_mins(1),
        false,
        7,
        "/auth/accesstokenrequest",
        Instant::now().checked_add(Duration::from_mins(1)),
    );
    ticket.bind_request(b"exact-request".to_vec());
    assert!(
        ticket
            .begin_claim_for_request(7, "/auth/accesstokenrequest", b"exact-request")
            .is_none()
    );
}

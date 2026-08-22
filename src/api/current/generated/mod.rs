// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0
// @generated
// Generator: tools/generate_openapi.py
// Source: https://partner.tradovate.com/openapi.json (snapshot 2026-08-21, sha256 37caeccf4b0913460a788fcaf4c902497059b8ffe6f6355512e6c08eaacde769)

//! Generated modules for the pinned current Partner contract.

/// Current account, balance, margin, and permission operations.
pub mod accounting;
/// Current alert operations and wire models.
pub mod alerts;
/// Current authentication operations and wire models.
pub mod authentication;
/// Current configuration and entitlement operations.
pub mod configuration;
/// Current contract-library operations and wire models.
pub mod contracts;
/// Current fee and subscription-plan operations.
pub mod fees;
/// Current fund-adjustment operations.
pub mod funds;
/// Validated generated provider identities.
pub mod ids;
mod manifest;
/// Current order, command, execution, and fill operations.
pub mod orders;
/// Current position and fill-pair operations.
pub mod positions;
/// Current risk-control operations and wire models.
pub mod risks;
/// Current user, subscription, and contact operations.
pub mod users;

pub use manifest::OPERATIONS;

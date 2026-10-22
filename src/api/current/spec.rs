// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Identity of the pinned current Partner contract snapshot and its schema gaps.

use super::support::SchemaGap;

/// SHA-256 of the pinned official current Partner `OpenAPI` JSON.
pub const SPEC_SHA256: &str = "37caeccf4b0913460a788fcaf4c902497059b8ffe6f6355512e6c08eaacde769";

/// Date on which the official current Partner contract was pinned.
pub const SPEC_SNAPSHOT_DATE: &str = "2026-08-21";

/// Current component schemas that cannot be represented losslessly without
/// inventing a value grammar or exposing raw JSON.
pub const SCHEMA_GAPS: &[SchemaGap] = &[
    SchemaGap {
        schema: "ExtraPreTradeRiskProducts",
        operations: &["/order/dryrun"],
        reason: "the current component is an object with no value schema",
    },
    SchemaGap {
        schema: "ExtraPreTradeRiskContracts",
        operations: &["/order/dryrun"],
        reason: "the current component is an object with no value schema",
    },
    SchemaGap {
        schema: "RollContractsResponseContracts",
        operations: &["/contract/rollcontracts"],
        reason: "the current response map has no key or value schema",
    },
];

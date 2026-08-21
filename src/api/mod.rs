// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Typed REST capabilities grouped by provider resource family.

mod orders;

pub mod current;

pub use orders::{
    ADVANCED_ORDER_TYPES_DOCUMENTATION_GAP, ADVANCED_ORDER_TYPES_DOCUMENTATION_GAPS, AttachedOrder,
    CancelOrder, CurrentDocumentationGap, CustomTag50, DRY_RUN_EXTRA_RISK_DOCUMENTATION_GAP,
    DryRun, DryRunExtraRisk, DryRunOrder, DryRunResponse, DryRunResponseRejectReason,
    EstimatedFillFee, LiquidatePosition, LiquidatePositions, LiquidationAuthority,
    MODIFY_ORDER_STRATEGY_DOCUMENTATION_GAP, ModifyOrder, MultiBracket, MultiBracketParams, OcoId,
    OcoPlacement, OrderFailureReason, OrderOrigin, OrderPlacement, OrderQuantity, OrderSide,
    OrderStrategyId, OrderStrategyReceipt, OrderStrategyStatus, OrderText, OrderType, Oso1Id,
    Oso2Id, OsoPlacement, PlaceOco, PlaceOrder, PlaceOrderBuilder, PlaceOso, RiskEvaluationDetails,
    STANDARD_ORDER_COMBINATIONS_DOCUMENTATION_GAPS, StartMultiBracketStrategy, StrategyInstanceId,
    TimeInForce,
};

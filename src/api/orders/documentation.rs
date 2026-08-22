// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Machine-readable gaps in the pinned current Partner contract.

const ADVANCED_TYPES_REASON: &str = "current field invariants are absent for LimitIfTouched, MIT, QTS, TrailingStop, and TrailingStopLimit";

/// Every current order surface where the Partner schema names advanced types
/// without defining their price, trigger, peg, or display-field invariants.
///
/// These entries cover the standalone placement, modification, dry-run, OCO,
/// OSO, and both OSO bracket positions. The safe handwritten builders do not
/// transmit those variants until the current contract publishes the missing
/// combinations.
pub const ADVANCED_ORDER_TYPES_DOCUMENTATION_GAPS: [CurrentDocumentationGap; 8] = [
    CurrentDocumentationGap::new("/order/placeorder", "orderType", ADVANCED_TYPES_REASON),
    CurrentDocumentationGap::new("/order/modifyorder", "orderType", ADVANCED_TYPES_REASON),
    CurrentDocumentationGap::new("/order/dryrun", "orders[].orderType", ADVANCED_TYPES_REASON),
    CurrentDocumentationGap::new("/order/placeoco", "orderType", ADVANCED_TYPES_REASON),
    CurrentDocumentationGap::new("/order/placeoco", "other.orderType", ADVANCED_TYPES_REASON),
    CurrentDocumentationGap::new("/order/placeoso", "orderType", ADVANCED_TYPES_REASON),
    CurrentDocumentationGap::new(
        "/order/placeoso",
        "bracket1.orderType",
        ADVANCED_TYPES_REASON,
    ),
    CurrentDocumentationGap::new(
        "/order/placeoso",
        "bracket2.orderType",
        ADVANCED_TYPES_REASON,
    ),
];

/// Backward-compatible first entry in
/// [`ADVANCED_ORDER_TYPES_DOCUMENTATION_GAPS`].
pub const ADVANCED_ORDER_TYPES_DOCUMENTATION_GAP: CurrentDocumentationGap =
    ADVANCED_ORDER_TYPES_DOCUMENTATION_GAPS[0];

/// Standard Stop/StopLimit combinations whose field grammar is not specified
/// by the current endpoint prose, even though the scalar schema permits them.
///
/// OCO is absent because its current documentation explicitly defines both
/// combinations. Standalone Stop placement is absent because the current
/// place-order example identifies `price` as the single Stop value.
pub const STANDARD_ORDER_COMBINATIONS_DOCUMENTATION_GAPS: [CurrentDocumentationGap; 6] = [
    CurrentDocumentationGap::new(
        "/order/placeorder",
        "orderType=StopLimit",
        "the current endpoint does not define its price and trigger field combination",
    ),
    CurrentDocumentationGap::new(
        "/order/modifyorder",
        "orderType=Stop|StopLimit",
        "the current endpoint does not identify the wire field for the Stop trigger",
    ),
    CurrentDocumentationGap::new(
        "/order/dryrun",
        "orders[].orderType=Stop|StopLimit",
        "the current endpoint publishes no cross-field order grammar",
    ),
    CurrentDocumentationGap::new(
        "/order/placeoso",
        "orderType=Stop|StopLimit",
        "the current endpoint documents only Market and Limit parent combinations",
    ),
    CurrentDocumentationGap::new(
        "/order/placeoso",
        "bracket1.orderType=Stop|StopLimit",
        "the current endpoint documents only Market and Limit bracket combinations",
    ),
    CurrentDocumentationGap::new(
        "/order/placeoso",
        "bracket2.orderType=Stop|StopLimit",
        "the current endpoint documents only Market and Limit bracket combinations",
    ),
];

/// A current operation or field that cannot be represented safely because its
/// official wire grammar is absent from the pinned Partner contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CurrentDocumentationGap {
    endpoint: &'static str,
    field: &'static str,
    reason: &'static str,
}

impl CurrentDocumentationGap {
    pub(super) const fn new(
        endpoint: &'static str,
        field: &'static str,
        reason: &'static str,
    ) -> Self {
        Self {
            endpoint,
            field,
            reason,
        }
    }

    /// Returns the current Partner endpoint containing the gap.
    #[must_use]
    pub const fn endpoint(self) -> &'static str {
        self.endpoint
    }

    /// Returns the undocumented request field.
    #[must_use]
    pub const fn field(self) -> &'static str {
        self.field
    }

    /// Explains why the crate deliberately withholds the field or operation.
    #[must_use]
    pub const fn reason(self) -> &'static str {
        self.reason
    }
}

// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Current Partner customer, subscription, and batch mutation policy.

mod accounts;
mod batches;
mod documents;
mod profiles;
mod subscriptions;
mod validation;

pub use accounts::PartnerSubAccountRequest;
pub use batches::{
    EvaluationAccountBatchResult, EvaluationAccountCreation, EvaluationAccountsRequest,
    EvaluationUserBatchResult, EvaluationUserCreation, EvaluationUsersRequest,
};
pub use documents::{CustomerApplicationDocumentRequest, PartnerSubAccountDocumentRequest};
pub use profiles::{ContactInfoUpdateRequest, OrganizationMemberRequest};
pub use subscriptions::{MarketDataSubscriptionPurchase, ValidatedCancelEverything};

#[cfg(test)]
mod tests;

// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Request-aware completion policy for current Partner REST mutations.

mod alerts;
mod common;
pub mod customer;
mod entity_write;
pub(super) mod risk_control;
mod risks;
mod users;
mod workspace;

pub use entity_write::{
    CreateMarketDataSubscriptionRequest, CreatePoaContactRequest,
    CreateTradovateSubscriptionRequest, CreateUserPluginRequest,
    UpdateMarketDataSubscriptionRequest, UpdatePoaContactRequest, UpdateUserPluginRequest,
};

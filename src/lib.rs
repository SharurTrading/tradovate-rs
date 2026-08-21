// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Exact-decimal, asynchronous access to Tradovate REST APIs.
//!
//! This crate owns provider-native transport and protocol semantics. Consumers
//! remain responsible for canonical instrument identity, routing, risk, and
//! portfolio truth.
//!
//! ```no_run
//! use tradovate_client::{Client, DeviceId, Environment, auth::Credentials};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let credentials = Credentials::builder("user", "dedicated-api-password")
//!     .app_id("my-app")
//!     .app_version("1.0")
//!     .numeric_client_id(123)
//!     .secret("api-key-secret")
//!     .device_id(DeviceId::new("stable-device-id")?)
//!     .build()?;
//! let client = Client::builder(Environment::Demo).build()?;
//! let session = client.authenticate(&credentials).await?;
//! println!("authenticated user {}", session.user_id());
//! # Ok(())
//! # }
//! ```
#![forbid(unsafe_code)]

mod decimal;
mod environment;
mod error;
mod ids;
#[path = "realtime/control.rs"]
mod provider_control;
mod rate_limit;

pub mod api;
pub mod auth;
pub mod client;
#[cfg(test)]
mod realtime;

pub use client::{Client, ClientBuilder};
pub use environment::{EndpointSet, Environment};
pub use error::{ConfigError, Error, PenaltyTicket};
pub use ids::{
    AccountId, AccountSpec, ClientOrderId, CommandId, ContractId, ContractMaturityId, DeviceId,
    IdentifierError, OrderId, PositionId, Symbol, UserId,
};
pub use rust_decimal::Decimal;

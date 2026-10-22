// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Shared, bounded rolling-window request admission.

mod account;
mod admission;
mod cooldown;
mod governor;
mod window;

pub(crate) use admission::RateAdmission;
pub(crate) use governor::{HOUR, RateGovernor};

#[cfg(test)]
#[path = "tests.rs"]
mod tests;

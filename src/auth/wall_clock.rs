// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! The UTC clock used by the authenticated session.

use std::{fmt, sync::Arc};

use jiff::Timestamp;

/// Source of UTC instants for session expiry.
#[derive(Clone, Default)]
pub(crate) struct WallClock(Option<Arc<dyn Fn() -> Timestamp + Send + Sync>>);

impl WallClock {
    pub(crate) const SYSTEM: Self = Self(None);

    pub(crate) fn supplied(clock: impl Fn() -> Timestamp + Send + Sync + 'static) -> Self {
        Self(Some(Arc::new(clock)))
    }

    pub(super) fn now(&self) -> Timestamp {
        self.0.as_ref().map_or_else(Timestamp::now, |clock| clock())
    }
}

impl fmt::Debug for WallClock {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(if self.0.is_some() {
            "Supplied"
        } else {
            "System"
        })
    }
}

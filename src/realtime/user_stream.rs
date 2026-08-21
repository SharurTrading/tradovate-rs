// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Typed current Partner user bootstrap and `props` events.
//!
//! Contract reviewed 2026-08-22 against the current Partner user-sync and
//! architecture pages:
//! <https://partner.tradovate.com/overview/core-concepts/web-sockets/user-syncrequest>
//! and <https://partner.tradovate.com/overview/core-concepts/architecture-overview>,
//! plus the current cross-environment signature example in
//! <https://partner.tradovate.com/overview/prop-firm-management/create-and-manage-users>.
//! B2B multipart completion remains documentation-blocked.

pub(super) mod decode;
mod entity_batch;

pub use entity_batch::UserEntityBatch;

use super::ProviderCode;

/// A current or forward-compatible `props.eventType` value.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum PropertyEventType {
    /// A new provider entity was created.
    Created,
    /// An existing provider entity changed.
    Updated,
    /// A provider entity was deleted.
    Deleted,
    /// A bounded future provider operation.
    Unknown(ProviderCode),
}

/// One current typed `props` delta.
#[derive(Clone, Debug)]
pub struct PropertyEvent {
    pub(super) event_type: PropertyEventType,
    pub(super) entities: UserEntityBatch,
}

impl PropertyEvent {
    /// Returns whether entities were created, updated, or deleted.
    #[must_use]
    pub const fn event_type(&self) -> &PropertyEventType {
        &self.event_type
    }

    /// Returns typed provider entities.
    #[must_use]
    pub const fn entities(&self) -> &UserEntityBatch {
        &self.entities
    }
}

/// A complete validated single-response bootstrap snapshot.
#[derive(Clone, Debug)]
pub struct UserBootstrap {
    pub(super) entities: Box<[UserEntityBatch]>,
}

impl UserBootstrap {
    /// Returns typed entity collections from the initial snapshot.
    #[must_use]
    pub const fn entities(&self) -> &[UserEntityBatch] {
        &self.entities
    }

    /// Returns whether the bootstrap used the only currently implemented
    /// single-response profile.
    #[must_use]
    pub const fn is_single_response(&self) -> bool {
        true
    }
}

/// A typed user socket event.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum UserStreamEvent {
    /// The initial snapshot, published before co-batched deltas and readiness.
    Bootstrap(UserBootstrap),
    /// One or more `props` deltas in provider order.
    Properties(Box<[PropertyEvent]>),
}

#[cfg(test)]
#[path = "user_stream/tests.rs"]
mod tests;

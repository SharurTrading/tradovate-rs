// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Bounded Tradovate real-time protocol and single-generation connections.

mod actor;
mod codec;
mod config;
mod connection;
mod error;
mod market_data;
mod types;

pub use codec::{Error as CodecError, RequestId};
pub(crate) use codec::{Event, FrameCodec, Response, ServerFrame, ServerMessage};
pub use config::RealtimeConfig;
pub use connection::RealtimeConnection;
pub use error::RealtimeError;
pub use market_data::MarketDataChannel;
pub use types::{
    ConnectionId, DisconnectReason, RealtimeNotice, RealtimeState, ResyncReason, SocketKind,
};
pub(crate) use types::{RealtimeEvent, RealtimeEventPayload};

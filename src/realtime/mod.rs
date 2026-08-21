// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Production-public, bounded Tradovate realtime connections and typed events.
//!
//! Public payloads never expose raw JSON. The current Partner documentation
//! names B2B multipart synchronization and replay control/clock families without
//! publishing a safe completion or payload schema; see
//! [`DOCUMENTATION_BLOCKED_CAPABILITIES`] for that explicit boundary.

mod actor;
mod bounded;
mod chart;
mod codec;
mod config;
mod connection;
mod error;
mod event;
mod market_data;
mod types;
mod user_stream;
mod user_sync;

pub use chart::{
    Bar, BarPacket, ChartDescription, ChartElementUnit, ChartEvent, ChartPacket, ChartRequest,
    ChartRequestBuilder, ChartSubscription, ChartSubscriptionId, ChartTimeRange,
    ChartUnderlyingType, HistoricalChartId, RealtimeChartId, Tick, TickId, TickPacket,
};
pub use codec::{Error as CodecError, RequestId};
pub(crate) use codec::{Event, FrameCodec, Response, ServerFrame, ServerMessage};
pub use config::RealtimeConfig;
pub use connection::RealtimeConnection;
pub use error::{RealtimeError, RealtimeEventKind, RealtimePayloadError};
pub use event::{
    DOCUMENTATION_BLOCKED_CAPABILITIES, DocumentationBlockedCapability, DocumentationBlockedEvent,
    ProviderCode, RealtimeEventPayload, ShutdownEvent, ShutdownReason, UnmatchedResponse,
};
pub use market_data::{
    DepthLevel, DepthOfMarket, Histogram, MarketDataChannel, MarketDataEvent, MarketDataTarget,
    Quote, QuoteEntry, QuoteEntryKind, TradeDate,
};
pub use types::{
    ConnectionId, DisconnectReason, RealtimeEvent, RealtimeState, ResyncReason, SocketKind,
};
pub use user_stream::{
    PropertyEvent, PropertyEventType, UserBootstrap, UserEntityBatch, UserStreamEvent,
};
pub use user_sync::{UserSyncConfig, UserSyncEntityType, UserSyncShardBy, UserSyncSharding};

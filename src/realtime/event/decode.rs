// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Conversion from bounded private wire envelopes to public typed events.

use serde::Deserialize;
use serde_json::value::RawValue;

use super::{
    DocumentationBlockedCapability, DocumentationBlockedEvent, ProviderCode, RealtimeEventPayload,
    ShutdownEvent, ShutdownReason, UnmatchedResponse, validate_shutdown_explanation,
};
use crate::realtime::{
    Event, RealtimeError, RealtimeEventKind, RealtimePayloadError, Response, ServerMessage, chart,
    market_data, user_stream,
};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct WireShutdown {
    reason_code: String,
    #[serde(default)]
    reason: Option<String>,
}

pub(crate) fn bootstrap(response: &Response) -> Result<RealtimeEventPayload, RealtimeError> {
    user_stream::decode::bootstrap(response).map(RealtimeEventPayload::User)
}

pub(crate) fn message(message: ServerMessage) -> Result<RealtimeEventPayload, RealtimeError> {
    match message {
        ServerMessage::Response(response) => Ok(RealtimeEventPayload::UnmatchedResponse(
            UnmatchedResponse::new(response.request_id(), response.status()),
        )),
        ServerMessage::Event(event) => event_payload(event),
        ServerMessage::Unknown(raw) => {
            drop(raw);
            let kind = ProviderCode::from_wire("unclassified-message".to_owned()).ok_or(
                RealtimeError::InvalidEvent {
                    kind: RealtimeEventKind::Unsupported,
                    reason: RealtimePayloadError::InvalidValue,
                },
            )?;
            Ok(RealtimeEventPayload::Unsupported(kind))
        }
    }
}

fn event_payload(event: Event) -> Result<RealtimeEventPayload, RealtimeError> {
    match event {
        Event::Properties(data) => {
            user_stream::decode::properties(data.as_deref()).map(RealtimeEventPayload::User)
        }
        Event::Shutdown(data) => shutdown(data.as_deref()).map(RealtimeEventPayload::Shutdown),
        Event::MarketData(data) => {
            market_data::decode::event(data.as_deref()).map(RealtimeEventPayload::MarketData)
        }
        Event::Chart(data) => {
            chart::decode::event(data.as_deref()).map(RealtimeEventPayload::Chart)
        }
        Event::Clock(data) => {
            drop(data);
            Ok(RealtimeEventPayload::DocumentationBlocked(
                DocumentationBlockedEvent::new(DocumentationBlockedCapability::ReplayClockPayload),
            ))
        }
        Event::Unknown { kind, raw } => {
            drop(raw);
            ProviderCode::from_wire(kind)
                .map(RealtimeEventPayload::Unsupported)
                .ok_or(RealtimeError::InvalidEvent {
                    kind: RealtimeEventKind::Unsupported,
                    reason: RealtimePayloadError::LimitExceeded,
                })
        }
    }
}

fn shutdown(data: Option<&RawValue>) -> Result<ShutdownEvent, RealtimeError> {
    let data = data.ok_or_else(|| invalid_shutdown(RealtimePayloadError::MissingData))?;
    let wire = serde_json::from_str::<WireShutdown>(data.get())
        .map_err(|_| invalid_shutdown(RealtimePayloadError::Malformed))?;
    if wire
        .reason
        .as_deref()
        .is_some_and(|reason| !validate_shutdown_explanation(reason))
    {
        return Err(invalid_shutdown(RealtimePayloadError::LimitExceeded));
    }
    let reason_code = match wire.reason_code.as_str() {
        "Maintenance" => ShutdownReason::Maintenance,
        "ConnectionQuotaReached" => ShutdownReason::ConnectionQuotaReached,
        "IPQuotaReached" => ShutdownReason::IpQuotaReached,
        _ => ShutdownReason::Unknown(
            ProviderCode::from_wire(wire.reason_code)
                .ok_or_else(|| invalid_shutdown(RealtimePayloadError::InvalidValue))?,
        ),
    };
    Ok(ShutdownEvent {
        reason_code,
        explanation: wire.reason,
    })
}

fn invalid_shutdown(reason: RealtimePayloadError) -> RealtimeError {
    RealtimeError::InvalidEvent {
        kind: RealtimeEventKind::Shutdown,
        reason,
    }
}

// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Encoding and decoding for Tradovate's SockJS-derived WebSocket framing.

mod decode;

use std::fmt;

use serde_json::value::RawValue;
use thiserror::Error;

/// A client request identifier, unique within one WebSocket connection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RequestId(u64);

impl RequestId {
    /// Creates a request identifier.
    #[must_use]
    pub(crate) const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the integer carried on the wire.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl fmt::Display for RequestId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

/// A response to a client request.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct Response {
    request_id: RequestId,
    status: u16,
    data: Option<Box<RawValue>>,
}

impl Response {
    /// Returns the identifier of the corresponding client request.
    #[must_use]
    pub const fn request_id(&self) -> RequestId {
        self.request_id
    }

    /// Returns the HTTP-style response status.
    #[must_use]
    pub const fn status(&self) -> u16 {
        self.status
    }

    /// Returns the optional response payload without imposing domain semantics.
    #[must_use]
    pub fn data(&self) -> Option<&RawValue> {
        self.data.as_deref()
    }
}

/// A server-pushed Tradovate event.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Event {
    /// An entity was created, updated, or deleted.
    Properties(Option<Box<RawValue>>),
    /// The server is preparing to close the connection.
    Shutdown(Option<Box<RawValue>>),
    /// A market-data update.
    MarketData(Option<Box<RawValue>>),
    /// A chart update.
    Chart(Option<Box<RawValue>>),
    /// A market-replay clock update.
    Clock(Option<Box<RawValue>>),
    /// A forward-compatible event whose kind is not known by this crate.
    Unknown {
        /// The value of the event's `e` field.
        kind: String,
        /// The complete event object, including fields unknown to this crate.
        raw: Box<RawValue>,
    },
}

/// One object carried inside an `a[...]` server frame.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum ServerMessage {
    /// A response correlated to a client request.
    Response(Response),
    /// An unsolicited server event.
    Event(Event),
    /// A forward-compatible object that is neither a response nor an event.
    Unknown(Box<RawValue>),
}

/// A decoded SockJS-derived server frame.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum ServerFrame {
    /// The server accepted the WebSocket connection.
    Open,
    /// A server heartbeat.
    Heartbeat,
    /// One or more response or event objects.
    Messages(Vec<ServerMessage>),
    /// The server closed the logical `SockJS` session.
    Close {
        /// `SockJS` close code.
        code: u16,
        /// Human-readable close reason.
        reason: String,
    },
}

/// Failures produced while encoding or decoding real-time frames.
///
/// Variants contain only structural metadata. Raw frames, request bodies, and
/// access tokens are never retained or rendered by [`Debug`](fmt::Debug) or
/// [`Display`](fmt::Display).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
#[non_exhaustive]
pub enum Error {
    /// A zero-byte frame limit cannot admit any protocol frame.
    #[error("maximum frame size must be greater than zero")]
    ZeroFrameLimit,
    /// A zero-message limit cannot admit a response or event object.
    #[error("maximum messages per frame must be greater than zero")]
    ZeroMessageLimit,
    /// A frame exceeds the configured hard byte limit.
    #[error("frame is {actual_bytes} bytes, exceeding the {max_bytes}-byte limit")]
    FrameTooLarge {
        /// Encoded frame length.
        actual_bytes: usize,
        /// Configured hard limit.
        max_bytes: usize,
    },
    /// A server message frame contains too many response/event objects.
    #[error(
        "server frame contains {actual_messages} messages, exceeding the {max_messages}-message limit"
    )]
    TooManyMessages {
        /// Number of top-level objects in the server message frame.
        actual_messages: usize,
        /// Configured hard message-count limit.
        max_messages: usize,
    },
    /// The server sent an empty text frame.
    #[error("server frame is empty")]
    EmptyServerFrame,
    /// The server frame used an unsupported prefix.
    #[error("unsupported server frame kind {kind:?}")]
    UnsupportedServerFrame {
        /// Unsupported one-character prefix.
        kind: char,
    },
    /// An open or heartbeat frame unexpectedly carried a payload.
    #[error("server frame kind {kind:?} must not carry a payload")]
    UnexpectedPayload {
        /// Frame prefix whose payload was unexpected.
        kind: char,
    },
    /// A JSON-bearing server frame was malformed.
    #[error("invalid JSON in server frame kind {kind:?} at line {line}, column {column}")]
    InvalidJson {
        /// Frame prefix whose payload failed to decode.
        kind: char,
        /// One-based line reported by `serde_json`.
        line: usize,
        /// One-based column reported by `serde_json`.
        column: usize,
    },
    /// A response carried no unsigned integer request identifier.
    #[error("response field `i` must be an unsigned integer")]
    InvalidResponseRequestId,
    /// A response status was absent, negative, fractional, or outside `u16`.
    #[error("response field `s` must be an unsigned 16-bit integer")]
    InvalidResponseStatus,
    /// A client endpoint would break the four-field framing.
    #[error("client endpoint must not contain CR or LF")]
    EndpointContainsLineBreak,
    /// A client query would break the four-field framing.
    #[error("client query must not contain CR or LF")]
    QueryContainsLineBreak,
}

/// A stateless encoder/decoder with a hard per-frame byte limit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrameCodec {
    max_frame_bytes: usize,
    max_messages_per_frame: usize,
}

impl FrameCodec {
    /// Creates a codec with explicit byte and decoded-message hard limits.
    ///
    /// # Errors
    ///
    /// Returns [`Error::ZeroFrameLimit`] or [`Error::ZeroMessageLimit`] when a
    /// supplied limit is zero.
    pub const fn new(max_frame_bytes: usize, max_messages_per_frame: usize) -> Result<Self, Error> {
        if max_frame_bytes == 0 {
            Err(Error::ZeroFrameLimit)
        } else if max_messages_per_frame == 0 {
            Err(Error::ZeroMessageLimit)
        } else {
            Ok(Self {
                max_frame_bytes,
                max_messages_per_frame,
            })
        }
    }

    /// Decodes one complete Tradovate server text frame.
    ///
    /// # Errors
    ///
    /// Returns an [`enum@Error`] for oversized, empty, unsupported, or malformed
    /// frames, and for response objects whose correlation fields are invalid.
    pub fn decode(self, frame: &str) -> Result<ServerFrame, Error> {
        self.ensure_size(frame.len())?;

        let mut characters = frame.chars();
        let kind = characters.next().ok_or(Error::EmptyServerFrame)?;
        let payload = characters.as_str();

        match kind {
            'o' => empty_payload(kind, payload, ServerFrame::Open),
            'h' => empty_payload(kind, payload, ServerFrame::Heartbeat),
            'a' => decode::messages(payload, self.max_messages_per_frame),
            'c' => decode::close(payload),
            _ => Err(Error::UnsupportedServerFrame { kind }),
        }
    }

    /// Encodes the exact four-field Tradovate client request frame.
    ///
    /// The body is the final field and may contain line breaks. The endpoint and
    /// query may not, because they precede later fields.
    ///
    /// # Errors
    ///
    /// Returns an [`enum@Error`] when the endpoint or query contains CR/LF, or when
    /// the encoded frame exceeds the configured byte limit.
    pub fn encode_request(
        self,
        endpoint: &str,
        request_id: RequestId,
        query: &str,
        body: &str,
    ) -> Result<String, Error> {
        validate_client_fields(endpoint, query)?;

        let request_id = request_id.to_string();
        let actual_bytes = endpoint
            .len()
            .saturating_add(request_id.len())
            .saturating_add(query.len())
            .saturating_add(body.len())
            .saturating_add(3);
        self.ensure_size(actual_bytes)?;

        Ok(format!("{endpoint}\n{request_id}\n{query}\n{body}"))
    }

    /// Validates a request before its values are cloned into the actor queue.
    ///
    /// The byte calculation reserves the full decimal width of a future `u64`
    /// request identifier, so an accepted request remains encodable after the
    /// actor assigns its identifier.
    pub(super) fn validate_request(
        self,
        endpoint: &str,
        query: &str,
        body: &str,
    ) -> Result<(), Error> {
        const MAX_REQUEST_ID_DIGITS: usize = 20;

        validate_client_fields(endpoint, query)?;
        let actual_bytes = endpoint
            .len()
            .saturating_add(MAX_REQUEST_ID_DIGITS)
            .saturating_add(query.len())
            .saturating_add(body.len())
            .saturating_add(3);
        self.ensure_size(actual_bytes)
    }

    /// Encodes the special `authorize` request without retaining the token.
    ///
    /// # Errors
    ///
    /// Returns [`Error::FrameTooLarge`] when the encoded request exceeds the
    /// configured byte limit. The error contains lengths only, never the token.
    pub fn encode_authorize(
        self,
        request_id: RequestId,
        access_token: &str,
    ) -> Result<String, Error> {
        self.encode_request("authorize", request_id, "", access_token)
    }

    /// Encodes the client heartbeat frame, `[]`.
    ///
    /// # Errors
    ///
    /// Returns [`Error::FrameTooLarge`] when the configured limit is below two
    /// bytes.
    pub fn encode_heartbeat(self) -> Result<String, Error> {
        self.ensure_size(2)?;
        Ok("[]".to_owned())
    }

    fn ensure_size(self, actual_bytes: usize) -> Result<(), Error> {
        if actual_bytes > self.max_frame_bytes {
            Err(Error::FrameTooLarge {
                actual_bytes,
                max_bytes: self.max_frame_bytes,
            })
        } else {
            Ok(())
        }
    }
}

fn empty_payload(kind: char, payload: &str, frame: ServerFrame) -> Result<ServerFrame, Error> {
    if payload.is_empty() {
        Ok(frame)
    } else {
        Err(Error::UnexpectedPayload { kind })
    }
}

fn validate_client_fields(endpoint: &str, query: &str) -> Result<(), Error> {
    if contains_line_break(endpoint) {
        return Err(Error::EndpointContainsLineBreak);
    }
    if contains_line_break(query) {
        return Err(Error::QueryContainsLineBreak);
    }
    Ok(())
}

fn contains_line_break(value: &str) -> bool {
    value.bytes().any(|byte| matches!(byte, b'\r' | b'\n'))
}

#[cfg(test)]
mod tests;

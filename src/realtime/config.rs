// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Bounded real-time connection configuration.

use std::time::Duration;

use super::RealtimeError;

const DEFAULT_MAX_FRAME_BYTES: usize = 8 * 1024 * 1024;
const DEFAULT_MAX_MESSAGES_PER_FRAME: usize = 65_536;
const DEFAULT_MAX_PENDING_REQUESTS: usize = 4_096;
const DEFAULT_MAX_COLLECTION_ENTRIES: usize = 65_536;
const DEFAULT_COMMAND_CAPACITY: usize = 4_096;
const DEFAULT_EVENT_CAPACITY: usize = 65_536;
const DEFAULT_REQUEST_TIMEOUT: Duration = Duration::from_secs(10);
const DEFAULT_WRITE_TIMEOUT: Duration = Duration::from_secs(10);
const DEFAULT_LIVENESS_TIMEOUT: Duration = Duration::from_secs(10);
/// Resource ceilings and timeouts for one real-time socket generation.
#[must_use]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RealtimeConfig {
    max_frame_bytes: usize,
    max_messages_per_frame: usize,
    max_pending_requests: usize,
    max_collection_entries: usize,
    command_capacity: usize,
    event_capacity: usize,
    request_timeout: Duration,
    write_timeout: Duration,
    liveness_timeout: Duration,
}

impl RealtimeConfig {
    /// Sets the socket transmission deadline (default ten seconds), independent
    /// of caller request timeouts. An unfinished write is a transport failure.
    pub const fn write_timeout(mut self, timeout: Duration) -> Self {
        self.write_timeout = timeout;
        self
    }

    /// Returns the socket write deadline.
    #[must_use]
    pub const fn write_deadline(&self) -> Duration {
        self.write_timeout
    }

    /// Sets the maximum entries in each decoded array or object (default 65,536).
    /// This is a configurable client memory control, not a provider quota.
    /// Exceeding it on an active socket retains a gap without closing the socket.
    pub const fn max_collection_entries(mut self, limit: usize) -> Self {
        self.max_collection_entries = limit;
        self
    }

    /// Returns the per-collection decoding limit.
    #[must_use]
    pub const fn collection_entries_limit(&self) -> usize {
        self.max_collection_entries
    }

    /// Sets the hard encoded size limit for every inbound and outbound frame.
    /// Defaults to 8 MiB. This is a caller resource
    /// policy, not a documented provider payload limit.
    pub const fn max_frame_bytes(mut self, bytes: usize) -> Self {
        self.max_frame_bytes = bytes;
        self
    }

    /// Sets the maximum number of objects accepted in one inbound message frame.
    pub const fn max_messages_per_frame(mut self, limit: usize) -> Self {
        self.max_messages_per_frame = limit;
        self
    }

    /// Sets the maximum outstanding requests, not the number of subscriptions.
    /// Saturation rejects new admission locally; completed requests free slots.
    pub const fn max_pending_requests(mut self, limit: usize) -> Self {
        self.max_pending_requests = limit;
        self
    }

    /// Sets the caller-to-actor queue capacity. Admission waits asynchronously
    /// up to the request deadline; expiry before enqueueing proves not sent.
    pub const fn command_capacity(mut self, capacity: usize) -> Self {
        self.command_capacity = capacity;
        self
    }

    /// Sets the event capacity. Saturation retains a continuity gap after the
    /// accepted prefix and fences data until the consumer acknowledges recovery.
    pub const fn event_capacity(mut self, capacity: usize) -> Self {
        self.event_capacity = capacity;
        self
    }

    /// Sets the deadline for connection setup, authorization, and requests.
    pub const fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = timeout;
        self
    }

    /// Sets the idle interval before an active WebSocket ping and its pong wait.
    /// Silence alone never terminates the socket. Only a missing matching pong
    /// after a transmitted probe fails liveness. Processing an already-buffered
    /// message grants at most one read grace interval per probe; further batches
    /// cannot renew it. Outgoing heartbeats remain independent.
    pub const fn liveness_timeout(mut self, timeout: Duration) -> Self {
        self.liveness_timeout = timeout;
        self
    }

    /// Returns the hard encoded frame limit.
    #[must_use]
    pub const fn frame_bytes_limit(&self) -> usize {
        self.max_frame_bytes
    }

    /// Returns the per-frame decoded-message limit.
    #[must_use]
    pub const fn messages_per_frame_limit(&self) -> usize {
        self.max_messages_per_frame
    }

    /// Returns the pending-request limit.
    #[must_use]
    pub const fn pending_requests_limit(&self) -> usize {
        self.max_pending_requests
    }

    /// Returns the command channel capacity.
    #[must_use]
    pub const fn command_channel_capacity(&self) -> usize {
        self.command_capacity
    }

    /// Returns the event channel capacity.
    #[must_use]
    pub const fn event_channel_capacity(&self) -> usize {
        self.event_capacity
    }

    /// Returns the request and setup timeout.
    #[must_use]
    pub const fn request_deadline(&self) -> Duration {
        self.request_timeout
    }

    /// Returns the idle probe interval and pong timeout.
    #[must_use]
    pub const fn liveness_deadline(&self) -> Duration {
        self.liveness_timeout
    }

    pub(super) fn validate(self) -> Result<Self, RealtimeError> {
        positive(self.max_frame_bytes, "max_frame_bytes")?;
        if self.max_frame_bytes == usize::MAX {
            return Err(RealtimeError::InvalidConfiguration {
                field: "max_frame_bytes",
                reason: "must be a finite transport byte limit",
            });
        }
        positive(self.max_messages_per_frame, "max_messages_per_frame")?;
        positive(self.max_collection_entries, "max_collection_entries")?;
        positive(self.max_pending_requests, "max_pending_requests")?;
        positive(self.command_capacity, "command_capacity")?;
        positive(self.event_capacity, "event_capacity")?;
        for (value, field) in [
            (self.command_capacity, "command_capacity"),
            (self.event_capacity, "event_capacity"),
            (self.max_pending_requests, "max_pending_requests"),
        ] {
            if value > tokio::sync::Semaphore::MAX_PERMITS {
                return Err(RealtimeError::InvalidConfiguration {
                    field,
                    reason: "exceeds Tokio's representable permit range",
                });
            }
        }
        if self.max_collection_entries > isize::MAX.cast_unsigned() {
            return Err(RealtimeError::InvalidConfiguration {
                field: "max_collection_entries",
                reason: "exceeds the representable decoded collection length",
            });
        }
        duration(self.write_timeout, "write_timeout")?;
        duration(self.request_timeout, "request_timeout")?;
        duration(self.liveness_timeout, "liveness_timeout")?;
        Ok(self)
    }
}

impl Default for RealtimeConfig {
    fn default() -> Self {
        Self {
            max_frame_bytes: DEFAULT_MAX_FRAME_BYTES,
            max_messages_per_frame: DEFAULT_MAX_MESSAGES_PER_FRAME,
            max_pending_requests: DEFAULT_MAX_PENDING_REQUESTS,
            max_collection_entries: DEFAULT_MAX_COLLECTION_ENTRIES,
            command_capacity: DEFAULT_COMMAND_CAPACITY,
            event_capacity: DEFAULT_EVENT_CAPACITY,
            request_timeout: DEFAULT_REQUEST_TIMEOUT,
            write_timeout: DEFAULT_WRITE_TIMEOUT,
            liveness_timeout: DEFAULT_LIVENESS_TIMEOUT,
        }
    }
}

fn positive(value: usize, field: &'static str) -> Result<(), RealtimeError> {
    if value == 0 {
        Err(RealtimeError::InvalidConfiguration {
            field,
            reason: "must be positive",
        })
    } else {
        Ok(())
    }
}

fn duration(value: Duration, field: &'static str) -> Result<(), RealtimeError> {
    if value.is_zero() {
        return Err(RealtimeError::InvalidConfiguration {
            field,
            reason: "must be positive",
        });
    }
    if tokio::time::Instant::now().checked_add(value).is_none() {
        return Err(RealtimeError::InvalidConfiguration {
            field,
            reason: "is too large for a monotonic deadline",
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_bounded_and_valid() {
        assert!(RealtimeConfig::default().validate().is_ok());
    }

    #[test]
    fn unlimited_transport_sentinel_is_rejected() {
        assert!(matches!(
            RealtimeConfig::default()
                .max_frame_bytes(usize::MAX)
                .validate(),
            Err(RealtimeError::InvalidConfiguration {
                field: "max_frame_bytes",
                ..
            })
        ));
    }

    #[test]
    fn every_zero_resource_limit_is_rejected() {
        let invalid = [
            RealtimeConfig::default().max_frame_bytes(0),
            RealtimeConfig::default().max_messages_per_frame(0),
            RealtimeConfig::default().max_collection_entries(0),
            RealtimeConfig::default().max_pending_requests(0),
            RealtimeConfig::default().command_capacity(0),
            RealtimeConfig::default().event_capacity(0),
        ];
        assert!(invalid.into_iter().all(|config| config.validate().is_err()));
    }

    #[test]
    fn zero_timeouts_are_rejected() {
        let zero = Duration::ZERO;
        assert!(
            RealtimeConfig::default()
                .write_timeout(zero)
                .validate()
                .is_err()
        );
        assert!(
            RealtimeConfig::default()
                .request_timeout(zero)
                .validate()
                .is_err()
        );
        assert!(
            RealtimeConfig::default()
                .liveness_timeout(zero)
                .validate()
                .is_err()
        );
    }

    #[test]
    fn unrepresentable_capacities_are_rejected() {
        assert!(
            RealtimeConfig::default()
                .command_capacity(usize::MAX)
                .validate()
                .is_err()
        );
        assert!(
            RealtimeConfig::default()
                .event_capacity(usize::MAX)
                .validate()
                .is_err()
        );
        assert!(
            RealtimeConfig::default()
                .max_pending_requests(usize::MAX)
                .validate()
                .is_err()
        );
        assert!(
            RealtimeConfig::default()
                .request_timeout(Duration::MAX)
                .validate()
                .is_err()
        );
    }
}

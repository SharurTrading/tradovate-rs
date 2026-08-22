// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Bounded real-time connection configuration.

use std::time::Duration;

use super::RealtimeError;

const DEFAULT_MAX_FRAME_BYTES: usize = 8 * 1024 * 1024;
const DEFAULT_MAX_MESSAGES_PER_FRAME: usize = 4_096;
const DEFAULT_MAX_PENDING_REQUESTS: usize = 32;
const DEFAULT_COMMAND_CAPACITY: usize = 8;
const DEFAULT_EVENT_CAPACITY: usize = 32;
const DEFAULT_REQUEST_TIMEOUT: Duration = Duration::from_secs(10);
const DEFAULT_LIVENESS_TIMEOUT: Duration = Duration::from_secs(10);
const MAX_FRAME_BYTES: usize = 64 * 1024 * 1024;
const MAX_MESSAGES_PER_FRAME: usize = 65_536;
const MAX_PENDING_REQUESTS: usize = 4_096;
const MAX_CHANNEL_CAPACITY: usize = 65_536;
const MAX_COMMAND_BUFFER_BYTES: usize = 64 * 1024 * 1024;
const MAX_EVENT_BUFFER_BYTES: usize = 256 * 1024 * 1024;
const MAX_PENDING_RESPONSE_BYTES: usize = 256 * 1024 * 1024;
const MAX_TIMEOUT: Duration = Duration::from_hours(24);

/// Resource ceilings and timeouts for one real-time socket generation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RealtimeConfig {
    max_frame_bytes: usize,
    max_messages_per_frame: usize,
    max_pending_requests: usize,
    command_capacity: usize,
    event_capacity: usize,
    request_timeout: Duration,
    liveness_timeout: Duration,
}

impl RealtimeConfig {
    /// Sets the hard encoded size limit for every inbound and outbound frame.
    /// Validation also checks this limit against each queue capacity so their
    /// worst-case aggregate byte budgets remain bounded.
    #[must_use]
    pub const fn max_frame_bytes(mut self, bytes: usize) -> Self {
        self.max_frame_bytes = bytes;
        self
    }

    /// Sets the maximum number of objects accepted in one inbound message frame.
    #[must_use]
    pub const fn max_messages_per_frame(mut self, limit: usize) -> Self {
        self.max_messages_per_frame = limit;
        self
    }

    /// Sets the maximum number of requests awaiting responses. Validation
    /// rejects values whose worst-case response bytes exceed 256 MiB.
    #[must_use]
    pub const fn max_pending_requests(mut self, limit: usize) -> Self {
        self.max_pending_requests = limit;
        self
    }

    /// Sets the bounded caller-to-actor command capacity. Validation rejects
    /// values whose worst-case queued request bytes exceed 64 MiB.
    #[must_use]
    pub const fn command_capacity(mut self, capacity: usize) -> Self {
        self.command_capacity = capacity;
        self
    }

    /// Sets the bounded actor-to-caller event capacity. Validation rejects
    /// values whose worst-case queued event bytes exceed 256 MiB.
    #[must_use]
    pub const fn event_capacity(mut self, capacity: usize) -> Self {
        self.event_capacity = capacity;
        self
    }

    /// Sets the deadline for connection setup, authorization, and requests.
    #[must_use]
    pub const fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = timeout;
        self
    }

    /// Sets the maximum interval without any inbound socket traffic.
    #[must_use]
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

    /// Returns the inbound liveness timeout.
    #[must_use]
    pub const fn liveness_deadline(&self) -> Duration {
        self.liveness_timeout
    }

    pub(super) fn validate(self) -> Result<Self, RealtimeError> {
        positive(self.max_frame_bytes, "max_frame_bytes")?;
        positive(self.max_messages_per_frame, "max_messages_per_frame")?;
        positive(self.max_pending_requests, "max_pending_requests")?;
        positive(self.command_capacity, "command_capacity")?;
        positive(self.event_capacity, "event_capacity")?;
        maximum(self.max_frame_bytes, MAX_FRAME_BYTES, "max_frame_bytes")?;
        maximum(
            self.max_messages_per_frame,
            MAX_MESSAGES_PER_FRAME,
            "max_messages_per_frame",
        )?;
        maximum(
            self.max_pending_requests,
            MAX_PENDING_REQUESTS,
            "max_pending_requests",
        )?;
        maximum(
            self.command_capacity,
            MAX_CHANNEL_CAPACITY,
            "command_capacity",
        )?;
        maximum(self.event_capacity, MAX_CHANNEL_CAPACITY, "event_capacity")?;
        aggregate_budget(
            self.max_frame_bytes,
            self.command_capacity,
            MAX_COMMAND_BUFFER_BYTES,
            "max_frame_bytes * command_capacity",
            "exceeds the 64 MiB aggregate command budget",
        )?;
        aggregate_budget(
            self.max_frame_bytes,
            self.event_capacity,
            MAX_EVENT_BUFFER_BYTES,
            "max_frame_bytes * event_capacity",
            "exceeds the 256 MiB aggregate event budget",
        )?;
        aggregate_budget(
            self.max_frame_bytes,
            self.max_pending_requests,
            MAX_PENDING_RESPONSE_BYTES,
            "max_frame_bytes * max_pending_requests",
            "exceeds the 256 MiB aggregate pending-response budget",
        )?;
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
            command_capacity: DEFAULT_COMMAND_CAPACITY,
            event_capacity: DEFAULT_EVENT_CAPACITY,
            request_timeout: DEFAULT_REQUEST_TIMEOUT,
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
    if value > MAX_TIMEOUT {
        return Err(RealtimeError::InvalidConfiguration {
            field,
            reason: "exceeds the 24-hour hard maximum",
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

fn maximum(value: usize, maximum: usize, field: &'static str) -> Result<(), RealtimeError> {
    if value > maximum {
        Err(RealtimeError::InvalidConfiguration {
            field,
            reason: "exceeds the hard safety maximum",
        })
    } else {
        Ok(())
    }
}

fn aggregate_budget(
    item_bytes: usize,
    capacity: usize,
    maximum: usize,
    field: &'static str,
    reason: &'static str,
) -> Result<(), RealtimeError> {
    let total = item_bytes
        .checked_mul(capacity)
        .ok_or(RealtimeError::InvalidConfiguration {
            field,
            reason: "aggregate byte calculation overflowed",
        })?;
    if total > maximum {
        Err(RealtimeError::InvalidConfiguration { field, reason })
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_bounded_and_valid() {
        assert!(RealtimeConfig::default().validate().is_ok());
    }

    #[test]
    fn every_zero_resource_limit_is_rejected() {
        let invalid = [
            RealtimeConfig::default().max_frame_bytes(0),
            RealtimeConfig::default().max_messages_per_frame(0),
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
    fn oversized_resource_requests_are_rejected() {
        let invalid = [
            RealtimeConfig::default().max_frame_bytes(MAX_FRAME_BYTES + 1),
            RealtimeConfig::default().max_messages_per_frame(MAX_MESSAGES_PER_FRAME + 1),
            RealtimeConfig::default().max_pending_requests(MAX_PENDING_REQUESTS + 1),
            RealtimeConfig::default().command_capacity(MAX_CHANNEL_CAPACITY + 1),
            RealtimeConfig::default().event_capacity(MAX_CHANNEL_CAPACITY + 1),
        ];
        assert!(invalid.into_iter().all(|config| config.validate().is_err()));
    }

    #[test]
    fn timeouts_above_one_day_are_rejected() {
        let oversized = MAX_TIMEOUT + Duration::from_nanos(1);
        assert!(
            RealtimeConfig::default()
                .request_timeout(oversized)
                .validate()
                .is_err()
        );
        assert!(
            RealtimeConfig::default()
                .liveness_timeout(oversized)
                .validate()
                .is_err()
        );
    }

    #[test]
    fn aggregate_buffer_budgets_reject_individually_valid_cross_products() {
        let invalid = [
            RealtimeConfig::default().command_capacity(DEFAULT_COMMAND_CAPACITY + 1),
            RealtimeConfig::default().event_capacity(DEFAULT_EVENT_CAPACITY + 1),
            RealtimeConfig::default().max_pending_requests(DEFAULT_MAX_PENDING_REQUESTS + 1),
        ];
        assert!(invalid.into_iter().all(|config| config.validate().is_err()));
    }
}

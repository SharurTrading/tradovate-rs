// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Bounded request correlation and deadline ownership.

use std::{collections::HashMap, future};

use tokio::{sync::oneshot, time::Instant};

use crate::realtime::{RealtimeError, RequestId, Response};

struct Pending {
    endpoint: &'static str,
    deadline: Instant,
    reply: oneshot::Sender<Result<Response, RealtimeError>>,
}

pub(super) enum PendingReply {
    Active {
        endpoint: &'static str,
        reply: oneshot::Sender<Result<Response, RealtimeError>>,
    },
    Expired(oneshot::Sender<Result<Response, RealtimeError>>),
}

pub(super) struct PendingRequests {
    entries: HashMap<RequestId, Pending>,
    next_request_id: u64,
}

pub(super) async fn wait_for_deadline(deadline: Option<Instant>) {
    match deadline {
        Some(deadline) => tokio::time::sleep_until(deadline).await,
        None => future::pending().await,
    }
}

impl PendingRequests {
    pub(super) fn with_capacity(capacity: usize, next_request_id: u64) -> Self {
        Self {
            entries: HashMap::with_capacity(capacity),
            next_request_id,
        }
    }

    pub(super) fn len(&self) -> usize {
        self.entries.len()
    }

    pub(super) fn allocate_request_id(&mut self) -> Result<RequestId, RealtimeError> {
        let request_id = RequestId::new(self.next_request_id);
        self.next_request_id = self
            .next_request_id
            .checked_add(1)
            .ok_or(RealtimeError::RequestIdExhausted)?;
        Ok(request_id)
    }

    pub(super) fn insert(
        &mut self,
        request_id: RequestId,
        endpoint: &'static str,
        deadline: Instant,
        reply: oneshot::Sender<Result<Response, RealtimeError>>,
    ) {
        self.entries.insert(
            request_id,
            Pending {
                endpoint,
                deadline,
                reply,
            },
        );
    }

    pub(super) fn remove_for_response(
        &mut self,
        request_id: RequestId,
        observed_at: Instant,
    ) -> Option<PendingReply> {
        self.entries.remove(&request_id).map(|pending| {
            if pending.deadline <= observed_at {
                PendingReply::Expired(pending.reply)
            } else {
                PendingReply::Active {
                    endpoint: pending.endpoint,
                    reply: pending.reply,
                }
            }
        })
    }

    pub(super) fn next_deadline(&self) -> Option<Instant> {
        self.entries.values().map(|pending| pending.deadline).min()
    }

    pub(super) fn expire(&mut self) -> Option<RequestId> {
        let now = Instant::now();
        let expired = self
            .entries
            .iter()
            .filter_map(|(request_id, pending)| (pending.deadline <= now).then_some(*request_id))
            .collect::<Vec<_>>();
        let first = expired.first().copied();
        for request_id in expired {
            if let Some(pending) = self.entries.remove(&request_id) {
                drop(
                    pending
                        .reply
                        .send(Err(RealtimeError::RequestTimeout { request_id })),
                );
            }
        }
        first
    }

    pub(super) fn drain_uncertain(&mut self) {
        for (request_id, pending) in self.entries.drain() {
            drop(
                pending
                    .reply
                    .send(Err(RealtimeError::RequestOutcomeUncertain { request_id })),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    #[test]
    fn response_observed_at_deadline_is_expired() {
        let mut pending = PendingRequests::with_capacity(1, 2);
        let request_id = RequestId::new(2);
        let deadline = Instant::now();
        let (reply, _response) = oneshot::channel();
        pending.insert(request_id, "fixture", deadline, reply);

        assert!(matches!(
            pending.remove_for_response(request_id, deadline),
            Some(PendingReply::Expired(_))
        ));
    }

    #[test]
    fn response_observed_before_deadline_is_active() {
        let mut pending = PendingRequests::with_capacity(1, 2);
        let request_id = RequestId::new(2);
        let observed_at = Instant::now();
        let deadline = observed_at + Duration::from_secs(1);
        let (reply, _response) = oneshot::channel();
        pending.insert(request_id, "fixture", deadline, reply);

        assert!(matches!(
            pending.remove_for_response(request_id, observed_at),
            Some(PendingReply::Active {
                endpoint: "fixture",
                ..
            })
        ));
    }
}

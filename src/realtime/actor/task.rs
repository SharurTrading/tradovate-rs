// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Termination evidence after explicit destruction of the socket-producing future.

use super::{ActorInput, run};
use crate::realtime::{
    ConnectionId, RealtimeError, RealtimeState, admission::Admission, delivery::Delivery,
};
use std::{
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};
use tokio::sync::watch;
use tokio_util::sync::CancellationToken;

pub(in crate::realtime) fn tracked(
    input: ActorInput,
) -> impl Future<Output = Result<(), RealtimeError>> + Send {
    Tracked {
        connection_id: input.connection_id,
        admission: Arc::clone(&input.admission),
        events: Arc::clone(&input.events),
        state: input.state.clone(),
        cancellation: input.cancellation.clone(),
        future: Some(Box::pin(run(input))),
        result: None,
    }
}

struct Tracked<F> {
    future: Option<Pin<Box<F>>>,
    result: Option<Result<(), RealtimeError>>,
    connection_id: ConnectionId,
    admission: Arc<Admission>,
    events: Arc<Delivery>,
    state: watch::Sender<RealtimeState>,
    cancellation: CancellationToken,
}

impl<F> Tracked<F> {
    fn finish(&mut self, result: Result<(), RealtimeError>) {
        self.result = Some(result);
        self.admission.end();
        self.cancellation.cancel();
        if self.events.end_if_absent(result)
            && matches!(*self.state.borrow(), RealtimeState::Ready { .. })
        {
            super::lifecycle::publish_active_terminal(&self.state, self.connection_id, result);
        }
    }
}

impl<F: Future<Output = Result<(), RealtimeError>>> Future for Tracked<F> {
    type Output = Result<(), RealtimeError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        let Some(future) = this.future.as_mut() else {
            return Poll::Ready(this.result.unwrap_or(Err(RealtimeError::ActorTaskFailed)));
        };
        let Poll::Ready(result) = future.as_mut().poll(cx) else {
            return Poll::Pending;
        };
        drop(this.future.take());
        this.finish(result);
        Poll::Ready(result)
    }
}

impl<F> Drop for Tracked<F> {
    fn drop(&mut self) {
        // Do not rely on async-generator field drop order. On panic/abort/runtime
        // shutdown, both socket halves and pending reply producers are destroyed
        // before any consumer can observe generation-ended evidence.
        drop(self.future.take());
        if self.result.is_none() {
            self.finish(Err(RealtimeError::ActorTaskFailed));
        }
    }
}

// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use super::*;
use crate::realtime::session::reserve_command_slot;
use tokio::time::Instant;

struct DropProbe(Option<oneshot::Sender<()>>);

impl Drop for DropProbe {
    fn drop(&mut self) {
        if let Some(sender) = self.0.take() {
            let _send_result = sender.send(());
        }
    }
}

fn command() -> Command {
    let (reply, _response) = oneshot::channel();
    Command::Request {
        connection_id: ConnectionId::new(99),
        request_id: super::super::RequestId::new(2),
        invocation: super::super::admission::Invocation::new(),
        endpoint: "fixture",
        query: String::new(),
        body: String::new(),
        deadline: Instant::now() + Duration::from_secs(1),
        reply,
    }
}

#[tokio::test]
async fn full_command_queue_honors_the_pre_send_deadline() {
    let (commands, _receiver) = mpsc::channel(1);
    assert!(commands.try_send(command()).is_ok());
    let cancellation = CancellationToken::new();
    let deadline = Instant::now() + Duration::from_millis(10);

    let result =
        reserve_command_slot(&commands, &cancellation, deadline, ConnectionId::new(99)).await;

    assert!(matches!(result, Err(RealtimeError::RequestQueueTimeout)));
}

#[tokio::test]
async fn cancelled_queue_admission_is_stale_and_retains_the_actual_terminal_error() {
    let connection_id = ConnectionId::new(99);
    let (commands, _receiver) = mpsc::channel(1);
    assert!(commands.try_send(command()).is_ok());
    let cancellation = CancellationToken::new();
    let delivery = super::super::delivery::Delivery::new(connection_id, 1);
    let reserve = reserve_command_slot(
        &commands,
        &cancellation,
        Instant::now() + Duration::from_secs(1),
        ConnectionId::new(99),
    );
    tokio::pin!(reserve);
    assert!(futures_util::poll!(&mut reserve).is_pending());
    delivery.end(Err(RealtimeError::ServerClosed));
    cancellation.cancel();
    assert!(
        matches!(reserve.await, Err(RealtimeError::StaleGeneration { connection_id: id }) if id == connection_id)
    );
    assert!(matches!(
        delivery.recv().await.map(RealtimeEvent::into_payload),
        Some(super::super::RealtimeEventPayload::GenerationEnded(Err(
            RealtimeError::ServerClosed
        )))
    ));
}

#[tokio::test]
async fn dropping_connection_cancels_and_tracks_its_actor_task() {
    let (probe_dropped, dropped) = oneshot::channel();
    let (started, actor_started) = oneshot::channel();
    let tasks = TaskTracker::new();
    let cancellation = CancellationToken::new();
    let cancelled = cancellation.clone();
    let actor = tasks.spawn(async move {
        let _probe = DropProbe(Some(probe_dropped));
        let _send_result = started.send(());
        cancelled.cancelled().await;
        Ok::<(), RealtimeError>(())
    });
    assert!(actor_started.await.is_ok());

    let connection_id = ConnectionId::new(99);
    let (commands, _command_receiver) = mpsc::channel(1);
    let events = Arc::new(super::super::delivery::Delivery::new(connection_id, 1));
    let (_state_sender, state) = watch::channel(RealtimeState::Ready { connection_id });
    let codec = FrameCodec::new(128, 8);
    let Ok(codec) = codec else {
        panic!("fixture codec limits must validate");
    };
    let connection = RealtimeConnection {
        connection_id,
        kind: SocketKind::MarketData,
        commands,
        events,
        state,
        cancellation,
        request_abandoned: Arc::new(tokio::sync::Notify::new()),
        tasks: tasks.clone(),
        admission: Arc::new(super::super::admission::Admission::new()),
        request_timeout: Duration::from_secs(1),
        codec,
    };

    tasks.close();
    drop(connection);
    tasks.wait().await;
    assert!(actor.await.is_ok());

    assert!(
        tokio::time::timeout(Duration::from_secs(1), dropped)
            .await
            .is_ok()
    );
}

// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

use super::*;

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

    let result = reserve_command_slot(
        &commands,
        &cancellation,
        deadline,
        RealtimeError::ActorStopped,
    )
    .await;

    assert!(matches!(result, Err(RealtimeError::RequestQueueTimeout)));
}

#[tokio::test]
async fn dropping_connection_aborts_its_actor_task() {
    let (probe_dropped, dropped) = oneshot::channel();
    let (started, actor_started) = oneshot::channel();
    let actor = tokio::spawn(async move {
        let _probe = DropProbe(Some(probe_dropped));
        let _send_result = started.send(());
        std::future::pending::<Result<(), RealtimeError>>().await
    });
    assert!(actor_started.await.is_ok());

    let connection_id = ConnectionId::new(99);
    let (commands, _command_receiver) = mpsc::channel(1);
    let (_event_sender, events) = mpsc::channel(1);
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
        cancellation: CancellationToken::new(),
        request_abandoned: CancellationToken::new(),
        actor: Some(actor),
        request_timeout: Duration::from_secs(1),
        codec,
    };

    drop(connection);

    assert!(
        tokio::time::timeout(Duration::from_secs(1), dropped)
            .await
            .is_ok()
    );
}

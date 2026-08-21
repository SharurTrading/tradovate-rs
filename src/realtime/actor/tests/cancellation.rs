// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

use tokio::sync::oneshot;

use super::*;

#[tokio::test]
async fn disconnect_drains_every_pending_request() {
    let (listener, url) = bind().await;
    let server = tokio::spawn(async move {
        let mut socket = accept(listener).await;
        authorize(&mut socket).await;
        assert_eq!(next_text(&mut socket).await, "order/placeorder\n3\n\n{}");
        send_message(&mut socket, Message::Close(None)).await;
    });
    let client = authenticated_client(&url, "access", None);
    let mut connection = connect(&client, SocketKind::User, RealtimeConfig::default()).await;
    assert!(matches!(
        connection
            .request_non_mutating("order/placeorder", "", "{}")
            .await,
        Err(RealtimeError::RequestOutcomeUncertain { request_id })
            if request_id == RequestId::new(3)
    ));
    assert!(matches!(
        await_terminal_state(&mut connection).await,
        RealtimeState::ResyncRequired {
            reason: ResyncReason::ConnectionLost,
            ..
        }
    ));
    assert!(matches!(
        connection.shutdown().await,
        Err(RealtimeError::ServerClosed)
    ));
    join(server).await;
}

#[tokio::test]
async fn abandoning_an_admitted_request_poisons_the_generation() {
    let (listener, url) = bind().await;
    let (admitted, request_admitted) = oneshot::channel();
    let (release, server_released) = oneshot::channel();
    let server = tokio::spawn(async move {
        let mut socket = accept(listener).await;
        authorize(&mut socket).await;
        assert_eq!(next_text(&mut socket).await, "contract/item\n3\n\n{}");
        assert!(admitted.send(()).is_ok());
        assert!(server_released.await.is_ok());
        drop(socket);
    });
    let client = authenticated_client(&url, "access", None);
    let mut connection = connect(&client, SocketKind::User, RealtimeConfig::default()).await;
    let mut request = Box::pin(connection.request_non_mutating("contract/item", "", "{}"));

    let admission = time::timeout(Duration::from_secs(1), async {
        tokio::select! {
            result = &mut request => panic!("request completed before admission: {result:?}"),
            result = request_admitted => result,
        }
    })
    .await;
    assert!(matches!(admission, Ok(Ok(()))));
    drop(request);

    assert!(matches!(
        await_terminal_state(&mut connection).await,
        RealtimeState::ResyncRequired {
            reason: ResyncReason::RequestAbandoned,
            ..
        }
    ));
    assert!(matches!(
        connection.shutdown().await,
        Err(RealtimeError::ResyncRequired {
            reason: ResyncReason::RequestAbandoned,
            ..
        })
    ));
    assert!(release.send(()).is_ok());
    join(server).await;
}

// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use super::*;
use std::time::Duration;

/// Freshly allocated loopback ports the fixture tries before it reports itself
/// broken. On a shared runner another process can rebind a released ephemeral
/// port between the listener drop and the connect, so one accepted connection
/// is not evidence of anything; five in a row is a broken fixture.
const FRESH_PORT_ATTEMPTS: usize = 5;

/// Bound on one probe's request. A refused loopback connect fails at once; the
/// bound only matters when an unrelated listener accepted the connection and
/// then stalled, so the probe can abandon it and move on to a fresh port.
const PROBE_WAIT: Duration = Duration::from_secs(5);

/// Allocates a loopback port, releases it, and connects to it once.
///
/// Returns the refused-connection transport error. Returns `None` when an
/// unrelated listener rebound the released port and accepted the TCP
/// connection, whether it then answered, failed the HTTP exchange, or stalled;
/// the accidental connection is closed before the probe returns.
async fn probe_released_loopback_port() -> Option<reqwest::Error> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await;
    let Ok(listener) = listener else {
        panic!("fixture listener must bind");
    };
    let Ok(address) = listener.local_addr() else {
        panic!("fixture listener must have an address");
    };
    drop(listener);

    let request = reqwest::Client::new().get(format!("http://{address}/v1/renew"));
    match tokio::time::timeout(PROBE_WAIT, request.send()).await {
        Ok(Err(source)) if source.is_connect() => Some(source),
        accepted => {
            // Dropping the response, the post-connect error, or the elapsed
            // wait (whose pending request was already dropped) closes the
            // connection the unrelated listener accepted.
            drop(accepted);
            None
        }
    }
}

#[tokio::test]
async fn connection_failure_is_a_definitive_pre_send_renewal_failure() {
    let mut refused = None;
    for _ in 0..FRESH_PORT_ATTEMPTS {
        if let Some(source) = probe_released_loopback_port().await {
            refused = Some(source);
            break;
        }
    }
    let Some(source) = refused else {
        panic!(
            "fixture endpoint must refuse the connection: every one of \
             {FRESH_PORT_ATTEMPTS} released loopback ports accepted it"
        );
    };
    assert!(source.is_connect());
    assert!(renewal_is_definitive(&Error::Transport { source }));
}

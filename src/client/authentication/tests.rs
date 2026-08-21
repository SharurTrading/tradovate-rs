// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

use super::*;

#[tokio::test]
async fn connection_failure_is_a_definitive_pre_send_renewal_failure() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await;
    let Ok(listener) = listener else {
        panic!("fixture listener must bind");
    };
    let Ok(address) = listener.local_addr() else {
        panic!("fixture listener must have an address");
    };
    drop(listener);

    let request = reqwest::Client::new().get(format!("http://{address}/v1/renew"));
    let Err(source) = request.send().await else {
        panic!("fixture endpoint must refuse the connection");
    };
    assert!(source.is_connect());
    assert!(renewal_is_definitive(&Error::Transport { source }));
}

// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use super::*;
use crate::Environment;

fn demo_client() -> Client {
    Client::builder(Environment::Demo)
        .build()
        .unwrap_or_else(|error| panic!("demo client must build: {error}"))
}

#[tokio::test]
async fn empty_token_is_rejected_before_transmission() {
    let client = demo_client();
    assert!(matches!(
        client.adopt_session("").await,
        Err(Error::InvalidRequest {
            field: "access_token",
            ..
        })
    ));
}

#[tokio::test]
async fn whitespace_token_is_rejected_before_transmission() {
    let client = demo_client();
    assert!(matches!(
        client.adopt_session("synthetic bearer token").await,
        Err(Error::InvalidRequest {
            field: "access_token",
            ..
        })
    ));
    assert!(client.session_info().is_none());
}

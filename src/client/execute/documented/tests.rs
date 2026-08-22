// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use std::time::Duration;

use httpmock::{Mock, prelude::*};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};
use serde_json::json;

use super::*;
use crate::{
    EndpointSet, UserId,
    api::current::support::{CurrentQuery, CurrentRequest, push_query_value},
    auth::{InstalledSession, SessionInfo},
};

const CURRENT_QUERY_ENDPOINT: &str = "/fixture/current-query";
const QUERY_ENDPOINT: &str = "/fixture/query";
const MUTATION_ENDPOINT: &str = "/fixture/mutate";

#[derive(Serialize)]
struct FixtureBody {
    value: u32,
}

impl CurrentRequest for FixtureBody {
    fn validate_current(&self) -> Result<(), Error> {
        Ok(())
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
struct FixtureQueryResponse {
    value: u32,
}

struct FixtureCurrentQuery {
    ids: Vec<u64>,
}

impl CurrentQuery for FixtureCurrentQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, Error> {
        let mut pairs = Vec::new();
        for id in &self.ids {
            push_query_value(&mut pairs, "ids", id)?;
        }
        Ok(pairs)
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
struct FixtureMutationResponse {
    accepted: Option<bool>,
    command_id: Option<u64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RequiredMutationResponse {
    accepted: bool,
    command_id: u64,
}

impl DocumentedMutationResponse for FixtureMutationResponse {
    fn mutation_outcome(&self) -> MutationOutcome {
        match (self.accepted, self.command_id) {
            (Some(true), Some(_)) => MutationOutcome::Success,
            (Some(false), None) => MutationOutcome::Rejected,
            _ => MutationOutcome::Ambiguous,
        }
    }

    fn has_success_evidence(&self) -> bool {
        self.accepted == Some(true) || self.command_id.is_some()
    }
}

impl DocumentedMutationResponse for RequiredMutationResponse {
    fn mutation_outcome(&self) -> MutationOutcome {
        if self.accepted && self.command_id != 0 {
            MutationOutcome::Success
        } else if !self.accepted {
            MutationOutcome::Rejected
        } else {
            MutationOutcome::Ambiguous
        }
    }

    fn has_success_evidence(&self) -> bool {
        self.accepted || self.command_id != 0
    }
}

#[tokio::test]
async fn current_get_encodes_repeated_query_keys() {
    let server = MockServer::start_async().await;
    let query = server
        .mock_async(|when, then| {
            when.method(GET)
                .path("/v1/fixture/current-query")
                .header("authorization", "Bearer synthetic-access-token")
                .query_param("ids", "11")
                .query_param("ids", "12");
            then.status(200).json_body(json!({ "value": 13 }));
        })
        .await;
    let client = authenticated_client(&server);

    let result = client
        .get_current::<FixtureQueryResponse, _>(
            CURRENT_QUERY_ENDPOINT,
            &FixtureCurrentQuery { ids: vec![11, 12] },
        )
        .await;

    assert!(matches!(result, Ok(FixtureQueryResponse { value: 13 })));
    query.assert_async().await;
}

#[tokio::test]
async fn post_query_uses_authenticated_query_transport() {
    let server = MockServer::start_async().await;
    let query = server
        .mock_async(|when, then| {
            when.method(POST)
                .path("/v1/fixture/query")
                .header("authorization", "Bearer synthetic-access-token")
                .json_body(json!({ "value": 7 }));
            then.status(200).json_body(json!({ "value": 11 }));
        })
        .await;
    let client = authenticated_client(&server);

    let result = client
        .post_query::<FixtureQueryResponse, _>(QUERY_ENDPOINT, &FixtureBody { value: 7 })
        .await;

    assert!(matches!(result, Ok(FixtureQueryResponse { value: 11 })));
    query.assert_async().await;
}

#[tokio::test]
async fn documented_success_resolves_the_mutation_attempt() {
    let server = MockServer::start_async().await;
    let mutation = mutation_mock(&server, json!({ "accepted": true, "commandId": 41 })).await;
    let client = authenticated_client(&server);

    let result = client
        .post_documented_mutation::<FixtureMutationResponse, _>(
            MUTATION_ENDPOINT,
            &FixtureBody { value: 7 },
        )
        .await;

    assert!(matches!(
        result,
        Ok(FixtureMutationResponse {
            command_id: Some(41),
            ..
        })
    ));
    assert!(!client.mutation_reconciliation_required());
    mutation.assert_async().await;
}

#[tokio::test]
async fn documented_rejection_resolves_without_latching() {
    let server = MockServer::start_async().await;
    let mutation = mutation_mock(&server, json!({ "accepted": false })).await;
    let client = authenticated_client(&server);

    let result = client
        .post_documented_mutation::<FixtureMutationResponse, _>(
            MUTATION_ENDPOINT,
            &FixtureBody { value: 7 },
        )
        .await;

    assert!(matches!(result, Err(Error::Business { .. })));
    assert!(!client.mutation_reconciliation_required());
    mutation.assert_async().await;
}

#[tokio::test]
async fn penalty_control_precedes_required_response_decoding() {
    let server = MockServer::start_async().await;
    let body = json!({ "p-ticket": "secret-ticket", "p-time": 15 });
    let mutation = mutation_mock(&server, body).await;
    let client = authenticated_client(&server);

    let result = client
        .post_documented_mutation::<RequiredMutationResponse, _>(
            MUTATION_ENDPOINT,
            &FixtureBody { value: 7 },
        )
        .await;

    assert!(matches!(
        result,
        Err(Error::ProviderPenalty {
            retry_after,
            captcha_required: false,
            ..
        }) if retry_after == Duration::from_secs(15)
    ));
    assert!(!client.mutation_reconciliation_required());
    assert!(matches!(
        client
            .post_documented_mutation::<RequiredMutationResponse, _>(
                MUTATION_ENDPOINT,
                &FixtureBody { value: 7 },
            )
            .await,
        Err(Error::LocalRateLimit { .. })
    ));
    mutation.assert_calls_async(1).await;
}

#[tokio::test]
async fn incomplete_success_is_ambiguous_and_latches() {
    let server = MockServer::start_async().await;
    let mutation = mutation_mock(&server, json!({ "accepted": true })).await;
    let query = server
        .mock_async(|when, then| {
            when.method(POST)
                .path("/v1/fixture/query")
                .json_body(json!({ "value": 7 }));
            then.status(200).json_body(json!({ "value": 11 }));
        })
        .await;
    let client = authenticated_client(&server);

    let result = client
        .post_documented_mutation::<FixtureMutationResponse, _>(
            MUTATION_ENDPOINT,
            &FixtureBody { value: 7 },
        )
        .await;

    assert!(matches!(result, Err(Error::AmbiguousMutation { .. })));
    assert!(client.mutation_reconciliation_required());
    assert!(matches!(
        client
            .post_documented_mutation::<FixtureMutationResponse, _>(
                MUTATION_ENDPOINT,
                &FixtureBody { value: 8 },
            )
            .await,
        Err(Error::MutationReconciliationRequired { .. })
    ));
    assert!(matches!(
        client
            .post_query::<FixtureQueryResponse, _>(QUERY_ENDPOINT, &FixtureBody { value: 7 })
            .await,
        Ok(FixtureQueryResponse { value: 11 })
    ));
    mutation.assert_calls_async(1).await;
    query.assert_async().await;
}

#[tokio::test]
async fn cancelling_documented_mutation_latches_before_another_send() {
    let server = MockServer::start_async().await;
    let mutation = server
        .mock_async(|when, then| {
            when.method(POST).path("/v1/fixture/mutate");
            then.status(200)
                .delay(Duration::from_millis(500))
                .json_body(json!({ "accepted": true, "commandId": 41 }));
        })
        .await;
    let client = authenticated_client(&server);
    let pending = tokio::spawn({
        let client = client.clone();
        async move {
            client
                .post_documented_mutation::<FixtureMutationResponse, _>(
                    MUTATION_ENDPOINT,
                    &FixtureBody { value: 7 },
                )
                .await
        }
    });

    await_admission(&mutation).await;
    pending.abort();
    let Err(join_error) = pending.await else {
        panic!("aborted task must not complete");
    };
    assert!(join_error.is_cancelled());
    assert!(client.mutation_reconciliation_required());
    assert!(matches!(
        client
            .post_documented_mutation::<FixtureMutationResponse, _>(
                MUTATION_ENDPOINT,
                &FixtureBody { value: 8 },
            )
            .await,
        Err(Error::MutationReconciliationRequired { .. })
    ));
    mutation.assert_calls_async(1).await;
}

async fn mutation_mock(server: &MockServer, response: serde_json::Value) -> Mock<'_> {
    server
        .mock_async(|when, then| {
            when.method(POST)
                .path("/v1/fixture/mutate")
                .header("authorization", "Bearer synthetic-access-token")
                .json_body(json!({ "value": 7 }));
            then.status(200).json_body(response);
        })
        .await
}

async fn await_admission(mutation: &Mock<'_>) {
    let observed = tokio::time::timeout(Duration::from_secs(1), async {
        while mutation.calls_async().await == 0 {
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await;
    assert!(observed.is_ok(), "fixture never observed the mutation");
}

pub(super) fn authenticated_client(server: &MockServer) -> Client {
    let base = server.base_url();
    let websocket = base.replacen("http://", "ws://", 1);
    let endpoints = EndpointSet::custom(
        &format!("{base}/v1"),
        &format!("{websocket}/v1/websocket"),
        &format!("{websocket}/v1/websocket"),
        &format!("{websocket}/v1/websocket"),
    )
    .unwrap_or_else(|error| panic!("fixture endpoints must validate: {error}"));
    let client = Client::builder_with_endpoints(endpoints)
        .build()
        .unwrap_or_else(|error| panic!("fixture client must build: {error}"));
    let expires_at = "2035-01-01T00:00:00Z"
        .parse::<Timestamp>()
        .unwrap_or_else(|error| panic!("fixture timestamp must parse: {error}"));
    let user_id =
        UserId::new(1).unwrap_or_else(|error| panic!("fixture user must validate: {error}"));
    let session = InstalledSession::try_new(
        "synthetic-access-token".to_owned(),
        None,
        SessionInfo::new(user_id, expires_at, false),
    )
    .unwrap_or_else(|error| panic!("fixture session must validate: {error}"));
    let authentication = client.tokens.begin_authentication();
    assert!(authentication.commit(session).is_ok());
    client
}

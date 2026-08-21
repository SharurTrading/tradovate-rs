// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

use serde_json::value::RawValue;

use super::*;
use crate::api::current::users::Order;
use crate::realtime::{
    FrameCodec, RealtimeError, RealtimePayloadError, RequestId, ServerFrame, ServerMessage,
    UserSyncEntityType,
};

#[test]
fn bootstrap_reuses_typed_current_rest_entities() {
    let codec = FrameCodec::new(4_096, 4);
    let Ok(codec) = codec else {
        panic!("fixture codec must validate");
    };
    let frame =
        codec.decode(r#"a[{"i":2,"s":200,"d":{"users":[],"contractGroups":[],"orders":[]}}]"#);
    let Ok(ServerFrame::Messages(mut messages)) = frame else {
        panic!("fixture response must decode");
    };
    let Some(ServerMessage::Response(response)) = messages.pop() else {
        panic!("fixture must contain a response");
    };
    assert_eq!(response.request_id(), RequestId::new(2));
    let decoded = decode::bootstrap(&response);
    let Ok(UserStreamEvent::Bootstrap(bootstrap)) = decoded else {
        panic!("typed empty entity collections must decode");
    };
    assert_eq!(bootstrap.entities().len(), 3);
    assert!(bootstrap.entities().iter().all(UserEntityBatch::is_empty));
}

#[test]
fn bootstrap_decodes_every_pinned_sync_message_collection() {
    let codec = FrameCodec::new(16_384, 4);
    let Ok(codec) = codec else {
        panic!("fixture codec must validate");
    };
    let frame = codec.decode(
        r#"a[{"i":2,"s":200,"d":{"users":[],"userProperties":[],"properties":[],
        "accounts":[],"accountRiskStatuses":[],"marginSnapshots":[],
        "userAccountAutoLiqs":[],"cashBalances":[],"currencies":[],"positions":[],
        "fillPairs":[],"orders":[],"contracts":[],"contractMaturities":[],
        "products":[],"exchanges":[],"spreadDefinitions":[],"commands":[],
        "commandReports":[],"executionReports":[],"orderVersions":[],"fills":[],
        "fillFees":[],"orderStrategies":[],"orderStrategyLinks":[],"userPlugins":[],
        "annualReviews":[],"userReadStatuses":[],"userPromoCodes":[],
        "contractGroups":[],"orderStrategyTypes":[]}}]"#,
    );
    let Ok(ServerFrame::Messages(mut messages)) = frame else {
        panic!("complete current bootstrap must decode");
    };
    let Some(ServerMessage::Response(response)) = messages.pop() else {
        panic!("fixture must contain a response");
    };
    let decoded = decode::bootstrap(&response);
    let Ok(UserStreamEvent::Bootstrap(bootstrap)) = decoded else {
        panic!("all current entity collections must decode");
    };
    assert_eq!(bootstrap.entities().len(), 31);
    assert!(bootstrap.entities().iter().all(UserEntityBatch::is_empty));
    assert!(
        bootstrap
            .entities()
            .iter()
            .all(|batch| !matches!(batch, UserEntityBatch::Unsupported { .. }))
    );
    let actual = bootstrap
        .entities()
        .iter()
        .map(UserEntityBatch::entity_type)
        .collect::<std::collections::BTreeSet<_>>();
    let expected = UserSyncEntityType::ALL
        .iter()
        .copied()
        .map(UserSyncEntityType::as_str)
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(actual, expected);
}

#[test]
fn partial_bootstrap_missing_a_pinned_required_collection_is_rejected() {
    let codec = FrameCodec::new(4_096, 4);
    let Ok(codec) = codec else {
        panic!("fixture codec must validate");
    };
    let frame = codec.decode(r#"a[{"i":2,"s":200,"d":{"users":[]}}]"#);
    let Ok(ServerFrame::Messages(mut messages)) = frame else {
        panic!("fixture response must decode");
    };
    let Some(ServerMessage::Response(response)) = messages.pop() else {
        panic!("fixture must contain a response");
    };
    assert!(decode::bootstrap(&response).is_err());
}

#[test]
fn props_accepts_singleton_and_batch_wrappers_with_typed_entities() {
    let direct = serde_json::from_str::<Order>(
        r#"{"id":10,"accountId":20,"contractId":30,"timestamp":"2026-08-21T00:00:00Z",
        "action":"Buy","ordStatus":"Working","admin":false}"#,
    );
    assert!(
        direct.is_ok(),
        "generated current order must decode: {direct:?}"
    );
    let order_event = raw(r#"{"entityType":"order","eventType":"Updated","entity":{
        "id":10,"accountId":20,"contractId":30,"timestamp":"2026-08-21T00:00:00Z",
        "action":"Buy","ordStatus":"Working","admin":false}}"#);
    assert!(decode::properties(Some(&order_event)).is_ok());
    let future_event =
        raw(r#"{"entityType":"futureEntity","eventType":"FutureMutation","entity":[{},{}]}"#);
    assert!(decode::properties(Some(&future_event)).is_ok());
    let signature_event = raw(
        r#"{"entityType":"OtherEnvAdminAlertSignal","eventType":"Created","entity":{
        "adminAlertSignal":{"timestamp":"2026-08-21T00:00:00Z","adminAlertId":70,
        "text":"signature recorded","emailSent":false,"subjectId":12081}}}"#,
    );
    let signature = decode::properties(Some(&signature_event));
    assert!(matches!(
        signature,
        Ok(UserStreamEvent::Properties(events))
            if matches!(events[0].entities(),
                UserEntityBatch::OtherEnvironmentAdminAlertSignals(values) if values.len() == 1)
    ));
    let raw = raw(r#"[
          {"entityType":"order","eventType":"Updated","entity":{
            "id":10,"accountId":20,"contractId":30,"timestamp":"2026-08-21T00:00:00Z",
            "action":"Buy","ordStatus":"Working","admin":false}},
          {"entityType":"futureEntity","eventType":"FutureMutation","entity":[{},{}]}
        ]"#);
    let decoded = decode::properties(Some(&raw));
    let events = match decoded {
        Ok(UserStreamEvent::Properties(events)) => events,
        Ok(UserStreamEvent::Bootstrap(_)) => panic!("fixture must be a properties event"),
        Err(error) => panic!("current props batch must decode: {error:?}"),
    };
    assert_eq!(events.len(), 2);
    assert!(matches!(events[0].entities(), UserEntityBatch::Orders(values) if values.len() == 1));
    assert!(matches!(
        events[1].entities(),
        UserEntityBatch::Unsupported { entity_type, item_count: 2 }
            if entity_type.as_str() == "futureEntity"
    ));
    assert!(matches!(
        events[1].event_type(),
        PropertyEventType::Unknown(value) if value.as_str() == "FutureMutation"
    ));
}

#[test]
fn malformed_props_never_becomes_an_untyped_public_event() {
    let raw = raw(r#"{"entityType":"order","eventType":"Updated"}"#);
    assert!(decode::properties(Some(&raw)).is_err());
}

#[test]
fn oversized_bootstrap_object_is_a_typed_limit_failure() {
    let optional = (0..(super::decode::MAX_BOOTSTRAP_COLLECTIONS - 1))
        .map(|index| format!(r#""future{index}":[]"#))
        .collect::<Vec<_>>()
        .join(",");
    let payload = format!(r#"{{"users":[],"contractGroups":[],{optional}}}"#);
    let frame_payload = format!(r#"a[{{"i":2,"s":200,"d":{payload}}}]"#);
    let codec = FrameCodec::new(frame_payload.len(), 4);
    let Ok(codec) = codec else {
        panic!("fixture codec must validate");
    };
    let frame = codec.decode(&frame_payload);
    let Ok(ServerFrame::Messages(mut messages)) = frame else {
        panic!("fixture response must decode");
    };
    let Some(ServerMessage::Response(response)) = messages.pop() else {
        panic!("fixture must contain a response");
    };

    assert!(matches!(
        decode::bootstrap(&response),
        Err(RealtimeError::InvalidEvent {
            reason: RealtimePayloadError::LimitExceeded,
            ..
        })
    ));
}

fn raw(value: &str) -> Box<RawValue> {
    let result = serde_json::from_str(value);
    let Ok(raw) = result else {
        panic!("test JSON must be valid");
    };
    raw
}

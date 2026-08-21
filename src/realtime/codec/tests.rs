// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

use serde_json::value::RawValue;

use super::*;

fn codec(max_frame_bytes: usize) -> FrameCodec {
    codec_with_message_limit(max_frame_bytes, 64)
}

fn codec_with_message_limit(max_frame_bytes: usize, max_messages: usize) -> FrameCodec {
    let Ok(codec) = FrameCodec::new(max_frame_bytes, max_messages) else {
        panic!("test frame limit must be valid");
    };
    codec
}

fn decode(frame: &str) -> ServerFrame {
    let Ok(frame) = codec(4_096).decode(frame) else {
        panic!("test frame must decode");
    };
    frame
}

#[test]
fn zero_frame_limit_is_rejected() {
    assert!(matches!(FrameCodec::new(0, 1), Err(Error::ZeroFrameLimit)));
    assert!(matches!(
        FrameCodec::new(1, 0),
        Err(Error::ZeroMessageLimit)
    ));
}

#[test]
fn open_and_heartbeat_frames_decode() {
    assert!(matches!(decode("o"), ServerFrame::Open));
    assert!(matches!(decode("h"), ServerFrame::Heartbeat));
}

#[test]
fn open_and_heartbeat_frames_reject_payloads() {
    for (frame, kind) in [("o[]", 'o'), ("h[]", 'h')] {
        assert!(matches!(
            codec(16).decode(frame),
            Err(Error::UnexpectedPayload { kind: actual }) if actual == kind
        ));
    }
}

#[test]
fn data_frame_decodes_multiple_responses_and_events_in_order() {
    let frame = decode(
        r#"a[{"s":200,"i":2,"d":{"ok":true}},{"e":"md","d":{"quotes":[]}},{"e":"futureEvent","d":{"price":"5000.25"},"extra":true}]"#,
    );
    let ServerFrame::Messages(messages) = frame else {
        panic!("expected a message frame");
    };
    assert_eq!(messages.len(), 3);

    let ServerMessage::Response(response) = &messages[0] else {
        panic!("expected a response");
    };
    assert_eq!(response.request_id(), RequestId::new(2));
    assert_eq!(response.status(), 200);
    assert_eq!(response.data().map(RawValue::get), Some(r#"{"ok":true}"#));

    let ServerMessage::Event(Event::MarketData(Some(market_data))) = &messages[1] else {
        panic!("expected market data");
    };
    assert_eq!(market_data.get(), r#"{"quotes":[]}"#);

    let ServerMessage::Event(Event::Unknown { kind, raw }) = &messages[2] else {
        panic!("expected an unknown event");
    };
    assert_eq!(kind, "futureEvent");
    assert_eq!(
        raw.get(),
        r#"{"e":"futureEvent","d":{"price":"5000.25"},"extra":true}"#
    );
}

#[test]
fn all_documented_event_kinds_are_classified() {
    let ServerFrame::Messages(messages) = decode(
        r#"a[{"e":"props","d":null},{"e":"shutdown"},{"e":"chart","d":[]},{"e":"clock","d":"{\"s\":20}"}]"#,
    ) else {
        panic!("expected a message frame");
    };

    let [
        ServerMessage::Event(Event::Properties(Some(properties))),
        ServerMessage::Event(Event::Shutdown(None)),
        ServerMessage::Event(Event::Chart(Some(chart))),
        ServerMessage::Event(Event::Clock(Some(clock))),
    ] = messages.as_slice()
    else {
        panic!("expected all documented event variants");
    };
    assert_eq!(properties.get(), "null");
    assert_eq!(chart.get(), "[]");
    assert_eq!(clock.get(), r#""{\"s\":20}""#);
}

#[test]
fn unclassified_array_values_are_preserved() {
    let ServerFrame::Messages(messages) = decode(r#"a[{"future":{"exact":"1.25"}}]"#) else {
        panic!("expected a message frame");
    };
    let [ServerMessage::Unknown(raw)] = messages.as_slice() else {
        panic!("expected one unknown message");
    };
    assert_eq!(raw.get(), r#"{"future":{"exact":"1.25"}}"#);
}

#[test]
fn financial_number_tokens_are_preserved_without_float_conversion() {
    const EXACT: &str = "12345678901234567890.123456789012345678";
    let frame = format!(r#"a[{{"e":"md","d":{{"price":{EXACT}}}}}]"#);
    let ServerFrame::Messages(messages) = decode(&frame) else {
        panic!("expected a message frame");
    };
    let [ServerMessage::Event(Event::MarketData(Some(payload)))] = messages.as_slice() else {
        panic!("expected one market-data event");
    };
    assert_eq!(payload.get(), format!(r#"{{"price":{EXACT}}}"#));
}

#[test]
fn close_frame_decodes_code_and_reason() {
    assert!(matches!(
        decode(r#"c[3000,"maintenance"]"#),
        ServerFrame::Close { code: 3_000, reason } if reason == "maintenance"
    ));
}

#[test]
fn malformed_json_and_close_shapes_are_rejected_without_echoing_input() {
    const SECRET: &str = "super-secret-token";
    let malformed = format!(r#"a[{{"token":"{SECRET}"}}"#);
    let Err(error) = codec(4_096).decode(&malformed) else {
        panic!("malformed JSON must fail");
    };
    assert!(matches!(error, Error::InvalidJson { kind: 'a', .. }));
    assert!(!error.to_string().contains(SECRET));
    assert!(!format!("{error:?}").contains(SECRET));

    assert!(matches!(
        codec(32).decode(r"c[3000]"),
        Err(Error::InvalidJson { kind: 'c', .. })
    ));
}

#[test]
fn malformed_response_correlation_fields_are_rejected() {
    for frame in [r#"a[{"i":-1,"s":200}]"#, r#"a[{"i":1}]"#] {
        assert!(matches!(
            codec(64).decode(frame),
            Err(Error::InvalidResponseRequestId | Error::InvalidResponseStatus)
        ));
    }
    assert!(matches!(
        codec(64).decode(r#"a[{"i":1,"s":65536}]"#),
        Err(Error::InvalidResponseStatus)
    ));
}

#[test]
fn unsupported_empty_and_non_array_frames_are_rejected() {
    assert!(matches!(codec(16).decode(""), Err(Error::EmptyServerFrame)));
    assert!(matches!(
        codec(16).decode("x[]"),
        Err(Error::UnsupportedServerFrame { kind: 'x' })
    ));
    assert!(matches!(
        codec(16).decode("a{}"),
        Err(Error::InvalidJson { kind: 'a', .. })
    ));
}

#[test]
fn hard_frame_limit_is_enforced_at_the_byte_boundary() {
    let frame = r#"a[{"e":"md"}]"#;
    assert!(codec(frame.len()).decode(frame).is_ok());
    assert!(matches!(
        codec(frame.len() - 1).decode(frame),
        Err(Error::FrameTooLarge {
            actual_bytes,
            max_bytes
        }) if actual_bytes == frame.len() && max_bytes == frame.len() - 1
    ));
}

#[test]
fn message_count_is_bounded_before_array_allocation() {
    let frame = r#"a[{"e":"md"},{"e":"chart"},{"e":"clock"}]"#;
    assert!(matches!(
        codec_with_message_limit(256, 2).decode(frame),
        Err(Error::TooManyMessages {
            actual_messages: 3,
            max_messages: 2
        })
    ));
    assert!(codec_with_message_limit(256, 3).decode(frame).is_ok());
}

#[test]
fn truncated_message_arrays_over_the_limit_fail_closed() {
    for frame in [
        r#"a[{"e":"md"},{"e":"chart"},{"e":"clock""#,
        r#"a[{"e":"md"},{"e":"chart"},{"e":"clock"},"#,
    ] {
        assert!(matches!(
            codec_with_message_limit(256, 2).decode(frame),
            Err(Error::TooManyMessages {
                actual_messages: 3,
                max_messages: 2
            })
        ));
    }
}

#[test]
fn truncated_array_at_the_limit_remains_a_bounded_json_error() {
    let frame = r#"a[{"e":"future","d":{"nested":[1,2,3]}},{"e":"clock""#;
    assert!(matches!(
        codec_with_message_limit(256, 2).decode(frame),
        Err(Error::InvalidJson { kind: 'a', .. })
    ));
}

#[test]
fn message_count_ignores_nested_and_escaped_separators() {
    let frame = r#"a[{"e":"future","d":{"text":"a,]\\\"b","nested":[1,2,3]}},{"e":"clock"}]"#;
    assert!(codec_with_message_limit(256, 2).decode(frame).is_ok());
}

#[test]
fn request_encoding_has_exactly_four_newline_separated_fields() {
    let encoded = codec(256).encode_request(
        "contract/rollcontract",
        RequestId::new(33),
        "",
        r#"{"name":"YMZ6","forward":true}"#,
    );
    assert_eq!(
        encoded,
        Ok("contract/rollcontract\n33\n\n{\"name\":\"YMZ6\",\"forward\":true}".to_owned())
    );

    assert_eq!(
        codec(64).encode_request("executionReport/list", RequestId::new(4), "", ""),
        Ok("executionReport/list\n4\n\n".to_owned())
    );
}

#[test]
fn endpoint_and_query_line_breaks_are_rejected_but_body_line_breaks_are_allowed() {
    for endpoint in ["bad\nendpoint", "bad\rendpoint"] {
        assert_eq!(
            codec(128).encode_request(endpoint, RequestId::new(1), "", "{}"),
            Err(Error::EndpointContainsLineBreak)
        );
    }
    for query in ["a=1\nb=2", "a=1\rb=2"] {
        assert_eq!(
            codec(128).encode_request("entity/list", RequestId::new(1), query, ""),
            Err(Error::QueryContainsLineBreak)
        );
    }
    assert_eq!(
        codec(128).encode_request("entity/list", RequestId::new(1), "", "{\n}"),
        Ok("entity/list\n1\n\n{\n}".to_owned())
    );
}

#[test]
fn request_preflight_reserves_the_widest_request_identifier() {
    let codec = codec(32);
    assert!(codec.validate_request("e", "", "12345678").is_ok());
    assert!(matches!(
        codec.validate_request("e", "", "123456789"),
        Err(Error::FrameTooLarge {
            actual_bytes: 33,
            max_bytes: 32
        })
    ));
    assert_eq!(
        codec.validate_request("bad\nendpoint", "", ""),
        Err(Error::EndpointContainsLineBreak)
    );
}

#[test]
fn authorization_and_heartbeat_encoding_match_the_protocol() {
    assert_eq!(
        codec(128).encode_authorize(RequestId::new(2), "access-token"),
        Ok("authorize\n2\n\naccess-token".to_owned())
    );
    assert_eq!(codec(2).encode_heartbeat(), Ok("[]".to_owned()));
    assert!(matches!(
        codec(1).encode_heartbeat(),
        Err(Error::FrameTooLarge {
            actual_bytes: 2,
            max_bytes: 1
        })
    ));
}

#[test]
fn authorization_size_errors_never_render_the_token() {
    const TOKEN: &str = "never-render-this-token";
    let Err(error) = codec(8).encode_authorize(RequestId::new(7), TOKEN) else {
        panic!("authorization must exceed the test limit");
    };

    assert!(!error.to_string().contains(TOKEN));
    assert!(!format!("{error:?}").contains(TOKEN));
}

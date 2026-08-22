// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

use serde_json::json;

use super::*;
use crate::auth::wire::AccessTokenRequest;

fn device_id() -> DeviceId {
    DeviceId::new("synthetic-device")
        .unwrap_or_else(|error| panic!("fixture device ID must validate: {error}"))
}

#[test]
fn only_name_and_password_are_required() {
    let credentials = Credentials::builder("synthetic-user", "synthetic-password")
        .build()
        .unwrap_or_else(|error| panic!("minimal credentials must validate: {error}"));
    let request = AccessTokenRequest::new(&credentials, None);
    let encoded = serde_json::to_value(request)
        .unwrap_or_else(|error| panic!("minimal credentials must encode: {error}"));

    assert_eq!(
        encoded,
        json!({
            "name": "synthetic-user",
            "password": "synthetic-password"
        })
    );
}

#[test]
fn supplied_optional_fields_use_the_documented_wire_names() {
    let credentials = Credentials::builder("synthetic-user", "synthetic-password")
        .app_id("synthetic-app")
        .app_version("1.0")
        .numeric_client_id(123)
        .secret("synthetic-secret")
        .device_id(device_id())
        .hibp_check(false)
        .build()
        .unwrap_or_else(|error| panic!("full credentials must validate: {error}"));
    let request = AccessTokenRequest::new(&credentials, Some("synthetic-ticket"));
    let encoded = serde_json::to_value(request)
        .unwrap_or_else(|error| panic!("full credentials must encode: {error}"));

    assert_eq!(
        encoded,
        json!({
            "name": "synthetic-user",
            "password": "synthetic-password",
            "appId": "synthetic-app",
            "appVersion": "1.0",
            "cid": 123,
            "sec": "synthetic-secret",
            "deviceId": "synthetic-device",
            "hibpCheck": false,
            "p-ticket": "synthetic-ticket"
        })
    );
}

#[test]
fn text_client_identifier_preserves_its_wire_type() {
    let credentials = Credentials::builder("synthetic-user", "synthetic-password")
        .text_client_id("synthetic-client")
        .build()
        .unwrap_or_else(|error| panic!("text client ID must validate: {error}"));
    let encoded = serde_json::to_value(AccessTokenRequest::new(&credentials, None))
        .unwrap_or_else(|error| panic!("text client ID must encode: {error}"));

    assert_eq!(encoded.get("cid"), Some(&json!("synthetic-client")));
}

#[test]
fn supplied_optional_text_is_validated() {
    for result in [
        Credentials::builder("synthetic-user", "synthetic-password")
            .app_id("")
            .build(),
        Credentials::builder("synthetic-user", "synthetic-password")
            .app_version(" 1.0")
            .build(),
        Credentials::builder("synthetic-user", "synthetic-password")
            .text_client_id("client\n")
            .build(),
        Credentials::builder("synthetic-user", "synthetic-password")
            .secret("")
            .build(),
    ] {
        assert!(matches!(result, Err(Error::Configuration(_))));
    }
}

#[test]
fn debug_redacts_every_sensitive_value() {
    let builder = Credentials::builder("secret-user", "secret-password")
        .app_id("secret-app-id")
        .app_version("secret-app-version")
        .numeric_client_id(123)
        .secret("secret-key")
        .hibp_check(true)
        .device_id(
            DeviceId::new("secret-device")
                .unwrap_or_else(|error| panic!("fixture device ID must validate: {error}")),
        );
    let builder_debug = format!("{builder:?}");
    assert!(!builder_debug.contains("secret-user"));
    assert!(!builder_debug.contains("secret-password"));
    assert!(!builder_debug.contains("secret-key"));
    assert!(!builder_debug.contains("secret-device"));
    assert!(!builder_debug.contains("secret-app-id"));
    assert!(!builder_debug.contains("secret-app-version"));
    assert!(!builder_debug.contains("123"));
    assert!(!builder_debug.contains("hibp_check: Some(true)"));
    assert!(builder_debug.contains("hibp_check: Some(\"[REDACTED]\")"));

    let credentials = builder
        .build()
        .unwrap_or_else(|error| panic!("credentials must validate: {error}"));
    let credentials_debug = format!("{credentials:?}");
    assert!(!credentials_debug.contains("secret-user"));
    assert!(!credentials_debug.contains("secret-password"));
    assert!(!credentials_debug.contains("secret-key"));
    assert!(!credentials_debug.contains("secret-device"));
    assert!(!credentials_debug.contains("secret-app-id"));
    assert!(!credentials_debug.contains("secret-app-version"));
    assert!(!credentials_debug.contains("123"));
    assert!(!credentials_debug.contains("hibp_check: Some(true)"));
    assert!(credentials_debug.contains("hibp_check: Some(\"[REDACTED]\")"));
}

#[test]
fn official_wire_length_limits_are_enforced() {
    let oversized_name = "x".repeat(SHORT_FIELD_LIMIT + 1);
    let name_result = Credentials::builder(oversized_name, "password").build();
    assert!(matches!(name_result, Err(Error::Configuration(_))));

    let maximum_password = "x".repeat(PASSWORD_LIMIT);
    assert!(
        Credentials::builder("synthetic-user", maximum_password)
            .build()
            .is_ok()
    );

    let oversized_password = "x".repeat(PASSWORD_LIMIT + 1);
    let password_result = Credentials::builder("synthetic-user", oversized_password).build();
    assert!(matches!(password_result, Err(Error::Configuration(_))));
}

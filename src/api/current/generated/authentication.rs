// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0
// @generated
// Generator: tools/generate_openapi.py
// Source: https://partner.tradovate.com/openapi.json (snapshot 2026-08-21, sha256 37caeccf4b0913460a788fcaf4c902497059b8ffe6f6355512e6c08eaacde769)

// Provider wire fields remain schema-auditable even when they repeat
// their type name; wide schema-faithful builders remain one generated
// unit so regeneration and source review cannot drift field subsets.
#![allow(clippy::struct_field_names, clippy::too_many_lines)]

//! Current authentication operations and wire models.

/// Current wire model `AccessTokenRequest`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct AccessTokenRequest {
    #[serde(rename = "hibpCheck", default, skip_serializing_if = "Option::is_none")]
    hibp_check: Option<bool>,
    #[serde(rename = "name")]
    name: crate::api::current::SecretValue,
    #[serde(rename = "password")]
    password: crate::api::current::SecretValue,
    #[serde(rename = "appId", default, skip_serializing_if = "Option::is_none")]
    app_id: Option<String>,
    #[serde(
        rename = "appVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    app_version: Option<String>,
    #[serde(rename = "deviceId", default, skip_serializing_if = "Option::is_none")]
    device_id: Option<crate::DeviceId>,
    #[serde(rename = "cid", default, skip_serializing_if = "Option::is_none")]
    cid: Option<String>,
    #[serde(rename = "sec", default, skip_serializing_if = "Option::is_none")]
    sec: Option<crate::api::current::SecretValue>,
}

impl AccessTokenRequest {
    /// Returns wire field `hibpCheck`.
    #[must_use]
    pub fn hibp_check(&self) -> Option<&bool> {
        self.hibp_check.as_ref()
    }

    /// Reports whether secret field `name` is present.
    #[must_use]
    pub const fn has_name(&self) -> bool {
        true
    }

    /// Reports whether secret field `password` is present.
    #[must_use]
    pub const fn has_password(&self) -> bool {
        true
    }

    /// Returns wire field `appId`.
    #[must_use]
    pub fn app_id(&self) -> Option<&str> {
        self.app_id.as_deref()
    }

    /// Returns wire field `appVersion`.
    #[must_use]
    pub fn app_version(&self) -> Option<&str> {
        self.app_version.as_deref()
    }

    /// Returns wire field `deviceId`.
    #[must_use]
    pub fn device_id(&self) -> Option<&crate::DeviceId> {
        self.device_id.as_ref()
    }

    /// Returns wire field `cid`.
    #[must_use]
    pub fn cid(&self) -> Option<&str> {
        self.cid.as_deref()
    }

    /// Reports whether secret field `sec` is present.
    #[must_use]
    pub const fn has_sec(&self) -> bool {
        self.sec.is_some()
    }

    /// Starts a builder for [`AccessTokenRequest`].
    pub fn builder() -> AccessTokenRequestBuilder {
        AccessTokenRequestBuilder::default()
    }
}

/// Builder for [`AccessTokenRequest`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct AccessTokenRequestBuilder {
    hibp_check: Option<bool>,
    name: Option<crate::api::current::SecretValue>,
    password: Option<crate::api::current::SecretValue>,
    app_id: Option<String>,
    app_version: Option<String>,
    device_id: Option<crate::DeviceId>,
    cid: Option<String>,
    sec: Option<crate::api::current::SecretValue>,
}

impl AccessTokenRequestBuilder {
    /// Sets wire field `hibpCheck`.
    pub fn hibp_check(mut self, value: bool) -> Self {
        self.hibp_check = Some(value);
        self
    }

    /// Sets wire field `name`.
    pub fn name(mut self, value: crate::api::current::SecretValue) -> Self {
        self.name = Some(value);
        self
    }

    /// Sets wire field `password`.
    pub fn password(mut self, value: crate::api::current::SecretValue) -> Self {
        self.password = Some(value);
        self
    }

    /// Sets wire field `appId`.
    pub fn app_id(mut self, value: impl Into<String>) -> Self {
        self.app_id = Some(value.into());
        self
    }

    /// Sets wire field `appVersion`.
    pub fn app_version(mut self, value: impl Into<String>) -> Self {
        self.app_version = Some(value.into());
        self
    }

    /// Sets wire field `deviceId`.
    pub fn device_id(mut self, value: crate::DeviceId) -> Self {
        self.device_id = Some(value);
        self
    }

    /// Sets wire field `cid`.
    pub fn cid(mut self, value: impl Into<String>) -> Self {
        self.cid = Some(value.into());
        self
    }

    /// Sets wire field `sec`.
    pub fn sec(mut self, value: crate::api::current::SecretValue) -> Self {
        self.sec = Some(value);
        self
    }

    /// Validates required fields and builds [`AccessTokenRequest`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<AccessTokenRequest, crate::api::current::BuildError> {
        let name = self
            .name
            .ok_or(crate::api::current::BuildError::missing("name"))?;
        let password = self
            .password
            .ok_or(crate::api::current::BuildError::missing("password"))?;
        Ok(AccessTokenRequest {
            hibp_check: self.hibp_check,
            name,
            password,
            app_id: self.app_id,
            app_version: self.app_version,
            device_id: self.device_id,
            cid: self.cid,
            sec: self.sec,
        })
    }
}

impl crate::api::current::support::CurrentRequest for AccessTokenRequest {
    fn validate_current(&self) -> Result<(), crate::Error> {
        Ok(())
    }
}

/// Current wire model `OAuthMeResponse`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct OAuthMeResponse {
    #[serde(rename = "errorText", default, skip_serializing_if = "Option::is_none")]
    error_text: Option<String>,
    #[serde(rename = "userId", default, skip_serializing_if = "Option::is_none")]
    user_id: Option<crate::UserId>,
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(rename = "fullName", default, skip_serializing_if = "Option::is_none")]
    full_name: Option<String>,
    #[serde(rename = "email", default, skip_serializing_if = "Option::is_none")]
    email: Option<String>,
    #[serde(
        rename = "emailVerified",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    email_verified: Option<bool>,
    #[serde(rename = "isTrial", default, skip_serializing_if = "Option::is_none")]
    is_trial: Option<bool>,
    #[serde(
        rename = "organizationName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    organization_name: Option<String>,
    #[serde(
        rename = "currentAccountPlan",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    current_account_plan: Option<String>,
    #[serde(
        rename = "currentMDSubs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    current_md_subs: Option<Vec<String>>,
    #[serde(
        rename = "currentBalance",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(with = "crate::decimal::option")]
    current_balance: Option<crate::Decimal>,
    #[serde(
        rename = "activePlugins",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    active_plugins: Option<Vec<String>>,
}

impl OAuthMeResponse {
    /// Returns wire field `errorText`.
    #[must_use]
    pub fn error_text(&self) -> Option<&str> {
        self.error_text.as_deref()
    }

    /// Returns wire field `userId`.
    #[must_use]
    pub fn user_id(&self) -> Option<&crate::UserId> {
        self.user_id.as_ref()
    }

    /// Returns wire field `name`.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Returns wire field `fullName`.
    #[must_use]
    pub fn full_name(&self) -> Option<&str> {
        self.full_name.as_deref()
    }

    /// Returns wire field `email`.
    #[must_use]
    pub fn email(&self) -> Option<&str> {
        self.email.as_deref()
    }

    /// Returns wire field `emailVerified`.
    #[must_use]
    pub fn email_verified(&self) -> Option<&bool> {
        self.email_verified.as_ref()
    }

    /// Returns wire field `isTrial`.
    #[must_use]
    pub fn is_trial(&self) -> Option<&bool> {
        self.is_trial.as_ref()
    }

    /// Returns wire field `organizationName`.
    #[must_use]
    pub fn organization_name(&self) -> Option<&str> {
        self.organization_name.as_deref()
    }

    /// Returns wire field `currentAccountPlan`.
    #[must_use]
    pub fn current_account_plan(&self) -> Option<&str> {
        self.current_account_plan.as_deref()
    }

    /// Returns wire field `currentMDSubs`.
    #[must_use]
    pub fn current_md_subs(&self) -> Option<&[String]> {
        self.current_md_subs.as_deref()
    }

    /// Returns wire field `currentBalance`.
    #[must_use]
    pub fn current_balance(&self) -> Option<&crate::Decimal> {
        self.current_balance.as_ref()
    }

    /// Returns wire field `activePlugins`.
    #[must_use]
    pub fn active_plugins(&self) -> Option<&[String]> {
        self.active_plugins.as_deref()
    }

    /// Starts a builder for [`OAuthMeResponse`].
    pub fn builder() -> OAuthMeResponseBuilder {
        OAuthMeResponseBuilder::default()
    }
}

/// Builder for [`OAuthMeResponse`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct OAuthMeResponseBuilder {
    error_text: Option<String>,
    user_id: Option<crate::UserId>,
    name: Option<String>,
    full_name: Option<String>,
    email: Option<String>,
    email_verified: Option<bool>,
    is_trial: Option<bool>,
    organization_name: Option<String>,
    current_account_plan: Option<String>,
    current_md_subs: Option<Vec<String>>,
    current_balance: Option<crate::Decimal>,
    active_plugins: Option<Vec<String>>,
}

impl OAuthMeResponseBuilder {
    /// Sets wire field `errorText`.
    pub fn error_text(mut self, value: impl Into<String>) -> Self {
        self.error_text = Some(value.into());
        self
    }

    /// Sets wire field `userId`.
    pub fn user_id(mut self, value: crate::UserId) -> Self {
        self.user_id = Some(value);
        self
    }

    /// Sets wire field `name`.
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Sets wire field `fullName`.
    pub fn full_name(mut self, value: impl Into<String>) -> Self {
        self.full_name = Some(value.into());
        self
    }

    /// Sets wire field `email`.
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    /// Sets wire field `emailVerified`.
    pub fn email_verified(mut self, value: bool) -> Self {
        self.email_verified = Some(value);
        self
    }

    /// Sets wire field `isTrial`.
    pub fn is_trial(mut self, value: bool) -> Self {
        self.is_trial = Some(value);
        self
    }

    /// Sets wire field `organizationName`.
    pub fn organization_name(mut self, value: impl Into<String>) -> Self {
        self.organization_name = Some(value.into());
        self
    }

    /// Sets wire field `currentAccountPlan`.
    pub fn current_account_plan(mut self, value: impl Into<String>) -> Self {
        self.current_account_plan = Some(value.into());
        self
    }

    /// Sets wire field `currentMDSubs`.
    pub fn current_md_subs(mut self, value: Vec<String>) -> Self {
        self.current_md_subs = Some(value);
        self
    }

    /// Sets wire field `currentBalance`.
    pub fn current_balance(mut self, value: crate::Decimal) -> Self {
        self.current_balance = Some(value);
        self
    }

    /// Sets wire field `activePlugins`.
    pub fn active_plugins(mut self, value: Vec<String>) -> Self {
        self.active_plugins = Some(value);
        self
    }

    /// Validates required fields and builds [`OAuthMeResponse`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<OAuthMeResponse, crate::api::current::BuildError> {
        Ok(OAuthMeResponse {
            error_text: self.error_text,
            user_id: self.user_id,
            name: self.name,
            full_name: self.full_name,
            email: self.email,
            email_verified: self.email_verified,
            is_trial: self.is_trial,
            organization_name: self.organization_name,
            current_account_plan: self.current_account_plan,
            current_md_subs: self.current_md_subs,
            current_balance: self.current_balance,
            active_plugins: self.active_plugins,
        })
    }
}

/// Current wire model `OAuthToken`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct OAuthToken {
    #[serde(rename = "grant_type")]
    grant_type: String,
    #[serde(rename = "code", default, skip_serializing_if = "Option::is_none")]
    code: Option<crate::api::current::SecretValue>,
    #[serde(
        rename = "redirect_uri",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    redirect_uri: Option<String>,
    #[serde(rename = "client_id", default, skip_serializing_if = "Option::is_none")]
    client_id: Option<String>,
    #[serde(
        rename = "client_secret",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    client_secret: Option<crate::api::current::SecretValue>,
    #[serde(rename = "httpAuth", default, skip_serializing_if = "Option::is_none")]
    http_auth: Option<crate::api::current::SecretValue>,
    #[serde(
        rename = "refresh_token",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    refresh_token: Option<crate::api::current::SecretValue>,
    #[serde(
        rename = "code_verifier",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    code_verifier: Option<crate::api::current::SecretValue>,
    #[serde(rename = "resource", default, skip_serializing_if = "Option::is_none")]
    resource: Option<String>,
    #[serde(rename = "assertion", default, skip_serializing_if = "Option::is_none")]
    assertion: Option<crate::api::current::SecretValue>,
}

impl OAuthToken {
    /// Returns wire field `grant_type`.
    #[must_use]
    pub fn grant_type(&self) -> &str {
        &self.grant_type
    }

    /// Reports whether secret field `code` is present.
    #[must_use]
    pub const fn has_code(&self) -> bool {
        self.code.is_some()
    }

    /// Returns wire field `redirect_uri`.
    #[must_use]
    pub fn redirect_uri(&self) -> Option<&str> {
        self.redirect_uri.as_deref()
    }

    /// Returns wire field `client_id`.
    #[must_use]
    pub fn client_id(&self) -> Option<&str> {
        self.client_id.as_deref()
    }

    /// Reports whether secret field `client_secret` is present.
    #[must_use]
    pub const fn has_client_secret(&self) -> bool {
        self.client_secret.is_some()
    }

    /// Reports whether secret field `httpAuth` is present.
    #[must_use]
    pub const fn has_http_auth(&self) -> bool {
        self.http_auth.is_some()
    }

    /// Reports whether secret field `refresh_token` is present.
    #[must_use]
    pub const fn has_refresh_token(&self) -> bool {
        self.refresh_token.is_some()
    }

    /// Reports whether secret field `code_verifier` is present.
    #[must_use]
    pub const fn has_code_verifier(&self) -> bool {
        self.code_verifier.is_some()
    }

    /// Returns wire field `resource`.
    #[must_use]
    pub fn resource(&self) -> Option<&str> {
        self.resource.as_deref()
    }

    /// Reports whether secret field `assertion` is present.
    #[must_use]
    pub const fn has_assertion(&self) -> bool {
        self.assertion.is_some()
    }

    /// Starts a builder for [`OAuthToken`].
    pub fn builder() -> OAuthTokenBuilder {
        OAuthTokenBuilder::default()
    }
}

/// Builder for [`OAuthToken`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct OAuthTokenBuilder {
    grant_type: Option<String>,
    code: Option<crate::api::current::SecretValue>,
    redirect_uri: Option<String>,
    client_id: Option<String>,
    client_secret: Option<crate::api::current::SecretValue>,
    http_auth: Option<crate::api::current::SecretValue>,
    refresh_token: Option<crate::api::current::SecretValue>,
    code_verifier: Option<crate::api::current::SecretValue>,
    resource: Option<String>,
    assertion: Option<crate::api::current::SecretValue>,
}

impl OAuthTokenBuilder {
    /// Sets wire field `grant_type`.
    pub fn grant_type(mut self, value: impl Into<String>) -> Self {
        self.grant_type = Some(value.into());
        self
    }

    /// Sets wire field `code`.
    pub fn code(mut self, value: crate::api::current::SecretValue) -> Self {
        self.code = Some(value);
        self
    }

    /// Sets wire field `redirect_uri`.
    pub fn redirect_uri(mut self, value: impl Into<String>) -> Self {
        self.redirect_uri = Some(value.into());
        self
    }

    /// Sets wire field `client_id`.
    pub fn client_id(mut self, value: impl Into<String>) -> Self {
        self.client_id = Some(value.into());
        self
    }

    /// Sets wire field `client_secret`.
    pub fn client_secret(mut self, value: crate::api::current::SecretValue) -> Self {
        self.client_secret = Some(value);
        self
    }

    /// Sets wire field `httpAuth`.
    pub fn http_auth(mut self, value: crate::api::current::SecretValue) -> Self {
        self.http_auth = Some(value);
        self
    }

    /// Sets wire field `refresh_token`.
    pub fn refresh_token(mut self, value: crate::api::current::SecretValue) -> Self {
        self.refresh_token = Some(value);
        self
    }

    /// Sets wire field `code_verifier`.
    pub fn code_verifier(mut self, value: crate::api::current::SecretValue) -> Self {
        self.code_verifier = Some(value);
        self
    }

    /// Sets wire field `resource`.
    pub fn resource(mut self, value: impl Into<String>) -> Self {
        self.resource = Some(value.into());
        self
    }

    /// Sets wire field `assertion`.
    pub fn assertion(mut self, value: crate::api::current::SecretValue) -> Self {
        self.assertion = Some(value);
        self
    }

    /// Validates required fields and builds [`OAuthToken`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<OAuthToken, crate::api::current::BuildError> {
        let grant_type = self
            .grant_type
            .ok_or(crate::api::current::BuildError::missing("grant_type"))?;
        if grant_type.is_empty() || grant_type.trim() != grant_type {
            return Err(crate::api::current::BuildError::invalid(
                "grant_type",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        Ok(OAuthToken {
            grant_type,
            code: self.code,
            redirect_uri: self.redirect_uri,
            client_id: self.client_id,
            client_secret: self.client_secret,
            http_auth: self.http_auth,
            refresh_token: self.refresh_token,
            code_verifier: self.code_verifier,
            resource: self.resource,
            assertion: self.assertion,
        })
    }
}

impl crate::api::current::support::CurrentRequest for OAuthToken {
    fn validate_current(&self) -> Result<(), crate::Error> {
        if self.grant_type.is_empty() || self.grant_type.trim() != self.grant_type {
            return Err(crate::Error::InvalidRequest {
                field: "grant_type",
                reason: "must be non-empty and have no surrounding whitespace",
            });
        }
        Ok(())
    }
}

/// Current wire model `OAuthTokenResponse`.
#[derive(Clone, Debug, serde::Deserialize)]
#[non_exhaustive]
pub struct OAuthTokenResponse {
    #[serde(
        rename = "access_token",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    access_token: Option<crate::api::current::SecretValue>,
    #[serde(
        rename = "refresh_token",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    refresh_token: Option<crate::api::current::SecretValue>,
    #[serde(
        rename = "token_type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    token_type: Option<String>,
    #[serde(
        rename = "expires_in",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    expires_in: Option<i64>,
    #[serde(
        rename = "refresh_token_expires_in",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    refresh_token_expires_in: Option<i64>,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    error: Option<String>,
    #[serde(
        rename = "error_description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    error_description: Option<String>,
    #[serde(rename = "id_token", default, skip_serializing_if = "Option::is_none")]
    id_token: Option<crate::api::current::SecretValue>,
}

impl OAuthTokenResponse {
    /// Reports whether secret field `access_token` is present.
    #[must_use]
    pub const fn has_access_token(&self) -> bool {
        self.access_token.is_some()
    }

    pub(crate) fn access_token_secret(&self) -> Option<&crate::api::current::SecretValue> {
        self.access_token.as_ref()
    }

    /// Reports whether secret field `refresh_token` is present.
    #[must_use]
    pub const fn has_refresh_token(&self) -> bool {
        self.refresh_token.is_some()
    }

    /// Returns wire field `token_type`.
    #[must_use]
    pub fn token_type(&self) -> Option<&str> {
        self.token_type.as_deref()
    }

    /// Returns wire field `expires_in`.
    #[must_use]
    pub fn expires_in(&self) -> Option<&i64> {
        self.expires_in.as_ref()
    }

    /// Returns wire field `refresh_token_expires_in`.
    #[must_use]
    pub fn refresh_token_expires_in(&self) -> Option<&i64> {
        self.refresh_token_expires_in.as_ref()
    }

    /// Returns wire field `error`.
    #[must_use]
    pub fn error(&self) -> Option<&str> {
        self.error.as_deref()
    }

    /// Returns wire field `error_description`.
    #[must_use]
    pub fn error_description(&self) -> Option<&str> {
        self.error_description.as_deref()
    }

    /// Reports whether secret field `id_token` is present.
    #[must_use]
    pub const fn has_id_token(&self) -> bool {
        self.id_token.is_some()
    }
}

impl crate::Client {
    /// Calls the current `GET /auth/me` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn auth_me(&self) -> Result<OAuthMeResponse, crate::Error> {
        self.get_without_query("/auth/me").await
    }
}

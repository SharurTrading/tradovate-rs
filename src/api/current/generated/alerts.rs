// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary
// @generated
// Generator: tools/generate_openapi.py
// Source: https://partner.tradovate.com/openapi.json (snapshot 2026-08-21, sha256 37caeccf4b0913460a788fcaf4c902497059b8ffe6f6355512e6c08eaacde769)

// Provider wire fields remain schema-auditable even when they repeat
// their type name; wide schema-faithful builders remain one generated
// unit so regeneration and source review cannot drift field subsets.
#![allow(clippy::struct_field_names, clippy::too_many_lines)]

//! Current alert operations and wire models.

/// Current wire model `AdminAlertSignal`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct AdminAlertSignal {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    id: Option<super::ids::AdminAlertSignalId>,
    #[serde(rename = "timestamp")]
    timestamp: jiff::Timestamp,
    #[serde(rename = "adminAlertId")]
    admin_alert_id: super::ids::AdminAlertId,
    #[serde(
        rename = "relatedToAccountId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    related_to_account_id: Option<crate::AccountId>,
    #[serde(
        rename = "relatedToUserId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    related_to_user_id: Option<crate::UserId>,
    #[serde(
        rename = "ownedByAdminId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    owned_by_admin_id: Option<super::ids::OwnedByAdminId>,
    #[serde(rename = "completed", default, skip_serializing_if = "Option::is_none")]
    completed: Option<jiff::Timestamp>,
    #[serde(rename = "text")]
    text: String,
    #[serde(rename = "emailSent")]
    email_sent: bool,
    #[serde(rename = "subjectId")]
    subject_id: super::ids::SubjectId,
    #[serde(rename = "claimedAt", default, skip_serializing_if = "Option::is_none")]
    claimed_at: Option<jiff::Timestamp>,
}

impl AdminAlertSignal {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> Option<&super::ids::AdminAlertSignalId> {
        self.id.as_ref()
    }

    /// Returns wire field `timestamp`.
    #[must_use]
    pub fn timestamp(&self) -> &jiff::Timestamp {
        &self.timestamp
    }

    /// Returns wire field `adminAlertId`.
    #[must_use]
    pub fn admin_alert_id(&self) -> &super::ids::AdminAlertId {
        &self.admin_alert_id
    }

    /// Returns wire field `relatedToAccountId`.
    #[must_use]
    pub fn related_to_account_id(&self) -> Option<&crate::AccountId> {
        self.related_to_account_id.as_ref()
    }

    /// Returns wire field `relatedToUserId`.
    #[must_use]
    pub fn related_to_user_id(&self) -> Option<&crate::UserId> {
        self.related_to_user_id.as_ref()
    }

    /// Returns wire field `ownedByAdminId`.
    #[must_use]
    pub fn owned_by_admin_id(&self) -> Option<&super::ids::OwnedByAdminId> {
        self.owned_by_admin_id.as_ref()
    }

    /// Returns wire field `completed`.
    #[must_use]
    pub fn completed(&self) -> Option<&jiff::Timestamp> {
        self.completed.as_ref()
    }

    /// Returns wire field `text`.
    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Returns wire field `emailSent`.
    #[must_use]
    pub fn email_sent(&self) -> &bool {
        &self.email_sent
    }

    /// Returns wire field `subjectId`.
    #[must_use]
    pub fn subject_id(&self) -> &super::ids::SubjectId {
        &self.subject_id
    }

    /// Returns wire field `claimedAt`.
    #[must_use]
    pub fn claimed_at(&self) -> Option<&jiff::Timestamp> {
        self.claimed_at.as_ref()
    }

    /// Starts a builder for [`AdminAlertSignal`].
    pub fn builder() -> AdminAlertSignalBuilder {
        AdminAlertSignalBuilder::default()
    }
}

/// Builder for [`AdminAlertSignal`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct AdminAlertSignalBuilder {
    id: Option<super::ids::AdminAlertSignalId>,
    timestamp: Option<jiff::Timestamp>,
    admin_alert_id: Option<super::ids::AdminAlertId>,
    related_to_account_id: Option<crate::AccountId>,
    related_to_user_id: Option<crate::UserId>,
    owned_by_admin_id: Option<super::ids::OwnedByAdminId>,
    completed: Option<jiff::Timestamp>,
    text: Option<String>,
    email_sent: Option<bool>,
    subject_id: Option<super::ids::SubjectId>,
    claimed_at: Option<jiff::Timestamp>,
}

impl AdminAlertSignalBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::AdminAlertSignalId) -> Self {
        self.id = Some(value);
        self
    }

    /// Sets wire field `timestamp`.
    pub fn timestamp(mut self, value: jiff::Timestamp) -> Self {
        self.timestamp = Some(value);
        self
    }

    /// Sets wire field `adminAlertId`.
    pub fn admin_alert_id(mut self, value: super::ids::AdminAlertId) -> Self {
        self.admin_alert_id = Some(value);
        self
    }

    /// Sets wire field `relatedToAccountId`.
    pub fn related_to_account_id(mut self, value: crate::AccountId) -> Self {
        self.related_to_account_id = Some(value);
        self
    }

    /// Sets wire field `relatedToUserId`.
    pub fn related_to_user_id(mut self, value: crate::UserId) -> Self {
        self.related_to_user_id = Some(value);
        self
    }

    /// Sets wire field `ownedByAdminId`.
    pub fn owned_by_admin_id(mut self, value: super::ids::OwnedByAdminId) -> Self {
        self.owned_by_admin_id = Some(value);
        self
    }

    /// Sets wire field `completed`.
    pub fn completed(mut self, value: jiff::Timestamp) -> Self {
        self.completed = Some(value);
        self
    }

    /// Sets wire field `text`.
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Sets wire field `emailSent`.
    pub fn email_sent(mut self, value: bool) -> Self {
        self.email_sent = Some(value);
        self
    }

    /// Sets wire field `subjectId`.
    pub fn subject_id(mut self, value: super::ids::SubjectId) -> Self {
        self.subject_id = Some(value);
        self
    }

    /// Sets wire field `claimedAt`.
    pub fn claimed_at(mut self, value: jiff::Timestamp) -> Self {
        self.claimed_at = Some(value);
        self
    }

    /// Validates required fields and builds [`AdminAlertSignal`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<AdminAlertSignal, crate::api::current::BuildError> {
        let timestamp = self
            .timestamp
            .ok_or(crate::api::current::BuildError::missing("timestamp"))?;
        let admin_alert_id = self
            .admin_alert_id
            .ok_or(crate::api::current::BuildError::missing("adminAlertId"))?;
        let text = self
            .text
            .ok_or(crate::api::current::BuildError::missing("text"))?;
        let email_sent = self
            .email_sent
            .ok_or(crate::api::current::BuildError::missing("emailSent"))?;
        let subject_id = self
            .subject_id
            .ok_or(crate::api::current::BuildError::missing("subjectId"))?;
        Ok(AdminAlertSignal {
            id: self.id,
            timestamp,
            admin_alert_id,
            related_to_account_id: self.related_to_account_id,
            related_to_user_id: self.related_to_user_id,
            owned_by_admin_id: self.owned_by_admin_id,
            completed: self.completed,
            text,
            email_sent,
            subject_id,
            claimed_at: self.claimed_at,
        })
    }
}

/// Current wire model `AdminAlertSignalResponse`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct AdminAlertSignalResponse {
    #[serde(rename = "errorText", default, skip_serializing_if = "Option::is_none")]
    error_text: Option<String>,
    #[serde(
        rename = "adminAlertSignal",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    admin_alert_signal: Option<AdminAlertSignal>,
}

impl AdminAlertSignalResponse {
    /// Returns wire field `errorText`.
    #[must_use]
    pub fn error_text(&self) -> Option<&str> {
        self.error_text.as_deref()
    }

    /// Returns wire field `adminAlertSignal`.
    #[must_use]
    pub fn admin_alert_signal(&self) -> Option<&AdminAlertSignal> {
        self.admin_alert_signal.as_ref()
    }

    /// Starts a builder for [`AdminAlertSignalResponse`].
    pub fn builder() -> AdminAlertSignalResponseBuilder {
        AdminAlertSignalResponseBuilder::default()
    }
}

/// Builder for [`AdminAlertSignalResponse`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct AdminAlertSignalResponseBuilder {
    error_text: Option<String>,
    admin_alert_signal: Option<AdminAlertSignal>,
}

impl AdminAlertSignalResponseBuilder {
    /// Sets wire field `errorText`.
    pub fn error_text(mut self, value: impl Into<String>) -> Self {
        self.error_text = Some(value.into());
        self
    }

    /// Sets wire field `adminAlertSignal`.
    pub fn admin_alert_signal(mut self, value: AdminAlertSignal) -> Self {
        self.admin_alert_signal = Some(value);
        self
    }

    /// Validates required fields and builds [`AdminAlertSignalResponse`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<AdminAlertSignalResponse, crate::api::current::BuildError> {
        Ok(AdminAlertSignalResponse {
            error_text: self.error_text,
            admin_alert_signal: self.admin_alert_signal,
        })
    }
}

/// Current wire model `Alert`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct Alert {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    id: Option<super::ids::AlertId>,
    #[serde(rename = "timestamp")]
    timestamp: jiff::Timestamp,
    #[serde(rename = "userId")]
    user_id: crate::UserId,
    #[serde(rename = "status")]
    status: AlertStatus,
    #[serde(rename = "expression")]
    expression: String,
    #[serde(
        rename = "validUntil",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    valid_until: Option<jiff::Timestamp>,
    #[serde(
        rename = "triggerLimits",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    trigger_limits: Option<i64>,
    #[serde(
        rename = "triggeredCounter",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    triggered_counter: Option<i64>,
    #[serde(rename = "failure", default, skip_serializing_if = "Option::is_none")]
    failure: Option<String>,
    #[serde(rename = "message", default, skip_serializing_if = "Option::is_none")]
    message: Option<String>,
}

impl Alert {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> Option<&super::ids::AlertId> {
        self.id.as_ref()
    }

    /// Returns wire field `timestamp`.
    #[must_use]
    pub fn timestamp(&self) -> &jiff::Timestamp {
        &self.timestamp
    }

    /// Returns wire field `userId`.
    #[must_use]
    pub fn user_id(&self) -> &crate::UserId {
        &self.user_id
    }

    /// Returns wire field `status`.
    #[must_use]
    pub fn status(&self) -> &AlertStatus {
        &self.status
    }

    /// Returns wire field `expression`.
    #[must_use]
    pub fn expression(&self) -> &str {
        &self.expression
    }

    /// Returns wire field `validUntil`.
    #[must_use]
    pub fn valid_until(&self) -> Option<&jiff::Timestamp> {
        self.valid_until.as_ref()
    }

    /// Returns wire field `triggerLimits`.
    #[must_use]
    pub fn trigger_limits(&self) -> Option<&i64> {
        self.trigger_limits.as_ref()
    }

    /// Returns wire field `triggeredCounter`.
    #[must_use]
    pub fn triggered_counter(&self) -> Option<&i64> {
        self.triggered_counter.as_ref()
    }

    /// Returns wire field `failure`.
    #[must_use]
    pub fn failure(&self) -> Option<&str> {
        self.failure.as_deref()
    }

    /// Returns wire field `message`.
    #[must_use]
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }

    /// Starts a builder for [`Alert`].
    pub fn builder() -> AlertBuilder {
        AlertBuilder::default()
    }
}

/// Builder for [`Alert`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct AlertBuilder {
    id: Option<super::ids::AlertId>,
    timestamp: Option<jiff::Timestamp>,
    user_id: Option<crate::UserId>,
    status: Option<AlertStatus>,
    expression: Option<String>,
    valid_until: Option<jiff::Timestamp>,
    trigger_limits: Option<i64>,
    triggered_counter: Option<i64>,
    failure: Option<String>,
    message: Option<String>,
}

impl AlertBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::AlertId) -> Self {
        self.id = Some(value);
        self
    }

    /// Sets wire field `timestamp`.
    pub fn timestamp(mut self, value: jiff::Timestamp) -> Self {
        self.timestamp = Some(value);
        self
    }

    /// Sets wire field `userId`.
    pub fn user_id(mut self, value: crate::UserId) -> Self {
        self.user_id = Some(value);
        self
    }

    /// Sets wire field `status`.
    pub fn status(mut self, value: AlertStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Sets wire field `expression`.
    pub fn expression(mut self, value: impl Into<String>) -> Self {
        self.expression = Some(value.into());
        self
    }

    /// Sets wire field `validUntil`.
    pub fn valid_until(mut self, value: jiff::Timestamp) -> Self {
        self.valid_until = Some(value);
        self
    }

    /// Sets wire field `triggerLimits`.
    pub fn trigger_limits(mut self, value: i64) -> Self {
        self.trigger_limits = Some(value);
        self
    }

    /// Sets wire field `triggeredCounter`.
    pub fn triggered_counter(mut self, value: i64) -> Self {
        self.triggered_counter = Some(value);
        self
    }

    /// Sets wire field `failure`.
    pub fn failure(mut self, value: impl Into<String>) -> Self {
        self.failure = Some(value.into());
        self
    }

    /// Sets wire field `message`.
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Validates required fields and builds [`Alert`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<Alert, crate::api::current::BuildError> {
        let timestamp = self
            .timestamp
            .ok_or(crate::api::current::BuildError::missing("timestamp"))?;
        let user_id = self
            .user_id
            .ok_or(crate::api::current::BuildError::missing("userId"))?;
        let status = self
            .status
            .ok_or(crate::api::current::BuildError::missing("status"))?;
        let expression = self
            .expression
            .ok_or(crate::api::current::BuildError::missing("expression"))?;
        Ok(Alert {
            id: self.id,
            timestamp,
            user_id,
            status,
            expression,
            valid_until: self.valid_until,
            trigger_limits: self.trigger_limits,
            triggered_counter: self.triggered_counter,
            failure: self.failure,
            message: self.message,
        })
    }
}

/// Current wire model `AlertResponse`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct AlertResponse {
    #[serde(rename = "errorText", default, skip_serializing_if = "Option::is_none")]
    error_text: Option<String>,
    #[serde(rename = "alert", default, skip_serializing_if = "Option::is_none")]
    alert: Option<Alert>,
}

impl AlertResponse {
    /// Returns wire field `errorText`.
    #[must_use]
    pub fn error_text(&self) -> Option<&str> {
        self.error_text.as_deref()
    }

    /// Returns wire field `alert`.
    #[must_use]
    pub fn alert(&self) -> Option<&Alert> {
        self.alert.as_ref()
    }

    /// Starts a builder for [`AlertResponse`].
    pub fn builder() -> AlertResponseBuilder {
        AlertResponseBuilder::default()
    }
}

/// Builder for [`AlertResponse`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct AlertResponseBuilder {
    error_text: Option<String>,
    alert: Option<Alert>,
}

impl AlertResponseBuilder {
    /// Sets wire field `errorText`.
    pub fn error_text(mut self, value: impl Into<String>) -> Self {
        self.error_text = Some(value.into());
        self
    }

    /// Sets wire field `alert`.
    pub fn alert(mut self, value: Alert) -> Self {
        self.alert = Some(value);
        self
    }

    /// Validates required fields and builds [`AlertResponse`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<AlertResponse, crate::api::current::BuildError> {
        Ok(AlertResponse {
            error_text: self.error_text,
            alert: self.alert,
        })
    }
}

/// Current wire model `AlertSignal`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct AlertSignal {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    id: Option<super::ids::AlertSignalId>,
    #[serde(rename = "timestamp")]
    timestamp: jiff::Timestamp,
    #[serde(rename = "alertId")]
    alert_id: super::ids::AlertId,
    #[serde(rename = "isRead")]
    is_read: bool,
    #[serde(rename = "text")]
    text: String,
}

impl AlertSignal {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> Option<&super::ids::AlertSignalId> {
        self.id.as_ref()
    }

    /// Returns wire field `timestamp`.
    #[must_use]
    pub fn timestamp(&self) -> &jiff::Timestamp {
        &self.timestamp
    }

    /// Returns wire field `alertId`.
    #[must_use]
    pub fn alert_id(&self) -> &super::ids::AlertId {
        &self.alert_id
    }

    /// Returns wire field `isRead`.
    #[must_use]
    pub fn is_read(&self) -> &bool {
        &self.is_read
    }

    /// Returns wire field `text`.
    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Starts a builder for [`AlertSignal`].
    pub fn builder() -> AlertSignalBuilder {
        AlertSignalBuilder::default()
    }
}

/// Builder for [`AlertSignal`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct AlertSignalBuilder {
    id: Option<super::ids::AlertSignalId>,
    timestamp: Option<jiff::Timestamp>,
    alert_id: Option<super::ids::AlertId>,
    is_read: Option<bool>,
    text: Option<String>,
}

impl AlertSignalBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::AlertSignalId) -> Self {
        self.id = Some(value);
        self
    }

    /// Sets wire field `timestamp`.
    pub fn timestamp(mut self, value: jiff::Timestamp) -> Self {
        self.timestamp = Some(value);
        self
    }

    /// Sets wire field `alertId`.
    pub fn alert_id(mut self, value: super::ids::AlertId) -> Self {
        self.alert_id = Some(value);
        self
    }

    /// Sets wire field `isRead`.
    pub fn is_read(mut self, value: bool) -> Self {
        self.is_read = Some(value);
        self
    }

    /// Sets wire field `text`.
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Validates required fields and builds [`AlertSignal`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<AlertSignal, crate::api::current::BuildError> {
        let timestamp = self
            .timestamp
            .ok_or(crate::api::current::BuildError::missing("timestamp"))?;
        let alert_id = self
            .alert_id
            .ok_or(crate::api::current::BuildError::missing("alertId"))?;
        let is_read = self
            .is_read
            .ok_or(crate::api::current::BuildError::missing("isRead"))?;
        let text = self
            .text
            .ok_or(crate::api::current::BuildError::missing("text"))?;
        Ok(AlertSignal {
            id: self.id,
            timestamp,
            alert_id,
            is_read,
            text,
        })
    }
}

/// Current provider values for `AlertStatus`.
///
/// Unknown response values are preserved for forward compatibility but cannot
/// be serialized into a request.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum AlertStatus {
    /// Provider value `Active`.
    Active,
    /// Provider value `Expired`.
    Expired,
    /// Provider value `Failed`.
    Failed,
    /// Provider value `Inactive`.
    Inactive,
    /// Provider value `TriggeredOut`.
    TriggeredOut,
    /// A provider value added after the pinned specification.
    Unknown(String),
}

impl AlertStatus {
    /// Returns the exact provider spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Active => "Active",
            Self::Expired => "Expired",
            Self::Failed => "Failed",
            Self::Inactive => "Inactive",
            Self::TriggeredOut => "TriggeredOut",
            Self::Unknown(value) => value,
        }
    }
}

impl serde::Serialize for AlertStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if matches!(self, Self::Unknown(_)) {
            return Err(serde::ser::Error::custom(
                "undocumented enum values cannot be sent",
            ));
        }
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for AlertStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "Active" => Self::Active,
            "Expired" => Self::Expired,
            "Failed" => Self::Failed,
            "Inactive" => Self::Inactive,
            "TriggeredOut" => Self::TriggeredOut,
            _ => Self::Unknown(value),
        })
    }
}

/// Current wire model `CompleteAlertSignal`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct CompleteAlertSignal {
    #[serde(rename = "adminAlertSignalId")]
    admin_alert_signal_id: super::ids::AdminAlertSignalId,
}

impl CompleteAlertSignal {
    /// Returns wire field `adminAlertSignalId`.
    #[must_use]
    pub fn admin_alert_signal_id(&self) -> &super::ids::AdminAlertSignalId {
        &self.admin_alert_signal_id
    }

    /// Starts a builder for [`CompleteAlertSignal`].
    pub fn builder() -> CompleteAlertSignalBuilder {
        CompleteAlertSignalBuilder::default()
    }
}

/// Builder for [`CompleteAlertSignal`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct CompleteAlertSignalBuilder {
    admin_alert_signal_id: Option<super::ids::AdminAlertSignalId>,
}

impl CompleteAlertSignalBuilder {
    /// Sets wire field `adminAlertSignalId`.
    pub fn admin_alert_signal_id(mut self, value: super::ids::AdminAlertSignalId) -> Self {
        self.admin_alert_signal_id = Some(value);
        self
    }

    /// Validates required fields and builds [`CompleteAlertSignal`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<CompleteAlertSignal, crate::api::current::BuildError> {
        let admin_alert_signal_id =
            self.admin_alert_signal_id
                .ok_or(crate::api::current::BuildError::missing(
                    "adminAlertSignalId",
                ))?;
        Ok(CompleteAlertSignal {
            admin_alert_signal_id,
        })
    }
}

impl crate::api::current::support::CurrentRequest for CompleteAlertSignal {
    fn validate_current(&self) -> Result<(), crate::Error> {
        Ok(())
    }
}

/// Current wire model `CreateAlert`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct CreateAlert {
    #[serde(rename = "expression")]
    expression: String,
    #[serde(
        rename = "validUntil",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    valid_until: Option<jiff::Timestamp>,
    #[serde(
        rename = "triggerLimits",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    trigger_limits: Option<i64>,
    #[serde(rename = "message", default, skip_serializing_if = "Option::is_none")]
    message: Option<String>,
}

impl CreateAlert {
    /// Returns wire field `expression`.
    #[must_use]
    pub fn expression(&self) -> &str {
        &self.expression
    }

    /// Returns wire field `validUntil`.
    #[must_use]
    pub fn valid_until(&self) -> Option<&jiff::Timestamp> {
        self.valid_until.as_ref()
    }

    /// Returns wire field `triggerLimits`.
    #[must_use]
    pub fn trigger_limits(&self) -> Option<&i64> {
        self.trigger_limits.as_ref()
    }

    /// Returns wire field `message`.
    #[must_use]
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }

    /// Starts a builder for [`CreateAlert`].
    pub fn builder() -> CreateAlertBuilder {
        CreateAlertBuilder::default()
    }
}

/// Builder for [`CreateAlert`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct CreateAlertBuilder {
    expression: Option<String>,
    valid_until: Option<jiff::Timestamp>,
    trigger_limits: Option<i64>,
    message: Option<String>,
}

impl CreateAlertBuilder {
    /// Sets wire field `expression`.
    pub fn expression(mut self, value: impl Into<String>) -> Self {
        self.expression = Some(value.into());
        self
    }

    /// Sets wire field `validUntil`.
    pub fn valid_until(mut self, value: jiff::Timestamp) -> Self {
        self.valid_until = Some(value);
        self
    }

    /// Sets wire field `triggerLimits`.
    pub fn trigger_limits(mut self, value: i64) -> Self {
        self.trigger_limits = Some(value);
        self
    }

    /// Sets wire field `message`.
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Validates required fields and builds [`CreateAlert`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<CreateAlert, crate::api::current::BuildError> {
        let expression = self
            .expression
            .ok_or(crate::api::current::BuildError::missing("expression"))?;
        if expression.is_empty() || expression.trim() != expression {
            return Err(crate::api::current::BuildError::invalid(
                "expression",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        Ok(CreateAlert {
            expression,
            valid_until: self.valid_until,
            trigger_limits: self.trigger_limits,
            message: self.message,
        })
    }
}

impl crate::api::current::support::CurrentRequest for CreateAlert {
    fn validate_current(&self) -> Result<(), crate::Error> {
        if self.expression.is_empty() || self.expression.trim() != self.expression {
            return Err(crate::Error::InvalidRequest {
                field: "expression",
                reason: "must be non-empty and have no surrounding whitespace",
            });
        }
        Ok(())
    }
}

/// Current wire model `DeleteAlert`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct DeleteAlert {
    #[serde(rename = "alertId")]
    alert_id: super::ids::AlertId,
}

impl DeleteAlert {
    /// Returns wire field `alertId`.
    #[must_use]
    pub fn alert_id(&self) -> &super::ids::AlertId {
        &self.alert_id
    }

    /// Starts a builder for [`DeleteAlert`].
    pub fn builder() -> DeleteAlertBuilder {
        DeleteAlertBuilder::default()
    }
}

/// Builder for [`DeleteAlert`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct DeleteAlertBuilder {
    alert_id: Option<super::ids::AlertId>,
}

impl DeleteAlertBuilder {
    /// Sets wire field `alertId`.
    pub fn alert_id(mut self, value: super::ids::AlertId) -> Self {
        self.alert_id = Some(value);
        self
    }

    /// Validates required fields and builds [`DeleteAlert`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<DeleteAlert, crate::api::current::BuildError> {
        let alert_id = self
            .alert_id
            .ok_or(crate::api::current::BuildError::missing("alertId"))?;
        Ok(DeleteAlert { alert_id })
    }
}

impl crate::api::current::support::CurrentRequest for DeleteAlert {
    fn validate_current(&self) -> Result<(), crate::Error> {
        Ok(())
    }
}

/// Current wire model `DismissAlert`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct DismissAlert {
    #[serde(rename = "alertId")]
    alert_id: super::ids::AlertId,
}

impl DismissAlert {
    /// Returns wire field `alertId`.
    #[must_use]
    pub fn alert_id(&self) -> &super::ids::AlertId {
        &self.alert_id
    }

    /// Starts a builder for [`DismissAlert`].
    pub fn builder() -> DismissAlertBuilder {
        DismissAlertBuilder::default()
    }
}

/// Builder for [`DismissAlert`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct DismissAlertBuilder {
    alert_id: Option<super::ids::AlertId>,
}

impl DismissAlertBuilder {
    /// Sets wire field `alertId`.
    pub fn alert_id(mut self, value: super::ids::AlertId) -> Self {
        self.alert_id = Some(value);
        self
    }

    /// Validates required fields and builds [`DismissAlert`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<DismissAlert, crate::api::current::BuildError> {
        let alert_id = self
            .alert_id
            .ok_or(crate::api::current::BuildError::missing("alertId"))?;
        Ok(DismissAlert { alert_id })
    }
}

impl crate::api::current::support::CurrentRequest for DismissAlert {
    fn validate_current(&self) -> Result<(), crate::Error> {
        Ok(())
    }
}

/// Current wire model `MarkReadAlertSignal`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct MarkReadAlertSignal {
    #[serde(rename = "alertId")]
    alert_id: super::ids::AlertId,
    #[serde(rename = "alertSignalId")]
    alert_signal_id: super::ids::AlertSignalId,
}

impl MarkReadAlertSignal {
    /// Returns wire field `alertId`.
    #[must_use]
    pub fn alert_id(&self) -> &super::ids::AlertId {
        &self.alert_id
    }

    /// Returns wire field `alertSignalId`.
    #[must_use]
    pub fn alert_signal_id(&self) -> &super::ids::AlertSignalId {
        &self.alert_signal_id
    }

    /// Starts a builder for [`MarkReadAlertSignal`].
    pub fn builder() -> MarkReadAlertSignalBuilder {
        MarkReadAlertSignalBuilder::default()
    }
}

/// Builder for [`MarkReadAlertSignal`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct MarkReadAlertSignalBuilder {
    alert_id: Option<super::ids::AlertId>,
    alert_signal_id: Option<super::ids::AlertSignalId>,
}

impl MarkReadAlertSignalBuilder {
    /// Sets wire field `alertId`.
    pub fn alert_id(mut self, value: super::ids::AlertId) -> Self {
        self.alert_id = Some(value);
        self
    }

    /// Sets wire field `alertSignalId`.
    pub fn alert_signal_id(mut self, value: super::ids::AlertSignalId) -> Self {
        self.alert_signal_id = Some(value);
        self
    }

    /// Validates required fields and builds [`MarkReadAlertSignal`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<MarkReadAlertSignal, crate::api::current::BuildError> {
        let alert_id = self
            .alert_id
            .ok_or(crate::api::current::BuildError::missing("alertId"))?;
        let alert_signal_id = self
            .alert_signal_id
            .ok_or(crate::api::current::BuildError::missing("alertSignalId"))?;
        Ok(MarkReadAlertSignal {
            alert_id,
            alert_signal_id,
        })
    }
}

impl crate::api::current::support::CurrentRequest for MarkReadAlertSignal {
    fn validate_current(&self) -> Result<(), crate::Error> {
        Ok(())
    }
}

/// Current wire model `ModifyAlert`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ModifyAlert {
    #[serde(rename = "alertId")]
    alert_id: super::ids::AlertId,
    #[serde(rename = "expression")]
    expression: String,
    #[serde(
        rename = "validUntil",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    valid_until: Option<jiff::Timestamp>,
    #[serde(
        rename = "triggerLimits",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    trigger_limits: Option<i64>,
    #[serde(rename = "message", default, skip_serializing_if = "Option::is_none")]
    message: Option<String>,
}

impl ModifyAlert {
    /// Returns wire field `alertId`.
    #[must_use]
    pub fn alert_id(&self) -> &super::ids::AlertId {
        &self.alert_id
    }

    /// Returns wire field `expression`.
    #[must_use]
    pub fn expression(&self) -> &str {
        &self.expression
    }

    /// Returns wire field `validUntil`.
    #[must_use]
    pub fn valid_until(&self) -> Option<&jiff::Timestamp> {
        self.valid_until.as_ref()
    }

    /// Returns wire field `triggerLimits`.
    #[must_use]
    pub fn trigger_limits(&self) -> Option<&i64> {
        self.trigger_limits.as_ref()
    }

    /// Returns wire field `message`.
    #[must_use]
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }

    /// Starts a builder for [`ModifyAlert`].
    pub fn builder() -> ModifyAlertBuilder {
        ModifyAlertBuilder::default()
    }
}

/// Builder for [`ModifyAlert`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ModifyAlertBuilder {
    alert_id: Option<super::ids::AlertId>,
    expression: Option<String>,
    valid_until: Option<jiff::Timestamp>,
    trigger_limits: Option<i64>,
    message: Option<String>,
}

impl ModifyAlertBuilder {
    /// Sets wire field `alertId`.
    pub fn alert_id(mut self, value: super::ids::AlertId) -> Self {
        self.alert_id = Some(value);
        self
    }

    /// Sets wire field `expression`.
    pub fn expression(mut self, value: impl Into<String>) -> Self {
        self.expression = Some(value.into());
        self
    }

    /// Sets wire field `validUntil`.
    pub fn valid_until(mut self, value: jiff::Timestamp) -> Self {
        self.valid_until = Some(value);
        self
    }

    /// Sets wire field `triggerLimits`.
    pub fn trigger_limits(mut self, value: i64) -> Self {
        self.trigger_limits = Some(value);
        self
    }

    /// Sets wire field `message`.
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Validates required fields and builds [`ModifyAlert`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ModifyAlert, crate::api::current::BuildError> {
        let alert_id = self
            .alert_id
            .ok_or(crate::api::current::BuildError::missing("alertId"))?;
        let expression = self
            .expression
            .ok_or(crate::api::current::BuildError::missing("expression"))?;
        if expression.is_empty() || expression.trim() != expression {
            return Err(crate::api::current::BuildError::invalid(
                "expression",
                "must be non-empty and have no surrounding whitespace",
            ));
        }
        Ok(ModifyAlert {
            alert_id,
            expression,
            valid_until: self.valid_until,
            trigger_limits: self.trigger_limits,
            message: self.message,
        })
    }
}

impl crate::api::current::support::CurrentRequest for ModifyAlert {
    fn validate_current(&self) -> Result<(), crate::Error> {
        if self.expression.is_empty() || self.expression.trim() != self.expression {
            return Err(crate::Error::InvalidRequest {
                field: "expression",
                reason: "must be non-empty and have no surrounding whitespace",
            });
        }
        Ok(())
    }
}

/// Current wire model `ResetAlert`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct ResetAlert {
    #[serde(rename = "alertId")]
    alert_id: super::ids::AlertId,
}

impl ResetAlert {
    /// Returns wire field `alertId`.
    #[must_use]
    pub fn alert_id(&self) -> &super::ids::AlertId {
        &self.alert_id
    }

    /// Starts a builder for [`ResetAlert`].
    pub fn builder() -> ResetAlertBuilder {
        ResetAlertBuilder::default()
    }
}

/// Builder for [`ResetAlert`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct ResetAlertBuilder {
    alert_id: Option<super::ids::AlertId>,
}

impl ResetAlertBuilder {
    /// Sets wire field `alertId`.
    pub fn alert_id(mut self, value: super::ids::AlertId) -> Self {
        self.alert_id = Some(value);
        self
    }

    /// Validates required fields and builds [`ResetAlert`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<ResetAlert, crate::api::current::BuildError> {
        let alert_id = self
            .alert_id
            .ok_or(crate::api::current::BuildError::missing("alertId"))?;
        Ok(ResetAlert { alert_id })
    }
}

impl crate::api::current::support::CurrentRequest for ResetAlert {
    fn validate_current(&self) -> Result<(), crate::Error> {
        Ok(())
    }
}

/// Current wire model `TakeAlertSignalOwnership`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct TakeAlertSignalOwnership {
    #[serde(rename = "adminAlertSignalId")]
    admin_alert_signal_id: super::ids::AdminAlertSignalId,
}

impl TakeAlertSignalOwnership {
    /// Returns wire field `adminAlertSignalId`.
    #[must_use]
    pub fn admin_alert_signal_id(&self) -> &super::ids::AdminAlertSignalId {
        &self.admin_alert_signal_id
    }

    /// Starts a builder for [`TakeAlertSignalOwnership`].
    pub fn builder() -> TakeAlertSignalOwnershipBuilder {
        TakeAlertSignalOwnershipBuilder::default()
    }
}

/// Builder for [`TakeAlertSignalOwnership`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct TakeAlertSignalOwnershipBuilder {
    admin_alert_signal_id: Option<super::ids::AdminAlertSignalId>,
}

impl TakeAlertSignalOwnershipBuilder {
    /// Sets wire field `adminAlertSignalId`.
    pub fn admin_alert_signal_id(mut self, value: super::ids::AdminAlertSignalId) -> Self {
        self.admin_alert_signal_id = Some(value);
        self
    }

    /// Validates required fields and builds [`TakeAlertSignalOwnership`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<TakeAlertSignalOwnership, crate::api::current::BuildError> {
        let admin_alert_signal_id =
            self.admin_alert_signal_id
                .ok_or(crate::api::current::BuildError::missing(
                    "adminAlertSignalId",
                ))?;
        Ok(TakeAlertSignalOwnership {
            admin_alert_signal_id,
        })
    }
}

impl crate::api::current::support::CurrentRequest for TakeAlertSignalOwnership {
    fn validate_current(&self) -> Result<(), crate::Error> {
        Ok(())
    }
}

/// Typed query parameters for `/adminAlertSignal/deps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct AdminAlertSignalDependentsQuery {
    #[serde(rename = "masterid")]
    masterid: super::ids::ProviderEntityId,
}

impl AdminAlertSignalDependentsQuery {
    /// Returns wire field `masterid`.
    #[must_use]
    pub fn masterid(&self) -> &super::ids::ProviderEntityId {
        &self.masterid
    }

    /// Starts a builder for [`AdminAlertSignalDependentsQuery`].
    pub fn builder() -> AdminAlertSignalDependentsQueryBuilder {
        AdminAlertSignalDependentsQueryBuilder::default()
    }
}

/// Builder for [`AdminAlertSignalDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct AdminAlertSignalDependentsQueryBuilder {
    masterid: Option<super::ids::ProviderEntityId>,
}

impl AdminAlertSignalDependentsQueryBuilder {
    /// Sets wire field `masterid`.
    pub fn masterid(mut self, value: super::ids::ProviderEntityId) -> Self {
        self.masterid = Some(value);
        self
    }

    /// Validates required fields and builds [`AdminAlertSignalDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<AdminAlertSignalDependentsQuery, crate::api::current::BuildError> {
        let masterid = self
            .masterid
            .ok_or(crate::api::current::BuildError::missing("masterid"))?;
        Ok(AdminAlertSignalDependentsQuery { masterid })
    }
}

impl crate::api::current::support::CurrentQuery for AdminAlertSignalDependentsQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "masterid", &self.masterid)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /adminAlertSignal/deps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn admin_alert_signal_dependents(
        &self,
        query: &AdminAlertSignalDependentsQuery,
    ) -> Result<Vec<AdminAlertSignal>, crate::Error> {
        self.get_current("/adminAlertSignal/deps", query).await
    }
}

/// Typed query parameters for `/adminAlertSignal/item`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct AdminAlertSignalItemQuery {
    #[serde(rename = "id")]
    id: super::ids::AdminAlertSignalId,
}

impl AdminAlertSignalItemQuery {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> &super::ids::AdminAlertSignalId {
        &self.id
    }

    /// Starts a builder for [`AdminAlertSignalItemQuery`].
    pub fn builder() -> AdminAlertSignalItemQueryBuilder {
        AdminAlertSignalItemQueryBuilder::default()
    }
}

/// Builder for [`AdminAlertSignalItemQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct AdminAlertSignalItemQueryBuilder {
    id: Option<super::ids::AdminAlertSignalId>,
}

impl AdminAlertSignalItemQueryBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::AdminAlertSignalId) -> Self {
        self.id = Some(value);
        self
    }

    /// Validates required fields and builds [`AdminAlertSignalItemQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<AdminAlertSignalItemQuery, crate::api::current::BuildError> {
        let id = self
            .id
            .ok_or(crate::api::current::BuildError::missing("id"))?;
        Ok(AdminAlertSignalItemQuery { id })
    }
}

impl crate::api::current::support::CurrentQuery for AdminAlertSignalItemQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "id", &self.id)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /adminAlertSignal/item` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn admin_alert_signal_item(
        &self,
        query: &AdminAlertSignalItemQuery,
    ) -> Result<AdminAlertSignal, crate::Error> {
        self.get_current("/adminAlertSignal/item", query).await
    }
}

/// Typed query parameters for `/adminAlertSignal/items`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct AdminAlertSignalItemsQuery {
    #[serde(rename = "ids")]
    ids: Vec<super::ids::AdminAlertSignalId>,
}

impl AdminAlertSignalItemsQuery {
    /// Returns wire field `ids`.
    #[must_use]
    pub fn ids(&self) -> &[super::ids::AdminAlertSignalId] {
        &self.ids
    }

    /// Starts a builder for [`AdminAlertSignalItemsQuery`].
    pub fn builder() -> AdminAlertSignalItemsQueryBuilder {
        AdminAlertSignalItemsQueryBuilder::default()
    }
}

/// Builder for [`AdminAlertSignalItemsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct AdminAlertSignalItemsQueryBuilder {
    ids: Option<Vec<super::ids::AdminAlertSignalId>>,
}

impl AdminAlertSignalItemsQueryBuilder {
    /// Sets wire field `ids`.
    pub fn ids(mut self, value: Vec<super::ids::AdminAlertSignalId>) -> Self {
        self.ids = Some(value);
        self
    }

    /// Validates required fields and builds [`AdminAlertSignalItemsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<AdminAlertSignalItemsQuery, crate::api::current::BuildError> {
        let ids = self
            .ids
            .ok_or(crate::api::current::BuildError::missing("ids"))?;
        if ids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "ids",
                "must not be empty",
            ));
        }
        Ok(AdminAlertSignalItemsQuery { ids })
    }
}

impl crate::api::current::support::CurrentQuery for AdminAlertSignalItemsQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        if self.ids.is_empty() {
            return Err(crate::Error::InvalidRequest {
                field: "ids",
                reason: "must not be empty",
            });
        }
        let mut pairs = Vec::new();
        for value in &self.ids {
            crate::api::current::support::push_query_value(&mut pairs, "ids", value)?;
        }
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /adminAlertSignal/items` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn admin_alert_signal_items(
        &self,
        query: &AdminAlertSignalItemsQuery,
    ) -> Result<Vec<AdminAlertSignal>, crate::Error> {
        self.get_current("/adminAlertSignal/items", query).await
    }
}

/// Typed query parameters for `/adminAlertSignal/ldeps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct AdminAlertSignalLDependentsQuery {
    #[serde(rename = "masterids")]
    masterids: Vec<super::ids::ProviderEntityId>,
}

impl AdminAlertSignalLDependentsQuery {
    /// Returns wire field `masterids`.
    #[must_use]
    pub fn masterids(&self) -> &[super::ids::ProviderEntityId] {
        &self.masterids
    }

    /// Starts a builder for [`AdminAlertSignalLDependentsQuery`].
    pub fn builder() -> AdminAlertSignalLDependentsQueryBuilder {
        AdminAlertSignalLDependentsQueryBuilder::default()
    }
}

/// Builder for [`AdminAlertSignalLDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct AdminAlertSignalLDependentsQueryBuilder {
    masterids: Option<Vec<super::ids::ProviderEntityId>>,
}

impl AdminAlertSignalLDependentsQueryBuilder {
    /// Sets wire field `masterids`.
    pub fn masterids(mut self, value: Vec<super::ids::ProviderEntityId>) -> Self {
        self.masterids = Some(value);
        self
    }

    /// Validates required fields and builds [`AdminAlertSignalLDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(
        self,
    ) -> Result<AdminAlertSignalLDependentsQuery, crate::api::current::BuildError> {
        let masterids = self
            .masterids
            .ok_or(crate::api::current::BuildError::missing("masterids"))?;
        if masterids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "masterids",
                "must not be empty",
            ));
        }
        Ok(AdminAlertSignalLDependentsQuery { masterids })
    }
}

impl crate::api::current::support::CurrentQuery for AdminAlertSignalLDependentsQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        if self.masterids.is_empty() {
            return Err(crate::Error::InvalidRequest {
                field: "masterids",
                reason: "must not be empty",
            });
        }
        let mut pairs = Vec::new();
        for value in &self.masterids {
            crate::api::current::support::push_query_value(&mut pairs, "masterids", value)?;
        }
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /adminAlertSignal/ldeps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn admin_alert_signal_l_dependents(
        &self,
        query: &AdminAlertSignalLDependentsQuery,
    ) -> Result<Vec<AdminAlertSignal>, crate::Error> {
        self.get_current("/adminAlertSignal/ldeps", query).await
    }
}

impl crate::Client {
    /// Calls the current `GET /adminAlertSignal/list` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn admin_alert_signal_list(&self) -> Result<Vec<AdminAlertSignal>, crate::Error> {
        self.get_without_query("/adminAlertSignal/list").await
    }
}

/// Typed query parameters for `/alert/deps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct AlertDependentsQuery {
    #[serde(rename = "masterid")]
    masterid: super::ids::ProviderEntityId,
}

impl AlertDependentsQuery {
    /// Returns wire field `masterid`.
    #[must_use]
    pub fn masterid(&self) -> &super::ids::ProviderEntityId {
        &self.masterid
    }

    /// Starts a builder for [`AlertDependentsQuery`].
    pub fn builder() -> AlertDependentsQueryBuilder {
        AlertDependentsQueryBuilder::default()
    }
}

/// Builder for [`AlertDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct AlertDependentsQueryBuilder {
    masterid: Option<super::ids::ProviderEntityId>,
}

impl AlertDependentsQueryBuilder {
    /// Sets wire field `masterid`.
    pub fn masterid(mut self, value: super::ids::ProviderEntityId) -> Self {
        self.masterid = Some(value);
        self
    }

    /// Validates required fields and builds [`AlertDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<AlertDependentsQuery, crate::api::current::BuildError> {
        let masterid = self
            .masterid
            .ok_or(crate::api::current::BuildError::missing("masterid"))?;
        Ok(AlertDependentsQuery { masterid })
    }
}

impl crate::api::current::support::CurrentQuery for AlertDependentsQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "masterid", &self.masterid)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /alert/deps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn alert_dependents(
        &self,
        query: &AlertDependentsQuery,
    ) -> Result<Vec<Alert>, crate::Error> {
        self.get_current("/alert/deps", query).await
    }
}

/// Typed query parameters for `/alert/item`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct AlertItemQuery {
    #[serde(rename = "id")]
    id: super::ids::AlertId,
}

impl AlertItemQuery {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> &super::ids::AlertId {
        &self.id
    }

    /// Starts a builder for [`AlertItemQuery`].
    pub fn builder() -> AlertItemQueryBuilder {
        AlertItemQueryBuilder::default()
    }
}

/// Builder for [`AlertItemQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct AlertItemQueryBuilder {
    id: Option<super::ids::AlertId>,
}

impl AlertItemQueryBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::AlertId) -> Self {
        self.id = Some(value);
        self
    }

    /// Validates required fields and builds [`AlertItemQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<AlertItemQuery, crate::api::current::BuildError> {
        let id = self
            .id
            .ok_or(crate::api::current::BuildError::missing("id"))?;
        Ok(AlertItemQuery { id })
    }
}

impl crate::api::current::support::CurrentQuery for AlertItemQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "id", &self.id)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /alert/item` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn alert_item(&self, query: &AlertItemQuery) -> Result<Alert, crate::Error> {
        self.get_current("/alert/item", query).await
    }
}

/// Typed query parameters for `/alert/items`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct AlertItemsQuery {
    #[serde(rename = "ids")]
    ids: Vec<super::ids::AlertId>,
}

impl AlertItemsQuery {
    /// Returns wire field `ids`.
    #[must_use]
    pub fn ids(&self) -> &[super::ids::AlertId] {
        &self.ids
    }

    /// Starts a builder for [`AlertItemsQuery`].
    pub fn builder() -> AlertItemsQueryBuilder {
        AlertItemsQueryBuilder::default()
    }
}

/// Builder for [`AlertItemsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct AlertItemsQueryBuilder {
    ids: Option<Vec<super::ids::AlertId>>,
}

impl AlertItemsQueryBuilder {
    /// Sets wire field `ids`.
    pub fn ids(mut self, value: Vec<super::ids::AlertId>) -> Self {
        self.ids = Some(value);
        self
    }

    /// Validates required fields and builds [`AlertItemsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<AlertItemsQuery, crate::api::current::BuildError> {
        let ids = self
            .ids
            .ok_or(crate::api::current::BuildError::missing("ids"))?;
        if ids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "ids",
                "must not be empty",
            ));
        }
        Ok(AlertItemsQuery { ids })
    }
}

impl crate::api::current::support::CurrentQuery for AlertItemsQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        if self.ids.is_empty() {
            return Err(crate::Error::InvalidRequest {
                field: "ids",
                reason: "must not be empty",
            });
        }
        let mut pairs = Vec::new();
        for value in &self.ids {
            crate::api::current::support::push_query_value(&mut pairs, "ids", value)?;
        }
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /alert/items` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn alert_items(&self, query: &AlertItemsQuery) -> Result<Vec<Alert>, crate::Error> {
        self.get_current("/alert/items", query).await
    }
}

/// Typed query parameters for `/alert/ldeps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct AlertLDependentsQuery {
    #[serde(rename = "masterids")]
    masterids: Vec<super::ids::ProviderEntityId>,
}

impl AlertLDependentsQuery {
    /// Returns wire field `masterids`.
    #[must_use]
    pub fn masterids(&self) -> &[super::ids::ProviderEntityId] {
        &self.masterids
    }

    /// Starts a builder for [`AlertLDependentsQuery`].
    pub fn builder() -> AlertLDependentsQueryBuilder {
        AlertLDependentsQueryBuilder::default()
    }
}

/// Builder for [`AlertLDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct AlertLDependentsQueryBuilder {
    masterids: Option<Vec<super::ids::ProviderEntityId>>,
}

impl AlertLDependentsQueryBuilder {
    /// Sets wire field `masterids`.
    pub fn masterids(mut self, value: Vec<super::ids::ProviderEntityId>) -> Self {
        self.masterids = Some(value);
        self
    }

    /// Validates required fields and builds [`AlertLDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<AlertLDependentsQuery, crate::api::current::BuildError> {
        let masterids = self
            .masterids
            .ok_or(crate::api::current::BuildError::missing("masterids"))?;
        if masterids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "masterids",
                "must not be empty",
            ));
        }
        Ok(AlertLDependentsQuery { masterids })
    }
}

impl crate::api::current::support::CurrentQuery for AlertLDependentsQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        if self.masterids.is_empty() {
            return Err(crate::Error::InvalidRequest {
                field: "masterids",
                reason: "must not be empty",
            });
        }
        let mut pairs = Vec::new();
        for value in &self.masterids {
            crate::api::current::support::push_query_value(&mut pairs, "masterids", value)?;
        }
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /alert/ldeps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn alert_l_dependents(
        &self,
        query: &AlertLDependentsQuery,
    ) -> Result<Vec<Alert>, crate::Error> {
        self.get_current("/alert/ldeps", query).await
    }
}

/// Typed query parameters for `/alertSignal/deps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct AlertSignalDependentsQuery {
    #[serde(rename = "masterid")]
    masterid: super::ids::ProviderEntityId,
}

impl AlertSignalDependentsQuery {
    /// Returns wire field `masterid`.
    #[must_use]
    pub fn masterid(&self) -> &super::ids::ProviderEntityId {
        &self.masterid
    }

    /// Starts a builder for [`AlertSignalDependentsQuery`].
    pub fn builder() -> AlertSignalDependentsQueryBuilder {
        AlertSignalDependentsQueryBuilder::default()
    }
}

/// Builder for [`AlertSignalDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct AlertSignalDependentsQueryBuilder {
    masterid: Option<super::ids::ProviderEntityId>,
}

impl AlertSignalDependentsQueryBuilder {
    /// Sets wire field `masterid`.
    pub fn masterid(mut self, value: super::ids::ProviderEntityId) -> Self {
        self.masterid = Some(value);
        self
    }

    /// Validates required fields and builds [`AlertSignalDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<AlertSignalDependentsQuery, crate::api::current::BuildError> {
        let masterid = self
            .masterid
            .ok_or(crate::api::current::BuildError::missing("masterid"))?;
        Ok(AlertSignalDependentsQuery { masterid })
    }
}

impl crate::api::current::support::CurrentQuery for AlertSignalDependentsQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "masterid", &self.masterid)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /alertSignal/deps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn alert_signal_dependents(
        &self,
        query: &AlertSignalDependentsQuery,
    ) -> Result<Vec<AlertSignal>, crate::Error> {
        self.get_current("/alertSignal/deps", query).await
    }
}

/// Typed query parameters for `/alertSignal/item`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct AlertSignalItemQuery {
    #[serde(rename = "id")]
    id: super::ids::AlertSignalId,
}

impl AlertSignalItemQuery {
    /// Returns wire field `id`.
    #[must_use]
    pub fn id(&self) -> &super::ids::AlertSignalId {
        &self.id
    }

    /// Starts a builder for [`AlertSignalItemQuery`].
    pub fn builder() -> AlertSignalItemQueryBuilder {
        AlertSignalItemQueryBuilder::default()
    }
}

/// Builder for [`AlertSignalItemQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct AlertSignalItemQueryBuilder {
    id: Option<super::ids::AlertSignalId>,
}

impl AlertSignalItemQueryBuilder {
    /// Sets wire field `id`.
    pub fn id(mut self, value: super::ids::AlertSignalId) -> Self {
        self.id = Some(value);
        self
    }

    /// Validates required fields and builds [`AlertSignalItemQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<AlertSignalItemQuery, crate::api::current::BuildError> {
        let id = self
            .id
            .ok_or(crate::api::current::BuildError::missing("id"))?;
        Ok(AlertSignalItemQuery { id })
    }
}

impl crate::api::current::support::CurrentQuery for AlertSignalItemQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        let mut pairs = Vec::new();
        crate::api::current::support::push_query_value(&mut pairs, "id", &self.id)?;
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /alertSignal/item` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn alert_signal_item(
        &self,
        query: &AlertSignalItemQuery,
    ) -> Result<AlertSignal, crate::Error> {
        self.get_current("/alertSignal/item", query).await
    }
}

/// Typed query parameters for `/alertSignal/items`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct AlertSignalItemsQuery {
    #[serde(rename = "ids")]
    ids: Vec<super::ids::AlertSignalId>,
}

impl AlertSignalItemsQuery {
    /// Returns wire field `ids`.
    #[must_use]
    pub fn ids(&self) -> &[super::ids::AlertSignalId] {
        &self.ids
    }

    /// Starts a builder for [`AlertSignalItemsQuery`].
    pub fn builder() -> AlertSignalItemsQueryBuilder {
        AlertSignalItemsQueryBuilder::default()
    }
}

/// Builder for [`AlertSignalItemsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct AlertSignalItemsQueryBuilder {
    ids: Option<Vec<super::ids::AlertSignalId>>,
}

impl AlertSignalItemsQueryBuilder {
    /// Sets wire field `ids`.
    pub fn ids(mut self, value: Vec<super::ids::AlertSignalId>) -> Self {
        self.ids = Some(value);
        self
    }

    /// Validates required fields and builds [`AlertSignalItemsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<AlertSignalItemsQuery, crate::api::current::BuildError> {
        let ids = self
            .ids
            .ok_or(crate::api::current::BuildError::missing("ids"))?;
        if ids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "ids",
                "must not be empty",
            ));
        }
        Ok(AlertSignalItemsQuery { ids })
    }
}

impl crate::api::current::support::CurrentQuery for AlertSignalItemsQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        if self.ids.is_empty() {
            return Err(crate::Error::InvalidRequest {
                field: "ids",
                reason: "must not be empty",
            });
        }
        let mut pairs = Vec::new();
        for value in &self.ids {
            crate::api::current::support::push_query_value(&mut pairs, "ids", value)?;
        }
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /alertSignal/items` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn alert_signal_items(
        &self,
        query: &AlertSignalItemsQuery,
    ) -> Result<Vec<AlertSignal>, crate::Error> {
        self.get_current("/alertSignal/items", query).await
    }
}

/// Typed query parameters for `/alertSignal/ldeps`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct AlertSignalLDependentsQuery {
    #[serde(rename = "masterids")]
    masterids: Vec<super::ids::ProviderEntityId>,
}

impl AlertSignalLDependentsQuery {
    /// Returns wire field `masterids`.
    #[must_use]
    pub fn masterids(&self) -> &[super::ids::ProviderEntityId] {
        &self.masterids
    }

    /// Starts a builder for [`AlertSignalLDependentsQuery`].
    pub fn builder() -> AlertSignalLDependentsQueryBuilder {
        AlertSignalLDependentsQueryBuilder::default()
    }
}

/// Builder for [`AlertSignalLDependentsQuery`].
#[must_use = "a wire-model builder does nothing until build is called"]
#[derive(Clone, Debug, Default)]
pub struct AlertSignalLDependentsQueryBuilder {
    masterids: Option<Vec<super::ids::ProviderEntityId>>,
}

impl AlertSignalLDependentsQueryBuilder {
    /// Sets wire field `masterids`.
    pub fn masterids(mut self, value: Vec<super::ids::ProviderEntityId>) -> Self {
        self.masterids = Some(value);
        self
    }

    /// Validates required fields and builds [`AlertSignalLDependentsQuery`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::api::current::BuildError`] when a required field is absent or invalid.
    pub fn build(self) -> Result<AlertSignalLDependentsQuery, crate::api::current::BuildError> {
        let masterids = self
            .masterids
            .ok_or(crate::api::current::BuildError::missing("masterids"))?;
        if masterids.is_empty() {
            return Err(crate::api::current::BuildError::invalid(
                "masterids",
                "must not be empty",
            ));
        }
        Ok(AlertSignalLDependentsQuery { masterids })
    }
}

impl crate::api::current::support::CurrentQuery for AlertSignalLDependentsQuery {
    fn encode_pairs(&self) -> Result<Vec<(&'static str, String)>, crate::Error> {
        if self.masterids.is_empty() {
            return Err(crate::Error::InvalidRequest {
                field: "masterids",
                reason: "must not be empty",
            });
        }
        let mut pairs = Vec::new();
        for value in &self.masterids {
            crate::api::current::support::push_query_value(&mut pairs, "masterids", value)?;
        }
        Ok(pairs)
    }
}

impl crate::Client {
    /// Calls the current `GET /alertSignal/ldeps` operation.
    ///
    /// # Errors
    ///
    /// Returns a typed local validation, authentication, rate, transport,
    /// provider-control, response-bound, or decoding failure. Mutations may
    /// additionally return an ambiguous outcome requiring reconciliation.
    pub async fn alert_signal_l_dependents(
        &self,
        query: &AlertSignalLDependentsQuery,
    ) -> Result<Vec<AlertSignal>, crate::Error> {
        self.get_current("/alertSignal/ldeps", query).await
    }
}

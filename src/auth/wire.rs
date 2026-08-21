// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Private authentication wire models.

use jiff::Timestamp;
use serde::{Serialize, Serializer};

use super::{ApiClientId, Credentials};
use crate::UserId;
use crate::client::ControlWireResponse;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AccessTokenRequest<'a> {
    name: &'a str,
    password: &'a str,
    app_id: &'a str,
    app_version: &'a str,
    #[serde(serialize_with = "serialize_client_id")]
    cid: &'a ApiClientId,
    sec: &'a str,
    device_id: &'a str,
    hibp_check: bool,
    #[serde(rename = "p-ticket", skip_serializing_if = "Option::is_none")]
    penalty_ticket: Option<&'a str>,
}

impl<'a> AccessTokenRequest<'a> {
    pub(crate) fn new(credentials: &'a Credentials, penalty_ticket: Option<&'a str>) -> Self {
        Self {
            name: credentials.name(),
            password: credentials.password(),
            app_id: credentials.app_id(),
            app_version: credentials.app_version(),
            cid: credentials.client_id(),
            sec: credentials.secret(),
            device_id: credentials.device_id().as_str(),
            hibp_check: credentials.hibp_check(),
            penalty_ticket,
        }
    }
}

fn serialize_client_id<S>(value: &ApiClientId, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        ApiClientId::Numeric(value) => serializer.serialize_u64(*value),
        ApiClientId::Text(value) => serializer.serialize_str(value),
    }
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AccessTokenResponse {
    pub(crate) access_token: String,
    #[serde(default)]
    pub(crate) md_access_token: Option<String>,
    pub(crate) expiration_time: Timestamp,
    pub(crate) user_id: UserId,
    #[serde(default)]
    pub(crate) has_market_data: bool,
}

impl ControlWireResponse for AccessTokenResponse {
    fn has_success_evidence(body: &[u8]) -> bool {
        serde_json::from_slice::<AccessTokenResponse>(body).is_ok()
    }
}

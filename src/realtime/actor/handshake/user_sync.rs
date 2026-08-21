// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Fixed initial user-synchronization profile and bootstrap validation.
//!
//! Contract reviewed 2026-08-21 against Tradovate's official User/Syncrequest
//! documentation: <https://partner.tradovate.com/overview/core-concepts/web-sockets/user-syncrequest>.

use std::collections::BTreeMap;

use serde::{Deserialize, Deserializer, Serialize, de::IgnoredAny};
use serde_json::value::RawValue;
use tokio::time;
use tokio_tungstenite::tungstenite::Message;

use super::{ResponseWait, Socket, send_heartbeat_aware, wait_for_response};
use crate::{
    provider_control::{self, PenaltyControl, ResponseControl},
    rate_limit::RateGovernor,
    realtime::{RealtimeError, Response, ServerMessage},
};

const RATE_ENDPOINT: &str = "/user/syncrequest";

const SNAPSHOT_FIELDS: &[&str] = &[
    "accounts",
    "accountRiskStatuses",
    "cashBalances",
    "commands",
    "commandReports",
    "executionReports",
    "fills",
    "fillPairs",
    "orders",
    "orderStrategies",
    "positions",
    "products",
    "users",
];

const ENTITY_TYPES: &[&str] = &[
    "account",
    "accountRiskStatus",
    "cashBalance",
    "command",
    "commandReport",
    "executionReport",
    "fill",
    "fillPair",
    "order",
    "orderStrategy",
    "position",
    "product",
    "user",
];

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UserSyncBody {
    split_responses: bool,
    entity_types: &'static [&'static str],
}

pub(super) async fn perform(
    socket: &mut Socket,
    heartbeat: &mut time::Interval,
    heartbeat_deadline: &mut time::Instant,
    rate_limits: &RateGovernor,
    staged: &mut Vec<ServerMessage>,
    wait: ResponseWait<'_>,
) -> Result<(Response, u64), RealtimeError> {
    let body = serde_json::to_string(&UserSyncBody {
        split_responses: false,
        entity_types: ENTITY_TYPES,
    })
    .map_err(|_| RealtimeError::Protocol)?;
    let frame = wait
        .codec
        .encode_request("user/syncrequest", wait.request_id, "", &body)?;
    let retry_after = rate_limits.try_admit_authenticated(RATE_ENDPOINT);
    if !retry_after.is_zero() {
        return Err(RealtimeError::LocalRateLimit {
            endpoint: RATE_ENDPOINT,
            retry_after,
        });
    }
    send_heartbeat_aware(
        wait.connection_id,
        socket,
        Message::text(frame),
        *heartbeat_deadline,
        wait.deadline,
        wait.timeout_error,
        wait.cancellation,
    )
    .await?;
    let response = wait_for_response(socket, heartbeat, heartbeat_deadline, wait, staged).await?;
    match validate(response, rate_limits)? {
        SyncResponse::Bootstrap(response) => Ok((response, next_request_id(wait.request_id)?)),
        SyncResponse::Penalty(penalty) => {
            let (ticket, retry_after, captcha_required) = penalty.into_parts();
            drop(ticket);
            if captcha_required {
                rate_limits.apply_captcha_lockout(RATE_ENDPOINT, retry_after);
            } else {
                rate_limits.apply_endpoint_cooldown(RATE_ENDPOINT, retry_after);
            }
            Err(RealtimeError::UserSyncPenalty {
                retry_after,
                captcha_required,
            })
        }
    }
}

fn next_request_id(request_id: crate::realtime::RequestId) -> Result<u64, RealtimeError> {
    request_id
        .value()
        .checked_add(1)
        .ok_or(RealtimeError::RequestIdExhausted)
}

enum SyncResponse {
    Bootstrap(Response),
    Penalty(PenaltyControl),
}

fn validate(response: Response, rate_limits: &RateGovernor) -> Result<SyncResponse, RealtimeError> {
    if response.status() == 429 {
        let retry_after = std::time::Duration::from_hours(1);
        rate_limits.apply_global_cooldown(retry_after);
        return Err(RealtimeError::ProviderRateLimit {
            request_id: response.request_id(),
            retry_after,
        });
    }
    if response.status() != 200 {
        return Err(RealtimeError::UserSyncRejected {
            status: response.status(),
        });
    }
    let data = response
        .data()
        .ok_or(RealtimeError::UserSyncInvalidBootstrap)?;
    let snapshot_present = inspect_snapshot(data)?;
    match provider_control::inspect(response.data()) {
        Ok(ResponseControl::Payload) if snapshot_present => {}
        Ok(ResponseControl::Penalty(penalty)) if !snapshot_present => {
            return Ok(SyncResponse::Penalty(penalty));
        }
        Ok(
            ResponseControl::Payload
            | ResponseControl::Penalty(_)
            | ResponseControl::BusinessFailure { .. },
        )
        | Err(_) => return Err(RealtimeError::UserSyncInvalidBootstrap),
    }
    Ok(SyncResponse::Bootstrap(response))
}

fn inspect_snapshot(data: &RawValue) -> Result<bool, RealtimeError> {
    let fields = serde_json::from_str::<BTreeMap<String, Box<RawValue>>>(data.get())
        .map_err(|_| RealtimeError::UserSyncInvalidBootstrap)?;
    let mut snapshot_field_seen = false;
    for field in SNAPSHOT_FIELDS {
        if let Some(value) = fields.get(*field) {
            if serde_json::from_str::<Vec<EntityObject>>(value.get()).is_err() {
                return Err(RealtimeError::UserSyncInvalidBootstrap);
            }
            snapshot_field_seen = true;
        }
    }
    Ok(snapshot_field_seen)
}

struct EntityObject;

impl<'de> Deserialize<'de> for EntityObject {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EntityObjectVisitor;

        impl<'de> serde::de::Visitor<'de> for EntityObjectVisitor {
            type Value = EntityObject;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("a user-sync entity object")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: serde::de::MapAccess<'de>,
            {
                while map.next_entry::<IgnoredAny, IgnoredAny>()?.is_some() {}
                Ok(EntityObject)
            }
        }

        deserializer.deserialize_map(EntityObjectVisitor)
    }
}

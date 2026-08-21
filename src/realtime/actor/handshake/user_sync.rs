// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Validated initial user-synchronization request and bootstrap readiness.
//!
//! Contract reviewed 2026-08-22 against Tradovate's official User/Syncrequest
//! documentation: <https://partner.tradovate.com/overview/core-concepts/web-sockets/user-syncrequest>.

use std::collections::BTreeMap;

use serde::{Deserialize, Deserializer, de::IgnoredAny};
use serde_json::value::RawValue;
use tokio::time;
use tokio_tungstenite::tungstenite::Message;

use super::{ResponseWait, Socket, send_heartbeat_aware, wait_for_response};
use crate::{
    provider_control::{self, PenaltyControl, ResponseControl},
    rate_limit::RateGovernor,
    realtime::{RealtimeError, Response, ServerMessage, UserSyncConfig, user_stream::decode},
};

const RATE_ENDPOINT: &str = "/user/syncrequest";

pub(super) async fn perform(
    socket: &mut Socket,
    heartbeat: &mut time::Interval,
    heartbeat_deadline: &mut time::Instant,
    rate_limits: &RateGovernor,
    sync_config: &UserSyncConfig,
    staged: &mut Vec<ServerMessage>,
    wait: ResponseWait<'_>,
) -> Result<(Response, u64), RealtimeError> {
    let body = sync_config.encode()?;
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
    let snapshot = inspect_snapshot(data)?;
    match provider_control::inspect(response.data()) {
        Ok(ResponseControl::Payload) if snapshot.complete => {}
        Ok(ResponseControl::Penalty(penalty)) if !snapshot.evidence => {
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

fn inspect_snapshot(data: &RawValue) -> Result<SnapshotInspection, RealtimeError> {
    let fields = serde_json::from_str::<BTreeMap<String, Box<RawValue>>>(data.get())
        .map_err(|_| RealtimeError::UserSyncInvalidBootstrap)?;
    let mut evidence = false;
    for field in decode::BOOTSTRAP_COLLECTIONS {
        if let Some(value) = fields.get(*field) {
            if serde_json::from_str::<Vec<EntityObject>>(value.get()).is_err() {
                return Err(RealtimeError::UserSyncInvalidBootstrap);
            }
            evidence = true;
        }
    }
    Ok(SnapshotInspection {
        evidence,
        complete: fields.contains_key("users") && fields.contains_key("contractGroups"),
    })
}

#[derive(Clone, Copy)]
struct SnapshotInspection {
    evidence: bool,
    complete: bool,
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

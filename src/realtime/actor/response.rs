// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Provider-control classification for active correlated responses.

use std::time::Duration;

use crate::{
    provider_control::{self, ResponseControl},
    rate_limit::RateGovernor,
    realtime::{RealtimeError, RequestId, Response},
};

pub(super) fn classify(
    response: Response,
    request_id: RequestId,
    endpoint: &'static str,
    rate_limits: &RateGovernor,
) -> Result<Response, RealtimeError> {
    if response.status() == 429 {
        let retry_after = Duration::from_hours(1);
        rate_limits.apply_global_cooldown(retry_after);
        return Err(RealtimeError::ProviderRateLimit {
            request_id,
            retry_after,
        });
    }
    if !(200..300).contains(&response.status()) {
        return Err(RealtimeError::ProviderRejected {
            request_id,
            status: response.status(),
        });
    }
    match provider_control::inspect(response.data()) {
        Ok(ResponseControl::Payload) => Ok(response),
        Ok(ResponseControl::BusinessFailure { violation_count }) => {
            Err(RealtimeError::ProviderBusinessFailure {
                request_id,
                violation_count,
            })
        }
        Ok(ResponseControl::Penalty(penalty)) => {
            let (ticket, retry_after, captcha_required) = penalty.into_parts();
            drop(ticket);
            if captcha_required {
                rate_limits.apply_captcha_lockout(endpoint, retry_after);
            } else {
                rate_limits.apply_endpoint_cooldown(endpoint, retry_after);
            }
            Err(RealtimeError::ProviderPenalty {
                request_id,
                retry_after,
                captcha_required,
            })
        }
        Err(_) => Err(RealtimeError::Protocol),
    }
}

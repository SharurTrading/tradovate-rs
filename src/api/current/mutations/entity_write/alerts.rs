// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Request-aware alert creation, modification, and signal read marking.

use crate::{
    Client, Error,
    api::current::{
        alerts::{AlertResponse, CreateAlert, MarkReadAlertSignal, ModifyAlert},
        support::CurrentRequest,
    },
    client::MutationAssessment,
};

impl Client {
    /// Creates one alert for the authenticated user.
    ///
    /// # Errors
    ///
    /// Returns validation, authentication, rate, transport, provider-control,
    /// decoding, or ambiguity errors. Success requires a newly assigned alert
    /// ID and an exact echo of every request field.
    pub async fn alert_create_alert(&self, request: &CreateAlert) -> Result<AlertResponse, Error> {
        request.validate_current()?;
        self.post_reviewed_mutation("/alert/createalert", request, assess_create)
            .await
    }

    /// Modifies one existing alert.
    ///
    /// # Errors
    ///
    /// Returns validation, authentication, rate, transport, provider-control,
    /// decoding, or ambiguity errors. Success requires the exact alert ID and
    /// an exact echo of every requested alert field.
    pub async fn alert_modify_alert(&self, request: &ModifyAlert) -> Result<AlertResponse, Error> {
        request.validate_current()?;
        self.post_reviewed_mutation("/alert/modifyalert", request, assess_modify)
            .await
    }

    /// Marks one alert signal as read.
    ///
    /// The current response can echo the parent alert but cannot identify the
    /// requested alert signal. A successful 2xx response therefore remains
    /// ambiguous and requires reconciliation through alert-signal queries.
    ///
    /// # Errors
    ///
    /// Returns validation, authentication, rate, transport, provider-control,
    /// decoding, business-rejection, or ambiguous-mutation errors. Under the
    /// pinned response schema this method cannot return definitive success.
    pub async fn alert_mark_read_alert_signal(
        &self,
        request: &MarkReadAlertSignal,
    ) -> Result<AlertResponse, Error> {
        request.validate_current()?;
        self.post_reviewed_mutation("/alert/markreadalertsignal", request, assess_mark_read)
            .await
    }
}

fn assess_create(response: &AlertResponse, request: &CreateAlert) -> MutationAssessment {
    let evidence = response.alert().is_some();
    let exact = response.alert().is_some_and(|alert| {
        alert.id().is_some()
            && alert.expression() == request.expression()
            && alert.valid_until() == request.valid_until()
            && alert.trigger_limits() == request.trigger_limits()
            && alert.message() == request.message()
    });
    assess_alert_response(response, exact, evidence)
}

fn assess_modify(response: &AlertResponse, request: &ModifyAlert) -> MutationAssessment {
    let evidence = response.alert().is_some();
    let exact = response.alert().is_some_and(|alert| {
        alert.id() == Some(request.alert_id())
            && alert.expression() == request.expression()
            && alert.valid_until() == request.valid_until()
            && alert.trigger_limits() == request.trigger_limits()
            && alert.message() == request.message()
    });
    assess_alert_response(response, exact, evidence)
}

fn assess_mark_read(response: &AlertResponse, _: &MarkReadAlertSignal) -> MutationAssessment {
    // Even an exact parent alert cannot prove which signal was marked. Any
    // returned alert ID also makes an error envelope contradictory rather than
    // a definitive rejection, including an unexpected parent-ID mismatch.
    let evidence = response.alert().is_some();
    if has_error(response) && !evidence {
        MutationAssessment::rejected()
    } else {
        MutationAssessment::ambiguous(evidence)
    }
}

fn assess_alert_response(
    response: &AlertResponse,
    exact: bool,
    evidence: bool,
) -> MutationAssessment {
    if has_error(response) {
        return if evidence {
            MutationAssessment::ambiguous(true)
        } else {
            MutationAssessment::rejected()
        };
    }
    if exact {
        MutationAssessment::success()
    } else {
        MutationAssessment::ambiguous(evidence)
    }
}

fn has_error(response: &AlertResponse) -> bool {
    response.error_text().is_some_and(|value| !value.is_empty())
}

#[cfg(test)]
#[path = "tests/alerts.rs"]
mod tests;

// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Alert and administrative alert-signal mutations.

use crate::{
    Client, Error,
    api::current::alerts::{
        AdminAlertSignalResponse, AlertResponse, CompleteAlertSignal, DeleteAlert, DismissAlert,
        ResetAlert, TakeAlertSignalOwnership,
    },
    client::MutationAssessment,
};

use super::common::exact_entity;

impl Client {
    /// Completes one administrative alert signal.
    ///
    /// # Errors
    ///
    /// Returns local validation, authentication, rate, transport, provider,
    /// decoding, or ambiguous-mutation errors. The returned signal must echo
    /// the exact requested identity before resolution.
    pub async fn admin_alert_signal_complete_alert_signal(
        &self,
        request: &CompleteAlertSignal,
    ) -> Result<AdminAlertSignalResponse, Error> {
        self.post_reviewed_mutation(
            "/adminAlertSignal/completealertsignal",
            request,
            assess_complete_alert_signal,
        )
        .await
    }

    /// Takes ownership of one administrative alert signal.
    ///
    /// # Errors
    ///
    /// Returns local validation, authentication, rate, transport, provider,
    /// decoding, or ambiguous-mutation errors. The returned signal must echo
    /// the exact requested identity before resolution.
    pub async fn admin_alert_signal_take_alert_signal_ownership(
        &self,
        request: &TakeAlertSignalOwnership,
    ) -> Result<AdminAlertSignalResponse, Error> {
        self.post_reviewed_mutation(
            "/adminAlertSignal/takealertsignalownership",
            request,
            assess_take_alert_signal_ownership,
        )
        .await
    }

    /// Deletes one alert.
    ///
    /// # Errors
    ///
    /// Returns local validation, authentication, rate, transport, provider,
    /// decoding, or ambiguous-mutation errors. The returned alert must echo the
    /// exact requested identity before resolution.
    pub async fn alert_delete_alert(&self, request: &DeleteAlert) -> Result<AlertResponse, Error> {
        self.post_reviewed_mutation("/alert/deletealert", request, assess_delete_alert)
            .await
    }

    /// Dismisses one alert.
    ///
    /// # Errors
    ///
    /// Returns local validation, authentication, rate, transport, provider,
    /// decoding, or ambiguous-mutation errors. The returned alert must echo the
    /// exact requested identity before resolution.
    pub async fn alert_dismiss_alert(
        &self,
        request: &DismissAlert,
    ) -> Result<AlertResponse, Error> {
        self.post_reviewed_mutation("/alert/dismissalert", request, assess_dismiss_alert)
            .await
    }

    /// Resets one alert.
    ///
    /// # Errors
    ///
    /// Returns local validation, authentication, rate, transport, provider,
    /// decoding, or ambiguous-mutation errors. The returned alert must echo the
    /// exact requested identity before resolution.
    pub async fn alert_reset_alert(&self, request: &ResetAlert) -> Result<AlertResponse, Error> {
        self.post_reviewed_mutation("/alert/resetalert", request, assess_reset_alert)
            .await
    }
}

fn assess_complete_alert_signal(
    response: &AdminAlertSignalResponse,
    request: &CompleteAlertSignal,
) -> MutationAssessment {
    let exact = response
        .admin_alert_signal()
        .and_then(|signal| signal.id())
        .is_some_and(|id| id == request.admin_alert_signal_id());
    exact_entity(response.error_text(), exact)
}

fn assess_take_alert_signal_ownership(
    response: &AdminAlertSignalResponse,
    request: &TakeAlertSignalOwnership,
) -> MutationAssessment {
    let exact = response
        .admin_alert_signal()
        .and_then(|signal| signal.id())
        .is_some_and(|id| id == request.admin_alert_signal_id());
    exact_entity(response.error_text(), exact)
}

fn assess_delete_alert(response: &AlertResponse, request: &DeleteAlert) -> MutationAssessment {
    let exact = response
        .alert()
        .and_then(|alert| alert.id())
        .is_some_and(|id| id == request.alert_id());
    exact_entity(response.error_text(), exact)
}

fn assess_dismiss_alert(response: &AlertResponse, request: &DismissAlert) -> MutationAssessment {
    let exact = response
        .alert()
        .and_then(|alert| alert.id())
        .is_some_and(|id| id == request.alert_id());
    exact_entity(response.error_text(), exact)
}

fn assess_reset_alert(response: &AlertResponse, request: &ResetAlert) -> MutationAssessment {
    let exact = response
        .alert()
        .and_then(|alert| alert.id())
        .is_some_and(|id| id == request.alert_id());
    exact_entity(response.error_text(), exact)
}

#[cfg(test)]
#[path = "alerts/tests.rs"]
mod tests;

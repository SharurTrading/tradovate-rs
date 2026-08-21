// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Shared response-evidence classifiers for current mutations.

use crate::client::MutationAssessment;

use super::super::{risks::DeleteResultResponse, users::SimpleResponse};

pub(super) fn simple_ok(response: &SimpleResponse) -> MutationAssessment {
    let success = *response.ok();
    assess_boolean(response.error_text(), success)
}

pub(super) fn delete_result(response: &DeleteResultResponse) -> MutationAssessment {
    match response.success().copied() {
        Some(success) => assess_boolean(response.error_text(), success),
        None if has_error(response.error_text()) => MutationAssessment::rejected(),
        None => MutationAssessment::ambiguous(false),
    }
}

pub(super) fn exact_entity(error: Option<&str>, exact: bool) -> MutationAssessment {
    if has_error(error) {
        return if exact {
            MutationAssessment::ambiguous(true)
        } else {
            MutationAssessment::rejected()
        };
    }
    if exact {
        MutationAssessment::success()
    } else {
        MutationAssessment::ambiguous(false)
    }
}

fn assess_boolean(error: Option<&str>, success: bool) -> MutationAssessment {
    if has_error(error) {
        return if success {
            MutationAssessment::ambiguous(true)
        } else {
            MutationAssessment::rejected()
        };
    }
    if success {
        MutationAssessment::success()
    } else {
        MutationAssessment::rejected()
    }
}

fn has_error(error: Option<&str>) -> bool {
    error.is_some_and(|error| !error.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        api::current::{risks::DeleteResultResponse, users::SimpleResponse},
        client::MutationOutcome,
    };

    #[test]
    fn simple_response_requires_true_without_a_contradictory_error() {
        let accepted = simple(true, None);
        assert_eq!(simple_ok(&accepted).outcome(), MutationOutcome::Success);

        let rejected = simple(false, None);
        assert_eq!(simple_ok(&rejected).outcome(), MutationOutcome::Rejected);

        let contradiction = simple(true, Some("denied"));
        let assessment = simple_ok(&contradiction);
        assert_eq!(assessment.outcome(), MutationOutcome::Ambiguous);
        assert!(assessment.has_success_evidence());
    }

    #[test]
    fn delete_response_requires_an_explicit_boolean() {
        let accepted = delete(Some(true));
        assert_eq!(delete_result(&accepted).outcome(), MutationOutcome::Success);

        let rejected = delete(Some(false));
        assert_eq!(
            delete_result(&rejected).outcome(),
            MutationOutcome::Rejected
        );

        let incomplete = delete(None);
        assert_eq!(
            delete_result(&incomplete).outcome(),
            MutationOutcome::Ambiguous
        );
    }

    #[test]
    fn entity_identity_mismatch_and_error_contradiction_are_ambiguous() {
        assert_eq!(exact_entity(None, true).outcome(), MutationOutcome::Success);
        assert_eq!(
            exact_entity(None, false).outcome(),
            MutationOutcome::Ambiguous
        );
        let contradiction = exact_entity(Some("denied"), true);
        assert_eq!(contradiction.outcome(), MutationOutcome::Ambiguous);
        assert!(contradiction.has_success_evidence());
    }

    fn simple(ok: bool, error: Option<&str>) -> SimpleResponse {
        let builder = SimpleResponse::builder().ok(ok);
        let builder = match error {
            Some(error) => builder.error_text(error),
            None => builder,
        };
        builder
            .build()
            .unwrap_or_else(|error| panic!("simple response fixture: {error}"))
    }

    fn delete(success: Option<bool>) -> DeleteResultResponse {
        let builder = DeleteResultResponse::builder();
        let builder = match success {
            Some(success) => builder.success(success),
            None => builder,
        };
        builder
            .build()
            .unwrap_or_else(|error| panic!("delete response fixture: {error}"))
    }
}

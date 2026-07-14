// File:
//   - runtime_completion.rs
// Path:
//   - src/rmv/src/application/runtime_completion.rs
//
// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE
// Path-Rule:
//   - All paths in this header are repository-root relative.
//
// Boundary-Contract:
// - Owns:
//   - rmv use-case orchestration for application runtime completion.
// - Must-Not:
//   - Depend on driven adapters, parse local routes, or encode writer-specific
//   - syntax.
// - Allows:
//   - Use-case orchestration, planning, reporting, and calls through declared
//   - ports.
// - Split-When:
//   - Split when runtime completion contains two independently testable
//   - contracts.
// - Merge-When:
//   - Another rmv module owns the same application boundary with no distinct
//   - invariant.
// - Summary:
//   - Runtime movie completion planning.
// - Description:
//   - Defines runtime completion data and behavior for rmv application.
// - Usage:
//   - Called by entrypoints after ports and adapters are selected by the
//   - caller.
// - Defaults:
//   - No concrete adapter is selected unless the caller supplies one through a
//   - port.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Runtime movie completion planning.
//!
//! This use case intentionally stores no media bytes. It validates that an
//! incomplete movie evidence item can be completed only when the operator
//! supplies a distinct runtime candidate at execution time.

use std::path::PathBuf;

use crate::domain::{
    MovieEvidence, RuntimeCompletionDecision, RuntimeCompletionRule,
    RuntimeMovieCandidate,
};

#[derive(Debug, Clone, PartialEq, Eq)]
/// Runtimecompletionplan.
pub struct RuntimeCompletionPlan {
    /// Logical name.
    pub logical_name: String,
    /// Incomplete hash.
    pub incomplete_hash: String,
    /// Candidate path.
    pub candidate_path: PathBuf,
    /// Candidate hash.
    pub candidate_hash: String,
    /// Action.
    pub action: RuntimeCompletionAction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Runtimecompletionaction.
pub enum RuntimeCompletionAction {
    /// Item.
    UseCandidateForLocalExport,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Runtimecompletionplanner.
pub struct RuntimeCompletionPlanner {
    /// Rule.
    pub rule: RuntimeCompletionRule,
}

impl RuntimeCompletionPlanner {
    /// Plan.
    ///
    /// # Errors
    ///
    /// Returns an error when validation, parsing, or filesystem access fails.
    pub fn plan(
        &self,
        evidence: &MovieEvidence,
        candidate: Option<&RuntimeMovieCandidate>,
    ) -> Result<RuntimeCompletionPlan, RuntimeCompletionDecision> {
        match self
            .rule
            .decide(
                evidence, candidate,
            ) {
            RuntimeCompletionDecision::Ready {
                logical_name,
                candidate_path,
                candidate_sha256,
            } => Ok(
                RuntimeCompletionPlan {
                    logical_name,
                    incomplete_hash: evidence
                        .sha256
                        .hex(),
                    candidate_path,
                    candidate_hash: candidate_sha256.hex(),
                    action: RuntimeCompletionAction::UseCandidateForLocalExport,
                },
            ),
            decision @ RuntimeCompletionDecision::Incomplete {
                ..
            } => Err(decision),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::{RuntimeCompletionAction, RuntimeCompletionPlanner};
    use crate::domain::{
        MovieEvidence, MovieKind, RuntimeCompletionDecision,
        RuntimeCompletionRule, RuntimeMovieCandidate, Sha256,
    };

    fn hash(value: &[u8]) -> Sha256 {
        Sha256::digest(value)
    }

    #[test]
    fn creates_local_export_plan_when_runtime_candidate_is_valid() {
        let evidence = MovieEvidence {
            logical_name: "gracie".to_owned(),
            byte_len: 2_887,
            sha256: hash(b"small"),
            kind: MovieKind::OggNamedRmv,
        };
        let candidate = RuntimeMovieCandidate {
            path: PathBuf::from("selected/gracie.rmv"),
            byte_len: 851_028,
            sha256: hash(b"large"),
            kind: MovieKind::BinkV1,
        };
        let planner = RuntimeCompletionPlanner {
            rule: RuntimeCompletionRule {
                logical_name: "gracie".to_owned(),
                accepted_kind: MovieKind::BinkV1,
                min_byte_len: 100_000,
            },
        };

        let plan_result = planner.plan(
            &evidence,
            Some(&candidate),
        );
        assert!(
            plan_result.is_ok(),
            "runtime candidate should produce an export plan"
        );
        let Ok(plan) = plan_result else {
            return;
        };
        assert_eq!(
            plan.logical_name,
            "gracie"
        );
        assert_eq!(
            plan.candidate_path,
            PathBuf::from("selected/gracie.rmv")
        );
        assert_eq!(
            plan.action,
            RuntimeCompletionAction::UseCandidateForLocalExport
        );
    }

    #[test]
    fn fails_closed_without_candidate() {
        let evidence = MovieEvidence {
            logical_name: "gracie".to_owned(),
            byte_len: 2_887,
            sha256: hash(b"small"),
            kind: MovieKind::OggNamedRmv,
        };
        let planner = RuntimeCompletionPlanner {
            rule: RuntimeCompletionRule {
                logical_name: "gracie".to_owned(),
                accepted_kind: MovieKind::BinkV1,
                min_byte_len: 100_000,
            },
        };

        assert_eq!(
            planner.plan(
                &evidence, None
            ),
            Err(
                RuntimeCompletionDecision::Incomplete {
                    logical_name: "gracie".to_owned(),
                    reason: "missing-runtime-candidate".to_owned(),
                }
            )
        );
    }
}

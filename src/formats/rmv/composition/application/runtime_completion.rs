// Copyright:
//   - Copyright © 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Runtime completion application service.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Runtime completion application service.
// - Description:
//   - Implements the declared application service responsibility for rmv.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Runtime completion application service.

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
        match self.rule.decide(evidence, candidate) {
            RuntimeCompletionDecision::Ready {
                logical_name,
                candidate_path,
                candidate_sha256,
            } => Ok(RuntimeCompletionPlan {
                logical_name,
                incomplete_hash: evidence.sha256.hex(),
                candidate_path,
                candidate_hash: candidate_sha256.hex(),
                action: RuntimeCompletionAction::UseCandidateForLocalExport,
            }),
            decision @ RuntimeCompletionDecision::Incomplete { .. } => {
                Err(decision)
            },
        }
    }
}

#[cfg(test)]
// jig-ignore-next-line: exact test module path is indivisible
#[path = "../../../../../tests/formats/rmv/unit/application/runtime_completion/tests.rs"]
mod tests;

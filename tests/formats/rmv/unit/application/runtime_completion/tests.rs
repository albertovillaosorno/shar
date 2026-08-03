// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Tests unit tests.
// - Must-Not:
//   - Own production behavior or broaden the tested API surface.
// - Allows:
//   - Private test fixtures and assertions for the owning source module.
// - Split-When:
//   - Split when an independent fixture family gains separate ownership.
// - Merge-When:
//   - Merge when another test module owns the identical evidence.
// - Summary:
//   - Tests unit tests.
// - Description:
//   - Preserves unit-test access through a test-only path module.
// - Usage:
//   - Included only by the owning source module under cfg(test).
// - Defaults:
//   - Test setup and assertions fail explicitly.
//

//! Tests unit tests.

use std::path::PathBuf;

use super::{RuntimeCompletionAction, RuntimeCompletionPlanner};
use crate::domain::{
    MovieEvidence, MovieKind, RuntimeCompletionDecision, RuntimeCompletionRule,
    RuntimeMovieCandidate, Sha256,
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

    let plan_result = planner.plan(&evidence, Some(&candidate));
    assert!(
        plan_result.is_ok(),
        "runtime candidate should produce an export plan"
    );
    let Ok(plan) = plan_result else {
        return;
    };
    assert_eq!(plan.logical_name, "gracie");
    assert_eq!(plan.candidate_path, PathBuf::from("selected/gracie.rmv"));
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
        planner.plan(&evidence, None),
        Err(RuntimeCompletionDecision::Incomplete {
            logical_name: "gracie".to_owned(),
            reason: "missing-runtime-candidate".to_owned(),
        })
    );
}

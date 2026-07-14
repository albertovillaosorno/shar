// File:
//   - runtime_source_tests.rs
// Path:
//   - src/rmv/src/domain/runtime_source_tests.rs
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
//   - Focused regression coverage for runtime movie completion policy.
// - Must-Not:
//   - Implement runtime completion policy or perform filesystem I/O.
// - Allows:
//   - Construct domain evidence and candidates for pure policy assertions.
// - Split-When:
//   - Split when one independent runtime policy family exceeds this suite.
// - Merge-When:
//   - Merge when runtime completion policy returns to a smaller focused suite.
// - Summary:
//   - Runtime completion domain regressions.
// - Description:
//   - Verifies fail-closed rule configuration and identity checks.
//   - Verifies runtime format and size checks.
// - Usage:
//   - Compiled only for RMV domain tests.
// - Defaults:
//   - No production behavior or external resources.
//
// ADRs:
// - docs/adr/rmv/local-movie-overrides.md
//
// Large file:
//   - true
//   - Reason: this file is the cohesive runtime completion regression suite and
//   - contains no production responsibilities.
//

//! Runtime completion policy regression coverage.
//!
//! These tests keep construction-heavy evidence out of the production policy
//! module while exercising its public domain contract.

use std::path::PathBuf;

use super::{
    MovieEvidence, RuntimeCompletionDecision, RuntimeCompletionRule,
    RuntimeMovieCandidate,
};
use crate::domain::{MovieKind, Sha256};

fn hash(value: &[u8]) -> Sha256 {
    Sha256::digest(value)
}

#[test]
fn fails_closed_without_runtime_candidate() {
    let rule = RuntimeCompletionRule {
        logical_name: "gracie".to_owned(),
        accepted_kind: MovieKind::BinkV1,
        min_byte_len: 100_000,
    };
    let evidence = MovieEvidence {
        logical_name: "gracie".to_owned(),
        byte_len: 2_887,
        sha256: hash(b"small"),
        kind: MovieKind::OggNamedRmv,
    };
    assert_eq!(
        rule.decide(
            &evidence, None
        ),
        RuntimeCompletionDecision::Incomplete {
            logical_name: "gracie".to_owned(),
            reason: "missing-runtime-candidate".to_owned(),
        }
    );
}

#[test]
fn rejects_minimums_too_small_for_movie_bytes() {
    for min_byte_len in 1_u64..=4 {
        let rule = RuntimeCompletionRule {
            logical_name: "gracie".to_owned(),
            accepted_kind: MovieKind::BinkV1,
            min_byte_len,
        };
        let evidence = MovieEvidence {
            logical_name: "gracie".to_owned(),
            byte_len: 1,
            sha256: hash(b"placeholder"),
            kind: MovieKind::OggNamedRmv,
        };
        assert_eq!(
            rule.decide(
                &evidence, None
            ),
            RuntimeCompletionDecision::Incomplete {
                logical_name: "gracie".to_owned(),
                reason: "invalid-min-byte-len".to_owned(),
            }
        );
    }
}

#[test]
fn rejects_zero_minimum_completion_rules() {
    let rule = RuntimeCompletionRule {
        logical_name: "gracie".to_owned(),
        accepted_kind: MovieKind::BinkV1,
        min_byte_len: 0,
    };
    let evidence = MovieEvidence {
        logical_name: "gracie".to_owned(),
        byte_len: 1,
        sha256: hash(b"placeholder"),
        kind: MovieKind::OggNamedRmv,
    };
    let candidate = RuntimeMovieCandidate {
        path: PathBuf::from("selected/gracie.rmv"),
        byte_len: 1,
        sha256: hash(b"x"),
        kind: MovieKind::BinkV1,
    };
    assert_eq!(
        rule.decide(
            &evidence,
            Some(&candidate)
        ),
        RuntimeCompletionDecision::Incomplete {
            logical_name: "gracie".to_owned(),
            reason: "invalid-min-byte-len".to_owned(),
        }
    );
}

#[test]
fn rejects_placeholder_kinds_as_complete_candidates() {
    let rule = RuntimeCompletionRule {
        logical_name: "gracie".to_owned(),
        accepted_kind: MovieKind::OggNamedRmv,
        min_byte_len: 5,
    };
    let evidence = MovieEvidence {
        logical_name: "gracie".to_owned(),
        byte_len: 1,
        sha256: hash(b"placeholder"),
        kind: MovieKind::RadicalMovieHeader,
    };
    let candidate = RuntimeMovieCandidate {
        path: PathBuf::from("selected/gracie.rmv"),
        byte_len: 10,
        sha256: hash(b"other-placeholder"),
        kind: MovieKind::OggNamedRmv,
    };
    assert_eq!(
        rule.decide(
            &evidence,
            Some(&candidate)
        ),
        RuntimeCompletionDecision::Incomplete {
            logical_name: "gracie".to_owned(),
            reason: "invalid-accepted-kind".to_owned(),
        }
    );
}

#[test]
fn rejects_unknown_as_a_complete_candidate_kind() {
    let rule = RuntimeCompletionRule {
        logical_name: "gracie".to_owned(),
        accepted_kind: MovieKind::Unknown,
        min_byte_len: 5,
    };
    let evidence = MovieEvidence {
        logical_name: "gracie".to_owned(),
        byte_len: 1,
        sha256: hash(b"placeholder"),
        kind: MovieKind::OggNamedRmv,
    };
    let candidate = RuntimeMovieCandidate {
        path: PathBuf::from("selected/gracie.rmv"),
        byte_len: 10,
        sha256: hash(b"unidentified"),
        kind: MovieKind::Unknown,
    };
    assert_eq!(
        rule.decide(
            &evidence,
            Some(&candidate)
        ),
        RuntimeCompletionDecision::Incomplete {
            logical_name: "gracie".to_owned(),
            reason: "invalid-accepted-kind".to_owned(),
        }
    );
}

#[test]
fn rejects_signature_only_completion_candidates() {
    let rule = RuntimeCompletionRule {
        logical_name: "gracie".to_owned(),
        accepted_kind: MovieKind::BinkV1,
        min_byte_len: 5,
    };
    let evidence = MovieEvidence {
        logical_name: "gracie".to_owned(),
        byte_len: 1,
        sha256: hash(b"placeholder"),
        kind: MovieKind::OggNamedRmv,
    };
    let candidate = RuntimeMovieCandidate {
        path: PathBuf::from("selected/gracie.rmv"),
        byte_len: 4,
        sha256: hash(b"BIKi"),
        kind: MovieKind::BinkV1,
    };
    assert_eq!(
        rule.decide(
            &evidence,
            Some(&candidate)
        ),
        RuntimeCompletionDecision::Incomplete {
            logical_name: "gracie".to_owned(),
            reason: "candidate-truncated".to_owned(),
        }
    );
}

#[test]
fn rejects_candidates_shorter_than_a_container_signature() {
    let rule = RuntimeCompletionRule {
        logical_name: "gracie".to_owned(),
        accepted_kind: MovieKind::BinkV1,
        min_byte_len: 5,
    };
    let evidence = MovieEvidence {
        logical_name: "gracie".to_owned(),
        byte_len: 1,
        sha256: hash(b"placeholder"),
        kind: MovieKind::OggNamedRmv,
    };
    let candidate = RuntimeMovieCandidate {
        path: PathBuf::from("selected/gracie.rmv"),
        byte_len: 1,
        sha256: hash(b"x"),
        kind: MovieKind::BinkV1,
    };
    assert_eq!(
        rule.decide(
            &evidence,
            Some(&candidate)
        ),
        RuntimeCompletionDecision::Incomplete {
            logical_name: "gracie".to_owned(),
            reason: "candidate-truncated".to_owned(),
        }
    );
}

#[test]
fn rejects_empty_runtime_candidate() {
    let rule = RuntimeCompletionRule {
        logical_name: "gracie".to_owned(),
        accepted_kind: MovieKind::BinkV1,
        min_byte_len: 5,
    };
    let evidence = MovieEvidence {
        logical_name: "gracie".to_owned(),
        byte_len: 1,
        sha256: hash(b"placeholder"),
        kind: MovieKind::OggNamedRmv,
    };
    let candidate = RuntimeMovieCandidate {
        path: PathBuf::from("selected/gracie.rmv"),
        byte_len: 0,
        sha256: hash(b""),
        kind: MovieKind::BinkV1,
    };
    assert_eq!(
        rule.decide(
            &evidence,
            Some(&candidate)
        ),
        RuntimeCompletionDecision::Incomplete {
            logical_name: "gracie".to_owned(),
            reason: "candidate-empty".to_owned(),
        }
    );
}

#[test]
fn rejects_replacement_when_source_is_already_complete() {
    let rule = RuntimeCompletionRule {
        logical_name: "gracie".to_owned(),
        accepted_kind: MovieKind::BinkV1,
        min_byte_len: 100_000,
    };
    let evidence = MovieEvidence {
        logical_name: "gracie".to_owned(),
        byte_len: 200_000,
        sha256: hash(b"complete"),
        kind: MovieKind::BinkV1,
    };
    let candidate = RuntimeMovieCandidate {
        path: PathBuf::from("selected/gracie.rmv"),
        byte_len: 851_028,
        sha256: hash(b"replacement"),
        kind: MovieKind::BinkV1,
    };
    assert_eq!(
        rule.decide(
            &evidence,
            Some(&candidate)
        ),
        RuntimeCompletionDecision::Incomplete {
            logical_name: "gracie".to_owned(),
            reason: "source-already-complete".to_owned(),
        }
    );
}

#[test]
fn rejects_invalid_logical_movie_names() {
    for logical_name in [
        "",
        ".",
        "..",
        "folder/name",
        r"folder
// cspell:disable-next-line -- ame
ame",
        "CON",
        "movie.",
        "movie ",
        "movie?alt",
    ] {
        let rule = RuntimeCompletionRule {
            logical_name: logical_name.to_owned(),
            accepted_kind: MovieKind::BinkV1,
            min_byte_len: 100_000,
        };
        let evidence = MovieEvidence {
            logical_name: logical_name.to_owned(),
            byte_len: 2_887,
            sha256: hash(b"small"),
            kind: MovieKind::OggNamedRmv,
        };
        assert_eq!(
            rule.decide(
                &evidence, None
            ),
            RuntimeCompletionDecision::Incomplete {
                logical_name: logical_name.to_owned(),
                reason: "invalid-logical-name".to_owned(),
            },
            "invalid logical movie name was accepted: {logical_name:?}"
        );
    }
}

#[test]
fn rejects_unicode_expansion_aliases() {
    let rule = RuntimeCompletionRule {
        // cspell:disable-next-line -- straße
        logical_name: "straße".to_owned(),
        accepted_kind: MovieKind::BinkV1,
        min_byte_len: 100_000,
    };
    let evidence = MovieEvidence {
        // cspell:disable-next-line -- straße
        logical_name: "straße".to_owned(),
        byte_len: 2_887,
        sha256: hash(b"small"),
        kind: MovieKind::OggNamedRmv,
    };
    let candidate = RuntimeMovieCandidate {
        // cspell:disable-next-line -- strasse
        path: PathBuf::from("selected/strasse.rmv"),
        byte_len: 851_028,
        sha256: hash(b"large"),
        kind: MovieKind::BinkV1,
    };
    assert_eq!(
        rule.decide(
            &evidence,
            Some(&candidate)
        ),
        RuntimeCompletionDecision::Incomplete {
            // cspell:disable-next-line -- straße
            logical_name: "straße".to_owned(),
            reason: "candidate-name-mismatch".to_owned(),
        }
    );
}

#[test]
fn accepts_unicode_case_variants_of_movie_identity() {
    let rule = RuntimeCompletionRule {
        // cspell:disable-next-line -- GRÄCIE
        logical_name: "GRÄCIE".to_owned(),
        accepted_kind: MovieKind::BinkV1,
        min_byte_len: 100_000,
    };
    let evidence = MovieEvidence {
        // cspell:disable-next-line -- gräcie
        logical_name: "gräcie".to_owned(),
        byte_len: 2_887,
        sha256: hash(b"small"),
        kind: MovieKind::OggNamedRmv,
    };
    let candidate = RuntimeMovieCandidate {
        // cspell:disable-next-line -- Gräcie
        path: PathBuf::from("selected/Gräcie.RMV"),
        byte_len: 851_028,
        sha256: hash(b"large"),
        kind: MovieKind::BinkV1,
    };
    assert!(
        matches!(
            rule.decide(
                &evidence,
                Some(&candidate)
            ),
            RuntimeCompletionDecision::Ready { .. }
        )
    );
}

#[test]
fn accepts_ascii_case_variants_in_movie_evidence() {
    let rule = RuntimeCompletionRule {
        logical_name: "Gracie".to_owned(),
        accepted_kind: MovieKind::BinkV1,
        min_byte_len: 100_000,
    };
    let evidence = MovieEvidence {
        logical_name: "gracie".to_owned(),
        byte_len: 2_887,
        sha256: hash(b"small"),
        kind: MovieKind::OggNamedRmv,
    };
    let candidate = RuntimeMovieCandidate {
        path: PathBuf::from("selected/GRACIE.rmv"),
        byte_len: 851_028,
        sha256: hash(b"large"),
        kind: MovieKind::BinkV1,
    };
    assert!(
        matches!(
            rule.decide(
                &evidence,
                Some(&candidate)
            ),
            RuntimeCompletionDecision::Ready { .. }
        )
    );
}

#[test]
fn accepts_ascii_case_variants_of_the_same_movie_name() {
    let rule = RuntimeCompletionRule {
        logical_name: "Gracie".to_owned(),
        accepted_kind: MovieKind::BinkV1,
        min_byte_len: 100_000,
    };
    let evidence = MovieEvidence {
        logical_name: "Gracie".to_owned(),
        byte_len: 2_887,
        sha256: hash(b"small"),
        kind: MovieKind::OggNamedRmv,
    };
    let candidate = RuntimeMovieCandidate {
        path: PathBuf::from("selected/gracie.RMV"),
        byte_len: 851_028,
        sha256: hash(b"large"),
        kind: MovieKind::BinkV1,
    };
    assert!(
        matches!(
            rule.decide(
                &evidence,
                Some(&candidate)
            ),
            RuntimeCompletionDecision::Ready { .. }
        )
    );
}

#[test]
fn rejects_candidate_for_a_different_movie_name() {
    let rule = RuntimeCompletionRule {
        logical_name: "gracie".to_owned(),
        accepted_kind: MovieKind::BinkV1,
        min_byte_len: 100_000,
    };
    let evidence = MovieEvidence {
        logical_name: "gracie".to_owned(),
        byte_len: 2_887,
        sha256: hash(b"small"),
        kind: MovieKind::OggNamedRmv,
    };
    let candidate = RuntimeMovieCandidate {
        path: PathBuf::from("selected/other.rmv"),
        byte_len: 851_028,
        sha256: hash(b"large"),
        kind: MovieKind::BinkV1,
    };
    assert_eq!(
        rule.decide(
            &evidence,
            Some(&candidate)
        ),
        RuntimeCompletionDecision::Incomplete {
            logical_name: "gracie".to_owned(),
            reason: "candidate-name-mismatch".to_owned(),
        }
    );
}

#[test]
fn accepts_distinct_runtime_candidate() {
    let rule = RuntimeCompletionRule {
        logical_name: "gracie".to_owned(),
        accepted_kind: MovieKind::BinkV1,
        min_byte_len: 100_000,
    };
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
    assert_eq!(
        rule.decide(
            &evidence,
            Some(&candidate)
        ),
        RuntimeCompletionDecision::Ready {
            logical_name: "gracie".to_owned(),
            candidate_path: PathBuf::from("selected/gracie.rmv"),
            candidate_sha256: candidate.sha256,
        }
    );
}

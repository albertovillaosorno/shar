// File:
//   - runtime_source.rs
// Path:
//   - src/rmv/src/domain/runtime_source.rs
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
//   - Pure rmv domain rules for domain runtime source.
// - Must-Not:
//   - Read files, parse generated indexes, invoke CLI code, or call writer
//   - adapters.
// - Allows:
//   - Value objects, invariant checks, and pure evidence-to-domain translation.
// - Split-When:
//   - Split when runtime source contains two independently testable contracts.
// - Merge-When:
//   - Another rmv module owns the same domain boundary with no distinct
//   - invariant.
// - Summary:
//   - Runtimemoviecandidate.
// - Description:
//   - Defines runtime source data and behavior for rmv domain.
// - Usage:
//   - Imported through crate domain facades or sibling domain modules.
// - Defaults:
//   - No filesystem paths, no external process calls, and no implicit IO
//   - defaults.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Runtimemoviecandidate.
//!
//! This boundary keeps runtimemoviecandidate explicit and returns
//! deterministic results to rmv callers.
use std::path::PathBuf;

use crate::domain::{MovieKind, Sha256, is_windows_safe_component};

#[derive(Debug, Clone, PartialEq, Eq)]
/// Movieevidence.
pub struct MovieEvidence {
    /// Logical name.
    pub logical_name: String,
    /// Byte len.
    pub byte_len: u64,
    /// Sha256.
    pub sha256: Sha256,
    /// Kind.
    pub kind: MovieKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Runtimemoviecandidate.
pub struct RuntimeMovieCandidate {
    /// Path.
    pub path: PathBuf,
    /// Byte len.
    pub byte_len: u64,
    /// Sha256.
    pub sha256: Sha256,
    /// Kind.
    pub kind: MovieKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Runtimecompletionrule.
pub struct RuntimeCompletionRule {
    /// Logical name.
    pub logical_name: String,
    /// Accepted kind.
    pub accepted_kind: MovieKind,
    /// Min byte len.
    pub min_byte_len: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Runtimecompletiondecision.
pub enum RuntimeCompletionDecision {
    /// Item.
    Ready {
        /// Logical name.
        logical_name: String,
        /// Candidate path.
        candidate_path: PathBuf,
        /// Candidate sha256.
        candidate_sha256: Sha256,
    },
    /// Item.
    Incomplete {
        /// Logical name.
        logical_name: String,
        /// Reason.
        reason: String,
    },
}

impl RuntimeCompletionRule {
    #[must_use]
    /// Decide.
    pub fn decide(
        &self,
        evidence: &MovieEvidence,
        candidate: Option<&RuntimeMovieCandidate>,
    ) -> RuntimeCompletionDecision {
        if !is_windows_safe_component(&self.logical_name) {
            return incomplete_runtime(
                &self.logical_name,
                "invalid-logical-name",
            );
        }
        if self.min_byte_len <= 4 {
            return incomplete_runtime(
                &self.logical_name,
                "invalid-min-byte-len",
            );
        }
        if !matches!(
            self.accepted_kind,
            MovieKind::BinkV1 | MovieKind::BinkV2 | MovieKind::XboxXmvLike
        ) {
            return incomplete_runtime(
                &self.logical_name,
                "invalid-accepted-kind",
            );
        }
        if !movie_names_match(
            &evidence.logical_name,
            &self.logical_name,
        ) {
            return incomplete_runtime(
                &evidence.logical_name,
                "logical-name-mismatch",
            );
        }
        if evidence.kind == self.accepted_kind
            && evidence.byte_len >= self.min_byte_len
        {
            return incomplete_runtime(
                &self.logical_name,
                "source-already-complete",
            );
        }
        let Some(runtime_candidate) = candidate else {
            return incomplete_runtime(
                &self.logical_name,
                "missing-runtime-candidate",
            );
        };

        let rejection_reason = match () {
            () if !candidate_matches_logical_name(
                &runtime_candidate.path,
                &self.logical_name,
            ) =>
            {
                Some("candidate-name-mismatch")
            }
            () if runtime_candidate.kind != self.accepted_kind => {
                Some("candidate-kind-mismatch")
            }
            () if runtime_candidate.byte_len == 0 => Some("candidate-empty"),
            () if runtime_candidate.byte_len <= 4 => {
                Some("candidate-truncated")
            }
            () if runtime_candidate.byte_len < self.min_byte_len => {
                Some("candidate-too-small")
            }
            () if runtime_candidate.sha256 == evidence.sha256 => {
                Some("candidate-same-as-incomplete-input")
            }
            () => None,
        };

        if let Some(reason) = rejection_reason {
            return incomplete_runtime(
                &self.logical_name,
                reason,
            );
        }
        RuntimeCompletionDecision::Ready {
            logical_name: self
                .logical_name
                .clone(),
            candidate_path: runtime_candidate
                .path
                .clone(),
            candidate_sha256: runtime_candidate.sha256,
        }
    }
}

/// Compares movie identities using Unicode case folding suitable for Windows
/// filename identity while preserving exact equality as the fast path.
fn movie_names_match(
    left: &str,
    right: &str,
) -> bool {
    if left == right {
        return true;
    }
    if left
        .chars()
        .count()
        != right
            .chars()
            .count()
    {
        return false;
    }
    left.to_uppercase() == right.to_uppercase()
}

/// Verifies that a local override is the same named RMV file required by the
/// runtime completion rule.
fn candidate_matches_logical_name(
    path: &std::path::Path,
    logical_name: &str,
) -> bool {
    path.file_stem()
        .and_then(std::ffi::OsStr::to_str)
        .is_some_and(
            |stem| {
                movie_names_match(
                    stem,
                    logical_name,
                )
            },
        )
        && path
            .extension()
            .and_then(std::ffi::OsStr::to_str)
            .is_some_and(|extension| extension.eq_ignore_ascii_case("rmv"))
}

/// Centralizes runtime rejection construction so completion checks keep one
/// fail-closed vocabulary for packaging decisions.
fn incomplete_runtime(
    logical_name: &str,
    reason: &str,
) -> RuntimeCompletionDecision {
    RuntimeCompletionDecision::Incomplete {
        logical_name: logical_name.to_owned(),
        reason: reason.to_owned(),
    }
}

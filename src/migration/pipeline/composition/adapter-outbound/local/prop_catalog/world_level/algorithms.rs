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
//   - Algorithms outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Algorithms outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Algorithms outbound adapter.

use std::ffi::OsStr;
use std::path::Path;

use fbx::domain::mesh::MeshAsset;

use self::model::{FbxFingerprint, FbxRepairAlgorithm};
use crate::domain::PipelineError;

mod dataset;
mod model;

/// Minimum accepted structural similarity in basis points.
const REQUIRED_SIMILARITY: u16 = 9_900;
/// Minimum lead over the second structural candidate in basis points.
const REQUIRED_MARGIN: u16 = 25;
/// Complete structural similarity score in basis points.
const COMPLETE_SIMILARITY: u16 = 10_000;

/// Apply the uniquely matched repair registered for one generated FBX.
///
/// Matching priority is exact relative path, exact filename stem, unique
/// prefix, then structural similarity of at least 99 percent. Similarity must
/// also lead the runner-up by the configured ambiguity margin. An empty
/// registry is a deterministic no-op.
///
/// # Errors
///
/// Returns an error when identity or similarity matching is ambiguous, count
/// arithmetic fails, or the selected repair rejects its source-derived meshes.
pub(super) fn apply_registered_algorithm(
    relative_path: &str,
    meshes: &mut [MeshAsset],
) -> Result<(), PipelineError> {
    let registry = dataset::registered_algorithms();
    if registry.is_empty() {
        return Ok(());
    }
    let fingerprint = FbxFingerprint::from_meshes(meshes)?;
    let Some(algorithm) =
        select_algorithm(relative_path, fingerprint, registry)?
    else {
        return Ok(());
    };
    ensure_source_compatible(relative_path, fingerprint, algorithm)?;
    (algorithm.apply)(relative_path, meshes)
}

/// Reject one selected repair when its registered source structure has drifted.
fn ensure_source_compatible(
    relative_path: &str,
    fingerprint: FbxFingerprint,
    algorithm: &FbxRepairAlgorithm,
) -> Result<(), PipelineError> {
    let score = similarity(fingerprint, algorithm.source_fingerprint)?;
    if score < REQUIRED_SIMILARITY {
        return Err(PipelineError::new(format!(
            "FBX repair source fingerprint changed: \
                     {relative_path}:{score}"
        )));
    }
    Ok(())
}

/// Select one repair using deterministic identity and similarity precedence.
fn select_algorithm<'algorithm>(
    relative_path: &str,
    fingerprint: FbxFingerprint,
    registry: &'algorithm [FbxRepairAlgorithm],
) -> Result<Option<&'algorithm FbxRepairAlgorithm>, PipelineError> {
    let normalized_path = normalize_path(relative_path);
    if let Some(found) = unique_candidate(
        registry.iter().filter(|algorithm| {
            normalize_path(algorithm.relative_path) == normalized_path
        }),
        relative_path,
        "relative path",
    )? {
        return Ok(Some(found));
    }

    let stem = normalized_stem(relative_path)?;
    if let Some(found) = unique_candidate(
        registry.iter().filter(|algorithm| {
            normalize_identity(algorithm.file_stem) == stem
        }),
        relative_path,
        "filename stem",
    )? {
        return Ok(Some(found));
    }

    if let Some(found) = unique_candidate(
        registry.iter().filter(|algorithm| {
            let prefix = normalize_identity(algorithm.file_prefix);
            !prefix.is_empty() && stem.starts_with(&prefix)
        }),
        relative_path,
        "filename prefix",
    )? {
        return Ok(Some(found));
    }

    select_similar(relative_path, fingerprint, registry)
}

/// Return one candidate only when one matching stage is unambiguous.
fn unique_candidate<'algorithm>(
    candidates: impl Iterator<Item = &'algorithm FbxRepairAlgorithm>,
    relative_path: &str,
    stage: &str,
) -> Result<Option<&'algorithm FbxRepairAlgorithm>, PipelineError> {
    let mut matches = candidates;
    let first = matches.next();
    if matches.next().is_some() {
        return Err(PipelineError::new(format!(
            "FBX repair {stage} is ambiguous: {relative_path}"
        )));
    }
    Ok(first)
}

/// Select one structurally similar candidate or fail closed on ambiguity.
fn select_similar<'algorithm>(
    relative_path: &str,
    fingerprint: FbxFingerprint,
    registry: &'algorithm [FbxRepairAlgorithm],
) -> Result<Option<&'algorithm FbxRepairAlgorithm>, PipelineError> {
    let mut scored = registry
        .iter()
        .map(|algorithm| {
            similarity(fingerprint, algorithm.source_fingerprint)
                .map(|score| (algorithm, score))
        })
        .collect::<Result<Vec<_>, _>>()?;
    scored.sort_unstable_by(|left, right| {
        right
            .1
            .cmp(&left.1)
            .then_with(|| left.0.relative_path.cmp(right.0.relative_path))
    });
    let Some((best, best_score)) = scored.first().copied() else {
        return Ok(None);
    };
    if best_score < REQUIRED_SIMILARITY {
        return Ok(None);
    }
    if let Some((_, second_score)) = scored.get(1).copied() {
        let margin = best_score.checked_sub(second_score).ok_or_else(|| {
            PipelineError::new("FBX repair similarity ordering changed")
        })?;
        if margin < REQUIRED_MARGIN {
            return Err(PipelineError::new(format!(
                "FBX repair structural match is ambiguous: \
                         {relative_path}:{best_score}:{second_score}"
            )));
        }
    }
    Ok(Some(best))
}

/// Return the weakest count similarity across all structural dimensions.
fn similarity(
    left: FbxFingerprint,
    right: FbxFingerprint,
) -> Result<u16, PipelineError> {
    let mut weakest = COMPLETE_SIMILARITY;
    for (left_dimension, right_dimension) in
        left.dimensions().into_iter().zip(right.dimensions())
    {
        weakest = weakest.min(ratio(left_dimension, right_dimension)?);
    }
    Ok(weakest)
}

/// Return one count ratio in basis points.
fn ratio(left: u64, right: u64) -> Result<u16, PipelineError> {
    if left == right {
        return Ok(COMPLETE_SIMILARITY);
    }
    let smaller = left.min(right);
    let larger = left.max(right);
    let scaled = smaller
        .checked_mul(u64::from(COMPLETE_SIMILARITY))
        .ok_or_else(|| PipelineError::new("FBX repair ratio overflowed"))?;
    let value = scaled.checked_div(larger).ok_or_else(|| {
        PipelineError::new("FBX repair ratio divided by zero")
    })?;
    u16::try_from(value).map_err(|error| {
        PipelineError::new(format!(
            "FBX repair ratio conversion failed: {error}"
        ))
    })
}

/// Normalize one portable relative path for case-insensitive matching.
fn normalize_path(value: &str) -> String {
    value
        .replace('\\', "/")
        .trim_matches('/')
        .to_ascii_lowercase()
}

/// Return one normalized filename stem.
fn normalized_stem(relative_path: &str) -> Result<String, PipelineError> {
    Path::new(relative_path)
        .file_stem()
        .and_then(OsStr::to_str)
        .map(normalize_identity)
        .filter(|stem| !stem.is_empty())
        .ok_or_else(|| {
            PipelineError::new(format!(
                "FBX repair filename stem is missing: {relative_path}"
            ))
        })
}

/// Normalize one filename identity to lowercase ASCII words separated by `-`.
fn normalize_identity(value: &str) -> String {
    let mut result = String::with_capacity(value.len());
    let mut pending_separator = false;
    for character in value.chars() {
        if character.is_ascii_alphanumeric() {
            if pending_separator && !result.is_empty() {
                result.push('-');
            }
            result.push(character.to_ascii_lowercase());
            pending_separator = false;
        } else if !result.is_empty() {
            pending_separator = true;
        }
    }
    result
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/prop_catalog/world_level/algorithms/tests.rs"]
mod tests;

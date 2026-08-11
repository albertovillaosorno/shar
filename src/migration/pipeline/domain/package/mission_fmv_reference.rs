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
//   - Canonical package binding for objective `SetFMVInfo` story movies.
// - Must-Not:
//   - Infer playback, audio, music-stop, transition, or completion behavior.
// - Allows:
//   - Bind exact authored RMV identities to reviewed story-movie packages.
// - Split-When:
//   - Movie playback policy or another source movie namespace gains authority.
// - Merge-When:
//   - Final mission-definition compilation owns these exact media references.
// - Summary:
//   - Mission FMV package-reference preflight.
// - Description:
//   - Resolves typed story FMV source paths to one canonical movie member.
// - Usage:
//   - Runs after objective semantics and phase-three package-index intake.
// - Defaults:
//   - Missing, ambiguous, malformed, or memberless story movies fail closed.
//

//! Canonical package binding for objective `SetFMVInfo` story movies.

use std::path::Path;

use super::{
    MissionObjectiveDirective, MissionObjectiveSemanticReport, PackageRole,
    PhaseThreePackageIndex, PhaseThreePackageRow,
};

/// One authored mission FMV bound to a canonical story-movie package.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionFmvReferenceBinding {
    owner_stage_source_ordinal: usize,
    owner_stage_sequence_ordinal: usize,
    objective_source_ordinal: usize,
    source_ordinal: usize,
    rmv_path: String,
    legacy_argument: Option<String>,
    package_id: String,
    package_root: String,
    package_subcategory: String,
    movie_id: String,
    movie_path: String,
}

impl MissionFmvReferenceBinding {
    /// Return source `AddStage` ordinal owning the FMV objective.
    #[must_use]
    pub const fn owner_stage_source_ordinal(&self) -> usize {
        self.owner_stage_source_ordinal
    }

    /// Return dense authored stage ordinal owning the FMV objective.
    #[must_use]
    pub const fn owner_stage_sequence_ordinal(&self) -> usize {
        self.owner_stage_sequence_ordinal
    }

    /// Return the source `AddObjective` ordinal.
    #[must_use]
    pub const fn objective_source_ordinal(&self) -> usize {
        self.objective_source_ordinal
    }

    /// Return the source `SetFMVInfo` ordinal.
    #[must_use]
    pub const fn source_ordinal(&self) -> usize {
        self.source_ordinal
    }

    /// Return the exact authored RMV path.
    #[must_use]
    pub fn rmv_path(&self) -> &str {
        &self.rmv_path
    }

    /// Return the optional exact opaque compatibility argument.
    #[must_use]
    pub fn legacy_argument(&self) -> Option<&str> {
        self.legacy_argument.as_deref()
    }

    /// Return the canonical package id.
    #[must_use]
    pub fn package_id(&self) -> &str {
        &self.package_id
    }

    /// Return the canonical package root.
    #[must_use]
    pub fn package_root(&self) -> &str {
        &self.package_root
    }

    /// Return the exact story-movie package subcategory.
    #[must_use]
    pub fn package_subcategory(&self) -> &str {
        &self.package_subcategory
    }

    /// Return the canonical converted movie member id.
    #[must_use]
    pub fn movie_id(&self) -> &str {
        &self.movie_id
    }

    /// Return the canonical converted movie member path.
    #[must_use]
    pub fn movie_path(&self) -> &str {
        &self.movie_path
    }
}

/// Canonical mission FMV references in source order.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MissionFmvReferenceReport {
    bindings: Vec<MissionFmvReferenceBinding>,
}

impl MissionFmvReferenceReport {
    /// Return bindings in source objective/directive order.
    #[must_use]
    pub fn bindings(&self) -> &[MissionFmvReferenceBinding] {
        &self.bindings
    }
}

/// Bind every typed objective `SetFMVInfo` to one canonical story movie.
///
/// # Errors
///
/// Returns an error when an authored RMV path is unsafe, absent, ambiguous, or
/// does not own exactly one canonical converted movie member.
pub fn preflight_mission_fmv_references(
    index: &PhaseThreePackageIndex,
    objectives: &MissionObjectiveSemanticReport,
) -> Result<MissionFmvReferenceReport, String> {
    let mut bindings = Vec::new();
    for objective in objectives.objectives() {
        for directive in objective.directives() {
            let MissionObjectiveDirective::FmvInfo {
                source_ordinal,
                rmv_path,
                legacy_argument,
            } = directive
            else {
                continue;
            };
            let package = resolve_story_movie(index, rmv_path)?;
            let [movie_id] = package.ids_for_role(PackageRole::Movie) else {
                return Err(
                    "mission FMV package must own exactly one movie id"
                        .to_owned(),
                );
            };
            let matching = package
                .members()
                .iter()
                .filter(|member| {
                    member.role == PackageRole::Movie && member.id == *movie_id
                })
                .collect::<Vec<_>>();
            let [member] = matching.as_slice() else {
                return Err(
                    "mission FMV movie id has no unique physical member"
                        .to_owned(),
                );
            };
            bindings.push(MissionFmvReferenceBinding {
                owner_stage_source_ordinal:
                    objective.owner_stage_source_ordinal(),
                owner_stage_sequence_ordinal:
                    objective.owner_stage_sequence_ordinal(),
                objective_source_ordinal: objective.source_ordinal(),
                source_ordinal: *source_ordinal,
                rmv_path: rmv_path.clone(),
                legacy_argument: legacy_argument.clone(),
                package_id: package.package_id.clone(),
                package_root: package.package_root.clone(),
                package_subcategory: package.subcategory().to_owned(),
                movie_id: movie_id.clone(),
                movie_path: member.path.clone(),
            });
        }
    }
    Ok(MissionFmvReferenceReport { bindings })
}

fn resolve_story_movie<'index>(
    index: &'index PhaseThreePackageIndex,
    rmv_path: &str,
) -> Result<&'index PhaseThreePackageRow, String> {
    let stem = story_movie_stem(rmv_path)?;
    let subcategory = format!("movies/story/{stem}");
    let matching = index
        .packages()
        .iter()
        .filter(|package| {
            package.category() == "movies"
                && package.subcategory().eq_ignore_ascii_case(&subcategory)
        })
        .collect::<Vec<_>>();
    let [package] = matching.as_slice() else {
        return Err(format!(
            "mission FMV `{rmv_path}` has no unique story-movie package"
        ));
    };
    Ok(*package)
}

fn story_movie_stem(rmv_path: &str) -> Result<String, String> {
    if rmv_path.is_empty()
        || rmv_path != rmv_path.trim()
        || rmv_path.chars().any(char::is_control)
        || rmv_path.contains(char::from(92))
        || rmv_path.contains('/')
        || rmv_path.contains(':')
    {
        return Err(
            "mission FMV path is not a safe authored basename".to_owned(),
        );
    }
    let path = Path::new(rmv_path);
    if !path
        .extension()
        .and_then(|value| value.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case("rmv"))
    {
        return Err("mission FMV path must use the RMV extension".to_owned());
    }
    let stem = path
        .file_stem()
        .and_then(|value| value.to_str())
        .ok_or_else(|| "mission FMV path has no UTF-8 stem".to_owned())?;
    if stem.is_empty()
        || !stem
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
    {
        return Err("mission FMV stem is malformed".to_owned());
    }
    Ok(stem.to_ascii_lowercase())
}

#[cfg(test)]
// jig-ignore-next-line: exact Rust test-module path is indivisible.
#[path = "../../../../../tests/migration/pipeline/unit/domain/package/mission_fmv_reference/tests.rs"]
mod tests;

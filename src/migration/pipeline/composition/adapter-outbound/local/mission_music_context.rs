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
//   - Filesystem-backed binding of mission music-state source pairs to the
//     canonical compiled score metadata for the same level.
// - Must-Not:
//   - Infer playback, transition, event, mix, or decoded RADMusic state-machine
//     semantics from symbol names or adjacency.
// - Allows:
//   - Validate exact package/script provenance and reviewed named-asset
//     windows.
// - Split-When:
//   - Music events or decoded RADMusic structures gain independent authority.
// - Merge-When:
//   - Final mission audio compilation owns the same source metadata contract.
// - Summary:
//   - Mission music-state metadata context.
// - Description:
//   - Binds authored state/value tokens to exact offsets in indexed level score
//     metadata while preserving the structural-evidence boundary.
// - Usage:
//   - Runs once after cross-source mission snapshots and package-index intake.
// - Defaults:
//   - Missing, ambiguous, malformed, or wrong-level metadata fails closed.
//

//! Filesystem-backed mission music-state metadata context.

use std::collections::BTreeMap;
use std::path::{Component, Path, PathBuf};

use schoenwald_filesystem::adapters::driving::local::read_utf8;
use serde_json::Value;

use super::mission_locator_context::MissionLocatorScriptSnapshot;
use crate::domain::{
    MissionStageDirective, PackageRole, PhaseThreePackageIndex,
    PhaseThreePackageMember, PhaseThreePackageRow, PipelineError,
    PipelineOutcome, compile_mission_scope_graphs,
    preflight_mission_stage_semantics,
};

const SCORE_SUBCATEGORY: &str = "music/bank-01/score-library";
const MUSIC_SCHEMA: &str = "shar-schoenwald.radmusic-compiled.v3";

#[derive(Clone, Debug, Eq, PartialEq)]
struct NamedAsset {
    offset: u64,
    value: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct LevelMusicMetadata {
    script_id: String,
    script_path: String,
    named_assets: Vec<NamedAsset>,
}

/// One exact authored music-state pair bound to compiled source offsets.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct MissionMusicStateBinding {
    source_path: String,
    owner_stage_source_ordinal: usize,
    owner_stage_sequence_ordinal: usize,
    source_ordinal: usize,
    level: u8,
    state_name: String,
    state_value: String,
    package_id: String,
    script_id: String,
    script_path: String,
    state_offset: u64,
    value_offset: u64,
}

impl MissionMusicStateBinding {
    #[cfg(test)]
    pub(super) fn source_path(&self) -> &str {
        &self.source_path
    }

    #[cfg(test)]
    pub(super) const fn owner_stage_source_ordinal(&self) -> usize {
        self.owner_stage_source_ordinal
    }

    #[cfg(test)]
    pub(super) const fn owner_stage_sequence_ordinal(&self) -> usize {
        self.owner_stage_sequence_ordinal
    }

    #[cfg(test)]
    pub(super) const fn source_ordinal(&self) -> usize {
        self.source_ordinal
    }

    #[cfg(test)]
    pub(super) const fn level(&self) -> u8 {
        self.level
    }

    #[cfg(test)]
    pub(super) fn state_name(&self) -> &str {
        &self.state_name
    }

    #[cfg(test)]
    pub(super) fn state_value(&self) -> &str {
        &self.state_value
    }

    #[cfg(test)]
    pub(super) fn package_id(&self) -> &str {
        &self.package_id
    }

    #[cfg(test)]
    pub(super) fn script_id(&self) -> &str {
        &self.script_id
    }

    #[cfg(test)]
    pub(super) fn script_path(&self) -> &str {
        &self.script_path
    }

    #[cfg(test)]
    pub(super) const fn offsets(&self) -> (u64, u64) {
        (self.state_offset, self.value_offset)
    }
}

/// Reviewed mission music-state bindings in source order.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(super) struct MissionMusicStateReport {
    bindings: Vec<MissionMusicStateBinding>,
}

impl MissionMusicStateReport {
    #[cfg(test)]
    pub(super) fn bindings(&self) -> &[MissionMusicStateBinding] {
        &self.bindings
    }
}

/// Bind every typed mission `SetMusicState` pair to same-level score metadata.
///
/// # Errors
///
/// Returns an error for package/script drift, unsafe member paths, malformed
/// metadata, invalid source levels, or non-unique reviewed symbol windows.
pub(super) fn preflight_mission_music_states(
    index: &PhaseThreePackageIndex,
    extracted_root: &Path,
    snapshots: &[MissionLocatorScriptSnapshot],
) -> PipelineOutcome<MissionMusicStateReport> {
    let score = score_package(index)?;
    let mut cache = BTreeMap::<u8, LevelMusicMetadata>::new();
    let mut bindings = Vec::new();
    for snapshot in snapshots {
        let scopes = compile_mission_scope_graphs(snapshot.evidence())
            .map_err(|error| {
                PipelineError::new(format!(
                    "mission music scope preflight failed: {error}"
                ))
            })?;
        let stages = preflight_mission_stage_semantics(&scopes)
            .map_err(|error| {
                PipelineError::new(format!(
                    "mission music stage preflight failed: {error}"
                ))
            })?;
        let mut directives = Vec::new();
        for stage in stages.stages() {
            for directive in stage.directives() {
                if let MissionStageDirective::MusicState {
                    source_ordinal,
                    state_name,
                    state_value,
                } = directive
                {
                    directives.push((
                        stage.source_ordinal(),
                        stage.sequence_ordinal(),
                        *source_ordinal,
                        state_name,
                        state_value,
                    ));
                }
            }
        }
        if directives.is_empty() {
            continue;
        }
        let level = source_level(snapshot.source_path())?;
        if !cache.contains_key(&level) {
            let metadata = load_level_music_metadata(
                score,
                extracted_root,
                level,
            )?;
            let previous = cache.insert(level, metadata);
            debug_assert!(previous.is_none());
        }
        let metadata = cache.get(&level).ok_or_else(|| {
            PipelineError::new("mission music metadata cache lost a level")
        })?;
        for (
            owner_stage_source_ordinal,
            owner_stage_sequence_ordinal,
            source_ordinal,
            state_name,
            state_value,
        ) in directives
        {
            let (state_offset, value_offset) = resolve_named_asset_window(
                &metadata.named_assets,
                state_name,
                state_value,
            )?;
            bindings.push(MissionMusicStateBinding {
                source_path: snapshot.source_path().to_owned(),
                owner_stage_source_ordinal,
                owner_stage_sequence_ordinal,
                source_ordinal,
                level,
                state_name: state_name.to_owned(),
                state_value: state_value.to_owned(),
                package_id: score.package_id.clone(),
                script_id: metadata.script_id.clone(),
                script_path: metadata.script_path.clone(),
                state_offset,
                value_offset,
            });
        }
    }
    Ok(MissionMusicStateReport { bindings })
}

fn score_package(
    index: &PhaseThreePackageIndex,
) -> PipelineOutcome<&PhaseThreePackageRow> {
    let matching = index
        .packages()
        .iter()
        .filter(|package| {
            package.category() == "music"
                && package.subcategory() == SCORE_SUBCATEGORY
        })
        .collect::<Vec<_>>();
    let [package] = matching.as_slice() else {
        return Err(PipelineError::new(
            "mission music score-library package is not unique",
        ));
    };
    Ok(*package)
}

fn load_level_music_metadata(
    score: &PhaseThreePackageRow,
    extracted_root: &Path,
    level: u8,
) -> PipelineOutcome<LevelMusicMetadata> {
    let expected_path = format!("{}/l{level}_music.json", score.package_root);
    let matching = score
        .members()
        .iter()
        .filter(|member| {
            member.role == PackageRole::Script
                && member.path.eq_ignore_ascii_case(&expected_path)
        })
        .collect::<Vec<_>>();
    let [member] = matching.as_slice() else {
        return Err(PipelineError::new(format!(
            "mission music level {level} script member is not unique"
        )));
    };
    validate_script_member(member)?;
    let physical = resolve_member_path(extracted_root, &member.path)?;
    let text = read_utf8(&physical).map_err(|error| {
        PipelineError::new(format!(
            "mission music metadata read failed: {error}"
        ))
    })?;
    Ok(LevelMusicMetadata {
        script_id: member.id.clone(),
        script_path: member.path.clone(),
        named_assets: parse_named_assets(&text)?,
    })
}

fn validate_script_member(
    member: &PhaseThreePackageMember,
) -> PipelineOutcome<()> {
    if member.unit_type != "script"
        || member.kind != "runtime-asset"
        || member.source_chunk_kind != "none"
    {
        return Err(PipelineError::new(
            "mission music script member classification drifted",
        ));
    }
    Ok(())
}

fn resolve_member_path(
    extracted_root: &Path,
    published: &str,
) -> PipelineOutcome<PathBuf> {
    let path = Path::new(published);
    let mut components = path.components();
    if components.next() != Some(Component::Normal("extracted".as_ref())) {
        return Err(PipelineError::new(
            "mission music member path is outside extracted root",
        ));
    }
    let relative = components.as_path();
    if relative
        .components()
        .any(|component| !matches!(component, Component::Normal(_)))
    {
        return Err(PipelineError::new(
            "mission music member path is not canonical",
        ));
    }
    Ok(extracted_root.join(relative))
}

fn parse_named_assets(text: &str) -> PipelineOutcome<Vec<NamedAsset>> {
    let value = serde_json::from_str::<Value>(text).map_err(|error| {
        PipelineError::new(format!(
            "mission music metadata JSON failed: {error}"
        ))
    })?;
    let object = value.as_object().ok_or_else(|| {
        PipelineError::new("mission music metadata must be a JSON object")
    })?;
    if object.get("schema").and_then(Value::as_str) != Some(MUSIC_SCHEMA) {
        return Err(PipelineError::new(
            "mission music metadata schema drifted",
        ));
    }
    let rows = object
        .get("named_assets")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            PipelineError::new(
                "mission music metadata named_assets must be an array",
            )
        })?;
    let mut assets = Vec::with_capacity(rows.len());
    let mut previous = None;
    for row in rows {
        let row = row.as_object().ok_or_else(|| {
            PipelineError::new("mission music named asset must be an object")
        })?;
        let offset = row
            .get("offset")
            .and_then(Value::as_u64)
            .ok_or_else(|| {
                PipelineError::new(
                    "mission music named asset offset is invalid",
                )
            })?;
        let value = row
            .get("value")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                PipelineError::new(
                    "mission music named asset value is invalid",
                )
            })?;
        if value.is_empty() || value.chars().any(char::is_control) {
            return Err(PipelineError::new(
                "mission music named asset value is malformed",
            ));
        }
        if previous.is_some_and(|prior| prior >= offset) {
            return Err(PipelineError::new(
                "mission music named asset offsets are not increasing",
            ));
        }
        previous = Some(offset);
        assets.push(NamedAsset {
            offset,
            value: value.to_owned(),
        });
    }
    Ok(assets)
}

fn resolve_named_asset_window(
    assets: &[NamedAsset],
    state_name: &str,
    state_value: &str,
) -> PipelineOutcome<(u64, u64)> {
    let mut matches = Vec::new();
    for (index, asset) in assets.iter().enumerate() {
        if asset.value != state_name {
            continue;
        }
        let end = index.saturating_add(3).min(assets.len());
        for candidate in assets
            .get(index.saturating_add(1)..end)
            .unwrap_or_default()
        {
            if candidate.value == state_value {
                matches.push((asset.offset, candidate.offset));
            }
        }
    }
    let [binding] = matches.as_slice() else {
        return Err(PipelineError::new(format!(
            "mission music state pair `{state_name}`/`{state_value}` has no \
             unique reviewed metadata window"
        )));
    };
    Ok(*binding)
}

pub(super) fn source_level(source_path: &str) -> PipelineOutcome<u8> {
    let normalized = source_path.replace(char::from(92), "/");
    let matching = normalized
        .split('/')
        .filter_map(|segment| segment.strip_prefix("level"))
        .filter(|suffix| {
            suffix.len() == 2
                && suffix.bytes().all(|byte| byte.is_ascii_digit())
        })
        .filter_map(|suffix| suffix.parse::<u8>().ok())
        .collect::<Vec<_>>();
    let [level] = matching.as_slice() else {
        return Err(PipelineError::new(
            "mission music source path has no unique level segment",
        ));
    };
    if !(1..=7).contains(level) {
        return Err(PipelineError::new(
            "mission music source level is outside base levels",
        ));
    }
    Ok(*level)
}

#[cfg(test)]
// jig-ignore-next-line: exact Rust test-module path is indivisible.
#[path = "../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/mission_music_context/tests.rs"]
mod tests;

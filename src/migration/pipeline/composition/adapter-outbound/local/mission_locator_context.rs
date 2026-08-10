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
//   - Cross-script active package context for typed mission locator binding.
// - Must-Not:
//   - Read files, resolve locators, or infer runtime package precedence.
// - Allows:
//   - Pair selected mission init/load scripts and their level-load family.
// - Split-When:
//   - Inventory-section provenance gains an independently reusable model.
// - Merge-When:
//   - Prepare-Unreal owns this exact cross-script package context directly.
// - Summary:
//   - Mission locator active-package context composition.
// - Description:
//   - Preserves level, mission, and initial dynamic package context without
//     assigning unsupported runtime lookup precedence.
// - Usage:
//   - Runs after lossless mission-script and package-load preflight.
// - Defaults:
//   - Missing sibling sources, unsafe paths, or ambiguous pairing fail closed.
//

//! Cross-script active-package context for mission locator resolution.

use std::collections::{BTreeMap, BTreeSet};

use crate::domain::{
    MissionInitializationBinding, MissionInitializationDirective,
    MissionLocatorActivePackageReport, MissionLocatorActivePackages, MissionScriptEvidence,
    compile_mission_scope_graphs, preflight_mission_initialization,
};

const MISSION_ROOT: &str = "extracted/game/scripts/missions/";
const INIT_SUFFIX: &str = "i.mfk.json";
const LOAD_SUFFIX: &str = "l.mfk.json";
const LEVEL_LOAD_SUFFIX: &str = "level.mfk.json";

/// One already-validated mission source plus its explicit P3D package loads.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct MissionLocatorScriptSnapshot {
    source_path: String,
    evidence: MissionScriptEvidence,
    package_roots: Vec<String>,
}

impl MissionLocatorScriptSnapshot {
    /// Build one cross-script locator context input.
    pub(super) const fn new(
        source_path: String,
        evidence: MissionScriptEvidence,
        package_roots: Vec<String>,
    ) -> Self {
        Self {
            source_path,
            evidence,
            package_roots,
        }
    }

    /// Return the exact normalized source path.
    pub(super) fn source_path(&self) -> &str {
        &self.source_path
    }

    /// Return structurally validated mission evidence.
    pub(super) const fn evidence(&self) -> &MissionScriptEvidence {
        &self.evidence
    }
}

/// Active package reports indexed by the exact selected-mission source path.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(super) struct MissionLocatorSourceContexts {
    by_source_path: BTreeMap<String, MissionLocatorActivePackageReport>,
}

impl MissionLocatorSourceContexts {
    /// Return active packages for one exact selected-mission source.
    pub(super) fn get(&self, source_path: &str) -> Option<&MissionLocatorActivePackageReport> {
        self.by_source_path.get(source_path)
    }

    /// Return the number of selected mission sources with package context.
    #[cfg(test)]
    pub(super) fn len(&self) -> usize {
        self.by_source_path.len()
    }
}

/// Build deterministic active-package context from validated script snapshots.
///
/// # Errors
///
/// Fails when a selected mission source cannot be paired exactly with its load
/// script and level-load family, or when source identities collide.
pub(super) fn build_mission_locator_source_contexts(
    snapshots: &[MissionLocatorScriptSnapshot],
    indexed_package_roots: &BTreeSet<String>,
) -> Result<MissionLocatorSourceContexts, String> {
    let mut by_path = BTreeMap::new();
    for snapshot in snapshots {
        if by_path
            .insert(snapshot.source_path.as_str(), snapshot)
            .is_some()
        {
            return Err("mission locator context source path is duplicated".to_owned());
        }
    }
    let available = by_path.keys().copied().collect::<BTreeSet<_>>();
    let mut contexts = BTreeMap::new();
    for snapshot in snapshots {
        let scopes = compile_mission_scope_graphs(&snapshot.evidence)?;
        if scopes.missions().is_empty() {
            continue;
        }
        let [mission] = scopes.missions() else {
            return Err("mission locator context source selects multiple missions".to_owned());
        };
        let mission_id = mission.source_mission_id();
        let (level_load_path, mission_load_path) =
            locator_context_paths(&snapshot.source_path, mission_id, &available)?;
        let level_load = by_path
            .get(level_load_path.as_str())
            .ok_or_else(|| "mission locator level-load source disappeared".to_owned())?;
        let mission_load = by_path
            .get(mission_load_path.as_str())
            .ok_or_else(|| "mission locator mission-load source disappeared".to_owned())?;
        let mut script_package_roots = level_load.package_roots.clone();
        script_package_roots.extend(mission_load.package_roots.iter().cloned());
        let initialization = preflight_mission_initialization(&scopes)?;
        let [initialization] = initialization.missions() else {
            return Err("mission locator initialization context drifted".to_owned());
        };
        if initialization.mission_id() != mission_id {
            return Err("mission locator initialization identity drifted".to_owned());
        }
        let initial_dynamic_package_roots = initial_dynamic_package_roots(
            initialization,
            indexed_package_roots,
        )?;
        let active = MissionLocatorActivePackages::new_with_initial_dynamic(
            mission_id.to_owned(),
            script_package_roots,
            initial_dynamic_package_roots,
        )?;
        let report = MissionLocatorActivePackageReport::from_missions(vec![active])?;
        if contexts
            .insert(snapshot.source_path.clone(), report)
            .is_some()
        {
            return Err("mission locator context source was rebound".to_owned());
        }
    }
    Ok(MissionLocatorSourceContexts {
        by_source_path: contexts,
    })
}

fn initial_dynamic_package_roots(
    initialization: &MissionInitializationBinding,
    indexed_package_roots: &BTreeSet<String>,
) -> Result<Vec<String>, String> {
    let mut roots = Vec::new();
    for directive in initialization.directives() {
        let (MissionInitializationDirective::DynamicLoad { p3d_files, .. }
        | MissionInitializationDirective::StreetRacePropsLoad { p3d_files, .. }) = directive
        else {
            continue;
        };
        for p3d_file in p3d_files {
            let root = initial_dynamic_package_root(p3d_file)?;
            if !indexed_package_roots.contains(&root) {
                return Err(format!(
                    "mission locator initial dynamic package is not indexed: {root}"
                ));
            }
            roots.push(root);
        }
    }
    Ok(roots)
}

fn initial_dynamic_package_root(p3d_file: &str) -> Result<String, String> {
    let normalized = p3d_file.replace(char::from(92), "/").to_ascii_lowercase();
    let Some(without_extension) = normalized.strip_suffix(".p3d") else {
        return Err("mission locator initial dynamic P3D path is malformed".to_owned());
    };
    if without_extension.is_empty()
        || without_extension.starts_with('/')
        || without_extension.contains(':')
        || without_extension
            .split('/')
            .any(|segment| segment.is_empty() || segment == "." || segment == "..")
    {
        return Err("mission locator initial dynamic P3D path is malformed".to_owned());
    }
    Ok(if without_extension.starts_with("art/") {
        format!("extracted/{without_extension}")
    } else {
        format!("extracted/art/{without_extension}")
    })
}

fn locator_context_paths(
    source_path: &str,
    mission_id: &str,
    available: &BTreeSet<&str>,
) -> Result<(String, String), String> {
    if mission_id.is_empty()
        || mission_id != mission_id.trim()
        || mission_id.contains('/')
        || mission_id.contains(char::from(92))
        || mission_id.chars().any(char::is_control)
    {
        return Err("mission locator selected mission id is malformed".to_owned());
    }
    let relative = source_path
        .strip_prefix(MISSION_ROOT)
        .ok_or_else(|| "mission locator selected source is outside the mission root".to_owned())?;
    let (level_dir, file_name) = relative
        .split_once('/')
        .ok_or_else(|| "mission locator selected source has no level directory".to_owned())?;
    if level_dir.is_empty() || file_name.contains('/') {
        return Err("mission locator selected source path is malformed".to_owned());
    }
    let expected_init = format!("{mission_id}{INIT_SUFFIX}");
    if file_name != expected_init {
        return Err("mission locator selected id does not match its init source path".to_owned());
    }
    let directory = format!("{MISSION_ROOT}{level_dir}/");
    let mission_load_path = format!("{directory}{mission_id}{LOAD_SUFFIX}");
    if !available.contains(mission_load_path.as_str()) {
        return Err("mission locator paired load source is missing".to_owned());
    }

    let mut best_level_load: Option<(&str, usize)> = None;
    for candidate in available {
        let Some(candidate_file) = candidate.strip_prefix(&directory) else {
            continue;
        };
        if candidate_file.contains('/') {
            continue;
        }
        let Some(family_prefix) = candidate_file.strip_suffix(LEVEL_LOAD_SUFFIX) else {
            continue;
        };
        if !mission_id.starts_with(family_prefix) {
            continue;
        }
        let score = family_prefix.len();
        match best_level_load {
            None => best_level_load = Some((candidate, score)),
            Some((_current, current_score)) if score > current_score => {
                best_level_load = Some((candidate, score));
            }
            Some((current, current_score)) if score == current_score && current != *candidate => {
                return Err("mission locator level-load family is ambiguous".to_owned());
            }
            Some(_) => {}
        }
    }
    let (level_load_path, _score) =
        best_level_load.ok_or_else(|| "mission locator level-load family is missing".to_owned())?;
    Ok((level_load_path.to_owned(), mission_load_path))
}

#[cfg(test)]
#[path = "../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/mission_locator_context/tests.rs"]
mod tests;

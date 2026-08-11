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
//   - Static sibling-load context for level-init locator binding.
// - Must-Not:
//   - Read files, resolve locators, or infer runtime package precedence.
// - Allows:
//   - Pair selected mission init/load scripts and their level-load family.
//   - Pair level-init setup scripts with their exact family load sibling.
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
const LEVEL_INIT_SUFFIX: &str = "i.mfk.json";
const LEVEL_SETUP_ANCHOR_COMMANDS: &[&str] = &[
    "addambientcharacter",
    "addambientnpcwaypoint",
    "addbonusmissionnpcwaypoint",
    "addnpccharacterbonusmission",
    "addpurchasecarnpcwaypoint",
    "addpurchasecarreward",
    "setbonusmissiondialoguepos",
];

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

/// Cross-source context for one selected mission init source.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct MissionLocatorSourceContext {
    active_packages: MissionLocatorActivePackageReport,
    level_setup_source_path: Option<String>,
}

impl MissionLocatorSourceContext {
    /// Return one mission's active package evidence.
    #[cfg(test)]
    pub(super) fn mission(
        &self,
        mission_id: &str,
    ) -> Option<&MissionLocatorActivePackages> {
        self.active_packages.mission(mission_id)
    }

    /// Return all active package evidence for locator binding.
    pub(super) const fn active_packages(
        &self,
    ) -> &MissionLocatorActivePackageReport {
        &self.active_packages
    }

    /// Return the paired level setup source when it exists.
    pub(super) fn level_setup_source_path(&self) -> Option<&str> {
        self.level_setup_source_path.as_deref()
    }
}

/// Mission contexts indexed by exact selected-mission source path.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(super) struct MissionLocatorSourceContexts {
    by_source_path: BTreeMap<String, MissionLocatorSourceContext>,
}

impl MissionLocatorSourceContexts {
    /// Return context for one exact selected-mission source.
    pub(super) fn get(
        &self,
        source_path: &str,
    ) -> Option<&MissionLocatorSourceContext> {
        self.by_source_path.get(source_path)
    }

    /// Return the number of selected mission sources with package context.
    #[cfg(test)]
    pub(super) fn len(&self) -> usize {
        self.by_source_path.len()
    }
}

/// Static package roots visible while one level-init setup script executes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct MissionLevelLocatorSourceContext {
    load_source_path: String,
    package_roots: Vec<String>,
}

impl MissionLevelLocatorSourceContext {
    /// Return the exact paired family load source path.
    pub(super) fn load_source_path(&self) -> &str {
        &self.load_source_path
    }

    /// Return static package roots in authored load order.
    pub(super) fn package_roots(&self) -> &[String] {
        &self.package_roots
    }
}

/// Static level-locator contexts indexed by exact setup source path.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(super) struct MissionLevelLocatorSourceContexts {
    by_source_path: BTreeMap<String, MissionLevelLocatorSourceContext>,
}

impl MissionLevelLocatorSourceContexts {
    /// Return static context for one exact level-init setup source.
    pub(super) fn get(
        &self,
        source_path: &str,
    ) -> Option<&MissionLevelLocatorSourceContext> {
        self.by_source_path.get(source_path)
    }

    /// Return the number of level-init setup sources with static context.
    #[cfg(test)]
    pub(super) fn len(&self) -> usize {
        self.by_source_path.len()
    }
}

/// Pair every level-init setup source with its exact family load sibling.
///
/// # Errors
///
/// Fails when source identities collide, a setup source does not follow the
/// `<family>i.mfk.json` convention, or its `<family>.mfk.json` sibling is
/// missing or publishes no P3D package roots.
pub(super) fn build_level_locator_source_contexts(
    snapshots: &[MissionLocatorScriptSnapshot],
) -> Result<MissionLevelLocatorSourceContexts, String> {
    let mut by_path = BTreeMap::new();
    for snapshot in snapshots {
        if by_path
            .insert(snapshot.source_path.as_str(), snapshot)
            .is_some()
        {
            return Err(
                "level locator context source path is duplicated".to_owned(),
            );
        }
    }
    let available = by_path.keys().copied().collect::<BTreeSet<_>>();
    let mut contexts = BTreeMap::new();
    for snapshot in snapshots {
        if !has_level_locator_setup(&snapshot.evidence) {
            continue;
        }
        let load_source_path = level_setup_load_path(
            &snapshot.source_path,
            &available,
        )?;
        let load = by_path
            .get(load_source_path.as_str())
            .ok_or_else(|| {
                "level locator load sibling disappeared".to_owned()
            })?;
        if load.package_roots.is_empty() {
            return Err(
                "level locator load sibling has no P3D packages".to_owned(),
            );
        }
        let context = MissionLevelLocatorSourceContext {
            load_source_path,
            package_roots: load.package_roots.clone(),
        };
        if contexts
            .insert(snapshot.source_path.clone(), context)
            .is_some()
        {
            return Err("level locator setup source was rebound".to_owned());
        }
    }
    Ok(MissionLevelLocatorSourceContexts {
        by_source_path: contexts,
    })
}

fn has_level_locator_setup(evidence: &MissionScriptEvidence) -> bool {
    evidence.invocations().iter().any(|invocation| {
        LEVEL_SETUP_ANCHOR_COMMANDS.contains(&invocation.name())
    })
}

fn level_setup_load_path(
    source_path: &str,
    available: &BTreeSet<&str>,
) -> Result<String, String> {
    let relative = source_path
        .strip_prefix(MISSION_ROOT)
        .ok_or_else(|| {
            "level locator setup source is outside mission root".to_owned()
        })?;
    let (level_dir, file_name) = relative
        .split_once('/')
        .ok_or_else(|| {
            "level locator setup source has no level directory".to_owned()
        })?;
    if level_dir.is_empty() || file_name.contains('/') {
        return Err("level locator setup source path is malformed".to_owned());
    }
    let family = file_name
        .strip_suffix(LEVEL_INIT_SUFFIX)
        .ok_or_else(|| {
            "level locator setup source is not a family init script".to_owned()
        })?;
    if family.is_empty() {
        return Err("level locator setup family is empty".to_owned());
    }
    let load_path = format!("{MISSION_ROOT}{level_dir}/{family}.mfk.json");
    if !available.contains(load_path.as_str()) {
        return Err("level locator family load sibling is missing".to_owned());
    }
    Ok(load_path)
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
        let active_packages =
            MissionLocatorActivePackageReport::from_missions(vec![active])?;
        let level_setup_source_path = level_load_path
            .strip_suffix(".mfk.json")
            .map(|prefix| format!("{prefix}i.mfk.json"))
            .filter(|path| available.contains(path.as_str()));
        let context = MissionLocatorSourceContext {
            active_packages,
            level_setup_source_path,
        };
        if contexts
            .insert(snapshot.source_path.clone(), context)
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

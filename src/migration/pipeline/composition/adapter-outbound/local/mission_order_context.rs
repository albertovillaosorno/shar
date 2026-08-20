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
//   - Cross-script authored AddMission registration order and sibling pairing.
// - Must-Not:
//   - Infer unlock, completion, prerequisite, or progression behavior.
// - Allows:
//   - Preserve source order and validate matching init/load mission siblings.
//   - Require each paired init source to select the declared mission id.
// - Split-When:
//   - Runtime mission progression gains an authoritative independent graph.
// - Merge-When:
//   - Final level mission registry compilation owns this exact ordering.
// - Summary:
//   - Authored cross-script mission registration preflight.
// - Description:
//   - Validates AddMission order without promoting it to progression policy.
// - Usage:
//   - Runs over already-validated mission-script snapshots.
// - Defaults:
//   - Duplicate, missing, malformed, or mismatched siblings fail closed.
//

//! Cross-script authored mission registration order.

use std::collections::BTreeMap;

use super::mission_locator_context::MissionLocatorScriptSnapshot;

const MISSION_ROOT: &str = "extracted/game/scripts/missions/";

/// One authored `AddMission` declaration with exact sibling provenance.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct MissionOrderBinding {
    source_ordinal: usize,
    sequence_ordinal: usize,
    mission_id: String,
    init_source_path: String,
    load_source_path: String,
}

impl MissionOrderBinding {
    /// Return the source `AddMission` ordinal.
    #[cfg(test)]
    pub(super) const fn source_ordinal(&self) -> usize {
        self.source_ordinal
    }

    /// Return the dense registration ordinal in the load source.
    #[cfg(test)]
    pub(super) const fn sequence_ordinal(&self) -> usize {
        self.sequence_ordinal
    }

    /// Return the exact authored mission identity.
    #[cfg(test)]
    pub(super) fn mission_id(&self) -> &str {
        &self.mission_id
    }

    /// Return the exact paired mission init source path.
    #[cfg(test)]
    pub(super) fn init_source_path(&self) -> &str {
        &self.init_source_path
    }

    /// Return the exact paired mission load source path.
    #[cfg(test)]
    pub(super) fn load_source_path(&self) -> &str {
        &self.load_source_path
    }
}

/// One load source's authored mission registration sequence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct MissionOrderSourceReport {
    source_path: String,
    registrations: Vec<MissionOrderBinding>,
}

impl MissionOrderSourceReport {
    /// Return the source path that authored the registration order.
    #[cfg(test)]
    pub(super) fn source_path(&self) -> &str {
        &self.source_path
    }

    /// Return mission registrations in authored order.
    #[cfg(test)]
    pub(super) fn registrations(&self) -> &[MissionOrderBinding] {
        &self.registrations
    }
}

/// Validate authored registrations against exact init/load siblings.
///
/// # Errors
///
/// Fails on duplicate source paths, malformed or duplicate mission ids,
/// missing init/load siblings, or init selection drift.
pub(super) fn build_mission_order_source_reports(
    snapshots: &[MissionLocatorScriptSnapshot],
) -> Result<Vec<MissionOrderSourceReport>, String> {
    let mut by_path = BTreeMap::new();
    for snapshot in snapshots {
        if by_path.insert(snapshot.source_path(), snapshot).is_some() {
            return Err("mission order source path is duplicated".to_owned());
        }
    }

    let mut reports = Vec::new();
    let mut seen_registration_targets = BTreeMap::new();
    for snapshot in snapshots {
        let declarations = snapshot
            .evidence()
            .invocations()
            .iter()
            .filter(|invocation| invocation.name() == "addmission")
            .collect::<Vec<_>>();
        if declarations.is_empty() {
            continue;
        }
        let directory = mission_source_directory(snapshot.source_path())?;
        let mut seen = BTreeMap::new();
        let mut registrations = Vec::with_capacity(declarations.len());
        let mut previous_ordinal = None;
        for (sequence_ordinal, declaration) in
            declarations.into_iter().enumerate()
        {
            let [mission_id] = declaration.arguments() else {
                return Err("AddMission must have one argument".to_owned());
            };
            validate_mission_id(mission_id)?;
            if seen
                .insert(mission_id.as_str(), declaration.ordinal())
                .is_some()
            {
                return Err("mission registration id is duplicated".to_owned());
            }
            if previous_ordinal
                .is_some_and(|ordinal| declaration.ordinal() <= ordinal)
            {
                let message = concat!(
                    "mission registration ordinals are ",
                    "not increasing"
                );
                return Err(message.to_owned());
            }
            previous_ordinal = Some(declaration.ordinal());

            let init_source_path = format!("{directory}{mission_id}i.mfk.json");
            if seen_registration_targets
                .insert(init_source_path.clone(), snapshot.source_path())
                .is_some()
            {
                return Err(
                    "mission registration id is duplicated within level"
                        .to_owned(),
                );
            }
            let load_source_path = format!("{directory}{mission_id}l.mfk.json");
            let init = by_path
                .get(init_source_path.as_str())
                .ok_or_else(|| {
                    "mission registration init sibling is missing".to_owned()
                })?;
            if !by_path.contains_key(load_source_path.as_str()) {
                return Err(
                    "mission registration load sibling is missing".to_owned(),
                );
            }
            validate_selected_mission(init, mission_id)?;
            registrations.push(MissionOrderBinding {
                source_ordinal: declaration.ordinal(),
                sequence_ordinal,
                mission_id: mission_id.clone(),
                init_source_path,
                load_source_path,
            });
        }
        reports.push(MissionOrderSourceReport {
            source_path: snapshot.source_path().to_owned(),
            registrations,
        });
    }
    Ok(reports)
}

fn mission_source_directory(source_path: &str) -> Result<String, String> {
    let relative = source_path
        .strip_prefix(MISSION_ROOT)
        .ok_or_else(|| {
            "mission registration source is outside mission root".to_owned()
        })?;
    let (level, file_name) = relative
        .split_once('/')
        .ok_or_else(|| {
            "mission registration source has no level directory".to_owned()
        })?;
    if level.is_empty() || file_name.is_empty() || file_name.contains('/') {
        return Err("mission registration source path is malformed".to_owned());
    }
    Ok(format!("{MISSION_ROOT}{level}/"))
}

fn validate_mission_id(mission_id: &str) -> Result<(), String> {
    let bytes = mission_id.as_bytes();
    if bytes.is_empty()
        || !bytes.first().is_some_and(u8::is_ascii_alphanumeric)
        || !bytes.last().is_some_and(u8::is_ascii_alphanumeric)
        || !bytes.iter().copied().all(|byte| {
            byte.is_ascii_lowercase()
                || byte.is_ascii_digit()
                || matches!(byte, b'-' | b'_')
        })
    {
        return Err("mission registration id is malformed".to_owned());
    }
    Ok(())
}

fn validate_selected_mission(
    snapshot: &MissionLocatorScriptSnapshot,
    mission_id: &str,
) -> Result<(), String> {
    let selected = snapshot
        .evidence()
        .invocations()
        .iter()
        .filter(|invocation| invocation.name() == "selectmission")
        .collect::<Vec<_>>();
    let [selection] = selected.as_slice() else {
        return Err(
            "mission registration init selection count drifted".to_owned(),
        );
    };
    let [selected_id] = selection.arguments() else {
        return Err("SelectMission must have one argument".to_owned());
    };
    if selected_id != mission_id {
        return Err(
            "mission registration init selects a different id".to_owned(),
        );
    }
    Ok(())
}

#[cfg(test)]
// jig-ignore-next-line: exact Rust test-module path is indivisible.
#[path = "../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/mission_order_context/tests.rs"]
mod tests;

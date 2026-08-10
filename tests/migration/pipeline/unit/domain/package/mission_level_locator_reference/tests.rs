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
//   - Unit evidence for level-init locator lookup constraints.
// - Must-Not:
//   - Add runtime lifecycle policy beyond source-backed lookup timing.
// - Allows:
//   - Verify exact CarStart ordering and conservative generic ambiguity.
// - Split-When:
//   - Integration fixtures require independent setup reports.
// - Merge-When:
//   - The level locator domain boundary disappears.
// - Summary:
//   - Level locator reference unit tests.
// - Description:
//   - Locks exact-vs-generic package lookup behavior.
// - Usage:
//   - Compiled as a child of mission_level_locator_reference.
// - Defaults:
//   - Unsupported precedence remains ambiguous.
//

//! Unit evidence for static level locator reference binding.

use super::*;
use crate::domain::{
    MissionLocatorCatalogEntry, MissionScopeReport,
    compile_mission_scope_graphs, preflight_mission_script,
};

fn entry(
    source_name: &str,
    locator_type: u32,
    package_id: &str,
    package_root: &str,
) -> Result<MissionLocatorCatalogEntry, String> {
    MissionLocatorCatalogEntry::new(
        source_name.to_owned(),
        locator_type,
        format!("type-{locator_type}"),
        format!("{package_id}-locator"),
        package_id.to_owned(),
        package_root.to_owned(),
        format!(
            "{package_root}/components/srr_locator/{package_id}-locator.json"
        ),
    )
}

fn catalog() -> Result<MissionLocatorCatalog, String> {
    MissionLocatorCatalog::from_entries(vec![
        entry("shared", 3, "level-a", "extracted/art/level-a")?,
        entry("shared", 3, "level-b", "extracted/art/level-b")?,
        entry("generic", 3, "level-a", "extracted/art/level-a")?,
        entry("generic", 9, "level-b", "extracted/art/level-b")?,
    ])
}

#[test]
fn exact_dialogue_lookup_uses_authored_package_order() -> Result<(), String> {
    let catalog = catalog()?;
    let mut bindings = Vec::new();
    push_reference(
        &mut bindings,
        &catalog,
        &[
            "extracted/art/level-b".to_owned(),
            "extracted/art/level-a".to_owned(),
        ],
        7,
        MissionLevelLocatorRole::BonusDialoguePlayer,
        "shared",
    )?;
    let [binding] = bindings.as_slice() else {
        return Err("exact level locator binding count changed".to_owned());
    };
    let MissionLocatorResolution::Resolved(resolved) = binding.resolution()
    else {
        return Err("exact CarStart lookup did not resolve".to_owned());
    };
    assert_eq!(resolved.entry().package_id(), "level-b");
    Ok(())
}

#[test]
fn generic_lookup_preserves_cross_package_ambiguity() -> Result<(), String> {
    let catalog = catalog()?;
    let mut bindings = Vec::new();
    push_reference(
        &mut bindings,
        &catalog,
        &[
            "extracted/art/level-a".to_owned(),
            "extracted/art/level-b".to_owned(),
        ],
        8,
        MissionLevelLocatorRole::AmbientSpawn,
        "generic",
    )?;
    let [binding] = bindings.as_slice() else {
        return Err("generic level locator binding count changed".to_owned());
    };
    let MissionLocatorResolution::Ambiguous(candidates) = binding.resolution()
    else {
        return Err("generic locator precedence was inferred".to_owned());
    };
    assert_eq!(candidates.len(), 2);
    Ok(())
}

fn level_vehicle_scopes() -> Result<MissionScopeReport, String> {
    let document = serde_json::json!({
        "schema":"shar-schoenwald.straggler.mission-script.v3",
        "source_extension":"mfk",
        "route_class":"mission",
        "source_bytes":64,
        "context_command_count":0,
        "context_adaptation_count":0,
        "context_adaptations":[],
        "context_finding_count":0,
        "context_findings":[],
        "statement_count":1,
        "unique_command_count":1,
        "load_p3d_reference_count":0,
        "mission_flow_command_count":0,
        "vehicle_physics_command_count":0,
        "semantic_family":"mission-script",
        "command_counts":{"initlevelplayervehicle":1},
        "source_statements":[
            "InitLevelPlayerVehicle(\"famil_v\",\"start\",\"DEFAULT\");"
        ],
        "p3d_references":[],
        "command_invocations":[{
            "ordinal":1,
            "name":"initlevelplayervehicle",
            "args_raw":"\"famil_v\",\"start\",\"DEFAULT\"",
            "semantic_role":"mission-script",
            "arguments":["famil_v","start","DEFAULT"]
        }]
    });
    let evidence = preflight_mission_script(
        &serde_json::to_string(&document).map_err(|error| error.to_string())?,
    )?;
    compile_mission_scope_graphs(&evidence)
}

#[test]
fn level_player_vehicle_uses_exact_car_start_lookup() -> Result<(), String> {
    let catalog = MissionLocatorCatalog::from_entries(vec![
        entry("start", 3, "level-a", "extracted/art/level-a")?,
        entry("start", 3, "level-b", "extracted/art/level-b")?,
    ])?;
    let mut bindings = Vec::new();
    push_level_vehicle_references(
        &mut bindings,
        &catalog,
        &[
            "extracted/art/level-b".to_owned(),
            "extracted/art/level-a".to_owned(),
        ],
        &level_vehicle_scopes()?,
    )?;
    let [binding] = bindings.as_slice() else {
        return Err("level player vehicle binding count changed".to_owned());
    };
    assert_eq!(binding.role(), MissionLevelLocatorRole::LevelPlayerVehicle);
    assert_eq!(binding.source_name(), "start");
    let MissionLocatorResolution::Resolved(resolved) = binding.resolution()
    else {
        return Err("level player vehicle CarStart did not resolve".to_owned());
    };
    assert_eq!(resolved.entry().package_id(), "level-b");
    Ok(())
}

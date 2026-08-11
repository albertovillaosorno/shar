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
//   - Cross-script mission locator context composition unit regressions.
// - Must-Not:
//   - Read installed-game files or infer unsupported inventory precedence.
// - Allows:
//   - Synthetic mission init/load families and package-root evidence.
// - Split-When:
//   - Dynamic package-history fixtures require independent integration tests.
// - Merge-When:
//   - Locator context composition no longer has adapter-specific behavior.
// - Summary:
//   - Mission locator context adapter tests.
// - Description:
//   - Proves exact source pairing, level-family selection, and safe Dyna paths.
// - Usage:
//   - Included only by the local mission locator context adapter in tests.
// - Defaults:
//   - Missing siblings, cross-level ids, and unsafe paths fail closed.
//

//! Unit evidence for mission locator context composition.

use std::collections::BTreeSet;

use serde_json::json;

use super::*;
use crate::domain::preflight_mission_script;

fn mission_evidence(mission_id: Option<&str>) -> Result<MissionScriptEvidence, String> {
    let (invocations, commands, statements, p3d_references, mission_flow_count, context_count) =
        mission_id.map_or_else(
            || {
                (
                    vec![json!({
                        "ordinal":1,"name":"loadp3dfile",
                        "args_raw":"\"art/missions/level01/dummy.p3d\"",
                        "semantic_role":"asset-load",
                        "arguments":["art/missions/level01/dummy.p3d"]
                    })],
                    json!({"loadp3dfile":1}),
                    vec!["LoadP3DFile(\"art/missions/level01/dummy.p3d\");".to_owned()],
                    vec!["art/missions/level01/dummy.p3d".to_owned()],
                    0,
                    0,
                )
            },
            |mission_id| {
                (
                    vec![
                        json!({"ordinal":1,"name":"selectmission","args_raw":format!("\"{mission_id}\""),"semantic_role":"mission-script","arguments":[mission_id]}),
                        json!({"ordinal":2,"name":"addstage","args_raw":"0","semantic_role":"mission-stage","arguments":["0"]}),
                        json!({"ordinal":3,"name":"addobjective","args_raw":"\"dummy\"","semantic_role":"mission-objective","arguments":["dummy"]}),
                        json!({"ordinal":4,"name":"closeobjective","args_raw":"","semantic_role":"mission-objective","arguments":[]}),
                        json!({"ordinal":5,"name":"closestage","args_raw":"","semantic_role":"mission-stage","arguments":[]}),
                        json!({"ordinal":6,"name":"closemission","args_raw":"","semantic_role":"mission-script","arguments":[]}),
                    ],
                    json!({
                        "selectmission":1,"addstage":1,"addobjective":1,
                        "closeobjective":1,"closestage":1,"closemission":1
                    }),
                    vec![
                        format!("SelectMission(\"{mission_id}\");"),
                        "AddStage(0);".to_owned(),
                        "AddObjective(\"dummy\");".to_owned(),
                        "CloseObjective();".to_owned(),
                        "CloseStage();".to_owned(),
                        "CloseMission();".to_owned(),
                    ],
                    Vec::new(),
                    6,
                    6,
                )
            },
        );
    let count = invocations.len();
    let value = json!({
        "schema":"shar-schoenwald.straggler.mission-script.v3",
        "source_extension":"mfk","route_class":"mission","source_bytes":64,
        "context_command_count":context_count,"context_adaptation_count":0,
        "context_adaptations":[],"context_finding_count":0,"context_findings":[],
        "statement_count":count,"unique_command_count":commands.as_object().map_or(0, serde_json::Map::len),
        "load_p3d_reference_count":p3d_references.len(),"mission_flow_command_count":mission_flow_count,
        "vehicle_physics_command_count":0,"semantic_family":"mission-script",
        "command_counts":commands,"source_statements":statements,
        "p3d_references":p3d_references,"command_invocations":invocations
    });
    preflight_mission_script(&serde_json::to_string(&value).map_err(|error| error.to_string())?)
}

fn snapshot(
    path: &str,
    mission_id: Option<&str>,
    roots: &[&str],
) -> Result<MissionLocatorScriptSnapshot, String> {
    Ok(MissionLocatorScriptSnapshot::new(
        path.to_owned(),
        mission_evidence(mission_id)?,
        roots.iter().map(|root| (*root).to_owned()).collect(),
    ))
}

fn dynamic_mission_evidence(
    mission_id: &str,
    source_data: &str,
) -> Result<MissionScriptEvidence, String> {
    let value = json!({
        "schema":"shar-schoenwald.straggler.mission-script.v3",
        "source_extension":"mfk","route_class":"mission","source_bytes":64,
        "context_command_count":6,"context_adaptation_count":0,
        "context_adaptations":[],"context_finding_count":0,
        "context_findings":[],"statement_count":7,"unique_command_count":7,
        "load_p3d_reference_count":0,"mission_flow_command_count":6,
        "vehicle_physics_command_count":0,"semantic_family":"mission-script",
        "command_counts":{
            "selectmission":1,"setdynaloaddata":1,"addstage":1,
            "addobjective":1,"closeobjective":1,"closestage":1,
            "closemission":1
        },
        "source_statements":[
            format!("SelectMission(\"{mission_id}\");"),
            format!("SetDynaLoadData(\"{source_data}\");"),
            "AddStage(0);","AddObjective(\"dummy\");",
            "CloseObjective();","CloseStage();","CloseMission();"
        ],
        "p3d_references":[],
        "command_invocations":[
            {"ordinal":1,"name":"selectmission",
             "args_raw":format!("\"{mission_id}\""),
             "semantic_role":"mission-script","arguments":[mission_id]},
            {"ordinal":2,"name":"setdynaloaddata",
             "args_raw":format!("\"{source_data}\""),
             "semantic_role":"mission-script","arguments":[source_data]},
            {"ordinal":3,"name":"addstage","args_raw":"0",
             "semantic_role":"mission-stage","arguments":["0"]},
            {"ordinal":4,"name":"addobjective","args_raw":"\"dummy\"",
             "semantic_role":"mission-objective","arguments":["dummy"]},
            {"ordinal":5,"name":"closeobjective","args_raw":"",
             "semantic_role":"mission-objective","arguments":[]},
            {"ordinal":6,"name":"closestage","args_raw":"",
             "semantic_role":"mission-stage","arguments":[]},
            {"ordinal":7,"name":"closemission","args_raw":"",
             "semantic_role":"mission-script","arguments":[]}
        ]
    });
    preflight_mission_script(
        &serde_json::to_string(&value).map_err(|error| error.to_string())?,
    )
}

fn dynamic_snapshot(
    path: &str,
    mission_id: &str,
    source_data: &str,
) -> Result<MissionLocatorScriptSnapshot, String> {
    Ok(MissionLocatorScriptSnapshot::new(
        path.to_owned(),
        dynamic_mission_evidence(mission_id, source_data)?,
        Vec::new(),
    ))
}

#[test]
fn chooses_longest_matching_level_load_family() -> Result<(), String> {
    let available = BTreeSet::from([
        "extracted/game/scripts/missions/level02/level.mfk.json",
        "extracted/game/scripts/missions/level02/e3level.mfk.json",
        "extracted/game/scripts/missions/level02/e3m1l.mfk.json",
    ]);
    let (level, load) = locator_context_paths(
        "extracted/game/scripts/missions/level02/e3m1i.mfk.json",
        "e3m1",
        &available,
    )?;
    if level != "extracted/game/scripts/missions/level02/e3level.mfk.json"
        || load != "extracted/game/scripts/missions/level02/e3m1l.mfk.json"
    {
        return Err("special mission family did not select e3 level load".to_owned());
    }
    Ok(())
}

#[test]
fn regular_mission_stays_on_base_level_family() -> Result<(), String> {
    let available = BTreeSet::from([
        "extracted/game/scripts/missions/level02/level.mfk.json",
        "extracted/game/scripts/missions/level02/e3level.mfk.json",
        "extracted/game/scripts/missions/level02/m1l.mfk.json",
    ]);
    let (level, _load) = locator_context_paths(
        "extracted/game/scripts/missions/level02/m1i.mfk.json",
        "m1",
        &available,
    )?;
    if level != "extracted/game/scripts/missions/level02/level.mfk.json" {
        return Err("regular mission escaped base level family".to_owned());
    }
    Ok(())
}

#[test]
fn combines_level_and_mission_load_packages_per_source() -> Result<(), String> {
    let snapshots = vec![
        snapshot(
            "extracted/game/scripts/missions/level01/level.mfk.json",
            None,
            &["extracted/art/missions/level01/level", "extracted/art/l1"],
        )?,
        snapshot(
            "extracted/game/scripts/missions/level01/m1l.mfk.json",
            None,
            &["extracted/art/missions/level01/m1", "extracted/art/l1"],
        )?,
        snapshot(
            "extracted/game/scripts/missions/level01/m1i.mfk.json",
            Some("m1"),
            &[],
        )?,
    ];
    let contexts = build_mission_locator_source_contexts(&snapshots, &BTreeSet::new())?;
    if contexts.len() != 1 {
        return Err("locator context count drifted".to_owned());
    }
    let context = contexts
        .get("extracted/game/scripts/missions/level01/m1i.mfk.json")
        .ok_or_else(|| "m1 locator context is missing".to_owned())?;
    let mission = context
        .mission("m1")
        .ok_or_else(|| "m1 active package report is missing".to_owned())?;
    if mission.package_roots()
        != [
            "extracted/art/missions/level01/level".to_owned(),
            "extracted/art/l1".to_owned(),
            "extracted/art/missions/level01/m1".to_owned(),
        ]
    {
        return Err(format!(
            "active package roots drifted: {:?}",
            mission.package_roots()
        ));
    }
    Ok(())
}

#[test]
fn rejects_selected_id_that_disagrees_with_init_filename() -> Result<(), String> {
    let available = BTreeSet::from([
        "extracted/game/scripts/missions/level01/level.mfk.json",
        "extracted/game/scripts/missions/level01/m1l.mfk.json",
    ]);
    let Err(error) = locator_context_paths(
        "extracted/game/scripts/missions/level01/m2i.mfk.json",
        "m1",
        &available,
    ) else {
        return Err("selected mission/source mismatch did not fail closed".to_owned());
    };
    if !error.contains("does not match") {
        return Err(format!("unexpected mismatch diagnostic: {error}"));
    }
    Ok(())
}

#[test]
fn same_mission_id_in_different_levels_keeps_separate_contexts() -> Result<(), String> {
    let snapshots = vec![
        snapshot(
            "extracted/game/scripts/missions/level01/level.mfk.json",
            None,
            &["extracted/art/missions/level01/level"],
        )?,
        snapshot(
            "extracted/game/scripts/missions/level01/m1l.mfk.json",
            None,
            &["extracted/art/missions/level01/m1"],
        )?,
        snapshot(
            "extracted/game/scripts/missions/level01/m1i.mfk.json",
            Some("m1"),
            &[],
        )?,
        snapshot(
            "extracted/game/scripts/missions/level02/level.mfk.json",
            None,
            &["extracted/art/missions/level02/level"],
        )?,
        snapshot(
            "extracted/game/scripts/missions/level02/m1l.mfk.json",
            None,
            &["extracted/art/missions/level02/m1"],
        )?,
        snapshot(
            "extracted/game/scripts/missions/level02/m1i.mfk.json",
            Some("m1"),
            &[],
        )?,
    ];
    let contexts = build_mission_locator_source_contexts(&snapshots, &BTreeSet::new())?;
    if contexts.len() != 2 {
        return Err("repeated mission id collapsed across levels".to_owned());
    }
    let level01 = contexts
        .get("extracted/game/scripts/missions/level01/m1i.mfk.json")
        .and_then(|report| report.mission("m1"))
        .ok_or_else(|| "level01 m1 context is missing".to_owned())?;
    let level02 = contexts
        .get("extracted/game/scripts/missions/level02/m1i.mfk.json")
        .and_then(|report| report.mission("m1"))
        .ok_or_else(|| "level02 m1 context is missing".to_owned())?;
    if level01
        .package_roots()
        .iter()
        .any(|root| root.contains("level02"))
        || level02
            .package_roots()
            .iter()
            .any(|root| root.contains("level01"))
    {
        return Err("mission package context leaked across levels".to_owned());
    }
    Ok(())
}

#[test]
fn selected_mission_without_load_sibling_fails_closed() -> Result<(), String> {
    let snapshots = vec![
        snapshot(
            "extracted/game/scripts/missions/level01/level.mfk.json",
            None,
            &["extracted/art/missions/level01/level"],
        )?,
        snapshot(
            "extracted/game/scripts/missions/level01/m1i.mfk.json",
            Some("m1"),
            &[],
        )?,
    ];
    let Err(error) = build_mission_locator_source_contexts(&snapshots, &BTreeSet::new()) else {
        return Err("missing mission load sibling did not fail closed".to_owned());
    };
    if !error.contains("paired load source is missing") {
        return Err(format!("unexpected missing-load diagnostic: {error}"));
    }
    Ok(())
}

#[test]
fn unindexed_initial_dynamic_package_stays_explicit() -> Result<(), String> {
    let snapshots = vec![
        snapshot(
            "extracted/game/scripts/missions/level01/level.mfk.json",
            None,
            &["extracted/art/missions/level01/level"],
        )?,
        snapshot(
            "extracted/game/scripts/missions/level01/m1l.mfk.json",
            None,
            &["extracted/art/missions/level01/m1"],
        )?,
        dynamic_snapshot(
            "extracted/game/scripts/missions/level01/m1i.mfk.json",
            "m1",
            "l1z7.p3d;l1missing.p3d",
        )?,
    ];
    let indexed = BTreeSet::from(["extracted/art/l1z7".to_owned()]);
    let contexts = build_mission_locator_source_contexts(&snapshots, &indexed)?;
    let context = contexts
        .get("extracted/game/scripts/missions/level01/m1i.mfk.json")
        .ok_or_else(|| "m1 locator context is missing".to_owned())?;
    let mission = context
        .mission("m1")
        .ok_or_else(|| "m1 active package report is missing".to_owned())?;
    if mission.package_roots()
        != [
            "extracted/art/missions/level01/level".to_owned(),
            "extracted/art/missions/level01/m1".to_owned(),
            "extracted/art/l1z7".to_owned(),
        ]
    {
        return Err(format!(
            "indexed Dyna package visibility drifted: {:?}",
            mission.package_roots()
        ));
    }
    if context.unindexed_initial_dynamic_package_roots()
        != ["extracted/art/l1missing".to_owned()]
    {
        return Err("unindexed Dyna package evidence was lost".to_owned());
    }
    Ok(())
}

#[test]
fn initial_dynamic_p3d_uses_implicit_art_root() -> Result<(), String> {
    if initial_dynamic_package_root("L1Z7.P3D")? != "extracted/art/l1z7" {
        return Err("implicit Dyna Load Data art root drifted".to_owned());
    }
    if initial_dynamic_package_root(r"art\missions\level01\raceprops\sr1.p3d")?
        != "extracted/art/missions/level01/raceprops/sr1"
    {
        return Err("explicit Dyna Load Data art root drifted".to_owned());
    }
    Ok(())
}

#[test]
fn initial_dynamic_p3d_rejects_unsafe_paths() -> Result<(), String> {
    for value in ["../l1z7.p3d", "C:/l1z7.p3d", "/l1z7.p3d", "l1//z7.p3d"] {
        if initial_dynamic_package_root(value).is_ok() {
            return Err(format!("unsafe initial Dyna P3D was accepted: {value}"));
        }
    }
    Ok(())
}

fn level_setup_evidence() -> Result<MissionScriptEvidence, String> {
    let value = json!({
        "schema":"shar-schoenwald.straggler.mission-script.v3",
        "source_extension":"mfk","route_class":"mission","source_bytes":64,
        "context_command_count":0,"context_adaptation_count":0,
        "context_adaptations":[],"context_finding_count":0,
        "context_findings":[],"statement_count":1,"unique_command_count":1,
        "load_p3d_reference_count":0,"mission_flow_command_count":0,
        "vehicle_physics_command_count":0,"semantic_family":"mission-script",
        "command_counts":{"addambientcharacter":1},
        "source_statements":["AddAmbientCharacter(\"apu\",\"apu_loc\");"],
        "p3d_references":[],
        "command_invocations":[{
            "ordinal":1,"name":"addambientcharacter",
            "args_raw":"\"apu\",\"apu_loc\"",
            "semantic_role":"mission-script",
            "arguments":["apu","apu_loc"]
        }]
    });
    preflight_mission_script(
        &serde_json::to_string(&value).map_err(|error| error.to_string())?,
    )
}

fn player_vehicle_only_evidence() -> Result<MissionScriptEvidence, String> {
    let value = json!({
        "schema":"shar-schoenwald.straggler.mission-script.v3",
        "source_extension":"mfk","route_class":"mission","source_bytes":64,
        "context_command_count":0,"context_adaptation_count":0,
        "context_adaptations":[],"context_finding_count":0,
        "context_findings":[],"statement_count":1,"unique_command_count":1,
        "load_p3d_reference_count":0,"mission_flow_command_count":0,
        "vehicle_physics_command_count":0,"semantic_family":"mission-script",
        "command_counts":{"initlevelplayervehicle":1},
        "source_statements":[
            r#"InitLevelPlayerVehicle("famil_v","start","DEFAULT");"#
        ],
        "p3d_references":[],
        "command_invocations":[{
            "ordinal":1,"name":"initlevelplayervehicle",
            "args_raw":r#""famil_v","start","DEFAULT""#,
            "semantic_role":"mission-script",
            "arguments":["famil_v","start","DEFAULT"]
        }]
    });
    preflight_mission_script(
        &serde_json::to_string(&value).map_err(|error| error.to_string())?,
    )
}

fn level_setup_snapshot(
    path: &str,
) -> Result<MissionLocatorScriptSnapshot, String> {
    Ok(MissionLocatorScriptSnapshot::new(
        path.to_owned(),
        level_setup_evidence()?,
        Vec::new(),
    ))
}

#[test]
fn pairs_level_setup_with_exact_family_load_sibling() -> Result<(), String> {
    let snapshots = vec![
        snapshot(
            "extracted/game/scripts/missions/level01/level.mfk.json",
            None,
            &["extracted/art/missions/level01/level"],
        )?,
        level_setup_snapshot(
            "extracted/game/scripts/missions/level01/leveli.mfk.json",
        )?,
        snapshot(
            "extracted/game/scripts/missions/level01/demo.mfk.json",
            None,
            &["extracted/art/missions/level01/demo"],
        )?,
        level_setup_snapshot(
            "extracted/game/scripts/missions/level01/demoi.mfk.json",
        )?,
        snapshot(
            "extracted/game/scripts/missions/level02/e3level.mfk.json",
            None,
            &["extracted/art/missions/level02/e3level"],
        )?,
        level_setup_snapshot(
            "extracted/game/scripts/missions/level02/e3leveli.mfk.json",
        )?,
    ];
    let contexts = build_level_locator_source_contexts(&snapshots)?;
    assert_eq!(contexts.len(), 3);
    for (init, load, root) in [
        ("level01/leveli", "level01/level", "level01/level"),
        ("level01/demoi", "level01/demo", "level01/demo"),
        ("level02/e3leveli", "level02/e3level", "level02/e3level"),
    ] {
        let init = format!("{MISSION_ROOT}{init}.mfk.json");
        let context = contexts
            .get(&init)
            .ok_or_else(|| format!("missing level context for {init}"))?;
        assert_eq!(
            context.load_source_path(),
            format!("{MISSION_ROOT}{load}.mfk.json")
        );
        assert_eq!(
            context.package_roots(),
            [format!("extracted/art/missions/{root}")]
        );
    }
    Ok(())
}

#[test]
fn level_setup_without_family_load_sibling_fails_closed(
) -> Result<(), String> {
    let snapshots = vec![level_setup_snapshot(
        "extracted/game/scripts/missions/level01/demoi.mfk.json",
    )?];
    let Err(error) = build_level_locator_source_contexts(&snapshots) else {
        return Err("missing level setup load sibling was accepted".to_owned());
    };
    if !error.contains("load sibling is missing") {
        return Err(format!("unexpected missing-sibling diagnostic: {error}"));
    }
    Ok(())
}

#[test]
fn selected_mission_remembers_level_setup_sibling() -> Result<(), String> {
    let snapshots = vec![
        snapshot(
            "extracted/game/scripts/missions/level01/level.mfk.json",
            None,
            &["extracted/art/missions/level01/level"],
        )?,
        level_setup_snapshot(
            "extracted/game/scripts/missions/level01/leveli.mfk.json",
        )?,
        snapshot(
            "extracted/game/scripts/missions/level01/m1l.mfk.json",
            None,
            &["extracted/art/missions/level01/m1"],
        )?,
        snapshot(
            "extracted/game/scripts/missions/level01/m1i.mfk.json",
            Some("m1"),
            &[],
        )?,
    ];
    let contexts = build_mission_locator_source_contexts(
        &snapshots,
        &BTreeSet::new(),
    )?;
    let context = contexts
        .get("extracted/game/scripts/missions/level01/m1i.mfk.json")
        .ok_or_else(|| "m1 context is missing".to_owned())?;
    assert_eq!(
        context.level_setup_source_path(),
        Some("extracted/game/scripts/missions/level01/leveli.mfk.json")
    );
    Ok(())
}

#[test]
fn player_vehicle_only_source_is_not_level_setup() -> Result<(), String> {
    let snapshots = vec![MissionLocatorScriptSnapshot::new(
        "extracted/game/scripts/missions/level01/m6i.mfk.json".to_owned(),
        player_vehicle_only_evidence()?,
        Vec::new(),
    )];
    let contexts = build_level_locator_source_contexts(&snapshots)?;
    assert_eq!(contexts.len(), 0);
    Ok(())
}

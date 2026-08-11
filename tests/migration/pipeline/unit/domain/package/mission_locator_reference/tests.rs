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
//   - Unit evidence for typed mission locator package-context resolution.
// - Must-Not:
//   - Invent package precedence, locator types, or navigation behavior.
// - Allows:
//   - Verify scope ownership and resolved/missing/ambiguous outcomes.
// - Split-When:
//   - Runtime locator lookup gains independent authoritative fixtures.
// - Merge-When:
//   - Locator-reference domain tests own this exact lookup contract.
// - Summary:
//   - Mission locator-reference unit tests.
// - Description:
//   - Locks package visibility, type constraints, and owner provenance.
// - Usage:
//   - Compiled with the mission locator-reference domain module.
// - Defaults:
//   - Missing and ambiguous evidence remains explicit and fail-closed by
//     caller.
//

//! Unit evidence for typed mission locator-reference binding.

use serde_json::json;

use super::*;
use crate::domain::MissionLocatorCatalogEntry;

fn entry(name: &str, package: &str) -> Result<MissionLocatorCatalogEntry, String> {
    let root = format!("extracted/art/missions/level01/{package}");
    MissionLocatorCatalogEntry::new(
        name.to_owned(),
        CAR_START_LOCATOR_TYPE,
        "car_start".to_owned(),
        format!("locator-{package}-{name}"),
        format!("package-{package}"),
        root.clone(),
        format!("{root}/components/srr_locator/{name}.json"),
    )
}

fn mission_json() -> Result<String, String> {
    let value = json!({
        "schema": "shar-schoenwald.straggler.mission-script.v3",
        "source_extension": "mfk",
        "route_class": "mission",
        "source_bytes": 512,
        "context_command_count": 6,
        "context_adaptation_count": 0,
        "context_adaptations": [],
        "context_finding_count": 0,
        "context_findings": [],
        "statement_count": 10,
        "unique_command_count": 10,
        "load_p3d_reference_count": 0,
        "mission_flow_command_count": 7,
        "vehicle_physics_command_count": 0,
        "semantic_family": "mission-script",
        "command_counts": {
            "selectmission": 1,
            "initlevelplayervehicle": 1,
            "addstage": 1,
            "addstagevehicle": 1,
            "addobjective": 1,
            "addnpc": 1,
            "setobjtargetvehicle": 1,
            "closeobjective": 1,
            "closestage": 1,
            "closemission": 1
        },
        "source_statements": [
            "SelectMission(\"m1\");",
            "InitLevelPlayerVehicle(\"cletu_v\",\"start\",\"OTHER\");",
            "AddStage(\"locked\",\"car\",\"cletu_v\");",
            r#"AddStageVehicle("cletu_v","carstart","chase","scripts\cars\cletu_v.con","none");"#,
            "AddObjective(\"getin\",\"cletu_v\");",
            "AddNPC(\"brn_unf\",\"npc_loc\");",
            "SetObjTargetVehicle(\"cletu_v\");",
            "CloseObjective();","CloseStage();","CloseMission();"
        ],
        "p3d_references": [],
        "command_invocations": [
            {"ordinal":1,"name":"selectmission","args_raw":"\"m1\"","semantic_role":"mission-script","arguments":["m1"]},
            {"ordinal":2,"name":"initlevelplayervehicle","args_raw":"\"cletu_v\",\"start\",\"OTHER\"","semantic_role":"mission-script","arguments":["cletu_v","start","OTHER"]},
            {"ordinal":3,"name":"addstage","args_raw":"\"locked\",\"car\",\"cletu_v\"","semantic_role":"mission-stage","arguments":["locked","car","cletu_v"]},
            {
                "ordinal":4,"name":"addstagevehicle",
                "args_raw":r#""cletu_v","carstart","chase","scripts\cars\cletu_v.con","none""#,
                "semantic_role":"mission-stage",
                "arguments":["cletu_v","carstart","chase",r"scripts\cars\cletu_v.con","none"]
            },
            {"ordinal":5,"name":"addobjective","args_raw":"\"getin\",\"cletu_v\"","semantic_role":"mission-objective","arguments":["getin","cletu_v"]},
            {"ordinal":6,"name":"addnpc","args_raw":"\"brn_unf\",\"npc_loc\"","semantic_role":"mission-script","arguments":["brn_unf","npc_loc"]},
            {"ordinal":7,"name":"setobjtargetvehicle","args_raw":"\"cletu_v\"","semantic_role":"mission-script","arguments":["cletu_v"]},
            {"ordinal":8,"name":"closeobjective","args_raw":"","semantic_role":"mission-objective","arguments":[]},
            {"ordinal":9,"name":"closestage","args_raw":"","semantic_role":"mission-stage","arguments":[]},
            {"ordinal":10,"name":"closemission","args_raw":"","semantic_role":"mission-script","arguments":[]}
        ]
    });
    serde_json::to_string(&value).map_err(|error| error.to_string())
}

fn reports() -> Result<
    (
        MissionScopeReport,
        MissionInitializationReport,
        MissionStageSemanticReport,
        MissionObjectiveSemanticReport,
    ),
    String,
> {
    let evidence = crate::domain::preflight_mission_script(&mission_json()?)?;
    let scopes = crate::domain::compile_mission_scope_graphs(&evidence)?;
    let initialization = crate::domain::preflight_mission_initialization(&scopes)?;
    let stages = crate::domain::preflight_mission_stage_semantics(&scopes)?;
    let objectives = crate::domain::preflight_mission_objective_semantics(&scopes)?;
    Ok((scopes, initialization, stages, objectives))
}

fn active(roots: &[&str]) -> Result<MissionLocatorActivePackageReport, String> {
    MissionLocatorActivePackageReport::from_missions(vec![MissionLocatorActivePackages::new(
        "m1".to_owned(),
        roots.iter().map(|root| (*root).to_owned()).collect(),
    )?])
}

#[test]
fn resolves_car_start_roles_inside_active_package() -> Result<(), String> {
    let (scopes, initialization, stages, objectives) = reports()?;
    let catalog = MissionLocatorCatalog::from_entries(vec![
        entry("start", "m1")?,
        entry("carstart", "m1")?,
        entry("npc_loc", "m1")?,
    ])?;
    let active = active(&["extracted/art/missions/level01/m1"])?;
    let report = preflight_mission_locator_references(
        &catalog,
        &active,
        &scopes,
        &initialization,
        &stages,
        &objectives,
    )?;
    let [mission] = report.missions() else {
        return Err("locator fixture changed mission count".to_owned());
    };
    if mission.mission_id() != "m1"
        || mission.references().len() != 3
        || !report.has_only_resolved_references()
    {
        return Err("locator resolution envelope drifted".to_owned());
    }
    let roles = mission
        .references()
        .iter()
        .map(MissionLocatorReferenceBinding::role)
        .collect::<Vec<_>>();
    if roles
        != [
            MissionLocatorRole::InitializationPlayerVehicle,
            MissionLocatorRole::StageVehicle,
            MissionLocatorRole::ObjectiveNpc,
        ]
    {
        return Err(format!("locator roles drifted: {roles:?}"));
    }
    let [initialization, stage, objective] = mission.references() else {
        return Err("locator reference ownership count drifted".to_owned());
    };
    assert_eq!(initialization.owner_stage_source_ordinal(), None);
    assert_eq!(initialization.owner_stage_sequence_ordinal(), None);
    assert_eq!(initialization.owner_objective_source_ordinal(), None);
    assert_eq!(stage.owner_stage_source_ordinal(), Some(3));
    assert_eq!(stage.owner_stage_sequence_ordinal(), Some(0));
    assert_eq!(stage.owner_objective_source_ordinal(), None);
    assert_eq!(objective.owner_stage_source_ordinal(), Some(3));
    assert_eq!(objective.owner_stage_sequence_ordinal(), Some(0));
    assert_eq!(objective.owner_objective_source_ordinal(), Some(5));

    if mission.references().iter().any(|binding| {
        binding.type_constraint() != MissionLocatorTypeConstraint::Exact(CAR_START_LOCATOR_TYPE)
    }) {
        return Err("documented CarStart constraints drifted".to_owned());
    }
    Ok(())
}

#[test]
fn post_dyna_exact_lookup_keeps_history_dependent_ambiguity() -> Result<(), String> {
    let catalog = MissionLocatorCatalog::from_entries(vec![
        entry("walk_start", "m1")?,
        entry("walk_start", "dyna")?,
    ])?;
    let active = MissionLocatorActivePackages::new_with_initial_dynamic(
        "m1".to_owned(),
        vec!["extracted/art/missions/level01/m1".to_owned()],
        vec!["extracted/art/missions/level01/dyna".to_owned()],
    )?;
    let mut references = Vec::new();
    push_locator(
        &catalog,
        &active,
        &mut references,
        6,
        MissionLocatorRole::InitializationWalk,
        "walk_start",
        MissionLocatorTypeConstraint::Exact(CAR_START_LOCATOR_TYPE),
    )?;
    if !matches!(
        references[0].resolution(),
        MissionLocatorResolution::Ambiguous(entries) if entries.len() == 2
    ) {
        return Err("post-Dyna exact lookup invented stable precedence".to_owned());
    }
    Ok(())
}

#[test]
fn camera_best_side_uses_verified_package_lookup_order() -> Result<(), String> {
    let name = "bm1_bestside";
    let catalog = MissionLocatorCatalog::from_entries(vec![
        entry(name, "bm1")?,
        entry(name, "level")?,
    ])?;
    let active = MissionLocatorActivePackages::new(
        "bm1".to_owned(),
        vec![
            "extracted/art/missions/level01/level".to_owned(),
            "extracted/art/missions/level01/bm1".to_owned(),
        ],
    )?;
    let mut references = Vec::new();
    push_locator(
        &catalog,
        &active,
        &mut references,
        17,
        MissionLocatorRole::ObjectiveCameraBestSide,
        name,
        MissionLocatorTypeConstraint::Any,
    )?;
    let [binding] = references.as_slice() else {
        return Err("best-side locator binding count drifted".to_owned());
    };
    let MissionLocatorResolution::Resolved(reference) = binding.resolution()
    else {
        return Err("best-side locator did not honor package order".to_owned());
    };
    assert_eq!(
        reference.entry().package_root(),
        "extracted/art/missions/level01/level"
    );
    Ok(())
}

#[test]
fn exact_script_lookup_uses_static_load_precedence() -> Result<(), String> {
    let catalog = MissionLocatorCatalog::from_entries(vec![
        entry("shared_start", "level")?,
        entry("shared_start", "m1")?,
    ])?;
    let active = MissionLocatorActivePackages::new(
        "m1".to_owned(),
        vec![
            "extracted/art/missions/level01/level".to_owned(),
            "extracted/art/missions/level01/m1".to_owned(),
        ],
    )?;
    let mut references = Vec::new();
    push_locator(
        &catalog,
        &active,
        &mut references,
        4,
        MissionLocatorRole::InitializationResetVehicle,
        "shared_start",
        MissionLocatorTypeConstraint::Exact(CAR_START_LOCATOR_TYPE),
    )?;
    let MissionLocatorResolution::Resolved(reference) =
        references[0].resolution()
    else {
        return Err("exact script locator did not use load precedence".to_owned());
    };
    if reference.entry().package_root()
        != "extracted/art/missions/level01/level"
    {
        return Err("exact script locator did not choose first static load".to_owned());
    }

    references.clear();
    push_locator(
        &catalog,
        &active,
        &mut references,
        5,
        MissionLocatorRole::StageSafeZone,
        "shared_start",
        MissionLocatorTypeConstraint::Any,
    )?;
    if !matches!(
        references[0].resolution(),
        MissionLocatorResolution::Ambiguous(entries) if entries.len() == 2
    ) {
        return Err("generic locator lookup invented subtype precedence".to_owned());
    }
    Ok(())
}

#[test]
fn separates_script_and_post_dyna_locator_visibility() -> Result<(), String> {
    let static_root = "extracted/art/missions/level01/level";
    let dyna_root = "extracted/art/missions/level01/dyna";
    let active = MissionLocatorActivePackages::new_with_initial_dynamic(
        "m1".to_owned(),
        vec![static_root.to_owned()],
        vec![dyna_root.to_owned()],
    )?;
    if active.script_package_roots() != [static_root.to_owned()]
        || active.package_roots()
            != [static_root.to_owned(), dyna_root.to_owned()]
    {
        return Err("locator visibility phases drifted".to_owned());
    }

    let catalog = MissionLocatorCatalog::from_entries(vec![entry("late", "dyna")?])?;
    let mut references = Vec::new();
    push_locator(
        &catalog,
        &active,
        &mut references,
        7,
        MissionLocatorRole::StageWaypoint,
        "late",
        MissionLocatorTypeConstraint::Any,
    )?;
    push_locator(
        &catalog,
        &active,
        &mut references,
        8,
        MissionLocatorRole::InitializationWalk,
        "late",
        MissionLocatorTypeConstraint::Any,
    )?;
    if !matches!(
        references[0].resolution(),
        MissionLocatorResolution::Missing
    ) {
        return Err("script-time locator saw post-script Dyna package".to_owned());
    }
    let MissionLocatorResolution::Resolved(reference) = references[1].resolution()
    else {
        return Err("post-Dyna locator did not see initial Dyna package".to_owned());
    };
    if reference.entry().package_root() != dyna_root {
        return Err("post-Dyna locator resolved the wrong package".to_owned());
    }
    Ok(())
}

#[test]
fn rejects_missing_active_mission_context() -> Result<(), String> {
    let (scopes, initialization, stages, objectives) = reports()?;
    let catalog = MissionLocatorCatalog::from_entries(vec![
        entry("start", "m1")?,
        entry("carstart", "m1")?,
        entry("npc_loc", "m1")?,
    ])?;
    let active = MissionLocatorActivePackageReport::from_missions(Vec::new())?;
    let result = preflight_mission_locator_references(
        &catalog,
        &active,
        &scopes,
        &initialization,
        &stages,
        &objectives,
    );
    if !matches!(result, Err(message) if message.contains("context is missing")) {
        return Err("missing mission locator context did not fail closed".to_owned());
    }
    Ok(())
}

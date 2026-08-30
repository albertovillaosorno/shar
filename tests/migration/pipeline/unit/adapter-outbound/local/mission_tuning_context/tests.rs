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
//   - Synthetic regressions for mission-local tuning evidence joins.
// - Must-Not:
//   - Read game files or assign gameplay meaning to opaque tuning values.
// - Allows:
//   - Exercise normalized mission preflights and deterministic JSONL output.
// - Split-When:
//   - One tuning family gains independent publication semantics.
// - Merge-When:
//   - Another regression suite owns the identical composition boundary.
// - Summary:
//   - Mission-local tuning composition regressions.
// - Description:
//   - Pins source ownership, exact arguments, and physical vehicle provenance.
// - Usage:
//   - Compiled only with the owning adapter module under cfg(test).
// - Defaults:
//   - Drifted semantic joins fail closed.
//

//! Synthetic mission-local tuning composition regressions.

use serde_json::{Value, json};

use super::*;
use crate::domain::{
    compile_mission_scope_graphs, preflight_mission_condition_semantics,
    preflight_mission_initialization, preflight_mission_objective_semantics,
    preflight_mission_references, preflight_mission_stage_semantics,
    preflight_mission_vehicle_attributes,
};
use crate::preflight_mission_script;

fn catalog() -> crate::domain::MissionReferenceCatalog {
    crate::domain::MissionReferenceCatalog::from_vehicle_entries_for_tests(&[
        ("famil_v", "vehicle-famil", "cars/character-rigs/famil-v"),
        ("smith_v", "vehicle-smith", "cars/character-rigs/smith-v"),
        ("cVan", "vehicle-cvan", "cars/traffic/cvan"),
    ])
}

fn invocation(
    ordinal: usize,
    name: &str,
    args_raw: &str,
    role: &str,
    arguments: &[&str],
) -> Value {
    json!({
        "ordinal": ordinal,
        "name": name,
        "args_raw": args_raw,
        "semantic_role": role,
        "arguments": arguments,
    })
}

fn document() -> Result<String, String> {
    let statements = [
        "SetCarAttributes(\"famil_v\",1,1.5,2.50,4);",
        "SelectMission(\"m1\");",
        "AddStage();",
        "SetVehicleAIParams(\"smith_v\",-10,-9);",
        "SetStageAIRaceCatchupParams(\"smith_v\",80,0.5,1.0,1.50);",
        "AddObjective(\"dump\");",
        "SetStageAITargetCatchupParams(\"cVan\",20,70);",
        "CloseObjective();",
        "CloseStage();",
        "CloseMission();",
    ];
    let mut counts = serde_json::Map::new();
    for command in [
        "setcarattributes",
        "selectmission",
        "addstage",
        "setvehicleaiparams",
        "setstageairacecatchupparams",
        "addobjective",
        "setstageaitargetcatchupparams",
        "closeobjective",
        "closestage",
        "closemission",
    ] {
        drop(counts.insert(command.to_owned(), json!(1)));
    }
    serde_json::to_string(&json!({
        "schema": "shar-schoenwald.straggler.mission-script.v3",
        "source_extension": "mfk",
        "route_class": "mission",
        "source_bytes": 320,
        "context_command_count": 6,
        "context_adaptation_count": 0,
        "context_adaptations": [],
        "context_finding_count": 0,
        "context_findings": [],
        "statement_count": statements.len(),
        "unique_command_count": counts.len(),
        "load_p3d_reference_count": 0,
        "mission_flow_command_count": 8,
        "vehicle_physics_command_count": 0,
        "semantic_family": "mission-script",
        "command_counts": counts,
        "source_statements": statements,
        "p3d_references": [],
        "command_invocations": [
            invocation(
                1,
                "setcarattributes",
                "\"famil_v\",1,1.5,2.50,4",
                "mission-script",
                &["famil_v", "1", "1.5", "2.50", "4"],
            ),
            invocation(2,"selectmission","\"m1\"","mission-script", &["m1"]),
            invocation(3,"addstage","","mission-stage", &[]),
            invocation(
                4,
                "setvehicleaiparams",
                "\"smith_v\",-10,-9",
                "mission-script",
                &["smith_v", "-10", "-9"],
            ),
            invocation(
                5,
                "setstageairacecatchupparams",
                "\"smith_v\",80,0.5,1.0,1.50",
                "mission-stage",
                &["smith_v", "80", "0.5", "1.0", "1.50"],
            ),
            invocation(
                6,
                "addobjective",
                "\"dump\"",
                "mission-objective",
                &["dump"],
            ),
            invocation(
                7,
                "setstageaitargetcatchupparams",
                "\"cVan\",20,70",
                "mission-stage",
                &["cVan", "20", "70"],
            ),
            invocation(8,"closeobjective","","mission-objective", &[]),
            invocation(9,"closestage","","mission-stage", &[]),
            invocation(10,"closemission","","mission-script", &[]),
        ],
    }))
    .map_err(|error| error.to_string())
}

fn render() -> Result<String, String> {
    let evidence = preflight_mission_script(&document()?)?;
    let scopes = compile_mission_scope_graphs(&evidence)?;
    let objectives = preflight_mission_objective_semantics(&scopes)?;
    let conditions = preflight_mission_condition_semantics(&scopes)?;
    let initialization = preflight_mission_initialization(&scopes)?;
    let stages = preflight_mission_stage_semantics(&scopes)?;
    let catalog = catalog();
    let references = preflight_mission_references(
        &catalog,
        &scopes,
        &objectives,
        &conditions,
        &initialization,
        &stages,
    )?;
    let attributes = preflight_mission_vehicle_attributes(&catalog, &scopes)?;
    render_mission_tuning(
        "script-abc123",
        &scopes,
        &stages,
        &references,
        &attributes,
    )
    .map_err(|error| error.to_string())
}

#[test]
fn renders_unscoped_stage_and_objective_tuning_in_source_order()
-> Result<(), String> {
    let output = render()?;
    let rows = output
        .lines()
        .map(|line| {
            serde_json::from_str::<Value>(line)
                .map_err(|error| error.to_string())
        })
        .collect::<Result<Vec<_>, _>>()?;
    let [unscoped, stage, race, objective] = rows.as_slice() else {
        return Err(format!("mission tuning row count drifted: {}", rows.len()));
    };
    assert_eq!(
        unscoped,
        &json!({
            "arguments": ["famil_v", "1", "1.5", "2.50", "4"],
            "command": "setcarattributes",
            "mission_source_id": "script-abc123",
            "owner_mission_id": null,
            "owner_objective_source_ordinal": null,
            "owner_stage_sequence_ordinal": null,
            "owner_stage_source_ordinal": null,
            "schema": MISSION_TUNING_SCHEMA,
            "scope": "unscoped",
            "source_ordinal": 1,
            "vehicle": {
                "package_id": "vehicle-famil",
                "package_subcategory": "cars/character-rigs/famil-v",
                "source_id": "famil_v",
            },
            "vehicle_id": "famil_v",
        })
    );
    assert_eq!(
        stage,
        &json!({
            "arguments": ["smith_v", "-10", "-9"],
            "command": "setvehicleaiparams",
            "mission_source_id": "script-abc123",
            "owner_mission_id": "m1",
            "owner_objective_source_ordinal": null,
            "owner_stage_sequence_ordinal": 0,
            "owner_stage_source_ordinal": 3,
            "schema": MISSION_TUNING_SCHEMA,
            "scope": "stage",
            "source_ordinal": 4,
            "vehicle": {
                "package_id": "vehicle-smith",
                "package_subcategory": "cars/character-rigs/smith-v",
                "source_id": "smith_v",
            },
            "vehicle_id": "smith_v",
        })
    );
    assert_eq!(
        race,
        &json!({
            "arguments": ["smith_v", "80", "0.5", "1.0", "1.50"],
            "command": "setstageairacecatchupparams",
            "mission_source_id": "script-abc123",
            "owner_mission_id": "m1",
            "owner_objective_source_ordinal": null,
            "owner_stage_sequence_ordinal": 0,
            "owner_stage_source_ordinal": 3,
            "schema": MISSION_TUNING_SCHEMA,
            "scope": "stage",
            "source_ordinal": 5,
            "vehicle": {
                "package_id": "vehicle-smith",
                "package_subcategory": "cars/character-rigs/smith-v",
                "source_id": "smith_v",
            },
            "vehicle_id": "smith_v",
        })
    );
    assert_eq!(
        objective,
        &json!({
            "arguments": ["cVan", "20", "70"],
            "command": "setstageaitargetcatchupparams",
            "mission_source_id": "script-abc123",
            "owner_mission_id": "m1",
            "owner_objective_source_ordinal": 6,
            "owner_stage_sequence_ordinal": 0,
            "owner_stage_source_ordinal": 3,
            "schema": MISSION_TUNING_SCHEMA,
            "scope": "objective",
            "source_ordinal": 7,
            "vehicle": {
                "package_id": "vehicle-cvan",
                "package_subcategory": "cars/traffic/cvan",
                "source_id": "cVan",
            },
            "vehicle_id": "cVan",
        })
    );
    Ok(())
}

#[test]
fn rejects_noncanonical_source_identity() -> Result<(), String> {
    let evidence = preflight_mission_script(&document()?)?;
    let scopes = compile_mission_scope_graphs(&evidence)?;
    let stages = preflight_mission_stage_semantics(&scopes)?;
    let objectives = preflight_mission_objective_semantics(&scopes)?;
    let conditions = preflight_mission_condition_semantics(&scopes)?;
    let initialization = preflight_mission_initialization(&scopes)?;
    let catalog = catalog();
    let references = preflight_mission_references(
        &catalog,
        &scopes,
        &objectives,
        &conditions,
        &initialization,
        &stages,
    )?;
    let attributes = preflight_mission_vehicle_attributes(&catalog, &scopes)?;
    if render_mission_tuning(
        "Script/ABC",
        &scopes,
        &stages,
        &references,
        &attributes,
    )
    .is_ok()
    {
        return Err(
            "noncanonical mission tuning source id was accepted".to_owned()
        );
    }
    Ok(())
}

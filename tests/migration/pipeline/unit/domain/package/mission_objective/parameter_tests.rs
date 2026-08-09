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
//   - Typed AddObjective parameter compiler regressions.
// - Must-Not:
//   - Compile objective-scoped commands or Unreal mission assets.
// - Allows:
//   - Exercise reviewed route, wager, vehicle, and costume parameter shapes.
// - Split-When:
//   - Repository-corpus coverage requires an independent integration fixture.
// - Merge-When:
//   - Another test module owns this exact parameter boundary.
// - Summary:
//   - Mission objective parameter tests.
// - Description:
//   - Proves reviewed positional values become typed evidence fail closed.
// - Usage:
//   - Included by the objective parameter module under cfg(test).
// - Defaults:
//   - Unknown route tokens and malformed references fail.
//

//! Typed `AddObjective` parameter compiler tests.

use serde_json::json;

use super::{
    MissionObjectiveParameters, MissionRoadArrowBinding, MissionRoadArrowMode,
    preflight_mission_objective_parameters,
};
use crate::domain::preflight_mission_script;

fn objective_document(arguments: &[&str]) -> Result<String, String> {
    let args_raw = arguments
        .iter()
        .map(|value| format!("\"{value}\""))
        .collect::<Vec<_>>()
        .join(",");
    let mut counts = serde_json::Map::new();
    for command in [
        "selectmission",
        "addstage",
        "addobjective",
        "closeobjective",
        "closestage",
        "closemission",
    ] {
        drop(counts.insert(command.to_owned(), json!(1)));
    }
    serde_json::to_string(&json!({
        "schema":"shar-schoenwald.straggler.mission-script.v3",
        "source_extension":"mfk","route_class":"mission","source_bytes":96,
        "context_command_count":6,"context_adaptation_count":0,
        "context_adaptations":[],"context_finding_count":0,"context_findings":[],
        "statement_count":6,"unique_command_count":6,"load_p3d_reference_count":0,
        "mission_flow_command_count":6,"vehicle_physics_command_count":0,
        "semantic_family":"mission-script","command_counts":counts,
        "source_statements":[
            "SelectMission(\"m1\");","AddStage(0);",
            format!("AddObjective({args_raw});"),"CloseObjective();",
            "CloseStage();","CloseMission();"
        ],
        "p3d_references":[],
        "command_invocations":[
            {"ordinal":1,"name":"selectmission","args_raw":"\"m1\"","semantic_role":"mission-script","arguments":["m1"]},
            {"ordinal":2,"name":"addstage","args_raw":"0","semantic_role":"mission-stage","arguments":["0"]},
            {"ordinal":3,"name":"addobjective","args_raw":args_raw,"semantic_role":"mission-objective","arguments":arguments},
            {"ordinal":4,"name":"closeobjective","args_raw":"","semantic_role":"mission-objective","arguments":[]},
            {"ordinal":5,"name":"closestage","args_raw":"","semantic_role":"mission-stage","arguments":[]},
            {"ordinal":6,"name":"closemission","args_raw":"","semantic_role":"mission-script","arguments":[]}
        ]
    }))
    .map_err(|error| error.to_string())
}

fn parameters(
    arguments: &[&str],
) -> Result<MissionObjectiveParameters, String> {
    let evidence = preflight_mission_script(&objective_document(arguments)?)?;
    let report = preflight_mission_objective_parameters(&evidence)?;
    let [binding] = report.objectives() else {
        return Err("objective parameter fixture count changed".to_owned());
    };
    Ok(binding.parameters().clone())
}

#[test]
fn types_purchase_and_forced_vehicle_references() -> Result<(), String> {
    if parameters(&["buycar", "plowk_v"])?
        != (MissionObjectiveParameters::BuyVehicle {
            vehicle_id: "plowk_v".to_owned(),
        })
        || parameters(&["buyskin", "l_cool"])?
            != (MissionObjectiveParameters::BuyCostume {
                costume_id: "l_cool".to_owned(),
            })
        || parameters(&["getin", "skinn_v"])?
            != (MissionObjectiveParameters::EnterVehicle {
                vehicle_id: "skinn_v".to_owned(),
            })
    {
        return Err("typed purchase or get-in reference changed".to_owned());
    }
    Ok(())
}

#[test]
fn distinguishes_get_in_route_from_vehicle_reference() -> Result<(), String> {
    let expected = MissionObjectiveParameters::RoadArrows(
        MissionRoadArrowBinding::Effective(MissionRoadArrowMode::Neither),
    );
    if parameters(&["getin", "neither"])? != expected {
        return Err("get-in road-arrow parameter changed".to_owned());
    }
    Ok(())
}

#[test]
fn types_race_gamble_and_route_independently() -> Result<(), String> {
    let expected = MissionObjectiveParameters::Race {
        gamble: true,
        road_arrows: Some(MissionRoadArrowBinding::Effective(
            MissionRoadArrowMode::Intersection,
        )),
    };
    if parameters(&["race", "gamble", "intersection"])? != expected {
        return Err("race gamble/route parameter mapping changed".to_owned());
    }
    Ok(())
}

#[test]
fn preserves_reviewed_misspelled_route_as_unrecognized() -> Result<(), String> {
    let result = parameters(&["goto", "niether"])?;
    let MissionObjectiveParameters::RoadArrows(binding) = result else {
        return Err("legacy route typo stopped being route evidence".to_owned());
    };
    if binding.effective_mode().is_some()
        || binding.legacy_unrecognized_token() != Some("niether")
        || binding.legacy_unrecognized_code()
            != Some("legacy-road-arrow-token-niether-unrecognized-v1")
    {
        return Err("legacy route typo mapping changed".to_owned());
    }
    Ok(())
}

#[test]
fn rejects_unreviewed_route_and_reference_shapes() -> Result<(), String> {
    for arguments in [
        vec!["goto", "sideways"],
        vec!["race", "gamble", "niether"],
        vec!["buycar", "../../car"],
        vec!["buycar", "famil"],
        vec!["getin", "not-a-reviewed-vehicle"],
    ] {
        let evidence =
            preflight_mission_script(&objective_document(&arguments)?)?;
        if preflight_mission_objective_parameters(&evidence).is_ok() {
            return Err(format!(
                "unreviewed objective parameter was accepted: {arguments:?}"
            ));
        }
    }
    Ok(())
}

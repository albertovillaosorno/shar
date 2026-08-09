// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT

//! Typed direct `AddCondition` parameter regressions.

use serde_json::json;

use super::{
    MissionConditionParameters, preflight_mission_condition_parameters,
};
use crate::domain::preflight_mission_script;

fn condition_document(arguments: &[&str]) -> Result<String, String> {
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
        "addcondition",
        "closecondition",
        "closestage",
        "closemission",
    ] {
        drop(counts.insert(command.to_owned(), json!(1)));
    }
    serde_json::to_string(&json!({
        "schema":"shar-schoenwald.straggler.mission-script.v3",
        "source_extension":"mfk","route_class":"mission","source_bytes":96,
        "context_command_count":8,"context_adaptation_count":0,
        "context_adaptations":[],"context_finding_count":0,"context_findings":[],
        "statement_count":8,"unique_command_count":8,"load_p3d_reference_count":0,
        "mission_flow_command_count":6,"vehicle_physics_command_count":0,
        "semantic_family":"mission-script","command_counts":counts,
        "source_statements":[
            "SelectMission(\"m1\");","AddStage();","AddObjective(\"dummy\");",
            "CloseObjective();",format!("AddCondition({args_raw});"),
            "CloseCondition();","CloseStage();","CloseMission();"
        ],
        "p3d_references":[],
        "command_invocations":[
            {"ordinal":1,"name":"selectmission","args_raw":"\"m1\"","semantic_role":"mission-script","arguments":["m1"]},
            {"ordinal":2,"name":"addstage","args_raw":"","semantic_role":"mission-stage","arguments":[]},
            {"ordinal":3,"name":"addobjective","args_raw":"\"dummy\"","semantic_role":"mission-objective","arguments":["dummy"]},
            {"ordinal":4,"name":"closeobjective","args_raw":"","semantic_role":"mission-objective","arguments":[]},
            {"ordinal":5,"name":"addcondition","args_raw":args_raw,"semantic_role":"mission-script","arguments":arguments},
            {"ordinal":6,"name":"closecondition","args_raw":"","semantic_role":"mission-script","arguments":[]},
            {"ordinal":7,"name":"closestage","args_raw":"","semantic_role":"mission-stage","arguments":[]},
            {"ordinal":8,"name":"closemission","args_raw":"","semantic_role":"mission-script","arguments":[]}
        ]
    }))
    .map_err(|error| error.to_string())
}

fn parameters(
    arguments: &[&str],
) -> Result<MissionConditionParameters, String> {
    let evidence = preflight_mission_script(&condition_document(arguments)?)?;
    let report = preflight_mission_condition_parameters(&evidence)?;
    let [binding] = report.conditions() else {
        return Err("condition parameter fixture count changed".to_owned());
    };
    Ok(binding.parameters().clone())
}

#[test]
fn types_keepbarrel_without_inventing_meaning() -> Result<(), String> {
    if parameters(&["keepbarrel", "3"])?
        != (MissionConditionParameters::KeepBarrelLegacyValue { value: 3 })
    {
        return Err("keepbarrel legacy value changed".to_owned());
    }
    Ok(())
}

#[test]
fn preserves_exact_damage_legacy_token() -> Result<(), String> {
    let MissionConditionParameters::DamageLegacyToken { source_token, code } =
        parameters(&["damage", "neither"])?
    else {
        return Err("damage legacy token was not retained".to_owned());
    };
    if source_token != "neither"
        || code != "legacy-damage-condition-neither-parameter-v1"
    {
        return Err("damage legacy token evidence changed".to_owned());
    }
    Ok(())
}

#[test]
fn rejects_unobserved_condition_parameter_values() -> Result<(), String> {
    for arguments in [
        vec!["keepbarrel", "0"],
        vec!["keepbarrel", "5"],
        vec!["damage", "both"],
        vec!["timeout", "1"],
    ] {
        let evidence =
            preflight_mission_script(&condition_document(&arguments)?)?;
        if preflight_mission_condition_parameters(&evidence).is_ok() {
            return Err(format!(
                "unreviewed condition parameter was accepted: {arguments:?}"
            ));
        }
    }
    Ok(())
}

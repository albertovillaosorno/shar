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
//   - Mission condition alias-registry unit tests.
// - Must-Not:
//   - Materialize Unreal mission assets.
// - Allows:
//   - Exhaustive closed-table and fail-closed regression checks.
// - Split-When:
//   - Modifier-schema tests gain an independent lifecycle.
// - Merge-When:
//   - Another test module owns this exact registry boundary.
// - Summary:
//   - Mission condition alias-registry tests.
// - Description:
//   - Proves all observed AddCondition aliases are registered exactly once.
// - Usage:
//   - Included by the mission-condition domain module under cfg(test).
// - Defaults:
//   - Unknown aliases and arity drift fail.
//

//! Mission condition alias-registry tests.

use super::{CONDITION_ALIASES, condition_alias_schema};

#[test]
fn observed_condition_aliases_have_one_versioned_schema() -> Result<(), String>
{
    let expected = [
        ("damage", "legacy-mission-condition.damage.v1", 1, 2),
        (
            "followdistance",
            "legacy-mission-condition.follow-distance.v1",
            1,
            1,
        ),
        (
            "keepbarrel",
            "legacy-mission-condition.keep-barrel.v1",
            2,
            2,
        ),
        (
            "outofvehicle",
            "legacy-mission-condition.out-of-vehicle.v1",
            1,
            1,
        ),
        ("position", "legacy-mission-condition.position.v1", 1, 1),
        ("race", "legacy-mission-condition.race.v1", 1, 1),
        ("timeout", "legacy-mission-condition.timeout.v1", 1, 1),
    ];
    if CONDITION_ALIASES.len() != expected.len() {
        return Err("condition alias registry size changed".to_owned());
    }
    for (alias, schema_id, minimum, maximum) in expected {
        let schema = condition_alias_schema(alias)
            .ok_or_else(|| format!("missing condition alias: {alias}"))?;
        if schema.schema_id != schema_id
            || schema.minimum_arguments != minimum
            || schema.maximum_arguments != maximum
        {
            return Err(format!("condition alias contract changed: {alias}"));
        }
    }
    Ok(())
}

#[test]
fn condition_registry_rejects_unknown_or_case_drifted_aliases() {
    assert!(condition_alias_schema("unknown").is_none());
    assert!(condition_alias_schema("Timeout").is_none());
}

use serde_json::json;

use super::modifier::{CONDITION_MODIFIERS, condition_modifier_schema};
use super::{
    preflight_mission_condition_commands, preflight_mission_conditions,
};
use crate::domain::preflight_mission_script;

fn mission_with_condition_modifier(
    condition_arguments: &[&str],
    command: &str,
    command_arguments: &[&str],
) -> Result<String, String> {
    let condition_raw = condition_arguments
        .iter()
        .enumerate()
        .map(|(index, value)| {
            if index == 0 {
                format!("\"{value}\"")
            } else {
                (*value).to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join(",");
    let command_raw = command_arguments.join(",");
    let command_role = if command.contains("stage") {
        "mission-stage"
    } else if command.contains("objective") {
        "mission-objective"
    } else if command.contains("reward") {
        "mission-reward"
    } else if command.contains("loadp3d") {
        "asset-load"
    } else {
        "mission-script"
    };
    let mut command_counts = serde_json::Map::new();
    for name in [
        "selectmission",
        "addstage",
        "addcondition",
        command,
        "closecondition",
        "closestage",
        "closemission",
    ] {
        let current = command_counts
            .get(name)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0);
        let next = current
            .checked_add(1)
            .ok_or_else(|| "command count overflowed".to_owned())?;
        drop(command_counts.insert(name.to_owned(), json!(next)));
    }
    serde_json::to_string(&json!({
        "schema": "shar-schoenwald.straggler.mission-script.v3",
        "source_extension": "mfk",
        "route_class": "mission",
        "source_bytes": 128,
        "context_command_count": 6,
        "context_adaptation_count": 0,
        "context_adaptations": [],
        "context_finding_count": 0,
        "context_findings": [],
        "statement_count": 7,
        "unique_command_count": command_counts.len(),
        "load_p3d_reference_count": 0,
        "mission_flow_command_count": if command.contains("stage")
            || command.contains("mission")
            || command.contains("objective") { 5 } else { 4 },
        "vehicle_physics_command_count": 0,
        "semantic_family": "mission-script",
        "command_counts": command_counts,
        "source_statements": [
            "SelectMission(\"m1\");",
            "AddStage(0);",
            format!("AddCondition({condition_raw});"),
            format!("{command}({command_raw});"),
            "CloseCondition();",
            "CloseStage();",
            "CloseMission();"
        ],
        "p3d_references": [],
        "command_invocations": [
            {"ordinal":1,"name":"selectmission","args_raw":"\"m1\"","semantic_role":"mission-script","arguments":["m1"]},
            {"ordinal":2,"name":"addstage","args_raw":"0","semantic_role":"mission-stage","arguments":["0"]},
            {"ordinal":3,"name":"addcondition","args_raw":condition_raw,"semantic_role":"mission-script","arguments":condition_arguments},
            {"ordinal":4,"name":command,"args_raw":command_raw,"semantic_role":command_role,"arguments":command_arguments},
            {"ordinal":5,"name":"closecondition","args_raw":"","semantic_role":"mission-script","arguments":[]},
            {"ordinal":6,"name":"closestage","args_raw":"","semantic_role":"mission-stage","arguments":[]},
            {"ordinal":7,"name":"closemission","args_raw":"","semantic_role":"mission-script","arguments":[]}
        ]
    }))
    .map_err(|error| error.to_string())
}

fn reviewed_keepbarrel_adaptation_document() -> Result<String, String> {
    let mut statements = (1..=116)
        .map(|ordinal| format!("legacy non-call statement {ordinal}"))
        .collect::<Vec<_>>();
    for (index, value) in [
        (0, "SelectMission(\"m7\");"),
        (1, "AddStage(0);"),
        (111, "StageStartMusicEvent(\"L7_drama\");"),
        (112, "AddCondition(\"keepbarrel\", 2);"),
        (113, "ShowStageComplete();"),
        (114, "CloseStage();"),
        (115, "CloseMission();"),
    ] {
        let slot = statements.get_mut(index).ok_or_else(|| {
            "adaptation fixture index is out of range".to_owned()
        })?;
        *slot = value.to_owned();
    }
    serde_json::to_string(&json!({
        "schema": "shar-schoenwald.straggler.mission-script.v3",
        "source_extension": "mfk",
        "route_class": "mission",
        "source_bytes": 512,
        "context_command_count": 5,
        "context_adaptation_count": 1,
        "context_adaptations": [{
            "ordinal": 114,
            "command": "showstagecomplete",
            "code": "legacy-l7-m7i-close-keepbarrel-before-stage-complete-v1"
        }],
        "context_finding_count": 0,
        "context_findings": [],
        "statement_count": 116,
        "unique_command_count": 7,
        "load_p3d_reference_count": 0,
        "mission_flow_command_count": 6,
        "vehicle_physics_command_count": 0,
        "semantic_family": "mission-script",
        "command_counts": {
            "addcondition":1,
            "addstage":1,
            "closemission":1,
            "closestage":1,
            "selectmission":1,
            "showstagecomplete":1,
            "stagestartmusicevent":1
        },
        "source_statements": statements,
        "p3d_references": [],
        "command_invocations": [
            {"ordinal":1,"name":"selectmission","args_raw":"\"m7\"","semantic_role":"mission-script","arguments":["m7"]},
            {"ordinal":2,"name":"addstage","args_raw":"0","semantic_role":"mission-stage","arguments":["0"]},
            {"ordinal":112,"name":"stagestartmusicevent","args_raw":"\"L7_drama\"","semantic_role":"mission-stage","arguments":["L7_drama"]},
            {"ordinal":113,"name":"addcondition","args_raw":"\"keepbarrel\", 2","semantic_role":"mission-script","arguments":["keepbarrel","2"]},
            {"ordinal":114,"name":"showstagecomplete","args_raw":"","semantic_role":"mission-stage","arguments":[]},
            {"ordinal":115,"name":"closestage","args_raw":"","semantic_role":"mission-stage","arguments":[]},
            {"ordinal":116,"name":"closemission","args_raw":"","semantic_role":"mission-script","arguments":[]}
        ]
    }))
    .map_err(|error| error.to_string())
}

#[test]
fn condition_scope_registry_keeps_exact_commands() -> Result<(), String> {
    if CONDITION_MODIFIERS.len() != 9 {
        return Err("condition modifier registry size changed".to_owned());
    }
    let follow =
        condition_modifier_schema("followdistance", "setfollowdistances")
            .ok_or_else(|| "follow-distance modifier disappeared".to_owned())?;
    if follow.argument_counts != [2]
        || condition_modifier_schema("keepbarrel", "showstagecomplete")
            .is_some()
        || condition_modifier_schema("timeout", "setcondtime").is_some()
    {
        return Err("condition modifier scope or arity changed".to_owned());
    }
    Ok(())
}

#[test]
fn condition_scope_preflight_accepts_exact_scope_and_rejects_drift()
-> Result<(), String> {
    let evidence = preflight_mission_script(&mission_with_condition_modifier(
        &["followdistance"],
        "setfollowdistances",
        &["150", "75"],
    )?)?;
    drop(preflight_mission_conditions(&evidence)?);
    let report = preflight_mission_condition_commands(&evidence)?;
    let [command] = report.commands() else {
        return Err(
            "single condition modifier fixture changed count".to_owned()
        );
    };
    if command.condition_alias() != "followdistance"
        || command.ordinal() != 4
        || command.command() != "setfollowdistances"
        || command.arguments() != ["150", "75"]
    {
        return Err("typed condition modifier changed".to_owned());
    }

    for (condition, command_name, arguments) in [
        ("timeout", "setcondtime", vec!["1000"]),
        ("followdistance", "setfollowdistances", vec!["150"]),
    ] {
        let evidence =
            preflight_mission_script(&mission_with_condition_modifier(
                &[condition],
                command_name,
                &arguments,
            )?)?;
        if preflight_mission_condition_commands(&evidence).is_ok() {
            return Err(format!(
                "condition-scoped drift was accepted: {condition}/{command_name}"
            ));
        }
    }
    Ok(())
}

#[test]
fn keepbarrel_adaptation_closes_scope_before_stage_complete()
-> Result<(), String> {
    let evidence =
        preflight_mission_script(&reviewed_keepbarrel_adaptation_document()?)?;
    let report = preflight_mission_condition_commands(&evidence)?;
    if !report.commands().is_empty() {
        return Err("stage completion leaked into keepbarrel condition scope"
            .to_owned());
    }
    Ok(())
}

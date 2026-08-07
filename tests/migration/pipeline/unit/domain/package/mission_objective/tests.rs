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
//   - Mission objective alias-registry unit tests.
// - Must-Not:
//   - Materialize Unreal mission assets.
// - Allows:
//   - Exhaustive closed-table and fail-closed regression checks.
// - Split-When:
//   - Parameter-schema tests gain an independent lifecycle.
// - Merge-When:
//   - Another test module owns this exact registry boundary.
// - Summary:
//   - Mission objective alias-registry tests.
// - Description:
//   - Proves all observed aliases are terminally mapped or unavailable.
// - Usage:
//   - Included by the mission-objective domain module under cfg(test).
// - Defaults:
//   - Unknown aliases and arity drift fail.
//

//! Mission objective alias-registry tests.

use super::{OBJECTIVE_ALIASES, objective_alias_schema};

#[test]
fn observed_objective_aliases_have_one_terminal_registry_result()
-> Result<(), String> {
    let expected = [
        ("buycar", Some("buy_vehicle"), 2, 2),
        ("buyskin", Some("buy_costume"), 2, 2),
        ("coins", Some("wager_entry"), 1, 1),
        ("delivery", Some("deliver"), 1, 2),
        ("destroy", Some("destroy"), 1, 2),
        ("destroyboss", Some("boss_phase"), 1, 1),
        ("dialogue", Some("dialogue"), 1, 1),
        ("dummy", None, 1, 1),
        ("dump", Some("dumped_collectible"), 1, 2),
        ("fmv", Some("cinematic"), 1, 1),
        ("follow", Some("follow"), 1, 2),
        ("getin", Some("enter_vehicle"), 1, 2),
        ("gooutside", Some("exit_interior"), 1, 1),
        ("goto", Some("travel"), 1, 2),
        ("interior", Some("enter_interior"), 1, 2),
        ("losetail", Some("avoid"), 1, 2),
        ("pickupitem", Some("item_pickup"), 1, 1),
        ("race", Some("race"), 1, 3),
        ("talkto", Some("talk"), 1, 2),
        ("timer", Some("timer"), 1, 1),
    ];
    if OBJECTIVE_ALIASES.len() != expected.len() {
        return Err("objective alias registry size changed".to_owned());
    }
    for (alias, kind, minimum, maximum) in expected {
        let schema = objective_alias_schema(alias)
            .ok_or_else(|| format!("missing objective alias: {alias}"))?;
        if schema.canonical_kind != kind
            || schema.minimum_arguments != minimum
            || schema.maximum_arguments != maximum
        {
            return Err(format!("objective alias contract changed: {alias}"));
        }
    }
    Ok(())
}

#[test]
fn dummy_is_explicitly_unavailable_and_unknown_aliases_do_not_resolve()
-> Result<(), String> {
    let dummy = objective_alias_schema("dummy")
        .ok_or_else(|| "dummy objective alias disappeared".to_owned())?;
    if dummy.canonical_kind.is_some()
        || dummy.unavailable_code
            != Some("legacy-dummy-objective-unavailable-v1")
        || objective_alias_schema("execute_arbitrary_script").is_some()
        || objective_alias_schema("GOTO").is_some()
    {
        return Err("objective availability boundary changed".to_owned());
    }
    Ok(())
}

use serde_json::json;

use super::{MissionObjectiveBinding, preflight_mission_objectives};
use crate::domain::preflight_mission_script;

fn mission_with_objective(arguments: &[&str]) -> Result<String, String> {
    let invocation_arguments = arguments
        .iter()
        .map(|value| json!(value))
        .collect::<Vec<_>>();
    let alias = arguments.first().copied().unwrap_or_default();
    let mut command_counts = serde_json::Map::new();
    for command in [
        "selectmission",
        "addstage",
        "addobjective",
        "closeobjective",
        "closestage",
        "closemission",
    ] {
        drop(command_counts.insert(command.to_owned(), json!(1)));
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
        "statement_count": 6,
        "unique_command_count": 6,
        "load_p3d_reference_count": 0,
        "mission_flow_command_count": 0,
        "vehicle_physics_command_count": 0,
        "semantic_family": "mission-script",
        "command_counts": command_counts,
        "source_statements": [
            "SelectMission(\"m1\");",
            "AddStage(0);",
            format!("AddObjective(\"{alias}\");"),
            "CloseObjective();",
            "CloseStage();",
            "CloseMission();"
        ],
        "p3d_references": [],
        "command_invocations": [
            {
                "ordinal": 1,
                "name": "selectmission",
                "args_raw": "\"m1\"",
                "semantic_role": "mission-script",
                "arguments": ["m1"]
            },
            {
                "ordinal": 2,
                "name": "addstage",
                "args_raw": "0",
                "semantic_role": "mission-stage",
                "arguments": ["0"]
            },
            {
                "ordinal": 3,
                "name": "addobjective",
                "args_raw": alias,
                "semantic_role": "mission-stage",
                "arguments": invocation_arguments
            },
            {
                "ordinal": 4,
                "name": "closeobjective",
                "args_raw": "",
                "semantic_role": "mission-stage",
                "arguments": []
            },
            {
                "ordinal": 5,
                "name": "closestage",
                "args_raw": "",
                "semantic_role": "mission-stage",
                "arguments": []
            },
            {
                "ordinal": 6,
                "name": "closemission",
                "args_raw": "",
                "semantic_role": "mission-script",
                "arguments": []
            }
        ]
    }))
    .map_err(|error| error.to_string())
}

fn only_objective(
    arguments: &[&str],
) -> Result<MissionObjectiveBinding, String> {
    let evidence =
        preflight_mission_script(&mission_with_objective(arguments)?)?;
    let report = preflight_mission_objectives(&evidence)?;
    let [objective] = report.objectives() else {
        return Err("single objective fixture changed count".to_owned());
    };
    Ok(objective.clone())
}

#[test]
fn semantic_evidence_maps_reviewed_objective_and_retains_legacy_parameters()
-> Result<(), String> {
    let objective = only_objective(&["race", "gamble", "both"])?;
    if objective.ordinal() != 3
        || objective.source_alias() != "race"
        || objective.canonical_kind() != Some("race")
        || objective.legacy_parameters() != ["gamble", "both"]
        || !objective.is_mapped()
        || objective.unavailable_code().is_some()
    {
        return Err("typed race objective mapping changed".to_owned());
    }
    Ok(())
}

#[test]
fn semantic_evidence_retains_explicit_dummy_unavailability()
-> Result<(), String> {
    let objective = only_objective(&["dummy"])?;
    if objective.canonical_kind().is_some()
        || objective.is_mapped()
        || objective.unavailable_code()
            != Some("legacy-dummy-objective-unavailable-v1")
    {
        return Err("dummy unavailability was lost".to_owned());
    }
    Ok(())
}

#[test]
fn semantic_evidence_rejects_unknown_alias_and_arity_drift()
-> Result<(), String> {
    for arguments in [
        vec!["unknown"],
        vec!["buycar"],
        vec!["timer", "unexpected"],
        vec!["race", "one", "two", "three"],
    ] {
        let evidence =
            preflight_mission_script(&mission_with_objective(&arguments)?)?;
        if preflight_mission_objectives(&evidence).is_ok() {
            return Err(format!(
                "invalid objective call was accepted: {arguments:?}"
            ));
        }
    }
    Ok(())
}

use super::modifier::{OBJECTIVE_MODIFIERS, objective_modifier_schema};
use super::preflight_mission_objective_commands;

fn mission_with_objective_modifier(
    objective_arguments: &[&str],
    command: &str,
    command_arguments: &[&str],
) -> Result<String, String> {
    let mut value: serde_json::Value =
        serde_json::from_str(&mission_with_objective(objective_arguments)?)
            .map_err(|error| error.to_string())?;
    let statement_count = value
        .get_mut("statement_count")
        .ok_or_else(|| "fixture statement count disappeared".to_owned())?;
    *statement_count = json!(7);
    let unique_count = value
        .get_mut("unique_command_count")
        .ok_or_else(|| "fixture unique command count disappeared".to_owned())?;
    *unique_count = json!(7);
    let statements = value
        .get_mut("source_statements")
        .and_then(serde_json::Value::as_array_mut)
        .ok_or_else(|| "fixture source statements disappeared".to_owned())?;
    statements.insert(3, json!(format!("{command}();")));
    let invocations = value
        .get_mut("command_invocations")
        .and_then(serde_json::Value::as_array_mut)
        .ok_or_else(|| "fixture command invocations disappeared".to_owned())?;
    for invocation in invocations.iter_mut().skip(3) {
        let ordinal = invocation
            .get("ordinal")
            .and_then(serde_json::Value::as_u64)
            .ok_or_else(|| {
                "fixture invocation ordinal disappeared".to_owned()
            })?;
        let ordinal_value = invocation.get_mut("ordinal").ok_or_else(|| {
            "fixture invocation ordinal disappeared".to_owned()
        })?;
        *ordinal_value = json!(ordinal.saturating_add(1));
    }
    invocations.insert(
        3,
        json!({
            "ordinal": 4,
            "name": command,
            "args_raw": command_arguments.join(","),
            "semantic_role": "mission-stage",
            "arguments": command_arguments
        }),
    );
    let counts = value
        .get_mut("command_counts")
        .and_then(serde_json::Value::as_object_mut)
        .ok_or_else(|| "fixture command counts disappeared".to_owned())?;
    drop(counts.insert(command.to_owned(), json!(1)));
    serde_json::to_string(&value).map_err(|error| error.to_string())
}

#[test]
fn objective_scope_registry_keeps_exact_observed_command_arities()
-> Result<(), String> {
    if OBJECTIVE_MODIFIERS.len() != 93 {
        return Err("objective modifier registry size changed".to_owned());
    }
    let talk_target = objective_modifier_schema("talkto", "settalktotarget")
        .ok_or_else(|| "talk target modifier disappeared".to_owned())?;
    if talk_target.argument_counts != [1, 3, 4]
        || objective_modifier_schema("talkto", "setdestination").is_some()
        || objective_modifier_schema("goto", "setdestination").is_none()
    {
        return Err("objective modifier scope or arity changed".to_owned());
    }
    Ok(())
}

#[test]
fn objective_scope_preflight_accepts_exact_scope_and_rejects_drift()
-> Result<(), String> {
    let valid = preflight_mission_script(&mission_with_objective_modifier(
        &["talkto"],
        "settalktotarget",
        &["apu", "0", "0"],
    )?)?;
    let report = preflight_mission_objective_commands(&valid)?;
    let [command] = report.commands() else {
        return Err("single modifier fixture changed count".to_owned());
    };
    if command.objective_alias() != "talkto"
        || command.ordinal() != 4
        || command.command() != "settalktotarget"
        || command.arguments() != ["apu", "0", "0"]
    {
        return Err("typed objective modifier changed".to_owned());
    }

    for (objective, command_name, arguments) in [
        ("talkto", "setdestination", vec!["target"]),
        ("talkto", "settalktotarget", vec!["a", "b"]),
    ] {
        let evidence =
            preflight_mission_script(&mission_with_objective_modifier(
                &[objective],
                command_name,
                &arguments,
            )?)?;
        if preflight_mission_objective_commands(&evidence).is_ok() {
            return Err(format!(
                "objective-scoped drift was accepted: {objective}/{command_name}"
            ));
        }
    }
    Ok(())
}

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
//   - Mission scope graph projection unit tests.
// - Must-Not:
//   - Infer gameplay parameter meaning or construct Unreal assets.
// - Allows:
//   - Synthetic exact-schema mission structure and ownership checks.
// - Split-When:
//   - Typed parameter compilation gains independent fixtures.
// - Merge-When:
//   - Mission scope projection loses independent policy.
// - Summary:
//   - Mission scope graph projection tests.
// - Description:
//   - Proves stage, objective, condition, and scoped command ownership.
// - Usage:
//   - Included by the mission-scope domain module under cfg(test).
// - Defaults:
//   - Invalid root-objective cardinality fails closed.
//

//! Mission scope graph projection tests.

use serde_json::{Value, json};

use super::{MissionConditionScope, compile_mission_scope_graphs};
use crate::domain::{
    MissionObjectiveParameters, MissionRoadArrowBinding, MissionRoadArrowMode,
    preflight_mission_script,
};

fn base_document(invocations: &Value, statement_count: usize) -> Value {
    let mut statements = (1..=statement_count)
        .map(|ordinal| format!("Statement{ordinal}();"))
        .collect::<Vec<_>>();
    let rows = invocations.as_array().cloned().unwrap_or_default();
    for row in &rows {
        let ordinal = row
            .get("ordinal")
            .and_then(Value::as_u64)
            .and_then(|value| usize::try_from(value).ok());
        let name = row.get("name").and_then(Value::as_str);
        let args_raw = row.get("args_raw").and_then(Value::as_str);
        if let (Some(ordinal), Some(name), Some(args_raw)) =
            (ordinal, name, args_raw)
            && let Some(slot) = ordinal
                .checked_sub(1)
                .and_then(|index| statements.get_mut(index))
        {
            *slot = format!("{name}({args_raw});");
        }
    }
    let mission_flow_command_count = rows
        .iter()
        .filter_map(|row| row.get("name").and_then(Value::as_str))
        .filter(|name| {
            name.contains("stage")
                || name.contains("mission")
                || name.contains("objective")
        })
        .count();
    json!({
        "schema": "shar-schoenwald.straggler.mission-script.v3",
        "source_extension": "mfk",
        "route_class": "mission",
        "source_bytes": 256,
        "context_command_count": statement_count,
        "context_adaptation_count": 0,
        "context_adaptations": [],
        "context_finding_count": 0,
        "context_findings": [],
        "statement_count": statement_count,
        "unique_command_count": 8,
        "load_p3d_reference_count": 0,
        "mission_flow_command_count": mission_flow_command_count,
        "vehicle_physics_command_count": 0,
        "semantic_family": "mission-script",
        "command_counts": {
            "selectmission": 1,
            "closemission": 1,
            "addstage": 2,
            "closestage": 2,
            "addobjective": 2,
            "closeobjective": 2,
            "addcondition": 2,
            "closecondition": 2
        },
        "source_statements": statements,
        "p3d_references": [],
        "command_invocations": invocations
    })
}

fn scope_document() -> Value {
    base_document(
        &json!([
            {"ordinal":1,"name":"selectmission","args_raw":"\"m1\"","semantic_role":"mission-script","arguments":["m1"]},
            {"ordinal":2,"name":"addstage","args_raw":"0,final","semantic_role":"mission-stage","arguments":["0","final"]},
            {"ordinal":3,"name":"addobjective","args_raw":"\"race\"","semantic_role":"mission-objective","arguments":["race"]},
            {"ordinal":4,"name":"addcondition","args_raw":"\"timeout\"","semantic_role":"mission-script","arguments":["timeout"]},
            {"ordinal":5,"name":"closecondition","args_raw":"","semantic_role":"mission-script","arguments":[]},
            {"ordinal":6,"name":"closeobjective","args_raw":"","semantic_role":"mission-objective","arguments":[]},
            {"ordinal":7,"name":"closestage","args_raw":"","semantic_role":"mission-stage","arguments":[]},
            {"ordinal":8,"name":"addstage","args_raw":"1","semantic_role":"mission-stage","arguments":["1"]},
            {"ordinal":9,"name":"addobjective","args_raw":"\"goto\",\"both\"","semantic_role":"mission-objective","arguments":["goto","both"]},
            {"ordinal":10,"name":"closeobjective","args_raw":"","semantic_role":"mission-objective","arguments":[]},
            {"ordinal":11,"name":"addcondition","args_raw":"\"damage\",\"neither\"","semantic_role":"mission-script","arguments":["damage","neither"]},
            {"ordinal":12,"name":"closecondition","args_raw":"","semantic_role":"mission-script","arguments":[]},
            {"ordinal":13,"name":"closestage","args_raw":"","semantic_role":"mission-stage","arguments":[]},
            {"ordinal":14,"name":"closemission","args_raw":"","semantic_role":"mission-script","arguments":[]}
        ]),
        14,
    )
}

fn preflight(
    value: &Value,
) -> Result<crate::domain::MissionScriptEvidence, String> {
    let text =
        serde_json::to_string(value).map_err(|error| error.to_string())?;
    preflight_mission_script(&text)
}

#[test]
fn projects_stage_objective_and_condition_ownership() -> Result<(), String> {
    let evidence = preflight(&scope_document())?;
    let report = compile_mission_scope_graphs(&evidence)?;
    let [mission] = report.missions() else {
        return Err("scope fixture changed mission count".to_owned());
    };
    if mission.source_ordinal() != 1
        || mission.source_mission_id() != "m1"
        || mission.stages().len() != 2
        || !mission.has_only_mapped_objectives()
        || !report.has_only_mapped_objectives()
    {
        return Err("mission scope envelope changed".to_owned());
    }

    let [first, second] = mission.stages() else {
        return Err("scope fixture stage count changed".to_owned());
    };
    if first.source_ordinal() != 2
        || first.sequence_ordinal() != 0
        || first.legacy_parameters() != ["0", "final"]
        || first.objective().binding().source_alias() != "race"
        || first.objective().binding().canonical_kind() != Some("race")
        || first.objective().commands().len() != 2
    {
        return Err("first stage scope changed".to_owned());
    }
    let [nested] = first.conditions() else {
        return Err("nested condition count changed".to_owned());
    };
    if nested.scope() != MissionConditionScope::Objective
        || nested.owner_objective_source_ordinal()
            != Some(first.objective().binding().ordinal())
        || nested.binding().source_alias() != "timeout"
        || nested.binding().schema_id() != "legacy-mission-condition.timeout.v1"
        || !nested.commands().is_empty()
    {
        return Err("nested condition ownership changed".to_owned());
    }

    let [stage_condition] = second.conditions() else {
        return Err("stage condition count changed".to_owned());
    };
    if second.sequence_ordinal() != 1
        || second.objective().binding().source_alias() != "goto"
        || second.objective().parameters().parameters()
            != &MissionObjectiveParameters::RoadArrows(
                MissionRoadArrowBinding::Effective(MissionRoadArrowMode::Both),
            )
        || stage_condition.scope() != MissionConditionScope::Stage
        || stage_condition.owner_objective_source_ordinal().is_some()
        || stage_condition.binding().source_alias() != "damage"
    {
        return Err("second stage ownership changed".to_owned());
    }
    Ok(())
}

#[test]
fn utility_script_without_mission_context_projects_no_graphs()
-> Result<(), String> {
    let value = json!({
        "schema": "shar-schoenwald.straggler.mission-script.v3",
        "source_extension": "mfk",
        "route_class": "mission",
        "source_bytes": 32,
        "context_command_count": 0,
        "context_adaptation_count": 0,
        "context_adaptations": [],
        "context_finding_count": 0,
        "context_findings": [],
        "statement_count": 1,
        "unique_command_count": 1,
        "load_p3d_reference_count": 0,
        "mission_flow_command_count": 0,
        "vehicle_physics_command_count": 0,
        "semantic_family": "mission-script",
        "command_counts": {"bindreward": 1},
        "source_statements": ["BindReward();"],
        "p3d_references": [],
        "command_invocations": [
            {"ordinal":1,"name":"bindreward","args_raw":"","semantic_role":"mission-reward","arguments":[]}
        ]
    });
    let report = compile_mission_scope_graphs(&preflight(&value)?)?;
    let [command] = report.unscoped_commands() else {
        return Err(
            "utility command was not retained outside mission scope".to_owned()
        );
    };
    if command.source_ordinal() != 1
        || command.name() != "bindreward"
        || command.args_raw() != ""
        || command.semantic_role() != "mission-reward"
        || !command.arguments().is_empty()
        || !report.missions().is_empty()
        || !report.has_only_mapped_objectives()
    {
        return Err("utility script scope projection changed".to_owned());
    }
    Ok(())
}

#[test]
fn unavailable_objective_remains_structural_but_incomplete()
-> Result<(), String> {
    let mut value = scope_document();
    let invocation = value
        .pointer_mut("/command_invocations/2")
        .ok_or_else(|| "objective fixture disappeared".to_owned())?;
    *invocation = json!({
        "ordinal": 3,
        "name": "addobjective",
        "args_raw": "\"dummy\"",
        "semantic_role": "mission-objective",
        "arguments": ["dummy"]
    });
    *value
        .pointer_mut("/command_invocations/3")
        .ok_or_else(|| "invocation 4 fixture disappeared".to_owned())? = json!({
        "ordinal":4,"name":"closeobjective","args_raw":"",
        "semantic_role":"mission-objective","arguments":[]
    });
    *value
        .pointer_mut("/command_invocations/4")
        .ok_or_else(|| "invocation 5 fixture disappeared".to_owned())? = json!({
        "ordinal":5,"name":"addcondition","args_raw":"\"timeout\"",
        "semantic_role":"mission-script","arguments":["timeout"]
    });
    *value
        .pointer_mut("/command_invocations/5")
        .ok_or_else(|| "invocation 6 fixture disappeared".to_owned())? = json!({
        "ordinal":6,"name":"closecondition","args_raw":"",
        "semantic_role":"mission-script","arguments":[]
    });
    *value
        .pointer_mut("/source_statements/2")
        .ok_or_else(|| "statement 3 fixture disappeared".to_owned())? =
        json!("AddObjective(\"dummy\");");
    *value
        .pointer_mut("/source_statements/3")
        .ok_or_else(|| "statement 4 fixture disappeared".to_owned())? =
        json!("CloseObjective();");
    *value
        .pointer_mut("/source_statements/4")
        .ok_or_else(|| "statement 5 fixture disappeared".to_owned())? =
        json!("AddCondition(\"timeout\");");
    *value
        .pointer_mut("/source_statements/5")
        .ok_or_else(|| "statement 6 fixture disappeared".to_owned())? =
        json!("CloseCondition();");
    let report = compile_mission_scope_graphs(&preflight(&value)?)?;
    let mission = report
        .missions()
        .first()
        .ok_or_else(|| "mission graph disappeared".to_owned())?;
    let objective = mission
        .stages()
        .first()
        .ok_or_else(|| "mission stage disappeared".to_owned())?
        .objective()
        .binding();
    if objective.source_alias() != "dummy"
        || objective.canonical_kind().is_some()
        || objective.unavailable_code()
            != Some("legacy-dummy-objective-unavailable-v1")
        || report.has_only_mapped_objectives()
    {
        return Err("unavailable objective boundary changed".to_owned());
    }
    Ok(())
}

#[test]
fn preserves_direct_mission_and_stage_commands_without_interpretation()
-> Result<(), String> {
    let value = json!({
        "schema": "shar-schoenwald.straggler.mission-script.v3",
        "source_extension": "mfk",
        "route_class": "mission",
        "source_bytes": 160,
        "context_command_count": 6,
        "context_adaptation_count": 0,
        "context_adaptations": [],
        "context_finding_count": 0,
        "context_findings": [],
        "statement_count": 8,
        "unique_command_count": 8,
        "load_p3d_reference_count": 0,
        "mission_flow_command_count": 7,
        "vehicle_physics_command_count": 0,
        "semantic_family": "mission-script",
        "command_counts": {
            "selectmission":1,"setdynaloaddata":1,"addstage":1,
            "setstagetime":1,"addobjective":1,"closeobjective":1,
            "closestage":1,"closemission":1
        },
        "source_statements": [
            "SelectMission(\"m1\");","SetDynaLoadData(\"l1\");",
            "AddStage(0);","SetStageTime(30);","AddObjective(\"goto\");",
            "CloseObjective();","CloseStage();","CloseMission();"
        ],
        "p3d_references": [],
        "command_invocations": [
            {"ordinal":1,"name":"selectmission","args_raw":"\"m1\"","semantic_role":"mission-script","arguments":["m1"]},
            {"ordinal":2,"name":"setdynaloaddata","args_raw":"\"l1\"","semantic_role":"mission-script","arguments":["l1"]},
            {"ordinal":3,"name":"addstage","args_raw":"0","semantic_role":"mission-stage","arguments":["0"]},
            {"ordinal":4,"name":"setstagetime","args_raw":"30","semantic_role":"mission-stage","arguments":["30"]},
            {"ordinal":5,"name":"addobjective","args_raw":"\"goto\"","semantic_role":"mission-objective","arguments":["goto"]},
            {"ordinal":6,"name":"closeobjective","args_raw":"","semantic_role":"mission-objective","arguments":[]},
            {"ordinal":7,"name":"closestage","args_raw":"","semantic_role":"mission-stage","arguments":[]},
            {"ordinal":8,"name":"closemission","args_raw":"","semantic_role":"mission-script","arguments":[]}
        ]
    });
    let report = compile_mission_scope_graphs(&preflight(&value)?)?;
    let [mission] = report.missions() else {
        return Err("direct-scope fixture changed mission count".to_owned());
    };
    let [mission_command] = mission.commands() else {
        return Err("mission-scope command was not retained".to_owned());
    };
    let [stage] = mission.stages() else {
        return Err("direct-scope fixture changed stage count".to_owned());
    };
    let [stage_command] = stage.commands() else {
        return Err("stage-scope command was not retained".to_owned());
    };
    if mission_command.source_ordinal() != 2
        || mission_command.name() != "setdynaloaddata"
        || mission_command.arguments() != ["l1"]
        || stage_command.source_ordinal() != 4
        || stage_command.name() != "setstagetime"
        || stage_command.arguments() != ["30"]
    {
        return Err(
            "direct command evidence changed during projection".to_owned()
        );
    }
    Ok(())
}

fn direct_command_document(
    command: &str,
    arguments: &[&str],
    stage_scope: bool,
) -> Value {
    let direct_ordinal = if stage_scope {
        3
    } else {
        2
    };
    let stage_ordinal = if stage_scope {
        2
    } else {
        3
    };
    let role = if command.contains("stage") {
        "mission-stage"
    } else if command.contains("objective") {
        "mission-objective"
    } else if command.contains("reward") {
        "mission-reward"
    } else {
        "mission-script"
    };
    let args_raw = arguments.join(",");
    let direct = json!({
        "ordinal": direct_ordinal,
        "name": command,
        "args_raw": args_raw,
        "semantic_role": role,
        "arguments": arguments
    });
    let stage = json!({
        "ordinal": stage_ordinal,
        "name": "addstage",
        "args_raw": "0",
        "semantic_role": "mission-stage",
        "arguments": ["0"]
    });
    let mut invocations = vec![json!({
        "ordinal":1,"name":"selectmission","args_raw":"\"m1\"",
        "semantic_role":"mission-script","arguments":["m1"]
    })];
    if stage_scope {
        invocations.push(stage);
        invocations.push(direct);
    } else {
        invocations.push(direct);
        invocations.push(stage);
    }
    invocations.extend([
        json!({"ordinal":4,"name":"addobjective","args_raw":"\"goto\"","semantic_role":"mission-objective","arguments":["goto"]}),
        json!({"ordinal":5,"name":"closeobjective","args_raw":"","semantic_role":"mission-objective","arguments":[]}),
        json!({"ordinal":6,"name":"closestage","args_raw":"","semantic_role":"mission-stage","arguments":[]}),
        json!({"ordinal":7,"name":"closemission","args_raw":"","semantic_role":"mission-script","arguments":[]}),
    ]);
    let mut counts = serde_json::Map::new();
    for name in [
        "selectmission",
        "addstage",
        "addobjective",
        "closeobjective",
        "closestage",
        "closemission",
    ] {
        drop(counts.insert(name.to_owned(), json!(1)));
    }
    drop(counts.insert(command.to_owned(), json!(1)));
    let mission_flow_command_count = invocations
        .iter()
        .filter_map(|row| row.get("name").and_then(Value::as_str))
        .filter(|name| {
            name.contains("stage")
                || name.contains("mission")
                || name.contains("objective")
        })
        .count();
    json!({
        "schema":"shar-schoenwald.straggler.mission-script.v3",
        "source_extension":"mfk","route_class":"mission","source_bytes":128,
        "context_command_count":6,"context_adaptation_count":0,
        "context_adaptations":[],"context_finding_count":0,"context_findings":[],
        "statement_count":7,"unique_command_count":7,"load_p3d_reference_count":0,
        "mission_flow_command_count":mission_flow_command_count,
        "vehicle_physics_command_count":0,
        "semantic_family":"mission-script","command_counts":counts,
        "source_statements": invocations.iter().map(|row| {
            let name = row.get("name").and_then(Value::as_str).unwrap_or("");
            let args = row.get("args_raw").and_then(Value::as_str).unwrap_or("");
            format!("{name}({args});")
        }).collect::<Vec<_>>(),
        "p3d_references":[],"command_invocations":invocations
    })
}

#[test]
fn rejects_mission_direct_command_in_stage_scope() -> Result<(), String> {
    let value = direct_command_document("setdynaloaddata", &["l1"], true);
    let evidence = preflight(&value)?;
    let Err(error) = compile_mission_scope_graphs(&evidence) else {
        return Err("mission command was accepted in stage scope".to_owned());
    };
    if !error.contains("not registered for its scope") {
        return Err(format!("unexpected direct-scope error: {error}"));
    }
    Ok(())
}

#[test]
fn rejects_unobserved_direct_command_arity() -> Result<(), String> {
    let value = direct_command_document("setstagetime", &["30", "extra"], true);
    let evidence = preflight(&value)?;
    let Err(error) = compile_mission_scope_graphs(&evidence) else {
        return Err("unobserved direct-command arity was accepted".to_owned());
    };
    if !error.contains("arity is not registered") {
        return Err(format!("unexpected direct-arity error: {error}"));
    }
    Ok(())
}

#[test]
fn nested_condition_modifier_belongs_to_most_specific_scope()
-> Result<(), String> {
    let value = json!({
        "schema":"shar-schoenwald.straggler.mission-script.v3",
        "source_extension":"mfk","route_class":"mission","source_bytes":192,
        "context_command_count":8,"context_adaptation_count":0,
        "context_adaptations":[],"context_finding_count":0,"context_findings":[],
        "statement_count":9,"unique_command_count":9,"load_p3d_reference_count":0,
        "mission_flow_command_count":6,"vehicle_physics_command_count":0,
        "semantic_family":"mission-script",
        "command_counts":{
            "selectmission":1,"addstage":1,"addobjective":1,"addcondition":1,
            "setcondminhealth":1,"closecondition":1,"closeobjective":1,
            "closestage":1,"closemission":1
        },
        "source_statements":[
            "SelectMission(\"m1\");","AddStage(0);","AddObjective(\"race\");",
            "AddCondition(\"damage\");","SetCondMinHealth(0.0);","CloseCondition();",
            "CloseObjective();","CloseStage();","CloseMission();"
        ],
        "p3d_references":[],
        "command_invocations":[
            {"ordinal":1,"name":"selectmission","args_raw":"\"m1\"","semantic_role":"mission-script","arguments":["m1"]},
            {"ordinal":2,"name":"addstage","args_raw":"0","semantic_role":"mission-stage","arguments":["0"]},
            {"ordinal":3,"name":"addobjective","args_raw":"\"race\"","semantic_role":"mission-objective","arguments":["race"]},
            {"ordinal":4,"name":"addcondition","args_raw":"\"damage\"","semantic_role":"mission-script","arguments":["damage"]},
            {"ordinal":5,"name":"setcondminhealth","args_raw":"0.0","semantic_role":"mission-script","arguments":["0.0"]},
            {"ordinal":6,"name":"closecondition","args_raw":"","semantic_role":"mission-script","arguments":[]},
            {"ordinal":7,"name":"closeobjective","args_raw":"","semantic_role":"mission-objective","arguments":[]},
            {"ordinal":8,"name":"closestage","args_raw":"","semantic_role":"mission-stage","arguments":[]},
            {"ordinal":9,"name":"closemission","args_raw":"","semantic_role":"mission-script","arguments":[]}
        ]
    });
    let report = compile_mission_scope_graphs(&preflight(&value)?)?;
    let mission = report
        .missions()
        .first()
        .ok_or_else(|| "mission graph disappeared".to_owned())?;
    let stage = mission
        .stages()
        .first()
        .ok_or_else(|| "mission stage disappeared".to_owned())?;
    let objective = stage.objective();
    let condition = stage
        .conditions()
        .first()
        .ok_or_else(|| "mission condition disappeared".to_owned())?;
    if objective.commands().len() != 2
        || objective
            .commands()
            .iter()
            .any(|command| command.command() == "setcondminhealth")
        || condition.commands().len() != 1
        || condition
            .commands()
            .first()
            .is_none_or(|command| command.command() != "setcondminhealth")
    {
        return Err("nested condition modifier ownership changed".to_owned());
    }
    Ok(())
}

fn unscoped_direct_document(command: &str, arguments: &[&str]) -> Value {
    let args_raw = arguments.join(",");
    json!({
        "schema":"shar-schoenwald.straggler.mission-script.v3",
        "source_extension":"mfk","route_class":"mission","source_bytes":64,
        "context_command_count":0,"context_adaptation_count":0,
        "context_adaptations":[],"context_finding_count":0,"context_findings":[],
        "statement_count":1,"unique_command_count":1,"load_p3d_reference_count":0,
        "mission_flow_command_count":0,"vehicle_physics_command_count":0,
        "semantic_family":"mission-script","command_counts":{command:1},
        "source_statements":[format!("{command}({args_raw});")],
        "p3d_references":[],
        "command_invocations":[{
            "ordinal":1,"name":command,"args_raw":args_raw,
            "semantic_role":"mission-script","arguments":arguments
        }]
    })
}

#[test]
fn accepts_registered_general_vehicle_initialization() -> Result<(), String> {
    let value = unscoped_direct_document("initlevelplayervehicle", &[
        "homer_v", "current", "DEFAULT",
    ]);
    let report = compile_mission_scope_graphs(&preflight(&value)?)?;
    let [command] = report.unscoped_commands() else {
        return Err(
            "general vehicle initialization was not retained".to_owned()
        );
    };
    if command.name() != "initlevelplayervehicle"
        || command.arguments() != ["homer_v", "current", "DEFAULT"]
    {
        return Err(
            "general vehicle initialization evidence changed".to_owned()
        );
    }
    Ok(())
}

#[test]
fn rejects_mission_only_direct_command_when_unscoped() -> Result<(), String> {
    let value = unscoped_direct_document("setdynaloaddata", &["l1"]);
    let evidence = preflight(&value)?;
    let Err(error) = compile_mission_scope_graphs(&evidence) else {
        return Err(
            "mission-only command was accepted outside a mission".to_owned()
        );
    };
    if !error.contains("not registered for its scope") {
        return Err(format!("unexpected unscoped command error: {error}"));
    }
    Ok(())
}

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
//   - Normalized mission-script semantic preflight unit tests.
// - Must-Not:
//   - Read repository game files or exercise Unreal mutation.
// - Allows:
//   - Synthetic exact-schema mission command evidence.
// - Split-When:
//   - Split when mission compilation gains a separate tested lifecycle.
// - Merge-When:
//   - Merge when mission evidence preflight loses independent policy.
// - Summary:
//   - Mission semantic preflight tests.
// - Description:
//   - Proves stale or structurally ambiguous MFK evidence fails closed.
// - Usage:
//   - Included only by the package mission-script module under cfg(test).
// - Defaults:
//   - No test relies on private extracted game data.
//

//! Mission semantic evidence preflight tests.

use serde_json::{Value, json};

use super::{MISSION_SCRIPT_SCHEMA, preflight_mission_script};

fn document() -> Value {
    json!({
        "schema": MISSION_SCRIPT_SCHEMA,
        "source_extension": "mfk",
        "route_class": "mission",
        "source_bytes": 128,
        "context_command_count": 4,
        "context_adaptation_count": 0,
        "context_adaptations": [],
        "context_finding_count": 0,
        "context_findings": [],
        "statement_count": 4,
        "unique_command_count": 4,
        "load_p3d_reference_count": 0,
        "mission_flow_command_count": 4,
        "vehicle_physics_command_count": 0,
        "semantic_family": "mission-script",
        "command_counts": {
            "addstage": 1,
            "closemission": 1,
            "closestage": 1,
            "selectmission": 1
        },
        "source_statements": [
            "SelectMission(\"m1\");",
            "AddStage(0);",
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
                "name": "closestage",
                "args_raw": "",
                "semantic_role": "mission-stage",
                "arguments": []
            },
            {
                "ordinal": 4,
                "name": "closemission",
                "args_raw": "",
                "semantic_role": "mission-script",
                "arguments": []
            }
        ]
    })
}

fn text(value: &Value) -> Result<String, String> {
    serde_json::to_string(value).map_err(|error| error.to_string())
}

fn set_pointer(
    value: &mut Value,
    pointer: &str,
    replacement: Value,
) -> Result<(), String> {
    let Some(target) = value.pointer_mut(pointer) else {
        return Err(format!("missing synthetic JSON pointer: {pointer}"));
    };
    *target = replacement;
    Ok(())
}

fn insert_top_level(
    value: &mut Value,
    field: &str,
    replacement: Value,
) -> Result<(), String> {
    let Some(object) = value.as_object_mut() else {
        return Err("synthetic mission document is not an object".to_owned());
    };
    drop(object.insert(field.to_owned(), replacement));
    Ok(())
}

fn rejected_error(
    value: &Value,
    accepted_message: &str,
) -> Result<String, String> {
    let result = preflight_mission_script(&text(value)?);
    let Err(error) = result else {
        return Err(accepted_message.to_owned());
    };
    Ok(error)
}

#[test]
fn accepts_exact_clean_v3_evidence() -> Result<(), String> {
    let evidence = preflight_mission_script(&text(&document())?)?;
    if evidence.source_bytes() != 128 || evidence.statement_count() != 4 {
        return Err("mission source summary changed".to_owned());
    }
    let invocations = evidence.invocations();
    if invocations.len() != 4 {
        return Err("typed mission invocation count changed".to_owned());
    }
    let Some(first) = invocations.first() else {
        return Err("typed mission invocation evidence is empty".to_owned());
    };
    if first.ordinal() != 1
        || first.name() != "selectmission"
        || first.args_raw() != "\"m1\""
        || first.semantic_role() != "mission-script"
        || first.arguments() != ["m1"]
    {
        return Err("typed mission invocation evidence changed".to_owned());
    }
    Ok(())
}

#[test]
fn accepts_exact_empty_inert_mission_source() -> Result<(), String> {
    let value = json!({
        "schema": MISSION_SCRIPT_SCHEMA,
        "source_extension": "mfk",
        "route_class": "mission",
        "source_bytes": 0,
        "context_command_count": 0,
        "context_adaptation_count": 0,
        "context_adaptations": [],
        "context_finding_count": 0,
        "context_findings": [],
        "statement_count": 0,
        "unique_command_count": 0,
        "load_p3d_reference_count": 0,
        "mission_flow_command_count": 0,
        "vehicle_physics_command_count": 0,
        "semantic_family": "mission-script",
        "command_counts": {},
        "source_statements": [],
        "p3d_references": [],
        "command_invocations": []
    });
    let evidence = preflight_mission_script(&text(&value)?)?;
    if evidence.source_bytes() != 0
        || evidence.statement_count() != 0
        || !evidence.invocations().is_empty()
    {
        return Err("empty inert mission evidence changed".to_owned());
    }
    Ok(())
}

#[test]
fn rejects_nonempty_source_bytes_with_zero_statements() -> Result<(), String> {
    let mut value = document();
    set_pointer(&mut value, "/statement_count", json!(0))?;
    set_pointer(&mut value, "/source_statements", json!([]))?;
    set_pointer(&mut value, "/command_invocations", json!([]))?;
    set_pointer(&mut value, "/command_counts", json!({}))?;
    set_pointer(&mut value, "/unique_command_count", json!(0))?;
    let error = rejected_error(&value, "nonempty source with zero statements was accepted")?;
    if !error.contains("inconsistent") {
        return Err(format!("unexpected source-byte error: {error}"));
    }
    Ok(())
}

#[test]
fn rejects_zero_source_bytes_with_nonempty_statements() -> Result<(), String> {
    let mut value = document();
    set_pointer(&mut value, "/source_bytes", json!(0))?;
    let error = rejected_error(&value, "contradictory source byte evidence was accepted")?;
    if !error.contains("inconsistent") {
        return Err(format!("unexpected source-byte error: {error}"));
    }
    Ok(())
}

#[test]
fn rejects_stale_schema_before_semantic_mapping() -> Result<(), String> {
    let mut value = document();
    set_pointer(
        &mut value,
        "/schema",
        Value::String("shar-schoenwald.straggler.mission-script.v2".to_owned()),
    )?;
    let error = rejected_error(&value, "stale schema was accepted")?;
    if !error.contains("schema") {
        return Err(format!("unexpected stale-schema error: {error}"));
    }
    Ok(())
}

#[test]
fn rejects_unresolved_context_findings() -> Result<(), String> {
    let mut value = document();
    set_pointer(&mut value, "/context_finding_count", json!(1))?;
    set_pointer(
        &mut value,
        "/context_findings",
        json!([{
            "ordinal": 3,
            "command": "closecondition",
            "code": "condition-close-without-open-condition"
        }]),
    )?;
    let error = rejected_error(&value, "context finding was accepted")?;
    if !error.contains("must be resolved") {
        return Err(format!("unexpected finding error: {error}"));
    }
    Ok(())
}

#[test]
fn rejects_inconsistent_context_finding_count() -> Result<(), String> {
    let mut value = document();
    set_pointer(&mut value, "/context_finding_count", json!(1))?;
    let error = rejected_error(&value, "finding-count drift was accepted")?;
    if !error.contains("finding count") {
        return Err(format!("unexpected finding-count error: {error}"));
    }
    Ok(())
}

#[test]
fn rejects_context_command_count_drift() -> Result<(), String> {
    let mut value = document();
    set_pointer(&mut value, "/context_command_count", json!(3))?;
    let error =
        rejected_error(&value, "context-command count drift was accepted")?;
    if !error.contains("context command count") {
        return Err(format!("unexpected context-count error: {error}"));
    }
    Ok(())
}

#[test]
fn rejects_command_histogram_drift() -> Result<(), String> {
    let mut value = document();
    set_pointer(&mut value, "/command_counts/addstage", json!(2))?;
    let error = rejected_error(&value, "command histogram drift was accepted")?;
    if !error.contains("histogram") {
        return Err(format!("unexpected histogram error: {error}"));
    }
    Ok(())
}

#[test]
fn rejects_nonmonotonic_command_ordinals() -> Result<(), String> {
    let mut value = document();
    set_pointer(&mut value, "/command_invocations/2/ordinal", json!(2))?;
    let error =
        rejected_error(&value, "duplicate command ordinal was accepted")?;
    if !error.contains("ordinals") {
        return Err(format!("unexpected ordinal error: {error}"));
    }
    Ok(())
}

#[test]
fn rejects_unknown_top_level_fields() -> Result<(), String> {
    let mut value = document();
    insert_top_level(&mut value, "unexpected_contract", json!(true))?;
    let error =
        rejected_error(&value, "unknown mission evidence field was accepted")?;
    if error != "normalized mission script JSON is invalid" {
        return Err(format!("unexpected unknown-field error: {error}"));
    }
    Ok(())
}

#[test]
fn accepts_horizontal_tabs_preserved_from_legacy_source() -> Result<(), String>
{
    let mut value = document();
    set_pointer(&mut value, "/source_statements/1", json!("AddStage(\t0 );"))?;
    set_pointer(&mut value, "/command_invocations/1/args_raw", json!("\t0"))?;
    set_pointer(
        &mut value,
        "/command_invocations/1/arguments/0",
        json!("0\t"),
    )?;
    drop(preflight_mission_script(&text(&value)?)?);
    Ok(())
}

#[test]
fn rejects_non_tab_control_characters_in_source_evidence() -> Result<(), String>
{
    let mut value = document();
    set_pointer(
        &mut value,
        "/source_statements/1",
        json!("AddStage(0);\u{0000}"),
    )?;
    let error =
        rejected_error(&value, "NUL-bearing mission evidence was accepted")?;
    if !error.contains("statement evidence") {
        return Err(format!("unexpected control-character error: {error}"));
    }
    Ok(())
}

fn reviewed_l2_adaptation_document() -> Value {
    let source_statements = (1..=72)
        .map(|ordinal| format!("Statement{ordinal}();"))
        .collect::<Vec<_>>();
    json!({
        "schema": MISSION_SCRIPT_SCHEMA,
        "source_extension": "mfk",
        "route_class": "mission",
        "source_bytes": 512,
        "context_command_count": 5,
        "context_adaptation_count": 1,
        "context_adaptations": [{
            "ordinal": 70,
            "command": "closecondition",
            "code": "legacy-l2-m6sdi-ignore-orphan-condition-close-v1"
        }],
        "context_finding_count": 0,
        "context_findings": [],
        "statement_count": 72,
        "unique_command_count": 7,
        "load_p3d_reference_count": 0,
        "mission_flow_command_count": 0,
        "vehicle_physics_command_count": 0,
        "semantic_family": "mission-script",
        "command_counts": {
            "addstage": 1,
            "addstagemusicchange": 1,
            "closecondition": 1,
            "closemission": 1,
            "closestage": 1,
            "selectmission": 1,
            "setstagemusicalwayson": 1
        },
        "source_statements": source_statements,
        "p3d_references": [],
        "command_invocations": [
            {
                "ordinal": 1,
                "name": "selectmission",
                "args_raw": "\"m6sd\"",
                "semantic_role": "mission-script",
                "arguments": ["m6sd"]
            },
            {
                "ordinal": 2,
                "name": "addstage",
                "args_raw": "0",
                "semantic_role": "mission-stage",
                "arguments": ["0"]
            },
            {
                "ordinal": 68,
                "name": "addstagemusicchange",
                "args_raw": "",
                "semantic_role": "mission-stage",
                "arguments": []
            },
            {
                "ordinal": 69,
                "name": "setstagemusicalwayson",
                "args_raw": "",
                "semantic_role": "mission-stage",
                "arguments": []
            },
            {
                "ordinal": 70,
                "name": "closecondition",
                "args_raw": "",
                "semantic_role": "mission-stage",
                "arguments": []
            },
            {
                "ordinal": 71,
                "name": "closestage",
                "args_raw": "",
                "semantic_role": "mission-stage",
                "arguments": []
            },
            {
                "ordinal": 72,
                "name": "closemission",
                "args_raw": "",
                "semantic_role": "mission-script",
                "arguments": []
            }
        ]
    })
}

#[test]
fn accepts_exact_reviewed_context_adaptation_as_typed_evidence()
-> Result<(), String> {
    let evidence =
        preflight_mission_script(&text(&reviewed_l2_adaptation_document())?)?;
    let [adaptation] = evidence.adaptations() else {
        return Err("reviewed adaptation count changed".to_owned());
    };
    if adaptation.ordinal() != 70
        || adaptation.command() != "closecondition"
        || adaptation.code()
            != "legacy-l2-m6sdi-ignore-orphan-condition-close-v1"
    {
        return Err("typed reviewed adaptation changed".to_owned());
    }
    Ok(())
}

#[test]
fn rejects_declared_adaptation_when_reviewed_fingerprint_drifts()
-> Result<(), String> {
    let mut value = reviewed_l2_adaptation_document();
    set_pointer(
        &mut value,
        "/command_invocations/3/name",
        json!("setstagemusicnotalwayson"),
    )?;
    set_pointer(
        &mut value,
        "/command_counts",
        json!({
            "addstage": 1,
            "addstagemusicchange": 1,
            "closecondition": 1,
            "closemission": 1,
            "closestage": 1,
            "selectmission": 1,
            "setstagemusicnotalwayson": 1
        }),
    )?;
    let error = rejected_error(
        &value,
        "drifted reviewed adaptation fingerprint was accepted",
    )?;
    if !error.contains("not reviewed") {
        return Err(format!("unexpected adaptation-drift error: {error}"));
    }
    Ok(())
}

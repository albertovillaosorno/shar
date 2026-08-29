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
//   - Normalized vehicle-tuning semantic preflight unit tests.
// - Must-Not:
//   - Read game source files or exercise Unreal mutation.
// - Allows:
//   - Synthetic exact-schema config-script command evidence.
// - Split-When:
//   - Tuning compilation gains a separate tested lifecycle.
// - Merge-When:
//   - Vehicle tuning evidence preflight loses independent policy.
// - Summary:
//   - Vehicle tuning semantic preflight tests.
// - Description:
//   - Proves normalized tuning evidence is reproducible and lossless.
// - Usage:
//   - Included only by the owning package domain module under cfg(test).
// - Defaults:
//   - Forged, stale, or malformed evidence fails closed.
//

//! Vehicle tuning semantic preflight tests.

use serde_json::{Value, json};

use crate::preflight_vehicle_tuning;

const SET_DRIVER: &str = concat!("set", "driver");
const SET_MASS: &str = concat!("set", "mass");
const SET_SOUND: &str = concat!("set", "sound");
const SET_STEERING: &str = concat!("set", "steering");

fn document() -> Value {
    json!({
        "schema": "shar-schoenwald.straggler.config-script.v2",
        "source_extension": "con",
        "route_class": "vehicle-config",
        "source_bytes": 82,
        "context_command_count": 0,
        "context_adaptation_count": 0,
        "context_adaptations": [],
        "context_finding_count": 0,
        "context_findings": [],
        "statement_count": 4,
        "unique_command_count": 3,
        "load_p3d_reference_count": 0,
        "mission_flow_command_count": 0,
        "vehicle_physics_command_count": 2,
        "semantic_family": "vehicle-config-script",
        "command_counts": std::collections::BTreeMap::from([
            (SET_DRIVER, 1),
            (SET_MASS, 2),
            (SET_SOUND, 1),
        ]),
        "source_statements": [
            "SetMass(1.0);",
            "SetMass(1.0); // preserve duplicate",
            "SetSound(\"car\");",
            "SetDriver(\"homer\");"
        ],
        "p3d_references": [],
        "command_invocations": [
            {
                "ordinal": 1,
                "name": SET_MASS,
                "args_raw": "1.0",
                "semantic_role": "vehicle-physics",
                "arguments": ["1.0"]
            },
            {
                "ordinal": 2,
                "name": SET_MASS,
                "args_raw": "1.0",
                "semantic_role": "vehicle-physics",
                "arguments": ["1.0"]
            },
            {
                "ordinal": 3,
                "name": SET_SOUND,
                "args_raw": "\"car\"",
                "semantic_role": "vehicle-sound",
                "arguments": ["car"]
            },
            {
                "ordinal": 4,
                "name": SET_DRIVER,
                "args_raw": "\"homer\"",
                "semantic_role": "vehicle-config",
                "arguments": ["homer"]
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
        return Err("synthetic tuning document is not an object".to_owned());
    };
    drop(object.insert(field.to_owned(), replacement));
    Ok(())
}

fn rejected(value: &Value, message: &str) -> Result<String, String> {
    match preflight_vehicle_tuning(&text(value)?) {
        Ok(_) => Err(message.to_owned()),
        Err(error) => Ok(error),
    }
}

#[test]
// jig-ignore-next-line: long identifier
fn preserves_authored_order_duplicates_and_numeric_lexemes() -> Result<(), String> {
    let evidence = preflight_vehicle_tuning(&text(&document())?)?;
    if evidence.route_class() != "vehicle-config"
        || evidence.source_statements().len() != 4
        || evidence.invocations().len() != 4
    {
        return Err("typed vehicle tuning evidence changed shape".to_owned());
    }
    let first = evidence
        .invocations()
        .first()
        .ok_or("missing first tuning invocation")?;
    let second = evidence
        .invocations()
        .get(1)
        .ok_or("missing duplicate tuning invocation")?;
    if first.ordinal() != 1
        || first.name() != SET_MASS
        || first.args_raw() != "1.0"
        || first.arguments() != ["1.0"]
        || second.ordinal() != 2
        || second.name() != first.name()
        || second.args_raw() != first.args_raw()
        || second.arguments() != first.arguments()
    {
        return Err("authored tuning command evidence changed".to_owned());
    }
    Ok(())
}

#[test]
// jig-ignore-next-line: long identifier
fn accepts_mission_route_provenance_without_reclassifying_tuning() -> Result<(), String> {
    let mut value = document();
    set_pointer(&mut value, "/route_class", json!("mission"))?;
    let evidence = preflight_vehicle_tuning(&text(&value)?)?;
    if evidence.route_class() != "mission" {
        return Err("mission route provenance was normalized away".to_owned());
    }
    Ok(())
}

#[test]
// jig-ignore-next-line: long identifier
fn accepts_unparsed_statement_with_gapped_invocation_ordinals() -> Result<(), String> {
    let mut value = document();
    set_pointer(&mut value, "/statement_count", json!(5))?;
    set_pointer(
        &mut value,
        "/source_statements",
        json!([
            "SetMass(1.0);",
            "legacy opaque statement",
            "SetMass(1.0); // preserve duplicate",
            "SetSound(\"car\");",
            "SetDriver(\"homer\");"
        ]),
    )?;
    for (index, ordinal) in [1, 3, 4, 5].into_iter().enumerate() {
        set_pointer(
            &mut value,
            &format!("/command_invocations/{index}/ordinal"),
            json!(ordinal),
        )?;
    }
    let evidence = preflight_vehicle_tuning(&text(&value)?)?;
    if evidence.source_statements().len() != 5
        || !matches!(
            evidence.invocations().get(1),
            Some(value) if value.ordinal() == 3
        )
    {
        return Err(
            "opaque tuning statement or ordinal gap was lost".to_owned(),
        );
    }
    Ok(())
}

#[test]
fn rejects_unknown_top_level_fields() -> Result<(), String> {
    let mut value = document();
    insert_top_level(&mut value, "future_field", json!(true))?;
    let error = rejected(&value, "unknown tuning field was accepted")?;
    if !error.contains("JSON is invalid") {
        return Err(format!("unexpected unknown-field error: {error}"));
    }
    Ok(())
}

#[test]
fn rejects_stale_schema_and_route_identity() -> Result<(), String> {
    let mut stale = document();
    set_pointer(
        &mut stale,
        "/schema",
        json!("shar-schoenwald.straggler.config-script.v1"),
    )?;
    let schema = rejected(&stale, "stale tuning schema was accepted")?;
    if !schema.contains("schema is not supported") {
        return Err(format!("unexpected stale-schema error: {schema}"));
    }
    let mut route = document();
    set_pointer(&mut route, "/route_class", json!("frontend-ui"))?;
    let route = rejected(&route, "unrelated tuning route was accepted")?;
    if !route.contains("routing identity") {
        return Err(format!("unexpected route error: {route}"));
    }
    Ok(())
}

#[test]
fn rejects_mission_context_evidence_in_config_script() -> Result<(), String> {
    let mut value = document();
    set_pointer(&mut value, "/context_adaptation_count", json!(1))?;
    set_pointer(
        &mut value,
        "/context_adaptations",
        json!([{
            "ordinal": 1,
            "command": SET_MASS,
            "code": "unexpected"
        }]),
    )?;
    let error = rejected(&value, "tuning context adaptation was accepted")?;
    if !error.contains("mission context evidence") {
        return Err(format!("unexpected context error: {error}"));
    }
    Ok(())
}

#[test]
fn rejects_forged_summary_and_histogram() -> Result<(), String> {
    let mut summary = document();
    set_pointer(&mut summary, "/vehicle_physics_command_count", json!(1))?;
    let summary = rejected(&summary, "forged tuning summary was accepted")?;
    if !summary.contains("summary is not reproducible") {
        return Err(format!("unexpected summary error: {summary}"));
    }
    let mut histogram = document();
    set_pointer(
        &mut histogram,
        &format!("/command_counts/{SET_MASS}"),
        json!(1),
    )?;
    let histogram =
        rejected(&histogram, "forged tuning histogram was accepted")?;
    if !histogram.contains("histogram") {
        return Err(format!("unexpected histogram error: {histogram}"));
    }
    Ok(())
}

#[test]
fn rejects_nonreproducible_role_and_arguments() -> Result<(), String> {
    let mut role = document();
    set_pointer(
        &mut role,
        "/command_invocations/0/semantic_role",
        json!("vehicle-config"),
    )?;
    let role = rejected(&role, "forged tuning semantic role was accepted")?;
    if !role.contains("semantic role is not reproducible") {
        return Err(format!("unexpected role error: {role}"));
    }
    let mut argument = document();
    set_pointer(
        &mut argument,
        "/command_invocations/0/args_raw",
        json!("1"),
    )?;
    let argument = rejected(&argument, "forged tuning argument was accepted")?;
    if !argument.contains("invocation evidence is not reproducible") {
        return Err(format!("unexpected replay error: {argument}"));
    }
    Ok(())
}

#[test]
// jig-ignore-next-line: long identifier
fn accepts_comment_only_source_bytes_without_statements() -> Result<(), String> {
    let value = json!({
        "schema": "shar-schoenwald.straggler.config-script.v2",
        "source_extension": "con",
        "route_class": "vehicle-config",
        "source_bytes": 18,
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
        "semantic_family": "vehicle-config-script",
        "command_counts": {},
        "source_statements": [],
        "p3d_references": [],
        "command_invocations": []
    });
    let evidence = preflight_vehicle_tuning(&text(&value)?)?;
    if evidence.source_bytes() != 18
        || !evidence.source_statements().is_empty()
        || !evidence.invocations().is_empty()
    {
        return Err("comment-only tuning source evidence changed".to_owned());
    }
    Ok(())
}

#[test]
fn keeps_physics_summary_separate_from_semantic_role() -> Result<(), String> {
    let value = json!({
        "schema": "shar-schoenwald.straggler.config-script.v2",
        "source_extension": "con",
        "route_class": "vehicle-config",
        "source_bytes": 24,
        "context_command_count": 0,
        "context_adaptation_count": 0,
        "context_adaptations": [],
        "context_finding_count": 0,
        "context_findings": [],
        "statement_count": 1,
        "unique_command_count": 1,
        "load_p3d_reference_count": 0,
        "mission_flow_command_count": 0,
        "vehicle_physics_command_count": 1,
        "semantic_family": "vehicle-config-script",
        "command_counts": std::collections::BTreeMap::from([
            (SET_STEERING, 1),
        ]),
        "source_statements": ["SetSteering(20.0);"],
        "p3d_references": [],
        "command_invocations": [{
            "ordinal": 1,
            "name": SET_STEERING,
            "args_raw": "20.0",
            "semantic_role": "vehicle-config",
            "arguments": ["20.0"]
        }]
    });
    let evidence = preflight_vehicle_tuning(&text(&value)?)?;
    let invocation = evidence
        .invocations()
        .first()
        .ok_or("missing steering tuning invocation")?;
    if invocation.semantic_role() != "vehicle-config" {
        return Err(
            "steering summary was confused with semantic role".to_owned(),
        );
    }
    Ok(())
}

#[test]
// jig-ignore-next-line: long identifier
fn reproduces_context_count_without_mission_context_evidence() -> Result<(), String> {
    let value = json!({
        "schema": "shar-schoenwald.straggler.config-script.v2",
        "source_extension": "con",
        "route_class": "mission",
        "source_bytes": 12,
        "context_command_count": 1,
        "context_adaptation_count": 0,
        "context_adaptations": [],
        "context_finding_count": 0,
        "context_findings": [],
        "statement_count": 1,
        "unique_command_count": 1,
        "load_p3d_reference_count": 0,
        "mission_flow_command_count": 1,
        "vehicle_physics_command_count": 0,
        "semantic_family": "vehicle-config-script",
        "command_counts": {"addstage": 1},
        "source_statements": ["AddStage(0);"],
        "p3d_references": [],
        "command_invocations": [{
            "ordinal": 1,
            "name": "addstage",
            "args_raw": "0",
            "semantic_role": "vehicle-config",
            "arguments": ["0"]
        }]
    });
    let evidence = preflight_vehicle_tuning(&text(&value)?)?;
    if evidence.invocations().len() != 1 {
        return Err("config-script context command was discarded".to_owned());
    }
    Ok(())
}

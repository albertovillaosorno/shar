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
//   - Contextual vehicle-tuning usage renderer regressions.
// - Must-Not:
//   - Read game files, repair authored paths, or map tuning setters.
// - Allows:
//   - Synthetic mission evidence and exact package-backed lookup fixtures.
// - Split-When:
//   - Usage rendering gains an independent wire protocol.
// - Merge-When:
//   - Another test module owns the identical renderer contract.
// - Summary:
//   - Vehicle-tuning usage renderer tests.
// - Description:
//   - Proves source scope and tuning resolution survive the full semantic path.
// - Usage:
//   - Included only by the owning local adapter under cfg(test).
// - Defaults:
//   - Missing tuning sources remain explicit JSON null values.
//

//! Contextual vehicle-tuning usage renderer regressions.

use serde_json::{Value, json};

use super::{VEHICLE_TUNING_USAGE_SCHEMA, render_vehicle_tuning_usage_report};
use crate::domain::{
    MissionReferenceCatalog, VehicleTuningSourceCatalog,
    compile_mission_scope_graphs, preflight_vehicle_tuning_usages,
};
use crate::preflight_mission_script;

fn mission_with_stage_vehicle(con_file: &str) -> Result<String, String> {
    let invocation = |ordinal, name: &str, args_raw: &str, role: &str,
                      arguments: Value| {
        json!({
            "ordinal": ordinal,
            "name": name,
            "args_raw": args_raw,
            "semantic_role": role,
            "arguments": arguments,
        })
    };
    let con_raw = format!("\"car_a\",\"start\",\"chase\",\"{con_file}\"");
    let rows = json!([
        invocation(
            1,
            "selectmission",
            "\"m1\"",
            "mission-script",
            json!(["m1"]),
        ),
        invocation(2, "addstage", "0", "mission-stage", json!(["0"])),
        invocation(
            3,
            "addobjective",
            "\"goto\",\"both\"",
            "mission-objective",
            json!(["goto", "both"]),
        ),
        invocation(
            4,
            "addstagevehicle",
            &con_raw,
            "mission-stage",
            json!(["car_a", "start", "chase", con_file]),
        ),
        invocation(5, "closeobjective", "", "mission-objective", json!([])),
        invocation(6, "closestage", "", "mission-stage", json!([])),
        invocation(7, "closemission", "", "mission-script", json!([])),
    ]);
    let statements = rows
        .as_array()
        .ok_or("mission fixture invocations are not an array")?
        .iter()
        .map(|row| {
            let name = row
                .get("name")
                .and_then(Value::as_str)
                .unwrap_or_default();
            let args = row
                .get("args_raw")
                .and_then(Value::as_str)
                .unwrap_or_default();
            format!("{name}({args});")
        })
        .collect::<Vec<_>>();
    let value = json!({
        "schema": "shar-schoenwald.straggler.mission-script.v3",
        "source_extension": "mfk",
        "route_class": "mission",
        "source_bytes": 256,
        "context_command_count": 6,
        "context_adaptation_count": 0,
        "context_adaptations": [],
        "context_finding_count": 0,
        "context_findings": [],
        "statement_count": 7,
        "unique_command_count": 7,
        "load_p3d_reference_count": 0,
        "mission_flow_command_count": 7,
        "vehicle_physics_command_count": 0,
        "semantic_family": "mission-script",
        "command_counts": {
            "addobjective": 1,
            "addstage": 1,
            "addstagevehicle": 1,
            "closemission": 1,
            "closeobjective": 1,
            "closestage": 1,
            "selectmission": 1
        },
        "source_statements": statements,
        "p3d_references": [],
        "command_invocations": rows,
    });
    serde_json::to_string(&value).map_err(|error| error.to_string())
}

fn render(con_file: &str, tuning_file: &str) -> Result<String, String> {
    let mission = preflight_mission_script(
        &mission_with_stage_vehicle(con_file)?,
    )?;
    let scopes = compile_mission_scope_graphs(&mission)?;
    let vehicles = MissionReferenceCatalog::from_vehicle_entries_for_tests(&[(
        "car_a",
        "vehicle-car-a",
        "cars/traffic-vehicles/car-a",
    )]);
    let tuning = VehicleTuningSourceCatalog::from_entries_for_tests(&[(
        tuning_file,
        "tuning-source-a",
        "tuning-package-a",
        "vehicle-tuning/mission/level-01",
    )])?;
    let report = preflight_vehicle_tuning_usages(
        "mission-source-a",
        &scopes,
        &vehicles,
        &tuning,
    )?;
    render_vehicle_tuning_usage_report(&report)
        .map_err(|error| error.to_string())
}

#[test]
fn renders_objective_owned_usage_with_exact_source_bindings()
-> Result<(), String> {
    let con_file = r"Missions\level01\M1race.con";
    let rendered = render(con_file, con_file)?;
    let value = serde_json::from_str::<Value>(rendered.trim_end())
        .map_err(|error| error.to_string())?;
    if value.get("schema").and_then(Value::as_str)
            != Some(VEHICLE_TUNING_USAGE_SCHEMA)
        || value.get("mission_source_id").and_then(Value::as_str)
            != Some("mission-source-a")
        || value.get("source_ordinal").and_then(Value::as_u64) != Some(4)
        || value.get("scope").and_then(Value::as_str) != Some("objective")
        || value
            .get("owner_objective_source_ordinal")
            .and_then(Value::as_u64)
            != Some(3)
        || value.get("con_file").and_then(Value::as_str) != Some(con_file)
        || value
            .pointer("/vehicle/package_id")
            .and_then(Value::as_str)
            != Some("vehicle-car-a")
        || value
            .pointer("/tuning_source/source_id")
            .and_then(Value::as_str)
            != Some("tuning-source-a")
    {
        return Err("contextual tuning usage lost exact provenance".to_owned());
    }
    if !rendered.ends_with('\n') || rendered.lines().count() != 1 {
        return Err(
            "contextual tuning usage is not one canonical JSONL row".to_owned(),
        );
    }
    Ok(())
}

#[test]
fn renders_missing_tuning_source_as_null_without_path_repair()
-> Result<(), String> {
    let authored = r"level05\M4chase.con";
    let rendered = render(authored, r"Missions\level05\M4chase.con")?;
    let value = serde_json::from_str::<Value>(rendered.trim_end())
        .map_err(|error| error.to_string())?;
    if value.get("con_file").and_then(Value::as_str) != Some(authored)
        || !value.get("tuning_source").is_some_and(Value::is_null)
    {
        return Err(
            "unresolved tuning usage was repaired or discarded".to_owned(),
        );
    }
    Ok(())
}

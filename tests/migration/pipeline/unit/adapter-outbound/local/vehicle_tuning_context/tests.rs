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
//   - Vehicle-tuning core renderer unit tests.
// - Must-Not:
//   - Depend on proprietary source files or Unreal Editor state.
// - Allows:
//   - Synthetic normalized config-script evidence.
// - Split-When:
//   - Renderer fixtures gain independent ownership.
// - Merge-When:
//   - Another test module owns the identical renderer contract.
// - Summary:
//   - Vehicle-tuning core renderer tests.
// - Description:
//   - Proves exact source-backed tuning preservation without interpretation.
// - Usage:
//   - Included only by the owning local adapter module under cfg(test).
// - Defaults:
//   - Serialization drift fails explicitly.
//

//! Vehicle-tuning core renderer tests.

use serde_json::json;

use super::{VEHICLE_TUNING_CORE_SCHEMA, render_vehicle_tuning_core};
use crate::domain::VehicleTuningEvidence;
use crate::preflight_vehicle_tuning;

fn normalized_tuning() -> Result<VehicleTuningEvidence, String> {
    let value = json!({
        "schema": "shar-schoenwald.straggler.config-script.v2",
        "source_extension": "con",
        "route_class": "vehicle-config",
        "source_bytes": 37,
        "context_command_count": 0,
        "context_adaptation_count": 0,
        "context_adaptations": [],
        "context_finding_count": 0,
        "context_findings": [],
        "statement_count": 2,
        "unique_command_count": 1,
        "load_p3d_reference_count": 0,
        "mission_flow_command_count": 0,
        "vehicle_physics_command_count": 1,
        "semantic_family": "vehicle-config-script",
        "command_counts": {concat!("set", "mass"): 1},
        "source_statements": [
            "opaque_token",
            concat!("Set", "Mass(1.0, car);"),
        ],
        "p3d_references": [],
        "command_invocations": [{
            "ordinal": 2,
            "name": concat!("set", "mass"),
            "args_raw": "1.0, car",
            "semantic_role": "vehicle-physics",
            "arguments": ["1.0", "car"],
        }],
    });
    let text = serde_json::to_string(&value)
        .map_err(|error| error.to_string())?;
    preflight_vehicle_tuning(&text)
}

#[test]
fn renders_exact_source_backed_tuning_core() -> Result<(), String> {
    let evidence = normalized_tuning()?;
    let rendered = render_vehicle_tuning_core("tuning-source-01", &evidence)
        .map_err(|error| error.to_string())?;
    let value = serde_json::from_str::<serde_json::Value>(rendered.trim_end())
        .map_err(|error| error.to_string())?;
    let object = value
        .as_object()
        .ok_or("vehicle tuning core is not an object")?;
    let statements = object
        .get("source_statements")
        .and_then(serde_json::Value::as_array)
        .ok_or("vehicle tuning core lost source statements")?;
    let command = object
        .get("commands")
        .and_then(serde_json::Value::as_array)
        .and_then(|commands| commands.first())
        .and_then(serde_json::Value::as_object)
        .ok_or("vehicle tuning core lost first command")?;
    let arguments = command
        .get("arguments")
        .and_then(serde_json::Value::as_array)
        .ok_or("vehicle tuning core lost command arguments")?;
    if object.get("schema").and_then(serde_json::Value::as_str)
            != Some(VEHICLE_TUNING_CORE_SCHEMA)
        || object.get("source_id").and_then(serde_json::Value::as_str)
            != Some("tuning-source-01")
        || object.get("route_class").and_then(serde_json::Value::as_str)
            != Some("vehicle-config")
        || object.get("source_bytes").and_then(serde_json::Value::as_u64)
            != Some(37)
        || statements.first().and_then(serde_json::Value::as_str)
            != Some("opaque_token")
        || command.get("ordinal").and_then(serde_json::Value::as_u64)
            != Some(2)
        || command.get("args_raw").and_then(serde_json::Value::as_str)
            != Some("1.0, car")
        || arguments.first().and_then(serde_json::Value::as_str) != Some("1.0")
    {
        return Err("vehicle tuning core lost source evidence".to_owned());
    }
    if !rendered.ends_with('\n') || rendered.lines().count() != 1 {
        return Err(
            "vehicle tuning core is not one canonical JSONL row".to_owned(),
        );
    }
    Ok(())
}

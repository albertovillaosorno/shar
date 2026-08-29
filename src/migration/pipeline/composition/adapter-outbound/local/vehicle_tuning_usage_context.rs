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
//   - Canonical vehicle-tuning usage JSONL serialization.
// - Must-Not:
//   - Resolve source identities, infer profile ownership, or emit Unreal
//     assets.
// - Allows:
//   - Serialize already-bound contextual tuning provenance exactly once.
// - Split-When:
//   - Usage publication gains an independent transaction lifecycle.
// - Merge-When:
//   - Another renderer owns the identical usage schema.
// - Summary:
//   - Vehicle-tuning usage renderer.
// - Description:
//   - Serializes reviewed mission-to-tuning applications deterministically.
// - Usage:
//   - Called after contextual tuning preflight succeeds.
// - Defaults:
//   - JSON serialization failures fail closed.
//

//! Canonical vehicle-tuning usage JSONL serialization.

use serde_json::{Value, json};

use crate::domain::{
    PipelineError, PipelineOutcome, VehicleTuningSourceReference,
    VehicleTuningUsageBinding, VehicleTuningUsageReport,
};

pub(super) const VEHICLE_TUNING_USAGE_SCHEMA: &str =
    "shar-schoenwald.vehicle-tuning-usage.v1";

/// Render one mission source's contextual tuning applications in source order.
///
/// # Errors
///
/// Returns an error when canonical JSON serialization fails.
pub(super) fn render_vehicle_tuning_usage_report(
    report: &VehicleTuningUsageReport,
) -> PipelineOutcome<String> {
    let mut output = String::new();
    for binding in report.bindings() {
        let value = usage_json(binding);
        let row = serde_json::to_string(&value).map_err(|_error| {
            PipelineError::new("vehicle tuning usage JSON serialization failed")
        })?;
        output.push_str(&row);
        output.push('\n');
    }
    Ok(output)
}

fn usage_json(binding: &VehicleTuningUsageBinding) -> Value {
    let tuning_source = binding
        .tuning_source()
        .map_or(Value::Null, tuning_source_json);
    json!({
        "command": binding.command(),
        "con_file": binding.con_file(),
        "mission_source_id": binding.mission_source_id(),
        "owner_mission_id": binding.owner_mission_id(),
        "owner_objective_source_ordinal":
            binding.owner_objective_source_ordinal(),
        "owner_stage_sequence_ordinal": binding.owner_stage_sequence_ordinal(),
        "schema": VEHICLE_TUNING_USAGE_SCHEMA,
        "scope": binding.scope().as_str(),
        "source_ordinal": binding.source_ordinal(),
        "tuning_source": tuning_source,
        "vehicle": {
            "package_id": binding.vehicle().package_id(),
            "package_subcategory": binding.vehicle().package_subcategory(),
            "source_id": binding.vehicle().source_id(),
        },
    })
}

fn tuning_source_json(reference: &VehicleTuningSourceReference) -> Value {
    json!({
        "package_id": reference.package_id(),
        "package_subcategory": reference.package_subcategory(),
        "source_id": reference.source_id(),
    })
}

#[cfg(test)]
// jig-ignore-next-line: exact test module path is indivisible
#[path = "../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/vehicle_tuning_usage_context/tests.rs"]
mod tests;

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
//   - Canonical source-backed vehicle-tuning core serialization.
// - Must-Not:
//   - Interpret gameplay units, map setters to Unreal fields, or emit assets.
// - Allows:
//   - Serialize preflighted tuning statements, command evidence, and exact
//     physical-vehicle package provenance when it resolves unambiguously.
// - Split-When:
//   - Native tuning application gains an independent lifecycle.
// - Merge-When:
//   - Another renderer owns the identical tuning-core schema.
// - Summary:
//   - Vehicle-tuning core renderer.
// - Description:
//   - Preserves validated tuning evidence for later semantic compilation.
// - Usage:
//   - Called after vehicle-tuning semantic preflight succeeds.
// - Defaults:
//   - Invalid source identities and serialization failures fail closed.
//

//! Canonical source-backed vehicle-tuning core serialization.

use serde_json::{Value, json};

use crate::domain::package::vehicle_tuning::{
    VehicleTuningCommandInvocation, VehicleTuningEvidence,
};
use crate::domain::MissionVehicleCatalogReference;
use crate::domain::{PipelineError, PipelineOutcome};

pub(super) const VEHICLE_TUNING_CORE_SCHEMA: &str =
    "shar-schoenwald.vehicle-tuning-core.v2";

/// Render one verified vehicle-tuning source as one canonical JSONL row.
///
/// # Errors
///
/// Returns an error when the source identity is not canonical or JSON
/// serialization fails.
pub(super) fn render_vehicle_tuning_core(
    source_id: &str,
    evidence: &VehicleTuningEvidence,
    physical_vehicle: Option<&MissionVehicleCatalogReference>,
) -> PipelineOutcome<String> {
    validate_source_id(source_id)?;
    let physical_vehicle = physical_vehicle.map_or(Value::Null, |vehicle| {
        json!({
            "package_id": vehicle.package_id(),
            "package_subcategory": vehicle.package_subcategory(),
            "source_id": vehicle.source_id(),
        })
    });
    let value = json!({
        "commands": evidence
            .invocations()
            .iter()
            .map(command_json)
            .collect::<Vec<_>>(),
        "physical_vehicle": physical_vehicle,
        "route_class": evidence.route_class(),
        "schema": VEHICLE_TUNING_CORE_SCHEMA,
        "source_bytes": evidence.source_bytes(),
        "source_id": source_id,
        "source_statements": evidence.source_statements(),
    });
    let mut text = serde_json::to_string(&value).map_err(|_error| {
        PipelineError::new("vehicle tuning core JSON serialization failed")
    })?;
    text.push('\n');
    Ok(text)
}

fn command_json(invocation: &VehicleTuningCommandInvocation) -> Value {
    json!({
        "arguments": invocation.arguments(),
        "args_raw": invocation.args_raw(),
        "name": invocation.name(),
        "ordinal": invocation.ordinal(),
        "semantic_role": invocation.semantic_role(),
    })
}

fn validate_source_id(source_id: &str) -> PipelineOutcome<()> {
    let bytes = source_id.as_bytes();
    if bytes.is_empty()
        || !bytes.first().is_some_and(u8::is_ascii_alphanumeric)
        || !bytes.last().is_some_and(u8::is_ascii_alphanumeric)
        || bytes.windows(2).any(|pair| pair == b"--")
        || !bytes.iter().copied().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-'
        })
    {
        return Err(PipelineError::new(
            "vehicle tuning core source identity is not canonical",
        ));
    }
    Ok(())
}

#[cfg(test)]
// jig-ignore-next-line: exact test module path is indivisible
#[path = "../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/vehicle_tuning_context/tests.rs"]
mod tests;

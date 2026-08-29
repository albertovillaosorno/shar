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
//   - Vehicle-tuning JSON wire decoding before semantic domain validation.
// - Must-Not:
//   - Interpret gameplay units, map tuning fields, or emit Unreal assets.
// - Allows:
//   - Decode exact normalized config-script evidence into pure domain records.
// - Split-When:
//   - Config-script wire schemas gain independently versioned intake
//     lifecycles.
// - Merge-When:
//   - Another composition module owns the identical tuning JSON boundary.
// - Summary:
//   - Vehicle-tuning JSON intake composition.
// - Description:
//   - Keeps serde decoding outside vehicle tuning semantic domain validation.
// - Usage:
//   - Used by prepare-unreal tuning intake and regression fixtures.
// - Defaults:
//   - Malformed or unknown JSON fields fail before semantic validation.
//

//! Vehicle-tuning JSON intake composition.

use std::collections::BTreeMap;

use serde::Deserialize;

use crate::domain::package::vehicle_tuning::{
    VehicleTuningCommandDocument, VehicleTuningDocument, VehicleTuningEvidence,
    preflight_vehicle_tuning_document,
};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct VehicleTuningWire {
    schema: String,
    source_extension: String,
    route_class: String,
    source_bytes: u64,
    context_command_count: usize,
    context_adaptation_count: usize,
    context_adaptations: Vec<serde_json::Value>,
    context_finding_count: usize,
    context_findings: Vec<serde_json::Value>,
    statement_count: usize,
    unique_command_count: usize,
    load_p3d_reference_count: usize,
    mission_flow_command_count: usize,
    vehicle_physics_command_count: usize,
    semantic_family: String,
    command_counts: BTreeMap<String, usize>,
    source_statements: Vec<String>,
    p3d_references: Vec<String>,
    command_invocations: Vec<VehicleTuningCommandWire>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct VehicleTuningCommandWire {
    ordinal: usize,
    name: String,
    args_raw: String,
    semantic_role: String,
    arguments: Vec<String>,
}

impl From<VehicleTuningWire> for VehicleTuningDocument {
    fn from(value: VehicleTuningWire) -> Self {
        Self {
            schema: value.schema,
            source_extension: value.source_extension,
            route_class: value.route_class,
            source_bytes: value.source_bytes,
            context_command_count: value.context_command_count,
            context_adaptation_count: value.context_adaptation_count,
            context_adaptations: value
                .context_adaptations
                .into_iter()
                .map(|_value| ())
                .collect(),
            context_finding_count: value.context_finding_count,
            context_findings: value
                .context_findings
                .into_iter()
                .map(|_value| ())
                .collect(),
            statement_count: value.statement_count,
            unique_command_count: value.unique_command_count,
            load_p3d_reference_count: value.load_p3d_reference_count,
            mission_flow_command_count: value.mission_flow_command_count,
            vehicle_physics_command_count: value.vehicle_physics_command_count,
            semantic_family: value.semantic_family,
            command_counts: value.command_counts,
            source_statements: value.source_statements,
            p3d_references: value.p3d_references,
            command_invocations: value
                .command_invocations
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }
}

impl From<VehicleTuningCommandWire> for VehicleTuningCommandDocument {
    fn from(value: VehicleTuningCommandWire) -> Self {
        Self {
            ordinal: value.ordinal,
            name: value.name,
            args_raw: value.args_raw,
            semantic_role: value.semantic_role,
            arguments: value.arguments,
        }
    }
}

/// Validate one normalized vehicle-tuning JSON document before compilation.
///
/// # Errors
///
/// Returns an error when JSON decoding or structural evidence validation fails.
pub fn preflight_vehicle_tuning(
    json: &str,
) -> Result<VehicleTuningEvidence, String> {
    let wire =
        serde_json::from_str::<VehicleTuningWire>(json).map_err(|_error| {
            "normalized vehicle tuning JSON is invalid".to_owned()
        })?;
    preflight_vehicle_tuning_document(wire.into())
}

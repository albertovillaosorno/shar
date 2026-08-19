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
//   - Mission-script JSON wire decoding before semantic domain validation.
// - Must-Not:
//   - Own mission gameplay semantics or emit Unreal assets.
// - Allows:
//   - Decode exact normalized mission evidence into pure domain records.
// - Split-When:
//   - Mission wire schemas gain independently versioned intake lifecycles.
// - Merge-When:
//   - Another composition module owns the identical mission JSON boundary.
// - Summary:
//   - Mission-script JSON intake composition.
// - Description:
//   - Keeps serde decoding outside mission semantic domain validation.
// - Usage:
//   - Used by prepare-unreal mission intake and mission regression fixtures.
// - Defaults:
//   - Malformed or unknown JSON fields fail before semantic validation.
//

//! Mission-script JSON intake composition.

use std::collections::BTreeMap;

use serde::Deserialize;

use crate::domain::package::mission_script::{
    MissionCommandDocument, MissionContextAdaptationDocument,
    MissionContextFinding, MissionScriptDocument, MissionScriptEvidence,
    preflight_mission_script_document,
};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct MissionScriptWire {
    schema: String,
    source_extension: String,
    route_class: String,
    source_bytes: u64,
    context_command_count: usize,
    context_adaptation_count: usize,
    context_adaptations: Vec<MissionContextAdaptationWire>,
    context_finding_count: usize,
    context_findings: Vec<MissionContextFindingWire>,
    statement_count: usize,
    unique_command_count: usize,
    load_p3d_reference_count: usize,
    mission_flow_command_count: usize,
    vehicle_physics_command_count: usize,
    semantic_family: String,
    command_counts: BTreeMap<String, usize>,
    source_statements: Vec<String>,
    p3d_references: Vec<String>,
    command_invocations: Vec<MissionCommandWire>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct MissionContextFindingWire {
    ordinal: usize,
    command: String,
    code: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct MissionContextAdaptationWire {
    ordinal: usize,
    command: String,
    code: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct MissionCommandWire {
    ordinal: usize,
    name: String,
    args_raw: String,
    semantic_role: String,
    arguments: Vec<String>,
}

impl From<MissionScriptWire> for MissionScriptDocument {
    fn from(value: MissionScriptWire) -> Self {
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
                .map(Into::into)
                .collect(),
            context_finding_count: value.context_finding_count,
            context_findings: value
                .context_findings
                .into_iter()
                .map(Into::into)
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

impl From<MissionContextFindingWire> for MissionContextFinding {
    fn from(value: MissionContextFindingWire) -> Self {
        Self {
            ordinal: value.ordinal,
            command: value.command,
            code: value.code,
        }
    }
}

impl From<MissionContextAdaptationWire> for MissionContextAdaptationDocument {
    fn from(value: MissionContextAdaptationWire) -> Self {
        Self {
            ordinal: value.ordinal,
            command: value.command,
            code: value.code,
        }
    }
}

impl From<MissionCommandWire> for MissionCommandDocument {
    fn from(value: MissionCommandWire) -> Self {
        Self {
            ordinal: value.ordinal,
            name: value.name,
            args_raw: value.args_raw,
            semantic_role: value.semantic_role,
            arguments: value.arguments,
        }
    }
}

/// Validate one normalized mission-script JSON document before compilation.
///
/// # Errors
///
/// Returns an error when JSON decoding or semantic evidence validation fails.
pub fn preflight_mission_script(
    json: &str,
) -> Result<MissionScriptEvidence, String> {
    let wire = serde_json::from_str::<MissionScriptWire>(json)
        .map_err(|_error| {
            "normalized mission script JSON is invalid".to_owned()
        })?;
    preflight_mission_script_document(wire.into())
}

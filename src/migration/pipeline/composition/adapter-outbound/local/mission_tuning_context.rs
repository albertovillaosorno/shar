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
//   - Lossless join and canonical serialization of mission-local vehicle-tuning
//     evidence already typed by package-domain preflights.
// - Must-Not:
//   - Name opaque tuning fields, assign gameplay units, map source values to
//     runtime fields, or emit Unreal assets.
// - Allows:
//   - Bind exact source arguments, source ownership, and physical vehicle
//     package provenance into deterministic JSONL rows.
// - Split-When:
//   - Native tuning application gains an independent lifecycle.
// - Merge-When:
//   - Final vehicle tuning compilation owns this exact preflight join.
// - Summary:
//   - Mission-local vehicle-tuning evidence renderer.
// - Description:
//   - Preserves reviewed SetCarAttributes and stage AI/catch-up tuples without
//     inventing gameplay interpretation.
// - Usage:
//   - Runs after mission scope, stage, participant, and vehicle-attribute
//     preflights succeed for the same stable source snapshot.
// - Defaults:
//   - Missing, duplicate, mismatched, or unowned semantic rows fail closed.
//

//! Mission-local vehicle-tuning evidence join and canonical JSONL renderer.

use std::collections::BTreeSet;

use serde_json::json;

use crate::domain::{
    MissionParticipantReference, MissionParticipantRole, MissionReferenceReport,
    MissionScopeReport, MissionStageDirective, MissionStageSemanticReport,
    MissionVehicleAttributeReport, MissionVehicleCatalogReference,
    MissionVehicleReference, PipelineError, PipelineOutcome,
};

pub(super) const MISSION_TUNING_SCHEMA: &str =
    "shar-schoenwald.mission-tuning.v1";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum BindingScope {
    Unscoped,
    Stage,
    Objective,
}

impl BindingScope {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Unscoped => "unscoped",
            Self::Stage => "stage",
            Self::Objective => "objective",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct MissionTuningBinding {
    command: String,
    arguments: Vec<String>,
    source_ordinal: usize,
    scope: BindingScope,
    owner_mission_id: Option<String>,
    owner_stage_source_ordinal: Option<usize>,
    owner_stage_sequence_ordinal: Option<usize>,
    owner_objective_source_ordinal: Option<usize>,
    vehicle_id: String,
    vehicle: MissionVehicleCatalogReference,
}

/// Join and render every reviewed mission-local vehicle-tuning tuple.
///
/// # Errors
///
/// Returns an error when the source id is noncanonical or when scope, stage,
/// participant, vehicle-attribute, or package-backed vehicle evidence drifts.
pub(super) fn render_mission_tuning(
    source_id: &str,
    scopes: &MissionScopeReport,
    stages: &MissionStageSemanticReport,
    references: &MissionReferenceReport,
    attributes: &MissionVehicleAttributeReport,
) -> PipelineOutcome<String> {
    validate_source_id(source_id)?;
    let mut bindings = Vec::new();
    append_vehicle_attributes(scopes, attributes, &mut bindings)?;
    append_stage_tuning(scopes, stages, references, &mut bindings)?;
    bindings.sort_by_key(|binding| binding.source_ordinal);
    let mut ordinals = BTreeSet::new();
    let mut output = String::new();
    for binding in bindings {
        if !ordinals.insert(binding.source_ordinal) {
            return Err(PipelineError::new(
                "mission tuning source ordinal is duplicated",
            ));
        }
        let value = json!({
            "arguments": binding.arguments,
            "command": binding.command,
            "mission_source_id": source_id,
            "owner_mission_id": binding.owner_mission_id,
            "owner_objective_source_ordinal":
                binding.owner_objective_source_ordinal,
            "owner_stage_sequence_ordinal":
                binding.owner_stage_sequence_ordinal,
            "owner_stage_source_ordinal": binding.owner_stage_source_ordinal,
            "schema": MISSION_TUNING_SCHEMA,
            "scope": binding.scope.as_str(),
            "source_ordinal": binding.source_ordinal,
            "vehicle": {
                "package_id": binding.vehicle.package_id(),
                "package_subcategory": binding.vehicle.package_subcategory(),
                "source_id": binding.vehicle.source_id(),
            },
            "vehicle_id": binding.vehicle_id,
        });
        let row = serde_json::to_string(&value).map_err(|_error| {
            PipelineError::new("mission tuning JSON serialization failed")
        })?;
        output.push_str(&row);
        output.push('\n');
    }
    Ok(output)
}

fn append_vehicle_attributes(
    scopes: &MissionScopeReport,
    attributes: &MissionVehicleAttributeReport,
    bindings: &mut Vec<MissionTuningBinding>,
) -> PipelineOutcome<()> {
    let source_commands = scopes
        .unscoped_commands()
        .iter()
        .filter(|command| command.name() == "setcarattributes")
        .collect::<Vec<_>>();
    if source_commands.len() != attributes.bindings().len() {
        return Err(PipelineError::new(
            "mission tuning vehicle-attribute report is incomplete",
        ));
    }
    for attribute in attributes.bindings() {
        let command = source_commands
            .iter()
            .copied()
            .find(|command| {
                command.source_ordinal() == attribute.source_ordinal()
            })
            .ok_or_else(|| {
                PipelineError::new(
                    concat!(
                        "mission tuning vehicle-attribute source command ",
                        "disappeared"
                    ),
                )
            })?;
        let [vehicle_id, first, second, third, fourth] = command.arguments()
        else {
            return Err(PipelineError::new(
                "mission tuning vehicle-attribute source shape drifted",
            ));
        };
        if vehicle_id != attribute.vehicle_id()
            || [first, second, third, fourth]
                .iter()
                .map(|value| value.as_str())
                .ne(attribute.source_values().iter().map(String::as_str))
            || attribute.vehicle().source_id() != attribute.vehicle_id()
        {
            return Err(PipelineError::new(
                "mission tuning vehicle-attribute evidence drifted",
            ));
        }
        bindings.push(MissionTuningBinding {
            command: command.name().to_owned(),
            arguments: command.arguments().to_vec(),
            source_ordinal: command.source_ordinal(),
            scope: BindingScope::Unscoped,
            owner_mission_id: None,
            owner_stage_source_ordinal: None,
            owner_stage_sequence_ordinal: None,
            owner_objective_source_ordinal: None,
            vehicle_id: attribute.vehicle_id().to_owned(),
            vehicle: attribute.vehicle().clone(),
        });
    }
    Ok(())
}

fn append_stage_tuning(
    scopes: &MissionScopeReport,
    stages: &MissionStageSemanticReport,
    references: &MissionReferenceReport,
    bindings: &mut Vec<MissionTuningBinding>,
) -> PipelineOutcome<()> {
    let mut typed_stages = stages.stages().iter();
    let mut reference_missions = references.missions().iter();
    for mission in scopes.missions() {
        let reference_mission = reference_missions.next().ok_or_else(|| {
            PipelineError::new(
                "mission tuning participant report is incomplete"
            )
        })?;
        if reference_mission.mission_id() != mission.source_mission_id() {
            return Err(PipelineError::new(
                "mission tuning participant mission identity drifted",
            ));
        }
        for stage in mission.stages() {
            let typed_stage = typed_stages.next().ok_or_else(|| {
                PipelineError::new("mission tuning stage report is incomplete")
            })?;
            if typed_stage.source_ordinal() != stage.source_ordinal()
                || typed_stage.sequence_ordinal() != stage.sequence_ordinal()
            {
                return Err(PipelineError::new(
                    "mission tuning stage ownership drifted",
                ));
            }
            let mut matched_directives = BTreeSet::new();
            for command in stage
                .commands()
                .iter()
                .filter(|command| is_stage_tuning_command(command.name()))
            {
                append_stage_binding(
                    mission.source_mission_id(),
                    stage,
                    typed_stage,
                    reference_mission.participants(),
                    command.source_ordinal(),
                    command.name(),
                    command.arguments(),
                    BindingScope::Stage,
                    None,
                    &mut matched_directives,
                    bindings,
                )?;
            }
            for command in stage
                .objective()
                .commands()
                .iter()
                .filter(|command| is_stage_tuning_command(command.command()))
            {
                append_stage_binding(
                    mission.source_mission_id(),
                    stage,
                    typed_stage,
                    reference_mission.participants(),
                    command.ordinal(),
                    command.command(),
                    command.arguments(),
                    BindingScope::Objective,
                    Some(stage.objective().binding().ordinal()),
                    &mut matched_directives,
                    bindings,
                )?;
            }
            let typed_tuning_ordinals = typed_stage
                .directives()
                .iter()
                .filter_map(tuning_directive_ordinal)
                .collect::<BTreeSet<_>>();
            if typed_tuning_ordinals != matched_directives {
                return Err(PipelineError::new(
                    "mission tuning typed stage directives are not fully owned",
                ));
            }
        }
    }
    if typed_stages.next().is_some() || reference_missions.next().is_some() {
        return Err(PipelineError::new(
            "mission tuning semantic reports contain unowned rows",
        ));
    }
    Ok(())
}

#[expect(
    clippy::too_many_arguments,
    reason = "Explicit owner evidence keeps mission tuning joins auditable."
)]
fn append_stage_binding(
    mission_id: &str,
    stage: &crate::domain::MissionScopeStage,
    typed_stage: &crate::domain::MissionStageSemanticBinding,
    participants: &[crate::domain::MissionResolvedParticipantReference],
    source_ordinal: usize,
    command: &str,
    arguments: &[String],
    scope: BindingScope,
    owner_objective_source_ordinal: Option<usize>,
    matched_directives: &mut BTreeSet<usize>,
    bindings: &mut Vec<MissionTuningBinding>,
) -> PipelineOutcome<()> {
    let directive = typed_stage
        .directives()
        .iter()
        .find(|directive| {
            tuning_directive_ordinal(directive) == Some(source_ordinal)
        })
        .ok_or_else(|| {
            PipelineError::new(
                "mission tuning source command lacks typed stage semantics",
            )
        })?;
    if !directive_matches(command, arguments, directive) {
        return Err(PipelineError::new(
            "mission tuning typed stage evidence drifted",
        ));
    }
    if !matched_directives.insert(source_ordinal) {
        return Err(PipelineError::new(
            "mission tuning typed stage directive is duplicated",
        ));
    }
    let vehicle_id = arguments.first().ok_or_else(|| {
        PipelineError::new("mission tuning stage vehicle identity disappeared")
    })?;
    let role = tuning_participant_role(command).ok_or_else(|| {
        PipelineError::new("mission tuning stage command is not reviewed")
    })?;
    let matches = participants
        .iter()
        .filter(|reference| {
            reference.source_ordinal() == source_ordinal
                && reference.role() == role
        })
        .collect::<Vec<_>>();
    let [reference] = matches.as_slice() else {
        return Err(PipelineError::new(
            "mission tuning physical vehicle reference is not unique",
        ));
    };
    if reference.owner_stage_source_ordinal() != Some(stage.source_ordinal())
        || reference.owner_stage_sequence_ordinal()
            != Some(stage.sequence_ordinal())
    {
        return Err(PipelineError::new(
            "mission tuning physical vehicle stage owner drifted",
        ));
    }
    let MissionParticipantReference::Vehicle(MissionVehicleReference::Catalog(
        vehicle,
    )) = reference.reference()
    else {
        return Err(PipelineError::new(
            "mission tuning participant is not a physical vehicle",
        ));
    };
    if vehicle.source_id() != vehicle_id {
        return Err(PipelineError::new(
            "mission tuning source and physical vehicle identities drifted",
        ));
    }
    bindings.push(MissionTuningBinding {
        command: command.to_owned(),
        arguments: arguments.to_vec(),
        source_ordinal,
        scope,
        owner_mission_id: Some(mission_id.to_owned()),
        owner_stage_source_ordinal: Some(stage.source_ordinal()),
        owner_stage_sequence_ordinal: Some(stage.sequence_ordinal()),
        owner_objective_source_ordinal,
        vehicle_id: vehicle_id.to_owned(),
        vehicle: vehicle.clone(),
    });
    Ok(())
}

fn is_stage_tuning_command(command: &str) -> bool {
    matches!(
        command,
        "setvehicleaiparams"
            | "setstageaitargetcatchupparams"
            | "setstageairacecatchupparams"
    )
}

fn tuning_participant_role(command: &str) -> Option<MissionParticipantRole> {
    match command {
        "setvehicleaiparams" => Some(MissionParticipantRole::StageVehicleAi),
        "setstageaitargetcatchupparams" => {
            Some(MissionParticipantRole::StageTargetCatchupVehicle)
        },
        "setstageairacecatchupparams" => {
            Some(MissionParticipantRole::StageRaceCatchupVehicle)
        },
        _ => None,
    }
}

const fn tuning_directive_ordinal(
    directive: &MissionStageDirective,
) -> Option<usize> {
    match directive {
        MissionStageDirective::VehicleAiTuning { source_ordinal, .. }
        | MissionStageDirective::TargetCatchupTuning { source_ordinal, .. }
        | MissionStageDirective::RaceCatchupTuning { source_ordinal, .. } => {
            Some(*source_ordinal)
        },
        _ => None,
    }
}

fn directive_matches(
    command: &str,
    arguments: &[String],
    directive: &MissionStageDirective,
) -> bool {
    match (command, arguments, directive) {
        (
            "setvehicleaiparams",
            [vehicle, first, second],
            MissionStageDirective::VehicleAiTuning {
                vehicle_id,
                source_first,
                source_second,
                ..
            },
        ) => {
            vehicle == vehicle_id
                && first.parse::<i32>().ok() == Some(*source_first)
                && second.parse::<i32>().ok() == Some(*source_second)
        },
        (
            "setstageaitargetcatchupparams",
            [vehicle, first, second],
            MissionStageDirective::TargetCatchupTuning {
                vehicle_id,
                source_first,
                source_second,
                ..
            },
        ) => {
            vehicle == vehicle_id
                && first.parse::<i32>().ok() == Some(*source_first)
                && second.parse::<i32>().ok() == Some(*source_second)
        },
        (
            "setstageairacecatchupparams",
            [vehicle, value, first, second, third],
            MissionStageDirective::RaceCatchupTuning {
                vehicle_id,
                source_value,
                source_factors,
                ..
            },
        ) => {
            vehicle == vehicle_id
                && value.parse::<u32>().ok() == Some(*source_value)
                && [first, second, third]
                    .iter()
                    .map(|factor| factor.as_str())
                    .eq(source_factors.iter().map(String::as_str))
        },
        _ => false,
    }
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
            "mission tuning source identity is not canonical",
        ));
    }
    Ok(())
}

#[cfg(test)]
// jig-ignore-next-line: exact Rust test-module path is indivisible.
#[path = "../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/mission_tuning_context/tests.rs"]
mod tests;

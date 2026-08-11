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
//   - Canonical JSON serialization of source-backed mission definition cores.
// - Must-Not:
//   - Add runtime transitions, infer semantic gaps, or contact Unreal Editor.
// - Allows:
//   - Exact typed definition-core evidence and stable source identities.
// - Split-When:
//   - Native asset application gains an independent serialization lifecycle.
// - Merge-When:
//   - Another renderer owns the identical mission-definition schema.
// - Summary:
//   - Mission definition-core canonical renderer.
// - Description:
//   - Emits deterministic versioned JSON without adding runtime behavior.
// - Usage:
//   - Called only after definition-core ownership validation succeeds.
// - Defaults:
//   - Invalid source identities and serialization failures fail closed.
//

//! Canonical JSON serialization for source-backed mission definition cores.

use serde_json::{Value, json};

use super::{
    MissionDefinitionConditionCoreBinding, MissionDefinitionCoreReport,
    MissionDefinitionStageCoreBinding,
};
use crate::domain::package::{
    MissionCollectibleWaypointBinding, MissionCountdownBinding,
    MissionPickupStatePropBinding, MissionPickupStatePropScope,
    MissionRoadArrowBinding, MissionRoadArrowMode,
};
use crate::domain::{
    MissionConditionParameters, MissionConditionScope,
    MissionObjectiveParameters, MissionStageKind,
    MissionStageTerminalOutcome, MissionStageTransitionMarkerKind,
    MissionStageVisualTransition, PipelineError, PipelineOutcome,
};

pub(in crate::adapters::driven::local) const MISSION_DEFINITION_CORE_SCHEMA: &str =
    "shar-schoenwald.mission-definition-core.v1";

/// Render one selected source definition core as canonical JSON ending in LF.
///
/// # Errors
///
/// Returns an error when the source identity is unsafe, the definition core
/// fails its ownership invariants, or JSON serialization fails.
pub(in crate::adapters::driven::local) fn render_definition_core(
    source_id: &str,
    report: &MissionDefinitionCoreReport,
) -> PipelineOutcome<String> {
    validate_source_id(source_id)?;
    report.validate()?;
    let value = json!({
        "mission_id": report.mission_id,
        "schema": MISSION_DEFINITION_CORE_SCHEMA,
        "source_id": source_id,
        "stages": report
            .stages
            .iter()
            .map(stage_json)
            .collect::<Vec<_>>(),
    });
    let mut text = serde_json::to_string(&value).map_err(|_error| {
        PipelineError::new("mission definition core JSON serialization failed")
    })?;
    text.push('\n');
    Ok(text)
}

fn validate_source_id(source_id: &str) -> PipelineOutcome<()> {
    let bytes = source_id.as_bytes();
    if bytes.is_empty()
        || bytes.len() > 240
        || !bytes.first().is_some_and(u8::is_ascii_alphanumeric)
        || !bytes.last().is_some_and(u8::is_ascii_alphanumeric)
        || bytes.windows(2).any(|pair| pair == b"--")
        || !bytes.iter().copied().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-'
        })
    {
        return Err(PipelineError::new(
            "mission definition core source identity is not canonical",
        ));
    }
    Ok(())
}

fn stage_json(stage: &MissionDefinitionStageCoreBinding) -> Value {
    json!({
        "checkpoint_source_ordinal": stage.checkpoint_source_ordinal,
        "collectible_waypoints": stage
            .collectible_waypoints
            .iter()
            .map(collectible_waypoint_json)
            .collect::<Vec<_>>(),
        "conditions": stage
            .conditions
            .iter()
            .map(condition_json)
            .collect::<Vec<_>>(),
        "countdown": stage.countdown.as_ref().map(countdown_json),
        "explicit_final": stage.explicit_final,
        "kind": stage_kind_json(&stage.kind),
        "next_authored_sequence_ordinal":
            stage.next_authored_sequence_ordinal,
        "objective": objective_json(stage),
        "objective_npc_waypoints": stage
            .objective_npc_waypoints
            .iter()
            .map(npc_waypoint_json)
            .collect::<Vec<_>>(),
        "pickup_state_props": stage
            .pickup_state_props
            .iter()
            .map(pickup_state_prop_json)
            .collect::<Vec<_>>(),
        "sequence_ordinal": stage.sequence_ordinal,
        "show_stage_complete": stage.show_stage_complete,
        "stage_source_ordinal": stage.stage_source_ordinal,
        "stay_in_black": stage.stay_in_black,
        "terminal": terminal_token(stage.terminal),
        "transition_markers": stage
            .transition_markers
            .iter()
            .map(|marker| json!({
                "kind": transition_marker_token(marker.kind()),
                "source_ordinal": marker.source_ordinal(),
            }))
            .collect::<Vec<_>>(),
        "visual_transition": visual_transition_token(stage.visual_transition),
    })
}

fn stage_kind_json(kind: &MissionStageKind) -> Value {
    match kind {
        MissionStageKind::Standard {
            legacy_flags,
            final_stage,
        } => json!({
            "final_stage": final_stage,
            "kind": "standard",
            "legacy_flags": legacy_flags,
        }),
        MissionStageKind::LockedVehicle { vehicle_id } => json!({
            "kind": "locked-vehicle",
            "vehicle_id": vehicle_id,
        }),
        MissionStageKind::LockedCostume { costume_id } => json!({
            "costume_id": costume_id,
            "kind": "locked-costume",
        }),
    }
}

fn objective_json(stage: &MissionDefinitionStageCoreBinding) -> Value {
    json!({
        "canonical_kind": stage.objective_canonical_kind,
        "parameters": objective_parameters_json(&stage.objective_parameters),
        "source_alias": stage.objective_source_alias,
        "source_ordinal": stage.objective_source_ordinal,
        "unavailable_code": stage.objective_unavailable_code,
    })
}

fn objective_parameters_json(parameters: &MissionObjectiveParameters) -> Value {
    match parameters {
        MissionObjectiveParameters::None => json!({"kind": "none"}),
        MissionObjectiveParameters::RoadArrows(binding) => json!({
            "kind": "road-arrows",
            "road_arrows": road_arrow_json(binding),
        }),
        MissionObjectiveParameters::BuyVehicle { vehicle_id } => json!({
            "kind": "buy-vehicle",
            "vehicle_id": vehicle_id,
        }),
        MissionObjectiveParameters::BuyCostume { costume_id } => json!({
            "costume_id": costume_id,
            "kind": "buy-costume",
        }),
        MissionObjectiveParameters::EnterVehicle { vehicle_id } => json!({
            "kind": "enter-vehicle",
            "vehicle_id": vehicle_id,
        }),
        MissionObjectiveParameters::Race {
            gamble,
            road_arrows,
        } => json!({
            "gamble": gamble,
            "kind": "race",
            "road_arrows": road_arrows.as_ref().map(road_arrow_json),
        }),
    }
}

fn road_arrow_json(binding: &MissionRoadArrowBinding) -> Value {
    match binding {
        MissionRoadArrowBinding::Effective(mode) => json!({
            "kind": "effective",
            "mode": road_arrow_mode_token(*mode),
        }),
        MissionRoadArrowBinding::LegacyUnrecognized {
            source_token,
            code,
        } => json!({
            "code": code,
            "kind": "legacy-unrecognized",
            "source_token": source_token,
        }),
    }
}

const fn road_arrow_mode_token(mode: MissionRoadArrowMode) -> &'static str {
    match mode {
        MissionRoadArrowMode::Both => "both",
        MissionRoadArrowMode::Neither => "neither",
        MissionRoadArrowMode::Intersection => "intersection",
        MissionRoadArrowMode::NearestRoad => "nearest-road",
    }
}

fn condition_json(condition: &MissionDefinitionConditionCoreBinding) -> Value {
    json!({
        "owner_objective_source_ordinal":
            condition.owner_objective_source_ordinal,
        "parameters": condition_parameters_json(&condition.parameters),
        "schema_id": condition.schema_id,
        "scope": condition_scope_token(condition.scope),
        "source_alias": condition.source_alias,
        "source_ordinal": condition.source_ordinal,
    })
}

fn condition_parameters_json(parameters: &MissionConditionParameters) -> Value {
    match parameters {
        MissionConditionParameters::None => json!({"kind": "none"}),
        MissionConditionParameters::KeepBarrelLegacyValue { value } => json!({
            "kind": "keep-barrel-legacy-value",
            "value": value,
        }),
        MissionConditionParameters::DamageLegacyToken {
            source_token,
            code,
        } => json!({
            "code": code,
            "kind": "damage-legacy-token",
            "source_token": source_token,
        }),
    }
}

const fn condition_scope_token(scope: MissionConditionScope) -> &'static str {
    match scope {
        MissionConditionScope::Stage => "stage",
        MissionConditionScope::Objective => "objective",
    }
}

fn countdown_json(countdown: &MissionCountdownBinding) -> Value {
    json!({
        "character_id": countdown.character_id(),
        "entries": countdown
            .entries()
            .iter()
            .map(|entry| json!({
                "duration_milliseconds": entry.duration_milliseconds(),
                "source_ordinal": entry.source_ordinal(),
                "token": entry.token(),
            }))
            .collect::<Vec<_>>(),
        "sequence_id": countdown.sequence_id(),
        "stage_sequence_ordinal": countdown.stage_sequence_ordinal(),
        "stage_source_ordinal": countdown.stage_source_ordinal(),
        "start_source_ordinal": countdown.start_source_ordinal(),
    })
}

fn collectible_waypoint_json(
    binding: &MissionCollectibleWaypointBinding,
) -> Value {
    json!({
        "collectible_index": binding.collectible_index(),
        "collectible_locator_id": binding.collectible_locator_id(),
        "collectible_source_ordinal": binding.collectible_source_ordinal(),
        "objective_source_ordinal": binding.objective_source_ordinal(),
        "source_ordinal": binding.source_ordinal(),
        "stage_sequence_ordinal": binding.stage_sequence_ordinal(),
        "stage_source_ordinal": binding.stage_source_ordinal(),
        "waypoint_index": binding.waypoint_index(),
        "waypoint_locator_id": binding.waypoint_locator_id(),
        "waypoint_source_ordinal": binding.waypoint_source_ordinal(),
    })
}

fn npc_waypoint_json(
    binding: &crate::domain::MissionObjectiveNpcWaypointBinding,
) -> Value {
    json!({
        "declaration_source_ordinal": binding.declaration_source_ordinal(),
        "npc_id": binding.npc_id(),
        "npc_locator_id": binding.npc_locator_id(),
        "objective_source_ordinal": binding.objective_source_ordinal(),
        "source_ordinal": binding.source_ordinal(),
        "stage_sequence_ordinal": binding.owner_stage_sequence_ordinal(),
        "stage_source_ordinal": binding.owner_stage_source_ordinal(),
        "waypoint_locator_id": binding.waypoint_locator_id(),
    })
}

fn pickup_state_prop_json(
    binding: &MissionPickupStatePropBinding,
) -> Value {
    json!({
        "declaration_scope": pickup_scope_json(binding.declaration_scope()),
        "declaration_source_ordinal": binding.declaration_source_ordinal(),
        "locator_id": binding.locator_id(),
        "objective_source_ordinal": binding.owner_objective_source_ordinal(),
        "source_state": binding.source_state(),
        "stage_sequence_ordinal": binding.owner_stage_sequence_ordinal(),
        "stage_source_ordinal": binding.owner_stage_source_ordinal(),
        "target_id": binding.target_id(),
        "target_source_ordinal": binding.target_source_ordinal(),
    })
}

fn pickup_scope_json(scope: MissionPickupStatePropScope) -> Value {
    match scope {
        MissionPickupStatePropScope::Mission => json!({
            "kind": "mission",
        }),
        MissionPickupStatePropScope::Stage {
            source_ordinal,
            sequence_ordinal,
        } => json!({
            "kind": "stage",
            "sequence_ordinal": sequence_ordinal,
            "source_ordinal": source_ordinal,
        }),
    }
}

const fn terminal_token(value: MissionStageTerminalOutcome) -> &'static str {
    match value {
        MissionStageTerminalOutcome::None => "none",
        MissionStageTerminalOutcome::ChapterTransition => "chapter-transition",
        MissionStageTerminalOutcome::GameCompletion => "game-completion",
    }
}

const fn visual_transition_token(
    value: MissionStageVisualTransition,
) -> &'static str {
    match value {
        MissionStageVisualTransition::None => "none",
        MissionStageVisualTransition::Iris => "iris",
        MissionStageVisualTransition::Fade => "fade",
    }
}

const fn transition_marker_token(
    value: MissionStageTransitionMarkerKind,
) -> &'static str {
    match value {
        MissionStageTransitionMarkerKind::IrisWipe => "iris-wipe",
        MissionStageTransitionMarkerKind::FadeOut => "fade-out",
        MissionStageTransitionMarkerKind::LevelOver => "level-over",
        MissionStageTransitionMarkerKind::GameOver => "game-over",
        MissionStageTransitionMarkerKind::StayInBlack => "stay-in-black",
        MissionStageTransitionMarkerKind::ShowStageComplete => {
            "show-stage-complete"
        },
    }
}

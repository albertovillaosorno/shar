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
//   - Tests unit tests.
// - Must-Not:
//   - Own production behavior or broaden the tested API surface.
// - Allows:
//   - Private test fixtures and assertions for the owning source module.
// - Split-When:
//   - Split when an independent fixture family gains separate ownership.
// - Merge-When:
//   - Merge when another test module owns the identical evidence.
// - Summary:
//   - Tests unit tests.
// - Description:
//   - Preserves unit-test access through a test-only path module.
// - Usage:
//   - Included only by the owning source module under cfg(test).
// - Defaults:
//   - Test setup and assertions fail explicitly.
//

//! Tests unit tests.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use super::{
    decode_straggler_text, normalize_game_stragglers,
    publish_generated_directory, recover_generated_transaction,
};

static TRANSACTION_CASE_ID: AtomicUsize = AtomicUsize::new(0);

fn transaction_case_root(label: &str) -> Result<PathBuf, String> {
    let ordinal = TRANSACTION_CASE_ID.fetch_add(1, Ordering::Relaxed);
    let root = repository_root()?.join(".temp").join("tests").join(format!(
        "stragglers-{label}-{}-{ordinal}",
        std::process::id()
    ));
    if root.exists() {
        fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    }
    fs::create_dir_all(&root).map_err(|error| error.to_string())?;
    Ok(root)
}

fn remove_transaction_case(root: &Path) -> Result<(), String> {
    if root.exists() {
        fs::remove_dir_all(root).map_err(|error| error.to_string())?;
    }
    Ok(())
}

#[test]
// jig-ignore-next-line: long identifier
fn failed_straggler_normalization_preserves_accepted_root() -> Result<(), String>
{
    let root = transaction_case_root("failure-preserves")?;
    let game = root.join("game-source");
    let extracted = root.join("extracted");
    fs::create_dir_all(&game).map_err(|error| error.to_string())?;
    fs::create_dir_all(extracted.join("game"))
        .map_err(|error| error.to_string())?;
    fs::write(game.join("broken.rsd"), b"not-rsd")
        .map_err(|error| error.to_string())?;
    fs::write(extracted.join("game/accepted.txt"), b"accepted")
        .map_err(|error| error.to_string())?;

    let result = normalize_game_stragglers(&game, &extracted);
    if result.is_ok() {
        return Err("invalid RSD unexpectedly normalized".to_owned());
    }
    if fs::read(extracted.join("game/accepted.txt"))
        .map_err(|error| error.to_string())?
        != b"accepted"
    {
        return Err("failed normalization changed accepted evidence".to_owned());
    }
    if extracted.join(".game.straggler-staging").exists()
        || extracted.join(".game.straggler-backup").exists()
    {
        return Err("failed normalization left transaction residue".to_owned());
    }
    remove_transaction_case(&root)
}

#[test]
// jig-ignore-next-line: long identifier
fn straggler_publication_replaces_root_and_removes_backup() -> Result<(), String>
{
    let root = transaction_case_root("publish")?;
    let output = root.join("game");
    let staging = root.join(".game.straggler-staging");
    let backup = root.join(".game.straggler-backup");
    fs::create_dir_all(&output).map_err(|error| error.to_string())?;
    fs::create_dir_all(&staging).map_err(|error| error.to_string())?;
    fs::write(output.join("old.txt"), b"old")
        .map_err(|error| error.to_string())?;
    fs::write(staging.join("new.txt"), b"new")
        .map_err(|error| error.to_string())?;

    publish_generated_directory(&staging, &output, &backup)
        .map_err(|error| error.to_string())?;
    if output.join("old.txt").exists()
        || fs::read(output.join("new.txt"))
            .map_err(|error| error.to_string())?
            != b"new"
        || staging.exists()
        || backup.exists()
    {
        return Err("straggler publication inventory is not exact".to_owned());
    }
    remove_transaction_case(&root)
}

#[test]
fn failed_straggler_publication_restores_accepted_root() -> Result<(), String> {
    let root = transaction_case_root("publish-rollback")?;
    let output = root.join("game");
    let staging = root.join(".game.straggler-staging");
    let backup = root.join(".game.straggler-backup");
    fs::create_dir_all(&output).map_err(|error| error.to_string())?;
    fs::write(output.join("accepted.txt"), b"accepted")
        .map_err(|error| error.to_string())?;

    let result = publish_generated_directory(&staging, &output, &backup);
    if result.is_ok() {
        return Err(
            "missing staging directory unexpectedly published".to_owned()
        );
    }
    if fs::read(output.join("accepted.txt"))
        .map_err(|error| error.to_string())?
        != b"accepted"
        || backup.exists()
    {
        return Err(
            "failed publication did not restore accepted root".to_owned()
        );
    }
    remove_transaction_case(&root)
}

#[test]
fn straggler_recovery_restores_backup_before_staging_cleanup()
-> Result<(), String> {
    let root = transaction_case_root("recover")?;
    let output = root.join("game");
    let staging = root.join(".game.straggler-staging");
    let backup = root.join(".game.straggler-backup");
    fs::create_dir_all(&backup).map_err(|error| error.to_string())?;
    fs::create_dir_all(&staging).map_err(|error| error.to_string())?;
    fs::write(backup.join("accepted.txt"), b"accepted")
        .map_err(|error| error.to_string())?;
    fs::write(staging.join("partial.txt"), b"partial")
        .map_err(|error| error.to_string())?;

    recover_generated_transaction(&output, &staging, &backup)
        .map_err(|error| error.to_string())?;
    if fs::read(output.join("accepted.txt"))
        .map_err(|error| error.to_string())?
        != b"accepted"
        || staging.exists()
        || backup.exists()
    {
        return Err(
            "straggler transaction recovery did not restore accepted root"
                .to_owned(),
        );
    }
    remove_transaction_case(&root)
}

#[test]
fn decodes_windows_1252_text_stragglers() {
    let result = decode_straggler_text(
        b"Logitech\xae Force",
        Path::new("synthetic/era.txt"),
        "txt",
    );
    assert!(
        result.as_deref() == Ok("Logitech\u{ae} Force"),
        "era Windows-1252 bytes must decode deterministically"
    );
}

#[test]
fn rejects_undefined_windows_1252_text_stragglers() {
    let result = decode_straggler_text(
        &[0x81_u8],
        Path::new("synthetic/invalid.txt"),
        "txt",
    );
    assert!(
        result.is_err(),
        "bytes Windows-1252 leaves undefined must fail closed"
    );
}

#[test]
fn normalized_config_path_preserves_authored_filename() {
    let output = super::normalized_json_path_at_root(
        Path::new("normalized"),
        Path::new("scripts/cars/CAR_A.con"),
    );
    assert_eq!(
        output,
        Path::new("normalized/scripts/cars/CAR_A.con.json"),
    );
}

#[test]
fn mission_v3_renderer_matches_semantic_preflight() -> Result<(), String> {
    let source = concat!(
        "SelectMission(\"m1\");\n",
        "AddStage(0);\n",
        "AddObjective(\"goto\");\n",
        "CloseObjective();\n",
        "CloseStage();\n",
        "CloseMission();\n",
    );
    let rendered = super::semantic_json_from_text(
        Path::new("scripts/missions/level01/m1i.mfk"),
        "mfk",
        source.as_bytes(),
        source,
    );
    let evidence = crate::preflight_mission_script(&rendered)?;
    if evidence.statement_count() != 6 || evidence.invocations().len() != 6 {
        return Err(
            "rendered mission evidence changed during preflight".to_owned()
        );
    }
    Ok(())
}

fn repository_root() -> Result<PathBuf, String> {
    let mut current = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    loop {
        if current.join("TODO.md").is_file()
            && current.join("game/scripts/missions").is_dir()
        {
            return Ok(current);
        }
        if !current.pop() {
            return Err("repository root was not found".to_owned());
        }
    }
}

fn mission_sources(root: &Path) -> Result<Vec<PathBuf>, String> {
    let mut pending = vec![root.join("game/scripts/missions")];
    let mut files = Vec::new();
    while let Some(directory) = pending.pop() {
        for entry in
            fs::read_dir(&directory).map_err(|error| error.to_string())?
        {
            let path = entry.map_err(|error| error.to_string())?.path();
            if path.is_dir() {
                pending.push(path);
            } else if path
                .extension()
                .and_then(|extension| extension.to_str())
                .is_some_and(|extension| extension.eq_ignore_ascii_case("mfk"))
            {
                files.push(path);
            }
        }
    }
    files.sort();
    Ok(files)
}

#[test]
fn repository_mission_corpus_passes_semantic_registries() -> Result<(), String>
{
    let root = repository_root()?;
    let game_root = root.join("game");
    let sources = mission_sources(&root)?;
    if sources.is_empty() {
        return Err("repository mission corpus is empty".to_owned());
    }
    let mut objective_count = 0usize;
    let mut typed_objective_parameter_count = 0usize;
    let mut typed_objective_reference_count = 0usize;
    let mut legacy_unrecognized_route_count = 0usize;
    let mut condition_count = 0usize;
    let mut typed_condition_parameter_count = 0usize;
    let mut keepbarrel_legacy_value_count = 0usize;
    let mut damage_legacy_token_count = 0usize;
    let mut adaptation_count = 0usize;
    let mut mission_graph_count = 0usize;
    let mut unscoped_command_count = 0usize;
    let mut mission_command_count = 0usize;
    let mut stage_command_count = 0usize;
    let mut objective_command_count = 0usize;
    let mut condition_command_count = 0usize;
    let mut stage_count = 0usize;
    let mut typed_stage_count = 0usize;
    let mut stage_directive_count = 0usize;
    let mut stage_character_count = 0usize;
    let mut stage_closed_direct_only_count = 0usize;
    let mut standard_stage_count = 0usize;
    let mut legacy_flag_stage_count = 0usize;
    let mut final_stage_count = 0usize;
    let mut locked_vehicle_stage_count = 0usize;
    let mut locked_costume_stage_count = 0usize;
    let mut set_time_directive_count = 0usize;
    let mut add_time_directive_count = 0usize;
    let mut checkpoint_directive_count = 0usize;
    let mut message_directive_count = 0usize;
    let mut stage_vehicle_directive_count = 0usize;
    let mut stage_activate_vehicle_count = 0usize;
    let mut stage_hud_icon_count = 0usize;
    let mut stage_fade_out_legacy_count = 0usize;
    let mut stage_iris_wipe_legacy_count = 0usize;
    let mut stage_max_traffic_count = 0usize;
    let mut stage_vehicle_ai_count = 0usize;
    let mut stage_target_catchup_count = 0usize;
    let mut stage_safe_zone_count = 0usize;
    let mut stage_stay_in_black_count = 0usize;
    let mut stage_game_over_count = 0usize;
    let mut stage_level_over_count = 0usize;
    let mut stage_show_complete_count = 0usize;
    let mut stage_disable_hit_and_run_count = 0usize;
    let mut waypoint_directive_count = 0usize;
    let mut initialization_directive_count = 0usize;
    let mut reset_in_car_count = 0usize;
    let mut reset_out_car_count = 0usize;
    let mut initial_walk_count = 0usize;
    let mut dynamic_load_count = 0usize;
    let mut street_race_props_load_count = 0usize;
    let mut street_race_props_unload_count = 0usize;
    let mut forced_car_count = 0usize;
    let mut initial_vehicle_count = 0usize;
    let mut mission_closed_remaining_count = 0usize;
    let mut objective_semantic_count = 0usize;
    let mut objective_directive_count = 0usize;
    let mut objective_npc_count = 0usize;
    let mut objective_npc_waypoint_count = 0usize;
    let mut objective_driver_count = 0usize;
    let mut objective_remove_driver_count = 0usize;
    let mut objective_remove_npc_count = 0usize;
    let mut objective_target_vehicle_count = 0usize;
    let mut objective_talk_target_count = 0usize;
    let mut objective_npc_animation_count = 0usize;
    let mut objective_player_animation_count = 0usize;
    let mut objective_ambient_randomize_count = 0usize;
    let mut objective_camera_best_side_count = 0usize;
    let mut objective_conversation_camera_count = 0usize;
    let mut objective_dialogue_info_count = 0usize;
    let mut objective_dialogue_positions_count = 0usize;
    let mut objective_distance_count = 0usize;
    let mut objective_par_time_count = 0usize;
    let mut objective_pickup_target_count = 0usize;
    let mut objective_goto_dialog_off_count = 0usize;
    let mut objective_must_action_trigger_count = 0usize;
    let mut objective_allow_rock_out_count = 0usize;
    let mut objective_destination_count = 0usize;
    let mut objective_presentation_bitmap_count = 0usize;
    let mut objective_fmv_info_count = 0usize;
    let mut objective_duration_count = 0usize;
    let mut objective_race_laps_count = 0usize;
    let mut objective_coin_fee_count = 0usize;
    let mut objective_collectible_count = 0usize;
    let mut objective_collectible_effect_count = 0usize;
    let mut objective_collectible_binding_count = 0usize;
    let mut condition_semantic_count = 0usize;
    let mut condition_directive_count = 0usize;
    let mut condition_health_count = 0usize;
    let mut condition_target_vehicle_count = 0usize;
    let mut condition_target_boss_count = 0usize;
    let mut condition_follow_distance_count = 0usize;
    let mut condition_time_value_count = 0usize;
    let mut condition_position_index_count = 0usize;
    let mut condition_hit_and_run_no_op_count = 0usize;
    let mut stage_condition_count = 0usize;
    let mut objective_condition_count = 0usize;
    let mut unavailable_objective_count = 0usize;
    let mut empty_placeholder_count = 0usize;
    for source_path in &sources {
        let bytes = fs::read(source_path).map_err(|error| error.to_string())?;
        if bytes.is_empty() {
            empty_placeholder_count = empty_placeholder_count.saturating_add(1);
            continue;
        }
        let relative = source_path
            .strip_prefix(&game_root)
            .map_err(|_error| "mission source escaped game root".to_owned())?;
        let text = decode_straggler_text(&bytes, relative, "mfk")
            .map_err(|error| error.to_string())?;
        let rendered = super::semantic_json_from_text(
            relative,
            "mfk",
            &bytes,
            text.as_ref(),
        );
        let evidence = crate::preflight_mission_script(&rendered)
            .map_err(|error| format!("{}: {error}", relative.display()))?;
        let objectives = crate::domain::preflight_mission_objectives(&evidence)
            .map_err(|error| format!("{}: {error}", relative.display()))?;
        let objective_parameters =
            crate::domain::preflight_mission_objective_parameters(&evidence)
                .map_err(|error| format!("{}: {error}", relative.display()))?;
        typed_objective_parameter_count = typed_objective_parameter_count
            .saturating_add(objective_parameters.objectives().len());
        for objective in objective_parameters.objectives() {
            use crate::domain::{
                MissionObjectiveParameters, MissionRoadArrowBinding,
            };
            match objective.parameters() {
                MissionObjectiveParameters::BuyVehicle { .. }
                | MissionObjectiveParameters::BuyCostume { .. }
                | MissionObjectiveParameters::EnterVehicle { .. } => {
                    typed_objective_reference_count =
                        typed_objective_reference_count.saturating_add(1);
                },
                MissionObjectiveParameters::RoadArrows(
                    MissionRoadArrowBinding::LegacyUnrecognized { .. },
                ) => {
                    legacy_unrecognized_route_count =
                        legacy_unrecognized_route_count.saturating_add(1);
                },
                MissionObjectiveParameters::None
                | MissionObjectiveParameters::RoadArrows(_)
                | MissionObjectiveParameters::Race { .. } => {},
            }
        }
        drop(
            crate::domain::preflight_mission_objective_commands(&evidence)
                .map_err(|error| format!("{}: {error}", relative.display()))?,
        );
        let conditions = crate::domain::preflight_mission_conditions(&evidence)
            .map_err(|error| format!("{}: {error}", relative.display()))?;
        let condition_parameters =
            crate::domain::preflight_mission_condition_parameters(&evidence)
                .map_err(|error| format!("{}: {error}", relative.display()))?;
        typed_condition_parameter_count = typed_condition_parameter_count
            .saturating_add(condition_parameters.conditions().len());
        for condition in condition_parameters.conditions() {
            use crate::domain::MissionConditionParameters;
            match condition.parameters() {
                MissionConditionParameters::KeepBarrelLegacyValue { .. } => {
                    keepbarrel_legacy_value_count =
                        keepbarrel_legacy_value_count.saturating_add(1);
                },
                MissionConditionParameters::DamageLegacyToken { .. } => {
                    damage_legacy_token_count =
                        damage_legacy_token_count.saturating_add(1);
                },
                MissionConditionParameters::None => {},
            }
        }
        drop(
            crate::domain::preflight_mission_condition_commands(&evidence)
                .map_err(|error| format!("{}: {error}", relative.display()))?,
        );
        objective_count =
            objective_count.saturating_add(objectives.objectives().len());
        condition_count =
            condition_count.saturating_add(conditions.conditions().len());
        adaptation_count =
            adaptation_count.saturating_add(evidence.adaptations().len());
        let scopes = crate::domain::compile_mission_scope_graphs(&evidence)
            .map_err(|error| format!("{}: {error}", relative.display()))?;
        let stage_semantics =
            crate::domain::preflight_mission_stage_semantics(&scopes)
                .map_err(|error| format!("{}: {error}", relative.display()))?;
        let initialization =
            crate::domain::preflight_mission_initialization(&scopes)
                .map_err(|error| format!("{}: {error}", relative.display()))?;
        let objective_semantics =
            crate::domain::preflight_mission_objective_semantics(&scopes)
                .map_err(|error| format!("{}: {error}", relative.display()))?;
        objective_semantic_count = objective_semantic_count
            .saturating_add(objective_semantics.objectives().len());
        for objective in objective_semantics.objectives() {
            for directive in objective.directives() {
                use crate::domain::MissionObjectiveDirective;
                objective_directive_count =
                    objective_directive_count.saturating_add(1);
                match directive {
                    MissionObjectiveDirective::Npc(_) => {
                        objective_npc_count =
                            objective_npc_count.saturating_add(1);
                    },
                    MissionObjectiveDirective::NpcWaypoint { .. } => {
                        objective_npc_waypoint_count =
                            objective_npc_waypoint_count.saturating_add(1);
                    },
                    MissionObjectiveDirective::Driver { .. } => {
                        objective_driver_count =
                            objective_driver_count.saturating_add(1);
                    },
                    MissionObjectiveDirective::RemoveDriver { .. } => {
                        objective_remove_driver_count =
                            objective_remove_driver_count.saturating_add(1);
                    },
                    MissionObjectiveDirective::RemoveNpc { .. } => {
                        objective_remove_npc_count =
                            objective_remove_npc_count.saturating_add(1);
                    },
                    MissionObjectiveDirective::TargetVehicle { .. } => {
                        objective_target_vehicle_count =
                            objective_target_vehicle_count.saturating_add(1);
                    },
                    MissionObjectiveDirective::TalkTarget { .. } => {
                        objective_talk_target_count =
                            objective_talk_target_count.saturating_add(1);
                    },
                    MissionObjectiveDirective::AmbientNpcAnimation { .. } => {
                        objective_npc_animation_count =
                            objective_npc_animation_count.saturating_add(1);
                    },
                    MissionObjectiveDirective::AmbientPcAnimation { .. } => {
                        objective_player_animation_count =
                            objective_player_animation_count.saturating_add(1);
                    },
                    MissionObjectiveDirective::AmbientAnimationRandomize {
                        ..
                    } => {
                        objective_ambient_randomize_count =
                            objective_ambient_randomize_count.saturating_add(1);
                    },
                    MissionObjectiveDirective::CameraBestSide { .. } => {
                        objective_camera_best_side_count =
                            objective_camera_best_side_count.saturating_add(1);
                    },
                    MissionObjectiveDirective::ConversationCamera { .. } => {
                        objective_conversation_camera_count =
                            objective_conversation_camera_count
                                .saturating_add(1);
                    },
                    MissionObjectiveDirective::DialogueInfo { .. } => {
                        objective_dialogue_info_count =
                            objective_dialogue_info_count.saturating_add(1);
                    },
                    MissionObjectiveDirective::DialoguePositions { .. } => {
                        objective_dialogue_positions_count =
                            objective_dialogue_positions_count
                                .saturating_add(1);
                    },
                    MissionObjectiveDirective::ObjectDistance { .. } => {
                        objective_distance_count =
                            objective_distance_count.saturating_add(1);
                    },
                    MissionObjectiveDirective::ParTime { .. } => {
                        objective_par_time_count =
                            objective_par_time_count.saturating_add(1);
                    },
                    MissionObjectiveDirective::PickupTarget { .. } => {
                        objective_pickup_target_count =
                            objective_pickup_target_count.saturating_add(1);
                    },
                    MissionObjectiveDirective::TurnGotoDialogOff { .. } => {
                        objective_goto_dialog_off_count =
                            objective_goto_dialog_off_count.saturating_add(1);
                    },
                    MissionObjectiveDirective::MustActionTrigger { .. } => {
                        objective_must_action_trigger_count =
                            objective_must_action_trigger_count
                                .saturating_add(1);
                    },
                    MissionObjectiveDirective::AllowRockOut { .. } => {
                        objective_allow_rock_out_count =
                            objective_allow_rock_out_count.saturating_add(1);
                    },
                    MissionObjectiveDirective::Destination { .. } => {
                        objective_destination_count =
                            objective_destination_count.saturating_add(1);
                    },
                    MissionObjectiveDirective::PresentationBitmap { .. } => {
                        objective_presentation_bitmap_count =
                            objective_presentation_bitmap_count
                                .saturating_add(1);
                    },
                    MissionObjectiveDirective::FmvInfo { .. } => {
                        objective_fmv_info_count =
                            objective_fmv_info_count.saturating_add(1);
                    },
                    MissionObjectiveDirective::Duration { .. } => {
                        objective_duration_count =
                            objective_duration_count.saturating_add(1);
                    },
                    MissionObjectiveDirective::RaceLaps { .. } => {
                        objective_race_laps_count =
                            objective_race_laps_count.saturating_add(1);
                    },
                    MissionObjectiveDirective::CoinFee { .. } => {
                        objective_coin_fee_count =
                            objective_coin_fee_count.saturating_add(1);
                    },
                    MissionObjectiveDirective::Collectible { .. } => {
                        objective_collectible_count =
                            objective_collectible_count.saturating_add(1);
                    },
                    MissionObjectiveDirective::CollectibleEffect { .. } => {
                        objective_collectible_effect_count =
                            objective_collectible_effect_count
                                .saturating_add(1);
                    },
                    MissionObjectiveDirective::BindCollectibleToWaypoint {
                        ..
                    } => {
                        objective_collectible_binding_count =
                            objective_collectible_binding_count
                                .saturating_add(1);
                    },
                }
            }
        }
        let condition_semantics =
            crate::domain::preflight_mission_condition_semantics(&scopes)
                .map_err(|error| format!("{}: {error}", relative.display()))?;
        condition_semantic_count = condition_semantic_count
            .saturating_add(condition_semantics.conditions().len());
        for condition in condition_semantics.conditions() {
            for directive in condition.directives() {
                use crate::domain::MissionConditionDirective;
                condition_directive_count =
                    condition_directive_count.saturating_add(1);
                match directive {
                    MissionConditionDirective::MinimumHealth { .. } => {
                        condition_health_count =
                            condition_health_count.saturating_add(1);
                    },
                    MissionConditionDirective::TargetVehicle { .. } => {
                        condition_target_vehicle_count =
                            condition_target_vehicle_count.saturating_add(1);
                    },
                    MissionConditionDirective::TargetBoss { .. } => {
                        condition_target_boss_count =
                            condition_target_boss_count.saturating_add(1);
                    },
                    MissionConditionDirective::FollowDistances { .. } => {
                        condition_follow_distance_count =
                            condition_follow_distance_count.saturating_add(1);
                    },
                    MissionConditionDirective::TimeValue { .. } => {
                        condition_time_value_count =
                            condition_time_value_count.saturating_add(1);
                    },
                    MissionConditionDirective::PositionIndex { .. } => {
                        condition_position_index_count =
                            condition_position_index_count.saturating_add(1);
                    },
                    MissionConditionDirective::LegacyHitAndRunNoOp { .. } => {
                        condition_hit_and_run_no_op_count =
                            condition_hit_and_run_no_op_count.saturating_add(1);
                    },
                }
            }
        }
        for mission in initialization.missions() {
            for directive in mission.directives() {
                use crate::domain::MissionInitializationDirective;
                initialization_directive_count =
                    initialization_directive_count.saturating_add(1);
                match directive {
                    MissionInitializationDirective::ResetPlayerInCar { .. } => {
                        reset_in_car_count =
                            reset_in_car_count.saturating_add(1);
                    },
                    MissionInitializationDirective::ResetPlayerOutCar {
                        ..
                    } => {
                        reset_out_car_count =
                            reset_out_car_count.saturating_add(1);
                    },
                    MissionInitializationDirective::InitialWalk { .. } => {
                        initial_walk_count =
                            initial_walk_count.saturating_add(1);
                    },
                    MissionInitializationDirective::DynamicLoad { .. } => {
                        dynamic_load_count =
                            dynamic_load_count.saturating_add(1);
                    },
                    MissionInitializationDirective::StreetRacePropsLoad {
                        ..
                    } => {
                        street_race_props_load_count =
                            street_race_props_load_count.saturating_add(1);
                    },
                    MissionInitializationDirective::StreetRacePropsUnload {
                        ..
                    } => {
                        street_race_props_unload_count =
                            street_race_props_unload_count.saturating_add(1);
                    },
                    MissionInitializationDirective::CollectibleStateProp { .. }
                    | MissionInitializationDirective::PlacePlayerCar { .. }
                    | MissionInitializationDirective::AnimatedCamera { .. }
                    | MissionInitializationDirective::AnimatedCameraMulticont {
                        ..
                    }
                    | MissionInitializationDirective::MissionStartCamera { .. }
                    | MissionInitializationDirective::MissionStartMulticont {
                        ..
                    }
                    | MissionInitializationDirective::ValidFailureHints { .. }
                    | MissionInitializationDirective::PresentationBitmap { .. }
                    | MissionInitializationDirective::HudVisibility { .. }
                    | MissionInitializationDirective::PedGroup { .. } => {
                        mission_closed_remaining_count =
                            mission_closed_remaining_count.saturating_add(1);
                    },
                    MissionInitializationDirective::ForcedCar { .. } => {
                        forced_car_count = forced_car_count.saturating_add(1);
                    },
                    MissionInitializationDirective::InitialPlayerVehicle {
                        ..
                    } => {
                        initial_vehicle_count =
                            initial_vehicle_count.saturating_add(1);
                    },
                }
            }
        }
        typed_stage_count =
            typed_stage_count.saturating_add(stage_semantics.stages().len());
        for stage in stage_semantics.stages() {
            use crate::domain::{MissionStageDirective, MissionStageKind};
            match stage.kind() {
                MissionStageKind::Standard {
                    legacy_flags,
                    final_stage,
                } => {
                    standard_stage_count =
                        standard_stage_count.saturating_add(1);
                    if legacy_flags.is_some() {
                        legacy_flag_stage_count =
                            legacy_flag_stage_count.saturating_add(1);
                    }
                    if *final_stage {
                        final_stage_count = final_stage_count.saturating_add(1);
                    }
                },
                MissionStageKind::LockedVehicle { .. } => {
                    locked_vehicle_stage_count =
                        locked_vehicle_stage_count.saturating_add(1);
                },
                MissionStageKind::LockedCostume { .. } => {
                    locked_costume_stage_count =
                        locked_costume_stage_count.saturating_add(1);
                },
            }
            for directive in stage.directives() {
                stage_directive_count = stage_directive_count.saturating_add(1);
                match directive {
                    MissionStageDirective::SetTimeSeconds { .. } => {
                        set_time_directive_count =
                            set_time_directive_count.saturating_add(1);
                    },
                    MissionStageDirective::AddTimeSeconds { .. } => {
                        add_time_directive_count =
                            add_time_directive_count.saturating_add(1);
                    },
                    MissionStageDirective::ResetCheckpoint { .. } => {
                        checkpoint_directive_count =
                            checkpoint_directive_count.saturating_add(1);
                    },
                    MissionStageDirective::MessageIndex { .. } => {
                        message_directive_count =
                            message_directive_count.saturating_add(1);
                    },
                    MissionStageDirective::Vehicle(_) => {
                        stage_vehicle_directive_count =
                            stage_vehicle_directive_count.saturating_add(1);
                    },
                    MissionStageDirective::ActivateVehicle { .. } => {
                        stage_activate_vehicle_count =
                            stage_activate_vehicle_count.saturating_add(1);
                    },
                    MissionStageDirective::HudIcon { .. } => {
                        stage_hud_icon_count =
                            stage_hud_icon_count.saturating_add(1);
                    },
                    MissionStageDirective::FadeOutLegacyArgument { .. } => {
                        stage_fade_out_legacy_count =
                            stage_fade_out_legacy_count.saturating_add(1);
                    },
                    MissionStageDirective::IrisWipeLegacyArgument { .. } => {
                        stage_iris_wipe_legacy_count =
                            stage_iris_wipe_legacy_count.saturating_add(1);
                    },
                    MissionStageDirective::MaxTrafficCars { .. } => {
                        stage_max_traffic_count =
                            stage_max_traffic_count.saturating_add(1);
                    },
                    MissionStageDirective::VehicleAiTuning { .. } => {
                        stage_vehicle_ai_count =
                            stage_vehicle_ai_count.saturating_add(1);
                    },
                    MissionStageDirective::TargetCatchupTuning { .. } => {
                        stage_target_catchup_count =
                            stage_target_catchup_count.saturating_add(1);
                    },
                    MissionStageDirective::SafeZone { .. } => {
                        stage_safe_zone_count =
                            stage_safe_zone_count.saturating_add(1);
                    },
                    MissionStageDirective::StayInBlack { .. } => {
                        stage_stay_in_black_count =
                            stage_stay_in_black_count.saturating_add(1);
                    },
                    MissionStageDirective::GameOver { .. } => {
                        stage_game_over_count =
                            stage_game_over_count.saturating_add(1);
                    },
                    MissionStageDirective::LevelOver { .. } => {
                        stage_level_over_count =
                            stage_level_over_count.saturating_add(1);
                    },
                    MissionStageDirective::ShowStageComplete { .. } => {
                        stage_show_complete_count =
                            stage_show_complete_count.saturating_add(1);
                    },
                    MissionStageDirective::DisableHitAndRun { .. } => {
                        stage_disable_hit_and_run_count =
                            stage_disable_hit_and_run_count.saturating_add(1);
                    },
                    MissionStageDirective::StageCharacter { .. } => {
                        stage_character_count =
                            stage_character_count.saturating_add(1);
                    },
                    MissionStageDirective::CollectibleStateProp { .. }
                    | MissionStageDirective::StageMusicChange { .. }
                    | MissionStageDirective::CountdownSequenceEntry { .. }
                    | MissionStageDirective::MissionAbortAllowed { .. }
                    | MissionStageDirective::GotoPsScreenWhenDone {
                        ..
                    }
                    | MissionStageDirective::NoTrafficForStage { .. }
                    | MissionStageDirective::PlacePlayerCar { .. }
                    | MissionStageDirective::PutMfPlayerInCar { .. }
                    | MissionStageDirective::CharacterToHide { .. }
                    | MissionStageDirective::CompletionDialog { .. }
                    | MissionStageDirective::DemoLoopTime { .. }
                    | MissionStageDirective::MusicState { .. }
                    | MissionStageDirective::StagePresentationBitmap { .. }
                    | MissionStageDirective::RaceEntryFee { .. }
                    | MissionStageDirective::RaceCatchupTuning { .. }
                    | MissionStageDirective::StageMusicAlwaysOn { .. }
                    | MissionStageDirective::SwapDefaultCarLocator { .. }
                    | MissionStageDirective::SwapForcedCarLocator { .. }
                    | MissionStageDirective::SwapPlayerLocator { .. }
                    | MissionStageDirective::StageStartMusicEvent { .. }
                    | MissionStageDirective::StartCountdown { .. }
                    | MissionStageDirective::SwapInDefaultCar { .. }
                    | MissionStageDirective::UseElapsedTime { .. } => {
                        stage_closed_direct_only_count =
                            stage_closed_direct_only_count.saturating_add(1);
                    },
                    MissionStageDirective::Waypoint { .. } => {
                        waypoint_directive_count =
                            waypoint_directive_count.saturating_add(1);
                    },
                }
            }
        }
        mission_graph_count =
            mission_graph_count.saturating_add(scopes.missions().len());
        unscoped_command_count = unscoped_command_count
            .saturating_add(scopes.unscoped_commands().len());
        for mission in scopes.missions() {
            mission_command_count =
                mission_command_count.saturating_add(mission.commands().len());
            stage_count = stage_count.saturating_add(mission.stages().len());
            for stage in mission.stages() {
                stage_command_count =
                    stage_command_count.saturating_add(stage.commands().len());
                objective_command_count = objective_command_count
                    .saturating_add(stage.objective().commands().len());
                if !stage.objective().binding().is_mapped() {
                    unavailable_objective_count =
                        unavailable_objective_count.saturating_add(1);
                }
                for condition in stage.conditions() {
                    condition_command_count = condition_command_count
                        .saturating_add(condition.commands().len());
                    match condition.scope() {
                        crate::domain::MissionConditionScope::Stage => {
                            stage_condition_count =
                                stage_condition_count.saturating_add(1);
                        },
                        crate::domain::MissionConditionScope::Objective => {
                            objective_condition_count =
                                objective_condition_count.saturating_add(1);
                        },
                    }
                }
            }
        }
    }
    if objective_count != 611
        || typed_objective_parameter_count != 611
        || typed_objective_reference_count != 11
        || legacy_unrecognized_route_count != 1
        || condition_count != 408
        || typed_condition_parameter_count != 408
        || keepbarrel_legacy_value_count != 10
        || damage_legacy_token_count != 1
        || adaptation_count != 2
        || mission_graph_count != 154
        || unscoped_command_count != 7705
        || mission_command_count != 811
        || stage_command_count != 2454
        || objective_command_count != 3605
        || condition_command_count != 375
        || stage_count != 611
        || typed_stage_count != 611
        || stage_directive_count != 2549
        || stage_character_count != 11
        || stage_closed_direct_only_count != 502
        || standard_stage_count != 601
        || legacy_flag_stage_count != 530
        || final_stage_count != 90
        || locked_vehicle_stage_count != 6
        || locked_costume_stage_count != 4
        || set_time_directive_count != 98
        || add_time_directive_count != 64
        || checkpoint_directive_count != 119
        || message_directive_count != 449
        || stage_vehicle_directive_count != 133
        || stage_activate_vehicle_count != 37
        || stage_hud_icon_count != 397
        || stage_fade_out_legacy_count != 14
        || stage_iris_wipe_legacy_count != 6
        || stage_max_traffic_count != 127
        || stage_vehicle_ai_count != 52
        || stage_target_catchup_count != 7
        || stage_safe_zone_count != 4
        || stage_stay_in_black_count != 5
        || stage_game_over_count != 1
        || stage_level_over_count != 3
        || stage_show_complete_count != 108
        || stage_disable_hit_and_run_count != 24
        || waypoint_directive_count != 388
        || initialization_directive_count != 811
        || reset_in_car_count != 79
        || reset_out_car_count != 67
        || initial_walk_count != 6
        || dynamic_load_count != 146
        || street_race_props_load_count != 22
        || street_race_props_unload_count != 22
        || forced_car_count != 16
        || initial_vehicle_count != 16
        || mission_closed_remaining_count != 437
        || objective_semantic_count != 611
        || objective_directive_count != 3498
        || objective_npc_count != 294
        || objective_npc_waypoint_count != 180
        || objective_driver_count != 1
        || objective_remove_driver_count != 6
        || objective_remove_npc_count != 3
        || objective_target_vehicle_count != 84
        || objective_talk_target_count != 95
        || objective_npc_animation_count != 355
        || objective_player_animation_count != 357
        || objective_ambient_randomize_count != 198
        || objective_camera_best_side_count != 92
        || objective_conversation_camera_count != 261
        || objective_dialogue_info_count != 128
        || objective_dialogue_positions_count != 98
        || objective_distance_count != 14
        || objective_par_time_count != 7
        || objective_pickup_target_count != 4
        || objective_goto_dialog_off_count != 40
        || objective_must_action_trigger_count != 8
        || objective_allow_rock_out_count != 1
        || objective_destination_count != 187
        || objective_presentation_bitmap_count != 53
        || objective_fmv_info_count != 6
        || objective_duration_count != 26
        || objective_race_laps_count != 21
        || objective_coin_fee_count != 7
        || objective_collectible_count != 635
        || objective_collectible_effect_count != 301
        || objective_collectible_binding_count != 36
        || condition_semantic_count != 408
        || condition_directive_count != 375
        || condition_health_count != 111
        || condition_target_vehicle_count != 130
        || condition_target_boss_count != 4
        || condition_follow_distance_count != 11
        || condition_time_value_count != 98
        || condition_position_index_count != 17
        || condition_hit_and_run_no_op_count != 4
        || stage_condition_count != 402
        || objective_condition_count != 6
        || unavailable_objective_count != 2
        || empty_placeholder_count != 8
    {
        return Err(format!(
            concat!(
                "mission corpus inventory changed: files={} objectives={} ",
                "typed_objective_parameters={} typed_objective_references={} ",
                "legacy_unrecognized_routes={} conditions={} ",
                "typed_condition_parameters={} keepbarrel_legacy_values={} ",
                "damage_legacy_tokens={} adaptations={} missions={} ",
                "unscoped_commands={} ",
                "mission_commands={} stage_commands={} ",
                "objective_commands={} condition_commands={} stages={} ",
                "typed_stages={} standard_stages={} legacy_flag_stages={} ",
                "final_stages={} locked_vehicle_stages={} ",
                "locked_costume_stages={} set_time={} add_time={} ",
                "checkpoints={} messages={} stage_vehicles={} activate={} ",
                "hud_icons={} fade_unused={} iris_unused={} max_traffic={} ",
                "waypoints={} ",
                "objective_semantics={} objective_directives={} npcs={} ",
                "npc_waypoints={} drivers={} remove_drivers={} remove_npcs={} ",
                "target_vehicles={} talk_targets={} npc_animations={} ",
                "pc_animations={} dialogue_info={} ",
                "dialogue_positions={} destinations={} ",
                "presentation_bitmaps={} fmv_info={} durations={} ",
                "race_laps={} ",
                "coin_fees={} collectibles={} collectible_effects={} ",
                "collectible_bindings={} ",
                "condition_semantics={} condition_directives={} health={} ",
                "condition_vehicles={} bosses={} follow_distances={} ",
                "condition_times={} position_indices={} hitrun_noops={} ",
                "stage_conditions={} objective_conditions={} ",
                "unavailable_objectives={} empty_placeholders={}"
            ),
            sources.len(),
            objective_count,
            typed_objective_parameter_count,
            typed_objective_reference_count,
            legacy_unrecognized_route_count,
            condition_count,
            typed_condition_parameter_count,
            keepbarrel_legacy_value_count,
            damage_legacy_token_count,
            adaptation_count,
            mission_graph_count,
            unscoped_command_count,
            mission_command_count,
            stage_command_count,
            objective_command_count,
            condition_command_count,
            stage_count,
            typed_stage_count,
            standard_stage_count,
            legacy_flag_stage_count,
            final_stage_count,
            locked_vehicle_stage_count,
            locked_costume_stage_count,
            set_time_directive_count,
            add_time_directive_count,
            checkpoint_directive_count,
            message_directive_count,
            stage_vehicle_directive_count,
            stage_activate_vehicle_count,
            stage_hud_icon_count,
            stage_fade_out_legacy_count,
            stage_iris_wipe_legacy_count,
            stage_max_traffic_count,
            waypoint_directive_count,
            objective_semantic_count,
            objective_directive_count,
            objective_npc_count,
            objective_npc_waypoint_count,
            objective_driver_count,
            objective_remove_driver_count,
            objective_remove_npc_count,
            objective_target_vehicle_count,
            objective_talk_target_count,
            objective_npc_animation_count,
            objective_player_animation_count,
            objective_dialogue_info_count,
            objective_dialogue_positions_count,
            objective_destination_count,
            objective_presentation_bitmap_count,
            objective_fmv_info_count,
            objective_duration_count,
            objective_race_laps_count,
            objective_coin_fee_count,
            objective_collectible_count,
            objective_collectible_effect_count,
            objective_collectible_binding_count,
            condition_semantic_count,
            condition_directive_count,
            condition_health_count,
            condition_target_vehicle_count,
            condition_target_boss_count,
            condition_follow_distance_count,
            condition_time_value_count,
            condition_position_index_count,
            condition_hit_and_run_no_op_count,
            stage_condition_count,
            objective_condition_count,
            unavailable_objective_count,
            empty_placeholder_count,
        ));
    }
    Ok(())
}

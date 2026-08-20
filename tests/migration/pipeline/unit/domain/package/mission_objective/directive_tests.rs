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
//   - Unit evidence for selected objective directive semantics.
// - Must-Not:
//   - Infer runtime behavior beyond source-backed directive typing.
// - Allows:
//   - Verify participant, target, waypoint, and presentation field mapping.
// - Split-When:
//   - A directive family requires independent reusable fixtures.
// - Merge-When:
//   - Objective directive tests move into one complete mission graph suite.
// - Summary:
//   - Selected objective directive semantic regressions.
// - Description:
//   - Locks reviewed objective-scoped command mappings and relationships.
// - Usage:
//   - Compiled as a child of the objective directive domain module.
// - Defaults:
//   - Malformed, unsupported, and unresolved directives fail closed.
//

//! Selected objective directive semantic regressions.

use super::{
    MissionAmbientAnimationCharacter, MissionConversationCameraToken,
    MissionObjectiveDirective, MissionObjectiveNpcReference,
    MissionObjectiveSemanticReport, compile_directive,
    preflight_mission_objective_npc_waypoints,
};

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

#[test]
fn types_participants_targets_and_waypoints() -> Result<(), String> {
    let npc = compile_directive(
        "talkto",
        4,
        "addnpc",
        &strings(&["marge", "m1_marge_sd", "SimpsonsHouse"]),
    )?
    .ok_or_else(|| "NPC directive disappeared".to_owned())?;
    let MissionObjectiveDirective::Npc(npc) = npc else {
        return Err("NPC directive kind changed".to_owned());
    };
    if npc.npc_id() != "marge"
        || npc.locator_id() != "m1_marge_sd"
        || npc.unused_argument() != Some("SimpsonsHouse")
    {
        return Err("NPC field mapping changed".to_owned());
    }

    let waypoint = compile_directive(
        "talkto",
        5,
        "addobjectivenpcwaypoint",
        &strings(&["marge", "marge_walk_1"]),
    )?;
    if waypoint
        != Some(MissionObjectiveDirective::NpcWaypoint {
            source_ordinal: 5,
            npc_id: "marge".to_owned(),
            locator_id: "marge_walk_1".to_owned(),
        })
    {
        return Err("NPC waypoint mapping changed".to_owned());
    }

    let target = compile_directive(
        "follow",
        6,
        "setobjtargetvehicle",
        &strings(&["cVan"]),
    )?;
    if target
        != Some(MissionObjectiveDirective::TargetVehicle {
            source_ordinal: 6,
            vehicle_id: "cVan".to_owned(),
        })
    {
        return Err("target vehicle mapping changed".to_owned());
    }
    Ok(())
}

#[test]
fn preserves_authored_talk_target_optionality() -> Result<(), String> {
    let minimal = compile_directive(
        "talkto",
        7,
        "settalktotarget",
        &strings(&["burns"]),
    )?;
    if minimal
        != Some(MissionObjectiveDirective::TalkTarget {
            source_ordinal: 7,
            npc_id: "burns".to_owned(),
            icon: None,
            icon_y_offset: None,
            trigger_radius: None,
        })
    {
        return Err("minimal talk target acquired invented defaults".to_owned());
    }

    let explicit = compile_directive(
        "talkto",
        8,
        "settalktotarget",
        &strings(&["apu", "0", "-0.3", "3.0"]),
    )?;
    if explicit
        != Some(MissionObjectiveDirective::TalkTarget {
            source_ordinal: 8,
            npc_id: "apu".to_owned(),
            icon: Some(0),
            icon_y_offset: Some("-0.3".to_owned()),
            trigger_radius: Some("3.0".to_owned()),
        })
    {
        return Err("explicit talk target evidence changed".to_owned());
    }
    Ok(())
}

#[test]
fn types_timing_fees_driver_and_binding() -> Result<(), String> {
    let duration =
        compile_directive("timer", 9, "setdurationtime", &strings(&["0.5"]))?;
    if duration
        != Some(MissionObjectiveDirective::Duration {
            source_ordinal: 9,
            source_seconds: "0.5".to_owned(),
        })
    {
        return Err("duration mapping changed".to_owned());
    }
    let laps = compile_directive("race", 10, "setracelaps", &strings(&["5"]))?;
    if laps
        != Some(MissionObjectiveDirective::RaceLaps {
            source_ordinal: 10,
            laps: 5,
        })
    {
        return Err("race lap mapping changed".to_owned());
    }
    let fee = compile_directive("coins", 11, "setcoinfee", &strings(&["35"]))?;
    if fee
        != Some(MissionObjectiveDirective::CoinFee {
            source_ordinal: 11,
            coins: 35,
        })
    {
        return Err("coin fee mapping changed".to_owned());
    }
    let binding = compile_directive(
        "dump",
        12,
        "bindcollectibleto",
        &strings(&["1", "3"]),
    )?;
    if binding
        != Some(MissionObjectiveDirective::BindCollectibleToWaypoint {
            source_ordinal: 12,
            collectible_index: 1,
            waypoint_index: 3,
        })
    {
        return Err("collectible binding mapping changed".to_owned());
    }
    let driver = compile_directive(
        "race",
        13,
        "adddriver",
        &strings(&["snake", "snake_v"]),
    )?;
    if driver
        != Some(MissionObjectiveDirective::Driver {
            source_ordinal: 13,
            npc_id: "snake".to_owned(),
            vehicle_id: "snake_v".to_owned(),
        })
    {
        return Err("driver mapping changed".to_owned());
    }
    Ok(())
}

#[test]
fn rejects_cross_alias_and_invalid_selected_values() -> Result<(), String> {
    for (alias, command, arguments) in [
        ("goto", "setdurationtime", strings(&["1"])),
        ("talkto", "settalktotarget", strings(&["marge", "3", "0"])),
        ("timer", "setdurationtime", strings(&["NaN"])),
        ("timer", "setdurationtime", strings(&["1e3"])),
        ("timer", "setdurationtime", strings(&["+1"])),
        ("timer", "setdurationtime", strings(&[".5"])),
        ("timer", "setdurationtime", strings(&["1."])),
        ("timer", "setdurationtime", strings(&["0"])),
        ("race", "setracelaps", strings(&["0"])),
        ("race", "setracelaps", strings(&["+1"])),
        ("dump", "bindcollectibleto", strings(&["-1", "0"])),
        ("follow", "setobjtargetvehicle", strings(&["../car"])),
    ] {
        if compile_directive(alias, 14, command, &arguments).is_ok() {
            return Err(format!(
                "invalid typed objective directive was accepted: {command}"
            ));
        }
    }
    Ok(())
}

#[test]
fn types_collectibles_without_interpreting_legacy_extensions()
-> Result<(), String> {
    let two = compile_directive(
        "delivery",
        15,
        "addcollectible",
        &strings(&["m1_tomato", "scien"]),
    )?;
    if two
        != Some(MissionObjectiveDirective::Collectible {
            source_ordinal: 15,
            locator_id: "m1_tomato".to_owned(),
            drawable_id: Some("scien".to_owned()),
            legacy_arguments: Vec::new(),
        })
    {
        return Err("two-field collectible mapping changed".to_owned());
    }
    let legacy = compile_directive(
        "delivery",
        16,
        "addcollectible",
        &strings(&["m3_ketchup7", "ketchup", "ketchup", "cletus"]),
    )?;
    if legacy
        != Some(MissionObjectiveDirective::Collectible {
            source_ordinal: 16,
            locator_id: "m3_ketchup7".to_owned(),
            drawable_id: Some("ketchup".to_owned()),
            legacy_arguments: strings(&["ketchup", "cletus"]),
        })
    {
        return Err("legacy collectible extension evidence changed".to_owned());
    }
    let effect = compile_directive(
        "delivery",
        17,
        "setcollectibleeffect",
        &strings(&["wrench_collect"]),
    )?;
    if effect
        != Some(MissionObjectiveDirective::CollectibleEffect {
            source_ordinal: 17,
            effect_id: "wrench_collect".to_owned(),
        })
    {
        return Err("collectible effect mapping changed".to_owned());
    }
    Ok(())
}

#[test]
fn types_destination_bitmap_and_fmv_source_references() -> Result<(), String> {
    let destination = compile_directive(
        "goto",
        18,
        "setdestination",
        &strings(&["m2_simpsonhouse_sd", "carsphere"]),
    )?;
    if destination
        != Some(MissionObjectiveDirective::Destination {
            source_ordinal: 18,
            destination_id: "m2_simpsonhouse_sd".to_owned(),
            marker_id: Some("carsphere".to_owned()),
        })
    {
        return Err("destination source mapping changed".to_owned());
    }
    let bitmap = compile_directive(
        "dialogue",
        19,
        "setpresentationbitmap",
        &strings(&["art/frontend/dynaload/images/mis01_00.p3d"]),
    )?;
    if bitmap
        != Some(MissionObjectiveDirective::PresentationBitmap {
            source_ordinal: 19,
            p3d_path: "art/frontend/dynaload/images/mis01_00.p3d".to_owned(),
        })
    {
        return Err("presentation bitmap source mapping changed".to_owned());
    }
    let fmv = compile_directive(
        "fmv",
        20,
        "setfmvinfo",
        &strings(&["fmv7.rmv", "stopmusic"]),
    )?;
    if fmv
        != Some(MissionObjectiveDirective::FmvInfo {
            source_ordinal: 20,
            rmv_path: "fmv7.rmv".to_owned(),
            legacy_argument: Some("stopmusic".to_owned()),
        })
    {
        return Err("FMV source mapping changed".to_owned());
    }
    Ok(())
}

#[test]
fn types_dialogue_participants_and_position_sources() -> Result<(), String> {
    let info = compile_directive(
        "dialogue",
        21,
        "setdialogueinfo",
        &strings(&["homer", "marge", "icecream", "0"]),
    )?;
    if info
        != Some(MissionObjectiveDirective::DialogueInfo {
            source_ordinal: 21,
            player_character_id: "homer".to_owned(),
            npc_character_id: "marge".to_owned(),
            dialogue_id: "icecream".to_owned(),
            legacy_zero: "0".to_owned(),
        })
    {
        return Err("dialogue-info mapping changed".to_owned());
    }
    let positions = compile_directive(
        "dialogue",
        22,
        "setdialoguepositions",
        &strings(&["m2_homer_ned", "m2_ned_sd", "mission2_carstart", "1"]),
    )?;
    if positions
        != Some(MissionObjectiveDirective::DialoguePositions {
            source_ordinal: 22,
            locator_ids: [
                "m2_homer_ned".to_owned(),
                "m2_ned_sd".to_owned(),
                "mission2_carstart".to_owned(),
            ],
            legacy_flag: Some("1".to_owned()),
        })
    {
        return Err("dialogue-position mapping changed".to_owned());
    }
    for arguments in [
        strings(&["homer", "marge", "icecream", "1"]),
        strings(&["a", "b", "c", "0"]),
        strings(&["../a", "b", "c"]),
    ] {
        let command = if arguments.len() == 4
            && arguments.first().is_some_and(|value| value == "homer")
        {
            "setdialogueinfo"
        } else {
            "setdialoguepositions"
        };
        if compile_directive("dialogue", 23, command, &arguments).is_ok() {
            return Err(format!(
                "unreviewed dialogue shape accepted: {command}"
            ));
        }
    }
    Ok(())
}

#[test]
fn types_objective_dialogue_animation_references() -> Result<(), String> {
    let npc = compile_directive(
        "dialogue",
        24,
        "addambientnpcanimation",
        &strings(&["dialogue_shaking_fist"]),
    )?;
    if npc
        != Some(MissionObjectiveDirective::AmbientNpcAnimation {
            source_ordinal: 24,
            animation_id: "dialogue_shaking_fist".to_owned(),
        })
    {
        return Err("NPC ambient animation mapping changed".to_owned());
    }
    let pc = compile_directive(
        "dialogue",
        25,
        "addambientpcanimation",
        &strings(&["dialogue_hands_in_air"]),
    )?;
    if pc
        != Some(MissionObjectiveDirective::AmbientPcAnimation {
            source_ordinal: 25,
            animation_id: "dialogue_hands_in_air".to_owned(),
        })
    {
        return Err("player ambient animation mapping changed".to_owned());
    }
    assert!(
        compile_directive(
            "dialogue",
            26,
            "addambientnpcanimation",
            &strings(&["dialogue_yes", "sr1"]),
        )
        .is_err()
    );
    Ok(())
}

#[test]
fn types_camera_animation_controls() -> Result<(), String> {
    let best_side = compile_directive(
        "dialogue",
        30,
        "setcambestside",
        &strings(&["m7_bestside"]),
    )?;
    if best_side
        != Some(MissionObjectiveDirective::CameraBestSide {
            source_ordinal: 30,
            locator_id: "m7_bestside".to_owned(),
        })
    {
        return Err("camera best-side mapping changed".to_owned());
    }
    let camera = compile_directive(
        "dialogue",
        31,
        "setconversationcam",
        &strings(&["6", "npc_far"]),
    )?;
    if camera
        != Some(MissionObjectiveDirective::ConversationCamera {
            source_ordinal: 31,
            source_slot: 6,
            camera: MissionConversationCameraToken::NpcFar,
        })
    {
        return Err("conversation-camera mapping changed".to_owned());
    }
    for (selector, expected) in [
        ("0", MissionAmbientAnimationCharacter::Player),
        ("1", MissionAmbientAnimationCharacter::Npc),
    ] {
        let actual = compile_directive(
            "dialogue",
            32,
            "ambientanimationrandomize",
            &strings(&[selector, "0"]),
        )?;
        if actual
            != Some(MissionObjectiveDirective::AmbientAnimationRandomize {
                source_ordinal: 32,
                character: expected,
                randomized: false,
            })
        {
            return Err("ambient-animation selector mapping changed".to_owned());
        }
    }
    let randomized = compile_directive(
        "dialogue",
        33,
        "ambientanimationrandomize",
        &strings(&["0", "1"]),
    )?;
    if randomized
        != Some(MissionObjectiveDirective::AmbientAnimationRandomize {
            source_ordinal: 33,
            character: MissionAmbientAnimationCharacter::Player,
            randomized: true,
        })
    {
        return Err(
            "ambient-animation randomization flag mapping changed".to_owned()
        );
    }
    Ok(())
}

#[test]
fn types_distance_pickup_par_time_and_exact_markers() -> Result<(), String> {
    let distance = compile_directive(
        "losetail",
        34,
        "setobjdistance",
        &strings(&["150"]),
    )?;
    if distance
        != Some(MissionObjectiveDirective::ObjectDistance {
            source_ordinal: 34,
            source_value: 150,
        })
    {
        return Err("objective source-distance mapping changed".to_owned());
    }
    let par_time =
        compile_directive("race", 35, "setpartime", &strings(&["195"]))?;
    if par_time
        != Some(MissionObjectiveDirective::ParTime {
            source_ordinal: 35,
            source_value: 195,
        })
    {
        return Err("race par-time mapping changed".to_owned());
    }
    let pickup = compile_directive(
        "pickupitem",
        36,
        "setpickuptarget",
        &strings(&["bombbarrel"]),
    )?;
    if pickup
        != Some(MissionObjectiveDirective::PickupTarget {
            source_ordinal: 36,
            target_id: "bombbarrel".to_owned(),
        })
    {
        return Err("pickup target mapping changed".to_owned());
    }
    for (command, expected) in [
        (
            "turngotodialogoff",
            MissionObjectiveDirective::TurnGotoDialogOff { source_ordinal: 37 },
        ),
        (
            "mustactiontrigger",
            MissionObjectiveDirective::MustActionTrigger { source_ordinal: 37 },
        ),
        ("allowrockout", MissionObjectiveDirective::AllowRockOut {
            source_ordinal: 37,
        }),
    ] {
        let actual = compile_directive("goto", 37, command, &[])?;
        if actual != Some(expected) {
            return Err(format!("objective marker mapping changed: {command}"));
        }
    }
    Ok(())
}

#[test]
fn rejects_unreviewed_camera_animation_and_source_values() -> Result<(), String>
{
    for (alias, command, arguments) in [
        ("dialogue", "setconversationcam", strings(&["7", "pc_far"])),
        ("dialogue", "setconversationcam", strings(&["+1", "pc_far"])),
        (
            "dialogue",
            "setconversationcam",
            strings(&["1", "hero_far"]),
        ),
        (
            "dialogue",
            "ambientanimationrandomize",
            strings(&["2", "0"]),
        ),
        (
            "dialogue",
            "ambientanimationrandomize",
            strings(&["0", "2"]),
        ),
        ("losetail", "setobjdistance", strings(&["0"])),
        ("race", "setpartime", strings(&["0"])),
        ("pickupitem", "setpickuptarget", strings(&["../bombbarrel"])),
        ("talkto", "setconversationcam", strings(&["0", "pc_far"])),
    ] {
        if compile_directive(alias, 38, command, &arguments).is_ok() {
            return Err(format!(
                "unreviewed objective control accepted: {command}"
            ));
        }
    }
    Ok(())
}

#[test]
fn pins_external_semantic_owners_for_objective_commands() -> Result<(), String>
{
    for command in [
        "activatevehicle",
        "addsafezone",
        "addstagecharacter",
        "addstagevehicle",
        "disablehitandrun",
        "setgameover",
        "setlevelover",
        "setstageaitargetcatchupparams",
        "setstagemessageindex",
        "setvehicleaiparams",
        "stayinblack",
        "addcondition",
        "closecondition",
    ] {
        if !super::command_has_external_semantic_owner(command) {
            return Err(format!(
                "external semantic owner disappeared: {command}"
            ));
        }
    }
    for command in ["setdestination", "setcoinfee", "setconversationcam"] {
        if super::command_has_external_semantic_owner(command) {
            return Err(format!(
                "objective-owned command escaped ownership: {command}"
            ));
        }
    }
    Ok(())
}


#[test]
fn semantic_binding_preserves_canonical_kind_and_unavailability(
) -> Result<(), String> {
    let mapped = MissionObjectiveSemanticReport::from_route_entries_for_tests(
        vec![(2, 0, 3, "goto".to_owned(), Vec::new())],
    );
    let [mapped] = mapped.objectives() else {
        return Err("mapped objective fixture changed count".to_owned());
    };
    assert_eq!(mapped.owner_stage_source_ordinal(), 2);
    assert_eq!(mapped.owner_stage_sequence_ordinal(), 0);
    assert_eq!(mapped.source_alias(), "goto");
    assert_eq!(mapped.canonical_kind(), Some("travel"));
    assert_eq!(mapped.unavailable_code(), None);

    let unavailable =
        MissionObjectiveSemanticReport::from_route_entries_for_tests(vec![(
            2,
            0,
            4,
            "dummy".to_owned(),
            Vec::new(),
        )]);
    let [unavailable] = unavailable.objectives() else {
        return Err("unavailable objective fixture changed count".to_owned());
    };
    assert_eq!(unavailable.source_alias(), "dummy");
    assert_eq!(unavailable.canonical_kind(), None);
    assert_eq!(
        unavailable.unavailable_code(),
        Some("legacy-dummy-objective-unavailable-v1")
    );
    Ok(())
}

#[test]
fn binds_objective_npc_waypoints_to_prior_declaration() -> Result<(), String> {
    let report = MissionObjectiveSemanticReport::from_route_entries_for_tests(
        vec![(
            2,
            0,
            3,
            "talkto".to_owned(),
            vec![
                MissionObjectiveDirective::Npc(MissionObjectiveNpcReference {
                    source_ordinal: 4,
                    npc_id: "marge".to_owned(),
                    locator_id: "marge_start".to_owned(),
                    unused_argument: None,
                }),
                MissionObjectiveDirective::NpcWaypoint {
                    source_ordinal: 5,
                    npc_id: "marge".to_owned(),
                    locator_id: "marge_walk_1".to_owned(),
                },
                MissionObjectiveDirective::NpcWaypoint {
                    source_ordinal: 6,
                    npc_id: "marge".to_owned(),
                    locator_id: "marge_walk_2".to_owned(),
                },
            ],
        )],
    );
    let result = preflight_mission_objective_npc_waypoints(&report)?;
    let [first, second] = result.waypoints() else {
        return Err("objective NPC waypoint count drifted".to_owned());
    };
    assert_eq!(first.owner_stage_source_ordinal(), 2);
    assert_eq!(first.owner_stage_sequence_ordinal(), 0);
    assert_eq!(first.objective_source_ordinal(), 3);
    assert_eq!(first.source_ordinal(), 5);
    assert_eq!(first.declaration_source_ordinal(), 4);
    assert_eq!(first.npc_id(), "marge");
    assert_eq!(first.npc_locator_id(), "marge_start");
    assert_eq!(second.waypoint_locator_id(), "marge_walk_2");
    Ok(())
}

#[test]
fn rejects_objective_waypoint_without_unique_prior_npc() -> Result<(), String> {
    let report = MissionObjectiveSemanticReport::from_route_entries_for_tests(
        vec![(
            2,
            0,
            3,
            "talkto".to_owned(),
            vec![MissionObjectiveDirective::NpcWaypoint {
                source_ordinal: 5,
                npc_id: "marge".to_owned(),
                locator_id: "marge_walk_1".to_owned(),
            }],
        )],
    );
    let result = preflight_mission_objective_npc_waypoints(&report);
    let Err(error) = result else {
        return Err("orphan objective NPC waypoint must fail".to_owned());
    };
    assert!(error.contains("no unique prior declaration"));
    Ok(())
}

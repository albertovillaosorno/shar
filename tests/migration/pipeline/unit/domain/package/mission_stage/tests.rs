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
//   - Mission stage semantic compiler regression tests.
// - Must-Not:
//   - Production behavior or private mission-script evidence.
// - Allows:
//   - Synthetic mission documents and typed stage assertions.
// - Split-When:
//   - One stage semantic family gains independent test ownership.
// - Merge-When:
//   - Stage semantics lose independent compiler behavior.
// - Summary:
//   - Mission stage semantic compiler regressions.
// - Description:
//   - Preserves focused unit evidence for the owning package-domain behavior.
// - Usage:
//   - Included only by the owning package-domain module under cfg(test).
// - Defaults:
//   - Invalid or ambiguous synthetic evidence fails closed.
//

//! Mission stage semantic compiler regressions.

use serde_json::{Value, json};

use super::{
    MissionStageDirective, MissionStageKind, MissionStageMessageKind,
    preflight_mission_stage_semantics,
};
use crate::domain::compile_mission_scope_graphs;
use crate::preflight_mission_script;

fn stage_document(
    stage_arguments: &[&str],
    message_index: &str,
    con_file: &str,
) -> Result<String, String> {
    let stage_raw = stage_arguments
        .iter()
        .map(|value| format!("\"{value}\""))
        .collect::<Vec<_>>()
        .join(",");
    let mut counts = serde_json::Map::new();
    for command in [
        "selectmission",
        "addstage",
        "setstagemessageindex",
        "setstagetime",
        "reset_to_here",
        "addstagevehicle",
        "addstagewaypoint",
        "addobjective",
        "closeobjective",
        "closestage",
        "closemission",
    ] {
        drop(counts.insert(command.to_owned(), json!(1)));
    }
    serde_json::to_string(&json!({
        "schema":"shar-schoenwald.straggler.mission-script.v3",
        "source_extension":"mfk","route_class":"mission","source_bytes":256,
        "context_command_count":6,"context_adaptation_count":0,
        "context_adaptations":[],
        "context_finding_count":0,
        "context_findings":[],
        "statement_count":11,
        "unique_command_count":11,
        "load_p3d_reference_count":0,
        "mission_flow_command_count":10,"vehicle_physics_command_count":0,
        "semantic_family":"mission-script","command_counts":counts,
        "source_statements":[
            "SelectMission(\"m1\");",format!("AddStage({stage_raw});"),
                        // jig-ignore-next-line: literal
            format!("SetStageMessageIndex({message_index});"),"SetStageTime(30);",
                        // jig-ignore-next-line: literal
                        "RESET_TO_HERE();",format!("AddStageVehicle(\"skinn_v\",\"m1_car\",\"chase\",\"{con_file}\",\"skinner\");"),
            "AddStageWaypoint(\"m1_path1\");","AddObjective(\"dummy\");",
            "CloseObjective();","CloseStage();","CloseMission();"
        ],
        "p3d_references":[],
        "command_invocations":[
            invocation(1,"selectmission","\"m1\"","mission-script", &["m1"]),
            invocation(2,"addstage",&stage_raw,"mission-stage",stage_arguments),
                        // jig-ignore-next-line: literal
            invocation(3,"setstagemessageindex",message_index,"mission-stage", &[message_index]),
            invocation(4,"setstagetime","30","mission-stage", &["30"]),
            invocation(5,"reset_to_here","","mission-script", &[]),
                        // jig-ignore-next-line: literal
            invocation(6,"addstagevehicle",&format!("\"skinn_v\",\"m1_car\",\"chase\",\"{con_file}\",\"skinner\""),"mission-stage", &["skinn_v","m1_car","chase",con_file,"skinner"]),
                        // jig-ignore-next-line: literal
            invocation(7,"addstagewaypoint","\"m1_path1\"","mission-stage", &["m1_path1"]),
                        // jig-ignore-next-line: literal
            invocation(8,"addobjective","\"dummy\"","mission-objective", &["dummy"]),
            invocation(9,"closeobjective","","mission-objective", &[]),
            invocation(10,"closestage","","mission-stage", &[]),
            invocation(11,"closemission","","mission-script", &[])
        ]
    }))
    .map_err(|error| error.to_string())
}

fn invocation(
    ordinal: usize,
    name: &str,
    args_raw: &str,
    semantic_role: &str,
    arguments: &[&str],
) -> Value {
    json!({
        "ordinal":ordinal,"name":name,"args_raw":args_raw,
        "semantic_role":semantic_role,"arguments":arguments
    })
}

fn semantics(
    stage_arguments: &[&str],
    message_index: &str,
    con_file: &str,
) -> Result<super::MissionStageSemanticReport, String> {
    let evidence = preflight_mission_script(&stage_document(
        stage_arguments,
        message_index,
        con_file,
    )?)?;
    let scopes = compile_mission_scope_graphs(&evidence)?;
    preflight_mission_stage_semantics(&scopes)
}

#[test]
fn types_locked_stage_timer_checkpoint_vehicle_and_waypoint()
-> Result<(), String> {
    let report = semantics(
        &["locked", "car", "homer_v"],
        "3",
        r"Missions\level01\M1Chase.con",
    )?;
    let [stage] = report.stages() else {
        return Err("typed stage fixture count changed".to_owned());
    };
    if stage.kind()
        != &(MissionStageKind::LockedVehicle {
            vehicle_id: "homer_v".to_owned(),
        })
        || stage.directives().len() != 5
    {
        return Err("locked stage header or directive count changed".to_owned());
    }
    let mut message = false;
    let mut timer = false;
    let mut checkpoint = false;
    let mut vehicle = false;
    let mut waypoint = false;
    for directive in stage.directives() {
        match directive {
            MissionStageDirective::MessageIndex { kind, index, .. } => {
                message =
                    *kind == MissionStageMessageKind::Locked && *index == 3;
            },
            MissionStageDirective::SetTimeSeconds { seconds, .. } => {
                timer = *seconds == 30;
            },
            MissionStageDirective::ResetCheckpoint { .. } => checkpoint = true,
            MissionStageDirective::Vehicle(reference) => {
                vehicle = reference.vehicle_id() == "skinn_v"
                    && reference.locator_id() == "m1_car"
                    && reference.behaviour() == "chase"
                    && reference.driver_id() == Some("skinner");
            },
            MissionStageDirective::Waypoint { locator_id, .. } => {
                waypoint = locator_id == "m1_path1";
            },
            MissionStageDirective::AddTimeSeconds { .. }
            | MissionStageDirective::ActivateVehicle { .. }
            | MissionStageDirective::HudIcon { .. }
            | MissionStageDirective::FadeOutLegacyArgument { .. }
            | MissionStageDirective::IrisWipeLegacyArgument { .. }
            | MissionStageDirective::MaxTrafficCars { .. }
            | MissionStageDirective::VehicleAiTuning { .. }
            | MissionStageDirective::TargetCatchupTuning { .. }
            | MissionStageDirective::SafeZone { .. }
            | MissionStageDirective::StayInBlack { .. }
            | MissionStageDirective::GameOver { .. }
            | MissionStageDirective::LevelOver { .. }
            | MissionStageDirective::ShowStageComplete { .. }
            | MissionStageDirective::DisableHitAndRun { .. }
            | MissionStageDirective::CollectibleStateProp { .. }
            | MissionStageDirective::StageCharacter { .. }
            | MissionStageDirective::StageMusicChange { .. }
            | MissionStageDirective::CountdownSequenceEntry { .. }
            | MissionStageDirective::MissionAbortAllowed { .. }
            | MissionStageDirective::GotoPsScreenWhenDone { .. }
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
            | MissionStageDirective::UseElapsedTime { .. } => {},
        }
    }
    if !(message && timer && checkpoint && vehicle && waypoint) {
        return Err("typed stage directive evidence changed".to_owned());
    }
    Ok(())
}

#[test]
fn rejects_message_namespace_and_vehicle_path_drift() -> Result<(), String> {
    let locked = semantics(
        &["locked", "skin", "b_casual"],
        "20",
        r"Missions\level01\M1Chase.con",
    );
    if locked.is_ok() {
        return Err("out-of-range locked message was accepted".to_owned());
    }
    let bad_con = semantics(&[], "20", r"..\escape.con");
    if bad_con.is_ok() {
        return Err("escaping stage vehicle CON path was accepted".to_owned());
    }
    Ok(())
}

#[test]
fn types_standard_and_final_message_namespace() -> Result<(), String> {
    for stage_arguments in [&[][..], &["final"][..]] {
        let report =
            semantics(stage_arguments, "299", r"Missions\level01\M1Chase.con")?;
        let [stage] = report.stages() else {
            return Err("message namespace fixture count changed".to_owned());
        };
        if !stage.directives().iter().any(|directive| {
            matches!(directive, MissionStageDirective::MessageIndex {
                kind: MissionStageMessageKind::Objective,
                index: 299,
                ..
            })
        }) {
            return Err("normal/final message namespace changed".to_owned());
        }
    }
    Ok(())
}

#[test]
fn types_activation_hud_and_documented_unused_transition_arguments()
-> Result<(), String> {
    let activate = super::compile_stage_directive(
        &(MissionStageKind::Standard {
            legacy_flags: None,
            final_stage: false,
        }),
        21,
        "activatevehicle",
        &["cMilk".to_owned(), "NULL".to_owned(), "target".to_owned()],
    )?;
    if activate
        != Some(MissionStageDirective::ActivateVehicle {
            source_ordinal: 21,
            vehicle_id: "cMilk".to_owned(),
            locator_id: "NULL".to_owned(),
            behaviour: "target".to_owned(),
        })
    {
        return Err("stage vehicle activation mapping changed".to_owned());
    }
    let hud = super::compile_stage_directive(
        &(MissionStageKind::Standard {
            legacy_flags: None,
            final_stage: false,
        }),
        22,
        "sethudicon",
        &["simpsons".to_owned()],
    )?;
    if hud
        != Some(MissionStageDirective::HudIcon {
            source_ordinal: 22,
            sprite_id: "simpsons".to_owned(),
        })
    {
        return Err("stage HUD icon mapping changed".to_owned());
    }
    for (name, expected) in [
        ("setfadeout", MissionStageDirective::FadeOutLegacyArgument {
            source_ordinal: 23,
            source_value: "0.1".to_owned(),
        }),
        (
            "setiriswipe",
            MissionStageDirective::IrisWipeLegacyArgument {
                source_ordinal: 23,
                source_value: "0.1".to_owned(),
            },
        ),
    ] {
        let actual = super::compile_stage_directive(
            &(MissionStageKind::Standard {
                legacy_flags: None,
                final_stage: false,
            }),
            23,
            name,
            &["0.1".to_owned()],
        )?;
        if actual != Some(expected) {
            return Err(format!("{name} legacy argument mapping changed"));
        }
    }
    Ok(())
}

#[test]
fn types_reviewed_max_traffic_domain() -> Result<(), String> {
    let stage = MissionStageKind::Standard {
        legacy_flags: None,
        final_stage: false,
    };
    for cars in 1_u8..=5 {
        let source = cars.to_string();
        let actual = super::compile_stage_directive(
            &stage,
            usize::from(cars),
            "setmaxtraffic",
            &[source],
        )?;
        if actual
            != Some(MissionStageDirective::MaxTrafficCars {
                source_ordinal: usize::from(cars),
                cars,
            })
        {
            return Err(format!("max-traffic mapping changed for {cars}"));
        }
    }
    for source in ["0", "6", "+1", "-1"] {
        if super::compile_stage_directive(&stage, 9, "setmaxtraffic", &[
            source.to_owned()
        ])
        .is_ok()
        {
            return Err(format!(
                "unreviewed max-traffic value accepted: {source}"
            ));
        }
    }
    Ok(())
}

#[test]
fn types_ai_safe_zone_and_stage_markers() -> Result<(), String> {
    let stage = MissionStageKind::Standard {
        legacy_flags: None,
        final_stage: false,
    };
    let ai =
        super::compile_stage_directive(&stage, 30, "setvehicleaiparams", &[
            "cHears".to_owned(),
            "-49".to_owned(),
            "-50".to_owned(),
        ])?;
    if ai
        != Some(MissionStageDirective::VehicleAiTuning {
            source_ordinal: 30,
            vehicle_id: "cHears".to_owned(),
            source_first: -49,
            source_second: -50,
        })
    {
        return Err("vehicle AI source tuple changed".to_owned());
    }
    let catchup = super::compile_stage_directive(
        &stage,
        31,
        "setstageaitargetcatchupparams",
        &["pizza".to_owned(), "20".to_owned(), "70".to_owned()],
    )?;
    if catchup
        != Some(MissionStageDirective::TargetCatchupTuning {
            source_ordinal: 31,
            vehicle_id: "pizza".to_owned(),
            source_first: 20,
            source_second: 70,
        })
    {
        return Err("target-catchup source tuple changed".to_owned());
    }
    let safe_zone =
        super::compile_stage_directive(&stage, 32, "addsafezone", &[
            "bm2_svt".to_owned(),
            "30".to_owned(),
        ])?;
    if safe_zone
        != Some(MissionStageDirective::SafeZone {
            source_ordinal: 32,
            locator_id: "bm2_svt".to_owned(),
            source_value: 30,
        })
    {
        return Err("safe-zone source tuple changed".to_owned());
    }
    for (name, expected) in [
        ("stayinblack", MissionStageDirective::StayInBlack {
            source_ordinal: 33,
        }),
        ("setgameover", MissionStageDirective::GameOver {
            source_ordinal: 33,
        }),
        ("setlevelover", MissionStageDirective::LevelOver {
            source_ordinal: 33,
        }),
        (
            "showstagecomplete",
            MissionStageDirective::ShowStageComplete { source_ordinal: 33 },
        ),
        (
            "disablehitandrun",
            MissionStageDirective::DisableHitAndRun { source_ordinal: 33 },
        ),
    ] {
        let actual = super::compile_stage_directive(&stage, 33, name, &[])?;
        if actual != Some(expected) {
            return Err(format!("stage source marker mapping changed: {name}"));
        }
    }
    for (name, arguments) in [
        ("setvehicleaiparams", vec!["car", "+1", "2"]),
        ("setstageaitargetcatchupparams", vec!["car", "1.0", "2"]),
        ("addsafezone", vec!["zone", "0"]),
        ("showstagecomplete", vec!["unexpected"]),
    ] {
        let arguments =
            arguments.into_iter().map(str::to_owned).collect::<Vec<_>>();
        if super::compile_stage_directive(&stage, 34, name, &arguments).is_ok()
        {
            return Err(format!(
                "unreviewed stage source tuple accepted: {name}"
            ));
        }
    }
    Ok(())
}

#[test]
fn closes_remaining_direct_stage_source_shapes() -> Result<(), String> {
    let countdown = super::compile_direct_stage_only_directive(
        40,
        "addtocountdownsequence",
        &["GO".to_owned(), "400".to_owned()],
    )?;
    if countdown
        != Some(MissionStageDirective::CountdownSequenceEntry {
            source_ordinal: 40,
            token: "GO".to_owned(),
            duration_milliseconds: 400,
        })
    {
        return Err("countdown-sequence source mapping changed".to_owned());
    }
    let demo =
        super::compile_direct_stage_only_directive(41, "setdemolooptime", &[
            "40000000000".to_owned(),
        ])?;
    if demo
        != Some(MissionStageDirective::DemoLoopTime {
            source_ordinal: 41,
            source_value: 40_000_000_000,
        })
    {
        return Err("demo-loop source mapping changed".to_owned());
    }
    let catchup = super::compile_direct_stage_only_directive(
        42,
        "setstageairacecatchupparams",
        &[
            "honor_v".to_owned(),
            "80".to_owned(),
            "0.6".to_owned(),
            "0.9".to_owned(),
            "1.50".to_owned(),
        ],
    )?;
    if catchup
        != Some(MissionStageDirective::RaceCatchupTuning {
            source_ordinal: 42,
            vehicle_id: "honor_v".to_owned(),
            source_value: 80,
            source_factors: [
                "0.6".to_owned(),
                "0.9".to_owned(),
                "1.50".to_owned(),
            ],
        })
    {
        return Err("race catch-up source mapping changed".to_owned());
    }
    let bitmap = super::compile_direct_stage_only_directive(
        43,
        "setpresentationbitmap",
        &["art/frontend/dynaload/images/mis01_00.p3d".to_owned()],
    )?;
    if bitmap
        != Some(MissionStageDirective::StagePresentationBitmap {
            source_ordinal: 43,
            p3d_path: "art/frontend/dynaload/images/mis01_00.p3d".to_owned(),
        })
    {
        return Err("stage presentation bitmap mapping changed".to_owned());
    }
    let stage = MissionStageKind::Standard {
        legacy_flags: None,
        final_stage: false,
    };
    let character =
        super::compile_stage_directive(&stage, 44, "addstagecharacter", &[
            "homer".to_owned(),
            "m5_homer_end".to_owned(),
            "current".to_owned(),
            "m5_car_end".to_owned(),
        ])?;
    if character
        != Some(MissionStageDirective::StageCharacter {
            source_ordinal: 44,
            character_id: "homer".to_owned(),
            character_locator_id: Some("m5_homer_end".to_owned()),
            vehicle_id: "current".to_owned(),
            vehicle_locator_id: "m5_car_end".to_owned(),
        })
    {
        return Err("stage character mapping changed".to_owned());
    }
    for (name, arguments) in [
        ("allowmissionabort", vec!["true"]),
        ("setpresentationbitmap", vec!["../escape.p3d"]),
        ("setstageairacecatchupparams", vec![
            "car", "80", "1e0", "1.0", "1.0",
        ]),
    ] {
        let arguments =
            arguments.into_iter().map(str::to_owned).collect::<Vec<_>>();
        if super::compile_direct_stage_only_directive(45, name, &arguments)
            .is_ok()
        {
            return Err(format!(
                "unreviewed direct-stage source accepted: {name}"
            ));
        }
    }
    Ok(())
}

#[test]
fn pins_objective_commands_delegated_to_stage_semantics() -> Result<(), String>
{
    let delegated = [
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
    ];
    for command in delegated {
        if !super::objective_command_has_stage_semantics(command) {
            return Err(format!(
                "delegated stage command disappeared: {command}"
            ));
        }
    }
    for command in ["setdestination", "setcoinfee", "addcondition"] {
        if super::objective_command_has_stage_semantics(command) {
            return Err(format!(
                "objective-only command delegated to stage: {command}"
            ));
        }
    }
    Ok(())
}

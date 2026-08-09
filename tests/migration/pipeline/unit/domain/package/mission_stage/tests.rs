// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT

//! Mission stage semantic compiler regressions.

use serde_json::{Value, json};

use super::{
    MissionStageDirective, MissionStageKind, MissionStageMessageKind,
    preflight_mission_stage_semantics,
};
use crate::domain::{compile_mission_scope_graphs, preflight_mission_script};

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
        "context_adaptations":[],"context_finding_count":0,"context_findings":[],
        "statement_count":11,"unique_command_count":11,"load_p3d_reference_count":0,
        "mission_flow_command_count":10,"vehicle_physics_command_count":0,
        "semantic_family":"mission-script","command_counts":counts,
        "source_statements":[
            "SelectMission(\"m1\");",format!("AddStage({stage_raw});"),
            format!("SetStageMessageIndex({message_index});"),"SetStageTime(30);",
            "RESET_TO_HERE();",format!("AddStageVehicle(\"skinn_v\",\"m1_car\",\"chase\",\"{con_file}\",\"skinner\");"),
            "AddStageWaypoint(\"m1_path1\");","AddObjective(\"dummy\");",
            "CloseObjective();","CloseStage();","CloseMission();"
        ],
        "p3d_references":[],
        "command_invocations":[
            invocation(1,"selectmission","\"m1\"","mission-script", &["m1"]),
            invocation(2,"addstage",&stage_raw,"mission-stage",stage_arguments),
            invocation(3,"setstagemessageindex",message_index,"mission-stage", &[message_index]),
            invocation(4,"setstagetime","30","mission-stage", &["30"]),
            invocation(5,"reset_to_here","","mission-script", &[]),
            invocation(6,"addstagevehicle",&format!("\"skinn_v\",\"m1_car\",\"chase\",\"{con_file}\",\"skinner\""),"mission-stage", &["skinn_v","m1_car","chase",con_file,"skinner"]),
            invocation(7,"addstagewaypoint","\"m1_path1\"","mission-stage", &["m1_path1"]),
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
            | MissionStageDirective::MaxTrafficCars { .. } => {},
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

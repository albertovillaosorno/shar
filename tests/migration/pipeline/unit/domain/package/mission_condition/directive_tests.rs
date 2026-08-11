// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT

//! Typed condition directive semantic regressions.

use super::{
    MissionConditionDirective, MissionConditionScope,
    MissionConditionSemanticBinding, compile_directive,
};

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}


#[test]
fn semantic_binding_preserves_condition_schema_identity() {
    let binding = MissionConditionSemanticBinding {
        owner_stage_source_ordinal: 2,
        owner_stage_sequence_ordinal: 0,
        source_ordinal: 3,
        source_alias: "timeout".to_owned(),
        scope: MissionConditionScope::Objective,
        schema_id: "legacy-mission-condition.timeout.v1",
        directives: Vec::new(),
    };
    assert_eq!(binding.owner_stage_source_ordinal(), 2);
    assert_eq!(binding.owner_stage_sequence_ordinal(), 0);
    assert_eq!(binding.source_ordinal(), 3);
    assert_eq!(binding.source_alias(), "timeout");
    assert_eq!(binding.scope(), MissionConditionScope::Objective);
    assert_eq!(
        binding.schema_id(),
        "legacy-mission-condition.timeout.v1"
    );
}

#[test]
fn types_damage_targets_and_source_health() -> Result<(), String> {
    let health =
        compile_directive("damage", 4, "setcondminhealth", &strings(&["0.0"]))?;
    if health
        != (MissionConditionDirective::MinimumHealth {
            source_ordinal: 4,
            source_value: "0.0".to_owned(),
        })
    {
        return Err("minimum-health evidence changed".to_owned());
    }
    let vehicle = compile_directive(
        "damage",
        5,
        "setcondtargetvehicle",
        &strings(&["skinn_v"]),
    )?;
    if vehicle
        != (MissionConditionDirective::TargetVehicle {
            source_ordinal: 5,
            vehicle_id: "skinn_v".to_owned(),
        })
    {
        return Err("condition vehicle target changed".to_owned());
    }
    let boss = compile_directive(
        "damage",
        6,
        "setobjtargetboss",
        &strings(&["Planet Express Ship"]),
    )?;
    if boss
        != (MissionConditionDirective::TargetBoss {
            source_ordinal: 6,
            source_label: "Planet Express Ship".to_owned(),
        })
    {
        return Err("boss target label changed".to_owned());
    }
    Ok(())
}

#[test]
fn types_follow_time_position_and_dummy_marker() -> Result<(), String> {
    let follow = compile_directive(
        "followdistance",
        7,
        "setfollowdistances",
        &strings(&["0", "120"]),
    )?;
    if follow
        != (MissionConditionDirective::FollowDistances {
            source_ordinal: 7,
            minimum: 0,
            maximum: 120,
        })
    {
        return Err("follow-distance evidence changed".to_owned());
    }
    let time = compile_directive(
        "outofvehicle",
        8,
        "setcondtime",
        &strings(&["10000"]),
    )?;
    if time
        != (MissionConditionDirective::TimeValue {
            source_ordinal: 8,
            source_value: 10000,
        })
    {
        return Err("condition time evidence changed".to_owned());
    }
    let position = compile_directive(
        "position",
        9,
        "setconditionposition",
        &strings(&["2"]),
    )?;
    if position
        != (MissionConditionDirective::PositionIndex {
            source_ordinal: 9,
            source_index: 2,
        })
    {
        return Err("condition position evidence changed".to_owned());
    }
    let marker = compile_directive("timeout", 10, "sethitnrun", &[])?;
    if !matches!(
        marker,
        MissionConditionDirective::LegacyHitAndRunNoOp { .. }
    ) {
        return Err("legacy SetHitNRun marker changed".to_owned());
    }
    Ok(())
}

#[test]
fn rejects_invalid_or_cross_alias_condition_directives() -> Result<(), String> {
    for (alias, command, arguments) in [
        ("damage", "setcondminhealth", strings(&["NaN"])),
        ("damage", "setcondminhealth", strings(&["1e3"])),
        ("damage", "setcondminhealth", strings(&["+1"])),
        ("damage", "setcondminhealth", strings(&[".5"])),
        ("damage", "setcondminhealth", strings(&["1."])),
        ("damage", "setcondminhealth", strings(&["-0.1"])),
        ("damage", "setcondtargetvehicle", strings(&["../car"])),
        (
            "followdistance",
            "setfollowdistances",
            strings(&["200", "0"]),
        ),
        ("outofvehicle", "setcondtime", strings(&["0"])),
        ("outofvehicle", "setcondtime", strings(&["+1"])),
        ("position", "setconditionposition", strings(&["0"])),
        ("timeout", "sethitnrun", strings(&["1"])),
        ("race", "setcondtime", strings(&["10000"])),
    ] {
        if compile_directive(alias, 11, command, &arguments).is_ok() {
            return Err(format!(
                "invalid typed condition directive was accepted: {command}"
            ));
        }
    }
    Ok(())
}

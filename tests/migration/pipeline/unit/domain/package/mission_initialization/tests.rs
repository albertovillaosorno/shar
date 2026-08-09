// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT

//! Mission initialization helper regressions.

use super::{validate_identity, validate_p3d_path};

#[test]
fn accepts_reviewed_locator_and_p3d_shapes() -> Result<(), String> {
    validate_identity("m4_carstart", "fixture locator")?;
    validate_p3d_path("l7i02.p3d")?;
    Ok(())
}

#[test]
fn rejects_escaping_or_non_p3d_load_references() {
    assert!(validate_p3d_path("../l1z1.p3d").is_err());
    assert!(validate_p3d_path("/l1z1.p3d").is_err());
    assert!(validate_p3d_path("l1z1.txt").is_err());
}

#[test]
fn rejects_path_shaped_locator_identity() {
    assert!(validate_identity("missions/m1", "fixture locator").is_err());
}

#[test]
fn preserves_street_race_load_and_unload_terminators() -> Result<(), String> {
    let load = super::compile_street_race_props(
        &["l1_sr1p.p3d;".to_owned()],
        ';',
        "load",
    )?;
    if load != ("l1_sr1p.p3d;".to_owned(), vec!["l1_sr1p.p3d".to_owned()]) {
        return Err("street-race load evidence changed".to_owned());
    }
    let unload = super::compile_street_race_props(
        &["l1_sr1p.p3d:".to_owned()],
        ':',
        "unload",
    )?;
    if unload != ("l1_sr1p.p3d:".to_owned(), vec!["l1_sr1p.p3d".to_owned()]) {
        return Err("street-race unload evidence changed".to_owned());
    }
    assert!(
        super::compile_street_race_props(
            &["l1_sr1p.p3d:".to_owned()],
            ';',
            "load",
        )
        .is_err()
    );
    assert!(
        super::compile_street_race_props(
            &["../escape.p3d;".to_owned()],
            ';',
            "load",
        )
        .is_err()
    );
    Ok(())
}

#[test]
fn types_remaining_mission_scope_sources() -> Result<(), String> {
    let camera = super::compile_remaining_directive(
        20,
        "setmissionstartcameraname",
        &["mission2camShape".to_owned()],
    )?;
    if camera
        != Some(super::MissionInitializationDirective::MissionStartCamera {
            source_ordinal: 20,
            camera_id: "mission2camShape".to_owned(),
        })
    {
        return Err("mission-start camera mapping changed".to_owned());
    }
    let hints =
        super::compile_remaining_directive(21, "setnumvalidfailurehints", &[
            "5".to_owned(),
        ])?;
    if hints
        != Some(super::MissionInitializationDirective::ValidFailureHints {
            source_ordinal: 21,
            count: 5,
        })
    {
        return Err("mission failure-hint mapping changed".to_owned());
    }
    let group = super::compile_remaining_directive(22, "usepedgroup", &[
        "7".to_owned()
    ])?;
    if group
        != Some(super::MissionInitializationDirective::PedGroup {
            source_ordinal: 22,
            group_index: 7,
        })
    {
        return Err("mission pedestrian-group mapping changed".to_owned());
    }
    let bitmap =
        super::compile_remaining_directive(23, "setpresentationbitmap", &[
            "art/frontend/dynaload/images/mis01_00.p3d".to_owned(),
        ])?;
    if bitmap
        != Some(super::MissionInitializationDirective::PresentationBitmap {
            source_ordinal: 23,
            p3d_path: "art/frontend/dynaload/images/mis01_00.p3d".to_owned(),
        })
    {
        return Err("mission presentation bitmap mapping changed".to_owned());
    }
    Ok(())
}

#[test]
fn rejects_unreviewed_mission_scope_values() -> Result<(), String> {
    for (name, arguments) in [
        ("setnumvalidfailurehints", vec!["4"]),
        ("usepedgroup", vec!["8"]),
        ("showhud", vec!["true"]),
        ("placeplayercar", vec!["other", "sr1_carstart"]),
        ("addcollectiblestateprop", vec!["bombbarrel", "barrel", "3"]),
        ("setpresentationbitmap", vec!["../escape.p3d"]),
    ] {
        let arguments =
            arguments.into_iter().map(str::to_owned).collect::<Vec<_>>();
        if super::compile_remaining_directive(24, name, &arguments).is_ok() {
            return Err(format!(
                "unreviewed mission-scope value accepted: {name}"
            ));
        }
    }
    Ok(())
}


#[test]
fn preserves_legacy_set_dyna_without_final_region_postfix() -> Result<(), String> {
    let p3d_files = super::reviewed_dynamic_p3d_files(
        "l7z6.p3d;l7r6.p3d;l7r4.p3d",
    )?;
    if p3d_files != ["l7z6.p3d", "l7r6.p3d", "l7r4.p3d"] {
        return Err("legacy terminal-less Dyna evidence drifted".to_owned());
    }
    Ok(())
}

#[test]
fn set_dyna_rejects_unload_and_world_sphere_operations() {
    for source in ["l1z1.p3d:", "l1i00.p3d$", "visibility_a*"] {
        assert!(
            super::reviewed_dynamic_p3d_files(source).is_err(),
            "unreviewed mission-start Dyna operation was accepted: {source}"
        );
    }
}

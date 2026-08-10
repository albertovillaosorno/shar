// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT

use super::*;

fn catalog() -> MissionReferenceCatalog {
    MissionReferenceCatalog::from_character_entries_for_tests(&[
        ("ralph", "ralph", "character-ralph", "characters/ralph/base-model"),
        ("cletus", "cletus", "character-cletus", "characters/cletus/base-model"),
    ])
}

#[test]
fn types_ambient_declaration_and_waypoint() -> Result<(), String> {
    let mut declarations = Vec::new();
    push_ambient(
        &mut declarations,
        &catalog(),
        17,
        &["ralph".to_owned(), "ambient_ralph".to_owned(), "1.3".to_owned()],
    )?;
    let [declaration] = declarations.as_slice() else {
        return Err("ambient declaration count changed".to_owned());
    };
    assert_eq!(declaration.kind(), MissionLevelNpcKind::Ambient);
    assert_eq!(declaration.runtime_character_id(), "ralph");
    assert_eq!(declaration.character().package_id(), "character-ralph");
    assert_eq!(declaration.choreo_id(), "npd");
    assert_eq!(declaration.locator_id(), "ambient_ralph");
    assert_eq!(declaration.ambient_radius_source(), Some("1.3"));

    let mut waypoints = Vec::new();
    push_waypoint(
        &mut waypoints,
        &declarations,
        MissionLevelNpcKind::Ambient,
        18,
        &["ralph".to_owned(), "ralph_walk1".to_owned()],
    )?;
    assert_eq!(waypoints[0].runtime_character_id(), "ralph");
    assert_eq!(waypoints[0].locator_id(), "ralph_walk1");
    Ok(())
}

#[test]
fn accepts_source_default_and_zero_ambient_radius() -> Result<(), String> {
    let mut declarations = Vec::new();
    push_ambient(
        &mut declarations,
        &catalog(),
        1,
        &["ralph".to_owned(), "ambient_ralph".to_owned()],
    )?;
    push_ambient(
        &mut declarations,
        &catalog(),
        2,
        &["ralph".to_owned(), "ambient_ralph_2".to_owned(), "0".to_owned()],
    )?;
    assert_eq!(declarations[0].ambient_radius_source(), None);
    assert_eq!(declarations[1].ambient_radius_source(), Some("0"));
    Ok(())
}

#[test]
fn types_bonus_runtime_name_and_metadata() -> Result<(), String> {
    let mut declarations = Vec::new();
    push_bonus(
        &mut declarations,
        &catalog(),
        12,
        &[
            "cletus".to_owned(),
            "npd".to_owned(),
            "bm1_cletus_sd".to_owned(),
            "bm1".to_owned(),
            "exclamation".to_owned(),
            "jug".to_owned(),
            "1".to_owned(),
            "exclamation_shadow".to_owned(),
        ],
    )?;
    let [binding] = declarations.as_slice() else {
        return Err("bonus declaration count changed".to_owned());
    };
    assert_eq!(binding.runtime_character_id(), "b_cletus");
    assert_eq!(binding.bonus_mission_id(), Some("bm1"));
    assert_eq!(binding.bonus_icon_id(), Some("exclamation"));
    assert_eq!(binding.bonus_dialogue_id(), Some("jug"));
    assert_eq!(binding.bonus_is_race(), Some(true));
    assert_eq!(binding.bonus_alternate_icon_id(), Some("exclamation_shadow"));
    Ok(())
}

#[test]
fn rejects_waypoint_without_unique_prior_matching_declaration() -> Result<(), String> {
    let mut declarations = Vec::new();
    push_ambient(
        &mut declarations,
        &catalog(),
        10,
        &["ralph".to_owned(), "ambient_ralph".to_owned()],
    )?;
    let mut waypoints = Vec::new();
    assert!(
        push_waypoint(
            &mut waypoints,
            &declarations,
            MissionLevelNpcKind::Ambient,
            9,
            &["ralph".to_owned(), "ralph_walk1".to_owned()],
        )
        .is_err()
    );
    assert!(
        push_waypoint(
            &mut waypoints,
            &declarations,
            MissionLevelNpcKind::BonusMission,
            11,
            &["ralph".to_owned(), "ralph_walk1".to_owned()],
        )
        .is_err()
    );
    Ok(())
}

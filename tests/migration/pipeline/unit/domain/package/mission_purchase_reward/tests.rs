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
//   - Purchase-car reward storefront unit regressions.
// - Must-Not:
//   - Assign price, ownership, unlock, or save-game behavior.
// - Allows:
//   - Synthetic source-backed storefront setup and character package binding.
// - Summary:
//   - AddPurchaseCarReward domain tests.
//

use super::*;

fn catalog() -> MissionReferenceCatalog {
    MissionReferenceCatalog::from_character_entries_for_tests(&[
        (
            "gil",
            "gil",
            "character-gil",
            "characters/gil/base-model",
        ),
        (
            "barney",
            "barney",
            "character-barney",
            "characters/barney/base-model",
        ),
    ])
}

#[test]
fn binds_gil_storefront_setup_without_purchase_policy() -> Result<(), String> {
    let mut bindings = Vec::new();
    push_binding(
        &mut bindings,
        &catalog(),
        82,
        &[
            "gil".to_owned(),
            "gil".to_owned(),
            "npd".to_owned(),
            "gil_loc".to_owned(),
            "1.3".to_owned(),
            "gil_car".to_owned(),
        ],
    )?;
    let [binding] = bindings.as_slice() else {
        return Err("purchase reward binding count changed".to_owned());
    };
    assert_eq!(binding.source_ordinal(), 82);
    assert_eq!(binding.action_locator_id(), "gil");
    assert_eq!(binding.seller(), MissionPurchaseRewardSeller::GilVendor);
    assert_eq!(binding.reward_character_id(), "reward_gil");
    assert_eq!(binding.character().participant_id(), "gil");
    assert_eq!(binding.character().package_id(), "character-gil");
    assert_eq!(binding.choreo_id(), "npd");
    assert_eq!(binding.position_locator_id(), "gil_loc");
    assert_eq!(binding.trigger_radius_source(), "1.3");
    assert_eq!(binding.car_start_locator_id(), "gil_car");
    Ok(())
}

#[test]
fn binds_playable_character_seller_from_simpson_action() -> Result<(), String> {
    let mut bindings = Vec::new();
    push_binding(
        &mut bindings,
        &catalog(),
        84,
        &[
            "simpson".to_owned(),
            "barney".to_owned(),
            "npd".to_owned(),
            "barney_loc".to_owned(),
            "1.3".to_owned(),
            "barney_car".to_owned(),
        ],
    )?;
    let [binding] = bindings.as_slice() else {
        return Err("purchase reward binding count changed".to_owned());
    };
    assert_eq!(
        binding.seller(),
        MissionPurchaseRewardSeller::LevelPlayableCharacter
    );
    assert_eq!(binding.reward_character_id(), "reward_barney");
    assert_eq!(binding.character().participant_id(), "barney");
    Ok(())
}

#[test]
fn rejects_unreviewed_action_and_bad_radius() {
    let mut bindings = Vec::new();
    let unreviewed = [
        "other".to_owned(),
        "gil".to_owned(),
        "npd".to_owned(),
        "gil_loc".to_owned(),
        "1.3".to_owned(),
        "gil_car".to_owned(),
    ];
    assert!(push_binding(&mut bindings, &catalog(), 1, &unreviewed).is_err());

    let bad_radius = [
        "gil".to_owned(),
        "gil".to_owned(),
        "npd".to_owned(),
        "gil_loc".to_owned(),
        "0".to_owned(),
        "gil_car".to_owned(),
    ];
    assert!(push_binding(&mut bindings, &catalog(), 2, &bad_radius).is_err());
}

#[test]
fn rejects_missing_character_package() {
    let mut bindings = Vec::new();
    let arguments = [
        "simpson".to_owned(),
        "homer".to_owned(),
        "npd".to_owned(),
        "homer_loc".to_owned(),
        "1.3".to_owned(),
        "homer_car".to_owned(),
    ];
    assert!(push_binding(&mut bindings, &catalog(), 3, &arguments).is_err());
}

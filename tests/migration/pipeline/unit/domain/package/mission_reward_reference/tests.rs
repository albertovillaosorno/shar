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
//   - BindReward package-reference unit regressions.
// - Must-Not:
//   - Assign unlock, progression, seller, or purchase runtime behavior.
// - Allows:
//   - Synthetic reward source tokens and canonical package bindings.
// - Split-When:
//   - Reward source-shape and package-binding tests diverge independently.
// - Merge-When:
//   - BindReward package-reference preflight loses independent policy.
// - Summary:
//   - BindReward package-reference tests.
// - Description:
//   - Proves observed five/seven argument shapes and fail-closed P3D binding.
// - Usage:
//   - Included only by the mission reward-reference domain module.
// - Defaults:
//   - Source reward policy tokens remain opaque.
//

use super::*;

fn catalog() -> MissionP3dReferenceCatalog {
    MissionP3dReferenceCatalog::from_entries_for_tests(&[(
        "extracted/art/cars/famil_v",
        "reward-package",
        "extracted/art/cars/famil_v",
    )])
}

#[test]
fn binds_observed_five_argument_shape() -> Result<(), String> {
    let mut bindings = Vec::new();
    push_binding(
        &mut bindings,
        &catalog(),
        7,
        &[
            "famil_v".to_owned(),
            r"art\cars\famil_v.p3d".to_owned(),
            "car".to_owned(),
            "defaultcar".to_owned(),
            "1".to_owned(),
        ],
    )?;
    let [binding] = bindings.as_slice() else {
        return Err("BindReward binding count changed".to_owned());
    };
    assert_eq!(binding.source_ordinal(), 7);
    assert_eq!(binding.reward_id(), "famil_v");
    assert_eq!(binding.source_reference(), r"art\cars\famil_v.p3d");
    assert_eq!(binding.reward_type_token(), "car");
    assert_eq!(binding.source_mode_token(), "defaultcar");
    assert_eq!(binding.source_level(), "1");
    assert_eq!(binding.source_cost(), None);
    assert_eq!(binding.source_vendor(), None);
    assert_eq!(binding.package_id(), "reward-package");
    assert_eq!(binding.package_root(), "extracted/art/cars/famil_v");
    Ok(())
}

#[test]
fn preserves_observed_seven_argument_shape_without_policy() -> Result<(), String> {
    let mut bindings = Vec::new();
    push_binding(
        &mut bindings,
        &catalog(),
        8,
        &[
            "famil_v".to_owned(),
            r"art\cars\famil_v.p3d".to_owned(),
            "car".to_owned(),
            "forsale".to_owned(),
            "1".to_owned(),
            "150".to_owned(),
            "simpson".to_owned(),
        ],
    )?;
    let [binding] = bindings.as_slice() else {
        return Err("BindReward binding count changed".to_owned());
    };
    assert_eq!(binding.source_cost(), Some("150"));
    assert_eq!(binding.source_vendor(), Some("simpson"));
    Ok(())
}

#[test]
fn rejects_unobserved_arity_and_malformed_unsigned_scalars() {
    let catalog = catalog();
    let mut bindings = Vec::new();
    assert!(
        push_binding(
            &mut bindings,
            &catalog,
            9,
            &["famil_v".to_owned()],
        )
        .is_err()
    );
    assert!(
        push_binding(
            &mut bindings,
            &catalog,
            10,
            &[
                "famil_v".to_owned(),
                r"art\cars\famil_v.p3d".to_owned(),
                "car".to_owned(),
                "defaultcar".to_owned(),
                "level1".to_owned(),
            ],
        )
        .is_err()
    );
    assert!(bindings.is_empty());
}

#[test]
fn rejects_source_level_outside_observed_base_levels() {
    for level in ["0", "8", "4294967295"] {
        let mut bindings = Vec::new();
        assert!(
            push_binding(
                &mut bindings,
                &catalog(),
                11,
                &[
                    "famil_v".to_owned(),
                    r"art\carsamil_v.p3d".to_owned(),
                    "car".to_owned(),
                    "defaultcar".to_owned(),
                    level.to_owned(),
                ],
            )
            .is_err()
        );
        assert!(bindings.is_empty());
    }
}

#[test]
fn missing_reward_package_fails_closed() {
    let mut bindings = Vec::new();
    assert!(
        push_binding(
            &mut bindings,
            &MissionP3dReferenceCatalog::empty_for_tests(),
            11,
            &[
                "famil_v".to_owned(),
                r"art\cars\famil_v.p3d".to_owned(),
                "car".to_owned(),
                "defaultcar".to_owned(),
                "1".to_owned(),
            ],
        )
        .is_err()
    );
    assert!(bindings.is_empty());
}

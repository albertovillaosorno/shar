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
//   - Asset loose unit tests.
// - Must-Not:
//   - Own production behavior or broaden the tested API surface.
// - Allows:
//   - Private assertions for the owning source module.
// - Split-When:
//   - Split when an independent fixture family gains separate ownership.
// - Merge-When:
//   - Merge when another test module owns the identical evidence.
// - Summary:
//   - Asset loose unit tests.
// - Description:
//   - Preserves unit-test access through a test-only path module.
// - Usage:
//   - Included only by the owning source module under cfg(test).
// - Defaults:
//   - Assertions fail explicitly.
//

//! Asset loose unit tests.

use super::*;

#[test]
fn rejects_control_characters_in_character_identities() {
    assert_eq!(
        CharacterAsset::new("character\nalias", Vec::new(), Vec::new(),),
        Err(CharacterError::NonCanonicalCharacterName)
    );
    assert_eq!(
        CharacterAsset::new(
            "character",
            vec![Bone {
                id: "root\nalias".to_owned(),
                parent_id: None,
                rest_matrix: [0f32; 16],
                source_identity: None,
                source_rig: None,
            },],
            Vec::new(),
        ),
        Err(CharacterError::NonCanonicalBoneId {
            bone: "root\nalias".to_owned(),
        })
    );
}

#[test]
fn source_provenance_preserves_composite_order_and_duplicates() {
    let result = CharacterSourceProvenance::new("skeleton", vec![
        "z-composite".to_owned(),
        "a-composite".to_owned(),
        "z-composite".to_owned(),
    ])
    .and_then(|provenance| {
        let skin = CompositeSkinSourceBinding::new(
            1,
            4,
            "skin-shape",
            false,
            Some(0.1),
        )?;
        provenance.with_composite_skin_bindings(vec![skin])
    })
    .and_then(|provenance| {
        let effect = CompositeEffectSourceBinding::new(
            0,
            2,
            "effect-shape",
            5,
            true,
            Some(0.3),
        )?;
        provenance.with_composite_effect_bindings(vec![effect])
    })
    .and_then(|provenance| {
        let binding = CompositePropSourceBinding::new(
            2,
            7,
            "prop-shape",
            3,
            true,
            Some(0.49),
        )?;
        provenance.with_composite_prop_bindings(vec![binding])
    });
    assert_eq!(
        result,
        Ok(CharacterSourceProvenance {
            skeleton_identity: "skeleton".to_owned(),
            composite_identities: vec![
                "z-composite".to_owned(),
                "a-composite".to_owned(),
                "z-composite".to_owned(),
            ],
            composite_skin_bindings: vec![CompositeSkinSourceBinding {
                composite_ordinal: 1,
                skin_index: 4,
                skin_identity: "skin-shape".to_owned(),
                translucent: false,
                sort_order_bits: Some(0.1_f32.to_bits()),
            }],
            composite_effect_bindings: vec![CompositeEffectSourceBinding {
                composite_ordinal: 0,
                effect_index: 2,
                effect_identity: "effect-shape".to_owned(),
                skeleton_joint_id: 5,
                translucent: true,
                sort_order_bits: Some(0.3_f32.to_bits()),
            }],
            composite_prop_bindings: vec![CompositePropSourceBinding {
                composite_ordinal: 2,
                prop_index: 7,
                prop_identity: "prop-shape".to_owned(),
                skeleton_joint_id: 3,
                translucent: true,
                sort_order_bits: Some(0.49_f32.to_bits()),
            }],
        })
    );
}

#[test]
fn rejects_invalid_composite_prop_source_provenance() {
    assert_eq!(
        CompositeSkinSourceBinding::new(
            0,
            0,
            "skin",
            false,
            Some(f32::INFINITY),
        ),
        Err(CharacterError::NonFiniteSourceSkinSortOrder {
            composite_ordinal: 0,
            skin_index: 0,
        })
    );

    assert_eq!(
        CompositeEffectSourceBinding::new(
            0,
            0,
            "effect",
            1,
            false,
            Some(f32::INFINITY),
        ),
        Err(CharacterError::NonFiniteSourceEffectSortOrder {
            composite_ordinal: 0,
            effect_index: 0,
        })
    );

    assert_eq!(
        CompositePropSourceBinding::new(
            0,
            0,
            "prop",
            1,
            false,
            Some(f32::INFINITY),
        ),
        Err(CharacterError::NonFiniteSourcePropSortOrder {
            composite_ordinal: 0,
            prop_index: 0,
        })
    );

    let out_of_bounds = CharacterSourceProvenance::new("skeleton", vec![
        "composite".to_owned(),
    ])
    .and_then(|provenance| {
        let binding =
            CompositePropSourceBinding::new(1, 0, "prop", 1, false, None)?;
        provenance.with_composite_prop_bindings(vec![binding])
    });
    assert_eq!(
        out_of_bounds,
        Err(CharacterError::SourcePropCompositeOutOfBounds {
            composite_ordinal: 1,
            composites: 1,
        })
    );

    let duplicate = CharacterSourceProvenance::new("skeleton", vec![
        "composite".to_owned(),
    ])
    .and_then(|provenance| {
        let first =
            CompositePropSourceBinding::new(0, 3, "left", 1, false, None)?;
        let second =
            CompositePropSourceBinding::new(0, 3, "right", 2, true, Some(0.5))?;
        provenance.with_composite_prop_bindings(vec![first, second])
    });
    assert_eq!(
        duplicate,
        Err(CharacterError::DuplicateSourceCompositePropIndex {
            composite_ordinal: 0,
            prop_index: 3,
        })
    );
}

#[test]
fn rejects_noncanonical_character_source_provenance() {
    assert_eq!(
        CharacterSourceProvenance::new(" skeleton", Vec::new()),
        Err(CharacterError::NonCanonicalSourceSkeletonIdentity {
            identity: " skeleton".to_owned(),
        })
    );
    assert_eq!(
        CharacterSourceProvenance::new("skeleton", vec![
            "composite\nalias".to_owned()
        ],),
        Err(CharacterError::NonCanonicalSourceCompositeIdentity {
            identity: "composite\nalias".to_owned(),
        })
    );
}

#[test]
fn rejects_noncanonical_source_bone_identity() {
    let result = CharacterAsset::new(
        "character",
        vec![Bone {
            id: "root".to_owned(),
            source_identity: Some(" root".to_owned()),
            parent_id: None,
            rest_matrix: [0f32; 16],
            source_rig: None,
        }],
        Vec::new(),
    );

    assert_eq!(
        result,
        Err(CharacterError::NonCanonicalBoneSourceIdentity {
            bone: "root".to_owned(),
            source_identity: " root".to_owned(),
        })
    );
}

#[test]
fn rejects_noncanonical_parent_identities() {
    let bones = vec![
        Bone {
            id: "root".to_owned(),
            parent_id: None,
            rest_matrix: [0f32; 16],
            source_identity: None,
            source_rig: None,
        },
        Bone {
            id: "child".to_owned(),
            parent_id: Some("root\nalias".to_owned()),
            rest_matrix: [0f32; 16],
            source_identity: None,
            source_rig: None,
        },
    ];

    assert_eq!(
        CharacterAsset::new("character", bones, Vec::new(),),
        Err(CharacterError::NonCanonicalParentId {
            bone: "child".to_owned(),
            parent: "root\nalias".to_owned(),
        })
    );
}

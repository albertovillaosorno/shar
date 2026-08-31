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
    ]);
    assert_eq!(
        result,
        Ok(CharacterSourceProvenance {
            skeleton_identity: "skeleton".to_owned(),
            composite_identities: vec![
                "z-composite".to_owned(),
                "a-composite".to_owned(),
                "z-composite".to_owned(),
            ],
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

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
            },],
            Vec::new(),
        ),
        Err(CharacterError::NonCanonicalBoneId {
            bone: "root\nalias".to_owned(),
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
        },
        Bone {
            id: "child".to_owned(),
            parent_id: Some("root\nalias".to_owned()),
            rest_matrix: [0f32; 16],
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

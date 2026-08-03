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
//   - Tests unit tests.
// - Must-Not:
//   - Own production behavior or broaden the tested API surface.
// - Allows:
//   - Private test fixtures and assertions for the owning source module.
// - Split-When:
//   - Split when an independent fixture family gains separate ownership.
// - Merge-When:
//   - Merge when another test module owns the identical evidence.
// - Summary:
//   - Tests unit tests.
// - Description:
//   - Preserves unit-test access through a test-only path module.
// - Usage:
//   - Included only by the owning source module under cfg(test).
// - Defaults:
//   - Test setup and assertions fail explicitly.
//

//! Tests unit tests.

use std::collections::BTreeMap;

use super::super::transform::identity;
use super::{
    LevelMeshSource, WorldObjectRole, explicit_placements,
    is_direct_world_mesh, object_role,
};

fn source(kind: &str) -> LevelMeshSource {
    LevelMeshSource {
        ordinal: 1,
        member_id: "house".to_owned(),
        mesh_name: "house".to_owned(),
        owner_name: "house-owner".to_owned(),
        owner_kind: kind.to_owned(),
    }
}

#[test]
fn explicit_placement_prefers_mesh_identity() {
    let mut placements = BTreeMap::new();
    let _previous = placements.insert("house".to_owned(), vec![identity()]);
    assert_eq!(
        explicit_placements(&source("srr_insta_entity_dsg"), &placements,)
            .len(),
        1
    );
}

#[test]
fn direct_entities_are_classified_without_invented_matrix() {
    assert!(is_direct_world_mesh(&source("srr_entity_dsg")));
    assert!(is_direct_world_mesh(&source("srr_static_phys_dsg")));
    assert!(
        explicit_placements(&source("srr_static_phys_dsg"), &BTreeMap::new(),)
            .is_empty()
    );
}

#[test]
fn definition_only_meshes_are_not_direct_world_geometry() {
    assert!(!is_direct_world_mesh(&source("srr_breakable_object")));
}

#[test]
fn source_owner_kinds_preserve_world_interaction_roles() {
    let mut tree = source("srr_dyna_phys_dsg");
    tree.mesh_name = "l1_treesm_shape".to_owned();
    assert_eq!(object_role(&tree), WorldObjectRole::Breakable);
    assert_eq!(
        object_role(&source("srr_static_phys_dsg")),
        WorldObjectRole::Interactable
    );
    assert_eq!(
        object_role(&source("srr_insta_anim_dyna_phys_dsg")),
        WorldObjectRole::Interactable
    );
    assert_eq!(
        object_role(&source("srr_entity_dsg")),
        WorldObjectRole::Static
    );
}

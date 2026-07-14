// File:
//   - deterministic_constructor_validation.rs
// Path:
//   - src/fbx/tests/deterministic_constructor_validation.rs
//
// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE
// Path-Rule:
//   - All paths in this header are repository-root relative.
//
// Boundary-Contract:
// - Owns:
//   - Regression coverage for deterministic FBX constructor ordering.
// - Must-Not:
//   - Read private assets, discover packages, or use filesystem fixtures.
// - Allows:
//   - Synthetic values and public constructor equality assertions.
// - Split-When:
//   - One aggregate requires an independent integration boundary.
// - Merge-When:
//   - Deterministic ordering moves behind one shared domain collection.
// - Summary:
//   - Protects equivalent constructor input from order-dependent results.
// - Description:
//   - Exercises semantically equal values supplied in different input orders.
// - Usage:
//   - Run through the fbx crate test target.
// - Defaults:
//   - No local files or external processes are required.
//
// ADRs:
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
//
// Large file:
//   - false
//

//! Regression coverage for deterministic FBX constructor ordering.
//!
//! Equivalent synthetic input must produce equal domain aggregates.

use fbx::domain::animation::{AnimationCapability, AnimationRequirement};
use fbx::domain::mesh::{MeshAsset, PrimitiveGroup};
use schoenwald_filesystem as _;
use serde as _;
use serde_json as _;

fn group(
    index: usize,
    shader: &str,
) -> Result<PrimitiveGroup, String> {
    PrimitiveGroup::new(
        index,
        shader,
        vec![
            [
                0.0, 0.0, 0.0,
            ],
            [
                1.0, 0.0, 0.0,
            ],
            [
                0.0, 1.0, 0.0,
            ],
        ],
        Vec::new(),
        &[
            0, 1, 2,
        ],
    )
    .map_err(|error| format!("valid group failed: {error:?}"))
}

#[test]
fn canonicalizes_mesh_asset_group_order() -> Result<(), String> {
    let group_zero = group(
        0, "zero",
    )?;
    let group_one = group(
        1, "one",
    )?;
    let first = MeshAsset::new(
        "mesh",
        vec![
            group_one.clone(),
            group_zero.clone(),
        ],
    );
    let second = MeshAsset::new(
        "mesh",
        vec![
            group_zero, group_one,
        ],
    );

    if first == second {
        Ok(())
    } else {
        Err("equivalent mesh groups retained caller order".to_owned())
    }
}

#[test]
fn canonicalizes_animation_requirement_member_order() {
    let first = AnimationRequirement::new(
        vec![
            "walk".to_owned(),
            "idle".to_owned(),
        ],
        AnimationCapability::PreservedOnly,
    );
    let second = AnimationRequirement::new(
        vec![
            "idle".to_owned(),
            "walk".to_owned(),
        ],
        AnimationCapability::PreservedOnly,
    );

    assert_eq!(
        first,
        second
    );
}

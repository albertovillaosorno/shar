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
//   - Deterministic constructor validation test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Deterministic constructor validation test module.
// - Description:
//   - Implements the declared test module responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Deterministic constructor validation test module.

use fbx::domain::animation::{AnimationCapability, AnimationRequirement};
use fbx::domain::mesh::{MeshAsset, PrimitiveGroup};
use png as _;
use schoenwald_filesystem as _;
use serde as _;
use serde_json as _;
use shar_sha256 as _;

fn group(index: usize, shader: &str) -> Result<PrimitiveGroup, String> {
    PrimitiveGroup::new(
        index,
        shader,
        vec![[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]],
        Vec::new(),
        &[0, 1, 2],
    )
    .map_err(|error| format!("valid group failed: {error:?}"))
}

#[test]
fn canonicalizes_mesh_asset_group_order() -> Result<(), String> {
    let group_zero = group(0, "zero")?;
    let group_one = group(1, "one")?;
    let first =
        MeshAsset::new("mesh", vec![group_one.clone(), group_zero.clone()]);
    let second = MeshAsset::new("mesh", vec![group_zero, group_one]);

    if first == second {
        Ok(())
    } else {
        Err("equivalent mesh groups retained caller order".to_owned())
    }
}

#[test]
fn canonicalizes_animation_requirement_member_order() {
    let first = AnimationRequirement::new(
        vec!["walk".to_owned(), "idle".to_owned()],
        AnimationCapability::PreservedOnly,
    );
    let second = AnimationRequirement::new(
        vec!["idle".to_owned(), "walk".to_owned()],
        AnimationCapability::PreservedOnly,
    );

    assert_eq!(first, second);
}

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

use super::portable_name;

#[test]
fn portable_world_name_has_no_hash_suffix() {
    assert_eq!(
        portable_name("Cypress Tree!"),
        Ok("cypress-tree".to_owned())
    );
}

#[test]
fn phone_stops_publish_as_readable_phone_booths() {
    assert_eq!(
        portable_name("l1_phonestop"),
        Ok("l1-phone-booth".to_owned())
    );
}

use fbx::domain::animation::{
    AnimationClip, BoneAnimationTrack, LocalTransformSample,
};
use fbx::domain::character::{CharacterAsset, SkinnedPart};
use fbx::domain::mesh::{MeshAsset, PrimitiveGroup};
use fbx::domain::skeleton::Bone;
use fbx::domain::skin::SkinInfluence;

use super::{WorldVariant, animation_key, merge_compatible};
use crate::adapters::driven::local::prop_catalog::model::PropRoute;
use crate::adapters::driven::local::prop_catalog::prepared::{
    PreparedGeometry, PreparedProp,
};

fn rigid_asset() -> Result<CharacterAsset, String> {
    let group = PrimitiveGroup::new(
        0,
        "material",
        vec![[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]],
        Vec::new(),
        &[0, 1, 2],
    )
    .map_err(|error| format!("world variant group failed: {error:?}"))?;
    let mesh = MeshAsset::new("part", vec![group])
        .map_err(|error| format!("world variant mesh failed: {error:?}"))?;
    let influences = (0_u32..3)
        .map(|vertex_index| SkinInfluence {
            vertex_index,
            bone_id: "root".to_owned(),
            weight: 1.,
        })
        .collect();
    CharacterAsset::new(
        "prop",
        vec![Bone {
            id: "root".to_owned(),
            parent_id: None,
            rest_matrix: [
                1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
            ],
            source_rig: None,
        }],
        vec![SkinnedPart {
            mesh,
            group_influences: vec![influences],
        }],
    )
    .map_err(|error| format!("world variant asset failed: {error:?}"))
}

fn clip(name: &str, x: f64) -> Result<AnimationClip, String> {
    AnimationClip::new(
        name,
        30.,
        false,
        1,
        vec![BoneAnimationTrack {
            bone_id: "root".to_owned(),
            samples: vec![LocalTransformSample {
                translation: [x, 0., 0.],
                rotation_wxyz: [1., 0., 0., 0.],
            }],
        }],
        Vec::new(),
    )
    .map_err(|error| format!("world variant clip failed: {error:?}"))
}


fn clip_x(clip: &AnimationClip) -> Result<f64, String> {
    clip.tracks
        .first()
        .and_then(|track| track.samples.first())
        .map(|sample| sample.translation[0])
        .ok_or_else(|| "world variant clip lost its first sample".to_owned())
}

#[test]
fn empty_variant_merge_preserves_canonical_clip_order(
) -> Result<(), String> {
    let left = clip("left", 1.)?;
    let right = clip("right", 2.)?;
    let (first, second) = if animation_key(&left) > animation_key(&right) {
        (left, right)
    } else {
        (right, left)
    };
    let first_x = clip_x(&first)?;
    let second_x = clip_x(&second)?;
    let mut canonical = WorldVariant {
        prepared: PreparedProp {
            route: PropRoute::RigidAnimated,
            signature: "before".to_owned(),
            geometry: PreparedGeometry::RigidAnimated {
                asset: rigid_asset()?,
                animations: vec![first, second],
            },
            materials: Vec::new(),
            textures: Vec::new(),
        },
        aliases: Vec::new(),
        visual_sha256: "visual".to_owned(),
        structural_sha256: "structural".to_owned(),
        rig_sha256: Some("rig".to_owned()),
    };
    merge_compatible(&mut canonical, Vec::new());
    let PreparedGeometry::RigidAnimated { animations, .. } =
        &canonical.prepared.geometry
    else {
        return Err("world variant unexpectedly became static".to_owned());
    };
    let actual = animations
        .iter()
        .map(clip_x)
        .collect::<Result<Vec<_>, _>>()?;
    if actual != [first_x, second_x] {
        return Err(format!(
            "empty merge changed canonical clip order: {actual:?}"
        ));
    }
    Ok(())
}

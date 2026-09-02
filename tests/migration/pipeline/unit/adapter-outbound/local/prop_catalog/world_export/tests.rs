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
use fbx::domain::character::{
    CharacterAsset, CharacterSourceProvenance, SkinnedPart,
};
use fbx::domain::mesh::{MeshAsset, PrimitiveGroup};
use fbx::domain::skeleton::Bone;
use fbx::domain::skin::SkinInfluence;

use super::{WorldVariant, animation_key, merge_compatible};
use crate::adapters::driven::local::prop_catalog::model::PropRoute;
use crate::adapters::driven::local::prop_catalog::prepare::{
    prepared_signature, rig_signature, visual_signature,
};
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
            source_identity: None,
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
fn prop_hashes_ignore_source_occurrence_provenance() -> Result<(), String> {
    let mut left_asset = rigid_asset()?;
    let mut right_asset = left_asset.clone();
    left_asset
        .parts
        .first_mut()
        .and_then(|part| part.mesh.groups.first_mut())
        .ok_or_else(|| "left hash fixture lost its primitive group".to_owned())?
        .source_ordinal = Some(41);
    right_asset
        .parts
        .first_mut()
        .and_then(|part| part.mesh.groups.first_mut())
        .ok_or_else(|| {
            "right hash fixture lost its primitive group".to_owned()
        })?
        .source_ordinal = Some(97);
    left_asset
        .bones
        .first_mut()
        .ok_or_else(|| "left hash fixture lost its root bone".to_owned())?
        .source_identity = Some("source-root-a".to_owned());
    right_asset
        .bones
        .first_mut()
        .ok_or_else(|| "right hash fixture lost its root bone".to_owned())?
        .source_identity = Some("source-root-b".to_owned());
    left_asset.source_provenance = Some(
        CharacterSourceProvenance::new(
            "source-skeleton-a",
            vec!["source-composite-a".to_owned()],
        )
        .map_err(|error| {
            format!("left aggregate provenance failed: {error:?}")
        })?,
    );
    right_asset.source_provenance = Some(
        CharacterSourceProvenance::new(
            "source-skeleton-b",
            vec!["source-composite-b".to_owned()],
        )
        .map_err(|error| {
            format!("right aggregate provenance failed: {error:?}")
        })?,
    );
    let left_clip = clip("animation-0000", 1.)?
        .with_source_identity("source-clip-a")
        .map_err(|error| format!("left source clip failed: {error:?}"))?;
    let right_clip = clip("animation-0000", 1.)?
        .with_source_identity("source-clip-b")
        .map_err(|error| format!("right source clip failed: {error:?}"))?;
    let left_geometry = PreparedGeometry::RigidAnimated {
        asset: left_asset,
        animations: vec![left_clip],
    };
    let right_geometry = PreparedGeometry::RigidAnimated {
        asset: right_asset,
        animations: vec![right_clip],
    };
    let left_signature = prepared_signature(
        PropRoute::RigidAnimated,
        &left_geometry,
        &[],
        &[],
    );
    let right_signature = prepared_signature(
        PropRoute::RigidAnimated,
        &right_geometry,
        &[],
        &[],
    );
    if left_signature != right_signature {
        return Err("source provenance changed prop semantic dedupe".to_owned());
    }
    let left = PreparedProp {
        route: PropRoute::RigidAnimated,
        signature: left_signature,
        geometry: left_geometry,
        materials: Vec::new(),
        textures: Vec::new(),
    };
    let right = PreparedProp {
        route: PropRoute::RigidAnimated,
        signature: right_signature,
        geometry: right_geometry,
        materials: Vec::new(),
        textures: Vec::new(),
    };
    if visual_signature(&left) != visual_signature(&right) {
        return Err("source ordinal changed prop visual dedupe".to_owned());
    }
    if rig_signature(&left) != rig_signature(&right) {
        return Err("source bone provenance changed prop rig dedupe".to_owned());
    }
    Ok(())
}

#[test]
fn animation_key_ignores_source_clip_identity() -> Result<(), String> {
    let left = clip("first-name", 1.)?
        .with_source_identity("source-a")
        .map_err(|error| format!("left source identity failed: {error:?}"))?;
    let right = clip("second-name", 1.)?
        .with_source_identity("source-b")
        .map_err(|error| format!("right source identity failed: {error:?}"))?;
    if animation_key(&left) == animation_key(&right) {
        Ok(())
    } else {
        Err("source clip provenance changed world animation dedupe".to_owned())
    }
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

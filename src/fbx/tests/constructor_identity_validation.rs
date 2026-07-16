// File:
//   - constructor_identity_validation.rs
// Path:
//   - src/fbx/tests/constructor_identity_validation.rs
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
//   - Regression coverage for FBX constructor identity validation.
// - Must-Not:
//   - Read private assets, discover packages, or use filesystem fixtures.
// - Allows:
//   - Synthetic identities and public constructor assertions.
// - Split-When:
//   - One aggregate requires an independent integration boundary.
// - Merge-When:
//   - Constructor identity rules move behind one shared value object.
// - Summary:
//   - Protects domain and adapter constructors from noncanonical identities.
// - Description:
//   - Exercises synthetic caller values at explicit construction boundaries.
// - Usage:
//   - Run through the fbx crate test target.
// - Defaults:
//   - No local files or external processes are required.
//
// ADRs:
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
//
// Large file:
//   - true
//   - Reason: Public identity constructors share one synthetic, filesystem-free
//   - regression boundary and remain cohesive as a single conformance suite.
//

//! Regression coverage for FBX constructor identity validation.
//!
//! Synthetic identities prove public constructors fail closed before planning.

use fbx::domain::animation::{
    AnimationCapability, AnimationClip, AnimationClipError,
    AnimationRequirement, AnimationRequirementError, BoneAnimationTrack,
    LocalTransformSample,
};
use fbx::domain::mesh::{MeshAsset, MeshError, PrimitiveGroup};
use fbx::domain::shader::{
    MaterialChannel, ShaderRequirement, ShaderRequirementError,
};
use fbx::domain::texture::{MaterialBinding, MaterialBindingError};
use png as _;
use schoenwald_filesystem as _;
use serde as _;
use serde_json as _;
use shar_sha256 as _;

fn positions() -> Vec<[f32; 3]> {
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
    ]
}

fn animation_track(bone_id: &str) -> BoneAnimationTrack {
    BoneAnimationTrack {
        bone_id: bone_id.to_owned(),
        samples: vec![
            LocalTransformSample {
                translation: [
                    0.0_f64, 0.0_f64, 0.0_f64,
                ],
                rotation_wxyz: [
                    1.0_f64, 0.0_f64, 0.0_f64, 0.0_f64,
                ],
            },
        ],
    }
}

#[test]
fn rejects_padded_shader_texture_member_ids() {
    let result = ShaderRequirement::new(
        "shader",
        MaterialChannel::Diffuse,
        Some(" texture".to_owned()),
    );

    assert_eq!(
        result,
        Err(ShaderRequirementError::NonCanonicalTextureMemberId)
    );
}

#[test]
fn rejects_padded_shader_requirement_ids() {
    let result = ShaderRequirement::new(
        " shader",
        MaterialChannel::Diffuse,
        None,
    );

    assert_eq!(
        result,
        Err(ShaderRequirementError::NonCanonicalShaderId)
    );
}

#[test]
fn rejects_control_characters_in_shader_requirement_ids() {
    assert_eq!(
        ShaderRequirement::new(
            "shader\nalias",
            MaterialChannel::Diffuse,
            None,
        ),
        Err(ShaderRequirementError::NonCanonicalShaderId)
    );
    assert_eq!(
        ShaderRequirement::new(
            "shader",
            MaterialChannel::Diffuse,
            Some("texture\nalias".to_owned()),
        ),
        Err(ShaderRequirementError::NonCanonicalTextureMemberId)
    );
}

#[test]
fn rejects_case_insensitive_animation_member_aliases() {
    let result = AnimationRequirement::new(
        vec![
            "Walk".to_owned(),
            "walk".to_owned(),
        ],
        AnimationCapability::PreservedOnly,
    );

    assert_eq!(
        result,
        Err(AnimationRequirementError::DuplicateMemberId)
    );
}

#[test]
fn rejects_control_characters_in_animation_clip_identities() {
    assert_eq!(
        AnimationClip::new(
            "walk\nalias",
            30.0_f64,
            false,
            1,
            vec![animation_track("root")],
            Vec::new(),
        ),
        Err(AnimationClipError::InvalidClipName)
    );
    assert_eq!(
        AnimationClip::new(
            "walk",
            30.0_f64,
            false,
            1,
            vec![animation_track("root\nalias")],
            Vec::new(),
        ),
        Err(AnimationClipError::InvalidBoneId)
    );
    assert_eq!(
        AnimationClip::new(
            "walk",
            30.0_f64,
            false,
            1,
            vec![animation_track("root")],
            vec!["helper\nalias".to_owned()],
        ),
        Err(AnimationClipError::InvalidIgnoredGroup)
    );
}

#[test]
fn rejects_padded_animation_member_ids() {
    let result = AnimationRequirement::new(
        vec![" animation".to_owned()],
        AnimationCapability::PreservedOnly,
    );

    assert_eq!(
        result,
        Err(AnimationRequirementError::NonCanonicalMemberId)
    );
}

#[test]
fn rejects_control_characters_in_animation_member_ids() {
    let result = AnimationRequirement::new(
        vec!["animation\nalias".to_owned()],
        AnimationCapability::PreservedOnly,
    );

    assert_eq!(
        result,
        Err(AnimationRequirementError::NonCanonicalMemberId)
    );
}

#[test]
fn rejects_material_texture_path_traversal() {
    let result = MaterialBinding::new(
        "material",
        Some("../texture.png".to_owned()),
    );

    assert_eq!(
        result,
        Err(MaterialBindingError::InvalidTextureFileName)
    );
}

#[test]
fn rejects_padded_material_texture_file_names() {
    let result = MaterialBinding::new(
        "material",
        Some(" texture.png".to_owned()),
    );

    assert_eq!(
        result,
        Err(MaterialBindingError::NonCanonicalTextureFileName)
    );
}

#[test]
fn rejects_padded_material_binding_names() {
    let result = MaterialBinding::new(
        " material",
        None,
    );

    assert_eq!(
        result,
        Err(MaterialBindingError::NonCanonicalMaterialName)
    );
}

#[test]
fn rejects_control_characters_in_material_binding_names() {
    let result = MaterialBinding::new(
        "material\nalias",
        None,
    );

    assert_eq!(
        result,
        Err(MaterialBindingError::NonCanonicalMaterialName)
    );
}

#[test]
fn rejects_padded_mesh_asset_names() {
    let result = PrimitiveGroup::new(
        0,
        "shader",
        positions(),
        Vec::new(),
        &[
            0, 1, 2,
        ],
    )
    .and_then(
        |group| {
            MeshAsset::new(
                " mesh",
                vec![group],
            )
        },
    );

    assert_eq!(
        result,
        Err(MeshError::NonCanonicalMeshName)
    );
}

#[test]
fn rejects_control_characters_in_mesh_identities() {
    let invalid_mesh = PrimitiveGroup::new(
        0,
        "shader",
        positions(),
        Vec::new(),
        &[
            0, 1, 2,
        ],
    )
    .and_then(
        |group| {
            MeshAsset::new(
                "mesh\nalias",
                vec![group],
            )
        },
    );
    let invalid_shader = PrimitiveGroup::new(
        0,
        "shader\nalias",
        positions(),
        Vec::new(),
        &[
            0, 1, 2,
        ],
    );

    assert_eq!(
        invalid_mesh,
        Err(MeshError::NonCanonicalMeshName)
    );
    assert_eq!(
        invalid_shader,
        Err(MeshError::NonCanonicalShader)
    );
}

#[test]
fn rejects_padded_primitive_group_shader_ids() {
    let result = PrimitiveGroup::new(
        0,
        " shader",
        positions(),
        Vec::new(),
        &[
            0, 1, 2,
        ],
    );

    assert_eq!(
        result,
        Err(MeshError::NonCanonicalShader)
    );
}

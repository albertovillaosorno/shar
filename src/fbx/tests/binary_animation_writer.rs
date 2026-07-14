// File:
//   - binary_animation_writer.rs
// Path:
//   - src/fbx/tests/binary_animation_writer.rs
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
//   - Regression coverage for deterministic skeletal animation FBX objects.
// - Must-Not:
//   - Read game assets, invoke Blender, or rely on machine-local fixture paths.
// - Allows:
//   - Synthetic skinned geometry, sampled bone clips, and temporary artifacts.
// - Split-When:
//   - Non-transform animation requires another FBX object family.
// - Merge-When:
//   - Binary character writer conformance owns animation-specific assertions.
// - Summary:
//   - Protects stacks, layers, curves, takes, timing, and byte determinism.
// - Description:
//   - Writes one synthetic animated character twice and checks FBX contracts.
// - Usage:
//   - Run through the fbx crate test target.
// - Defaults:
//   - Temporary artifacts are process-unique and removed after each test.
//
// ADRs:
// - docs/adr/fbx/export/fbx-output-contract-boundary.md
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
//
// Large file:
//   - true
//   - Reason: The synthetic character fixture, deterministic graph assertions,
//   - and overflow rejection exercise one binary animation writer contract and
//   - remain together so setup and artifact semantics stay directly auditable.
//

//! Regression coverage for deterministic skeletal animation FBX objects.
//!
//! A synthetic skinned character proves stack, layer, curve, take, timing,
//! and repeated-write contracts without private assets or Blender.

/// Shared paired-artifact test helper.
#[path = "common/binary_artifact.rs"]
pub mod binary_artifact;

use std::path::PathBuf;

use binary_artifact::read_binary_pair;
use fbx::adapters::driven::binary_character_writer::{
    CharacterBinaryFbxError, CharacterBinaryFbxSummary,
    write_binary_character_fbx,
};
use fbx::domain::animation::{
    AnimationClip, BoneAnimationTrack, LocalTransformSample,
};
use fbx::domain::character::{CharacterAsset, SkinnedPart};
use fbx::domain::mesh::{MeshAsset, PrimitiveGroup};
use fbx::domain::skeleton::Bone;
use fbx::domain::skin::SkinInfluence;
use fbx::domain::texture::MaterialBinding;
use schoenwald_filesystem as _;
use serde as _;
use serde_json as _;

const ANIMATION_TOKENS: [&[u8]; 14] = [
    b"AnimationStack",
    b"AnimationLayer",
    b"AnimationCurveNode",
    b"AnimationCurve",
    b"walk\0\x01AnimStack",
    b"KeyTime",
    b"KeyValueFloat",
    b"Lcl Translation",
    b"Lcl Rotation",
    b"walk.tak",
    b"FbxAnimStack",
    b"FbxAnimLayer",
    b"FbxAnimCurveNode",
    b"FbxNode",
];

fn character() -> Result<CharacterAsset, String> {
    let group = PrimitiveGroup::new(
        0,
        "skin",
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
        vec![
            [
                0.0, 0.0,
            ],
            [
                1.0, 0.0,
            ],
            [
                0.0, 1.0,
            ],
        ],
        &[
            0, 1, 2,
        ],
    )
    .map_err(
        |error| format!("synthetic animation geometry failed: {error:?}"),
    )?;
    let mesh = MeshAsset::new(
        "body",
        vec![group],
    )
    .map_err(|error| format!("synthetic animation mesh failed: {error:?}"))?;
    let influences = (0_u32..3)
        .map(
            |vertex_index| SkinInfluence {
                vertex_index,
                bone_id: "root".to_owned(),
                weight: 1.0,
            },
        )
        .collect();
    CharacterAsset::new(
        "animated",
        vec![
            Bone {
                id: "root".to_owned(),
                parent_id: None,
                rest_matrix: [
                    1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0,
                    0.0, 0.0, 0.0, 1.0,
                ],
            },
        ],
        vec![
            SkinnedPart {
                mesh,
                group_influences: vec![influences],
            },
        ],
    )
    .map_err(|error| format!("synthetic animated character failed: {error:?}"))
}

fn animation() -> Result<AnimationClip, String> {
    let half_turn = std::f64::consts::FRAC_1_SQRT_2;
    AnimationClip::new(
        "walk",
        30.0,
        true,
        3,
        vec![
            BoneAnimationTrack {
                bone_id: "root".to_owned(),
                samples: vec![
                    LocalTransformSample {
                        translation: [
                            0.0, 0.0, 0.0,
                        ],
                        rotation_wxyz: [
                            1.0, 0.0, 0.0, 0.0,
                        ],
                    },
                    LocalTransformSample {
                        translation: [
                            1.0, 0.0, 0.0,
                        ],
                        rotation_wxyz: [
                            half_turn, 0.0, 0.0, half_turn,
                        ],
                    },
                    LocalTransformSample {
                        translation: [
                            2.0, 0.0, 0.0,
                        ],
                        rotation_wxyz: [
                            0.0, 0.0, 0.0, 1.0,
                        ],
                    },
                ],
            },
        ],
        vec!["IK_Helper".to_owned()],
    )
    .map_err(|error| format!("synthetic animation clip failed: {error:?}"))
}

fn two_frame_animation(
    name: &str,
    frame_rate: f64,
) -> Result<AnimationClip, String> {
    let samples = vec![
        LocalTransformSample {
            translation: [
                0.0_f64, 0.0_f64, 0.0_f64,
            ],
            rotation_wxyz: [
                1.0_f64, 0.0_f64, 0.0_f64, 0.0_f64,
            ],
        },
        LocalTransformSample {
            translation: [
                0.0_f64, 0.0_f64, 0.0_f64,
            ],
            rotation_wxyz: [
                1.0_f64, 0.0_f64, 0.0_f64, 0.0_f64,
            ],
        },
    ];
    AnimationClip::new(
        name,
        frame_rate,
        false,
        samples.len(),
        vec![
            BoneAnimationTrack {
                bone_id: "root".to_owned(),
                samples,
            },
        ],
        Vec::new(),
    )
    .map_err(|error| format!("two-frame animation clip failed: {error:?}"))
}

fn overflowing_animation() -> Result<AnimationClip, String> {
    const KTIME_PER_SECOND: f64 = 46_186_158_000.0;
    const I64_EXCLUSIVE_MAX: f64 = 9_223_372_036_854_775_808.0;
    two_frame_animation(
        "overflow",
        KTIME_PER_SECOND / I64_EXCLUSIVE_MAX,
    )
}

fn sub_tick_animation() -> Result<AnimationClip, String> {
    const KTIME_PER_SECOND: f64 = 46_186_158_000.0;
    two_frame_animation(
        "sub-tick",
        KTIME_PER_SECOND * 2.0_f64,
    )
}

fn output_path(label: &str) -> PathBuf {
    std::env::temp_dir().join(
        format!(
            "fbx-binary-animation-{label}-{}.fbx",
            std::process::id()
        ),
    )
}

fn contains(
    bytes: &[u8],
    needle: &[u8],
) -> bool {
    bytes
        .windows(needle.len())
        .any(|window| window == needle)
}

#[test]
fn writes_deterministic_animation_graph() {
    let first_path = output_path("first");
    let second_path = output_path("second");
    let character_result = character();
    assert!(
        character_result.is_ok(),
        "synthetic animated character should build: {character_result:?}"
    );
    let Some(character) = character_result.ok() else {
        return;
    };
    let material_result = MaterialBinding::new(
        "skin", None,
    );
    assert!(
        material_result.is_ok(),
        "synthetic animation material should build: {material_result:?}"
    );
    let Some(material) = material_result.ok() else {
        return;
    };
    let animation_result = animation();
    assert!(
        animation_result.is_ok(),
        "synthetic animation should build: {animation_result:?}"
    );
    let Some(animation) = animation_result.ok() else {
        return;
    };
    let materials = vec![material];
    let clips = vec![animation];

    let first_summary = write_binary_character_fbx(
        &character,
        &materials,
        &[],
        &clips,
        &first_path,
    );
    let second_summary = write_binary_character_fbx(
        &character,
        &materials,
        &[],
        &clips,
        &second_path,
    );
    let artifacts = read_binary_pair(
        &first_path,
        &second_path,
        "animated FBX",
    );
    let Some((first, second)) = artifacts else {
        return;
    };

    assert_eq!(
        first_summary,
        Ok(
            CharacterBinaryFbxSummary {
                geometries: 1,
                bones: 1,
                clusters: 1,
                materials: 1,
                textures: 0,
                animations: 1,
            }
        )
    );
    assert_eq!(
        second_summary,
        first_summary
    );
    assert_eq!(
        first,
        second
    );
    for token in ANIMATION_TOKENS {
        assert!(
            contains(
                &first, token
            ),
            "missing binary FBX token: {token:?}"
        );
    }
}

#[test]
fn rejects_animation_time_above_signed_fbx_limit() {
    let path = output_path("overflow");
    let character_result = character();
    assert!(
        character_result.is_ok(),
        "synthetic animated character should build: {character_result:?}"
    );
    let Some(character) = character_result.ok() else {
        return;
    };
    let material_result = MaterialBinding::new(
        "skin", None,
    );
    assert!(
        material_result.is_ok(),
        "synthetic animation material should build: {material_result:?}"
    );
    let Some(material) = material_result.ok() else {
        return;
    };
    let animation_result = overflowing_animation();
    assert!(
        animation_result.is_ok(),
        "overflow animation should satisfy domain input: {animation_result:?}"
    );
    let Some(animation) = animation_result.ok() else {
        return;
    };

    let result = write_binary_character_fbx(
        &character,
        &[material],
        &[],
        &[animation],
        &path,
    );

    assert_eq!(
        result,
        Err(
            CharacterBinaryFbxError::AnimationPlan {
                reason: "TimeOverflow".to_owned(),
            }
        )
    );
    assert!(
        !path.exists(),
        "overflow rejection must not leave a success-like artifact"
    );
}

#[test]
fn rejects_animation_frames_below_one_ktime_tick() {
    let path = output_path("sub-tick");
    let character_result = character();
    assert!(
        character_result.is_ok(),
        "synthetic animated character should build: {character_result:?}"
    );
    let Some(character) = character_result.ok() else {
        return;
    };
    let material_result = MaterialBinding::new(
        "skin", None,
    );
    assert!(
        material_result.is_ok(),
        "synthetic animation material should build: {material_result:?}"
    );
    let Some(material) = material_result.ok() else {
        return;
    };
    let animation_result = sub_tick_animation();
    assert!(
        animation_result.is_ok(),
        "sub-tick animation should satisfy domain input: {animation_result:?}"
    );
    let Some(animation) = animation_result.ok() else {
        return;
    };

    let result = write_binary_character_fbx(
        &character,
        &[material],
        &[],
        &[animation],
        &path,
    );
    let artifact_created = path.exists();
    if artifact_created {
        let cleanup_result = std::fs::remove_file(&path);
        assert!(
            cleanup_result.is_ok(),
            "sub-tick failure evidence should clean its temporary artifact: \
             {cleanup_result:?}"
        );
    }

    assert_eq!(
        result,
        Err(
            CharacterBinaryFbxError::AnimationPlan {
                reason: "TimeResolution".to_owned(),
            }
        )
    );
    assert!(
        !artifact_created,
        "sub-tick animation rejection must precede artifact creation"
    );
}

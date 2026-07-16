// File:
//   - decoded_animation_source.rs
// Path:
//   - src/fbx/tests/decoded_animation_source.rs
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
//   - Regression coverage for decoded skeletal animation reconstruction.
// - Must-Not:
//   - Read game assets, use machine-local fixed paths, or invoke Blender.
// - Allows:
//   - Synthetic decoded JSON and process-unique temporary directories.
// - Split-When:
//   - Texture-name controllers gain an independent decoded adapter.
// - Merge-When:
//   - Animation adapter conformance tests own the same source contract.
// - Summary:
//   - Protects Pure3D channel semantics before FBX serialization.
// - Description:
//   - Exercises signed quaternions, compact vectors, interpolation, and
//   - binding.
// - Usage:
//   - Run through the fbx crate test target.
// - Defaults:
//   - Synthetic fixtures are removed after every regression.
//
// ADRs:
// - docs/adr/fbx/export/fbx-output-contract-boundary.md
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
//
// Large file:
//   - true
//   - Reason: The decoded JSON fixture, compact-channel assertions, and
//   - skeleton-binding checks form one inseparable source-contract regression.
//

//! Regression coverage for decoded skeletal animation reconstruction.
//!
//! Synthetic PTRN evidence protects signed quaternions, compact-vector axis
//! mappings, interpolation modes, NUL trimming, and helper-group exclusion.

use std::fs;
use std::path::PathBuf;

use fbx::adapters::driven::decoded_animation_source::load_animation_clips;
use fbx::domain::animation::BoneAnimationTrack;
use fbx::domain::animation::quaternion::decode_signed_i16_wxyz;
use fbx::domain::skeleton::Bone;
use png as _;
use schoenwald_filesystem as _;
use serde as _;
use serde_json as _;
use shar_sha256 as _;

const TOLERANCE: f64 = 1e-10;

fn temp_root() -> PathBuf {
    std::env::temp_dir().join(
        format!(
            "fbx-decoded-animation-{}",
            std::process::id()
        ),
    )
}

const fn rest_matrix(translation: [f32; 3]) -> [f32; 16] {
    [
        1.0,
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
        0.0,
        translation[0],
        translation[1],
        translation[2],
        1.0,
    ]
}

const fn fixture_json() -> &'static str {
    r#"{
      "schema":"animation",
      "name":"walk\u0000\u0000",
      "version":0,
      "type":"PTRN",
      "frames":3.0,
      "frame_rate":30.0,
      "cyclic":1,
      "sizes":[],
      "group_lists":[{"version":0,"num_groups":3,"groups":[
        {"version":0,"name":"Root\u0000","group_id":0,"num_channels":2,
         "channels":[
          {"kind":"vector1","version":0,"param":"TRAN","mapping":0,
           "constants":[1.0,2.0,3.0],"num_frames":2,"frames":[0,2],
           "values":[[1.0],[3.0]],"channel_metadata":[
             {"kind":"interpolation_mode","version":0,"mode":1}]},
          {"kind":"compressed_quaternion","version":0,"param":"ROT_",
           "num_frames":2,"frames":[0,2],
           "compressed_values":[[32769,0,0,0],[0,0,0,32767]],
           "channel_metadata":[
             {"kind":"interpolation_mode","version":0,"mode":1}]}
         ]},
        {"version":0,"name":"Child","group_id":1,"num_channels":1,
         "channels":[
          {"kind":"vector2","version":0,"param":"TRAN","mapping":1,
           "constants":[4.0,5.0,6.0],"num_frames":2,"frames":[0,2],
           "values":[[4.0,6.0],[8.0,10.0]],"channel_metadata":[
             {"kind":"interpolation_mode","version":0,"mode":1}]}
         ]},
        {"version":0,"name":"IK_Helper\u0000","group_id":2,
         "num_channels":1,"channels":[
          {"kind":"vector1","version":0,"param":"TRAN","mapping":1,
           "constants":[0.0,7.0,0.0],"num_frames":2,"frames":[0,2],
           "values":[[7.0],[9.0]],"channel_metadata":[
             {"kind":"interpolation_mode","version":0,"mode":0}]}
         ]}
      ]}],
      "loose_channels":[],
      "legacy_animation_extras":[]
    }"#
}

fn assert_vector_close(
    actual: [f64; 3],
    expected: [f64; 3],
) {
    for (actual_value, expected_value) in actual
        .iter()
        .zip(expected)
    {
        assert!(
            (actual_value - expected_value).abs() <= TOLERANCE,
            "vector component differed: actual={actual_value} \
             expected={expected_value}"
        );
    }
}

fn assert_quaternion_close(
    actual: [f64; 4],
    expected: [f64; 4],
) {
    for (actual_value, expected_value) in actual
        .iter()
        .zip(expected)
    {
        assert!(
            (actual_value - expected_value).abs() <= TOLERANCE,
            "quaternion component differed: actual={actual_value} \
             expected={expected_value}"
        );
    }
}

fn assert_root_track(track: &BoneAnimationTrack) {
    assert_eq!(
        track.bone_id, "Root",
        "root track should bind to the root skeleton bone"
    );
    assert_eq!(
        track
            .samples
            .len(),
        3,
        "root track should contain every integer source frame"
    );
    let Some(first) = track
        .samples
        .first()
    else {
        return;
    };
    let Some(middle) = track
        .samples
        .get(1)
    else {
        return;
    };
    let Some(last) = track
        .samples
        .get(2)
    else {
        return;
    };
    assert_vector_close(
        first.translation,
        [
            1.0_f64, 2.0_f64, 3.0_f64,
        ],
    );
    assert_vector_close(
        middle.translation,
        [
            2.0_f64, 2.0_f64, 3.0_f64,
        ],
    );
    assert_vector_close(
        last.translation,
        [
            3.0_f64, 2.0_f64, 3.0_f64,
        ],
    );
    assert_quaternion_close(
        first.rotation_wxyz,
        [
            -1.0_f64, 0.0_f64, 0.0_f64, 0.0_f64,
        ],
    );
    let middle_length = middle
        .rotation_wxyz
        .iter()
        .map(|value| value * value)
        .sum::<f64>();
    assert!(
        (middle_length - 1.0_f64).abs() < TOLERANCE,
        "interpolated root quaternion should remain unit length"
    );
}

fn assert_child_track(track: &BoneAnimationTrack) {
    assert_eq!(
        track.bone_id, "Child",
        "child track should bind to the child skeleton bone"
    );
    assert_eq!(
        track
            .samples
            .len(),
        3,
        "child track should contain every integer source frame"
    );
    let Some(first) = track
        .samples
        .first()
    else {
        return;
    };
    let Some(middle) = track
        .samples
        .get(1)
    else {
        return;
    };
    let Some(last) = track
        .samples
        .get(2)
    else {
        return;
    };
    assert_vector_close(
        first.translation,
        [
            4.0_f64, 5.0_f64, 6.0_f64,
        ],
    );
    assert_vector_close(
        middle.translation,
        [
            6.0_f64, 5.0_f64, 8.0_f64,
        ],
    );
    assert_vector_close(
        last.translation,
        [
            8.0_f64, 5.0_f64, 10.0_f64,
        ],
    );
}

#[test]
fn reconstructs_compact_channels_and_preserves_helper_evidence() {
    let root = temp_root();
    let path = root.join("animation.json");
    let setup = fs::create_dir_all(&root).and_then(
        |()| {
            fs::write(
                &path,
                fixture_json(),
            )
        },
    );
    assert!(
        setup.is_ok(),
        "synthetic animation fixture should be writable"
    );
    let bones = vec![
        Bone {
            id: "Root".to_owned(),
            parent_id: None,
            rest_matrix: rest_matrix(
                [
                    1.0_f32, 2.0_f32, 3.0_f32,
                ],
            ),
        },
        Bone {
            id: "Child".to_owned(),
            parent_id: Some("Root".to_owned()),
            rest_matrix: rest_matrix(
                [
                    4.0_f32, 5.0_f32, 6.0_f32,
                ],
            ),
        },
    ];
    let result = load_animation_clips(
        &[path.as_path()],
        &bones,
    );
    let cleanup = fs::remove_dir_all(&root);
    assert!(
        cleanup.is_ok(),
        "synthetic animation fixture should be removed"
    );
    assert!(
        result.is_ok(),
        "synthetic PTRN clip should decode: {result:?}"
    );
    let Some(clips) = result.ok() else {
        return;
    };
    assert_eq!(
        clips.len(),
        1
    );
    let Some(clip) = clips.first() else {
        return;
    };
    assert_eq!(
        clip.name,
        "walk"
    );
    assert!((clip.frame_rate - 30.0_f64).abs() <= TOLERANCE);
    assert!(clip.cyclic);
    assert_eq!(
        clip.frame_count,
        3
    );
    assert_eq!(
        clip.ignored_group_ids,
        vec!["IK_Helper"]
    );
    assert_eq!(
        clip.tracks
            .len(),
        2
    );
    let Some(root_track) = clip
        .tracks
        .first()
    else {
        return;
    };
    let Some(child_track) = clip
        .tracks
        .get(1)
    else {
        return;
    };
    assert_root_track(root_track);
    assert_child_track(child_track);
}

#[test]
fn decodes_compressed_words_as_signed_wxyz_components() {
    let result = decode_signed_i16_wxyz(
        [
            32_769, 0, 0, 0,
        ],
    );
    assert!(
        result.is_ok(),
        "signed identity quaternion should decode: {result:?}"
    );
    let Some(decoded) = result.ok() else {
        return;
    };
    assert_quaternion_close(
        decoded,
        [
            -1.0_f64, 0.0_f64, 0.0_f64, 0.0_f64,
        ],
    );
}

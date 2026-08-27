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
//   - Decoded animation source test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Decoded animation source test module.
// - Description:
//   - Implements the declared test module responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Decoded animation source test module.

use std::fs;
use std::path::PathBuf;

use fbx::adapters::driven::decoded_animation_source::{
    DecodedAnimationError, load_animation_clips,
};
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
    std::env::temp_dir()
        .join(format!("fbx-decoded-animation-{}", std::process::id()))
}

const fn rest_matrix(translation: [f32; 3]) -> [f32; 16] {
    [
        1.,
        0.,
        0.,
        0.,
        0.,
        1.,
        0.,
        0.,
        0.,
        0.,
        1.,
        0.,
        translation[0],
        translation[1],
        translation[2],
        1.,
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
      "group_lists":[{"version":0,"num_groups":4,"groups":[
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
        {"version":0,"name":"Z_Helper\u0000","group_id":2,
         "num_channels":1,"channels":[
          {"kind":"vector1","version":0,"param":"TRAN","mapping":1,
           "constants":[0.0,7.0,0.0],"num_frames":2,"frames":[0,2],
           "values":[[7.0],[9.0]],"channel_metadata":[
             {"kind":"interpolation_mode","version":0,"mode":0}]}
         ]},
        {"version":0,"name":"A_Helper","group_id":3,
         "num_channels":1,"channels":[
          {"kind":"vector1","version":0,"param":"TRAN","mapping":1,
           "constants":[0.0,5.0,0.0],"num_frames":2,"frames":[0,2],
           "values":[[5.0],[6.0]],"channel_metadata":[
             {"kind":"interpolation_mode","version":0,"mode":0}]}
         ]}
      ]}],
      "loose_channels":[],
      "legacy_animation_extras":[]
    }"#
}

fn assert_vector_close(actual: [f64; 3], expected: [f64; 3]) {
    for (actual_value, expected_value) in actual.iter().zip(expected) {
        assert!(
            (actual_value - expected_value).abs() <= TOLERANCE,
            "vector component differed: actual={actual_value} \
             expected={expected_value}"
        );
    }
}

fn assert_quaternion_close(actual: [f64; 4], expected: [f64; 4]) {
    for (actual_value, expected_value) in actual.iter().zip(expected) {
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
        track.samples.len(),
        3,
        "root track should contain every integer source frame"
    );
    let Some(first) = track.samples.first() else {
        return;
    };
    let Some(middle) = track.samples.get(1) else {
        return;
    };
    let Some(last) = track.samples.get(2) else {
        return;
    };
    assert_vector_close(first.translation, [1f64, 2f64, 3f64]);
    assert_vector_close(middle.translation, [2f64, 2f64, 3f64]);
    assert_vector_close(last.translation, [3f64, 2f64, 3f64]);
    assert_quaternion_close(first.rotation_wxyz, [-1f64, 0f64, 0f64, 0f64]);
    let middle_length = middle
        .rotation_wxyz
        .iter()
        .map(|value| value * value)
        .sum::<f64>();
    assert!(
        (middle_length - 1f64).abs() < TOLERANCE,
        "interpolated root quaternion should remain unit length"
    );
}

fn assert_child_track(track: &BoneAnimationTrack) {
    assert_eq!(
        track.bone_id, "Child",
        "child track should bind to the child skeleton bone"
    );
    assert_eq!(
        track.samples.len(),
        3,
        "child track should contain every integer source frame"
    );
    let Some(first) = track.samples.first() else {
        return;
    };
    let Some(middle) = track.samples.get(1) else {
        return;
    };
    let Some(last) = track.samples.get(2) else {
        return;
    };
    assert_vector_close(first.translation, [4f64, 5f64, 6f64]);
    assert_vector_close(middle.translation, [6f64, 5f64, 8f64]);
    assert_vector_close(last.translation, [8f64, 5f64, 10f64]);
}

#[test]
fn reconstructs_compact_channels_and_preserves_helper_evidence() {
    let root = temp_root();
    let path = root.join("animation.json");
    let setup = fs::create_dir_all(&root)
        .and_then(|()| fs::write(&path, fixture_json()));
    assert!(
        setup.is_ok(),
        "synthetic animation fixture should be writable"
    );
    let bones = vec![
        Bone {
            id: "Root".to_owned(),
            parent_id: None,
            rest_matrix: rest_matrix([1f32, 2f32, 3f32]),
            source_rig: None,
        },
        Bone {
            id: "Child".to_owned(),
            parent_id: Some("Root".to_owned()),
            rest_matrix: rest_matrix([4f32, 5f32, 6f32]),
            source_rig: None,
        },
    ];
    let result = load_animation_clips(&[path.as_path()], &bones);
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
    assert_eq!(clips.len(), 1);
    let Some(clip) = clips.first() else {
        return;
    };
    assert_eq!(clip.name, "walk");
    assert!((clip.frame_rate - 30f64).abs() <= TOLERANCE);
    assert!(clip.cyclic);
    assert_eq!(clip.frame_count, 3);
    assert_eq!(clip.ignored_group_ids, vec!["Z_Helper", "A_Helper"]);
    assert_eq!(clip.tracks.len(), 2);
    let Some(root_track) = clip.tracks.first() else {
        return;
    };
    let Some(child_track) = clip.tracks.get(1) else {
        return;
    };
    assert_root_track(root_track);
    assert_child_track(child_track);
}

#[test]
fn decodes_compressed_words_as_signed_wxyz_components() {
    let result = decode_signed_i16_wxyz([32_769, 0, 0, 0]);
    assert!(
        result.is_ok(),
        "signed identity quaternion should decode: {result:?}"
    );
    let Some(decoded) = result.ok() else {
        return;
    };
    assert_quaternion_close(decoded, [-1f64, 0f64, 0f64, 0f64]);
}

#[test]
fn rejects_unbound_top_level_animation_data() {
    let cases = [
        ("loose", "\"loose_channels\":[]", "\"loose_channels\":[{}]"),
        (
            "legacy",
            "\"legacy_animation_extras\":[]",
            "\"legacy_animation_extras\":[{}]",
        ),
    ];
    for (label, empty, populated) in cases {
        let root = temp_root().with_file_name(format!(
            "fbx-decoded-animation-{label}-{}",
            std::process::id()
        ));
        let path = root.join("animation.json");
        let fixture = fixture_json().replace(empty, populated);
        let setup =
            fs::create_dir_all(&root).and_then(|()| fs::write(&path, fixture));
        assert!(
            setup.is_ok(),
            "synthetic top-level fixture should be writable"
        );
        let bones = [Bone {
            id: "Root".to_owned(),
            parent_id: None,
            rest_matrix: rest_matrix([1., 2., 3.]),
            source_rig: None,
        }];
        let result = load_animation_clips(&[path.as_path()], &bones);
        let cleanup = fs::remove_dir_all(&root);
        assert!(
            cleanup.is_ok(),
            "synthetic top-level fixture should be removed"
        );

        assert_eq!(
            result,
            Err(DecodedAnimationError::UnsupportedTopLevelAnimationData)
        );
    }
}

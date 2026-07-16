// File:
//   - blender_review_helper.rs
// Path:
//   - src/fbx/tests/blender_review_helper.rs
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
//   - Synthetic regressions for deterministic Blender review helpers.
// - Must-Not:
//   - Invoke Blender, read game assets, or use machine-local fixture paths.
// - Allows:
//   - Write temporary standalone helpers from synthetic animation clips.
// - Split-When:
//   - Runtime Blender acceptance gains an independent fixture contract.
// - Merge-When:
//   - Another test owns the same helper materialization behavior.
// - Summary:
//   - Protects typed optional Blender helper generation.
// - Description:
//   - Verifies deterministic standalone output and native timing.
// - Usage:
//   - Run through the `fbx` crate integration-test target.
// - Defaults:
//   - Temporary generated files are removed after every test.
//
// ADRs:
// - docs/adr/fbx/export/fbx-output-contract-boundary.md
//
// Large file:
//   - false
//

//! Synthetic Blender review-helper generation regressions.
//!
//! These tests build clips from public synthetic values and materialize
//! standalone helpers in temporary directories. They prove byte determinism,
//! identity substitution, strict timing rejection, and the presence of the
//! armature, Action-slot, Pose Position, and native-timing preservation
//! contracts.
//!
//! Blender itself and private game assets remain outside the test boundary.
//! Runtime acceptance is performed separately against generated review
//! artifacts, while these tests permanently guard the deterministic generator.

use std::fs;
use std::path::PathBuf;

use fbx::adapters::driven::blender_review_helper::{
    HelperError, HelperSummary, write_review_helper,
};
use fbx::domain::animation::{
    AnimationClip, BoneAnimationTrack, LocalTransformSample,
};
use png as _;
use schoenwald_filesystem as _;
use serde as _;
use serde_json as _;
use shar_sha256 as _;

const fn sample() -> LocalTransformSample {
    LocalTransformSample {
        translation: [
            0.0, 0.0, 0.0,
        ],
        rotation_wxyz: [
            1.0, 0.0, 0.0, 0.0,
        ],
    }
}

fn clip(
    name: &str,
    frame_rate: f64,
) -> Result<AnimationClip, String> {
    AnimationClip::new(
        name,
        frame_rate,
        true,
        2,
        vec![
            BoneAnimationTrack {
                bone_id: "root".to_owned(),
                samples: vec![
                    sample(),
                    sample(),
                ],
            },
        ],
        Vec::new(),
    )
    .map_err(|error| format!("synthetic clip failed: {error:?}"))
}

fn temp_root(label: &str) -> PathBuf {
    std::env::temp_dir().join(
        format!(
            "fbx-blender-helper-{label}-{}",
            std::process::id()
        ),
    )
}

fn assert_script_contract(script: &str) -> Result<(), String> {
    for required in [
        "# File:\n#   - helper_template.py",
        "# Path:\n#   - src/fbx/python/shar_blender_review/helper_template.py",
        "# SPDX-License-Identifier:\n#   - MIT",
        "# Confidential:\n#   - false",
        "# License-File:\n#   - LICENSE",
        concat!(
            "# Path-Rule:\n",
            "#   - All paths in this header are repository-root relative.",
        ),
        "# ruff: noqa: INP001",
        "Experimental unsupported Blender FBX review helper",
        "FBX_FILE_NAME: str = \"characters-apu-base-model.fbx\"",
        "SOURCE_FPS: int = 30",
        "scene.render.fps = SOURCE_FPS",
        "armature.data.pose_position = \"POSE\"",
        "animation_data.action_slot = slot",
        "action.name.endswith(\"_loco_walk\")",
        "def _resolve_script_path",
        "for text in bpy.data.texts",
        "script_path = _resolve_script_path(bpy)",
        "text.name == direct.name",
    ] {
        if !script.contains(required) {
            return Err(
                format!("missing generated helper contract: {required}"),
            );
        }
    }
    for forbidden in [
        "All rights reserved",
        "script_path = Path(__file__).resolve()",
        "PREVIEW_FPS",
        "def _retime_action",
        "coordinate.x =",
        "point.interpolation =",
        "scene.sync_mode =",
        "__SHAR_FBX_FILE_NAME__",
    ] {
        if script.contains(forbidden) {
            return Err(format!("forbidden helper contract: {forbidden}"));
        }
    }
    Ok(())
}

#[test]
fn writes_deterministic_typed_review_bundle() -> Result<(), String> {
    let clip = clip(
        "apu_loco_walk",
        30.0,
    )?;
    let first_root = temp_root("first");
    let second_root = temp_root("second");
    fs::create_dir_all(&first_root)
        .map_err(|error| format!("first temp root failed: {error}"))?;
    fs::create_dir_all(&second_root)
        .map_err(|error| format!("second temp root failed: {error}"))?;
    let fbx_path = PathBuf::from("characters-apu-base-model.fbx");
    let first_path = first_root.join("characters-apu-base-model.blender.py");
    let second_path = second_root.join("characters-apu-base-model.blender.py");
    let first = write_review_helper(
        &fbx_path,
        std::slice::from_ref(&clip),
        &first_path,
    );
    let second = write_review_helper(
        &fbx_path,
        &[clip],
        &second_path,
    );
    let first_script = fs::read_to_string(&first_path)
        .map_err(|error| format!("first helper read failed: {error}"))?;
    let second_script = fs::read_to_string(&second_path)
        .map_err(|error| format!("second helper read failed: {error}"))?;
    let first_marker_exists = first_root
        .join("__init__.py")
        .exists();
    let second_marker_exists = second_root
        .join("__init__.py")
        .exists();
    fs::remove_dir_all(&first_root)
        .map_err(|error| format!("first cleanup failed: {error}"))?;
    fs::remove_dir_all(&second_root)
        .map_err(|error| format!("second cleanup failed: {error}"))?;

    let expected = Ok(
        HelperSummary {
            source_fps: 30,
            files: 1,
        },
    );
    if first != expected || second != expected {
        return Err(
            format!("unexpected helper summaries: {first:?} {second:?}"),
        );
    }
    if first_script != second_script {
        return Err(
            "repeated helper scripts must be byte-identical".to_owned(),
        );
    }
    if first_marker_exists || second_marker_exists {
        return Err("helper export must not emit __init__.py".to_owned());
    }
    assert_script_contract(&first_script)
}

#[test]
fn rejects_missing_and_mixed_animation_timing() -> Result<(), String> {
    let root = temp_root("invalid");
    fs::create_dir_all(&root)
        .map_err(|error| format!("invalid temp root failed: {error}"))?;
    let output = root.join("character.blender.py");
    let missing = write_review_helper(
        &PathBuf::from("character.fbx"),
        &[],
        &output,
    );
    let first = clip(
        "first", 30.0,
    )?;
    let second = clip(
        "second", 24.0,
    )?;
    let mixed = write_review_helper(
        &PathBuf::from("character.fbx"),
        &[
            first, second,
        ],
        &output,
    );
    fs::remove_dir_all(root)
        .map_err(|error| format!("invalid cleanup failed: {error}"))?;
    if missing != Err(HelperError::MissingAnimations) {
        return Err(format!("unexpected missing timing result: {missing:?}"));
    }
    if mixed != Err(HelperError::MixedFrameRate) {
        return Err(format!("unexpected mixed timing result: {mixed:?}"));
    }
    Ok(())
}

// File:
//   - domain_validation.rs
// Path:
//   - src/fbx/tests/domain_validation.rs
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
//   - Regression coverage for independent FBX domain value invariants.
// - Must-Not:
//   - Access private assets, perform filesystem discovery, or call adapters.
// - Allows:
//   - Synthetic identities, numeric values, and public domain constructors.
// - Split-When:
//   - One aggregate requires fixtures or adapter integration.
// - Merge-When:
//   - Domain value regressions move into a more specific existing test target.
// - Summary:
//   - Protects normalized domain values before planning and serialization.
// - Description:
//   - Exercises public constructors with deterministic synthetic evidence.
// - Usage:
//   - Run through the fbx crate test target.
// - Defaults:
//   - No local files, generated assets, or external processes are required.
//
// ADRs:
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
//
// Large file:
//   - false
//

//! Regression coverage for independent normalized FBX domain value invariants.
//!
//! Synthetic evidence verifies invalid values fail before application planning,
//! adapter staging, or deterministic scene serialization.

use std::path::PathBuf;

use fbx::adapters::driven::blender_scene_writer::{
    BlenderCommandPlan, BlenderCommandPlanError,
};
use fbx::adapters::driving::cli::{
    CliExportSelection, CliExportSelectionError,
};
use fbx::domain::animation::{
    AnimationCapability, AnimationRequirement, AnimationRequirementError,
};
use fbx::domain::shader::{
    MaterialChannel, ShaderRequirement, ShaderRequirementError,
};
use fbx::ports::scene_writer::{
    SceneArtifactError, SceneArtifactReceipt, SceneArtifactTarget,
};
use png as _;
use schoenwald_filesystem as _;
use serde as _;
use serde_json as _;
use shar_sha256 as _;

#[test]
fn rejects_incomplete_blender_command_plans() {
    assert_eq!(
        BlenderCommandPlan::new(
            "   ",
            "script.py",
            "output.fbx"
        ),
        Err(BlenderCommandPlanError::MissingExecutable)
    );
    assert_eq!(
        BlenderCommandPlan::new(
            "blender",
            "   ",
            "output.fbx"
        ),
        Err(BlenderCommandPlanError::MissingScript)
    );
    assert_eq!(
        BlenderCommandPlan::new(
            "blender",
            "script.py",
            "   "
        ),
        Err(BlenderCommandPlanError::MissingOutputFile)
    );
}

#[test]
fn rejects_incomplete_cli_export_selections() {
    assert_eq!(
        CliExportSelection::new(
            "   ",
            "output.fbx"
        ),
        Err(CliExportSelectionError::MissingPackageSelector)
    );
    assert_eq!(
        CliExportSelection::new(
            "package",
            PathBuf::new()
        ),
        Err(CliExportSelectionError::MissingOutputFile)
    );
}

#[test]
fn rejects_blank_scene_artifact_receipt_location() {
    assert_eq!(
        SceneArtifactReceipt::new("   "),
        Err(SceneArtifactError::MissingReceiptLocation)
    );
}

#[test]
fn rejects_blank_scene_artifact_target_identity() {
    assert_eq!(
        SceneArtifactTarget::new("   "),
        Err(SceneArtifactError::MissingArtifactId)
    );
}

#[test]
fn rejects_invalid_shader_requirement_identities() {
    assert_eq!(
        ShaderRequirement::new(
            "   ",
            MaterialChannel::Diffuse,
            None,
        ),
        Err(ShaderRequirementError::MissingShaderId)
    );
    assert_eq!(
        ShaderRequirement::new(
            "shader",
            MaterialChannel::Diffuse,
            Some("   ".to_owned()),
        ),
        Err(ShaderRequirementError::BlankTextureMemberId)
    );
}

#[test]
fn rejects_invalid_animation_member_identities() {
    assert_eq!(
        AnimationRequirement::new(
            vec!["   ".to_owned()],
            AnimationCapability::PreservedOnly,
        ),
        Err(AnimationRequirementError::BlankMemberId)
    );
    assert_eq!(
        AnimationRequirement::new(
            vec![
                "clip".to_owned(),
                "clip".to_owned()
            ],
            AnimationCapability::PreservedOnly,
        ),
        Err(AnimationRequirementError::DuplicateMemberId)
    );
}

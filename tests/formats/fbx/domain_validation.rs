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
//   - Domain validation test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Domain validation test module.
// - Description:
//   - Implements the declared test module responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Domain validation test module.

use std::path::PathBuf;

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
fn rejects_incomplete_cli_export_selections() {
    assert_eq!(
        CliExportSelection::new("   ", "output.fbx"),
        Err(CliExportSelectionError::MissingPackageSelector)
    );
    assert_eq!(
        CliExportSelection::new("package", PathBuf::new()),
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
        ShaderRequirement::new("   ", MaterialChannel::Diffuse, None,),
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
            vec!["clip".to_owned(), "clip".to_owned()],
            AnimationCapability::PreservedOnly,
        ),
        Err(AnimationRequirementError::DuplicateMemberId)
    );
}

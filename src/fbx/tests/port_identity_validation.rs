// File:
//   - port_identity_validation.rs
// Path:
//   - src/fbx/tests/port_identity_validation.rs
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
//   - Regression coverage for FBX port and adapter identity validation.
// - Must-Not:
//   - Read private assets, discover packages, or invoke external processes.
// - Allows:
//   - Synthetic request identities and public constructor assertions.
// - Split-When:
//   - One adapter requires filesystem or process integration evidence.
// - Merge-When:
//   - Port identity rules move behind one shared value object.
// - Summary:
//   - Protects driving and driven boundaries from noncanonical identities.
// - Description:
//   - Exercises synthetic values at explicit port and adapter constructors.
// - Usage:
//   - Run through the fbx crate test target.
// - Defaults:
//   - No local files or external processes are required.
//
// ADRs:
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
//
// Large file:
//   - false
//

//! Regression coverage for FBX port and adapter identity validation.
//!
//! Synthetic identities prove boundary constructors fail closed before work.

use fbx::adapters::driven::blender_scene_writer::{
    BlenderCommandPlan, BlenderCommandPlanError,
};
use fbx::adapters::driving::cli::{
    CliExportSelection, CliExportSelectionError,
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
fn rejects_padded_blender_output_file_ids() {
    let result = BlenderCommandPlan::new(
        "blender",
        "script.py",
        " output.fbx",
    );

    assert_eq!(
        result,
        Err(BlenderCommandPlanError::NonCanonicalOutputFile)
    );
}

#[test]
fn rejects_padded_blender_script_ids() {
    let result = BlenderCommandPlan::new(
        "blender",
        " script.py",
        "output.fbx",
    );

    assert_eq!(
        result,
        Err(BlenderCommandPlanError::NonCanonicalScript)
    );
}

#[test]
fn rejects_padded_blender_executable_ids() {
    let result = BlenderCommandPlan::new(
        " blender",
        "script.py",
        "output.fbx",
    );

    assert_eq!(
        result,
        Err(BlenderCommandPlanError::NonCanonicalExecutable)
    );
}

#[test]
fn rejects_control_characters_in_blender_command_fields() {
    assert_eq!(
        BlenderCommandPlan::new(
            "blender\nalias",
            "script.py",
            "output.fbx",
        ),
        Err(BlenderCommandPlanError::NonCanonicalExecutable)
    );
    assert_eq!(
        BlenderCommandPlan::new(
            "blender",
            "script\nalias.py",
            "output.fbx",
        ),
        Err(BlenderCommandPlanError::NonCanonicalScript)
    );
    assert_eq!(
        BlenderCommandPlan::new(
            "blender",
            "script.py",
            "output\nalias.fbx",
        ),
        Err(BlenderCommandPlanError::NonCanonicalOutputFile)
    );
}

#[test]
fn rejects_padded_cli_output_paths() {
    let result = CliExportSelection::new(
        "package",
        " output.fbx",
    );

    assert_eq!(
        result,
        Err(CliExportSelectionError::NonCanonicalOutputFile)
    );
}

#[test]
fn rejects_padded_cli_package_selectors() {
    let result = CliExportSelection::new(
        " package",
        "output.fbx",
    );

    assert_eq!(
        result,
        Err(CliExportSelectionError::NonCanonicalPackageSelector)
    );
}

#[test]
fn rejects_control_characters_in_cli_export_fields() {
    assert_eq!(
        CliExportSelection::new(
            "package\nalias",
            "output.fbx",
        ),
        Err(CliExportSelectionError::NonCanonicalPackageSelector)
    );
    assert_eq!(
        CliExportSelection::new(
            "package",
            "output\nalias.fbx",
        ),
        Err(CliExportSelectionError::NonCanonicalOutputFile)
    );
}

#[test]
fn rejects_padded_scene_artifact_receipt_locations() {
    let result = SceneArtifactReceipt::new(" location");

    assert_eq!(
        result,
        Err(SceneArtifactError::NonCanonicalReceiptLocation)
    );
}

#[test]
fn rejects_padded_scene_artifact_target_ids() {
    let result = SceneArtifactTarget::new(" artifact");

    assert_eq!(
        result,
        Err(SceneArtifactError::NonCanonicalArtifactId)
    );
}

#[test]
fn rejects_control_characters_in_scene_artifact_identities() {
    assert_eq!(
        SceneArtifactTarget::new("artifact\nalias"),
        Err(SceneArtifactError::NonCanonicalArtifactId)
    );
    assert_eq!(
        SceneArtifactReceipt::new("location\nalias"),
        Err(SceneArtifactError::NonCanonicalReceiptLocation)
    );
}

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
//   - Port identity validation test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Port identity validation test module.
// - Description:
//   - Implements the declared test module responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Port identity validation test module.

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
fn rejects_padded_cli_output_paths() {
    let result = CliExportSelection::new("package", " output.fbx");

    assert_eq!(result, Err(CliExportSelectionError::NonCanonicalOutputFile));
}

#[test]
fn rejects_padded_cli_package_selectors() {
    let result = CliExportSelection::new(" package", "output.fbx");

    assert_eq!(
        result,
        Err(CliExportSelectionError::NonCanonicalPackageSelector)
    );
}

#[test]
fn rejects_control_characters_in_cli_export_fields() {
    assert_eq!(
        CliExportSelection::new("package\nalias", "output.fbx",),
        Err(CliExportSelectionError::NonCanonicalPackageSelector)
    );
    assert_eq!(
        CliExportSelection::new("package", "output\nalias.fbx",),
        Err(CliExportSelectionError::NonCanonicalOutputFile)
    );
}

#[test]
fn rejects_padded_scene_artifact_receipt_locations() {
    let result = SceneArtifactReceipt::new(" location");

    assert_eq!(result, Err(SceneArtifactError::NonCanonicalReceiptLocation));
}

#[test]
fn rejects_padded_scene_artifact_target_ids() {
    let result = SceneArtifactTarget::new(" artifact");

    assert_eq!(result, Err(SceneArtifactError::NonCanonicalArtifactId));
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

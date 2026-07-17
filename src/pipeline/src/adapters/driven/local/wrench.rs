// File:
//   - wrench.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/wrench.rs
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
//   - Canonical standalone Wrench model export orchestration.
// - Must-Not:
//   - Export collection effects, billboard quads, particles, lights, or camera
//   - and gameplay data through FBX.
// - Allows:
//   - Staged assembly, deterministic verification, and atomic publication.
// - Split-When:
//   - Wrench assembly or artifact verification needs an independent boundary.
// - Merge-When:
//   - A stable generic model-prop exporter owns the same lifecycle contract.
// - Summary:
//   - Publishes only the original Wrench model, rig, material, and animation.
// - Description:
//   - Excludes the separate wrench-collection presentation and effects package.
// - Usage:
//   - Called by the `fbx-export-wrench` pipeline command.
// - Defaults:
//   - Output and staging directories must not already exist.
//
// ADRs:
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
// - docs/adr/fbx/export/fbx-output-contract-boundary.md
//
// Large file:
//   - false
//

//! Canonical standalone Wrench model export orchestration.

use std::path::{Path, PathBuf};

use fbx::adapters::driven::binary_character_writer::CharacterBinaryFbxSummary;
use schoenwald_filesystem::adapters::driving::local::create_dir_all;

use self::assembly::build_wrench;
use crate::domain::{PipelineError, StageReport};

mod artifact;
mod assembly;

/// Stable pipeline stage name.
const STAGE: &str = "fbx-export-wrench";
/// Canonical source package.
const SOURCE_PACKAGE_ID: &str = "extracted-art-wrench";
/// Exact package root required by the canonical package id.
const SOURCE_PACKAGE_ROOT: &str = "extracted/art/wrench";
/// Standalone asset identity written into the FBX scene.
const ASSET_NAME: &str = "wrench";
/// Canonical skeletal transform clip for the visible model.
const ANIMATION_MEMBER: &str = "components/animation/animation_0007.json";
/// Composite containing the visible mesh-to-joint binding.
const COMPOSITE_MEMBER: &str = "components/composite_drawable/wrench.json";
/// Canonical Wrench skeleton.
const SKELETON_MEMBER: &str = "components/skeleton/wrench.json";
/// The only actual polygon mesh in the package. Collection quads remain out.
const BODY_MESH_MEMBERS: [&str; 1] = ["components/mesh/wrench7Shape.json"];
/// Expected root and selected rigid-mesh joint.
const EXPECTED_BONES: usize = 2;
/// Expected primitive groups across the selected mesh.
const EXPECTED_GEOMETRIES: usize = 1;
/// Expected rigid clusters, one for the selected primitive group.
const EXPECTED_CLUSTERS: usize = 1;
/// Expected unique material identities.
const EXPECTED_MATERIALS: usize = 1;
/// Expected external texture bindings.
const EXPECTED_TEXTURE_BINDINGS: usize = 1;

/// Export one canonical Wrench model with its authored skeletal animation.
///
/// # Errors
///
/// Returns an error when staged assembly, verification, or publication fails.
pub(super) fn export_wrench(
    index_path: &Path,
    output_dir: &Path,
    base_root: &Path,
) -> Result<StageReport, PipelineError> {
    ensure_missing(
        output_dir,
        "Wrench output",
    )?;
    let staging = staging_path(output_dir)?;
    ensure_missing(
        &staging,
        "Wrench staging",
    )?;
    create_dir_all(&staging).map_err(
        |error| PipelineError::new(format!("Wrench staging failed: {error}")),
    )?;
    let result = build_wrench(
        index_path, &staging, base_root,
    )
    .and_then(
        |(files, bytes, summary)| {
            publish(
                &staging, output_dir, files, bytes, &summary,
            )
        },
    );
    if result.is_err() {
        let _cleanup_result = std::fs::remove_dir_all(&staging);
    }
    result
}

/// Publish verified staging and build the observable stage report.
fn publish(
    staging: &Path,
    output_dir: &Path,
    files: usize,
    bytes: u64,
    summary: &CharacterBinaryFbxSummary,
) -> Result<StageReport, PipelineError> {
    std::fs::rename(
        staging, output_dir,
    )
    .map_err(
        |error| {
            PipelineError::new(format!("Wrench publication failed: {error}"))
        },
    )?;
    Ok(
        StageReport {
            name: STAGE,
            files,
            bytes,
            note: format!(
                "published one canonical Wrench FBX with {} geometry, {} \
                 retained bones, and {} animation clip",
                summary.geometries, summary.bones, summary.animations
            ),
        },
    )
}

/// Require one path not to exist before deterministic publication.
fn ensure_missing(
    path: &Path,
    label: &str,
) -> Result<(), PipelineError> {
    if path.exists() {
        return Err(
            PipelineError::new(
                format!(
                    "{label} already exists: {}",
                    path.display()
                ),
            ),
        );
    }
    Ok(())
}

/// Build a sibling hidden staging path for one output directory.
fn staging_path(output_dir: &Path) -> Result<PathBuf, PipelineError> {
    let file_name = output_dir
        .file_name()
        .ok_or_else(
            || PipelineError::new("Wrench output has no final path segment"),
        )?;
    let mut staging_name = String::from(".");
    staging_name.push_str(&file_name.to_string_lossy());
    staging_name.push_str(".staging");
    Ok(
        output_dir
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .join(staging_name),
    )
}

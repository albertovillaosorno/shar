// File:
//   - wasp_camera.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/wasp_camera.rs
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
//   - Atomic publication of one canonical standalone Wasp Camera FBX package.
// - Must-Not:
//   - Select source components or implement material and artifact verification.
// - Allows:
//   - Delegate staged assembly and publish the verified directory atomically.
// - Split-When:
//   - Publication gains another transaction or output format.
// - Merge-When:
//   - A generic animated-prop publisher owns the same transaction contract.
// - Summary:
//   - Publishes one canonical animated Wasp Camera FBX package.
// - Description:
//   - Owns output absence, staging, cleanup, rename, and stage reporting.
// - Usage:
//   - Invoked by the `fbx-export-wasp-camera` pipeline command.
// - Defaults:
//   - Output must be absent and uses a deterministic sibling staging directory.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
// - docs/adr/fbx/export/fbx-output-contract-boundary.md
//
// Large file:
//   - true
//   - Reason: Atomic publication and its explicit source-selection constants
//   - remain together so the standalone asset contract is directly auditable.
//

//! Canonical standalone Wasp Camera FBX publication.
use std::path::{Path, PathBuf};

use fbx::adapters::driven::binary_character_writer::CharacterBinaryFbxSummary;
use fbx::domain::mesh::MeshAsset;
use fbx::domain::texture::MaterialBinding;
use schoenwald_filesystem::adapters::driving::local::{
    create_dir_all, path_kind,
};
use schoenwald_filesystem::domain::PathKind;

use self::assembly::{build_wasp_camera, build_wasp_guide_source};
use crate::domain::{PipelineError, StageReport};

mod artifact;
mod assembly;

/// One exact canonical Wasp texture payload for structural-guide assembly.
pub(in crate::adapters::driven::local) struct WaspGuideTexture {
    /// Portable texture file identity.
    pub(in crate::adapters::driven::local) file_name: String,
    /// Exact PNG payload bytes.
    pub(in crate::adapters::driven::local) bytes: Vec<u8>,
    /// Exact lowercase SHA-256.
    pub(in crate::adapters::driven::local) sha256: String,
}

/// Canonical static Wasp body and presentation authority for guide placement.
pub(in crate::adapters::driven::local) struct WaspGuideSource {
    /// Fourteen rest-pose-baked body meshes.
    pub(in crate::adapters::driven::local) meshes: Vec<MeshAsset>,
    /// Seven canonical body material bindings.
    pub(in crate::adapters::driven::local) materials: Vec<MaterialBinding>,
    /// Exact referenced external texture payloads.
    pub(in crate::adapters::driven::local) textures: Vec<WaspGuideTexture>,
}

/// Collect the canonical static Wasp body for source-backed guide placements.
///
/// # Errors
///
/// Returns an error when exact source selection, bind-pose baking, material
/// resolution, or texture hashing fails.
pub(in crate::adapters::driven::local) fn collect_wasp_guide_source(
    index_path: &Path,
    base_root: &Path,
    texture_dir: &Path,
) -> Result<WaspGuideSource, PipelineError> {
    build_wasp_guide_source(
        index_path,
        base_root,
        texture_dir,
    )
}

/// Stable pipeline stage name.
const STAGE: &str = "fbx-export-wasp-camera";
/// Canonical duplicate selected from the seven level FX packages.
const SOURCE_PACKAGE_ID: &str = "extracted-art-l01-fx";
/// Exact package root required by the canonical package id.
const SOURCE_PACKAGE_ROOT: &str = "extracted/art/l01_fx";
/// Standalone asset identity written into the FBX scene.
const ASSET_NAME: &str = "wasp-camera";
/// Canonical skeletal transform clip for the animated object factory.
const ANIMATION_MEMBER: &str = "components/animation/animation_0014.json";
/// Canonical composite containing rigid prop-to-joint bindings.
const COMPOSITE_MEMBER: &str = "components/composite_drawable/beecamera.json";
/// Canonical Wasp Camera skeleton.
const SKELETON_MEMBER: &str = "components/skeleton/beecamera.json";
/// Exact body meshes; FX, shield, ray, explosion, and billboard geometry stay
/// out.
const BODY_MESH_MEMBERS: [&str; 14] = [
    "components/mesh/BodyShape.json",
    "components/mesh/PelvisShape.json",
    "components/mesh/TailShape.json",
    "components/mesh/StingerShape.json",
    "components/mesh/Wing_LShape.json",
    "components/mesh/Wing_RShape.json",
    "components/mesh/wasp_armShape.json",
    "components/mesh/wasp_armShape1.json",
    "components/mesh/wasp_armShape2.json",
    "components/mesh/wasp_armShape3.json",
    "components/mesh/wasp_armShape4.json",
    "components/mesh/wasp_armShape5.json",
    "components/mesh/NeckShape.json",
    "components/mesh/headShape.json",
];
/// Expected pruned rig size: two roots plus fourteen selected body joints.
const EXPECTED_BONES: usize = 16;
/// Expected primitive groups across the fourteen selected body meshes.
const EXPECTED_GEOMETRIES: usize = 19;
/// Expected rigid clusters, one for each selected primitive group.
const EXPECTED_CLUSTERS: usize = 19;
/// Expected unique body material identities.
const EXPECTED_MATERIALS: usize = 7;
/// Expected FBX texture bindings across the body material identities.
const EXPECTED_TEXTURE_BINDINGS: usize = 7;

/// Export one canonical Wasp Camera body with its skeletal animation.
///
/// # Errors
///
/// Returns an error when staged assembly, verification, or publication fails.
pub(super) fn export_wasp_camera(
    index_path: &Path,
    output_dir: &Path,
    base_root: &Path,
) -> Result<StageReport, PipelineError> {
    ensure_missing(
        output_dir,
        "Wasp Camera output",
    )?;
    let staging = staging_path(output_dir)?;
    ensure_missing(
        &staging,
        "Wasp Camera staging",
    )?;
    create_dir_all(&staging).map_err(
        |error| {
            PipelineError::new(format!("Wasp Camera staging failed: {error}"))
        },
    )?;
    let result = build_wasp_camera(
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
            PipelineError::new(
                format!("Wasp Camera publication failed: {error}"),
            )
        },
    )?;
    Ok(
        StageReport {
            name: STAGE,
            files,
            bytes,
            note: format!(
                "published one canonical Wasp Camera FBX with {} body \
                 geometries, {} retained bones, and {} animation clip",
                summary.geometries, summary.bones, summary.animations
            ),
        },
    )
}

/// Require publication and staging roots to be absent.
fn ensure_missing(
    path: &Path,
    label: &str,
) -> Result<(), PipelineError> {
    match path_kind(path).map_err(
        |error| {
            PipelineError::new(format!("{label} inspection failed: {error}"))
        },
    )? {
        PathKind::Missing => Ok(()),
        _ => Err(PipelineError::new(format!("{label} already exists"))),
    }
}

/// Derive a deterministic sibling staging directory.
fn staging_path(output_dir: &Path) -> Result<PathBuf, PipelineError> {
    let file_name = output_dir
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(
            || PipelineError::new("Wasp Camera output has no UTF-8 name"),
        )?;
    let parent = output_dir
        .parent()
        .ok_or_else(
            || PipelineError::new("Wasp Camera output has no parent"),
        )?;
    Ok(parent.join(format!(".{file_name}.staging")))
}

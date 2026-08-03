// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Assembly outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Assembly outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Assembly outbound adapter.

use std::path::{Path, PathBuf};

use fbx::adapters::driven::binary_character_writer::{
    CharacterBinaryFbxSummary, write_binary_character_fbx,
};
use fbx::adapters::driven::decoded_animation_source::load_animation_clips;
use fbx::adapters::driven::decoded_rigid_prop_source;
use fbx::domain::character::CharacterAsset;
use schoenwald_filesystem::adapters::driving::local::create_dir_all;

use super::artifact::{
    inventory, resolve_materials, verify_body_scope, verify_summary,
};
use super::{
    ANIMATION_MEMBER, ASSET_NAME, BODY_MESH_MEMBERS, COMPOSITE_MEMBER,
    SKELETON_MEMBER, SOURCE_PACKAGE_ID, SOURCE_PACKAGE_ROOT,
};
use crate::domain::PipelineError;
use crate::domain::package::{PhaseThreePackageIndex, PhaseThreePackageRow};

/// Build and verify the staged standalone asset.
pub(super) fn build_wasp_camera(
    index_path: &Path,
    staging: &Path,
    base_root: &Path,
) -> Result<(usize, u64, CharacterBinaryFbxSummary), PipelineError> {
    let LoadedWaspBody {
        asset,
        package_root,
        animation_path,
    } = load_wasp_body(index_path, base_root)?;
    let animations =
        load_animation_clips(&[animation_path.as_path()], &asset.bones)
            .map_err(|error| {
                PipelineError::new(format!(
                    "Wasp Camera animation failed: {error:?}"
                ))
            })?;
    verify_animation(&animations)?;
    let texture_dir = staging.join("textures");
    create_dir_all(&texture_dir).map_err(|error| {
        PipelineError::new(format!(
            "Wasp Camera texture staging failed: {error}"
        ))
    })?;
    let materials = resolve_materials(&asset, &package_root, &texture_dir)?;
    let fbx_path = staging.join("wasp-camera.fbx");
    let summary =
        write_binary_character_fbx(&asset, &materials, &animations, &fbx_path)
            .map_err(|error| {
                PipelineError::new(format!("Wasp Camera FBX failed: {error:?}"))
            })?;
    verify_summary(&summary)?;
    let (files, bytes) = inventory(&fbx_path, &texture_dir, &materials)?;
    Ok((files, bytes, summary))
}

/// Canonical selected body plus the exact package paths it owns.
struct LoadedWaspBody {
    /// Rest-pose-baked selected body asset.
    asset: CharacterAsset,
    /// Decoded package root used for material resolution.
    package_root: PathBuf,
    /// Exact canonical animation path used only by the standalone exporter.
    animation_path: PathBuf,
}

/// Load and verify the one canonical selected Wasp body.
fn load_wasp_body(
    index_path: &Path,
    base_root: &Path,
) -> Result<LoadedWaspBody, PipelineError> {
    let index = PhaseThreePackageIndex::read(index_path)
        .map_err(|error| PipelineError::new(error.to_string()))?;
    let package = canonical_package(&index)?;
    let skeleton_path = member_path(package, SKELETON_MEMBER, base_root)?;
    let composite_path = member_path(package, COMPOSITE_MEMBER, base_root)?;
    let animation_path = member_path(package, ANIMATION_MEMBER, base_root)?;
    let mesh_paths = BODY_MESH_MEMBERS
        .iter()
        .map(|relative| member_path(package, relative, base_root))
        .collect::<Result<Vec<_>, _>>()?;
    let mesh_refs = mesh_paths.iter().map(PathBuf::as_path).collect::<Vec<_>>();
    let asset = decoded_rigid_prop_source::load_selected_rigid_prop_asset(
        ASSET_NAME,
        &skeleton_path,
        &mesh_refs,
        &composite_path,
    )
    .map_err(|error| {
        PipelineError::new(format!("Wasp Camera assembly failed: {error:?}"))
    })?;
    verify_body_scope(&asset)?;
    Ok(LoadedWaspBody {
        asset,
        package_root: base_root.join(SOURCE_PACKAGE_ROOT),
        animation_path,
    })
}

/// Return the exact canonical level FX package.
fn canonical_package(
    index: &PhaseThreePackageIndex,
) -> Result<&PhaseThreePackageRow, PipelineError> {
    let package = index.find_package(SOURCE_PACKAGE_ID).ok_or_else(|| {
        PipelineError::new("canonical Wasp Camera package is missing")
    })?;
    if package.package_root != SOURCE_PACKAGE_ROOT {
        return Err(PipelineError::new(format!(
            "canonical Wasp Camera root changed: {}",
            package.package_root
        )));
    }
    Ok(package)
}

/// Resolve the exact generated-index member path for one canonical component.
fn member_path(
    package: &PhaseThreePackageRow,
    relative: &str,
    base_root: &Path,
) -> Result<PathBuf, PipelineError> {
    let expected = format!("{SOURCE_PACKAGE_ROOT}/{relative}");
    let mut matches = package
        .members()
        .iter()
        .filter(|member| member.path == expected);
    let member = matches.next().ok_or_else(|| {
        PipelineError::new(format!("Wasp Camera member is missing: {expected}"))
    })?;
    if matches.next().is_some() {
        return Err(PipelineError::new(format!(
            "Wasp Camera member is duplicated: {expected}"
        )));
    }
    Ok(base_root.join(&member.path))
}

/// Require one canonical PTRN clip and no inferred state splits.
fn verify_animation(
    animations: &[fbx::domain::animation::AnimationClip],
) -> Result<(), PipelineError> {
    if animations.len() != 1
        || animations
            .first()
            .is_none_or(|clip| clip.name != "PTRN_beecamera")
    {
        return Err(PipelineError::new(
            "Wasp Camera must publish exactly the canonical \
                 PTRN_beecamera clip",
        ));
    }
    Ok(())
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/wasp_camera/assembly/tests.rs"]
mod tests;

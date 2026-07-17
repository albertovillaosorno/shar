// File:
//   - assembly.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/wrench/assembly.rs
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
//   - Canonical Wrench component selection and FBX assembly.
// - Must-Not:
//   - Select collection quads, effects, lights, particles, or inferred clips.
// - Allows:
//   - Generated-index lookup and decoded rigid-prop assembly.
// - Split-When:
//   - Source selection and serialization become independently reusable.
// - Merge-When:
//   - A stable generic model-prop assembler owns the same source contract.
// - Summary:
//   - Builds the visible Wrench model from exact indexed component evidence.
// - Description:
//   - Prunes unused skeleton branches and binds only `PTRN_wrench`.
// - Usage:
//   - Called by the Wrench export orchestrator.
// - Defaults:
//   - Package identity, root, component paths, and clip identity are exact.
//
// ADRs:
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
// - docs/adr/fbx/export/fbx-output-contract-boundary.md
//
// Large file:
//   - false
//

//! Canonical Wrench component selection and FBX assembly.

use std::path::{Path, PathBuf};

use fbx::adapters::driven::binary_character_writer::{
    CharacterBinaryFbxSummary, write_binary_character_fbx,
};
use fbx::adapters::driven::decoded_animation_source::load_animation_clips;
use fbx::adapters::driven::decoded_rigid_prop_source;
use schoenwald_filesystem::adapters::driving::local::create_dir_all;

use super::artifact::{
    inventory, resolve_materials, verify_model_scope, verify_summary,
};
use super::{
    ANIMATION_MEMBER, ASSET_NAME, BODY_MESH_MEMBERS, COMPOSITE_MEMBER,
    SKELETON_MEMBER, SOURCE_PACKAGE_ID, SOURCE_PACKAGE_ROOT,
};
use crate::domain::PipelineError;
use crate::domain::package::{PhaseThreePackageIndex, PhaseThreePackageRow};

/// Build and verify the staged standalone asset.
pub(super) fn build_wrench(
    index_path: &Path,
    staging: &Path,
    base_root: &Path,
) -> Result<
    (
        usize,
        u64,
        CharacterBinaryFbxSummary,
    ),
    PipelineError,
> {
    let index = PhaseThreePackageIndex::read(index_path)
        .map_err(|error| PipelineError::new(error.to_string()))?;
    let package = canonical_package(&index)?;
    let skeleton_path = member_path(
        package,
        SKELETON_MEMBER,
        base_root,
    )?;
    let composite_path = member_path(
        package,
        COMPOSITE_MEMBER,
        base_root,
    )?;
    let animation_path = member_path(
        package,
        ANIMATION_MEMBER,
        base_root,
    )?;
    let mesh_paths = BODY_MESH_MEMBERS
        .iter()
        .map(
            |relative| {
                member_path(
                    package, relative, base_root,
                )
            },
        )
        .collect::<Result<Vec<_>, _>>()?;
    let mesh_refs = mesh_paths
        .iter()
        .map(PathBuf::as_path)
        .collect::<Vec<_>>();
    let asset = decoded_rigid_prop_source::load_selected_rigid_prop_asset(
        ASSET_NAME,
        &skeleton_path,
        &mesh_refs,
        &composite_path,
    )
    .map_err(
        |error| {
            PipelineError::new(format!("Wrench assembly failed: {error:?}"))
        },
    )?;
    verify_model_scope(&asset)?;
    let animations = load_animation_clips(
        &[animation_path.as_path()],
        &asset.bones,
    )
    .map_err(
        |error| {
            PipelineError::new(format!("Wrench animation failed: {error:?}"))
        },
    )?;
    verify_animation(&animations)?;
    let texture_dir = staging.join("textures");
    create_dir_all(&texture_dir).map_err(
        |error| {
            PipelineError::new(
                format!("Wrench texture staging failed: {error}"),
            )
        },
    )?;
    let package_root = base_root.join(SOURCE_PACKAGE_ROOT);
    let materials = resolve_materials(
        &asset,
        &package_root,
        &texture_dir,
    )?;
    let fbx_path = staging.join("wrench.fbx");
    let summary = write_binary_character_fbx(
        &asset,
        &materials,
        &animations,
        &fbx_path,
    )
    .map_err(
        |error| PipelineError::new(format!("Wrench FBX failed: {error:?}")),
    )?;
    verify_summary(&summary)?;
    let (files, bytes) = inventory(
        &fbx_path,
        &texture_dir,
        &materials,
    )?;
    Ok(
        (
            files, bytes, summary,
        ),
    )
}

/// Return the exact canonical Wrench package.
fn canonical_package(
    index: &PhaseThreePackageIndex
) -> Result<&PhaseThreePackageRow, PipelineError> {
    let package = index
        .find_package(SOURCE_PACKAGE_ID)
        .ok_or_else(
            || PipelineError::new("canonical Wrench package is missing"),
        )?;
    if package.package_root != SOURCE_PACKAGE_ROOT {
        return Err(
            PipelineError::new(
                format!(
                    "canonical Wrench root changed: {}",
                    package.package_root
                ),
            ),
        );
    }
    Ok(package)
}

/// Resolve one exact generated-index member path.
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
    let member = matches
        .next()
        .ok_or_else(
            || {
                PipelineError::new(
                    format!("Wrench member is missing: {expected}"),
                )
            },
        )?;
    if matches
        .next()
        .is_some()
    {
        return Err(
            PipelineError::new(
                format!("Wrench member is duplicated: {expected}"),
            ),
        );
    }
    Ok(base_root.join(&member.path))
}

/// Require one canonical transform clip and no inferred state splits.
fn verify_animation(
    animations: &[fbx::domain::animation::AnimationClip]
) -> Result<(), PipelineError> {
    let Some(clip) = animations.first() else {
        return Err(PipelineError::new("Wrench animation clip is missing"));
    };
    if animations.len() != 1
        || clip.name != "PTRN_wrench"
        || clip.frame_count != 61
        || clip
            .frame_rate
            .to_bits()
            != 30.0_f64.to_bits()
        || !clip.cyclic
    {
        return Err(
            PipelineError::new(
                format!(
                    "Wrench must publish exactly the canonical 61-frame \
                     PTRN_wrench clip: {clip:?}"
                ),
            ),
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::BODY_MESH_MEMBERS;

    #[test]
    fn model_selection_excludes_collection_presentation() {
        assert_eq!(
            BODY_MESH_MEMBERS,
            ["components/mesh/wrench7Shape.json"]
        );
        assert!(
            !BODY_MESH_MEMBERS
                .contains(&"components/quad_group/wrenchShape4.json")
        );
        assert!(
            !BODY_MESH_MEMBERS
                .contains(&"components/composite_drawable/wrench_collect.json")
        );
    }
}

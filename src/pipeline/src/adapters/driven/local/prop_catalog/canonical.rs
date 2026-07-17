// File:
//   - canonical.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/prop_catalog/canonical.rs
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
//   - Source-name-independent canonical identities for prepared prop geometry.
// - Must-Not:
//   - Read files, resolve materials, serialize FBX, or alter numeric samples.
// - Allows:
//   - Stable part, bone, influence, and animation identity replacement.
// - Split-When:
//   - Static and animated identity rules diverge independently.
// - Merge-When:
//   - Source adapters emit canonical identities directly without data loss.
// - Summary:
//   - Makes visually identical level copies converge during semantic hashing.
// - Description:
//   - Preserves geometry, hierarchy, transforms, skinning, timing, and samples.
// - Usage:
//   - Applied after source loading and material content normalization.
// - Defaults:
//   - Parts and tracks retain deterministic source order after selected
//     sorting.
//
// ADRs:
// - docs/adr/fbx/export/fbx-output-contract-boundary.md
//
// Large file:
//   - false
//

//! Source-name-independent canonical prop identities.

use std::collections::BTreeMap;

use fbx::domain::animation::AnimationClip;
use fbx::domain::character::CharacterAsset;
use fbx::domain::mesh::MeshAsset;

use crate::domain::PipelineError;

/// Canonicalize static mesh names after material normalization.
pub(super) fn canonicalize_static_meshes(meshes: &mut [MeshAsset]) {
    meshes.sort_by(
        |left, right| {
            format!(
                "{:?}",
                left.groups
            )
            .cmp(
                &format!(
                    "{:?}",
                    right.groups
                ),
            )
        },
    );
    for (ordinal, mesh) in meshes
        .iter_mut()
        .enumerate()
    {
        mesh.name = format!("part-{ordinal:04}");
    }
}

/// Canonicalize one rigid asset and its exact clips without changing samples.
///
/// # Errors
///
/// Returns an error when a parent, influence, or track references a missing
/// bone.
pub(super) fn canonicalize_animated_asset(
    asset: &mut CharacterAsset,
    animations: &mut [AnimationClip],
) -> Result<(), PipelineError> {
    let bone_names = asset
        .bones
        .iter()
        .enumerate()
        .map(
            |(ordinal, bone)| {
                (
                    bone.id
                        .clone(),
                    format!("bone-{ordinal:04}"),
                )
            },
        )
        .collect::<BTreeMap<_, _>>();
    for bone in &mut asset.bones {
        bone.id = mapped_bone(
            &bone_names,
            &bone.id,
        )?;
        bone.parent_id = bone
            .parent_id
            .as_ref()
            .map(
                |parent| {
                    mapped_bone(
                        &bone_names,
                        parent,
                    )
                },
            )
            .transpose()?;
    }
    asset
        .parts
        .sort_by(
            |left, right| {
                format!(
                    "{:?}",
                    left.mesh
                        .groups
                )
                .cmp(
                    &format!(
                        "{:?}",
                        right
                            .mesh
                            .groups
                    ),
                )
            },
        );
    for (ordinal, part) in asset
        .parts
        .iter_mut()
        .enumerate()
    {
        part.mesh
            .name = format!("part-{ordinal:04}");
        for influence in part
            .group_influences
            .iter_mut()
            .flatten()
        {
            influence.bone_id = mapped_bone(
                &bone_names,
                &influence.bone_id,
            )?;
        }
    }
    "model".clone_into(&mut asset.name);
    for (clip_ordinal, clip) in animations
        .iter_mut()
        .enumerate()
    {
        clip.name = format!("animation-{clip_ordinal:04}");
        clip.ignored_group_ids
            .clear();
        for track in &mut clip.tracks {
            track.bone_id = mapped_bone(
                &bone_names,
                &track.bone_id,
            )?;
        }
        clip.tracks
            .sort_by(
                |left, right| {
                    left.bone_id
                        .cmp(&right.bone_id)
                },
            );
    }
    Ok(())
}

/// Resolve one original bone identity to its canonical ordinal identity.
fn mapped_bone(
    names: &BTreeMap<String, String>,
    source: &str,
) -> Result<String, PipelineError> {
    names
        .get(source)
        .cloned()
        .ok_or_else(
            || {
                PipelineError::new(
                    format!("canonical prop bone mapping is missing: {source}"),
                )
            },
        )
}

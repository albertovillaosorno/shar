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
//   - Canonical outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Canonical outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Canonical outbound adapter.

use std::collections::BTreeMap;

use fbx::domain::animation::AnimationClip;
use fbx::domain::character::CharacterAsset;
use fbx::domain::mesh::MeshAsset;

use crate::domain::PipelineError;

/// Canonicalize static mesh names in source mesh order.
pub(super) fn canonicalize_static_meshes(meshes: &mut [MeshAsset]) {
    for (ordinal, mesh) in meshes.iter_mut().enumerate() {
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
        .map(|(ordinal, bone)| (bone.id.clone(), format!("bone-{ordinal:04}")))
        .collect::<BTreeMap<_, _>>();
    for bone in &mut asset.bones {
        bone.id = mapped_bone(&bone_names, &bone.id)?;
        bone.parent_id = bone
            .parent_id
            .as_ref()
            .map(|parent| mapped_bone(&bone_names, parent))
            .transpose()?;
    }
    for (ordinal, part) in asset.parts.iter_mut().enumerate() {
        part.mesh.name = format!("part-{ordinal:04}");
        for influence in part.group_influences.iter_mut().flatten() {
            influence.bone_id = mapped_bone(&bone_names, &influence.bone_id)?;
        }
    }
    "model".clone_into(&mut asset.name);
    for (clip_ordinal, clip) in animations.iter_mut().enumerate() {
        clip.name = format!("animation-{clip_ordinal:04}");
        for track in &mut clip.tracks {
            track.bone_id = mapped_bone(&bone_names, &track.bone_id)?;
        }
        clip.tracks
            .sort_by(|left, right| left.bone_id.cmp(&right.bone_id));
    }
    Ok(())
}

/// Resolve one original bone identity to its canonical ordinal identity.
fn mapped_bone(
    names: &BTreeMap<String, String>,
    source: &str,
) -> Result<String, PipelineError> {
    names.get(source).cloned().ok_or_else(|| {
        PipelineError::new(format!(
            "canonical prop bone mapping is missing: {source}"
        ))
    })
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/prop_catalog/canonical/tests.rs"]
mod tests;

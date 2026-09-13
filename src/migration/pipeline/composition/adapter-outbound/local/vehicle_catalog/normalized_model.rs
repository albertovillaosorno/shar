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
//   - Vehicle projection into engine-neutral normalized skeletal-model JSON.
// - Must-Not:
//   - Apply Unreal, Chaos, FBX-root, or importer coordinate compensation.
// - Allows:
//   - Validated assembled geometry, rig, skin, animation, and grounding
//     evidence.
// - Split-When:
//   - Split when another model family consumes the same normalized contract.
// - Merge-When:
//   - Merge into a shared normalized-model adapter once another family adopts
//     it.
// - Summary:
//   - Normalized vehicle skeletal-model publication.
// - Description:
//   - Publishes complete model evidence before deprecated FBX target rebasing.
// - Usage:
//   - Called by vehicle preparation before the FBX compatibility artifact.
// - Defaults:
//   - Source basis and metric scale remain explicit and unchanged.
//

//! Normalized vehicle skeletal-model publication.

use std::path::Path;

use fbx::domain::animation::AnimationClip;
use fbx::domain::character::CharacterAsset;
use serde_json::{Value, json};
use shar_sha256::digest_hex;

use super::catalog::write_new;
use super::model::{GroundingRecord, NormalizedModelArtifactRecord};
use crate::domain::PipelineError;

/// Versioned engine-neutral skeletal-model payload schema.
pub(super) const NORMALIZED_SKELETAL_MODEL_SCHEMA: &str =
    "shar.normalized-skeletal-model.v1";

/// Publish one complete normalized skeletal-model payload.
pub(super) fn publish_normalized_skeletal_model(
    vehicle_dir: &Path,
    asset: &CharacterAsset,
    animations: &[AnimationClip],
    grounding: &GroundingRecord,
) -> Result<NormalizedModelArtifactRecord, PipelineError> {
    let value = normalized_model_value(asset, animations, grounding);
    let mut bytes = serde_json::to_vec_pretty(&value)
        .map_err(|error| PipelineError::new(error.to_string()))?;
    bytes.push(b'\n');
    let file_name = "model.normalized.json";
    write_new(&vehicle_dir.join(file_name), &bytes)?;
    Ok(NormalizedModelArtifactRecord {
        path: file_name.to_owned(),
        bytes: u64::try_from(bytes.len()).map_err(|error| {
            PipelineError::new(format!(
                "normalized vehicle model size overflowed: {error}"
            ))
        })?,
        sha256: digest_hex(&bytes),
        parts: asset.parts.len(),
        bones: asset.bones.len(),
        animations: animations.len(),
    })
}

fn normalized_model_value(
    asset: &CharacterAsset,
    animations: &[AnimationClip],
    grounding: &GroundingRecord,
) -> Value {
    json!({
        "schema": NORMALIZED_SKELETAL_MODEL_SCHEMA,
        "model_id": asset.name,
        "coordinate_system": {
            "handedness": "right-handed",
            "right_axis": "+X",
            "up_axis": "+Y",
            "forward_axis": "+Z",
            "unit": "meter"
        },
        "normalization": {
            "grounding_source": grounding.source,
            "grounding_offset_y": grounding.offset_y,
            "grounding_root_bone": grounding.root_bone,
            "target_basis_applied": false
        },
        "bones": asset.bones.iter().map(bone_value).collect::<Vec<_>>(),
        "parts": asset.parts.iter().map(|part| {
            json!({
                "mesh_id": part.mesh.name,
                "source_identity": part.mesh.source_identity,
                "cast_shadow": part.mesh.cast_shadow,
                "primitive_groups": part.mesh.groups.iter()
                    .zip(&part.group_influences)
                    .map(|(group, influences)| json!({
                        "index": group.index,
                        "source_identity": group.source_identity,
                        "source_ordinal": group.source_ordinal,
                        "material_id": group.shader,
                        "positions": group.positions,
                        "normals": group.normals,
                        "colors": group.colors,
                        "uv0": group.uvs,
                        "triangles": group.triangles,
                        "skin_influences": influences.iter().map(|influence| {
                            json!({
                                "vertex_index": influence.vertex_index,
                                "bone_id": influence.bone_id,
                                "weight": influence.weight
                            })
                        }).collect::<Vec<_>>()
                    }))
                    .collect::<Vec<_>>()
            })
        }).collect::<Vec<_>>(),
        "animations": animations.iter().map(animation_value).collect::<Vec<_>>()
    })
}

fn bone_value(bone: &fbx::domain::skeleton::Bone) -> Value {
    json!({
        "bone_id": bone.id,
        "source_identity": bone.source_identity,
        "parent_bone_id": bone.parent_id,
        "rest_matrix_row_major": bone.rest_matrix,
        "source_rig": bone.source_rig.map(|rig| json!({
            "dof": rig.dof,
            "free_axes": rig.free_axes,
            "primary_axis": rig.primary_axis,
            "secondary_axis": rig.secondary_axis,
            "twist_axis": rig.twist_axis,
            "mirror_map": rig.mirror_map.map(|mirror| json!({
                "index": mirror.index,
                "scale": mirror.scale
            })),
            "fix_flags": rig.fix_flags
        }))
    })
}

fn animation_value(animation: &AnimationClip) -> Value {
    json!({
        "animation_id": animation.name,
        "source_identity": animation.source_identity,
        "frame_rate": animation.frame_rate,
        "cyclic": animation.cyclic,
        "frame_count": animation.frame_count,
        "ignored_group_ids": animation.ignored_group_ids,
        "tracks": animation.tracks.iter().map(|track| json!({
            "bone_id": track.bone_id,
            "samples": track.samples.iter().map(|sample| json!({
                "translation": sample.translation,
                "rotation_wxyz": sample.rotation_wxyz
            })).collect::<Vec<_>>()
        })).collect::<Vec<_>>()
    })
}

#[cfg(test)]
// jig-ignore-next-line: repository test module path is indivisible
#[path = "../../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/vehicle_catalog/normalized_model/tests.rs"]
mod tests;

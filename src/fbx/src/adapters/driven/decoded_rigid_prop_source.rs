// File:
//   - decoded_rigid_prop_source.rs
// Path:
//   - src/fbx/src/adapters/driven/decoded_rigid_prop_source.rs
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
//   - Assembly of one selected rigid-prop subset from a decoded composite.
// - Must-Not:
//   - Include unselected effects, particles, collision, placement, or state
//   - behavior in the resulting FBX character-compatible asset.
// - Allows:
//   - Reuse decoded skeleton, mesh, composite, and rigid influence adapters.
// - Split-When:
//   - Another selected-prop family needs a distinct selection policy.
// - Merge-When:
//   - The strict complete-character loader adopts an explicit subset contract.
// - Summary:
//   - Builds a pruned rigid animated asset from selected composite meshes.
// - Description:
//   - Resolves selected prop-to-joint bindings and retains only required bones.
// - Usage:
//   - Used by standalone animated-prop exporters such as the Wasp Camera lane.
// - Defaults:
//   - Unselected composite props and skeleton branches are excluded.
//
// ADRs:
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
// - docs/adr/fbx/export/fbx-output-contract-boundary.md
//
// Large file:
//   - false
//

//! Selected rigid-prop assembly from decoded composite evidence.
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use super::decoded_component_source::read_mesh;
use super::decoded_skin_source::{
    SkinSourceError, composite_prop_bindings, load_skeleton,
    rigid_group_influences,
};
use crate::domain::character::{CharacterAsset, SkinnedPart};
use crate::domain::skeleton::Bone;

/// Load selected rigid meshes and prune unrelated skeleton branches.
///
/// # Errors
///
/// Returns an error when the skeleton, composite, mesh, binding, or resulting
/// character-compatible asset violates its decoded contract.
pub fn load_selected_rigid_prop_asset(
    name: &str,
    skeleton_path: &Path,
    mesh_paths: &[&Path],
    composite_path: &Path,
) -> Result<CharacterAsset, SkinSourceError> {
    let (skeleton_name, bones) = load_skeleton(skeleton_path)?;
    let bindings = composite_prop_bindings(
        composite_path,
        &skeleton_name,
        &[],
        bones.len(),
    )?
    .into_iter()
    .collect::<BTreeMap<_, _>>();
    let (parts, selected_joints) = load_selected_parts(
        &bones, mesh_paths, &bindings,
    )?;
    let retained_indices = retained_bone_indices(
        &bones,
        &selected_joints,
    )?;
    let retained_bones = bones
        .into_iter()
        .enumerate()
        .filter_map(
            |(index, bone)| {
                retained_indices
                    .contains(&index)
                    .then_some(bone)
            },
        )
        .collect();
    CharacterAsset::new(
        name,
        retained_bones,
        parts,
    )
    .map_err(SkinSourceError::Character)
}

/// Load selected meshes and collect their bound skeleton joints.
fn load_selected_parts(
    bones: &[Bone],
    mesh_paths: &[&Path],
    bindings: &BTreeMap<String, usize>,
) -> Result<
    (
        Vec<SkinnedPart>,
        BTreeSet<usize>,
    ),
    SkinSourceError,
> {
    let mut selected_names = BTreeSet::new();
    let mut selected_joints = BTreeSet::new();
    let mut parts = Vec::with_capacity(mesh_paths.len());
    for mesh_path in mesh_paths {
        let requested_id = mesh_member_id(mesh_path)?;
        let mesh = read_mesh(
            mesh_path,
            requested_id,
        )
        .map_err(
            |error| {
                SkinSourceError::Prop(
                    format!(
                        "rigid prop mesh decode failed for {}: {error:?}",
                        mesh_path.display()
                    ),
                )
            },
        )?;
        if !selected_names.insert(
            mesh.name
                .clone(),
        ) {
            return Err(
                SkinSourceError::Prop(
                    format!(
                        "duplicate selected rigid prop: {}",
                        mesh.name
                    ),
                ),
            );
        }
        let joint = bindings
            .get(&mesh.name)
            .copied()
            .ok_or_else(
                || {
                    SkinSourceError::Prop(
                        format!(
                            "selected rigid prop {} has no composite binding",
                            mesh.name
                        ),
                    )
                },
            )?;
        let bone_id = bones
            .get(joint)
            .map(
                |bone| {
                    bone.id
                        .clone()
                },
            )
            .ok_or_else(
                || {
                    SkinSourceError::Prop(
                        format!(
                            "selected rigid prop {} references missing joint \
                             {joint}",
                            mesh.name
                        ),
                    )
                },
            )?;
        selected_joints.extend([joint]);
        let group_influences = rigid_group_influences(
            &mesh, &bone_id,
        )?;
        parts.push(
            SkinnedPart {
                mesh,
                group_influences,
            },
        );
    }
    Ok(
        (
            parts,
            selected_joints,
        ),
    )
}

/// Resolve one selected mesh identity from its exact component path.
fn mesh_member_id(path: &Path) -> Result<&str, SkinSourceError> {
    path.file_stem()
        .and_then(|value| value.to_str())
        .ok_or_else(
            || {
                SkinSourceError::Prop(
                    format!(
                        "rigid prop mesh path has no UTF-8 file stem: {}",
                        path.display()
                    ),
                )
            },
        )
}

/// Retain every selected joint and its complete ancestor chain.
fn retained_bone_indices(
    bones: &[Bone],
    selected_joints: &BTreeSet<usize>,
) -> Result<BTreeSet<usize>, SkinSourceError> {
    let by_name = bones
        .iter()
        .enumerate()
        .map(
            |(index, bone)| {
                (
                    bone.id
                        .as_str(),
                    index,
                )
            },
        )
        .collect::<BTreeMap<_, _>>();
    let mut pending = selected_joints
        .iter()
        .copied()
        .collect::<Vec<_>>();
    let mut retained = BTreeSet::new();
    while let Some(index) = pending.pop() {
        if !retained.insert(index) {
            continue;
        }
        let bone = bones
            .get(index)
            .ok_or_else(
                || {
                    SkinSourceError::Prop(
                        format!(
                            "selected rigid prop references missing joint \
                             {index}"
                        ),
                    )
                },
            )?;
        if let Some(parent_id) = bone
            .parent_id
            .as_deref()
        {
            let parent_index = by_name
                .get(parent_id)
                .copied()
                .ok_or_else(
                    || {
                        SkinSourceError::Prop(
                            format!(
                                "selected rigid prop ancestor is missing: \
                                 {parent_id}"
                            ),
                        )
                    },
                )?;
            pending.push(parent_index);
        }
    }
    Ok(retained)
}

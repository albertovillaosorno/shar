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

use super::decoded_billboard_source::read_billboard_quad_group;
use super::decoded_component_source::read_mesh;
use super::decoded_skin_source::{
    CompositePropBinding, SkinSourceError, composite_bindings, load_skeleton,
    mark_transparent_mesh, rigid_group_influences,
};
use crate::domain::character::{CharacterAsset, SkinnedPart};
use crate::domain::mesh::MeshAsset;
use crate::domain::skeleton::Bone;
use crate::domain::transform::matrix::{multiply, widen};

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
    let bindings = composite_bindings(
        composite_path,
        &skeleton_name,
        &[],
        bones.len(),
    )?
    .props
    .into_iter()
    .map(
        |binding| {
            (
                binding
                    .name
                    .clone(),
                binding,
            )
        },
    )
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

/// One additional source component attached by original runtime joint identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SupplementalRigidPropBinding {
    /// Decoded component identity.
    pub component_name: String,
    /// Exact skeleton joint identity.
    pub joint_id: String,
}

/// Load selected rigid meshes once for every authored composite binding.
///
/// # Errors
///
/// Returns an error when skeleton, composite, mesh, binding, or resulting
/// character-compatible asset evidence is invalid.
pub fn load_instanced_rigid_prop_asset(
    name: &str,
    skeleton_path: &Path,
    mesh_paths: &[&Path],
    composite_path: &Path,
) -> Result<CharacterAsset, SkinSourceError> {
    load_instanced_rigid_prop_asset_with_billboards(
        name,
        skeleton_path,
        mesh_paths,
        &[],
        composite_path,
        &[],
    )
}

/// Load rigid meshes and billboard groups under composite and supplemental
/// original-runtime bindings.
///
/// # Errors
///
/// Returns an error when any source component, joint, transform, or resulting
/// character-compatible asset evidence is invalid.
pub fn load_instanced_rigid_prop_asset_with_billboards(
    name: &str,
    skeleton_path: &Path,
    mesh_paths: &[&Path],
    billboard_paths: &[&Path],
    composite_path: &Path,
    supplemental: &[SupplementalRigidPropBinding],
) -> Result<CharacterAsset, SkinSourceError> {
    let (skeleton_name, bones) = load_skeleton(skeleton_path)?;
    let mut bindings = composite_bindings(
        composite_path,
        &skeleton_name,
        &[],
        bones.len(),
    )?
    .props;
    let bone_indices = bones
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
    for binding in supplemental {
        if let Some(joint) = bone_indices
            .get(
                binding
                    .joint_id
                    .as_str(),
            )
            .copied()
        {
            bindings.push(
                CompositePropBinding {
                    name: binding
                        .component_name
                        .clone(),
                    joint,
                    translucent: false,
                },
            );
        }
    }
    let (meshes, required_meshes) = load_rigid_component_map(
        mesh_paths,
        billboard_paths,
    )?;
    let global_rest = global_rest_matrices(&bones)?;
    let mut occurrences = BTreeMap::<String, usize>::new();
    let mut bound_names = BTreeSet::new();
    let mut selected_joints = BTreeSet::new();
    let mut parts = Vec::new();
    for binding in bindings {
        let mesh_name = binding.name;
        let joint = binding.joint;
        let Some(source_mesh) = meshes.get(&mesh_name) else {
            continue;
        };
        let ordinal = occurrences
            .entry(mesh_name.clone())
            .or_insert(0);
        let mut mesh = source_mesh.clone();
        if binding.translucent {
            mark_transparent_mesh(&mut mesh);
        }
        let joint_matrix = global_rest
            .get(joint)
            .ok_or_else(
                || {
                    SkinSourceError::Prop(
                        format!(
                            "instanced rigid prop {mesh_name} has no global \
                             rest matrix for joint {joint}"
                        ),
                    )
                },
            )?;
        bake_rigid_mesh(
            &mut mesh,
            joint_matrix,
        )?;
        mesh.name = format!(
            "{}__joint_{joint:02}__instance_{:02}",
            mesh.name, *ordinal,
        );
        *ordinal = ordinal.saturating_add(1);
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
                            "instanced rigid prop {mesh_name} references \
                             missing joint {joint}"
                        ),
                    )
                },
            )?;
        selected_joints.insert(joint);
        bound_names.insert(mesh_name);
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
    let unbound = required_meshes
        .iter()
        .filter(|mesh_name| !bound_names.contains(*mesh_name))
        .cloned()
        .collect::<Vec<_>>();
    if !unbound.is_empty() {
        return Err(
            SkinSourceError::Prop(
                format!(
                    "selected instanced rigid props have no binding: {}",
                    unbound.join(", ")
                ),
            ),
        );
    }
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

/// Load one unique source map across mesh and billboard component families.
fn load_rigid_component_map(
    mesh_paths: &[&Path],
    billboard_paths: &[&Path],
) -> Result<
    (
        BTreeMap<String, MeshAsset>,
        BTreeSet<String>,
    ),
    SkinSourceError,
> {
    let mut meshes = load_rigid_mesh_map(mesh_paths)?;
    let required_meshes = meshes
        .keys()
        .cloned()
        .collect::<BTreeSet<_>>();
    for path in billboard_paths {
        let requested_id = mesh_member_id(path)?.trim_end_matches('_');
        let mesh = read_billboard_quad_group(
            path,
            requested_id,
        )
        .map_err(
            |error| {
                SkinSourceError::Prop(
                    format!(
                        "instanced billboard prop decode failed for {}: \
                         {error:?}",
                        path.display()
                    ),
                )
            },
        )?;
        let mesh_name = mesh
            .name
            .clone();
        if meshes
            .insert(
                mesh_name.clone(),
                mesh,
            )
            .is_some()
        {
            return Err(
                SkinSourceError::Prop(
                    format!("duplicate instanced rigid component: {mesh_name}"),
                ),
            );
        }
    }
    Ok(
        (
            meshes,
            required_meshes,
        ),
    )
}

/// Load one unique source mesh map for authored composite instancing.
fn load_rigid_mesh_map(
    mesh_paths: &[&Path]
) -> Result<BTreeMap<String, crate::domain::mesh::MeshAsset>, SkinSourceError> {
    let mut meshes = BTreeMap::new();
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
                        "instanced rigid prop mesh decode failed for {}: \
                         {error:?}",
                        mesh_path.display()
                    ),
                )
            },
        )?;
        let mesh_name = mesh
            .name
            .clone();
        if meshes
            .insert(
                mesh_name.clone(),
                mesh,
            )
            .is_some()
        {
            return Err(
                SkinSourceError::Prop(
                    format!(
                        "duplicate instanced rigid prop source mesh: \
                         {mesh_name}"
                    ),
                ),
            );
        }
    }
    Ok(meshes)
}

/// Load selected meshes and collect their bound skeleton joints.
fn load_selected_parts(
    bones: &[Bone],
    mesh_paths: &[&Path],
    bindings: &BTreeMap<String, CompositePropBinding>,
) -> Result<
    (
        Vec<SkinnedPart>,
        BTreeSet<usize>,
    ),
    SkinSourceError,
> {
    let global_rest = global_rest_matrices(bones)?;
    let mut selected_names = BTreeSet::new();
    let mut selected_joints = BTreeSet::new();
    let mut parts = Vec::with_capacity(mesh_paths.len());
    for mesh_path in mesh_paths {
        let requested_id = mesh_member_id(mesh_path)?;
        let mut mesh = read_mesh(
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
        let binding = bindings
            .get(&mesh.name)
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
        if binding.translucent {
            mark_transparent_mesh(&mut mesh);
        }
        let joint = binding.joint;
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
        let joint_matrix = global_rest
            .get(joint)
            .ok_or_else(
                || {
                    SkinSourceError::Prop(
                        format!(
                            "selected rigid prop {} has no global rest matrix \
                             for joint {joint}",
                            mesh.name
                        ),
                    )
                },
            )?;
        bake_rigid_mesh(
            &mut mesh,
            joint_matrix,
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

/// Compute every global rest matrix under the decoded row-vector convention.
fn global_rest_matrices(
    bones: &[Bone]
) -> Result<Vec<[f64; 16]>, SkinSourceError> {
    let ordinals = bones
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
    let mut global = Vec::with_capacity(bones.len());
    for bone in bones {
        let local = widen(&bone.rest_matrix);
        let matrix = match bone
            .parent_id
            .as_deref()
        {
            Some(parent) if parent != bone.id => {
                let parent_index = ordinals
                    .get(parent)
                    .copied()
                    .ok_or_else(
                        || {
                            SkinSourceError::Prop(
                                format!(
                                    "rigid prop rest hierarchy is missing \
                                     parent {parent}"
                                ),
                            )
                        },
                    )?;
                let parent_matrix = global
                    .get(parent_index)
                    .ok_or_else(
                        || {
                            SkinSourceError::Prop(
                                format!(
                                    "rigid prop parent {parent} does not \
                                     precede child {}",
                                    bone.id
                                ),
                            )
                        },
                    )?;
                multiply(
                    &local,
                    parent_matrix,
                )
            }
            Some(_) | None => local,
        };
        global.push(matrix);
    }
    Ok(global)
}

/// Bake one rigid mesh through the global rest transform of its authored joint.
fn bake_rigid_mesh(
    mesh: &mut MeshAsset,
    matrix: &[f64; 16],
) -> Result<(), SkinSourceError> {
    for group in &mut mesh.groups {
        for position in &mut group.positions {
            *position = transform_position(
                *position, matrix, &mesh.name,
            )?;
        }
        for normal in &mut group.normals {
            *normal = transform_normal(
                *normal, matrix, &mesh.name,
            )?;
        }
    }
    Ok(())
}

/// Transform one row-vector position and narrow finite output to source units.
fn transform_position(
    value: [f32; 3],
    matrix: &[f64; 16],
    mesh: &str,
) -> Result<[f32; 3], SkinSourceError> {
    let [
        x,
        y,
        z,
    ] = value.map(f64::from);
    narrow_vector(
        [
            x * matrix[0] + y * matrix[4] + z * matrix[8] + matrix[12],
            x * matrix[1] + y * matrix[5] + z * matrix[9] + matrix[13],
            x * matrix[2] + y * matrix[6] + z * matrix[10] + matrix[14],
        ],
        mesh,
        "position",
    )
}

/// Transform and normalize one direction without applying translation.
fn transform_normal(
    value: [f32; 3],
    matrix: &[f64; 16],
    mesh: &str,
) -> Result<[f32; 3], SkinSourceError> {
    let [
        x,
        y,
        z,
    ] = value.map(f64::from);
    let source_length = (x * x + y * y + z * z).sqrt();
    if source_length <= f64::EPSILON {
        return Ok(value);
    }
    let mut transformed = [
        x * matrix[0] + y * matrix[4] + z * matrix[8],
        x * matrix[1] + y * matrix[5] + z * matrix[9],
        x * matrix[2] + y * matrix[6] + z * matrix[10],
    ];
    let length = transformed
        .iter()
        .map(|component| component * component)
        .sum::<f64>()
        .sqrt();
    if !length.is_finite() || length <= f64::EPSILON {
        return Err(
            SkinSourceError::Prop(
                format!(
                    "rigid prop {mesh} has a degenerate transformed normal"
                ),
            ),
        );
    }
    for component in &mut transformed {
        *component /= length;
    }
    narrow_vector(
        transformed,
        mesh,
        "normal",
    )
}

/// Narrow one finite transformed vector to the FBX mesh scalar contract.
fn narrow_vector(
    value: [f64; 3],
    mesh: &str,
    role: &str,
) -> Result<[f32; 3], SkinSourceError> {
    if value
        .iter()
        .any(|component| !component.is_finite())
    {
        return Err(
            SkinSourceError::Prop(
                format!(
                    "rigid prop {mesh} produced a non-finite transformed \
                     {role}"
                ),
            ),
        );
    }
    let narrowed = value.map(|component| component as f32);
    if narrowed
        .iter()
        .any(|component| !component.is_finite())
    {
        return Err(
            SkinSourceError::Prop(
                format!(
                    "rigid prop {mesh} transformed {role} exceeds f32 range"
                ),
            ),
        );
    }
    Ok(narrowed)
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

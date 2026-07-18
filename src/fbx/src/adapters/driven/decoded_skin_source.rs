// File:
//   - decoded_skin_source.rs
// Path:
//   - src/fbx/src/adapters/driven/decoded_skin_source.rs
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
//   - The fbx adapter boundary for adapters driven decoded skin source.
// - Must-Not:
//   - Change domain semantics or bypass application and port contracts.
// - Allows:
//   - Filesystem, JSON, CLI, Blender, or serialization work behind explicit
//   - ports.
// - Split-When:
//   - Split when decoded skin source contains two independently testable
//   - contracts.
// - Merge-When:
//   - Another fbx module owns the same adapters boundary with no distinct
//   - invariant.
// - Summary:
//   - Decoded skeleton, skin, and composite loading for character export.
// - Description:
//   - Defines decoded skin source data and behavior for fbx adapters driven.
// - Usage:
//   - Constructed by composition roots or tests and passed behind port traits.
// - Defaults:
//   - Adapter defaults stay local to the adapter constructor or config.
//
// ADRs:
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
//
// Large file:
//   - true
//   - Reason: src/fbx/src/adapters/driven/decoded_skin_source.rs keeps the
//   - decoded skeleton, skin, and composite evidence loaders together because
//   - they share one strict JSON contract and one failure taxonomy; split when
//   - a family gains an independently testable contract.
//

//! Decoded skeleton, skin, and composite loading for character export.
//!
//! The loaders convert extracted component JSON into validated domain values.
//! Every count, palette slot, and identity is checked before the character
//! aggregate is built, so malformed extraction evidence fails closed instead
//! of producing a silently wrong FBX document.
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use schoenwald_filesystem::adapters::driving::local;
use serde::Deserialize;

use super::decoded_component_source::read_mesh;
use crate::domain::character::{CharacterAsset, CharacterError, SkinnedPart};
use crate::domain::mesh::{
    MeshAsset, MeshError, PrimitiveGroup, triangulate_strip,
};
use crate::domain::skeleton::Bone;
use crate::domain::skin::SkinInfluence;

/// Weight tolerance shared with the domain aggregate contract.
const WEIGHT_SUM_TOLERANCE: f32 = 1e-3;

/// Load one decoded character from explicit component file locations.
///
/// The caller supplies every component path from package-index evidence; this
/// adapter never discovers paths on its own.
///
/// # Errors
///
/// Returns an error when one component cannot be read, parsed, or validated
/// against the character contract.
#[expect(
    clippy::too_many_lines,
    reason = "Character assembly validates one complete skeleton, skin, prop, \
              and composite transaction."
)]
pub fn load_character(
    name: &str,
    skeleton_path: &Path,
    skin_paths: &[&Path],
    mesh_paths: &[&Path],
    composite_paths: &[&Path],
) -> Result<CharacterAsset, SkinSourceError> {
    let (skeleton_name, bones) = load_skeleton(skeleton_path)?;
    let mut parts = Vec::with_capacity(
        skin_paths
            .len()
            .saturating_add(mesh_paths.len()),
    );
    let mut part_names = Vec::with_capacity(skin_paths.len());
    for skin_path in skin_paths {
        let (part, decoded_skeleton_reference) = load_skin_part(
            skin_path, &bones,
        )?;
        if decoded_skeleton_reference != skeleton_name {
            return Err(
                SkinSourceError::SkeletonReferenceMismatch {
                    skin: part
                        .mesh
                        .name,
                    expected: skeleton_name,
                    found: decoded_skeleton_reference,
                },
            );
        }
        part_names.push(
            part.mesh
                .name
                .clone(),
        );
        parts.push(part);
    }
    let mut prop_bindings = BTreeMap::new();
    let mut translucent_skins = BTreeSet::new();
    for composite_path in composite_paths {
        let bindings = composite_bindings(
            composite_path,
            &skeleton_name,
            &part_names,
            bones.len(),
        )?;
        translucent_skins.extend(bindings.translucent_skins);
        for binding in bindings.props {
            let prop_name = binding
                .name
                .clone();
            if prop_bindings
                .insert(
                    prop_name.clone(),
                    binding,
                )
                .is_some()
            {
                return Err(
                    SkinSourceError::Prop(
                        format!(
                            "duplicate composite prop binding: {prop_name}"
                        ),
                    ),
                );
            }
        }
    }
    for part in &mut parts {
        if translucent_skins.contains(
            &part
                .mesh
                .name,
        ) {
            mark_transparent_mesh(&mut part.mesh);
        }
    }
    for mesh_path in mesh_paths {
        let requested_id = mesh_path
            .file_stem()
            .and_then(|value| value.to_str())
            .ok_or_else(
                || {
                    SkinSourceError::Prop(
                        format!(
                            "prop mesh path has no UTF-8 file stem: {}",
                            mesh_path.display()
                        ),
                    )
                },
            )?;
        let mut mesh = read_mesh(
            mesh_path,
            requested_id,
        )
        .map_err(
            |error| {
                SkinSourceError::Prop(
                    format!(
                        "prop mesh decode failed for {}: {error:?}",
                        mesh_path.display()
                    ),
                )
            },
        )?;
        let binding = prop_bindings
            .remove(&mesh.name)
            .ok_or_else(
                || {
                    SkinSourceError::Prop(
                        format!(
                            "prop mesh {} has no composite binding",
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
                            "prop mesh {} references missing joint {joint}",
                            mesh.name
                        ),
                    )
                },
            )?;
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
    if let Some((prop_name, _joint)) = prop_bindings.first_key_value() {
        return Err(
            SkinSourceError::Prop(
                format!("composite prop has no decoded mesh: {prop_name}"),
            ),
        );
    }
    CharacterAsset::new(
        name, bones, parts,
    )
    .map_err(SkinSourceError::Character)
}

/// Load one decoded skeleton into its name and ordered validated bones.
///
/// # Errors
///
/// Returns an error when the skeleton file cannot be read, parsed, or its
/// joint hierarchy violates the parent-before-child contract.
pub fn load_skeleton(
    path: &Path
) -> Result<
    (
        String,
        Vec<Bone>,
    ),
    SkinSourceError,
> {
    let decoded: DecodedSkeleton = read_json(path)?;
    if decoded.schema != "skeleton" {
        return Err(
            SkinSourceError::UnsupportedSchema {
                path: path_text(path),
                schema: decoded.schema,
            },
        );
    }
    if decoded.version != 0 {
        return Err(
            SkinSourceError::UnsupportedSkeletonVersion {
                path: path_text(path),
                version: decoded.version,
            },
        );
    }
    let actual_joint_count = decoded
        .joints
        .len();
    if u32::try_from(actual_joint_count) != Ok(decoded.joint_count) {
        return Err(
            SkinSourceError::JointCountMismatch {
                path: path_text(path),
                declared: decoded.joint_count,
                actual: actual_joint_count,
            },
        );
    }
    let skeleton_name = trim_decoded_identity(&decoded.name);
    if skeleton_name.is_empty() {
        return Err(
            SkinSourceError::BlankComponentName {
                path: path_text(path),
            },
        );
    }
    if decoded
        .joints
        .is_empty()
    {
        return Err(
            SkinSourceError::EmptySkeleton {
                path: path_text(path),
            },
        );
    }
    let mut bones = Vec::with_capacity(
        decoded
            .joints
            .len(),
    );
    let mut names = Vec::with_capacity(
        decoded
            .joints
            .len(),
    );
    for (index, joint) in decoded
        .joints
        .iter()
        .enumerate()
    {
        let joint_name = trim_decoded_identity(&joint.name);
        if joint_name.is_empty() {
            return Err(
                SkinSourceError::BlankJointName {
                    path: path_text(path),
                    joint: index,
                },
            );
        }
        let parent_id = joint_parent_id(
            path, index, joint, &names,
        )?;
        names.push(joint_name.clone());
        bones.push(
            Bone {
                id: joint_name,
                parent_id,
                rest_matrix: joint.rest_pose,
            },
        );
    }
    Ok(
        (
            skeleton_name,
            bones,
        ),
    )
}

/// Resolve one joint parent identity from previously loaded joint names.
fn joint_parent_id(
    path: &Path,
    index: usize,
    joint: &DecodedJoint,
    names: &[String],
) -> Result<Option<String>, SkinSourceError> {
    if index == 0 {
        if joint.parent != 0 {
            return Err(
                SkinSourceError::InvalidJointParent {
                    path: path_text(path),
                    joint: index,
                    parent: joint.parent,
                },
            );
        }
        return Ok(None);
    }
    if joint.parent == index {
        return Ok(None);
    }
    names
        .get(joint.parent)
        .map_or_else(
            || {
                Err(
                    SkinSourceError::InvalidJointParent {
                        path: path_text(path),
                        joint: index,
                        parent: joint.parent,
                    },
                )
            },
            |parent_name| Ok(Some(parent_name.clone())),
        )
}

/// Load one decoded skin file into a validated skinned part.
///
/// # Errors
///
/// Returns an error when geometry arrays, palette slots, or weight streams
/// violate the decoded skin contract.
pub fn load_skin_part(
    path: &Path,
    bones: &[Bone],
) -> Result<
    (
        SkinnedPart,
        String,
    ),
    SkinSourceError,
> {
    let decoded: DecodedSkin = read_json(path)?;
    if decoded.schema != "skin" {
        return Err(
            SkinSourceError::UnsupportedSchema {
                path: path_text(path),
                schema: decoded.schema,
            },
        );
    }
    if !matches!(
        decoded.version,
        3 | 4
    ) {
        return Err(
            SkinSourceError::UnsupportedSkinVersion {
                path: path_text(path),
                version: decoded.version,
            },
        );
    }
    let actual_group_count = decoded
        .prim_groups
        .len();
    if u32::try_from(actual_group_count) != Ok(decoded.primitive_group_count) {
        return Err(
            SkinSourceError::PrimitiveGroupCountMismatch {
                path: path_text(path),
                declared: decoded.primitive_group_count,
                actual: actual_group_count,
            },
        );
    }
    let skin_name = trim_decoded_identity(&decoded.name);
    if skin_name.is_empty() {
        return Err(
            SkinSourceError::BlankComponentName {
                path: path_text(path),
            },
        );
    }
    let mut groups = Vec::with_capacity(
        decoded
            .prim_groups
            .len(),
    );
    let mut group_influences = Vec::with_capacity(
        decoded
            .prim_groups
            .len(),
    );
    for (index, decoded_group) in decoded
        .prim_groups
        .iter()
        .enumerate()
    {
        let (group, influences) = load_group(
            path,
            index,
            decoded_group,
            bones,
        )?;
        groups.push(group);
        group_influences.push(influences);
    }
    let mesh = MeshAsset::new(
        skin_name, groups,
    )
    .map_err(
        |error| SkinSourceError::Mesh {
            path: path_text(path),
            error,
        },
    )?;
    Ok(
        (
            SkinnedPart {
                mesh,
                group_influences,
            },
            trim_decoded_identity(&decoded.skeleton_name),
        ),
    )
}

/// Load one decoded primitive group with its influences.
fn load_group(
    path: &Path,
    index: usize,
    decoded: &DecodedSkinGroup,
    bones: &[Bone],
) -> Result<
    (
        PrimitiveGroup,
        Vec<SkinInfluence>,
    ),
    SkinSourceError,
> {
    let actual_vertex_count = decoded
        .positions
        .len();
    if u32::try_from(actual_vertex_count) != Ok(decoded.vertex_count) {
        return Err(
            SkinSourceError::VertexCountMismatch {
                path: path_text(path),
                group: index,
                declared: decoded.vertex_count,
                actual: actual_vertex_count,
            },
        );
    }
    let actual_index_count = decoded
        .indices
        .len();
    if u32::try_from(actual_index_count) != Ok(decoded.index_count) {
        return Err(
            SkinSourceError::IndexCountMismatch {
                path: path_text(path),
                group: index,
                declared: decoded.index_count,
                actual: actual_index_count,
            },
        );
    }
    let actual_matrix_count = decoded
        .matrix_palette
        .len();
    if u32::try_from(actual_matrix_count) != Ok(decoded.matrix_count) {
        return Err(
            SkinSourceError::MatrixPaletteCountMismatch {
                path: path_text(path),
                group: index,
                declared: decoded.matrix_count,
                actual: actual_matrix_count,
            },
        );
    }
    let uvs = channel_zero_uvs(
        path, index, decoded,
    )?;
    let triangles = match decoded.prim_type {
        0 => decoded
            .indices
            .clone(),
        1 => triangulate_strip(&decoded.indices)
            .map_err(
                |error| SkinSourceError::Mesh {
                    path: path_text(path),
                    error,
                },
            )?
            .into_iter()
            .flatten()
            .collect(),
        other => {
            return Err(
                SkinSourceError::UnsupportedPrimType {
                    path: path_text(path),
                    group: index,
                    prim_type: other,
                },
            );
        }
    };
    let group = PrimitiveGroup::new(
        index,
        trim_decoded_identity(&decoded.shader),
        decoded
            .positions
            .clone(),
        uvs,
        &triangles,
    )
    .and_then(
        |group| {
            group.with_normals(
                decoded
                    .normals
                    .clone(),
            )
        },
    )
    .map_err(
        |error| SkinSourceError::Mesh {
            path: path_text(path),
            error,
        },
    )?;
    let influences = group_influences(
        path, index, decoded, bones,
    )?;
    Ok(
        (
            group, influences,
        ),
    )
}

/// Extract the single supported UV channel from one decoded group.
fn channel_zero_uvs(
    path: &Path,
    index: usize,
    decoded: &DecodedSkinGroup,
) -> Result<Vec<[f32; 2]>, SkinSourceError> {
    let mut channel_zero = None;
    for channel in &decoded.uvs {
        if channel.channel != 0 {
            return Err(
                SkinSourceError::UnsupportedUvChannel {
                    path: path_text(path),
                    group: index,
                    channel: channel.channel,
                },
            );
        }
        if channel_zero
            .replace(
                channel
                    .coords
                    .clone(),
            )
            .is_some()
        {
            return Err(
                SkinSourceError::DuplicateUvChannel {
                    path: path_text(path),
                    group: index,
                    channel: 0,
                },
            );
        }
    }
    Ok(channel_zero.unwrap_or_default())
}

/// Convert decoded palette bindings into explicit bone influences.
// Palette validation, reversed slot pairing, merging, and normalization form
// one ordered influence-conversion transaction with shared failure context.
#[expect(
    clippy::too_many_lines,
    reason = "One influence transaction preserves palette and weight \
              validation ordering."
)]
fn group_influences(
    path: &Path,
    index: usize,
    decoded: &DecodedSkinGroup,
    bones: &[Bone],
) -> Result<Vec<SkinInfluence>, SkinSourceError> {
    let palette = resolve_palette(
        path, index, decoded, bones,
    )?;
    if decoded
        .matrices
        .len()
        != decoded
            .positions
            .len()
    {
        return Err(
            SkinSourceError::MatrixCountMismatch {
                path: path_text(path),
                group: index,
                positions: decoded
                    .positions
                    .len(),
                matrices: decoded
                    .matrices
                    .len(),
            },
        );
    }
    if let Some(weights) = &decoded.weights
        && weights.len()
            != decoded
                .positions
                .len()
    {
        return Err(
            SkinSourceError::WeightCountMismatch {
                path: path_text(path),
                group: index,
                positions: decoded
                    .positions
                    .len(),
                weights: weights.len(),
            },
        );
    }
    let mut merged: BTreeMap<
        (
            u32,
            String,
        ),
        f32,
    > = BTreeMap::new();
    for (vertex, slots) in decoded
        .matrices
        .iter()
        .enumerate()
    {
        let vertex_index = match u32::try_from(vertex) {
            Ok(value) => value,
            Err(_conversion_error) => {
                return Err(
                    SkinSourceError::VertexIndexOverflow {
                        path: path_text(path),
                        group: index,
                        vertex,
                    },
                );
            }
        };
        let vertex_weights = vertex_weights(
            path,
            index,
            vertex,
            decoded
                .weights
                .as_deref(),
        )?;
        // Decoded slot tuples store the primary joint last: stored weight
        // `j` pairs with slot `3 - j`, and the complement weight pairs with
        // slot zero. Rigid vertices therefore carry their joint in the final
        // slot with an implicit weight of one.
        for (slot, weight) in slots
            .iter()
            .rev()
            .zip(vertex_weights)
        {
            if weight <= 0.0 {
                continue;
            }
            let Some(bone_id) = palette.get(usize::from(*slot)) else {
                return Err(
                    SkinSourceError::PaletteSlotOutOfRange {
                        path: path_text(path),
                        group: index,
                        vertex,
                        slot: *slot,
                        palette: palette.len(),
                    },
                );
            };
            let entry = merged
                .entry(
                    (
                        vertex_index,
                        (*bone_id).clone(),
                    ),
                )
                .or_insert(0.0);
            *entry += weight;
        }
    }
    Ok(
        merged
            .into_iter()
            .map(
                |((vertex_index, bone_id), weight)| SkinInfluence {
                    vertex_index,
                    bone_id,
                    weight,
                },
            )
            .collect(),
    )
}

/// Resolve palette slots into bone identities.
fn resolve_palette<'bones>(
    path: &Path,
    index: usize,
    decoded: &DecodedSkinGroup,
    bones: &'bones [Bone],
) -> Result<Vec<&'bones String>, SkinSourceError> {
    if decoded
        .matrix_palette
        .is_empty()
    {
        return Err(
            SkinSourceError::EmptyMatrixPalette {
                path: path_text(path),
                group: index,
            },
        );
    }
    decoded
        .matrix_palette
        .iter()
        .map(
            |joint| {
                usize::try_from(*joint)
                    .ok()
                    .and_then(|joint_index| bones.get(joint_index))
                    .map(|bone| &bone.id)
                    .ok_or_else(
                        || SkinSourceError::PaletteJointOutOfRange {
                            path: path_text(path),
                            group: index,
                            joint: *joint,
                            joints: bones.len(),
                        },
                    )
            },
        )
        .collect()
}

/// Expand one vertex weight record into four explicit weights.
fn vertex_weights(
    path: &Path,
    index: usize,
    vertex: usize,
    weights: Option<&[[f32; 3]]>,
) -> Result<[f32; 4], SkinSourceError> {
    let Some(stored_weights) = weights else {
        return Ok(
            [
                1.0, 0.0, 0.0, 0.0,
            ],
        );
    };
    let Some(stored) = stored_weights.get(vertex) else {
        return Err(
            SkinSourceError::WeightCountMismatch {
                path: path_text(path),
                group: index,
                positions: vertex,
                weights: stored_weights.len(),
            },
        );
    };
    let mut sum = 0.0_f32;
    for weight in stored {
        if !weight.is_finite() || *weight < 0.0 || *weight > 1.0 {
            return Err(
                SkinSourceError::InvalidStoredWeight {
                    path: path_text(path),
                    group: index,
                    vertex,
                },
            );
        }
        sum += *weight;
    }
    if sum > 1.0 + WEIGHT_SUM_TOLERANCE {
        return Err(
            SkinSourceError::InvalidStoredWeight {
                path: path_text(path),
                group: index,
                vertex,
            },
        );
    }
    let complement = (1.0 - sum).max(0.0);
    Ok(
        [
            stored[0], stored[1], stored[2], complement,
        ],
    )
}

/// Build one normalized full-weight influence for every rigid prop vertex.
pub(super) fn rigid_group_influences(
    mesh: &MeshAsset,
    bone_id: &str,
) -> Result<Vec<Vec<SkinInfluence>>, SkinSourceError> {
    mesh.groups
        .iter()
        .map(
            |group| {
                group
                    .positions
                    .iter()
                    .enumerate()
                    .map(
                        |(vertex, _position)| {
                            let vertex_index = u32::try_from(vertex).map_err(
                                |_error| {
                                    SkinSourceError::Prop(
                                        format!(
                                            "prop mesh {} exceeds the FBX \
                                             vertex index range",
                                            mesh.name
                                        ),
                                    )
                                },
                            )?;
                            Ok(
                                SkinInfluence {
                                    vertex_index,
                                    bone_id: bone_id.to_owned(),
                                    weight: 1.0,
                                },
                            )
                        },
                    )
                    .collect::<Result<Vec<_>, SkinSourceError>>()
            },
        )
        .collect()
}

/// One validated rigid prop binding from a decoded composite.
pub(super) struct CompositePropBinding {
    /// Referenced component identity.
    pub(super) name: String,
    /// Zero-based skeleton joint position.
    pub(super) joint: usize,
    /// Source composite marks the prop as translucent.
    pub(super) translucent: bool,
}

/// Validated skin and rigid-prop semantics from one composite.
pub(super) struct CompositeBindings {
    /// Rigid prop bindings.
    pub(super) props: Vec<CompositePropBinding>,
    /// Skin identities explicitly marked translucent.
    pub(super) translucent_skins: BTreeSet<String>,
}

/// Read one strict legacy zero-or-one flag.
fn binary_flag(
    value: &serde_json::Value,
    label: &str,
) -> Result<bool, SkinSourceError> {
    match value.as_u64() {
        Some(0) => Ok(false),
        Some(1) => Ok(true),
        _ => Err(
            SkinSourceError::Prop(
                format!("{label} is not an exact zero-or-one integer"),
            ),
        ),
    }
}

/// Mark geometry with shared semantic evidence without changing shader lookup.
pub(super) fn mark_transparent_mesh(mesh: &mut MeshAsset) {
    if !mesh
        .name
        .to_ascii_lowercase()
        .contains("transparent")
    {
        mesh.name
            .push_str("__transparent-source");
    }
}

/// Validate one composite and return its rigid prop-to-joint bindings.
#[expect(
    clippy::too_many_lines,
    reason = "Composite count, skeleton, skin, prop, and joint checks form \
              one atomic validation."
)]
pub(super) fn composite_bindings(
    path: &Path,
    skeleton_name: &str,
    part_names: &[String],
    bone_count: usize,
) -> Result<CompositeBindings, SkinSourceError> {
    let decoded: DecodedComposite = read_json(path)?;
    if decoded.schema != "composite_drawable" {
        return Err(
            SkinSourceError::UnsupportedSchema {
                path: path_text(path),
                schema: decoded.schema,
            },
        );
    }
    let actual_skin_count = decoded
        .skins
        .len();
    if u32::try_from(actual_skin_count) != Ok(decoded.skin_count) {
        return Err(
            SkinSourceError::CompositeSkinCountMismatch {
                path: path_text(path),
                declared: decoded.skin_count,
                actual: actual_skin_count,
            },
        );
    }
    let actual_prop_count = decoded
        .props
        .len();
    if u32::try_from(actual_prop_count) != Ok(decoded.prop_count) {
        return Err(
            SkinSourceError::CompositePropCountMismatch {
                path: path_text(path),
                declared: decoded.prop_count,
                actual: actual_prop_count,
            },
        );
    }
    let actual_effect_count = decoded
        .effects
        .len();
    if u32::try_from(actual_effect_count) != Ok(decoded.effect_count) {
        return Err(
            SkinSourceError::CompositeEffectCountMismatch {
                path: path_text(path),
                declared: decoded.effect_count,
                actual: actual_effect_count,
            },
        );
    }
    let referenced_skeleton = trim_decoded_identity(&decoded.skeleton_name);
    if referenced_skeleton != skeleton_name {
        return Err(
            SkinSourceError::SkeletonReferenceMismatch {
                skin: trim_decoded_identity(&decoded.name),
                expected: skeleton_name.to_owned(),
                found: referenced_skeleton,
            },
        );
    }
    let mut translucent_skins = BTreeSet::new();
    for skin in &decoded.skins {
        let skin_name = trim_decoded_identity(&skin.name);
        if !part_names.contains(&skin_name) {
            return Err(
                SkinSourceError::CompositeSkinMissing {
                    path: path_text(path),
                    skin: skin_name,
                },
            );
        }
        if binary_flag(
            &skin.is_translucent,
            "composite skin translucency",
        )? {
            translucent_skins.insert(skin_name);
        }
    }
    let mut bindings = Vec::with_capacity(
        decoded
            .props
            .len(),
    );
    for prop in decoded.props {
        if prop.kind != "prop" {
            return Err(
                SkinSourceError::Prop(
                    format!(
                        "composite {} contains unsupported prop kind {}",
                        path.display(),
                        prop.kind
                    ),
                ),
            );
        }
        let prop_name = trim_decoded_identity(&prop.name);
        if prop_name.is_empty() {
            return Err(
                SkinSourceError::Prop(
                    format!(
                        "composite {} contains a blank prop name",
                        path.display()
                    ),
                ),
            );
        }
        if prop.skeleton_joint_id >= bone_count {
            return Err(
                SkinSourceError::Prop(
                    format!(
                        "composite prop {prop_name} references joint {} \
                         outside {} bones",
                        prop.skeleton_joint_id, bone_count
                    ),
                ),
            );
        }
        bindings.push(
            CompositePropBinding {
                name: prop_name,
                joint: prop.skeleton_joint_id,
                translucent: binary_flag(
                    &prop.is_translucent,
                    "composite prop translucency",
                )?,
            },
        );
    }
    Ok(
        CompositeBindings {
            props: bindings,
            translucent_skins,
        },
    )
}

/// Trim decoded fixed-width identity padding without touching inner text.
fn trim_decoded_identity(value: &str) -> String {
    value
        .trim_end_matches('\u{0}')
        .trim()
        .to_owned()
}

/// Internal helper for the adapter implementation.
fn read_json<T>(path: &Path) -> Result<T, SkinSourceError>
where
    T: for<'de> Deserialize<'de>,
{
    let text = local::read_utf8(path).map_err(
        |source| SkinSourceError::Read {
            path: path_text(path),
            source: source.to_string(),
        },
    )?;
    let json_text = text
        .strip_prefix('\u{feff}')
        .unwrap_or(&text);
    serde_json::from_str(json_text).map_err(
        |source| SkinSourceError::Parse {
            path: path_text(path),
            source: source.to_string(),
        },
    )
}

/// Convert one component path to deterministic diagnostic text.
fn path_text(path: &Path) -> String {
    path.display()
        .to_string()
}

/// Decoded skin-source adapter error.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SkinSourceError {
    /// Component file could not be read.
    Read {
        /// Component path.
        path: String,
        /// IO error text.
        source: String,
    },
    /// Component file could not be parsed.
    Parse {
        /// Component path.
        path: String,
        /// JSON error text.
        source: String,
    },
    /// Component schema did not match the requested loader.
    UnsupportedSchema {
        /// Component path.
        path: String,
        /// Schema marker found in the file.
        schema: String,
    },
    /// Component name was empty after padding removal.
    BlankComponentName {
        /// Component path.
        path: String,
    },
    /// Skeleton contained no joints.
    EmptySkeleton {
        /// Skeleton path.
        path: String,
    },
    /// Skeleton declared a version unsupported by the decoded adapter.
    UnsupportedSkeletonVersion {
        /// Skeleton path.
        path: String,
        /// Version declared by the decoded source.
        version: u32,
    },
    /// Skeleton joint declaration differed from decoded joint records.
    JointCountMismatch {
        /// Skeleton path.
        path: String,
        /// Count declared by the decoded source.
        declared: u32,
        /// Number of decoded joint records.
        actual: usize,
    },
    /// Skin declared a version unsupported by the decoded adapter.
    UnsupportedSkinVersion {
        /// Skin path.
        path: String,
        /// Version declared by the decoded source.
        version: u32,
    },
    /// Skin primitive-group declaration differed from decoded groups.
    PrimitiveGroupCountMismatch {
        /// Skin path.
        path: String,
        /// Count declared by the decoded source.
        declared: u32,
        /// Number of decoded primitive groups.
        actual: usize,
    },
    /// Primitive-group vertex declaration differed from decoded positions.
    VertexCountMismatch {
        /// Skin path.
        path: String,
        /// Primitive-group position.
        group: usize,
        /// Count declared by the decoded source.
        declared: u32,
        /// Number of decoded vertex positions.
        actual: usize,
    },
    /// Primitive-group index declaration differed from decoded indices.
    IndexCountMismatch {
        /// Skin path.
        path: String,
        /// Primitive-group position.
        group: usize,
        /// Count declared by the decoded source.
        declared: u32,
        /// Number of decoded indices.
        actual: usize,
    },
    /// Declared matrix-palette count differed from decoded palette entries.
    MatrixPaletteCountMismatch {
        /// Skin path.
        path: String,
        /// Primitive-group position.
        group: usize,
        /// Count declared by the decoded source.
        declared: u32,
        /// Number of decoded matrix-palette entries.
        actual: usize,
    },
    /// Composite skin declaration differed from decoded bindings.
    CompositeSkinCountMismatch {
        /// Composite path.
        path: String,
        /// Count declared by the decoded source.
        declared: u32,
        /// Number of decoded skin bindings.
        actual: usize,
    },
    /// Composite prop declaration differed from decoded bindings.
    CompositePropCountMismatch {
        /// Composite path.
        path: String,
        /// Count declared by the decoded source.
        declared: u32,
        /// Number of decoded prop bindings.
        actual: usize,
    },
    /// Composite effect declaration differed from decoded bindings.
    CompositeEffectCountMismatch {
        /// Composite path.
        path: String,
        /// Count declared by the decoded source.
        declared: u32,
        /// Number of decoded effect bindings.
        actual: usize,
    },
    /// One joint name was empty after padding removal.
    BlankJointName {
        /// Skeleton path.
        path: String,
        /// Joint position inside the skeleton.
        joint: usize,
    },
    /// One joint referenced an invalid parent position.
    InvalidJointParent {
        /// Skeleton path.
        path: String,
        /// Joint position inside the skeleton.
        joint: usize,
        /// Invalid parent position.
        parent: usize,
    },
    /// Skin referenced a skeleton other than the loaded one.
    SkeletonReferenceMismatch {
        /// Skin or composite identity with the stale reference.
        skin: String,
        /// Loaded skeleton identity.
        expected: String,
        /// Referenced skeleton identity.
        found: String,
    },
    /// Composite referenced a skin that was not loaded.
    CompositeSkinMissing {
        /// Composite path.
        path: String,
        /// Missing skin identity.
        skin: String,
    },
    /// One group used an unsupported primitive topology.
    UnsupportedPrimType {
        /// Skin path.
        path: String,
        /// Primitive-group index.
        group: usize,
        /// Unsupported primitive type.
        prim_type: u32,
    },
    /// One group declared a UV channel outside the supported set.
    UnsupportedUvChannel {
        /// Skin path.
        path: String,
        /// Primitive-group index.
        group: usize,
        /// Unsupported channel identity.
        channel: u32,
    },
    /// One group repeated the supported UV channel.
    DuplicateUvChannel {
        /// Skin path.
        path: String,
        /// Primitive-group index.
        group: usize,
        /// Repeated channel identity.
        channel: u32,
    },
    /// Matrix bindings did not align with vertex positions.
    MatrixCountMismatch {
        /// Skin path.
        path: String,
        /// Primitive-group index.
        group: usize,
        /// Vertex position count.
        positions: usize,
        /// Matrix binding count.
        matrices: usize,
    },
    /// Weight records did not align with vertex positions.
    WeightCountMismatch {
        /// Skin path.
        path: String,
        /// Primitive-group index.
        group: usize,
        /// Vertex position count.
        positions: usize,
        /// Weight record count.
        weights: usize,
    },
    /// One stored weight was outside the normalized range.
    InvalidStoredWeight {
        /// Skin path.
        path: String,
        /// Primitive-group index.
        group: usize,
        /// Vertex index containing the invalid weight.
        vertex: usize,
    },
    /// One palette slot exceeded the group palette.
    PaletteSlotOutOfRange {
        /// Skin path.
        path: String,
        /// Primitive-group index.
        group: usize,
        /// Vertex index containing the invalid slot.
        vertex: usize,
        /// Invalid palette slot.
        slot: u8,
        /// Palette length.
        palette: usize,
    },
    /// One palette entry referenced a joint outside the skeleton.
    PaletteJointOutOfRange {
        /// Skin path.
        path: String,
        /// Primitive-group index.
        group: usize,
        /// Invalid joint position.
        joint: u32,
        /// Skeleton joint count.
        joints: usize,
    },
    /// One group produced more vertices than the index space allows.
    VertexIndexOverflow {
        /// Skin path.
        path: String,
        /// Primitive-group index.
        group: usize,
        /// Overflowing vertex position.
        vertex: usize,
    },
    /// One group palette was empty.
    EmptyMatrixPalette {
        /// Skin path.
        path: String,
        /// Primitive-group index.
        group: usize,
    },
    /// Mesh construction failed after decoding.
    Mesh {
        /// Skin path.
        path: String,
        /// Domain mesh error.
        error: MeshError,
    },
    /// Rigid composite prop loading or binding failed.
    Prop(String),
    /// Character aggregate validation failed.
    Character(CharacterError),
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
/// Internal data shape for the adapter implementation.
struct DecodedSkeleton {
    /// Decoded schema marker.
    schema: String,
    /// Skeleton display name with fixed-width padding.
    name: String,
    /// Decoded skeleton version.
    #[serde(rename = "version")]
    version: u32,
    /// Number of joints declared by the decoded source.
    #[serde(rename = "num_joints")]
    joint_count: u32,
    /// Decoded joints ordered parent-before-child.
    joints: Vec<DecodedJoint>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
/// Internal data shape for the adapter implementation.
struct DecodedJoint {
    /// Joint display name with fixed-width padding.
    name: String,
    /// Parent joint position.
    parent: usize,
    /// Degree-of-freedom mask retained for schema compatibility.
    #[serde(rename = "dof")]
    _dof: u32,
    /// Free-axis mask retained for schema compatibility.
    #[serde(rename = "free_axes")]
    _free_axes: u32,
    /// Primary axis retained for schema compatibility.
    #[serde(rename = "primary_axis")]
    _primary_axis: u32,
    /// Secondary axis retained for schema compatibility.
    #[serde(rename = "secondary_axis")]
    _secondary_axis: u32,
    /// Twist axis retained for schema compatibility.
    #[serde(rename = "twist_axis")]
    _twist_axis: u32,
    /// Local rest pose matrix in row-major order.
    rest_pose: [f32; 16],
    /// Joint metadata retained for schema compatibility.
    #[serde(
        default,
        rename = "joint_metadata"
    )]
    _joint_metadata: serde_json::Value,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
/// Internal data shape for the adapter implementation.
struct DecodedSkin {
    /// Decoded schema marker.
    schema: String,
    /// Skin display name with fixed-width padding.
    name: String,
    /// Decoded skin version.
    #[serde(rename = "version")]
    version: u32,
    /// Referenced skeleton name with fixed-width padding.
    skeleton_name: String,
    /// Number of primitive groups declared by the decoded source.
    #[serde(rename = "num_prim_groups")]
    primitive_group_count: u32,
    /// Decoded primitive groups.
    prim_groups: Vec<DecodedSkinGroup>,
    /// Bounding box retained for schema compatibility.
    #[serde(
        default,
        rename = "bounding_box"
    )]
    _bounding_box: serde_json::Value,
    /// Bounding sphere retained for schema compatibility.
    #[serde(
        default,
        rename = "bounding_sphere"
    )]
    _bounding_sphere: serde_json::Value,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
/// Internal data shape for the adapter implementation.
struct DecodedSkinGroup {
    /// Shader reference with fixed-width padding.
    shader: String,
    /// Vertex shader reference retained for schema compatibility.
    #[serde(rename = "vertex_shader")]
    _vertex_shader: String,
    /// Primitive topology selector.
    prim_type: u32,
    /// Vertex format mask retained for schema compatibility.
    #[serde(rename = "vertex_format")]
    _vertex_format: u32,
    /// Number of vertices declared by the decoded source.
    #[serde(rename = "vertex_count")]
    vertex_count: u32,
    /// Number of indices declared by the decoded source.
    #[serde(rename = "index_count")]
    index_count: u32,
    /// Number of matrix-palette entries declared by the decoded source.
    #[serde(rename = "matrix_count")]
    matrix_count: u32,
    /// Vertex positions.
    positions: Vec<[f32; 3]>,
    /// Packed normals retained for schema compatibility.
    #[serde(
        default,
        rename = "packed_normals"
    )]
    _packed_normals: serde_json::Value,
    /// Per-vertex normals aligned with positions.
    normals: Vec<[f32; 3]>,
    /// Per-vertex palette slots.
    matrices: Vec<[u8; 4]>,
    /// Optional stored per-vertex weights.
    #[serde(default)]
    weights: Option<Vec<[f32; 3]>>,
    /// Palette mapping slots to skeleton joints.
    matrix_palette: Vec<u32>,
    /// Primitive index stream.
    indices: Vec<u32>,
    /// UV channels decoded for this group.
    #[serde(default)]
    uvs: Vec<DecodedUvChannel>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
/// Internal data shape for the adapter implementation.
struct DecodedUvChannel {
    /// UV channel index.
    channel: u32,
    /// UV coordinates decoded for the channel.
    coords: Vec<[f32; 2]>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
/// Internal data shape for the adapter implementation.
struct DecodedComposite {
    /// Decoded schema marker.
    schema: String,
    /// Composite display name with fixed-width padding.
    name: String,
    /// Referenced skeleton name with fixed-width padding.
    skeleton_name: String,
    /// Number of skin bindings declared by the decoded source.
    #[serde(rename = "num_skins")]
    skin_count: u32,
    /// Skin bindings listed by the composite.
    skins: Vec<DecodedCompositeSkin>,
    /// Number of prop bindings declared by the decoded source.
    #[serde(
        default,
        rename = "num_props"
    )]
    prop_count: u32,
    /// Prop bindings listed by the composite.
    #[serde(default)]
    props: Vec<DecodedCompositeProp>,
    /// Number of effect bindings declared by the decoded source.
    #[serde(
        default,
        rename = "num_effects"
    )]
    effect_count: u32,
    /// Effect bindings listed by the composite.
    #[serde(default)]
    effects: Vec<serde_json::Value>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
/// One rigid mesh prop attached to a skeleton joint.
struct DecodedCompositeProp {
    /// Binding kind, required to be `prop`.
    kind: String,
    /// Referenced mesh name with fixed-width padding.
    name: String,
    /// Translucency flag retained for schema compatibility.
    #[serde(rename = "is_translucent")]
    is_translucent: serde_json::Value,
    /// Zero-based skeleton joint position owning the rigid prop.
    skeleton_joint_id: usize,
    /// Sort order retained for schema compatibility.
    #[serde(rename = "sort_order")]
    _sort_order: serde_json::Value,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
/// Internal data shape for the adapter implementation.
struct DecodedCompositeSkin {
    /// Binding kind retained for schema compatibility.
    #[serde(
        default,
        rename = "kind"
    )]
    _kind: String,
    /// Referenced skin name with fixed-width padding.
    name: String,
    /// Translucency flag retained for schema compatibility.
    #[serde(
        default,
        rename = "is_translucent"
    )]
    is_translucent: serde_json::Value,
    /// Sort order retained for schema compatibility.
    #[serde(
        default,
        rename = "sort_order"
    )]
    _sort_order: serde_json::Value,
}

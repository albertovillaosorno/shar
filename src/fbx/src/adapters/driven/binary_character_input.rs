// File:
//   - binary_character_input.rs
// Path:
//   - src/fbx/src/adapters/driven/binary_character_input.rs
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
//   - Serializer-local character material and bind-transform validation.
// - Must-Not:
//   - Select packages, validate package membership, or plan phase-three work.
// - Allows:
//   - Validate material bindings and precompute binary FBX bind transforms.
// - Split-When:
//   - Material and skeleton preparation gain independent serializer callers.
// - Merge-When:
//   - The binary character writer no longer needs pre-serialization checks.
// - Summary:
//   - Prepares one phase-three-resolved character aggregate for serialization.
// - Description:
//   - Enforces only invariants needed by the binary FBX character writer.
// - Usage:
//   - Consumed by the sibling binary character writer after package assembly.
// - Defaults:
//   - Rejects ambiguous materials and non-portable texture file identities.
//
// ADRs:
// - docs/adr/fbx/export/fbx-output-contract-boundary.md
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
//
// Large file:
//   - false
//

//! Serializer-local preparation for canonical binary character FBX output.

use std::collections::{BTreeMap, BTreeSet};

use super::binary_identity::{BinaryIdentityError, MaterialIds, material_ids};
use crate::domain::character::CharacterAsset;
use crate::domain::scene::identity::is_portable_path_segment;
use crate::domain::texture::{MaterialBinding, MaterialSemantics};
use crate::domain::transform::matrix::{
    MatrixError, TrsParts, decompose, multiply, widen,
};

/// Object id of the single bind pose.
pub(super) const POSE_ID: u64 = 6_000_000_000;

/// One deduplicated material slot with deterministic ids.
pub(super) struct MaterialSlot<'materials> {
    /// Deterministic object ids for the material triple.
    pub(super) ids: MaterialIds,
    /// Borrowed material binding.
    pub(super) binding: &'materials MaterialBinding,
    /// Effective semantics merged from material and source geometry evidence.
    pub(super) semantics: MaterialSemantics,
}

/// Precomputed local and global bind transforms for one bone.
pub(super) struct BoneTransform {
    /// Decomposed local rest transform.
    pub(super) local_parts: TrsParts,
    /// Accumulated global bind matrix in row-major order.
    pub(super) global_bind: [f64; 16],
}

/// Compute decomposed local parts and global binds for every bone.
pub(super) fn bone_transforms(
    character: &CharacterAsset
) -> Result<Vec<BoneTransform>, CharacterInputError> {
    let ordinals: BTreeMap<&str, usize> = character
        .bones
        .iter()
        .enumerate()
        .map(
            |(ordinal, bone)| {
                (
                    bone.id
                        .as_str(),
                    ordinal,
                )
            },
        )
        .collect();
    let mut transforms: Vec<BoneTransform> = Vec::with_capacity(
        character
            .bones
            .len(),
    );
    for bone in &character.bones {
        let local = widen(&bone.rest_matrix);
        let local_parts = decompose(&local).map_err(
            |error| CharacterInputError::UnsupportedRestMatrix {
                bone: bone
                    .id
                    .clone(),
                error,
            },
        )?;
        let global_bind = match &bone.parent_id {
            Some(parent) => {
                let parent_transform = ordinals
                    .get(parent.as_str())
                    .and_then(|ordinal| transforms.get(*ordinal))
                    .ok_or_else(
                        || CharacterInputError::MissingParentTransform {
                            bone: bone
                                .id
                                .clone(),
                            parent: parent.clone(),
                        },
                    )?;
                multiply(
                    &local,
                    &parent_transform.global_bind,
                )
            }
            None => local,
        };
        transforms.push(
            BoneTransform {
                local_parts,
                global_bind,
            },
        );
    }
    Ok(transforms)
}

/// Deduplicate material bindings by used shader in stable order.
pub(super) fn material_slots<'materials>(
    character: &CharacterAsset,
    materials: &'materials [MaterialBinding],
) -> Result<BTreeMap<String, MaterialSlot<'materials>>, CharacterInputError> {
    let mut used_shaders = BTreeSet::new();
    let mut geometry_semantics = BTreeMap::<String, MaterialSemantics>::new();
    for part in &character.parts {
        let part_semantics = MaterialSemantics::from_identities(
            &part
                .mesh
                .name,
            None,
        );
        for group in &part
            .mesh
            .groups
        {
            let _inserted = used_shaders.insert(
                group
                    .shader
                    .clone(),
            );
            geometry_semantics
                .entry(
                    group
                        .shader
                        .clone(),
                )
                .and_modify(
                    |current| {
                        *current = current.merge(part_semantics);
                    },
                )
                .or_insert(part_semantics);
        }
    }
    let mut bindings_by_name: BTreeMap<&str, &MaterialBinding> =
        BTreeMap::new();
    for binding in materials {
        if bindings_by_name
            .insert(
                binding
                    .material_name
                    .as_str(),
                binding,
            )
            .is_some()
        {
            return Err(
                CharacterInputError::DuplicateMaterialBinding {
                    material: binding
                        .material_name
                        .clone(),
                },
            );
        }
        if let Some(file_name) = &binding.texture_file_name
            && !is_portable_texture_file_name(file_name)
        {
            return Err(
                CharacterInputError::InvalidTextureFileName {
                    file_name: file_name.clone(),
                },
            );
        }
    }
    for binding in materials {
        if !used_shaders.contains(&binding.material_name) {
            return Err(
                CharacterInputError::UnusedMaterialBinding {
                    material: binding
                        .material_name
                        .clone(),
                },
            );
        }
    }
    let mut slots = BTreeMap::new();
    for (ordinal, shader) in used_shaders
        .iter()
        .enumerate()
    {
        let binding = bindings_by_name
            .get(shader.as_str())
            .ok_or_else(
                || CharacterInputError::MissingMaterialBinding {
                    shader: shader.clone(),
                },
            )?;
        let _previous = slots.insert(
            shader.clone(),
            MaterialSlot {
                ids: material_ids(ordinal)?,
                binding,
                semantics: binding
                    .semantics
                    .merge(
                        geometry_semantics
                            .get(shader)
                            .copied()
                            .unwrap_or_default(),
                    ),
            },
        );
    }
    Ok(slots)
}

/// Return whether one texture identity is one portable file name.
fn is_portable_texture_file_name(value: &str) -> bool {
    is_portable_path_segment(value)
}

/// Character aggregate rejected by binary serialization preparation.
#[derive(Clone, Debug, PartialEq)]
pub(super) enum CharacterInputError {
    /// One bone rest matrix could not map to an FBX local transform.
    UnsupportedRestMatrix {
        /// Bone identity with the unsupported matrix.
        bone: String,
        /// Decomposition failure detail.
        error: MatrixError,
    },
    /// One bone referenced a parent whose transform was not computed.
    MissingParentTransform {
        /// Child bone identity.
        bone: String,
        /// Referenced parent identity.
        parent: String,
    },
    /// One shader had no material binding.
    MissingMaterialBinding {
        /// Shader identity without a binding.
        shader: String,
    },
    /// One material binding repeated a material identity.
    DuplicateMaterialBinding {
        /// Repeated material identity.
        material: String,
    },
    /// One material binding was not used by any group.
    UnusedMaterialBinding {
        /// Unused material identity.
        material: String,
    },
    /// Texture file identity escaped the artifact texture directory.
    InvalidTextureFileName {
        /// Rejected texture file identity.
        file_name: String,
    },
    /// Serializer-local object identity allocation failed.
    Identity {
        /// Stable checked identity failure.
        reason: String,
    },
}

impl From<BinaryIdentityError> for CharacterInputError {
    fn from(error: BinaryIdentityError) -> Self {
        Self::Identity {
            reason: format!("{error:?}"),
        }
    }
}

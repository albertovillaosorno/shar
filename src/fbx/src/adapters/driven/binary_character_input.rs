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
//   - true
//   - Reason: one serializer-input boundary keeps source-binding validation,
//     semantic material variants, bind transforms, and shared errors aligned.
//   - Split: separate material and skeleton plans when either gains a second
//     serializer consumer.
//   - Validation: canonical FBX validation and binary material variant tests.
//   - Review: required whenever another input responsibility is added.
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

/// One deduplicated semantic material variant with deterministic ids.
pub(super) struct MaterialSlot<'materials> {
    /// Deterministic object ids for the material triple.
    pub(super) ids: MaterialIds,
    /// Borrowed source material binding.
    pub(super) binding: &'materials MaterialBinding,
    /// Effective semantics for this exact material variant.
    pub(super) semantics: MaterialSemantics,
    /// Unique portable FBX object identity for this variant.
    pub(super) object_name: String,
}

/// One complete source-binding and semantic-variant material plan.
pub(super) struct MaterialPlan<'materials> {
    /// Material variants keyed by source identity and semantic signature.
    pub(super) slots: BTreeMap<String, MaterialSlot<'materials>>,
    /// Original source bindings keyed by authored shader identity.
    pub(super) source_bindings:
        BTreeMap<&'materials str, &'materials MaterialBinding>,
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

/// Return effective source and geometry semantics for one exact mesh group.
#[must_use]
pub(super) fn effective_material_semantics(
    mesh_name: &str,
    binding: &MaterialBinding,
) -> MaterialSemantics {
    binding
        .semantics
        .merge(
            MaterialSemantics::from_identities(
                mesh_name, None,
            ),
        )
}

/// Build one stable internal key for a source material semantic variant.
#[must_use]
pub(super) fn material_variant_key(
    material_name: &str,
    semantics: MaterialSemantics,
) -> String {
    let signature = semantics
        .suffix()
        .unwrap_or_else(|| "opaque".to_owned());
    format!("{material_name}::{signature}")
}

/// Build one unique portable FBX object identity for a material variant.
fn material_object_name(
    material_name: &str,
    semantics: MaterialSemantics,
) -> String {
    semantics
        .suffix()
        .map_or_else(
            || material_name.to_owned(),
            |suffix| format!("{material_name}__{suffix}"),
        )
}

/// Deduplicate source bindings into exact semantic variants in stable order.
pub(super) fn material_slots<'materials>(
    character: &CharacterAsset,
    materials: &'materials [MaterialBinding],
) -> Result<MaterialPlan<'materials>, CharacterInputError> {
    let source_bindings = validated_source_bindings(
        character, materials,
    )?;
    let variants = material_variants(
        character,
        &source_bindings,
    )?;
    let mut slots = BTreeMap::new();
    for (ordinal, (key, (binding, semantics))) in variants
        .into_iter()
        .enumerate()
    {
        let _previous = slots.insert(
            key,
            MaterialSlot {
                ids: material_ids(ordinal)?,
                binding,
                semantics,
                object_name: material_object_name(
                    &binding.material_name,
                    semantics,
                ),
            },
        );
    }
    Ok(
        MaterialPlan {
            slots,
            source_bindings,
        },
    )
}

/// Validate source material identities and index every used shader binding.
fn validated_source_bindings<'materials>(
    character: &CharacterAsset,
    materials: &'materials [MaterialBinding],
) -> Result<
    BTreeMap<&'materials str, &'materials MaterialBinding>,
    CharacterInputError,
> {
    let used_shaders = character
        .parts
        .iter()
        .flat_map(
            |part| {
                part.mesh
                    .groups
                    .iter()
            },
        )
        .map(
            |group| {
                group
                    .shader
                    .as_str()
            },
        )
        .collect::<BTreeSet<_>>();
    let mut source_bindings = BTreeMap::new();
    for binding in materials {
        if source_bindings
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
        if !used_shaders.contains(
            binding
                .material_name
                .as_str(),
        ) {
            return Err(
                CharacterInputError::UnusedMaterialBinding {
                    material: binding
                        .material_name
                        .clone(),
                },
            );
        }
    }
    Ok(source_bindings)
}

/// Collect exact source-shader and geometry-semantic material variants.
fn material_variants<'materials>(
    character: &CharacterAsset,
    source_bindings: &BTreeMap<&'materials str, &'materials MaterialBinding>,
) -> Result<
    BTreeMap<
        String,
        (
            &'materials MaterialBinding,
            MaterialSemantics,
        ),
    >,
    CharacterInputError,
> {
    let mut variants = BTreeMap::new();
    for part in &character.parts {
        for group in &part
            .mesh
            .groups
        {
            let binding = source_bindings
                .get(
                    group
                        .shader
                        .as_str(),
                )
                .copied()
                .ok_or_else(
                    || CharacterInputError::MissingMaterialBinding {
                        shader: group
                            .shader
                            .clone(),
                    },
                )?;
            let semantics = effective_material_semantics(
                &part
                    .mesh
                    .name,
                binding,
            );
            let key = material_variant_key(
                &group.shader,
                semantics,
            );
            let _entry = variants
                .entry(key)
                .or_insert(
                    (
                        binding, semantics,
                    ),
                );
        }
    }
    Ok(variants)
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

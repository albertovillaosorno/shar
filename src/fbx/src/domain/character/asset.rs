// File:
//   - asset.rs
// Path:
//   - src/fbx/src/domain/character/asset.rs
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
//   - Pure fbx domain rules for domain character asset.
// - Must-Not:
//   - Read files, parse generated indexes, invoke CLI code, or call writer
//   - adapters.
// - Allows:
//   - Value objects, invariant checks, and pure evidence-to-domain translation.
// - Split-When:
//   - Split when asset contains two independently testable contracts.
// - Merge-When:
//   - Another fbx module owns the same domain boundary with no distinct
//   - invariant.
// - Summary:
//   - Skinned character aggregate with skeleton and bound mesh parts.
// - Description:
//   - Defines the character asset invariants for fbx domain character.
// - Usage:
//   - Imported through crate domain facades or sibling domain modules.
// - Defaults:
//   - No filesystem paths, no external process calls, and no implicit IO
//   - defaults.
//
// ADRs:
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
//
// Large file:
//   - true
//   - Reason: src/fbx/src/domain/character/asset.rs keeps the character
//   - aggregate, its skeleton ordering rules, and its influence coverage
//   - validation together because they enforce one invariant set; split when
//   - one rule family gains an independent contract.
//

//! Skinned character aggregate with skeleton and bound mesh parts.
use std::collections::BTreeSet;

use crate::domain::mesh::MeshAsset;
use crate::domain::skeleton::Bone;
use crate::domain::skin::SkinInfluence;

/// Maximum tolerated deviation of one vertex weight sum from one.
const WEIGHT_SUM_TOLERANCE: f32 = 1e-3;

/// One skinned mesh part bound to the shared character skeleton.
#[derive(Clone, Debug, PartialEq)]
pub struct SkinnedPart {
    /// Validated mesh geometry for this part.
    pub mesh: MeshAsset,
    /// Influences per primitive group, aligned with the mesh group order.
    pub group_influences: Vec<Vec<SkinInfluence>>,
}

/// Skinned character aggregate with skeleton and bound mesh parts.
// The explicit aggregate name distinguishes the character root from mesh parts.
#[expect(
    clippy::module_name_repetitions,
    reason = "CharacterAsset is the stable aggregate name across export \
              adapters."
)]
#[derive(Clone, Debug, PartialEq)]
pub struct CharacterAsset {
    /// Stable character name.
    pub name: String,
    /// Skeleton bones ordered so parents precede children.
    pub bones: Vec<Bone>,
    /// Skinned mesh parts bound to the shared skeleton.
    pub parts: Vec<SkinnedPart>,
}

impl CharacterAsset {
    /// Create a validated skinned character aggregate.
    ///
    /// # Errors
    ///
    /// Returns an error when the skeleton hierarchy, influence coverage, or
    /// weight normalization violates the character contract.
    pub fn new(
        name: impl Into<String>,
        bones: Vec<Bone>,
        parts: Vec<SkinnedPart>,
    ) -> Result<Self, CharacterError> {
        let character_name = name.into();
        if character_name
            .trim()
            .is_empty()
        {
            return Err(CharacterError::MissingCharacterName);
        }
        if character_name != character_name.trim()
            || character_name
                .chars()
                .any(char::is_control)
        {
            return Err(CharacterError::NonCanonicalCharacterName);
        }
        let bone_ids = validate_bones(&bones)?;
        if parts.is_empty() {
            return Err(CharacterError::MissingParts);
        }
        for part in &parts {
            validate_part(
                part, &bone_ids,
            )?;
        }
        Ok(
            Self {
                name: character_name,
                bones,
                parts,
            },
        )
    }
}

/// Validate skeleton ordering, identity, and matrix quality.
fn validate_bones(bones: &[Bone]) -> Result<BTreeSet<String>, CharacterError> {
    if bones.is_empty() {
        return Err(CharacterError::MissingBones);
    }
    let mut seen = BTreeSet::new();
    for bone in bones {
        if bone
            .id
            .trim()
            .is_empty()
        {
            return Err(CharacterError::MissingBoneId);
        }
        if bone.id
            != bone
                .id
                .trim()
            || bone
                .id
                .chars()
                .any(char::is_control)
        {
            return Err(
                CharacterError::NonCanonicalBoneId {
                    bone: bone
                        .id
                        .clone(),
                },
            );
        }
        if let Some(parent) = &bone.parent_id {
            if parent
                .trim()
                .is_empty()
                || parent != parent.trim()
                || parent
                    .chars()
                    .any(char::is_control)
            {
                return Err(
                    CharacterError::NonCanonicalParentId {
                        bone: bone
                            .id
                            .clone(),
                        parent: parent.clone(),
                    },
                );
            }
            if !seen.contains(parent) {
                return Err(
                    CharacterError::ParentNotBeforeChild {
                        bone: bone
                            .id
                            .clone(),
                        parent: parent.clone(),
                    },
                );
            }
        }
        if let Some(component) = bone
            .rest_matrix
            .iter()
            .position(|value| !value.is_finite())
        {
            return Err(
                CharacterError::NonFiniteRestMatrix {
                    bone: bone
                        .id
                        .clone(),
                    component,
                },
            );
        }
        if !seen.insert(
            bone.id
                .clone(),
        ) {
            return Err(
                CharacterError::DuplicateBoneId {
                    bone: bone
                        .id
                        .clone(),
                },
            );
        }
    }
    Ok(seen)
}

/// Validate one skinned part against the shared skeleton contract.
fn validate_part(
    part: &SkinnedPart,
    bone_ids: &BTreeSet<String>,
) -> Result<(), CharacterError> {
    if part
        .group_influences
        .len()
        != part
            .mesh
            .groups
            .len()
    {
        return Err(
            CharacterError::InfluenceGroupCountMismatch {
                mesh: part
                    .mesh
                    .name
                    .clone(),
                groups: part
                    .mesh
                    .groups
                    .len(),
                influence_groups: part
                    .group_influences
                    .len(),
            },
        );
    }
    for (group, influences) in part
        .mesh
        .groups
        .iter()
        .zip(&part.group_influences)
    {
        validate_group_influences(
            &part
                .mesh
                .name,
            group.index,
            group
                .positions
                .len(),
            influences,
            bone_ids,
        )?;
    }
    Ok(())
}

/// Validate influence coverage and weight quality for one primitive group.
fn validate_group_influences(
    mesh_name: &str,
    group_index: usize,
    vertex_count: usize,
    influences: &[SkinInfluence],
    bone_ids: &BTreeSet<String>,
) -> Result<(), CharacterError> {
    let mut weight_sums = vec![0.0_f32; vertex_count];
    for influence in influences {
        if !bone_ids.contains(&influence.bone_id) {
            return Err(
                CharacterError::UnknownInfluenceBone {
                    mesh: mesh_name.to_owned(),
                    group: group_index,
                    bone: influence
                        .bone_id
                        .clone(),
                },
            );
        }
        let vertex = match usize::try_from(influence.vertex_index) {
            Ok(value) => value,
            Err(_conversion_error) => {
                return Err(
                    CharacterError::InfluenceVertexOutOfBounds {
                        mesh: mesh_name.to_owned(),
                        group: group_index,
                        vertex: influence.vertex_index,
                        vertices: vertex_count,
                    },
                );
            }
        };
        let Some(sum) = weight_sums.get_mut(vertex) else {
            return Err(
                CharacterError::InfluenceVertexOutOfBounds {
                    mesh: mesh_name.to_owned(),
                    group: group_index,
                    vertex: influence.vertex_index,
                    vertices: vertex_count,
                },
            );
        };
        if !influence
            .weight
            .is_finite()
            || influence.weight <= 0.0
            || influence.weight > 1.0 + WEIGHT_SUM_TOLERANCE
        {
            return Err(
                CharacterError::InvalidInfluenceWeight {
                    mesh: mesh_name.to_owned(),
                    group: group_index,
                    vertex: influence.vertex_index,
                },
            );
        }
        *sum += influence.weight;
    }
    for (vertex, sum) in weight_sums
        .iter()
        .enumerate()
    {
        if (sum - 1.0).abs() > WEIGHT_SUM_TOLERANCE {
            return Err(
                CharacterError::UnnormalizedVertexWeights {
                    mesh: mesh_name.to_owned(),
                    group: group_index,
                    vertex,
                },
            );
        }
    }
    Ok(())
}

/// Character aggregate validation failure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CharacterError {
    /// Character identity was empty or whitespace-only.
    MissingCharacterName,
    /// Character identity carried surrounding whitespace.
    NonCanonicalCharacterName,
    /// Character skeleton contained no bones.
    MissingBones,
    /// One bone identity was empty or whitespace-only.
    MissingBoneId,
    /// One bone identity carried surrounding whitespace.
    NonCanonicalBoneId {
        /// Non-canonical bone identity.
        bone: String,
    },
    /// One bone identity appeared more than once.
    DuplicateBoneId {
        /// Repeated bone identity.
        bone: String,
    },
    /// One parent identity was empty or non-canonical.
    NonCanonicalParentId {
        /// Child bone carrying the malformed parent identity.
        bone: String,
        /// Malformed parent identity.
        parent: String,
    },
    /// One bone referenced a parent that did not precede it.
    ParentNotBeforeChild {
        /// Child bone identity.
        bone: String,
        /// Missing or later parent identity.
        parent: String,
    },
    /// One bone rest matrix contained a non-finite component.
    NonFiniteRestMatrix {
        /// Bone identity containing the invalid matrix.
        bone: String,
        /// Row-major component index inside the matrix.
        component: usize,
    },
    /// Character contained no skinned mesh parts.
    MissingParts,
    /// Influence groups did not align with mesh primitive groups.
    InfluenceGroupCountMismatch {
        /// Mesh identity with the mismatched binding.
        mesh: String,
        /// Primitive-group count in the mesh.
        groups: usize,
        /// Influence-group count supplied for the mesh.
        influence_groups: usize,
    },
    /// One influence referenced a bone outside the skeleton.
    UnknownInfluenceBone {
        /// Mesh identity containing the influence.
        mesh: String,
        /// Primitive-group index containing the influence.
        group: usize,
        /// Unknown bone identity.
        bone: String,
    },
    /// One influence referenced a vertex outside the group.
    InfluenceVertexOutOfBounds {
        /// Mesh identity containing the influence.
        mesh: String,
        /// Primitive-group index containing the influence.
        group: usize,
        /// Invalid vertex index.
        vertex: u32,
        /// Available vertex count.
        vertices: usize,
    },
    /// One influence weight was not usable.
    InvalidInfluenceWeight {
        /// Mesh identity containing the influence.
        mesh: String,
        /// Primitive-group index containing the influence.
        group: usize,
        /// Vertex index containing the invalid weight.
        vertex: u32,
    },
    /// One vertex accumulated weights that did not sum to one.
    UnnormalizedVertexWeights {
        /// Mesh identity containing the vertex.
        mesh: String,
        /// Primitive-group index containing the vertex.
        group: usize,
        /// Vertex index with the unnormalized weight sum.
        vertex: usize,
    },
}

#[cfg(test)]
#[test]
fn rejects_control_characters_in_character_identities() {
    assert_eq!(
        CharacterAsset::new(
            "character\nalias",
            Vec::new(),
            Vec::new(),
        ),
        Err(CharacterError::NonCanonicalCharacterName)
    );
    assert_eq!(
        CharacterAsset::new(
            "character",
            vec![
                Bone {
                    id: "root\nalias".to_owned(),
                    parent_id: None,
                    rest_matrix: [0.0_f32; 16],
                },
            ],
            Vec::new(),
        ),
        Err(
            CharacterError::NonCanonicalBoneId {
                bone: "root\nalias".to_owned(),
            }
        )
    );
}

#[cfg(test)]
#[test]
fn rejects_noncanonical_parent_identities() {
    let bones = vec![
        Bone {
            id: "root".to_owned(),
            parent_id: None,
            rest_matrix: [0.0_f32; 16],
        },
        Bone {
            id: "child".to_owned(),
            parent_id: Some("root\nalias".to_owned()),
            rest_matrix: [0.0_f32; 16],
        },
    ];

    assert_eq!(
        CharacterAsset::new(
            "character",
            bones,
            Vec::new(),
        ),
        Err(
            CharacterError::NonCanonicalParentId {
                bone: "child".to_owned(),
                parent: "root\nalias".to_owned(),
            }
        )
    );
}

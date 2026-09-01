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
//   - Asset domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Asset domain module.
// - Description:
//   - Implements the declared domain module responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Asset domain module.

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

/// One authored composite skin relationship retained as source provenance.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompositeSkinSourceBinding {
    /// Zero-based composite occurrence in retained source order.
    composite_ordinal: usize,
    /// Zero-based skin position within the authored composite.
    skin_index: usize,
    /// Authored skin/component identity.
    skin_identity: String,
    /// Authored binary translucency flag.
    translucent: bool,
    /// Exact source-width sort-order bits when the optional field was present.
    sort_order_bits: Option<u32>,
}

impl CompositeSkinSourceBinding {
    /// Create one validated source-only composite skin record.
    ///
    /// # Errors
    ///
    /// Returns an error when the skin identity is non-canonical or a supplied
    /// sort-order value is non-finite.
    pub fn new(
        composite_ordinal: usize,
        skin_index: usize,
        skin_identity: impl Into<String>,
        translucent: bool,
        sort_order: Option<f32>,
    ) -> Result<Self, CharacterError> {
        let skin_identity = skin_identity.into();
        if !canonical_source_identity(&skin_identity) {
            return Err(CharacterError::NonCanonicalSourceSkinIdentity {
                identity: skin_identity,
            });
        }
        let sort_order_bits = match sort_order {
            Some(value) if value.is_finite() => Some(value.to_bits()),
            Some(_value) => {
                return Err(CharacterError::NonFiniteSourceSkinSortOrder {
                    composite_ordinal,
                    skin_index,
                });
            },
            None => None,
        };
        Ok(Self {
            composite_ordinal,
            skin_index,
            skin_identity,
            translucent,
            sort_order_bits,
        })
    }

    /// Return the zero-based retained composite occurrence.
    #[must_use]
    pub const fn composite_ordinal(&self) -> usize {
        self.composite_ordinal
    }

    /// Return the zero-based authored skin position.
    #[must_use]
    pub const fn skin_index(&self) -> usize {
        self.skin_index
    }

    /// Return the authored skin/component identity.
    #[must_use]
    pub fn skin_identity(&self) -> &str {
        &self.skin_identity
    }

    /// Return the authored binary translucency flag.
    #[must_use]
    pub const fn translucent(&self) -> bool {
        self.translucent
    }

    /// Return exact source-width sort-order bits when authored.
    #[must_use]
    pub const fn sort_order_bits(&self) -> Option<u32> {
        self.sort_order_bits
    }
}

/// One authored composite prop relationship retained as source provenance.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompositePropSourceBinding {
    /// Zero-based composite occurrence in retained source order.
    composite_ordinal: usize,
    /// Zero-based prop position within the authored composite.
    prop_index: usize,
    /// Authored prop/component identity.
    prop_identity: String,
    /// Zero-based skeleton joint position referenced by the composite.
    skeleton_joint_id: usize,
    /// Authored binary translucency flag.
    translucent: bool,
    /// Exact source-width sort-order bits when the optional field was present.
    sort_order_bits: Option<u32>,
}

impl CompositePropSourceBinding {
    /// Create one validated source-only composite prop record.
    ///
    /// # Errors
    ///
    /// Returns an error when the prop identity is non-canonical or a supplied
    /// sort-order value is non-finite.
    pub fn new(
        composite_ordinal: usize,
        prop_index: usize,
        prop_identity: impl Into<String>,
        skeleton_joint_id: usize,
        translucent: bool,
        sort_order: Option<f32>,
    ) -> Result<Self, CharacterError> {
        let prop_identity = prop_identity.into();
        if !canonical_source_identity(&prop_identity) {
            return Err(CharacterError::NonCanonicalSourcePropIdentity {
                identity: prop_identity,
            });
        }
        let sort_order_bits = match sort_order {
            Some(value) if value.is_finite() => Some(value.to_bits()),
            Some(_value) => {
                return Err(CharacterError::NonFiniteSourcePropSortOrder {
                    composite_ordinal,
                    prop_index,
                });
            },
            None => None,
        };
        Ok(Self {
            composite_ordinal,
            prop_index,
            prop_identity,
            skeleton_joint_id,
            translucent,
            sort_order_bits,
        })
    }

    /// Return the zero-based retained composite occurrence.
    #[must_use]
    pub const fn composite_ordinal(&self) -> usize {
        self.composite_ordinal
    }

    /// Return the zero-based authored prop position.
    #[must_use]
    pub const fn prop_index(&self) -> usize {
        self.prop_index
    }

    /// Return the authored prop/component identity.
    #[must_use]
    pub fn prop_identity(&self) -> &str {
        &self.prop_identity
    }

    /// Return the zero-based authored skeleton joint position.
    #[must_use]
    pub const fn skeleton_joint_id(&self) -> usize {
        self.skeleton_joint_id
    }

    /// Return the authored binary translucency flag.
    #[must_use]
    pub const fn translucent(&self) -> bool {
        self.translucent
    }

    /// Return exact source-width sort-order bits when authored.
    #[must_use]
    pub const fn sort_order_bits(&self) -> Option<u32> {
        self.sort_order_bits
    }
}

/// Authored source relationships retained independently of publication naming.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CharacterSourceProvenance {
    /// Decoded skeleton identity shared by the assembled character.
    skeleton_identity: String,
    /// Decoded composite identities in supplied source order.
    composite_identities: Vec<String>,
    /// Authored composite skin rows in exact retained source order.
    composite_skin_bindings: Vec<CompositeSkinSourceBinding>,
    /// Authored composite prop rows in exact retained source order.
    composite_prop_bindings: Vec<CompositePropSourceBinding>,
}

impl CharacterSourceProvenance {
    /// Create one validated source-relationship record.
    ///
    /// # Errors
    ///
    /// Returns an error when a skeleton or composite identity is blank, padded,
    /// or contains control characters.
    pub fn new(
        skeleton_identity: impl Into<String>,
        composite_identities: Vec<String>,
    ) -> Result<Self, CharacterError> {
        let skeleton_identity = skeleton_identity.into();
        if !canonical_source_identity(&skeleton_identity) {
            return Err(CharacterError::NonCanonicalSourceSkeletonIdentity {
                identity: skeleton_identity,
            });
        }
        for identity in &composite_identities {
            if !canonical_source_identity(identity) {
                return Err(
                    CharacterError::NonCanonicalSourceCompositeIdentity {
                        identity: identity.clone(),
                    },
                );
            }
        }
        Ok(Self {
            skeleton_identity,
            composite_identities,
            composite_skin_bindings: Vec::new(),
            composite_prop_bindings: Vec::new(),
        })
    }

    /// Return the validated authored skeleton identity.
    #[must_use]
    pub fn skeleton_identity(&self) -> &str {
        &self.skeleton_identity
    }

    /// Return composite identities in exact retained source order.
    #[must_use]
    pub fn composite_identities(&self) -> &[String] {
        &self.composite_identities
    }

    /// Attach exact authored composite skin rows as source-only provenance.
    ///
    /// # Errors
    ///
    /// Returns an error when a row references a composite occurrence outside
    /// the retained identity vector or repeats one authored skin position.
    pub fn with_composite_skin_bindings(
        mut self,
        bindings: Vec<CompositeSkinSourceBinding>,
    ) -> Result<Self, CharacterError> {
        let mut seen = BTreeSet::new();
        for binding in &bindings {
            if binding.composite_ordinal >= self.composite_identities.len() {
                return Err(CharacterError::SourceSkinCompositeOutOfBounds {
                    composite_ordinal: binding.composite_ordinal,
                    composites: self.composite_identities.len(),
                });
            }
            if !seen.insert((binding.composite_ordinal, binding.skin_index)) {
                return Err(
                    CharacterError::DuplicateSourceCompositeSkinIndex {
                        composite_ordinal: binding.composite_ordinal,
                        skin_index: binding.skin_index,
                    },
                );
            }
        }
        self.composite_skin_bindings = bindings;
        Ok(self)
    }

    /// Return authored composite skin rows in exact retained source order.
    #[must_use]
    pub fn composite_skin_bindings(&self) -> &[CompositeSkinSourceBinding] {
        &self.composite_skin_bindings
    }

    /// Attach exact authored composite prop rows as source-only provenance.
    ///
    /// # Errors
    ///
    /// Returns an error when a row references a composite occurrence outside
    /// the retained identity vector or repeats one authored prop position.
    pub fn with_composite_prop_bindings(
        mut self,
        bindings: Vec<CompositePropSourceBinding>,
    ) -> Result<Self, CharacterError> {
        let mut seen = BTreeSet::new();
        for binding in &bindings {
            if binding.composite_ordinal >= self.composite_identities.len() {
                return Err(CharacterError::SourcePropCompositeOutOfBounds {
                    composite_ordinal: binding.composite_ordinal,
                    composites: self.composite_identities.len(),
                });
            }
            if !seen.insert((binding.composite_ordinal, binding.prop_index)) {
                return Err(
                    CharacterError::DuplicateSourceCompositePropIndex {
                        composite_ordinal: binding.composite_ordinal,
                        prop_index: binding.prop_index,
                    },
                );
            }
        }
        self.composite_prop_bindings = bindings;
        Ok(self)
    }

    /// Return authored composite prop rows in exact retained source order.
    #[must_use]
    pub fn composite_prop_bindings(&self) -> &[CompositePropSourceBinding] {
        &self.composite_prop_bindings
    }
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
    /// Stable publication character name.
    pub name: String,
    /// Optional authored skeleton-to-composite source relationship evidence.
    pub source_provenance: Option<CharacterSourceProvenance>,
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
        if character_name.trim().is_empty() {
            return Err(CharacterError::MissingCharacterName);
        }
        if character_name != character_name.trim()
            || character_name.chars().any(char::is_control)
        {
            return Err(CharacterError::NonCanonicalCharacterName);
        }
        let bone_ids = validate_bones(&bones)?;
        if parts.is_empty() {
            return Err(CharacterError::MissingParts);
        }
        for part in &parts {
            validate_part(part, &bone_ids)?;
        }
        Ok(Self {
            name: character_name,
            source_provenance: None,
            bones,
            parts,
        })
    }

    /// Attach validated authored skeleton-to-composite relationship evidence.
    #[must_use]
    pub fn with_source_provenance(
        mut self,
        source_provenance: CharacterSourceProvenance,
    ) -> Self {
        self.source_provenance = Some(source_provenance);
        self
    }
}

/// Return whether one source identity is already canonical.
fn canonical_source_identity(identity: &str) -> bool {
    !identity.trim().is_empty()
        && identity == identity.trim()
        && !identity.chars().any(char::is_control)
}

/// Validate skeleton ordering, identity, and matrix quality.
fn validate_bones(bones: &[Bone]) -> Result<BTreeSet<String>, CharacterError> {
    if bones.is_empty() {
        return Err(CharacterError::MissingBones);
    }
    let mut seen = BTreeSet::new();
    for bone in bones {
        if bone.id.trim().is_empty() {
            return Err(CharacterError::MissingBoneId);
        }
        if bone.id != bone.id.trim() || bone.id.chars().any(char::is_control) {
            return Err(CharacterError::NonCanonicalBoneId {
                bone: bone.id.clone(),
            });
        }
        if let Some(source_identity) = &bone.source_identity
            && (source_identity.trim().is_empty()
                || source_identity != source_identity.trim()
                || source_identity.chars().any(char::is_control))
        {
            return Err(CharacterError::NonCanonicalBoneSourceIdentity {
                bone: bone.id.clone(),
                source_identity: source_identity.clone(),
            });
        }
        if let Some(parent) = &bone.parent_id {
            if parent.trim().is_empty()
                || parent != parent.trim()
                || parent.chars().any(char::is_control)
            {
                return Err(CharacterError::NonCanonicalParentId {
                    bone: bone.id.clone(),
                    parent: parent.clone(),
                });
            }
            if !seen.contains(parent) {
                return Err(CharacterError::ParentNotBeforeChild {
                    bone: bone.id.clone(),
                    parent: parent.clone(),
                });
            }
        }
        if let Some(component) =
            bone.rest_matrix.iter().position(|value| !value.is_finite())
        {
            return Err(CharacterError::NonFiniteRestMatrix {
                bone: bone.id.clone(),
                component,
            });
        }
        if !seen.insert(bone.id.clone()) {
            return Err(CharacterError::DuplicateBoneId {
                bone: bone.id.clone(),
            });
        }
    }
    Ok(seen)
}

/// Validate one skinned part against the shared skeleton contract.
fn validate_part(
    part: &SkinnedPart,
    bone_ids: &BTreeSet<String>,
) -> Result<(), CharacterError> {
    if part.group_influences.len() != part.mesh.groups.len() {
        return Err(CharacterError::InfluenceGroupCountMismatch {
            mesh: part.mesh.name.clone(),
            groups: part.mesh.groups.len(),
            influence_groups: part.group_influences.len(),
        });
    }
    for (group, influences) in
        part.mesh.groups.iter().zip(&part.group_influences)
    {
        validate_group_influences(
            &part.mesh.name,
            group.index,
            group.positions.len(),
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
    let mut weight_sums = vec![0f32; vertex_count];
    for influence in influences {
        if !bone_ids.contains(&influence.bone_id) {
            return Err(CharacterError::UnknownInfluenceBone {
                mesh: mesh_name.to_owned(),
                group: group_index,
                bone: influence.bone_id.clone(),
            });
        }
        let vertex = match usize::try_from(influence.vertex_index) {
            Ok(value) => value,
            Err(_conversion_error) => {
                return Err(CharacterError::InfluenceVertexOutOfBounds {
                    mesh: mesh_name.to_owned(),
                    group: group_index,
                    vertex: influence.vertex_index,
                    vertices: vertex_count,
                });
            },
        };
        let Some(sum) = weight_sums.get_mut(vertex) else {
            return Err(CharacterError::InfluenceVertexOutOfBounds {
                mesh: mesh_name.to_owned(),
                group: group_index,
                vertex: influence.vertex_index,
                vertices: vertex_count,
            });
        };
        if !influence.weight.is_finite()
            || influence.weight <= 0.
            || influence.weight > 1. + WEIGHT_SUM_TOLERANCE
        {
            return Err(CharacterError::InvalidInfluenceWeight {
                mesh: mesh_name.to_owned(),
                group: group_index,
                vertex: influence.vertex_index,
            });
        }
        *sum += influence.weight;
    }
    for (vertex, sum) in weight_sums.iter().enumerate() {
        if (sum - 1.).abs() > WEIGHT_SUM_TOLERANCE {
            return Err(CharacterError::UnnormalizedVertexWeights {
                mesh: mesh_name.to_owned(),
                group: group_index,
                vertex,
            });
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
    /// Authored source skeleton identity was empty or non-canonical.
    NonCanonicalSourceSkeletonIdentity {
        /// Malformed authored skeleton identity.
        identity: String,
    },
    /// Authored source composite identity was empty or non-canonical.
    NonCanonicalSourceCompositeIdentity {
        /// Malformed authored composite identity.
        identity: String,
    },
    /// Authored composite skin identity was empty or non-canonical.
    NonCanonicalSourceSkinIdentity {
        /// Malformed authored skin identity.
        identity: String,
    },
    /// Authored skin sort order was present but non-finite.
    NonFiniteSourceSkinSortOrder {
        /// Retained composite occurrence containing the skin.
        composite_ordinal: usize,
        /// Authored skin position inside the composite.
        skin_index: usize,
    },
    /// Source skin provenance referenced a composite occurrence outside
    /// retained source.
    SourceSkinCompositeOutOfBounds {
        /// Invalid retained composite occurrence.
        composite_ordinal: usize,
        /// Number of retained composite occurrences.
        composites: usize,
    },
    /// Source skin provenance repeated one authored composite skin position.
    DuplicateSourceCompositeSkinIndex {
        /// Retained composite occurrence containing the duplicate.
        composite_ordinal: usize,
        /// Repeated authored skin position.
        skin_index: usize,
    },
    /// Authored composite prop identity was empty or non-canonical.
    NonCanonicalSourcePropIdentity {
        /// Malformed authored prop identity.
        identity: String,
    },
    /// Authored prop sort order was present but non-finite.
    NonFiniteSourcePropSortOrder {
        /// Retained composite occurrence containing the prop.
        composite_ordinal: usize,
        /// Authored prop position inside the composite.
        prop_index: usize,
    },
    /// Source prop provenance referenced a composite occurrence outside
    /// retained source.
    SourcePropCompositeOutOfBounds {
        /// Invalid retained composite occurrence.
        composite_ordinal: usize,
        /// Number of retained composite occurrences.
        composites: usize,
    },
    /// Source prop provenance repeated one authored composite prop position.
    DuplicateSourceCompositePropIndex {
        /// Retained composite occurrence containing the duplicate.
        composite_ordinal: usize,
        /// Repeated authored prop position.
        prop_index: usize,
    },
    /// Character skeleton contained no bones.
    MissingBones,
    /// One bone identity was empty or whitespace-only.
    MissingBoneId,
    /// One bone identity carried surrounding whitespace.
    NonCanonicalBoneId {
        /// Non-canonical bone identity.
        bone: String,
    },
    /// One authored bone source identity was empty or non-canonical.
    NonCanonicalBoneSourceIdentity {
        /// Publication bone carrying malformed provenance.
        bone: String,
        /// Malformed authored identity.
        source_identity: String,
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
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../tests/formats/fbx/unit/domain/character/asset/loose_tests.rs"]
mod loose_tests;

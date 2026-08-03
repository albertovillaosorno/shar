// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
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

#![expect(
    clippy::module_name_repetitions,
    reason = "Tests verify these intentional explicit file-local contracts \
              remain safe."
)]

use std::collections::BTreeSet;

use super::error::MeshError;
use super::primitive_group::PrimitiveGroup;

/// Normalized mesh asset ready for scene construction.
#[derive(Clone, Debug, PartialEq)]
pub struct MeshAsset {
    /// Stable mesh name.
    pub name: String,
    /// Primitive groups exported as scene geometry parts.
    pub groups: Vec<PrimitiveGroup>,
}

impl MeshAsset {
    /// Create a normalized mesh asset.
    ///
    /// # Errors
    ///
    /// Returns an error when the mesh identity is empty or whitespace-only.
    pub fn new(
        name: impl Into<String>,
        mut groups: Vec<PrimitiveGroup>,
    ) -> Result<Self, MeshError> {
        let mesh_name = name.into();
        if mesh_name.trim().is_empty() {
            return Err(MeshError::MissingMeshName);
        }
        if mesh_name != mesh_name.trim()
            || mesh_name.chars().any(char::is_control)
        {
            return Err(MeshError::NonCanonicalMeshName);
        }
        if groups.is_empty() {
            return Err(MeshError::MissingPrimitiveGroups);
        }
        let mut group_indices = BTreeSet::new();
        for group in &groups {
            if !group_indices.insert(group.index) {
                return Err(MeshError::DuplicatePrimitiveGroupIndex {
                    index: group.index,
                });
            }
        }
        groups.sort_unstable_by_key(|group| group.index);
        Ok(Self { name: mesh_name, groups })
    }
}

// File:
//   - asset.rs
// Path:
//   - src/fbx/src/domain/mesh/asset.rs
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
//   - Pure fbx domain rules for domain mesh asset.
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
//   - Normalized mesh asset ready for scene construction.
// - Description:
//   - Defines asset data and behavior for fbx domain mesh.
// - Usage:
//   - Imported through crate domain facades or sibling domain modules.
// - Defaults:
//   - No filesystem paths, no external process calls, and no implicit IO
//   - defaults.
//
// ADRs:
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
// - docs/adr/pipeline/unreal/unreal-manifest-and-package-taxonomy.md
//
// Large file:
//   - false
//

//! Normalized mesh asset ready for scene construction.
//!
//! This boundary keeps normalized mesh asset ready for scene construction
//! explicit and returns deterministic results to fbx callers.
// These exact file-local lints preserve explicit domain and binary contracts.
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
        if mesh_name
            .trim()
            .is_empty()
        {
            return Err(MeshError::MissingMeshName);
        }
        if mesh_name != mesh_name.trim()
            || mesh_name
                .chars()
                .any(char::is_control)
        {
            return Err(MeshError::NonCanonicalMeshName);
        }
        if groups.is_empty() {
            return Err(MeshError::MissingPrimitiveGroups);
        }
        let mut group_indices = BTreeSet::new();
        for group in &groups {
            if !group_indices.insert(group.index) {
                return Err(
                    MeshError::DuplicatePrimitiveGroupIndex {
                        index: group.index,
                    },
                );
            }
        }
        groups.sort_unstable_by_key(|group| group.index);
        Ok(
            Self {
                name: mesh_name,
                groups,
            },
        )
    }
}

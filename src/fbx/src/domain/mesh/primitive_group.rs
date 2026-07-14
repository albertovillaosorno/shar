// File:
//   - primitive_group.rs
// Path:
//   - src/fbx/src/domain/mesh/primitive_group.rs
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
//   - Pure fbx domain rules for domain mesh primitive group.
// - Must-Not:
//   - Read files, parse generated indexes, invoke CLI code, or call writer
//   - adapters.
// - Allows:
//   - Value objects, invariant checks, and pure evidence-to-domain translation.
// - Split-When:
//   - Split when primitive group contains two independently testable
//   - contracts.
// - Merge-When:
//   - Another fbx module owns the same domain boundary with no distinct
//   - invariant.
// - Summary:
//   - Primitive group with positions, UVs, indices, and shader binding.
// - Description:
//   - Defines primitive group data and behavior for fbx domain mesh.
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

//! Primitive group with positions, UVs, indices, and shader binding.
// These exact file-local lints preserve explicit domain and binary contracts.
#![expect(
    clippy::missing_const_for_fn,
    reason = "Tests verify these intentional explicit file-local contracts \
              remain safe."
)]

use super::error::MeshError;
use super::topology::{align_triangle_winding, triangulate_indices};

/// Primitive group with positions, UVs, indices, and shader binding.
#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveGroup {
    /// Deterministic group index inside the mesh.
    pub index: usize,
    /// Shader name from the decoded mesh primitive group.
    pub shader: String,
    /// Vertex positions.
    pub positions: Vec<[f32; 3]>,
    /// Optional per-vertex normals aligned with positions.
    pub normals: Vec<[f32; 3]>,
    /// Optional primary UV channel.
    pub uvs: Vec<[f32; 2]>,
    /// Triangle indices after deterministic triangulation.
    pub triangles: Vec<[u32; 3]>,
}

/// Validate one shader identity before primitive construction.
fn validate_shader_identity(shader: &str) -> Result<(), MeshError> {
    if shader
        .trim()
        .is_empty()
    {
        return Err(MeshError::MissingShader);
    }
    if shader != shader.trim()
        || shader
            .chars()
            .any(char::is_control)
    {
        return Err(MeshError::NonCanonicalShader);
    }
    Ok(())
}

impl PrimitiveGroup {
    /// Create one primitive group from decoded arrays.
    ///
    /// # Errors
    ///
    /// Returns an error when UV counts do not match positions or indices cannot
    /// be normalized into triangles.
    pub fn new(
        index: usize,
        shader: impl Into<String>,
        positions: Vec<[f32; 3]>,
        uvs: Vec<[f32; 2]>,
        indices: &[u32],
    ) -> Result<Self, MeshError> {
        let shader_name = shader.into();
        validate_shader_identity(&shader_name)?;
        if positions.is_empty() {
            return Err(MeshError::MissingPositions);
        }
        if indices.is_empty() {
            return Err(MeshError::MissingIndices);
        }
        for (vertex, position) in positions
            .iter()
            .enumerate()
        {
            if let Some(axis) = position
                .iter()
                .position(|component| !component.is_finite())
            {
                return Err(
                    MeshError::NonFinitePosition {
                        vertex,
                        axis,
                    },
                );
            }
        }
        if !uvs.is_empty() && uvs.len() != positions.len() {
            return Err(
                MeshError::UvCountMismatch {
                    shader: shader_name,
                    positions: positions.len(),
                    uvs: uvs.len(),
                },
            );
        }
        for (vertex, uv) in uvs
            .iter()
            .enumerate()
        {
            if let Some(axis) = uv
                .iter()
                .position(|component| !component.is_finite())
            {
                return Err(
                    MeshError::NonFiniteUv {
                        vertex,
                        axis,
                    },
                );
            }
        }
        if let Some(&invalid_index) = indices
            .iter()
            .find(
                |&&candidate| {
                    usize::try_from(candidate).map_or(
                        true,
                        |position_index| position_index >= positions.len(),
                    )
                },
            )
        {
            return Err(
                MeshError::IndexOutOfBounds {
                    index: invalid_index,
                    positions: positions.len(),
                },
            );
        }
        let triangles = triangulate_indices(indices)?;
        if let Some((triangle, _)) = triangles
            .iter()
            .enumerate()
            .find(
                |(_, triangle)| {
                    triangle[0] == triangle[1]
                        || triangle[0] == triangle[2]
                        || triangle[1] == triangle[2]
                },
            )
        {
            return Err(
                MeshError::RepeatedTriangleVertex {
                    triangle,
                },
            );
        }
        Ok(
            Self {
                index,
                shader: shader_name,
                positions,
                normals: Vec::new(),
                uvs,
                triangles,
            },
        )
    }

    /// Attach per-vertex normals validated against the position count.
    ///
    /// # Errors
    ///
    /// Returns an error when the normal count differs from the position count
    /// or one normal component is not finite.
    pub fn with_normals(
        mut self,
        normals: Vec<[f32; 3]>,
    ) -> Result<Self, MeshError> {
        if normals.len()
            != self
                .positions
                .len()
        {
            return Err(
                MeshError::NormalCountMismatch {
                    shader: self.shader,
                    positions: self
                        .positions
                        .len(),
                    normals: normals.len(),
                },
            );
        }
        for (vertex, normal) in normals
            .iter()
            .enumerate()
        {
            if let Some(axis) = normal
                .iter()
                .position(|component| !component.is_finite())
            {
                return Err(
                    MeshError::NonFiniteNormal {
                        vertex,
                        axis,
                    },
                );
            }
        }
        self.normals = normals;
        align_triangle_winding(
            &self.positions,
            &self.normals,
            &mut self.triangles,
        );
        Ok(self)
    }

    /// Returns true when the group has usable UVs.
    #[must_use]
    pub fn has_uvs(&self) -> bool {
        !self
            .uvs
            .is_empty()
    }

    /// Returns true when the group has usable per-vertex normals.
    #[must_use]
    pub fn has_normals(&self) -> bool {
        !self
            .normals
            .is_empty()
    }
}

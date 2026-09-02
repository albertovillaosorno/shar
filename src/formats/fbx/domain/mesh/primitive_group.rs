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
//   - Primitive group domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Primitive group domain module.
// - Description:
//   - Implements the declared domain module responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Primitive group domain module.

#![expect(
    clippy::missing_const_for_fn,
    reason = "Tests verify these intentional explicit file-local contracts \
              remain safe."
)]

use super::error::MeshError;
use super::topology::triangulate_indices;

/// Primitive group with positions, UVs, indices, and shader binding.
#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveGroup {
    /// Deterministic group index inside the mesh.
    pub index: usize,
    /// Optional authored source identity for this exact primitive group.
    pub source_identity: Option<String>,
    /// Exact package-level source chunk ordinal retained as provenance.
    pub source_ordinal: Option<usize>,
    /// Shader name from the decoded mesh primitive group.
    pub shader: String,
    /// Vertex positions.
    pub positions: Vec<[f32; 3]>,
    /// Optional per-vertex normals aligned with positions.
    pub normals: Vec<[f32; 3]>,
    /// Optional normalized per-vertex colors in RGBA order.
    pub colors: Vec<[f32; 4]>,
    /// Optional primary UV channel.
    pub uvs: Vec<[f32; 2]>,
    /// Triangle indices after deterministic triangulation.
    pub triangles: Vec<[u32; 3]>,
}

/// Validate one shader identity before primitive construction.
fn validate_shader_identity(shader: &str) -> Result<(), MeshError> {
    if shader.trim().is_empty() {
        return Err(MeshError::MissingShader);
    }
    if shader != shader.trim() || shader.chars().any(char::is_control) {
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
        for (vertex, position) in positions.iter().enumerate() {
            if let Some(axis) =
                position.iter().position(|component| !component.is_finite())
            {
                return Err(MeshError::NonFinitePosition { vertex, axis });
            }
        }
        if !uvs.is_empty() && uvs.len() != positions.len() {
            return Err(MeshError::UvCountMismatch {
                shader: shader_name,
                positions: positions.len(),
                uvs: uvs.len(),
            });
        }
        for (vertex, uv) in uvs.iter().enumerate() {
            if let Some(axis) =
                uv.iter().position(|component| !component.is_finite())
            {
                return Err(MeshError::NonFiniteUv { vertex, axis });
            }
        }
        if let Some(&invalid_index) = indices.iter().find(|&&candidate| {
            usize::try_from(candidate).map_or(true, |position_index| {
                position_index >= positions.len()
            })
        }) {
            return Err(MeshError::IndexOutOfBounds {
                index: invalid_index,
                positions: positions.len(),
            });
        }
        let triangles = triangulate_indices(indices)?;
        if let Some((triangle, _)) =
            triangles.iter().enumerate().find(|(_, triangle)| {
                triangle[0] == triangle[1]
                    || triangle[0] == triangle[2]
                    || triangle[1] == triangle[2]
            })
        {
            return Err(MeshError::RepeatedTriangleVertex { triangle });
        }
        Ok(Self {
            index,
            source_identity: None,
            source_ordinal: None,
            shader: shader_name,
            positions,
            normals: Vec::new(),
            colors: Vec::new(),
            uvs,
            triangles,
        })
    }

    /// Attach one canonical authored identity to this source group.
    ///
    /// # Errors
    ///
    /// Returns an error when the identity is empty, padded, or contains control
    /// characters.
    pub fn with_source_identity(
        mut self,
        source_identity: impl Into<String>,
    ) -> Result<Self, MeshError> {
        let identity = source_identity.into();
        if identity.trim().is_empty()
            || identity != identity.trim()
            || identity.chars().any(char::is_control)
        {
            return Err(MeshError::NonCanonicalPrimitiveGroupIdentity {
                index: self.index,
            });
        }
        self.source_identity = Some(identity);
        Ok(self)
    }

    /// Attach the exact package-level source chunk ordinal as provenance.
    #[must_use]
    pub const fn with_source_ordinal(mut self, source_ordinal: usize) -> Self {
        self.source_ordinal = Some(source_ordinal);
        self
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
        if normals.len() != self.positions.len() {
            return Err(MeshError::NormalCountMismatch {
                shader: self.shader,
                positions: self.positions.len(),
                normals: normals.len(),
            });
        }
        for (vertex, normal) in normals.iter().enumerate() {
            if let Some(axis) =
                normal.iter().position(|component| !component.is_finite())
            {
                return Err(MeshError::NonFiniteNormal { vertex, axis });
            }
        }
        self.normals = normals;
        Ok(self)
    }

    /// Attach normalized per-vertex colors validated against position count.
    ///
    /// # Errors
    ///
    /// Returns an error when the color count differs from the position count or
    /// one RGBA component is not finite.
    pub fn with_colors(
        mut self,
        colors: Vec<[f32; 4]>,
    ) -> Result<Self, MeshError> {
        if colors.len() != self.positions.len() {
            return Err(MeshError::ColorCountMismatch {
                shader: self.shader,
                positions: self.positions.len(),
                colors: colors.len(),
            });
        }
        for (vertex, color) in colors.iter().enumerate() {
            if let Some(axis) =
                color.iter().position(|component| !component.is_finite())
            {
                return Err(MeshError::NonFiniteColor { vertex, axis });
            }
        }
        self.colors = colors;
        Ok(self)
    }

    /// Returns true when the group has usable UVs.
    #[must_use]
    pub fn has_uvs(&self) -> bool {
        !self.uvs.is_empty()
    }

    /// Returns true when the group has usable per-vertex normals.
    #[must_use]
    pub fn has_normals(&self) -> bool {
        !self.normals.is_empty()
    }

    /// Returns true when the group has usable normalized per-vertex colors.
    #[must_use]
    pub fn has_colors(&self) -> bool {
        !self.colors.is_empty()
    }
}

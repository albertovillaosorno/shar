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
//   - Error domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Error domain module.
// - Description:
//   - Implements the declared domain module responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Error domain module.

#![expect(
    clippy::module_name_repetitions,
    reason = "Tests verify these intentional explicit file-local contracts \
              remain safe."
)]

/// Mesh validation and translation failure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MeshError {
    /// JSON schema was not a decoded mesh.
    UnsupportedSchema(String),
    /// UV channel length does not match positions.
    UvCountMismatch {
        /// Shader using the mismatched group.
        shader: String,
        /// Position count.
        positions: usize,
        /// UV count.
        uvs: usize,
    },
    /// Mesh aggregate reused a primitive-group index.
    DuplicatePrimitiveGroupIndex {
        /// Repeated group index.
        index: usize,
    },
    /// Mesh aggregate did not contain any primitive groups.
    MissingPrimitiveGroups,
    /// Mesh aggregate did not provide a usable identity.
    MissingMeshName,
    /// Mesh identity carried surrounding whitespace.
    NonCanonicalMeshName,
    /// Authored mesh source identity was blank, padded, or contained control
    /// characters.
    NonCanonicalMeshSourceIdentity,
    /// Primitive group did not provide any face indices.
    MissingIndices,
    /// Primitive group did not provide any vertex positions.
    MissingPositions,
    /// Primitive group did not provide a usable shader identity.
    MissingShader,
    /// Primitive-group shader identity carried surrounding whitespace.
    NonCanonicalShader,
    /// Primitive-group source identity was blank, padded, or contained control
    /// characters.
    NonCanonicalPrimitiveGroupIdentity {
        /// Primitive-group index carrying the invalid source identity.
        index: usize,
    },
    /// One position component was not finite.
    NonFinitePosition {
        /// Vertex containing the invalid component.
        vertex: usize,
        /// Component axis inside the vertex.
        axis: usize,
    },
    /// One UV component was not finite.
    NonFiniteUv {
        /// UV coordinate containing the invalid component.
        vertex: usize,
        /// Component axis inside the UV coordinate.
        axis: usize,
    },
    /// Normal count did not match the position count.
    NormalCountMismatch {
        /// Shader using the mismatched group.
        shader: String,
        /// Position count.
        positions: usize,
        /// Normal count.
        normals: usize,
    },
    /// One normal component was not finite.
    NonFiniteNormal {
        /// Normal containing the invalid component.
        vertex: usize,
        /// Component axis inside the normal.
        axis: usize,
    },
    /// Vertex-color count did not match the position count.
    ColorCountMismatch {
        /// Shader using the mismatched group.
        shader: String,
        /// Position count.
        positions: usize,
        /// Vertex-color count.
        colors: usize,
    },
    /// One normalized vertex-color component was not finite.
    NonFiniteColor {
        /// Color containing the invalid component.
        vertex: usize,
        /// Component axis inside RGBA.
        axis: usize,
    },
    /// One index referenced a vertex outside the position array.
    IndexOutOfBounds {
        /// Invalid vertex index.
        index: u32,
        /// Number of available positions.
        positions: usize,
    },
    /// One triangle reused a vertex index and could not form a face.
    RepeatedTriangleVertex {
        /// Triangle position inside the primitive group.
        triangle: usize,
    },
    /// Index list did not describe triangles or one quad.
    UnsupportedIndexCount(usize),
}

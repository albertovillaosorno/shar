// File:
//   - error.rs
// Path:
//   - src/fbx/src/domain/mesh/error.rs
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
//   - Pure fbx domain rules for domain mesh error.
// - Must-Not:
//   - Read files, parse generated indexes, invoke CLI code, or call writer
//   - adapters.
// - Allows:
//   - Value objects, invariant checks, and pure evidence-to-domain translation.
// - Split-When:
//   - Split when error contains two independently testable contracts.
// - Merge-When:
//   - Another fbx module owns the same domain boundary with no distinct
//   - invariant.
// - Summary:
//   - Mesh topology validation error.
// - Description:
//   - Defines error data and behavior for fbx domain mesh.
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

//! Mesh topology validation error.
//!
//! This boundary keeps mesh topology validation error explicit and returns
//! deterministic results to fbx callers.
// These exact file-local lints preserve explicit domain and binary contracts.
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
    /// Primitive group did not provide any face indices.
    MissingIndices,
    /// Primitive group did not provide any vertex positions.
    MissingPositions,
    /// Primitive group did not provide a usable shader identity.
    MissingShader,
    /// Primitive-group shader identity carried surrounding whitespace.
    NonCanonicalShader,
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

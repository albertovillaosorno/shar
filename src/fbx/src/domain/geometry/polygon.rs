// File:
//   - polygon.rs
// Path:
//   - src/fbx/src/domain/geometry/polygon.rs
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
//   - Pure fbx domain rules for domain geometry polygon.
// - Must-Not:
//   - Read files, parse generated indexes, invoke CLI code, or call writer
//   - adapters.
// - Allows:
//   - Value objects, invariant checks, and pure evidence-to-domain translation.
// - Split-When:
//   - Split when polygon contains two independently testable contracts.
// - Merge-When:
//   - Another fbx module owns the same domain boundary with no distinct
//   - invariant.
// - Summary:
//   - Polygon with explicit corner indices.
// - Description:
//   - Defines polygon data and behavior for fbx domain geometry.
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

//! Polygon with explicit corner indices.
//!
//! This boundary keeps polygon with explicit corner indices explicit and
//! returns deterministic results to fbx callers.
/// Polygon index sequence for one geometry face.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Polygon {
    /// Vertex indices for this polygon.
    pub vertex_indices: Vec<u32>,
    /// Material slot selected for this polygon.
    pub material_slot: Option<usize>,
}

// File:
//   - payload.rs
// Path:
//   - src/fbx/src/domain/geometry/payload.rs
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
//   - Pure fbx domain rules for domain geometry payload.
// - Must-Not:
//   - Read files, parse generated indexes, invoke CLI code, or call writer
//   - adapters.
// - Allows:
//   - Value objects, invariant checks, and pure evidence-to-domain translation.
// - Split-When:
//   - Split when payload contains two independently testable contracts.
// - Merge-When:
//   - Another fbx module owns the same domain boundary with no distinct
//   - invariant.
// - Summary:
//   - Geometry payload attached to a scene node.
// - Description:
//   - Defines payload data and behavior for fbx domain geometry.
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

//! Geometry payload attached to a scene node.
//!
//! This boundary keeps geometry payload attached to a scene node explicit and
//! returns deterministic results to fbx callers.
use super::polygon::Polygon;
use crate::domain::surface::{ColorLayer, NormalLayer, UvLayer};

/// Geometry payload attached to a scene node.
#[derive(Clone, Debug, PartialEq)]
pub struct Geometry {
    /// Stable geometry id.
    pub id: String,
    /// Mesh vertices in domain coordinate space.
    pub vertices: Vec<[f32; 3]>,
    /// Polygon vertex indices in domain order.
    pub polygons: Vec<Polygon>,
    /// Optional normal layer owned per polygon corner.
    pub normals: Option<NormalLayer>,
    /// Optional UV layers owned per polygon corner.
    pub uv_layers: Vec<UvLayer>,
    /// Optional color layers owned per polygon corner.
    pub color_layers: Vec<ColorLayer>,
}

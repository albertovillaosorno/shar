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
//   - Payload domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Payload domain module.
// - Description:
//   - Implements the declared domain module responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Payload domain module.

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

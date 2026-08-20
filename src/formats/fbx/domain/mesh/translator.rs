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
//   - Translator domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Translator domain module.
// - Description:
//   - Implements the declared domain module responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Translator domain module.

use super::asset::MeshAsset;
use crate::domain::geometry::{Geometry, Polygon};
use crate::domain::surface::UvLayer;

/// Convert resolved mesh evidence into domain geometry.
#[must_use]
pub fn mesh_asset_to_geometry(mesh: &MeshAsset) -> Vec<Geometry> {
    mesh.groups
        .iter()
        .map(|group| Geometry {
            id: format!("{}-geometry-{}", mesh.name, group.index),
            vertices: group.positions.clone(),
            polygons: group
                .triangles
                .iter()
                .map(|triangle| Polygon {
                    vertex_indices: triangle.to_vec(),
                    material_slot: Some(group.index),
                })
                .collect(),
            normals: None,
            uv_layers: polygon_corner_uv_layer(group).into_iter().collect(),
            color_layers: Vec::new(),
        })
        .collect()
}

/// Translate per-vertex UV evidence into polygon-corner order.
fn polygon_corner_uv_layer(
    group: &super::primitive_group::PrimitiveGroup,
) -> Option<UvLayer> {
    if group.uvs.is_empty() {
        return None;
    }
    let values = group
        .triangles
        .iter()
        .flat_map(|triangle| triangle.iter())
        .map(|&index| {
            usize::try_from(index)
                .ok()
                .and_then(|vertex| group.uvs.get(vertex))
                .copied()
        })
        .collect::<Option<Vec<_>>>()?;
    Some(UvLayer {
        name: "UVChannel_1".to_owned(),
        values,
    })
}

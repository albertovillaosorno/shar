// File:
//   - translator.rs
// Path:
//   - src/fbx/src/domain/mesh/translator.rs
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
//   - Pure fbx domain rules for domain mesh translator.
// - Must-Not:
//   - Read files, parse generated indexes, invoke CLI code, or call writer
//   - adapters.
// - Allows:
//   - Value objects, invariant checks, and pure evidence-to-domain translation.
// - Split-When:
//   - Split when translator contains two independently testable contracts.
// - Merge-When:
//   - Another fbx module owns the same domain boundary with no distinct
//   - invariant.
// - Summary:
//   - Convert resolved mesh evidence into domain geometry.
// - Description:
//   - Defines translator data and behavior for fbx domain mesh.
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

//! Convert resolved mesh evidence into domain geometry.
//!
//! This boundary keeps convert resolved mesh evidence into domain geometry
//! explicit and returns deterministic results to fbx callers.
use super::asset::MeshAsset;
use crate::domain::geometry::{Geometry, Polygon};
use crate::domain::surface::UvLayer;

/// Convert resolved mesh evidence into domain geometry.
#[must_use]
pub fn mesh_asset_to_geometry(mesh: &MeshAsset) -> Vec<Geometry> {
    mesh.groups
        .iter()
        .map(
            |group| Geometry {
                id: format!(
                    "{}-geometry-{}",
                    mesh.name, group.index
                ),
                vertices: group
                    .positions
                    .clone(),
                polygons: group
                    .triangles
                    .iter()
                    .map(
                        |triangle| Polygon {
                            vertex_indices: triangle.to_vec(),
                            material_slot: Some(group.index),
                        },
                    )
                    .collect(),
                normals: None,
                uv_layers: polygon_corner_uv_layer(group)
                    .into_iter()
                    .collect(),
                color_layers: Vec::new(),
            },
        )
        .collect()
}

/// Translate per-vertex UV evidence into polygon-corner order.
fn polygon_corner_uv_layer(
    group: &super::primitive_group::PrimitiveGroup
) -> Option<UvLayer> {
    if group
        .uvs
        .is_empty()
    {
        return None;
    }
    let values = group
        .triangles
        .iter()
        .flat_map(|triangle| triangle.iter())
        .map(
            |&index| {
                usize::try_from(index)
                    .ok()
                    .and_then(
                        |vertex| {
                            group
                                .uvs
                                .get(vertex)
                        },
                    )
                    .copied()
            },
        )
        .collect::<Option<Vec<_>>>()?;
    Some(
        UvLayer {
            name: "UVChannel_1".to_owned(),
            values,
        },
    )
}

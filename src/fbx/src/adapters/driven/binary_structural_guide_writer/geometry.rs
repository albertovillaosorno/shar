// File:
//   - geometry.rs
// Path:
//   - src/fbx/src/adapters/driven/binary_structural_guide_writer/geometry.rs
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
//   - One structural-guide geometry node and its four UV layers.
// - Must-Not:
//   - Change coordinates, normals, winding, atlas assignments, or material
//     ownership.
// - Allows:
//   - Optional per-polygon-vertex source normals, four UV layers, and one
//     all-same material.
// - Summary:
//   - Encodes the already validated single structural-guide mesh.
//
// LARGE-FILE:
// - owner: Structural-guide FBX adapter
// - reason: Position, topology, normal, UV, and layer metadata must remain
//   visibly aligned in one geometry serializer.
// - split: Scene objects and top-level document nodes remain separate.
// - validation: Four-UV import tests and manifest parity checks.
// - review: Split if non-triangle topology is ever supported.
//

//! Structural-guide geometry and layer serialization.

use super::super::binary_fbx::{BinaryNode, BinaryProperty};
use super::nodes::{
    GEOMETRY_ID, i32_node, id_property, layer_element, name_class, string,
    string_node,
};
use super::{
    STRUCTURAL_GUIDE_ASSET_NAME, STRUCTURAL_GUIDE_UV_NAMES,
    StructuralGuideFbxError, StructuralGuideMesh,
};

pub(super) fn geometry_node(
    mesh: &StructuralGuideMesh,
) -> Result<BinaryNode, StructuralGuideFbxError> {
    let positions = mesh
        .positions
        .iter()
        .flat_map(
            |position| {
                position
                    .iter()
                    .copied()
                    .map(f64::from)
            },
        )
        .collect();
    let polygon_indices = mesh
        .triangles
        .iter()
        .flat_map(
            |triangle| {
                [
                    i64::from(triangle[0]),
                    i64::from(triangle[1]),
                    -i64::from(triangle[2]) - 1,
                ]
            },
        )
        .map(
            |value| {
                i32::try_from(value).map_err(
                    |error| {
                        StructuralGuideFbxError::Encoding(
                            format!("polygon index conversion failed: {error}"),
                        )
                    },
                )
            },
        )
        .collect::<Result<Vec<_>, _>>()?;
    let mut children = vec![
        BinaryNode::leaf(
            "Vertices",
            vec![BinaryProperty::F64Array(positions)],
        ),
        BinaryNode::leaf(
            "PolygonVertexIndex",
            vec![BinaryProperty::I32Array(polygon_indices)],
        ),
        i32_node(
            "GeometryVersion",
            124,
        ),
    ];
    if !mesh
        .normals
        .is_empty()
    {
        children.push(normal_layer(mesh)?);
    }
    for (typed_index, (name, values)) in STRUCTURAL_GUIDE_UV_NAMES
        .into_iter()
        .zip(
            [
                mesh.atlas_uvs
                    .as_slice(),
                mesh.source_uvs
                    .as_slice(),
                mesh.atlas_offsets
                    .as_slice(),
                mesh.atlas_scales
                    .as_slice(),
            ],
        )
        .enumerate()
    {
        children.push(
            uv_layer(
                typed_index,
                name,
                values,
                &mesh.triangles,
            )?,
        );
    }
    children.push(material_layer());
    children.extend(
        layer_nodes(
            !mesh
                .normals
                .is_empty(),
        ),
    );
    Ok(
        BinaryNode::new(
            "Geometry",
            vec![
                id_property(GEOMETRY_ID)?,
                name_class(
                    STRUCTURAL_GUIDE_ASSET_NAME,
                    "Geometry",
                ),
                string("Mesh"),
            ],
            children,
        ),
    )
}

fn normal_layer(
    mesh: &StructuralGuideMesh,
) -> Result<BinaryNode, StructuralGuideFbxError> {
    let normals = corner_values3(
        &mesh.normals,
        &mesh.triangles,
        "normal",
    )?
    .into_iter()
    .flat_map(
        |normal| {
            normal
                .into_iter()
                .map(f64::from)
        },
    )
    .collect();
    Ok(
        BinaryNode::new(
            "LayerElementNormal",
            vec![BinaryProperty::I32(0)],
            vec![
                i32_node(
                    "Version", 101,
                ),
                string_node(
                    "Name", "",
                ),
                string_node(
                    "MappingInformationType",
                    "ByPolygonVertex",
                ),
                string_node(
                    "ReferenceInformationType",
                    "Direct",
                ),
                BinaryNode::leaf(
                    "Normals",
                    vec![BinaryProperty::F64Array(normals)],
                ),
            ],
        ),
    )
}

fn uv_layer(
    typed_index: usize,
    name: &str,
    values: &[[f32; 2]],
    triangles: &[[u32; 3]],
) -> Result<BinaryNode, StructuralGuideFbxError> {
    let coordinates = corner_values2(
        values, triangles, name,
    )?
    .into_iter()
    .flat_map(
        |uv| {
            uv.into_iter()
                .map(f64::from)
        },
    )
    .collect();
    let index = i32::try_from(typed_index).map_err(
        |error| {
            StructuralGuideFbxError::Encoding(
                format!("UV typed index conversion failed: {error}"),
            )
        },
    )?;
    Ok(
        BinaryNode::new(
            "LayerElementUV",
            vec![BinaryProperty::I32(index)],
            vec![
                i32_node(
                    "Version", 101,
                ),
                string_node(
                    "Name", name,
                ),
                string_node(
                    "MappingInformationType",
                    "ByPolygonVertex",
                ),
                string_node(
                    "ReferenceInformationType",
                    "Direct",
                ),
                BinaryNode::leaf(
                    "UV",
                    vec![BinaryProperty::F64Array(coordinates)],
                ),
            ],
        ),
    )
}

fn material_layer() -> BinaryNode {
    BinaryNode::new(
        "LayerElementMaterial",
        vec![BinaryProperty::I32(0)],
        vec![
            i32_node(
                "Version", 101,
            ),
            string_node(
                "Name", "",
            ),
            string_node(
                "MappingInformationType",
                "AllSame",
            ),
            string_node(
                "ReferenceInformationType",
                "IndexToDirect",
            ),
            BinaryNode::leaf(
                "Materials",
                vec![BinaryProperty::I32Array(vec![0])],
            ),
        ],
    )
}

fn layer_nodes(include_normals: bool) -> Vec<BinaryNode> {
    let mut primary = vec![
        i32_node(
            "Version", 100,
        ),
    ];
    if include_normals {
        primary.push(
            layer_element(
                "LayerElementNormal",
                0,
            ),
        );
    }
    primary.extend(
        [
            layer_element(
                "LayerElementUV",
                0,
            ),
            layer_element(
                "LayerElementMaterial",
                0,
            ),
        ],
    );
    let mut result = vec![
        BinaryNode::new(
            "Layer",
            vec![BinaryProperty::I32(0)],
            primary,
        ),
    ];
    for index in 1_i32..4_i32 {
        result.push(
            BinaryNode::new(
                "Layer",
                vec![BinaryProperty::I32(index)],
                vec![
                    i32_node(
                        "Version", 100,
                    ),
                    layer_element(
                        "LayerElementUV",
                        index,
                    ),
                ],
            ),
        );
    }
    result
}

fn corner_values3(
    values: &[[f32; 3]],
    triangles: &[[u32; 3]],
    channel: &'static str,
) -> Result<Vec<[f32; 3]>, StructuralGuideFbxError> {
    triangles
        .iter()
        .flat_map(
            |triangle| {
                triangle
                    .iter()
                    .copied()
            },
        )
        .map(
            |index| {
                usize::try_from(index)
                    .ok()
                    .and_then(|vertex| values.get(vertex))
                    .copied()
                    .ok_or(
                        StructuralGuideFbxError::IndexOutOfBounds {
                            index,
                            positions: values.len(),
                        },
                    )
            },
        )
        .collect::<Result<Vec<_>, _>>()
        .map_err(
            |error| match error {
                StructuralGuideFbxError::IndexOutOfBounds {
                    index,
                    positions,
                } => StructuralGuideFbxError::Encoding(
                    format!(
                        "{channel} corner index {index} exceeds {positions} \
                         values"
                    ),
                ),
                other => other,
            },
        )
}

fn corner_values2(
    values: &[[f32; 2]],
    triangles: &[[u32; 3]],
    channel: &str,
) -> Result<Vec<[f32; 2]>, StructuralGuideFbxError> {
    triangles
        .iter()
        .flat_map(
            |triangle| {
                triangle
                    .iter()
                    .copied()
            },
        )
        .map(
            |index| {
                usize::try_from(index)
                    .ok()
                    .and_then(|vertex| values.get(vertex))
                    .copied()
                    .ok_or_else(
                        || {
                            StructuralGuideFbxError::Encoding(
                                format!(
                                    "{channel} corner index {index} exceeds \
                                     {} values",
                                    values.len()
                                ),
                            )
                        },
                    )
            },
        )
        .collect()
}

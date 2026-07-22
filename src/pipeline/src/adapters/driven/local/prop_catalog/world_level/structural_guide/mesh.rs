// File:
//   - mesh.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/prop_catalog/world_level/
//     structural_guide/mesh.rs
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
//   - Exact-triangle deduplication and final Unreal-space guide mesh assembly.
// - Must-Not:
//   - Pack atlas pixels, alter source UV values, or serialize files.
// - Allows:
//   - Positive-determinant axis conversion, centimeter scaling, XY centering,
//     sea-level normalization, explicit normals, and loop duplication.
// - Summary:
//   - Builds one triangulated four-UV structural-guide mesh.
//
// LARGE-FILE:
// - owner: Structural-guide mesh assembly
// - reason: Triangle ownership, coordinate conversion, channel projection, and
//   coverage evidence must remain one auditable transformation.
// - split: Atlas and manifest publication remain separate modules.
// - validation: Geometry count, bounds, winding, UV, and Wasp coverage tests.
// - review: Split when a second guide coordinate policy appears.
//

//! Canonical structural-guide mesh assembly.

use std::collections::BTreeSet;

use fbx::adapters::driven::binary_structural_guide_writer::StructuralGuideMesh;
use fbx::domain::mesh::PrimitiveGroup;

use super::super::export::MasterContent;
use super::atlas::surface_key;
use super::model::{AtlasBuild, GuidePlacement, GuideSourceCounts};
use crate::domain::PipelineError;

/// Final source-space sea plane after the shared world-height algorithm.
const SEA_LEVEL_SOURCE_Y_METERS: f32 =
    super::super::movement::WORLD_HEIGHT_OFFSET_METERS;
/// Source world units are meters; Unreal guide coordinates are centimeters.
const CENTIMETERS_PER_METER: f32 = 100.0;
/// Landscape half-extent requested by the Unreal structural-guide contract.
const LANDSCAPE_HALF_EXTENT_CM: f32 = 201_600.0;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct PositionKey([u32; 3]);

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct TriangleKey([PositionKey; 3]);

/// Build one fully baked, centered, triangulated guide mesh.
pub(super) fn build(
    content: &MasterContent,
    atlas: &AtlasBuild,
) -> Result<
    (
        StructuralGuideMesh,
        GuideSourceCounts,
        GuidePlacement,
    ),
    PipelineError,
> {
    let mut meshes = content
        .meshes
        .clone();
    meshes.sort_by(
        |left, right| {
            left.name
                .cmp(&right.name)
        },
    );
    let mut counts = GuideSourceCounts {
        input_meshes: meshes.len(),
        ..GuideSourceCounts::default()
    };
    for mesh in &meshes {
        let normalized = mesh
            .name
            .to_ascii_lowercase();
        if normalized.contains("wasp") {
            counts.wasp_meshes = counts
                .wasp_meshes
                .saturating_add(1);
        }
        if is_prop_like(&normalized) {
            counts.prop_like_meshes = counts
                .prop_like_meshes
                .saturating_add(1);
        }
    }
    if counts.wasp_meshes == 0 {
        return Err(
            PipelineError::new(
                "structural-guide source omitted all Wasp Camera geometry",
            ),
        );
    }
    let mut result = StructuralGuideMesh {
        positions: Vec::new(),
        normals: Vec::new(),
        triangles: Vec::new(),
        source_uvs: Vec::new(),
        atlas_offsets: Vec::new(),
        atlas_scales: Vec::new(),
        atlas_flags: Vec::new(),
    };
    let mut owned = BTreeSet::<TriangleKey>::new();
    for mesh in &meshes {
        let mut groups = mesh
            .groups
            .iter()
            .collect::<Vec<_>>();
        groups.sort_by_key(|group| group.index);
        for group in groups {
            counts.input_groups = counts
                .input_groups
                .saturating_add(1);
            counts.input_triangles = counts
                .input_triangles
                .checked_add(
                    group
                        .triangles
                        .len(),
                )
                .ok_or_else(
                    || PipelineError::new("guide triangle count overflowed"),
                )?;
            append_group(
                group,
                atlas,
                &mut owned,
                &mut result,
                &mut counts,
            )?;
        }
    }
    if result
        .triangles
        .is_empty()
    {
        return Err(PipelineError::new("structural-guide mesh is empty"));
    }
    let horizontal_center = center_horizontal(&mut result.positions)?;
    validate_bounds(&result.positions)?;
    let placement = GuidePlacement {
        source_to_unreal_matrix_row_major: source_to_unreal_matrix(
            horizontal_center,
        ),
    };
    Ok(
        (
            result, counts, placement,
        ),
    )
}

fn append_group(
    group: &PrimitiveGroup,
    atlas: &AtlasBuild,
    owned: &mut BTreeSet<TriangleKey>,
    result: &mut StructuralGuideMesh,
    counts: &mut GuideSourceCounts,
) -> Result<(), PipelineError> {
    for triangle in &group.triangles {
        let key = surface_key(group);
        let assignment = atlas
            .assignments
            .get(&key)
            .copied()
            .ok_or_else(
                || {
                    PipelineError::new(
                        format!(
                            "structural-guide atlas assignment is missing: {}",
                            group.shader
                        ),
                    )
                },
            )?;
        let source_positions = triangle_values3(
            &group.positions,
            *triangle,
            "position",
        )?;
        let transformed = source_positions.map(to_unreal_centimeters);
        let fallback_normal = match face_normal(transformed) {
            Ok(normal) => normal,
            Err(_) => {
                counts.removed_degenerate_triangles = counts
                    .removed_degenerate_triangles
                    .checked_add(1)
                    .ok_or_else(
                        || {
                            PipelineError::new(
                                "guide degenerate triangle count overflowed",
                            )
                        },
                    )?;
                continue;
            }
        };
        let triangle_key = TriangleKey::new(transformed);
        if !owned.insert(triangle_key) {
            counts.removed_duplicate_triangles = counts
                .removed_duplicate_triangles
                .checked_add(1)
                .ok_or_else(
                    || PipelineError::new("guide duplicate count overflowed"),
                )?;
            continue;
        }
        if assignment.approximated_vertex_color {
            counts.approximated_vertex_color_triangles = counts
                .approximated_vertex_color_triangles
                .checked_add(1)
                .ok_or_else(
                    || {
                        PipelineError::new(
                            "guide approximated vertex-color count overflowed",
                        )
                    },
                )?;
        }
        let source_uvs = if group
            .uvs
            .is_empty()
        {
            [[0.0_f32; 2]; 3]
        } else {
            triangle_values2(
                &group.uvs, *triangle, "UV",
            )?
        };
        let mut repaired_normal = false;
        let normals = if group
            .normals
            .is_empty()
        {
            [fallback_normal; 3]
        } else {
            let values = triangle_values3(
                &group.normals,
                *triangle,
                "normal",
            )?;
            values.map(
                |value| match normalize(to_unreal_normal(value)) {
                    Ok(normal) => normal,
                    Err(_) => {
                        repaired_normal = true;
                        fallback_normal
                    }
                },
            )
        };
        if repaired_normal {
            counts.repaired_normal_triangles = counts
                .repaired_normal_triangles
                .checked_add(1)
                .ok_or_else(
                    || {
                        PipelineError::new(
                            "guide repaired normal count overflowed",
                        )
                    },
                )?;
        }
        let first = u32::try_from(
            result
                .positions
                .len(),
        )
        .map_err(|error| PipelineError::new(error.to_string()))?;
        for corner in 0..3 {
            result
                .positions
                .push(transformed[corner]);
            result
                .normals
                .push(normals[corner]);
            result
                .source_uvs
                .push(source_uvs[corner]);
            result
                .atlas_offsets
                .push(assignment.offset);
            result
                .atlas_scales
                .push(assignment.scale);
            result
                .atlas_flags
                .push(
                    [
                        assignment.repeat,
                        0.0,
                    ],
                );
        }
        result
            .triangles
            .push(
                [
                    first,
                    first
                        .checked_add(1)
                        .ok_or_else(
                            || PipelineError::new("guide index overflowed"),
                        )?,
                    first
                        .checked_add(2)
                        .ok_or_else(
                            || PipelineError::new("guide index overflowed"),
                        )?,
                ],
            );
    }
    Ok(())
}

impl TriangleKey {
    fn new(positions: [[f32; 3]; 3]) -> Self {
        let mut vertices = positions.map(PositionKey::new);
        vertices.sort_unstable();
        Self(vertices)
    }
}

impl PositionKey {
    fn new(position: [f32; 3]) -> Self {
        Self(position.map(normalized_bits))
    }
}

fn normalized_bits(value: f32) -> u32 {
    if value == 0.0 {
        0
    } else {
        value.to_bits()
    }
}

fn to_unreal_centimeters(position: [f32; 3]) -> [f32; 3] {
    [
        -position[2] * CENTIMETERS_PER_METER,
        -position[0] * CENTIMETERS_PER_METER,
        (position[1] - SEA_LEVEL_SOURCE_Y_METERS) * CENTIMETERS_PER_METER,
    ]
}

fn to_unreal_normal(normal: [f32; 3]) -> [f32; 3] {
    [
        -normal[2], -normal[0], normal[1],
    ]
}

fn normalize(value: [f32; 3]) -> Result<[f32; 3], PipelineError> {
    let length_squared = value[0].mul_add(
        value[0],
        value[1].mul_add(
            value[1],
            value[2] * value[2],
        ),
    );
    let length = length_squared.sqrt();
    if !length.is_finite() || length <= f32::EPSILON {
        return Err(
            PipelineError::new("structural-guide normal is degenerate"),
        );
    }
    Ok(
        [
            value[0] / length,
            value[1] / length,
            value[2] / length,
        ],
    )
}

fn face_normal(positions: [[f32; 3]; 3]) -> Result<[f32; 3], PipelineError> {
    let first = subtract(
        positions[1],
        positions[0],
    );
    let second = subtract(
        positions[2],
        positions[0],
    );
    normalize(
        [
            first[1].mul_add(
                second[2],
                -(first[2] * second[1]),
            ),
            first[2].mul_add(
                second[0],
                -(first[0] * second[2]),
            ),
            first[0].mul_add(
                second[1],
                -(first[1] * second[0]),
            ),
        ],
    )
}

fn subtract(
    left: [f32; 3],
    right: [f32; 3],
) -> [f32; 3] {
    [
        left[0] - right[0],
        left[1] - right[1],
        left[2] - right[2],
    ]
}

fn triangle_values3(
    values: &[[f32; 3]],
    triangle: [u32; 3],
    channel: &str,
) -> Result<[[f32; 3]; 3], PipelineError> {
    Ok(
        [
            value3(
                values,
                triangle[0],
                channel,
            )?,
            value3(
                values,
                triangle[1],
                channel,
            )?,
            value3(
                values,
                triangle[2],
                channel,
            )?,
        ],
    )
}

fn triangle_values2(
    values: &[[f32; 2]],
    triangle: [u32; 3],
    channel: &str,
) -> Result<[[f32; 2]; 3], PipelineError> {
    Ok(
        [
            value2(
                values,
                triangle[0],
                channel,
            )?,
            value2(
                values,
                triangle[1],
                channel,
            )?,
            value2(
                values,
                triangle[2],
                channel,
            )?,
        ],
    )
}

fn value3(
    values: &[[f32; 3]],
    index: u32,
    channel: &str,
) -> Result<[f32; 3], PipelineError> {
    usize::try_from(index)
        .ok()
        .and_then(|vertex| values.get(vertex))
        .copied()
        .ok_or_else(
            || {
                PipelineError::new(
                    format!(
                        "structural-guide {channel} index is invalid: {index}"
                    ),
                )
            },
        )
}

fn value2(
    values: &[[f32; 2]],
    index: u32,
    channel: &str,
) -> Result<[f32; 2], PipelineError> {
    usize::try_from(index)
        .ok()
        .and_then(|vertex| values.get(vertex))
        .copied()
        .ok_or_else(
            || {
                PipelineError::new(
                    format!(
                        "structural-guide {channel} index is invalid: {index}"
                    ),
                )
            },
        )
}

fn center_horizontal(
    positions: &mut [[f32; 3]]
) -> Result<[f32; 2], PipelineError> {
    let (low, high) = bounds(positions)?;
    let center = [
        (low[0] + high[0]) * 0.5,
        (low[1] + high[1]) * 0.5,
    ];
    for position in positions {
        position[0] -= center[0];
        position[1] -= center[1];
    }
    Ok(center)
}

fn source_to_unreal_matrix(horizontal_center: [f32; 2]) -> [f32; 16] {
    [
        0.0,
        -CENTIMETERS_PER_METER,
        0.0,
        0.0,
        0.0,
        0.0,
        CENTIMETERS_PER_METER,
        0.0,
        -CENTIMETERS_PER_METER,
        0.0,
        0.0,
        0.0,
        -horizontal_center[0],
        -horizontal_center[1],
        -SEA_LEVEL_SOURCE_Y_METERS * CENTIMETERS_PER_METER,
        1.0,
    ]
}

fn validate_bounds(positions: &[[f32; 3]]) -> Result<(), PipelineError> {
    let (low, high) = bounds(positions)?;
    let center = [
        (low[0] + high[0]) * 0.5,
        (low[1] + high[1]) * 0.5,
    ];
    if center
        .iter()
        .any(|value| value.abs() > 0.01)
    {
        return Err(
            PipelineError::new(
                format!(
                    "structural-guide horizontal center is not zero: {},{}",
                    center[0], center[1]
                ),
            ),
        );
    }
    if low[0] < -LANDSCAPE_HALF_EXTENT_CM
        || high[0] > LANDSCAPE_HALF_EXTENT_CM
        || low[1] < -LANDSCAPE_HALF_EXTENT_CM
        || high[1] > LANDSCAPE_HALF_EXTENT_CM
    {
        return Err(
            PipelineError::new(
                format!(
                    "structural-guide exceeds Landscape bounds: \
                     [{},{}]-[{},{}]",
                    low[0], low[1], high[0], high[1]
                ),
            ),
        );
    }
    if !(low[2] <= 0.0 && high[2] >= 0.0) {
        return Err(
            PipelineError::new(
                format!(
                    "structural-guide sea plane does not cross Z=0: {}..{}",
                    low[2], high[2]
                ),
            ),
        );
    }
    Ok(())
}

fn bounds(
    positions: &[[f32; 3]]
) -> Result<
    (
        [f32; 3],
        [f32; 3],
    ),
    PipelineError,
> {
    let Some(first) = positions
        .first()
        .copied()
    else {
        return Err(PipelineError::new("structural-guide bounds are empty"));
    };
    let mut low = first;
    let mut high = first;
    for position in positions
        .iter()
        .skip(1)
    {
        for axis in 0..3 {
            low[axis] = low[axis].min(position[axis]);
            high[axis] = high[axis].max(position[axis]);
        }
    }
    Ok(
        (
            low, high,
        ),
    )
}

fn is_prop_like(name: &str) -> bool {
    [
        "wasp", "prop", "coin", "crate", "box", "tree", "sign", "phone",
        "vending", "gag", "door", "race", "vehicle",
    ]
    .iter()
    .any(|token| name.contains(token))
}

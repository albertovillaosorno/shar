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
//   - Lossless concatenation of evaluated world-FBX geometry already expressed
//     under the shared world ReflectX root, plus final atlas UV channels.
// - Must-Not:
//   - Move, center, mirror, scale, deduplicate, repair, or otherwise
//     reinterpret source world-FBX geometry.
// - Allows:
//   - Deterministic mesh/group ordering and per-loop duplication required by
//     the one-mesh structural-guide writer.
// - Summary:
//   - Combines the exact post-world-export mesh set into one atlas-backed mesh.
//
// LARGE-FILE:
// - owner: Structural-guide mesh concatenation
// - reason: Geometry preservation and atlas-channel projection must remain one
//   auditable pass.
// - split: Atlas and manifest publication remain separate modules.
// - validation: Position, winding, UV, bounds, and source-count tests.
// - review: Split when another combined-FBX profile appears.
//

//! Lossless structural-guide mesh concatenation.

use fbx::adapters::driven::binary_structural_guide_writer::StructuralGuideMesh;
use fbx::domain::mesh::PrimitiveGroup;

use super::super::export::MasterContent;
use super::atlas::surface_key;
use super::model::{AtlasBuild, GuideSourceCounts};
use crate::domain::PipelineError;

/// Canonical source-space world-height datum already owned by normal FBXs.
const WORLD_HEIGHT_METERS: f32 =
    super::super::movement::WORLD_HEIGHT_OFFSET_METERS;
/// World half-extent represented by the 4,032-meter Landscape contract.
const WORLD_HALF_EXTENT_METERS: f32 = 2_016.0;

/// Concatenate the evaluated source-FBX geometry without further spatial
/// changes.
pub(super) fn build(
    content: &MasterContent,
    atlas: &AtlasBuild,
) -> Result<
    (
        StructuralGuideMesh,
        GuideSourceCounts,
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
    let groups_without_normals = meshes
        .iter()
        .flat_map(
            |mesh| {
                mesh.groups
                    .iter()
            },
        )
        .filter(
            |group| {
                group
                    .normals
                    .is_empty()
            },
        )
        .count();
    let include_normals = groups_without_normals == 0;
    let mut counts = GuideSourceCounts {
        input_meshes: meshes.len(),
        groups_without_normals,
        ..GuideSourceCounts::default()
    };
    let mut result = StructuralGuideMesh {
        positions: Vec::new(),
        normals: Vec::new(),
        triangles: Vec::new(),
        atlas_uvs: Vec::new(),
        source_uvs: Vec::new(),
        atlas_offsets: Vec::new(),
        atlas_scales: Vec::new(),
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
                include_normals,
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
    validate_world_fbx_bounds(&result.positions)?;
    Ok(
        (
            result, counts,
        ),
    )
}

fn append_group(
    group: &PrimitiveGroup,
    atlas: &AtlasBuild,
    include_normals: bool,
    result: &mut StructuralGuideMesh,
    counts: &mut GuideSourceCounts,
) -> Result<(), PipelineError> {
    let assignment = atlas
        .assignments
        .get(&surface_key(group))
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
    for triangle in &group.triangles {
        let positions = triangle_values3(
            &group.positions,
            *triangle,
            "position",
        )?;
        let normals = if include_normals {
            Some(
                triangle_values3(
                    &group.normals,
                    *triangle,
                    "normal",
                )?,
            )
        } else {
            None
        };
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
        let first = u32::try_from(
            result
                .positions
                .len(),
        )
        .map_err(|error| PipelineError::new(error.to_string()))?;
        for corner in 0..3 {
            result
                .positions
                .push(positions[corner]);
            if let Some(normals) = normals {
                result
                    .normals
                    .push(normals[corner]);
            }
            result
                .atlas_uvs
                .push(
                    atlas_uv(
                        source_uvs[corner],
                        assignment,
                    ),
                );
            result
                .source_uvs
                .push(source_uvs[corner]);
            result
                .atlas_offsets
                .push(assignment.offset);
            result
                .atlas_scales
                .push(assignment.scale);
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

fn atlas_uv(
    source: [f32; 2],
    assignment: super::model::AtlasAssignment,
) -> [f32; 2] {
    let normalized = if assignment.repeat >= 0.5 {
        source.map(|component| component.rem_euclid(1.0))
    } else {
        source.map(
            |component| {
                component.clamp(
                    0.0, 1.0,
                )
            },
        )
    };
    [
        normalized[0].mul_add(
            assignment.scale[0],
            assignment.offset[0],
        ),
        normalized[1].mul_add(
            assignment.scale[1],
            assignment.offset[1],
        ),
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

fn validate_world_fbx_bounds(
    positions: &[[f32; 3]],
) -> Result<(), PipelineError> {
    let (low, high) = bounds(positions)?;
    if low[0] < -WORLD_HALF_EXTENT_METERS
        || high[0] > WORLD_HALF_EXTENT_METERS
        || low[2] < -WORLD_HALF_EXTENT_METERS
        || high[2] > WORLD_HALF_EXTENT_METERS
    {
        return Err(
            PipelineError::new(
                format!(
                    "combined world FBX exceeds Landscape bounds: \
                     [{},{}]-[{},{}]",
                    low[0], low[2], high[0], high[2]
                ),
            ),
        );
    }
    if !(low[1] <= WORLD_HEIGHT_METERS && high[1] >= WORLD_HEIGHT_METERS) {
        return Err(
            PipelineError::new(
                format!(
                    "combined world FBX does not retain the 80-meter datum: \
                     {}..{}",
                    low[1], high[1]
                ),
            ),
        );
    }
    Ok(())
}

fn bounds(
    positions: &[[f32; 3]],
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
            if !position[axis].is_finite() {
                return Err(
                    PipelineError::new(
                        "combined world FBX position is non-finite",
                    ),
                );
            }
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

#[cfg(test)]
mod tests {
    use super::super::model::AtlasAssignment;
    use super::{WORLD_HEIGHT_METERS, atlas_uv, validate_world_fbx_bounds};

    fn assignment(repeat: f32) -> AtlasAssignment {
        AtlasAssignment {
            offset: [
                0.1, 0.2,
            ],
            scale: [
                0.3, 0.4,
            ],
            repeat,
            approximated_vertex_color: false,
        }
    }

    #[test]
    fn imported_uv_zero_bakes_repeating_atlas_mapping() {
        let mapped = atlas_uv(
            [
                -0.25, 1.25,
            ],
            assignment(1.0),
        );
        assert!((mapped[0] - 0.325).abs() <= f32::EPSILON);
        assert!((mapped[1] - 0.3).abs() <= f32::EPSILON);
    }

    #[test]
    fn imported_uv_zero_clamps_to_its_atlas_tile() {
        let mapped = atlas_uv(
            [
                -0.5, 1.5,
            ],
            assignment(0.0),
        );
        assert!((mapped[0] - 0.1).abs() <= f32::EPSILON);
        assert!((mapped[1] - 0.6).abs() <= f32::EPSILON);
    }

    #[test]
    fn world_height_is_validated_without_being_modified() -> Result<(), String>
    {
        let positions = [
            [
                -10.0, 79.0, -20.0,
            ],
            [
                10.0,
                WORLD_HEIGHT_METERS,
                20.0,
            ],
            [
                0.0, 81.0, 0.0,
            ],
        ];
        validate_world_fbx_bounds(&positions)
            .map_err(|error| error.to_string())?;
        assert_eq!(
            positions[1],
            [
                10.0, 80.0, 20.0,
            ]
        );
        Ok(())
    }
}

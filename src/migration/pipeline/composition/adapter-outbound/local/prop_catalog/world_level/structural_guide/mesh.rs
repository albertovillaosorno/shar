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
//   - Mesh outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Mesh outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Mesh outbound adapter.

#![expect(
    clippy::indexing_slicing,
    // jig-ignore-next-line: exact syntax is indivisible
    reason = "Triangle and fixed-axis indices are validated against source channels before mesh assembly."
)]

use fbx::adapters::driven::binary_structural_guide_writer::StructuralGuideMesh;
use fbx::domain::mesh::PrimitiveGroup;

use super::super::export::MasterContent;
use super::atlas::surface_key;
use super::model::{AtlasBuild, GuideSourceCounts};
use crate::domain::PipelineError;

/// Concatenate the evaluated source-FBX geometry without further spatial
/// changes.
pub(super) fn build(
    content: &MasterContent,
    atlas: &AtlasBuild,
) -> Result<(StructuralGuideMesh, GuideSourceCounts), PipelineError> {
    let mut meshes = content.meshes.clone();
    meshes.sort_by(|left, right| left.name.cmp(&right.name));
    let groups_without_normals = meshes
        .iter()
        .flat_map(|mesh| mesh.groups.iter())
        .filter(|group| group.normals.is_empty())
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
        let normalized = mesh.name.to_ascii_lowercase();
        if normalized.contains("wasp") {
            counts.wasp_meshes = counts.wasp_meshes.saturating_add(1);
        }
        if is_prop_like(&normalized) {
            counts.prop_like_meshes = counts.prop_like_meshes.saturating_add(1);
        }
        let mut groups = mesh.groups.iter().collect::<Vec<_>>();
        groups.sort_by_key(|group| group.index);
        for group in groups {
            counts.input_groups = counts.input_groups.saturating_add(1);
            counts.input_triangles = counts
                .input_triangles
                .checked_add(group.triangles.len())
                .ok_or_else(|| {
                    PipelineError::new("guide triangle count overflowed")
                })?;
            append_group(
                group,
                atlas,
                include_normals,
                &mut result,
                &mut counts,
            )?;
        }
    }
    if result.triangles.is_empty() {
        return Err(PipelineError::new("structural-guide mesh is empty"));
    }
    validate_world_fbx_bounds(&result.positions)?;
    Ok((result, counts))
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
        .ok_or_else(|| {
            PipelineError::new(format!(
                "structural-guide atlas assignment is missing: {}",
                group.shader
            ))
        })?;
    for triangle in &group.triangles {
        let positions =
            triangle_values3(&group.positions, *triangle, "position")?;
        let normals = if include_normals {
            Some(triangle_values3(&group.normals, *triangle, "normal")?)
        } else {
            None
        };
        let source_uvs = if group.uvs.is_empty() {
            [[0f32; 2]; 3]
        } else {
            triangle_values2(&group.uvs, *triangle, "UV")?
        };
        if assignment.approximated_vertex_color {
            counts.approximated_vertex_color_triangles = counts
                .approximated_vertex_color_triangles
                .checked_add(1)
                .ok_or_else(|| {
                    PipelineError::new(
                        "guide approximated vertex-color count overflowed",
                    )
                })?;
        }
        let first = u32::try_from(result.positions.len())
            .map_err(|error| PipelineError::new(error.to_string()))?;
        for corner in 0..3 {
            result.positions.push(positions[corner]);
            if let Some(normals) = normals {
                result.normals.push(normals[corner]);
            }
            result
                .atlas_uvs
                .push(atlas_uv(source_uvs[corner], assignment));
            result.source_uvs.push(source_uvs[corner]);
            result.atlas_offsets.push(assignment.offset);
            result.atlas_scales.push(assignment.scale);
        }
        result.triangles.push([
            first,
            first
                .checked_add(1)
                .ok_or_else(|| PipelineError::new("guide index overflowed"))?,
            first
                .checked_add(2)
                .ok_or_else(|| PipelineError::new("guide index overflowed"))?,
        ]);
    }
    Ok(())
}

fn atlas_uv(
    source: [f32; 2],
    assignment: super::model::AtlasAssignment,
) -> [f32; 2] {
    let normalized = if assignment.repeat >= 0.5 {
        source.map(|component| component.rem_euclid(1.))
    } else {
        source.map(|component| component.clamp(0., 1.))
    };
    [
        normalized[0].mul_add(assignment.scale[0], assignment.offset[0]),
        normalized[1].mul_add(assignment.scale[1], assignment.offset[1]),
    ]
}

fn triangle_values3(
    values: &[[f32; 3]],
    triangle: [u32; 3],
    channel: &str,
) -> Result<[[f32; 3]; 3], PipelineError> {
    Ok([
        value3(values, triangle[0], channel)?,
        value3(values, triangle[1], channel)?,
        value3(values, triangle[2], channel)?,
    ])
}

fn triangle_values2(
    values: &[[f32; 2]],
    triangle: [u32; 3],
    channel: &str,
) -> Result<[[f32; 2]; 3], PipelineError> {
    Ok([
        value2(values, triangle[0], channel)?,
        value2(values, triangle[1], channel)?,
        value2(values, triangle[2], channel)?,
    ])
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
        .ok_or_else(|| {
            PipelineError::new(format!(
                "structural-guide {channel} index is invalid: {index}"
            ))
        })
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
        .ok_or_else(|| {
            PipelineError::new(format!(
                "structural-guide {channel} index is invalid: {index}"
            ))
        })
}

fn validate_world_fbx_bounds(
    positions: &[[f32; 3]],
) -> Result<(), PipelineError> {
    let (low, high) = bounds(positions)?;
    if low
        .into_iter()
        .zip(high)
        .any(|(minimum, maximum)| minimum > maximum)
    {
        return Err(PipelineError::new(
            "combined world FBX bounds are inverted",
        ));
    }
    Ok(())
}

fn bounds(
    positions: &[[f32; 3]],
) -> Result<([f32; 3], [f32; 3]), PipelineError> {
    let Some(first) = positions.first().copied() else {
        return Err(PipelineError::new("structural-guide bounds are empty"));
    };
    if first.iter().any(|component| !component.is_finite()) {
        return Err(PipelineError::new(
            "combined world FBX position is non-finite",
        ));
    }
    let mut low = first;
    let mut high = first;
    for position in positions.iter().skip(1) {
        for axis in 0..3 {
            if !position[axis].is_finite() {
                return Err(PipelineError::new(
                    "combined world FBX position is non-finite",
                ));
            }
            low[axis] = low[axis].min(position[axis]);
            high[axis] = high[axis].max(position[axis]);
        }
    }
    Ok((low, high))
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
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/prop_catalog/world_level/structural_guide/mesh/tests.rs"]
mod tests;

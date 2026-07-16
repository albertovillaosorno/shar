// File:
//   - projection.rs
// Path:
//   - src/fbx/src/domain/texture/semantic/body/charts/projection.rs
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
//   - Deterministic non-degenerate orthographic projection selection for one
//   - connected flat-color chart.
// - Must-Not:
//   - Change topology, split vertices, pack atlas rectangles, or rasterize.
// - Allows:
//   - XY, XZ, and YZ projection scoring by minimum and total triangle area.
// - Split-When:
//   - A non-orthographic unwrap becomes an independently validated algorithm.
// - Merge-When:
//   - Chart discovery becomes the sole owner of projection selection.
// - Summary:
//   - Non-degenerate chart projection.
// - Description:
//   - Chooses the strongest projection only when every source triangle
//   - survives.
// - Usage:
//   - Called once for each connected semantic chart.
// - Defaults:
//   - Equal scores retain fixed XY, XZ, then YZ preference.
//
// ADRs:
// - docs/adr/fbx/export/character-semantic-texture-rig-and-outfit-contract.md
//
// Large file:
//   - true
//   - Reason: projection scoring, bounds, and checked source lookup form one
//   - complete geometric acceptance contract.
//

//! Deterministic non-degenerate orthographic chart projection.
#![expect(
    clippy::shadow_reuse,
    clippy::unneeded_field_pattern,
    unused_results,
    reason = "Private projection candidates intentionally reuse geometric \
              terms and discard replaced map values."
)]

use std::collections::BTreeMap;

use super::super::super::color::Rgba8;
use super::super::super::image::RgbaImageError;
use super::super::super::region::BodyRegion;
use super::super::super::sampling::TextureAddressMode;
use super::super::error::SemanticTextureError;
use super::super::recipe::GroupAddress;
use super::super::types::ProjectionAxis;
use super::model::{ProjectedChart, ProjectionBounds};
use crate::domain::mesh::PrimitiveGroup;

/// Minimum accepted doubled projected triangle area.
const MINIMUM_PROJECTED_AREA: f32 = 1.0e-10;

/// Immutable chart-projection ownership and sampling request.
pub(super) struct ProjectionRequest {
    /// Stable chart identity carried into the projection result.
    pub(super) id: String,
    /// Source primitive-group address being projected.
    pub(super) address: GroupAddress,
    /// Semantic body region assigned to the chart.
    pub(super) region: BodyRegion,
    /// Authored source color used by flat-color charts.
    pub(super) source_color: Rgba8,
    /// Triangles that must preserve source-texture sampling.
    pub(super) source_sampled_triangles: Vec<usize>,
    /// Source texture addressing policy used for UV normalization.
    pub(super) address_mode: TextureAddressMode,
}

/// Project one connected chart through source UVs or orthographic axes.
pub(super) fn project(
    request: ProjectionRequest,
    group: &PrimitiveGroup,
    triangle_indices: Vec<usize>,
    vertex_indices: Vec<usize>,
) -> Result<ProjectedChart, SemanticTextureError> {
    if !request
        .source_sampled_triangles
        .is_empty()
    {
        return source_uv_projection(
            request,
            group,
            triangle_indices,
            vertex_indices,
        );
    }
    let ProjectionRequest {
        id,
        address,
        region,
        source_color,
        source_sampled_triangles,
        address_mode: _,
    } = request;
    let mut best: Option<Candidate> = None;
    for axis in ProjectionAxis::ALL {
        let candidate = candidate(
            axis,
            group,
            &triangle_indices,
            &vertex_indices,
        )?;
        if candidate.minimum_area <= MINIMUM_PROJECTED_AREA {
            continue;
        }
        if best
            .as_ref()
            .is_none_or(
                |current| {
                    stronger(
                        &candidate, current,
                    )
                },
            )
        {
            best = Some(candidate);
        }
    }
    let Some(best) = best else {
        return Err(SemanticTextureError::DegenerateChartProjection(id));
    };
    Ok(
        ProjectedChart {
            id,
            group: address,
            region,
            source_color,
            sample_source: false,
            source_sampled_triangles,
            triangle_indices,
            vertex_indices,
            projection: best.axis,
            projected_positions: best.positions,
            bounds: best.bounds,
        },
    )
}

/// Preserve one patterned chart through its original source UV coordinates.
fn source_uv_projection(
    request: ProjectionRequest,
    group: &PrimitiveGroup,
    triangle_indices: Vec<usize>,
    vertex_indices: Vec<usize>,
) -> Result<ProjectedChart, SemanticTextureError> {
    let ProjectionRequest {
        id,
        address,
        region,
        source_color,
        source_sampled_triangles,
        address_mode,
    } = request;
    let mut positions = BTreeMap::new();
    let mut minimum = [f32::INFINITY; 2];
    let mut maximum = [f32::NEG_INFINITY; 2];
    for vertex in &vertex_indices {
        let uv = group
            .uvs
            .get(*vertex)
            .copied()
            .ok_or(SemanticTextureError::NumericOverflow)?;
        let uv = normalize_uv(
            uv,
            address_mode,
        )?;
        minimum[0] = minimum[0].min(uv[0]);
        minimum[1] = minimum[1].min(uv[1]);
        maximum[0] = maximum[0].max(uv[0]);
        maximum[1] = maximum[1].max(uv[1]);
        positions.insert(
            *vertex, uv,
        );
    }
    let bounds = ProjectionBounds {
        minimum,
        maximum,
    };
    if bounds.width() <= 0.0 || bounds.height() <= 0.0 {
        return Err(SemanticTextureError::DegenerateChartProjection(id));
    }
    Ok(
        ProjectedChart {
            id,
            group: address,
            region,
            source_color,
            sample_source: true,
            source_sampled_triangles,
            triangle_indices,
            vertex_indices,
            projection: ProjectionAxis::SourceUv,
            projected_positions: positions,
            bounds,
        },
    )
}

/// Normalize one finite UV coordinate through the declared address mode.
fn normalize_uv(
    uv: [f32; 2],
    address_mode: TextureAddressMode,
) -> Result<[f32; 2], SemanticTextureError> {
    if !uv[0].is_finite() || !uv[1].is_finite() {
        return Err(SemanticTextureError::Image(RgbaImageError::InvalidUv));
    }
    match address_mode {
        TextureAddressMode::Clamp => {
            if !(0.0..=1.0).contains(&uv[0]) || !(0.0..=1.0).contains(&uv[1]) {
                return Err(
                    SemanticTextureError::Image(RgbaImageError::InvalidUv),
                );
            }
            Ok(uv)
        }
        TextureAddressMode::Tile => Ok(
            [
                uv[0].rem_euclid(1.0),
                uv[1].rem_euclid(1.0),
            ],
        ),
    }
}

/// One valid projection candidate and its geometric score.
struct Candidate {
    /// Orthographic axis represented by this candidate.
    axis: ProjectionAxis,
    /// Projected position for every candidate vertex.
    positions: BTreeMap<usize, [f32; 2]>,
    /// Candidate bounds used to normalize and score the chart.
    bounds: ProjectionBounds,
    /// Smallest doubled triangle area in the candidate projection.
    minimum_area: f32,
    /// Sum of doubled triangle areas in the candidate projection.
    total_area: f32,
}

/// Build one candidate or mark it degenerate through a zero minimum area.
fn candidate(
    axis: ProjectionAxis,
    group: &PrimitiveGroup,
    triangle_indices: &[usize],
    vertex_indices: &[usize],
) -> Result<Candidate, SemanticTextureError> {
    let mut positions = BTreeMap::new();
    let mut minimum = [f32::INFINITY; 2];
    let mut maximum = [f32::NEG_INFINITY; 2];
    for vertex in vertex_indices {
        let position = group
            .positions
            .get(*vertex)
            .copied()
            .ok_or(SemanticTextureError::NumericOverflow)?;
        let projected = axis.project(position);
        minimum[0] = minimum[0].min(projected[0]);
        minimum[1] = minimum[1].min(projected[1]);
        maximum[0] = maximum[0].max(projected[0]);
        maximum[1] = maximum[1].max(projected[1]);
        positions.insert(
            *vertex, projected,
        );
    }
    let mut minimum_area = f32::INFINITY;
    let mut total_area = 0.0_f32;
    for triangle in triangle_indices {
        let indices = group
            .triangles
            .get(*triangle)
            .ok_or(SemanticTextureError::NumericOverflow)?;
        let points = [
            projected(
                &positions, indices[0],
            )?,
            projected(
                &positions, indices[1],
            )?,
            projected(
                &positions, indices[2],
            )?,
        ];
        let area = doubled_area(points);
        minimum_area = minimum_area.min(area);
        total_area += area;
    }
    Ok(
        Candidate {
            axis,
            positions,
            bounds: ProjectionBounds {
                minimum,
                maximum,
            },
            minimum_area,
            total_area,
        },
    )
}

/// Resolve one projected vertex.
fn projected(
    positions: &BTreeMap<usize, [f32; 2]>,
    vertex: u32,
) -> Result<[f32; 2], SemanticTextureError> {
    let vertex = usize::try_from(vertex)
        .map_err(|_error| SemanticTextureError::NumericOverflow)?;
    positions
        .get(&vertex)
        .copied()
        .ok_or(SemanticTextureError::NumericOverflow)
}

/// Return the doubled absolute area of one projected triangle.
fn doubled_area(points: [[f32; 2]; 3]) -> f32 {
    let first = [
        points[1][0] - points[0][0],
        points[1][1] - points[0][1],
    ];
    let second = [
        points[2][0] - points[0][0],
        points[2][1] - points[0][1],
    ];
    first[0]
        .mul_add(
            second[1],
            -(first[1] * second[0]),
        )
        .abs()
}

/// Compare valid projections without replacing an exact fixed-order tie.
fn stronger(
    candidate: &Candidate,
    current: &Candidate,
) -> bool {
    candidate
        .minimum_area
        .total_cmp(&current.minimum_area)
        .then_with(
            || {
                candidate
                    .total_area
                    .total_cmp(&current.total_area)
            },
        )
        .is_gt()
}

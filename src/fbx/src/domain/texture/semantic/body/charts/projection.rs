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
use std::collections::BTreeMap;

use super::super::super::color::Rgba8;
use super::super::super::region::BodyRegion;
use super::super::error::SemanticTextureError;
use super::super::recipe::GroupAddress;
use super::super::types::ProjectionAxis;
use super::model::{ProjectedChart, ProjectionBounds};
use crate::domain::mesh::PrimitiveGroup;

/// Minimum accepted doubled projected triangle area.
const MINIMUM_PROJECTED_AREA: f32 = 1.0e-10;

/// Project one connected chart through the strongest valid orthographic axes.
pub(super) fn project(
    id: String,
    address: GroupAddress,
    region: BodyRegion,
    source_color: Rgba8,
    group: &PrimitiveGroup,
    triangle_indices: Vec<usize>,
    vertex_indices: Vec<usize>,
) -> Result<ProjectedChart, SemanticTextureError> {
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
            triangle_indices,
            vertex_indices,
            projection: best.axis,
            projected_positions: best.positions,
            bounds: best.bounds,
        },
    )
}

/// One valid projection candidate and its geometric score.
struct Candidate {
    axis: ProjectionAxis,
    positions: BTreeMap<usize, [f32; 2]>,
    bounds: ProjectionBounds,
    minimum_area: f32,
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

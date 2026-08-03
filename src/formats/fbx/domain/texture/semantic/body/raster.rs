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
//   - Raster domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Raster domain module.
// - Description:
//   - Implements the declared domain module responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Raster domain module.

#![expect(
    clippy::shadow_reuse,
    reason = "Raster stages intentionally refine validated coordinate and \
              dimension terms."
)]

use super::super::image::RgbaImage;
use super::charts::model::PlacedChart;
use super::error::SemanticTextureError;
use crate::domain::mesh::PrimitiveGroup;

#[path = "raster/dilation.rs"]
mod dilation;
#[path = "raster/triangle.rs"]
mod triangle;
#[path = "raster/triangle_source.rs"]
mod triangle_source;

/// Rasterize one placed chart and dilate its color inside its reserved cell.
pub(super) fn rasterize(
    atlas: &mut RgbaImage,
    coverage: &mut [bool],
    source_texture: &RgbaImage,
    group: &PrimitiveGroup,
    chart: &PlacedChart,
    padding: u32,
) -> Result<(), SemanticTextureError> {
    if chart.public.sample_source {
        let covered =
            triangle_source::paint(atlas, coverage, source_texture, chart)?;
        if covered == 0 {
            return Err(SemanticTextureError::EmptyRasterizedChart(
                chart.public.id.clone(),
            ));
        }
        return dilation::apply(atlas, coverage, chart, padding);
    }
    let mut covered = 0_usize;
    for triangle_index in &chart.public.triangle_indices {
        let indices = group
            .triangles
            .get(*triangle_index)
            .ok_or(SemanticTextureError::NumericOverflow)?;
        let points = [
            point(chart, indices[0])?,
            point(chart, indices[1])?,
            point(chart, indices[2])?,
        ];
        let painted = triangle::paint(atlas, coverage, chart, points)?;
        covered = covered
            .checked_add(painted)
            .ok_or(SemanticTextureError::NumericOverflow)?;
    }
    if covered == 0 {
        return Err(SemanticTextureError::EmptyRasterizedChart(
            chart.public.id.clone(),
        ));
    }
    dilation::apply(atlas, coverage, chart, padding)
}

/// Resolve one mapped destination vertex.
fn point(
    chart: &PlacedChart,
    vertex: u32,
) -> Result<[f32; 2], SemanticTextureError> {
    let vertex = usize::try_from(vertex)
        .map_err(|_error| SemanticTextureError::NumericOverflow)?;
    chart
        .pixel_positions
        .get(&vertex)
        .copied()
        .ok_or(SemanticTextureError::NumericOverflow)
}

/// Resolve one checked row-major coverage index.
pub(super) fn coverage_index(
    width: u32,
    x: u32,
    y: u32,
) -> Result<usize, SemanticTextureError> {
    let width = usize::try_from(width)
        .map_err(|_error| SemanticTextureError::NumericOverflow)?;
    let x = usize::try_from(x)
        .map_err(|_error| SemanticTextureError::NumericOverflow)?;
    let y = usize::try_from(y)
        .map_err(|_error| SemanticTextureError::NumericOverflow)?;
    y.checked_mul(width)
        .and_then(|offset| offset.checked_add(x))
        .ok_or(SemanticTextureError::NumericOverflow)
}

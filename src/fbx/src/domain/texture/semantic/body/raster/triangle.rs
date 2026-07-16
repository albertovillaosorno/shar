// File:
//   - triangle.rs
// Path:
//   - src/fbx/src/domain/texture/semantic/body/raster/triangle.rs
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
//   - Pixel-center barycentric coverage for one placed flat-color triangle.
// - Must-Not:
//   - Dilate edges, select colors, map UVs, or leave the reserved chart cell.
// - Allows:
//   - Checked destination bounds and exact source-color writes.
// - Split-When:
//   - Patterned interpolation requires a distinct raster contract.
// - Merge-When:
//   - The raster facade becomes the sole owner of triangle coverage.
// - Summary:
//   - Flat-color triangle pixel coverage.
// - Description:
//   - Paints every covered pixel center inside the placed triangle bounds.
// - Usage:
//   - Called for every triangle in one placed chart.
// - Defaults:
//   - Triangle edges are included through a small deterministic tolerance.
//
// ADRs:
// - docs/adr/fbx/export/character-semantic-texture-rig-and-outfit-contract.md
//
// Large file:
//   - true
//   - Reason: checked bounds, barycentric coverage, pixel writes, and coverage
//   - accounting form one raster behavior.
//

//! Deterministic flat-color triangle pixel coverage.
#![expect(
    clippy::as_conversions,
    clippy::indexing_slicing,
    reason = "Validated raster bounds make pixel casts and triangle indexing \
              deterministic."
)]

use super::super::super::image::RgbaImage;
use super::super::charts::model::PlacedChart;
use super::super::error::SemanticTextureError;
use super::coverage_index;

/// Paint one projected triangle through pixel-center barycentric coverage.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    reason = "Triangle coordinates are finite and bounded inside one checked \
              non-negative chart cell before floor, ceiling, or pixel-center \
              conversion."
)]
pub(super) fn paint(
    atlas: &mut RgbaImage,
    coverage: &mut [bool],
    chart: &PlacedChart,
    points: [[f32; 2]; 3],
) -> Result<usize, SemanticTextureError> {
    let left = points
        .iter()
        .map(|point| point[0])
        .fold(
            f32::INFINITY,
            f32::min,
        )
        .floor() as u32;
    let right = points
        .iter()
        .map(|point| point[0])
        .fold(
            f32::NEG_INFINITY,
            f32::max,
        )
        .ceil() as u32;
    let top = points
        .iter()
        .map(|point| point[1])
        .fold(
            f32::INFINITY,
            f32::min,
        )
        .floor() as u32;
    let bottom = points
        .iter()
        .map(|point| point[1])
        .fold(
            f32::NEG_INFINITY,
            f32::max,
        )
        .ceil() as u32;
    let cell_right = chart
        .public
        .cell
        .right()
        .ok_or(SemanticTextureError::NumericOverflow)?;
    let cell_bottom = chart
        .public
        .cell
        .bottom()
        .ok_or(SemanticTextureError::NumericOverflow)?;
    let mut painted = 0_usize;
    for y in top.max(
        chart
            .public
            .cell
            .y,
    )..=bottom.min(cell_bottom)
    {
        for x in left.max(
            chart
                .public
                .cell
                .x,
        )..=right.min(cell_right)
        {
            let sample = [
                x as f32 + 0.5,
                y as f32 + 0.5,
            ];
            if !inside_triangle(
                sample, points,
            ) {
                continue;
            }
            let index = coverage_index(
                atlas.width(),
                x,
                y,
            )?;
            if !coverage[index] {
                painted = painted
                    .checked_add(1)
                    .ok_or(SemanticTextureError::NumericOverflow)?;
            }
            coverage[index] = true;
            atlas.set_pixel(
                x,
                y,
                chart
                    .public
                    .source_color,
            )?;
        }
    }
    Ok(painted)
}

/// Return whether one pixel center lies inside or on a triangle.
fn inside_triangle(
    point: [f32; 2],
    triangle: [[f32; 2]; 3],
) -> bool {
    let edges = [
        edge(
            triangle[0],
            triangle[1],
            point,
        ),
        edge(
            triangle[1],
            triangle[2],
            point,
        ),
        edge(
            triangle[2],
            triangle[0],
            point,
        ),
    ];
    let has_negative = edges
        .iter()
        .any(|value| *value < -1.0e-5);
    let has_positive = edges
        .iter()
        .any(|value| *value > 1.0e-5);
    !(has_negative && has_positive)
}

/// Return one signed two-dimensional edge function.
fn edge(
    first: [f32; 2],
    second: [f32; 2],
    point: [f32; 2],
) -> f32 {
    (point[0] - first[0]).mul_add(
        second[1] - first[1],
        -((point[1] - first[1]) * (second[0] - first[0])),
    )
}

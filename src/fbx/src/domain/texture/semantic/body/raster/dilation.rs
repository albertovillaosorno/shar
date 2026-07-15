// File:
//   - dilation.rs
// Path:
//   - src/fbx/src/domain/texture/semantic/body/raster/dilation.rs
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
//   - One-pixel-per-iteration edge dilation constrained to a chart's reserved
//   - atlas cell.
// - Must-Not:
//   - Paint original triangles, cross chart cells, change colors, or map UVs.
// - Allows:
//   - Checked eight-neighbor coverage expansion.
// - Split-When:
//   - Another padding filter becomes a supported texture contract.
// - Merge-When:
//   - Triangle coverage becomes the sole owner of chart padding.
// - Summary:
//   - Cell-bounded semantic chart edge dilation.
// - Description:
//   - Extends exact chart color into the declared padding without atlas bleed.
// - Usage:
//   - Called after at least one original chart pixel is painted.
// - Defaults:
//   - Each iteration reads a snapshot so expansion distance is exact.
//
// ADRs:
// - docs/adr/fbx/export/character-semantic-texture-rig-and-outfit-contract.md
//
// Large file:
//   - true
//   - Reason: checked cell bounds, snapshot iteration, neighbor search, and
//   - pixel writes form one deterministic padding behavior.
//

//! Deterministic chart-cell-bounded edge dilation.
use super::super::super::image::RgbaImage;
use super::super::charts::model::PlacedChart;
use super::super::error::SemanticTextureError;
use super::super::types::PixelRect;
use super::coverage_index;

/// Expand chart coverage by one pixel per iteration inside its own cell.
pub(super) fn apply(
    atlas: &mut RgbaImage,
    coverage: &mut [bool],
    chart: &PlacedChart,
    padding: u32,
) -> Result<(), SemanticTextureError> {
    let right = chart
        .public
        .cell
        .right()
        .ok_or(SemanticTextureError::NumericOverflow)?;
    let bottom = chart
        .public
        .cell
        .bottom()
        .ok_or(SemanticTextureError::NumericOverflow)?;
    for _iteration in 0..padding {
        let snapshot = coverage.to_vec();
        let mut additions = Vec::new();
        for y in chart
            .public
            .cell
            .y..=bottom
        {
            for x in chart
                .public
                .cell
                .x..=right
            {
                let index = coverage_index(
                    atlas.width(),
                    x,
                    y,
                )?;
                if snapshot[index] {
                    continue;
                }
                if has_covered_neighbor(
                    &snapshot,
                    atlas.width(),
                    chart
                        .public
                        .cell,
                    x,
                    y,
                )? {
                    additions.push(
                        (
                            x, y, index,
                        ),
                    );
                }
            }
        }
        for (x, y, index) in additions {
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
    Ok(())
}

/// Test the eight-neighbor set without leaving the reserved chart cell.
fn has_covered_neighbor(
    coverage: &[bool],
    width: u32,
    cell: PixelRect,
    x: u32,
    y: u32,
) -> Result<bool, SemanticTextureError> {
    let right = cell
        .right()
        .ok_or(SemanticTextureError::NumericOverflow)?;
    let bottom = cell
        .bottom()
        .ok_or(SemanticTextureError::NumericOverflow)?;
    for offset_y in -1_i32..=1 {
        for offset_x in -1_i32..=1 {
            if offset_x == 0 && offset_y == 0 {
                continue;
            }
            let Some(neighbor_x) = x.checked_add_signed(offset_x) else {
                continue;
            };
            let Some(neighbor_y) = y.checked_add_signed(offset_y) else {
                continue;
            };
            if neighbor_x < cell.x
                || neighbor_x > right
                || neighbor_y < cell.y
                || neighbor_y > bottom
            {
                continue;
            }
            if coverage[coverage_index(
                width, neighbor_x, neighbor_y,
            )?] {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

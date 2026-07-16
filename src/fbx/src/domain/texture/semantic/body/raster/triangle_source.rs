// File:
//   - triangle_source.rs
// Path:
//   - src/fbx/src/domain/texture/semantic/body/raster/triangle_source.rs
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
//   - Exact source-texel transfer for patterned semantic charts.
// - Must-Not:
//   - Change topology, classify evidence, or filter source texture pixels.
// - Allows:
//   - Copying each source texel into an integer-multiple atlas block.
// - Split-When:
//   - Filtering modes beyond deterministic nearest ownership are required.
// - Merge-When:
//   - Flat and source-sampled chart painters share one policy.
// - Summary:
//   - Patterned semantic chart texel-grid rasterizer.
// - Description:
//   - Preserves source texture boundaries exactly inside a modern atlas block.
// - Usage:
//   - Called only for charts projected through `source-uv`.
// - Defaults:
//   - Every source texel expands to a complete square block.
//
// ADRs:
// - docs/adr/fbx/export/character-semantic-texture-rig-and-outfit-contract.md
//
// Large file:
//   - false
//

//! Patterned semantic chart texel-grid rasterizer.
#![expect(
    clippy::indexing_slicing,
    reason = "Validated triangle and UV cardinalities bound source sampling \
              indices."
)]

use super::super::super::image::RgbaImage;
use super::super::charts::model::PlacedChart;
use super::super::error::SemanticTextureError;
use super::coverage_index;

/// Copy the complete source texture into an exact integer-scale atlas block.
pub(super) fn paint(
    atlas: &mut RgbaImage,
    coverage: &mut [bool],
    source: &RgbaImage,
    chart: &PlacedChart,
) -> Result<usize, SemanticTextureError> {
    let placement = chart
        .source_uv_placement
        .ok_or(SemanticTextureError::NumericOverflow)?;
    let mut painted = 0_usize;
    for source_y in 0..source.height() {
        let destination_y = placement.origin[1]
            .checked_add(
                source_y
                    .checked_mul(placement.scale)
                    .ok_or(SemanticTextureError::NumericOverflow)?,
            )
            .ok_or(SemanticTextureError::NumericOverflow)?;
        for source_x in 0..source.width() {
            let destination_x = placement.origin[0]
                .checked_add(
                    source_x
                        .checked_mul(placement.scale)
                        .ok_or(SemanticTextureError::NumericOverflow)?,
                )
                .ok_or(SemanticTextureError::NumericOverflow)?;
            let color = source.pixel(
                source_x, source_y,
            )?;
            for offset_y in 0..placement.scale {
                let y = destination_y
                    .checked_add(offset_y)
                    .ok_or(SemanticTextureError::NumericOverflow)?;
                for offset_x in 0..placement.scale {
                    let x = destination_x
                        .checked_add(offset_x)
                        .ok_or(SemanticTextureError::NumericOverflow)?;
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
                        x, y, color,
                    )?;
                }
            }
        }
    }
    Ok(painted)
}

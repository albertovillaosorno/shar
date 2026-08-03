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
//   - Triangle source domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Triangle source domain module.
// - Description:
//   - Implements the declared domain module responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Triangle source domain module.

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
            let color = source.pixel(source_x, source_y)?;
            for offset_y in 0..placement.scale {
                let y = destination_y
                    .checked_add(offset_y)
                    .ok_or(SemanticTextureError::NumericOverflow)?;
                for offset_x in 0..placement.scale {
                    let x = destination_x
                        .checked_add(offset_x)
                        .ok_or(SemanticTextureError::NumericOverflow)?;
                    let index = coverage_index(atlas.width(), x, y)?;
                    if !coverage[index] {
                        painted = painted
                            .checked_add(1)
                            .ok_or(SemanticTextureError::NumericOverflow)?;
                    }
                    coverage[index] = true;
                    atlas.set_pixel(x, y, color)?;
                }
            }
        }
    }
    Ok(painted)
}

// File:
//   - grid.rs
// Path:
//   - src/fbx/src/domain/texture/semantic/body/charts/packing/grid.rs
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
//   - Fixed semantic atlas columns, deterministic grid scoring, and checked
//   - cell rectangles.
// - Must-Not:
//   - Inspect chart geometry, calculate UVs, or rasterize pixels.
// - Allows:
//   - Integer partitioning and stable tie preferences.
// - Split-When:
//   - A different packing policy needs independent region ownership.
// - Merge-When:
//   - Coordinate mapping becomes the sole owner of cell selection.
// - Summary:
//   - Integer semantic atlas grid policy.
// - Description:
//   - Distributes remainder pixels by exact boundaries and maximizes interior
//   - chart dimensions.
// - Usage:
//   - Called by the packing facade once per semantic region and chart.
// - Defaults:
//   - Region columns follow BodyRegion order and ties prefer fewer columns.
//
// ADRs:
// - docs/adr/fbx/export/character-semantic-texture-rig-and-outfit-contract.md
//
// Large file:
//   - true
//   - Reason: semantic columns, grid scoring, and checked boundary distribution
//   - form one deterministic integer layout contract.
//

//! Deterministic integer semantic atlas grid policy.
use super::super::super::super::region::BodyRegion;
use super::super::super::error::SemanticTextureError;
use super::super::super::recipe::AtlasConfig;
use super::super::super::types::PixelRect;

/// One selected row-and-column grid.
#[derive(Clone, Copy)]
pub(super) struct Grid {
    columns: usize,
    rows: usize,
}

/// Return the fixed full-height column for one semantic region.
pub(super) fn semantic_column(
    config: &AtlasConfig,
    region: BodyRegion,
) -> Result<PixelRect, SemanticTextureError> {
    let count = u64::try_from(BodyRegion::ALL.len())
        .map_err(|_error| SemanticTextureError::NumericOverflow)?;
    let ordinal = u64::try_from(region.ordinal())
        .map_err(|_error| SemanticTextureError::NumericOverflow)?;
    let width = u64::from(config.width);
    let x = width
        .checked_mul(ordinal)
        .ok_or(SemanticTextureError::NumericOverflow)?
        / count;
    let right = width
        .checked_mul(ordinal + 1)
        .ok_or(SemanticTextureError::NumericOverflow)?
        / count;
    Ok(
        PixelRect {
            x: u32::try_from(x)
                .map_err(|_error| SemanticTextureError::NumericOverflow)?,
            y: 0,
            width: u32::try_from(right - x)
                .map_err(|_error| SemanticTextureError::NumericOverflow)?,
            height: config.height,
        },
    )
}

/// Choose the grid with the largest minimum interior dimension.
pub(super) fn choose(
    region: BodyRegion,
    rectangle: PixelRect,
    count: usize,
    padding: u32,
) -> Result<Grid, SemanticTextureError> {
    let mut best: Option<(
        Grid,
        GridScore,
    )> = None;
    for columns in 1..=count {
        let rows = count.div_ceil(columns);
        let columns_u32 = u32::try_from(columns)
            .map_err(|_error| SemanticTextureError::NumericOverflow)?;
        let rows_u32 = u32::try_from(rows)
            .map_err(|_error| SemanticTextureError::NumericOverflow)?;
        let cell_width = rectangle.width / columns_u32;
        let cell_height = rectangle.height / rows_u32;
        let inset = padding
            .checked_mul(2)
            .ok_or(SemanticTextureError::NumericOverflow)?;
        let Some(interior_width) = cell_width.checked_sub(inset) else {
            continue;
        };
        let Some(interior_height) = cell_height.checked_sub(inset) else {
            continue;
        };
        if interior_width < 2 || interior_height < 2 {
            continue;
        }
        let score = GridScore {
            minimum_dimension: interior_width.min(interior_height),
            area: u64::from(interior_width) * u64::from(interior_height),
            unused: columns
                .checked_mul(rows)
                .and_then(|slots| slots.checked_sub(count))
                .ok_or(SemanticTextureError::NumericOverflow)?,
            columns,
        };
        if best
            .as_ref()
            .is_none_or(|(_grid, current)| score.stronger_than(current))
        {
            best = Some(
                (
                    Grid {
                        columns,
                        rows,
                    },
                    score,
                ),
            );
        }
    }
    best.map(|(grid, _score)| grid)
        .ok_or(SemanticTextureError::RegionGridTooSmall(region))
}

/// Return one checked cell with remainder pixels distributed by boundaries.
pub(super) fn cell(
    region: PixelRect,
    grid: Grid,
    index: usize,
) -> Result<PixelRect, SemanticTextureError> {
    let column = index % grid.columns;
    let row = index / grid.columns;
    let left = partition_boundary(
        region.x,
        region.width,
        column,
        grid.columns,
    )?;
    let right = partition_boundary(
        region.x,
        region.width,
        column + 1,
        grid.columns,
    )?;
    let top = partition_boundary(
        region.y,
        region.height,
        row,
        grid.rows,
    )?;
    let bottom = partition_boundary(
        region.y,
        region.height,
        row + 1,
        grid.rows,
    )?;
    Ok(
        PixelRect {
            x: left,
            y: top,
            width: right - left,
            height: bottom - top,
        },
    )
}

/// Stable integer score for one candidate grid.
struct GridScore {
    minimum_dimension: u32,
    area: u64,
    unused: usize,
    columns: usize,
}

impl GridScore {
    /// Compare scores while preserving deterministic tie preferences.
    fn stronger_than(
        &self,
        other: &Self,
    ) -> bool {
        self.minimum_dimension > other.minimum_dimension
            || (self.minimum_dimension == other.minimum_dimension
                && (self.area > other.area
                    || (self.area == other.area
                        && (self.unused < other.unused
                            || (self.unused == other.unused
                                && self.columns < other.columns)))))
    }
}

/// Calculate one evenly partitioned boundary with checked arithmetic.
fn partition_boundary(
    origin: u32,
    length: u32,
    ordinal: usize,
    count: usize,
) -> Result<u32, SemanticTextureError> {
    let offset = u64::from(length)
        .checked_mul(
            u64::try_from(ordinal)
                .map_err(|_error| SemanticTextureError::NumericOverflow)?,
        )
        .ok_or(SemanticTextureError::NumericOverflow)?
        / u64::try_from(count)
            .map_err(|_error| SemanticTextureError::NumericOverflow)?;
    origin
        .checked_add(
            u32::try_from(offset)
                .map_err(|_error| SemanticTextureError::NumericOverflow)?,
        )
        .ok_or(SemanticTextureError::NumericOverflow)
}

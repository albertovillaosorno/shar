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
//   - Deterministic assembly of decoded Pure3D sprite image tiles.
// - Must-Not:
//   - Use sibling images, guess ambiguous grids, or perform filesystem I/O.
// - Allows:
//   - Source-derived grid validation, explicit tile orientation, overlap, crop.
// - Split-When:
//   - Sprite placement gains another independently evidenced layout family.
// - Merge-When:
//   - Another adapter owns the identical decoded sprite raster boundary.
// - Summary:
//   - Assemble source-backed sprite tiles into one logical RGBA image.
// - Description:
//   - Requires one unique structural grid and preserves row-major overwrite.
// - Usage:
//   - Runs after each embedded image tile has been decoded to RGBA8.
// - Defaults:
//   - Missing, malformed, or ambiguous tile evidence fails explicitly.
//

//! Source-backed `Pure3D` sprite tile assembly.

#![expect(
    clippy::arithmetic_side_effects,
    clippy::integer_division,
    reason = "Grid divisors and fixed RGBA pixel arithmetic are validated or \
              bounds-checked before use."
)]

use super::dds::DecodedRgbaImage;
use crate::P3dError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Sprite raster layout.
pub struct SpriteRasterLayout {
    /// Logical sprite width after overlap and crop.
    pub width: u32,
    /// Logical sprite height after overlap and crop.
    pub height: u32,
    /// Source `blit_border` value from the owning sprite chunk.
    pub blit_border: u32,
    /// Whether each decoded tile is vertically inverted before placement.
    pub flip_vertical: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Grid {
    columns: usize,
    rows: usize,
}

/// Assemble decoded sprite-owned tiles into the logical source raster.
///
/// # Errors
///
/// Returns an error when dimensions, pixel storage, overlap, or grid structure
/// are invalid or when the tile evidence admits anything other than one grid.
pub fn assemble_sprite_rgba(
    layout: SpriteRasterLayout,
    tiles: &[DecodedRgbaImage],
) -> Result<DecodedRgbaImage, P3dError> {
    validate_layout(layout)?;
    validate_tiles(tiles)?;
    let overlap_u32 = layout.blit_border.checked_mul(2).ok_or_else(|| {
        P3dError::invalid_source("sprite blit-border overlap overflowed")
    })?;
    let overlap = usize::try_from(overlap_u32).map_err(|error| {
        P3dError::invalid_source(format!(
            "sprite overlap exceeds usize: {error}"
        ))
    })?;
    let grid = unique_grid(layout, tiles, overlap)?;
    let width = usize::try_from(layout.width).map_err(|error| {
        P3dError::invalid_source(format!("sprite width exceeds usize: {error}"))
    })?;
    let height = usize::try_from(layout.height).map_err(|error| {
        P3dError::invalid_source(format!(
            "sprite height exceeds usize: {error}"
        ))
    })?;
    let output_len = width
        .checked_mul(height)
        .and_then(|value| value.checked_mul(4))
        .ok_or_else(|| {
            P3dError::invalid_source("sprite RGBA size overflowed")
        })?;
    let mut rgba = vec![0_u8; output_len];
    let column_origins = column_origins(tiles, grid, overlap)?;
    let row_origins = row_origins(tiles, grid, overlap)?;
    for row in 0..grid.rows {
        for column in 0..grid.columns {
            let ordinal = row
                .checked_mul(grid.columns)
                .and_then(|value| value.checked_add(column))
                .ok_or_else(|| {
                    P3dError::invalid_source("sprite tile ordinal overflowed")
                })?;
            let tile = tiles.get(ordinal).ok_or_else(|| {
                P3dError::invalid_source("sprite tile ordinal is out of bounds")
            })?;
            let origin_x = *column_origins.get(column).ok_or_else(|| {
                P3dError::invalid_source("sprite column origin is missing")
            })?;
            let origin_y = *row_origins.get(row).ok_or_else(|| {
                P3dError::invalid_source("sprite row origin is missing")
            })?;
            blit_tile(
                tile,
                origin_x,
                origin_y,
                width,
                height,
                layout.flip_vertical,
                &mut rgba,
            )?;
        }
    }
    Ok(DecodedRgbaImage {
        width: layout.width,
        height: layout.height,
        rgba,
    })
}

fn validate_layout(layout: SpriteRasterLayout) -> Result<(), P3dError> {
    if layout.width == 0 || layout.height == 0 {
        return Err(P3dError::invalid_source(
            "sprite logical dimensions must be nonzero",
        ));
    }
    Ok(())
}

fn validate_tiles(tiles: &[DecodedRgbaImage]) -> Result<(), P3dError> {
    if tiles.is_empty() {
        return Err(P3dError::invalid_source(
            "sprite has no decoded image tiles",
        ));
    }
    for tile in tiles {
        if tile.width == 0 || tile.height == 0 {
            return Err(P3dError::invalid_source(
                "sprite tile dimensions must be nonzero",
            ));
        }
        let width = usize::try_from(tile.width).map_err(|error| {
            P3dError::invalid_source(format!(
                "sprite tile width exceeds usize: {error}"
            ))
        })?;
        let height = usize::try_from(tile.height).map_err(|error| {
            P3dError::invalid_source(format!(
                "sprite tile height exceeds usize: {error}"
            ))
        })?;
        let expected = width
            .checked_mul(height)
            .and_then(|value| value.checked_mul(4))
            .ok_or_else(|| {
                P3dError::invalid_source("sprite tile RGBA size overflowed")
            })?;
        if tile.rgba.len() != expected {
            return Err(P3dError::invalid_source(
                "sprite tile RGBA storage does not match its dimensions",
            ));
        }
    }
    Ok(())
}

fn unique_grid(
    layout: SpriteRasterLayout,
    tiles: &[DecodedRgbaImage],
    overlap: usize,
) -> Result<Grid, P3dError> {
    let logical_width = usize::try_from(layout.width).map_err(|error| {
        P3dError::invalid_source(format!("sprite width exceeds usize: {error}"))
    })?;
    let logical_height = usize::try_from(layout.height).map_err(|error| {
        P3dError::invalid_source(format!(
            "sprite height exceeds usize: {error}"
        ))
    })?;
    let required_width =
        logical_width.checked_add(overlap).ok_or_else(|| {
            P3dError::invalid_source("sprite padded width overflowed")
        })?;
    let required_height =
        logical_height.checked_add(overlap).ok_or_else(|| {
            P3dError::invalid_source("sprite padded height overflowed")
        })?;
    let mut candidate = None;
    for columns in 1..=tiles.len() {
        if !tiles.len().is_multiple_of(columns) {
            continue;
        }
        let rows = tiles.len() / columns;
        let grid = Grid { columns, rows };
        let Some((coverage_width, coverage_height)) =
            grid_coverage(tiles, grid, overlap)?
        else {
            continue;
        };
        if coverage_width < required_width || coverage_height < required_height
        {
            continue;
        }
        if candidate.replace(grid).is_some() {
            return Err(P3dError::invalid_source(
                "sprite tile evidence admits multiple structural grids",
            ));
        }
    }
    candidate.ok_or_else(|| {
        P3dError::invalid_source("sprite tile evidence has no structural grid")
    })
}

fn grid_coverage(
    tiles: &[DecodedRgbaImage],
    grid: Grid,
    overlap: usize,
) -> Result<Option<(usize, usize)>, P3dError> {
    let first_row = tiles.get(..grid.columns).ok_or_else(|| {
        P3dError::invalid_source("sprite first tile row is out of bounds")
    })?;
    let widths = first_row
        .iter()
        .map(|tile| usize::try_from(tile.width))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| {
            P3dError::invalid_source(format!(
                "sprite tile width exceeds usize: {error}"
            ))
        })?;
    let mut row_heights = Vec::with_capacity(grid.rows);
    for row in 0..grid.rows {
        let start = row.checked_mul(grid.columns).ok_or_else(|| {
            P3dError::invalid_source("sprite grid row offset overflowed")
        })?;
        let end = start.checked_add(grid.columns).ok_or_else(|| {
            P3dError::invalid_source("sprite grid row end overflowed")
        })?;
        let row_tiles = tiles.get(start..end).ok_or_else(|| {
            P3dError::invalid_source("sprite grid row is out of bounds")
        })?;
        let mut row_height = None;
        for (column, tile) in row_tiles.iter().enumerate() {
            let Some(expected_width) = widths.get(column) else {
                return Err(P3dError::invalid_source(
                    "sprite grid width pattern is incomplete",
                ));
            };
            let tile_width = usize::try_from(tile.width).map_err(|error| {
                P3dError::invalid_source(format!(
                    "sprite tile width exceeds usize: {error}"
                ))
            })?;
            if tile_width != *expected_width {
                return Ok(None);
            }
            let tile_height =
                usize::try_from(tile.height).map_err(|error| {
                    P3dError::invalid_source(format!(
                        "sprite tile height exceeds usize: {error}"
                    ))
                })?;
            if let Some(expected_height) = row_height {
                if tile_height != expected_height {
                    return Ok(None);
                }
            } else {
                row_height = Some(tile_height);
            }
        }
        row_heights.push(row_height.ok_or_else(|| {
            P3dError::invalid_source("sprite grid row has no tiles")
        })?);
    }
    let width = overlapped_extent(&widths, overlap)?;
    let height = overlapped_extent(&row_heights, overlap)?;
    Ok(Some((width, height)))
}

fn overlapped_extent(
    sizes: &[usize],
    overlap: usize,
) -> Result<usize, P3dError> {
    let Some(first) = sizes.first().copied() else {
        return Err(P3dError::invalid_source("sprite grid extent is empty"));
    };
    validate_preceding_sizes(sizes, overlap)?;
    let mut extent = first;
    for size in sizes.iter().copied().skip(1) {
        extent = extent
            .checked_add(size.checked_sub(overlap).ok_or_else(|| {
                P3dError::invalid_source(
                    "sprite overlap subtraction underflowed",
                )
            })?)
            .ok_or_else(|| {
                P3dError::invalid_source("sprite grid extent overflowed")
            })?;
    }
    Ok(extent)
}

fn validate_preceding_sizes(
    sizes: &[usize],
    overlap: usize,
) -> Result<(), P3dError> {
    let preceding_count = sizes.len().saturating_sub(1);
    for size in sizes.iter().take(preceding_count) {
        if *size <= overlap {
            return Err(P3dError::invalid_source(
                "sprite overlap consumes an entire preceding tile dimension",
            ));
        }
    }
    Ok(())
}

fn column_origins(
    tiles: &[DecodedRgbaImage],
    grid: Grid,
    overlap: usize,
) -> Result<Vec<usize>, P3dError> {
    let first_row = tiles.get(..grid.columns).ok_or_else(|| {
        P3dError::invalid_source("sprite first tile row is out of bounds")
    })?;
    origins(
        first_row
            .iter()
            .map(|tile| usize::try_from(tile.width))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| {
                P3dError::invalid_source(format!(
                    "sprite tile width exceeds usize: {error}"
                ))
            })?
            .as_slice(),
        overlap,
    )
}

fn row_origins(
    tiles: &[DecodedRgbaImage],
    grid: Grid,
    overlap: usize,
) -> Result<Vec<usize>, P3dError> {
    let mut heights = Vec::with_capacity(grid.rows);
    for row in 0..grid.rows {
        let ordinal = row.checked_mul(grid.columns).ok_or_else(|| {
            P3dError::invalid_source("sprite row origin ordinal overflowed")
        })?;
        let tile = tiles.get(ordinal).ok_or_else(|| {
            P3dError::invalid_source("sprite row origin tile is missing")
        })?;
        heights.push(usize::try_from(tile.height).map_err(|error| {
            P3dError::invalid_source(format!(
                "sprite tile height exceeds usize: {error}"
            ))
        })?);
    }
    origins(&heights, overlap)
}

fn origins(sizes: &[usize], overlap: usize) -> Result<Vec<usize>, P3dError> {
    validate_preceding_sizes(sizes, overlap)?;
    let mut values = Vec::with_capacity(sizes.len());
    let mut current = 0_usize;
    for (index, size) in sizes.iter().copied().enumerate() {
        values.push(current);
        if index.saturating_add(1) == sizes.len() {
            continue;
        }
        let advance = size.checked_sub(overlap).ok_or_else(|| {
            P3dError::invalid_source("sprite overlap subtraction underflowed")
        })?;
        current = current.checked_add(advance).ok_or_else(|| {
            P3dError::invalid_source("sprite tile origin overflowed")
        })?;
    }
    Ok(values)
}

fn blit_tile(
    tile: &DecodedRgbaImage,
    origin_x: usize,
    origin_y: usize,
    output_width: usize,
    output_height: usize,
    flip_vertical: bool,
    output: &mut [u8],
) -> Result<(), P3dError> {
    let tile_width = usize::try_from(tile.width).map_err(|error| {
        P3dError::invalid_source(format!(
            "sprite tile width exceeds usize: {error}"
        ))
    })?;
    let tile_height = usize::try_from(tile.height).map_err(|error| {
        P3dError::invalid_source(format!(
            "sprite tile height exceeds usize: {error}"
        ))
    })?;
    for source_y in 0..tile_height {
        let read_y = if flip_vertical {
            tile_height
                .checked_sub(source_y)
                .and_then(|value| value.checked_sub(1))
                .ok_or_else(|| {
                    P3dError::invalid_source("sprite tile Y flip underflowed")
                })?
        } else {
            source_y
        };
        let target_y = origin_y.checked_add(source_y).ok_or_else(|| {
            P3dError::invalid_source("sprite target Y coordinate overflowed")
        })?;
        if target_y >= output_height {
            continue;
        }
        for source_x in 0..tile_width {
            let target_x = origin_x.checked_add(source_x).ok_or_else(|| {
                P3dError::invalid_source(
                    "sprite target X coordinate overflowed",
                )
            })?;
            if target_x >= output_width {
                continue;
            }
            let source_offset = read_y
                .checked_mul(tile_width)
                .and_then(|value| value.checked_add(source_x))
                .and_then(|value| value.checked_mul(4))
                .ok_or_else(|| {
                    P3dError::invalid_source(
                        "sprite source RGBA offset overflowed",
                    )
                })?;
            let target_offset = target_y
                .checked_mul(output_width)
                .and_then(|value| value.checked_add(target_x))
                .and_then(|value| value.checked_mul(4))
                .ok_or_else(|| {
                    P3dError::invalid_source(
                        "sprite target RGBA offset overflowed",
                    )
                })?;
            let source_end = source_offset.checked_add(4).ok_or_else(|| {
                P3dError::invalid_source("sprite source RGBA end overflowed")
            })?;
            let target_end = target_offset.checked_add(4).ok_or_else(|| {
                P3dError::invalid_source("sprite target RGBA end overflowed")
            })?;
            let source_pixel =
                tile.rgba.get(source_offset..source_end).ok_or_else(|| {
                    P3dError::invalid_source(
                        "sprite source RGBA pixel is out of bounds",
                    )
                })?;
            let target_pixel =
                output.get_mut(target_offset..target_end).ok_or_else(|| {
                    P3dError::invalid_source(
                        "sprite target RGBA pixel is out of bounds",
                    )
                })?;
            target_pixel.copy_from_slice(source_pixel);
        }
    }
    Ok(())
}

#[cfg(test)]
// jig-ignore-next-line: exact test module path is indivisible
#[path = "../../../../../tests/formats/p3d/unit/adapter-outbound/sprite_raster_tests.rs"]
mod tests;

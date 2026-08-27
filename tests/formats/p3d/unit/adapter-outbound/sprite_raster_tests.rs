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
//   - Pure3D sprite raster assembly unit tests.
// - Must-Not:
//   - Depend on proprietary sprite or PNG fixtures.
// - Allows:
//   - Synthetic tile grids that pin flip, overlap, crop, and rejection rules.
// - Split-When:
//   - Another sprite layout family gains independent fixture ownership.
// - Merge-When:
//   - Another test module owns the identical private assembly boundary.
// - Summary:
//   - Pins deterministic source-backed tile composition.
// - Description:
//   - Uses synthetic RGBA tiles only.
// - Usage:
//   - Included by the sprite raster adapter under cfg(test).
// - Defaults:
//   - Ambiguous or malformed grids fail explicitly.
//

//! `Pure3D` sprite raster assembly tests.

use super::{DecodedRgbaImage, SpriteRasterLayout, assemble_sprite_rgba};

fn tile(rows: [[u8; 4]; 4]) -> DecodedRgbaImage {
    let rgba = rows
        .into_iter()
        .flat_map(|color| std::iter::repeat_n(color, 4))
        .flatten()
        .collect();
    DecodedRgbaImage {
        width: 4,
        height: 4,
        rgba,
    }
}

fn solid(color: [u8; 4]) -> DecodedRgbaImage {
    tile([color; 4])
}

fn pixel(image: &DecodedRgbaImage, x: usize, y: usize) -> Option<&[u8]> {
    let width = usize::try_from(image.width).ok()?;
    let offset = y.checked_mul(width)?.checked_add(x)?.checked_mul(4)?;
    image.rgba.get(offset..offset.checked_add(4)?)
}

#[test]
fn assembles_grid_with_flip_and_overlap() -> Result<(), String> {
    let top_left = tile([
        [10, 0, 0, 255],
        [20, 0, 0, 255],
        [30, 0, 0, 255],
        [40, 0, 0, 255],
    ]);
    let tiles = [
        top_left,
        solid([0, 50, 0, 255]),
        solid([0, 0, 60, 255]),
        solid([70, 70, 0, 255]),
    ];
    let image = assemble_sprite_rgba(
        SpriteRasterLayout {
            width: 4,
            height: 4,
            blit_border: 1,
            flip_vertical: true,
        },
        &tiles,
    )
    .map_err(|error| error.to_string())?;
    assert_eq!(pixel(&image, 0, 0), Some([40, 0, 0, 255].as_slice()));
    assert_eq!(pixel(&image, 0, 1), Some([30, 0, 0, 255].as_slice()));
    assert_eq!(pixel(&image, 2, 0), Some([0, 50, 0, 255].as_slice()));
    assert_eq!(pixel(&image, 0, 2), Some([0, 0, 60, 255].as_slice()));
    assert_eq!(pixel(&image, 2, 2), Some([70, 70, 0, 255].as_slice()));
    Ok(())
}

#[test]
fn rejects_ambiguous_grid_instead_of_choosing_a_divisor() {
    let tiles = std::iter::repeat_with(|| solid([1, 2, 3, 4]))
        .take(4)
        .collect::<Vec<_>>();
    let result = assemble_sprite_rgba(
        SpriteRasterLayout {
            width: 4,
            height: 4,
            blit_border: 0,
            flip_vertical: true,
        },
        &tiles,
    );
    assert!(result.is_err());
}

#[test]
fn padded_extent_disambiguates_border_tiles() -> Result<(), String> {
    let tiles = std::iter::repeat_with(|| solid([1, 2, 3, 4]))
        .take(4)
        .collect::<Vec<_>>();
    let image = assemble_sprite_rgba(
        SpriteRasterLayout {
            width: 4,
            height: 4,
            blit_border: 1,
            flip_vertical: false,
        },
        &tiles,
    )
    .map_err(|error| error.to_string())?;
    assert_eq!((image.width, image.height), (4, 4));
    Ok(())
}

#[test]
fn preserves_source_rows_without_vertical_flip() -> Result<(), String> {
    let image = assemble_sprite_rgba(
        SpriteRasterLayout {
            width: 4,
            height: 4,
            blit_border: 0,
            flip_vertical: false,
        },
        &[tile([
            [10, 0, 0, 255],
            [20, 0, 0, 255],
            [30, 0, 0, 255],
            [40, 0, 0, 255],
        ])],
    )
    .map_err(|error| error.to_string())?;
    assert_eq!(pixel(&image, 0, 0), Some([10, 0, 0, 255].as_slice()));
    assert_eq!(pixel(&image, 0, 3), Some([40, 0, 0, 255].as_slice()));
    Ok(())
}

#[test]
fn rejects_inconsistent_row_width_pattern() {
    let mut odd = solid([1, 2, 3, 4]);
    odd.width = 8;
    odd.rgba = vec![0_u8; 8 * 4 * 4];
    let tiles = [
        solid([1, 0, 0, 255]),
        solid([2, 0, 0, 255]),
        solid([3, 0, 0, 255]),
        odd,
    ];
    let result = assemble_sprite_rgba(
        SpriteRasterLayout {
            width: 6,
            height: 6,
            blit_border: 1,
            flip_vertical: true,
        },
        &tiles,
    );
    assert!(result.is_err());
}

#[test]
fn rejects_overlap_that_consumes_a_preceding_tile() {
    let narrow = DecodedRgbaImage {
        width: 2,
        height: 4,
        rgba: vec![0_u8; 2 * 4 * 4],
    };
    let result = assemble_sprite_rgba(
        SpriteRasterLayout {
            width: 2,
            height: 4,
            blit_border: 1,
            flip_vertical: true,
        },
        &[narrow.clone(), narrow],
    );
    assert!(result.is_err());
}

#[test]
fn rejects_malformed_rgba_storage() {
    let malformed = DecodedRgbaImage {
        width: 4,
        height: 4,
        rgba: vec![0_u8; 3],
    };
    let result = assemble_sprite_rgba(
        SpriteRasterLayout {
            width: 4,
            height: 4,
            blit_border: 0,
            flip_vertical: true,
        },
        &[malformed],
    );
    assert!(result.is_err());
}

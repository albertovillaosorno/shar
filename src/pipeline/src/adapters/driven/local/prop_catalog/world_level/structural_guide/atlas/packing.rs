// File:
//   - packing.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/prop_catalog/world_level/
//     structural_guide/atlas/packing.rs
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
//   - Deterministic non-rotating atlas placement and two-pixel dilation.
// - Must-Not:
//   - Resize pixels, alter colors, infer materials, or serialize PNG/JSON.
// - Allows:
//   - Height-first shelf packing and checked row-major pixel writes.
// - Summary:
//   - Packs exact atlas variants into one 4096-square RGB buffer.
//
// Large file:
//   - false
//

//! Deterministic structural-guide atlas packing.

use super::{ATLAS_PADDING, ATLAS_SIZE, AtlasVariant};
use crate::domain::PipelineError;

pub(super) fn pack(
    variants: &mut [AtlasVariant]
) -> Result<Vec<[u8; 3]>, PipelineError> {
    variants.sort_by(
        |left, right| {
            right
                .height
                .cmp(&left.height)
                .then_with(
                    || {
                        right
                            .width
                            .cmp(&left.width)
                    },
                )
                .then_with(
                    || {
                        left.source_sha256
                            .cmp(&right.source_sha256)
                    },
                )
                .then_with(
                    || {
                        left.variant_sha256
                            .cmp(&right.variant_sha256)
                    },
                )
        },
    );
    let mut cursor_x = 0_u32;
    let mut cursor_y = 0_u32;
    let mut row_height = 0_u32;
    for variant in variants.iter_mut() {
        let packed_width = variant
            .width
            .checked_add(ATLAS_PADDING.saturating_mul(2))
            .ok_or_else(|| PipelineError::new("atlas tile width overflowed"))?;
        let packed_height = variant
            .height
            .checked_add(ATLAS_PADDING.saturating_mul(2))
            .ok_or_else(
                || PipelineError::new("atlas tile height overflowed"),
            )?;
        if packed_width > ATLAS_SIZE || packed_height > ATLAS_SIZE {
            return Err(
                PipelineError::new(
                    format!(
                        "atlas tile exceeds 4096 pixels: {}:{}x{}",
                        variant.source_name, variant.width, variant.height
                    ),
                ),
            );
        }
        if cursor_x.saturating_add(packed_width) > ATLAS_SIZE {
            cursor_x = 0;
            cursor_y = cursor_y
                .checked_add(row_height)
                .ok_or_else(
                    || PipelineError::new("atlas row offset overflowed"),
                )?;
            row_height = 0;
        }
        if cursor_y.saturating_add(packed_height) > ATLAS_SIZE {
            return Err(
                PipelineError::new(
                    format!(
                        "atlas entries do not fit 4096x4096 without resizing: \
                         {}",
                        variant.source_name
                    ),
                ),
            );
        }
        variant.useful_x = cursor_x + ATLAS_PADDING;
        variant.useful_y = cursor_y + ATLAS_PADDING;
        cursor_x = cursor_x
            .checked_add(packed_width)
            .ok_or_else(
                || PipelineError::new("atlas column offset overflowed"),
            )?;
        row_height = row_height.max(packed_height);
    }
    let pixel_count =
        usize::try_from(u64::from(ATLAS_SIZE) * u64::from(ATLAS_SIZE))
            .map_err(|error| PipelineError::new(error.to_string()))?;
    let mut atlas = vec![[0_u8; 3]; pixel_count];
    for variant in variants.iter() {
        blit_with_padding(
            &mut atlas, variant,
        )?;
    }
    Ok(atlas)
}

fn blit_with_padding(
    atlas: &mut [[u8; 3]],
    variant: &AtlasVariant,
) -> Result<(), PipelineError> {
    for y in 0..variant.height {
        for x in 0..variant.width {
            let source = source_pixel(
                variant, x, y,
            )?;
            set_atlas_pixel(
                atlas,
                variant.useful_x + x,
                variant.useful_y + y,
                source,
            )?;
        }
    }
    for distance in 1..=ATLAS_PADDING {
        let left_x = variant.useful_x - distance;
        let right_x = variant.useful_x + variant.width - 1 + distance;
        for y in 0..variant.height {
            set_atlas_pixel(
                atlas,
                left_x,
                variant.useful_y + y,
                source_pixel(
                    variant, 0, y,
                )?,
            )?;
            set_atlas_pixel(
                atlas,
                right_x,
                variant.useful_y + y,
                source_pixel(
                    variant,
                    variant.width - 1,
                    y,
                )?,
            )?;
        }
        let top_y = variant.useful_y - distance;
        let bottom_y = variant.useful_y + variant.height - 1 + distance;
        for x in 0..variant.width {
            set_atlas_pixel(
                atlas,
                variant.useful_x + x,
                top_y,
                source_pixel(
                    variant, x, 0,
                )?,
            )?;
            set_atlas_pixel(
                atlas,
                variant.useful_x + x,
                bottom_y,
                source_pixel(
                    variant,
                    x,
                    variant.height - 1,
                )?,
            )?;
        }
        for (x, y, source_x, source_y) in [
            (
                left_x, top_y, 0, 0,
            ),
            (
                right_x,
                top_y,
                variant.width - 1,
                0,
            ),
            (
                left_x,
                bottom_y,
                0,
                variant.height - 1,
            ),
            (
                right_x,
                bottom_y,
                variant.width - 1,
                variant.height - 1,
            ),
        ] {
            set_atlas_pixel(
                atlas,
                x,
                y,
                source_pixel(
                    variant, source_x, source_y,
                )?,
            )?;
        }
    }
    Ok(())
}

fn source_pixel(
    variant: &AtlasVariant,
    x: u32,
    y: u32,
) -> Result<[u8; 3], PipelineError> {
    let index =
        usize::try_from(u64::from(y) * u64::from(variant.width) + u64::from(x))
            .map_err(|error| PipelineError::new(error.to_string()))?;
    variant
        .pixels
        .get(index)
        .copied()
        .ok_or_else(|| PipelineError::new("atlas source pixel is missing"))
}

fn set_atlas_pixel(
    atlas: &mut [[u8; 3]],
    x: u32,
    y: u32,
    pixel: [u8; 3],
) -> Result<(), PipelineError> {
    if x >= ATLAS_SIZE || y >= ATLAS_SIZE {
        return Err(
            PipelineError::new("atlas write exceeded 4096-square bounds"),
        );
    }
    let index =
        usize::try_from(u64::from(y) * u64::from(ATLAS_SIZE) + u64::from(x))
            .map_err(|error| PipelineError::new(error.to_string()))?;
    let target = atlas
        .get_mut(index)
        .ok_or_else(|| PipelineError::new("atlas target pixel is missing"))?;
    *target = pixel;
    Ok(())
}

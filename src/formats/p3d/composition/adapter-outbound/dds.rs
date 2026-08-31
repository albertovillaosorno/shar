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
//   - Narrow legacy DDS block-compression decoding used by Pure3D evidence.
// - Must-Not:
//   - Accept unproven DDS variants, mip chains, or perform filesystem I/O.
// - Allows:
//   - Deterministic DXT1, DXT3, and DXT5 top-level decode into RGBA8 pixels.
// - Split-When:
//   - Another DDS family or independent image lifecycle needs support.
// - Merge-When:
//   - Another adapter owns the identical validated legacy DDS boundary.
// - Summary:
//   - Pure deterministic decoder for the source-backed DDS subset.
// - Description:
//   - Validates the demonstrated legacy DDS contract before BC1/2/3 decode.
// - Usage:
//   - Consumed by semantic sprite compilation after lossless P3D extraction.
// - Defaults:
//   - Unsupported or structurally ambiguous payloads fail explicitly.
//

//! Narrow legacy DDS decoder for source-backed `Pure3D` image payloads.

// CSpell:ignore dxt

#![expect(
    clippy::arithmetic_side_effects,
    clippy::integer_division,
    reason = "BC1/2/3 channel interpolation and exact block-grid arithmetic \
              require the integer formulas defined by the compressed format."
)]

use crate::P3dError;

const DDS_HEADER_BYTES: usize = 128;
const DDS_PIXEL_FORMAT_SIZE: u32 = 32;
const FOURCC_PIXEL_FORMAT_FLAG: u32 = 0x0000_0004;
const DXT1: [u8; 4] = *b"DXT1";
const DXT3: [u8; 4] = *b"DXT3";
const DXT5: [u8; 4] = *b"DXT5";

#[derive(Clone, Debug, Eq, PartialEq)]
/// Decoded RGBA image.
pub struct DecodedRgbaImage {
    /// Width.
    pub width: u32,
    /// Height.
    pub height: u32,
    /// Row-major RGBA8 pixels from the DDS top-level image.
    pub rgba: Vec<u8>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum BlockFormat {
    Dxt1,
    Dxt3,
    Dxt5,
}

impl BlockFormat {
    const fn bytes_per_block(self) -> usize {
        match self {
            Self::Dxt1 => 8,
            Self::Dxt3 | Self::Dxt5 => 16,
        }
    }
}

/// Decode the exact legacy DDS subset demonstrated by embedded `Pure3D`
/// sprites.
///
/// # Errors
///
/// Returns an error for malformed headers, unsupported formats, mip data,
/// non-four-pixel-aligned dimensions, or a payload length mismatch.
pub fn decode_legacy_dds(
    payload: &[u8],
) -> Result<DecodedRgbaImage, P3dError> {
    if payload.len() < DDS_HEADER_BYTES || payload.get(..4) != Some(b"DDS ") {
        return Err(P3dError::invalid_source("invalid legacy DDS signature"));
    }
    if read_u32(payload, 4) != Some(124)
        || read_u32(payload, 76) != Some(DDS_PIXEL_FORMAT_SIZE)
        || read_u32(payload, 80) != Some(FOURCC_PIXEL_FORMAT_FLAG)
    {
        return Err(P3dError::invalid_source(
            "unsupported legacy DDS header layout",
        ));
    }
    let height = required_u32(payload, 12, "DDS height is missing")?;
    let width = required_u32(payload, 16, "DDS width is missing")?;
    if width == 0 || height == 0 || width % 4 != 0 || height % 4 != 0 {
        return Err(P3dError::invalid_source(
            "DDS dimensions must be nonzero multiples of four",
        ));
    }
    if read_u32(payload, 28) != Some(0) {
        return Err(P3dError::invalid_source(
            "DDS mip chains are outside the supported source contract",
        ));
    }
    let fourcc = payload
        .get(84..88)
        .ok_or_else(|| P3dError::invalid_source("DDS FourCC is missing"))?;
    let format = match fourcc {
        value if value == DXT1 => BlockFormat::Dxt1,
        value if value == DXT3 => BlockFormat::Dxt3,
        value if value == DXT5 => BlockFormat::Dxt5,
        _ => {
            return Err(P3dError::invalid_source(
                "DDS FourCC is outside the supported DXT1/DXT3/DXT5 subset",
            ));
        },
    };
    let width_usize = usize::try_from(width)
        .map_err(|error| {
            P3dError::invalid_source(format!(
                "DDS width exceeds usize: {error}"
            ))
        })?;
    let height_usize = usize::try_from(height)
        .map_err(|error| {
            P3dError::invalid_source(format!(
                "DDS height exceeds usize: {error}"
            ))
        })?;
    let blocks_x = width_usize / 4;
    let blocks_y = height_usize / 4;
    let block_count = blocks_x
        .checked_mul(blocks_y)
        .ok_or_else(|| P3dError::invalid_source("DDS block count overflowed"))?;
    let encoded_bytes = block_count
        .checked_mul(format.bytes_per_block())
        .ok_or_else(|| {
            P3dError::invalid_source("DDS encoded size overflowed")
        })?;
    let expected_len = DDS_HEADER_BYTES
        .checked_add(encoded_bytes)
        .ok_or_else(|| P3dError::invalid_source("DDS total size overflowed"))?;
    if payload.len() != expected_len {
        return Err(P3dError::invalid_source(
            "DDS payload length does not match the top-level block image",
        ));
    }
    let encoded_u32 = u32::try_from(encoded_bytes)
        .map_err(|error| {
            P3dError::invalid_source(format!(
                "DDS encoded size exceeds u32: {error}"
            ))
        })?;
    if read_u32(payload, 20) != Some(encoded_u32) {
        return Err(P3dError::invalid_source(
            "DDS linear size does not match the block payload",
        ));
    }
    let pixel_bytes = width_usize
        .checked_mul(height_usize)
        .and_then(|count| count.checked_mul(4))
        .ok_or_else(|| P3dError::invalid_source("DDS RGBA size overflowed"))?;
    let mut rgba = vec![0_u8; pixel_bytes];
    decode_blocks(
        payload
            .get(DDS_HEADER_BYTES..)
            .ok_or_else(|| {
                P3dError::invalid_source("DDS block payload is missing")
            })?,
        format,
        width_usize,
        height_usize,
        &mut rgba,
    )?;
    Ok(DecodedRgbaImage {
        width,
        height,
        rgba,
    })
}

fn decode_blocks(
    encoded: &[u8],
    format: BlockFormat,
    width: usize,
    height: usize,
    rgba: &mut [u8],
) -> Result<(), P3dError> {
    let blocks_x = width / 4;
    let blocks_y = height / 4;
    let block_size = format.bytes_per_block();
    for block_y in 0..blocks_y {
        for block_x in 0..blocks_x {
            let block_ordinal = block_y
                .checked_mul(blocks_x)
                .and_then(|value| value.checked_add(block_x))
                .ok_or_else(|| {
                    P3dError::invalid_source("DDS block ordinal overflowed")
                })?;
            let block_start = block_ordinal
                .checked_mul(block_size)
                .ok_or_else(|| {
                    P3dError::invalid_source("DDS block offset overflowed")
                })?;
            let block_end = block_start
                .checked_add(block_size)
                .ok_or_else(|| {
                    P3dError::invalid_source("DDS block end overflowed")
                })?;
            let block = encoded.get(block_start..block_end).ok_or_else(|| {
                P3dError::invalid_source("DDS block payload is truncated")
            })?;
            let pixels = decode_block(block, format)?;
            copy_block(&pixels, block_x, block_y, width, rgba)?;
        }
    }
    Ok(())
}

fn decode_block(
    block: &[u8],
    format: BlockFormat,
) -> Result<[[u8; 4]; 16], P3dError> {
    match format {
        BlockFormat::Dxt1 => decode_color_block(block, true, [u8::MAX; 16]),
        BlockFormat::Dxt3 => {
            let alpha = decode_dxt3_alpha(block)?;
            let colors = block.get(8..16).ok_or_else(|| {
                P3dError::invalid_source("DXT3 color block is truncated")
            })?;
            decode_color_block(colors, false, alpha)
        },
        BlockFormat::Dxt5 => {
            let alpha = decode_dxt5_alpha(block)?;
            let colors = block.get(8..16).ok_or_else(|| {
                P3dError::invalid_source("DXT5 color block is truncated")
            })?;
            decode_color_block(colors, false, alpha)
        },
    }
}

fn decode_color_block(
    block: &[u8],
    bc1_transparency: bool,
    alpha: [u8; 16],
) -> Result<[[u8; 4]; 16], P3dError> {
    let color0 = required_u16(block, 0, "DXT color endpoint 0 is missing")?;
    let color1 = required_u16(block, 2, "DXT color endpoint 1 is missing")?;
    let indices = required_u32(block, 4, "DXT color indices are missing")?;
    let palette = color_palette(color0, color1, bc1_transparency);
    let mut output = [[0_u8; 4]; 16];
    for (pixel, value) in output.iter_mut().enumerate() {
        let shift = u32::try_from(pixel)
            .ok()
            .and_then(|item| item.checked_mul(2))
            .ok_or_else(|| {
                P3dError::invalid_source("DXT color shift overflowed")
            })?;
        let palette_index =
            usize::try_from((indices >> shift) & 0x3).map_err(|error| {
                P3dError::invalid_source(format!(
                    "DXT color index exceeds usize: {error}"
                ))
            })?;
        let color = palette.get(palette_index).ok_or_else(|| {
            P3dError::invalid_source("DXT color index exceeds palette")
        })?;
        let pixel_alpha = if bc1_transparency && color[3] == 0 {
            0
        } else {
            *alpha.get(pixel).ok_or_else(|| {
                P3dError::invalid_source("DXT alpha index exceeds block")
            })?
        };
        *value = [color[0], color[1], color[2], pixel_alpha];
    }
    Ok(output)
}

fn color_palette(
    color0: u16,
    color1: u16,
    bc1_transparency: bool,
) -> [[u8; 4]; 4] {
    let first = rgb565(color0);
    let second = rgb565(color1);
    let mut palette = [
        [first[0], first[1], first[2], u8::MAX],
        [second[0], second[1], second[2], u8::MAX],
        [0; 4],
        [0; 4],
    ];
    if !bc1_transparency || color0 > color1 {
        palette[2] = interpolate_color(first, second, 2, 1, 3);
        palette[3] = interpolate_color(first, second, 1, 2, 3);
    } else {
        palette[2] = interpolate_color(first, second, 1, 1, 2);
        palette[3] = [0, 0, 0, 0];
    }
    palette
}

fn rgb565(value: u16) -> [u8; 3] {
    let red = u8::try_from((value >> 11) & 0x1f).unwrap_or_default();
    let green = u8::try_from((value >> 5) & 0x3f).unwrap_or_default();
    let blue = u8::try_from(value & 0x1f).unwrap_or_default();
    [
        (red << 3) | (red >> 2),
        (green << 2) | (green >> 4),
        (blue << 3) | (blue >> 2),
    ]
}

fn interpolate_color(
    first: [u8; 3],
    second: [u8; 3],
    first_weight: u32,
    second_weight: u32,
    divisor: u32,
) -> [u8; 4] {
    let channel = |index: usize| {
        let first_value = first.get(index).copied().unwrap_or_default();
        let second_value = second.get(index).copied().unwrap_or_default();
        let numerator = u32::from(first_value)
            .saturating_mul(first_weight)
            .saturating_add(
                u32::from(second_value).saturating_mul(second_weight),
            );
        u8::try_from(numerator / divisor).unwrap_or(u8::MAX)
    };
    [channel(0), channel(1), channel(2), u8::MAX]
}

fn decode_dxt3_alpha(block: &[u8]) -> Result<[u8; 16], P3dError> {
    let encoded = required_u64(block, 0, "DXT3 alpha payload is missing")?;
    let mut output = [0_u8; 16];
    for (pixel, value) in output.iter_mut().enumerate() {
        let shift = u32::try_from(pixel)
            .ok()
            .and_then(|item| item.checked_mul(4))
            .ok_or_else(|| {
                P3dError::invalid_source("DXT3 alpha shift overflowed")
            })?;
        let nibble =
            u8::try_from((encoded >> shift) & 0xf).map_err(|error| {
                P3dError::invalid_source(format!(
                    "DXT3 alpha nibble exceeds u8: {error}"
                ))
            })?;
        *value = nibble.saturating_mul(17);
    }
    Ok(output)
}

fn decode_dxt5_alpha(block: &[u8]) -> Result<[u8; 16], P3dError> {
    let alpha0 = *block.first().ok_or_else(|| {
        P3dError::invalid_source("DXT5 alpha endpoint 0 is missing")
    })?;
    let alpha1 = *block.get(1).ok_or_else(|| {
        P3dError::invalid_source("DXT5 alpha endpoint 1 is missing")
    })?;
    let index_bytes = block.get(2..8).ok_or_else(|| {
        P3dError::invalid_source("DXT5 alpha indices are missing")
    })?;
    let mut packed = [0_u8; 8];
    packed
        .get_mut(..6)
        .ok_or_else(|| P3dError::invalid_source("DXT5 alpha staging failed"))?
        .copy_from_slice(index_bytes);
    let indices = u64::from_le_bytes(packed);
    let palette = alpha_palette(alpha0, alpha1);
    let mut output = [0_u8; 16];
    for (pixel, value) in output.iter_mut().enumerate() {
        let shift = u32::try_from(pixel)
            .ok()
            .and_then(|item| item.checked_mul(3))
            .ok_or_else(|| {
                P3dError::invalid_source("DXT5 alpha shift overflowed")
            })?;
        let palette_index =
            usize::try_from((indices >> shift) & 0x7).map_err(|error| {
                P3dError::invalid_source(format!(
                    "DXT5 alpha index exceeds usize: {error}"
                ))
            })?;
        *value = *palette.get(palette_index).ok_or_else(|| {
            P3dError::invalid_source("DXT5 alpha index exceeds palette")
        })?;
    }
    Ok(output)
}

fn alpha_palette(alpha0: u8, alpha1: u8) -> [u8; 8] {
    let mut palette = [alpha0, alpha1, 0, 0, 0, 0, 0, 0];
    if alpha0 > alpha1 {
        for index in 1_u32..=6 {
            let first_weight = 7_u32.saturating_sub(index);
            let numerator = u32::from(alpha0)
                .saturating_mul(first_weight)
                .saturating_add(u32::from(alpha1).saturating_mul(index));
            let target = usize::try_from(index.saturating_add(1)).unwrap_or(7);
            if let Some(value) = palette.get_mut(target) {
                *value = u8::try_from(numerator / 7).unwrap_or(u8::MAX);
            }
        }
    } else {
        for index in 1_u32..=4 {
            let first_weight = 5_u32.saturating_sub(index);
            let numerator = u32::from(alpha0)
                .saturating_mul(first_weight)
                .saturating_add(u32::from(alpha1).saturating_mul(index));
            let target = usize::try_from(index.saturating_add(1)).unwrap_or(5);
            if let Some(value) = palette.get_mut(target) {
                *value = u8::try_from(numerator / 5).unwrap_or(u8::MAX);
            }
        }
        palette[6] = 0;
        palette[7] = u8::MAX;
    }
    palette
}

fn copy_block(
    block: &[[u8; 4]; 16],
    block_x: usize,
    block_y: usize,
    width: usize,
    rgba: &mut [u8],
) -> Result<(), P3dError> {
    for local_y in 0..4_usize {
        for local_x in 0..4_usize {
            let source_index = local_y
                .checked_mul(4)
                .and_then(|value| value.checked_add(local_x))
                .ok_or_else(|| {
                    P3dError::invalid_source("DDS local pixel overflowed")
                })?;
            let x = block_x
                .checked_mul(4)
                .and_then(|value| value.checked_add(local_x))
                .ok_or_else(|| {
                    P3dError::invalid_source("DDS pixel x overflowed")
                })?;
            let y = block_y
                .checked_mul(4)
                .and_then(|value| value.checked_add(local_y))
                .ok_or_else(|| {
                    P3dError::invalid_source("DDS pixel y overflowed")
                })?;
            let destination = y
                .checked_mul(width)
                .and_then(|value| value.checked_add(x))
                .and_then(|value| value.checked_mul(4))
                .ok_or_else(|| {
                    P3dError::invalid_source("DDS RGBA offset overflowed")
                })?;
            let end = destination.checked_add(4).ok_or_else(|| {
                P3dError::invalid_source("DDS RGBA end overflowed")
            })?;
            let target = rgba.get_mut(destination..end).ok_or_else(|| {
                P3dError::invalid_source(
                    "DDS RGBA destination is out of bounds",
                )
            })?;
            let source = block.get(source_index).ok_or_else(|| {
                P3dError::invalid_source(
                    "DDS decoded block index is out of bounds",
                )
            })?;
            target.copy_from_slice(source);
        }
    }
    Ok(())
}

fn read_u32(bytes: &[u8], offset: usize) -> Option<u32> {
    let end = offset.checked_add(4)?;
    let raw: [u8; 4] = bytes.get(offset..end)?.try_into().ok()?;
    Some(u32::from_le_bytes(raw))
}

fn required_u32(
    bytes: &[u8],
    offset: usize,
    message: &str,
) -> Result<u32, P3dError> {
    read_u32(bytes, offset).ok_or_else(|| P3dError::invalid_source(message))
}

fn required_u16(
    bytes: &[u8],
    offset: usize,
    message: &str,
) -> Result<u16, P3dError> {
    let end = offset
        .checked_add(2)
        .ok_or_else(|| P3dError::invalid_source(message))?;
    let raw: [u8; 2] = bytes
        .get(offset..end)
        .ok_or_else(|| P3dError::invalid_source(message))?
        .try_into()
        .map_err(|error| {
            P3dError::invalid_source(format!("{message}: {error}"))
        })?;
    Ok(u16::from_le_bytes(raw))
}

fn required_u64(
    bytes: &[u8],
    offset: usize,
    message: &str,
) -> Result<u64, P3dError> {
    let end = offset
        .checked_add(8)
        .ok_or_else(|| P3dError::invalid_source(message))?;
    let raw: [u8; 8] = bytes
        .get(offset..end)
        .ok_or_else(|| P3dError::invalid_source(message))?
        .try_into()
        .map_err(|error| {
            P3dError::invalid_source(format!("{message}: {error}"))
        })?;
    Ok(u64::from_le_bytes(raw))
}

#[cfg(test)]
#[path = "../../../../../tests/formats/p3d/unit/adapter-outbound/dds_tests.rs"]
mod tests;

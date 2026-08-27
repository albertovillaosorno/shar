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
//   - Narrow legacy DDS decoder unit tests.
// - Must-Not:
//   - Depend on proprietary fixtures or external image decoders.
// - Allows:
//   - Synthetic DXT1, DXT3, DXT5 blocks and malformed header cases.
// - Split-When:
//   - Another DDS family gains independent fixture ownership.
// - Merge-When:
//   - Another test module owns the identical private decoder boundary.
// - Summary:
//   - Pins deterministic block decode and fail-closed validation.
// - Description:
//   - Uses synthetic redistributable DDS bytes only.
// - Usage:
//   - Included by the DDS adapter under cfg(test).
// - Defaults:
//   - Unsupported inputs fail explicitly.
//

//! Narrow legacy DDS decoder tests.

// CSpell:ignore dxt xfedc ffff

use super::{DecodedRgbaImage, decode_legacy_dds};

fn put(bytes: &mut [u8], offset: usize, value: &[u8]) {
    let end = offset.saturating_add(value.len());
    let target = bytes.get_mut(offset..end);
    assert!(target.is_some(), "synthetic DDS field must fit its fixed header");
    if let Some(target) = target {
        target.copy_from_slice(value);
    }
}

fn dds(fourcc: [u8; 4], block: &[u8]) -> Vec<u8> {
    let mut bytes = vec![0_u8; 128];
    put(&mut bytes, 0, b"DDS ");
    put(&mut bytes, 4, &124_u32.to_le_bytes());
    put(&mut bytes, 8, &0x000a_1007_u32.to_le_bytes());
    put(&mut bytes, 12, &4_u32.to_le_bytes());
    put(&mut bytes, 16, &4_u32.to_le_bytes());
    let size = u32::try_from(block.len()).unwrap_or(u32::MAX);
    put(&mut bytes, 20, &size.to_le_bytes());
    put(&mut bytes, 76, &32_u32.to_le_bytes());
    put(&mut bytes, 80, &4_u32.to_le_bytes());
    put(&mut bytes, 84, &fourcc);
    put(&mut bytes, 108, &0x0040_1008_u32.to_le_bytes());
    bytes.extend_from_slice(block);
    bytes
}

fn assert_pixel(image: &DecodedRgbaImage, pixel: usize, expected: [u8; 4]) {
    let start = pixel.saturating_mul(4);
    let end = start.saturating_add(4);
    assert_eq!(image.rgba.get(start..end), Some(expected.as_slice()));
}

#[test]
fn dxt1_decodes_four_color_palette() -> Result<(), String> {
    let mut block = [0_u8; 8];
    put(&mut block, 0, &0xf800_u16.to_le_bytes());
    put(&mut block, 2, &0x07e0_u16.to_le_bytes());
    put(&mut block, 4, &0xe4_u32.to_le_bytes());
    let image = decode_legacy_dds(&dds(*b"DXT1", &block))
        .map_err(|error| error.to_string())?;
    assert_eq!((image.width, image.height), (4, 4));
    assert_pixel(&image, 0, [255, 0, 0, 255]);
    assert_pixel(&image, 1, [0, 255, 0, 255]);
    assert_pixel(&image, 2, [170, 85, 0, 255]);
    assert_pixel(&image, 3, [85, 170, 0, 255]);
    Ok(())
}

#[test]
fn dxt1_preserves_one_bit_transparency_mode() -> Result<(), String> {
    let mut block = [0_u8; 8];
    put(&mut block, 0, &0x001f_u16.to_le_bytes());
    put(&mut block, 2, &0xf800_u16.to_le_bytes());
    put(&mut block, 4, &0xe4_u32.to_le_bytes());
    let image = decode_legacy_dds(&dds(*b"DXT1", &block))
        .map_err(|error| error.to_string())?;
    assert_pixel(&image, 2, [127, 0, 127, 255]);
    assert_pixel(&image, 3, [0, 0, 0, 0]);
    Ok(())
}

#[test]
fn dxt3_expands_explicit_alpha_nibbles() -> Result<(), String> {
    let mut block = [0_u8; 16];
    put(&mut block, 0, &0xfedc_ba98_7654_3210_u64.to_le_bytes());
    put(&mut block, 8, &u16::MAX.to_le_bytes());
    put(&mut block, 10, &0_u16.to_le_bytes());
    let image = decode_legacy_dds(&dds(*b"DXT3", &block))
        .map_err(|error| error.to_string())?;
    for pixel in 0..16_usize {
        let alpha = u8::try_from(pixel).unwrap_or(u8::MAX).saturating_mul(17);
        assert_pixel(&image, pixel, [255, 255, 255, alpha]);
    }
    Ok(())
}

#[test]
fn dxt5_interpolates_eight_alpha_values() -> Result<(), String> {
    let mut block = [0_u8; 16];
    block[0] = 255;
    block[1] = 0;
    let mut indices = 0_u64;
    for pixel in 0..16_u32 {
        indices |= u64::from(pixel % 8) << pixel.saturating_mul(3);
    }
    let index_bytes = indices.to_le_bytes();
    let alpha_indices = index_bytes.get(..6);
    assert!(alpha_indices.is_some(), "synthetic DXT5 indices must fit");
    if let Some(alpha_indices) = alpha_indices {
        put(&mut block, 2, alpha_indices);
    }
    put(&mut block, 8, &u16::MAX.to_le_bytes());
    let image = decode_legacy_dds(&dds(*b"DXT5", &block))
        .map_err(|error| error.to_string())?;
    let expected = [255, 0, 218, 182, 145, 109, 72, 36];
    for (pixel, alpha) in expected.into_iter().enumerate() {
        assert_pixel(&image, pixel, [255, 255, 255, alpha]);
    }
    Ok(())
}

#[test]
fn rejects_mips_unaligned_dimensions_and_trailing_bytes() {
    let block = [0_u8; 8];
    let mut mip = dds(*b"DXT1", &block);
    put(&mut mip, 28, &1_u32.to_le_bytes());
    assert!(decode_legacy_dds(&mip).is_err());

    let mut unaligned = dds(*b"DXT1", &block);
    put(&mut unaligned, 16, &3_u32.to_le_bytes());
    assert!(decode_legacy_dds(&unaligned).is_err());

    let mut trailing = dds(*b"DXT1", &block);
    trailing.push(0);
    assert!(decode_legacy_dds(&trailing).is_err());
}

#[test]
fn rejects_unknown_fourcc_and_wrong_linear_size() {
    let block = [0_u8; 8];
    assert!(decode_legacy_dds(&dds(*b"DX10", &block)).is_err());

    let mut wrong_size = dds(*b"DXT1", &block);
    put(&mut wrong_size, 20, &16_u32.to_le_bytes());
    assert!(decode_legacy_dds(&wrong_size).is_err());
}

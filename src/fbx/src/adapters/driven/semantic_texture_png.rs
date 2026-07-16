// File:
//   - semantic_texture_png.rs
// Path:
//   - src/fbx/src/adapters/driven/semantic_texture_png.rs
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
//   - Deterministic PNG byte decoding into the semantic RGBA image domain and
//   - deterministic RGBA PNG byte encoding.
// - Must-Not:
//   - Classify characters, change pixels, access the filesystem, or invoke an
//   - external image or content-authoring application.
// - Allows:
//   - Bounded PNG container validation, normalized eight-bit decoding, and sRGB
//   - RGBA encoding.
// - Split-When:
//   - Filesystem publication or another image container needs its own adapter.
// - Merge-When:
//   - Another driven adapter owns the same PNG byte contract.
// - Summary:
//   - Repository-owned semantic texture PNG adapter.
// - Description:
//   - Converts legacy indexed PNG and modern RGBA PNG bytes through the focused
//   - pure-Rust png crate without changing domain pixels.
// - Usage:
//   - Used by semantic character texture artifact adapters and round-trip
//   - tests.
// - Defaults:
//   - Decode expands to eight-bit channels and encode writes eight-bit RGBA.
//
// ADRs:
// - docs/adr/fbx/export/character-semantic-texture-rig-and-outfit-contract.md
// - docs/adr/fbx/export/fbx-output-contract-boundary.md
//
// Large file:
//   - true
//   - Reason: bounded decode, color expansion, deterministic encode, and
//   - adapter error mapping form one image-container boundary.
//

//! Deterministic semantic texture PNG byte adapter.
#![expect(
    clippy::indexing_slicing,
    reason = "PNG chunk and scanline indices are bounded by validated \
              dimensions and chunk lengths."
)]

use std::io::Cursor;

use crate::domain::texture::semantic::{Rgba8, RgbaImage, RgbaImageError};

/// PNG adapter failure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SemanticPngError {
    /// PNG decoding failed.
    Decode(String),
    /// Decoder could not declare a bounded output buffer size.
    MissingOutputBufferSize,
    /// Normalized output used an unsupported color or bit-depth combination.
    UnsupportedOutput {
        /// Decoder output color type.
        color_type: png::ColorType,
        /// Decoder output bit depth.
        bit_depth: png::BitDepth,
    },
    /// Normalized pixel bytes were not divisible by the declared channel count.
    MalformedPixelBuffer,
    /// Domain image validation failed.
    Image(RgbaImageError),
    /// PNG encoding failed.
    Encode(String),
}

impl From<RgbaImageError> for SemanticPngError {
    fn from(error: RgbaImageError) -> Self {
        Self::Image(error)
    }
}

/// Decode one PNG byte sequence into exact eight-bit RGBA pixels.
///
/// # Errors
///
/// Returns an error for malformed PNG, unsupported normalized output, malformed
/// pixel storage, or invalid image dimensions.
pub fn decode_png_bytes(bytes: &[u8]) -> Result<RgbaImage, SemanticPngError> {
    let mut decoder = png::Decoder::new(Cursor::new(bytes));
    decoder.set_transformations(png::Transformations::normalize_to_color8());
    let mut reader = decoder
        .read_info()
        .map_err(|error| SemanticPngError::Decode(error.to_string()))?;
    let buffer_size = reader
        .output_buffer_size()
        .ok_or(SemanticPngError::MissingOutputBufferSize)?;
    let mut buffer = vec![0_u8; buffer_size];
    let output = reader
        .next_frame(&mut buffer)
        .map_err(|error| SemanticPngError::Decode(error.to_string()))?;
    if output.bit_depth != png::BitDepth::Eight {
        return Err(
            SemanticPngError::UnsupportedOutput {
                color_type: output.color_type,
                bit_depth: output.bit_depth,
            },
        );
    }
    let pixels = rgba_pixels(
        output.color_type,
        &buffer[..output.buffer_size()],
    )?;
    RgbaImage::new(
        output.width,
        output.height,
        pixels,
    )
    .map_err(Into::into)
}

/// Encode one exact RGBA image into deterministic eight-bit sRGB PNG bytes.
///
/// # Errors
///
/// Returns an error when PNG header or image-data encoding fails.
pub fn encode_png_bytes(
    image: &RgbaImage
) -> Result<Vec<u8>, SemanticPngError> {
    let mut output = Vec::new();
    {
        let mut encoder = png::Encoder::new(
            &mut output,
            image.width(),
            image.height(),
        );
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.set_compression(png::Compression::Balanced);
        encoder.set_source_srgb(png::SrgbRenderingIntent::Perceptual);
        let mut writer = encoder
            .write_header()
            .map_err(|error| SemanticPngError::Encode(error.to_string()))?;
        writer
            .write_image_data(&image.rgba_bytes())
            .map_err(|error| SemanticPngError::Encode(error.to_string()))?;
    }
    Ok(output)
}

/// Expand normalized decoder output into exact RGBA pixels.
fn rgba_pixels(
    color_type: png::ColorType,
    bytes: &[u8],
) -> Result<Vec<Rgba8>, SemanticPngError> {
    let channels = color_type.samples();
    if channels == 0
        || !bytes
            .len()
            .is_multiple_of(channels)
    {
        return Err(SemanticPngError::MalformedPixelBuffer);
    }
    bytes
        .chunks_exact(channels)
        .map(
            |chunk| {
                rgba_pixel(
                    color_type, chunk,
                )
            },
        )
        .collect()
}

/// Expand one normalized pixel to RGBA.
const fn rgba_pixel(
    color_type: png::ColorType,
    channels: &[u8],
) -> Result<Rgba8, SemanticPngError> {
    let color = match color_type {
        png::ColorType::Grayscale => Rgba8::new(
            channels[0],
            channels[0],
            channels[0],
            u8::MAX,
        ),
        png::ColorType::GrayscaleAlpha => Rgba8::new(
            channels[0],
            channels[0],
            channels[0],
            channels[1],
        ),
        png::ColorType::Rgb => Rgba8::new(
            channels[0],
            channels[1],
            channels[2],
            u8::MAX,
        ),
        png::ColorType::Rgba => Rgba8::new(
            channels[0],
            channels[1],
            channels[2],
            channels[3],
        ),
        png::ColorType::Indexed => {
            return Err(
                SemanticPngError::UnsupportedOutput {
                    color_type,
                    bit_depth: png::BitDepth::Eight,
                },
            );
        }
    };
    Ok(color)
}

// File:
//   - image.rs
// Path:
//   - src/p3d/src/adapters/driven/image.rs
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
//   - Pure image signature classification for P3D embedded payloads.
// - Must-Not:
//   - Read files, choose output paths, or publish recovered artifacts.
// - Allows:
//   - Classify supported PNG, BMP, DDS, and TGA byte signatures.
// - Split-When:
//   - Another image family requires independently versioned validation.
// - Merge-When:
//   - Image signature evidence no longer has multiple adapter consumers.
// - Summary:
//   - Classifies supported P3D image payloads.
// - Description:
//   - Provides one signature SSOT for extraction and cache validation.
// - Usage:
//   - Called by the package extractor and cached artifact validator.
// - Defaults:
//   - Unknown or truncated byte streams return no image extension.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Pure image signature classification for P3D embedded payloads.
//!
//! Extraction and cache reuse share this boundary so declared media types
//! cannot diverge from the bytes that are actually published.

#[path = "image_bmp.rs"]
mod bmp;

use self::bmp::looks_like_bmp;

/// Complete PNG file signature.
const PNG_SIGNATURE: [u8; 8] = [
    0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a,
];
/// Big-endian byte form of the required thirteen-byte IHDR payload.
const PNG_IHDR_LENGTH_BYTES: [u8; 4] = [
    0, 0, 0, 13,
];
/// Four-byte zero value rejected for PNG width and height.
const PNG_ZERO_DIMENSION_BYTES: [u8; 4] = [
    0, 0, 0, 0,
];

/// Returns the supported lowercase extension identified by payload bytes.
pub(super) fn detect_image_extension(payload: &[u8]) -> Option<&'static str> {
    if looks_like_png(payload) {
        return Some("png");
    }
    if looks_like_bmp(payload) {
        return Some("bmp");
    }
    if looks_like_dds(payload) {
        return Some("dds");
    }
    if looks_like_tga(payload) {
        return Some("tga");
    }
    None
}

/// Returns whether one payload has a complete PNG signature and IHDR chunk.
fn looks_like_png(payload: &[u8]) -> bool {
    let has_ihdr_length = has_bytes_at(
        payload,
        8,
        &PNG_IHDR_LENGTH_BYTES,
    );
    let has_ihdr_type = has_bytes_at(
        payload, 12, b"IHDR",
    );
    let has_width = !has_bytes_at(
        payload,
        16,
        &PNG_ZERO_DIMENSION_BYTES,
    );
    let has_height = !has_bytes_at(
        payload,
        20,
        &PNG_ZERO_DIMENSION_BYTES,
    );
    payload.len() >= 33
        && payload.starts_with(&PNG_SIGNATURE)
        && has_ihdr_length
        && has_ihdr_type
        && has_width
        && has_height
}

/// Returns whether one payload has complete legacy DDS header evidence.
fn looks_like_dds(payload: &[u8]) -> bool {
    payload.len() >= 128
        && payload.starts_with(b"DDS ")
        && has_bytes_at(
            payload,
            4,
            b"|\x00\x00\x00",
        )
        && has_bytes_at(
            payload,
            76,
            b" \x00\x00\x00",
        )
        && !has_bytes_at(
            payload,
            12,
            b"\x00\x00\x00\x00",
        )
        && !has_bytes_at(
            payload,
            16,
            b"\x00\x00\x00\x00",
        )
}

/// Returns whether one exact byte sequence begins at a checked payload offset.
fn has_bytes_at(
    payload: &[u8],
    offset: usize,
    expected: &[u8],
) -> bool {
    let Some(end) = offset.checked_add(expected.len()) else {
        return false;
    };
    payload.get(offset..end) == Some(expected)
}

/// Returns whether one payload has a supported TGA image type byte.
fn looks_like_tga(payload: &[u8]) -> bool {
    payload.len() > 18
        && payload
            .get(2)
            .is_some_and(
                |image_type| {
                    matches!(
                        *image_type,
                        1 | 2 | 3 | 9 | 10 | 11
                    )
                },
            )
        && !has_bytes_at(
            payload,
            12,
            b"\x00\x00",
        )
        && !has_bytes_at(
            payload,
            14,
            b"\x00\x00",
        )
        && tga_pixel_depth_is_supported(payload)
        && tga_color_map_flag_is_supported(payload)
}

/// Returns whether one TGA image type agrees with the color-map flag.
fn tga_color_map_flag_is_supported(payload: &[u8]) -> bool {
    let Some(color_map_type) = payload.get(1) else {
        return false;
    };
    let Some(image_type) = payload.get(2) else {
        return false;
    };
    match *image_type {
        1 | 9 => {
            *color_map_type == 1
                && !has_bytes_at(
                    payload,
                    5,
                    b"\x00\x00",
                )
                && tga_palette_depth_is_supported(payload)
        }
        2 | 3 | 10 | 11 => *color_map_type == 0,
        _ => false,
    }
}

/// Returns whether a TGA palette uses a supported entry depth.
fn tga_palette_depth_is_supported(payload: &[u8]) -> bool {
    payload
        .get(7)
        .is_some_and(
            |entry_depth| {
                matches!(
                    *entry_depth,
                    15 | 16 | 24 | 32
                )
            },
        )
}

/// Returns whether one TGA image type supports the declared pixel depth.
fn tga_pixel_depth_is_supported(payload: &[u8]) -> bool {
    let Some(image_type) = payload.get(2) else {
        return false;
    };
    let Some(pixel_depth) = payload.get(16) else {
        return false;
    };
    match *image_type {
        1 | 9 => matches!(
            *pixel_depth,
            8 | 16
        ),
        2 | 10 => matches!(
            *pixel_depth,
            15 | 16 | 24 | 32
        ),
        3 | 11 => matches!(
            *pixel_depth,
            8 | 16
        ),
        _ => false,
    }
}

#[cfg(test)]
#[path = "image_tests.rs"]
mod tests;

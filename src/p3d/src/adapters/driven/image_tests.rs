// File:
//   - image_tests.rs
// Path:
//   - src/p3d/src/adapters/driven/image_tests.rs
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
//   - Regression coverage for supported P3D image signatures.
// - Must-Not:
//   - Read files, decode image pixels, or test extraction orchestration.
// - Allows:
//   - Classify independently authored synthetic image header bytes.
// - Split-When:
//   - One image family needs fixtures beyond compact signature bytes.
// - Merge-When:
//   - Signature classification no longer has independent behavior.
// - Summary:
//   - P3D image signature regressions.
// - Description:
//   - Verifies complete signatures and rejects misleading prefixes.
// - Usage:
//   - Included by image.rs under cfg(test).
// - Defaults:
//   - Test inputs are synthetic and contain no third-party image content.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Regression tests for P3D image signature classification.
//!
//! Prefixes must not be accepted unless they carry the complete structural
//! signature required by the declared image container.

use super::detect_image_extension;

#[test]
fn png_requires_the_complete_eight_byte_signature() {
    let misleading = detect_image_extension(b"\x89PNG\0\0\0\0");
    let mut complete_bytes = [0_u8; 33];
    complete_bytes[..8].copy_from_slice(b"\x89PNG\r\n\x1a\n");
    complete_bytes[8..12].copy_from_slice(&13_u32.to_be_bytes());
    complete_bytes[12..16].copy_from_slice(b"IHDR");
    complete_bytes[16..20].copy_from_slice(&1_u32.to_be_bytes());
    complete_bytes[20..24].copy_from_slice(&1_u32.to_be_bytes());
    let complete = detect_image_extension(&complete_bytes);
    assert_eq!(
        misleading,
        None
    );
    assert_eq!(
        complete,
        Some("png")
    );
}

#[test]
fn dds_requires_the_complete_legacy_header() {
    let truncated = detect_image_extension(b"DDS ");
    let mut complete_bytes = [0_u8; 128];
    complete_bytes[..4].copy_from_slice(b"DDS ");
    complete_bytes[4..8].copy_from_slice(&124_u32.to_le_bytes());
    complete_bytes[12..16].copy_from_slice(&1_u32.to_le_bytes());
    complete_bytes[16..20].copy_from_slice(&1_u32.to_le_bytes());
    complete_bytes[76..80].copy_from_slice(&32_u32.to_le_bytes());
    let complete = detect_image_extension(&complete_bytes);
    assert_eq!(
        truncated,
        None
    );
    assert_eq!(
        complete,
        Some("dds")
    );
}

#[test]
fn tga_requires_nonzero_dimensions() {
    let mut header = [0_u8; 19];
    header[2] = 2;
    let zero_sized = detect_image_extension(&header);
    header[12] = 1;
    header[14] = 1;
    header[16] = 24;
    let nonzero = detect_image_extension(&header);
    assert_eq!(
        zero_sized,
        None
    );
    assert_eq!(
        nonzero,
        Some("tga")
    );
}

#[test]
fn tga_requires_a_compatible_pixel_depth() {
    let mut header = [0_u8; 19];
    header[2] = 2;
    header[12] = 1;
    header[14] = 1;
    let missing_depth = detect_image_extension(&header);
    header[16] = 24;
    let supported_depth = detect_image_extension(&header);
    assert_eq!(
        missing_depth,
        None
    );
    assert_eq!(
        supported_depth,
        Some("tga")
    );
}

#[test]
fn tga_color_map_flag_matches_the_image_type() {
    let mut header = [0_u8; 19];
    header[1] = 1;
    header[2] = 2;
    header[12] = 1;
    header[14] = 1;
    header[16] = 24;
    let contradictory = detect_image_extension(&header);
    header[1] = 0;
    let consistent = detect_image_extension(&header);
    assert_eq!(
        contradictory,
        None
    );
    assert_eq!(
        consistent,
        Some("tga")
    );
}

#[test]
fn color_mapped_tga_requires_palette_entries() {
    let mut header = [0_u8; 19];
    header[1] = 1;
    header[2] = 1;
    header[12] = 1;
    header[14] = 1;
    header[16] = 8;
    let empty_palette = detect_image_extension(&header);
    header[5] = 1;
    header[7] = 24;
    let populated_palette = detect_image_extension(&header);
    assert_eq!(
        empty_palette,
        None
    );
    assert_eq!(
        populated_palette,
        Some("tga")
    );
}

#[test]
fn color_mapped_tga_requires_a_supported_palette_depth() {
    let mut header = [0_u8; 19];
    header[1] = 1;
    header[2] = 1;
    header[5] = 1;
    header[12] = 1;
    header[14] = 1;
    header[16] = 8;
    let missing_entry_depth = detect_image_extension(&header);
    header[7] = 24;
    let supported_entry_depth = detect_image_extension(&header);
    assert_eq!(
        missing_entry_depth,
        None
    );
    assert_eq!(
        supported_entry_depth,
        Some("tga")
    );
}

#[test]
fn dds_requires_nonzero_dimensions() {
    let mut header = [0_u8; 128];
    header[..4].copy_from_slice(b"DDS ");
    header[4..8].copy_from_slice(&124_u32.to_le_bytes());
    header[76..80].copy_from_slice(&32_u32.to_le_bytes());
    let zero_sized = detect_image_extension(&header);
    header[12..16].copy_from_slice(&1_u32.to_le_bytes());
    header[16..20].copy_from_slice(&1_u32.to_le_bytes());
    let nonzero = detect_image_extension(&header);
    assert_eq!(
        zero_sized,
        None
    );
    assert_eq!(
        nonzero,
        Some("dds")
    );
}

#[test]
fn png_requires_ihdr_as_the_first_chunk() {
    let signature_only = detect_image_extension(b"\x89PNG\r\n\x1a\n");
    let mut with_ihdr = [0_u8; 33];
    with_ihdr[..8].copy_from_slice(b"\x89PNG\r\n\x1a\n");
    with_ihdr[8..12].copy_from_slice(&13_u32.to_be_bytes());
    with_ihdr[12..16].copy_from_slice(b"IHDR");
    with_ihdr[16..20].copy_from_slice(&1_u32.to_be_bytes());
    with_ihdr[20..24].copy_from_slice(&1_u32.to_be_bytes());
    let complete = detect_image_extension(&with_ihdr);
    assert_eq!(
        signature_only,
        None
    );
    assert_eq!(
        complete,
        Some("png")
    );
}

#[test]
fn png_requires_nonzero_dimensions() {
    let mut header = [0_u8; 33];
    header[..8].copy_from_slice(b"\x89PNG\r\n\x1a\n");
    header[8..12].copy_from_slice(&13_u32.to_be_bytes());
    header[12..16].copy_from_slice(b"IHDR");
    let zero_sized = detect_image_extension(&header);
    header[16..20].copy_from_slice(&1_u32.to_be_bytes());
    header[20..24].copy_from_slice(&1_u32.to_be_bytes());
    let nonzero = detect_image_extension(&header);
    assert_eq!(
        zero_sized,
        None
    );
    assert_eq!(
        nonzero,
        Some("png")
    );
}

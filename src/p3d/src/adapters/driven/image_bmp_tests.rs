// File:
//   - image_bmp_tests.rs
// Path:
//   - src/p3d/src/adapters/driven/image_bmp_tests.rs
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
//   - Regression coverage for supported BMP structural evidence.
// - Must-Not:
//   - Read files, decode pixels, or test non-BMP image containers.
// - Allows:
//   - Classify independently authored synthetic BMP header and payload bytes.
// - Split-When:
//   - One DIB family requires independently maintained fixture builders.
// - Merge-When:
//   - BMP validation no longer has behavior distinct from image signatures.
// - Summary:
//   - BMP structural validation regressions.
// - Description:
//   - Verifies bounded file-header, DIB-header, and pixel-payload evidence.
// - Usage:
//   - Included by image_bmp.rs under cfg(test).
// - Defaults:
//   - Test payloads are synthetic and contain no third-party image content.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Regression tests for supported BMP structural evidence.
//!
//! Synthetic payloads isolate one file-header or DIB-header invariant at a
//! time while exercising the shared image classifier.

use super::super::detect_image_extension;

#[test]
fn bmp_requires_file_and_dib_headers() {
    let truncated = detect_image_extension(b"BM");
    let mut complete_bytes = [0_u8; 27];
    complete_bytes[..2].copy_from_slice(b"BM");
    complete_bytes[2..6].copy_from_slice(&27_u32.to_le_bytes());
    complete_bytes[10..14].copy_from_slice(&26_u32.to_le_bytes());
    complete_bytes[14..18].copy_from_slice(&12_u32.to_le_bytes());
    complete_bytes[18..20].copy_from_slice(&1_u16.to_le_bytes());
    complete_bytes[20..22].copy_from_slice(&1_u16.to_le_bytes());
    complete_bytes[22..24].copy_from_slice(&1_u16.to_le_bytes());
    complete_bytes[24..26].copy_from_slice(&24_u16.to_le_bytes());
    let complete = detect_image_extension(&complete_bytes);
    assert_eq!(
        truncated,
        None
    );
    assert_eq!(
        complete,
        Some("bmp")
    );
}

#[test]
fn bmp_requires_nonzero_dimensions() {
    let mut header = [0_u8; 27];
    header[..2].copy_from_slice(b"BM");
    header[2..6].copy_from_slice(&27_u32.to_le_bytes());
    header[10..14].copy_from_slice(&26_u32.to_le_bytes());
    header[14..18].copy_from_slice(&12_u32.to_le_bytes());
    header[22..24].copy_from_slice(&1_u16.to_le_bytes());
    header[24..26].copy_from_slice(&24_u16.to_le_bytes());
    let zero_sized = detect_image_extension(&header);
    header[18..20].copy_from_slice(&1_u16.to_le_bytes());
    header[20..22].copy_from_slice(&1_u16.to_le_bytes());
    let nonzero = detect_image_extension(&header);
    assert_eq!(
        zero_sized,
        None
    );
    assert_eq!(
        nonzero,
        Some("bmp")
    );
}

#[test]
fn bmp_requires_one_color_plane() {
    let mut header = [0_u8; 27];
    header[..2].copy_from_slice(b"BM");
    header[2..6].copy_from_slice(&27_u32.to_le_bytes());
    header[10..14].copy_from_slice(&26_u32.to_le_bytes());
    header[14..18].copy_from_slice(&12_u32.to_le_bytes());
    header[18..20].copy_from_slice(&1_u16.to_le_bytes());
    header[20..22].copy_from_slice(&1_u16.to_le_bytes());
    header[24..26].copy_from_slice(&24_u16.to_le_bytes());
    let missing_plane = detect_image_extension(&header);
    header[22..24].copy_from_slice(&1_u16.to_le_bytes());
    let one_plane = detect_image_extension(&header);
    assert_eq!(
        missing_plane,
        None
    );
    assert_eq!(
        one_plane,
        Some("bmp")
    );
}

#[test]
fn bmp_requires_a_supported_pixel_depth() {
    let mut header = [0_u8; 27];
    header[..2].copy_from_slice(b"BM");
    header[2..6].copy_from_slice(&27_u32.to_le_bytes());
    header[10..14].copy_from_slice(&26_u32.to_le_bytes());
    header[14..18].copy_from_slice(&12_u32.to_le_bytes());
    header[18..20].copy_from_slice(&1_u16.to_le_bytes());
    header[20..22].copy_from_slice(&1_u16.to_le_bytes());
    header[22..24].copy_from_slice(&1_u16.to_le_bytes());
    let missing_depth = detect_image_extension(&header);
    header[24..26].copy_from_slice(&24_u16.to_le_bytes());
    let supported_depth = detect_image_extension(&header);
    assert_eq!(
        missing_depth,
        None
    );
    assert_eq!(
        supported_depth,
        Some("bmp")
    );
}

#[test]
fn bmp_requires_pixel_payload_bytes() {
    let mut header_only = [0_u8; 26];
    header_only[..2].copy_from_slice(b"BM");
    header_only[2..6].copy_from_slice(&26_u32.to_le_bytes());
    header_only[10..14].copy_from_slice(&26_u32.to_le_bytes());
    header_only[14..18].copy_from_slice(&12_u32.to_le_bytes());
    header_only[18..20].copy_from_slice(&1_u16.to_le_bytes());
    header_only[20..22].copy_from_slice(&1_u16.to_le_bytes());
    header_only[22..24].copy_from_slice(&1_u16.to_le_bytes());
    header_only[24..26].copy_from_slice(&24_u16.to_le_bytes());
    let missing_pixels = detect_image_extension(&header_only);
    let mut with_pixels = [0_u8; 27];
    with_pixels[..26].copy_from_slice(&header_only);
    with_pixels[2..6].copy_from_slice(&27_u32.to_le_bytes());
    let present_pixels = detect_image_extension(&with_pixels);
    assert_eq!(
        missing_pixels,
        None
    );
    assert_eq!(
        present_pixels,
        Some("bmp")
    );
}

#[test]
fn bmp_declared_size_matches_the_payload() {
    let mut payload = [0_u8; 28];
    payload[..2].copy_from_slice(b"BM");
    payload[2..6].copy_from_slice(&27_u32.to_le_bytes());
    payload[10..14].copy_from_slice(&26_u32.to_le_bytes());
    payload[14..18].copy_from_slice(&12_u32.to_le_bytes());
    payload[18..20].copy_from_slice(&1_u16.to_le_bytes());
    payload[20..22].copy_from_slice(&1_u16.to_le_bytes());
    payload[22..24].copy_from_slice(&1_u16.to_le_bytes());
    payload[24..26].copy_from_slice(&24_u16.to_le_bytes());
    let undersized = detect_image_extension(&payload);
    payload[2..6].copy_from_slice(&28_u32.to_le_bytes());
    let exact = detect_image_extension(&payload);
    assert_eq!(
        undersized,
        None
    );
    assert_eq!(
        exact,
        Some("bmp")
    );
}

#[test]
fn bmp_rejects_unknown_compression_modes() {
    let mut payload = [0_u8; 55];
    payload[..2].copy_from_slice(b"BM");
    payload[2..6].copy_from_slice(&55_u32.to_le_bytes());
    payload[10..14].copy_from_slice(&54_u32.to_le_bytes());
    payload[14..18].copy_from_slice(&40_u32.to_le_bytes());
    payload[18..22].copy_from_slice(&1_i32.to_le_bytes());
    payload[22..26].copy_from_slice(&1_i32.to_le_bytes());
    payload[26..28].copy_from_slice(&1_u16.to_le_bytes());
    payload[28..30].copy_from_slice(&24_u16.to_le_bytes());
    payload[30..34].copy_from_slice(&99_u32.to_le_bytes());
    let unknown = detect_image_extension(&payload);
    payload[30..34].copy_from_slice(&0_u32.to_le_bytes());
    let known = detect_image_extension(&payload);
    assert_eq!(
        unknown,
        None
    );
    assert_eq!(
        known,
        Some("bmp")
    );
}

#[test]
fn bmp_compression_matches_the_pixel_depth() {
    let mut payload = [0_u8; 55];
    payload[..2].copy_from_slice(b"BM");
    payload[2..6].copy_from_slice(&55_u32.to_le_bytes());
    payload[10..14].copy_from_slice(&54_u32.to_le_bytes());
    payload[14..18].copy_from_slice(&40_u32.to_le_bytes());
    payload[18..22].copy_from_slice(&1_i32.to_le_bytes());
    payload[22..26].copy_from_slice(&1_i32.to_le_bytes());
    payload[26..28].copy_from_slice(&1_u16.to_le_bytes());
    payload[28..30].copy_from_slice(&24_u16.to_le_bytes());
    payload[30..34].copy_from_slice(&1_u32.to_le_bytes());
    payload[34..38].copy_from_slice(&1_u32.to_le_bytes());
    let incompatible = detect_image_extension(&payload);
    payload[28..30].copy_from_slice(&8_u16.to_le_bytes());
    let compatible = detect_image_extension(&payload);
    assert_eq!(
        incompatible,
        None
    );
    assert_eq!(
        compatible,
        Some("bmp")
    );
}

#[test]
fn top_down_bmp_rejects_rle_compression() {
    let mut payload = [0_u8; 55];
    payload[..2].copy_from_slice(b"BM");
    payload[2..6].copy_from_slice(&55_u32.to_le_bytes());
    payload[10..14].copy_from_slice(&54_u32.to_le_bytes());
    payload[14..18].copy_from_slice(&40_u32.to_le_bytes());
    payload[18..22].copy_from_slice(&1_i32.to_le_bytes());
    payload[22..26].copy_from_slice(&(-1_i32).to_le_bytes());
    payload[26..28].copy_from_slice(&1_u16.to_le_bytes());
    payload[28..30].copy_from_slice(&8_u16.to_le_bytes());
    payload[30..34].copy_from_slice(&1_u32.to_le_bytes());
    let compressed = detect_image_extension(&payload);
    payload[30..34].copy_from_slice(&0_u32.to_le_bytes());
    let uncompressed = detect_image_extension(&payload);
    assert_eq!(
        compressed,
        None
    );
    assert_eq!(
        uncompressed,
        Some("bmp")
    );
}

#[test]
fn bmp_image_size_fits_the_pixel_payload() {
    let mut payload = [0_u8; 55];
    payload[..2].copy_from_slice(b"BM");
    payload[2..6].copy_from_slice(&55_u32.to_le_bytes());
    payload[10..14].copy_from_slice(&54_u32.to_le_bytes());
    payload[14..18].copy_from_slice(&40_u32.to_le_bytes());
    payload[18..22].copy_from_slice(&1_i32.to_le_bytes());
    payload[22..26].copy_from_slice(&1_i32.to_le_bytes());
    payload[26..28].copy_from_slice(&1_u16.to_le_bytes());
    payload[28..30].copy_from_slice(&8_u16.to_le_bytes());
    payload[30..34].copy_from_slice(&1_u32.to_le_bytes());
    payload[34..38].copy_from_slice(&2_u32.to_le_bytes());
    let oversized = detect_image_extension(&payload);
    payload[34..38].copy_from_slice(&1_u32.to_le_bytes());
    let contained = detect_image_extension(&payload);
    assert_eq!(
        oversized,
        None
    );
    assert_eq!(
        contained,
        Some("bmp")
    );
}

#[test]
fn compressed_bmp_requires_a_declared_image_size() {
    let mut payload = [0_u8; 55];
    payload[..2].copy_from_slice(b"BM");
    payload[2..6].copy_from_slice(&55_u32.to_le_bytes());
    payload[10..14].copy_from_slice(&54_u32.to_le_bytes());
    payload[14..18].copy_from_slice(&40_u32.to_le_bytes());
    payload[18..22].copy_from_slice(&1_i32.to_le_bytes());
    payload[22..26].copy_from_slice(&1_i32.to_le_bytes());
    payload[26..28].copy_from_slice(&1_u16.to_le_bytes());
    payload[28..30].copy_from_slice(&8_u16.to_le_bytes());
    payload[30..34].copy_from_slice(&1_u32.to_le_bytes());
    let missing_size = detect_image_extension(&payload);
    payload[34..38].copy_from_slice(&1_u32.to_le_bytes());
    let declared_size = detect_image_extension(&payload);
    assert_eq!(
        missing_size,
        None
    );
    assert_eq!(
        declared_size,
        Some("bmp")
    );
}

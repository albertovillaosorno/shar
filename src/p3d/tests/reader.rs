// File:
//   - reader.rs
// Path:
//   - src/p3d/tests/reader.rs
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
//   - Regression coverage for shared `Pure3D` decoder reader invariants.
// - Must-Not:
//   - Access private assets, perform filesystem discovery, or duplicate decoder
//   - implementation logic.
// - Allows:
//   - Public reader inputs and caller-visible result assertions.
// - Split-When:
//   - Split when another decoder boundary needs independent fixtures.
// - Merge-When:
//   - Reader regressions no longer require a distinct integration boundary.
// - Summary:
//   - Protects malformed-input rejection and lossless `PString` decoding.
// - Description:
//   - Exercises the public decoder reader contract with synthetic byte slices.
// - Usage:
//   - Run through the p3d crate test target.
// - Defaults:
//   - No local files, generated assets, or external processes are required.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Regression coverage for shared `Pure3D` decoder reader invariants.
//!
//! Synthetic byte slices protect malformed-input rejection and lossless
//! `PString` decoding without relying on local assets.

use p3d::adapters::driven::decoders::reader::{
    Reader, SubChunk, read_instances_header, read_u32, subchunks,
};
use schoenwald_cli as _;
use schoenwald_filesystem as _;
use serde_json as _;
use shar_json_text as _;

#[test]
fn subchunks_rejects_trailing_bytes() {
    let bytes = [
        1, 0, 0, 0, 12, 0, 0, 0, 12, 0, 0, 0, 0,
    ];

    assert!(
        subchunks(
            &bytes,
            0,
            bytes.len()
        )
        .is_none()
    );
}

#[test]
fn pstring_preserves_significant_whitespace() {
    let bytes = [
        3, b' ', b'a', b' ', 0,
    ];
    let mut reader = Reader::new(
        &bytes, 0,
    );

    assert_eq!(
        reader
            .pstring()
            .as_deref(),
        Some(" a ")
    );
}

#[test]
fn pstring_does_not_discard_leading_null_data() {
    let bytes = [
        2, 0, b'a', 0,
    ];
    let mut reader = Reader::new(
        &bytes, 0,
    );

    assert_eq!(
        reader
            .pstring()
            .as_deref(),
        Some("\0a")
    );
}

#[test]
fn instances_header_falls_back_when_extended_fields_are_absent() {
    let mut bytes = vec![
        0, 0, 0, 0, 19, 0, 0, 0, 19, 0, 0, 0,
    ];
    bytes.extend_from_slice(
        &[
            6, b'l', b'e', b'g', b'a', b'c', b'y',
        ],
    );
    let child = SubChunk {
        id: 0,
        offset: 0,
        header_size: bytes.len(),
        total_size: bytes.len(),
    };

    assert_eq!(
        read_instances_header(
            &bytes, &child
        ),
        Some(
            (
                0,
                0,
                String::from("legacy")
            )
        )
    );
}

#[test]
fn pstring_rejects_invalid_utf8() {
    let bytes = [
        1, 0xff,
    ];
    let mut reader = Reader::new(
        &bytes, 0,
    );

    assert!(
        reader
            .pstring()
            .is_none()
    );
}

#[test]
fn fixed_width_reads_reject_cursor_overflow() {
    let mut integer_reader = Reader::new(
        &[],
        usize::MAX,
    );
    let mut float_reader = Reader::new(
        &[],
        usize::MAX,
    );

    assert!(
        integer_reader
            .u32()
            .is_none()
    );
    assert!(
        float_reader
            .f32()
            .is_none()
    );
}

#[test]
fn absolute_u32_read_rejects_offset_overflow() {
    assert!(
        read_u32(
            &[],
            usize::MAX
        )
        .is_none()
    );
}

#[test]
fn failed_skip_preserves_cursor_position() {
    let mut reader = Reader::new(
        &[0],
        0,
    );

    assert!(
        reader
            .skip(2)
            .is_none()
    );
    assert_eq!(
        reader.pos(),
        0
    );
}

#[test]
fn subchunks_rejects_ranges_outside_buffer() {
    assert!(
        subchunks(
            &[],
            1,
            1
        )
        .is_none()
    );
    assert!(
        subchunks(
            &[],
            usize::MAX,
            usize::MAX
        )
        .is_none()
    );
}

#[test]
fn pstring_preserves_declared_trailing_null_data() {
    let bytes = [
        2, b'a', 0,
    ];
    let mut reader = Reader::new(
        &bytes, 0,
    );

    assert_eq!(
        reader
            .pstring()
            .as_deref(),
        Some("a\0")
    );
}

#[test]
fn invalid_utf8_pstring_preserves_cursor_position() {
    let bytes = [
        1, 0xff,
    ];
    let mut reader = Reader::new(
        &bytes, 0,
    );

    assert!(
        reader
            .pstring()
            .is_none()
    );
    assert_eq!(
        reader.pos(),
        0
    );
}

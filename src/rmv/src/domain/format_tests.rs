// File:
//   - format_tests.rs
// Path:
//   - src/rmv/src/domain/format_tests.rs
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
//   - Focused regression coverage for RMV movie format classification.
// - Must-Not:
//   - Implement format classification or perform filesystem I/O.
// - Allows:
//   - Construct deterministic in-memory headers for pure format assertions.
// - Split-When:
//   - Split when one independent container family exceeds this suite.
// - Merge-When:
//   - Merge when format classification returns to a smaller focused suite.
// - Summary:
//   - RMV movie format classification regressions.
// - Description:
//   - Verifies supported signatures and mandatory container header fields.
// - Usage:
//   - Compiled only for RMV domain tests.
// - Defaults:
//   - No production behavior or external resources.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! RMV movie format classification regression coverage.
//!
//! These tests keep construction-heavy binary headers out of the production
//! classifier while exercising supported signatures, declared sizes, frame
//! metadata, dimensions, and cadence invariants.

use super::MovieKind;

/// Zeroed storage for one complete Bink header fixture.
const BINK_HEADER_STORAGE: [u8; 36] = [0_u8; 36];

fn bink_header(signature: [u8; 4]) -> [u8; 36] {
    let mut bytes = BINK_HEADER_STORAGE;
    for (target, source) in bytes
        .iter_mut()
        .zip(signature)
    {
        *target = source;
    }
    for (field_index, field) in bytes
        .chunks_mut(4)
        .enumerate()
        .skip(1)
    {
        let value = match field_index {
            1 => 28_u32,
            2 | 8 => 1,
            3 => 4,
            5 => 640,
            6 => 480,
            7 => 30,
            _ => 0,
        };
        field.copy_from_slice(&value.to_le_bytes());
    }
    bytes
}

fn set_header_field(
    bytes: &mut [u8; MovieKind::HEADER_PROBE_LEN],
    field_index: usize,
    value: u32,
) {
    for (index, field) in bytes
        .chunks_mut(4)
        .enumerate()
    {
        if index == field_index {
            field.copy_from_slice(&value.to_le_bytes());
            return;
        }
    }
}

fn assert_rejects_header_field(
    signature: [u8; 4],
    field_index: usize,
    value: u32,
) {
    let mut bytes = bink_header(signature);
    set_header_field(
        &mut bytes,
        field_index,
        value,
    );
    assert_eq!(
        MovieKind::from_bytes(&bytes),
        MovieKind::Unknown
    );
}

#[test]
fn rejects_unsupported_bink_revisions() {
    assert_eq!(
        MovieKind::from_prefix(b"BIKa"),
        MovieKind::Unknown
    );
    assert_eq!(
        MovieKind::from_prefix(b"BIKz"),
        MovieKind::Unknown
    );
    assert_eq!(
        MovieKind::from_prefix(b"KB2b"),
        MovieKind::Unknown
    );
    assert_eq!(
        MovieKind::from_prefix(b"KB2e"),
        MovieKind::Unknown
    );
    assert_eq!(
        MovieKind::from_prefix(b"BK2i"),
        MovieKind::Unknown
    );
}

#[test]
fn classifies_supported_structural_bink_headers() {
    let bink_v1 = bink_header(*b"BIKi");
    assert_eq!(
        MovieKind::from_bytes(&bink_v1),
        MovieKind::BinkV1
    );
    let bink_v2 = bink_header(*b"KB2i");
    assert_eq!(
        MovieKind::from_bytes(&bink_v2),
        MovieKind::BinkV2
    );
}

fn assert_rejects_required_fields(signature: [u8; 4]) {
    assert_rejects_header_field(
        signature, 1, 0,
    );
    assert_rejects_header_field(
        signature, 2, 0,
    );
    assert_rejects_header_field(
        signature, 2, 1_000_001,
    );
    assert_rejects_header_field(
        signature, 3, 37,
    );
    assert_rejects_header_field(
        signature, 5, 0,
    );
    assert_rejects_header_field(
        signature, 5, 7_681,
    );
    assert_rejects_header_field(
        signature, 6, 0,
    );
    assert_rejects_header_field(
        signature, 6, 4_801,
    );
    assert_rejects_header_field(
        signature, 7, 0,
    );
    assert_rejects_header_field(
        signature, 8, 0,
    );
}

#[test]
fn rejects_malformed_mandatory_bink_header_fields() {
    let bink_v1 = bink_header(*b"BIKi");
    assert_eq!(
        MovieKind::from_sized_header(
            &bink_v1[..35],
            36,
        ),
        MovieKind::Unknown
    );
    let bink_v2 = bink_header(*b"KB2i");
    assert_eq!(
        MovieKind::from_sized_header(
            &bink_v2[..35],
            36,
        ),
        MovieKind::Unknown
    );
    assert_rejects_required_fields(*b"BIKi");
    assert_rejects_required_fields(*b"KB2i");
}

#[test]
fn builds_bink_header_signature() {
    let bytes = bink_header(*b"BIKi");
    assert_eq!(
        MovieKind::from_prefix(&bytes),
        MovieKind::BinkV1
    );
}

#[test]
fn classifies_xbox_xmv_like_credit_movie_header() {
    let mut bytes = [0_u8; 32];
    bytes[12..16].copy_from_slice(b"xobX");
    assert_eq!(
        MovieKind::from_prefix(&bytes),
        MovieKind::XboxXmvLike
    );
}

#[test]
fn rejects_truncated_bink_v1_signature() {
    assert_eq!(
        MovieKind::from_prefix(b"BIK"),
        MovieKind::Unknown
    );
}

#[test]
fn rejects_truncated_bink_v2_signature() {
    assert_eq!(
        MovieKind::from_prefix(b"KB2"),
        MovieKind::Unknown
    );
}

#[test]
fn rejects_bink_signatures_without_alphabetic_version_bytes() {
    for malformed in [
        b"BIK\0".as_slice(),
        b"BIK1",
        b"KB2\0",
        b"BK2-",
    ] {
        assert_eq!(
            MovieKind::from_prefix(malformed),
            MovieKind::Unknown,
            "malformed Bink signature was accepted: {malformed:?}"
        );
    }
}

#[test]
fn rejects_rmv_substrings_outside_the_header_prefix() {
    assert_eq!(
        MovieKind::from_prefix(b"metadata-rmv-payload"),
        MovieKind::Unknown
    );
}

#[test]
fn classifies_radical_movie_header_prefix() {
    assert_eq!(
        MovieKind::from_prefix(b"rmvgcn10"),
        MovieKind::RadicalMovieHeader
    );
}

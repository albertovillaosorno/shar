// File:
//   - extract.rs
// Path:
//   - src/p3d/tests/extract.rs
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
//   - Regression coverage for public Pure3D compression preparation invariants.
// - Must-Not:
//   - Access private assets, perform filesystem discovery, or duplicate the
//   - production decompressor.
// - Allows:
//   - Synthetic compressed byte streams and public preparation assertions.
// - Split-When:
//   - Another compression family requires independent fixtures.
// - Merge-When:
//   - Compression regressions no longer require a distinct test boundary.
// - Summary:
//   - Protects fail-closed P3DZ preparation.
// - Description:
//   - Exercises public compressed-source preparation with synthetic streams.
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

//! Regression coverage for public `Pure3D` compression preparation invariants.
//!
//! Synthetic literal streams prove malformed `P3DZ` framing fails closed
//! without relying on local game assets.

use p3d::domain::prepare_p3d_bytes;
use schoenwald_cli as _;
use schoenwald_filesystem as _;
use serde_json as _;
use shar_json_text as _;

fn literal_p3dz(payload: &[u8]) -> Option<Vec<u8>> {
    let payload_len = u8::try_from(payload.len()).ok()?;
    let compressed_size = u32::try_from(
        payload
            .len()
            .checked_add(1)?,
    )
    .ok()?;
    let decompressed_size = u32::try_from(payload.len()).ok()?;
    let mut source = Vec::new();
    source.extend_from_slice(b"P3DZ");
    source.extend_from_slice(&decompressed_size.to_le_bytes());
    source.extend_from_slice(&compressed_size.to_le_bytes());
    source.extend_from_slice(&decompressed_size.to_le_bytes());
    source.push(payload_len);
    source.extend_from_slice(payload);
    Some(source)
}

const fn minimal_root() -> [u8; 12] {
    [
        0x50, 0x33, 0x44, 0xff, 12, 0, 0, 0, 12, 0, 0, 0,
    ]
}

#[test]
fn p3dz_rejects_trailing_container_bytes() -> Result<(), String> {
    let mut source = literal_p3dz(&minimal_root())
        .ok_or_else(|| String::from("fixture should encode"))?;
    source.push(0);

    if prepare_p3d_bytes(&source).is_err() {
        Ok(())
    } else {
        Err(String::from("trailing P3DZ bytes must be rejected"))
    }
}

#[test]
fn p3dz_rejects_trailing_compressed_block_bytes() -> Result<(), String> {
    let mut source = literal_p3dz(&minimal_root())
        .ok_or_else(|| String::from("fixture should encode"))?;
    let expanded_size = 14_u32.to_le_bytes();
    source
        .get_mut(8..12)
        .ok_or_else(|| String::from("compressed-size field should exist"))?
        .copy_from_slice(&expanded_size);
    source.push(0);

    if prepare_p3d_bytes(&source).is_err() {
        Ok(())
    } else {
        Err(String::from("trailing compressed block bytes must be rejected"))
    }
}

#[test]
fn p3dz_rejects_zero_sized_blocks() -> Result<(), String> {
    let source = literal_p3dz(&minimal_root())
        .ok_or_else(|| String::from("fixture should encode"))?;
    let mut with_empty_block = Vec::new();
    with_empty_block.extend_from_slice(
        source
            .get(0..8)
            .ok_or_else(|| String::from("container header should exist"))?,
    );
    with_empty_block.extend_from_slice(&[0; 8]);
    with_empty_block.extend_from_slice(
        source
            .get(8..)
            .ok_or_else(|| String::from("block stream should exist"))?,
    );

    if prepare_p3d_bytes(&with_empty_block).is_err() {
        Ok(())
    } else {
        Err(String::from("zero-sized P3DZ blocks must be rejected"))
    }
}

#[test]
fn p3dz_rejects_blocks_larger_than_declared_output() {
    let mut source = Vec::new();
    source.extend_from_slice(b"P3DZ");
    source.extend_from_slice(&12_u32.to_le_bytes());
    source.extend_from_slice(&1_u32.to_le_bytes());
    source.extend_from_slice(&13_u32.to_le_bytes());
    source.push(0);

    let result = prepare_p3d_bytes(&source);
    assert_eq!(
        result
            .err()
            .map(|error| error.to_string())
            .as_deref(),
        Some("P3DZ block exceeds declared output size")
    );
}

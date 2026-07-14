// File:
//   - chunk.rs
// Path:
//   - src/p3d/tests/chunk.rs
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
//   - Regression coverage for public Pure3D document framing invariants.
// - Must-Not:
//   - Access private assets, perform filesystem discovery, or duplicate parser
//   - implementation logic.
// - Allows:
//   - Synthetic byte streams and public parser result assertions.
// - Split-When:
//   - Another document boundary needs independently maintained fixtures.
// - Merge-When:
//   - Chunk framing regressions no longer require a distinct test boundary.
// - Summary:
//   - Protects fail-closed Pure3D document parsing.
// - Description:
//   - Exercises public chunk analysis with synthetic malformed documents.
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

//! Regression coverage for public `Pure3D` document framing invariants.
//!
//! Synthetic byte streams prove malformed child regions fail closed without
//! relying on local game assets.

use p3d::analyze_p3d;
use schoenwald_cli as _;
use schoenwald_filesystem as _;
use serde_json as _;
use shar_json_text as _;

#[test]
fn document_rejects_malformed_child_region() {
    let bytes = [
        0x50, 0x33, 0x44, 0xff, 12, 0, 0, 0, 13, 0, 0, 0, 0,
    ];

    assert!(analyze_p3d(&bytes).is_err());
}

#[test]
fn document_rejects_chunks_after_root_container() {
    let bytes = [
        0x50, 0x33, 0x44, 0xff, 12, 0, 0, 0, 12, 0, 0, 0, 1, 0, 0, 0, 12, 0, 0,
        0, 12, 0, 0, 0,
    ];

    assert!(analyze_p3d(&bytes).is_err());
}

#[test]
fn unknown_chunk_kind_uses_unknown_identity() {
    assert_eq!(
        p3d::ChunkKind::Unknown.label(),
        "unknown"
    );
}

fn nested_document(depth: usize) -> Option<Vec<u8>> {
    let chunk_count = depth.checked_add(1)?;
    let mut bytes = Vec::new();
    for level in 0..chunk_count {
        let remaining = chunk_count.checked_sub(level)?;
        let total_size_usize = remaining.checked_mul(12)?;
        let total_size_u32 = u32::try_from(total_size_usize).ok()?;
        let id = if level == 0 {
            0xff44_3350_u32
        } else {
            // cspell:disable-next-line -- xdead
            0xdead_beef_u32
        };
        bytes.extend_from_slice(&id.to_le_bytes());
        bytes.extend_from_slice(&12_u32.to_le_bytes());
        bytes.extend_from_slice(&total_size_u32.to_le_bytes());
    }
    Some(bytes)
}

#[test]
fn document_rejects_excessive_chunk_nesting() -> Result<(), String> {
    let bytes = nested_document(257)
        .ok_or_else(|| String::from("nested fixture should encode"))?;

    if analyze_p3d(&bytes).is_err() {
        Ok(())
    } else {
        Err(String::from("excessive chunk nesting must be rejected"))
    }
}

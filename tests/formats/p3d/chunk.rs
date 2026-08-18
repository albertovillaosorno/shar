// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Chunk test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Chunk test module.
// - Description:
//   - Implements the declared test module responsibility for p3d.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Chunk test module.

use p3d::analyze_p3d;
use schoenwald_cli as _;
use schoenwald_filesystem as _;
use serde_json as _;
use shar_json_text as _;
use shar_sha256 as _;

#[test]
fn document_rejects_malformed_child_region() {
    let bytes = [0x50, 0x33, 0x44, 0xff, 12, 0, 0, 0, 13, 0, 0, 0, 0];

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
    assert_eq!(p3d::ChunkKind::Unknown.label(), "unknown");
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

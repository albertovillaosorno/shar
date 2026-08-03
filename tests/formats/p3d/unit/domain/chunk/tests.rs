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
//   - Tests unit tests.
// - Must-Not:
//   - Own production behavior or broaden the tested API surface.
// - Allows:
//   - Private test fixtures and assertions for the owning source module.
// - Split-When:
//   - Split when an independent fixture family gains separate ownership.
// - Merge-When:
//   - Merge when another test module owns the identical evidence.
// - Summary:
//   - Tests unit tests.
// - Description:
//   - Preserves unit-test access through a test-only path module.
// - Usage:
//   - Included only by the owning source module under cfg(test).
// - Defaults:
//   - Test setup and assertions fail explicitly.
//

//! Tests unit tests.

use super::{Endian, P3dError, analyze_p3d, parse_range, read_u32};

#[test]
fn invalid_source_error_escapes_control_characters() {
    let error = P3dError::invalid_source("invalid\nsource");

    assert_eq!(error.to_string(), r"invalid\nsource");
}

#[test]
fn chunk_u32_reader_rejects_offset_overflow() -> Result<(), String> {
    let read = read_u32(&[], usize::MAX, Endian::Little);
    if read.is_ok() {
        return Err(String::from(
            "chunk u32 reads must reject an offset that cannot \
                 contain four bytes",
        ));
    }
    Ok(())
}

#[test]
fn empty_maximum_range_does_not_overflow() -> Result<(), String> {
    let mut chunks = Vec::new();
    parse_range(
        &[],
        Endian::Little,
        usize::MAX,
        usize::MAX,
        0,
        None,
        &mut chunks,
    )
    .map_err(|error| error.to_string())
}
#[test]
fn parses_minimal_root() -> Result<(), String> {
    let bytes = [0x50, 0x33, 0x44, 0xff, 12, 0, 0, 0, 12, 0, 0, 0];
    let document = analyze_p3d(&bytes).map_err(|error| error.to_string())?;
    if document.chunks.len() != 1 {
        return Err("minimal root must produce exactly one chunk".to_owned());
    }
    let root = document.chunks.first().ok_or_else(|| {
        "parsed document must contain the root chunk".to_owned()
    })?;
    if root.kind.label() != "root" {
        return Err("minimal root chunk must retain the root kind".to_owned());
    }
    Ok(())
}

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

use super::{bytes_to_json, offset_references_json};

#[test]
fn offset_reference_rows_are_valid_json_objects() {
    let words = [4_u32];
    let bytes = [0_u8; 8];
    let actual = offset_references_json(&words, &bytes, &[]);
    let quote = char::from(34);
    assert!(actual.starts_with(&format!("{{{quote}word_index{quote}:0,")));
    assert!(actual.contains(&format!("{quote}source_offset{quote}:0")));
    assert!(actual.contains(&format!("{quote}target_offset{quote}:4")));
    let mut suffix = String::new();
    suffix.push(quote);
    suffix.push_str("target_kind");
    suffix.push(quote);
    suffix.push(':');
    suffix.push(quote);
    suffix.push_str("aligned_data");
    suffix.push(quote);
    suffix.push('}');
    assert!(actual.ends_with(&suffix));
    assert!(!actual.contains("r##"));
}

#[test]
fn exposes_radmusic_symbols_and_offsets() {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&16_u32.to_le_bytes());
    bytes.extend_from_slice(&0_u32.to_le_bytes());
    bytes.extend_from_slice(&16_u32.to_le_bytes());
    bytes.extend_from_slice(&24_u32.to_le_bytes());
    bytes.extend_from_slice(
        b"radmusic_comp\0comp\0theme_region\0Placeholder1\0",
    );
    let json = bytes_to_json(&bytes, "sample.rms");
    assert!(json.contains("\"format_name\":\"radmusic_comp\""));
    assert!(json.contains("theme_region"));
    assert!(json.contains("Placeholder1"));
    assert!(json.contains("\"target_kind\":\"symbol\""));
}

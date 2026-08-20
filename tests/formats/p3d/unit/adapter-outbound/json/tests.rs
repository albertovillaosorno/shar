// Copyright:
//   - Copyright © 2026 Alberto Villa Osorno.
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

use super::{render_f32, validate_document, validate_json_lines};

#[test]
fn non_finite_floats_render_as_json_null() {
    assert_eq!(render_f32(f32::NAN, String::from("NaN"),), "null");
    assert_eq!(render_f32(f32::INFINITY, String::from("inf"),), "null");
    assert_eq!(render_f32(2.5, String::from("2.500"),), "2.500");
}

#[test]
fn rejects_raw_controls_and_accepts_escaped_controls() {
    let quote = char::from(34);
    let slash = char::from(92);
    let nul = char::from(0);
    let raw = format!("{{{quote}name{quote}:{quote}bad{nul}value{quote}}}");
    let escaped =
        format!("{{{quote}name{quote}:{quote}good{slash}u0000value{quote}}}");
    let first_row = format!("{{{quote}row{quote}:1}}");
    let second_row = format!("{{{quote}row{quote}:2}}");
    let rows = format!(
        "{first_row}
{second_row}
"
    );
    assert!(validate_document(raw.as_bytes(), "raw-control",).is_err());
    assert!(validate_document(escaped.as_bytes(), "escaped-control",).is_ok());
    assert!(validate_json_lines(&rows, "rows",).is_ok());
}

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

use super::super::json::JsonObject;
use super::append_summary;

#[test]
fn does_not_classify_embedded_missing_phrases() {
    assert_eq!(
        super::category_for("feature is not foundational"),
        "build-log"
    );
}

#[test]
fn does_not_classify_embedded_error_substrings() {
    assert_eq!(super::category_for("terror level increased"), "build-log");
}

#[test]
fn preserves_nonempty_error_line_whitespace() -> Result<(), String> {
    let mut json = JsonObject::new();
    append_summary(&mut json, "  ERROR failed  ");
    let output = json.finish();
    if output.contains("\"source_lines\":[\"  ERROR failed  \"]")
        && output.contains("\"raw\":\"  ERROR failed  \"")
    {
        Ok(())
    } else {
        Err(format!("error-line whitespace was lost: {output}"))
    }
}

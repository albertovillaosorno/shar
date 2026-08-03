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
//   - Escape test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Escape test module.
// - Description:
//   - Implements the declared test module responsibility for json text.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Escape test module.

#[test]
fn preserves_text_and_escapes_all_json_controls() {
    let input = format!(
        "a\"b\\c{}{}\n\r\t{}",
        char::from(8),
        char::from(12),
        char::from(0),
    );

    assert_eq!(
        shar_json_text::escape(&input),
        "a\\\"b\\\\c\\b\\f\\n\\r\\t\\u0000"
    );
}

// File:
//   - escape.rs
// Path:
//   - src/json-text/tests/escape.rs
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
//   - Regression evidence for the shared JSON text escape contract.
// - Must-Not:
//   - Test complete documents or duplicate the escaping implementation.
// - Allows:
//   - Synthetic delimiters, controls, and Unicode scalar values.
// - Summary:
//   - Proves lossless JSON string-content escaping.
//
// ADRs:
// - docs/adr/engineering/architecture/project-core-separation.md
//
// Large file:
//   - false
//

//! Regression evidence for shared JSON string-content escaping.

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

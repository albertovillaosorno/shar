// File:
//   - unicode_character_controls.rs
// Path:
//   - src/rtf/tests/unicode_character_controls.rs
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
//   - Deterministic RTF regression coverage for Unicode character controls.
// - Must-Not:
//   - Depend on private documents or parser implementation details.
// - Allows:
//   - Public fixtures and caller-visible Unicode assertions.
// - Split-When:
//   - Split when a control family needs independent fixture infrastructure.
// - Merge-When:
//   - Another RTF test module owns the same Unicode-character contract.
// - Summary:
//   - Verifies semantic Unicode characters represented by RTF control words.
// - Description:
//   - Exercises public conversion for zero-width and directional characters.
// - Usage:
//   - Executed through cargo test for the rtf crate.
// - Defaults:
//   - Fixtures remain deterministic and repository-local.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Public regression coverage for Unicode character control words.

use rtf::rtf_to_markdown;
use schoenwald_cli as _;
use schoenwald_filesystem as _;

#[test]
fn joining_controls_preserve_zero_width_characters() {
    let markdown = rtf_to_markdown(br"{\rtf1 A\zwj B\zwnj C}");

    assert_eq!(
        markdown,
        "A\u{200D}B\u{200C}C\n"
    );
}

#[test]
fn break_controls_preserve_zero_width_characters() {
    let markdown = rtf_to_markdown(br"{\rtf1 A\zwbo B\zwnbo C}");

    assert_eq!(
        markdown,
        "A\u{200B}B\u{FEFF}C\n"
    );
}

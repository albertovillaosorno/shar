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
//   - Unicode character controls test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Unicode character controls test module.
// - Description:
//   - Implements the declared test module responsibility for rtf.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Unicode character controls test module.

use rtf::rtf_to_markdown;
use schoenwald_cli as _;
use schoenwald_filesystem as _;

#[test]
fn joining_controls_preserve_zero_width_characters() {
    let markdown = rtf_to_markdown(br"{\rtf1 A\zwj B\zwnj C}");

    assert_eq!(markdown, "A\u{200D}B\u{200C}C\n");
}

#[test]
fn break_controls_preserve_zero_width_characters() {
    let markdown = rtf_to_markdown(br"{\rtf1 A\zwbo B\zwnbo C}");

    assert_eq!(markdown, "A\u{200B}B\u{FEFF}C\n");
}

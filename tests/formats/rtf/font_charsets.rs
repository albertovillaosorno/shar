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
//   - Font charsets test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Font charsets test module.
// - Description:
//   - Implements the declared test module responsibility for rtf.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Font charsets test module.

use rtf::rtf_to_markdown;
use schoenwald_cli as _;
use schoenwald_filesystem as _;

#[test]
fn ordinary_o_circumflex_is_not_rewritten_as_trademark() {
    let markdown = rtf_to_markdown(br"{\rtf1 \'d4}");

    assert_eq!(markdown, "Ô\n");
}

#[test]
fn symbol_font_trademark_stays_trademark() {
    let input = concat!(
        r"{\rtf1{\fonttbl{\f0\fcharset0 Arial;}",
        r"{\f1\fcharset2 Symbol;}}\f1 \'d4}"
    );
    let markdown = rtf_to_markdown(input.as_bytes());

    assert_eq!(markdown, "™\n");
}

#[test]
fn plain_control_restores_default_font_charset() {
    let input = concat!(
        r"{\rtf1\deff0{\fonttbl{\f0\fcharset0 Arial;}",
        r"{\f1\fcharset2 Symbol;}}\f1 \'d4\plain \'d4}"
    );
    let markdown = rtf_to_markdown(input.as_bytes());

    assert_eq!(markdown, "™Ô\n");
}

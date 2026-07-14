// File:
//   - font_charsets.rs
// Path:
//   - src/rtf/tests/font_charsets.rs
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
//   - Deterministic RTF regression coverage for font charset decoding.
// - Must-Not:
//   - Depend on private documents or implementation-specific parser state.
// - Allows:
//   - Public font-table fixtures and caller-visible Unicode assertions.
// - Split-When:
//   - Split when a charset family requires independent fixture infrastructure.
// - Merge-When:
//   - Another RTF test module owns the same font-charset contract.
// - Summary:
//   - Verifies font-aware decoding without corrupting ordinary ANSI text.
// - Description:
//   - Exercises public conversion behavior for ANSI and Symbol font runs.
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

//! Public regression coverage for RTF font charset decoding.

use rtf::rtf_to_markdown;
use schoenwald_cli as _;
use schoenwald_filesystem as _;

#[test]
fn ordinary_o_circumflex_is_not_rewritten_as_trademark() {
    let markdown = rtf_to_markdown(br"{\rtf1 \'d4}");

    assert_eq!(
        markdown,
        "Ô\n"
    );
}

#[test]
fn symbol_font_trademark_stays_trademark() {
    let input = concat!(
        r"{\rtf1{\fonttbl{\f0\fcharset0 Arial;}",
        r"{\f1\fcharset2 Symbol;}}\f1 \'d4}"
    );
    let markdown = rtf_to_markdown(input.as_bytes());

    assert_eq!(
        markdown,
        "™\n"
    );
}

#[test]
fn plain_control_restores_default_font_charset() {
    let input = concat!(
        r"{\rtf1\deff0{\fonttbl{\f0\fcharset0 Arial;}",
        r"{\f1\fcharset2 Symbol;}}\f1 \'d4\plain \'d4}"
    );
    let markdown = rtf_to_markdown(input.as_bytes());

    assert_eq!(
        markdown,
        "™Ô\n"
    );
}

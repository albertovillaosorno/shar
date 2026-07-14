// File:
//   - malformed_hex_controls.rs
// Path:
//   - src/rtf/tests/malformed_hex_controls.rs
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
//   - Deterministic RTF regression coverage for malformed hexadecimal controls.
// - Must-Not:
//   - Depend on private documents or parser implementation details.
// - Allows:
//   - Public malformed fixtures and caller-visible recovery assertions.
// - Split-When:
//   - Split when another malformed-control family needs independent fixtures.
// - Merge-When:
//   - Another RTF test module owns the same hexadecimal recovery contract.
// - Summary:
//   - Verifies malformed hexadecimal escapes cannot consume group delimiters.
// - Description:
//   - Exercises public parser recovery around structural RTF braces.
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

//! Public regression coverage for malformed hexadecimal RTF controls.

use rtf::rtf_to_markdown;
use schoenwald_cli as _;
use schoenwald_filesystem as _;

#[test]
fn malformed_hex_escape_does_not_consume_group_delimiter() {
    let markdown = rtf_to_markdown(br"{\rtf1 A{\fonttbl \'4}B}");

    assert_eq!(
        markdown,
        "AB\n"
    );
}

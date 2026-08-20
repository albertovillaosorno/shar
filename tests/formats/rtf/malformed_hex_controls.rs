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
//   - Malformed hex controls test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Malformed hex controls test module.
// - Description:
//   - Implements the declared test module responsibility for rtf.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Malformed hex controls test module.

use rtf::rtf_to_markdown;
use schoenwald_cli as _;
use schoenwald_filesystem as _;

#[test]
fn malformed_hex_escape_does_not_consume_group_delimiter() {
    let markdown = rtf_to_markdown(br"{\rtf1 A{\fonttbl \'4}B}");

    assert_eq!(markdown, "AB\n");
}

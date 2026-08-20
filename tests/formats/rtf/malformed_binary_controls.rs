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
//   - Malformed binary controls test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Malformed binary controls test module.
// - Description:
//   - Implements the declared test module responsibility for rtf.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Malformed binary controls test module.

use rtf::rtf_to_markdown;
use schoenwald_cli as _;
use schoenwald_filesystem as _;

#[test]
fn invalid_binary_lengths_hide_opaque_group_payloads() {
    let missing = rtf_to_markdown(br"{\rtf1 A{\bin payload}B}");
    let negative = rtf_to_markdown(br"{\rtf1 A{\bin-1 payload}B}");

    assert_eq!(missing, "AB\n");
    assert_eq!(negative, "AB\n");
}

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
//   - Special character controls test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Special character controls test module.
// - Description:
//   - Implements the declared test module responsibility for rtf.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Special character controls test module.

use rtf::rtf_to_markdown;
use schoenwald_cli as _;
use schoenwald_filesystem as _;

#[test]
fn nonbreaking_hyphen_remains_nonbreaking() {
    let markdown = rtf_to_markdown(br"{\rtf1 co\_operate}");

    assert_eq!(markdown, "co‑operate\n");
}

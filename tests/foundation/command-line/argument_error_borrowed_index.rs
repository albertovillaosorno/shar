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
//   - Argument error borrowed index test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Argument error borrowed index test module.
// - Description:
//   - Implements the declared test module responsibility for command line.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Argument error borrowed index test module.

use schoenwald_cli::ArgumentError;

#[test]
fn index_accessor_accepts_a_borrowed_error() {
    let accessor: fn(&ArgumentError) -> usize = ArgumentError::index;
    let error = ArgumentError::non_unicode(4);

    assert_eq!(accessor(&error), 4);
    assert_eq!(error.to_string(), "command argument 5 is not valid Unicode");
}

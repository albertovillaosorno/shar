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
//   - Argument error position test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Argument error position test module.
// - Description:
//   - Implements the declared test module responsibility for command line.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Argument error position test module.

use schoenwald_cli::ArgumentError;

#[test]
fn first_argument_is_rendered_as_position_one() {
    let error = ArgumentError::non_unicode(0);

    assert_eq!(error.to_string(), "command argument 1 is not valid Unicode");
    assert_eq!(error.index(), 0);
}

#[test]
fn maximum_argument_index_has_a_distinct_one_based_position() {
    let error = ArgumentError::non_unicode(usize::MAX);
    let expected_position = u128::try_from(usize::MAX)
        .unwrap_or(u128::MAX)
        .saturating_add(1);

    assert_eq!(
        error.to_string(),
        format!("command argument {expected_position} is not valid Unicode")
    );
}

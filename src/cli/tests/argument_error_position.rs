// File:
//   - argument_error_position.rs
// Path:
//   - src/cli/tests/argument_error_position.rs
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
//   - Regression coverage for user-facing argument positions.
// - Must-Not:
//   - Change the zero-based programmatic index contract.
// - Allows:
//   - Verify the first argument diagnostic through Display.
// - Split-When:
//   - Another argument-error rendering behavior needs separate coverage.
// - Merge-When:
//   - Argument diagnostics no longer expose positions.
// - Summary:
//   - Argument position regression.
// - Description:
//   - Proves human-facing diagnostics use one-based positions.
// - Usage:
//   - Executed by the schoenwald-cli integration test target.
// - Defaults:
//   - Internal indices remain zero-based.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Regression coverage for human-facing argument positions.
//!
//! Diagnostics use one-based positions while accessors remain zero-based.

use schoenwald_cli::ArgumentError;

#[test]
fn first_argument_is_rendered_as_position_one() {
    let error = ArgumentError::non_unicode(0);

    assert_eq!(
        error.to_string(),
        "command argument 1 is not valid Unicode"
    );
    assert_eq!(
        error.index(),
        0
    );
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

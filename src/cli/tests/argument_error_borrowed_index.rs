// File:
//   - argument_error_borrowed_index.rs
// Path:
//   - src/cli/tests/argument_error_borrowed_index.rs
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
//   - Regression coverage for the borrowed argument-index accessor.
// - Must-Not:
//   - Depend on ArgumentError remaining Copy.
// - Allows:
//   - Verify the accessor through its function-pointer signature.
// - Split-When:
//   - Another error-accessor signature needs independent coverage.
// - Merge-When:
//   - ArgumentError no longer exposes an index.
// - Summary:
//   - Borrowed index regression.
// - Description:
//   - Proves index inspection does not consume the error value.
// - Usage:
//   - Executed by the schoenwald-cli integration test target.
// - Defaults:
//   - The accessor returns the internal zero-based index.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Regression coverage for non-consuming argument-error inspection.
//!
//! Accessors must not depend on the error type remaining Copy.

use schoenwald_cli::ArgumentError;

#[test]
fn index_accessor_accepts_a_borrowed_error() {
    let accessor: fn(&ArgumentError) -> usize = ArgumentError::index;
    let error = ArgumentError::non_unicode(4);

    assert_eq!(
        accessor(&error),
        4
    );
    assert_eq!(
        error.to_string(),
        "command argument 5 is not valid Unicode"
    );
}

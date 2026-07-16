// File:
//   - output_error.rs
// Path:
//   - src/cli/tests/support/output_error.rs
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
//   - Shared integration-test assertions for required CLI output failures.
// - Must-Not:
//   - Define production behavior or access operating-system process state.
// - Allows:
//   - Inspect one completed invocation and return its public output error.
// - Split-When:
//   - Another helper family gains a separate test responsibility.
// - Merge-When:
//   - A production API owns the same assertion boundary.
// - Summary:
//   - Shared CLI output-error test helper.
// - Description:
//   - Centralizes repeated required-error extraction from invocation results.
// - Usage:
//   - Imported by CLI integration tests that inspect `OutputError` details.
// - Defaults:
//   - A successful invocation is an immediate test failure.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Shared assertions for CLI integration tests.

use schoenwald_cli::{ExitStatus, OutputError};

/// Return the required output error from one completed invocation.
#[must_use]
#[expect(
    clippy::unwrap_used,
    reason = "This test helper intentionally requires the invocation error."
)]
pub fn output_error(result: Result<ExitStatus, OutputError>) -> OutputError {
    result.unwrap_err()
}

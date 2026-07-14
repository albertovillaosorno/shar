// File:
//   - line_termination.rs
// Path:
//   - src/cli/tests/line_termination.rs
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
//   - Regression coverage for idempotent line termination.
// - Must-Not:
//   - Normalize embedded text or choose platform-specific endings.
// - Allows:
//   - Verify LF and CRLF caller input through public outcomes.
// - Split-When:
//   - Another line-construction behavior needs independent fixtures.
// - Merge-When:
//   - Command outcomes no longer own line helpers.
// - Summary:
//   - Line termination regression.
// - Description:
//   - Proves existing line endings do not gain a blank line.
// - Usage:
//   - Executed by the schoenwald-cli integration test target.
// - Defaults:
//   - Line helpers add LF only when no terminator exists.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Regression coverage for command-outcome line termination.
//!
//! Existing LF and CRLF endings must remain single terminators.

use schoenwald_cli::{CommandOutcome, OutputChunk};

#[test]
fn stdout_line_does_not_duplicate_an_existing_lf() {
    let outcome = CommandOutcome::success().stdout_line("ready\n");

    assert_eq!(
        outcome
            .output()
            .first()
            .map(OutputChunk::text),
        Some("ready\n")
    );
}

#[test]
fn stderr_line_does_not_duplicate_an_existing_crlf() {
    let outcome = CommandOutcome::failure().stderr_line("problem\r\n");

    assert_eq!(
        outcome
            .output()
            .first()
            .map(OutputChunk::text),
        Some("problem\r\n")
    );
}

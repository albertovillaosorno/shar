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
//   - Line termination test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Line termination test module.
// - Description:
//   - Implements the declared test module responsibility for command line.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Line termination test module.

use schoenwald_cli::{CommandOutcome, OutputChunk};

#[test]
fn stdout_line_does_not_duplicate_an_existing_lf() {
    let outcome = CommandOutcome::success().stdout_line("ready\n");

    assert_eq!(
        outcome.output().first().map(OutputChunk::text),
        Some("ready\n")
    );
}

#[test]
fn stderr_line_does_not_duplicate_an_existing_crlf() {
    let outcome = CommandOutcome::failure().stderr_line("problem\r\n");

    assert_eq!(
        outcome.output().first().map(OutputChunk::text),
        Some("problem\r\n")
    );
}

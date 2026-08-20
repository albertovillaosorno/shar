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
//   - Output error test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Output error test module.
// - Description:
//   - Implements the declared test module responsibility for command line.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Output error test module.

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

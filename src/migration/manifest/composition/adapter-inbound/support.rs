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
//   - Support inbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Support inbound adapter.
// - Description:
//   - Implements the declared inbound adapter responsibility for manifest.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Support inbound adapter.

use schoenwald_cli::CommandOutcome;

/// Returns a usage failure when an invocation supplies excess arguments.
#[must_use]
pub(super) fn reject_extra_arguments(
    arguments: &[String],
    maximum: usize,
    usage: &str,
) -> Option<CommandOutcome> {
    (arguments.len() > maximum)
        .then(|| CommandOutcome::failure().stderr_line(usage))
}

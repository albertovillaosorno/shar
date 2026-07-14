// File:
//   - support.rs
// Path:
//   - src/game-manifest/src/adapters/driving/support.rs
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
//   - Shared argument-count rejection for game-manifest CLI adapters.
// - Must-Not:
//   - Select domain defaults, execute use cases, or access process state.
// - Allows:
//   - Return one process-neutral usage failure for excess arguments.
// - Split-When:
//   - Split when another shared inbound rule becomes independently testable.
// - Merge-When:
//   - Another module owns the same game-manifest CLI policy mechanism.
// - Summary:
//   - Shared inbound argument guard.
// - Description:
//   - Prevents four command adapters from drifting in excess-argument behavior.
// - Usage:
//   - Called before each command interprets its optional arguments.
// - Defaults:
//   - Maximum counts exclude the executable name.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Shared process-neutral argument guard for manifest commands.
//!
//! Command-specific defaults and messages remain in the owning CLI adapter.
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

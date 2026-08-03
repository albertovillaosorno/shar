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
//   - Ephemeral structural audit composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Ephemeral structural audit composition module.
// - Description:
//   - Implements the declared composition module responsibility for manifest.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Ephemeral structural audit composition module.

use std::process::ExitCode;

use schoenwald_cli as _;
use schoenwald_filesystem as _;

fn main() -> ExitCode {
    game_manifest::adapters::driving::structural_audit_cli::run_env()
}

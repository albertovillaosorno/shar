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
//   - Binary composition root for the algorithm CLI.
// - Must-Not:
//   - Own algorithm behavior beyond dependency composition.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - One responsibility gains an independent lifecycle.
// - Merge-When:
//   - Another module owns the identical responsibility.
// - Summary:
//   - Algorithm executable composition root.
// - Description:
//   - Binary composition root for the algorithm CLI.
// - Usage:
//   - Used through the owning algorithm function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Algorithm command composition root.

use std::process::ExitCode;

use chacha20poly1305 as _;
use same_file as _;
use schoenwald_cli as _;
use schoenwald_filesystem as _;
use serde as _;
use serde_json as _;
use shar_sha256 as _;

fn main() -> ExitCode {
    shar_algorithm::adapters::driving::cli::run_env()
}

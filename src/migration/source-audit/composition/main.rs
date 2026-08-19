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
//   - Executable composition root for validate-source-deep.
// - Must-Not:
//   - Implement validation policy or bypass the inbound CLI adapter.
// - Allows:
//   - Bind the source-audit library to one process entrypoint.
// - Split-When:
//   - One responsibility gains an independent lifecycle.
// - Merge-When:
//   - Another module owns the identical responsibility.
// - Summary:
//   - Deep source validator composition root.
// - Description:
//   - Executable composition root for validate-source-deep.
// - Usage:
//   - Used through the owning source-audit function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Read-only deep source validation composition root.

use std::process::ExitCode;

use p3d as _;
use rcf as _;
use rmv as _;
use rsd as _;
use schoenwald_cli as _;
use schoenwald_filesystem as _;

fn main() -> ExitCode {
    shar_source_audit::adapters::driving::cli::run_env()
}

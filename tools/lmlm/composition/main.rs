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
//   - Process composition root for the LMLM compatibility binary.
// - Must-Not:
//   - Implement archive parsing or conversion policy directly.
// - Allows:
//   - Delegate process execution to the inbound CLI adapter.
// - Split-When:
//   - The process gains another independently deployed entry point.
// - Merge-When:
//   - Another composition root owns the identical process lifecycle.
// - Summary:
//   - LMLM process composition root.
// - Description:
//   - Starts the compatibility CLI through the shared command-line boundary.
// - Usage:
//   - Compiled as the `shar-lmlm` binary.
// - Defaults:
//   - Process status follows the shared CLI outcome.
//

//! Standalone LMLM compatibility process entry point.

#[path = "adapter-inbound/cli.rs"]
mod cli;

fn main() -> std::process::ExitCode {
    schoenwald_cli::run_process(&cli::LmlmProgram)
}

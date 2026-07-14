// File:
//   - main.rs
// Path:
//   - src/rcf/src/main.rs
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
//   - The minimal process entrypoint for the RCF CLI.
// - Must-Not:
//   - Decode arguments, construct storage policy, or execute use cases.
// - Allows:
//   - Delegation to the public driving adapter.
// - Split-When:
//   - Split when the process gains an independent runtime entrypoint.
// - Merge-When:
//   - The package no longer publishes a command-line executable.
// - Summary:
//   - Thin RCF executable entrypoint.
// - Description:
//   - Delegates all command behavior to the library-owned CLI adapter.
// - Usage:
//   - Invoked as the `rcf` binary.
// - Defaults:
//   - Exit status comes from the driving adapter.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Thin process entrypoint for the RCF driving adapter.

use std::process::ExitCode;

use schoenwald_cli as _;
use schoenwald_filesystem as _;

fn main() -> ExitCode {
    rcf::adapters::driving::cli::run_env()
}

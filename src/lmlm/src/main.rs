// File:
//   - main.rs
// Path:
//   - src/lmlm/src/main.rs
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
//   - The minimal process entrypoint for the LMLM CLI.
// - Must-Not:
//   - Decode arguments, select adapters, or execute use cases directly.
// - Allows:
//   - Delegation to the public driving adapter.
// - Split-When:
//   - Split when another process entrypoint is published.
// - Merge-When:
//   - The package no longer publishes this executable.
// - Summary:
//   - Thin LMLM executable entrypoint.
// - Description:
//   - Delegates command behavior to the library-owned CLI adapter.
// - Usage:
//   - Invoked as the `lmlm-extract` binary.
// - Defaults:
//   - Exit status comes from the driving adapter.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Thin process entrypoint for the LMLM driving adapter.
//!
//! All request decoding and dependency composition remain in the library.
use std::process::ExitCode;

use schoenwald_cli as _;
use schoenwald_filesystem as _;

fn main() -> ExitCode {
    lmlm::adapters::driving::cli::run_env()
}

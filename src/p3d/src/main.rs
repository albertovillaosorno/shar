// File:
//   - main.rs
// Path:
//   - src/p3d/src/main.rs
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
//   - The minimal single-package Pure3D process entrypoint.
// - Must-Not:
//   - Decode arguments, select adapters, or execute use cases directly.
// - Allows:
//   - Delegation to the public driving adapter.
// - Split-When:
//   - Split when another process entrypoint is published.
// - Merge-When:
//   - The package no longer publishes this executable.
// - Summary:
//   - Thin Pure3D single-package executable.
// - Description:
//   - Delegates command behavior to the library-owned driving adapter.
// - Usage:
//   - Invoked as the `p3d-extract` binary.
// - Defaults:
//   - Exit status comes from the driving adapter.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Thin process entrypoint for single-package `Pure3D` extraction.
//!
//! All argument handling and dependency composition live in the driving
//! adapter.
use std::process::ExitCode;

use schoenwald_cli as _;
use schoenwald_filesystem as _;
use serde_json as _;
use shar_json_text as _;

fn main() -> ExitCode {
    p3d::adapters::driving::single_cli::run_env()
}

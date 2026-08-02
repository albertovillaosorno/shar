// File:
//   - batch_extract.rs
// Path: src/formats/p3d/composition/bin/batch_extract.rs
//
// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
// Path-Rule:
//   - All paths in this header are repository-root relative.
//
// Boundary-Contract:
// - Owns:
//   - The minimal batch Pure3D process entrypoint.
// - Must-Not:
//   - Decode arguments, select adapters, or execute use cases directly.
// - Allows:
//   - Delegation to the public batch driving adapter.
// - Split-When:
//   - Split when another independently shipped batch binary is introduced.
// - Merge-When:
//   - The package no longer publishes this executable.
// - Summary:
//   - Thin `p3d-batch-extract` process entrypoint.
// - Description:
//   - Delegates command behavior to the library-owned batch CLI adapter.
// - Usage:
//   - Invoked as the `p3d-batch-extract` binary.
// - Defaults:
//   - Exit status comes from the driving adapter.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Thin process entrypoint for batch `Pure3D` extraction.

use std::process::ExitCode;

use schoenwald_cli as _;
use schoenwald_filesystem as _;
use serde_json as _;
use shar_json_text as _;

fn main() -> ExitCode {
    p3d::adapters::driving::batch_cli::run_env()
}

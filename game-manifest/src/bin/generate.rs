// File:
//   - generate.rs
// Path:
//   - game-manifest/src/bin/generate.rs
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
//   - The minimal process entrypoint for minimum game-manifest generation.
// - Must-Not:
//   - Parse arguments, access storage, or execute use cases directly.
// - Allows:
//   - Delegate once to the library-owned driving adapter.
// - Split-When:
//   - Split only when another independently shipped binary is introduced.
// - Merge-When:
//   - Another entrypoint owns the same executable contract.
// - Summary:
//   - Thin `generate-manifest` process entrypoint.
// - Description:
//   - Keeps command behavior inside the driving CLI adapter.
// - Usage:
//   - Invoked by Cargo as the `generate-manifest` binary.
// - Defaults:
//   - Returns the exit code produced by the driving adapter.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Thin process entrypoint for minimum game-manifest generation.

use std::process::ExitCode;

use schoenwald_cli as _;
use schoenwald_filesystem as _;

fn main() -> ExitCode {
    game_manifest::adapters::driving::generate_cli::run_env()
}

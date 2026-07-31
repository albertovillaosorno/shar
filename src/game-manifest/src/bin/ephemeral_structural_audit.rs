// File:
//   - ephemeral_structural_audit.rs
// Path:
//   - src/game-manifest/src/bin/ephemeral_structural_audit.rs
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
//   - The minimal process entrypoint for ephemeral structural audit.
// - Must-Not:
//   - Parse arguments, access storage, or execute use cases directly.
// - Allows:
//   - Delegate once to the library-owned driving adapter.
// - Split-When:
//   - Split only when another independently shipped binary is introduced.
// - Merge-When:
//   - Another entrypoint owns the same executable contract.
// - Summary:
//   - Thin `ephemeral_structural_audit` process entrypoint.
// - Description:
//   - Keeps command behavior inside the driving CLI adapter.
// - Usage:
//   - Invoked by Cargo as the `ephemeral_structural_audit` binary.
// - Defaults:
//   - Returns the exit code produced by the driving adapter.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Thin process entrypoint for ephemeral structural audit.

use std::process::ExitCode;

use schoenwald_cli as _;
use schoenwald_filesystem as _;

fn main() -> ExitCode {
    game_manifest::adapters::driving::structural_audit_cli::run_env()
}

// File:
//   - main.rs
// Path:
//   - src/pipeline/src/main.rs
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
//   - The minimal process entrypoint for the pipeline binary.
// - Must-Not:
//   - Parse arguments, render diagnostics, access storage, or run phases.
// - Allows:
//   - Delegate once to the pipeline driving adapter.
// - Split-When:
//   - Split only when another independently shipped binary is introduced.
// - Merge-When:
//   - Another binary entrypoint owns the same executable contract.
// - Summary:
//   - Thin pipeline process entrypoint.
// - Description:
//   - Keeps all command behavior inside the driving CLI adapter.
// - Usage:
//   - Invoked by Cargo as the `pipeline` binary.
// - Defaults:
//   - Returns the exit code produced by the driving adapter.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Thin process entrypoint for the extraction and planning pipeline.
//!
//! All argument policy and diagnostics live in the driving CLI adapter while
//! shared process mechanics map its outcome to the operating-system exit code.

use std::process::ExitCode;

use fbx as _;
use game_manifest as _;
use lmlm as _;
use p3d as _;
use rcf as _;
use rmv as _;
use rsd as _;
use rtf as _;
use schoenwald_cli as _;
use schoenwald_filesystem as _;
use serde_json as _;
use shar_json_text as _;
use shar_sha256 as _;

fn main() -> ExitCode {
    pipeline::adapters::driving::run_env()
}

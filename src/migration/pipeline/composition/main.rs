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
//   - Main composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Main composition module.
// - Description:
//   - Implements the declared composition module responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Main composition module.

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
use serde as _;
use serde_json as _;
use shar_sha256 as _;
use shar_unreal_conversion as _;

fn main() -> ExitCode {
    pipeline::adapters::driving::run_env()
}

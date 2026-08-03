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
//   - Filesystem lib.rs.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Filesystem lib.rs.
// - Description:
//   - Implements the declared lib.rs responsibility for filesystem.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Filesystem lib.rs.

#[path = "adapters.rs"]
pub mod adapters;
#[path = "application.rs"]
pub mod application;
#[path = "../domain/mod.rs"]
pub mod domain;
#[path = "io_context.rs"]
mod io_context;
#[path = "local.rs"]
mod local;
#[path = "path_validation.rs"]
mod path_validation;
#[path = "ports.rs"]
pub mod ports;
#[path = "std_filesystem.rs"]
mod std_filesystem;

pub use domain::{
    DiagnosticPath, PathKind, RootedPathError, resolve_under,
    validate_portable_path,
};

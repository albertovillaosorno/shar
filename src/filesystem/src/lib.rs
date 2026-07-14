// File:
//   - lib.rs
// Path:
//   - src/filesystem/src/lib.rs
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
//   - The shared filesystem crate public hexagonal facade.
// - Must-Not:
//   - Encode caller domains, select paths implicitly, or become a utility dump.
// - Allows:
//   - Expose pure path safety, use cases, ports, and adapter composition.
// - Split-When:
//   - Split when one stable mechanism becomes an independent crate.
// - Merge-When:
//   - Another facade owns the same crate-level contracts.
// - Summary:
//   - Shared filesystem public facade.
// - Description:
//   - Provides narrow reusable local IO without cross-domain policy coupling.
// - Usage:
//   - Used only from filesystem-facing adapters of workspace crates.
// - Defaults:
//   - Callers supply every path and policy choice explicitly.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Hexagonal shared filesystem mechanisms for Schoenwald crates.
//!
//! Stable local IO is centralized while domain policy remains with consumers.
pub mod adapters;
pub mod application;
pub mod domain;
pub mod ports;

pub use domain::{
    PathKind, RootedPathError, resolve_under, validate_portable_path,
};

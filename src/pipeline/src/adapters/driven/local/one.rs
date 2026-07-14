// File:
//   - one.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/one.rs
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
//   - The one contract for pipeline phase.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute one.
// - Split-When:
//   - Split when one contains two independently testable contracts.
// - Merge-When:
//   - Another pipeline module owns the same module boundary with no distinct
//   - invariant.
// - Summary:
//   - Pub mod extract module.
// - Description:
//   - Defines one data and behavior for pipeline phase.
// - Usage:
//   - Used by pipeline phase code that needs one.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/minor-unit-taxonomy-value-case.md
//
// Large file:
//   - false
//

//! Pub mod extract module.
//!
//! This boundary keeps pub mod extract module explicit and returns
//! deterministic results to pipeline callers.
/// Generated tree cleanup.
mod cleanup;
/// Pub mod extract module.
pub(in crate::adapters::driven::local) mod extract;
/// Generated JSON validation.
mod json_output;
/// Lmlm stage.
mod lmlm_stage;
/// Media dependencies.
mod media_dependencies;
/// Rms.
mod rms;
/// Spt.
mod spt;

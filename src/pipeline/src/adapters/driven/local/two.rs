// File:
//   - two.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/two.rs
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
//   - The two contract for pipeline phase.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute two.
// - Split-When:
//   - Split when two contains two independently testable contracts.
// - Merge-When:
//   - Another pipeline module owns the same module boundary with no distinct
//   - invariant.
// - Summary:
//   - Defines minor_units for this module boundary.
// - Description:
//   - Defines two data and behavior for pipeline phase.
// - Usage:
//   - Used by pipeline phase code that needs two.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/minor-unit-taxonomy-value-case.md
//
// Large file:
//   - false
//

//! Defines `minor_units` for this module boundary.
//!
//! This boundary keeps `minor_units` behavior explicit
//! and returns deterministic results to pipeline callers.
mod text;
// Retain the complete localization decoder family and its focused regression
// suite until a dedicated port consumes every decoder variant.
#[expect(
    dead_code,
    unused_imports,
    reason = "Retained decoders remain regression-tested for a planned \
              adapter."
)]
mod localization;
/// Stragglers.
mod stragglers;
pub(in crate::adapters::driven::local) mod units;

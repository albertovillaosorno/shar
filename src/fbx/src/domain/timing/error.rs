// File:
//   - error.rs
// Path:
//   - src/fbx/src/domain/timing/error.rs
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
//   - Pure fbx domain rules for domain timing error.
// - Must-Not:
//   - Read files, parse generated indexes, invoke CLI code, or call writer
//   - adapters.
// - Allows:
//   - Value objects, invariant checks, and pure evidence-to-domain translation.
// - Split-When:
//   - Split when error contains two independently testable contracts.
// - Merge-When:
//   - Another fbx module owns the same domain boundary with no distinct
//   - invariant.
// - Summary:
//   - Timing policy validation error.
// - Description:
//   - Defines error data and behavior for fbx domain timing.
// - Usage:
//   - Imported through crate domain facades or sibling domain modules.
// - Defaults:
//   - No filesystem paths, no external process calls, and no implicit IO
//   - defaults.
//
// ADRs:
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
// - docs/adr/pipeline/unreal/unreal-manifest-and-package-taxonomy.md
//
// Large file:
//   - false
//

//! Timing policy validation error.
//!
//! This boundary keeps timing policy validation error explicit and returns
//! deterministic results to fbx callers.
// These exact file-local lints preserve explicit domain and binary contracts.
#![expect(
    clippy::module_name_repetitions,
    reason = "Tests verify these intentional explicit file-local contracts \
              remain safe."
)]

/// Timing validation failure for animation export.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TimingPolicyError {
    /// Frame rate must be finite and positive.
    InvalidFrameRate,
}

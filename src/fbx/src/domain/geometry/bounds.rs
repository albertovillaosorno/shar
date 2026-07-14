// File:
//   - bounds.rs
// Path:
//   - src/fbx/src/domain/geometry/bounds.rs
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
//   - Pure fbx domain rules for domain geometry bounds.
// - Must-Not:
//   - Read files, parse generated indexes, invoke CLI code, or call writer
//   - adapters.
// - Allows:
//   - Value objects, invariant checks, and pure evidence-to-domain translation.
// - Split-When:
//   - Split when bounds contains two independently testable contracts.
// - Merge-When:
//   - Another fbx module owns the same domain boundary with no distinct
//   - invariant.
// - Summary:
//   - Axis-aligned bounds for one geometry payload.
// - Description:
//   - Defines bounds data and behavior for fbx domain geometry.
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

//! Axis-aligned bounds for one geometry payload.
//!
//! This boundary keeps axis-aligned bounds for one geometry payload explicit
//! and returns deterministic results to fbx callers.
// These exact file-local lints preserve explicit domain and binary contracts.
#![expect(
    clippy::module_name_repetitions,
    reason = "Tests verify these intentional explicit file-local contracts \
              remain safe."
)]

/// Axis-aligned bounds for validated geometry.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GeometryBounds {
    /// Minimum XYZ coordinate.
    pub minimum: [f32; 3],
    /// Maximum XYZ coordinate.
    pub maximum: [f32; 3],
}

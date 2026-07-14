// File:
//   - system.rs
// Path:
//   - src/fbx/src/domain/coordinate/system.rs
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
//   - Pure fbx domain rules for domain coordinate system.
// - Must-Not:
//   - Read files, parse generated indexes, invoke CLI code, or call writer
//   - adapters.
// - Allows:
//   - Value objects, invariant checks, and pure evidence-to-domain translation.
// - Split-When:
//   - Split when system contains two independently testable contracts.
// - Merge-When:
//   - Another fbx module owns the same domain boundary with no distinct
//   - invariant.
// - Summary:
//   - Preserve decoded axes until validation proves a conversion is needed.
// - Description:
//   - Defines system data and behavior for fbx domain coordinate.
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

//! Preserve decoded axes until validation proves a conversion is needed.
//!
//! This boundary keeps preserve decoded axes until validation proves a
//! conversion is needed explicit and returns deterministic results to fbx
//! callers.
// These exact file-local lints preserve explicit domain and binary contracts.
#![expect(
    clippy::module_name_repetitions,
    reason = "Tests verify these intentional explicit file-local contracts \
              remain safe."
)]

/// Coordinate system used by imported and exported scenes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CoordinateSystem {
    /// Preserve decoded axes until validation proves a conversion is needed.
    PreserveSource,
    /// Normalize to an FBX-friendly right-handed coordinate system.
    FbxRightHanded,
}

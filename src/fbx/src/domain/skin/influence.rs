// File:
//   - influence.rs
// Path:
//   - src/fbx/src/domain/skin/influence.rs
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
//   - Pure fbx domain rules for domain skin influence.
// - Must-Not:
//   - Read files, parse generated indexes, invoke CLI code, or call writer
//   - adapters.
// - Allows:
//   - Value objects, invariant checks, and pure evidence-to-domain translation.
// - Split-When:
//   - Split when influence contains two independently testable contracts.
// - Merge-When:
//   - Another fbx module owns the same domain boundary with no distinct
//   - invariant.
// - Summary:
//   - One bone influence on one vertex.
// - Description:
//   - Defines influence data and behavior for fbx domain skin.
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

//! One bone influence on one vertex.
//!
//! This boundary keeps one bone influence on one vertex explicit and returns
//! deterministic results to fbx callers.
// These exact file-local lints preserve explicit domain and binary contracts.
#![expect(
    clippy::module_name_repetitions,
    reason = "Tests verify these intentional explicit file-local contracts \
              remain safe."
)]

/// Skin influence assigning one bone weight to a vertex.
#[derive(Clone, Debug, PartialEq)]
pub struct SkinInfluence {
    /// Vertex index affected by the influence.
    pub vertex_index: u32,
    /// Stable bone id.
    pub bone_id: String,
    /// Influence weight.
    pub weight: f32,
}

// File:
//   - bone.rs
// Path:
//   - src/fbx/src/domain/skeleton/bone.rs
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
//   - Pure fbx domain rules for domain skeleton bone.
// - Must-Not:
//   - Read files, parse generated indexes, invoke CLI code, or call writer
//   - adapters.
// - Allows:
//   - Value objects, invariant checks, and pure evidence-to-domain translation.
// - Split-When:
//   - Split when bone contains two independently testable contracts.
// - Merge-When:
//   - Another fbx module owns the same domain boundary with no distinct
//   - invariant.
// - Summary:
//   - Bone in a normalized skeleton.
// - Description:
//   - Defines bone data and behavior for fbx domain skeleton.
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

//! Bone in a normalized skeleton.
//!
//! This boundary keeps bone in a normalized skeleton explicit and returns
//! deterministic results to fbx callers.
/// Skeleton bone with stable parent and transform data.
#[derive(Clone, Debug, PartialEq)]
pub struct Bone {
    /// Stable bone id.
    pub id: String,
    /// Optional parent bone id.
    pub parent_id: Option<String>,
    /// Rest transform matrix in row-major order.
    pub rest_matrix: [f32; 16],
}

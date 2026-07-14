// File:
//   - local.rs
// Path:
//   - src/fbx/src/domain/transform/local.rs
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
//   - Pure fbx domain rules for domain transform local.
// - Must-Not:
//   - Read files, parse generated indexes, invoke CLI code, or call writer
//   - adapters.
// - Allows:
//   - Value objects, invariant checks, and pure evidence-to-domain translation.
// - Split-When:
//   - Split when local contains two independently testable contracts.
// - Merge-When:
//   - Another fbx module owns the same domain boundary with no distinct
//   - invariant.
// - Summary:
//   - Local transform for a scene node.
// - Description:
//   - Defines local data and behavior for fbx domain transform.
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

//! Local transform for a scene node.
//!
//! This boundary keeps local transform for a scene node explicit and returns
//! deterministic results to fbx callers.
/// Local transform relative to a scene parent.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform {
    /// Translation vector.
    pub translation: [f32; 3],
    /// Rotation quaternion.
    pub rotation: [f32; 4],
    /// Scale vector.
    pub scale: [f32; 3],
}

impl Transform {
    /// Create an identity transform for nodes that have no explicit transform
    /// evidence yet.
    #[must_use]
    pub const fn identity() -> Self {
        Self {
            translation: [
                0.0, 0.0, 0.0,
            ],
            rotation: [
                0.0, 0.0, 0.0, 1.0,
            ],
            scale: [
                1.0, 1.0, 1.0,
            ],
        }
    }
}

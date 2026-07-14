// File:
//   - definition.rs
// Path:
//   - src/fbx/src/domain/camera/definition.rs
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
//   - Pure fbx domain rules for domain camera definition.
// - Must-Not:
//   - Read files, parse generated indexes, invoke CLI code, or call writer
//   - adapters.
// - Allows:
//   - Value objects, invariant checks, and pure evidence-to-domain translation.
// - Split-When:
//   - Split when definition contains two independently testable contracts.
// - Merge-When:
//   - Another fbx module owns the same domain boundary with no distinct
//   - invariant.
// - Summary:
//   - Camera attached to a scene node.
// - Description:
//   - Defines definition data and behavior for fbx domain camera.
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

//! Camera attached to a scene node.
//!
//! This boundary keeps camera attached to a scene node explicit and returns
//! deterministic results to fbx callers.
/// Validated camera definition used by scene export.
#[derive(Clone, Debug, PartialEq)]
pub struct Camera {
    /// Stable camera id.
    pub id: String,
    /// Vertical field of view in degrees.
    pub vertical_fov_degrees: f32,
}

// File:
//   - node.rs
// Path:
//   - src/fbx/src/domain/scene/node.rs
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
//   - Pure fbx domain rules for domain scene node.
// - Must-Not:
//   - Read files, parse generated indexes, invoke CLI code, or call writer
//   - adapters.
// - Allows:
//   - Value objects, invariant checks, and pure evidence-to-domain translation.
// - Split-When:
//   - Split when node contains two independently testable contracts.
// - Merge-When:
//   - Another fbx module owns the same domain boundary with no distinct
//   - invariant.
// - Summary:
//   - Node in the normalized scene hierarchy.
// - Description:
//   - Defines node data and behavior for fbx domain scene.
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

//! Node in the normalized scene hierarchy.
//!
//! This boundary keeps node in the normalized scene hierarchy explicit and
//! returns deterministic results to fbx callers.
// These exact file-local lints preserve explicit domain and binary contracts.
#![expect(
    clippy::module_name_repetitions,
    reason = "Tests verify these intentional explicit file-local contracts \
              remain safe."
)]

use crate::domain::geometry::Geometry;
use crate::domain::transform::Transform;

/// Node in the normalized scene hierarchy.
#[derive(Clone, Debug, PartialEq)]
pub struct SceneNode {
    /// Stable node id.
    pub id: String,
    /// Optional parent id.
    pub parent_id: Option<String>,
    /// Local transform relative to the parent.
    pub local_transform: Transform,
    /// Optional geometry attached to the node.
    pub geometry: Option<Geometry>,
}

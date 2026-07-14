// File:
//   - aggregate.rs
// Path:
//   - src/fbx/src/domain/scene/aggregate.rs
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
//   - Pure fbx domain rules for domain scene aggregate.
// - Must-Not:
//   - Read files, parse generated indexes, invoke CLI code, or call writer
//   - adapters.
// - Allows:
//   - Value objects, invariant checks, and pure evidence-to-domain translation.
// - Split-When:
//   - Split when aggregate contains two independently testable contracts.
// - Merge-When:
//   - Another fbx module owns the same domain boundary with no distinct
//   - invariant.
// - Summary:
//   - Stable scene assembled before any writer adapter serializes it.
// - Description:
//   - Defines aggregate data and behavior for fbx domain scene.
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

//! Stable scene assembled before any writer adapter serializes it.
use super::node::SceneNode;
use crate::domain::capability::CapabilityReport;
use crate::domain::material::Material;

/// Stable scene assembled before any writer adapter serializes it.
#[derive(Clone, Debug, PartialEq)]
pub struct Scene {
    /// Stable scene id selected by the application layer.
    pub id: String,
    /// Nodes that form the scene hierarchy.
    pub nodes: Vec<SceneNode>,
    /// Materials referenced by scene geometry.
    pub materials: Vec<Material>,
    /// Explicit capability decisions for converted and preserved evidence.
    pub capabilities: CapabilityReport,
}

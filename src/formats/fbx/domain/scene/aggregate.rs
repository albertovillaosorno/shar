// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Aggregate domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Aggregate domain module.
// - Description:
//   - Implements the declared domain module responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Aggregate domain module.

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

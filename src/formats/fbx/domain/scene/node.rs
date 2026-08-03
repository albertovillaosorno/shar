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
//   - Node domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Node domain module.
// - Description:
//   - Implements the declared domain module responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Node domain module.

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

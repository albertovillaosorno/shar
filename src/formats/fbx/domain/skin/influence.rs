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
//   - Influence domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Influence domain module.
// - Description:
//   - Implements the declared domain module responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Influence domain module.

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

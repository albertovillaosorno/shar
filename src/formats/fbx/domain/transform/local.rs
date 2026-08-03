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
//   - Local domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Local domain module.
// - Description:
//   - Implements the declared domain module responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Local domain module.

/// Local translation, rotation, and scale transform.
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
            translation: [0., 0., 0.],
            rotation: [0., 0., 0., 1.],
            scale: [1., 1., 1.],
        }
    }
}

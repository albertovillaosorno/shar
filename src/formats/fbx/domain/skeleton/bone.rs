// Copyright:
//   - Copyright © 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Bone domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Bone domain module.
// - Description:
//   - Implements the declared domain module responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Bone domain module.

/// Exact source rig controls preserved alongside one FBX bone.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BoneSourceRig {
    /// Source degree-of-freedom mask.
    pub dof: u32,
    /// Source free-axis mask.
    pub free_axes: u32,
    /// Source primary-axis selector.
    pub primary_axis: u32,
    /// Source secondary-axis selector.
    pub secondary_axis: u32,
    /// Source twist-axis selector.
    pub twist_axis: u32,
    /// Optional decoded mirror-map record.
    pub mirror_map: Option<BoneMirrorMap>,
    /// Optional decoded joint-fix flags.
    pub fix_flags: Option<u32>,
}

/// Exact decoded mirror-map values attached to one source joint.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BoneMirrorMap {
    /// Authored mirror-map index.
    pub index: u32,
    /// Authored mirror-map scale.
    pub scale: [f32; 3],
}

/// One canonical skeleton bone and its bind-pose relationship.
#[derive(Clone, Debug, PartialEq)]
pub struct Bone {
    /// Stable bone id.
    pub id: String,
    /// Optional parent bone id.
    pub parent_id: Option<String>,
    /// Rest transform matrix in row-major order.
    pub rest_matrix: [f32; 16],
    /// Exact source rig controls that are not standard FBX TRS fields.
    pub source_rig: Option<BoneSourceRig>,
}

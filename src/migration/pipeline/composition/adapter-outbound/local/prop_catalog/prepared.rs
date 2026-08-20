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
//   - Prepared outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Prepared outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Prepared outbound adapter.

use fbx::domain::animation::AnimationClip;
use fbx::domain::character::CharacterAsset;
use fbx::domain::mesh::MeshAsset;
use fbx::domain::texture::MaterialBinding;

use super::model::PropRoute;

/// One canonical external texture payload.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct PreparedTexture {
    /// Content-derived portable PNG file name.
    pub(super) file_name: String,
    /// Exact external PNG payload bytes.
    pub(super) bytes: Vec<u8>,
    /// Exact lowercase payload SHA-256.
    pub(super) sha256: String,
}

/// Static or rigid-animated canonical model payload.
#[derive(Clone, Debug, PartialEq)]
pub(super) enum PreparedGeometry {
    /// One or more static meshes with no synthetic rig or animation.
    Static(Vec<MeshAsset>),
    /// One rigid-bound model and its authored model-transform clips.
    RigidAnimated {
        /// Canonical rigid asset, bones, meshes, and influences.
        asset: CharacterAsset,
        /// Canonical authored model-transform animations.
        animations: Vec<AnimationClip>,
    },
}

/// One candidate normalized for semantic deduplication and FBX output.
#[derive(Clone, Debug, PartialEq)]
pub(super) struct PreparedProp {
    /// Static or rigid-animated publication route.
    pub(super) route: PropRoute,
    /// Semantic signature used for deterministic deduplication.
    pub(super) signature: String,
    /// Canonical geometry and optional rigid-animation payload.
    pub(super) geometry: PreparedGeometry,
    /// Canonical diffuse material bindings.
    pub(super) materials: Vec<MaterialBinding>,
    /// Referenced external texture payloads.
    pub(super) textures: Vec<PreparedTexture>,
}

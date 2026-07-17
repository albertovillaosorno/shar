// File:
//   - prepared.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/prop_catalog/prepared.rs
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
//   - Canonical in-memory prop asset and texture payload records.
// - Must-Not:
//   - Read source components, write FBX, or publish catalog files.
// - Allows:
//   - Static and rigid-animated model variants with normalized identities.
// - Split-When:
//   - Static and animated prepared assets gain independent lifecycles.
// - Merge-When:
//   - Export can consume source-domain assets without canonical adaptation.
// - Summary:
//   - Carries one deduplicable model artifact before binary serialization.
// - Description:
//   - Keeps texture payload bytes separate from portable material bindings.
// - Usage:
//   - Produced by preparation and consumed by export/publication.
// - Defaults:
//   - One prepared asset has one semantic SHA-256 signature.
//
// ADRs:
// - docs/adr/fbx/export/fbx-output-contract-boundary.md
//
// Large file:
//   - false
//

//! Canonical in-memory prop assets before binary serialization.

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

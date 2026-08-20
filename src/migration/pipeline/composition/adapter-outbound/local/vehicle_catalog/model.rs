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
//   - Model outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Model outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Model outbound adapter.

use fbx::adapters::driven::binary_character_writer::CharacterBinaryFbxSummary;
use fbx::domain::texture::MaterialSemantics;

/// One semantic geometry object published inside a vehicle FBX.
#[derive(Clone, Debug)]
pub(super) struct PartRecord {
    /// Published semantic object identity.
    pub(super) name: String,
    /// Canonical source mesh identity.
    pub(super) source_mesh: String,
    /// Stable semantic part role.
    pub(super) role: &'static str,
    /// Canonical source shader identity.
    pub(super) shader: String,
    /// Compact overlapping material and geometry semantics.
    pub(super) semantics: MaterialSemantics,
    /// Retained skeleton joints influencing this object.
    pub(super) bones: Vec<String>,
}

/// One published texture payload and its semantic state role.
#[derive(Clone, Debug)]
pub(super) struct TextureRecord {
    /// Repository-relative texture artifact path.
    pub(super) path: String,
    /// Stable semantic part role.
    pub(super) role: &'static str,
    /// Exact texture artifact byte length.
    pub(super) bytes: u64,
    /// Exact texture artifact SHA-256 digest.
    pub(super) sha256: String,
}

/// Source-backed vertical normalization recorded for one vehicle FBX.
#[derive(Clone, Debug)]
pub(super) struct GroundingRecord {
    /// Stable source-evidence strategy used to derive the translation.
    pub(super) source: &'static str,
    /// Exact authored-axis translation applied before FBX serialization.
    pub(super) offset_y: f64,
    /// Authored skeleton root receiving the same translation.
    pub(super) root_bone: String,
}

/// One completed vehicle artifact and catalog record.
#[derive(Clone, Debug)]
pub(super) struct VehicleRecord {
    /// Stable vehicle identity.
    pub(super) vehicle: String,
    /// Canonical source package identity.
    pub(super) package_id: String,
    /// Canonical source package subcategory.
    pub(super) subcategory: String,
    /// Repository-relative vehicle FBX path.
    pub(super) fbx_path: String,
    /// Exact vehicle FBX byte length.
    pub(super) fbx_bytes: u64,
    /// Exact vehicle FBX SHA-256 digest.
    pub(super) fbx_sha256: String,
    /// Binary FBX object-family summary.
    pub(super) summary: CharacterBinaryFbxSummary,
    /// Source-backed vertical normalization applied to this FBX.
    pub(super) grounding: GroundingRecord,
    /// Deterministically ordered semantic part records.
    pub(super) parts: Vec<PartRecord>,
    /// Source geometry intentionally preserved outside the main FBX.
    pub(super) deferred_geometry: Vec<String>,
    /// Number of nonvisual wheel proxies retained for runtime semantics.
    pub(super) hidden_wheel_proxies: usize,
    /// Published skeletal animation artifact paths.
    pub(super) animations: Vec<String>,
    /// Published effect-animation sidecar paths.
    pub(super) effect_animation_sidecars: Vec<String>,
    /// Published texture artifact records.
    pub(super) textures: Vec<TextureRecord>,
    /// Published shader evidence paths.
    pub(super) shaders: Vec<String>,
}

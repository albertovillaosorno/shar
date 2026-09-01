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

/// One exact source controller-to-target relationship for an effect sidecar.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct EffectControllerRecord {
    /// Authored frame-controller identity.
    pub(super) controller_identity: String,
    /// Exact normalized frame-controller component kind.
    pub(super) controller_kind: String,
    /// Exact source frame-controller chunk ordinal.
    pub(super) controller_source_ordinal: usize,
    /// Decoded source frame-controller version.
    pub(super) controller_version: usize,
    /// Decoded source frame-controller type.
    pub(super) controller_type: String,
    /// Exact finite source frame-offset bits.
    pub(super) frame_offset_bits: u32,
    /// Exact normalized target component kind.
    pub(super) target_kind: String,
    /// Authored hierarchy identity targeted by the controller.
    pub(super) target_identity: String,
    /// Exact source target component chunk ordinal.
    pub(super) target_source_ordinal: usize,
}

/// One physical same-package texture occurrence retained without selection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct EffectTextureOccurrenceRecord {
    /// Exact normalized physical texture member identity.
    pub(super) member_id: String,
    /// Exact source texture component ordinal.
    pub(super) source_ordinal: usize,
    /// Exact lowercase SHA-256 of the normalized PNG payload.
    pub(super) sha256: String,
}

/// One logical TEX entity identity and every matching package-local occurrence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct EffectTextureReferenceRecord {
    /// Canonical logical texture identity from the entity channel.
    pub(super) identity: String,
    /// Physical package-local occurrences in source ordinal order.
    pub(super) occurrences: Vec<EffectTextureOccurrenceRecord>,
}

/// One non-skeletal vehicle animation preserved as a source sidecar.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct EffectAnimationRecord {
    /// Published repository-relative sidecar path.
    pub(super) path: String,
    /// Authored animation identity.
    pub(super) identity: String,
    /// Authored animation family.
    pub(super) animation_type: String,
    /// Exact source animation chunk ordinal.
    pub(super) source_ordinal: usize,
    /// Exact supported controller relationship when the source declares one.
    pub(super) controller: Option<EffectControllerRecord>,
    /// Logical TEX references with every package-local physical occurrence.
    pub(super) texture_references: Vec<EffectTextureReferenceRecord>,
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
    /// Published effect-animation sidecars with exact source relationships.
    pub(super) effect_animation_sidecars: Vec<EffectAnimationRecord>,
    /// Published texture artifact records.
    pub(super) textures: Vec<TextureRecord>,
    /// Published shader evidence paths.
    pub(super) shaders: Vec<String>,
}

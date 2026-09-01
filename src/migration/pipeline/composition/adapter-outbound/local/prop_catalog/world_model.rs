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
//   - World model outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - World model outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! World model outbound adapter.

use fbx::adapters::driven::binary_character_writer::CharacterBinaryFbxSummary;

use super::model::{PropAlias, PropRoute, TextureRecord};

/// One same-name source variant omitted from the canonical FBX.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct OmittedWorldVariant {
    /// Full semantic signature including presentation and animation.
    pub(super) semantic_sha256: String,
    /// Geometry signature including presentation channels.
    pub(super) visual_sha256: String,
    /// Position and topology signature excluding presentation channels.
    pub(super) structural_sha256: String,
    /// Static or rigid-animated route owned by the variant.
    pub(super) route: PropRoute,
    /// Number of source occurrences represented by the variant.
    pub(super) source_count: usize,
}

/// One canonical published world prop and its retained provenance.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct ExportedWorldProp {
    /// Readable hash-free asset identity.
    pub(super) asset_id: String,
    /// Static or rigid-animated publication route.
    pub(super) route: PropRoute,
    /// Full semantic signature of the published variant.
    pub(super) semantic_sha256: String,
    /// Geometry signature including presentation channels.
    pub(super) visual_sha256: String,
    /// Position and topology signature excluding presentation channels.
    pub(super) structural_sha256: String,
    /// Optional rigid-binding signature.
    pub(super) rig_sha256: Option<String>,
    /// Catalog-relative FBX path.
    pub(super) fbx_path: String,
    /// Exact FBX byte count.
    pub(super) fbx_bytes: u64,
    /// Exact FBX SHA-256.
    pub(super) fbx_sha256: String,
    /// Binary object-family summary.
    pub(super) summary: CharacterBinaryFbxSummary,
    /// Published external texture records.
    pub(super) textures: Vec<TextureRecord>,
    /// Every source occurrence represented by the readable name.
    pub(super) aliases: Vec<PropAlias>,
    /// Number of structurally compatible variants merged into the FBX.
    pub(super) merged_compatible_variants: usize,
    /// Structurally incompatible same-name variants retained as evidence.
    pub(super) omitted_visual_variants: Vec<OmittedWorldVariant>,
}

/// Aggregate counts for one complete world-prop publication.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(super) struct WorldCatalogCounts {
    /// Terrain-world packages re-extracted from the game tree.
    pub(super) source_packages: usize,
    /// Model-bearing occurrences before readable-name consolidation.
    pub(super) occurrences: usize,
    /// Published readable asset names.
    pub(super) assets: usize,
    /// Published static assets.
    pub(super) static_assets: usize,
    /// Published rigid-animated assets.
    pub(super) animated_assets: usize,
    /// Structurally compatible variants merged into canonical assets.
    pub(super) merged_variants: usize,
    /// Structurally incompatible same-name variants retained in the catalog.
    pub(super) omitted_variants: usize,
    /// Source aliases with exact primary world-model provenance.
    pub(super) primary_source_bindings: usize,
    /// Primary selected mesh occurrences retained in authored route order.
    pub(super) primary_selected_meshes: usize,
    /// Primary source aliases with a matching composite occurrence.
    pub(super) primary_matched_composites: usize,
    /// Primary source aliases with an authored skeleton relationship.
    pub(super) primary_referenced_skeletons: usize,
    /// Primary source aliases whose PTRN animation is consumed by the FBX
    /// route.
    pub(super) primary_exported_ptrn_animations: usize,
    /// Non-mesh composite render bindings retained as deferred source evidence.
    pub(super) deferred_render_bindings: usize,
    /// Deferred render bindings with exact source billboard presentation.
    pub(super) deferred_billboard_bindings: usize,
    /// Source billboard child quads retained in authored order.
    pub(super) deferred_billboard_quads: usize,
    /// Same-name source shader occurrences retained for deferred billboards.
    pub(super) deferred_billboard_shader_occurrences: usize,
    /// Deferred billboards with more than one same-name shader occurrence.
    pub(super) deferred_billboard_shader_ambiguities: usize,
    /// Logical texture references retained for deferred billboards.
    pub(super) deferred_billboard_texture_references: usize,
    /// Preferred physical texture occurrences retained for deferred billboards.
    pub(super) deferred_billboard_texture_occurrences: usize,
    /// Deferred texture references whose preferred occurrences differ by
    /// payload.
    pub(super) deferred_billboard_texture_ambiguities: usize,
    /// Deferred render bindings with exact source controller relationships.
    pub(super) deferred_controller_bindings: usize,
    /// Deferred controllers with strict validated BQG source payloads.
    pub(super) deferred_controller_animation_payloads: usize,
}

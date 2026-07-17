// File:
//   - world_model.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/prop_catalog/world_model.rs
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
//   - Published world-prop records and consolidation evidence.
// - Must-Not:
//   - Read source packages or write FBX bytes.
// - Allows:
//   - Hash-free names, merged clips, and omitted-variant provenance.
// - Summary:
//   - Represents one canonical world prop per readable source name.
//
// Large file:
//   - false

//! Published world-prop records.

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
}

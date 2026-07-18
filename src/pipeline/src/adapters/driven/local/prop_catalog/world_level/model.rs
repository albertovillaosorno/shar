// File:
//   - model.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/prop_catalog/world_level/model.rs
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
//   - Typed master-world export records and provenance counters.
// - Must-Not:
//   - Read source packages, transform geometry, or write artifacts.
// - Allows:
//   - Level ownership, package provenance, review layers, counts, and hashes.
// - Summary:
//   - Carries one deterministic separated master-world FBX result.
//
// ADRs:
// - docs/adr/pipeline/unreal/world-assembly-from-normalized-chunks.md
// - docs/adr/fbx/export/fbx-output-contract-boundary.md
//
// Large file:
//   - false
//

//! Typed records for one separated master-world FBX publication.

use fbx::adapters::driven::binary_character_writer::CharacterBinaryFbxSummary;

use super::super::model::TextureRecord;

/// One normalized source package represented in the master-world scene.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct LevelPackageRecord {
    pub(super) level: String,
    pub(super) package_id: String,
    pub(super) subcategory: String,
    pub(super) coordinate_reference: bool,
    pub(super) source_meshes: usize,
    pub(super) discarded_degenerate_triangles: usize,
    pub(super) authored_placements: usize,
    pub(super) reference_placements: usize,
    pub(super) canonical_placement_fallbacks: usize,
    pub(super) reference_coordinate_meshes: usize,
    pub(super) canonical_coordinate_meshes: usize,
    pub(super) review_definitions: usize,
    pub(super) collision_meshes: usize,
    pub(super) reference_collision_meshes: usize,
    pub(super) discarded_collision_triangles: usize,
    pub(super) interior_packages: usize,
}

/// One published master-world FBX and complete analysis provenance.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct ExportedWorldMaster {
    pub(super) fbx_path: String,
    pub(super) fbx_bytes: u64,
    pub(super) fbx_sha256: String,
    pub(super) summary: CharacterBinaryFbxSummary,
    pub(super) textures: Vec<TextureRecord>,
    pub(super) packages: Vec<LevelPackageRecord>,
    pub(super) source_levels: usize,
    pub(super) source_meshes: usize,
    pub(super) discarded_degenerate_triangles: usize,
    pub(super) authored_placements: usize,
    pub(super) reference_placements: usize,
    pub(super) canonical_placement_fallbacks: usize,
    pub(super) reference_coordinate_meshes: usize,
    pub(super) canonical_coordinate_meshes: usize,
    pub(super) review_definitions: usize,
    pub(super) review_similarity_groups: usize,
    pub(super) collision_meshes: usize,
    pub(super) reference_collision_meshes: usize,
    pub(super) discarded_collision_triangles: usize,
    pub(super) interior_packages: usize,
}

/// Aggregate counters for one complete master-world publication.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(super) struct WorldMasterCounts {
    pub(super) source_levels: usize,
    pub(super) source_packages: usize,
    pub(super) coordinate_reference_packages: usize,
    pub(super) coordinate_fallback_packages: usize,
    pub(super) interior_packages: usize,
    pub(super) source_meshes: usize,
    pub(super) discarded_degenerate_triangles: usize,
    pub(super) authored_placements: usize,
    pub(super) reference_placements: usize,
    pub(super) canonical_placement_fallbacks: usize,
    pub(super) reference_coordinate_meshes: usize,
    pub(super) canonical_coordinate_meshes: usize,
    pub(super) review_definitions: usize,
    pub(super) review_similarity_groups: usize,
    pub(super) collision_meshes: usize,
    pub(super) reference_collision_meshes: usize,
    pub(super) discarded_collision_triangles: usize,
}

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
//   - Typed globally aligned world-package export records and counters.
// - Must-Not:
//   - Read source packages, transform geometry, or write artifacts.
// - Allows:
//   - Package provenance, artifact hashes, semantic counts, and shared origin.
// - Summary:
//   - Carries one deterministic collection of independently importable FBX
//     files.
//
// ADRs:
// - docs/adr/pipeline/unreal/world-assembly-from-normalized-chunks.md
// - docs/adr/fbx/export/fbx-output-contract-boundary.md
//
// Large file:
//   - false
//

//! Typed records for globally aligned world-package FBX publication.

use fbx::adapters::driven::binary_character_writer::CharacterBinaryFbxSummary;

use super::super::model::TextureRecord;

/// One written static FBX artifact at the shared world origin.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct WorldFbxRecord {
    /// Repository-relative artifact path.
    pub(super) path: String,
    /// Exact artifact byte length.
    pub(super) bytes: u64,
    /// Exact artifact SHA-256 digest.
    pub(super) sha256: String,
    /// Binary FBX object-family summary.
    pub(super) summary: CharacterBinaryFbxSummary,
    /// Overlapping semantic surface counts.
    pub(super) surface_semantics: WorldSurfaceSemanticCounts,
}

/// One normalized source package and its independently importable artifacts.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct WorldPackageRecord {
    /// Stable world scope identity.
    pub(super) scope: String,
    /// Canonical normalized package identity.
    pub(super) package_id: String,
    /// Canonical package subcategory.
    pub(super) subcategory: String,
    /// Whether private coordinate evidence aligned this package.
    pub(super) coordinate_reference: bool,
    /// Whether this package belongs to an interior world scope.
    pub(super) interior: bool,
    /// Stable narrative map identity, absent for auxiliary bonus areas.
    pub(super) map_group: Option<String>,
    /// Baked source-space map translation in whole source units.
    pub(super) map_offset: [i16; 3],
    /// Whether this artifact belongs in the normal root-FBX import set.
    pub(super) normal_import: bool,
    /// Number of canonical source meshes considered.
    pub(super) source_meshes: usize,
    /// Number of rejected degenerate render triangles.
    pub(super) discarded_degenerate_triangles: usize,
    /// Number of authored mesh placements.
    pub(super) authored_placements: usize,
    /// Number of placements using verified coordinate evidence.
    pub(super) reference_placements: usize,
    /// Number of placements retaining canonical coordinates.
    pub(super) canonical_placement_fallbacks: usize,
    /// Number of render meshes using verified coordinates.
    pub(super) reference_coordinate_meshes: usize,
    /// Number of render meshes retaining canonical coordinates.
    pub(super) canonical_coordinate_meshes: usize,
    /// Number of definition-only meshes isolated for review.
    pub(super) review_definitions: usize,
    /// Number of independently selectable geometry groups.
    pub(super) independent_item_geometries: usize,
    /// Number of source-backed breakable geometry groups.
    pub(super) breakable_geometries: usize,
    /// Number of source-backed interactable geometry groups.
    pub(super) interactable_geometries: usize,
    /// Number of deterministic review similarity groups.
    pub(super) review_similarity_groups: usize,
    /// Number of source collision meshes excluded from FBX publication.
    pub(super) excluded_collision_meshes: usize,
    /// Number of excluded collision meshes with verified coordinates.
    pub(super) reference_excluded_collision_meshes: usize,
    /// Number of rejected degenerate collision triangles.
    pub(super) discarded_collision_triangles: usize,
    /// Optional independently importable world-geometry artifact.
    pub(super) world_fbx: Option<WorldFbxRecord>,
    /// Optional definition-only review artifact.
    pub(super) review_fbx: Option<WorldFbxRecord>,
}

/// One complete package collection sharing a zero origin and texture authority.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct ExportedWorldCollection {
    /// Deterministically ordered package publication records.
    pub(super) packages: Vec<WorldPackageRecord>,
    /// Deduplicated shared texture authority.
    pub(super) textures: Vec<TextureRecord>,
    /// Overlapping semantic surface counts.
    pub(super) surface_semantics: WorldSurfaceSemanticCounts,
}

/// Overlapping semantic material and geometry counts across written FBX files.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(super) struct WorldSurfaceSemanticCounts {
    /// Number of transparent material variants.
    pub(super) transparent_materials: usize,
    /// Number of glass material variants.
    pub(super) glass_materials: usize,
    /// Number of mirror material variants.
    pub(super) mirror_materials: usize,
    /// Number of reflective material variants.
    pub(super) reflective_materials: usize,
    /// Number of light-emitter material variants.
    pub(super) light_emitter_materials: usize,
    /// Number of visual-effect material variants.
    pub(super) visual_effect_materials: usize,
    /// Number of transparent geometry groups.
    pub(super) transparent_geometries: usize,
    /// Number of glass geometry groups.
    pub(super) glass_geometries: usize,
    /// Number of mirror geometry groups.
    pub(super) mirror_geometries: usize,
    /// Number of reflective geometry groups.
    pub(super) reflective_geometries: usize,
    /// Number of light-emitter geometry groups.
    pub(super) light_emitter_geometries: usize,
    /// Number of visual-effect geometry groups.
    pub(super) visual_effect_geometries: usize,
}

impl WorldSurfaceSemanticCounts {
    /// Add one package artifact's overlapping semantic counts.
    pub(super) const fn add(
        &mut self,
        additional: Self,
    ) {
        self.transparent_materials = self
            .transparent_materials
            .saturating_add(additional.transparent_materials);
        self.glass_materials = self
            .glass_materials
            .saturating_add(additional.glass_materials);
        self.mirror_materials = self
            .mirror_materials
            .saturating_add(additional.mirror_materials);
        self.reflective_materials = self
            .reflective_materials
            .saturating_add(additional.reflective_materials);
        self.light_emitter_materials = self
            .light_emitter_materials
            .saturating_add(additional.light_emitter_materials);
        self.visual_effect_materials = self
            .visual_effect_materials
            .saturating_add(additional.visual_effect_materials);
        self.transparent_geometries = self
            .transparent_geometries
            .saturating_add(additional.transparent_geometries);
        self.glass_geometries = self
            .glass_geometries
            .saturating_add(additional.glass_geometries);
        self.mirror_geometries = self
            .mirror_geometries
            .saturating_add(additional.mirror_geometries);
        self.reflective_geometries = self
            .reflective_geometries
            .saturating_add(additional.reflective_geometries);
        self.light_emitter_geometries = self
            .light_emitter_geometries
            .saturating_add(additional.light_emitter_geometries);
        self.visual_effect_geometries = self
            .visual_effect_geometries
            .saturating_add(additional.visual_effect_geometries);
    }
}

/// Aggregate counters for one complete globally aligned world collection.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(super) struct WorldCollectionCounts {
    /// Number of distinct world scopes.
    pub(super) source_scopes: usize,
    /// Number of normalized source packages.
    pub(super) source_packages: usize,
    /// Number of world-geometry FBX artifacts across normal and auxiliary
    /// sets.
    pub(super) world_fbx_files: usize,
    /// Number of root FBXs in the normal three-map import set.
    pub(super) normal_world_fbx_files: usize,
    /// Number of auxiliary FBXs excluded from normal import.
    pub(super) auxiliary_world_fbx_files: usize,
    /// Number of disjoint narrative map groups.
    pub(super) narrative_map_groups: usize,
    /// Number of review-gallery FBX artifacts.
    pub(super) review_fbx_files: usize,
    /// Number of packages without publishable geometry.
    pub(super) packages_without_geometry: usize,
    /// Number of packages using coordinate reference evidence.
    pub(super) coordinate_reference_packages: usize,
    /// Number of packages retaining canonical coordinates.
    pub(super) coordinate_fallback_packages: usize,
    /// Number of interior packages.
    pub(super) interior_packages: usize,
    /// Number of bonus-area packages.
    pub(super) bonus_area_packages: usize,
    /// Number of canonical source meshes considered.
    pub(super) source_meshes: usize,
    /// Number of rejected degenerate render triangles.
    pub(super) discarded_degenerate_triangles: usize,
    /// Number of authored mesh placements.
    pub(super) authored_placements: usize,
    /// Number of placements using verified coordinate evidence.
    pub(super) reference_placements: usize,
    /// Number of placements retaining canonical coordinates.
    pub(super) canonical_placement_fallbacks: usize,
    /// Number of render meshes using verified coordinates.
    pub(super) reference_coordinate_meshes: usize,
    /// Number of render meshes retaining canonical coordinates.
    pub(super) canonical_coordinate_meshes: usize,
    /// Number of definition-only meshes isolated for review.
    pub(super) review_definitions: usize,
    /// Number of independently selectable geometry groups.
    pub(super) independent_item_geometries: usize,
    /// Number of source-backed breakable geometry groups.
    pub(super) breakable_geometries: usize,
    /// Number of source-backed interactable geometry groups.
    pub(super) interactable_geometries: usize,
    /// Number of deterministic review similarity groups.
    pub(super) review_similarity_groups: usize,
    /// Number of source collision meshes excluded from FBX publication.
    pub(super) excluded_collision_meshes: usize,
    /// Number of excluded collision meshes with verified coordinates.
    pub(super) reference_excluded_collision_meshes: usize,
    /// Number of rejected degenerate collision triangles.
    pub(super) discarded_collision_triangles: usize,
}

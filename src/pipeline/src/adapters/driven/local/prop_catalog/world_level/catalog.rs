// File:
//   - catalog.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/prop_catalog/world_level/catalog.
//     rs
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
//   - Master-world provenance counts and deterministic catalog serialization.
// - Must-Not:
//   - Read P3D packages, transform geometry, or write FBX payloads.
// - Allows:
//   - JSON projection of source, coordinate, review, collision, and hash
//     evidence.
// - Summary:
//   - Serializes the separated master-world analysis catalog.
//
// Large file:
//   - false
//

//! Master-world catalog aggregation and deterministic JSON publication.

use std::fs;
use std::io::Write as _;
use std::path::Path;

use serde_json::{Value, json};

use super::model::{ExportedWorldMaster, WorldMasterCounts};
use crate::domain::PipelineError;

/// Aggregate complete publication counts from one master record.
pub(super) fn counts(
    source_packages: usize,
    master: &ExportedWorldMaster,
) -> WorldMasterCounts {
    WorldMasterCounts {
        source_levels: master.source_levels,
        source_packages,
        coordinate_reference_packages: master
            .packages
            .iter()
            .filter(|package| package.coordinate_reference)
            .count(),
        coordinate_fallback_packages: master
            .packages
            .iter()
            .filter(|package| !package.coordinate_reference)
            .count(),
        interior_packages: master.interior_packages,
        source_meshes: master.source_meshes,
        discarded_degenerate_triangles: master.discarded_degenerate_triangles,
        authored_placements: master.authored_placements,
        reference_placements: master.reference_placements,
        canonical_placement_fallbacks: master.canonical_placement_fallbacks,
        reference_coordinate_meshes: master.reference_coordinate_meshes,
        canonical_coordinate_meshes: master.canonical_coordinate_meshes,
        review_definitions: master.review_definitions,
        review_similarity_groups: master.review_similarity_groups,
        collision_meshes: master.collision_meshes,
        reference_collision_meshes: master.reference_collision_meshes,
        discarded_collision_triangles: master.discarded_collision_triangles,
    }
}

/// Write the deterministic master-world catalog.
pub(super) fn write_catalog(
    output_root: &Path,
    counts: WorldMasterCounts,
    master: &ExportedWorldMaster,
) -> Result<(), PipelineError> {
    let value = json!({
        "schema": "shar.world-master-analysis.v1",
        "boundary": {
            "canonical_model_authority": concat!(
                "all topology, materials, UVs, colors, identities, textures, ",
                "and collision indices come from original game P3D packages"
            ),
            "private_coordinate_reference": concat!(
                "an operator-supplied untracked package set may contribute ",
                "only ",
                "scene matrices and topology-verified positions or normals"
            ),
            "review_gallery": concat!(
                "definition-only meshes are retained and co-located in groups ",
                "whose coarse shape similarity score is at least 0.50"
            ),
            "manual_authoring": concat!(
                "the FBX is a separated inspection baseline intended for ",
                "manual ",
                "Blender restructuring; it is not a shipping runtime assembly"
            ),
            "excluded": [
                "gameplay scripts and mission state",
                "audio and particle behavior",
                "camera logic and runtime triggers",
                "reference-only geometry or textures"
            ]
        },
        "counts": counts_value(counts),
        "master": master_value(master)
    });
    let bytes = serde_json::to_vec_pretty(&value)
        .map_err(|error| PipelineError::new(error.to_string()))?;
    let path = output_root.join("world-master.catalog.json");
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)
        .map_err(
            |error| {
                PipelineError::new(
                    format!(
                        "world master catalog create failed for {}: {error}",
                        path.display()
                    ),
                )
            },
        )?;
    file.write_all(&bytes)
        .map_err(|error| PipelineError::new(error.to_string()))
}

/// Render aggregate counts to stable JSON keys.
fn counts_value(counts: WorldMasterCounts) -> Value {
    json!({
        "source_levels": counts.source_levels,
        "source_packages": counts.source_packages,
        "coordinate_reference_packages": counts.coordinate_reference_packages,
        "coordinate_fallback_packages": counts.coordinate_fallback_packages,
        "interior_packages": counts.interior_packages,
        "source_meshes": counts.source_meshes,
        "discarded_degenerate_triangles": counts.discarded_degenerate_triangles,
        "authored_placements": counts.authored_placements,
        "reference_placements": counts.reference_placements,
        "canonical_placement_fallbacks": counts.canonical_placement_fallbacks,
        "reference_coordinate_meshes": counts.reference_coordinate_meshes,
        "canonical_coordinate_meshes": counts.canonical_coordinate_meshes,
        "review_definitions": counts.review_definitions,
        "review_similarity_groups": counts.review_similarity_groups,
        "collision_meshes": counts.collision_meshes,
        "reference_collision_meshes": counts.reference_collision_meshes,
        "discarded_collision_triangles": counts.discarded_collision_triangles
    })
}

/// Render the complete master artifact and every package record.
fn master_value(master: &ExportedWorldMaster) -> Value {
    json!({
        "fbx": {
            "path": master.fbx_path,
            "bytes": master.fbx_bytes,
            "sha256": master.fbx_sha256,
            "geometries": master.summary.geometries,
            "bones": master.summary.bones,
            "clusters": master.summary.clusters,
            "materials": master.summary.materials,
            "textures": master.summary.textures,
            "animations": master.summary.animations
        },
        "textures": master.textures.iter().map(
            |texture| json!({
                "file_name": texture.file_name,
                "bytes": texture.bytes,
                "sha256": texture.sha256
            }),
        ).collect::<Vec<_>>(),
        "packages": master.packages.iter().map(
            |package| json!({
                "level": package.level,
                "package_id": package.package_id,
                "subcategory": package.subcategory,
                "coordinate_reference": package.coordinate_reference,
                "source_meshes": package.source_meshes,
                "discarded_degenerate_triangles": package
                    .discarded_degenerate_triangles,
                "authored_placements": package.authored_placements,
                "reference_placements": package.reference_placements,
                "canonical_placement_fallbacks": package
                    .canonical_placement_fallbacks,
                "reference_coordinate_meshes": package
                    .reference_coordinate_meshes,
                "canonical_coordinate_meshes": package
                    .canonical_coordinate_meshes,
                "review_definitions": package.review_definitions,
                "collision_meshes": package.collision_meshes,
                "reference_collision_meshes": package
                    .reference_collision_meshes,
                "discarded_collision_triangles": package
                    .discarded_collision_triangles,
                "interior_packages": package.interior_packages
            }),
        ).collect::<Vec<_>>()
    })
}

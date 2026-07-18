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
//   - World-package provenance and shared-origin transform manifests.
// - Must-Not:
//   - Read P3D packages, transform geometry, or write FBX payloads.
// - Allows:
//   - Deterministic JSON projection of artifacts, counts, and import
//     transforms.
// - Summary:
//   - Serializes the globally aligned world-package collection manifests.
//
// Large file:
//   - true
//   - Reason: catalog counts, artifact records, and the paired transform
//     manifest are one deterministic publication contract.
//   - Split: separate JSON projections when a second catalog consumer exists.
//   - Validation: canonical pipeline validation and deterministic generation.
//   - Review: required whenever another manifest family is added.
//

//! Globally aligned world-package catalog and transform manifests.

use std::collections::BTreeSet;
use std::fs;
use std::io::Write as _;
use std::path::Path;

use serde_json::{Value, json};

use super::model::{
    ExportedWorldCollection, WorldCollectionCounts, WorldFbxRecord,
    WorldPackageRecord, WorldSurfaceSemanticCounts,
};
use crate::domain::PipelineError;

/// Aggregate complete publication counts from one world collection.
#[expect(
    clippy::too_many_lines,
    reason = "World counts must stay aligned with one collection."
)]
pub(super) fn counts(
    source_packages: usize,
    collection: &ExportedWorldCollection,
) -> WorldCollectionCounts {
    let packages = &collection.packages;
    WorldCollectionCounts {
        source_scopes: packages
            .iter()
            .map(
                |package| {
                    package
                        .scope
                        .as_str()
                },
            )
            .collect::<BTreeSet<_>>()
            .len(),
        source_packages,
        world_fbx_files: packages
            .iter()
            .filter(
                |package| {
                    package
                        .world_fbx
                        .is_some()
                },
            )
            .count(),
        review_fbx_files: packages
            .iter()
            .filter(
                |package| {
                    package
                        .review_fbx
                        .is_some()
                },
            )
            .count(),
        packages_without_geometry: packages
            .iter()
            .filter(
                |package| {
                    package
                        .world_fbx
                        .is_none()
                        && package
                            .review_fbx
                            .is_none()
                },
            )
            .count(),
        coordinate_reference_packages: packages
            .iter()
            .filter(|package| package.coordinate_reference)
            .count(),
        coordinate_fallback_packages: packages
            .iter()
            .filter(|package| !package.coordinate_reference)
            .count(),
        interior_packages: packages
            .iter()
            .filter(|package| package.interior)
            .count(),
        bonus_area_packages: packages
            .iter()
            .filter(|package| package.scope == "bonus-area")
            .count(),
        source_meshes: sum(
            packages,
            |package| package.source_meshes,
        ),
        discarded_degenerate_triangles: sum(
            packages,
            |package| package.discarded_degenerate_triangles,
        ),
        authored_placements: sum(
            packages,
            |package| package.authored_placements,
        ),
        reference_placements: sum(
            packages,
            |package| package.reference_placements,
        ),
        canonical_placement_fallbacks: sum(
            packages,
            |package| package.canonical_placement_fallbacks,
        ),
        reference_coordinate_meshes: sum(
            packages,
            |package| package.reference_coordinate_meshes,
        ),
        canonical_coordinate_meshes: sum(
            packages,
            |package| package.canonical_coordinate_meshes,
        ),
        review_definitions: sum(
            packages,
            |package| package.review_definitions,
        ),
        review_similarity_groups: sum(
            packages,
            |package| package.review_similarity_groups,
        ),
        collision_meshes: sum(
            packages,
            |package| package.collision_meshes,
        ),
        reference_collision_meshes: sum(
            packages,
            |package| package.reference_collision_meshes,
        ),
        discarded_collision_triangles: sum(
            packages,
            |package| package.discarded_collision_triangles,
        ),
    }
}

/// Write the provenance catalog and shared-origin transform manifest.
pub(super) fn write_catalogs(
    output_root: &Path,
    counts: WorldCollectionCounts,
    collection: &ExportedWorldCollection,
) -> Result<(), PipelineError> {
    let catalog = catalog_value(
        counts, collection,
    );
    let transforms = transforms_value(collection);
    write_json(
        &output_root.join("world.catalog.json"),
        &catalog,
    )?;
    write_json(
        &output_root.join("world.transforms.json"),
        &transforms,
    )
}

/// Render the complete WIP collection catalog.
fn catalog_value(
    counts: WorldCollectionCounts,
    collection: &ExportedWorldCollection,
) -> Value {
    json!({
        "schema": "shar.world-package-collection.v1",
        "status": "wip-inspection-checkpoint",
        "boundary": {
            "canonical_model_authority": concat!(
                "topology, materials, UVs, colors, identities, textures, ",
                "and collision indices come from original game P3D packages"
            ),
            "private_coordinate_reference": concat!(
                "an operator-supplied untracked package set may contribute ",
                "only scene matrices and topology-verified coordinates"
            ),
            "shared_origin": [0.0_f64, 0.0_f64, 0.0_f64],
            "root_import_contract": concat!(
                "every FBX directly below this directory has baked global ",
                "coordinates and an identity object transform"
            ),
            "variant_isolation": concat!(
                "bonus, day, night, Halloween, and level variants remain ",
                "independent files and are never merged into one scene"
            ),
            "review_isolation": concat!(
                "definition-only galleries live below review/ and must not be ",
                "included in the normal root-FBX bulk import"
            ),
            "manual_authoring": concat!(
                "this collection is an inspection and manual reconstruction ",
                "baseline, not a finished runtime world"
            )
        },
        "counts": counts_value(counts),
        "surface_semantics": semantics_value(collection.surface_semantics),
        "textures": collection.textures.iter().map(
            |texture| json!({
                "file_name": texture.file_name,
                "bytes": texture.bytes,
                "sha256": texture.sha256
            }),
        ).collect::<Vec<_>>(),
        "packages": collection
            .packages
            .iter()
            .map(package_value)
            .collect::<Vec<_>>()
    })
}

/// Render the root-FBX identity transform manifest.
fn transforms_value(collection: &ExportedWorldCollection) -> Value {
    json!({
        "schema": "shar.world-package-transforms.v2",
        "shared_origin": [0.0_f64, 0.0_f64, 0.0_f64],
        "import": concat!(
            "select only the root *.fbx files; add no per-file placement ",
            "offsets; preserve the importer-created SHAR_Export_Root axis ",
            "conversion transform"
        ),
        "authored_root": {
            "name": "SHAR_Export_Root",
            "preserve_imported_transform": true
        },
        "files": collection
            .packages
            .iter()
            .filter_map(
                |package| {
                    package.world_fbx.as_ref().map(
                        |artifact| json!({
                            "path": artifact.path,
                            "scope": package.scope,
                            "package_id": package.package_id,
                            "subcategory": package.subcategory,
                            "interior": package.interior,
                            "coordinates_baked": true,
                            "additional_translation": [
                                0.0_f64, 0.0_f64, 0.0_f64
                            ],
                            "additional_rotation_degrees": [
                                0.0_f64, 0.0_f64, 0.0_f64
                            ],
                            "additional_scale": [
                                1.0_f64, 1.0_f64, 1.0_f64
                            ]
                        }),
                    )
                },
            )
            .collect::<Vec<_>>()
    })
}

/// Sum one package counter selected by a pure projection.
fn sum(
    packages: &[WorldPackageRecord],
    select: fn(&WorldPackageRecord) -> usize,
) -> usize {
    packages
        .iter()
        .map(select)
        .sum()
}

/// Render aggregate counts to stable JSON keys.
fn counts_value(counts: WorldCollectionCounts) -> Value {
    json!({
        "source_scopes": counts.source_scopes,
        "source_packages": counts.source_packages,
        "world_fbx_files": counts.world_fbx_files,
        "review_fbx_files": counts.review_fbx_files,
        "packages_without_geometry": counts.packages_without_geometry,
        "coordinate_reference_packages": counts.coordinate_reference_packages,
        "coordinate_fallback_packages": counts.coordinate_fallback_packages,
        "interior_packages": counts.interior_packages,
        "bonus_area_packages": counts.bonus_area_packages,
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

/// Render one package plus its normal-import and isolated-review artifacts.
fn package_value(package: &WorldPackageRecord) -> Value {
    json!({
        "scope": package.scope,
        "package_id": package.package_id,
        "subcategory": package.subcategory,
        "coordinate_reference": package.coordinate_reference,
        "interior": package.interior,
        "source_meshes": package.source_meshes,
        "discarded_degenerate_triangles": package
            .discarded_degenerate_triangles,
        "authored_placements": package.authored_placements,
        "reference_placements": package.reference_placements,
        "canonical_placement_fallbacks": package.canonical_placement_fallbacks,
        "reference_coordinate_meshes": package.reference_coordinate_meshes,
        "canonical_coordinate_meshes": package.canonical_coordinate_meshes,
        "review_definitions": package.review_definitions,
        "review_similarity_groups": package.review_similarity_groups,
        "collision_meshes": package.collision_meshes,
        "reference_collision_meshes": package.reference_collision_meshes,
        "discarded_collision_triangles": package.discarded_collision_triangles,
        "world_fbx": package.world_fbx.as_ref().map(artifact_value),
        "review_fbx": package.review_fbx.as_ref().map(artifact_value)
    })
}

/// Render one written FBX artifact with its overlapping surface semantics.
fn artifact_value(artifact: &WorldFbxRecord) -> Value {
    json!({
        "path": artifact.path,
        "bytes": artifact.bytes,
        "sha256": artifact.sha256,
        "geometries": artifact.summary.geometries,
        "bones": artifact.summary.bones,
        "clusters": artifact.summary.clusters,
        "materials": artifact.summary.materials,
        "textures": artifact.summary.textures,
        "animations": artifact.summary.animations,
        "surface_semantics": semantics_value(artifact.surface_semantics)
    })
}

/// Render overlapping semantic material and geometry counts.
fn semantics_value(counts: WorldSurfaceSemanticCounts) -> Value {
    json!({
        "materials": {
            "transparent": counts.transparent_materials,
            "glass": counts.glass_materials,
            "mirror": counts.mirror_materials,
            "reflective": counts.reflective_materials,
            "light_emitter": counts.light_emitter_materials,
            "visual_effect": counts.visual_effect_materials
        },
        "geometries": {
            "transparent": counts.transparent_geometries,
            "glass": counts.glass_geometries,
            "mirror": counts.mirror_geometries,
            "reflective": counts.reflective_geometries,
            "light_emitter": counts.light_emitter_geometries,
            "visual_effect": counts.visual_effect_geometries
        }
    })
}

/// Create one deterministic pretty JSON file without replacement.
fn write_json(
    path: &Path,
    value: &Value,
) -> Result<(), PipelineError> {
    let bytes = serde_json::to_vec_pretty(value)
        .map_err(|error| PipelineError::new(error.to_string()))?;
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(
            |error| {
                PipelineError::new(
                    format!("world manifest create failed: {error}"),
                )
            },
        )?;
    file.write_all(&bytes)
        .map_err(|error| PipelineError::new(error.to_string()))
}

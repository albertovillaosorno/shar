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
use super::movement_catalog::coordinate_movements_value;
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
        normal_world_fbx_files: packages
            .iter()
            .filter(
                |package| {
                    package.normal_import
                        && package
                            .world_fbx
                            .is_some()
                },
            )
            .count(),
        auxiliary_world_fbx_files: packages
            .iter()
            .filter(
                |package| {
                    !package.normal_import
                        && package
                            .world_fbx
                            .is_some()
                },
            )
            .count(),
        narrative_map_groups: packages
            .iter()
            .filter_map(
                |package| {
                    package
                        .map_group
                        .as_deref()
                },
            )
            .collect::<BTreeSet<_>>()
            .len(),
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
        independent_item_geometries: sum(
            packages,
            |package| package.independent_item_geometries,
        ),
        breakable_geometries: sum(
            packages,
            |package| package.breakable_geometries,
        ),
        interactable_geometries: sum(
            packages,
            |package| package.interactable_geometries,
        ),
        review_similarity_groups: sum(
            packages,
            |package| package.review_similarity_groups,
        ),
        excluded_collision_meshes: sum(
            packages,
            |package| package.excluded_collision_meshes,
        ),
        reference_excluded_collision_meshes: sum(
            packages,
            |package| package.reference_excluded_collision_meshes,
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
    let movements = coordinate_movements_value(collection);
    write_json(
        &output_root.join("world.catalog.json"),
        &catalog,
    )?;
    write_json(
        &output_root.join("world.transforms.json"),
        &transforms,
    )?;
    write_json(
        &output_root.join("world.coordinate-movements.json"),
        &movements,
    )
}

/// Render the complete separated world collection catalog.
fn catalog_value(
    counts: WorldCollectionCounts,
    collection: &ExportedWorldCollection,
) -> Value {
    json!({
        "schema": "shar.world-package-collection.v3",
        "status": "authored-coordinate-movement-baseline",
        "boundary": {
            "canonical_model_authority": concat!(
                "topology, materials, UVs, colors, identities, and textures ",
                "come from original game P3D packages"
            ),
            "collision_exclusion": concat!(
                "source collision indices are counted for audit but no ",
                "collision geometry or collision material enters any FBX"
            ),
            "private_coordinate_reference": concat!(
                "an operator-supplied untracked package set may contribute ",
                "only scene matrices and topology-verified coordinates"
            ),
            "three_map_layout": concat!(
                "levels 1, 4, and 7 share map-01-04-07; levels 2 and 5 share ",
                "map-02-05; levels 3 and 6 share map-03-06; independent map ",
                "bounds must remain disjoint"
            ),
            "root_import_contract": concat!(
                "only narrative-level FBXs live at the root; coordinates and ",
                "map-group separation are baked, so no per-file offset is added"
            ),
            "auxiliary_isolation": concat!(
                "bonus-area FBXs live below auxiliary/ and are excluded from ",
                "the normal three-map bulk import"
            ),
            "review_isolation": concat!(
                "definition-only galleries live below review/ and ",
                "are excluded from normal and auxiliary world imports"
            ),
            "interior_uv_policy": concat!(
                "interior packages preserve authored U coordinates; selective ",
                "orientation correction applies only to non-interior graphics"
            ),
            "object_semantics": concat!(
                "source-backed breakable and interactable owners plus ",
                "spatially separated items remain selectable Blender objects"
            ),
            "coordinate_movement": concat!(
                "named package movements are applied above geometry and ",
                "published separately for collision, doors, objects, spawns, ",
                "missions, triggers, cameras, locators, and lights"
            ),
            "manual_evidence": concat!(
                "operator-authored FBX comparisons define reviewed movement ",
                "constants but are never read or modified by production export"
            )
        },
        "map_groups": [
            {
                "id": "map-01-04-07",
                "levels": [1, 4, 7],
                "baked_offset": [0, 0, 0]
            },
            {
                "id": "map-02-05",
                "levels": [2, 5],
                "baked_offset": [8192, 0, 0]
            },
            {
                "id": "map-03-06",
                "levels": [3, 6],
                "baked_offset": [16384, 0, 0]
            }
        ],
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
    let normal_files = transform_files(
        collection, true,
    );
    let auxiliary_files = transform_files(
        collection, false,
    );
    json!({
        "schema": "shar.world-package-transforms.v4",
        "shared_origin": [0.0_f64, 0.0_f64, 0.0_f64],
        "normal_import": concat!(
            "import only root *.fbx files; add no per-file placement offsets; ",
            "preserve each importer-created SHAR_Export_Root axis conversion"
        ),
        "auxiliary_import": concat!(
            "files below auxiliary/ are optional bonus-area evidence and are ",
            "not part of the three narrative map groups"
        ),
        "authored_root": {
            "name": "SHAR_Export_Root",
            "preserve_imported_transform": true
        },
        "files": normal_files,
        "auxiliary_files": auxiliary_files
    })
}

/// Render one normal or auxiliary transform-manifest file list.
fn transform_files(
    collection: &ExportedWorldCollection,
    normal_import: bool,
) -> Vec<Value> {
    collection
        .packages
        .iter()
        .filter(|package| package.normal_import == normal_import)
        .filter_map(
            |package| {
                package
                    .world_fbx
                    .as_ref()
                    .map(
                        |artifact| {
                            transform_file_value(
                                package, artifact,
                            )
                        },
                    )
            },
        )
        .collect()
}

/// Render one world FBX import-transform record.
fn transform_file_value(
    package: &WorldPackageRecord,
    artifact: &WorldFbxRecord,
) -> Value {
    json!({
        "path": artifact.path,
        "scope": package.scope,
        "package_id": package.package_id,
        "subcategory": package.subcategory,
        "interior": package.interior,
        "map_group": package.map_group,
        "baked_map_offset": package.map_offset,
        "coordinate_movement": package.coordinate_movement,
        "coordinates_baked": true,
        "additional_translation": [0.0_f64, 0.0_f64, 0.0_f64],
        "additional_rotation_degrees": [0.0_f64, 0.0_f64, 0.0_f64],
        "additional_scale": [1.0_f64, 1.0_f64, 1.0_f64]
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
    let reference_excluded = counts.reference_excluded_collision_meshes;
    json!({
        "source_scopes": counts.source_scopes,
        "source_packages": counts.source_packages,
        "world_fbx_files": counts.world_fbx_files,
        "normal_world_fbx_files": counts.normal_world_fbx_files,
        "auxiliary_world_fbx_files": counts.auxiliary_world_fbx_files,
        "narrative_map_groups": counts.narrative_map_groups,
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
        "independent_item_geometries": counts.independent_item_geometries,
        "breakable_geometries": counts.breakable_geometries,
        "interactable_geometries": counts.interactable_geometries,
        "review_similarity_groups": counts.review_similarity_groups,
        "excluded_collision_meshes": counts.excluded_collision_meshes,
        "reference_excluded_collision_meshes": reference_excluded,
        "discarded_collision_triangles": counts.discarded_collision_triangles
    })
}

/// Render one package plus its normal-import and isolated-review artifacts.
fn package_value(package: &WorldPackageRecord) -> Value {
    let reference_excluded = package.reference_excluded_collision_meshes;
    json!({
        "scope": package.scope,
        "package_id": package.package_id,
        "subcategory": package.subcategory,
        "coordinate_reference": package.coordinate_reference,
        "interior": package.interior,
        "map_group": package.map_group,
        "map_offset": package.map_offset,
        "normal_import": package.normal_import,
        "coordinate_movement": package.coordinate_movement,
        "source_meshes": package.source_meshes,
        "discarded_degenerate_triangles": package
            .discarded_degenerate_triangles,
        "authored_placements": package.authored_placements,
        "reference_placements": package.reference_placements,
        "canonical_placement_fallbacks": package.canonical_placement_fallbacks,
        "reference_coordinate_meshes": package.reference_coordinate_meshes,
        "canonical_coordinate_meshes": package.canonical_coordinate_meshes,
        "review_definitions": package.review_definitions,
        "independent_item_geometries": package.independent_item_geometries,
        "breakable_geometries": package.breakable_geometries,
        "interactable_geometries": package.interactable_geometries,
        "review_similarity_groups": package.review_similarity_groups,
        "excluded_collision_meshes": package.excluded_collision_meshes,
        "reference_excluded_collision_meshes": reference_excluded,
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

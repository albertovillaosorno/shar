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
//   - Catalog outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Catalog outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Catalog outbound adapter.

use std::collections::BTreeSet;
use std::fs;
use std::io::Write as _;
use std::path::Path;

use serde_json::{Value, json};

use super::model::{
    ExportedWorldCollection, WorldCollectionCounts, WorldFbxRecord,
    WorldInteriorRecord, WorldPackageRecord, WorldSurfaceSemanticCounts,
    WorldTopologyEvidenceRecord,
};
use crate::domain::PipelineError;

/// Aggregate complete publication counts from one world collection.
pub(super) fn counts(
    source_packages: usize,
    collection: &ExportedWorldCollection,
) -> WorldCollectionCounts {
    let packages = &collection.packages;
    WorldCollectionCounts {
        source_scopes: packages
            .iter()
            .map(|package| package.scope.as_str())
            .collect::<BTreeSet<_>>()
            .len(),
        source_packages,
        world_fbx_files: packages
            .iter()
            .filter(|package| package.world_fbx.is_some())
            .count()
            .saturating_add(collection.interiors.len())
            .saturating_add(
                collection
                    .interiors
                    .iter()
                    .filter(|interior| interior.halloween_fbx.is_some())
                    .count(),
            ),
        normal_world_fbx_files: packages
            .iter()
            .filter(|package| package.world_fbx.is_some())
            .count()
            .saturating_add(collection.interiors.len())
            .saturating_add(
                collection
                    .interiors
                    .iter()
                    .filter(|interior| interior.halloween_fbx.is_some())
                    .count(),
            ),
        narrative_map_groups: packages
            .iter()
            .filter_map(|package| package.map_group.as_deref())
            .collect::<BTreeSet<_>>()
            .len(),
        review_fbx_files: packages
            .iter()
            .filter(|package| package.review_fbx.is_some())
            .count(),
        packages_without_geometry: packages
            .iter()
            .filter(|package| {
                !package.interior
                    && package.world_fbx.is_none()
                    && package.review_fbx.is_none()
            })
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
        interior_base_fbx_files: collection.interiors.len(),
        interior_halloween_fbx_files: collection
            .interiors
            .iter()
            .filter(|interior| interior.halloween_fbx.is_some())
            .count(),
        source_meshes: sum(packages, |package| package.source_meshes),
        discarded_degenerate_triangles: sum(packages, |package| {
            package.discarded_degenerate_triangles
        }),
        unreal_omitted_repeated_index_triangles:
            target_omitted_repeated_index_triangles(collection),
        authored_placements: sum(packages, |package| {
            package.authored_placements
        }),
        reference_placements: sum(packages, |package| {
            package.reference_placements
        }),
        canonical_placement_fallbacks: sum(packages, |package| {
            package.canonical_placement_fallbacks
        }),
        reference_coordinate_meshes: sum(packages, |package| {
            package.reference_coordinate_meshes
        }),
        canonical_coordinate_meshes: sum(packages, |package| {
            package.canonical_coordinate_meshes
        }),
        review_definitions: sum(packages, |package| package.review_definitions),
        independent_item_geometries: sum(packages, |package| {
            package.independent_item_geometries
        }),
        breakable_geometries: sum(packages, |package| {
            package.breakable_geometries
        }),
        interactable_geometries: sum(packages, |package| {
            package.interactable_geometries
        }),
        review_similarity_groups: sum(packages, |package| {
            package.review_similarity_groups
        }),
        excluded_collision_meshes: sum(packages, |package| {
            package.excluded_collision_meshes
        }),
        reference_excluded_collision_meshes: sum(packages, |package| {
            package.reference_excluded_collision_meshes
        }),
        discarded_collision_triangles: sum(packages, |package| {
            package.discarded_collision_triangles
        }),
    }
}

/// Write the provenance catalog and shared-origin transform manifest.
pub(super) fn write_catalogs(
    output_root: &Path,
    counts: WorldCollectionCounts,
    collection: &ExportedWorldCollection,
) -> Result<(), PipelineError> {
    let catalog = catalog_value(counts, collection);
    let transforms = transforms_value(collection);
    write_json(&output_root.join("world.catalog.json"), &catalog)?;
    write_json(&output_root.join("world.transforms.json"), &transforms)
}

/// Render the complete separated world collection catalog.
fn catalog_value(
    counts: WorldCollectionCounts,
    collection: &ExportedWorldCollection,
) -> Value {
    json!({
        "schema": "shar.world-package-collection.v6",
        "status": "source-authored-fbx-baseline",
        "boundary": {
            "canonical_model_authority": concat!(
                "topology, positions, materials, UVs, colors, identities, ",
                "and textures come from original game P3D packages"
            ),
            "source_spatial_contract": concat!(
                "world and interior geometry retains source-authored spatial ",
                "coordinates; export never changes the center of, raises, ",
                "translates, rotates, scales, or stitches narrative maps"
            ),
            "fbx_axis_contract": concat!(
                "SHAR_Export_Root reflects source X exactly once for the ",
                "FBX-to-Unreal basis conversion; no geometry correction is ",
                "baked into package meshes"
            ),
            "uv_contract": concat!(
                "every authored U and V value is preserved without heuristic ",
                "mirroring, clamping, or remapping"
            ),
            "collision_exclusion": concat!(
                "source collision indices are counted for audit but no ",
                "collision geometry or collision material enters any FBX"
            ),
            "unreal_topology_contract": concat!(
                "source-authored repeated-index triangles remain exact in ",
                "paired topology sidecars and are omitted only from Unreal ",
                "target FBX geometry because the verified importer cannot ",
                "consume them safely"
            ),
            "narrative_groups": concat!(
                "map group labels describe recurring story families only and ",
                "never imply coordinate offsets or assembly transforms"
            ),
            "review_isolation": concat!(
                "definition-only galleries live below review/ and remain ",
                "excluded from normal world imports"
            ),
            "interior_policy": concat!(
                "interior packages preserve source coordinates and authored ",
                "UVs without base-game relocation, fuse by stable identity, ",
                "and publish Level 7 Halloween geometry only when absent from ",
                "the canonical base; interior relocation belongs to mods"
            )
        },
        "map_groups": [
            {"id": "map-01-04-07", "levels": [1, 4, 7]},
            {"id": "map-02-05", "levels": [2, 5]},
            {"id": "map-03-06", "levels": [3, 6]}
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
            .collect::<Vec<_>>(),
        "interiors": collection
            .interiors
            .iter()
            .map(interior_value)
            .collect::<Vec<_>>()
    })
}

/// Render the root-FBX identity transform manifest.
fn transforms_value(collection: &ExportedWorldCollection) -> Value {
    let files = transform_files(collection);
    json!({
        "schema": "shar.world-package-transforms.v7",
        "shared_origin": [0.0_f64, 0.0_f64, 0.0_f64],
        "import_contract": concat!(
            "import generated FBXs at identity placement and preserve each ",
            "importer-created SHAR_Export_Root basis conversion; source ",
            "coordinates remain untouched"
        ),
        "authored_root": {
            "name": "SHAR_Export_Root",
            "preserve_imported_transform": true
        },
        "files": files
    })
}

/// Render the generated transform-manifest file list.
fn transform_files(collection: &ExportedWorldCollection) -> Vec<Value> {
    let mut files = collection
        .packages
        .iter()
        .filter_map(|package| {
            package
                .world_fbx
                .as_ref()
                .map(|artifact| transform_file_value(package, artifact))
        })
        .collect::<Vec<_>>();
    for interior in &collection.interiors {
        files.push(interior_transform_file_value(
            interior,
            &interior.base_fbx,
            "base",
        ));
        if let Some(artifact) = interior.halloween_fbx.as_ref() {
            files.push(interior_transform_file_value(
                interior,
                artifact,
                "halloween-additions",
            ));
        }
    }
    files.sort_by(|left, right| {
        left["path"].as_str().cmp(&right["path"].as_str())
    });
    files
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
        "source_coordinates_preserved": true,
        "fbx_root_policy": "ReflectX",
        "import_location": [0.0_f64, 0.0_f64, 0.0_f64],
        "import_rotation_degrees": [0.0_f64, 0.0_f64, 0.0_f64],
        "import_scale": [1.0_f64, 1.0_f64, 1.0_f64]
    })
}

/// Render one fused interior FBX import-transform record.
fn interior_transform_file_value(
    interior: &WorldInteriorRecord,
    artifact: &WorldFbxRecord,
    role: &str,
) -> Value {
    json!({
        "path": artifact.path,
        "scope": "fused-interior",
        "interior_id": interior.identity,
        "interior_name": interior.name,
        "interior_role": role,
        "source_package_ids": interior.source_package_ids,
        "source_coordinates_preserved": true,
        "fbx_root_policy": "ReflectX",
        "import_location": [0.0_f64, 0.0_f64, 0.0_f64],
        "import_rotation_degrees": [0.0_f64, 0.0_f64, 0.0_f64],
        "import_scale": [1.0_f64, 1.0_f64, 1.0_f64]
    })
}

/// Count target-only repeated-index omissions across every published FBX.
fn target_omitted_repeated_index_triangles(
    collection: &ExportedWorldCollection,
) -> usize {
    let packages = collection.packages.iter().flat_map(|package| {
        [package.world_fbx.as_ref(), package.review_fbx.as_ref()]
            .into_iter()
            .flatten()
    });
    let interiors = collection.interiors.iter().flat_map(|interior| {
        [Some(&interior.base_fbx), interior.halloween_fbx.as_ref()]
            .into_iter()
            .flatten()
    });
    packages
        .chain(interiors)
        .map(|artifact| artifact.unreal_omitted_repeated_index_triangles)
        .sum()
}

/// Render one exact source-topology evidence sidecar record.
fn topology_evidence_value(record: &WorldTopologyEvidenceRecord) -> Value {
    json!({
        "path": record.path,
        "bytes": record.bytes,
        "sha256": record.sha256,
        "repeated_index_triangles": record.repeated_index_triangles
    })
}

/// Sum one package counter selected by a pure projection.
fn sum(
    packages: &[WorldPackageRecord],
    select: fn(&WorldPackageRecord) -> usize,
) -> usize {
    packages.iter().map(select).sum()
}

/// Render aggregate counts to stable JSON keys.
fn counts_value(counts: WorldCollectionCounts) -> Value {
    let reference_excluded = counts.reference_excluded_collision_meshes;
    json!({
        "source_scopes": counts.source_scopes,
        "source_packages": counts.source_packages,
        "world_fbx_files": counts.world_fbx_files,
        "normal_world_fbx_files": counts.normal_world_fbx_files,
        "narrative_map_groups": counts.narrative_map_groups,
        "review_fbx_files": counts.review_fbx_files,
        "packages_without_geometry": counts.packages_without_geometry,
        "coordinate_reference_packages": counts.coordinate_reference_packages,
        "coordinate_fallback_packages": counts.coordinate_fallback_packages,
        "interior_packages": counts.interior_packages,
        "interior_base_fbx_files": counts.interior_base_fbx_files,
        "interior_halloween_fbx_files": counts.interior_halloween_fbx_files,
        "source_meshes": counts.source_meshes,
        "discarded_degenerate_triangles": counts.discarded_degenerate_triangles,
        "unreal_omitted_repeated_index_triangles":
            counts.unreal_omitted_repeated_index_triangles,
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

/// Render one fused interior and its optional additive Halloween artifact.
fn interior_value(interior: &WorldInteriorRecord) -> Value {
    json!({
        "identity": interior.identity,
        "name": interior.name,
        "source_package_ids": interior.source_package_ids,
        "base_source_package_ids": interior.base_source_package_ids,
        "halloween_source_package_ids": interior.halloween_source_package_ids,
        "removed_duplicate_triangles": interior.removed_duplicate_triangles,
        "base_fbx": artifact_value(&interior.base_fbx),
        "halloween_fbx": interior.halloween_fbx.as_ref().map(artifact_value)
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
        "source_coordinates_preserved": true,
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
        "unreal_omitted_repeated_index_triangles":
            artifact.unreal_omitted_repeated_index_triangles,
        "topology_evidence": artifact.topology_evidence
            .as_ref()
            .map(topology_evidence_value),
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
fn write_json(path: &Path, value: &Value) -> Result<(), PipelineError> {
    let bytes = serde_json::to_vec_pretty(value)
        .map_err(|error| PipelineError::new(error.to_string()))?;
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| {
            PipelineError::new(format!("world manifest create failed: {error}"))
        })?;
    file.write_all(&bytes)
        .map_err(|error| PipelineError::new(error.to_string()))
}

// File:
//   - world_catalog.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/prop_catalog/world_catalog.rs
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
//   - Deterministic world-prop catalog rendering and aggregate counts.
// - Must-Not:
//   - Select source variants or write FBX bytes.
// - Summary:
//   - Records one canonical world prop per readable name.
//
// Large file:
//   - false

//! World-prop catalog rendering.

use std::fs;
use std::path::Path;

use serde_json::{Value, json};

use super::model::PropRoute;
use super::world_model::{ExportedWorldProp, WorldCatalogCounts};
use crate::domain::PipelineError;

/// Compute aggregate world-prop source and publication counts.
pub(super) fn world_counts(
    source_packages: usize,
    occurrences: usize,
    assets: &[ExportedWorldProp],
) -> WorldCatalogCounts {
    WorldCatalogCounts {
        source_packages,
        occurrences,
        assets: assets.len(),
        static_assets: assets
            .iter()
            .filter(|asset| asset.route == PropRoute::Static)
            .count(),
        animated_assets: assets
            .iter()
            .filter(|asset| asset.route == PropRoute::RigidAnimated)
            .count(),
        merged_variants: assets
            .iter()
            .map(|asset| asset.merged_compatible_variants)
            .sum(),
        omitted_variants: assets
            .iter()
            .map(
                |asset| {
                    asset
                        .omitted_visual_variants
                        .len()
                },
            )
            .sum(),
    }
}

/// Write one deterministic world-prop catalog.
///
/// # Errors
///
/// Returns an error when JSON rendering or file publication fails.
pub(super) fn write_world_catalog(
    root: &Path,
    counts: WorldCatalogCounts,
    assets: &[ExportedWorldProp],
) -> Result<(), PipelineError> {
    let payload = json!({
        "schema": "shar.world-model-props.v1",
        "boundary": {
            "output": concat!(
                "one hash-free FBX directory per readable ",
                "world-prop name"
            ),
            "compatible_variants": concat!(
                "merge variants with identical positions, topology, and rig; ",
                "preserve distinct authored clips and texture payloads"
            ),
            "incompatible_variants": concat!(
                "select the richest canonical model and retain omitted ",
                "evidence in this catalog"
            ),
            "unreal_assets": [
                "placement and locators",
                "physics and collision",
                "particles and effects",
                concat!(
                    "tree foliage presentation not owned by the selected ",
                    "trunk meshes"
                ),
                "scripts and gameplay state"
            ]
        },
        "counts": {
            "source_packages": counts.source_packages,
            "model_occurrences": counts.occurrences,
            "unique_names": counts.assets,
            "static_assets": counts.static_assets,
            "rigid_animated_assets": counts.animated_assets,
            "merged_compatible_variants": counts.merged_variants,
            "omitted_visual_variants": counts.omitted_variants
        },
        "assets": assets.iter().map(asset_value).collect::<Vec<_>>()
    });
    let mut bytes = serde_json::to_vec_pretty(&payload).map_err(
        |error| {
            PipelineError::new(
                format!("world prop catalog JSON failed: {error}"),
            )
        },
    )?;
    bytes.push(b'\n');
    fs::write(
        root.join("world-props.catalog.json"),
        bytes,
    )
    .map_err(
        |error| {
            PipelineError::new(
                format!("world prop catalog write failed: {error}"),
            )
        },
    )
}

/// Render one published world prop and its retained provenance.
fn asset_value(asset: &ExportedWorldProp) -> Value {
    json!({
        "asset_id": asset.asset_id,
        "route": asset.route.as_str(),
        "semantic_sha256": asset.semantic_sha256,
        "visual_sha256": asset.visual_sha256,
        "structural_sha256": asset.structural_sha256,
        "rig_sha256": asset.rig_sha256,
        "merged_compatible_variants": asset.merged_compatible_variants,
        "fbx": {
            "path": asset.fbx_path,
            "bytes": asset.fbx_bytes,
            "sha256": asset.fbx_sha256,
            "geometries": asset.summary.geometries,
            "bones": asset.summary.bones,
            "clusters": asset.summary.clusters,
            "materials": asset.summary.materials,
            "textures": asset.summary.textures,
            "animations": asset.summary.animations
        },
        "textures": asset.textures.iter().map(|texture| json!({
            "file_name": texture.file_name,
            "bytes": texture.bytes,
            "sha256": texture.sha256
        })).collect::<Vec<_>>(),
        "sources": asset.aliases.iter().map(|alias| json!({
            "package_id": alias.package_id,
            "subcategory": alias.subcategory,
            "owner_kind": alias.owner_kind,
            "owner_name": alias.owner_name,
            "container_key": alias.container_key
        })).collect::<Vec<_>>(),
        "omitted_visual_variants": asset.omitted_visual_variants.iter()
            .map(|variant| json!({
                "semantic_sha256": variant.semantic_sha256,
                "visual_sha256": variant.visual_sha256,
                "structural_sha256": variant.structural_sha256,
                "route": variant.route.as_str(),
                "source_count": variant.source_count
            }))
            .collect::<Vec<_>>()
    })
}

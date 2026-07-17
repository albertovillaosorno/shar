// File:
//   - catalog.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/prop_catalog/catalog.rs
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
//   - Deterministic original-game prop catalog rendering and inventory totals.
// - Must-Not:
//   - Discover sources, write FBX, or infer missing model capabilities.
// - Allows:
//   - Public-safe provenance, summary counters, hashes, and non-FBX boundaries.
// - Split-When:
//   - Catalog rendering and filesystem inventory gain separate consumers.
// - Merge-When:
//   - Publication owns the same final root transaction without duplication.
// - Summary:
//   - Records every unique model and all duplicate mission/world occurrences.
// - Description:
//   - Explicitly routes non-model evidence to Phase 6 Unreal Assets.
// - Usage:
//   - Written once before atomic prop catalog publication.
// - Defaults:
//   - Assets and aliases are already sorted by upstream deterministic rules.
//
// ADRs:
// - docs/adr/fbx/export/fbx-output-contract-boundary.md
// - docs/adr/pipeline/unreal/native-asset-translation-and-no-copy-paste.md
//
// Large file:
//   - false
//

//! Deterministic original-game prop catalog rendering and inventory totals.

use std::fs;
use std::path::Path;

use serde_json::{Value, json};

use super::model::{ExportedProp, PropCatalogCounts, PropFamily, PropRoute};
use crate::domain::PipelineError;

/// Compute aggregate source and unique-asset counters.
pub(super) fn catalog_counts(
    source_packages: usize,
    occurrences: usize,
    assets: &[ExportedProp],
) -> PropCatalogCounts {
    PropCatalogCounts {
        source_packages,
        occurrences,
        assets: assets.len(),
        mission_assets: assets
            .iter()
            .filter(|asset| asset.family == PropFamily::Missions)
            .count(),
        static_assets: assets
            .iter()
            .filter(|asset| asset.route == PropRoute::Static)
            .count(),
        animated_assets: assets
            .iter()
            .filter(|asset| asset.route == PropRoute::RigidAnimated)
            .count(),
    }
}

/// Write the deterministic mission-prop catalog.
///
/// # Errors
///
/// Returns an error when JSON serialization or file publication fails.
pub(super) fn write_catalog(
    root: &Path,
    counts: PropCatalogCounts,
    assets: &[ExportedProp],
) -> Result<(), PipelineError> {
    let payload = json!({
        "schema": "shar.mission-model-props.v1",
        "boundary": {
            "fbx_includes": [
                "model geometry",
                "diffuse materials and external textures",
                "vertex colors",
                "authored rigid skeletons and exact PTRN clips"
            ],
            "phase_6_unreal_assets": [
                "physics and collision",
                "placement and locators",
                "cameras and lights",
                "particles and sounds",
                "scripts and gameplay logic",
                "quad-only mission markers and data-only packages"
            ]
        },
        "counts": {
            "source_packages": counts.source_packages,
            "model_occurrences": counts.occurrences,
            "unique_assets": counts.assets,
            "mission_assets": counts.mission_assets,
            "static_assets": counts.static_assets,
            "rigid_animated_assets": counts.animated_assets
        },
        "assets": assets.iter().map(asset_value).collect::<Vec<_>>()
    });
    let mut bytes = serde_json::to_vec_pretty(&payload).map_err(
        |error| {
            PipelineError::new(
                format!("mission prop catalog JSON failed: {error}"),
            )
        },
    )?;
    bytes.push(b'\n');
    fs::write(
        root.join("mission-props.catalog.json"),
        bytes,
    )
    .map_err(
        |error| {
            PipelineError::new(
                format!("mission prop catalog write failed: {error}"),
            )
        },
    )
}

/// Inventory every final file under the staging catalog root.
///
/// # Errors
///
/// Returns an error when directory traversal or byte totals fail.
pub(super) fn inventory(
    root: &Path
) -> Result<
    (
        usize,
        u64,
    ),
    PipelineError,
> {
    let mut stack = vec![root.to_path_buf()];
    let mut files = 0_usize;
    let mut bytes = 0_u64;
    while let Some(directory) = stack.pop() {
        for entry in fs::read_dir(&directory).map_err(
            |error| {
                PipelineError::new(
                    format!(
                        "prop catalog inventory failed for {}: {error}",
                        directory.display()
                    ),
                )
            },
        )? {
            let path = entry
                .map_err(|error| PipelineError::new(error.to_string()))?
                .path();
            if path.is_dir() {
                stack.push(path);
            } else if path.is_file() {
                files = files
                    .checked_add(1)
                    .ok_or_else(
                        || {
                            PipelineError::new(
                                "prop catalog file count overflowed",
                            )
                        },
                    )?;
                bytes = bytes
                    .checked_add(
                        fs::metadata(&path)
                            .map_err(
                                |error| {
                                    PipelineError::new(
                                        format!(
                                            "prop catalog metadata failed: \
                                             {error}"
                                        ),
                                    )
                                },
                            )?
                            .len(),
                    )
                    .ok_or_else(
                        || {
                            PipelineError::new(
                                "prop catalog byte total overflowed",
                            )
                        },
                    )?;
            }
        }
    }
    Ok(
        (
            files, bytes,
        ),
    )
}

/// Render one exported asset and all source aliases.
fn asset_value(asset: &ExportedProp) -> Value {
    json!({
        "asset_id": asset.asset_id,
        "family": asset.family.as_str(),
        "route": asset.route.as_str(),
        "semantic_sha256": asset.signature,
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
        })).collect::<Vec<_>>()
    })
}

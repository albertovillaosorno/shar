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
//   - World catalog outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - World catalog outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! World catalog outbound adapter.

use std::fs;
use std::path::Path;

use serde_json::{Value, json};

use super::model::{
    DeferredControllerBinding, DeferredRenderBinding, PropRoute,
};
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
            .map(|asset| asset.omitted_visual_variants.len())
            .sum(),
        deferred_render_bindings: assets
            .iter()
            .flat_map(|asset| asset.aliases.iter())
            .map(|alias| alias.deferred_render_bindings.len())
            .sum(),
        deferred_controller_bindings: assets
            .iter()
            .flat_map(|asset| asset.aliases.iter())
            .flat_map(|alias| alias.deferred_render_bindings.iter())
            .filter(|binding| binding.controller.is_some())
            .count(),
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
        "schema": "shar.world-model-props.v3",
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
            "deferred_render_bindings": concat!(
                "retain authored non-mesh composite prop relationships and ",
                "exact controller/animation links as source evidence without ",
                "substituting static FBX geometry"
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
            "omitted_visual_variants": counts.omitted_variants,
            "deferred_render_bindings": counts.deferred_render_bindings,
            "deferred_controller_bindings":
                counts.deferred_controller_bindings
        },
        "assets": assets.iter().map(asset_value).collect::<Vec<_>>()
    });
    let mut bytes = serde_json::to_vec_pretty(&payload).map_err(|error| {
        PipelineError::new(format!("world prop catalog JSON failed: {error}"))
    })?;
    bytes.push(b'\n');
    fs::write(root.join("world-props.catalog.json"), bytes).map_err(|error| {
        PipelineError::new(format!("world prop catalog write failed: {error}"))
    })
}

/// Render one exact deferred controller and animation relationship.
fn deferred_controller_value(binding: &DeferredControllerBinding) -> Value {
    json!({
        "controller_identity": binding.controller_identity,
        "controller_kind": binding.controller_kind,
        "controller_member_id": binding.controller_member_id,
        "controller_source_ordinal": binding.controller_source_ordinal,
        "controller_version": binding.controller_version,
        "controller_type": binding.controller_type,
        "frame_offset": f32::from_bits(binding.frame_offset_bits),
        "animation_identity": binding.animation_identity,
        "animation_member_id": binding.animation_member_id,
        "animation_source_ordinal": binding.animation_source_ordinal,
        "animation_version": binding.animation_version,
        "animation_type": binding.animation_type
    })
}

/// Render one deferred non-mesh composite relationship without inference.
fn deferred_binding_value(binding: &DeferredRenderBinding) -> Value {
    json!({
        "composite_prop_index": binding.composite_prop_index,
        "source_identity": binding.source_identity,
        "skeleton_joint_id": binding.skeleton_joint_id,
        "is_translucent": binding.is_translucent,
        "component_kind": binding.component_kind,
        "component_member_id": binding.component_member_id,
        "source_ordinal": binding.source_ordinal,
        "controller": binding.controller.as_ref().map(deferred_controller_value)
    })
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
            "container_key": alias.container_key,
            "deferred_render_bindings": alias.deferred_render_bindings
                .iter()
                .map(deferred_binding_value)
                .collect::<Vec<_>>()
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

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/prop_catalog/world_catalog/tests.rs"]
mod tests;

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
//   - Canonical editor-facing projection of verified world material evidence.
// - Must-Not:
//   - Read source catalogs, contact Unreal Editor, or claim native readiness.
// - Allows:
//   - Deduplicate already verified presentations and retain exact slot joins.
// - Split-When:
//   - Native operation selection gains an independent readiness lifecycle.
// - Merge-When:
//   - Another adapter owns the identical world-material evidence projection.
// - Summary:
//   - Verified world material plan-evidence renderer.
// - Description:
//   - Renders canonical textures, master families, material presentations, and
//   - per-FBX slot assignments without reparsing source packages.
// - Usage:
//   - Published by prepare-unreal as a revision-bound semantic artifact.
// - Defaults:
//   - Missing catalogs render an explicit empty document; conflicts fail
//   - closed.
//

//! Verified world material plan-evidence renderer.

use std::collections::BTreeMap;

use serde_json::{Value, json};
use shar_unreal_conversion::domain::{
    WORLD_MATERIAL_SOURCE_SCHEMA, WorldMaterialAlphaCompare,
    WorldMaterialBlendFamily, WorldMaterialMasterFamily,
    WorldMaterialNativeMasterClassification, WorldMaterialNativeMasterRecipe,
    WorldMaterialPresentation, WorldMaterialSemantics,
    WorldMaterialShaderFamily, classify_world_material_native_master,
};

use super::unreal_world_material_catalog::VerifiedWorldMaterialCatalog;
use crate::domain::{PipelineError, PipelineOutcome};

pub(super) const WORLD_MATERIAL_PLAN_SCHEMA: &str =
    "shar-schoenwald.unreal-world-material-evidence.v2";

/// Render one canonical world-material evidence document.
///
/// # Errors
///
/// Returns an error if retained verifier counts drift, one global presentation
/// digest maps to conflicting state, or serialization fails.
pub(super) fn render_world_material_plan(
    catalog: Option<&VerifiedWorldMaterialCatalog>,
) -> PipelineOutcome<String> {
    let Some(catalog) = catalog else {
        return serialize_plan(&json!({
            "schema": WORLD_MATERIAL_PLAN_SCHEMA,
            "source_schema": WORLD_MATERIAL_SOURCE_SCHEMA,
            "counts": {
                "artifacts": 0,
                "bindings": 0,
                "slots": 0,
                "master_families": 0,
                "native_master_recipes": 0,
                "native_master_ready_presentations": 0,
                "native_master_blocked_presentations": 0,
                "textures": 0,
                "presentations": 0
            },
            "textures": [],
            "master_families": [],
            "native_master_recipes": [],
            "presentations": [],
            "artifacts": []
        }));
    };
    if catalog.artifact_count != catalog.artifacts.len() {
        return Err(PipelineError::new(
            "verified world material artifact count drifted",
        ));
    }

    let mut masters = BTreeMap::<String, WorldMaterialMasterFamily>::new();
    let mut presentations =
        BTreeMap::<String, WorldMaterialPresentation>::new();
    let mut assignment_count = 0usize;
    for artifact in &catalog.artifacts {
        assignment_count = assignment_count
            .saturating_add(artifact.projection.assignments().len());
        for presentation in artifact.projection.presentations() {
            let master = presentation.raster.master;
            let master_identity = master.identity();
            if masters
                .insert(master_identity.clone(), master)
                .is_some_and(|existing| existing != master)
            {
                return Err(PipelineError::new(
                    "world material master identity has conflicting state",
                ));
            }
            match presentations.get(&presentation.slot_presentation_sha256) {
                Some(existing) if existing != presentation => {
                    return Err(PipelineError::new(
                        "world material presentation digest conflicts globally",
                    ));
                },
                Some(_) => {},
                None => {
                    let _previous = presentations.insert(
                        presentation.slot_presentation_sha256.clone(),
                        presentation.clone(),
                    );
                },
            }
        }
    }
    if assignment_count != catalog.slot_count
        || masters.len() != catalog.master_family_count
    {
        return Err(PipelineError::new(
            "verified world material retained projection counts drifted",
        ));
    }

    let texture_values = catalog
        .textures
        .iter()
        .map(|texture| {
            json!({
                "file_name": texture.file_name,
                "source_path": format!(
                    "world-assets/textures/{}",
                    texture.file_name
                ),
                "bytes": texture.bytes,
                "sha256": texture.sha256
            })
        })
        .collect::<Vec<_>>();
    let master_values = masters
        .iter()
        .map(|(identity, master)| master_value(identity, *master))
        .collect::<Vec<_>>();
    let mut native_recipes =
        BTreeMap::<String, WorldMaterialNativeMasterRecipe>::new();
    let mut native_ready_count = 0usize;
    let mut native_blocked_count = 0usize;
    let mut presentation_values = Vec::with_capacity(presentations.len());
    for presentation in presentations.values() {
        let classification = classify_world_material_native_master(
            presentation.raster.master,
            presentation.semantics,
        );
        let recipe_identity = if let Some(recipe) = classification.recipe() {
            native_ready_count = native_ready_count.saturating_add(1);
            let identity = recipe.identity();
            if native_recipes
                .insert(identity.clone(), recipe)
                .is_some_and(|existing| existing != recipe)
            {
                return Err(PipelineError::new(
                    "native world master recipe identity conflicts",
                ));
            }
            Some(identity)
        } else {
            native_blocked_count = native_blocked_count.saturating_add(1);
            None
        };
        presentation_values.push(presentation_value(
            presentation,
            recipe_identity.as_deref(),
            &classification,
        ));
    }
    let native_recipe_values = native_recipes
        .iter()
        .map(|(identity, recipe)| native_recipe_value(identity, *recipe))
        .collect::<Vec<_>>();
    let artifact_values = catalog
        .artifacts
        .iter()
        .map(|artifact| {
            json!({
                "source_path": format!("world-assets/{}", artifact.path),
                "bytes": artifact.bytes,
                "sha256": artifact.sha256,
                "assignments": artifact
                    .projection
                    .assignments()
                    .iter()
                    .map(|assignment| json!({
                        "slot_name": assignment.slot_name,
                        "source_material_name": assignment.source_material_name,
                        "binding_sha256": assignment.binding_sha256,
                        "presentation_sha256": assignment.presentation_sha256,
                        "slot_presentation_sha256":
                            assignment.slot_presentation_sha256
                    }))
                    .collect::<Vec<_>>()
            })
        })
        .collect::<Vec<_>>();

    serialize_plan(&json!({
        "schema": WORLD_MATERIAL_PLAN_SCHEMA,
        "source_schema": WORLD_MATERIAL_SOURCE_SCHEMA,
        "counts": {
            "artifacts": catalog.artifact_count,
            "bindings": catalog.binding_count,
            "slots": catalog.slot_count,
            "master_families": catalog.master_family_count,
            "native_master_recipes": native_recipes.len(),
            "native_master_ready_presentations": native_ready_count,
            "native_master_blocked_presentations": native_blocked_count,
            "textures": catalog.textures.len(),
            "presentations": presentations.len()
        },
        "textures": texture_values,
        "master_families": master_values,
        "native_master_recipes": native_recipe_values,
        "presentations": presentation_values,
        "artifacts": artifact_values
    }))
}

fn master_value(identity: &str, master: WorldMaterialMasterFamily) -> Value {
    json!({
        "identity": identity,
        "shader": shader_token(master.shader),
        "blend": blend_token(master.blend),
        "alpha_compare": master.alpha_compare.map(alpha_compare_token),
        "two_sided": master.two_sided,
        "lit": master.lit
    })
}

fn presentation_value(
    presentation: &WorldMaterialPresentation,
    native_recipe_identity: Option<&str>,
    classification: &WorldMaterialNativeMasterClassification,
) -> Value {
    json!({
        "slot_presentation_sha256": presentation.slot_presentation_sha256,
        "texture_file_name": presentation.texture_file_name,
        "texture_sha256": presentation.texture_sha256,
        "base_color_rgba8": presentation.base_color_rgba8,
        "master_family": presentation.raster.master.identity(),
        "alpha_reference_bits":
            presentation.raster.instance.alpha_reference_bits,
        "semantics": semantics_value(presentation.semantics),
        "native_master_recipe": native_recipe_identity,
        "native_master_blockers": classification
            .blockers()
            .iter()
            .map(|blocker| blocker.code())
            .collect::<Vec<_>>()
    })
}

fn native_recipe_value(
    identity: &str,
    recipe: WorldMaterialNativeMasterRecipe,
) -> Value {
    json!({
        "identity": identity,
        "blend_family": recipe.blend.tool_token(),
        "alpha_test": recipe.alpha_test,
        "render_both_faces": recipe.render_both_faces
    })
}

fn semantics_value(semantics: WorldMaterialSemantics) -> Value {
    json!({
        "transparent": semantics.transparent,
        "glass": semantics.glass,
        "mirror": semantics.mirror,
        "reflective": semantics.reflective,
        "light_emitter": semantics.light_emitter,
        "visual_effect": semantics.visual_effect
    })
}

const fn shader_token(shader: WorldMaterialShaderFamily) -> &'static str {
    match shader {
        WorldMaterialShaderFamily::Simple => "simple",
        WorldMaterialShaderFamily::Environment => "environment",
        WorldMaterialShaderFamily::RuntimeError => "runtime-error",
    }
}

const fn blend_token(blend: WorldMaterialBlendFamily) -> &'static str {
    match blend {
        WorldMaterialBlendFamily::Disabled => "disabled",
        WorldMaterialBlendFamily::SourceAlpha => "source-alpha",
        WorldMaterialBlendFamily::Additive => "additive",
    }
}

const fn alpha_compare_token(
    compare: WorldMaterialAlphaCompare,
) -> &'static str {
    match compare {
        WorldMaterialAlphaCompare::Greater => "greater",
    }
}

fn serialize_plan(value: &Value) -> PipelineOutcome<String> {
    let mut text = serde_json::to_string(&value).map_err(|_error| {
        PipelineError::new("serialize world material evidence failed")
    })?;
    text.push('\n');
    Ok(text)
}

#[cfg(test)]
// jig-ignore-next-line: repository test module path is indivisible
#[path = "../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/unreal_world_material_plan/tests.rs"]
mod tests;

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
//   - Canonical generated destinations and reviewed native construction
//     requests for verified world textures, masters, and Material Instances.
// - Must-Not:
//   - Read source files, contact Unreal Editor, assign mesh slots, save assets,
//     or promote global operation readiness.
// - Allows:
//   - Content-addressed Texture2D identities and typed simple/unlit master and
//     Material Instance request projection from already verified evidence.
// - Split-When:
//   - Slot assignment, save/read-back, or another material family gains an
//     independent native construction lifecycle.
// - Merge-When:
//   - Another adapter owns identical world-material destination selection.
// - Summary:
//   - Native world-material construction request projection.
// - Description:
//   - Selects generated Unreal identities without depending on package-local
//     duplicate texture imports and validates master/instance requests through
//     the asset-conversion domain before sidecar publication.
// - Usage:
//   - Called by world-material evidence rendering after global presentation
//     deduplication and native master classification.
// - Defaults:
//   - Unsupported presentations emit no Material Instance request; verified
//   - textures remain available for later reviewed material families.
//

//! Native world-material construction request projection.

use std::collections::BTreeMap;

use serde_json::{Value, json};
use shar_unreal_conversion::domain::{
    WorldMaterialNativeInstanceRequest, WorldMaterialNativeMasterRecipe,
    WorldMaterialNativeMasterRequest, WorldMaterialPresentation,
};

use super::unreal_world_material_catalog::VerifiedWorldTexture;
use crate::domain::{PipelineError, PipelineOutcome};

const WORLD_TEXTURE_FOLDER: &str = "/Game/Generated/SHAR/Textures/World";
const WORLD_MASTER_FOLDER: &str =
    "/Game/Generated/SHAR/Materials/World/Masters";
const WORLD_INSTANCE_FOLDER: &str =
    "/Game/Generated/SHAR/Materials/World/Instances";

/// Canonical native construction projection retained in world-material
/// evidence.
#[derive(Debug)]
pub(super) struct WorldMaterialNativeConstructionPlan {
    pub(super) texture_requests: Vec<Value>,
    pub(super) master_requests: Vec<Value>,
    pub(super) instance_requests: Vec<Value>,
}

/// Build deterministic generated destinations for reviewed world presentation.
///
/// # Errors
///
/// Returns an error when verified texture identity drifts, a presentation names
/// a texture absent from the verified catalog, or a selected destination fails
/// one of the reviewed typed native request contracts.
pub(super) fn plan_world_material_native_construction(
    textures: &[VerifiedWorldTexture],
    presentations: &BTreeMap<String, WorldMaterialPresentation>,
    recipes: &BTreeMap<String, WorldMaterialNativeMasterRecipe>,
) -> PipelineOutcome<WorldMaterialNativeConstructionPlan> {
    let mut texture_paths = BTreeMap::<String, String>::new();
    let mut texture_request_values = BTreeMap::<String, Value>::new();
    for texture in textures {
        let expected_file_name = format!("texture-{}.png", texture.sha256);
        if texture.file_name != expected_file_name {
            return Err(PipelineError::new(
                concat!(
                    "verified world texture identity drifted before native ",
                    "planning"
                ),
            ));
        }
        let asset_name = format!("T_World_{}", texture.sha256);
        let package_path = format!("{WORLD_TEXTURE_FOLDER}/{asset_name}");
        let object_path = format!("{package_path}.{asset_name}");
        if texture_paths
            .insert(texture.sha256.clone(), object_path.clone())
            .is_some()
        {
            return Err(PipelineError::new(
                "verified world texture digest is duplicated",
            ));
        }
        let previous = texture_request_values.insert(
            texture.sha256.clone(),
            json!({
                "sha256": texture.sha256,
                "source_path": format!(
                    "world-assets/textures/{}",
                    texture.file_name
                ),
                "bytes": texture.bytes,
                "folder_path": WORLD_TEXTURE_FOLDER,
                "asset_name": asset_name,
                "package_path": package_path,
                "object_path": object_path
            }),
        );
        if previous.is_some() {
            return Err(PipelineError::new(
                "verified world texture request identity is duplicated",
            ));
        }
    }
    let texture_requests = texture_request_values.into_values().collect();

    let mut master_paths = BTreeMap::<String, String>::new();
    let mut master_requests = Vec::with_capacity(recipes.len());
    for (identity, recipe) in recipes {
        if identity != &recipe.identity() {
            return Err(PipelineError::new(
                "native world master recipe identity drifted before planning",
            ));
        }
        let asset_name = format!(
            "M_SHAR_World_{}",
            canonical_name_fragment(identity)
        );
        let request = WorldMaterialNativeMasterRequest::new(
            *recipe,
            WORLD_MASTER_FOLDER,
            &asset_name,
        )
        .map_err(|_error| {
            PipelineError::new("native world master request is invalid")
        })?;
        if master_paths
            .insert(identity.clone(), request.object_path().to_owned())
            .is_some()
        {
            return Err(PipelineError::new(
                "native world master request identity is duplicated",
            ));
        }
        master_requests.push(json!({
            "recipe_identity": identity,
            "blend_family": recipe.blend.tool_token(),
            "alpha_test": recipe.alpha_test,
            "render_both_faces": recipe.render_both_faces,
            "folder_path": request.folder_path(),
            "asset_name": request.asset_name(),
            "package_path": request.package_path(),
            "object_path": request.object_path()
        }));
    }

    let mut instance_requests = Vec::new();
    for (presentation_identity, presentation) in presentations {
        if presentation_identity != &presentation.slot_presentation_sha256 {
            return Err(PipelineError::new(
                "world material presentation identity drifted before planning",
            ));
        }
        let classification = shar_unreal_conversion::domain::
            classify_world_material_native_master(
                presentation.raster.master,
                presentation.semantics,
            );
        let Some(recipe) = classification.recipe() else {
            continue;
        };
        let recipe_identity = recipe.identity();
        let parent_material_path = master_paths
            .get(&recipe_identity)
            .ok_or_else(|| {
                PipelineError::new(
                    "native world presentation has no planned master request",
                )
            })?;
        let base_color_texture_path = presentation
            .texture_sha256
            .as_ref()
            .map(|sha256| {
                texture_paths.get(sha256).ok_or_else(|| {
                    PipelineError::new(
                        "native world presentation texture is not planned",
                    )
                })
            })
            .transpose()?;
        let asset_name = format!("MI_World_{presentation_identity}");
        let request = WorldMaterialNativeInstanceRequest::new(
            presentation,
            recipe,
            WORLD_INSTANCE_FOLDER,
            &asset_name,
            parent_material_path,
            base_color_texture_path.map(String::as_str),
        )
        .map_err(|_error| {
            PipelineError::new(
                "native world Material Instance request is invalid",
            )
        })?;
        instance_requests.push(json!({
            "slot_presentation_sha256": presentation_identity,
            "recipe_identity": recipe_identity,
            "texture_sha256": presentation.texture_sha256,
            "folder_path": request.folder_path(),
            "asset_name": request.asset_name(),
            "package_path": request.package_path(),
            "object_path": request.object_path(),
            "parent_material_path": request.parent_material_path(),
            "base_color_texture_path": request.base_color_texture_path(),
            "base_color_tint": request.base_color_tint(),
            "set_alpha_reference": request.set_alpha_reference(),
            "alpha_reference": request.alpha_reference(),
            "alpha_reference_bits":
                presentation.raster.instance.alpha_reference_bits
        }));
    }

    Ok(WorldMaterialNativeConstructionPlan {
        texture_requests,
        master_requests,
        instance_requests,
    })
}

fn canonical_name_fragment(identity: &str) -> String {
    let mut output = String::with_capacity(identity.len());
    let mut separator = false;
    for byte in identity.bytes() {
        if byte.is_ascii_alphanumeric() {
            if separator && !output.is_empty() {
                output.push('_');
            }
            output.push(char::from(byte));
            separator = false;
        } else if !output.is_empty() {
            separator = true;
        }
    }
    output
}

#[cfg(test)]
// jig-ignore-next-line: repository test module path is indivisible
#[path = "../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/unreal_world_material_native_plan/tests.rs"]
mod tests;

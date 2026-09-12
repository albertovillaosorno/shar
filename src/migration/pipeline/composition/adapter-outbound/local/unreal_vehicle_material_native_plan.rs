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
//   - Canonical generated destinations and construction requests for the
//     reviewed vehicle simple/unlit and opaque simple/lit subsets.
// - Must-Not:
//   - Read source files, contact Unreal Editor, assign mesh slots, save assets,
//     or reuse world-only material policy.
// - Allows:
//   - Content-addressed vehicle Texture2D requests, deduplicated reviewed
//     masters, and one Material Instance request per graph-ready vehicle slot.
// - Split-When:
//   - Slot assignment or runtime presentation mutation gains native policy.
// - Merge-When:
//   - Another vehicle adapter owns identical generated destination selection.
// - Summary:
//   - Native vehicle-material construction request projection.
// - Description:
//   - Projects verified simple/unlit and opaque simple/lit PDDI slots into
//   - create-only requests while preserving source texture and render state.
// - Usage:
//   - Called by vehicle-material evidence rendering after catalog verification.
// - Defaults:
//   - Unsupported slots emit no construction request and remain blocked.
//

//! Native vehicle-material construction request projection.

use std::collections::BTreeMap;

use serde_json::{Value, json};
use shar_sha256::digest_hex;

use super::unreal_vehicle_catalog::VerifiedVehicleFbxArtifact;
use super::unreal_vehicle_material_plan::{
    is_simple_lit_opaque_graph_candidate, is_simple_unlit_graph_candidate,
};
use crate::domain::{PipelineError, PipelineOutcome};

const VEHICLE_TEXTURE_FOLDER: &str = "/Game/Generated/SHAR/Textures/Vehicles";
const VEHICLE_MASTER_FOLDER: &str =
    "/Game/Generated/SHAR/Materials/Vehicles/Masters";
const VEHICLE_INSTANCE_FOLDER: &str =
    "/Game/Generated/SHAR/Materials/Vehicles/Instances";

#[derive(Debug, Default)]
pub(super) struct VehicleMaterialNativeConstructionPlan {
    pub(super) texture_requests: Vec<Value>,
    pub(super) master_requests: Vec<Value>,
    pub(super) instance_requests: Vec<Value>,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum MasterRecipe {
    SimpleUnlit {
        blend_mode: u32,
        alpha_test: bool,
        two_sided: bool,
    },
    SimpleLitOpaque {
        shininess_bits: u32,
        two_sided: bool,
    },
}

impl MasterRecipe {
    fn identity(self) -> String {
        match self {
            Self::SimpleUnlit {
                blend_mode,
                alpha_test,
                two_sided,
            } => format!(
                "simple-unlit-blend-{}-alpha-test-{}-{}",
                match blend_mode {
                    0 => "opaque",
                    1 => "alpha",
                    2 => "additive",
                    _ => "unsupported",
                },
                if alpha_test { "on" } else { "off" },
                if two_sided { "both-faces" } else { "one-sided" }
            ),
            Self::SimpleLitOpaque {
                shininess_bits,
                two_sided,
            } => format!(
                "simple-lit-opaque-shininess-{shininess_bits:08x}-{}",
                if two_sided { "both-faces" } else { "one-sided" }
            ),
        }
    }

    const fn alpha_test(self) -> bool {
        match self {
            Self::SimpleUnlit { alpha_test, .. } => alpha_test,
            Self::SimpleLitOpaque { .. } => false,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct TextureRequest {
    sha256: String,
    source_path: String,
    bytes: u64,
}

/// Build deterministic native requests for every reviewed vehicle graph slot.
///
/// # Errors
///
/// Returns an error when a graph-ready slot has inconsistent texture evidence,
/// invalid alpha state, or a generated identity collides with conflicting
/// source evidence.
pub(super) fn plan_vehicle_material_native_construction(
    catalog: &[VerifiedVehicleFbxArtifact],
) -> PipelineOutcome<VehicleMaterialNativeConstructionPlan> {
    let mut textures = BTreeMap::<String, TextureRequest>::new();
    let mut recipes = BTreeMap::<MasterRecipe, String>::new();
    let mut instances = Vec::<Value>::new();

    for vehicle in catalog {
        let source_folder = vehicle
            .evidence
            .path
            .rsplit_once('/')
            .map(|(folder, _)| folder)
            .ok_or_else(|| {
                PipelineError::new(
                    "vehicle FBX path has no native material source folder",
                )
            })?;
        for (slot_index, slot) in vehicle.material_slots.iter().enumerate() {
            let recipe = if is_simple_unlit_graph_candidate(slot) {
                MasterRecipe::SimpleUnlit {
                    blend_mode: slot.raster.blend_mode,
                    alpha_test: slot.raster.alpha_test,
                    two_sided: slot.raster.two_sided,
                }
            } else if is_simple_lit_opaque_graph_candidate(slot) {
                MasterRecipe::SimpleLitOpaque {
                    shininess_bits: slot.raster.shininess_bits,
                    two_sided: slot.raster.two_sided,
                }
            } else {
                continue;
            };
            let recipe_identity = recipe.identity();
            let _planned_identity = recipes
                .entry(recipe)
                .or_insert_with(|| recipe_identity.clone());

            let texture_sha256 = slot.texture_sha256.as_deref();
            let texture_path = match (
                slot.texture_path.as_deref(),
                slot.texture_size_bytes,
                texture_sha256,
            ) {
                (None, None, None) => None,
                (Some(path), Some(bytes), Some(sha256)) => {
                    let request = TextureRequest {
                        sha256: sha256.to_owned(),
                        source_path: format!("{source_folder}/{path}"),
                        bytes,
                    };
                    match textures.get(sha256) {
                        Some(existing)
                            if existing.bytes != request.bytes =>
                        {
                            return Err(PipelineError::new(
                                "vehicle texture digest has conflicting size",
                            ));
                        },
                        Some(existing) => {
                            if request.source_path < existing.source_path {
                                let _old = textures.insert(
                                    sha256.to_owned(),
                                    request,
                                );
                            }
                        },
                        None => {
                            let _old =
                                textures.insert(sha256.to_owned(), request);
                        },
                    }
                    Some(texture_object_path(sha256))
                },
                _ => {
                    return Err(PipelineError::new(
                        "vehicle graph slot texture evidence is incomplete",
                    ));
                },
            };
            let alpha_reference = if recipe.alpha_test() {
                let bits = slot.raster.alpha_reference_bits.ok_or_else(|| {
                    PipelineError::new(
                        "vehicle alpha-test slot lost alpha reference",
                    )
                })?;
                let value = f32::from_bits(bits);
                if !value.is_finite() || !(0.0..=1.0).contains(&value) {
                    return Err(PipelineError::new(
                        "vehicle alpha reference is not normalized",
                    ));
                }
                Some((bits, value))
            } else {
                None
            };
            let request_identity = digest_hex(
                format!(
                    "{}\0{}\0{}",
                    vehicle.evidence.package_id, slot_index, slot.slot_name
                )
                .as_bytes(),
            );
            let asset_name = format!("MI_Vehicle_{request_identity}");
            let package_path =
                format!("{VEHICLE_INSTANCE_FOLDER}/{asset_name}");
            let parent = master_object_path(recipe);
            let base_color_rgba8 = match recipe {
                MasterRecipe::SimpleLitOpaque { .. } => {
                    slot.raster.diffuse_rgba8
                },
                MasterRecipe::SimpleUnlit { .. } => slot.base_color_rgba8,
            };
            instances.push(json!({
                "request_identity": request_identity,
                "package_id": vehicle.evidence.package_id,
                "source_fbx": vehicle.evidence.path,
                "slot_index": slot_index,
                "slot_name": slot.slot_name,
                "source_material_name": slot.source_material_name,
                "recipe_identity": recipe_identity,
                "texture_sha256": texture_sha256,
                "folder_path": VEHICLE_INSTANCE_FOLDER,
                "asset_name": asset_name,
                "package_path": package_path,
                "object_path": format!("{package_path}.{asset_name}"),
                "parent_material_path": parent,
                "base_color_texture_path": texture_path,
                "base_color_tint": base_color_rgba8.map(|value| {
                    f64::from(value) / 255.0
                }),
                "set_alpha_reference": alpha_reference.is_some(),
                "alpha_reference": alpha_reference.map(|(_, value)| value),
                "alpha_reference_bits": alpha_reference.map(|(bits, _)| bits)
            }));
        }
    }

    let texture_requests = textures
        .into_values()
        .map(|request| {
            let asset_name = format!("T_Vehicle_{}", request.sha256);
            let package_path = format!("{VEHICLE_TEXTURE_FOLDER}/{asset_name}");
            json!({
                "sha256": request.sha256,
                "source_path": request.source_path,
                "bytes": request.bytes,
                "folder_path": VEHICLE_TEXTURE_FOLDER,
                "asset_name": asset_name,
                "package_path": package_path,
                "object_path": format!("{package_path}.{asset_name}")
            })
        })
        .collect();
    let mut master_requests = recipes
        .into_iter()
        .map(|(recipe, identity)| master_request(recipe, &identity))
        .collect::<Vec<_>>();
    master_requests.sort_by(|left, right| {
        left["recipe_identity"]
            .as_str()
            .cmp(&right["recipe_identity"].as_str())
    });
    instances.sort_by(|left, right| {
        left["request_identity"]
            .as_str()
            .cmp(&right["request_identity"].as_str())
    });
    Ok(VehicleMaterialNativeConstructionPlan {
        texture_requests,
        master_requests,
        instance_requests: instances,
    })
}

fn texture_object_path(sha256: &str) -> String {
    let asset_name = format!("T_Vehicle_{sha256}");
    format!("{VEHICLE_TEXTURE_FOLDER}/{asset_name}.{asset_name}")
}

fn master_request(recipe: MasterRecipe, identity: &str) -> Value {
    let asset_name = master_asset_name(recipe);
    let package_path = format!("{VEHICLE_MASTER_FOLDER}/{asset_name}");
    match recipe {
        MasterRecipe::SimpleUnlit {
            blend_mode,
            alpha_test,
            two_sided,
        } => json!({
            "recipe_identity": identity,
            "shader_family": "simple",
            "lit": false,
            "blend_mode": blend_mode,
            "alpha_test": alpha_test,
            "alpha_compare": 4,
            "two_sided": two_sided,
            "folder_path": VEHICLE_MASTER_FOLDER,
            "asset_name": asset_name,
            "package_path": package_path,
            "object_path": format!("{package_path}.{asset_name}"),
        }),
        MasterRecipe::SimpleLitOpaque {
            shininess_bits,
            two_sided,
        } => json!({
            "recipe_identity": identity,
            "shader_family": "simple",
            "lit": true,
            "blend_mode": 0,
            "alpha_test": false,
            "alpha_compare": 4,
            "two_sided": two_sided,
            "source_ambient_rgba8": [0, 0, 0, 255],
            "source_specular_rgba8": [0, 0, 0, 255],
            "source_emissive_rgba8": [0, 0, 0, 255],
            "source_shininess_bits": shininess_bits,
            "source_shininess": f32::from_bits(shininess_bits),
            "folder_path": VEHICLE_MASTER_FOLDER,
            "asset_name": asset_name,
            "package_path": package_path,
            "object_path": format!("{package_path}.{asset_name}"),
        }),
    }
}

fn master_asset_name(recipe: MasterRecipe) -> String {
    match recipe {
        MasterRecipe::SimpleUnlit {
            blend_mode,
            alpha_test,
            two_sided,
        } => {
            let blend = match blend_mode {
                0 => "Opaque",
                1 => "Alpha",
                2 => "Additive",
                _ => "Unsupported",
            };
            format!(
                "M_SHAR_Vehicle_SimpleUnlit_{blend}_AlphaTest{}_{}",
                if alpha_test { "On" } else { "Off" },
                if two_sided { "TwoSided" } else { "OneSided" }
            )
        },
        MasterRecipe::SimpleLitOpaque {
            shininess_bits,
            two_sided,
        } => format!(
            "M_SHAR_Vehicle_SimpleLit_Opaque_Shininess{shininess_bits:08X}_{}",
            if two_sided { "TwoSided" } else { "OneSided" }
        ),
    }
}

fn master_object_path(recipe: MasterRecipe) -> String {
    let asset_name = master_asset_name(recipe);
    format!("{VEHICLE_MASTER_FOLDER}/{asset_name}.{asset_name}")
}

#[cfg(test)]
// jig-ignore-next-line: repository test module path is indivisible
#[path = "../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/unreal_vehicle_material_native_plan/tests.rs"]
mod tests;

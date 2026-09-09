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
//   - Verification of generated vehicle FBX catalog evidence for Unreal plans.
// - Must-Not:
//   - Infer gameplay construction or collapse vehicle support packages.
// - Allows:
//   - Read the deterministic vehicle catalog and verify its FBX payload bytes.
// - Split-When:
//   - Split when vehicle plan promotion gains an independent lifecycle.
// - Merge-When:
//   - Merge when another adapter owns identical vehicle artifact verification.
// - Summary:
//   - Generated vehicle catalog verifier.
// - Description:
//   - Converts verified vehicle-catalog FBX rows into generic Unreal FBX
//     artifact evidence without claiming that the full semantic package is
//     ready.
// - Usage:
//   - Used by prepare-unreal before any future vehicle plan promotion.
// - Defaults:
//   - Missing roots remain absent; malformed or stale roots fail closed.
//

//! Generated vehicle FBX catalog verification.

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use serde_json::Value;
use shar_sha256::digest_hex;

use super::unreal_fbx_catalog::{
    FBX_VERSION, binary_fbx_version, io_error, validate_ancestor_chain,
    validate_digest, validate_directory_metadata, validate_public_identifier,
    validate_regular_file, validate_relative_path,
};
use crate::domain::{
    PipelineError, PipelineOutcome, UnrealFbxArtifactEvidence,
};

const CATALOG_FILE: &str = "vehicles.catalog.json";
const CATALOG_SCHEMA: &str = "shar.vehicle-catalog.v8";
const LOGICAL_ROOT: &str = "vehicle-assets";

/// Effective semantic flags for one exact vehicle material slot.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct VerifiedVehicleMaterialSemantics {
    pub transparent: bool,
    pub glass: bool,
    pub mirror: bool,
    pub reflective: bool,
    pub light_emitter: bool,
    pub visual_effect: bool,
}

/// One verified FBX material slot plus exact shader and texture evidence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct VerifiedVehicleMaterialArtifact {
    pub slot_name: String,
    pub source_material_name: String,
    pub base_color_rgba8: [u8; 4],
    pub semantics: VerifiedVehicleMaterialSemantics,
    pub shader_path: String,
    pub shader_size_bytes: u64,
    pub shader_sha256: String,
    pub texture_path: Option<String>,
    pub texture_size_bytes: Option<u64>,
    pub texture_sha256: Option<String>,
}

/// One verified source collision or physics sidecar for a vehicle.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct VerifiedVehiclePhysicsArtifact {
    pub path: String,
    pub package_member_id: String,
    pub source_path: String,
    pub kind: String,
    pub source_chunk_kind: String,
    pub source_ordinal: u64,
    pub source_identity: String,
    pub size_bytes: u64,
    pub sha256: String,
}

/// One verified bone-local source collision primitive.
#[derive(Clone, Debug, PartialEq)]
pub(super) enum VerifiedVehiclePhysicsPrimitive {
    Sphere {
        bone_name: String,
        center_m: [f32; 3],
        radius_m: f32,
    },
    OrientedBox {
        bone_name: String,
        center_m: [f32; 3],
        axes: [[f32; 3]; 3],
        half_extents_m: [f32; 3],
    },
    Cylinder {
        bone_name: String,
        center_m: [f32; 3],
        axis: [f32; 3],
        half_length_m: f32,
        radius_m: f32,
        flat_end: bool,
    },
}

/// One verified same-name source physics rig recipe.
#[derive(Clone, Debug, PartialEq)]
pub(super) struct VerifiedVehiclePhysicsRig {
    pub identity: String,
    pub joint_count: u64,
    pub primitives: Vec<VerifiedVehiclePhysicsPrimitive>,
}

/// One verified vehicle FBX plus its exact package subcategory and physics.
#[derive(Clone, Debug, PartialEq)]
pub(super) struct VerifiedVehicleFbxArtifact {
    pub evidence: UnrealFbxArtifactEvidence,
    pub subcategory: String,
    pub material_slots: Vec<VerifiedVehicleMaterialArtifact>,
    pub physics_sidecars: Vec<VerifiedVehiclePhysicsArtifact>,
    pub physics_rigs: Vec<VerifiedVehiclePhysicsRig>,
}

/// Verify the generated vehicle FBX rows when the catalog root exists.
///
/// This verifies FBX, source material artifacts, and source physics evidence.
/// Only the FBX is promoted as import evidence; native Material Instances,
/// Physics Assets, wheels, and runtime construction remain separate work.
///
/// # Errors
///
/// Returns an error for malformed, duplicated, unsafe, stale, linked, or
/// unsupported vehicle FBX evidence.
pub(super) fn verified_vehicle_fbx_catalog(
    root: &Path,
) -> PipelineOutcome<Option<Vec<VerifiedVehicleFbxArtifact>>> {
    let metadata = match fs::symlink_metadata(root) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(None);
        },
        Err(error) => {
            return Err(io_error(
                "inspect generated vehicle catalog root",
                &error,
            ));
        },
        Ok(metadata) => metadata,
    };
    validate_directory_metadata(&metadata)?;
    let catalog_path = root.join(CATALOG_FILE);
    validate_regular_file(&catalog_path, "generated vehicle catalog")?;
    let text = fs::read_to_string(&catalog_path)
        .map_err(|error| io_error("read generated vehicle catalog", &error))?;
    let root_value = serde_json::from_str::<Value>(&text).map_err(|_error| {
        PipelineError::new("generated vehicle catalog contains invalid JSON")
    })?;
    let object = root_value.as_object().ok_or_else(|| {
        PipelineError::new("generated vehicle catalog must be a JSON object")
    })?;
    if object.get("schema").and_then(Value::as_str) != Some(CATALOG_SCHEMA) {
        return Err(PipelineError::new(
            "generated vehicle catalog schema is not supported",
        ));
    }
    let vehicles = object
        .get("vehicles")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            PipelineError::new("generated vehicle catalog has no vehicle rows")
        })?;
    let declared = object
        .get("counts")
        .and_then(Value::as_object)
        .and_then(|counts| counts.get("vehicles"))
        .and_then(Value::as_u64)
        .ok_or_else(|| {
            PipelineError::new("generated vehicle catalog has no vehicle count")
        })?;
    if declared != u64::try_from(vehicles.len()).unwrap_or(u64::MAX) {
        return Err(PipelineError::new(
            "generated vehicle catalog vehicle count is stale",
        ));
    }
    let declared_material_slots = object
        .get("counts")
        .and_then(Value::as_object)
        .and_then(|counts| counts.get("material_slots"))
        .and_then(Value::as_u64)
        .ok_or_else(|| {
            PipelineError::new(
                "generated vehicle catalog has no material-slot count",
            )
        })?;
    let declared_physics = object
        .get("counts")
        .and_then(Value::as_object)
        .and_then(|counts| counts.get("physics_sidecars"))
        .and_then(Value::as_u64)
        .ok_or_else(|| {
            PipelineError::new(
                "generated vehicle catalog has no physics-sidecar count",
            )
        })?;
    let declared_rigs = object
        .get("counts")
        .and_then(Value::as_object)
        .and_then(|counts| counts.get("physics_rigs"))
        .and_then(Value::as_u64)
        .ok_or_else(|| {
            PipelineError::new(
                "generated vehicle catalog has no physics-rig count",
            )
        })?;
    let declared_primitives = object
        .get("counts")
        .and_then(Value::as_object)
        .and_then(|counts| counts.get("physics_primitives"))
        .and_then(Value::as_u64)
        .ok_or_else(|| {
            PipelineError::new(
                "generated vehicle catalog has no physics-primitive count",
            )
        })?;

    let mut package_ids = BTreeSet::new();
    let mut paths = BTreeSet::new();
    let mut result = Vec::with_capacity(vehicles.len());
    let mut verified_material_slot_count = 0_u64;
    let mut verified_physics_count = 0_u64;
    let mut verified_rig_count = 0_u64;
    let mut verified_primitive_count = 0_u64;
    for row in vehicles {
        let row = row.as_object().ok_or_else(|| {
            PipelineError::new("generated vehicle catalog row is not an object")
        })?;
        let package_id = required_string(row, "package_id")?;
        validate_public_identifier(&package_id)?;
        if !package_ids.insert(package_id.clone()) {
            return Err(PipelineError::new(
                "generated vehicle catalog contains a duplicate package",
            ));
        }
        let subcategory = required_string(row, "subcategory")?;
        validate_vehicle_subcategory(&subcategory)?;
        let vehicle = required_string(row, "vehicle")?;
        validate_vehicle_name(&vehicle)?;
        let fbx = row.get("fbx").and_then(Value::as_object).ok_or_else(|| {
            PipelineError::new(
                "generated vehicle catalog row has no FBX record",
            )
        })?;
        let relative_path = required_string(fbx, "path")?;
        validate_relative_path(&relative_path)?;
        let expected_path = format!("{vehicle}/{vehicle}.fbx");
        if relative_path != expected_path
            || !paths.insert(relative_path.clone())
        {
            return Err(PipelineError::new(
                "generated vehicle FBX path is not canonical or unique",
            ));
        }
        let size_bytes = required_u64(fbx, "bytes")?;
        let expected_sha256 = required_string(fbx, "sha256")?;
        validate_digest(&expected_sha256)?;
        let path = root.join(&relative_path);
        validate_regular_file(&path, "generated vehicle FBX")?;
        validate_ancestor_chain(root, &path)?;
        let bytes = fs::read(&path)
            .map_err(|error| io_error("read generated vehicle FBX", &error))?;
        let actual_size = u64::try_from(bytes.len()).unwrap_or(u64::MAX);
        if actual_size != size_bytes || digest_hex(&bytes) != expected_sha256 {
            return Err(PipelineError::new(
                "generated vehicle FBX bytes do not match the catalog",
            ));
        }
        let version = binary_fbx_version(&bytes)?;
        if version != FBX_VERSION {
            return Err(PipelineError::new(
                "generated vehicle FBX version is not supported",
            ));
        }
        let material_slots = verify_vehicle_material_slots(
            root,
            &vehicle,
            fbx,
            row.get("material_slots"),
        )?;
        let material_slot_count =
            u64::try_from(material_slots.len()).unwrap_or(u64::MAX);
        verified_material_slot_count = verified_material_slot_count
            .checked_add(material_slot_count)
            .ok_or_else(|| {
                PipelineError::new(
                    "generated vehicle material-slot count overflowed",
                )
            })?;
        let physics = verify_vehicle_physics_sidecars(
            root,
            &vehicle,
            row.get("physics_sidecars"),
        )?;
        let physics_rigs = verify_vehicle_physics_rigs(
            row.get("physics_rigs"),
            &physics,
        )?;
        verified_physics_count = verified_physics_count
            .checked_add(u64::try_from(physics.len()).unwrap_or(u64::MAX))
            .ok_or_else(|| {
                PipelineError::new("generated vehicle physics count overflowed")
            })?;
        verified_rig_count = verified_rig_count
            .checked_add(u64::try_from(physics_rigs.len()).unwrap_or(u64::MAX))
            .ok_or_else(|| {
                PipelineError::new(
                    "generated vehicle physics-rig count overflowed",
                )
            })?;
        verified_primitive_count = verified_primitive_count
            .checked_add(
                physics_rigs
                    .iter()
                    .map(|rig| {
                        u64::try_from(rig.primitives.len()).unwrap_or(u64::MAX)
                    })
                    .try_fold(0_u64, u64::checked_add)
                    .ok_or_else(|| {
                        PipelineError::new(
                            "vehicle physics-primitive count overflowed",
                        )
                    })?,
            )
            .ok_or_else(|| {
                PipelineError::new(
                    "generated vehicle physics-primitive count overflowed",
                )
            })?;
        result.push(VerifiedVehicleFbxArtifact {
            evidence: UnrealFbxArtifactEvidence {
                package_id,
                path: format!("{LOGICAL_ROOT}/{relative_path}"),
                size_bytes: actual_size,
                sha256: expected_sha256,
                fbx_version: version,
            },
            subcategory,
            material_slots,
            physics_sidecars: physics,
            physics_rigs,
        });
    }
    if verified_material_slot_count != declared_material_slots {
        return Err(PipelineError::new(
            "generated vehicle catalog material-slot count is stale",
        ));
    }
    if verified_physics_count != declared_physics {
        return Err(PipelineError::new(
            "generated vehicle catalog physics-sidecar count is stale",
        ));
    }
    if verified_rig_count != declared_rigs {
        return Err(PipelineError::new(
            "generated vehicle catalog physics-rig count is stale",
        ));
    }
    if verified_primitive_count != declared_primitives {
        return Err(PipelineError::new(
            "generated vehicle catalog physics-primitive count is stale",
        ));
    }
    result.sort_by(|left, right| {
        left.evidence.package_id.cmp(&right.evidence.package_id)
    });
    Ok(Some(result))
}

fn verify_vehicle_material_slots(
    root: &Path,
    vehicle: &str,
    fbx: &serde_json::Map<String, Value>,
    value: Option<&Value>,
) -> PipelineOutcome<Vec<VerifiedVehicleMaterialArtifact>> {
    let slots = value.and_then(Value::as_array).ok_or_else(|| {
        PipelineError::new(
            "generated vehicle catalog row has no material slots",
        )
    })?;
    let declared = required_u64(fbx, "materials")?;
    if declared != u64::try_from(slots.len()).unwrap_or(u64::MAX) {
        return Err(PipelineError::new(
            "generated vehicle material slots disagree with FBX summary",
        ));
    }
    let mut slot_names = BTreeSet::new();
    let mut result = Vec::with_capacity(slots.len());
    for slot in slots {
        let slot = slot.as_object().ok_or_else(|| {
            PipelineError::new(
                "generated vehicle material slot is not an object",
            )
        })?;
        let slot_name = required_string(slot, "slot_name")?;
        let source_material_name =
            required_string(slot, "source_material_name")?;
        validate_source_identity(&slot_name)?;
        validate_source_identity(&source_material_name)?;
        if !slot_names.insert(slot_name.clone()) {
            return Err(PipelineError::new(
                "generated vehicle material slot identity is duplicated",
            ));
        }
        let base_color_rgba8 = required_rgba8(slot, "base_color_rgba8")?;
        let semantics = required_material_semantics(slot)?;
        let shader = slot
            .get("shader")
            .and_then(Value::as_object)
            .ok_or_else(|| {
                PipelineError::new(
                    "generated vehicle material slot has no shader evidence",
                )
            })?;
        let (shader_path, shader_size_bytes, shader_sha256) =
            verify_material_artifact(
                root,
                vehicle,
                shader,
                "shaders/",
                ".json",
            )?;
        let shader_bytes = fs::read(root.join(vehicle).join(&shader_path))
            .map_err(|error| {
                io_error("read generated vehicle material shader", &error)
            })?;
        let shader_document = serde_json::from_slice::<Value>(&shader_bytes)
            .map_err(|_error| {
                PipelineError::new(
                    "generated vehicle material shader contains invalid JSON",
                )
            })?;
        if shader_document.get("schema").and_then(Value::as_str)
            != Some("shader")
        {
            return Err(PipelineError::new(
                "generated vehicle material shader schema is inconsistent",
            ));
        }
        let shader_identity = shader_document
            .get("name")
            .and_then(Value::as_str)
            .map(|name| name.trim_end_matches('\0'));
        if shader_identity != Some(source_material_name.as_str()) {
            return Err(PipelineError::new(
                "generated vehicle material shader identity is inconsistent",
            ));
        }
        let texture = match slot.get("texture") {
            None | Some(Value::Null) => None,
            Some(value) => {
                let texture = value.as_object().ok_or_else(|| {
                    PipelineError::new(
                        "generated vehicle material texture is not an object",
                    )
                })?;
                Some(verify_material_artifact(
                    root, vehicle, texture, "textures/", ".png",
                )?)
            },
        };
        result.push(VerifiedVehicleMaterialArtifact {
            slot_name,
            source_material_name,
            base_color_rgba8,
            semantics,
            shader_path,
            shader_size_bytes,
            shader_sha256,
            texture_path: texture.as_ref().map(|item| item.0.clone()),
            texture_size_bytes: texture.as_ref().map(|item| item.1),
            texture_sha256: texture.map(|item| item.2),
        });
    }
    Ok(result)
}

fn verify_material_artifact(
    root: &Path,
    vehicle: &str,
    record: &serde_json::Map<String, Value>,
    prefix: &str,
    suffix: &str,
) -> PipelineOutcome<(String, u64, String)> {
    let path = required_string(record, "path")?;
    validate_relative_path(&path)?;
    if !path.starts_with(prefix) || !path.ends_with(suffix) {
        return Err(PipelineError::new(
            "generated vehicle material artifact path is not canonical",
        ));
    }
    let size_bytes = required_u64(record, "bytes")?;
    let sha256 = required_string(record, "sha256")?;
    validate_digest(&sha256)?;
    let full_path = root.join(vehicle).join(&path);
    validate_regular_file(&full_path, "generated vehicle material artifact")?;
    validate_ancestor_chain(root, &full_path)?;
    let bytes = fs::read(&full_path).map_err(|error| {
        io_error("read generated vehicle material artifact", &error)
    })?;
    if u64::try_from(bytes.len()).unwrap_or(u64::MAX) != size_bytes
        || digest_hex(&bytes) != sha256
    {
        return Err(PipelineError::new(
            concat!(
                "generated vehicle material artifact bytes do not match ",
                "the catalog"
            ),
        ));
    }
    Ok((path, size_bytes, sha256))
}

fn required_rgba8(
    object: &serde_json::Map<String, Value>,
    field: &str,
) -> PipelineOutcome<[u8; 4]> {
    let values = object.get(field).and_then(Value::as_array).ok_or_else(|| {
        PipelineError::new("generated vehicle material base color is invalid")
    })?;
    let [r, g, b, a] = values.as_slice() else {
        return Err(PipelineError::new(
            "generated vehicle material base color component count is invalid",
        ));
    };
    let component = |value: &Value| {
        value
            .as_u64()
            .and_then(|value| u8::try_from(value).ok())
            .ok_or_else(|| {
                PipelineError::new(concat!(
                    "generated vehicle material base color component ",
                    "is invalid"
                ))
            })
    };
    Ok([component(r)?, component(g)?, component(b)?, component(a)?])
}

fn required_material_semantics(
    slot: &serde_json::Map<String, Value>,
) -> PipelineOutcome<VerifiedVehicleMaterialSemantics> {
    let semantics = slot
        .get("surface_semantics")
        .and_then(Value::as_object)
        .ok_or_else(|| {
            PipelineError::new(
                "generated vehicle material slot has no surface semantics",
            )
        })?;
    let flag = |field: &str| {
        semantics.get(field).and_then(Value::as_bool).ok_or_else(|| {
            PipelineError::new(format!(
                "generated vehicle material semantic flag is invalid: {field}"
            ))
        })
    };
    Ok(VerifiedVehicleMaterialSemantics {
        transparent: flag("transparent")?,
        glass: flag("glass")?,
        mirror: flag("mirror")?,
        reflective: flag("reflective")?,
        light_emitter: flag("light_emitter")?,
        visual_effect: flag("visual_effect")?,
    })
}

fn verify_vehicle_physics_sidecars(
    root: &Path,
    vehicle: &str,
    value: Option<&Value>,
) -> PipelineOutcome<Vec<VerifiedVehiclePhysicsArtifact>> {
    let sidecars = value.and_then(Value::as_array).ok_or_else(|| {
        PipelineError::new(
            "generated vehicle catalog row has no physics sidecars",
        )
    })?;
    if sidecars.is_empty() {
        return Err(PipelineError::new(
            "generated vehicle catalog row has empty physics sidecars",
        ));
    }
    let mut member_ids = BTreeSet::new();
    let mut paths = BTreeSet::new();
    let mut ordinals = BTreeSet::new();
    let mut source_kinds = BTreeSet::new();
    let mut result = Vec::with_capacity(sidecars.len());
    for sidecar in sidecars {
        let sidecar = sidecar.as_object().ok_or_else(|| {
            PipelineError::new(
                "generated vehicle physics sidecar is not an object",
            )
        })?;
        let path = required_string(sidecar, "path")?;
        validate_relative_path(&path)?;
        let package_member_id = required_string(sidecar, "package_member_id")?;
        validate_public_identifier(&package_member_id)?;
        let source_path = required_string(sidecar, "source_path")?;
        validate_relative_path(&source_path)?;
        let kind = required_string(sidecar, "kind")?;
        let source_chunk_kind = required_string(sidecar, "source_chunk_kind")?;
        let source_ordinal = required_u64(sidecar, "source_ordinal")?;
        let size_bytes = required_u64(sidecar, "bytes")?;
        let sha256 = required_string(sidecar, "sha256")?;
        validate_digest(&sha256)?;
        let (family, expected_kind) = match source_chunk_kind.as_str() {
            "simulation_collision_object" => ("collision", "p3d-collision"),
            "simulation_physics_object" => ("physics", "p3d-physics"),
            _ => {
                return Err(PipelineError::new(
                    "generated vehicle physics sidecar kind is unsupported",
                ));
            },
        };
        let expected_path =
            format!("physics/{family}__ordinal_{source_ordinal:06}.json");
        if path != expected_path || kind != expected_kind {
            return Err(PipelineError::new(
                "generated vehicle physics sidecar identity is not canonical",
            ));
        }
        if !member_ids.insert(package_member_id.clone())
            || !paths.insert(path.clone())
            || !ordinals.insert(source_ordinal)
        {
            return Err(PipelineError::new(
                "generated vehicle physics sidecar is duplicated",
            ));
        }
        let _source_kind_was_new =
            source_kinds.insert(source_chunk_kind.clone());
        let full_path = root.join(vehicle).join(&path);
        validate_regular_file(&full_path, "generated vehicle physics sidecar")?;
        validate_ancestor_chain(root, &full_path)?;
        let bytes = fs::read(&full_path).map_err(|error| {
            io_error("read generated vehicle physics sidecar", &error)
        })?;
        let actual_size = u64::try_from(bytes.len()).unwrap_or(u64::MAX);
        if actual_size != size_bytes || digest_hex(&bytes) != sha256 {
            return Err(PipelineError::new(
                "generated vehicle physics bytes do not match the catalog",
            ));
        }
        let document = serde_json::from_slice::<Value>(&bytes)
            .map_err(|_error| {
                PipelineError::new(
                    "generated vehicle physics sidecar contains invalid JSON",
                )
            })?;
        if document.get("schema").and_then(Value::as_str)
            != Some(source_chunk_kind.as_str())
        {
            return Err(PipelineError::new(
                "generated vehicle physics sidecar schema is inconsistent",
            ));
        }
        let source_identity = document
            .get("name")
            .and_then(Value::as_str)
            .map(|value| value.trim_end_matches('\0'))
            .ok_or_else(|| {
                PipelineError::new(
                    "generated vehicle physics sidecar has no source identity",
                )
            })?;
        validate_source_identity(source_identity)?;
        result.push(VerifiedVehiclePhysicsArtifact {
            path: format!("{LOGICAL_ROOT}/{vehicle}/{path}"),
            package_member_id,
            source_path,
            kind,
            source_chunk_kind,
            source_ordinal,
            source_identity: source_identity.to_owned(),
            size_bytes,
            sha256,
        });
    }
    if !source_kinds.contains("simulation_collision_object")
        || !source_kinds.contains("simulation_physics_object")
    {
        return Err(PipelineError::new(
            "generated vehicle physics sidecars are incomplete",
        ));
    }
    Ok(result)
}

fn verify_vehicle_physics_rigs(
    value: Option<&Value>,
    sidecars: &[VerifiedVehiclePhysicsArtifact],
) -> PipelineOutcome<Vec<VerifiedVehiclePhysicsRig>> {
    let rigs = value.and_then(Value::as_array).ok_or_else(|| {
        PipelineError::new("generated vehicle catalog row has no physics rigs")
    })?;
    if rigs.is_empty() {
        return Err(PipelineError::new(
            "generated vehicle catalog row has empty physics rigs",
        ));
    }
    let collision_identities = sidecars
        .iter()
        .filter(|sidecar| {
            sidecar.source_chunk_kind == "simulation_collision_object"
        })
        .map(|sidecar| sidecar.source_identity.clone())
        .collect::<BTreeSet<_>>();
    let physics_identities = sidecars
        .iter()
        .filter(|sidecar| {
            sidecar.source_chunk_kind == "simulation_physics_object"
        })
        .map(|sidecar| sidecar.source_identity.clone())
        .collect::<BTreeSet<_>>();
    if collision_identities != physics_identities {
        return Err(PipelineError::new(
            "generated vehicle physics sidecar rig identities disagree",
        ));
    }
    let mut identities = BTreeSet::new();
    let mut result = Vec::with_capacity(rigs.len());
    for rig in rigs {
        let rig = rig.as_object().ok_or_else(|| {
            PipelineError::new("generated vehicle physics rig is not an object")
        })?;
        let identity = required_string(rig, "identity")?;
        validate_source_identity(&identity)?;
        if !identities.insert(identity.clone()) {
            return Err(PipelineError::new(
                "generated vehicle physics rig identity is duplicated",
            ));
        }
        if rig.get("coordinate_space").and_then(Value::as_str)
            != Some("source-bone-local")
            || rig.get("unit").and_then(Value::as_str) != Some("meter")
        {
            return Err(PipelineError::new(
                "generated vehicle physics rig space or unit is unsupported",
            ));
        }
        let joint_count = required_u64(rig, "joint_count")?;
        if joint_count == 0 {
            return Err(PipelineError::new(
                "generated vehicle physics rig joint count is invalid",
            ));
        }
        let primitives = rig
            .get("primitives")
            .and_then(Value::as_array)
            .ok_or_else(|| {
                PipelineError::new(
                    "generated vehicle physics rig has no primitives",
                )
            })?;
        if primitives.is_empty() {
            return Err(PipelineError::new(
                "generated vehicle physics rig has empty primitives",
            ));
        }
        let primitives = primitives
            .iter()
            .map(verify_vehicle_physics_primitive)
            .collect::<PipelineOutcome<Vec<_>>>()?;
        result.push(VerifiedVehiclePhysicsRig {
            identity,
            joint_count,
            primitives,
        });
    }
    if identities != collision_identities {
        return Err(PipelineError::new(
            "generated vehicle physics recipes do not match source rigs",
        ));
    }
    Ok(result)
}

fn verify_vehicle_physics_primitive(
    value: &Value,
) -> PipelineOutcome<VerifiedVehiclePhysicsPrimitive> {
    let primitive = value.as_object().ok_or_else(|| {
        PipelineError::new(
            "generated vehicle physics primitive is not an object",
        )
    })?;
    let bone_name = required_string(primitive, "bone_name")?;
    validate_source_identity(&bone_name)?;
    let center_m = required_vec3(primitive, "center_m")?;
    match primitive.get("kind").and_then(Value::as_str) {
        Some("sphere") => Ok(VerifiedVehiclePhysicsPrimitive::Sphere {
            bone_name,
            center_m,
            radius_m: required_positive_f32(primitive, "radius_m")?,
        }),
        Some("oriented-box") => {
            let axes_value = primitive
                .get("axes")
                .and_then(Value::as_array)
                .ok_or_else(|| {
                    PipelineError::new(
                        "generated vehicle box primitive has no axes",
                    )
                })?;
            let [axis0, axis1, axis2] = axes_value.as_slice() else {
                return Err(PipelineError::new(
                    "generated vehicle box primitive axis count is invalid",
                ));
            };
            let axes = [
                value_vec3(axis0)?,
                value_vec3(axis1)?,
                value_vec3(axis2)?,
            ];
            validate_verified_basis(&axes)?;
            Ok(VerifiedVehiclePhysicsPrimitive::OrientedBox {
                bone_name,
                center_m,
                axes,
                half_extents_m: required_positive_vec3(
                    primitive,
                    "half_extents_m",
                )?,
            })
        },
        Some("cylinder") => {
            let axis = required_vec3(primitive, "axis")?;
            validate_verified_unit_axis(&axis)?;
            let flat_end = primitive
                .get("flat_end")
                .and_then(Value::as_bool)
                .ok_or_else(|| {
                    PipelineError::new(
                        "generated vehicle cylinder flat-end flag is invalid",
                    )
                })?;
            Ok(VerifiedVehiclePhysicsPrimitive::Cylinder {
                bone_name,
                center_m,
                axis,
                half_length_m: required_positive_f32(
                    primitive,
                    "half_length_m",
                )?,
                radius_m: required_positive_f32(primitive, "radius_m")?,
                flat_end,
            })
        },
        _ => Err(PipelineError::new(
            "generated vehicle physics primitive kind is unsupported",
        )),
    }
}

fn required_vec3(
    object: &serde_json::Map<String, Value>,
    field: &str,
) -> PipelineOutcome<[f32; 3]> {
    object.get(field).ok_or_else(|| {
        PipelineError::new("generated vehicle physics vector is missing")
    }).and_then(value_vec3)
}

fn required_positive_vec3(
    object: &serde_json::Map<String, Value>,
    field: &str,
) -> PipelineOutcome<[f32; 3]> {
    let values = required_vec3(object, field)?;
    if values.iter().any(|value| *value <= 0.0) {
        return Err(PipelineError::new(
            "generated vehicle physics dimensions are not positive",
        ));
    }
    Ok(values)
}

fn value_vec3(value: &Value) -> PipelineOutcome<[f32; 3]> {
    let values = value.as_array().ok_or_else(|| {
        PipelineError::new("generated vehicle physics vector is invalid")
    })?;
    let [x, y, z] = values.as_slice() else {
        return Err(PipelineError::new(
            "generated vehicle physics vector component count is invalid",
        ));
    };
    Ok([
        finite_value(x)?,
        finite_value(y)?,
        finite_value(z)?,
    ])
}

fn finite_value(value: &Value) -> PipelineOutcome<f32> {
    value
        .as_number()
        .and_then(|number| number.to_string().parse::<f32>().ok())
        .filter(|number| number.is_finite())
        .ok_or_else(|| {
            PipelineError::new(
                "generated vehicle physics number is not finite",
            )
        })
}

fn required_positive_f32(
    object: &serde_json::Map<String, Value>,
    field: &str,
) -> PipelineOutcome<f32> {
    let value = object.get(field).ok_or_else(|| {
        PipelineError::new("generated vehicle physics dimension is missing")
    }).and_then(finite_value)?;
    if value <= 0.0 {
        return Err(PipelineError::new(
            "generated vehicle physics dimension is not positive",
        ));
    }
    Ok(value)
}

fn validate_verified_unit_axis(axis: &[f32; 3]) -> PipelineOutcome<()> {
    const TOLERANCE: f32 = 1.0e-5;
    let squared = axis.iter().map(|value| value * value).sum::<f32>();
    if (squared - 1.0).abs() > TOLERANCE {
        return Err(PipelineError::new(
            "generated vehicle physics axis is not unit length",
        ));
    }
    Ok(())
}

fn validate_verified_basis(axes: &[[f32; 3]; 3]) -> PipelineOutcome<()> {
    const TOLERANCE: f32 = 1.0e-5;
    for axis in axes {
        validate_verified_unit_axis(axis)?;
    }
    let dot = |left: &[f32; 3], right: &[f32; 3]| {
        left.iter()
            .zip(right)
            .map(|(left, right)| left * right)
            .sum::<f32>()
    };
    if dot(&axes[0], &axes[1]).abs() > TOLERANCE
        || dot(&axes[0], &axes[2]).abs() > TOLERANCE
        || dot(&axes[1], &axes[2]).abs() > TOLERANCE
    {
        return Err(PipelineError::new(
            "generated vehicle physics box axes are not orthogonal",
        ));
    }
    let cross = [
        axes[0][1] * axes[1][2] - axes[0][2] * axes[1][1],
        axes[0][2] * axes[1][0] - axes[0][0] * axes[1][2],
        axes[0][0] * axes[1][1] - axes[0][1] * axes[1][0],
    ];
    if dot(&cross, &axes[2]) <= 0.0 {
        return Err(PipelineError::new(
            "generated vehicle physics box basis is reflected",
        ));
    }
    Ok(())
}

fn validate_source_identity(value: &str) -> PipelineOutcome<()> {
    if value.is_empty()
        || value != value.trim()
        || value.chars().any(char::is_control)
    {
        return Err(PipelineError::new(
            "generated vehicle source identity is invalid",
        ));
    }
    Ok(())
}

fn required_string(
    object: &serde_json::Map<String, Value>,
    field: &str,
) -> PipelineOutcome<String> {
    object
        .get(field)
        .and_then(Value::as_str)
        .map(ToOwned::to_owned)
        .ok_or_else(|| {
            PipelineError::new(format!(
                "generated vehicle catalog is missing string field {field}"
            ))
        })
}

fn required_u64(
    object: &serde_json::Map<String, Value>,
    field: &str,
) -> PipelineOutcome<u64> {
    object.get(field).and_then(Value::as_u64).ok_or_else(|| {
        PipelineError::new(format!(
            "generated vehicle catalog is missing unsigned field {field}"
        ))
    })
}

fn validate_vehicle_subcategory(value: &str) -> PipelineOutcome<()> {
    if !value.starts_with("cars/")
        || value.contains(char::from(92))
        || value.contains(':')
        || value.chars().any(char::is_control)
        || value
            .split('/')
            .any(|part| part.is_empty() || part == "." || part == "..")
    {
        return Err(PipelineError::new(
            "generated vehicle subcategory is not canonical",
        ));
    }
    Ok(())
}

fn validate_vehicle_name(value: &str) -> PipelineOutcome<()> {
    let bytes = value.as_bytes();
    if bytes.is_empty()
        || !bytes.first().is_some_and(u8::is_ascii_alphanumeric)
        || !bytes.last().is_some_and(u8::is_ascii_alphanumeric)
        || !bytes.iter().copied().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-'
        })
    {
        return Err(PipelineError::new(
            "generated vehicle identity is not canonical",
        ));
    }
    Ok(())
}

#[cfg(test)]
// jig-ignore-next-line: repository test module path is indivisible
#[path = "../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/unreal_vehicle_catalog/tests.rs"]
mod tests;

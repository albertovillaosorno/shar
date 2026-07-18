// File:
//   - prepare.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/vehicle_catalog/prepare.rs
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
//   - Vehicle assembly, semantic part separation, materials, and FBX writing.
// - Must-Not:
//   - Select the catalog package set or publish the root catalog.
// - Allows:
//   - Rigid skeleton binding, animation loading, textures, and shader sidecars.
// - Summary:
//   - Prepares and publishes one vehicle artifact.
//
// Large file:
//   - true
//   - Reason: one vehicle transaction keeps pivots, parts, materials, animation,
//     textures, and verification atomic.
//

//! Preparation and publication of one semantically separated vehicle.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use fbx::adapters::driven::binary_character_writer::write_binary_character_fbx;
use fbx::adapters::driven::decoded_animation_source::load_animation_clips;
use fbx::adapters::driven::decoded_component_source::{
    DecodedComponentError, DecodedComponentSource,
};
use fbx::adapters::driven::decoded_rigid_prop_source::load_instanced_rigid_prop_asset;
use fbx::domain::animation::AnimationClip;
use fbx::domain::character::{CharacterAsset, SkinnedPart};
use fbx::domain::mesh::MeshAsset;
use fbx::domain::texture::MaterialBinding;
use fbx::ports::component_source::ComponentSource as _;
use serde_json::Value;
use shar_sha256::digest_hex;

use super::catalog::{recursive_files, write_new};
use super::model::{PartRecord, TextureRecord, VehicleRecord};
use super::source::{
    VehicleTextureAuthority, decoded_name, png_files, relative_art_root,
    select_vehicle_composite, select_vehicle_skeleton, vehicle_mesh_paths,
};
use crate::domain::PipelineError;
use crate::domain::package::PhaseThreePackageRow;

/// Export one vehicle while preserving every authored render component.
pub(super) fn export_vehicle(
    package: &PhaseThreePackageRow,
    normalized_root: &Path,
    staging: &Path,
    authority: &VehicleTextureAuthority,
) -> Result<VehicleRecord, PipelineError> {
    let relative = relative_art_root(package)?;
    let package_root = normalized_root.join(&relative);
    let vehicle = vehicle_identity(&package.subcategory)?;
    let vehicle_dir = staging.join(&vehicle);
    let texture_dir = vehicle_dir.join("textures");
    let shader_dir = vehicle_dir.join("shaders");
    fs::create_dir_all(&texture_dir).map_err(
        |error| PipelineError::new(format!("vehicle texture output failed: {error}")),
    )?;
    fs::create_dir_all(&shader_dir).map_err(
        |error| PipelineError::new(format!("vehicle shader output failed: {error}")),
    )?;
    let skeleton = select_vehicle_skeleton(&package_root, &vehicle)?;
    let composite = select_vehicle_composite(&package_root, &vehicle)?;
    let mesh_paths = vehicle_mesh_paths(package, &package_root)?;
    let (mesh_paths, deferred_geometry) = partition_vehicle_meshes(
        &mesh_paths,
        &vehicle_dir,
    )?;
    let mesh_refs = mesh_paths.iter().map(PathBuf::as_path).collect::<Vec<_>>();
    let asset = load_instanced_rigid_prop_asset(
        &vehicle,
        &skeleton,
        &mesh_refs,
        &composite,
    )
    .map_err(
        |error| {
            PipelineError::new(format!(
                "vehicle rigid assembly failed for {}: {error:?}",
                package.package_id
            ))
        },
    )?;
    let (mut separated, parts) = separate_vehicle_parts(asset)?;
    let (materials, shaders) = resolve_vehicle_materials(
        package,
        &package_root,
        &texture_dir,
        &shader_dir,
        authority,
        &mut separated,
    )?;
    let (animations, effect_animation_sidecars) = load_vehicle_animations(
        package,
        &package_root,
        &vehicle_dir,
        &separated,
    )?;
    let fbx_path = vehicle_dir.join(format!("{vehicle}.fbx"));
    let summary = write_binary_character_fbx(
        &separated,
        &materials,
        &animations,
        &fbx_path,
    )
    .map_err(
        |error| {
            PipelineError::new(format!(
                "vehicle FBX serialization failed for {}: {error:?}",
                package.package_id
            ))
        },
    )?;
    verify_binary_fbx(&fbx_path)?;
    publish_unreferenced_textures(&package_root, &texture_dir, &materials)?;
    let textures = texture_records(&vehicle_dir)?;
    let fbx_payload = fs::read(&fbx_path)
        .map_err(|error| PipelineError::new(error.to_string()))?;
    let record = VehicleRecord {
        vehicle: vehicle.clone(),
        package_id: package.package_id.clone(),
        subcategory: package.subcategory.clone(),
        fbx_path: format!("{vehicle}/{vehicle}.fbx"),
        fbx_bytes: u64::try_from(fbx_payload.len())
            .map_err(|_| PipelineError::new("vehicle FBX size overflowed"))?,
        fbx_sha256: digest_hex(&fbx_payload),
        summary,
        parts,
        deferred_geometry,
        animations: animations.iter().map(|clip| clip.name.clone()).collect(),
        effect_animation_sidecars,
        textures,
        shaders,
    };
    super::catalog::write_vehicle_catalog(&vehicle_dir, &record)?;
    Ok(record)
}

/// Resolve one readable vehicle identity from the generated subcategory.
fn vehicle_identity(subcategory: &str) -> Result<String, PipelineError> {
    let value = subcategory
        .rsplit('/')
        .next()
        .map(portable_name)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| PipelineError::new("vehicle subcategory has no identity"))?;
    Ok(value)
}

/// Separate fully invalid geometry from meshes that can be exported safely.
fn partition_vehicle_meshes(
    paths: &[PathBuf],
    vehicle_dir: &Path,
) -> Result<(Vec<PathBuf>, Vec<String>), PipelineError> {
    let mut retained = Vec::new();
    let mut deferred = Vec::new();
    for path in paths {
        let bytes = fs::read(path)
            .map_err(|error| PipelineError::new(error.to_string()))?;
        let value: Value = serde_json::from_slice(&bytes)
            .map_err(|error| PipelineError::new(error.to_string()))?;
        let mut positions = 0_usize;
        let mut invalid = 0_usize;
        for group in value
            .get("prim_groups")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
        {
            for position in group
                .get("positions")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
            {
                positions = positions.saturating_add(1);
                let valid = position
                    .as_array()
                    .is_some_and(
                        |components| {
                            components.len() == 3
                                && components.iter().all(
                                    |component| component.as_f64().is_some(),
                                )
                        },
                    );
                invalid = invalid.saturating_add(usize::from(!valid));
            }
        }
        if positions == 0 {
            return Err(PipelineError::new(format!(
                "vehicle mesh has no position evidence: {}",
                path.display()
            )));
        }
        if invalid == 0 {
            retained.push(path.clone());
            continue;
        }
        if invalid != positions {
            return Err(PipelineError::new(format!(
                "vehicle mesh has partially invalid positions: {} of {} in {}",
                invalid,
                positions,
                path.display()
            )));
        }
        let output_dir = vehicle_dir.join("geometry").join("deferred");
        fs::create_dir_all(&output_dir)
            .map_err(|error| PipelineError::new(error.to_string()))?;
        let name = portable_name(&decoded_name(path)?);
        let file_name = format!("{name}.json");
        let payload = serde_json::to_vec_pretty(&value)
            .map_err(|error| PipelineError::new(error.to_string()))?;
        write_new(&output_dir.join(&file_name), &payload)?;
        deferred.push(format!("geometry/deferred/{file_name}"));
    }
    if retained.is_empty() {
        return Err(PipelineError::new(
            "vehicle has no recoverable render geometry",
        ));
    }
    deferred.sort();
    Ok((retained, deferred))
}

/// Split every primitive group into a named semantic vehicle object.
fn separate_vehicle_parts(
    asset: CharacterAsset,
) -> Result<(CharacterAsset, Vec<PartRecord>), PipelineError> {
    let mut parts = Vec::new();
    let mut records = Vec::new();
    let mut used_names = BTreeMap::<String, usize>::new();
    for part in asset.parts {
        for (group, influences) in part
            .mesh
            .groups
            .into_iter()
            .zip(part.group_influences)
        {
            let role = vehicle_part_role(&part.mesh.name, &group.shader);
            let base = format!("{}__{role}", portable_name(&part.mesh.name));
            let ordinal = used_names.entry(base.clone()).or_insert(0);
            let name = if *ordinal == 0 {
                base.clone()
            } else {
                format!("{base}__{:02}", *ordinal)
            };
            *ordinal = ordinal.saturating_add(1);
            let bones = influences
                .iter()
                .map(|influence| influence.bone_id.clone())
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>();
            let shader = group.shader.clone();
            let source_mesh = part.mesh.name.clone();
            let mesh = MeshAsset::new(&name, vec![group]).map_err(
                |error| {
                    PipelineError::new(format!(
                        "vehicle semantic mesh {name} failed: {error:?}"
                    ))
                },
            )?;
            parts.push(SkinnedPart {
                mesh,
                group_influences: vec![influences],
            });
            records.push(PartRecord {
                name,
                source_mesh,
                role,
                shader,
                bones,
            });
        }
    }
    let separated = CharacterAsset::new(asset.name, asset.bones, parts).map_err(
        |error| PipelineError::new(format!("vehicle semantic asset failed: {error:?}")),
    )?;
    records.sort_by(|left, right| left.name.cmp(&right.name));
    Ok((separated, records))
}

/// Classify one vehicle geometry group without inventing unsupported parts.
fn vehicle_part_role(mesh_name: &str, shader_name: &str) -> &'static str {
    let mesh = mesh_name
        .split("__joint_")
        .next()
        .unwrap_or(mesh_name)
        .to_ascii_lowercase();
    let shader = shader_name.to_ascii_lowercase();
    if mesh.contains("trunk") {
        "trunk"
    } else if mesh.contains("hood") {
        "hood"
    } else if mesh.contains("doord") {
        "driver-door"
    } else if mesh.contains("doorp") {
        "passenger-door"
    } else if mesh.contains("door") {
        "door"
    } else if is_wheel_identity(&mesh) {
        "wheel"
    } else if shader.contains("windsh") || shader.contains("glass") {
        "glass"
    } else if shader.contains("light")
        || shader.contains("flare")
        || shader.contains("siren")
        || shader.contains("glow")
    {
        "light"
    } else if mesh.contains("driver") || shader.contains("char_swatches") {
        "driver"
    } else if mesh.contains("radar")
        || mesh.contains("dish")
        || mesh.contains("mic")
        || mesh.contains("cam")
        || mesh.contains("extra")
    {
        "accessory"
    } else if shader.contains("int") || shader.contains("engine") {
        "interior"
    } else {
        "body"
    }
}

/// Return whether one authored mesh identity represents a wheel object.
fn is_wheel_identity(mesh: &str) -> bool {
    mesh.contains("wheel")
        || mesh.starts_with("wshape")
        || matches!(
            mesh.strip_suffix("shape"),
            Some("w0" | "w1" | "w2" | "w3")
        )
}

/// Resolve used shaders, preserve authored identities, and publish JSON sidecars.
fn resolve_vehicle_materials(
    package: &PhaseThreePackageRow,
    package_root: &Path,
    texture_dir: &Path,
    shader_dir: &Path,
    authority: &VehicleTextureAuthority,
    asset: &mut CharacterAsset,
) -> Result<(Vec<MaterialBinding>, Vec<String>), PipelineError> {
    let shader_names = asset
        .parts
        .iter()
        .flat_map(|part| part.mesh.groups.iter())
        .map(|group| group.shader.clone())
        .collect::<BTreeSet<_>>();
    let source = DecodedComponentSource::new(package_root, texture_dir);
    let mut by_source = BTreeMap::new();
    let mut by_material = BTreeMap::<String, MaterialBinding>::new();
    for shader in shader_names {
        let binding = match source.resolve_material(&shader) {
            Ok(binding) => binding,
            Err(DecodedComponentError::MissingTexture { texture, .. }) => {
                let external = authority
                    .resolve(&texture, &package.subcategory)?
                    .ok_or_else(
                        || {
                            PipelineError::new(format!(
                                "vehicle shader {shader} has no texture authority for {texture}"
                            ))
                        },
                    )?;
                source
                    .resolve_material_with_external_texture(&shader, external)
                    .map_err(
                        |error| {
                            PipelineError::new(format!(
                                "vehicle shared texture failed for {shader}: {error:?}"
                            ))
                        },
                    )?
            }
            Err(error) => {
                return Err(PipelineError::new(format!(
                    "vehicle material {shader} failed: {error:?}"
                )))
            }
        };
        let material_name = binding.material_name.clone();
        if let Some(previous) = by_material.insert(material_name.clone(), binding.clone())
            && previous != binding
        {
            return Err(PipelineError::new(format!(
                "vehicle material identity conflicts: {material_name}"
            )));
        }
        by_source.insert(shader.clone(), material_name);
        publish_shader_document(package_root, shader_dir, &shader)?;
    }
    for group in asset
        .parts
        .iter_mut()
        .flat_map(|part| part.mesh.groups.iter_mut())
    {
        group.shader = by_source
            .get(&group.shader)
            .ok_or_else(|| PipelineError::new("vehicle material rename is missing"))?
            .clone();
    }
    let shaders = by_source
        .into_values()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    Ok((by_material.into_values().collect(), shaders))
}

/// Export skeletal clips and preserve texture/effect animations as sidecars.
fn load_vehicle_animations(
    package: &PhaseThreePackageRow,
    package_root: &Path,
    vehicle_dir: &Path,
    asset: &CharacterAsset,
) -> Result<(Vec<AnimationClip>, Vec<String>), PipelineError> {
    let mut paths = package
        .members()
        .iter()
        .filter(
            |member| {
                member.kind == "p3d-animation"
                    && member.source_chunk_kind == "animation"
            },
        )
        .map(
            |member| {
                let file_name = Path::new(&member.path)
                    .file_name()
                    .ok_or_else(
                        || PipelineError::new("vehicle animation has no file name"),
                    )?;
                Ok(package_root
                    .join("components")
                    .join("animation")
                    .join(file_name))
            },
        )
        .collect::<Result<Vec<_>, PipelineError>>()?;
    paths.sort();
    paths.dedup();
    let mut skeletal_paths = Vec::new();
    let mut sidecars = Vec::new();
    let mut used_names = BTreeMap::<String, usize>::new();
    for path in paths {
        let bytes = fs::read(&path)
            .map_err(|error| PipelineError::new(error.to_string()))?;
        let value: Value = serde_json::from_slice(&bytes)
            .map_err(|error| PipelineError::new(error.to_string()))?;
        let name = value
            .get("name")
            .and_then(Value::as_str)
            .unwrap_or("animation")
            .trim_end_matches(' ')
            .trim();
        let kind = value
            .get("type")
            .and_then(Value::as_str)
            .unwrap_or_default();
        if kind.eq_ignore_ascii_case("PTRN") || name.starts_with("PTRN_") {
            skeletal_paths.push(path);
            continue;
        }
        let output_dir = vehicle_dir.join("animations").join("effects");
        fs::create_dir_all(&output_dir)
            .map_err(|error| PipelineError::new(error.to_string()))?;
        let base = portable_name(name);
        let ordinal = used_names.entry(base.clone()).or_insert(0);
        let file_name = if *ordinal == 0 {
            format!("{base}.json")
        } else {
            format!("{base}__{:02}.json", *ordinal)
        };
        *ordinal = ordinal.saturating_add(1);
        let payload = serde_json::to_vec_pretty(&value)
            .map_err(|error| PipelineError::new(error.to_string()))?;
        write_new(&output_dir.join(&file_name), &payload)?;
        sidecars.push(format!("animations/effects/{file_name}"));
    }
    sidecars.sort();
    if skeletal_paths.is_empty() {
        return Ok((Vec::new(), sidecars));
    }
    let refs = skeletal_paths
        .iter()
        .map(PathBuf::as_path)
        .collect::<Vec<_>>();
    let clips = load_animation_clips(&refs, &asset.bones).map_err(
        |error| {
            PipelineError::new(format!(
                "vehicle animation assembly failed for {}: {error:?}",
                package.package_id
            ))
        },
    )?;
    Ok((clips, sidecars))
}

/// Publish every unreferenced local PNG as damage or alternate appearance data.
fn publish_unreferenced_textures(
    package_root: &Path,
    texture_dir: &Path,
    materials: &[MaterialBinding],
) -> Result<(), PipelineError> {
    let referenced = materials
        .iter()
        .filter_map(|material| material.texture_file_name.as_deref())
        .map(|name| name.to_ascii_lowercase())
        .collect::<BTreeSet<_>>();
    let source_dir = package_root.join("components").join("texture");
    for source in png_files(&source_dir)? {
        let file_name = source
            .file_name()
            .and_then(|value| value.to_str())
            .ok_or_else(|| PipelineError::new("vehicle texture has no UTF-8 name"))?;
        if referenced.contains(&file_name.to_ascii_lowercase()) {
            continue;
        }
        let role = texture_state_role(file_name);
        let destination_dir = texture_dir.join(role);
        fs::create_dir_all(&destination_dir)
            .map_err(|error| PipelineError::new(error.to_string()))?;
        fs::copy(&source, destination_dir.join(file_name)).map_err(
            |error| {
                PipelineError::new(format!(
                    "vehicle alternate texture copy failed for {}: {error}",
                    source.display()
                ))
            },
        )?;
    }
    Ok(())
}

/// Classify one unreferenced texture into damage or alternate state storage.
fn texture_state_role(file_name: &str) -> &'static str {
    let lower = file_name.to_ascii_lowercase();
    if lower.contains("dam") || lower.contains("damage") {
        "damage"
    } else {
        "alternates"
    }
}

/// Publish one normalized decoded shader document under its semantic identity.
fn publish_shader_document(
    package_root: &Path,
    output_dir: &Path,
    shader: &str,
) -> Result<(), PipelineError> {
    let source = find_shader_document(package_root, shader)?;
    let value: Value = serde_json::from_slice(
        &fs::read(&source).map_err(|error| PipelineError::new(error.to_string()))?,
    )
    .map_err(|error| PipelineError::new(error.to_string()))?;
    let bytes = serde_json::to_vec_pretty(&value)
        .map_err(|error| PipelineError::new(error.to_string()))?;
    let path = output_dir.join(format!("{}.json", portable_name(shader)));
    write_new(&path, &bytes)
}

/// Find a shader JSON by decoded fixed-width identity, not source file padding.
fn find_shader_document(
    package_root: &Path,
    shader: &str,
) -> Result<PathBuf, PipelineError> {
    let directory = package_root.join("components").join("shader");
    let mut matches = fs::read_dir(&directory)
        .map_err(|error| PipelineError::new(error.to_string()))?
        .map(
            |entry| {
                entry
                    .map(|value| value.path())
                    .map_err(|error| PipelineError::new(error.to_string()))
            },
        )
        .collect::<Result<Vec<_>, _>>()?;
    matches.retain(
        |path| {
            path.is_file()
                && decoded_name(path)
                    .is_ok_and(|name| name.eq_ignore_ascii_case(shader))
        },
    );
    matches.sort();
    match matches.as_slice() {
        [path] => Ok(path.clone()),
        [] => Err(PipelineError::new(format!(
            "vehicle shader document is missing: {shader}"
        ))),
        _ => Err(PipelineError::new(format!(
            "vehicle shader document is ambiguous: {shader}"
        ))),
    }
}

/// Inventory every published PNG below one vehicle directory.
fn texture_records(vehicle_dir: &Path) -> Result<Vec<TextureRecord>, PipelineError> {
    let texture_root = vehicle_dir.join("textures");
    let mut records = Vec::new();
    for path in recursive_files(&texture_root)? {
        if path
            .extension()
            .and_then(|value| value.to_str())
            .is_none_or(|value| !value.eq_ignore_ascii_case("png"))
        {
            continue;
        }
        let relative = path
            .strip_prefix(vehicle_dir)
            .map_err(|error| PipelineError::new(error.to_string()))?
            .to_string_lossy()
            .replace('\\', "/");
        let role = if relative.starts_with("textures/damage/") {
            "damage"
        } else if relative.starts_with("textures/alternates/") {
            "alternate"
        } else {
            "normal"
        };
        let bytes = fs::read(&path)
            .map_err(|error| PipelineError::new(error.to_string()))?;
        records.push(TextureRecord {
            path: relative,
            role,
            bytes: u64::try_from(bytes.len())
                .map_err(|_| PipelineError::new("vehicle texture size overflowed"))?,
            sha256: digest_hex(&bytes),
        });
    }
    records.sort_by(|left, right| left.path.cmp(&right.path));
    Ok(records)
}

/// Verify canonical external-texture binary FBX 7.7 output.
fn verify_binary_fbx(path: &Path) -> Result<(), PipelineError> {
    const MAGIC: &[u8] = b"Kaydara FBX Binary  \0\x1a\0";
    let bytes = fs::read(path).map_err(|error| PipelineError::new(error.to_string()))?;
    let version = bytes
        .get(23..27)
        .and_then(|slice| <[u8; 4]>::try_from(slice).ok())
        .map(u32::from_le_bytes);
    if bytes.get(..MAGIC.len()) != Some(MAGIC)
        || version != Some(7700)
        || bytes
            .windows(b"Content".len())
            .any(|window| window == b"Content")
    {
        return Err(PipelineError::new(format!(
            "vehicle binary FBX verification failed: {}",
            path.display()
        )));
    }
    Ok(())
}

/// Produce a portable readable object or sidecar identity.
fn portable_name(value: &str) -> String {
    let mut output = String::new();
    let mut separated = false;
    for character in value.trim_end_matches('\0').chars() {
        if character.is_ascii_alphanumeric() {
            output.push(character.to_ascii_lowercase());
            separated = false;
        } else if !separated && !output.is_empty() {
            output.push('-');
            separated = true;
        }
    }
    output.trim_end_matches('-').to_owned()
}

#[cfg(test)]
mod tests {
    use super::{is_wheel_identity, texture_state_role, vehicle_part_role};

    #[test]
    fn semantic_roles_keep_moving_and_glass_parts_separate() {
        assert_eq!(vehicle_part_role("TrunkRotShape", "trunk_m"), "trunk");
        assert_eq!(
            vehicle_part_role("DoorDRotShape", "door_m"),
            "driver-door"
        );
        assert_eq!(
            vehicle_part_role("homer_vShape", "WindsheildT_m"),
            "glass"
        );
        assert_eq!(vehicle_part_role("w0Shape", "wheel_m"), "wheel");
    }

    #[test]
    fn wheel_identity_does_not_capture_unrelated_body_names() {
        assert!(is_wheel_identity("wshape3"));
        assert!(is_wheel_identity("w2shape"));
        assert!(!is_wheel_identity("windowshape"));
    }

    #[test]
    fn damage_textures_receive_a_distinct_sidecar_role() {
        assert_eq!(texture_state_role("homer_vDoorDDam.png"), "damage");
        assert_eq!(texture_state_role("homer_vSideFL.png"), "alternates");
    }
}

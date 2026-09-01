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
//   - Prepare outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Prepare outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Prepare outbound adapter.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use fbx::adapters::driven::binary_character_writer::write_binary_character_fbx;
use fbx::adapters::driven::decoded_animation_source::load_animation_clips;
use fbx::adapters::driven::decoded_billboard_source::read_billboard_quad_group;
use fbx::adapters::driven::decoded_component_source::{
    DecodedComponentError, DecodedComponentSource, read_shader_source_evidence,
};
use fbx::adapters::driven::decoded_rigid_prop_source::{
    SupplementalRigidPropBinding,
    load_instanced_rigid_prop_asset_with_billboards,
};
use fbx::domain::animation::AnimationClip;
use fbx::domain::character::{CharacterAsset, SkinnedPart};
use fbx::domain::mesh::MeshAsset;
use fbx::domain::texture::{MaterialBinding, MaterialSemantics};
use fbx::ports::component_source::ComponentSource as _;
use serde_json::Value;
use shar_sha256::digest_hex;

use super::catalog::{recursive_files, write_new};
use super::model::{
    EffectAnimationRecord, EffectControllerRecord, GroundingRecord, PartRecord,
    TextureRecord, VehicleRecord,
};
use super::source::{
    VehicleTextureAuthority, common_headlight_quad_groups, decoded_name,
    png_files, relative_art_root, select_vehicle_composite,
    select_vehicle_skeleton,
    vehicle_animation_paths, vehicle_mesh_paths, vehicle_quad_group_paths,
};
use crate::domain::PipelineError;
use crate::domain::package::PhaseThreePackageRow;

/// Export one vehicle while preserving every authored render component.
#[expect(
    clippy::too_many_lines,
    reason = "Vehicle assembly and catalog publication must remain atomic."
)]
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
    fs::create_dir_all(&texture_dir).map_err(|error| {
        PipelineError::new(format!("vehicle texture output failed: {error}"))
    })?;
    fs::create_dir_all(&shader_dir).map_err(|error| {
        PipelineError::new(format!("vehicle shader output failed: {error}"))
    })?;
    let skeleton = select_vehicle_skeleton(&package_root, &vehicle)?;
    let composite = select_vehicle_composite(&package_root, &vehicle)?;
    let source_mesh_paths = vehicle_mesh_paths(package, &package_root)?;
    let (retained_mesh_paths, mut deferred_geometry) =
        partition_vehicle_meshes(&source_mesh_paths, &vehicle_dir)?;
    let mesh_refs = retained_mesh_paths
        .iter()
        .map(PathBuf::as_path)
        .collect::<Vec<_>>();
    let source_billboard_paths =
        vehicle_quad_group_paths(package, &package_root)?;
    let (mut retained_billboard_paths, deferred_billboards) =
        partition_vehicle_billboards(&source_billboard_paths, &vehicle_dir)?;
    deferred_geometry.extend(deferred_billboards);
    deferred_geometry.sort();
    let (common_root, common_headlights) =
        common_headlight_quad_groups(normalized_root)?;
    let mut supplemental = Vec::new();
    for path in &common_headlights {
        let component_name = decoded_name(path)?;
        for joint_id in ["hll", "hlr"] {
            supplemental.push(SupplementalRigidPropBinding {
                component_name: component_name.clone(),
                joint_id: joint_id.to_owned(),
            });
        }
    }
    retained_billboard_paths.extend(common_headlights);
    let billboard_refs = retained_billboard_paths
        .iter()
        .map(PathBuf::as_path)
        .collect::<Vec<_>>();
    let assembled_asset = load_instanced_rigid_prop_asset_with_billboards(
        &vehicle,
        &skeleton,
        &mesh_refs,
        &billboard_refs,
        &composite,
        &supplemental,
    )
    .map_err(|error| {
        PipelineError::new(format!(
            "vehicle rigid assembly failed for {}: {error:?}",
            package.package_id
        ))
    })?;
    let hidden_proxy_indices =
        hidden_wheel_proxy_indices(&assembled_asset, &vehicle);
    let (grounded_asset, ground_offset, root_bone, grounding_source) =
        if vehicle == "mono-v" {
            let (asset, offset, root) =
                ground_monorail_asset(assembled_asset, &hidden_proxy_indices)?;
            (
                asset,
                offset,
                root,
                "visible-body-with-authored-wheel-proxies",
            )
        } else {
            let (asset, offset, root) = ground_vehicle_asset(assembled_asset)?;
            (asset, offset, root, "road-wheel-surfaces")
        };
    let (mut prepared_asset, wheel_proxy_sidecars, hidden_wheel_proxies) =
        mark_hidden_wheel_proxies(
            grounded_asset,
            &vehicle_dir,
            &hidden_proxy_indices,
        )?;
    deferred_geometry.extend(wheel_proxy_sidecars);
    deferred_geometry.sort();
    let (materials, shaders) = resolve_vehicle_materials(
        package,
        &package_root,
        &common_root,
        &texture_dir,
        &shader_dir,
        authority,
        &mut prepared_asset,
    )?;
    let (separated, parts) =
        separate_vehicle_parts(prepared_asset, &materials)?;
    let (mut animations, effect_animation_sidecars) = load_vehicle_animations(
        package,
        &package_root,
        &vehicle_dir,
        &separated,
    )?;
    ground_vehicle_animations(&mut animations, &root_bone, ground_offset)?;
    let fbx_path = vehicle_dir.join(format!("{vehicle}.fbx"));
    let summary = write_binary_character_fbx(
        &separated,
        &materials,
        &animations,
        &fbx_path,
    )
    .map_err(|error| {
        PipelineError::new(format!(
            "vehicle FBX serialization failed for {}: {error:?}",
            package.package_id
        ))
    })?;
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
        fbx_bytes: u64::try_from(fbx_payload.len()).map_err(|error| {
            PipelineError::new(format!("vehicle FBX size overflowed: {error}"))
        })?,
        fbx_sha256: digest_hex(&fbx_payload),
        summary,
        grounding: GroundingRecord {
            source: grounding_source,
            offset_y: ground_offset,
            root_bone,
        },
        parts,
        deferred_geometry,
        hidden_wheel_proxies,
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
        .ok_or_else(|| {
            PipelineError::new("vehicle subcategory has no identity")
        })?;
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
                let valid = position.as_array().is_some_and(|components| {
                    components.len() == 3
                        && components
                            .iter()
                            .all(|component| component.as_f64().is_some())
                });
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
                "vehicle mesh has partially invalid positions: {} of \
                         {} in {}",
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

/// Defer malformed billboard evidence without inventing source geometry.
fn partition_vehicle_billboards(
    paths: &[PathBuf],
    vehicle_dir: &Path,
) -> Result<(Vec<PathBuf>, Vec<String>), PipelineError> {
    let mut retained = Vec::new();
    let mut deferred = Vec::new();
    for path in paths {
        let identity = decoded_name(path)?;
        match read_billboard_quad_group(path, &identity) {
            Ok(_mesh) => retained.push(path.clone()),
            Err(_error) => {
                let directory =
                    vehicle_dir.join("geometry").join("deferred-billboards");
                fs::create_dir_all(&directory)
                    .map_err(|error| PipelineError::new(error.to_string()))?;
                let file_name = path
                    .file_name()
                    .and_then(|value| value.to_str())
                    .ok_or_else(|| {
                        PipelineError::new(
                            "deferred billboard path has no UTF-8 file \
                                 name",
                        )
                    })?;
                let destination = directory.join(file_name);
                let _copied_bytes = fs::copy(path, &destination)
                    .map_err(|error| PipelineError::new(error.to_string()))?;
                deferred
                    .push(format!("geometry/deferred-billboards/{file_name}"));
            },
        }
    }
    deferred.sort();
    Ok((retained, deferred))
}

/// Return the one authored skeleton root identity.
fn skeleton_root_id(asset: &CharacterAsset) -> Result<String, PipelineError> {
    let roots = asset
        .bones
        .iter()
        .filter(|bone| bone.parent_id.is_none())
        .map(|bone| bone.id.clone())
        .collect::<Vec<_>>();
    let [root] = roots.as_slice() else {
        return Err(PipelineError::new(
            "vehicle requires exactly one skeleton root",
        ));
    };
    Ok(root.clone())
}

/// Ground one fully assembled vehicle by its four authored road-wheel surfaces.
fn ground_vehicle_asset(
    mut asset: CharacterAsset,
) -> Result<(CharacterAsset, f64, String), PipelineError> {
    let mut wheel_bones = BTreeSet::new();
    let mut lowest = f32::INFINITY;
    for part in &asset.parts {
        let bound_wheels = part
            .group_influences
            .iter()
            .flatten()
            .filter_map(|influence| {
                is_road_wheel_bone(&influence.bone_id)
                    .then_some(influence.bone_id.clone())
            })
            .collect::<BTreeSet<_>>();
        if bound_wheels.is_empty() {
            continue;
        }
        wheel_bones.extend(bound_wheels);
        for position in part
            .mesh
            .groups
            .iter()
            .flat_map(|group| group.positions.iter())
        {
            lowest = lowest.min(position[1]);
        }
    }
    if wheel_bones.len() != 4 || !lowest.is_finite() {
        return Err(PipelineError::new(format!(
            "vehicle grounding requires four road-wheel surfaces, \
                     found {}",
            wheel_bones.len()
        )));
    }
    let offset = -lowest;
    for position in asset
        .parts
        .iter_mut()
        .flat_map(|part| part.mesh.groups.iter_mut())
        .flat_map(|group| group.positions.iter_mut())
    {
        position[1] += offset;
    }
    let roots = asset
        .bones
        .iter()
        .enumerate()
        .filter_map(|(index, bone)| bone.parent_id.is_none().then_some(index))
        .collect::<Vec<_>>();
    let [root_index] = roots.as_slice() else {
        return Err(PipelineError::new(
            "vehicle grounding requires exactly one skeleton root",
        ));
    };
    let root = asset.bones.get_mut(*root_index).ok_or_else(|| {
        PipelineError::new("vehicle grounding root is missing")
    })?;
    root.rest_matrix[13] += offset;
    if !root.rest_matrix[13].is_finite() {
        return Err(PipelineError::new(
            "vehicle grounding produced a non-finite root translation",
        ));
    }
    let root_id = root.id.clone();
    Ok((asset, f64::from(offset), root_id))
}

/// Select exactly four source-backed non-visual road-wheel proxies.
fn hidden_wheel_proxy_indices(
    asset: &CharacterAsset,
    vehicle: &str,
) -> BTreeSet<usize> {
    let candidates = asset
        .parts
        .iter()
        .enumerate()
        .filter_map(|(index, part)| {
            let bones = part_bone_ids(part);
            (bones.len() == 1
                && bones.iter().all(|bone| is_road_wheel_bone(bone)))
            .then_some(index)
        })
        .collect::<BTreeSet<_>>();
    let candidate_bones = candidates
        .iter()
        .filter_map(|index| asset.parts.get(*index))
        .flat_map(part_bone_ids)
        .collect::<BTreeSet<_>>();
    let expected_bones = [
        "w0".to_owned(),
        "w1".to_owned(),
        "w2".to_owned(),
        "w3".to_owned(),
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();
    if candidates.len() != 4 || candidate_bones != expected_bones {
        return BTreeSet::new();
    }

    if matches!(vehicle, "hbike-v" | "frink-v" | "mono-v") {
        return candidates;
    }

    let has_visual_wheel_geometry = asset
        .parts
        .iter()
        .flat_map(|part| &part.mesh.groups)
        .any(|group| is_visual_wheel_material(&group.shader));
    let all_box_proxies = candidates.iter().all(|index| {
        asset.parts.get(*index).is_some_and(|part| {
            part.mesh
                .groups
                .iter()
                .all(|group| !is_visual_wheel_material(&group.shader))
                && is_axis_aligned_box_proxy(&part.mesh)
        })
    });
    if has_visual_wheel_geometry && all_box_proxies {
        candidates
    } else {
        BTreeSet::new()
    }
}

/// Ground the monorail body while preserving wheel-proxy rest placement.
fn ground_monorail_asset(
    mut asset: CharacterAsset,
    hidden_proxies: &BTreeSet<usize>,
) -> Result<(CharacterAsset, f64, String), PipelineError> {
    if hidden_proxies.len() != 4 {
        return Err(PipelineError::new(
            "monorail grounding requires four hidden wheel proxies",
        ));
    }
    let mut lowest = f32::INFINITY;
    for (index, part) in asset.parts.iter().enumerate() {
        if hidden_proxies.contains(&index) {
            continue;
        }
        for position in
            part.mesh.groups.iter().flat_map(|group| &group.positions)
        {
            lowest = lowest.min(position[1]);
        }
    }
    if !lowest.is_finite() {
        return Err(PipelineError::new(
            "monorail visible geometry has no finite ground surface",
        ));
    }
    let offset = -lowest;
    for (index, part) in asset.parts.iter_mut().enumerate() {
        for position in part
            .mesh
            .groups
            .iter_mut()
            .flat_map(|group| &mut group.positions)
        {
            position[1] += offset;
            if hidden_proxies.contains(&index) {
                position[1] -= offset;
            }
        }
    }
    let root_id = skeleton_root_id(&asset)?;
    for bone in &mut asset.bones {
        if bone.id == root_id {
            bone.rest_matrix[13] += offset;
        } else if is_road_wheel_bone(&bone.id) {
            bone.rest_matrix[13] -= offset;
        }
        if !bone.rest_matrix[13].is_finite() {
            return Err(PipelineError::new(
                "monorail grounding produced a non-finite bind translation",
            ));
        }
    }
    Ok((asset, f64::from(offset), root_id))
}

/// Mark non-visual road-wheel proxies invisible while retaining exact evidence.
fn mark_hidden_wheel_proxies(
    mut asset: CharacterAsset,
    vehicle_dir: &Path,
    candidate_indices: &BTreeSet<usize>,
) -> Result<(CharacterAsset, Vec<String>, usize), PipelineError> {
    if candidate_indices.is_empty() {
        return Ok((asset, Vec::new(), 0));
    }
    let mut proxies = Vec::new();
    for (index, part) in asset.parts.iter_mut().enumerate() {
        if !candidate_indices.contains(&index) {
            continue;
        }
        let (minimum, maximum) = mesh_bounds(&part.mesh)?;
        let groups = part
            .mesh
            .groups
            .iter()
            .map(|group| {
                serde_json::json!({
                    "shader": group.shader,
                    "positions": group.positions,
                    "normals": group.normals,
                    "uvs": group.uvs,
                    "triangles": group.triangles
                })
            })
            .collect::<Vec<_>>();
        proxies.push(serde_json::json!({
            "source_mesh": part.mesh.name,
            "bones": part_bone_ids(part),
            "bounds": {
                "minimum": minimum,
                "maximum": maximum
            },
            "groups": groups
        }));
        if !part.mesh.name.contains("__hidden-wheel-proxy") {
            part.mesh.name.push_str("__hidden-wheel-proxy");
        }
    }
    let directory = vehicle_dir.join("geometry");
    fs::create_dir_all(&directory)
        .map_err(|error| PipelineError::new(error.to_string()))?;
    let file_name = "hidden-wheel-proxies.json";
    let payload = serde_json::to_vec_pretty(&serde_json::json!({
        "schema": "vehicle-wheel-proxies",
        "policy": concat!(
            "present in FBX with Visibility=0; retained for pivots, ",
            "animation, grounding evidence, and later physics work"
        ),
        "proxy_count": proxies.len(),
        "proxies": proxies
    }))
    .map_err(|error| PipelineError::new(error.to_string()))?;
    write_new(&directory.join(file_name), &payload)?;
    Ok((
        asset,
        vec![format!("geometry/{file_name}")],
        candidate_indices.len(),
    ))
}

/// Collect distinct bone identities referenced by one rigid part.
fn part_bone_ids(part: &SkinnedPart) -> BTreeSet<String> {
    part.group_influences
        .iter()
        .flatten()
        .map(|influence| influence.bone_id.clone())
        .collect()
}

/// Return whether one material identity represents authored visible wheel art.
fn is_visual_wheel_material(value: &str) -> bool {
    let identity = value.to_ascii_lowercase();
    identity.contains("wheel")
        || identity.contains("tire")
        || identity.contains("tyre")
}

/// Recognize one axis-aligned eight-corner box used as a physics wheel proxy.
fn is_axis_aligned_box_proxy(mesh: &MeshAsset) -> bool {
    let positions = mesh
        .groups
        .iter()
        .flat_map(|group| &group.positions)
        .collect::<Vec<_>>();
    if positions.len() < 8 {
        return false;
    }
    let corner_result = positions
        .iter()
        .map(|position| {
            let [x, y, z] = **position;
            Some([
                quantized_proxy_axis(x)?,
                quantized_proxy_axis(y)?,
                quantized_proxy_axis(z)?,
            ])
        })
        .collect::<Option<BTreeSet<_>>>();
    let Some(corners) = corner_result else {
        return false;
    };
    if corners.len() != 8 {
        return false;
    }
    let x_values = corners
        .iter()
        .map(|[x, _y, _z]| *x)
        .collect::<BTreeSet<_>>();
    let y_values = corners
        .iter()
        .map(|[_x, y, _z]| *y)
        .collect::<BTreeSet<_>>();
    let z_values = corners
        .iter()
        .map(|[_x, _y, z]| *z)
        .collect::<BTreeSet<_>>();
    x_values.len() == 2 && y_values.len() == 2 && z_values.len() == 2
}

/// Quantize one finite proxy coordinate after checking the target integer
/// range.
#[expect(
    clippy::as_conversions,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    reason = "The rounded value is checked against the exact i64 range before \
              conversion."
)]
fn quantized_proxy_axis(value: f32) -> Option<i64> {
    let rounded = f64::from(value).mul_add(100_000f64, 0f64).round();
    if !rounded.is_finite()
        || rounded < i64::MIN as f64
        || rounded > i64::MAX as f64
    {
        return None;
    }
    Some(rounded as i64)
}

/// Calculate finite mesh bounds for deferred proxy evidence.
fn mesh_bounds(
    mesh: &MeshAsset,
) -> Result<([f32; 3], [f32; 3]), PipelineError> {
    let mut minimum = [f32::INFINITY; 3];
    let mut maximum = [f32::NEG_INFINITY; 3];
    for position in mesh.groups.iter().flat_map(|group| &group.positions) {
        for ((minimum_axis, maximum_axis), value) in minimum
            .iter_mut()
            .zip(maximum.iter_mut())
            .zip(position.iter().copied())
        {
            *minimum_axis = minimum_axis.min(value);
            *maximum_axis = maximum_axis.max(value);
        }
    }
    if minimum
        .iter()
        .chain(maximum.iter())
        .any(|value| !value.is_finite())
    {
        return Err(PipelineError::new(
            "vehicle wheel proxy has non-finite bounds",
        ));
    }
    Ok((minimum, maximum))
}

/// Return whether one skeleton identity is one of the four road-wheel pivots.
fn is_road_wheel_bone(value: &str) -> bool {
    matches!(
        value.to_ascii_lowercase().as_str(),
        "w0" | "w1" | "w2" | "w3"
    )
}

/// Apply the same grounding translation to root animation samples.
fn ground_vehicle_animations(
    clips: &mut [AnimationClip],
    root_bone: &str,
    offset: f64,
) -> Result<(), PipelineError> {
    for clip in clips {
        if let Some(track) = clip
            .tracks
            .iter_mut()
            .find(|track| track.bone_id == root_bone)
        {
            for sample in &mut track.samples {
                sample.translation[1] += offset;
                if !sample.translation[1].is_finite() {
                    return Err(PipelineError::new(format!(
                        "vehicle grounding produced a non-finite root \
                                 sample in {}",
                        clip.name
                    )));
                }
            }
        }
    }
    Ok(())
}

/// Split every primitive group into a named semantic vehicle object.
fn separate_vehicle_parts(
    asset: CharacterAsset,
    materials: &[MaterialBinding],
) -> Result<(CharacterAsset, Vec<PartRecord>), PipelineError> {
    let semantics_by_material = materials
        .iter()
        .map(|material| (material.material_name.as_str(), material.semantics))
        .collect::<BTreeMap<_, _>>();
    let mut parts = Vec::new();
    let mut records = Vec::new();
    let mut used_names = BTreeMap::<String, usize>::new();
    for part in asset.parts {
        let source_identity = part.mesh.source_identity.clone();
        let cast_shadow = part.mesh.cast_shadow;
        for (group, influences) in
            part.mesh.groups.into_iter().zip(part.group_influences)
        {
            let material_semantics = semantics_by_material
                .get(group.shader.as_str())
                .copied()
                .ok_or_else(|| {
                    PipelineError::new(format!(
                        "vehicle semantic material is missing: {}",
                        group.shader
                    ))
                })?;
            let semantics = vehicle_part_semantics(
                &part.mesh.name,
                &group.shader,
                material_semantics,
            );
            let role =
                vehicle_part_role(&part.mesh.name, &group.shader, semantics);
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
            let mesh = MeshAsset::new(&name, vec![group])
                .and_then(|mesh| match source_identity.clone() {
                    Some(identity) => mesh.with_source_identity(identity),
                    None => Ok(mesh),
                })
                .map(|mesh| mesh.with_cast_shadow(cast_shadow))
                .map_err(|error| {
                    PipelineError::new(format!(
                        "vehicle semantic mesh {name} failed: {error:?}"
                    ))
                })?;
            parts.push(SkinnedPart {
                mesh,
                group_influences: vec![influences],
            });
            records.push(PartRecord {
                name,
                source_mesh,
                role,
                shader,
                semantics,
                bones,
            });
        }
    }
    let source_provenance = asset.source_provenance;
    let separated = CharacterAsset::new(asset.name, asset.bones, parts)
        .map(|asset| match source_provenance {
            Some(provenance) => asset.with_source_provenance(provenance),
            None => asset,
        })
        .map_err(|error| {
            PipelineError::new(format!(
                "vehicle semantic asset failed: {error:?}"
            ))
        })?;
    Ok((separated, records))
}

/// Merge shared material, source-mesh, and exact runtime light evidence.
fn vehicle_part_semantics(
    mesh_name: &str,
    shader_name: &str,
    material_semantics: MaterialSemantics,
) -> MaterialSemantics {
    let identity_semantics =
        MaterialSemantics::from_identities(mesh_name, Some(shader_name));
    let runtime_semantics = MaterialSemantics::default()
        .with_light_emitter(runtime_light_shape(mesh_name));
    material_semantics
        .merge(identity_semantics)
        .merge(runtime_semantics)
}

/// Classify one vehicle geometry group without inventing unsupported parts.
fn vehicle_part_role(
    mesh_name: &str,
    shader_name: &str,
    semantics: MaterialSemantics,
) -> &'static str {
    let hidden_wheel_proxy = mesh_name.contains("hidden-wheel-proxy");
    let mesh = mesh_name
        .split("__joint_")
        .next()
        .unwrap_or(mesh_name)
        .to_ascii_lowercase();
    let shader = shader_name.to_ascii_lowercase();
    if hidden_wheel_proxy {
        "hidden-wheel-proxy"
    } else if semantics.is_mirror() {
        "mirror"
    } else if semantics.is_glass() {
        "glass"
    } else if semantics.is_visual_effect() {
        "vfx"
    } else if semantics.is_light_emitter() {
        "light-emitter"
    } else if mesh.contains("trunk") {
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
    } else if semantics.is_reflective() {
        "reflective"
    } else if semantics.is_transparent() {
        "transparent"
    } else {
        "body"
    }
}

/// Recognize exact vehicle light prop identities used by the original runtime.
fn runtime_light_shape(value: &str) -> bool {
    let compact = value
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .collect::<String>()
        .to_ascii_lowercase();
    [
        "brake1shape",
        "brake2shape",
        "brake3shape",
        "brake4shape",
        "rev1shape",
        "rev2shape",
        "rev3shape",
        "rev4shape",
    ]
    .iter()
    .any(|identity| compact.contains(identity))
}

/// Return whether one authored mesh identity represents a road wheel object.
fn is_wheel_identity(mesh: &str) -> bool {
    if mesh.contains("steering") {
        return false;
    }
    mesh.contains("wheel")
        || mesh.starts_with("wshape")
        || matches!(mesh.strip_suffix("shape"), Some("w0" | "w1" | "w2" | "w3"))
}

/// Resolve used shaders, preserve authored identities, and publish JSON
/// sidecars.
fn resolve_vehicle_materials(
    package: &PhaseThreePackageRow,
    package_root: &Path,
    common_root: &Path,
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
    let mut by_source = BTreeMap::new();
    let mut by_material = BTreeMap::<String, MaterialBinding>::new();
    for shader in shader_names {
        let material_root =
            shader_material_root(package_root, common_root, &shader)?;
        let source = DecodedComponentSource::new(&material_root, texture_dir);
        let binding = match source.resolve_material(&shader) {
            Ok(binding) => binding,
            Err(DecodedComponentError::MissingTexture { texture, .. }) => {
                let external = authority
                    .resolve(&texture, &package.subcategory)?
                    .ok_or_else(|| {
                        PipelineError::new(format!(
                            "vehicle shader {shader} has no texture \
                                     authority for {texture}"
                        ))
                    })?;
                source
                    .resolve_material_with_external_texture(&shader, external)
                    .map_err(|error| {
                        PipelineError::new(format!(
                            "vehicle shared texture failed for \
                                     {shader}: {error:?}"
                        ))
                    })?
            },
            Err(error) => {
                return Err(PipelineError::new(format!(
                    "vehicle material {shader} failed: {error:?}"
                )));
            },
        };
        let material_name = binding.material_name.clone();
        if let Some(previous) =
            by_material.insert(material_name.clone(), binding.clone())
            && previous != binding
        {
            return Err(PipelineError::new(format!(
                "vehicle material identity conflicts: {material_name}"
            )));
        }
        let _previous_source = by_source.insert(shader.clone(), material_name);
        publish_shader_document(&material_root, shader_dir, &shader)?;
    }
    for group in asset
        .parts
        .iter_mut()
        .flat_map(|part| part.mesh.groups.iter_mut())
    {
        group.shader = by_source
            .get(&group.shader)
            .ok_or_else(|| {
                PipelineError::new("vehicle material rename is missing")
            })?
            .clone();
    }
    let shaders = by_source
        .into_values()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    Ok((by_material.into_values().collect(), shaders))
}

/// Select the exact package that owns one used shader identity.
fn shader_material_root(
    package_root: &Path,
    common_root: &Path,
    shader: &str,
) -> Result<PathBuf, PipelineError> {
    let local = find_shader_document(package_root, shader);
    if local.is_ok() {
        return Ok(package_root.to_path_buf());
    }
    let common = find_shader_document(common_root, shader);
    if common.is_ok() {
        return Ok(common_root.to_path_buf());
    }
    Err(PipelineError::new(format!(
        "vehicle material shader is missing from local and common \
                 packages: {shader}"
    )))
}

/// Return one decoded vehicle animation identity without repairing source text.
fn vehicle_animation_name(value: &Value) -> Result<&str, PipelineError> {
    let name = value
        .get("name")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            PipelineError::new("vehicle animation has no source name")
        })?;
    let clean = name.trim_end_matches('\u{0}');
    if clean.is_empty()
        || clean != clean.trim()
        || clean.chars().any(char::is_control)
    {
        return Err(PipelineError::new(
            "vehicle animation identity is non-canonical",
        ));
    }
    Ok(clean)
}

/// Export skeletal clips and preserve texture/effect animations as sidecars.
fn load_vehicle_animations(
    package: &PhaseThreePackageRow,
    package_root: &Path,
    vehicle_dir: &Path,
    asset: &CharacterAsset,
) -> Result<(Vec<AnimationClip>, Vec<EffectAnimationRecord>), PipelineError> {
    let paths = vehicle_animation_paths(package, package_root)?;
    let mut skeletal_paths = Vec::new();
    let mut sidecars = Vec::new();
    let mut used_names = BTreeMap::<String, usize>::new();
    for path in paths {
        let bytes = fs::read(&path)
            .map_err(|error| PipelineError::new(error.to_string()))?;
        let value: Value = serde_json::from_slice(&bytes)
            .map_err(|error| PipelineError::new(error.to_string()))?;
        let name = vehicle_animation_name(&value)?;
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
        let source_ordinal = vehicle_animation_source_ordinal(package, &path)?;
        let controller = vehicle_effect_controller_relationship(
            package_root,
            name,
            kind,
        )?;
        if kind.eq_ignore_ascii_case("TEX_") {
            let relationship = controller.as_ref().ok_or_else(|| {
                PipelineError::new(
                    "vehicle texture animation has no controller target",
                )
            })?;
            let initial_texture = vehicle_target_shader_texture_identity(
                package_root,
                relationship,
            )?;
            validate_vehicle_texture_animation(
                &value,
                &relationship.target_identity,
                &initial_texture,
            )?;
        }
        let payload = serde_json::to_vec_pretty(&value)
            .map_err(|error| PipelineError::new(error.to_string()))?;
        write_new(&output_dir.join(&file_name), &payload)?;
        sidecars.push(EffectAnimationRecord {
            path: format!("animations/effects/{file_name}"),
            identity: name.to_owned(),
            animation_type: kind.to_owned(),
            source_ordinal,
            controller,
        });
    }
    if skeletal_paths.is_empty() {
        return Ok((Vec::new(), sidecars));
    }
    let refs = skeletal_paths
        .iter()
        .map(PathBuf::as_path)
        .collect::<Vec<_>>();
    let clips = load_animation_clips(&refs, &asset.bones).map_err(|error| {
        PipelineError::new(format!(
            "vehicle animation assembly failed for {}: {error:?}",
            package.package_id
        ))
    })?;
    Ok((clips, sidecars))
}

/// Read the canonical texture identity from the exact target shader occurrence.
fn vehicle_target_shader_texture_identity(
    package_root: &Path,
    relationship: &EffectControllerRecord,
) -> Result<String, PipelineError> {
    if relationship.target_kind != "shader" {
        return Err(PipelineError::new(
            "vehicle texture animation target is not a shader",
        ));
    }
    let ledger_path = package_root.join("components.jsonl");
    let ledger = fs::read_to_string(&ledger_path)
        .map_err(|error| PipelineError::new(error.to_string()))?;
    let target_ordinal = u64::try_from(relationship.target_source_ordinal)
        .map_err(|error| PipelineError::new(error.to_string()))?;
    let rows = ledger
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            serde_json::from_str::<Value>(line)
                .map_err(|error| PipelineError::new(error.to_string()))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let matches = rows
        .iter()
        .filter(|row| {
            row.get("kind").and_then(Value::as_str) == Some("shader")
                && row.get("ordinal").and_then(Value::as_u64)
                    == Some(target_ordinal)
        })
        .collect::<Vec<_>>();
    let [row] = matches.as_slice() else {
        return Err(PipelineError::new(
            "vehicle texture animation target shader ordinal is ambiguous",
        ));
    };
    let relative = row.get("path").and_then(Value::as_str).ok_or_else(|| {
        PipelineError::new("vehicle target shader ledger row has no path")
    })?;
    let shader_path = package_root.join("components").join(relative);
    let evidence = read_shader_source_evidence(
        &shader_path,
        &relationship.target_identity,
    )
    .map_err(|error| {
        PipelineError::new(format!(
            "vehicle texture animation target shader is invalid: {error:?}"
        ))
    })?;
    evidence.texture_reference.ok_or_else(|| {
        PipelineError::new(
            "vehicle texture animation target shader has no TEX identity",
        )
    })
}

/// Validate the supported vehicle texture-animation evidence envelope.
fn validate_vehicle_texture_animation(
    value: &Value,
    target_identity: &str,
    initial_texture_identity: &str,
) -> Result<(), PipelineError> {
    if value.get("schema").and_then(Value::as_str) != Some("animation")
        || value.get("version").and_then(Value::as_u64) != Some(0)
        || value.get("type").and_then(Value::as_str) != Some("TEX_")
    {
        return Err(PipelineError::new(
            "vehicle texture animation document is unsupported",
        ));
    }
    let frame_count = value
        .get("frames")
        .and_then(Value::as_f64)
        .filter(|frames| frames.is_finite() && *frames > 0.)
        .ok_or_else(|| {
            PipelineError::new(
                "vehicle texture animation frame span is invalid",
            )
        })?;
    let _frame_rate = value
        .get("frame_rate")
        .and_then(Value::as_f64)
        .filter(|rate| rate.is_finite() && *rate > 0.)
        .ok_or_else(|| {
            PipelineError::new(
                "vehicle texture animation frame rate is invalid",
            )
        })?;
    if value.get("cyclic").and_then(Value::as_u64).is_none_or(|flag| flag > 1)
        || !empty_array(value, "loose_channels")
        || !empty_array(value, "legacy_animation_extras")
    {
        return Err(PipelineError::new(
            "vehicle texture animation top-level evidence is unsupported",
        ));
    }
    let sizes = required_array(value, "sizes", "texture animation sizes")?;
    let [size] = sizes.as_slice() else {
        return Err(PipelineError::new(
            "vehicle texture animation must have one size record",
        ));
    };
    if size.get("version").and_then(Value::as_u64) != Some(1)
        || ["pc", "ps2", "xbox", "gc"]
            .iter()
            .any(|field| size.get(*field).and_then(Value::as_u64).is_none())
    {
        return Err(PipelineError::new(
            "vehicle texture animation size evidence is unsupported",
        ));
    }
    let lists = required_array(
        value,
        "group_lists",
        "texture animation group lists",
    )?;
    let [list] = lists.as_slice() else {
        return Err(PipelineError::new(
            "vehicle texture animation must have one group list",
        ));
    };
    let groups = required_array(list, "groups", "texture animation groups")?;
    if list.get("version").and_then(Value::as_u64) != Some(0)
        || list.get("num_groups").and_then(Value::as_u64) != Some(1)
    {
        return Err(PipelineError::new(
            "vehicle texture animation group list is unsupported",
        ));
    }
    let [group] = groups.as_slice() else {
        return Err(PipelineError::new(
            "vehicle texture animation must have one target group",
        ));
    };
    let expected_group = format!("TEX_{target_identity}");
    let group_identity = vehicle_component_identity(
        group,
        "name",
        "texture animation group",
    )?;
    let channels = required_array(
        group,
        "channels",
        "texture animation channels",
    )?;
    if group.get("version").and_then(Value::as_u64) != Some(0)
        || group.get("group_id").and_then(Value::as_u64) != Some(0)
        || group.get("num_channels").and_then(Value::as_u64) != Some(1)
        || group_identity != expected_group
    {
        return Err(PipelineError::new(
            "vehicle texture animation target group is unsupported",
        ));
    }
    let [channel] = channels.as_slice() else {
        return Err(PipelineError::new(
            "vehicle texture animation must have one entity channel",
        ));
    };
    validate_vehicle_texture_channel(
        channel,
        frame_count,
        initial_texture_identity,
    )
}

/// Validate one exact entity channel without assigning runtime texture meaning.
fn validate_vehicle_texture_channel(
    channel: &Value,
    frame_count: f64,
    initial_texture_identity: &str,
) -> Result<(), PipelineError> {
    if channel.get("kind").and_then(Value::as_str) != Some("entity")
        || channel.get("version").and_then(Value::as_u64) != Some(0)
        || channel.get("param").and_then(Value::as_str) != Some("TEX_")
    {
        return Err(PipelineError::new(
            "vehicle texture animation channel is unsupported",
        ));
    }
    let declared = channel
        .get("num_frames")
        .and_then(Value::as_u64)
        .and_then(|count| usize::try_from(count).ok())
        .filter(|count| *count > 0)
        .ok_or_else(|| {
            PipelineError::new("vehicle texture animation key count is invalid")
        })?;
    let frames = required_array(channel, "frames", "texture animation frames")?;
    let values = required_array(channel, "values", "texture animation values")?;
    if frames.len() != declared || values.len() != declared {
        return Err(PipelineError::new(
            "vehicle texture animation key counts disagree",
        ));
    }
    let mut previous = None;
    for frame in frames {
        let raw_frame = frame.as_u64().ok_or_else(|| {
            PipelineError::new("vehicle texture animation frame is invalid")
        })?;
        let frame = u16::try_from(raw_frame).map_err(|error| {
            PipelineError::new(format!(
                "vehicle texture animation frame is invalid: {error}"
            ))
        })?;
        if f64::from(frame) >= frame_count
            || previous.is_some_and(|prior| prior >= frame)
        {
            return Err(PipelineError::new(
                concat!(
                    "vehicle texture animation frames are unordered or out ",
                    "of range"
                ),
            ));
        }
        previous = Some(frame);
    }
    let canonical_values = values
        .iter()
        .map(|value| {
            let raw = value.as_str().ok_or_else(|| {
                PipelineError::new(
                    "vehicle texture animation value is not an identity",
                )
            })?;
            vehicle_source_identity(raw, "texture animation value")
        })
        .collect::<Result<Vec<_>, _>>()?;
    let frame_zero_texture = frames
        .iter()
        .zip(canonical_values.iter())
        .find_map(|(frame, identity)| {
            (frame.as_u64() == Some(0)).then_some(identity.as_str())
        })
        .ok_or_else(|| {
            PipelineError::new(
                "vehicle texture animation has no frame-zero texture key",
            )
        })?;
    if frame_zero_texture != initial_texture_identity {
        return Err(PipelineError::new(
            "vehicle texture animation frame zero contradicts target shader",
        ));
    }
    let metadata = required_array(
        channel,
        "channel_metadata",
        "texture animation metadata",
    )?;
    let [entry] = metadata.as_slice() else {
        return Err(PipelineError::new(
            "vehicle texture animation interpolation metadata is unsupported",
        ));
    };
    if entry.get("kind").and_then(Value::as_str)
        != Some("interpolation_mode")
        || entry.get("version").and_then(Value::as_u64) != Some(0)
        || entry.get("mode").and_then(Value::as_u64) != Some(1)
    {
        return Err(PipelineError::new(
            "vehicle texture animation interpolation metadata is unsupported",
        ));
    }
    Ok(())
}

/// Read one required JSON array from decoded source evidence.
fn required_array<'value>(
    value: &'value Value,
    field: &str,
    label: &str,
) -> Result<&'value Vec<Value>, PipelineError> {
    value.get(field).and_then(Value::as_array).ok_or_else(|| {
        PipelineError::new(format!("vehicle {label} are missing or invalid"))
    })
}

/// Return whether one required decoded source field is an empty array.
fn empty_array(value: &Value, field: &str) -> bool {
    value.get(field).and_then(Value::as_array).is_some_and(Vec::is_empty)
}

/// Resolve the exact source ordinal for one projected animation path.
fn vehicle_animation_source_ordinal(
    package: &PhaseThreePackageRow,
    path: &Path,
) -> Result<usize, PipelineError> {
    let file_name = path.file_name().ok_or_else(|| {
        PipelineError::new("vehicle effect animation path has no file name")
    })?;
    let matches = package
        .members()
        .iter()
        .filter(|member| {
            member.kind == "p3d-animation"
                && member.source_chunk_kind == "animation"
                && Path::new(&member.path).file_name() == Some(file_name)
        })
        .collect::<Vec<_>>();
    let [member] = matches.as_slice() else {
        return Err(PipelineError::new(
            "vehicle effect animation has ambiguous package membership",
        ));
    };
    member.source_chunk_ordinal.ok_or_else(|| {
        PipelineError::new("vehicle effect animation has no source ordinal")
    })
}

/// Resolve one supported effect animation-to-controller-to-target relationship.
fn vehicle_effect_controller_relationship(
    package_root: &Path,
    animation_identity: &str,
    animation_type: &str,
) -> Result<Option<EffectControllerRecord>, PipelineError> {
    let spec = if animation_type.eq_ignore_ascii_case("BQG_") {
        Some(("BQG", "quad_group"))
    } else if animation_type.eq_ignore_ascii_case("TEX_") {
        Some(("TEX", "shader"))
    } else {
        None
    };
    let Some((expected_controller_type, expected_target_kind)) = spec else {
        return Ok(None);
    };
    let ledger_path = package_root.join("components.jsonl");
    let ledger = fs::read_to_string(&ledger_path)
        .map_err(|error| PipelineError::new(error.to_string()))?;
    let rows = ledger
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            serde_json::from_str::<Value>(line)
                .map_err(|error| PipelineError::new(error.to_string()))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let mut controllers = Vec::new();
    for row in rows.iter().filter(|row| {
        matches!(
            row.get("kind").and_then(Value::as_str),
            Some(
                "frame_controller"
                    | "frame_controller_variant_a"
                    | "frame_controller_variant_b"
            )
        )
    }) {
        let controller = read_vehicle_ledger_member(package_root, row)?;
        let declared_animation = vehicle_component_identity(
            &controller,
            "animation_name",
            "controller animation",
        )?;
        if declared_animation == animation_identity {
            controllers.push((row, controller));
        }
    }
    let [(controller_row, controller)] = controllers.as_slice() else {
        return Err(PipelineError::new(format!(
            "vehicle {animation_type} animation has ambiguous controller \
             relationship: {animation_identity}"
        )));
    };
    if controller.get("schema").and_then(Value::as_str)
        != Some("frame_controller")
    {
        return Err(PipelineError::new(
            "vehicle effect controller schema is not frame_controller",
        ));
    }
    let controller_identity =
        vehicle_component_identity(controller, "name", "controller")?;
    let controller_type =
        vehicle_component_identity(controller, "type", "controller type")?;
    if controller_type != expected_controller_type {
        return Err(PipelineError::new(format!(
            "vehicle {animation_type} animation uses unexpected controller \
             type: {controller_type}"
        )));
    }
    let controller_version = vehicle_component_usize(
        controller,
        "version",
        "effect controller version",
    )?;
    let frame_offset_value = controller
        .get("frame_offset")
        .cloned()
        .ok_or_else(|| {
            PipelineError::new("vehicle effect controller has no frame offset")
        })?;
    let frame_offset: f32 = serde_json::from_value(frame_offset_value)
        .map_err(|error| {
            PipelineError::new(format!(
                "vehicle effect controller frame offset is invalid: {error}"
            ))
        })?;
    if !frame_offset.is_finite() {
        return Err(PipelineError::new(
            "vehicle effect controller frame offset is not finite",
        ));
    }
    let target_identity = vehicle_component_identity(
        controller,
        "hierarchy_name",
        "controller hierarchy",
    )?;
    let controller_source_ordinal = vehicle_ledger_ordinal(controller_row)?;
    let mut targets = Vec::new();
    for row in rows.iter().filter(|row| {
        row.get("kind").and_then(Value::as_str) == Some(expected_target_kind)
    }) {
        let target = read_vehicle_ledger_member(package_root, row)?;
        let Some(raw_identity) = target.get("name").and_then(Value::as_str)
        else {
            continue;
        };
        let identity =
            vehicle_source_identity(raw_identity, "controller target")?;
        if identity == target_identity {
            targets.push(row);
        }
    }
    let [target_row] = targets.as_slice() else {
        return Err(PipelineError::new(format!(
            "vehicle {animation_type} controller has ambiguous target \
             relationship: {target_identity}"
        )));
    };
    let target_kind = target_row
        .get("kind")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            PipelineError::new("vehicle effect target ledger row has no kind")
        })?;
    if target_kind != expected_target_kind {
        return Err(PipelineError::new(format!(
            "vehicle {animation_type} controller target kind is {target_kind}, \
             expected {expected_target_kind}"
        )));
    }
    Ok(Some(EffectControllerRecord {
        controller_identity,
        controller_kind: controller_row
            .get("kind")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                PipelineError::new(
                    "vehicle effect controller ledger row has no kind",
                )
            })?
            .to_owned(),
        controller_source_ordinal,
        controller_version,
        controller_type,
        frame_offset_bits: frame_offset.to_bits(),
        target_kind: target_kind.to_owned(),
        target_identity,
        target_source_ordinal: vehicle_ledger_ordinal(target_row)?,
    }))
}

/// Read one exact normalized component addressed by a ledger row.
fn read_vehicle_ledger_member(
    package_root: &Path,
    row: &Value,
) -> Result<Value, PipelineError> {
    let relative = row.get("path").and_then(Value::as_str).ok_or_else(|| {
        PipelineError::new("vehicle component ledger row has no path")
    })?;
    let bytes = fs::read(package_root.join("components").join(relative))
        .map_err(|error| PipelineError::new(error.to_string()))?;
    serde_json::from_slice(&bytes)
        .map_err(|error| PipelineError::new(error.to_string()))
}

/// Read one canonical decoded component identity without repairing source text.
fn vehicle_component_identity(
    value: &Value,
    field: &str,
    label: &str,
) -> Result<String, PipelineError> {
    let raw = value.get(field).and_then(Value::as_str).ok_or_else(|| {
        PipelineError::new(format!("vehicle {label} has no source identity"))
    })?;
    vehicle_source_identity(raw, label)
}

/// Validate one decoded source identity while removing only trailing NUL
/// padding.
fn vehicle_source_identity(
    raw: &str,
    label: &str,
) -> Result<String, PipelineError> {
    let clean = raw.trim_end_matches('\u{0}');
    if clean.is_empty()
        || clean != clean.trim()
        || clean.chars().any(char::is_control)
    {
        return Err(PipelineError::new(format!(
            "vehicle {label} identity is non-canonical"
        )));
    }
    Ok(clean.to_owned())
}

/// Read one unsigned source field without narrowing silently.
fn vehicle_component_usize(
    value: &Value,
    field: &str,
    label: &str,
) -> Result<usize, PipelineError> {
    let raw = value.get(field).and_then(Value::as_u64).ok_or_else(|| {
        PipelineError::new(format!("vehicle {label} is missing or invalid"))
    })?;
    usize::try_from(raw).map_err(|error| PipelineError::new(error.to_string()))
}

/// Read one source ordinal from a normalized component ledger row.
fn vehicle_ledger_ordinal(row: &Value) -> Result<usize, PipelineError> {
    let ordinal = row.get("ordinal").and_then(Value::as_u64).ok_or_else(|| {
        PipelineError::new("vehicle component ledger row has no source ordinal")
    })?;
    usize::try_from(ordinal)
        .map_err(|error| PipelineError::new(error.to_string()))
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
        .map(str::to_ascii_lowercase)
        .collect::<BTreeSet<_>>();
    let source_dir = package_root.join("components").join("texture");
    for source in png_files(&source_dir)? {
        let file_name = source
            .file_name()
            .and_then(|value| value.to_str())
            .ok_or_else(|| {
                PipelineError::new("vehicle texture has no UTF-8 name")
            })?;
        if referenced.contains(&file_name.to_ascii_lowercase()) {
            continue;
        }
        let role = texture_state_role(file_name);
        let destination_dir = texture_dir.join(role);
        fs::create_dir_all(&destination_dir)
            .map_err(|error| PipelineError::new(error.to_string()))?;
        let _copied_bytes = fs::copy(&source, destination_dir.join(file_name))
            .map_err(|error| {
                PipelineError::new(format!(
                    "vehicle alternate texture copy failed for {}: {error}",
                    source.display()
                ))
            })?;
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
        &fs::read(&source)
            .map_err(|error| PipelineError::new(error.to_string()))?,
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
        .map(|entry| {
            entry
                .map(|value| value.path())
                .map_err(|error| PipelineError::new(error.to_string()))
        })
        .collect::<Result<Vec<_>, _>>()?;
    matches.retain(|path| {
        path.is_file()
            && decoded_name(path)
                .is_ok_and(|name| name.eq_ignore_ascii_case(shader))
    });
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
fn texture_records(
    vehicle_dir: &Path,
) -> Result<Vec<TextureRecord>, PipelineError> {
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
            bytes: u64::try_from(bytes.len()).map_err(|error| {
                PipelineError::new(format!(
                    "vehicle texture size overflowed: {error}"
                ))
            })?,
            sha256: digest_hex(&bytes),
        });
    }
    records.sort_by(|left, right| left.path.cmp(&right.path));
    Ok(records)
}

/// Verify canonical external-texture binary FBX 7.7 output.
fn verify_binary_fbx(path: &Path) -> Result<(), PipelineError> {
    const MAGIC: &[u8] = b"Kaydara FBX Binary  \0\x1a\0";
    let bytes = fs::read(path)
        .map_err(|error| PipelineError::new(error.to_string()))?;
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
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/vehicle_catalog/prepare/tests.rs"]
mod tests;

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/vehicle_catalog/prepare/grounding_tests.rs"]
mod grounding_tests;

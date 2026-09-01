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
//   - World inventory outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - World inventory outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! World inventory outbound adapter.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use fbx::adapters::driven::decoded_billboard_animation_source::
    read_billboard_animation_source_evidence;
use fbx::adapters::driven::decoded_billboard_source::{
    BillboardQuadEvidence, read_billboard_source_evidence,
};
use fbx::adapters::driven::decoded_component_source;

use super::extraction::relative_art_root;
use super::inventory_common::{
    CompositeEvidence, clean_identity, ledger_member_id, read_component_name,
    read_composite, read_json, required_string, required_usize,
};
use super::model::{
    DeferredBillboardBinding, DeferredBillboardQuadBinding,
    DeferredControllerBinding, DeferredRenderBinding,
    DeferredShaderOccurrenceBinding, DeferredShaderParameterBinding,
    DeferredTextureOccurrenceBinding, DeferredTextureReferenceBinding,
    PropCandidate, PropFamily, PropRoute,
};
use super::texture_authority::SharedTextureAuthority;
use super::world_ledger::{LedgerRow, read_world_ledger};
use crate::domain::PipelineError;
use crate::domain::package::PhaseThreePackageIndex;

/// World containers whose nested mesh evidence belongs in the model catalog.
const MODEL_CONTAINERS: [&str; 7] = [
    "srr_dyna_phys_dsg",
    "srr_insta_anim_dyna_phys_dsg",
    "srr_breakable_object",
    "srr_anim_dsg",
    "srr_anim_coll_dsg",
    "state_prop",
    "animated_object_factory",
];

/// Discover every model-bearing terrain-world occurrence.
///
/// # Errors
///
/// Returns an error when ledger ownership or component associations are
/// ambiguous or malformed.
pub(super) fn discover_world_candidates(
    index: &PhaseThreePackageIndex,
    normalized_root: &Path,
    texture_authority: &SharedTextureAuthority,
) -> Result<Vec<PropCandidate>, PipelineError> {
    let mut candidates = Vec::new();
    for package in index
        .packages()
        .iter()
        .filter(|package| package.category == "terrain-world")
    {
        let relative = relative_art_root(package)?;
        let root = normalized_root.join(&relative);
        if !root.join("components.jsonl").is_file() {
            continue;
        }
        let ledger = read_world_ledger(&root)?;
        let package_relationship_rows = ledger
            .groups
            .values()
            .flatten()
            .filter(|row| {
                matches!(
                    row.kind.as_str(),
                    "quad_group"
                        | "frame_controller"
                        | "frame_controller_variant_a"
                        | "frame_controller_variant_b"
                        | "animation"
                        | "shader"
                )
            })
            .cloned()
            .collect::<Vec<_>>();
        for (container, rows) in ledger.groups {
            let Some(owner) = ledger.owners.get(&container) else {
                continue;
            };
            if !MODEL_CONTAINERS.contains(&owner.kind.as_str()) {
                continue;
            }
            let mesh_ids = source_ordered_mesh_ids(&rows)?;
            if mesh_ids.is_empty() {
                continue;
            }
            let mesh_names = decoded_mesh_names(&root, &mesh_ids)?;
            let association = associate_composite(
                &root,
                &rows,
                &package_relationship_rows,
                &mesh_names,
                texture_authority,
                &package.subcategory,
            )?;
            let (
                owner_name,
                selected,
                deferred_render_bindings,
                composite,
                skeleton,
                animation,
                route,
            ) = match association {
                Some(association) => association,
                None => static_association(owner, mesh_ids)?,
            };
            if selected.is_empty() {
                return Err(PipelineError::new(format!(
                    "world prop retained no model meshes: {} {}",
                    package.package_id, owner.name
                )));
            }
            candidates.push(PropCandidate {
                family: PropFamily::TerrainWorld,
                package_id: package.package_id.clone(),
                subcategory: package.subcategory.clone(),
                relative_root: relative.clone(),
                owner_kind: owner.kind.clone(),
                owner_name,
                container_key: container.to_string(),
                mesh_ids: selected,
                deferred_render_bindings,
                composite_id: composite,
                skeleton_id: skeleton,
                animation_id: animation,
                route,
            });
        }
    }
    candidates.sort();
    Ok(candidates)
}

/// Project one owner's mesh members through exact source component ordinals.
fn source_ordered_mesh_ids(
    rows: &[LedgerRow],
) -> Result<Vec<String>, PipelineError> {
    let mut meshes = rows
        .iter()
        .filter(|row| row.kind == "mesh")
        .collect::<Vec<_>>();
    meshes.sort_by_key(|row| row.ordinal);
    meshes
        .into_iter()
        .map(|row| ledger_member_id(&row.path, "mesh"))
        .collect()
}

/// Decode selected mesh names into member ids for composite matching.
fn decoded_mesh_names(
    root: &Path,
    mesh_ids: &[String],
) -> Result<BTreeMap<String, String>, PipelineError> {
    let mut names = BTreeMap::new();
    for member in mesh_ids {
        let path = root.join("components/mesh").join(format!("{member}.json"));
        let name = read_component_name(&path)?;
        if names.insert(name.clone(), member.clone()).is_some() {
            return Err(PipelineError::new(format!(
                "world prop repeats mesh identity {name}"
            )));
        }
    }
    Ok(names)
}

/// Associate one container with its exact composite, skeleton, and PTRN clip.
type Association = (
    String,
    Vec<String>,
    Vec<DeferredRenderBinding>,
    Option<String>,
    Option<String>,
    Option<String>,
    PropRoute,
);

/// Build the static fallback when no exact composite association exists.
fn static_association(
    owner: &LedgerRow,
    mesh_ids: Vec<String>,
) -> Result<Association, PipelineError> {
    Ok((
        clean_identity(&owner.name)?,
        mesh_ids,
        Vec::new(),
        None,
        None,
        None,
        PropRoute::Static,
    ))
}

/// Associate one world owner with its composite, skeleton, and model clip.
///
/// # Errors
///
/// Returns an error when member identities are ambiguous or malformed.
fn associate_composite(
    root: &Path,
    rows: &[LedgerRow],
    package_relationship_rows: &[LedgerRow],
    mesh_names: &BTreeMap<String, String>,
    texture_authority: &SharedTextureAuthority,
    source_subcategory: &str,
) -> Result<Option<Association>, PipelineError> {
    let mut matches = Vec::new();
    for row in rows.iter().filter(|row| row.kind == "composite_drawable") {
        let member = ledger_member_id(&row.path, "composite_drawable")?;
        let path = root
            .join("components/composite_drawable")
            .join(format!("{member}.json"));
        let composite = read_composite(&path)?;
        let selected = composite
            .prop_names
            .iter()
            .filter_map(|name| mesh_names.get(name))
            .cloned()
            .collect::<Vec<_>>();
        if !selected.is_empty() {
            matches.push((composite, selected));
        }
    }
    if matches.len() > 1 {
        return Err(PipelineError::new(
            "world prop container has multiple matching model composites",
        ));
    }
    let Some((composite, selected)) = matches.pop() else {
        return Ok(None);
    };
    let deferred_render_bindings = deferred_render_bindings(
        root,
        rows,
        package_relationship_rows,
        &composite,
        mesh_names,
        texture_authority,
        source_subcategory,
    )?;
    let skeleton =
        named_member(root, rows, "skeleton", &composite.skeleton_name)?;
    let clip_name = format!("PTRN_{}", composite.skeleton_name);
    let animation = named_member(root, rows, "animation", &clip_name)?;
    let animated = skeleton.is_some() && animation.is_some();
    Ok(Some((
        composite.name,
        selected,
        deferred_render_bindings,
        animated.then_some(composite.member_id),
        animated.then_some(skeleton).flatten(),
        animated.then_some(animation).flatten(),
        if animated {
            PropRoute::RigidAnimated
        } else {
            PropRoute::Static
        },
    )))
}

/// Retain authored non-mesh composite bindings without inventing FBX semantics.
fn deferred_render_bindings(
    root: &Path,
    rows: &[LedgerRow],
    package_relationship_rows: &[LedgerRow],
    composite: &CompositeEvidence,
    mesh_names: &BTreeMap<String, String>,
    texture_authority: &SharedTextureAuthority,
    source_subcategory: &str,
) -> Result<Vec<DeferredRenderBinding>, PipelineError> {
    let mut bindings = Vec::new();
    for (composite_prop_index, binding) in
        composite.prop_bindings.iter().enumerate()
    {
        if mesh_names.contains_key(&binding.name) {
            continue;
        }
        let mut matches = matching_quad_groups(rows, &binding.name)?;
        if matches.is_empty() {
            matches = matching_quad_groups(
                package_relationship_rows,
                &binding.name,
            )?;
        }
        if matches.len() > 1 {
            return Err(PipelineError::new(format!(
                "world prop repeats quad-group identity {}",
                binding.name
            )));
        }
        let resolved = matches.pop();
        let billboard = resolved
            .map(|row| {
                deferred_billboard_binding(
                    root,
                    package_relationship_rows,
                    row,
                    &binding.name,
                    texture_authority,
                    source_subcategory,
                )
            })
            .transpose()?;
        let (component_kind, component_member_id, source_ordinal) =
            if let Some(row) = resolved {
                (
                    Some(row.kind.clone()),
                    Some(ledger_member_id(&row.path, "quad_group")?),
                    Some(row.ordinal),
                )
            } else {
                (None, None, None)
            };
        let expected_animation_groups = billboard.as_ref().map(|billboard| {
            billboard
                .quads
                .iter()
                .map(|quad| quad.identity.as_str())
                .collect::<Vec<_>>()
        });
        let controller = deferred_controller_binding(
            root,
            package_relationship_rows,
            &binding.name,
            expected_animation_groups.as_deref(),
        )?;
        bindings.push(DeferredRenderBinding {
            composite_prop_index,
            source_identity: binding.name.clone(),
            skeleton_joint_id: binding.skeleton_joint_id,
            is_translucent: binding.is_translucent,
            component_kind,
            component_member_id,
            source_ordinal,
            billboard,
            controller,
        });
    }
    Ok(bindings)
}

/// Retain one exact source billboard group without interpreting presentation.
fn deferred_billboard_binding(
    root: &Path,
    package_relationship_rows: &[LedgerRow],
    row: &LedgerRow,
    expected_identity: &str,
    texture_authority: &SharedTextureAuthority,
    source_subcategory: &str,
) -> Result<DeferredBillboardBinding, PipelineError> {
    let path = root.join("components").join(&row.path);
    let evidence = read_billboard_source_evidence(&path, expected_identity)
        .map_err(|error| {
            PipelineError::new(format!(
                "world prop billboard evidence failed: {error:?}"
            ))
        })?;
    let shader_occurrences = deferred_shader_occurrences(
        root,
        package_relationship_rows,
        &evidence.shader_identity,
    )?;
    let texture_references = deferred_texture_references(
        &shader_occurrences,
        texture_authority,
        source_subcategory,
    )?;
    let quads = evidence
        .quads
        .iter()
        .map(deferred_billboard_quad_binding)
        .collect();
    Ok(DeferredBillboardBinding {
        version: evidence.version,
        shader_identity: evidence.shader_identity,
        shader_occurrences,
        texture_references,
        z_test: evidence.z_test,
        z_write: evidence.z_write,
        fog: evidence.fog,
        quads,
    })
}

/// Retain every same-name shader occurrence without choosing one relationship.
fn deferred_shader_occurrences(
    root: &Path,
    rows: &[LedgerRow],
    shader_identity: &str,
) -> Result<Vec<DeferredShaderOccurrenceBinding>, PipelineError> {
    let mut occurrences = Vec::new();
    for row in rows.iter().filter(|row| row.kind == "shader") {
        let row_identity = clean_identity(&row.name)?;
        if !row_identity.eq_ignore_ascii_case(shader_identity) {
            continue;
        }
        let path = root.join("components").join(&row.path);
        let evidence = decoded_component_source::read_shader_source_evidence(
            &path,
            shader_identity,
        )
            .map_err(|error| {
                PipelineError::new(format!(
                    "world prop billboard shader evidence failed: {error:?}"
                ))
            })?;
        occurrences.push(DeferredShaderOccurrenceBinding {
            member_id: ledger_member_id(&row.path, "shader")?,
            source_ordinal: row.ordinal,
            schema: evidence.schema,
            identity: evidence.identity,
            version: evidence.version,
            platform_shader_name: evidence.platform_shader_name,
            translucency: evidence.translucency,
            vertex_needs: evidence.vertex_needs,
            vertex_mask: evidence.vertex_mask,
            parameter_count: evidence.parameter_count,
            texture_reference: evidence.texture_reference,
            params: evidence
                .params
                .into_iter()
                .map(|parameter| DeferredShaderParameterBinding {
                    kind: parameter.kind,
                    param: parameter.param,
                    value: parameter.value,
                })
                .collect(),
        });
    }
    occurrences.sort_by_key(|occurrence| occurrence.source_ordinal);
    Ok(occurrences)
}

/// Retain every preferred physical source for each decoded shader texture
/// token.
fn deferred_texture_references(
    shader_occurrences: &[DeferredShaderOccurrenceBinding],
    texture_authority: &SharedTextureAuthority,
    source_subcategory: &str,
) -> Result<Vec<DeferredTextureReferenceBinding>, PipelineError> {
    let mut seen = BTreeSet::new();
    let mut references = Vec::new();
    for texture_reference in shader_occurrences
        .iter()
        .filter_map(|shader| shader.texture_reference.as_deref())
    {
        if !seen.insert(texture_reference.to_owned()) {
            continue;
        }
        let occurrences = texture_authority
            .preferred_occurrences(texture_reference, source_subcategory)?
            .into_iter()
            .map(|occurrence| DeferredTextureOccurrenceBinding {
                package_id: occurrence.package_id,
                subcategory: occurrence.subcategory,
                package_member_id: occurrence.package_member_id,
                member_id: occurrence.member_id,
                source_ordinal: occurrence.source_ordinal,
                sha256: occurrence.sha256,
            })
            .collect();
        references.push(DeferredTextureReferenceBinding {
            identity: texture_reference.to_owned(),
            occurrences,
        });
    }
    Ok(references)
}

/// Retain exact floating-point bits for one authored billboard child.
fn deferred_billboard_quad_binding(
    quad: &BillboardQuadEvidence,
) -> DeferredBillboardQuadBinding {
    DeferredBillboardQuadBinding {
        identity: quad.identity.clone(),
        version: quad.version,
        billboard_mode: quad.billboard_mode.clone(),
        translation_bits: quad.translation.map(f32::to_bits),
        colour: quad.colour,
        uv_bits: quad.uvs.map(|uv| uv.map(f32::to_bits)),
        width_bits: quad.width.to_bits(),
        height_bits: quad.height.to_bits(),
        distance_bits: quad.distance.to_bits(),
        uv_offset_bits: quad.uv_offset.map(f32::to_bits),
        rotation_wxyz_bits: quad.rotation_wxyz.map(f32::to_bits),
        cutoff_mode: quad.cutoff_mode.clone(),
        uv_offset_range_bits: quad.uv_offset_range.map(f32::to_bits),
        source_range_bits: quad.source_range.to_bits(),
        edge_range_bits: quad.edge_range.to_bits(),
        perspective: quad.perspective,
    }
}

/// Resolve one exact source controller and its declared animation relationship.
fn deferred_controller_binding(
    root: &Path,
    rows: &[LedgerRow],
    expected_hierarchy: &str,
    expected_animation_groups: Option<&[&str]>,
) -> Result<Option<DeferredControllerBinding>, PipelineError> {
    let mut controllers = Vec::new();
    for row in rows.iter().filter(|row| {
        matches!(
            row.kind.as_str(),
            "frame_controller"
                | "frame_controller_variant_a"
                | "frame_controller_variant_b"
        )
    }) {
        let value = read_json(&root.join("components").join(&row.path))?;
        let hierarchy =
            clean_identity(&required_string(&value, "hierarchy_name")?)?;
        if hierarchy == expected_hierarchy {
            controllers.push((row, value));
        }
    }
    if controllers.len() > 1 {
        return Err(PipelineError::new(format!(
            "world prop repeats frame-controller hierarchy {expected_hierarchy}"
        )));
    }
    let Some((row, value)) = controllers.pop() else {
        return Ok(None);
    };
    if required_string(&value, "schema")? != "frame_controller" {
        return Err(PipelineError::new(
            "world prop controller schema is not frame_controller",
        ));
    }
    let controller_identity =
        clean_identity(&required_string(&value, "name")?)?;
    let controller_type = clean_identity(&required_string(&value, "type")?)?;
    let animation_identity =
        clean_identity(&required_string(&value, "animation_name")?)?;
    let frame_offset_value = value
        .get("frame_offset")
        .cloned()
        .ok_or_else(|| {
            PipelineError::new("world prop controller has no frame offset")
        })?;
    let frame_offset: f32 = serde_json::from_value(frame_offset_value)
        .map_err(|error| {
            PipelineError::new(format!(
                "world prop controller frame offset is invalid: {error}"
            ))
        })?;
    if !frame_offset.is_finite() {
        return Err(PipelineError::new(
            "world prop controller frame offset is not finite",
        ));
    }
    let mut animations = Vec::new();
    for animation_row in rows.iter().filter(|row| row.kind == "animation") {
        let path = root.join("components").join(&animation_row.path);
        if read_component_name(&path)? == animation_identity {
            animations.push(animation_row);
        }
    }
    if animations.len() > 1 {
        return Err(PipelineError::new(format!(
            "world prop repeats controller animation {animation_identity}"
        )));
    }
    let (
        animation_member_id,
        animation_source_ordinal,
        animation_version,
        animation_type,
        animation_source,
    ) = if let Some(animation_row) = animations.pop() {
        let animation_path = root.join("components").join(&animation_row.path);
        let animation_value = read_json(&animation_path)?;
        let evidence = read_billboard_animation_source_evidence(
            &animation_path,
            &animation_identity,
        )
        .map_err(|error| {
            PipelineError::new(format!(
                "world prop BQG animation evidence failed: {error:?}"
            ))
        })?;
        if let Some(expected_groups) = expected_animation_groups {
            let actual_groups = evidence
                .group_lists
                .iter()
                .flat_map(|list| list.groups.iter())
                .map(|group| group.identity.as_str())
                .collect::<Vec<_>>();
            if actual_groups != expected_groups {
                return Err(PipelineError::new(format!(
                    "world prop BQG animation groups do not match billboard \
                     children: {animation_identity}"
                )));
            }
        }
        let source_value = serde_json::to_value(evidence).map_err(|error| {
            PipelineError::new(format!(
                "world prop BQG animation evidence JSON failed: {error}"
            ))
        })?;
        (
            Some(ledger_member_id(&animation_row.path, "animation")?),
            Some(animation_row.ordinal),
            Some(required_usize(&animation_value, "version")?),
            Some(clean_identity(&required_string(&animation_value, "type")?)?),
            Some(source_value),
        )
    } else {
        (None, None, None, None, None)
    };
    Ok(Some(DeferredControllerBinding {
        controller_identity,
        controller_kind: row.kind.clone(),
        controller_member_id: ledger_member_id(&row.path, &row.kind)?,
        controller_source_ordinal: row.ordinal,
        controller_version: required_usize(&value, "version")?,
        controller_type,
        frame_offset_bits: frame_offset.to_bits(),
        animation_identity,
        animation_member_id,
        animation_source_ordinal,
        animation_version,
        animation_type,
        animation_source,
    }))
}

/// Find every quad group matching one authored identity in one row scope.
fn matching_quad_groups<'rows>(
    rows: &'rows [LedgerRow],
    expected: &str,
) -> Result<Vec<&'rows LedgerRow>, PipelineError> {
    rows.iter()
        .filter(|row| row.kind == "quad_group")
        .filter_map(|row| match clean_identity(&row.name) {
            Ok(name) if name == expected => Some(Ok(row)),
            Ok(_) => None,
            Err(error) => Some(Err(error)),
        })
        .collect()
}

/// Find one same-container component by decoded identity.
fn named_member(
    root: &Path,
    rows: &[LedgerRow],
    family: &str,
    expected: &str,
) -> Result<Option<String>, PipelineError> {
    let mut matches = Vec::new();
    for row in rows.iter().filter(|row| row.kind == family) {
        let member = ledger_member_id(&row.path, family)?;
        let path = root
            .join("components")
            .join(family)
            .join(format!("{member}.json"));
        if read_component_name(&path)? == expected {
            matches.push(member);
        }
    }
    if matches.len() > 1 {
        return Err(PipelineError::new(format!(
            "world prop repeats {family} identity {expected}"
        )));
    }
    Ok(matches.pop())
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/prop_catalog/world_inventory/tests.rs"]
mod tests;

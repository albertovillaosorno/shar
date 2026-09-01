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
    PropCandidate, PropFamily, PropRoute, WorldPrimaryMemberBinding,
    WorldPrimaryMeshOrder, WorldPrimarySourceBinding,
};
use super::texture_authority::SharedTextureAuthority;
use super::world_ledger::{LedgerRow, read_world_ledger};
use crate::domain::PipelineError;
use crate::domain::package::{
    PackageRole, PhaseThreePackageIndex, PhaseThreePackageMember,
    PhaseThreePackageRow,
};

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
        let source_members = phase_three_source_members(package)?;
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
        let deferred_authority = DeferredRenderAuthority {
            source_members: &source_members,
            texture_authority,
            source_subcategory: &package.subcategory,
        };
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
                owner,
                &rows,
                &package_relationship_rows,
                &mesh_names,
                &deferred_authority,
            )?;
            let (
                owner_name,
                selected,
                deferred_render_bindings,
                composite,
                skeleton,
                animation,
                world_primary_source,
                route,
            ) = match association {
                Some(association) => association,
                None => static_association(
                    owner,
                    &rows,
                    mesh_ids,
                    &source_members,
                )?,
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
                world_primary_source: Some(world_primary_source),
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

/// Map exact normalized source coordinates to validated phase-three members.
fn phase_three_source_members(
    package: &PhaseThreePackageRow,
) -> Result<BTreeMap<(String, usize), PhaseThreePackageMember>, PipelineError> {
    let prefix = format!("{}/components/", package.package_root);
    let mut members = BTreeMap::new();
    for member in package
        .members()
        .iter()
        .filter(|member| member.source_chunk_ordinal.is_some())
    {
        let relative = member.path.strip_prefix(&prefix).ok_or_else(|| {
            PipelineError::new(
                "world source package member left its component root",
            )
        })?;
        let ordinal = member.source_chunk_ordinal.ok_or_else(|| {
            PipelineError::new(
                "world source package member has no source ordinal",
            )
        })?;
        members.extend([((relative.to_owned(), ordinal), member.clone())]);
    }
    Ok(members)
}

/// Resolve one ledger row to its exact classified phase-three member.
fn phase_three_source_member_id(
    members: &BTreeMap<(String, usize), PhaseThreePackageMember>,
    row: &LedgerRow,
    role: PackageRole,
    kind: &str,
) -> Result<String, PipelineError> {
    members
        .get(&(row.path.clone(), row.ordinal))
        .filter(|member| {
            member.role == role
                && member.kind == kind
                && member.source_chunk_kind == row.kind
        })
        .map(|member| member.id.clone())
        .ok_or_else(|| {
            PipelineError::new(format!(
                "world source occurrence has no phase-three member: {}@{}",
                row.path, row.ordinal
            ))
        })
}

/// Return the expected phase-three classification for one world owner.
fn primary_world_owner_classification(
    kind: &str,
) -> Result<(PackageRole, &'static str), PipelineError> {
    match kind {
        "srr_dyna_phys_dsg" | "srr_insta_anim_dyna_phys_dsg" => {
            Ok((PackageRole::Physics, "p3d-physics"))
        },
        "state_prop" | "animated_object_factory" => {
            Ok((PackageRole::World, "p3d-animated-prop"))
        },
        "srr_breakable_object" | "srr_anim_dsg" | "srr_anim_coll_dsg" => {
            Ok((PackageRole::World, "p3d-world-dsg"))
        },
        _ => Err(PipelineError::new(format!(
            "world primary source has unsupported owner kind {kind}"
        ))),
    }
}

/// Convert one classified ledger row into primary physical provenance.
fn primary_world_member_binding(
    row: &LedgerRow,
    family: &str,
    role: PackageRole,
    kind: &str,
    source_members: &BTreeMap<(String, usize), PhaseThreePackageMember>,
) -> Result<WorldPrimaryMemberBinding, PipelineError> {
    Ok(WorldPrimaryMemberBinding {
        package_member_id: phase_three_source_member_id(
            source_members,
            row,
            role,
            kind,
        )?,
        member_id: ledger_member_id(&row.path, family)?,
        source_ordinal: row.ordinal,
    })
}

/// Resolve one selected normalized member id back to its exact ledger row.
fn selected_world_member_row<'rows>(
    rows: &'rows [LedgerRow],
    family: &str,
    member_id: &str,
) -> Result<&'rows LedgerRow, PipelineError> {
    let expected_path = format!("{family}/{member_id}.json");
    rows.iter()
        .find(|row| row.kind == family && row.path == expected_path)
        .ok_or_else(|| {
            PipelineError::new(format!(
                "world selected {family} member has no ledger row: {member_id}"
            ))
        })
}

/// Route relationships retained beside primary world member provenance.
#[derive(Clone, Copy)]
struct WorldPrimaryRelationships<'rows> {
    mesh_order: WorldPrimaryMeshOrder,
    matched_composite: Option<&'rows LedgerRow>,
    referenced_skeleton: Option<&'rows LedgerRow>,
    exported_ptrn_animation: Option<&'rows LedgerRow>,
}

/// Build exact primary world provenance without changing route semantics.
fn primary_world_source_binding(
    owner: &LedgerRow,
    rows: &[LedgerRow],
    selected_mesh_ids: &[String],
    relationships: WorldPrimaryRelationships<'_>,
    source_members: &BTreeMap<(String, usize), PhaseThreePackageMember>,
) -> Result<WorldPrimarySourceBinding, PipelineError> {
    let (owner_role, owner_kind) =
        primary_world_owner_classification(&owner.kind)?;
    let owner_binding = primary_world_member_binding(
        owner,
        &owner.kind,
        owner_role,
        owner_kind,
        source_members,
    )?;
    let selected_meshes = selected_mesh_ids
        .iter()
        .map(|member_id| {
            let row = selected_world_member_row(rows, "mesh", member_id)?;
            primary_world_member_binding(
                row,
                "mesh",
                PackageRole::Model,
                "p3d-mesh",
                source_members,
            )
        })
        .collect::<Result<Vec<_>, PipelineError>>()?;
    let matched_composite = relationships.matched_composite
        .map(|row| {
            primary_world_member_binding(
                row,
                "composite_drawable",
                PackageRole::Model,
                "p3d-composite-drawable",
                source_members,
            )
        })
        .transpose()?;
    let referenced_skeleton = relationships.referenced_skeleton
        .map(|row| {
            primary_world_member_binding(
                row,
                "skeleton",
                PackageRole::Animation,
                "p3d-skeleton",
                source_members,
            )
        })
        .transpose()?;
    let exported_ptrn_animation = relationships.exported_ptrn_animation
        .map(|row| {
            primary_world_member_binding(
                row,
                "animation",
                PackageRole::Animation,
                "p3d-animation",
                source_members,
            )
        })
        .transpose()?;
    Ok(WorldPrimarySourceBinding {
        owner: owner_binding,
        mesh_order: relationships.mesh_order,
        selected_meshes,
        matched_composite,
        referenced_skeleton,
        exported_ptrn_animation,
    })
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

/// Source-backed authorities needed while retaining deferred render evidence.
struct DeferredRenderAuthority<'a> {
    source_members: &'a BTreeMap<(String, usize), PhaseThreePackageMember>,
    texture_authority: &'a SharedTextureAuthority,
    source_subcategory: &'a str,
}

/// Associate one container with its exact composite, skeleton, and PTRN clip.
type Association = (
    String,
    Vec<String>,
    Vec<DeferredRenderBinding>,
    Option<String>,
    Option<String>,
    Option<String>,
    WorldPrimarySourceBinding,
    PropRoute,
);

/// Build the static fallback when no exact composite association exists.
fn static_association(
    owner: &LedgerRow,
    rows: &[LedgerRow],
    mesh_ids: Vec<String>,
    source_members: &BTreeMap<(String, usize), PhaseThreePackageMember>,
) -> Result<Association, PipelineError> {
    let primary = primary_world_source_binding(
        owner,
        rows,
        &mesh_ids,
        WorldPrimaryRelationships {
            mesh_order: WorldPrimaryMeshOrder::SourceOrdinal,
            matched_composite: None,
            referenced_skeleton: None,
            exported_ptrn_animation: None,
        },
        source_members,
    )?;
    Ok((
        clean_identity(&owner.name)?,
        mesh_ids,
        Vec::new(),
        None,
        None,
        None,
        primary,
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
    owner: &LedgerRow,
    rows: &[LedgerRow],
    package_relationship_rows: &[LedgerRow],
    mesh_names: &BTreeMap<String, String>,
    authority: &DeferredRenderAuthority<'_>,
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
            matches.push((row, composite, selected));
        }
    }
    if matches.len() > 1 {
        return Err(PipelineError::new(
            "world prop container has multiple matching model composites",
        ));
    }
    let Some((composite_row, composite, selected)) = matches.pop() else {
        return Ok(None);
    };
    let deferred_render_bindings = deferred_render_bindings(
        root,
        rows,
        package_relationship_rows,
        &composite,
        mesh_names,
        authority,
    )?;
    let skeleton =
        named_member(root, rows, "skeleton", &composite.skeleton_name)?;
    let clip_name = format!("PTRN_{}", composite.skeleton_name);
    let animation = named_member(root, rows, "animation", &clip_name)?;
    let animated = skeleton.is_some() && animation.is_some();
    let primary = primary_world_source_binding(
        owner,
        rows,
        &selected,
        WorldPrimaryRelationships {
            mesh_order: WorldPrimaryMeshOrder::CompositeProp,
            matched_composite: Some(composite_row),
            referenced_skeleton: skeleton
                .as_ref()
                .map(|(row, _member)| *row),
            exported_ptrn_animation: animated
                .then(|| animation.as_ref().map(|(row, _member)| *row))
                .flatten(),
        },
        authority.source_members,
    )?;
    let skeleton_member = skeleton.map(|(_row, member)| member);
    let animation_member = animation.map(|(_row, member)| member);
    Ok(Some((
        composite.name,
        selected,
        deferred_render_bindings,
        animated.then_some(composite.member_id),
        animated.then_some(skeleton_member).flatten(),
        animated.then_some(animation_member).flatten(),
        primary,
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
    authority: &DeferredRenderAuthority<'_>,
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
                    authority,
                )
            })
            .transpose()?;
        let (
            component_kind,
            component_package_member_id,
            component_member_id,
            source_ordinal,
        ) = if let Some(row) = resolved {
            (
                Some(row.kind.clone()),
                Some(phase_three_source_member_id(
                    authority.source_members,
                    row,
                    PackageRole::Model,
                    "p3d-mesh",
                )?),
                Some(ledger_member_id(&row.path, "quad_group")?),
                Some(row.ordinal),
            )
        } else {
            (None, None, None, None)
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
            authority.source_members,
        )?;
        bindings.push(DeferredRenderBinding {
            composite_prop_index,
            source_identity: binding.name.clone(),
            skeleton_joint_id: binding.skeleton_joint_id,
            is_translucent: binding.is_translucent,
            component_kind,
            component_package_member_id,
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
    authority: &DeferredRenderAuthority<'_>,
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
        authority.source_members,
        &evidence.shader_identity,
    )?;
    let texture_references = deferred_texture_references(
        &shader_occurrences,
        authority.texture_authority,
        authority.source_subcategory,
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
    source_members: &BTreeMap<(String, usize), PhaseThreePackageMember>,
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
        let package_member_id = phase_three_source_member_id(
            source_members,
            row,
            PackageRole::Material,
            "p3d-shader",
        )?;
        occurrences.push(DeferredShaderOccurrenceBinding {
            package_member_id,
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
    source_members: &BTreeMap<(String, usize), PhaseThreePackageMember>,
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
        animation_package_member_id,
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
            Some(phase_three_source_member_id(
                source_members,
                animation_row,
                PackageRole::Animation,
                "p3d-animation",
            )?),
            Some(ledger_member_id(&animation_row.path, "animation")?),
            Some(animation_row.ordinal),
            Some(required_usize(&animation_value, "version")?),
            Some(clean_identity(&required_string(&animation_value, "type")?)?),
            Some(source_value),
        )
    } else {
        (None, None, None, None, None, None)
    };
    Ok(Some(DeferredControllerBinding {
        controller_identity,
        controller_kind: row.kind.clone(),
        controller_package_member_id: phase_three_source_member_id(
            source_members,
            row,
            PackageRole::Controller,
            "p3d-controller",
        )?,
        controller_member_id: ledger_member_id(&row.path, &row.kind)?,
        controller_source_ordinal: row.ordinal,
        controller_version: required_usize(&value, "version")?,
        controller_type,
        frame_offset_bits: frame_offset.to_bits(),
        animation_identity,
        animation_package_member_id,
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
fn named_member<'rows>(
    root: &Path,
    rows: &'rows [LedgerRow],
    family: &str,
    expected: &str,
) -> Result<Option<(&'rows LedgerRow, String)>, PipelineError> {
    let mut matches = Vec::new();
    for row in rows.iter().filter(|row| row.kind == family) {
        let member = ledger_member_id(&row.path, family)?;
        let path = root
            .join("components")
            .join(family)
            .join(format!("{member}.json"));
        if read_component_name(&path)? == expected {
            matches.push((row, member));
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

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

use std::collections::BTreeMap;
use std::path::Path;

use super::extraction::relative_art_root;
use super::inventory_common::{
    CompositeEvidence, clean_identity, ledger_member_id, read_component_name,
    read_composite,
};
use super::model::{
    DeferredRenderBinding, PropCandidate, PropFamily, PropRoute,
};
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
            let association = associate_composite(&root, &rows, &mesh_names)?;
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
    mesh_names: &BTreeMap<String, String>,
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
    let deferred_render_bindings =
        deferred_render_bindings(rows, &composite, mesh_names)?;
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
    rows: &[LedgerRow],
    composite: &CompositeEvidence,
    mesh_names: &BTreeMap<String, String>,
) -> Result<Vec<DeferredRenderBinding>, PipelineError> {
    let mut bindings = Vec::new();
    for (composite_prop_index, binding) in
        composite.prop_bindings.iter().enumerate()
    {
        if mesh_names.contains_key(&binding.name) {
            continue;
        }
        let mut matches = Vec::new();
        for row in rows.iter().filter(|row| row.kind == "quad_group") {
            if clean_identity(&row.name)? == binding.name {
                matches.push(row);
            }
        }
        if matches.len() > 1 {
            return Err(PipelineError::new(format!(
                "world prop repeats quad-group identity {}",
                binding.name
            )));
        }
        let resolved = matches.pop();
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
        bindings.push(DeferredRenderBinding {
            composite_prop_index,
            source_identity: binding.name.clone(),
            skeleton_joint_id: binding.skeleton_joint_id,
            is_translucent: binding.is_translucent,
            component_kind,
            component_member_id,
            source_ordinal,
        });
    }
    Ok(bindings)
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

// File:
//   - world_inventory.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/prop_catalog/world_inventory.rs
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
//   - Terrain-world dynamic, animated, and breakable model candidate discovery.
// - Must-Not:
//   - Export placement, collision, particles, sounds, or data-only containers.
// - Allows:
//   - Exact ledger-owner grouping and rigid composite/clip association.
// - Split-When:
//   - Static and animated world containers need independent policies.
// - Merge-When:
//   - Mission and world inventories share identical source semantics.
// - Summary:
//   - Converts nested world-model evidence into static or rigid FBX candidates.
// - Description:
//   - Uses container ordinals rather than name heuristics to group model parts.
// - Usage:
//   - Called after selected P3D packages are re-extracted.
// - Defaults:
//   - Missing exact PTRN evidence routes one model to static rest presentation.
//
// ADRs:
// - docs/adr/fbx/extraction/source-discovery-boundary.md
// - docs/adr/fbx/export/fbx-output-contract-boundary.md
//
// Large file:
//   - true
//   - Reason: Exact container, composite, skeleton, and clip association forms
//   - one ordered world-inventory transaction.
//

//! Terrain-world model-prop candidate discovery.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use super::extraction::relative_art_root;
use super::inventory_common::{
    clean_identity, ledger_member_id, read_component_name, read_composite,
};
use super::model::{PropCandidate, PropFamily, PropRoute};
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
        if !root
            .join("components.jsonl")
            .is_file()
        {
            continue;
        }
        let ledger = read_world_ledger(&root)?;
        for (container, rows) in ledger.groups {
            let Some(owner) = ledger
                .owners
                .get(&container)
            else {
                continue;
            };
            if !MODEL_CONTAINERS.contains(
                &owner
                    .kind
                    .as_str(),
            ) {
                continue;
            }
            let mut mesh_ids = rows
                .iter()
                .filter(|row| row.kind == "mesh")
                .map(
                    |row| {
                        ledger_member_id(
                            &row.path, "mesh",
                        )
                    },
                )
                .collect::<Result<Vec<_>, _>>()?;
            mesh_ids.sort();
            mesh_ids.dedup();
            if mesh_ids.is_empty() {
                continue;
            }
            let mesh_names = decoded_mesh_names(
                &root, &mesh_ids,
            )?;
            let association = associate_composite(
                &root,
                &rows,
                &mesh_names,
            )?;
            let (owner_name, selected, composite, skeleton, animation, route) =
                association.unwrap_or_else(
                    || {
                        static_association(
                            owner, mesh_ids,
                        )
                    },
                );
            if selected.is_empty() {
                return Err(
                    PipelineError::new(
                        format!(
                            "world prop retained no model meshes: {} {}",
                            package.package_id, owner.name
                        ),
                    ),
                );
            }
            candidates.push(
                PropCandidate {
                    family: PropFamily::TerrainWorld,
                    package_id: package
                        .package_id
                        .clone(),
                    subcategory: package
                        .subcategory
                        .clone(),
                    relative_root: relative.clone(),
                    owner_kind: owner
                        .kind
                        .clone(),
                    owner_name,
                    container_key: container.to_string(),
                    mesh_ids: selected,
                    composite_id: composite,
                    skeleton_id: skeleton,
                    animation_id: animation,
                    route,
                },
            );
        }
    }
    candidates.sort();
    Ok(candidates)
}

/// Decode selected mesh names into member ids for composite matching.
fn decoded_mesh_names(
    root: &Path,
    mesh_ids: &[String],
) -> Result<BTreeMap<String, String>, PipelineError> {
    mesh_ids
        .iter()
        .map(
            |member| {
                let path = root
                    .join("components/mesh")
                    .join(format!("{member}.json"));
                Ok(
                    (
                        read_component_name(&path)?,
                        member.clone(),
                    ),
                )
            },
        )
        .collect()
}

/// Associate one container with its exact composite, skeleton, and PTRN clip.
type Association = (
    String,
    Vec<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    PropRoute,
);

/// Build the static fallback when no exact composite association exists.
fn static_association(
    owner: &LedgerRow,
    mesh_ids: Vec<String>,
) -> Association {
    (
        clean_identity(&owner.name),
        mesh_ids,
        None,
        None,
        None,
        PropRoute::Static,
    )
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
    for row in rows
        .iter()
        .filter(|row| row.kind == "composite_drawable")
    {
        let member = ledger_member_id(
            &row.path,
            "composite_drawable",
        )?;
        let path = root
            .join("components/composite_drawable")
            .join(format!("{member}.json"));
        let composite = read_composite(&path)?;
        let selected = composite
            .prop_names
            .iter()
            .filter_map(|name| mesh_names.get(name))
            .cloned()
            .collect::<BTreeSet<_>>();
        if !selected.is_empty() {
            matches.push(
                (
                    composite, selected,
                ),
            );
        }
    }
    if matches.len() > 1 {
        return Err(
            PipelineError::new(
                "world prop container has multiple matching model composites",
            ),
        );
    }
    let Some((composite, selected)) = matches.pop() else {
        return Ok(None);
    };
    let skeleton = named_member(
        root,
        rows,
        "skeleton",
        &composite.skeleton_name,
    )?;
    let clip_name = format!(
        "PTRN_{}",
        composite.skeleton_name
    );
    let animation = named_member(
        root,
        rows,
        "animation",
        &clip_name,
    )?;
    let animated = skeleton.is_some() && animation.is_some();
    Ok(
        Some(
            (
                composite.name,
                selected
                    .into_iter()
                    .collect(),
                animated.then_some(composite.member_id),
                animated
                    .then_some(skeleton)
                    .flatten(),
                animated
                    .then_some(animation)
                    .flatten(),
                if animated {
                    PropRoute::RigidAnimated
                } else {
                    PropRoute::Static
                },
            ),
        ),
    )
}

/// Find one same-container component by decoded identity.
fn named_member(
    root: &Path,
    rows: &[LedgerRow],
    family: &str,
    expected: &str,
) -> Result<Option<String>, PipelineError> {
    let mut matches = Vec::new();
    for row in rows
        .iter()
        .filter(|row| row.kind == family)
    {
        let member = ledger_member_id(
            &row.path, family,
        )?;
        let path = root
            .join("components")
            .join(family)
            .join(format!("{member}.json"));
        if read_component_name(&path)? == expected {
            matches.push(member);
        }
    }
    if matches.len() > 1 {
        return Err(
            PipelineError::new(
                format!("world prop repeats {family} identity {expected}"),
            ),
        );
    }
    Ok(matches.pop())
}

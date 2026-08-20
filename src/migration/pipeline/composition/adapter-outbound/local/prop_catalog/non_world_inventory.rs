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
//   - Non world inventory outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Non world inventory outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Non world inventory outbound adapter.

use std::collections::BTreeSet;
use std::path::Path;

use super::extraction::{is_selected_package, relative_art_root};
use super::inventory_common::{
    component_name_map, component_paths, read_composite,
};
use super::model::{PropCandidate, PropFamily, PropRoute};
use crate::domain::PipelineError;
use crate::domain::package::{PhaseThreePackageIndex, PhaseThreePackageRow};

/// Discover every model-bearing card or mission occurrence.
///
/// # Errors
///
/// Returns an error when component identities are ambiguous or malformed.
pub(super) fn discover_non_world_candidates(
    index: &PhaseThreePackageIndex,
    normalized_root: &Path,
) -> Result<Vec<PropCandidate>, PipelineError> {
    let mut candidates = Vec::new();
    for package in index
        .packages()
        .iter()
        .filter(|package| is_selected_package(package))
    {
        let relative = relative_art_root(package)?;
        let family = package_family(package)?;
        discover_package(
            package,
            family,
            &relative,
            &normalized_root.join(&relative),
            &mut candidates,
        )?;
    }
    candidates.sort();
    Ok(candidates)
}

/// Resolve one selected package to its publication family.
fn package_family(
    package: &PhaseThreePackageRow,
) -> Result<PropFamily, PipelineError> {
    match package.category.as_str() {
        "cards" => Ok(PropFamily::Cards),
        "missions" => Ok(PropFamily::Missions),
        category => Err(PipelineError::new(format!(
            "unsupported non-world prop category: {category}"
        ))),
    }
}

/// Discover one normalized card or mission package.
fn discover_package(
    package: &PhaseThreePackageRow,
    family: PropFamily,
    relative_root: &Path,
    root: &Path,
    output: &mut Vec<PropCandidate>,
) -> Result<(), PipelineError> {
    let meshes = component_name_map(root, "mesh")?;
    let skeletons = component_name_map(root, "skeleton")?;
    let animations = component_name_map(root, "animation")?;
    let mut referenced = BTreeSet::new();
    for composite_path in component_paths(root, "composite_drawable")? {
        let composite = read_composite(&composite_path)?;
        let selected = composite
            .prop_names
            .iter()
            .filter_map(|name| meshes.get(name))
            .cloned()
            .collect::<Vec<_>>();
        if selected.is_empty() {
            continue;
        }
        referenced.extend(selected.iter().cloned());
        if !is_publishable_composite(family, &composite.name) {
            continue;
        }
        let skeleton = skeletons.get(&composite.skeleton_name).cloned();
        let clip_name = format!("PTRN_{}", composite.skeleton_name);
        let animation = animations.get(&clip_name).cloned();
        let animated = skeleton.is_some() && animation.is_some();
        output.push(PropCandidate {
            family,
            package_id: package.package_id.clone(),
            subcategory: package.subcategory.clone(),
            relative_root: relative_root.to_path_buf(),
            owner_kind: "composite_drawable".to_owned(),
            owner_name: composite.name,
            container_key: composite.member_id.clone(),
            mesh_ids: selected,
            composite_id: animated.then_some(composite.member_id),
            skeleton_id: animated.then_some(skeleton).flatten(),
            animation_id: animated.then_some(animation).flatten(),
            route: if animated {
                PropRoute::RigidAnimated
            } else {
                PropRoute::Static
            },
        });
    }
    append_standalone_meshes(
        package,
        family,
        relative_root,
        meshes,
        &referenced,
        output,
    );
    Ok(())
}

/// Decide whether one composite belongs in the public family catalog.
fn is_publishable_composite(family: PropFamily, name: &str) -> bool {
    family != PropFamily::Cards
        || name.starts_with("card_")
        || name == "phone_icon"
}

/// Add real non-world prop meshes that are not owned by a composite.
fn append_standalone_meshes(
    package: &PhaseThreePackageRow,
    family: PropFamily,
    relative_root: &Path,
    meshes: std::collections::BTreeMap<String, String>,
    referenced: &BTreeSet<String>,
    output: &mut Vec<PropCandidate>,
) {
    if package.subcategory.ends_with("/models/level") {
        return;
    }
    for (mesh_name, mesh_id) in meshes {
        if referenced.contains(&mesh_id) {
            continue;
        }
        output.push(PropCandidate {
            family,
            package_id: package.package_id.clone(),
            subcategory: package.subcategory.clone(),
            relative_root: relative_root.to_path_buf(),
            owner_kind: "mesh".to_owned(),
            owner_name: mesh_name,
            container_key: mesh_id.clone(),
            mesh_ids: vec![mesh_id],
            composite_id: None,
            skeleton_id: None,
            animation_id: None,
            route: PropRoute::Static,
        });
    }
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/prop_catalog/non_world_inventory/tests.rs"]
mod tests;

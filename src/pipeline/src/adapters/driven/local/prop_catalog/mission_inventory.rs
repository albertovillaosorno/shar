// File:
//   - mission_inventory.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/prop_catalog/mission_inventory.rs
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
//   - Mission model-prop candidate discovery.
// - Must-Not:
//   - Export quad-only markers, shadows, cameras, particles, or level geometry.
// - Allows:
//   - Exact mesh-to-composite, skeleton, and PTRN association.
// - Split-When:
//   - Standalone and composite mission models need independent policy.
// - Merge-When:
//   - World and mission inventories share identical container semantics.
// - Summary:
//   - Finds race flags, finish lines, pickups, and other real mission models.
// - Description:
//   - Selects only composite props that resolve to decoded mesh components.
// - Usage:
//   - Called after selected P3D packages are re-extracted.
// - Defaults:
//   - Exact `PTRN_<skeleton>` is the only accepted authored model clip.
//
// ADRs:
// - docs/adr/fbx/extraction/source-discovery-boundary.md
// - docs/adr/fbx/export/fbx-output-contract-boundary.md
//
// Large file:
//   - false
//

//! Mission model-prop candidate discovery.

use std::collections::BTreeSet;
use std::path::Path;

use super::extraction::{is_selected_package, relative_art_root};
use super::inventory_common::{
    component_name_map, component_paths, read_composite,
};
use super::model::{PropCandidate, PropFamily, PropRoute};
use crate::domain::PipelineError;
use crate::domain::package::{PhaseThreePackageIndex, PhaseThreePackageRow};

/// Discover every model-bearing mission occurrence.
///
/// # Errors
///
/// Returns an error when component identities are ambiguous or malformed.
pub(super) fn discover_mission_candidates(
    index: &PhaseThreePackageIndex,
    normalized_root: &Path,
) -> Result<Vec<PropCandidate>, PipelineError> {
    let mut candidates = Vec::new();
    for package in index
        .packages()
        .iter()
        .filter(
            |package| {
                is_selected_package(package) && package.category == "missions"
            },
        )
    {
        let relative = relative_art_root(package)?;
        discover_package(
            package,
            &relative,
            &normalized_root.join(&relative),
            &mut candidates,
        )?;
    }
    candidates.sort();
    Ok(candidates)
}

/// Discover one normalized mission package.
fn discover_package(
    package: &PhaseThreePackageRow,
    relative_root: &Path,
    root: &Path,
    output: &mut Vec<PropCandidate>,
) -> Result<(), PipelineError> {
    let meshes = component_name_map(
        root, "mesh",
    )?;
    let skeletons = component_name_map(
        root, "skeleton",
    )?;
    let animations = component_name_map(
        root,
        "animation",
    )?;
    let mut referenced = BTreeSet::new();
    for composite_path in component_paths(
        root,
        "composite_drawable",
    )? {
        let composite = read_composite(&composite_path)?;
        let mut selected = composite
            .prop_names
            .iter()
            .filter_map(|name| meshes.get(name))
            .cloned()
            .collect::<Vec<_>>();
        selected.sort();
        selected.dedup();
        if selected.is_empty() {
            continue;
        }
        referenced.extend(
            selected
                .iter()
                .cloned(),
        );
        let skeleton = skeletons
            .get(&composite.skeleton_name)
            .cloned();
        let clip_name = format!(
            "PTRN_{}",
            composite.skeleton_name
        );
        let animation = animations
            .get(&clip_name)
            .cloned();
        let animated = skeleton.is_some() && animation.is_some();
        output.push(
            PropCandidate {
                family: PropFamily::Missions,
                package_id: package
                    .package_id
                    .clone(),
                subcategory: package
                    .subcategory
                    .clone(),
                relative_root: relative_root.to_path_buf(),
                owner_kind: "composite_drawable".to_owned(),
                owner_name: composite.name,
                container_key: composite
                    .member_id
                    .clone(),
                mesh_ids: selected,
                composite_id: animated.then_some(composite.member_id),
                skeleton_id: animated
                    .then_some(skeleton)
                    .flatten(),
                animation_id: animated
                    .then_some(animation)
                    .flatten(),
                route: if animated {
                    PropRoute::RigidAnimated
                } else {
                    PropRoute::Static
                },
            },
        );
    }
    append_standalone_meshes(
        package,
        relative_root,
        meshes,
        &referenced,
        output,
    );
    Ok(())
}

/// Add real mission meshes that are not owned by a composite.
fn append_standalone_meshes(
    package: &PhaseThreePackageRow,
    relative_root: &Path,
    meshes: std::collections::BTreeMap<String, String>,
    referenced: &BTreeSet<String>,
    output: &mut Vec<PropCandidate>,
) {
    if package
        .subcategory
        .ends_with("/models/level")
    {
        return;
    }
    for (mesh_name, mesh_id) in meshes {
        if referenced.contains(&mesh_id) {
            continue;
        }
        output.push(
            PropCandidate {
                family: PropFamily::Missions,
                package_id: package
                    .package_id
                    .clone(),
                subcategory: package
                    .subcategory
                    .clone(),
                relative_root: relative_root.to_path_buf(),
                owner_kind: "mesh".to_owned(),
                owner_name: mesh_name,
                container_key: mesh_id.clone(),
                mesh_ids: vec![mesh_id],
                composite_id: None,
                skeleton_id: None,
                animation_id: None,
                route: PropRoute::Static,
            },
        );
    }
}

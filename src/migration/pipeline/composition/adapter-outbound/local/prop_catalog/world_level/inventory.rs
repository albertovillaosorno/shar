// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Inventory outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Inventory outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Inventory outbound adapter.

use std::collections::BTreeMap;
use std::path::Path;

use super::super::extraction::is_world_level_package;
use super::super::inventory_common::{clean_identity, ledger_member_id};
use super::super::world_ledger::read_world_ledger;
use super::transform::Matrix;
use crate::domain::PipelineError;
use crate::domain::package::{PhaseThreePackageIndex, PhaseThreePackageRow};

/// One nested mesh and its exact owning world container.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct LevelMeshSource {
    /// Source component ordinal inside the owning P3D package.
    pub(super) ordinal: usize,
    /// Normalized mesh component file stem.
    pub(super) member_id: String,
    /// Clean source mesh identity used for placement matching.
    pub(super) mesh_name: String,
    /// Clean top-level owner identity.
    pub(super) owner_name: String,
    /// Top-level normalized owner family.
    pub(super) owner_kind: String,
}

/// Source-backed downstream interaction role for one world model owner.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum WorldObjectRole {
    /// Ordinary static presentation with no proven interaction role.
    Static,
    /// Breakable object or authored tree owner.
    Breakable,
    /// Dynamic-physics or animated-collision object.
    Interactable,
}

impl WorldObjectRole {
    /// Stable suffix added to exported Blender object identities.
    #[must_use]
    pub(super) const fn suffix(self) -> Option<&'static str> {
        match self {
            Self::Static => None,
            Self::Breakable => Some("breakable"),
            Self::Interactable => Some("interactable"),
        }
    }
}

/// Resolve one mesh owner's interaction role from its exact container kind.
#[must_use]
pub(super) fn object_role(source: &LevelMeshSource) -> WorldObjectRole {
    match source.owner_kind.as_str() {
        "srr_breakable_object" | "srr_tree_dsg" => WorldObjectRole::Breakable,
        "srr_static_phys_dsg"
        | "srr_dyna_phys_dsg"
        | "srr_insta_anim_dyna_phys_dsg"
        | "srr_anim_coll_dsg"
        | "srr_insta_static_phys_dsg" => WorldObjectRole::Interactable,
        _ => WorldObjectRole::Static,
    }
}

/// Return every terrain-world package in deterministic import order.
pub(super) fn world_packages(
    index: &PhaseThreePackageIndex,
) -> Vec<&PhaseThreePackageRow> {
    let mut packages = index
        .packages()
        .iter()
        .filter(|package| is_world_level_package(package))
        .collect::<Vec<_>>();
    packages.sort_by(|left, right| {
        (&left.subcategory, &left.package_id)
            .cmp(&(&right.subcategory, &right.package_id))
    });
    packages
}

/// Return the independent source scope owning one world package.
pub(super) fn package_scope(
    package: &PhaseThreePackageRow,
) -> Result<String, PipelineError> {
    if let Some(rest) = package.subcategory.strip_prefix("terrain-world/level-")
    {
        let level = rest
            .get(0..2)
            .filter(|value| {
                value.chars().all(|character| character.is_ascii_digit())
            })
            .ok_or_else(|| {
                PipelineError::new(format!(
                    "world package has no two-digit level scope: {}",
                    package.subcategory
                ))
            })?;
        return Ok(format!("level-{level}"));
    }
    Err(PipelineError::new(format!(
        "world package has no supported import scope: {}",
        package.subcategory
    )))
}

/// Return whether one package is an explicitly owned interior.
#[must_use]
pub(super) fn is_interior(package: &PhaseThreePackageRow) -> bool {
    package.subcategory.contains("/interiors/")
}

/// Read every nested render mesh from one freshly normalized package.
///
/// # Errors
///
/// Returns an error when ledger ownership or component paths are malformed.
pub(super) fn package_meshes(
    root: &Path,
) -> Result<Vec<LevelMeshSource>, PipelineError> {
    let manifest = root.join("components.jsonl");
    if !manifest.is_file() {
        return Ok(Vec::new());
    }
    let ledger = read_world_ledger(root)?;
    let mut meshes = Vec::new();
    for (owner_ordinal, rows) in &ledger.groups {
        let Some(owner) = ledger.owners.get(owner_ordinal) else {
            continue;
        };
        for row in rows.iter().filter(|row| row.kind == "mesh") {
            let member_id = ledger_member_id(&row.path, "mesh")?;
            meshes.push(LevelMeshSource {
                ordinal: row.ordinal,
                member_id,
                mesh_name: clean_identity(&row.name),
                owner_name: clean_identity(&owner.name),
                owner_kind: owner.kind.clone(),
            });
        }
    }
    meshes.sort_by_key(|source| source.ordinal);
    Ok(meshes)
}

/// Resolve explicit authored placement matrices for one source mesh.
#[must_use]
pub(super) fn explicit_placements(
    source: &LevelMeshSource,
    placements: &BTreeMap<String, Vec<Matrix>>,
) -> Vec<Matrix> {
    placements
        .get(&source.mesh_name)
        .or_else(|| placements.get(&source.owner_name))
        .cloned()
        .unwrap_or_default()
}

/// Return whether one source mesh is authored directly in world space.
#[must_use]
pub(super) fn is_direct_world_mesh(source: &LevelMeshSource) -> bool {
    matches!(
        source.owner_kind.as_str(),
        "srr_entity_dsg" | "srr_static_phys_dsg"
    )
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/prop_catalog/world_level/inventory/tests.rs"]
mod tests;

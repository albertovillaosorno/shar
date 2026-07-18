// File:
//   - inventory.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/prop_catalog/world_level/
//     inventory.rs
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
//   - Level ownership and nested render-mesh inventory.
// - Must-Not:
//   - Load mesh payloads, apply transforms, or write artifacts.
// - Allows:
//   - Package grouping, ledger joins, and placement-route classification.
// - Summary:
//   - Maps normalized level packages to every recovered render mesh.
//
// ADRs:
// - docs/adr/pipeline/unreal/world-assembly-from-normalized-chunks.md
//
// Large file:
//   - false
//

//! Level ownership and recovered render-mesh inventory.

use std::collections::BTreeMap;
use std::path::Path;

use super::super::extraction::is_world_level_package;
use super::super::inventory_common::clean_identity;
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

/// Deterministic main-level package grouping.
pub(super) fn packages_by_level(
    index: &PhaseThreePackageIndex
) -> Result<BTreeMap<String, Vec<&PhaseThreePackageRow>>, PipelineError> {
    let mut levels = BTreeMap::<String, Vec<&PhaseThreePackageRow>>::new();
    for package in index
        .packages()
        .iter()
        .filter(|package| is_world_level_package(package))
    {
        let level = package_level(package)?;
        levels
            .entry(level)
            .or_default()
            .push(package);
    }
    if levels.len() != 7
        || levels
            .keys()
            .map(String::as_str)
            .ne(
                [
                    "01", "02", "03", "04", "05", "06", "07",
                ],
            )
    {
        return Err(
            PipelineError::new(
                "world level package index does not contain levels 01 through \
                 07",
            ),
        );
    }
    Ok(levels)
}

/// Return one package's explicit two-digit main-level identity.
/// Parse one package main-level identity.
fn package_level(
    package: &PhaseThreePackageRow
) -> Result<String, PipelineError> {
    let value = package
        .subcategory
        .strip_prefix("terrain-world/level-")
        .and_then(|rest| rest.get(0..2))
        .filter(
            |level| {
                level
                    .chars()
                    .all(|character| character.is_ascii_digit())
            },
        )
        .ok_or_else(
            || {
                PipelineError::new(
                    format!(
                        "world package has no main-level identity: {}",
                        package.subcategory
                    ),
                )
            },
        )?;
    Ok(value.to_owned())
}

/// Return whether one package is an explicitly owned interior.
#[must_use]
pub(super) fn is_interior(package: &PhaseThreePackageRow) -> bool {
    package
        .subcategory
        .contains("/interiors/")
}

/// Read every nested render mesh from one freshly normalized package.
///
/// # Errors
///
/// Returns an error when ledger ownership or component paths are malformed.
pub(super) fn package_meshes(
    root: &Path
) -> Result<Vec<LevelMeshSource>, PipelineError> {
    let manifest = root.join("components.jsonl");
    if !manifest.is_file() {
        return Ok(Vec::new());
    }
    let ledger = read_world_ledger(root)?;
    let mut meshes = Vec::new();
    for (owner_ordinal, rows) in &ledger.groups {
        let Some(owner) = ledger
            .owners
            .get(owner_ordinal)
        else {
            continue;
        };
        for row in rows
            .iter()
            .filter(|row| row.kind == "mesh")
        {
            let member_id = Path::new(&row.path)
                .file_stem()
                .and_then(|value| value.to_str())
                .ok_or_else(
                    || {
                        PipelineError::new(
                            format!(
                                "world level mesh path has no file stem: {}",
                                row.path
                            ),
                        )
                    },
                )?
                .to_owned();
            meshes.push(
                LevelMeshSource {
                    ordinal: row.ordinal,
                    member_id,
                    mesh_name: clean_identity(&row.name),
                    owner_name: clean_identity(&owner.name),
                    owner_kind: owner
                        .kind
                        .clone(),
                },
            );
        }
    }
    meshes.sort_by(
        |left, right| {
            (
                &left.owner_kind,
                &left.owner_name,
                left.ordinal,
            )
                .cmp(
                    &(
                        &right.owner_kind,
                        &right.owner_name,
                        right.ordinal,
                    ),
                )
        },
    );
    meshes.dedup();
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
        source
            .owner_kind
            .as_str(),
        "srr_entity_dsg" | "srr_static_phys_dsg"
    )
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::super::transform::identity;
    use super::{LevelMeshSource, explicit_placements, is_direct_world_mesh};

    fn source(kind: &str) -> LevelMeshSource {
        LevelMeshSource {
            ordinal: 1,
            member_id: "house".to_owned(),
            mesh_name: "house".to_owned(),
            owner_name: "house-owner".to_owned(),
            owner_kind: kind.to_owned(),
        }
    }

    #[test]
    fn explicit_placement_prefers_mesh_identity() {
        let mut placements = BTreeMap::new();
        let _previous = placements.insert(
            "house".to_owned(),
            vec![identity()],
        );
        assert_eq!(
            explicit_placements(
                &source("srr_insta_entity_dsg"),
                &placements,
            )
            .len(),
            1
        );
    }

    #[test]
    fn direct_entities_are_classified_without_invented_matrix() {
        assert!(is_direct_world_mesh(&source("srr_entity_dsg")));
        assert!(is_direct_world_mesh(&source("srr_static_phys_dsg")));
        assert!(
            explicit_placements(
                &source("srr_static_phys_dsg"),
                &BTreeMap::new(),
            )
            .is_empty()
        );
    }

    #[test]
    fn definition_only_meshes_are_not_direct_world_geometry() {
        assert!(!is_direct_world_mesh(&source("srr_breakable_object")));
    }
}

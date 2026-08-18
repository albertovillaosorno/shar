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
//   - Coordinate outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Coordinate outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Coordinate outbound adapter.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use fbx::adapters::driven::decoded_component_source::read_mesh_for_analysis;
use fbx::domain::mesh::MeshAsset;

use super::inventory::{
    LevelMeshSource, explicit_placements, is_direct_world_mesh, package_meshes,
};
use super::scenegraph::placement_map;
use super::transform::Matrix;
use crate::domain::PipelineError;

/// Spatial evidence resolved for one canonical package.
pub(super) struct PackageCoordinates {
    /// Explicit drawable placements from the canonical package.
    canonical_placements: BTreeMap<String, Vec<Matrix>>,
    /// Explicit drawable placements from the connected-map reference.
    reference_placements: BTreeMap<String, Vec<Matrix>>,
    /// Reference positions and normals keyed by canonical mesh ordinal.
    direct_reference_meshes: BTreeMap<usize, MeshAsset>,
    /// Whether the package used a connected-map P3D reference.
    pub(super) uses_reference: bool,
}

impl PackageCoordinates {
    /// Preserve one package exactly in its own source coordinate space.
    ///
    /// This policy intentionally exposes no placement matrices and no connected
    /// reference mesh. Base-game interiors use it because interior relocation
    /// belongs to optional mod behavior rather than faithful reconstruction.
    #[must_use]
    pub(super) const fn preserve_source() -> Self {
        Self {
            canonical_placements: BTreeMap::new(),
            reference_placements: BTreeMap::new(),
            direct_reference_meshes: BTreeMap::new(),
            uses_reference: false,
        }
    }

    /// Resolve coordinate evidence for one canonical package.
    ///
    /// # Errors
    ///
    /// Returns an error when placement documents or reference meshes are
    /// malformed, or canonical source and mesh inventories are misaligned.
    pub(super) fn resolve(
        canonical_sources: &[LevelMeshSource],
        canonical_meshes: &[MeshAsset],
        canonical_root: &Path,
        reference_root: Option<&Path>,
    ) -> Result<Self, PipelineError> {
        if canonical_sources.len() != canonical_meshes.len() {
            return Err(PipelineError::new(
                "world coordinate canonical source and mesh counts differ",
            ));
        }
        let canonical_placements = placement_map(canonical_root)?;
        let reference_placements = reference_root
            .map_or_else(|| Ok(BTreeMap::new()), placement_map)?;
        let direct_reference_meshes = reference_root.map_or_else(
            || Ok(BTreeMap::new()),
            |root| {
                match_direct_reference_meshes(
                    canonical_sources,
                    canonical_meshes,
                    root,
                )
            },
        )?;
        Ok(Self {
            canonical_placements,
            reference_placements,
            direct_reference_meshes,
            uses_reference: reference_root.is_some(),
        })
    }

    /// Resolve one source's explicit placement matrices and their authority.
    #[must_use]
    pub(super) fn placements(
        &self,
        source: &LevelMeshSource,
    ) -> (Vec<Matrix>, bool) {
        let reference = explicit_placements(source, &self.reference_placements);
        if !reference.is_empty() {
            return (reference, true);
        }
        (
            explicit_placements(source, &self.canonical_placements),
            false,
        )
    }

    /// Copy topology-compatible reference positions and normals into one
    /// canonical direct-world mesh.
    ///
    /// Returns `true` when reference coordinates were applied.
    ///
    /// # Errors
    ///
    /// Returns an error when a previously matched reference mesh no longer has
    /// the same canonical topology.
    pub(super) fn apply_direct_reference(
        &self,
        source: &LevelMeshSource,
        canonical: &mut MeshAsset,
    ) -> Result<bool, PipelineError> {
        let Some(reference) = self.direct_reference_meshes.get(&source.ordinal)
        else {
            return Ok(false);
        };
        transplant_coordinates(canonical, reference)?;
        Ok(true)
    }
}

/// Match direct-world canonical meshes to exact coordinate-reference identities.
fn match_direct_reference_meshes(
    canonical_sources: &[LevelMeshSource],
    canonical_meshes: &[MeshAsset],
    reference_root: &Path,
) -> Result<BTreeMap<usize, MeshAsset>, PipelineError> {
    let reference_sources = package_meshes(reference_root)?;
    let reference_meshes =
        load_reference_meshes(&reference_sources, reference_root);
    let mut used = BTreeSet::new();
    let mut matched = BTreeMap::new();
    for (canonical_source, canonical_mesh) in canonical_sources
        .iter()
        .zip(canonical_meshes)
        .filter(|(source, _mesh)| is_direct_world_mesh(source))
    {
        let Some(index) = exact_reference_match(
            canonical_source,
            canonical_mesh,
            &reference_meshes,
            &used,
        )? else {
            continue;
        };
        let _inserted = used.insert(index);
        let reference = reference_meshes
            .get(index)
            .ok_or_else(|| {
                PipelineError::new("world coordinate match index is missing")
            })?
            .1
            .clone();
        let _previous = matched.insert(canonical_source.ordinal, reference);
    }
    Ok(matched)
}

/// Select one exact owner/name/topology reference and reject ambiguity.
fn exact_reference_match(
    canonical_source: &LevelMeshSource,
    canonical_mesh: &MeshAsset,
    references: &[(LevelMeshSource, MeshAsset)],
    used: &BTreeSet<usize>,
) -> Result<Option<usize>, PipelineError> {
    let matches = references
        .iter()
        .enumerate()
        .filter(|(index, (source, mesh))| {
            !used.contains(index)
                && same_owner(canonical_source, source)
                && canonical_source.mesh_name == source.mesh_name
                && topology_matches(canonical_mesh, mesh)
        })
        .map(|(index, _)| index)
        .collect::<Vec<_>>();
    match matches.as_slice() {
        [] => Ok(None),
        [index] => Ok(Some(*index)),
        _ => Err(PipelineError::new(
            "world coordinate reference identity is ambiguous",
        )),
    }
}

/// Load parseable direct-world reference meshes in deterministic source order.
fn load_reference_meshes(
    sources: &[LevelMeshSource],
    root: &Path,
) -> Vec<(LevelMeshSource, MeshAsset)> {
    let mut recovered = Vec::new();
    for source in sources.iter().filter(|source| is_direct_world_mesh(source)) {
        let Ok((mesh, _discarded)) =
            read_mesh_for_analysis(root, &source.member_id)
        else {
            continue;
        };
        recovered.push((source.clone(), mesh));
    }
    recovered
}

/// Return whether two sources have the same exact spatial owner.
fn same_owner(left: &LevelMeshSource, right: &LevelMeshSource) -> bool {
    left.owner_kind == right.owner_kind && left.owner_name == right.owner_name
}

/// Return whether reference coordinates can be applied without changing
/// canonical topology or vertex-domain alignment.
fn topology_matches(canonical: &MeshAsset, reference: &MeshAsset) -> bool {
    canonical.groups.len() == reference.groups.len()
        && canonical.groups.iter().zip(&reference.groups).all(
            |(left, right)| {
                left.index == right.index
                    && left.positions.len() == right.positions.len()
                    && left.normals.len() == right.normals.len()
                    && left.triangles == right.triangles
            },
        )
}

/// Copy only spatial vertex channels from one topology-compatible reference.
fn transplant_coordinates(
    canonical: &mut MeshAsset,
    reference: &MeshAsset,
) -> Result<(), PipelineError> {
    if !topology_matches(canonical, reference) {
        return Err(PipelineError::new(
            "world coordinate reference topology differs from canonical \
                 mesh",
        ));
    }
    for (target, source) in canonical.groups.iter_mut().zip(&reference.groups) {
        target.positions.clone_from(&source.positions);
        target.normals.clone_from(&source.normals);
    }
    Ok(())
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/prop_catalog/world_level/coordinate/tests.rs"]
mod tests;

// File:
//   - coordinate.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/prop_catalog/world_level/
//     coordinate.rs
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
//   - Coordinate-only joins between canonical and connected-map packages.
// - Must-Not:
//   - Copy reference materials, UVs, colors, topology, or texture payloads.
// - Allows:
//   - Scenegraph placement lookup and topology-verified position
//     transplantation.
// - Summary:
//   - Applies connected-map spatial evidence to canonical game meshes.
//
// ADRs:
// - docs/adr/pipeline/unreal/world-assembly-from-normalized-chunks.md
// - docs/adr/fbx/export/fbx-output-contract-boundary.md
//
// Large file:
//   - false
//

//! Coordinate-only connected-map authority for assembled world levels.

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
            return Err(
                PipelineError::new(
                    "world coordinate canonical source and mesh counts differ",
                ),
            );
        }
        let canonical_placements = placement_map(canonical_root)?;
        let reference_placements = reference_root.map_or_else(
            || Ok(BTreeMap::new()),
            placement_map,
        )?;
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
        Ok(
            Self {
                canonical_placements,
                reference_placements,
                direct_reference_meshes,
                uses_reference: reference_root.is_some(),
            },
        )
    }

    /// Resolve one source's explicit placement matrices and their authority.
    #[must_use]
    pub(super) fn placements(
        &self,
        source: &LevelMeshSource,
    ) -> (
        Vec<Matrix>,
        bool,
    ) {
        let reference = explicit_placements(
            source,
            &self.reference_placements,
        );
        if !reference.is_empty() {
            return (
                reference, true,
            );
        }
        (
            explicit_placements(
                source,
                &self.canonical_placements,
            ),
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
        let Some(reference) = self
            .direct_reference_meshes
            .get(&source.ordinal)
        else {
            return Ok(false);
        };
        transplant_coordinates(
            canonical, reference,
        )?;
        Ok(true)
    }
}

/// Match direct-world canonical meshes to coordinate-only reference meshes.
fn match_direct_reference_meshes(
    canonical_sources: &[LevelMeshSource],
    canonical_meshes: &[MeshAsset],
    reference_root: &Path,
) -> Result<BTreeMap<usize, MeshAsset>, PipelineError> {
    let reference_sources = package_meshes(reference_root)?;
    let reference_meshes = load_reference_meshes(
        &reference_sources,
        reference_root,
    );
    let mut used = BTreeSet::new();
    let mut matched = BTreeMap::new();
    for (canonical_source, canonical_mesh) in canonical_sources
        .iter()
        .zip(canonical_meshes)
        .filter(|(source, _mesh)| is_direct_world_mesh(source))
    {
        let exact = reference_meshes
            .iter()
            .enumerate()
            .find(
                |(index, (source, mesh))| {
                    !used.contains(index)
                        && same_owner(
                            canonical_source,
                            source,
                        )
                        && canonical_source.mesh_name == source.mesh_name
                        && topology_matches(
                            canonical_mesh,
                            mesh,
                        )
                },
            )
            .map(|(index, _)| index);
        let selected = exact
            .or_else(
                || {
                    reference_meshes
                        .iter()
                        .enumerate()
                        .find(
                            |(index, (source, mesh))| {
                                !used.contains(index)
                                    && same_owner(
                                        canonical_source,
                                        source,
                                    )
                                    && topology_matches(
                                        canonical_mesh,
                                        mesh,
                                    )
                            },
                        )
                        .map(|(index, _)| index)
                },
            )
            .or_else(
                || {
                    unique_topology_match(
                        canonical_mesh,
                        canonical_sources,
                        canonical_meshes,
                        &reference_meshes,
                        &used,
                    )
                },
            );
        let Some(index) = selected else {
            continue;
        };
        let _inserted = used.insert(index);
        let reference = reference_meshes
            .get(index)
            .ok_or_else(
                || {
                    PipelineError::new(
                        "world coordinate match index is missing",
                    )
                },
            )?
            .1
            .clone();
        let _previous = matched.insert(
            canonical_source.ordinal,
            reference,
        );
    }
    Ok(matched)
}

/// Select one topology-only reference only when both sides are unambiguous.
fn unique_topology_match(
    canonical: &MeshAsset,
    canonical_sources: &[LevelMeshSource],
    canonical_meshes: &[MeshAsset],
    references: &[(
        LevelMeshSource,
        MeshAsset,
    )],
    used: &BTreeSet<usize>,
) -> Option<usize> {
    let canonical_matches = canonical_sources
        .iter()
        .zip(canonical_meshes)
        .filter(
            |(source, mesh)| {
                is_direct_world_mesh(source)
                    && topology_matches(
                        canonical, mesh,
                    )
            },
        )
        .count();
    if canonical_matches != 1 {
        return None;
    }
    let matches = references
        .iter()
        .enumerate()
        .filter(
            |(index, (_source, mesh))| {
                !used.contains(index)
                    && topology_matches(
                        canonical, mesh,
                    )
            },
        )
        .map(|(index, _)| index)
        .collect::<Vec<_>>();
    if matches.len() == 1 {
        matches
            .first()
            .copied()
    } else {
        None
    }
}

/// Load parseable direct-world reference meshes in deterministic source order.
fn load_reference_meshes(
    sources: &[LevelMeshSource],
    root: &Path,
) -> Vec<(
    LevelMeshSource,
    MeshAsset,
)> {
    let mut recovered = Vec::new();
    for source in sources
        .iter()
        .filter(|source| is_direct_world_mesh(source))
    {
        let Ok((mesh, _discarded)) = read_mesh_for_analysis(
            root,
            &source.member_id,
        ) else {
            continue;
        };
        recovered.push(
            (
                source.clone(),
                mesh,
            ),
        );
    }
    recovered
}

/// Return whether two sources have the same exact spatial owner.
fn same_owner(
    left: &LevelMeshSource,
    right: &LevelMeshSource,
) -> bool {
    left.owner_kind == right.owner_kind && left.owner_name == right.owner_name
}

/// Return whether reference coordinates can be applied without changing
/// canonical topology or vertex-domain alignment.
fn topology_matches(
    canonical: &MeshAsset,
    reference: &MeshAsset,
) -> bool {
    canonical
        .groups
        .len()
        == reference
            .groups
            .len()
        && canonical
            .groups
            .iter()
            .zip(&reference.groups)
            .all(
                |(left, right)| {
                    left.index == right.index
                        && left
                            .positions
                            .len()
                            == right
                                .positions
                                .len()
                        && left
                            .normals
                            .len()
                            == right
                                .normals
                                .len()
                        && left.triangles == right.triangles
                },
            )
}

/// Copy only spatial vertex channels from one topology-compatible reference.
fn transplant_coordinates(
    canonical: &mut MeshAsset,
    reference: &MeshAsset,
) -> Result<(), PipelineError> {
    if !topology_matches(
        canonical, reference,
    ) {
        return Err(
            PipelineError::new(
                "world coordinate reference topology differs from canonical \
                 mesh",
            ),
        );
    }
    for (target, source) in canonical
        .groups
        .iter_mut()
        .zip(&reference.groups)
    {
        target
            .positions
            .clone_from(&source.positions);
        target
            .normals
            .clone_from(&source.normals);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use fbx::domain::mesh::{MeshAsset, PrimitiveGroup};

    use super::{
        LevelMeshSource, topology_matches, transplant_coordinates,
        unique_topology_match,
    };

    fn mesh(
        shader: &str,
        offset: f32,
    ) -> Result<MeshAsset, String> {
        let group = PrimitiveGroup::new(
            0,
            shader,
            vec![
                [
                    offset, 0.0, 0.0,
                ],
                [
                    offset + 1.0,
                    0.0,
                    0.0,
                ],
                [
                    offset, 1.0, 0.0,
                ],
            ],
            vec![
                [
                    0.0, 0.0,
                ],
                [
                    1.0, 0.0,
                ],
                [
                    0.0, 1.0,
                ],
            ],
            &[
                0, 1, 2,
            ],
        )
        .map_err(|error| format!("group failed: {error:?}"))?
        .with_normals(
            vec![
                [
                    0.0, 0.0, 1.0,
                ],
                [
                    0.0, 0.0, 1.0,
                ],
                [
                    0.0, 0.0, 1.0,
                ],
            ],
        )
        .map_err(|error| format!("normals failed: {error:?}"))?;
        MeshAsset::new(
            "mesh",
            vec![group],
        )
        .map_err(|error| format!("mesh failed: {error:?}"))
    }

    type TestResult = Result<(), String>;

    #[test]
    fn coordinate_transplant_keeps_canonical_presentation() -> TestResult {
        let mut canonical = mesh(
            "canonical-material",
            0.0,
        )?;
        let reference = mesh(
            "reference-material",
            100.0,
        )?;
        if !topology_matches(
            &canonical, &reference,
        ) {
            return Err("compatible topology was rejected".to_owned());
        }
        transplant_coordinates(
            &mut canonical,
            &reference,
        )
        .map_err(|error| error.to_string())?;
        assert_eq!(
            canonical.groups[0].shader,
            "canonical-material"
        );
        assert_eq!(
            canonical.groups[0].uvs,
            vec![
                [
                    0.0, 0.0
                ],
                [
                    1.0, 0.0
                ],
                [
                    0.0, 1.0
                ]
            ]
        );
        assert_eq!(
            canonical.groups[0].positions[0],
            [
                100.0, 0.0, 0.0
            ]
        );
        Ok(())
    }

    #[test]
    fn topology_mismatch_blocks_coordinate_transplant() -> Result<(), String> {
        let canonical = mesh(
            "canonical-material",
            0.0,
        )?;
        let mut reference = mesh(
            "reference-material",
            10.0,
        )?;
        reference.groups[0].triangles[0] = [
            0, 2, 1,
        ];
        assert!(
            !topology_matches(
                &canonical, &reference,
            )
        );
        Ok(())
    }
    #[test]
    fn unique_topology_match_handles_zero_reference_candidates()
    -> Result<(), String> {
        let canonical = mesh(
            "canonical-material",
            0.0,
        )?;
        let source = LevelMeshSource {
            ordinal: 1,
            member_id: "mesh".to_owned(),
            mesh_name: "mesh".to_owned(),
            owner_name: "mesh".to_owned(),
            owner_kind: "srr_entity_dsg".to_owned(),
        };
        assert_eq!(
            unique_topology_match(
                &canonical,
                &[source],
                std::slice::from_ref(&canonical),
                &[],
                &BTreeSet::new(),
            ),
            None
        );
        Ok(())
    }
}

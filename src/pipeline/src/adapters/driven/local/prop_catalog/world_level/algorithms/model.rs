// File:
//   - model.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/prop_catalog/world_level/
//     algorithms/model.rs
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
//   - Per-FBX repair identities, structural fingerprints, and mutation function
//     contracts.
// - Must-Not:
//   - Read edited FBX evidence, infer algorithms at runtime, or write files.
// - Allows:
//   - Source-dependent deterministic mesh, UV, normal, color, topology, and
//     material corrections.
// - Summary:
//   - Defines the stable data contract for manually derived FBX repairs.
//
// ADRs:
// - docs/adr/pipeline/unreal/world-assembly-from-normalized-chunks.md
//
// Large file:
//   - false
//

//! Stable data contract for manually derived world-FBX repairs.

use fbx::domain::mesh::MeshAsset;

use crate::domain::PipelineError;

/// Total-count fingerprint resilient to Blender object ordering and metadata.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct FbxFingerprint {
    /// Mesh object count.
    pub(super) meshes: u64,
    /// Primitive-group count across every mesh.
    pub(super) groups: u64,
    /// Position count across every primitive group.
    pub(super) positions: u64,
    /// Triangle count across every primitive group.
    pub(super) triangles: u64,
    /// Primary UV count across every primitive group.
    pub(super) uvs: u64,
    /// Normal count across every primitive group.
    pub(super) normals: u64,
    /// Vertex-color count across every primitive group.
    pub(super) colors: u64,
}

impl FbxFingerprint {
    /// Build an order-independent structural fingerprint.
    ///
    /// # Errors
    ///
    /// Returns an error when any collection count cannot fit or aggregate.
    pub(super) fn from_meshes(
        meshes: &[MeshAsset]
    ) -> Result<Self, PipelineError> {
        let mut result = Self {
            meshes: count(
                meshes.len(),
                "mesh",
            )?,
            groups: 0,
            positions: 0,
            triangles: 0,
            uvs: 0,
            normals: 0,
            colors: 0,
        };
        for mesh in meshes {
            result.groups = add_count(
                result.groups,
                mesh.groups
                    .len(),
                "group",
            )?;
            for group in &mesh.groups {
                result.positions = add_count(
                    result.positions,
                    group
                        .positions
                        .len(),
                    "position",
                )?;
                result.triangles = add_count(
                    result.triangles,
                    group
                        .triangles
                        .len(),
                    "triangle",
                )?;
                result.uvs = add_count(
                    result.uvs,
                    group
                        .uvs
                        .len(),
                    "UV",
                )?;
                result.normals = add_count(
                    result.normals,
                    group
                        .normals
                        .len(),
                    "normal",
                )?;
                result.colors = add_count(
                    result.colors,
                    group
                        .colors
                        .len(),
                    "color",
                )?;
            }
        }
        Ok(result)
    }

    /// Return the seven comparison dimensions in stable order.
    pub(super) const fn dimensions(self) -> [u64; 7] {
        [
            self.meshes,
            self.groups,
            self.positions,
            self.triangles,
            self.uvs,
            self.normals,
            self.colors,
        ]
    }
}

/// Deterministic transformation authored from one original/edited FBX pair.
pub(super) type FbxRepairFunction =
    fn(&str, &mut [MeshAsset]) -> Result<(), PipelineError>;

/// One compiled per-FBX repair and its source-recognition evidence.
#[derive(Clone, Copy)]
pub(super) struct FbxRepairAlgorithm {
    /// Canonical generated relative path, using `/` separators.
    pub(super) relative_path: &'static str,
    /// Canonical normalized filename stem.
    pub(super) file_stem: &'static str,
    /// Optional normalized prefix used only when it resolves uniquely.
    pub(super) file_prefix: &'static str,
    /// Original generated-asset fingerprint captured when the repair was
    /// derived.
    pub(super) source_fingerprint: FbxFingerprint,
    /// Source-dependent transformation implementation.
    pub(super) apply: FbxRepairFunction,
}

/// Convert one collection length into the fingerprint counter domain.
fn count(
    value: usize,
    subject: &str,
) -> Result<u64, PipelineError> {
    u64::try_from(value).map_err(
        |error| {
            PipelineError::new(
                format!("FBX repair {subject} count overflowed: {error}"),
            )
        },
    )
}

/// Add one collection length to a fingerprint counter.
fn add_count(
    current: u64,
    value: usize,
    subject: &str,
) -> Result<u64, PipelineError> {
    current
        .checked_add(
            count(
                value, subject,
            )?,
        )
        .ok_or_else(
            || {
                PipelineError::new(
                    format!("FBX repair {subject} total overflowed"),
                )
            },
        )
}

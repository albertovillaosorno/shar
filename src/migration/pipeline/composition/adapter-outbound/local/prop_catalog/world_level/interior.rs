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
//   - Interior outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Interior outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Interior outbound adapter.

use std::collections::BTreeSet;

use fbx::domain::mesh::{MeshAsset, PrimitiveGroup};
use shar_sha256::Sha256;
#[cfg(test)]
use shar_sha256::digest_hex;

use crate::domain::PipelineError;

/// One stable source-backed interior family.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(super) struct InteriorIdentity {
    /// Stable source identifier such as `i01`.
    pub(super) id: &'static str,
    /// Portable semantic folder name.
    pub(super) name: &'static str,
    /// Whether Level 7 contributes an additive Halloween overlay.
    pub(super) halloween_overlay: bool,
}

/// Resolve one source package into its stable interior identity.
#[must_use]
pub(super) fn identity_for_package(
    package_id: &str,
) -> Option<InteriorIdentity> {
    let identity = match package_id {
        "extracted-art-l1i00"
        | "extracted-art-l4i00"
        | "extracted-art-l7i00" => InteriorIdentity {
            id: "i00",
            name: "elementary-school",
            halloween_overlay: true,
        },
        "extracted-art-l1i01"
        | "extracted-art-l4i01"
        | "extracted-art-l7i01" => InteriorIdentity {
            id: "i01",
            name: "kwik-e-mart",
            halloween_overlay: true,
        },
        "extracted-art-l1i02"
        | "extracted-art-l4i02"
        | "extracted-art-l7i02" => InteriorIdentity {
            id: "i02",
            name: "simpsons-house",
            halloween_overlay: true,
        },
        "extracted-art-l2i03" | "extracted-art-l5i03" => InteriorIdentity {
            id: "i03",
            name: "dmv",
            halloween_overlay: false,
        },
        "extracted-art-l2i04" | "extracted-art-l5i04" => InteriorIdentity {
            id: "i04",
            name: "moes-tavern",
            halloween_overlay: false,
        },
        "extracted-art-l3i05" | "extracted-art-l6i05" => InteriorIdentity {
            id: "i05",
            name: "androids-dungeon",
            halloween_overlay: false,
        },
        "extracted-art-l3i06" | "extracted-art-l6i06" => InteriorIdentity {
            id: "i06",
            name: "observatory",
            halloween_overlay: false,
        },
        "extracted-art-l4i07" | "extracted-art-l7i07" => InteriorIdentity {
            id: "i07",
            name: "barts-room",
            halloween_overlay: true,
        },
        _ => return None,
    };
    Some(identity)
}

/// Return the narrative level encoded by one interior package identity.
#[must_use]
pub(super) fn package_level(package_id: &str) -> Option<u8> {
    match package_id {
        "extracted-art-l1i00"
        | "extracted-art-l1i01"
        | "extracted-art-l1i02" => Some(1),
        "extracted-art-l2i03" | "extracted-art-l2i04" => Some(2),
        "extracted-art-l3i05" | "extracted-art-l3i06" => Some(3),
        "extracted-art-l4i00"
        | "extracted-art-l4i01"
        | "extracted-art-l4i02"
        | "extracted-art-l4i07" => Some(4),
        "extracted-art-l5i03" | "extracted-art-l5i04" => Some(5),
        "extracted-art-l6i05" | "extracted-art-l6i06" => Some(6),
        "extracted-art-l7i00"
        | "extracted-art-l7i01"
        | "extracted-art-l7i02"
        | "extracted-art-l7i07" => Some(7),
        _ => None,
    }
}

/// Return whether one package contributes only Level 7 Halloween additions.
#[must_use]
pub(super) fn is_halloween_package(package_id: &str) -> bool {
    matches!(
        package_id,
        "extracted-art-l7i00"
            | "extracted-art-l7i01"
            | "extracted-art-l7i02"
            | "extracted-art-l7i07"
    )
}

/// Quantized orientation-independent world-space triangle identity.
#[cfg(test)]
pub(super) type InteriorTriangleKey = [[i64; 3]; 3];

/// Exact source-authored corner identity used by fused-interior ownership.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct InteriorCornerIdentity {
    /// Exact source position bits.
    position: [u32; 3],
    /// Exact optional source UV bits.
    uv: Option<[u32; 2]>,
    /// Exact optional source normal bits.
    normal: Option<[u32; 3]>,
    /// Exact optional source color bits.
    color: Option<[u32; 4]>,
}

/// Exact source-authored face ownership for one fused interior identity.
#[derive(Debug, Default)]
pub(super) struct InteriorGeometryOwnership {
    /// Stable face digests already published by the fused interior.
    faces: BTreeSet<[u8; 32]>,
}

impl InteriorGeometryOwnership {
    /// Claim one triangle unless an exact source-authored face already owns it.
    fn claim(
        &mut self,
        mesh_name: &str,
        group: &PrimitiveGroup,
        triangle: &[u32; 3],
    ) -> Result<bool, PipelineError> {
        let fingerprint = source_face_fingerprint(mesh_name, group, triangle)?;
        Ok(self.faces.insert(fingerprint))
    }
}

/// Retain triangles unless an exact source-authored face was already published.
///
/// Ownership includes mesh identity, primitive-group identity, shader binding,
/// topology, positions, UVs, normals, and colors. Nearby geometry, alternate
/// triangulation, or distinct presentation is therefore preserved rather than
/// treated as a duplicate without source-backed proof of equivalence.
///
/// # Errors
///
/// Returns an error when one triangle references a missing vertex/channel or
/// the duplicate-triangle counter overflows.
#[cfg(test)]
pub(super) fn retain_unowned_triangles(
    mesh: MeshAsset,
    owned: &mut InteriorGeometryOwnership,
) -> Result<(Option<MeshAsset>, usize), PipelineError> {
    let ownership_mesh = mesh.clone();
    retain_unowned_triangles_with_ownership(mesh, &ownership_mesh, owned)
}

/// Retain source triangles using an exact ownership mesh.
///
/// The ownership mesh is cloned from the unmodified source-space package.
/// Fusion decisions therefore compare source-authored identity and channels,
/// while the aligned render mesh remains the publication payload.
///
/// # Errors
///
/// Returns an error when render and ownership topology diverge or one triangle
/// references a missing ownership vertex/channel.
pub(super) fn retain_unowned_triangles_with_ownership(
    mut mesh: MeshAsset,
    ownership_mesh: &MeshAsset,
    owned: &mut InteriorGeometryOwnership,
) -> Result<(Option<MeshAsset>, usize), PipelineError> {
    if mesh.name != ownership_mesh.name
        || mesh.groups.len() != ownership_mesh.groups.len()
    {
        return Err(PipelineError::new(
            "interior ownership mesh identity or group count changed",
        ));
    }
    let mut retained_groups = Vec::new();
    let mut removed_triangles = 0_usize;
    for (mut group, ownership_group) in
        mesh.groups.into_iter().zip(&ownership_mesh.groups)
    {
        if group.index != ownership_group.index
            || group.shader != ownership_group.shader
            || group.positions.len() != ownership_group.positions.len()
            || group.triangles != ownership_group.triangles
        {
            return Err(PipelineError::new(
                "interior ownership mesh topology changed",
            ));
        }
        let source_triangles = std::mem::take(&mut group.triangles);
        let mut retained_triangles = Vec::with_capacity(source_triangles.len());
        for triangle in source_triangles {
            if owned.claim(&ownership_mesh.name, ownership_group, &triangle)? {
                retained_triangles.push(triangle);
            } else {
                removed_triangles =
                    removed_triangles.checked_add(1).ok_or_else(|| {
                        PipelineError::new(
                            "interior duplicate triangle count overflowed",
                        )
                    })?;
            }
        }
        if !retained_triangles.is_empty() {
            group.triangles = retained_triangles;
            retained_groups.push(group);
        }
    }
    if retained_groups.is_empty() {
        return Ok((None, removed_triangles));
    }
    mesh.groups = retained_groups;
    Ok((Some(mesh), removed_triangles))
}

/// Build one exact source-authored face fingerprint independent of cyclic
/// corner start while preserving authored winding.
fn source_face_fingerprint(
    mesh_name: &str,
    group: &PrimitiveGroup,
    triangle: &[u32; 3],
) -> Result<[u8; 32], PipelineError> {
    let [first_index, second_index, third_index] = *triangle;
    let first = source_corner_identity(group, first_index)?;
    let second = source_corner_identity(group, second_index)?;
    let third = source_corner_identity(group, third_index)?;
    let corners = [first, second, third]
        .min([second, third, first])
        .min([third, first, second]);
    let mut state = Sha256::new();
    hash_text(&mut state, mesh_name)?;
    let group_index = u64::try_from(group.index).map_err(|error| {
        PipelineError::new(format!("interior group index overflowed: {error}"))
    })?;
    state.update(&group_index.to_le_bytes());
    hash_text(&mut state, &group.shader)?;
    for corner in corners {
        hash_bits(&mut state, corner.position);
        hash_optional_bits(&mut state, corner.uv);
        hash_optional_bits(&mut state, corner.normal);
        hash_optional_bits(&mut state, corner.color);
    }
    Ok(state.finalize())
}

/// Resolve exact source-authored channels for one triangle corner.
fn source_corner_identity(
    group: &PrimitiveGroup,
    index: u32,
) -> Result<InteriorCornerIdentity, PipelineError> {
    let vertex = usize::try_from(index).map_err(|error| {
        PipelineError::new(format!(
            "interior triangle index overflowed: {error}"
        ))
    })?;
    let position = group
        .positions
        .get(vertex)
        .copied()
        .ok_or_else(|| {
            PipelineError::new("interior triangle index is missing")
        })?
        .map(f32::to_bits);
    Ok(InteriorCornerIdentity {
        position,
        uv: source_channel(&group.uvs, vertex, "UV")?,
        normal: source_channel(&group.normals, vertex, "normal")?,
        color: source_channel(&group.colors, vertex, "color")?,
    })
}

/// Resolve one optional source channel at a triangle corner.
fn source_channel<const SIZE: usize>(
    values: &[[f32; SIZE]],
    vertex: usize,
    label: &str,
) -> Result<Option<[u32; SIZE]>, PipelineError> {
    if values.is_empty() {
        return Ok(None);
    }
    values
        .get(vertex)
        .copied()
        .map(|value| Some(value.map(f32::to_bits)))
        .ok_or_else(|| {
            PipelineError::new(format!(
                "interior source {label} is missing for vertex {vertex}"
            ))
        })
}

/// Hash one length-delimited source identity string.
fn hash_text(state: &mut Sha256, value: &str) -> Result<(), PipelineError> {
    let length = u64::try_from(value.len()).map_err(|error| {
        PipelineError::new(format!(
            "interior identity length overflowed: {error}"
        ))
    })?;
    state.update(&length.to_le_bytes());
    state.update(value.as_bytes());
    Ok(())
}

/// Hash one fixed-size array of exact floating-point bits.
fn hash_bits<const SIZE: usize>(state: &mut Sha256, values: [u32; SIZE]) {
    for value in values {
        state.update(&value.to_le_bytes());
    }
}

/// Hash one optional exact floating-point channel with an explicit marker.
fn hash_optional_bits<const SIZE: usize>(
    state: &mut Sha256,
    values: Option<[u32; SIZE]>,
) {
    match values {
        Some(values) => {
            state.update(&[1]);
            hash_bits(state, values);
        },
        None => state.update(&[0]),
    }
}

/// Build a geometry-only mesh key after reviewed world placement.
///
/// Triangle coordinates are quantized to one millimeter, each triangle is
/// orientation-independent, and the complete triangle set is sorted before
/// hashing. Names, materials, UVs, normals, vertex indices, and source package
/// ordering therefore cannot create false variant ownership.
///
/// # Errors
///
/// Returns an error when one triangle references a missing vertex.
#[cfg(test)]
pub(super) fn geometry_key(mesh: &MeshAsset) -> Result<String, PipelineError> {
    let mut triangles = Vec::<InteriorTriangleKey>::new();
    for group in &mesh.groups {
        for triangle in &group.triangles {
            triangles.push(triangle_geometry_key(&group.positions, triangle)?);
        }
    }
    triangles.sort_unstable();
    let mut bytes = Vec::with_capacity(triangles.len().saturating_mul(72));
    for triangle in triangles {
        for point in triangle {
            for component in point {
                bytes.extend_from_slice(&component.to_le_bytes());
            }
        }
    }
    Ok(digest_hex(&bytes))
}

/// Build one orientation-independent quantized world-space triangle identity.
#[cfg(test)]
fn triangle_geometry_key(
    positions: &[[f32; 3]],
    triangle: &[u32; 3],
) -> Result<InteriorTriangleKey, PipelineError> {
    let mut points = [[0_i64; 3]; 3];
    for (point, index) in points.iter_mut().zip(triangle) {
        let position = positions
            .get(usize::try_from(*index).map_err(|error| {
                PipelineError::new(format!(
                    "interior triangle index overflowed: {error}"
                ))
            })?)
            .ok_or_else(|| {
                PipelineError::new("interior triangle index is missing")
            })?;
        *point = position.map(quantize_component);
    }
    points.sort_unstable();
    Ok(points)
}

/// Quantize one finite source coordinate to one millimeter.
#[cfg(test)]
#[expect(
    clippy::as_conversions,
    clippy::cast_possible_truncation,
    reason = "rounded finite test coordinates intentionally become millimeter \
              cells"
)]
fn quantize_component(value: f32) -> i64 {
    (f64::from(value) * 1_000.).round() as i64
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/prop_catalog/world_level/interior/tests.rs"]
mod tests;

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

#[cfg(test)]
use fbx::domain::mesh::MeshAsset;
#[cfg(test)]
use shar_sha256::digest_hex;

#[cfg(test)]
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

/// Build a geometry-only mesh key after reviewed world placement.
///
/// Triangle coordinates are quantized to one millimeter, each triangle is
/// orientation-independent, and the complete triangle multiset is sorted before
/// hashing. This test-only diagnostic preserves authored triangle multiplicity.
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

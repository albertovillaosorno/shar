// File:
//   - interior.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/prop_catalog/world_level/
//     interior.rs
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
//   - Stable interior identities, reviewed package movements, Halloween roles,
//     and geometry-only deduplication evidence.
// - Must-Not:
//   - Read Blender files, infer transforms at runtime, write FBX files, or own
//     exterior movement.
// - Allows:
//   - Package lookup, exact reviewed affine matrices, and quantized triangle
//     signatures independent of names, materials, UVs, and vertex ordering.
// - Summary:
//   - Defines deterministic fused-interior generation authority.
//
// ADRs:
// - docs/adr/pipeline/unreal/world-assembly-from-normalized-chunks.md
//
// Large file:
//   - false

//! Deterministic reviewed interior placement and fused-geometry identity.

use std::collections::BTreeMap;

use fbx::domain::mesh::MeshAsset;
#[cfg(test)]
use shar_sha256::digest_hex;

use crate::domain::PipelineError;
use crate::domain::coordinate_movement::CoordinateMatrix;

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
    package_id: &str
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

/// Resolve one reviewed source-space movement, including the global height.
#[expect(
    clippy::excessive_precision,
    clippy::too_many_lines,
    clippy::unreadable_literal,
    reason = "reviewed matrix decimals preserve operator-derived evidence \
              exactly"
)]
#[must_use]
pub(super) fn movement_for_package(
    package_id: &str
) -> Option<(
    &'static str,
    CoordinateMatrix,
)> {
    let (id, matrix) = match package_id {
        "extracted-art-l1i00" => Some(
            (
                "interior-i00-level-01-reviewed-placement-and-global-height",
                [
                    1.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    1.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    -1.000000000_f32,
                    0.000000000_f32,
                    559.171508789_f32,
                    68.039663406_f32,
                    -943.929138184_f32,
                    1.000000000_f32,
                ],
            ),
        ),
        "extracted-art-l1i01" => Some(
            (
                "interior-i01-level-01-reviewed-placement-and-global-height",
                [
                    1.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    1.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    -1.000000000_f32,
                    0.000000000_f32,
                    289.275665283_f32,
                    68.569345566_f32,
                    -609.082275391_f32,
                    1.000000000_f32,
                ],
            ),
        ),
        "extracted-art-l1i02" => Some(
            (
                "interior-i02-level-01-reviewed-placement-and-global-height",
                [
                    0.000000000_f32,
                    0.000000000_f32,
                    1.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    1.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    1.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    169.568115234_f32,
                    66.762950989_f32,
                    314.965209961_f32,
                    1.000000000_f32,
                ],
            ),
        ),
        "extracted-art-l2i03" => Some(
            (
                "interior-i03-level-02-reviewed-placement-and-global-height",
                [
                    0.000000000_f32,
                    0.000000000_f32,
                    1.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    1.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    1.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    -432.246429443_f32,
                    70.251951309_f32,
                    7580.905761719_f32,
                    1.000000000_f32,
                ],
            ),
        ),
        "extracted-art-l2i04" => Some(
            (
                "interior-i04-level-02-reviewed-placement-and-global-height",
                [
                    1.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    1.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    -1.000000000_f32,
                    0.000000000_f32,
                    7276.824218750_f32,
                    70.512592407_f32,
                    -741.648315430_f32,
                    1.000000000_f32,
                ],
            ),
        ),
        "extracted-art-l3i05" => Some(
            (
                "interior-i05-level-03-reviewed-placement-and-global-height",
                [
                    0.000000000_f32,
                    0.000000000_f32,
                    1.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    1.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    1.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    -387.678131104_f32,
                    70.453786942_f32,
                    16651.007812500_f32,
                    1.000000000_f32,
                ],
            ),
        ),
        "extracted-art-l3i06" => Some(
            (
                "interior-i06-level-03-reviewed-placement-and-global-height",
                [
                    0.000000000_f32,
                    0.000000000_f32,
                    1.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    1.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    1.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    -892.374633789_f32,
                    83.478519531_f32,
                    16660.835937500_f32,
                    1.000000000_f32,
                ],
            ),
        ),
        "extracted-art-l4i00" => Some(
            (
                "interior-i00-level-04-reviewed-placement-and-global-height",
                [
                    1.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    1.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    -1.000000000_f32,
                    0.000000000_f32,
                    559.171020508_f32,
                    68.039663406_f32,
                    -943.929138184_f32,
                    1.000000000_f32,
                ],
            ),
        ),
        "extracted-art-l4i01" => Some(
            (
                "interior-i01-level-04-reviewed-placement-and-global-height",
                [
                    1.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    1.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    -1.000000000_f32,
                    0.000000000_f32,
                    289.275665283_f32,
                    68.569345566_f32,
                    -609.082275391_f32,
                    1.000000000_f32,
                ],
            ),
        ),
        "extracted-art-l4i02" => Some(
            (
                "interior-i02-level-04-reviewed-placement-and-global-height",
                [
                    0.000000000_f32,
                    0.000000000_f32,
                    1.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    1.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    1.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    169.568115234_f32,
                    66.763021561_f32,
                    314.964569092_f32,
                    1.000000000_f32,
                ],
            ),
        ),
        "extracted-art-l4i07" => Some(
            (
                "interior-i07-level-04-reviewed-placement-and-global-height",
                [
                    -1.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    1.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    1.000000000_f32,
                    0.000000000_f32,
                    -723.226440430_f32,
                    70.065055939_f32,
                    252.888824463_f32,
                    1.000000000_f32,
                ],
            ),
        ),
        "extracted-art-l5i03" => Some(
            (
                "interior-i03-level-05-reviewed-placement-and-global-height",
                [
                    0.000000000_f32,
                    0.000000000_f32,
                    1.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    1.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    1.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    -432.246429443_f32,
                    70.251951309_f32,
                    7580.905273438_f32,
                    1.000000000_f32,
                ],
            ),
        ),
        "extracted-art-l5i04" => Some(
            (
                "interior-i04-level-05-reviewed-placement-and-global-height",
                [
                    1.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    1.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    -1.000000000_f32,
                    0.000000000_f32,
                    7276.824218750_f32,
                    70.512592407_f32,
                    -741.648315430_f32,
                    1.000000000_f32,
                ],
            ),
        ),
        "extracted-art-l6i05" => Some(
            (
                "interior-i05-level-06-reviewed-placement-and-global-height",
                [
                    0.000000000_f32,
                    0.000000000_f32,
                    1.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    1.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    1.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    -387.678131104_f32,
                    70.453786942_f32,
                    16651.007812500_f32,
                    1.000000000_f32,
                ],
            ),
        ),
        "extracted-art-l6i06" => Some(
            (
                "interior-i06-level-06-reviewed-placement-and-global-height",
                [
                    0.000000000_f32,
                    0.000000000_f32,
                    1.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    1.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    1.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    -892.374633789_f32,
                    83.478519531_f32,
                    16660.835937500_f32,
                    1.000000000_f32,
                ],
            ),
        ),
        "extracted-art-l7i00" => Some(
            (
                "interior-i00-level-07-reviewed-placement-and-global-height",
                [
                    1.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    1.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    -1.000000000_f32,
                    0.000000000_f32,
                    559.170532227_f32,
                    68.039663406_f32,
                    -943.929138184_f32,
                    1.000000000_f32,
                ],
            ),
        ),
        "extracted-art-l7i01" => Some(
            (
                "interior-i01-level-07-reviewed-placement-and-global-height",
                [
                    1.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    1.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    -1.000000000_f32,
                    0.000000000_f32,
                    289.275177002_f32,
                    68.573124023_f32,
                    -609.082275391_f32,
                    1.000000000_f32,
                ],
            ),
        ),
        "extracted-art-l7i02" => Some(
            (
                "interior-i02-level-07-reviewed-placement-and-global-height",
                [
                    0.000000000_f32,
                    0.000000000_f32,
                    1.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    1.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    1.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    169.568115234_f32,
                    66.762950989_f32,
                    314.964202881_f32,
                    1.000000000_f32,
                ],
            ),
        ),
        "extracted-art-l7i07" => Some(
            (
                "interior-i07-level-07-reviewed-placement-and-global-height",
                [
                    -1.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    1.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    0.000000000_f32,
                    1.000000000_f32,
                    0.000000000_f32,
                    -723.225952148_f32,
                    70.065055939_f32,
                    252.888824463_f32,
                    1.000000000_f32,
                ],
            ),
        ),
        _ => None,
    }?;
    Some(
        (
            id,
            correct_source_x_basis(
                normalize_recurring_family_origin(
                    package_id, matrix,
                ),
            ),
        ),
    )
}

/// Remove the original recurring-family map displacement from one interior.
///
/// The source stores Zone 2 at an 8,192-meter X displacement and Zone 3 at a
/// 16,384-meter X displacement. The reviewed matrices include those source
/// coordinates, while the connected native world already owns family placement.
/// Remove the source displacement in the translated axis selected by each
/// reviewed interior orientation before the common FBX basis conversion.
fn normalize_recurring_family_origin(
    package_id: &str,
    mut matrix: CoordinateMatrix,
) -> CoordinateMatrix {
    match package_id {
        "extracted-art-l2i03" | "extracted-art-l5i03" => {
            matrix[14] -= 8_192.0;
        }
        "extracted-art-l2i04" | "extracted-art-l5i04" => {
            matrix[12] -= 8_192.0;
        }
        "extracted-art-l3i05"
        | "extracted-art-l3i06"
        | "extracted-art-l6i05"
        | "extracted-art-l6i06" => {
            matrix[14] -= 16_384.0;
        }
        _ => {}
    }
    matrix
}

/// Pre-compose one reviewed matrix with the source-to-FBX X reflection.
///
/// The operator review scene uses Blender package coordinates, while production
/// meshes remain in the source coordinate basis until the shared FBX export
/// root performs its final X reflection. Negating the source-X row preserves
/// the reviewed placement after that common import conversion.
fn correct_source_x_basis(mut matrix: CoordinateMatrix) -> CoordinateMatrix {
    for component in &mut matrix[..4] {
        *component = -*component;
    }
    matrix
}

/// Quantized orientation-independent world-space triangle identity.
#[cfg(test)]
pub(super) type InteriorTriangleKey = [[i64; 3]; 3];

/// Maximum reviewed placement noise accepted for duplicate ownership.
const INTERIOR_DUPLICATE_TOLERANCE_METERS: f32 = 0.005;
/// Coarse cell size used to query owned planar surface coverage.
const INTERIOR_SURFACE_BUCKET_METERS: f32 = 5.0;
/// One triangle in final reviewed world coordinates.
type InteriorTriangle = [[f32; 3]; 3];
/// Coarse centroid bucket used to bound tolerant duplicate searches.
type InteriorTriangleBucket = [i64; 3];
/// Coarse vertex bucket used to recognize alternate triangulation.
type InteriorPointBucket = [i64; 3];
/// Coarse surface cell containing one triangle's world-space bounds.
type InteriorSurfaceBucket = [i64; 3];

/// Spatially bounded geometry ownership for one fused interior identity.
#[derive(Debug, Default)]
pub(super) struct InteriorGeometryOwnership {
    /// Exact tolerant triangle candidates indexed by centroid cell.
    triangles: BTreeMap<InteriorTriangleBucket, Vec<InteriorTriangle>>,
    /// Owned triangle vertices indexed by tolerant point cell.
    points: BTreeMap<InteriorPointBucket, Vec<InteriorTriangle>>,
    /// Coplanar coverage candidates indexed by coarse bounds cells.
    surfaces: BTreeMap<InteriorSurfaceBucket, Vec<InteriorTriangle>>,
}

impl InteriorGeometryOwnership {
    /// Claim one triangle unless reviewed geometry already owns its surface.
    fn claim(
        &mut self,
        positions: &[[f32; 3]],
        triangle: &[u32; 3],
    ) -> Result<bool, PipelineError> {
        let candidate = triangle_points(
            positions, triangle,
        )?;
        if self.has_matching_triangle(&candidate)
            || self.reuses_coplanar_owned_surface(&candidate)
        {
            return Ok(false);
        }
        self.triangles
            .entry(triangle_bucket(&candidate))
            .or_default()
            .push(candidate);
        for point in candidate {
            self.points
                .entry(point_bucket(point))
                .or_default()
                .push(candidate);
        }
        for bucket in triangle_surfaces(&candidate) {
            self.surfaces
                .entry(bucket)
                .or_default()
                .push(candidate);
        }
        Ok(true)
    }

    /// Return whether one orientation-independent triangle is already owned.
    fn has_matching_triangle(
        &self,
        candidate: &InteriorTriangle,
    ) -> bool {
        neighboring_buckets(triangle_bucket(candidate)).any(
            |nearby| {
                self.triangles
                    .get(&nearby)
                    .is_some_and(
                        |owned| {
                            owned
                                .iter()
                                .any(
                                    |existing| {
                                        triangles_within_tolerance(
                                            candidate, existing,
                                        )
                                    },
                                )
                        },
                    )
            },
        )
    }

    /// Recognize the same planar surface even when its diagonal changed.
    fn reuses_coplanar_owned_surface(
        &self,
        candidate: &InteriorTriangle,
    ) -> bool {
        if !candidate
            .iter()
            .all(|point| self.contains_owned_point(*point))
        {
            return false;
        }
        triangle_surface_samples(candidate)
            .into_iter()
            .all(
                |sample| {
                    neighboring_buckets(surface_bucket(sample))
                        .filter_map(
                            |nearby| {
                                self.surfaces
                                    .get(&nearby)
                            },
                        )
                        .flatten()
                        .any(
                            |owned| {
                                triangles_share_plane(
                                    candidate, owned,
                                ) && point_is_inside_triangle(
                                    sample, owned,
                                )
                            },
                        )
                },
            )
    }

    /// Return whether one reviewed point already belongs to owned geometry.
    fn contains_owned_point(
        &self,
        candidate: [f32; 3],
    ) -> bool {
        neighboring_buckets(point_bucket(candidate))
            .filter_map(
                |nearby| {
                    self.points
                        .get(&nearby)
                },
            )
            .flatten()
            .any(
                |triangle| {
                    triangle
                        .iter()
                        .any(
                            |point| {
                                points_within_tolerance(
                                    candidate, *point,
                                )
                            },
                        )
                },
            )
    }
}

/// Retain only triangles not already owned by one fused interior publication.
///
/// Material, UV, normal, color, and source mesh identity remain attached to
/// every retained triangle. Ownership compares final reviewed geometry within a
/// five-millimeter tolerance, which covers the measured Blender placement noise
/// without making names, materials, UVs, or vertex ordering authoritative.
///
/// # Errors
///
/// Returns an error when one triangle references a missing vertex or the
/// duplicate-triangle counter overflows.
pub(super) fn retain_unowned_triangles(
    mut mesh: MeshAsset,
    owned: &mut InteriorGeometryOwnership,
) -> Result<
    (
        Option<MeshAsset>,
        usize,
    ),
    PipelineError,
> {
    let mut retained_groups = Vec::new();
    let mut removed_triangles = 0_usize;
    for mut group in mesh.groups {
        let source_triangles = std::mem::take(&mut group.triangles);
        let mut retained_triangles = Vec::with_capacity(source_triangles.len());
        for triangle in source_triangles {
            if owned.claim(
                &group.positions,
                &triangle,
            )? {
                retained_triangles.push(triangle);
            } else {
                removed_triangles = removed_triangles
                    .checked_add(1)
                    .ok_or_else(
                        || {
                            PipelineError::new(
                                "interior duplicate triangle count overflowed",
                            )
                        },
                    )?;
            }
        }
        if !retained_triangles.is_empty() {
            group.triangles = retained_triangles;
            retained_groups.push(group);
        }
    }
    if retained_groups.is_empty() {
        return Ok(
            (
                None,
                removed_triangles,
            ),
        );
    }
    mesh.groups = retained_groups;
    Ok(
        (
            Some(mesh),
            removed_triangles,
        ),
    )
}

/// Resolve one triangle into final world-space points.
fn triangle_points(
    positions: &[[f32; 3]],
    triangle: &[u32; 3],
) -> Result<InteriorTriangle, PipelineError> {
    let mut points = [[0.0_f32; 3]; 3];
    for (point, index) in points
        .iter_mut()
        .zip(triangle)
    {
        let position = positions
            .get(
                usize::try_from(*index).map_err(
                    |error| {
                        PipelineError::new(
                            format!(
                                "interior triangle index overflowed: {error}"
                            ),
                        )
                    },
                )?,
            )
            .ok_or_else(
                || PipelineError::new("interior triangle index is missing"),
            )?;
        *point = *position;
    }
    Ok(points)
}

/// Return one coarse centroid bucket for a tolerant triangle search.
fn triangle_bucket(triangle: &InteriorTriangle) -> InteriorTriangleBucket {
    point_bucket(triangle_centroid(triangle))
}

/// Return one triangle centroid in reviewed world coordinates.
fn triangle_centroid(triangle: &InteriorTriangle) -> [f32; 3] {
    let mut centroid = [0.0_f32; 3];
    for point in triangle {
        for (component, point_component) in centroid
            .iter_mut()
            .zip(point)
        {
            *component += *point_component / 3.0;
        }
    }
    centroid
}

/// Return interior samples proving that one candidate surface is fully covered.
fn triangle_surface_samples(triangle: &InteriorTriangle) -> [[f32; 3]; 4] {
    let [
        first,
        second,
        third,
    ] = *triangle;
    [
        triangle_centroid(triangle),
        midpoint(
            first, second,
        ),
        midpoint(
            second, third,
        ),
        midpoint(
            third, first,
        ),
    ]
}

/// Return the midpoint between two reviewed world-space points.
const fn midpoint(
    left: [f32; 3],
    right: [f32; 3],
) -> [f32; 3] {
    let [
        left_x,
        left_y,
        left_z,
    ] = left;
    let [
        right_x,
        right_y,
        right_z,
    ] = right;
    [
        f32::midpoint(
            left_x, right_x,
        ),
        f32::midpoint(
            left_y, right_y,
        ),
        f32::midpoint(
            left_z, right_z,
        ),
    ]
}

/// Return one coarse bucket for a reviewed world-space point.
#[expect(
    clippy::as_conversions,
    clippy::cast_possible_truncation,
    reason = "finite floored world coordinates intentionally become integer \
              cells"
)]
fn point_bucket(point: [f32; 3]) -> InteriorPointBucket {
    point.map(
        |component| {
            (component / INTERIOR_DUPLICATE_TOLERANCE_METERS).floor() as i64
        },
    )
}

/// Return one coarse bucket for planar surface coverage.
#[expect(
    clippy::as_conversions,
    clippy::cast_possible_truncation,
    reason = "finite floored world coordinates intentionally become integer \
              cells"
)]
fn surface_bucket(point: [f32; 3]) -> InteriorSurfaceBucket {
    point.map(
        |component| (component / INTERIOR_SURFACE_BUCKET_METERS).floor() as i64,
    )
}

/// Return every coarse cell touched by one triangle's world-space bounds.
fn triangle_surfaces(
    triangle: &InteriorTriangle
) -> Vec<InteriorSurfaceBucket> {
    let [
        first,
        second,
        third,
    ] = *triangle;
    let mut low = first;
    let mut high = first;
    for point in [
        second, third,
    ] {
        for ((low_component, high_component), point_component) in low
            .iter_mut()
            .zip(high.iter_mut())
            .zip(point)
        {
            *low_component = low_component.min(point_component);
            *high_component = high_component.max(point_component);
        }
    }
    let [
        low_x,
        low_y,
        low_z,
    ] = surface_bucket(low);
    let [
        high_x,
        high_y,
        high_z,
    ] = surface_bucket(high);
    let mut result = Vec::new();
    for x in low_x..=high_x {
        for y in low_y..=high_y {
            for z in low_z..=high_z {
                result.push(
                    [
                        x, y, z,
                    ],
                );
            }
        }
    }
    result
}

/// Return the 27 buckets touching one quantized three-dimensional cell.
#[expect(
    clippy::arithmetic_side_effects,
    clippy::indexing_slicing,
    reason = "three bounded offsets fill one fixed 27-cell neighborhood"
)]
fn neighboring_buckets(center: [i64; 3]) -> std::array::IntoIter<[i64; 3], 27> {
    let mut result = [[0_i64; 3]; 27];
    let mut index = 0_usize;
    for x_offset in -1_i64..=1_i64 {
        for y_offset in -1_i64..=1_i64 {
            for z_offset in -1_i64..=1_i64 {
                result[index] = [
                    center[0] + x_offset,
                    center[1] + y_offset,
                    center[2] + z_offset,
                ];
                index += 1;
            }
        }
    }
    result.into_iter()
}

/// Return whether two tolerant triangles describe the same supporting plane.
fn triangles_share_plane(
    left: &InteriorTriangle,
    right: &InteriorTriangle,
) -> bool {
    let Some(left_normal) = triangle_normal(left) else {
        return false;
    };
    let Some(right_normal) = triangle_normal(right) else {
        return false;
    };
    if dot(
        left_normal,
        right_normal,
    )
    .abs()
        < 0.999
    {
        return false;
    }
    let [
        left_origin,
        _,
        _,
    ] = *left;
    let [
        right_origin,
        _,
        _,
    ] = *right;
    left.iter()
        .all(
            |point| {
                point_plane_distance(
                    *point,
                    right_origin,
                    right_normal,
                ) <= INTERIOR_DUPLICATE_TOLERANCE_METERS
            },
        )
        && right
            .iter()
            .all(
                |point| {
                    point_plane_distance(
                        *point,
                        left_origin,
                        left_normal,
                    ) <= INTERIOR_DUPLICATE_TOLERANCE_METERS
                },
            )
}

/// Return one normalized triangle normal, or `None` for degenerate geometry.
fn triangle_normal(triangle: &InteriorTriangle) -> Option<[f32; 3]> {
    let [
        origin,
        second_point,
        third_point,
    ] = *triangle;
    let first_edge = subtract(
        second_point,
        origin,
    );
    let second_edge = subtract(
        third_point,
        origin,
    );
    let normal = cross(
        first_edge,
        second_edge,
    );
    let length = dot(
        normal, normal,
    )
    .sqrt();
    if length <= f32::EPSILON {
        return None;
    }
    Some(normal.map(|component| component / length))
}

/// Return whether one coplanar point lies inside a reviewed triangle.
fn point_is_inside_triangle(
    point: [f32; 3],
    triangle: &InteriorTriangle,
) -> bool {
    let [
        origin,
        second_point,
        third_point,
    ] = *triangle;
    let first = subtract(
        third_point,
        origin,
    );
    let second = subtract(
        second_point,
        origin,
    );
    let offset = subtract(
        point, origin,
    );
    let first_squared = dot(
        first, first,
    );
    let first_second = dot(
        first, second,
    );
    let second_squared = dot(
        second, second,
    );
    let first_offset = dot(
        first, offset,
    );
    let second_offset = dot(
        second, offset,
    );
    let denominator = first_squared.mul_add(
        second_squared,
        -(first_second * first_second),
    );
    if denominator.abs() <= f32::EPSILON {
        return false;
    }
    let inverse = denominator.recip();
    let first_weight = second_squared.mul_add(
        first_offset,
        -(first_second * second_offset),
    ) * inverse;
    let second_weight = first_squared.mul_add(
        second_offset,
        -(first_second * first_offset),
    ) * inverse;
    let third_edge = subtract(
        third_point,
        second_point,
    );
    let longest_edge = first_squared
        .max(second_squared)
        .max(
            dot(
                third_edge, third_edge,
            ),
        )
        .sqrt();
    if longest_edge <= f32::EPSILON {
        return false;
    }
    let minimum_altitude = denominator
        .abs()
        .sqrt()
        / longest_edge;
    if minimum_altitude <= f32::EPSILON {
        return false;
    }
    let tolerance =
        (INTERIOR_DUPLICATE_TOLERANCE_METERS / minimum_altitude).min(0.05);
    first_weight >= -tolerance
        && second_weight >= -tolerance
        && first_weight + second_weight <= 1.0 + tolerance
}

/// Subtract one reviewed world-space point from another.
fn subtract(
    left: [f32; 3],
    right: [f32; 3],
) -> [f32; 3] {
    let [
        left_x,
        left_y,
        left_z,
    ] = left;
    let [
        right_x,
        right_y,
        right_z,
    ] = right;
    [
        left_x - right_x,
        left_y - right_y,
        left_z - right_z,
    ]
}

/// Return one three-dimensional cross product.
fn cross(
    left: [f32; 3],
    right: [f32; 3],
) -> [f32; 3] {
    let [
        left_x,
        left_y,
        left_z,
    ] = left;
    let [
        right_x,
        right_y,
        right_z,
    ] = right;
    [
        left_y.mul_add(
            right_z,
            -(left_z * right_y),
        ),
        left_z.mul_add(
            right_x,
            -(left_x * right_z),
        ),
        left_x.mul_add(
            right_y,
            -(left_y * right_x),
        ),
    ]
}

/// Return one three-dimensional dot product.
fn dot(
    left: [f32; 3],
    right: [f32; 3],
) -> f32 {
    let [
        left_x,
        left_y,
        left_z,
    ] = left;
    let [
        right_x,
        right_y,
        right_z,
    ] = right;
    left_x.mul_add(
        right_x,
        left_y.mul_add(
            right_y,
            left_z * right_z,
        ),
    )
}

/// Return one point's absolute distance from a normalized plane.
fn point_plane_distance(
    point: [f32; 3],
    plane_point: [f32; 3],
    plane_normal: [f32; 3],
) -> f32 {
    dot(
        subtract(
            point,
            plane_point,
        ),
        plane_normal,
    )
    .abs()
}

/// Compare triangles independent of corner order within reviewed placement
/// noise.
fn triangles_within_tolerance(
    left: &InteriorTriangle,
    right: &InteriorTriangle,
) -> bool {
    let [
        left_first,
        left_second,
        left_third,
    ] = *left;
    let [
        right_first,
        right_second,
        right_third,
    ] = *right;
    [
        [
            right_first,
            right_second,
            right_third,
        ],
        [
            right_first,
            right_third,
            right_second,
        ],
        [
            right_second,
            right_first,
            right_third,
        ],
        [
            right_second,
            right_third,
            right_first,
        ],
        [
            right_third,
            right_first,
            right_second,
        ],
        [
            right_third,
            right_second,
            right_first,
        ],
    ]
    .into_iter()
    .any(
        |[
            first,
            second,
            third,
        ]| {
            points_within_tolerance(
                left_first, first,
            ) && points_within_tolerance(
                left_second,
                second,
            ) && points_within_tolerance(
                left_third, third,
            )
        },
    )
}

/// Compare two points within the measured reviewed-placement tolerance.
fn points_within_tolerance(
    left: [f32; 3],
    right: [f32; 3],
) -> bool {
    let delta = subtract(
        left, right,
    );
    dot(
        delta, delta,
    ) <= INTERIOR_DUPLICATE_TOLERANCE_METERS
        * INTERIOR_DUPLICATE_TOLERANCE_METERS
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
            triangles.push(
                triangle_geometry_key(
                    &group.positions,
                    triangle,
                )?,
            );
        }
    }
    triangles.sort_unstable();
    let mut bytes = Vec::with_capacity(
        triangles
            .len()
            .saturating_mul(72),
    );
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
    for (point, index) in points
        .iter_mut()
        .zip(triangle)
    {
        let position = positions
            .get(
                usize::try_from(*index).map_err(
                    |error| {
                        PipelineError::new(
                            format!(
                                "interior triangle index overflowed: {error}"
                            ),
                        )
                    },
                )?,
            )
            .ok_or_else(
                || PipelineError::new("interior triangle index is missing"),
            )?;
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
    (f64::from(value) * 1_000.0).round() as i64
}

#[cfg(test)]
mod tests {
    use fbx::domain::mesh::{MeshAsset, PrimitiveGroup};

    use super::{
        InteriorGeometryOwnership, geometry_key, identity_for_package,
        is_halloween_package, movement_for_package, retain_unowned_triangles,
    };

    #[test]
    fn all_nineteen_source_packages_have_reviewed_movements() {
        let packages = [
            "extracted-art-l1i00",
            "extracted-art-l1i01",
            "extracted-art-l1i02",
            "extracted-art-l2i03",
            "extracted-art-l2i04",
            "extracted-art-l3i05",
            "extracted-art-l3i06",
            "extracted-art-l4i00",
            "extracted-art-l4i01",
            "extracted-art-l4i02",
            "extracted-art-l4i07",
            "extracted-art-l5i03",
            "extracted-art-l5i04",
            "extracted-art-l6i05",
            "extracted-art-l6i06",
            "extracted-art-l7i00",
            "extracted-art-l7i01",
            "extracted-art-l7i02",
            "extracted-art-l7i07",
        ];
        assert_eq!(
            packages.len(),
            19
        );
        for package in packages {
            assert!(
                identity_for_package(package).is_some(),
                "{package}"
            );
            assert!(
                movement_for_package(package).is_some(),
                "{package}"
            );
        }
    }

    #[test]
    fn only_level_seven_halloween_packages_are_overlays() {
        assert!(is_halloween_package("extracted-art-l7i00"));
        assert!(is_halloween_package("extracted-art-l7i01"));
        assert!(is_halloween_package("extracted-art-l7i02"));
        assert!(is_halloween_package("extracted-art-l7i07"));
        assert!(!is_halloween_package("extracted-art-l4i07"));
        assert!(!is_halloween_package("extracted-art-l6i06"));
    }

    #[test]
    fn kwik_e_mart_reviewed_movement_preserves_fbx_import_basis()
    -> Result<(), String> {
        let (_, matrix) = movement_for_package("extracted-art-l4i01")
            .ok_or_else(
                || String::from("Level 4 Kwik-E-Mart movement is missing"),
            )?;
        let source = [
            492.979_58_f32,
            -20.000_023_f32,
            -307.126_68_f32,
        ];
        let moved = [
            source[0].mul_add(
                matrix[0],
                source[1].mul_add(
                    matrix[4],
                    source[2].mul_add(
                        matrix[8], matrix[12],
                    ),
                ),
            ),
            source[0].mul_add(
                matrix[1],
                source[1].mul_add(
                    matrix[5],
                    source[2].mul_add(
                        matrix[9], matrix[13],
                    ),
                ),
            ),
            source[0].mul_add(
                matrix[2],
                source[1].mul_add(
                    matrix[6],
                    source[2].mul_add(
                        matrix[10], matrix[14],
                    ),
                ),
            ),
        ];
        let blender_import = [
            -moved[0], moved[2], moved[1],
        ];
        let expected = [
            203.703_92_f32,
            -301.955_6_f32,
            48.569_32_f32,
        ];
        if blender_import
            .iter()
            .zip(expected)
            .any(|(actual, wanted)| (*actual - wanted).abs() > 0.001)
        {
            return Err(
                format!("Kwik-E-Mart import basis changed: {blender_import:?}"),
            );
        }
        let (_, level_one_matrix) = movement_for_package("extracted-art-l1i01")
            .ok_or_else(
                || String::from("Level 1 Kwik-E-Mart movement is missing"),
            )?;
        if level_one_matrix
            .iter()
            .zip(matrix)
            .any(|(left, right)| (*left - right).abs() > 0.000_001)
        {
            return Err(
                String::from(
                    "Level 1 and Level 4 Kwik-E-Mart placements diverged",
                ),
            );
        }
        Ok(())
    }

    #[test]
    fn recurring_interior_family_origins_are_cancelled() -> Result<(), String> {
        for (package, translation_index, expected) in [
            (
                "extracted-art-l2i03",
                14_usize,
                -611.094_24_f32,
            ),
            (
                "extracted-art-l2i04",
                12_usize,
                -915.175_8_f32,
            ),
            (
                "extracted-art-l3i05",
                14_usize,
                267.007_8_f32,
            ),
            (
                "extracted-art-l3i06",
                14_usize,
                276.835_94_f32,
            ),
            (
                "extracted-art-l5i03",
                14_usize,
                -611.094_7_f32,
            ),
            (
                "extracted-art-l5i04",
                12_usize,
                -915.175_8_f32,
            ),
            (
                "extracted-art-l6i05",
                14_usize,
                267.007_8_f32,
            ),
            (
                "extracted-art-l6i06",
                14_usize,
                276.835_94_f32,
            ),
        ] {
            let (_, matrix) = movement_for_package(package).ok_or_else(
                || format!("interior movement is missing: {package}"),
            )?;
            let actual = *matrix
                .get(translation_index)
                .ok_or_else(
                    || {
                        format!(
                            "interior translation index is missing: {package}"
                        )
                    },
                )?;
            if (actual - expected).abs() > 0.001 {
                return Err(
                    format!(
                        "recurring family origin remained for {package}: \
                         {actual} != {expected}"
                    ),
                );
            }
        }
        Ok(())
    }

    #[test]
    fn geometry_key_ignores_vertex_order_but_preserves_position()
    -> Result<(), String> {
        let first_group = PrimitiveGroup::new(
            0,
            "material",
            vec![
                [
                    0.0, 0.0, 0.0,
                ],
                [
                    1.0, 0.0, 0.0,
                ],
                [
                    0.0, 1.0, 0.0,
                ],
            ],
            Vec::new(),
            &[
                0, 1, 2,
            ],
        )
        .map_err(|error| format!("first group failed: {error:?}"))?;
        let reordered_group = PrimitiveGroup::new(
            0,
            "other-material",
            vec![
                [
                    0.0, 1.0, 0.0,
                ],
                [
                    0.0, 0.0, 0.0,
                ],
                [
                    1.0, 0.0, 0.0,
                ],
            ],
            Vec::new(),
            &[
                1, 2, 0,
            ],
        )
        .map_err(|error| format!("reordered group failed: {error:?}"))?;
        let shifted_group = PrimitiveGroup::new(
            0,
            "material",
            vec![
                [
                    0.002, 0.0, 0.0,
                ],
                [
                    1.002, 0.0, 0.0,
                ],
                [
                    0.002, 1.0, 0.0,
                ],
            ],
            Vec::new(),
            &[
                0, 1, 2,
            ],
        )
        .map_err(|error| format!("shifted group failed: {error:?}"))?;
        let first = MeshAsset::new(
            "first",
            vec![first_group],
        )
        .map_err(|error| format!("first mesh failed: {error:?}"))?;
        let reordered = MeshAsset::new(
            "reordered",
            vec![reordered_group],
        )
        .map_err(|error| format!("reordered mesh failed: {error:?}"))?;
        let shifted = MeshAsset::new(
            "shifted",
            vec![shifted_group],
        )
        .map_err(|error| format!("shifted mesh failed: {error:?}"))?;
        let first_key =
            geometry_key(&first).map_err(|error| error.to_string())?;
        let reordered_key =
            geometry_key(&reordered).map_err(|error| error.to_string())?;
        let shifted_key =
            geometry_key(&shifted).map_err(|error| error.to_string())?;
        if first_key != reordered_key {
            return Err(
                String::from("vertex ordering changed the geometry key"),
            );
        }
        if first_key == shifted_key {
            return Err(
                String::from("world placement did not change the geometry key"),
            );
        }
        Ok(())
    }

    #[test]
    #[expect(
        clippy::too_many_lines,
        reason = "explicit mixed-mesh fixture preserves both source triangles"
    )]
    fn halloween_mixed_mesh_retains_only_new_triangles() -> Result<(), String> {
        let base_group = PrimitiveGroup::new(
            0,
            "base-material",
            vec![
                [
                    0.0, 0.0, 0.0,
                ],
                [
                    1.0, 0.0, 0.0,
                ],
                [
                    0.0, 1.0, 0.0,
                ],
            ],
            Vec::new(),
            &[
                0, 1, 2,
            ],
        )
        .map_err(|error| format!("base group failed: {error:?}"))?;
        let mixed_group = PrimitiveGroup::new(
            0,
            "halloween-material",
            vec![
                [
                    0.004, 0.0, 0.0,
                ],
                [
                    1.004, 0.0, 0.0,
                ],
                [
                    0.004, 1.0, 0.0,
                ],
                [
                    1.0, 1.0, 0.0,
                ],
            ],
            Vec::new(),
            &[
                0, 1, 2, 1, 3, 2,
            ],
        )
        .map_err(|error| format!("mixed group failed: {error:?}"))?;
        let base = MeshAsset::new(
            "base",
            vec![base_group],
        )
        .map_err(|error| format!("base mesh failed: {error:?}"))?;
        let mixed = MeshAsset::new(
            "mixed",
            vec![mixed_group],
        )
        .map_err(|error| format!("mixed mesh failed: {error:?}"))?;
        let mut owned = InteriorGeometryOwnership::default();
        let (retained_base, removed_base) = retain_unowned_triangles(
            base, &mut owned,
        )
        .map_err(|error| error.to_string())?;
        if retained_base.is_none() || removed_base != 0 {
            return Err(
                String::from("canonical base triangle was not retained"),
            );
        }
        let (retained_overlay_option, removed_overlay) =
            retain_unowned_triangles(
                mixed, &mut owned,
            )
            .map_err(|error| error.to_string())?;
        let retained_overlay = retained_overlay_option
            .ok_or_else(|| String::from("Halloween addition was removed"))?;
        if removed_overlay != 1 {
            return Err(
                format!(
                    "expected one repeated base triangle, found \
                     {removed_overlay}"
                ),
            );
        }
        let retained_group = retained_overlay
            .groups
            .first()
            .ok_or_else(
                || String::from("Halloween overlay group is missing"),
            )?;
        if retained_overlay
            .groups
            .len()
            != 1
            || retained_group
                .triangles
                .len()
                != 1
        {
            return Err(
                String::from(
                    "Halloween overlay did not retain exactly one new triangle",
                ),
            );
        }
        if retained_group.shader != "halloween-material" {
            return Err(
                String::from(
                    "Halloween triangle lost its original material authority",
                ),
            );
        }
        Ok(())
    }

    #[test]
    fn alternate_planar_triangulation_reuses_owned_surface()
    -> Result<(), String> {
        let base_group = PrimitiveGroup::new(
            0,
            "base-material",
            vec![
                [
                    0.0, 0.0, 0.0,
                ],
                [
                    1.0, 0.0, 0.0,
                ],
                [
                    1.0, 1.0, 0.0,
                ],
                [
                    0.0, 1.0, 0.0,
                ],
            ],
            Vec::new(),
            &[
                0, 1, 2, 0, 2, 3,
            ],
        )
        .map_err(|error| format!("base square failed: {error:?}"))?;
        let alternate_group = PrimitiveGroup::new(
            0,
            "halloween-material",
            vec![
                [
                    0.004, 0.0, 0.0,
                ],
                [
                    1.004, 0.0, 0.0,
                ],
                [
                    1.004, 1.0, 0.0,
                ],
                [
                    0.004, 1.0, 0.0,
                ],
            ],
            Vec::new(),
            &[
                0, 1, 3, 1, 2, 3,
            ],
        )
        .map_err(|error| format!("alternate square failed: {error:?}"))?;
        let base = MeshAsset::new(
            "base-square",
            vec![base_group],
        )
        .map_err(|error| format!("base mesh failed: {error:?}"))?;
        let alternate = MeshAsset::new(
            "alternate-square",
            vec![alternate_group],
        )
        .map_err(|error| format!("alternate mesh failed: {error:?}"))?;
        let mut owned = InteriorGeometryOwnership::default();
        let (retained_base, removed_base) = retain_unowned_triangles(
            base, &mut owned,
        )
        .map_err(|error| error.to_string())?;
        if retained_base.is_none() || removed_base != 0 {
            return Err(String::from("canonical square was not retained"));
        }
        let (retained_alternate, removed_alternate) = retain_unowned_triangles(
            alternate, &mut owned,
        )
        .map_err(|error| error.to_string())?;
        if retained_alternate.is_some() || removed_alternate != 2 {
            return Err(
                format!(
                    "alternate planar triangulation remained: retained={}, \
                     removed={removed_alternate}",
                    retained_alternate.is_some(),
                ),
            );
        }
        Ok(())
    }

    #[test]
    #[expect(
        clippy::too_many_lines,
        reason = "explicit separated-surface fixture preserves its coverage \
                  gap"
    )]
    fn uncovered_planar_span_with_owned_vertices_is_retained()
    -> Result<(), String> {
        let base_group = PrimitiveGroup::new(
            0,
            "base-material",
            vec![
                [
                    0.0, 0.0, 0.0,
                ],
                [
                    1.0, 0.0, 0.0,
                ],
                [
                    0.0, 1.0, 0.0,
                ],
                [
                    3.0, 0.0, 0.0,
                ],
                [
                    4.0, 0.0, 0.0,
                ],
                [
                    4.0, 1.0, 0.0,
                ],
            ],
            Vec::new(),
            &[
                0, 1, 2, 3, 4, 5,
            ],
        )
        .map_err(|error| format!("separated base failed: {error:?}"))?;
        let spanning_group = PrimitiveGroup::new(
            0,
            "new-material",
            vec![
                [
                    1.0, 0.0, 0.0,
                ],
                [
                    3.0, 0.0, 0.0,
                ],
                [
                    4.0, 1.0, 0.0,
                ],
            ],
            Vec::new(),
            &[
                0, 1, 2,
            ],
        )
        .map_err(|error| format!("planar span failed: {error:?}"))?;
        let base = MeshAsset::new(
            "separated-base",
            vec![base_group],
        )
        .map_err(|error| format!("base mesh failed: {error:?}"))?;
        let spanning = MeshAsset::new(
            "planar-span",
            vec![spanning_group],
        )
        .map_err(|error| format!("spanning mesh failed: {error:?}"))?;
        let mut owned = InteriorGeometryOwnership::default();
        let (retained_base, removed_base) = retain_unowned_triangles(
            base, &mut owned,
        )
        .map_err(|error| error.to_string())?;
        if retained_base.is_none() || removed_base != 0 {
            return Err(String::from("separated base was not retained"));
        }
        let (retained_spanning_option, removed_spanning) =
            retain_unowned_triangles(
                spanning, &mut owned,
            )
            .map_err(|error| error.to_string())?;
        let retained_spanning = retained_spanning_option
            .ok_or_else(|| String::from("uncovered planar span was removed"))?;
        let retained_group = retained_spanning
            .groups
            .first()
            .ok_or_else(
                || String::from("uncovered planar span group is missing"),
            )?;
        if removed_spanning != 0
            || retained_spanning
                .groups
                .len()
                != 1
            || retained_group
                .triangles
                .len()
                != 1
        {
            return Err(
                format!(
                    "uncovered planar span changed: groups={}, \
                     removed={removed_spanning}",
                    retained_spanning
                        .groups
                        .len(),
                ),
            );
        }
        Ok(())
    }
}

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

/// Resolve one reviewed source-space movement, including the final height.
#[must_use]
pub(super) fn movement_for_package(
    package_id: &str,
) -> Option<(&'static str, CoordinateMatrix)> {
    reviewed_movement_for_package(package_id)
        .map(|(id, matrix)| (id, apply_canonical_world_height(matrix)))
}

/// Resolve the reviewed movement before the Unreal datum presentation raise.
#[expect(
    clippy::excessive_precision,
    clippy::too_many_lines,
    clippy::unreadable_literal,
    reason = "reviewed matrix decimals preserve operator-derived evidence \
              exactly"
)]
#[must_use]
pub(super) fn reviewed_movement_for_package(
    package_id: &str,
) -> Option<(&'static str, CoordinateMatrix)> {
    let (id, matrix) = match package_id {
        "extracted-art-l1i00" => Some((
            "interior-i00-level-01-reviewed-placement-and-global-height",
            [
                1f32,
                0f32,
                0f32,
                0f32,
                0f32,
                1f32,
                0f32,
                0f32,
                0f32,
                0f32,
                -1f32,
                0f32,
                559.171508789_f32,
                68.039663406_f32,
                -943.929138184_f32,
                1f32,
            ],
        )),
        "extracted-art-l1i01" => Some((
            "interior-i01-level-01-reviewed-placement-and-global-height",
            [
                1f32,
                0f32,
                0f32,
                0f32,
                0f32,
                1f32,
                0f32,
                0f32,
                0f32,
                0f32,
                -1f32,
                0f32,
                289.275665283_f32,
                68.569345566_f32,
                -609.082275391_f32,
                1f32,
            ],
        )),
        "extracted-art-l1i02" => Some((
            "interior-i02-level-01-reviewed-placement-and-global-height",
            [
                0f32,
                0f32,
                1f32,
                0f32,
                0f32,
                1f32,
                0f32,
                0f32,
                1f32,
                0f32,
                0f32,
                0f32,
                169.568115234_f32,
                66.762950989_f32,
                314.965209961_f32,
                1f32,
            ],
        )),
        "extracted-art-l2i03" => Some((
            "interior-i03-level-02-reviewed-placement-and-global-height",
            [
                0f32,
                0f32,
                1f32,
                0f32,
                0f32,
                1f32,
                0f32,
                0f32,
                1f32,
                0f32,
                0f32,
                0f32,
                -432.246429443_f32,
                70.251951309_f32,
                7580.905761719_f32,
                1f32,
            ],
        )),
        "extracted-art-l2i04" => Some((
            "interior-i04-level-02-reviewed-placement-and-global-height",
            [
                1f32,
                0f32,
                0f32,
                0f32,
                0f32,
                1f32,
                0f32,
                0f32,
                0f32,
                0f32,
                -1f32,
                0f32,
                7276.824218750_f32,
                70.512592407_f32,
                -741.648315430_f32,
                1f32,
            ],
        )),
        "extracted-art-l3i05" => Some((
            "interior-i05-level-03-reviewed-placement-and-global-height",
            [
                0f32,
                0f32,
                1f32,
                0f32,
                0f32,
                1f32,
                0f32,
                0f32,
                1f32,
                0f32,
                0f32,
                0f32,
                -387.678131104_f32,
                70.453786942_f32,
                16651.007812500_f32,
                1f32,
            ],
        )),
        "extracted-art-l3i06" => Some((
            "interior-i06-level-03-reviewed-placement-and-global-height",
            [
                0f32,
                0f32,
                1f32,
                0f32,
                0f32,
                1f32,
                0f32,
                0f32,
                1f32,
                0f32,
                0f32,
                0f32,
                -892.374633789_f32,
                83.478519531_f32,
                16660.835937500_f32,
                1f32,
            ],
        )),
        "extracted-art-l4i00" => Some((
            "interior-i00-level-04-reviewed-placement-and-global-height",
            [
                1f32,
                0f32,
                0f32,
                0f32,
                0f32,
                1f32,
                0f32,
                0f32,
                0f32,
                0f32,
                -1f32,
                0f32,
                559.171020508_f32,
                68.039663406_f32,
                -943.929138184_f32,
                1f32,
            ],
        )),
        "extracted-art-l4i01" => Some((
            "interior-i01-level-04-reviewed-placement-and-global-height",
            [
                1f32,
                0f32,
                0f32,
                0f32,
                0f32,
                1f32,
                0f32,
                0f32,
                0f32,
                0f32,
                -1f32,
                0f32,
                289.275665283_f32,
                68.569345566_f32,
                -609.082275391_f32,
                1f32,
            ],
        )),
        "extracted-art-l4i02" => Some((
            "interior-i02-level-04-reviewed-placement-and-global-height",
            [
                0f32,
                0f32,
                1f32,
                0f32,
                0f32,
                1f32,
                0f32,
                0f32,
                1f32,
                0f32,
                0f32,
                0f32,
                169.568115234_f32,
                66.763021561_f32,
                314.964569092_f32,
                1f32,
            ],
        )),
        "extracted-art-l4i07" => Some((
            "interior-i07-level-04-reviewed-placement-and-global-height",
            [
                -1f32,
                0f32,
                0f32,
                0f32,
                0f32,
                1f32,
                0f32,
                0f32,
                0f32,
                0f32,
                1f32,
                0f32,
                -723.226440430_f32,
                70.065055939_f32,
                252.888824463_f32,
                1f32,
            ],
        )),
        "extracted-art-l5i03" => Some((
            "interior-i03-level-05-reviewed-placement-and-global-height",
            [
                0f32,
                0f32,
                1f32,
                0f32,
                0f32,
                1f32,
                0f32,
                0f32,
                1f32,
                0f32,
                0f32,
                0f32,
                -432.246429443_f32,
                70.251951309_f32,
                7580.905273438_f32,
                1f32,
            ],
        )),
        "extracted-art-l5i04" => Some((
            "interior-i04-level-05-reviewed-placement-and-global-height",
            [
                1f32,
                0f32,
                0f32,
                0f32,
                0f32,
                1f32,
                0f32,
                0f32,
                0f32,
                0f32,
                -1f32,
                0f32,
                7276.824218750_f32,
                70.512592407_f32,
                -741.648315430_f32,
                1f32,
            ],
        )),
        "extracted-art-l6i05" => Some((
            "interior-i05-level-06-reviewed-placement-and-global-height",
            [
                0f32,
                0f32,
                1f32,
                0f32,
                0f32,
                1f32,
                0f32,
                0f32,
                1f32,
                0f32,
                0f32,
                0f32,
                -387.678131104_f32,
                70.453786942_f32,
                16651.007812500_f32,
                1f32,
            ],
        )),
        "extracted-art-l6i06" => Some((
            "interior-i06-level-06-reviewed-placement-and-global-height",
            [
                0f32,
                0f32,
                1f32,
                0f32,
                0f32,
                1f32,
                0f32,
                0f32,
                1f32,
                0f32,
                0f32,
                0f32,
                -892.374633789_f32,
                83.478519531_f32,
                16660.835937500_f32,
                1f32,
            ],
        )),
        "extracted-art-l7i00" => Some((
            "interior-i00-level-07-reviewed-placement-and-global-height",
            [
                1f32,
                0f32,
                0f32,
                0f32,
                0f32,
                1f32,
                0f32,
                0f32,
                0f32,
                0f32,
                -1f32,
                0f32,
                559.170532227_f32,
                68.039663406_f32,
                -943.929138184_f32,
                1f32,
            ],
        )),
        "extracted-art-l7i01" => Some((
            "interior-i01-level-07-reviewed-placement-and-global-height",
            [
                1f32,
                0f32,
                0f32,
                0f32,
                0f32,
                1f32,
                0f32,
                0f32,
                0f32,
                0f32,
                -1f32,
                0f32,
                289.275177002_f32,
                68.573124023_f32,
                -609.082275391_f32,
                1f32,
            ],
        )),
        "extracted-art-l7i02" => Some((
            "interior-i02-level-07-reviewed-placement-and-global-height",
            [
                0f32,
                0f32,
                1f32,
                0f32,
                0f32,
                1f32,
                0f32,
                0f32,
                1f32,
                0f32,
                0f32,
                0f32,
                169.568115234_f32,
                66.762950989_f32,
                314.964202881_f32,
                1f32,
            ],
        )),
        "extracted-art-l7i07" => Some((
            "interior-i07-level-07-reviewed-placement-and-global-height",
            [
                -1f32,
                0f32,
                0f32,
                0f32,
                0f32,
                1f32,
                0f32,
                0f32,
                0f32,
                0f32,
                1f32,
                0f32,
                -723.225952148_f32,
                70.065055939_f32,
                252.888824463_f32,
                1f32,
            ],
        )),
        _ => None,
    }?;
    Some((
        id,
        correct_source_x_basis(normalize_recurring_family_origin(
            package_id, matrix,
        )),
    ))
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
            matrix[14] -= 8_192.;
        },
        "extracted-art-l2i04" | "extracted-art-l5i04" => {
            matrix[12] -= 8_192.;
        },
        "extracted-art-l3i05"
        | "extracted-art-l3i06"
        | "extracted-art-l6i05"
        | "extracted-art-l6i06" => {
            matrix[14] -= 16_384.;
        },
        _ => {},
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

/// Replace the superseded reviewed reference height with the canonical datum.
///
/// The historical matrices remain unchanged as placement evidence. Publication
/// subtracts their embedded 43.396-meter reference and bakes exactly 80.0
/// meters into geometry and every decoded coordinate family.
fn apply_canonical_world_height(
    mut matrix: CoordinateMatrix,
) -> CoordinateMatrix {
    matrix[13] += super::movement::WORLD_HEIGHT_OFFSET_METERS
        - super::movement::LEGACY_REVIEWED_HEIGHT_OFFSET_METERS;
    matrix
}

/// Quantized orientation-independent world-space triangle identity.
#[cfg(test)]
pub(super) type InteriorTriangleKey = [[i64; 3]; 3];

/// Maximum reviewed placement noise accepted for duplicate ownership.
const INTERIOR_DUPLICATE_TOLERANCE_METERS: f32 = 0.005;
/// Coarse cell size used to query owned planar surface coverage.
const INTERIOR_SURFACE_BUCKET_METERS: f32 = 5.;
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
        let candidate = triangle_points(positions, triangle)?;
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
            self.surfaces.entry(bucket).or_default().push(candidate);
        }
        Ok(true)
    }

    /// Return whether one orientation-independent triangle is already owned.
    fn has_matching_triangle(&self, candidate: &InteriorTriangle) -> bool {
        neighboring_buckets(triangle_bucket(candidate)).any(|nearby| {
            self.triangles.get(&nearby).is_some_and(|owned| {
                owned.iter().any(|existing| {
                    triangles_within_tolerance(candidate, existing)
                })
            })
        })
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
            .all(|sample| {
                neighboring_buckets(surface_bucket(sample))
                    .filter_map(|nearby| self.surfaces.get(&nearby))
                    .flatten()
                    .any(|owned| {
                        triangles_share_plane(candidate, owned)
                            && point_is_inside_triangle(sample, owned)
                    })
            })
    }

    /// Return whether one reviewed point already belongs to owned geometry.
    fn contains_owned_point(&self, candidate: [f32; 3]) -> bool {
        neighboring_buckets(point_bucket(candidate))
            .filter_map(|nearby| self.points.get(&nearby))
            .flatten()
            .any(|triangle| {
                triangle
                    .iter()
                    .any(|point| points_within_tolerance(candidate, *point))
            })
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
#[cfg(test)]
pub(super) fn retain_unowned_triangles(
    mesh: MeshAsset,
    owned: &mut InteriorGeometryOwnership,
) -> Result<(Option<MeshAsset>, usize), PipelineError> {
    let ownership_mesh = mesh.clone();
    retain_unowned_triangles_with_ownership(mesh, &ownership_mesh, owned)
}

/// Retain final-datum triangles using an exact pre-datum ownership mesh.
///
/// The ownership mesh is cloned before the 80-meter Unreal alignment raise
/// and transformed with the previously reviewed matrix. This preserves every
/// established fusion decision while the retained mesh keeps its final raised
/// coordinates.
///
/// # Errors
///
/// Returns an error when render and ownership topology diverge or one triangle
/// references a missing ownership vertex.
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
            if owned.claim(&ownership_group.positions, &triangle)? {
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

/// Resolve one triangle from the exact reviewed ownership positions.
fn triangle_points(
    positions: &[[f32; 3]],
    triangle: &[u32; 3],
) -> Result<InteriorTriangle, PipelineError> {
    let mut points = [[0f32; 3]; 3];
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
    let mut centroid = [0f32; 3];
    for point in triangle {
        for (component, point_component) in centroid.iter_mut().zip(point) {
            *component += *point_component / 3.;
        }
    }
    centroid
}

/// Return interior samples proving that one candidate surface is fully covered.
fn triangle_surface_samples(triangle: &InteriorTriangle) -> [[f32; 3]; 4] {
    let [first, second, third] = *triangle;
    [
        triangle_centroid(triangle),
        midpoint(first, second),
        midpoint(second, third),
        midpoint(third, first),
    ]
}

/// Return the midpoint between two reviewed world-space points.
const fn midpoint(left: [f32; 3], right: [f32; 3]) -> [f32; 3] {
    let [left_x, left_y, left_z] = left;
    let [right_x, right_y, right_z] = right;
    [
        f32::midpoint(left_x, right_x),
        f32::midpoint(left_y, right_y),
        f32::midpoint(left_z, right_z),
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
    point.map(|component| {
        (component / INTERIOR_DUPLICATE_TOLERANCE_METERS).floor() as i64
    })
}

/// Return one coarse bucket for planar surface coverage.
#[expect(
    clippy::as_conversions,
    clippy::cast_possible_truncation,
    reason = "finite floored world coordinates intentionally become integer \
              cells"
)]
fn surface_bucket(point: [f32; 3]) -> InteriorSurfaceBucket {
    point.map(|component| {
        (component / INTERIOR_SURFACE_BUCKET_METERS).floor() as i64
    })
}

/// Return every coarse cell touched by one triangle's world-space bounds.
fn triangle_surfaces(
    triangle: &InteriorTriangle,
) -> Vec<InteriorSurfaceBucket> {
    let [first, second, third] = *triangle;
    let mut low = first;
    let mut high = first;
    for point in [second, third] {
        for ((low_component, high_component), point_component) in
            low.iter_mut().zip(high.iter_mut()).zip(point)
        {
            *low_component = low_component.min(point_component);
            *high_component = high_component.max(point_component);
        }
    }
    let [low_x, low_y, low_z] = surface_bucket(low);
    let [high_x, high_y, high_z] = surface_bucket(high);
    let mut result = Vec::new();
    for x in low_x..=high_x {
        for y in low_y..=high_y {
            for z in low_z..=high_z {
                result.push([x, y, z]);
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
    if dot(left_normal, right_normal).abs() < 0.999 {
        return false;
    }
    let [left_origin, _, _] = *left;
    let [right_origin, _, _] = *right;
    left.iter().all(|point| {
        point_plane_distance(*point, right_origin, right_normal)
            <= INTERIOR_DUPLICATE_TOLERANCE_METERS
    }) && right.iter().all(|point| {
        point_plane_distance(*point, left_origin, left_normal)
            <= INTERIOR_DUPLICATE_TOLERANCE_METERS
    })
}

/// Return one normalized triangle normal, or `None` for degenerate geometry.
fn triangle_normal(triangle: &InteriorTriangle) -> Option<[f32; 3]> {
    let [origin, second_point, third_point] = *triangle;
    let first_edge = subtract(second_point, origin);
    let second_edge = subtract(third_point, origin);
    let normal = cross(first_edge, second_edge);
    let length = dot(normal, normal).sqrt();
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
    let [origin, second_point, third_point] = *triangle;
    let first = subtract(third_point, origin);
    let second = subtract(second_point, origin);
    let offset = subtract(point, origin);
    let first_squared = dot(first, first);
    let first_second = dot(first, second);
    let second_squared = dot(second, second);
    let first_offset = dot(first, offset);
    let second_offset = dot(second, offset);
    let denominator =
        first_squared.mul_add(second_squared, -(first_second * first_second));
    if denominator.abs() <= f32::EPSILON {
        return false;
    }
    let inverse = denominator.recip();
    let first_weight = second_squared
        .mul_add(first_offset, -(first_second * second_offset))
        * inverse;
    let second_weight = first_squared
        .mul_add(second_offset, -(first_second * first_offset))
        * inverse;
    let third_edge = subtract(third_point, second_point);
    let longest_edge = first_squared
        .max(second_squared)
        .max(dot(third_edge, third_edge))
        .sqrt();
    if longest_edge <= f32::EPSILON {
        return false;
    }
    let minimum_altitude = denominator.abs().sqrt() / longest_edge;
    if minimum_altitude <= f32::EPSILON {
        return false;
    }
    let tolerance =
        (INTERIOR_DUPLICATE_TOLERANCE_METERS / minimum_altitude).min(0.05);
    first_weight >= -tolerance
        && second_weight >= -tolerance
        && first_weight + second_weight <= 1. + tolerance
}

/// Subtract one reviewed world-space point from another.
fn subtract(left: [f32; 3], right: [f32; 3]) -> [f32; 3] {
    let [left_x, left_y, left_z] = left;
    let [right_x, right_y, right_z] = right;
    [left_x - right_x, left_y - right_y, left_z - right_z]
}

/// Return one three-dimensional cross product.
fn cross(left: [f32; 3], right: [f32; 3]) -> [f32; 3] {
    let [left_x, left_y, left_z] = left;
    let [right_x, right_y, right_z] = right;
    [
        left_y.mul_add(right_z, -(left_z * right_y)),
        left_z.mul_add(right_x, -(left_x * right_z)),
        left_x.mul_add(right_y, -(left_y * right_x)),
    ]
}

/// Return one three-dimensional dot product.
fn dot(left: [f32; 3], right: [f32; 3]) -> f32 {
    let [left_x, left_y, left_z] = left;
    let [right_x, right_y, right_z] = right;
    left_x.mul_add(right_x, left_y.mul_add(right_y, left_z * right_z))
}

/// Return one point's absolute distance from a normalized plane.
fn point_plane_distance(
    point: [f32; 3],
    plane_point: [f32; 3],
    plane_normal: [f32; 3],
) -> f32 {
    dot(subtract(point, plane_point), plane_normal).abs()
}

/// Compare triangles independent of corner order within reviewed placement
/// noise.
fn triangles_within_tolerance(
    left: &InteriorTriangle,
    right: &InteriorTriangle,
) -> bool {
    let [left_first, left_second, left_third] = *left;
    let [right_first, right_second, right_third] = *right;
    [
        [right_first, right_second, right_third],
        [right_first, right_third, right_second],
        [right_second, right_first, right_third],
        [right_second, right_third, right_first],
        [right_third, right_first, right_second],
        [right_third, right_second, right_first],
    ]
    .into_iter()
    .any(|[first, second, third]| {
        points_within_tolerance(left_first, first)
            && points_within_tolerance(left_second, second)
            && points_within_tolerance(left_third, third)
    })
}

/// Compare two points within the measured reviewed-placement tolerance.
fn points_within_tolerance(left: [f32; 3], right: [f32; 3]) -> bool {
    let delta = subtract(left, right);
    dot(delta, delta)
        <= INTERIOR_DUPLICATE_TOLERANCE_METERS
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

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
//   - Topology domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Topology domain module.
// - Description:
//   - Implements the declared domain module responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Topology domain module.

use super::error::MeshError;

/// Convert decoded indices into triangles.
///
/// # Errors
///
/// Returns an error when the index list is neither a triangle list nor one
/// quad.
pub fn triangulate_indices(
    indices: &[u32],
) -> Result<Vec<[u32; 3]>, MeshError> {
    if indices.is_empty() {
        return Err(MeshError::UnsupportedIndexCount(0));
    }
    let (triangles, remainder) = indices.as_chunks::<3>();
    if remainder.is_empty() {
        return Ok(triangles.to_vec());
    }
    if let [first, second, third, fourth] = indices {
        return Ok(vec![[*first, *second, *third], [*first, *third, *fourth]]);
    }
    Err(MeshError::UnsupportedIndexCount(indices.len()))
}

/// Convert one triangle-strip index stream into triangles.
///
/// Degenerate stitching triangles that repeat a vertex are skipped, and the
/// winding of each emitted triangle follows the strip position parity so the
/// original facing survives the conversion.
///
/// # Errors
///
/// Returns an error when the strip is shorter than one triangle.
pub fn triangulate_strip(indices: &[u32]) -> Result<Vec<[u32; 3]>, MeshError> {
    if indices.len() < 3 {
        return Err(MeshError::UnsupportedIndexCount(indices.len()));
    }
    let mut triangles = Vec::new();
    for (position, window) in indices.windows(3).enumerate() {
        let [first, second, third] = window else {
            continue;
        };
        if first == second || second == third || first == third {
            continue;
        }
        if position % 2 == 0 {
            triangles.push([*first, *second, *third]);
        } else {
            triangles.push([*second, *first, *third]);
        }
    }
    if triangles.is_empty() {
        return Err(MeshError::UnsupportedIndexCount(indices.len()));
    }
    Ok(triangles)
}

/// Align triangle winding with authoritative per-vertex normals.
///
/// Degenerate faces and zero-length normal sums retain their decoded order.
pub(super) fn align_triangle_winding(
    positions: &[[f32; 3]],
    normals: &[[f32; 3]],
    triangles: &mut [[u32; 3]],
) {
    for triangle in triangles {
        let vertex = |index: u32| {
            usize::try_from(index)
                .ok()
                .and_then(|position| positions.get(position))
        };
        let normal = |index: u32| {
            usize::try_from(index)
                .ok()
                .and_then(|position| normals.get(position))
        };
        let (Some(first), Some(second), Some(third)) = (
            vertex(triangle[0]),
            vertex(triangle[1]),
            vertex(triangle[2]),
        ) else {
            continue;
        };
        let (Some(first_normal), Some(second_normal), Some(third_normal)) = (
            normal(triangle[0]),
            normal(triangle[1]),
            normal(triangle[2]),
        ) else {
            continue;
        };
        let first_edge = [
            second[0] - first[0],
            second[1] - first[1],
            second[2] - first[2],
        ];
        let second_edge = [
            third[0] - first[0],
            third[1] - first[1],
            third[2] - first[2],
        ];
        let face_normal = [
            first_edge[1]
                .mul_add(second_edge[2], -first_edge[2] * second_edge[1]),
            first_edge[2]
                .mul_add(second_edge[0], -first_edge[0] * second_edge[2]),
            first_edge[0]
                .mul_add(second_edge[1], -first_edge[1] * second_edge[0]),
        ];
        let authored_normal = [
            first_normal[0] + second_normal[0] + third_normal[0],
            first_normal[1] + second_normal[1] + third_normal[1],
            first_normal[2] + second_normal[2] + third_normal[2],
        ];
        let face_length_squared = face_normal
            .iter()
            .map(|component| component * component)
            .sum::<f32>();
        let authored_length_squared = authored_normal
            .iter()
            .map(|component| component * component)
            .sum::<f32>();
        let alignment = face_normal
            .iter()
            .zip(authored_normal)
            .map(|(face, authored)| face * authored)
            .sum::<f32>();
        if face_length_squared > f32::EPSILON
            && authored_length_squared > f32::EPSILON
            && alignment < 0.
        {
            triangle.swap(1, 2);
        }
    }
}

#[cfg(test)]
#[path = "../../../../../tests/formats/fbx/unit/domain/mesh/topology/tests.rs"]
mod tests;

// File:
//   - topology.rs
// Path:
//   - src/fbx/src/domain/mesh/topology.rs
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
//   - Pure fbx domain rules for domain mesh topology.
// - Must-Not:
//   - Read files, parse generated indexes, invoke CLI code, or call writer
//   - adapters.
// - Allows:
//   - Value objects, invariant checks, and pure evidence-to-domain translation.
// - Split-When:
//   - Split when topology contains two independently testable contracts.
// - Merge-When:
//   - Another fbx module owns the same domain boundary with no distinct
//   - invariant.
// - Summary:
//   - Normalize decoded triangle topology and authored facing.
// - Description:
//   - Defines topology data and behavior for fbx domain mesh.
// - Usage:
//   - Imported through crate domain facades or sibling domain modules.
// - Defaults:
//   - No filesystem paths, no external process calls, and no implicit IO
//   - defaults.
//
// ADRs:
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
// - docs/adr/pipeline/unreal/unreal-manifest-and-package-taxonomy.md
//
// Large file:
//   - false
//

//! Normalize decoded triangle topology and authored facing.
//!
//! This boundary triangulates decoded index streams and aligns geometric face
//! winding with authoritative per-vertex normals.
use super::error::MeshError;

/// Convert decoded indices into triangles.
///
/// # Errors
///
/// Returns an error when the index list is neither a triangle list nor one
/// quad.
pub fn triangulate_indices(
    indices: &[u32]
) -> Result<Vec<[u32; 3]>, MeshError> {
    if indices.is_empty() {
        return Err(MeshError::UnsupportedIndexCount(0));
    }
    let (triangles, remainder) = indices.as_chunks::<3>();
    if remainder.is_empty() {
        return Ok(triangles.to_vec());
    }
    if let [
        first,
        second,
        third,
        fourth,
    ] = indices
    {
        return Ok(
            vec![
                [
                    *first, *second, *third,
                ],
                [
                    *first, *third, *fourth,
                ],
            ],
        );
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
    for (position, window) in indices
        .windows(3)
        .enumerate()
    {
        let [
            first,
            second,
            third,
        ] = window
        else {
            continue;
        };
        if first == second || second == third || first == third {
            continue;
        }
        if position % 2 == 0 {
            triangles.push(
                [
                    *first, *second, *third,
                ],
            );
        } else {
            triangles.push(
                [
                    *second, *first, *third,
                ],
            );
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
            first_edge[1].mul_add(
                second_edge[2],
                -first_edge[2] * second_edge[1],
            ),
            first_edge[2].mul_add(
                second_edge[0],
                -first_edge[0] * second_edge[2],
            ),
            first_edge[0].mul_add(
                second_edge[1],
                -first_edge[1] * second_edge[0],
            ),
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
            && alignment < 0.0
        {
            triangle.swap(
                1, 2,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::triangulate_indices;

    #[test]
    fn empty_index_stream_is_rejected() {
        let result = triangulate_indices(&[]);

        assert_eq!(
            result,
            Err(super::MeshError::UnsupportedIndexCount(0))
        );
    }

    #[test]
    fn quad_triangles_preserve_winding() {
        let result = triangulate_indices(
            &[
                0, 1, 2, 3,
            ],
        );

        assert_eq!(
            result,
            Ok(
                vec![
                    [
                        0, 1, 2
                    ],
                    [
                        0, 2, 3
                    ]
                ]
            )
        );
    }
}

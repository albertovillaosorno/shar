// File:
//   - transform.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/prop_catalog/world_level/
//     transform.rs
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
//   - Row-vector transform composition and geometry baking for level analysis.
// - Must-Not:
//   - Parse scenegraphs, read files, or write FBX bytes.
// - Allows:
//   - Position, normal, and triangle-winding transformation.
// - Summary:
//   - Bakes authored scene matrices into static analysis meshes.
//
// ADRs:
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
//
// Large file:
//   - false
//

//! Row-major row-vector transform helpers for static analysis scenes.

#![expect(
    clippy::arithmetic_side_effects,
    clippy::indexing_slicing,
    clippy::suboptimal_flops,
    reason = "Deterministic matrix math intentionally avoids fused rounding."
)]

use fbx::domain::mesh::MeshAsset;

use crate::domain::PipelineError;

/// Row-major row-vector affine transform.
pub(super) type Matrix = [f32; 16];

/// Return the affine identity transform.
#[must_use]
pub(super) const fn identity() -> Matrix {
    [
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
        0.0, 1.0,
    ]
}

/// Build one translation-only transform.
#[must_use]
pub(super) const fn translation(value: [f32; 3]) -> Matrix {
    let mut matrix = identity();
    matrix[12] = value[0];
    matrix[13] = value[1];
    matrix[14] = value[2];
    matrix
}

/// Multiply row-vector matrices as `first` followed by `second`.
#[must_use]
pub(super) fn multiply(
    first: &Matrix,
    second: &Matrix,
) -> Matrix {
    let mut product = [0.0_f32; 16];
    for row in 0..4 {
        for column in 0..4 {
            let mut sum = 0.0_f32;
            for inner in 0..4 {
                sum += first[row * 4 + inner] * second[inner * 4 + column];
            }
            product[row * 4 + column] = sum;
        }
    }
    product
}

/// Stable matrix identity for placement deduplication.
#[must_use]
pub(super) fn matrix_key(matrix: &Matrix) -> [u32; 16] {
    matrix.map(f32::to_bits)
}

/// Transform one mesh in place and assign its final unique scene identity.
///
/// # Errors
///
/// Returns an error when the affine basis is degenerate or produces non-finite
/// geometry.
pub(super) fn bake_mesh(
    mesh: &mut MeshAsset,
    matrix: &Matrix,
    final_name: String,
) -> Result<(), PipelineError> {
    let determinant = determinant(matrix);
    if !determinant.is_finite() || determinant.abs() <= f32::EPSILON {
        return Err(PipelineError::new("world level transform is degenerate"));
    }
    let normal_matrix = inverse_transpose(
        matrix,
        determinant,
    );
    for group in &mut mesh.groups {
        for position in &mut group.positions {
            *position = transform_position(
                *position, matrix,
            )?;
        }
        for normal in &mut group.normals {
            *normal = transform_normal(
                *normal,
                &normal_matrix,
            )?;
        }
        if determinant < 0.0 {
            for triangle in &mut group.triangles {
                triangle.swap(
                    1, 2,
                );
            }
        }
    }
    mesh.name = final_name;
    Ok(())
}

/// Return one mesh axis-aligned bound after any existing baking.
#[must_use]
pub(super) fn mesh_bounds(
    mesh: &MeshAsset
) -> (
    [f32; 3],
    [f32; 3],
) {
    let mut low = [f32::INFINITY; 3];
    let mut high = [f32::NEG_INFINITY; 3];
    for position in mesh
        .groups
        .iter()
        .flat_map(
            |group| {
                group
                    .positions
                    .iter()
            },
        )
    {
        for axis in 0..3 {
            low[axis] = low[axis].min(position[axis]);
            high[axis] = high[axis].max(position[axis]);
        }
    }
    (
        low, high,
    )
}

/// Transform one position by a row-vector affine matrix.
fn transform_position(
    value: [f32; 3],
    matrix: &Matrix,
) -> Result<[f32; 3], PipelineError> {
    let transformed = [
        value[0] * matrix[0]
            + value[1] * matrix[4]
            + value[2] * matrix[8]
            + matrix[12],
        value[0] * matrix[1]
            + value[1] * matrix[5]
            + value[2] * matrix[9]
            + matrix[13],
        value[0] * matrix[2]
            + value[1] * matrix[6]
            + value[2] * matrix[10]
            + matrix[14],
    ];
    if transformed
        .iter()
        .all(|component| component.is_finite())
    {
        Ok(transformed)
    } else {
        Err(PipelineError::new("world level position became non-finite"))
    }
}

/// Transform and normalize one surface normal.
fn transform_normal(
    value: [f32; 3],
    matrix: &[[f32; 3]; 3],
) -> Result<[f32; 3], PipelineError> {
    let transformed = [
        value[0] * matrix[0][0]
            + value[1] * matrix[1][0]
            + value[2] * matrix[2][0],
        value[0] * matrix[0][1]
            + value[1] * matrix[1][1]
            + value[2] * matrix[2][1],
        value[0] * matrix[0][2]
            + value[1] * matrix[1][2]
            + value[2] * matrix[2][2],
    ];
    let length = (transformed[0] * transformed[0]
        + transformed[1] * transformed[1]
        + transformed[2] * transformed[2])
        .sqrt();
    if !length.is_finite() || length <= f32::EPSILON {
        return Err(PipelineError::new("world level normal became degenerate"));
    }
    Ok(
        [
            transformed[0] / length,
            transformed[1] / length,
            transformed[2] / length,
        ],
    )
}

/// Return the determinant of one affine basis.
fn determinant(matrix: &Matrix) -> f32 {
    matrix[0] * (matrix[5] * matrix[10] - matrix[6] * matrix[9])
        - matrix[1] * (matrix[4] * matrix[10] - matrix[6] * matrix[8])
        + matrix[2] * (matrix[4] * matrix[9] - matrix[5] * matrix[8])
}

/// Build the inverse-transpose normal basis.
fn inverse_transpose(
    matrix: &Matrix,
    determinant: f32,
) -> [[f32; 3]; 3] {
    let inverse = [
        [
            (matrix[5] * matrix[10] - matrix[6] * matrix[9]) / determinant,
            (matrix[2] * matrix[9] - matrix[1] * matrix[10]) / determinant,
            (matrix[1] * matrix[6] - matrix[2] * matrix[5]) / determinant,
        ],
        [
            (matrix[6] * matrix[8] - matrix[4] * matrix[10]) / determinant,
            (matrix[0] * matrix[10] - matrix[2] * matrix[8]) / determinant,
            (matrix[2] * matrix[4] - matrix[0] * matrix[6]) / determinant,
        ],
        [
            (matrix[4] * matrix[9] - matrix[5] * matrix[8]) / determinant,
            (matrix[1] * matrix[8] - matrix[0] * matrix[9]) / determinant,
            (matrix[0] * matrix[5] - matrix[1] * matrix[4]) / determinant,
        ],
    ];
    [
        [
            inverse[0][0],
            inverse[1][0],
            inverse[2][0],
        ],
        [
            inverse[0][1],
            inverse[1][1],
            inverse[2][1],
        ],
        [
            inverse[0][2],
            inverse[1][2],
            inverse[2][2],
        ],
    ]
}

#[cfg(test)]
mod tests {
    use super::{identity, matrix_key, multiply, translation};

    #[test]
    fn row_vector_composition_preserves_translation_order() {
        let first = translation(
            [
                1.0, 2.0, 3.0,
            ],
        );
        let second = translation(
            [
                4.0, 5.0, 6.0,
            ],
        );
        let product = multiply(
            &first, &second,
        );
        assert_eq!(
            product[12..15],
            [
                5.0, 7.0, 9.0
            ]
        );
        assert_eq!(
            matrix_key(
                &multiply(
                    &identity(),
                    &first
                )
            ),
            matrix_key(&first)
        );
    }
}

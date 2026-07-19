// File:
//   - matrix.rs
// Path:
//   - src/pipeline/src/domain/coordinate_movement/matrix.rs
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
//   - Fixed-size row-vector affine matrix primitives.
// - Must-Not:
//   - Assign world meaning, read files, or mutate adapter-owned records.
// - Allows:
//   - Matrix construction, composition, stable keys, and coordinate mapping.
// - Summary:
//   - Pure matrix mechanics for coordinate movement.
//
// ADRs:
// - docs/adr/pipeline/unreal/world-assembly-from-normalized-chunks.md
//
// Large file:
//   - false
//

//! Fixed-size affine matrix mechanics.

use super::{CoordinateMatrix, MovementError};

/// Return one affine identity matrix.
#[must_use]
pub const fn identity_matrix() -> CoordinateMatrix {
    [
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
        0.0, 1.0,
    ]
}

/// Build one translation-only affine matrix.
#[must_use]
pub const fn translation_matrix(value: [f32; 3]) -> CoordinateMatrix {
    let mut matrix = identity_matrix();
    matrix[12] = value[0];
    matrix[13] = value[1];
    matrix[14] = value[2];
    matrix
}

/// Multiply row-vector matrices as `first` followed by `second`.
#[must_use]
pub fn multiply_matrices(
    first: &CoordinateMatrix,
    second: &CoordinateMatrix,
) -> CoordinateMatrix {
    let mut product = [0.0_f32; 16];
    for row in 0..4 {
        for column in 0..4 {
            let mut sum = 0.0_f32;
            for inner in 0..4 {
                sum = first[row * 4 + inner].mul_add(
                    second[inner * 4 + column],
                    sum,
                );
            }
            product[row * 4 + column] = sum;
        }
    }
    product
}

/// Stable matrix identity for placement deduplication.
#[must_use]
pub fn matrix_key(matrix: &CoordinateMatrix) -> [u32; 16] {
    matrix.map(f32::to_bits)
}

/// Transform one position by one row-vector affine matrix.
pub(super) fn transform_point(
    value: [f32; 3],
    matrix: &CoordinateMatrix,
) -> Result<[f32; 3], MovementError> {
    if !value
        .iter()
        .all(|component| component.is_finite())
    {
        return Err(MovementError::NonFiniteCoordinate);
    }
    finite_coordinate(
        [
            value[0].mul_add(
                matrix[0],
                value[1].mul_add(
                    matrix[4],
                    value[2].mul_add(
                        matrix[8], matrix[12],
                    ),
                ),
            ),
            value[0].mul_add(
                matrix[1],
                value[1].mul_add(
                    matrix[5],
                    value[2].mul_add(
                        matrix[9], matrix[13],
                    ),
                ),
            ),
            value[0].mul_add(
                matrix[2],
                value[1].mul_add(
                    matrix[6],
                    value[2].mul_add(
                        matrix[10], matrix[14],
                    ),
                ),
            ),
        ],
    )
}

/// Transform one direction by one affine basis without translation.
pub(super) fn transform_direction(
    value: [f32; 3],
    matrix: &CoordinateMatrix,
) -> Result<[f32; 3], MovementError> {
    if !value
        .iter()
        .all(|component| component.is_finite())
    {
        return Err(MovementError::NonFiniteCoordinate);
    }
    finite_coordinate(
        [
            value[0].mul_add(
                matrix[0],
                value[1].mul_add(
                    matrix[4],
                    value[2] * matrix[8],
                ),
            ),
            value[0].mul_add(
                matrix[1],
                value[1].mul_add(
                    matrix[5],
                    value[2] * matrix[9],
                ),
            ),
            value[0].mul_add(
                matrix[2],
                value[1].mul_add(
                    matrix[6],
                    value[2] * matrix[10],
                ),
            ),
        ],
    )
}

/// Return one finite coordinate unchanged.
fn finite_coordinate(value: [f32; 3]) -> Result<[f32; 3], MovementError> {
    value
        .iter()
        .all(|component| component.is_finite())
        .then_some(value)
        .ok_or(MovementError::NonFiniteCoordinate)
}

/// Return one affine basis determinant.
pub(super) fn determinant(matrix: &CoordinateMatrix) -> f32 {
    let first_minor = matrix[5].mul_add(
        matrix[10],
        -(matrix[6] * matrix[9]),
    );
    let second_minor = matrix[4].mul_add(
        matrix[10],
        -(matrix[6] * matrix[8]),
    );
    let third_minor = matrix[4].mul_add(
        matrix[9],
        -(matrix[5] * matrix[8]),
    );
    matrix[0].mul_add(
        first_minor,
        (-matrix[1]).mul_add(
            second_minor,
            matrix[2] * third_minor,
        ),
    )
}

// File:
//   - affine_inverse.rs
// Path:
//   - src/fbx/src/domain/transform/affine_inverse.rs
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
//   - Pure inversion of finite affine row-vector matrices.
// - Must-Not:
//   - Read files, serialize FBX nodes, or choose scene object identities.
// - Allows:
//   - Checked three-by-three basis inversion and affine translation recovery.
// - Split-When:
//   - General projective inversion becomes a separate supported contract.
// - Merge-When:
//   - The matrix module adopts affine inversion without formatter drift.
// - Summary:
//   - Inverts row-major row-vector affine matrices for FBX bind transforms.
// - Description:
//   - Rejects non-finite, non-affine, and singular matrices before returning
//   - an inverse that composes to identity in both multiplication orders.
// - Usage:
//   - Used by binary FBX cluster serialization and synthetic regressions.
// - Defaults:
//   - Uses the transform domain's established one-thousandth affine tolerance.
//
// ADRs:
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
//
// Large file:
//   - false
//

//! Pure affine inversion for row-major row-vector matrices.
//!
//! The decoded transform domain stores translation in components 12 through
//! 14. This helper preserves that convention and fails explicitly before an
//! invalid bind matrix can reach a binary FBX cluster.

/// Maximum tolerated deviation from the affine identity column.
const AFFINE_TOLERANCE: f64 = 1e-3;

/// Affine matrix inversion failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InverseError {
    /// One matrix component was not finite.
    NonFiniteComponent {
        /// Row-major component index.
        component: usize,
    },
    /// The final column was not the affine identity column.
    NotAffine,
    /// The three-by-three basis was singular.
    Singular,
}

/// Invert one finite affine row-major row-vector matrix.
///
/// # Errors
///
/// Returns an error when the matrix is non-finite, non-affine, or has a
/// singular three-by-three basis.
// Explicit cofactor terms stay in textbook order so reviewers can compare
// every matrix component directly with the deterministic inverse contract.
#[expect(
    clippy::suboptimal_flops,
    reason = "Cofactor formulas preserve explicit affine matrix inversion \
              order"
)]
pub fn invert_affine(matrix: &[f64; 16]) -> Result<[f64; 16], InverseError> {
    if let Some(component) = matrix
        .iter()
        .position(|value| !value.is_finite())
    {
        return Err(
            InverseError::NonFiniteComponent {
                component,
            },
        );
    }
    if matrix[3].abs() > AFFINE_TOLERANCE
        || matrix[7].abs() > AFFINE_TOLERANCE
        || matrix[11].abs() > AFFINE_TOLERANCE
        || (matrix[15] - 1.0).abs() > AFFINE_TOLERANCE
    {
        return Err(InverseError::NotAffine);
    }
    let basis_00 = matrix[0];
    let basis_01 = matrix[1];
    let basis_02 = matrix[2];
    let basis_10 = matrix[4];
    let basis_11 = matrix[5];
    let basis_12 = matrix[6];
    let basis_20 = matrix[8];
    let basis_21 = matrix[9];
    let basis_22 = matrix[10];
    let determinant = basis_00 * (basis_11 * basis_22 - basis_12 * basis_21)
        - basis_01 * (basis_10 * basis_22 - basis_12 * basis_20)
        + basis_02 * (basis_10 * basis_21 - basis_11 * basis_20);
    if determinant.abs() <= AFFINE_TOLERANCE {
        return Err(InverseError::Singular);
    }
    let reciprocal = 1.0_f64 / determinant;
    let inverse_basis = [
        (basis_11 * basis_22 - basis_12 * basis_21) * reciprocal,
        (basis_02 * basis_21 - basis_01 * basis_22) * reciprocal,
        (basis_01 * basis_12 - basis_02 * basis_11) * reciprocal,
        (basis_12 * basis_20 - basis_10 * basis_22) * reciprocal,
        (basis_00 * basis_22 - basis_02 * basis_20) * reciprocal,
        (basis_02 * basis_10 - basis_00 * basis_12) * reciprocal,
        (basis_10 * basis_21 - basis_11 * basis_20) * reciprocal,
        (basis_01 * basis_20 - basis_00 * basis_21) * reciprocal,
        (basis_00 * basis_11 - basis_01 * basis_10) * reciprocal,
    ];
    let translation = [
        matrix[12], matrix[13], matrix[14],
    ];
    let inverse_translation = [
        -(translation[0] * inverse_basis[0]
            + translation[1] * inverse_basis[3]
            + translation[2] * inverse_basis[6]),
        -(translation[0] * inverse_basis[1]
            + translation[1] * inverse_basis[4]
            + translation[2] * inverse_basis[7]),
        -(translation[0] * inverse_basis[2]
            + translation[1] * inverse_basis[5]
            + translation[2] * inverse_basis[8]),
    ];
    Ok(
        [
            inverse_basis[0],
            inverse_basis[1],
            inverse_basis[2],
            0.0,
            inverse_basis[3],
            inverse_basis[4],
            inverse_basis[5],
            0.0,
            inverse_basis[6],
            inverse_basis[7],
            inverse_basis[8],
            0.0,
            inverse_translation[0],
            inverse_translation[1],
            inverse_translation[2],
            1.0,
        ],
    )
}

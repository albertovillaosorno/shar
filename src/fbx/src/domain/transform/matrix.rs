// File:
//   - matrix.rs
// Path:
//   - src/fbx/src/domain/transform/matrix.rs
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
//   - Pure fbx domain rules for domain transform matrix.
// - Must-Not:
//   - Read files, parse generated indexes, invoke CLI code, or call writer
//   - adapters.
// - Allows:
//   - Value objects, invariant checks, and pure evidence-to-domain translation.
// - Split-When:
//   - Split when matrix contains two independently testable contracts.
// - Merge-When:
//   - Another fbx module owns the same domain boundary with no distinct
//   - invariant.
// - Summary:
//   - Row-major row-vector matrix composition and TRS decomposition.
// - Description:
//   - Defines deterministic matrix behavior for fbx domain transform.
// - Usage:
//   - Imported through crate domain facades or sibling domain modules.
// - Defaults:
//   - No filesystem paths, no external process calls, and no implicit IO
//   - defaults.
//
// ADRs:
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
//
// Large file:
//   - false
//

//! Row-major row-vector matrix composition and TRS decomposition.
//!
//! Decoded rest poses and FBX both store row-major matrices under the
//! row-vector convention, with translation in components 12 through 14, so
//! composition multiplies the local matrix by the parent global matrix
//! without any transposition.
// Matrix arithmetic operates on fixed 16-component arrays whose indices are
// compile-time constants, so no arithmetic can leave the array bounds.
#![expect(
    clippy::arithmetic_side_effects,
    reason = "Fixed-size matrix arithmetic uses constant indices and finite \
              validated components."
)]
// All matrix indices below are compile-time constants inside fixed
// sixteen-component arrays, so indexing cannot leave the array bounds.
#![expect(
    clippy::indexing_slicing,
    reason = "All matrix indices are compile-time constants inside fixed \
              sixteen-component arrays."
)]
// Expanded matrix products and decomposition formulas remain visible for direct
// review against row-vector equations; all numeric literals infer f64 locally.
#![expect(
    clippy::default_numeric_fallback,
    clippy::suboptimal_flops,
    reason = "Expanded f64 matrix equations preserve direct deterministic \
              review and parity."
)]

/// Maximum tolerated deviation for affine and orthonormal checks.
const ORTHONORMAL_TOLERANCE: f64 = 1e-3;

/// Translation, rotation, and scale parts decomposed from one local matrix.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TrsParts {
    /// Translation vector.
    pub translation: [f64; 3],
    /// Euler rotation in degrees applied in X, then Y, then Z order.
    pub rotation_degrees: [f64; 3],
    /// Per-axis scale factors.
    pub scale: [f64; 3],
}

/// Matrix decomposition failure.
// The explicit public name distinguishes decomposition errors from other
// domains.
#[expect(
    clippy::module_name_repetitions,
    reason = "MatrixError is the stable public transform failure contract \
              name."
)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MatrixError {
    /// One matrix component was not finite.
    NonFiniteComponent {
        /// Row-major component index.
        component: usize,
    },
    /// The last matrix column was not the affine identity column.
    NotAffine,
    /// One basis row collapsed below usable scale.
    DegenerateScale {
        /// Row index with the degenerate scale.
        row: usize,
    },
    /// The rotation basis was mirrored and cannot map to FBX rotation.
    NegativeDeterminant,
    /// The rotation basis carried shear beyond tolerance.
    NonOrthonormalRotation,
}

/// Widen one decoded row-major matrix to double precision.
#[must_use]
pub fn widen(matrix: &[f32; 16]) -> [f64; 16] {
    let mut wide = [0.0_f64; 16];
    for (target, value) in wide
        .iter_mut()
        .zip(matrix.iter())
    {
        *target = f64::from(*value);
    }
    wide
}

/// Multiply two row-major row-vector matrices as `first` then `second`.
#[must_use]
pub fn multiply(
    first: &[f64; 16],
    second: &[f64; 16],
) -> [f64; 16] {
    let mut product = [0.0_f64; 16];
    for row in 0..4 {
        for column in 0..4 {
            let mut sum = 0.0;
            for inner in 0..4 {
                sum += first[row * 4 + inner] * second[inner * 4 + column];
            }
            product[row * 4 + column] = sum;
        }
    }
    product
}

/// Compose one row-major row-vector matrix from decomposed TRS parts.
#[must_use]
pub fn compose(parts: &TrsParts) -> [f64; 16] {
    let radians = [
        parts.rotation_degrees[0].to_radians(),
        parts.rotation_degrees[1].to_radians(),
        parts.rotation_degrees[2].to_radians(),
    ];
    let (sin_x, cos_x) = radians[0].sin_cos();
    let (sin_y, cos_y) = radians[1].sin_cos();
    let (sin_z, cos_z) = radians[2].sin_cos();
    let rotation_x = [
        1.0, 0.0, 0.0, 0.0, 0.0, cos_x, sin_x, 0.0, 0.0, -sin_x, cos_x, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ];
    let rotation_y = [
        cos_y, 0.0, -sin_y, 0.0, 0.0, 1.0, 0.0, 0.0, sin_y, 0.0, cos_y, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ];
    let rotation_z = [
        cos_z, sin_z, 0.0, 0.0, -sin_z, cos_z, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ];
    let mut matrix = multiply(
        &multiply(
            &rotation_x,
            &rotation_y,
        ),
        &rotation_z,
    );
    for row in 0..3 {
        for column in 0..3 {
            matrix[row * 4 + column] *= parts.scale[row];
        }
    }
    matrix[12] = parts.translation[0];
    matrix[13] = parts.translation[1];
    matrix[14] = parts.translation[2];
    matrix
}

/// Decompose one row-major row-vector local matrix into TRS parts.
///
/// # Errors
///
/// Returns an error when the matrix is not finite, not affine, mirrored,
/// degenerate, or sheared beyond tolerance.
pub fn decompose(matrix: &[f64; 16]) -> Result<TrsParts, MatrixError> {
    if let Some(component) = matrix
        .iter()
        .position(|value| !value.is_finite())
    {
        return Err(
            MatrixError::NonFiniteComponent {
                component,
            },
        );
    }
    if matrix[3].abs() > ORTHONORMAL_TOLERANCE
        || matrix[7].abs() > ORTHONORMAL_TOLERANCE
        || matrix[11].abs() > ORTHONORMAL_TOLERANCE
        || (matrix[15] - 1.0).abs() > ORTHONORMAL_TOLERANCE
    {
        return Err(MatrixError::NotAffine);
    }
    let mut scale = [0.0_f64; 3];
    let mut basis = [[0.0_f64; 3]; 3];
    for row in 0..3 {
        let vector = [
            matrix[row * 4],
            matrix[row * 4 + 1],
            matrix[row * 4 + 2],
        ];
        let length = (vector[0] * vector[0]
            + vector[1] * vector[1]
            + vector[2] * vector[2])
            .sqrt();
        if length <= ORTHONORMAL_TOLERANCE {
            return Err(
                MatrixError::DegenerateScale {
                    row,
                },
            );
        }
        scale[row] = length;
        basis[row] = [
            vector[0] / length,
            vector[1] / length,
            vector[2] / length,
        ];
    }
    let determinant = basis[0][0]
        * (basis[1][1] * basis[2][2] - basis[1][2] * basis[2][1])
        - basis[0][1] * (basis[1][0] * basis[2][2] - basis[1][2] * basis[2][0])
        + basis[0][2] * (basis[1][0] * basis[2][1] - basis[1][1] * basis[2][0]);
    if determinant <= 0.0 {
        return Err(MatrixError::NegativeDeterminant);
    }
    for row in 0..3 {
        for other in 0..3 {
            let dot = basis[row][0] * basis[other][0]
                + basis[row][1] * basis[other][1]
                + basis[row][2] * basis[other][2];
            let expected = if row == other {
                1.0
            } else {
                0.0
            };
            if (dot - expected).abs() > ORTHONORMAL_TOLERANCE {
                return Err(MatrixError::NonOrthonormalRotation);
            }
        }
    }
    Ok(
        TrsParts {
            translation: [
                matrix[12], matrix[13], matrix[14],
            ],
            rotation_degrees: euler_degrees(&basis),
            scale,
        },
    )
}

/// Extract X-then-Y-then-Z euler angles in degrees from one rotation basis.
fn euler_degrees(basis: &[[f64; 3]; 3]) -> [f64; 3] {
    let sin_y = -basis[0][2];
    if sin_y.abs() < 1.0 - 1e-6 {
        [
            basis[1][2]
                .atan2(basis[2][2])
                .to_degrees(),
            sin_y
                .asin()
                .to_degrees(),
            basis[0][1]
                .atan2(basis[0][0])
                .to_degrees(),
        ]
    } else {
        let clamped = sin_y.clamp(
            -1.0, 1.0,
        );
        [
            (basis[1][0] * clamped)
                .atan2(basis[1][1])
                .to_degrees(),
            clamped
                .asin()
                .to_degrees(),
            0.0,
        ]
    }
}

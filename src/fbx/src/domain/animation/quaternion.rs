// File:
//   - quaternion.rs
// Path:
//   - src/fbx/src/domain/animation/quaternion.rs
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
//   - Pure WXYZ quaternion normalization, interpolation, and matrix conversion.
// - Must-Not:
//   - Parse source chunks, choose animation targets, or emit writer objects.
// - Allows:
//   - Deterministic finite arithmetic for animation transform reconstruction.
// - Split-When:
//   - Additional rotation representations require independent invariants.
// - Merge-When:
//   - The transform matrix domain owns identical quaternion operations.
// - Summary:
//   - Quaternion math shared by decoded animation and FBX writing.
// - Description:
//   - Implements signed-normalized source decoding and shortest-path slerp.
// - Usage:
//   - Used by animation adapters and writer-side Euler conversion.
// - Defaults:
//   - Quaternion component order is W, X, Y, Z.
//
// ADRs:
// - docs/adr/fbx/export/fbx-output-contract-boundary.md
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
//
// Large file:
//   - false
//

//! Quaternion math shared by decoded animation and FBX writing.

// The formulas intentionally remain expanded for direct comparison with the
// audited WXYZ quaternion and row-matrix equations. All literals infer f64 from
// explicit signatures and operands, so fused rewrites would obscure the model.
#![expect(
    clippy::default_numeric_fallback,
    clippy::suboptimal_flops,
    reason = "Expanded f64 quaternion formulas preserve directly auditable \
              equations."
)]

/// Quaternion-domain failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    /// A quaternion component was non-finite.
    NonFinite,
    /// Quaternion magnitude was too small to normalize.
    Degenerate,
}

/// Decode four source words as signed normalized WXYZ components.
///
/// # Errors
///
/// Returns an error when the decoded quaternion cannot be normalized.
pub fn decode_signed_i16_wxyz(values: [u16; 4]) -> Result<[f64; 4], Error> {
    normalize(
        [
            f64::from(i16::from_le_bytes(values[0].to_le_bytes())) / 32_767.0,
            f64::from(i16::from_le_bytes(values[1].to_le_bytes())) / 32_767.0,
            f64::from(i16::from_le_bytes(values[2].to_le_bytes())) / 32_767.0,
            f64::from(i16::from_le_bytes(values[3].to_le_bytes())) / 32_767.0,
        ],
    )
}

/// Normalize one WXYZ quaternion.
///
/// # Errors
///
/// Returns an error when a component is non-finite or the magnitude collapses.
pub fn normalize(values: [f64; 4]) -> Result<[f64; 4], Error> {
    if values
        .iter()
        .any(|value| !value.is_finite())
    {
        return Err(Error::NonFinite);
    }
    let length = values
        .iter()
        .map(|value| value * value)
        .sum::<f64>()
        .sqrt();
    if length <= f64::EPSILON {
        return Err(Error::Degenerate);
    }
    Ok(
        [
            values[0] / length,
            values[1] / length,
            values[2] / length,
            values[3] / length,
        ],
    )
}

/// Interpolate two unit WXYZ quaternions along the shortest spherical path.
#[must_use]
pub fn slerp(
    start: [f64; 4],
    mut end: [f64; 4],
    amount: f64,
) -> [f64; 4] {
    let mut dot = start
        .iter()
        .zip(end.iter())
        .map(|(left, right)| left * right)
        .sum::<f64>();
    if dot < 0.0 {
        for component in &mut end {
            *component = -*component;
        }
        dot = -dot;
    }
    if dot > 0.9995 {
        return normalize(
            [
                start[0] + amount * (end[0] - start[0]),
                start[1] + amount * (end[1] - start[1]),
                start[2] + amount * (end[2] - start[2]),
                start[3] + amount * (end[3] - start[3]),
            ],
        )
        .unwrap_or(start);
    }
    let theta = dot
        .clamp(
            -1.0, 1.0,
        )
        .acos();
    let sin_theta = theta.sin();
    if sin_theta.abs() <= f64::EPSILON {
        return start;
    }
    let start_weight = ((1.0 - amount) * theta).sin() / sin_theta;
    let end_weight = (amount * theta).sin() / sin_theta;
    normalize(
        [
            start[0] * start_weight + end[0] * end_weight,
            start[1] * start_weight + end[1] * end_weight,
            start[2] * start_weight + end[2] * end_weight,
            start[3] * start_weight + end[3] * end_weight,
        ],
    )
    .unwrap_or(start)
}

/// Convert one unit WXYZ quaternion to a row-major row-vector matrix.
#[must_use]
pub fn to_row_matrix(
    quaternion: [f64; 4],
    translation: [f64; 3],
) -> [f64; 16] {
    let [
        w,
        x,
        y,
        z,
    ] = quaternion;
    let xx = x * x;
    let yy = y * y;
    let zz = z * z;
    let xy = x * y;
    let xz = x * z;
    let yz = y * z;
    let wx = w * x;
    let wy = w * y;
    let wz = w * z;
    [
        1.0 - 2.0 * (yy + zz),
        2.0 * (xy + wz),
        2.0 * (xz - wy),
        0.0,
        2.0 * (xy - wz),
        1.0 - 2.0 * (xx + zz),
        2.0 * (yz + wx),
        0.0,
        2.0 * (xz + wy),
        2.0 * (yz - wx),
        1.0 - 2.0 * (xx + yy),
        0.0,
        translation[0],
        translation[1],
        translation[2],
        1.0,
    ]
}

/// Convert one orthonormal row-major row-vector matrix to a WXYZ quaternion.
///
/// # Errors
///
/// Returns an error when the derived quaternion cannot be normalized.
pub fn from_row_matrix(matrix: &[f64; 16]) -> Result<[f64; 4], Error> {
    let trace = matrix[0] + matrix[5] + matrix[10];
    let quaternion = if trace > 0.0 {
        let scale = (trace + 1.0).sqrt() * 2.0;
        [
            0.25 * scale,
            (matrix[6] - matrix[9]) / scale,
            (matrix[8] - matrix[2]) / scale,
            (matrix[1] - matrix[4]) / scale,
        ]
    } else if matrix[0] > matrix[5] && matrix[0] > matrix[10] {
        let scale = (1.0 + matrix[0] - matrix[5] - matrix[10]).sqrt() * 2.0;
        [
            (matrix[6] - matrix[9]) / scale,
            0.25 * scale,
            (matrix[1] + matrix[4]) / scale,
            (matrix[2] + matrix[8]) / scale,
        ]
    } else if matrix[5] > matrix[10] {
        let scale = (1.0 + matrix[5] - matrix[0] - matrix[10]).sqrt() * 2.0;
        [
            (matrix[8] - matrix[2]) / scale,
            (matrix[1] + matrix[4]) / scale,
            0.25 * scale,
            (matrix[6] + matrix[9]) / scale,
        ]
    } else {
        let scale = (1.0 + matrix[10] - matrix[0] - matrix[5]).sqrt() * 2.0;
        [
            (matrix[1] - matrix[4]) / scale,
            (matrix[2] + matrix[8]) / scale,
            (matrix[6] + matrix[9]) / scale,
            0.25 * scale,
        ]
    };
    normalize(quaternion)
}

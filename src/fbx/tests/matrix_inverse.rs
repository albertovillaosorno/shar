// File:
//   - matrix_inverse.rs
// Path:
//   - src/fbx/tests/matrix_inverse.rs
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
//   - Regression coverage for affine row-vector matrix inversion.
// - Must-Not:
//   - Read files, invoke adapters, or depend on extracted game evidence.
// - Allows:
//   - Synthetic TRS matrices, singular matrices, and tolerance assertions.
// - Split-When:
//   - Matrix decomposition or interpolation needs an independent fixture set.
// - Merge-When:
//   - Transform algebra regressions move into one shared conformance suite.
// - Summary:
//   - Protects inverse bind-matrix construction for binary FBX clusters.
// - Description:
//   - Verifies both multiplication orders and deterministic singular rejection.
// - Usage:
//   - Run through the fbx crate test target.
// - Defaults:
//   - Comparisons use a fixed numerical tolerance and no external inputs.
//
// ADRs:
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
//
// Large file:
//   - false
//

//! Regression coverage for affine row-vector matrix inversion.
//!
//! Synthetic transforms prove both multiplication orders return identity and
//! that singular bases fail explicitly before cluster bind matrices are
//! serialized. The test uses no private assets or machine-local evidence.

use fbx::domain::transform::affine_inverse::{InverseError, invert_affine};
use fbx::domain::transform::matrix::{TrsParts, compose, multiply};
use png as _;
use schoenwald_filesystem as _;
use serde as _;
use serde_json as _;
use shar_sha256 as _;

const TOLERANCE: f64 = 1e-9;
const IDENTITY: [f64; 16] = [
    1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0,
    1.0,
];

fn assert_matrix_close(
    actual: &[f64; 16],
    expected: &[f64; 16],
) {
    for (index, (actual_value, expected_value)) in actual
        .iter()
        .zip(expected)
        .enumerate()
    {
        assert!(
            (actual_value - expected_value).abs() <= TOLERANCE,
            "component {index} differed: actual={actual_value} \
             expected={expected_value}"
        );
    }
}

#[test]
fn affine_inverse_round_trips_in_both_row_vector_orders() {
    let matrix = compose(
        &TrsParts {
            translation: [
                2.5_f64, -3.75_f64, 9.125_f64,
            ],
            rotation_degrees: [
                27.0_f64, -41.0_f64, 83.0_f64,
            ],
            scale: [
                1.25_f64, 0.75_f64, 2.5_f64,
            ],
        },
    );

    let inverse_result = invert_affine(&matrix);
    assert!(
        inverse_result.is_ok(),
        "synthetic affine matrix should be invertible: {inverse_result:?}"
    );
    let Some(inverse) = inverse_result.ok() else {
        return;
    };

    assert_matrix_close(
        &multiply(
            &matrix, &inverse,
        ),
        &IDENTITY,
    );
    assert_matrix_close(
        &multiply(
            &inverse, &matrix,
        ),
        &IDENTITY,
    );
}

#[test]
fn affine_inverse_rejects_singular_basis() {
    let singular: [f64; 16] = [
        1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
        0.0, 1.0,
    ];

    assert_eq!(
        invert_affine(&singular),
        Err(InverseError::Singular)
    );
}

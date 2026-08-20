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
//   - Matrix inverse test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Matrix inverse test module.
// - Description:
//   - Implements the declared test module responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Matrix inverse test module.

use fbx::domain::transform::affine_inverse::{InverseError, invert_affine};
use fbx::domain::transform::matrix::{TrsParts, compose, multiply};
use png as _;
use schoenwald_filesystem as _;
use serde as _;
use serde_json as _;
use shar_sha256 as _;

const TOLERANCE: f64 = 1e-9;
const IDENTITY: [f64; 16] = [
    1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
];

fn assert_matrix_close(actual: &[f64; 16], expected: &[f64; 16]) {
    for (index, (actual_value, expected_value)) in
        actual.iter().zip(expected).enumerate()
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
    let matrix = compose(&TrsParts {
        translation: [2.5_f64, -3.75_f64, 9.125_f64],
        rotation_degrees: [27f64, -41f64, 83f64],
        scale: [1.25_f64, 0.75_f64, 2.5_f64],
    });

    let inverse_result = invert_affine(&matrix);
    assert!(
        inverse_result.is_ok(),
        "synthetic affine matrix should be invertible: {inverse_result:?}"
    );
    let Some(inverse) = inverse_result.ok() else {
        return;
    };

    assert_matrix_close(&multiply(&matrix, &inverse), &IDENTITY);
    assert_matrix_close(&multiply(&inverse, &matrix), &IDENTITY);
}

#[test]
fn affine_inverse_rejects_singular_basis() {
    let singular: [f64; 16] = [
        1., 0., 0., 0., 0., 0., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
    ];

    assert_eq!(invert_affine(&singular), Err(InverseError::Singular));
}

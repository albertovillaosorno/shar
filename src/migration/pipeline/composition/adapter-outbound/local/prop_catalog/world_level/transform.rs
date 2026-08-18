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
//   - Transform outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Transform outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Transform outbound adapter.

#![expect(
    clippy::arithmetic_side_effects,
    clippy::indexing_slicing,
    reason = "Fixed-size source matrix math uses bounded 4x4 indices."
)]

use fbx::domain::mesh::MeshAsset;

use crate::domain::PipelineError;

/// Row-major row-vector affine transform.
pub(super) type Matrix = [f32; 16];

/// Return one affine identity matrix.
#[must_use]
pub(super) const fn identity() -> Matrix {
    [
        1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
    ]
}

/// Build one source-authored translation matrix.
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
pub(super) fn multiply(first: &Matrix, second: &Matrix) -> Matrix {
    let mut product = [0f32; 16];
    for row in 0..4 {
        for column in 0..4 {
            let mut sum = 0f32;
            for inner in 0..4 {
                sum = first[row * 4 + inner]
                    .mul_add(second[inner * 4 + column], sum);
            }
            product[row * 4 + column] = sum;
        }
    }
    product
}

/// Stable source-matrix identity for transform unit evidence.
#[cfg(test)]
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
    let normal_matrix = inverse_transpose(matrix, determinant);
    for group in &mut mesh.groups {
        for position in &mut group.positions {
            *position = transform_position(*position, matrix)?;
        }
        for normal in &mut group.normals {
            *normal = transform_normal(*normal, &normal_matrix)?;
        }
        if determinant < 0. {
            for triangle in &mut group.triangles {
                triangle.swap(1, 2);
            }
        }
    }
    mesh.name = final_name;
    Ok(())
}

/// Return one mesh axis-aligned bound after any existing baking.
#[must_use]
pub(super) fn mesh_bounds(mesh: &MeshAsset) -> ([f32; 3], [f32; 3]) {
    let mut low = [f32::INFINITY; 3];
    let mut high = [f32::NEG_INFINITY; 3];
    for position in mesh.groups.iter().flat_map(|group| group.positions.iter())
    {
        for axis in 0..3 {
            low[axis] = low[axis].min(position[axis]);
            high[axis] = high[axis].max(position[axis]);
        }
    }
    (low, high)
}

/// Transform one position by a row-vector affine matrix.
fn transform_position(
    value: [f32; 3],
    matrix: &Matrix,
) -> Result<[f32; 3], PipelineError> {
    let transformed = [
        value[0].mul_add(
            matrix[0],
            value[1]
                .mul_add(matrix[4], value[2].mul_add(matrix[8], matrix[12])),
        ),
        value[0].mul_add(
            matrix[1],
            value[1]
                .mul_add(matrix[5], value[2].mul_add(matrix[9], matrix[13])),
        ),
        value[0].mul_add(
            matrix[2],
            value[1]
                .mul_add(matrix[6], value[2].mul_add(matrix[10], matrix[14])),
        ),
    ];
    if transformed.iter().all(|component| component.is_finite()) {
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
        value[0].mul_add(
            matrix[0][0],
            value[1].mul_add(matrix[1][0], value[2] * matrix[2][0]),
        ),
        value[0].mul_add(
            matrix[0][1],
            value[1].mul_add(matrix[1][1], value[2] * matrix[2][1]),
        ),
        value[0].mul_add(
            matrix[0][2],
            value[1].mul_add(matrix[1][2], value[2] * matrix[2][2]),
        ),
    ];
    let length_squared = transformed[0].mul_add(
        transformed[0],
        transformed[1].mul_add(transformed[1], transformed[2] * transformed[2]),
    );
    let length = length_squared.sqrt();
    if !length.is_finite() || length <= f32::EPSILON {
        return Err(PipelineError::new("world level normal became degenerate"));
    }
    Ok([
        transformed[0] / length,
        transformed[1] / length,
        transformed[2] / length,
    ])
}

/// Return the determinant of one affine basis.
fn determinant(matrix: &Matrix) -> f32 {
    let first_minor = matrix[5].mul_add(matrix[10], -(matrix[6] * matrix[9]));
    let second_minor = matrix[4].mul_add(matrix[10], -(matrix[6] * matrix[8]));
    let third_minor = matrix[4].mul_add(matrix[9], -(matrix[5] * matrix[8]));
    matrix[0].mul_add(
        first_minor,
        (-matrix[1]).mul_add(second_minor, matrix[2] * third_minor),
    )
}

/// Build the inverse-transpose normal basis.
fn inverse_transpose(matrix: &Matrix, determinant: f32) -> [[f32; 3]; 3] {
    let inverse = [
        [
            matrix[5].mul_add(matrix[10], -(matrix[6] * matrix[9]))
                / determinant,
            matrix[2].mul_add(matrix[9], -(matrix[1] * matrix[10]))
                / determinant,
            matrix[1].mul_add(matrix[6], -(matrix[2] * matrix[5]))
                / determinant,
        ],
        [
            matrix[6].mul_add(matrix[8], -(matrix[4] * matrix[10]))
                / determinant,
            matrix[0].mul_add(matrix[10], -(matrix[2] * matrix[8]))
                / determinant,
            matrix[2].mul_add(matrix[4], -(matrix[0] * matrix[6]))
                / determinant,
        ],
        [
            matrix[4].mul_add(matrix[9], -(matrix[5] * matrix[8]))
                / determinant,
            matrix[1].mul_add(matrix[8], -(matrix[0] * matrix[9]))
                / determinant,
            matrix[0].mul_add(matrix[5], -(matrix[1] * matrix[4]))
                / determinant,
        ],
    ];
    [
        [inverse[0][0], inverse[1][0], inverse[2][0]],
        [inverse[0][1], inverse[1][1], inverse[2][1]],
        [inverse[0][2], inverse[1][2], inverse[2][2]],
    ]
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/prop_catalog/world_level/transform/tests.rs"]
mod tests;

// File:
//   - coordinate_movement.rs
// Path:
//   - src/pipeline/src/domain/coordinate_movement.rs
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
//   - Pure affine coordinate movement shared by every world-record family.
// - Must-Not:
//   - Read files, classify packages, mutate meshes, or serialize manifests.
// - Allows:
//   - Transform points, directions, placements, and axis-aligned bounds.
// - Summary:
//   - One coordinate movement contract above geometry-specific adapters.
//
// ADRs:
// - docs/adr/pipeline/unreal/world-assembly-from-normalized-chunks.md
//
// Large file:
//   - false
//

//! Pure affine movement for geometry and non-geometry world coordinates.

#![expect(
    clippy::arithmetic_side_effects,
    clippy::indexing_slicing,
    reason = "Deterministic affine math uses fixed-size row-vector matrices."
)]

use std::fmt;

/// Row-major row-vector affine matrix.
pub type CoordinateMatrix = [f32; 16];

/// World-record families that must remain aligned under one movement.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum CoordinateSubject {
    /// Render geometry positions and normals.
    Geometry,
    /// Collision geometry and physics primitives.
    Collision,
    /// Movable or static door placements and pivots.
    Door,
    /// General object or prop placements.
    ObjectPlacement,
    /// Character appearance or respawn locations.
    CharacterSpawn,
    /// Object, vehicle, or collectible spawn locations.
    ObjectSpawn,
    /// Mission-owned positions, volumes, and directional records.
    MissionPlacement,
    /// Trigger positions, scales, and placement matrices.
    Trigger,
    /// Camera positions and directional bases.
    Camera,
    /// Generic, action, and directional locator records.
    Locator,
    /// Authored light positions and directions.
    Light,
}

impl CoordinateSubject {
    /// Return the stable manifest identity for this coordinate family.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Geometry => "geometry",
            Self::Collision => "collision",
            Self::Door => "door",
            Self::ObjectPlacement => "object-placement",
            Self::CharacterSpawn => "character-spawn",
            Self::ObjectSpawn => "object-spawn",
            Self::MissionPlacement => "mission-placement",
            Self::Trigger => "trigger",
            Self::Camera => "camera",
            Self::Locator => "locator",
            Self::Light => "light",
        }
    }
}

/// One named affine movement applied consistently across world-record families.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CoordinateMovement {
    /// Stable product-facing movement identity.
    id: &'static str,
    /// Row-vector affine transform.
    matrix: CoordinateMatrix,
    /// Record families owned by this movement contract.
    subjects: &'static [CoordinateSubject],
}

impl CoordinateMovement {
    /// Construct one statically authored movement.
    #[must_use]
    pub const fn new(
        id: &'static str,
        matrix: CoordinateMatrix,
        subjects: &'static [CoordinateSubject],
    ) -> Self {
        Self {
            id,
            matrix,
            subjects,
        }
    }

    /// Return the stable movement identity.
    #[must_use]
    pub const fn id(self) -> &'static str {
        self.id
    }

    /// Return the complete affine matrix.
    #[must_use]
    pub const fn matrix(self) -> CoordinateMatrix {
        self.matrix
    }

    /// Return every coordinate family covered by the movement.
    #[must_use]
    pub const fn subjects(self) -> &'static [CoordinateSubject] {
        self.subjects
    }

    /// Return the affine basis determinant.
    #[must_use]
    pub fn determinant(self) -> f32 {
        determinant(&self.matrix)
    }

    /// Validate finite, non-degenerate affine movement state.
    ///
    /// # Errors
    ///
    /// Returns an error for empty identity, non-finite values, non-affine
    /// matrices, degenerate bases, or an empty subject contract.
    pub fn validate(self) -> Result<(), MovementError> {
        if self
            .id
            .is_empty()
        {
            return Err(MovementError::MissingIdentity);
        }
        if self
            .subjects
            .is_empty()
        {
            return Err(MovementError::MissingSubjects);
        }
        if !self
            .matrix
            .iter()
            .all(|value| value.is_finite())
        {
            return Err(MovementError::NonFiniteMatrix);
        }
        if self.matrix[3].abs() > f32::EPSILON
            || self.matrix[7].abs() > f32::EPSILON
            || self.matrix[11].abs() > f32::EPSILON
            || (self.matrix[15] - 1.0).abs() > f32::EPSILON
        {
            return Err(MovementError::NonAffineMatrix);
        }
        let basis_determinant = self.determinant();
        if !basis_determinant.is_finite()
            || basis_determinant.abs() <= f32::EPSILON
        {
            return Err(MovementError::DegenerateBasis);
        }
        Ok(())
    }

    /// Transform one position, including translation.
    ///
    /// # Errors
    ///
    /// Returns an error when the input or output is non-finite.
    pub fn transform_point(
        self,
        value: [f32; 3],
    ) -> Result<[f32; 3], MovementError> {
        transform_point(
            value,
            &self.matrix,
        )
    }

    /// Transform one direction without translation.
    ///
    /// # Errors
    ///
    /// Returns an error when the input or output is non-finite.
    pub fn transform_direction(
        self,
        value: [f32; 3],
    ) -> Result<[f32; 3], MovementError> {
        transform_direction(
            value,
            &self.matrix,
        )
    }

    /// Compose one authored placement followed by this movement.
    #[must_use]
    pub fn transform_placement(
        self,
        placement: &CoordinateMatrix,
    ) -> CoordinateMatrix {
        multiply_matrices(
            placement,
            &self.matrix,
        )
    }

    /// Transform all eight corners of one axis-aligned bound.
    ///
    /// # Errors
    ///
    /// Returns an error when either bound or a transformed corner is invalid.
    pub fn transform_bounds(
        self,
        low: [f32; 3],
        high: [f32; 3],
    ) -> Result<
        (
            [f32; 3],
            [f32; 3],
        ),
        MovementError,
    > {
        if (0..3).any(|axis| low[axis] > high[axis]) {
            return Err(MovementError::InvertedBounds);
        }
        let mut transformed_low = [f32::INFINITY; 3];
        let mut transformed_high = [f32::NEG_INFINITY; 3];
        for x in [
            low[0], high[0],
        ] {
            for y in [
                low[1], high[1],
            ] {
                for z in [
                    low[2], high[2],
                ] {
                    let point = self.transform_point(
                        [
                            x, y, z,
                        ],
                    )?;
                    for axis in 0..3 {
                        transformed_low[axis] =
                            transformed_low[axis].min(point[axis]);
                        transformed_high[axis] =
                            transformed_high[axis].max(point[axis]);
                    }
                }
            }
        }
        Ok(
            (
                transformed_low,
                transformed_high,
            ),
        )
    }
}

/// Affine movement validation or transformation failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MovementError {
    /// Movement identity was empty.
    MissingIdentity,
    /// No coordinate families were declared.
    MissingSubjects,
    /// Matrix contained NaN or infinity.
    NonFiniteMatrix,
    /// Matrix was not affine in row-vector convention.
    NonAffineMatrix,
    /// Matrix basis was not invertible.
    DegenerateBasis,
    /// Input or transformed coordinate contained NaN or infinity.
    NonFiniteCoordinate,
    /// Axis-aligned bound had a low component above its high component.
    InvertedBounds,
}

impl fmt::Display for MovementError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let message = match self {
            Self::MissingIdentity => "coordinate movement identity is missing",
            Self::MissingSubjects => "coordinate movement subjects are missing",
            Self::NonFiniteMatrix => "coordinate movement matrix is non-finite",
            Self::NonAffineMatrix => "coordinate movement matrix is not affine",
            Self::DegenerateBasis => "coordinate movement basis is degenerate",
            Self::NonFiniteCoordinate => {
                "coordinate movement value is non-finite"
            }
            Self::InvertedBounds => "coordinate movement bounds are inverted",
        };
        formatter.write_str(message)
    }
}

mod matrix;

use matrix::{determinant, transform_direction, transform_point};
pub use matrix::{
    identity_matrix, matrix_key, multiply_matrices, translation_matrix,
};

#[cfg(test)]
#[path = "coordinate_movement_tests.rs"]
mod tests;

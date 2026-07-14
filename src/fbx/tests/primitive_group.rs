// File:
//   - primitive_group.rs
// Path:
//   - src/fbx/tests/primitive_group.rs
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
//   - Regression coverage for normalized FBX primitive-group invariants.
// - Must-Not:
//   - Access private assets, perform filesystem discovery, or duplicate domain
//   - implementation logic.
// - Allows:
//   - Synthetic mesh arrays and caller-visible domain error assertions.
// - Split-When:
//   - Another mesh aggregate requires an independent integration boundary.
// - Merge-When:
//   - Primitive-group regressions no longer require a distinct test target.
// - Summary:
//   - Protects primitive-group validation before scene construction.
// - Description:
//   - Exercises public mesh construction with synthetic geometry evidence.
// - Usage:
//   - Run through the fbx crate test target.
// - Defaults:
//   - No local files, generated assets, or external processes are required.
//
// ADRs:
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
//
// Large file:
//   - false
//

//! Regression coverage for normalized FBX primitive-group invariants.
//!
//! Synthetic arrays protect caller-visible validation without local assets.

// This integration target inherits the crate's serialization dependencies
// even though these pure domain regressions do not deserialize fixtures.
use fbx::domain::mesh::{MeshError, PrimitiveGroup};
use schoenwald_filesystem as _;
use serde as _;
use serde_json as _;

#[test]
fn reports_missing_face_indices() {
    let result = PrimitiveGroup::new(
        0,
        "shader",
        vec![
            [
                0.0, 0.0, 0.0,
            ],
            [
                1.0, 0.0, 0.0,
            ],
            [
                0.0, 1.0, 0.0,
            ],
        ],
        Vec::new(),
        &[],
    );

    assert_eq!(
        result,
        Err(MeshError::MissingIndices)
    );
}

#[test]
fn reports_missing_position_evidence() {
    let result = PrimitiveGroup::new(
        0,
        "shader",
        Vec::new(),
        Vec::new(),
        &[
            0, 1, 2,
        ],
    );

    assert_eq!(
        result,
        Err(MeshError::MissingPositions)
    );
}

#[test]
fn rejects_triangles_with_repeated_vertex_indices() {
    let result = PrimitiveGroup::new(
        0,
        "shader",
        vec![
            [
                0.0, 0.0, 0.0,
            ],
            [
                1.0, 0.0, 0.0,
            ],
            [
                0.0, 1.0, 0.0,
            ],
        ],
        Vec::new(),
        &[
            0, 0, 1,
        ],
    );

    assert_eq!(
        result,
        Err(
            MeshError::RepeatedTriangleVertex {
                triangle: 0
            }
        )
    );
}

#[test]
fn rejects_blank_shader_identity() {
    for shader in [
        "", "   ",
    ] {
        let result = PrimitiveGroup::new(
            0,
            shader,
            vec![
                [
                    0.0, 0.0, 0.0,
                ],
                [
                    1.0, 0.0, 0.0,
                ],
                [
                    0.0, 1.0, 0.0,
                ],
            ],
            Vec::new(),
            &[
                0, 1, 2,
            ],
        );

        assert_eq!(
            result,
            Err(MeshError::MissingShader)
        );
    }
}

#[test]
fn rejects_non_finite_uvs() {
    let result = PrimitiveGroup::new(
        0,
        "shader",
        vec![
            [
                0.0, 0.0, 0.0,
            ],
            [
                1.0, 0.0, 0.0,
            ],
            [
                0.0, 1.0, 0.0,
            ],
        ],
        vec![
            [
                f32::INFINITY,
                0.0,
            ],
            [
                1.0, 0.0,
            ],
            [
                0.0, 1.0,
            ],
        ],
        &[
            0, 1, 2,
        ],
    );

    assert_eq!(
        result,
        Err(
            MeshError::NonFiniteUv {
                vertex: 0,
                axis: 0
            }
        )
    );
}

#[test]
fn rejects_non_finite_positions() {
    let result = PrimitiveGroup::new(
        0,
        "shader",
        vec![
            [
                f32::NAN,
                0.0,
                0.0,
            ],
            [
                1.0, 0.0, 0.0,
            ],
            [
                0.0, 1.0, 0.0,
            ],
        ],
        Vec::new(),
        &[
            0, 1, 2,
        ],
    );

    assert_eq!(
        result,
        Err(
            MeshError::NonFinitePosition {
                vertex: 0,
                axis: 0,
            }
        )
    );
}

#[test]
fn rejects_indices_outside_position_range() {
    let result = PrimitiveGroup::new(
        0,
        "shader",
        vec![
            [
                0.0, 0.0, 0.0,
            ],
            [
                1.0, 0.0, 0.0,
            ],
            [
                0.0, 1.0, 0.0,
            ],
        ],
        Vec::new(),
        &[
            0, 1, 3,
        ],
    );

    assert_eq!(
        result,
        Err(
            MeshError::IndexOutOfBounds {
                index: 3,
                positions: 3,
            }
        )
    );
}

#[test]
fn reverses_winding_that_opposes_authored_normals() {
    let result = PrimitiveGroup::new(
        0,
        "shader",
        vec![
            [
                0.0, 0.0, 0.0,
            ],
            [
                0.0, 1.0, 0.0,
            ],
            [
                1.0, 0.0, 0.0,
            ],
        ],
        Vec::new(),
        &[
            0, 1, 2,
        ],
    )
    .and_then(
        |group| {
            group.with_normals(
                vec![
                    [
                        0.0, 0.0, 1.0,
                    ],
                    [
                        0.0, 0.0, 1.0,
                    ],
                    [
                        0.0, 0.0, 1.0,
                    ],
                ],
            )
        },
    );
    assert!(
        result.is_ok(),
        "primitive group should accept authored normals: {result:?}"
    );
    let Some(group) = result.ok() else {
        return;
    };

    assert_eq!(
        group.triangles,
        vec![
            [
                0, 2, 1,
            ],
        ]
    );
}

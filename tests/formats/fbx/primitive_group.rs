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
//   - Primitive group test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Primitive group test module.
// - Description:
//   - Implements the declared test module responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Primitive group test module.

use fbx::domain::mesh::{MeshError, PrimitiveGroup};
use png as _;
use schoenwald_filesystem as _;
use serde as _;
use serde_json as _;
use shar_sha256 as _;

#[test]
fn reports_missing_face_indices() {
    let result = PrimitiveGroup::new(
        0,
        "shader",
        vec![[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]],
        Vec::new(),
        &[],
    );

    assert_eq!(result, Err(MeshError::MissingIndices));
}

#[test]
fn reports_missing_position_evidence() {
    let result =
        PrimitiveGroup::new(0, "shader", Vec::new(), Vec::new(), &[0, 1, 2]);

    assert_eq!(result, Err(MeshError::MissingPositions));
}

#[test]
fn rejects_triangles_with_repeated_vertex_indices() {
    let result = PrimitiveGroup::new(
        0,
        "shader",
        vec![[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]],
        Vec::new(),
        &[0, 0, 1],
    );

    assert_eq!(
        result,
        Err(MeshError::RepeatedTriangleVertex { triangle: 0 })
    );
}

#[test]
fn rejects_blank_shader_identity() {
    for shader in ["", "   "] {
        let result = PrimitiveGroup::new(
            0,
            shader,
            vec![[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]],
            Vec::new(),
            &[0, 1, 2],
        );

        assert_eq!(result, Err(MeshError::MissingShader));
    }
}

#[test]
fn rejects_non_finite_uvs() {
    let result = PrimitiveGroup::new(
        0,
        "shader",
        vec![[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]],
        vec![[f32::INFINITY, 0.], [1., 0.], [0., 1.]],
        &[0, 1, 2],
    );

    assert_eq!(result, Err(MeshError::NonFiniteUv { vertex: 0, axis: 0 }));
}

#[test]
fn rejects_non_finite_positions() {
    let result = PrimitiveGroup::new(
        0,
        "shader",
        vec![[f32::NAN, 0., 0.], [1., 0., 0.], [0., 1., 0.]],
        Vec::new(),
        &[0, 1, 2],
    );

    assert_eq!(
        result,
        Err(MeshError::NonFinitePosition { vertex: 0, axis: 0 })
    );
}

#[test]
fn rejects_indices_outside_position_range() {
    let result = PrimitiveGroup::new(
        0,
        "shader",
        vec![[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]],
        Vec::new(),
        &[0, 1, 3],
    );

    assert_eq!(
        result,
        Err(MeshError::IndexOutOfBounds { index: 3, positions: 3 })
    );
}

#[test]
fn preserves_winding_that_opposes_authored_normals() {
    let result = PrimitiveGroup::new(
        0,
        "shader",
        vec![[0., 0., 0.], [0., 1., 0.], [1., 0., 0.]],
        Vec::new(),
        &[0, 1, 2],
    )
    .and_then(|group| {
        group.with_normals(vec![[0., 0., 1.], [0., 0., 1.], [0., 0., 1.]])
    });
    assert!(
        result.is_ok(),
        "primitive group should accept authored normals: {result:?}"
    );
    let Some(group) = result.ok() else {
        return;
    };

    assert_eq!(group.triangles, vec![[0, 1, 2]]);
}

// File:
//   - mesh_asset.rs
// Path:
//   - src/fbx/tests/mesh_asset.rs
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
//   - Regression coverage for normalized FBX mesh-asset invariants.
// - Must-Not:
//   - Access private assets, perform filesystem discovery, or duplicate domain
//   - implementation logic.
// - Allows:
//   - Synthetic mesh aggregates and caller-visible domain assertions.
// - Split-When:
//   - Another aggregate requires an independent integration boundary.
// - Merge-When:
//   - Mesh-asset regressions no longer require a distinct test target.
// - Summary:
//   - Protects mesh aggregate validation before scene construction.
// - Description:
//   - Exercises public mesh-asset construction with synthetic evidence.
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

//! Regression coverage for normalized FBX mesh-asset invariants.
//!
//! Synthetic aggregates protect caller-visible validation without local assets.

// This integration target inherits serialization dependencies without
// fixtures.
use fbx::domain::mesh::{
    MeshAsset, MeshError, PrimitiveGroup, mesh_asset_to_geometry,
};
use png as _;
use schoenwald_filesystem as _;
use serde as _;
use serde_json as _;
use shar_sha256 as _;

fn primitive_group(index: usize) -> PrimitiveGroup {
    PrimitiveGroup {
        index,
        shader: format!("shader-{index}"),
        positions: vec![
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
        normals: Vec::new(),
        colors: Vec::new(),
        uvs: Vec::new(),
        triangles: vec![
            [
                0, 1, 2,
            ],
        ],
    }
}

#[test]
fn preserves_stable_group_identity_during_translation() {
    let result = MeshAsset::new(
        "mesh",
        vec![primitive_group(7)],
    )
    .map(
        |mesh| {
            mesh_asset_to_geometry(&mesh)
                .first()
                .map(
                    |geometry| {
                        (
                            geometry
                                .id
                                .clone(),
                            geometry
                                .polygons
                                .first()
                                .and_then(|polygon| polygon.material_slot),
                        )
                    },
                )
        },
    );

    assert_eq!(
        result,
        Ok(
            Some(
                (
                    "mesh-geometry-7".to_owned(),
                    Some(7)
                )
            )
        )
    );
}

#[test]
fn preserves_uvs_during_geometry_translation() {
    let mut group = primitive_group(0);
    group.uvs = vec![
        [
            0.0, 0.0,
        ],
        [
            1.0, 0.0,
        ],
        [
            0.0, 1.0,
        ],
    ];
    let result = MeshAsset::new(
        "mesh",
        vec![group],
    )
    .map(
        |mesh| {
            mesh_asset_to_geometry(&mesh)
                .first()
                .and_then(
                    |geometry| {
                        geometry
                            .uv_layers
                            .first()
                    },
                )
                .map(
                    |layer| {
                        layer
                            .values
                            .clone()
                    },
                )
        },
    );

    assert_eq!(
        result,
        Ok(
            Some(
                vec![
                    [
                        0.0, 0.0
                    ],
                    [
                        1.0, 0.0
                    ],
                    [
                        0.0, 1.0
                    ],
                ]
            )
        )
    );
}

#[test]
fn rejects_duplicate_primitive_group_indices() {
    let result = MeshAsset::new(
        "mesh",
        vec![
            primitive_group(0),
            primitive_group(0),
        ],
    );

    assert_eq!(
        result,
        Err(
            MeshError::DuplicatePrimitiveGroupIndex {
                index: 0
            }
        )
    );
}

#[test]
fn rejects_meshes_without_primitive_groups() {
    let result = MeshAsset::new(
        "mesh",
        Vec::new(),
    );

    assert_eq!(
        result,
        Err(MeshError::MissingPrimitiveGroups)
    );
}

#[test]
fn rejects_blank_mesh_names() {
    let result = MeshAsset::new(
        "   ",
        Vec::new(),
    );

    assert_eq!(
        result,
        Err(MeshError::MissingMeshName)
    );
}

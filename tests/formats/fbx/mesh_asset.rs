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
//   - Mesh asset test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Mesh asset test module.
// - Description:
//   - Implements the declared test module responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Mesh asset test module.

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
        source_identity: None,
        source_ordinal: None,
        shader: format!("shader-{index}"),
        positions: vec![[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]],
        normals: Vec::new(),
        colors: Vec::new(),
        uvs: Vec::new(),
        triangles: vec![[0, 1, 2]],
    }
}

#[test]
fn preserves_stable_group_identity_during_translation() {
    let result = MeshAsset::new("mesh", vec![primitive_group(7)]).map(|mesh| {
        mesh_asset_to_geometry(&mesh).first().map(|geometry| {
            (
                geometry.id.clone(),
                geometry
                    .polygons
                    .first()
                    .and_then(|polygon| polygon.material_slot),
            )
        })
    });

    assert_eq!(result, Ok(Some(("mesh-geometry-7".to_owned(), Some(7)))));
}

#[test]
fn preserves_uvs_during_geometry_translation() {
    let mut group = primitive_group(0);
    group.uvs = vec![[0., 0.], [1., 0.], [0., 1.]];
    let result = MeshAsset::new("mesh", vec![group]).map(|mesh| {
        mesh_asset_to_geometry(&mesh)
            .first()
            .and_then(|geometry| geometry.uv_layers.first())
            .map(|layer| layer.values.clone())
    });

    assert_eq!(result, Ok(Some(vec![[0., 0.], [1., 0.], [0., 1.],])));
}

#[test]
fn rejects_duplicate_primitive_group_indices() {
    let result =
        MeshAsset::new("mesh", vec![primitive_group(0), primitive_group(0)]);

    assert_eq!(
        result,
        Err(MeshError::DuplicatePrimitiveGroupIndex { index: 0 })
    );
}

#[test]
fn rejects_meshes_without_primitive_groups() {
    let result = MeshAsset::new("mesh", Vec::new());

    assert_eq!(result, Err(MeshError::MissingPrimitiveGroups));
}

#[test]
fn rejects_blank_mesh_names() {
    let result = MeshAsset::new("   ", Vec::new());

    assert_eq!(result, Err(MeshError::MissingMeshName));
}

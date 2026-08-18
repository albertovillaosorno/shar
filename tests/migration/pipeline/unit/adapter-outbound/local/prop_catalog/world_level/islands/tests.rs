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
//   - Tests unit tests.
// - Must-Not:
//   - Own production behavior or broaden the tested API surface.
// - Allows:
//   - Private test fixtures and assertions for the owning source module.
// - Split-When:
//   - Split when an independent fixture family gains separate ownership.
// - Merge-When:
//   - Merge when another test module owns the identical evidence.
// - Summary:
//   - Tests unit tests.
// - Description:
//   - Preserves unit-test access through a test-only path module.
// - Usage:
//   - Included only by the owning source module under cfg(test).
// - Defaults:
//   - Test setup and assertions fail explicitly.
//

//! Tests unit tests.

use fbx::domain::mesh::{MeshAsset, PrimitiveGroup};

use super::split_distant_islands;

/// Build two disconnected triangles separated on the X axis.
fn synthetic_mesh(second_origin: f32) -> Result<MeshAsset, String> {
    let group = PrimitiveGroup::new(
        0,
        "material",
        vec![
            [0., 0., 0.],
            [1., 0., 0.],
            [0., 1., 0.],
            [second_origin, 0., 0.],
            [second_origin + 1., 0., 0.],
            [second_origin, 1., 0.],
        ],
        vec![[0., 0.], [1., 0.], [0., 1.], [0., 0.], [1., 0.], [0., 1.]],
        &[0, 1, 2, 3, 4, 5],
    )
    .map_err(|error| format!("group failed: {error:?}"))?;
    MeshAsset::new("aggregate", vec![group])
        .map_err(|error| format!("mesh failed: {error:?}"))
}

#[test]
fn distant_components_become_independent_objects() -> Result<(), String> {
    let separated = split_distant_islands(synthetic_mesh(20.)?)
        .map_err(|error| error.to_string())?;
    if separated.len() != 2 {
        return Err(format!("expected two objects, got {}", separated.len()));
    }
    if separated
        .iter()
        .any(|mesh| !mesh.name.contains("__independent-object-"))
    {
        return Err("split objects lack stable semantic names".to_owned());
    }
    Ok(())
}

#[test]
fn distant_split_preserves_authored_positions_and_uvs() -> Result<(), String> {
    let source = synthetic_mesh(20.)?;
    let source_group = source
        .groups
        .first()
        .ok_or_else(|| "source fixture group is missing".to_owned())?;
    let mut expected_positions = source_group.positions.clone();
    let mut expected_uvs = source_group.uvs.clone();

    let separated = split_distant_islands(source)
        .map_err(|error| error.to_string())?;
    let mut actual_positions = separated
        .iter()
        .flat_map(|mesh| mesh.groups.iter())
        .flat_map(|group| group.positions.iter().copied())
        .collect::<Vec<_>>();
    let mut actual_uvs = separated
        .iter()
        .flat_map(|mesh| mesh.groups.iter())
        .flat_map(|group| group.uvs.iter().copied())
        .collect::<Vec<_>>();

    expected_positions.sort_by_key(|value| value.map(f32::to_bits));
    actual_positions.sort_by_key(|value| value.map(f32::to_bits));
    expected_uvs.sort_by_key(|value| value.map(f32::to_bits));
    actual_uvs.sort_by_key(|value| value.map(f32::to_bits));

    if actual_positions != expected_positions {
        return Err("distant split changed authored positions".to_owned());
    }
    if actual_uvs != expected_uvs {
        return Err("distant split changed authored UVs".to_owned());
    }
    Ok(())
}

#[test]
fn nearby_components_remain_one_visual_object() -> Result<(), String> {
    let separated = split_distant_islands(synthetic_mesh(1.5)?)
        .map_err(|error| error.to_string())?;
    let first = separated
        .first()
        .ok_or_else(|| "nearby mesh disappeared".to_owned())?;
    if separated.len() != 1 || first.name != "aggregate" {
        return Err("nearby geometry was split apart".to_owned());
    }
    Ok(())
}

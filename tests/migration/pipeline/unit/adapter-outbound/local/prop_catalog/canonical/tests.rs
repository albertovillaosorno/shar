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
//   - Prop canonicalization ordering regressions.
// - Must-Not:
//   - Exercise filesystem publication or source extraction.
// - Allows:
//   - Synthetic FBX-domain fixtures and private canonicalization helpers.
// - Split-When:
//   - Split when another canonicalization family gains independent fixtures.
// - Merge-When:
//   - Merge when another test module owns identical prop ordering evidence.
// - Summary:
//   - Prop canonicalization ordering tests.
// - Description:
//   - Proves canonical naming does not replace source mesh or part ordering.
// - Usage:
//   - Included only by the prop canonicalization module under cfg(test).
// - Defaults:
//   - Synthetic source order remains authoritative.
//

//! Prop canonicalization ordering regressions.

use fbx::domain::character::{CharacterAsset, SkinnedPart};
use fbx::domain::mesh::{MeshAsset, PrimitiveGroup};
use fbx::domain::skeleton::Bone;
use fbx::domain::skin::SkinInfluence;

use super::{canonicalize_animated_asset, canonicalize_static_meshes};

fn mesh(name: &str, shader: &str) -> Result<MeshAsset, String> {
    let group = PrimitiveGroup::new(
        0,
        shader,
        vec![[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]],
        Vec::new(),
        &[0, 1, 2],
    )
    .map_err(|error| format!("primitive group failed: {error:?}"))?;
    MeshAsset::new(name, vec![group])
        .map_err(|error| format!("mesh failed: {error:?}"))
}

fn part(name: &str, shader: &str) -> Result<SkinnedPart, String> {
    let mesh = mesh(name, shader)?;
    let influences = (0_u32..3)
        .map(|vertex_index| SkinInfluence {
            vertex_index,
            bone_id: "root".to_owned(),
            weight: 1.,
        })
        .collect::<Vec<_>>();
    Ok(SkinnedPart {
        mesh,
        group_influences: vec![influences],
    })
}

fn root_bone() -> Bone {
    Bone {
        id: "root".to_owned(),
        parent_id: None,
        rest_matrix: [
            1., 0., 0., 0.,
            0., 1., 0., 0.,
            0., 0., 1., 0.,
            0., 0., 0., 1.,
        ],
        source_rig: None,
    }
}

#[test]
fn static_canonical_names_preserve_source_mesh_order() -> Result<(), String> {
    let mut meshes = vec![
        mesh("z-mesh", "z-source")?,
        mesh("a-mesh", "a-source")?,
    ];
    canonicalize_static_meshes(&mut meshes);
    let actual = meshes
        .iter()
        .map(|mesh| {
            mesh.groups
                .first()
                .map(|group| (mesh.name.as_str(), group.shader.as_str()))
                .ok_or_else(|| {
                    "canonical static mesh lost its group".to_owned()
                })
        })
        .collect::<Result<Vec<_>, _>>()?;
    assert_eq!(
        actual,
        [("part-0000", "z-source"), ("part-0001", "a-source")]
    );
    Ok(())
}

#[test]
fn animated_canonical_names_preserve_source_part_order() -> Result<(), String> {
    let mut asset = CharacterAsset::new(
        "source-model",
        vec![root_bone()],
        vec![part("z-part", "z-source")?, part("a-part", "a-source")?],
    )
    .map_err(|error| format!("character failed: {error:?}"))?;
    canonicalize_animated_asset(&mut asset, &mut [])
        .map_err(|error| error.to_string())?;
    let actual = asset
        .parts
        .iter()
        .map(|part| {
            part.mesh
                .groups
                .first()
                .map(|group| (part.mesh.name.as_str(), group.shader.as_str()))
                .ok_or_else(|| {
                    "canonical animated part lost its group".to_owned()
                })
        })
        .collect::<Result<Vec<_>, _>>()?;
    assert_eq!(
        actual,
        [("part-0000", "z-source"), ("part-0001", "a-source")]
    );
    Ok(())
}

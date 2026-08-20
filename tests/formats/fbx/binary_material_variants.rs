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
//   - Binary material variants test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Binary material variants test module.
// - Description:
//   - Implements the declared test module responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Binary material variants test module.

use std::fs;
use std::path::{Path, PathBuf};

use fbx::adapters::driven::binary_character_writer::write_binary_model_fbx;
use fbx::domain::mesh::{MeshAsset, PrimitiveGroup};
use fbx::domain::texture::MaterialBinding;
use png as _;
use schoenwald_filesystem as _;
use serde as _;
use serde_json as _;
use shar_sha256 as _;

fn mesh(name: &str) -> Result<MeshAsset, String> {
    let group = PrimitiveGroup::new(
        0,
        "shared_m",
        vec![[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]],
        vec![[0., 0.], [1., 0.], [0., 1.]],
        &[0, 1, 2],
    )
    .map_err(|error| format!("primitive group failed: {error:?}"))?;
    MeshAsset::new(name, vec![group])
        .map_err(|error| format!("mesh failed: {error:?}"))
}

fn output_path(label: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "fbx-binary-material-variants-{label}-{}.fbx",
        std::process::id()
    ))
}

fn remove_if_present(path: &Path) -> Result<(), String> {
    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(format!("temporary FBX cleanup failed: {error}")),
    }
}

fn contains_token(bytes: &[u8], token: &str) -> bool {
    bytes
        .windows(token.len())
        .any(|window| window == token.as_bytes())
}

#[test]
fn shared_shader_creates_isolated_light_material_variant() -> Result<(), String>
{
    let path = output_path("semantic-split");
    remove_if_present(&path)?;
    let material = MaterialBinding::new("shared_m", None)
        .map_err(|error| format!("material failed: {error:?}"))?;
    let summary = write_binary_model_fbx(
        "material-variants",
        &[mesh("vehicle-body")?, mesh("lightsShape")?],
        &[material],
        &path,
    )
    .map_err(|error| format!("FBX write failed: {error:?}"))?;
    if summary.materials != 2 {
        return Err(format!(
            "shared shader did not split into two semantic variants: \
                 {summary:?}"
        ));
    }
    let bytes =
        fs::read(&path).map_err(|error| format!("FBX read failed: {error}"))?;
    if !contains_token(&bytes, "shared_m__light-emitter") {
        return Err("light material variant identity is missing".to_owned());
    }
    remove_if_present(&path)?;
    Ok(())
}

#[test]
fn material_objects_preserve_first_group_use_order() -> Result<(), String> {
    let path = output_path("source-order");
    remove_if_present(&path)?;
    let groups = ["zebra_m", "alpha_m"]
        .into_iter()
        .enumerate()
        .map(|(index, shader)| {
            PrimitiveGroup::new(
                index,
                shader,
                vec![[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]],
                vec![[0., 0.], [1., 0.], [0., 1.]],
                &[0, 1, 2],
            )
            .map_err(|error| format!("primitive group failed: {error:?}"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let model = MeshAsset::new("body", groups)
        .map_err(|error| format!("mesh failed: {error:?}"))?;
    let materials = ["zebra_m", "alpha_m"]
        .into_iter()
        .map(|name| {
            MaterialBinding::new(name, None)
                .map_err(|error| format!("material failed: {error:?}"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let _summary =
        write_binary_model_fbx("material-order", &[model], &materials, &path)
            .map_err(|error| format!("FBX write failed: {error:?}"))?;
    let bytes =
        fs::read(&path).map_err(|error| format!("FBX read failed: {error}"))?;
    let zebra = bytes
        .windows("zebra_m".len())
        .position(|window| window == b"zebra_m")
        .ok_or("zebra material identity is missing")?;
    let alpha = bytes
        .windows("alpha_m".len())
        .position(|window| window == b"alpha_m")
        .ok_or("alpha material identity is missing")?;
    remove_if_present(&path)?;
    if zebra >= alpha {
        return Err(format!(
            "material objects lost first-use order: zebra={zebra}, alpha={alpha}"
        ));
    }
    Ok(())
}

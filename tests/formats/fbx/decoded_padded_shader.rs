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
//   - Decoded padded shader test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Decoded padded shader test module.
// - Description:
//   - Implements the declared test module responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Decoded padded shader test module.

use std::fs;
use std::path::PathBuf;

use fbx::adapters::driven::decoded_component_source::DecodedComponentSource;
use fbx::ports::component_source::ComponentSource;
use png as _;
use schoenwald_filesystem as _;
use serde as _;
use serde_json as _;
use shar_sha256 as _;

fn temp_root() -> PathBuf {
    std::env::temp_dir()
        .join(format!("fbx-decoded-padded-shader-{}", std::process::id()))
}

#[test]
fn resolves_padded_shader_member() -> Result<(), String> {
    let root = temp_root();
    let shader_dir = root.join("components").join("shader");
    fs::create_dir_all(&shader_dir)
        .and_then(|()| {
            fs::write(
                shader_dir.join("char_swatches_m_.json"),
                concat!(
                    r#"{"schema":"shader","name":"char_swatches_m\u0000","#,
                    r#""version":0,"pddi_shader_name":"simple","#,
                    r#""has_translucency":0,"num_params":0,"params":[]}"#,
                ),
            )
        })
        .map_err(|error| error.to_string())?;
    let source = DecodedComponentSource::new(&root, root.join("textures"));
    let result = source.resolve_material("char_swatches_m");
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    let material =
        result.map_err(|error| format!("padded shader failed: {error:?}"))?;
    if material.material_name == "char_swatches_m"
        && material.texture_file_name.is_none()
    {
        Ok(())
    } else {
        Err(format!("unexpected padded material: {material:?}"))
    }
}

#[test]
fn resolves_case_insensitive_padded_shader_member() -> Result<(), String> {
    let root = temp_root().with_extension("case");
    let shader_dir = root.join("components").join("shader");
    fs::create_dir_all(&shader_dir)
        .and_then(|()| {
            fs::write(
                shader_dir.join("vent_m__.json"),
                concat!(
                    r#"{"schema":"shader","name":"Vent_m\u0000\u0000","#,
                    r#""version":0,"pddi_shader_name":"simple","#,
                    r#""has_translucency":0,"num_params":0,"params":[]}"#,
                ),
            )
        })
        .map_err(|error| error.to_string())?;
    let source = DecodedComponentSource::new(&root, root.join("textures"));
    let result = source.resolve_material("vEnT_m");
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    let material = result.map_err(|error| {
        format!("case-insensitive padded shader failed: {error:?}")
    })?;
    if material.material_name == "Vent_m"
        && material.texture_file_name.is_none()
    {
        Ok(())
    } else {
        Err(format!(
            "unexpected case-insensitive material: {material:?}"
        ))
    }
}

#[test]
fn resolves_fixed_width_tga_reference_to_decoded_png() -> Result<(), String> {
    let root = temp_root().with_extension("tga");
    let shader_dir = root.join("components").join("shader");
    let texture_dir = root.join("components").join("texture");
    fs::create_dir_all(&shader_dir)
        .and_then(|()| fs::create_dir_all(&texture_dir))
        .and_then(|()| {
            fs::write(
                shader_dir.join("wings_m_.json"),
                concat!(
                    r#"{"schema":"shader","name":"wings_m\u0000","#,
                    r#""version":0,"pddi_shader_name":"simple","#,
                    r#""has_translucency":1,"num_params":1,"params":[{"#,
                    r#""kind":"texture","param":"TEX","#,
                    r#""value":"wings.tga\u0000\u0000\u0000"}]}"#,
                ),
            )
        })
        .and_then(|()| fs::write(texture_dir.join("wings.png"), b"png"))
        .map_err(|error| error.to_string())?;
    let output = root.join("textures");
    let source = DecodedComponentSource::new(&root, &output);
    let result = source.resolve_material("wings_m");
    let material = result.map_err(|error| {
        format!("fixed-width TGA reference failed: {error:?}")
    })?;
    let staged = output.join("wings.png");
    let staged_exists = staged.is_file();
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    if material.texture_file_name.as_deref() == Some("wings.png")
        && staged_exists
    {
        Ok(())
    } else {
        Err(format!(
            "unexpected fixed-width texture material: {material:?}"
        ))
    }
}

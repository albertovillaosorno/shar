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
//   - Decoded billboard source test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Decoded billboard source test module.
// - Description:
//   - Implements the declared test module responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Decoded billboard source test module.

use std::fs;
use std::path::PathBuf;

use fbx::adapters::driven::binary_character_writer::write_binary_model_fbx;
use fbx::adapters::driven::decoded_billboard_source::read_billboard_quad_group;
use fbx::domain::texture::MaterialBinding;
use png as _;
use schoenwald_filesystem as _;
use serde as _;
use serde_json as _;
use shar_sha256 as _;

fn fixture_path(label: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "fbx-decoded-billboard-{label}-{}.json",
        std::process::id()
    ))
}

#[test]
fn decodes_authored_billboard_quad_geometry() -> Result<(), String> {
    let path = fixture_path("geometry");
    fs::write(
        &path,
        concat!(
            r#"{"schema":"quad_group","version":0,"name":"brake1Shape\u0000","#,
            r#""shader":"brakeFlareA_m\u0000","z_test":1,"z_write":0,"fog":0,"#,
            r#""num_quads":1,"quads":[{"name":"brake1Shape\u0000","#,
            r#""version":2,"billboard_mode":"LYAX","#,
            r#""translation":[2,3,4],"colour":1291780096,"#,
            r#""uvs":[[0,0],[1,0],[1,1],[0,1]],"#,
            r#""width":2,"height":4,"distance":0,"uv_offset":[0,0],"#,
            r#""rotation_wxyz":[1,0,0,0],"cutoff_mode":"SNG","#,
            r#""uv_offset_range":[0,0],"source_range":1,"#,
            r#""edge_range":0.5,"perspective":true}]}"#,
        ),
    )
    .map_err(|error| error.to_string())?;
    let mesh = read_billboard_quad_group(&path, "brake1Shape")
        .map_err(|error| format!("billboard decode failed: {error:?}"))?;
    fs::remove_file(&path).map_err(|error| error.to_string())?;
    if mesh.name != "brake1Shape" || mesh.groups.len() != 1 {
        return Err(format!("unexpected billboard mesh: {mesh:?}"));
    }
    let group = mesh
        .groups
        .first()
        .ok_or_else(|| "billboard mesh has no primitive group".to_owned())?;
    if group.positions
        != vec![[1., 1., 4.], [3., 1., 4.], [3., 5., 4.], [1., 5., 4.]]
        || group.shader != "brakeFlareA_m"
        || group.source_identity.as_deref() != Some("brake1Shape")
    {
        return Err(format!("billboard geometry changed: {group:?}"));
    }
    Ok(())
}

#[test]
fn writes_authored_quad_identity_as_fbx_metadata() -> Result<(), String> {
    let path = fixture_path("identity");
    let output = std::env::temp_dir().join(format!(
        "fbx-decoded-billboard-identity-{}.fbx",
        std::process::id()
    ));
    let _cleanup_output = fs::remove_file(&output);
    fs::write(
        &path,
        concat!(
            r#"{"schema":"quad_group","version":0,"name":"groupShape","#,
            r#""shader":"material","z_test":1,"z_write":0,"fog":0,"#,
            r#""num_quads":1,"quads":[{"name":"leafShape","#,
            r#""version":2,"billboard_mode":"LYAX","#,
            r#""translation":[0,0,0],"colour":4294967295,"#,
            r#""uvs":[[0,0],[1,0],[1,1],[0,1]],"#,
            r#""width":1,"height":1,"distance":0,"uv_offset":[0,0],"#,
            r#""rotation_wxyz":[1,0,0,0],"cutoff_mode":"N0NE","#,
            r#""uv_offset_range":[0,0],"source_range":0,"#,
            r#""edge_range":0,"perspective":true}]}"#
        ),
    )
    .map_err(|error| error.to_string())?;
    let mesh = read_billboard_quad_group(&path, "groupShape")
        .map_err(|error| format!("billboard decode failed: {error:?}"))?;
    fs::remove_file(&path).map_err(|error| error.to_string())?;
    let material = MaterialBinding::new("material", None)
        .map_err(|error| format!("material failed: {error:?}"))?;
    let _summary =
        write_binary_model_fbx("billboard", &[mesh], &[material], &output)
            .map_err(|error| {
                format!("billboard FBX write failed: {error:?}")
            })?;
    let bytes = fs::read(&output).map_err(|error| error.to_string())?;
    fs::remove_file(&output).map_err(|error| error.to_string())?;
    if !bytes
        .windows("leafShape".len())
        .any(|window| window == b"leafShape")
    {
        return Err(
            "authored quad identity was not preserved in FBX".to_owned()
        );
    }
    Ok(())
}

#[test]
fn rejects_space_padded_authored_quad_identity() -> Result<(), String> {
    let path = fixture_path("padded-identity");
    fs::write(
        &path,
        concat!(
            r#"{"schema":"quad_group","version":0,"name":"groupShape","#,
            r#""shader":"material","z_test":1,"z_write":0,"fog":0,"#,
            r#""num_quads":1,"quads":[{"name":" leafShape","#,
            r#""version":2,"billboard_mode":"LYAX","#,
            r#""translation":[0,0,0],"colour":4294967295,"#,
            r#""uvs":[[0,0],[1,0],[1,1],[0,1]],"#,
            r#""width":1,"height":1,"distance":0,"uv_offset":[0,0],"#,
            r#""rotation_wxyz":[1,0,0,0],"cutoff_mode":"N0NE","#,
            r#""uv_offset_range":[0,0],"source_range":0,"#,
            r#""edge_range":0,"perspective":true}]}"#
        ),
    )
    .map_err(|error| error.to_string())?;
    let result = read_billboard_quad_group(&path, "groupShape");
    fs::remove_file(&path).map_err(|error| error.to_string())?;
    if result.is_ok() {
        return Err("space-padded authored quad identity was repaired".to_owned());
    }
    Ok(())
}

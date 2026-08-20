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

use fbx::adapters::driven::decoded_billboard_source::read_billboard_quad_group;
use png as _;
use schoenwald_filesystem as _;
use serde as _;
use serde_json as _;
use shar_sha256 as _;

fn fixture_path() -> PathBuf {
    std::env::temp_dir()
        .join(format!("fbx-decoded-billboard-{}.json", std::process::id()))
}

#[test]
fn decodes_authored_billboard_quad_geometry() -> Result<(), String> {
    let path = fixture_path();
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
    {
        return Err(format!("billboard geometry changed: {group:?}"));
    }
    Ok(())
}

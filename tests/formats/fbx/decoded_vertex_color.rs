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
//   - Decoded vertex color test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Decoded vertex color test module.
// - Description:
//   - Implements the declared test module responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Decoded vertex color test module.

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
        .join(format!("fbx-decoded-vertex-color-{}", std::process::id()))
}

#[test]
fn decodes_pddi_aarrggbb_into_normalized_rgba() -> Result<(), String> {
    let root = temp_root();
    let mesh_dir = root.join("components").join("mesh");
    let mesh_json = concat!(
        r#"{"schema":"mesh","name":"color_mesh","prim_groups":[{"#,
        r#""shader":"color_m","positions":[[0,0,0],[1,0,0],[0,1,0]],"#,
        r#""colours":[4294901760,2147548928,1073742079],"#,
        r#""indices":[0,1,2]}]}"#,
    );
    fs::create_dir_all(&mesh_dir)
        .and_then(|()| fs::write(mesh_dir.join("color_mesh.json"), mesh_json))
        .map_err(|error| error.to_string())?;
    let source = DecodedComponentSource::new(&root, root.join("textures"));
    let result = source.load_mesh("color_mesh");
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    let mesh = result
        .map_err(|error| format!("vertex-color decode failed: {error:?}"))?;
    let Some(group) = mesh.groups.first() else {
        return Err("vertex-color mesh has no primitive group".to_owned());
    };
    let expected = [[1., 0., 0., 1.], [0., 1., 0., 128. / 255.], [
        0.,
        0.,
        1.,
        64. / 255.,
    ]];
    if group.colors == expected {
        Ok(())
    } else {
        Err(format!(
            "unexpected normalized vertex colors: {:?}",
            group.colors
        ))
    }
}

// File:
//   - decoded_vertex_color.rs
// Path:
//   - src/fbx/tests/decoded_vertex_color.rs
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
//   - Regression coverage for decoded PDDI vertex-color channel order.
// - Must-Not:
//   - Read private assets or accept unvalidated color counts.
// - Allows:
//   - Synthetic decoded mesh JSON with packed `0xAARRGGBB` values.
// - Split-When:
//   - Another decoded color encoding needs an independent contract.
// - Merge-When:
//   - Decoded mesh tests adopt color-channel conformance.
// - Summary:
//   - Proves PDDI colors become normalized FBX RGBA values.
// - Description:
//   - Loads three packed colors and checks channel order and normalization.
// - Usage:
//   - Run through the fbx crate test target.
// - Defaults:
//   - Temporary fixture roots are process-specific and removed after the test.
//
// ADRs:
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
// - docs/adr/fbx/export/fbx-output-contract-boundary.md
//
// Large file:
//   - false
//

//! Regression coverage for PDDI packed vertex colors.
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
    std::env::temp_dir().join(
        format!(
            "fbx-decoded-vertex-color-{}",
            std::process::id()
        ),
    )
}

#[test]
fn decodes_pddi_aarrggbb_into_normalized_rgba() -> Result<(), String> {
    let root = temp_root();
    let mesh_dir = root
        .join("components")
        .join("mesh");
    let mesh_json = concat!(
        r#"{"schema":"mesh","name":"color_mesh","prim_groups":[{"#,
        r#""shader":"color_m","positions":[[0,0,0],[1,0,0],[0,1,0]],"#,
        r#""colours":[4294901760,2147548928,1073742079],"#,
        r#""indices":[0,1,2]}]}"#,
    );
    fs::create_dir_all(&mesh_dir)
        .and_then(
            |()| {
                fs::write(
                    mesh_dir.join("color_mesh.json"),
                    mesh_json,
                )
            },
        )
        .map_err(|error| error.to_string())?;
    let source = DecodedComponentSource::new(
        &root,
        root.join("textures"),
    );
    let result = source.load_mesh("color_mesh");
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    let mesh = result
        .map_err(|error| format!("vertex-color decode failed: {error:?}"))?;
    let Some(group) = mesh
        .groups
        .first()
    else {
        return Err("vertex-color mesh has no primitive group".to_owned());
    };
    let expected = [
        [
            1.0, 0.0, 0.0, 1.0,
        ],
        [
            0.0,
            1.0,
            0.0,
            128.0 / 255.0,
        ],
        [
            0.0,
            0.0,
            1.0,
            64.0 / 255.0,
        ],
    ];
    if group.colors == expected {
        Ok(())
    } else {
        Err(
            format!(
                "unexpected normalized vertex colors: {:?}",
                group.colors
            ),
        )
    }
}

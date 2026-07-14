// File:
//   - decoded_mesh_source.rs
// Path:
//   - src/fbx/tests/decoded_mesh_source.rs
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
//   - Regression coverage for decoded FBX mesh resolution boundaries.
// - Must-Not:
//   - Read private assets, discover packages, or use fixed machine paths.
// - Allows:
//   - Synthetic mesh JSON and process-unique temporary directories.
// - Split-When:
//   - Surface-layer decoding requires an independent test boundary.
// - Merge-When:
//   - Mesh regressions move into shared adapter conformance tests.
// - Summary:
//   - Protects decoded mesh identity and evidence before domain translation.
// - Description:
//   - Exercises decoded mesh resolution with synthetic evidence.
// - Usage:
//   - Run through the fbx crate test target.
// - Defaults:
//   - Temporary directories are process-unique and removed by each regression.
//
// ADRs:
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
//
// Large file:
//   - false
//

//! Regression coverage for decoded FBX mesh resolution boundaries.
//!
//! Synthetic mesh evidence verifies stable identity without private assets or
//! machine-local dependency routes.

use std::fs;
use std::path::PathBuf;

use fbx::adapters::driven::decoded_component_source::{
    DecodedComponentError, DecodedComponentSource,
};
use fbx::ports::component_source::ComponentSource;
use schoenwald_filesystem as _;
use serde as _;
use serde_json as _;

fn temp_root(label: &str) -> PathBuf {
    std::env::temp_dir().join(
        format!(
            "fbx-decoded-mesh-{label}-{}",
            std::process::id()
        ),
    )
}

#[test]
fn rejects_windows_device_member_ids() {
    let root = temp_root("windows-device-name");
    let source = DecodedComponentSource::new(
        &root,
        root.join("textures"),
    );
    let result = source.load_mesh("CON");

    assert_eq!(
        result,
        Err(DecodedComponentError::InvalidMemberId("CON".to_owned()))
    );
}

#[test]
fn rejects_member_ids_with_nonportable_file_characters() {
    let root = temp_root("nonportable-character");
    let source = DecodedComponentSource::new(
        &root,
        root.join("textures"),
    );
    let result = source.load_mesh("mesh:stream");

    assert_eq!(
        result,
        Err(DecodedComponentError::InvalidMemberId("mesh:stream".to_owned()))
    );
}

#[test]
fn rejects_member_ids_with_trailing_dots() {
    let root = temp_root("trailing-dot");
    let source = DecodedComponentSource::new(
        &root,
        root.join("textures"),
    );
    let result = source.load_mesh("mesh.");

    assert_eq!(
        result,
        Err(DecodedComponentError::InvalidMemberId("mesh.".to_owned()))
    );
}

#[test]
fn rejects_member_ids_with_surrounding_whitespace() {
    let root = temp_root("member-whitespace");
    let mesh_dir = root
        .join("components")
        .join("mesh");
    let mesh_json = concat!(
        r#"{"schema":"mesh","name":" mesh","prim_groups":[{"#,
        r#""shader":"shader","positions":[[0,0,0],[1,0,0],[0,1,0]],"#,
        r#""indices":[0,1,2]}]}"#,
    );
    let setup_result = fs::create_dir_all(&mesh_dir).and_then(
        |()| {
            fs::write(
                mesh_dir.join(" mesh.json"),
                mesh_json,
            )
        },
    );
    assert!(setup_result.is_ok());
    let source = DecodedComponentSource::new(
        &root,
        root.join("textures"),
    );
    let result = source.load_mesh(" mesh");
    let _cleanup_result = fs::remove_dir_all(&root);

    assert_eq!(
        result,
        Err(DecodedComponentError::InvalidMemberId(" mesh".to_owned()))
    );
}

#[test]
fn rejects_declared_uv_channels_without_coordinates() {
    let root = temp_root("empty-uv-channel");
    let mesh_dir = root
        .join("components")
        .join("mesh");
    let mesh_json = concat!(
        r#"{"schema":"mesh","name":"mesh","prim_groups":[{"#,
        r#""shader":"shader","positions":[[0,0,0],[1,0,0],[0,1,0]],"#,
        r#""indices":[0,1,2],"uvs":[{"channel":0,"coords":[]}]}]}"#,
    );
    let setup_result = fs::create_dir_all(&mesh_dir).and_then(
        |()| {
            fs::write(
                mesh_dir.join("mesh.json"),
                mesh_json,
            )
        },
    );
    assert!(setup_result.is_ok());
    let source = DecodedComponentSource::new(
        &root,
        root.join("textures"),
    );
    let result = source.load_mesh("mesh");
    let _cleanup_result = fs::remove_dir_all(&root);

    assert_eq!(
        result,
        Err(
            DecodedComponentError::EmptyUvChannel {
                group: 0,
                channel: 0,
            }
        )
    );
}

#[test]
fn rejects_mesh_identity_mismatches() {
    let root = temp_root("identity-mismatch");
    let mesh_dir = root
        .join("components")
        .join("mesh");
    let mesh_json = concat!(
        r#"{"schema":"mesh","name":"decoded","prim_groups":[{"#,
        r#""shader":"shader","positions":[[0,0,0],[1,0,0],[0,1,0]],"#,
        r#""indices":[0,1,2]}]}"#,
    );
    let setup_result = fs::create_dir_all(&mesh_dir).and_then(
        |()| {
            fs::write(
                mesh_dir.join("requested.json"),
                mesh_json,
            )
        },
    );
    assert!(setup_result.is_ok());
    let source = DecodedComponentSource::new(
        &root,
        root.join("textures"),
    );
    let result = source.load_mesh("requested");
    let _cleanup_result = fs::remove_dir_all(&root);

    assert_eq!(
        result,
        Err(
            DecodedComponentError::MeshIdentityMismatch {
                requested: "requested".to_owned(),
                decoded: "decoded".to_owned(),
            }
        )
    );
}

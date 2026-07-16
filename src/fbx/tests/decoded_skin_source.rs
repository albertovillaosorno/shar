// File:
//   - decoded_skin_source.rs
// Path:
//   - src/fbx/tests/decoded_skin_source.rs
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
//   - Regression coverage for decoded skeleton and skin count contracts.
// - Must-Not:
//   - Read private assets, discover packages, or invoke external applications.
// - Allows:
//   - Synthetic decoded JSON and process-unique temporary files.
// - Split-When:
//   - Skin geometry fixtures gain an independent conformance boundary.
// - Merge-When:
//   - Decoded character regressions move into shared adapter tests.
// - Summary:
//   - Protects decoded skeleton declarations before character construction.
// - Description:
//   - Exercises declared count validation with deterministic synthetic JSON.
// - Usage:
//   - Run through the fbx crate test target.
// - Defaults:
//   - Temporary files are process-unique and removed by each regression.
//
// ADRs:
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
//
// Large file:
//   - true
//   - Reason: Synthetic skeleton, skin, and composite count regressions share
//   - one public decoded-source boundary and deterministic temporary fixtures.
//

//! Regression coverage for decoded skeleton and skin count contracts.
//!
//! Synthetic JSON proves contradictory extraction evidence fails before FBX
//! character construction without reading private game assets.

use std::fs;
use std::path::PathBuf;

use fbx::adapters::driven::decoded_skin_source::{
    SkinSourceError, load_character, load_skeleton, load_skin_part,
};
use png as _;
use schoenwald_filesystem as _;
use serde as _;
use serde_json as _;
use shar_sha256 as _;

fn temp_path(label: &str) -> PathBuf {
    std::env::temp_dir().join(
        format!(
            "fbx-decoded-{label}-{}.json",
            std::process::id()
        ),
    )
}

#[test]
fn rejects_declared_joint_count_mismatch() -> Result<(), String> {
    let path = temp_path("skeleton-count");
    let fixture = concat!(
        r#"{"schema":"skeleton","name":"skeleton","version":0,"#,
        r#""num_joints":2,"joints":[{"name":"root","parent":0,"#,
        r#""dof":0,"free_axes":0,"primary_axis":0,"secondary_axis":0,"#,
        r#""twist_axis":0,"rest_pose":[1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1]}]}"#,
    );
    fs::write(
        &path, fixture,
    )
    .map_err(|write_error| write_error.to_string())?;

    let error = load_skeleton(&path).err();

    fs::remove_file(&path).map_err(|remove_error| remove_error.to_string())?;
    let expected = Some(
        SkinSourceError::JointCountMismatch {
            path: path
                .display()
                .to_string(),
            declared: 2,
            actual: 1,
        },
    );
    if error == expected {
        Ok(())
    } else {
        Err("declared skeleton joint-count mismatch was accepted".to_owned())
    }
}

#[test]
fn rejects_declared_primitive_group_count_mismatch() -> Result<(), String> {
    let path = temp_path("skin-group-count");
    let fixture = concat!(
        r#"{"schema":"skin","name":"skin","version":3,"#,
        r#""skeleton_name":"skeleton","num_prim_groups":1,"#,
        r#""prim_groups":[]}"#,
    );
    fs::write(
        &path, fixture,
    )
    .map_err(|write_error| write_error.to_string())?;

    let error = load_skin_part(
        &path,
        &[],
    )
    .err();

    fs::remove_file(&path).map_err(|remove_error| remove_error.to_string())?;
    match error {
        Some(SkinSourceError::PrimitiveGroupCountMismatch {
            declared: 1,
            actual: 0,
            ..
        }) => Ok(()),
        _ => Err("skin group-count mismatch was accepted".to_owned()),
    }
}

#[test]
fn rejects_declared_vertex_count_mismatch() -> Result<(), String> {
    let path = temp_path("skin-vertex-count");
    let fixture = concat!(
        r#"{"schema":"skin","name":"skin","version":3,"#,
        r#""skeleton_name":"skeleton","num_prim_groups":1,"#,
        r#""prim_groups":[{"shader":"shader","vertex_shader":"","#,
        r#""prim_type":0,"vertex_format":0,"vertex_count":1,"#,
        r#""index_count":0,"matrix_count":0,"positions":[],"normals":[],"#,
        r#""matrices":[],"matrix_palette":[],"indices":[],"uvs":[]}]}"#,
    );
    fs::write(
        &path, fixture,
    )
    .map_err(|write_error| write_error.to_string())?;

    let error = load_skin_part(
        &path,
        &[],
    )
    .err();

    fs::remove_file(&path).map_err(|remove_error| remove_error.to_string())?;
    match error {
        Some(SkinSourceError::VertexCountMismatch {
            group: 0,
            declared: 1,
            actual: 0,
            ..
        }) => Ok(()),
        _ => Err("skin vertex-count mismatch was accepted".to_owned()),
    }
}

#[test]
fn rejects_declared_index_count_mismatch() -> Result<(), String> {
    let path = temp_path("skin-index-count");
    let fixture = concat!(
        r#"{"schema":"skin","name":"skin","version":3,"#,
        r#""skeleton_name":"skeleton","num_prim_groups":1,"#,
        r#""prim_groups":[{"shader":"shader","vertex_shader":"","#,
        r#""prim_type":0,"vertex_format":0,"vertex_count":0,"#,
        r#""index_count":1,"matrix_count":0,"positions":[],"normals":[],"#,
        r#""matrices":[],"matrix_palette":[],"indices":[],"uvs":[]}]}"#,
    );
    fs::write(
        &path, fixture,
    )
    .map_err(|write_error| write_error.to_string())?;

    let error = load_skin_part(
        &path,
        &[],
    )
    .err();

    fs::remove_file(&path).map_err(|remove_error| remove_error.to_string())?;
    match error {
        Some(SkinSourceError::IndexCountMismatch {
            group: 0,
            declared: 1,
            actual: 0,
            ..
        }) => Ok(()),
        _ => Err("skin index-count mismatch was accepted".to_owned()),
    }
}

#[test]
fn rejects_declared_matrix_palette_count_mismatch() -> Result<(), String> {
    let path = temp_path("skin-matrix-count");
    let fixture = concat!(
        r#"{"schema":"skin","name":"skin","version":3,"#,
        r#""skeleton_name":"skeleton","num_prim_groups":1,"#,
        r#""prim_groups":[{"shader":"shader","vertex_shader":"","#,
        r#""prim_type":0,"vertex_format":0,"vertex_count":0,"#,
        r#""index_count":0,"matrix_count":1,"positions":[],"normals":[],"#,
        r#""matrices":[],"matrix_palette":[],"indices":[],"uvs":[]}]}"#,
    );
    fs::write(
        &path, fixture,
    )
    .map_err(|write_error| write_error.to_string())?;

    let error = load_skin_part(
        &path,
        &[],
    )
    .err();

    fs::remove_file(&path).map_err(|remove_error| remove_error.to_string())?;
    match error {
        Some(SkinSourceError::MatrixPaletteCountMismatch {
            group: 0,
            declared: 1,
            actual: 0,
            ..
        }) => Ok(()),
        _ => Err("skin matrix-count mismatch was accepted".to_owned()),
    }
}

#[test]
fn rejects_declared_composite_skin_count_mismatch() -> Result<(), String> {
    let skeleton_path = temp_path("composite-count-skeleton");
    let composite_path = temp_path("composite-count");
    let skeleton_fixture = concat!(
        r#"{"schema":"skeleton","name":"skeleton","version":0,"#,
        r#""num_joints":1,"joints":[{"name":"root","parent":0,"#,
        r#""dof":0,"free_axes":0,"primary_axis":0,"secondary_axis":0,"#,
        r#""twist_axis":0,"rest_pose":[1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1]}]}"#,
    );
    let composite_fixture = concat!(
        r#"{"schema":"composite_drawable","name":"character","#,
        r#""skeleton_name":"skeleton","num_skins":1,"skins":[]}"#,
    );
    fs::write(
        &skeleton_path,
        skeleton_fixture,
    )
    .map_err(|write_error| write_error.to_string())?;
    fs::write(
        &composite_path,
        composite_fixture,
    )
    .map_err(|write_error| write_error.to_string())?;

    let composite_paths = [composite_path.as_path()];
    let error = load_character(
        "character",
        &skeleton_path,
        &[],
        &[],
        &composite_paths,
    )
    .err();

    fs::remove_file(&skeleton_path)
        .map_err(|remove_error| remove_error.to_string())?;
    fs::remove_file(&composite_path)
        .map_err(|remove_error| remove_error.to_string())?;
    match error {
        Some(SkinSourceError::CompositeSkinCountMismatch {
            declared: 1,
            actual: 0,
            ..
        }) => Ok(()),
        _ => Err("composite skin-count mismatch was accepted".to_owned()),
    }
}

#[test]
fn rejects_declared_composite_prop_count_mismatch() -> Result<(), String> {
    let skeleton_path = temp_path("composite-prop-count-skeleton");
    let composite_path = temp_path("composite-prop-count");
    let skeleton_fixture = concat!(
        r#"{"schema":"skeleton","name":"skeleton","version":0,"#,
        r#""num_joints":1,"joints":[{"name":"root","parent":0,"#,
        r#""dof":0,"free_axes":0,"primary_axis":0,"secondary_axis":0,"#,
        r#""twist_axis":0,"rest_pose":[1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1]}]}"#,
    );
    let composite_fixture = concat!(
        r#"{"schema":"composite_drawable","name":"character","#,
        r#""skeleton_name":"skeleton","num_skins":0,"skins":[],"#,
        r#""num_props":1,"props":[]}"#,
    );
    fs::write(
        &skeleton_path,
        skeleton_fixture,
    )
    .map_err(|write_error| write_error.to_string())?;
    fs::write(
        &composite_path,
        composite_fixture,
    )
    .map_err(|write_error| write_error.to_string())?;

    let composite_paths = [composite_path.as_path()];
    let error = load_character(
        "character",
        &skeleton_path,
        &[],
        &[],
        &composite_paths,
    )
    .err();

    fs::remove_file(&skeleton_path)
        .map_err(|remove_error| remove_error.to_string())?;
    fs::remove_file(&composite_path)
        .map_err(|remove_error| remove_error.to_string())?;
    match error {
        Some(SkinSourceError::CompositePropCountMismatch {
            declared: 1,
            actual: 0,
            ..
        }) => Ok(()),
        _ => Err("composite prop-count mismatch was accepted".to_owned()),
    }
}

#[test]
fn rejects_declared_composite_effect_count_mismatch() -> Result<(), String> {
    let skeleton_path = temp_path("composite-effect-count-skeleton");
    let composite_path = temp_path("composite-effect-count");
    let skeleton_fixture = concat!(
        r#"{"schema":"skeleton","name":"skeleton","version":0,"#,
        r#""num_joints":1,"joints":[{"name":"root","parent":0,"#,
        r#""dof":0,"free_axes":0,"primary_axis":0,"secondary_axis":0,"#,
        r#""twist_axis":0,"rest_pose":[1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1]}]}"#,
    );
    let composite_fixture = concat!(
        r#"{"schema":"composite_drawable","name":"character","#,
        r#""skeleton_name":"skeleton","num_skins":0,"skins":[],"#,
        r#""num_props":0,"props":[],"num_effects":1,"effects":[]}"#,
    );
    fs::write(
        &skeleton_path,
        skeleton_fixture,
    )
    .map_err(|write_error| write_error.to_string())?;
    fs::write(
        &composite_path,
        composite_fixture,
    )
    .map_err(|write_error| write_error.to_string())?;

    let composite_paths = [composite_path.as_path()];
    let error = load_character(
        "character",
        &skeleton_path,
        &[],
        &[],
        &composite_paths,
    )
    .err();

    fs::remove_file(&skeleton_path)
        .map_err(|remove_error| remove_error.to_string())?;
    fs::remove_file(&composite_path)
        .map_err(|remove_error| remove_error.to_string())?;
    match error {
        Some(SkinSourceError::CompositeEffectCountMismatch {
            declared: 1,
            actual: 0,
            ..
        }) => Ok(()),
        _ => Err("composite effect-count mismatch was accepted".to_owned()),
    }
}

#[test]
fn rejects_unsupported_skeleton_version() -> Result<(), String> {
    let path = temp_path("skeleton-version");
    let fixture = concat!(
        r#"{"schema":"skeleton","name":"skeleton","version":1,"#,
        r#""num_joints":1,"joints":[{"name":"root","parent":0,"#,
        r#""dof":0,"free_axes":0,"primary_axis":0,"secondary_axis":0,"#,
        r#""twist_axis":0,"rest_pose":[1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1]}]}"#,
    );
    fs::write(
        &path, fixture,
    )
    .map_err(|write_error| write_error.to_string())?;

    let error = load_skeleton(&path).err();

    fs::remove_file(&path).map_err(|remove_error| remove_error.to_string())?;
    match error {
        Some(SkinSourceError::UnsupportedSkeletonVersion {
            version: 1,
            ..
        }) => Ok(()),
        _ => Err("unsupported skeleton version was accepted".to_owned()),
    }
}

#[test]
fn rejects_unsupported_skin_version() -> Result<(), String> {
    let path = temp_path("skin-version");
    let fixture = concat!(
        r#"{"schema":"skin","name":"skin","version":2,"#,
        r#""skeleton_name":"skeleton","num_prim_groups":0,"#,
        r#""prim_groups":[]}"#,
    );
    fs::write(
        &path, fixture,
    )
    .map_err(|write_error| write_error.to_string())?;

    let error = load_skin_part(
        &path,
        &[],
    )
    .err();

    fs::remove_file(&path).map_err(|remove_error| remove_error.to_string())?;
    match error {
        Some(SkinSourceError::UnsupportedSkinVersion {
            version: 2,
            ..
        }) => Ok(()),
        _ => Err("unsupported skin version was accepted".to_owned()),
    }
}

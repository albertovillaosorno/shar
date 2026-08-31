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
//   - Decoded skin source test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Decoded skin source test module.
// - Description:
//   - Implements the declared test module responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Decoded skin source test module.

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
    std::env::temp_dir()
        .join(format!("fbx-decoded-{label}-{}.json", std::process::id()))
}

#[test]
fn rejects_skeleton_identity_with_surrounding_whitespace() -> Result<(), String>
{
    let path = temp_path("skeleton-name-whitespace");
    let fixture = concat!(
        r#"{"schema":"skeleton","name":" skeleton","version":0,"#,
        r#""num_joints":1,"joints":[{"name":"root","parent":0,"#,
        r#""dof":0,"free_axes":0,"primary_axis":0,"secondary_axis":0,"#,
        r#""twist_axis":0,"rest_pose":[1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1]}]}"#,
    );
    fs::write(&path, fixture).map_err(|error| error.to_string())?;
    let result = load_skeleton(&path);
    fs::remove_file(&path).map_err(|error| error.to_string())?;
    match result {
        Err(SkinSourceError::NonCanonicalIdentity { field, .. })
            if field == "skeleton name" =>
        {
            Ok(())
        },
        other => Err(format!("unexpected skeleton identity result: {other:?}")),
    }
}

#[test]
fn rejects_joint_identity_with_surrounding_whitespace() -> Result<(), String> {
    let path = temp_path("joint-name-whitespace");
    let fixture = concat!(
        r#"{"schema":"skeleton","name":"skeleton","version":0,"#,
        r#""num_joints":1,"joints":[{"name":" root","parent":0,"#,
        r#""dof":0,"free_axes":0,"primary_axis":0,"secondary_axis":0,"#,
        r#""twist_axis":0,"rest_pose":[1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1]}]}"#,
    );
    fs::write(&path, fixture).map_err(|error| error.to_string())?;
    let result = load_skeleton(&path);
    fs::remove_file(&path).map_err(|error| error.to_string())?;
    match result {
        Err(SkinSourceError::NonCanonicalIdentity { field, .. })
            if field == "joint name" =>
        {
            Ok(())
        },
        other => Err(format!("unexpected joint identity result: {other:?}")),
    }
}

#[test]
fn rejects_composite_skeleton_reference_with_surrounding_whitespace()
-> Result<(), String> {
    let skeleton_path = temp_path("composite-space-skeleton");
    let composite_path = temp_path("composite-space-reference");
    let skeleton_fixture = concat!(
        r#"{"schema":"skeleton","name":"skeleton","version":0,"#,
        r#""num_joints":1,"joints":[{"name":"root","parent":0,"#,
        r#""dof":0,"free_axes":0,"primary_axis":0,"secondary_axis":0,"#,
        r#""twist_axis":0,"rest_pose":[1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1]}]}"#,
    );
    let composite_fixture = concat!(
        r#"{"schema":"composite_drawable","name":"character","#,
        r#""skeleton_name":" skeleton","num_skins":0,"skins":[],"#,
        r#""num_props":0,"props":[],"num_effects":0,"effects":[]}"#,
    );
    fs::write(&skeleton_path, skeleton_fixture)
        .and_then(|()| fs::write(&composite_path, composite_fixture))
        .map_err(|error| error.to_string())?;
    let composite_paths = [composite_path.as_path()];
    let result =
        load_character("character", &skeleton_path, &[], &[], &composite_paths);
    fs::remove_file(&skeleton_path).map_err(|error| error.to_string())?;
    fs::remove_file(&composite_path).map_err(|error| error.to_string())?;
    match result {
        Err(SkinSourceError::NonCanonicalIdentity { field, .. })
            if field == "composite skeleton name" =>
        {
            Ok(())
        },
        other => {
            Err(format!("unexpected composite identity result: {other:?}"))
        },
    }
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
    fs::write(&path, fixture).map_err(|write_error| write_error.to_string())?;

    let error = load_skeleton(&path).err();

    fs::remove_file(&path).map_err(|remove_error| remove_error.to_string())?;
    let expected = Some(SkinSourceError::JointCountMismatch {
        path: path.display().to_string(),
        declared: 2,
        actual: 1,
    });
    if error == expected {
        Ok(())
    } else {
        Err("declared skeleton joint-count mismatch was accepted".to_owned())
    }
}

#[test]
fn preserves_typed_source_joint_rig_semantics() -> Result<(), String> {
    let path = temp_path("skeleton-source-rig");
    let fixture = concat!(
        r#"{"schema":"skeleton","name":"skeleton","version":0,"#,
        r#""num_joints":1,"joints":[{"name":"root","parent":0,"#,
        r#""dof":4294967295,"free_axes":4294967295,"#,
        r#""primary_axis":4294967295,"secondary_axis":4294967295,"#,
        r#""twist_axis":4294967295,"#,
        r#""rest_pose":[1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1],"#,
        r#""joint_metadata":[{"kind":"joint_mirror_map","index":7,"#,
        r#""scale":[1,1,1]},{"kind":"joint_fix_flag","flags":1}]}]}"#,
    );
    fs::write(&path, fixture).map_err(|error| error.to_string())?;
    let (_name, bones) =
        load_skeleton(&path).map_err(|error| format!("{error:?}"))?;
    fs::remove_file(&path).map_err(|error| error.to_string())?;
    let rig = bones
        .first()
        .and_then(|bone| bone.source_rig)
        .ok_or_else(|| "source rig metadata was dropped".to_owned())?;
    if rig.dof != u32::MAX
        || rig.free_axes != u32::MAX
        || rig.primary_axis != u32::MAX
        || rig.secondary_axis != u32::MAX
        || rig.twist_axis != u32::MAX
        || rig.fix_flags != Some(1)
    {
        return Err("source rig scalar controls changed".to_owned());
    }
    let mirror = rig
        .mirror_map
        .ok_or_else(|| "source mirror map was dropped".to_owned())?;
    if mirror.index != 7 || mirror.scale != [1., 1., 1.] {
        return Err("source mirror map changed".to_owned());
    }
    Ok(())
}

#[test]
fn rejects_unknown_source_joint_metadata() -> Result<(), String> {
    let path = temp_path("skeleton-unknown-rig-metadata");
    let fixture = concat!(
        r#"{"schema":"skeleton","name":"skeleton","version":0,"#,
        r#""num_joints":1,"joints":[{"name":"root","parent":0,"#,
        r#""dof":0,"free_axes":0,"primary_axis":0,"#,
        r#""secondary_axis":0,"twist_axis":0,"#,
        r#""rest_pose":[1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1],"#,
        r#""joint_metadata":[{"kind":"unknown_rig_record"}]}]}"#,
    );
    fs::write(&path, fixture).map_err(|error| error.to_string())?;
    let error = load_skeleton(&path).err();
    fs::remove_file(&path).map_err(|error| error.to_string())?;
    match error {
        Some(SkinSourceError::Parse { path: found, .. })
            if found == path.display().to_string() =>
        {
            Ok(())
        },
        _ => Err("unknown source joint metadata was accepted".to_owned()),
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
    fs::write(&path, fixture).map_err(|write_error| write_error.to_string())?;

    let error = load_skin_part(&path, &[]).err();

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
    fs::write(&path, fixture).map_err(|write_error| write_error.to_string())?;

    let error = load_skin_part(&path, &[]).err();

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
    fs::write(&path, fixture).map_err(|write_error| write_error.to_string())?;

    let error = load_skin_part(&path, &[]).err();

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
fn rejects_four_index_skin_triangle_list_as_malformed_source()
-> Result<(), String> {
    let path = temp_path("skin-four-index-triangle-list");
    let fixture = concat!(
        r#"{"schema":"skin","name":"skin","version":3,"#,
        r#""skeleton_name":"skeleton","num_prim_groups":1,"#,
        r#""prim_groups":[{"shader":"shader","vertex_shader":"","#,
        r#""prim_type":0,"vertex_format":0,"vertex_count":4,"#,
        r#""index_count":4,"matrix_count":0,"#,
        r#""positions":[[0,0,0],[1,0,0],[1,1,0],[0,1,0]],"#,
        r#""normals":[[0,0,1],[0,0,1],[0,0,1],[0,0,1]],"#,
        r#""matrices":[],"matrix_palette":[],"indices":[0,1,2,3],"#,
        r#""uvs":[]}]}"#,
    );
    fs::write(&path, fixture).map_err(|error| error.to_string())?;
    let error = load_skin_part(&path, &[]).err();
    fs::remove_file(&path).map_err(|error| error.to_string())?;

    match error {
        Some(SkinSourceError::Mesh {
            error: fbx::domain::mesh::MeshError::UnsupportedIndexCount(4),
            ..
        }) => Ok(()),
        other => Err(format!(
            "malformed skin triangle list was accepted: {other:?}"
        )),
    }
}

#[test]
fn rejects_unsupported_skin_uv_channels() -> Result<(), String> {
    let path = temp_path("skin-unsupported-uv-channel");
    let fixture = concat!(
        r#"{"schema":"skin","name":"skin","version":3,"#,
        r#""skeleton_name":"skeleton","num_prim_groups":1,"#,
        r#""prim_groups":[{"shader":"shader","vertex_shader":"","#,
        r#""prim_type":0,"vertex_format":0,"vertex_count":3,"#,
        r#""index_count":3,"matrix_count":0,"#,
        r#""positions":[[0,0,0],[1,0,0],[0,1,0]],"normals":[],"#,
        r#""matrices":[],"matrix_palette":[],"indices":[0,1,2],"#,
        r#""uvs":[{"channel":1,"coords":[[0,0],[1,0],[0,1]]}]}]}"#,
    );
    fs::write(&path, fixture).map_err(|error| error.to_string())?;
    let error = load_skin_part(&path, &[]).err();
    fs::remove_file(&path).map_err(|error| error.to_string())?;

    match error {
        Some(SkinSourceError::UnsupportedUvChannel {
            group: 0,
            channel: 1,
            ..
        }) => Ok(()),
        other => Err(format!(
            "unsupported skin UV channel was accepted: {other:?}"
        )),
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
    fs::write(&path, fixture).map_err(|write_error| write_error.to_string())?;

    let error = load_skin_part(&path, &[]).err();

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
    fs::write(&skeleton_path, skeleton_fixture)
        .map_err(|write_error| write_error.to_string())?;
    fs::write(&composite_path, composite_fixture)
        .map_err(|write_error| write_error.to_string())?;

    let composite_paths = [composite_path.as_path()];
    let error =
        load_character("character", &skeleton_path, &[], &[], &composite_paths)
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
    fs::write(&skeleton_path, skeleton_fixture)
        .map_err(|write_error| write_error.to_string())?;
    fs::write(&composite_path, composite_fixture)
        .map_err(|write_error| write_error.to_string())?;

    let composite_paths = [composite_path.as_path()];
    let error =
        load_character("character", &skeleton_path, &[], &[], &composite_paths)
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
    fs::write(&skeleton_path, skeleton_fixture)
        .map_err(|write_error| write_error.to_string())?;
    fs::write(&composite_path, composite_fixture)
        .map_err(|write_error| write_error.to_string())?;

    let composite_paths = [composite_path.as_path()];
    let error =
        load_character("character", &skeleton_path, &[], &[], &composite_paths)
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
    fs::write(&path, fixture).map_err(|write_error| write_error.to_string())?;

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
    fs::write(&path, fixture).map_err(|write_error| write_error.to_string())?;

    let error = load_skin_part(&path, &[]).err();

    fs::remove_file(&path).map_err(|remove_error| remove_error.to_string())?;
    match error {
        Some(SkinSourceError::UnsupportedSkinVersion {
            version: 2, ..
        }) => Ok(()),
        _ => Err("unsupported skin version was accepted".to_owned()),
    }
}

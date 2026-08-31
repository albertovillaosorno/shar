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
//   - Decoded rigid prop source test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Decoded rigid prop source test module.
// - Description:
//   - Implements the declared test module responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Decoded rigid prop source test module.

use std::fs;
use std::path::{Path, PathBuf};

use fbx::adapters::driven::decoded_rigid_prop_source;
use fbx::adapters::driven::decoded_skin_source::SkinSourceError;
use png as _;
use schoenwald_filesystem as _;
use serde as _;
use serde_json as _;
use shar_sha256 as _;

fn temp_root(label: &str) -> PathBuf {
    std::env::temp_dir()
        .join(format!("fbx-rigid-prop-{label}-{}", std::process::id()))
}

const fn skeleton_json() -> &'static str {
    concat!(
        r#"{"schema":"skeleton","name":"rig","version":0,"#,
        r#""num_joints":4,"joints":["#,
        r#"{"name":"root","parent":0,"dof":0,"free_axes":0,"#,
        r#""primary_axis":0,"secondary_axis":0,"twist_axis":0,"#,
        r#""rest_pose":[1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1]},"#,
        r#"{"name":"body","parent":0,"dof":0,"free_axes":0,"#,
        r#""primary_axis":0,"secondary_axis":0,"twist_axis":0,"#,
        r#""rest_pose":[1,0,0,0,0,1,0,0,0,0,1,0,2,3,4,1]},"#,
        r#"{"name":"wing","parent":1,"dof":0,"free_axes":0,"#,
        r#""primary_axis":0,"secondary_axis":0,"twist_axis":0,"#,
        r#""rest_pose":[1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1]},"#,
        r#"{"name":"glow","parent":1,"dof":0,"free_axes":0,"#,
        r#""primary_axis":0,"secondary_axis":0,"twist_axis":0,"#,
        r#""rest_pose":[1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1]}]}"#,
    )
}

const fn composite_json() -> &'static str {
    concat!(
        r#"{"schema":"composite_drawable","name":"rig","#,
        r#""skeleton_name":"rig","num_skins":0,"skins":[],"#,
        r#""num_props":2,"props":["#,
        r#"{"kind":"prop","name":"BodyShape","is_translucent":1,"#,
        r#""skeleton_joint_id":1,"sort_order":0},"#,
        r#"{"kind":"prop","name":"GlowShape","is_translucent":1,"#,
        r#""skeleton_joint_id":3,"sort_order":0}],"#,
        r#""num_effects":1,"effects":["#,
        r#"{"kind":"effect","name":"ParticleShape","is_translucent":1,"#,
        r#""skeleton_joint_id":3,"sort_order":0}]}"#,
    )
}

fn mesh_json(name: &str) -> String {
    format!(
        concat!(
            r#"{{"schema":"mesh","name":"{}","prim_groups":[{{"#,
            r#""shader":"body_m","positions":[[0,0,0],[1,0,0],[0,1,0]],"#,
            r#""indices":[0,1,2]}}]}}"#,
        ),
        name
    )
}

fn mesh_with_normals_json(name: &str) -> String {
    format!(
        concat!(
            r#"{{"schema":"mesh","name":"{}","prim_groups":[{{"#,
            r#""shader":"body_m","positions":[[0,0,0],[1,0,0],[0,1,0]],"#,
            r#""normals":[[0.70710677,0.70710677,0],"#,
            r#"[0.70710677,0.70710677,0],[0.70710677,0.70710677,0]],"#,
            r#""indices":[0,1,2]}}]}}"#,
        ),
        name
    )
}

const fn scaled_skeleton_json() -> &'static str {
    concat!(
        r#"{"schema":"skeleton","name":"rig","version":0,"#,
        r#""num_joints":4,"joints":["#,
        r#"{"name":"root","parent":0,"dof":0,"free_axes":0,"#,
        r#""primary_axis":0,"secondary_axis":0,"twist_axis":0,"#,
        r#""rest_pose":[1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1]},"#,
        r#"{"name":"body","parent":0,"dof":0,"free_axes":0,"#,
        r#""primary_axis":0,"secondary_axis":0,"twist_axis":0,"#,
        r#""rest_pose":[2,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1]},"#,
        r#"{"name":"wing","parent":1,"dof":0,"free_axes":0,"#,
        r#""primary_axis":0,"secondary_axis":0,"twist_axis":0,"#,
        r#""rest_pose":[1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1]},"#,
        r#"{"name":"glow","parent":1,"dof":0,"free_axes":0,"#,
        r#""primary_axis":0,"secondary_axis":0,"twist_axis":0,"#,
        r#""rest_pose":[1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1]}]}"#,
    )
}

fn write_fixture(
    root: &Path,
    mesh_name: &str,
) -> Result<(PathBuf, PathBuf, PathBuf), String> {
    let skeleton_path = root.join("skeleton.json");
    let composite_path = root.join("composite.json");
    let mesh_path = root.join(format!("{mesh_name}.json"));
    fs::create_dir_all(root)
        .and_then(|()| fs::write(&skeleton_path, skeleton_json()))
        .and_then(|()| fs::write(&composite_path, composite_json()))
        .and_then(|()| fs::write(&mesh_path, mesh_json(mesh_name)))
        .map_err(|error| error.to_string())?;
    Ok((skeleton_path, composite_path, mesh_path))
}

fn remove_fixture(root: &Path) -> Result<(), String> {
    fs::remove_dir_all(root).map_err(|error| error.to_string())
}

#[test]
fn loads_selected_prop_from_collision_renamed_physical_path()
-> Result<(), String> {
    let root = temp_root("collision-renamed-selection");
    let skeleton_path = root.join("skeleton.json");
    let composite_path = root.join("composite.json");
    let mesh_path = root.join("BodyShape__ordinal_7.json");
    fs::create_dir_all(&root)
        .and_then(|()| fs::write(&skeleton_path, skeleton_json()))
        .and_then(|()| fs::write(&composite_path, composite_json()))
        .and_then(|()| fs::write(&mesh_path, mesh_json("BodyShape")))
        .map_err(|error| error.to_string())?;

    let selected = decoded_rigid_prop_source::load_selected_rigid_prop_asset(
        "selected",
        &skeleton_path,
        &[mesh_path.as_path()],
        &composite_path,
    );
    let instanced = decoded_rigid_prop_source::load_instanced_rigid_prop_asset(
        "instanced",
        &skeleton_path,
        &[mesh_path.as_path()],
        &composite_path,
    );
    remove_fixture(&root)?;
    for (label, result) in [("selected", selected), ("instanced", instanced)] {
        let asset = result.map_err(|error| {
            format!("collision-renamed {label} load failed: {error:?}")
        })?;
        let part = asset.parts.first().ok_or_else(|| {
            format!("collision-renamed {label} load produced no part")
        })?;
        if part.mesh.source_identity.as_deref() != Some("BodyShape") {
            return Err(format!(
                concat!(
                    "collision-renamed {} load changed authored ",
                    "identity: {:?}"
                ),
                label, part.mesh.source_identity
            ));
        }
    }
    Ok(())
}

#[test]
fn loads_selected_prop_and_prunes_unselected_branches() -> Result<(), String> {
    let root = temp_root("selection");
    let (skeleton_path, composite_path, mesh_path) =
        write_fixture(&root, "BodyShape")?;

    let result = decoded_rigid_prop_source::load_selected_rigid_prop_asset(
        "selected",
        &skeleton_path,
        &[mesh_path.as_path()],
        &composite_path,
    );
    remove_fixture(&root)?;
    let asset =
        result.map_err(|error| format!("selection failed: {error:?}"))?;

    let provenance = asset.source_provenance.as_ref().ok_or_else(|| {
        "selected rigid prop lost source relationships".to_owned()
    })?;
    if provenance.skeleton_identity() != "rig"
        || provenance.composite_identities() != ["rig"]
    {
        return Err(format!("unexpected source relationships: {provenance:?}"));
    }
    let bone_ids = asset
        .bones
        .iter()
        .map(|bone| bone.id.as_str())
        .collect::<Vec<_>>();
    if bone_ids != ["root", "body"] {
        return Err(format!("unexpected retained bones: {bone_ids:?}"));
    }
    let Some(part) = asset.parts.first() else {
        return Err("selected rigid prop produced no part".to_owned());
    };
    if asset.parts.len() != 1
        || part.mesh.name != "BodyShape__transparent-source"
    {
        return Err(
            "selected rigid prop did not preserve one body mesh".to_owned()
        );
    }
    let positions = &part
        .mesh
        .groups
        .first()
        .ok_or_else(|| "selected rigid prop has no primitive group".to_owned())?
        .positions;
    if positions != &[[2., 3., 4.], [3., 3., 4.], [2., 4., 4.]] {
        return Err(format!(
            "selected rigid prop did not bake its authored rest \
                 transform: {positions:?}"
        ));
    }
    if part.group_influences.iter().flatten().any(|influence| {
        influence.bone_id != "body"
            || (influence.weight - 1.).abs() > f32::EPSILON
    }) {
        return Err(
            "selected rigid prop was not fully bound to body".to_owned()
        );
    }
    Ok(())
}

#[test]
fn rejects_selected_mesh_without_composite_binding() -> Result<(), String> {
    let root = temp_root("missing-binding");
    let (skeleton_path, composite_path, mesh_path) =
        write_fixture(&root, "MissingShape")?;

    let result = decoded_rigid_prop_source::load_selected_rigid_prop_asset(
        "selected",
        &skeleton_path,
        &[mesh_path.as_path()],
        &composite_path,
    );
    remove_fixture(&root)?;

    match result {
        Err(SkinSourceError::Prop(reason))
            if reason.contains("has no composite binding") =>
        {
            Ok(())
        },
        other => Err(format!("missing binding was accepted: {other:?}")),
    }
}

#[test]
fn rejects_duplicate_selected_prop_binding() -> Result<(), String> {
    let root = temp_root("duplicate-binding");
    let (skeleton_path, composite_path, mesh_path) =
        write_fixture(&root, "BodyShape")?;
    let mut composite =
        serde_json::from_str::<serde_json::Value>(composite_json())
            .map_err(|error| error.to_string())?;
    let object = composite
        .as_object_mut()
        .ok_or_else(|| "composite fixture is not an object".to_owned())?;
    let count = object
        .get_mut("num_props")
        .ok_or_else(|| "composite fixture has no prop count".to_owned())?;
    *count = serde_json::json!(3);
    object
        .get_mut("props")
        .and_then(serde_json::Value::as_array_mut)
        .ok_or_else(|| "composite fixture has no prop array".to_owned())?
        .push(serde_json::json!({
            "kind": "prop",
            "name": "BodyShape",
            "is_translucent": 0,
            "skeleton_joint_id": 2,
            "sort_order": 0,
        }));
    fs::write(
        &composite_path,
        serde_json::to_vec(&composite).map_err(|error| error.to_string())?,
    )
    .map_err(|error| error.to_string())?;

    let result = decoded_rigid_prop_source::load_selected_rigid_prop_asset(
        "selected",
        &skeleton_path,
        &[mesh_path.as_path()],
        &composite_path,
    );
    remove_fixture(&root)?;

    match result {
        Err(SkinSourceError::Prop(reason))
            if reason.contains("duplicate selected rigid prop binding") =>
        {
            Ok(())
        },
        other => Err(format!("duplicate prop binding was accepted: {other:?}")),
    }
}

#[test]
fn allows_duplicate_unselected_prop_binding() -> Result<(), String> {
    let root = temp_root("duplicate-unselected-binding");
    let (skeleton_path, composite_path, mesh_path) =
        write_fixture(&root, "BodyShape")?;
    let mut composite =
        serde_json::from_str::<serde_json::Value>(composite_json())
            .map_err(|error| error.to_string())?;
    let object = composite
        .as_object_mut()
        .ok_or_else(|| "composite fixture is not an object".to_owned())?;
    let count = object
        .get_mut("num_props")
        .ok_or_else(|| "composite fixture has no prop count".to_owned())?;
    *count = serde_json::json!(3);
    object
        .get_mut("props")
        .and_then(serde_json::Value::as_array_mut)
        .ok_or_else(|| "composite fixture has no prop array".to_owned())?
        .push(serde_json::json!({
            "kind": "prop",
            "name": "GlowShape",
            "is_translucent": 0,
            "skeleton_joint_id": 2,
            "sort_order": 0,
        }));
    fs::write(
        &composite_path,
        serde_json::to_vec(&composite).map_err(|error| error.to_string())?,
    )
    .map_err(|error| error.to_string())?;

    let result = decoded_rigid_prop_source::load_selected_rigid_prop_asset(
        "selected",
        &skeleton_path,
        &[mesh_path.as_path()],
        &composite_path,
    );
    remove_fixture(&root)?;
    let asset = result.map_err(|error| {
        format!("unselected duplicate binding was rejected: {error:?}")
    })?;
    let part = asset
        .parts
        .first()
        .ok_or_else(|| "selected binding produced no part".to_owned())?;
    if asset.parts.len() != 1
        || part.mesh.name != "BodyShape__transparent-source"
    {
        return Err(
            "selected binding changed by unrelated duplicate".to_owned()
        );
    }
    Ok(())
}

#[test]
fn bakes_normals_with_inverse_transpose_under_nonuniform_scale()
-> Result<(), String> {
    let root = temp_root("normal-scale");
    let (skeleton_path, composite_path, mesh_path) =
        write_fixture(&root, "BodyShape")?;
    fs::write(&skeleton_path, scaled_skeleton_json())
        .and_then(|()| {
            fs::write(&mesh_path, mesh_with_normals_json("BodyShape"))
        })
        .map_err(|error| error.to_string())?;

    let result = decoded_rigid_prop_source::load_selected_rigid_prop_asset(
        "scaled",
        &skeleton_path,
        &[mesh_path.as_path()],
        &composite_path,
    );
    remove_fixture(&root)?;
    let asset =
        result.map_err(|error| format!("scaled prop failed: {error:?}"))?;
    let normal = asset
        .parts
        .first()
        .and_then(|part| part.mesh.groups.first())
        .and_then(|group| group.normals.first())
        .ok_or("scaled rigid prop has no authored normal")?;
    let expected = [0.447_213_6_f32, 0.894_427_2_f32, 0.];
    if normal
        .iter()
        .zip(expected)
        .any(|(actual, expected)| (*actual - expected).abs() > 1e-5)
    {
        return Err(format!(
            "nonuniform-scale normal used position basis: {normal:?}"
        ));
    }
    Ok(())
}

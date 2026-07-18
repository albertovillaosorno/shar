// File:
//   - decoded_rigid_prop_source.rs
// Path:
//   - src/fbx/tests/decoded_rigid_prop_source.rs
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
//   - Regression coverage for selected rigid-prop assembly and rig pruning.
// - Must-Not:
//   - Read private game assets or mirror implementation logic in assertions.
// - Allows:
//   - Synthetic decoded skeleton, composite, and mesh fixtures.
// - Split-When:
//   - Another selected-prop behavior needs an independent fixture family.
// - Merge-When:
//   - Decoded skin-source tests adopt the selected-prop public contract.
// - Summary:
//   - Proves unselected composite props and skeleton branches stay excluded.
// - Description:
//   - Builds one synthetic rigid body while rejecting an unbound selection.
// - Usage:
//   - Run through the fbx crate test target.
// - Defaults:
//   - Temporary fixture roots are process-specific and removed after each test.
//
// ADRs:
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
// - docs/adr/fbx/export/fbx-output-contract-boundary.md
//
// Large file:
//   - true
//   - Reason: Two behavioral regressions share one compact decoded fixture
//   - family so selection, pruning, and rejection evidence remain auditable.
//

//! Regression coverage for selected rigid-prop assembly.
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
    std::env::temp_dir().join(
        format!(
            "fbx-rigid-prop-{label}-{}",
            std::process::id()
        ),
    )
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

fn write_fixture(
    root: &Path,
    mesh_name: &str,
) -> Result<
    (
        PathBuf,
        PathBuf,
        PathBuf,
    ),
    String,
> {
    let skeleton_path = root.join("skeleton.json");
    let composite_path = root.join("composite.json");
    let mesh_path = root.join(format!("{mesh_name}.json"));
    fs::create_dir_all(root)
        .and_then(
            |()| {
                fs::write(
                    &skeleton_path,
                    skeleton_json(),
                )
            },
        )
        .and_then(
            |()| {
                fs::write(
                    &composite_path,
                    composite_json(),
                )
            },
        )
        .and_then(
            |()| {
                fs::write(
                    &mesh_path,
                    mesh_json(mesh_name),
                )
            },
        )
        .map_err(|error| error.to_string())?;
    Ok(
        (
            skeleton_path,
            composite_path,
            mesh_path,
        ),
    )
}

fn remove_fixture(root: &Path) -> Result<(), String> {
    fs::remove_dir_all(root).map_err(|error| error.to_string())
}

#[test]
fn loads_selected_prop_and_prunes_unselected_branches() -> Result<(), String> {
    let root = temp_root("selection");
    let (skeleton_path, composite_path, mesh_path) = write_fixture(
        &root,
        "BodyShape",
    )?;

    let result = decoded_rigid_prop_source::load_selected_rigid_prop_asset(
        "selected",
        &skeleton_path,
        &[mesh_path.as_path()],
        &composite_path,
    );
    remove_fixture(&root)?;
    let asset =
        result.map_err(|error| format!("selection failed: {error:?}"))?;

    let bone_ids = asset
        .bones
        .iter()
        .map(
            |bone| {
                bone.id
                    .as_str()
            },
        )
        .collect::<Vec<_>>();
    if bone_ids
        != [
            "root", "body",
        ]
    {
        return Err(format!("unexpected retained bones: {bone_ids:?}"));
    }
    let Some(part) = asset
        .parts
        .first()
    else {
        return Err("selected rigid prop produced no part".to_owned());
    };
    if asset
        .parts
        .len()
        != 1
        || part
            .mesh
            .name
            != "BodyShape__transparent-source"
    {
        return Err(
            "selected rigid prop did not preserve one body mesh".to_owned(),
        );
    }
    let positions = &part
        .mesh
        .groups[0]
        .positions;
    if positions
        != &[
            [
                2.0, 3.0, 4.0,
            ],
            [
                3.0, 3.0, 4.0,
            ],
            [
                2.0, 4.0, 4.0,
            ],
        ]
    {
        return Err(
            format!(
                "selected rigid prop did not bake its authored rest \
                 transform: {positions:?}"
            ),
        );
    }
    if part
        .group_influences
        .iter()
        .flatten()
        .any(
            |influence| {
                influence.bone_id != "body"
                    || (influence.weight - 1.0).abs() > f32::EPSILON
            },
        )
    {
        return Err(
            "selected rigid prop was not fully bound to body".to_owned(),
        );
    }
    Ok(())
}

#[test]
fn rejects_selected_mesh_without_composite_binding() -> Result<(), String> {
    let root = temp_root("missing-binding");
    let (skeleton_path, composite_path, mesh_path) = write_fixture(
        &root,
        "MissingShape",
    )?;

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
        }
        other => Err(format!("missing binding was accepted: {other:?}")),
    }
}

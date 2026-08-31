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
//   - World inventory unit tests.
// - Must-Not:
//   - Own production behavior or broaden the tested API surface.
// - Allows:
//   - Private test fixtures and assertions for the owning source module.
// - Split-When:
//   - Split when an independent fixture family gains separate ownership.
// - Merge-When:
//   - Merge when another test module owns the identical evidence.
// - Summary:
//   - World inventory unit tests.
// - Description:
//   - Preserves unit-test access through a test-only path module.
// - Usage:
//   - Included only by the owning source module under cfg(test).
// - Defaults:
//   - Test setup and assertions fail explicitly.
//

//! World inventory unit tests.

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use super::{
    LedgerRow, decoded_mesh_names, deferred_controller_binding,
    deferred_render_bindings, source_ordered_mesh_ids,
};
use crate::adapters::driven::local::prop_catalog::inventory_common::{
    CompositeEvidence, CompositePropEvidence,
};

static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);

fn fixture_root(label: &str) -> Result<PathBuf, String> {
    let sequence = NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed);
    let root = std::env::temp_dir().join(format!(
        "shar-world-inventory-map-{label}-{}-{sequence}",
        std::process::id()
    ));
    for family in ["mesh", "quad_group", "frame_controller", "animation"] {
        fs::create_dir_all(root.join("components").join(family))
            .map_err(|error| error.to_string())?;
    }
    Ok(root)
}

#[test]
fn decoded_mesh_names_reject_duplicate_source_identity() -> Result<(), String> {
    let root = fixture_root("duplicate-name")?;
    fs::write(root.join("components/mesh/001.json"), r#"{"name":"shared"}"#)
        .map_err(|error| error.to_string())?;
    fs::write(root.join("components/mesh/002.json"), r#"{"name":"shared"}"#)
        .map_err(|error| error.to_string())?;
    // jig-ignore-next-line: literal
    let result = decoded_mesh_names(&root, &["001".to_owned(), "002".to_owned()]);
    drop(fs::remove_dir_all(&root));
    let Err(error) = result else {
        return Err("duplicate decoded mesh identity was accepted".to_owned());
    };
    if !error
        .to_string()
        .contains("world prop repeats mesh identity shared")
    {
        // jig-ignore-next-line: literal
        return Err(format!("unexpected duplicate mesh identity error: {error}"));
    }
    Ok(())
}

#[test]
fn source_mesh_ids_follow_component_ordinals() -> Result<(), String> {
    let rows = vec![
        LedgerRow {
            ordinal: 20,
            depth: 2,
            container_ordinal: 1,
            name: "lexically-first".to_owned(),
            path: "mesh/a.json".to_owned(),
            kind: "mesh".to_owned(),
        },
        LedgerRow {
            ordinal: 10,
            depth: 2,
            container_ordinal: 1,
            name: "source-first".to_owned(),
            path: "mesh/z.json".to_owned(),
            kind: "mesh".to_owned(),
        },
    ];
    let actual = source_ordered_mesh_ids(&rows)
        .map_err(|error| error.to_string())?;
    assert_eq!(actual, ["z", "a"]);
    Ok(())
}

#[test]
fn deferred_bindings_resolve_unique_package_quad_group_occurrence()
-> Result<(), String> {
    let composite = CompositeEvidence {
        member_id: "composite".to_owned(),
        name: "owner".to_owned(),
        skeleton_name: "rig".to_owned(),
        prop_names: vec![
            "body".to_owned(),
            "beam".to_owned(),
            "logical-only".to_owned(),
        ],
        prop_bindings: vec![
            CompositePropEvidence {
                name: "body".to_owned(),
                skeleton_joint_id: 0,
                is_translucent: false,
            },
            CompositePropEvidence {
                name: "beam".to_owned(),
                skeleton_joint_id: 3,
                is_translucent: true,
            },
            CompositePropEvidence {
                name: "logical-only".to_owned(),
                skeleton_joint_id: 4,
                is_translucent: false,
            },
        ],
    };
    let rows = Vec::new();
    let package_quad_groups = vec![LedgerRow {
        ordinal: 22,
        depth: 1,
        container_ordinal: 22,
        name: "beam\x00".to_owned(),
        path: "quad_group/beam__ordinal_22.json".to_owned(),
        kind: "quad_group".to_owned(),
    }];
    let meshes = BTreeMap::from([(
        "body".to_owned(),
        "body__ordinal_10".to_owned(),
    )]);

    let root = fixture_root("deferred-package-quad")?;
    fs::write(
        root.join("components/quad_group/beam__ordinal_22.json"),
        concat!(
            r#"{"schema":"quad_group","version":0,"name":"beam","#,
            r#""shader":"glow_m","z_test":1,"z_write":0,"fog":0,"#,
            r#""num_quads":1,"quads":[{"name":"beam-child","#,
            r#""version":2,"billboard_mode":"YAX","#,
            r#""translation":[2.5,-3,4],"colour":305419896,"#,
            r#""uvs":[[0.1,0.2],[0.9,0.2],[0.9,0.8],[0.1,0.8]],"#,
            r#""width":2.25,"height":4.5,"distance":-0.35,"#,
            r#""uv_offset":[0.25,-0.5],"rotation_wxyz":[1,0,0,0],"#,
            r#""cutoff_mode":"DBL","uv_offset_range":[0.5,0.75],"#,
            r#""source_range":1.25,"edge_range":0.625,"#,
            r#""perspective":false}]}"#,
        ),
    )
    .map_err(|error| error.to_string())?;
    let actual = deferred_render_bindings(
        &root,
        &rows,
        &package_quad_groups,
        &composite,
        &meshes,
    )
        .map_err(|error| error.to_string())?;
    drop(fs::remove_dir_all(&root));
    if actual.len() != 2 {
        return Err(format!("unexpected deferred binding count: {actual:?}"));
    }
    let beam = actual
        .first()
        .ok_or_else(|| "resolved deferred binding is missing".to_owned())?;
    if beam.composite_prop_index != 1
        || beam.source_identity != "beam"
        || beam.skeleton_joint_id != 3
        || !beam.is_translucent
        || beam.component_kind.as_deref() != Some("quad_group")
        || beam.component_member_id.as_deref() != Some("beam__ordinal_22")
        || beam.source_ordinal != Some(22)
        || beam.controller.is_some()
    {
        return Err(format!("resolved deferred binding changed: {beam:?}"));
    }
    let billboard = beam
        .billboard
        .as_ref()
        .ok_or_else(|| "resolved billboard evidence is missing".to_owned())?;
    let quad = billboard
        .quads
        .first()
        .ok_or_else(|| "resolved billboard child is missing".to_owned())?;
    if billboard.version != 0
        || billboard.shader_identity != "glow_m"
        || billboard.z_test != 1
        || billboard.z_write != 0
        || billboard.fog != 0
        || billboard.quads.len() != 1
        || quad.identity != "beam-child"
        || quad.version != 2
        || quad.billboard_mode != "YAX"
        || quad.translation_bits != [2.5_f32, -3., 4.].map(f32::to_bits)
        || quad.colour != 305_419_896
        || quad.width_bits != 2.25_f32.to_bits()
        || quad.height_bits != 4.5_f32.to_bits()
        || quad.distance_bits != (-0.35_f32).to_bits()
        || quad.cutoff_mode != "DBL"
        || quad.source_range_bits != 1.25_f32.to_bits()
        || quad.edge_range_bits != 0.625_f32.to_bits()
        || quad.perspective
    {
        return Err(format!("billboard evidence changed: {billboard:?}"));
    }
    let logical = actual
        .get(1)
        .ok_or_else(|| "logical deferred binding is missing".to_owned())?;
    if logical.composite_prop_index != 2
        || logical.source_identity != "logical-only"
        || logical.skeleton_joint_id != 4
        || logical.is_translucent
        || logical.component_kind.is_some()
        || logical.component_member_id.is_some()
        || logical.source_ordinal.is_some()
        || logical.billboard.is_some()
    {
        return Err(format!("logical deferred binding changed: {logical:?}"));
    }
    Ok(())
}

#[test]
fn deferred_controller_retains_exact_animation_relationship()
-> Result<(), String> {
    let root = fixture_root("controller-animation")?;
    fs::write(
        root.join("components/frame_controller/BQG_beam.json"),
        concat!(
            r#"{"schema":"frame_controller","name":"BQG_beam","#,
            r#""version":0,"type":"BQG","frame_offset":1.25,"#,
            r#""hierarchy_name":"beam","animation_name":"BQG_beam"}"#,
        ),
    )
    .map_err(|error| error.to_string())?;
    fs::write(
        root.join("components/animation/animation_0001.json"),
        concat!(
            r#"{"schema":"animation","name":"BQG_beam","#,
            r#""version":0,"type":"BQG_"}"#,
        ),
    )
    .map_err(|error| error.to_string())?;
    let rows = vec![
        LedgerRow {
            ordinal: 43,
            depth: 2,
            container_ordinal: 1,
            name: "BQG_beam".to_owned(),
            path: "frame_controller/BQG_beam.json".to_owned(),
            kind: "frame_controller".to_owned(),
        },
        LedgerRow {
            ordinal: 41,
            depth: 2,
            container_ordinal: 1,
            name: "BQG_beam".to_owned(),
            path: "animation/animation_0001.json".to_owned(),
            kind: "animation".to_owned(),
        },
    ];
    let result = deferred_controller_binding(&root, &rows, "beam")
        .map_err(|error| error.to_string());
    drop(fs::remove_dir_all(&root));
    let binding = result?
        .ok_or_else(|| "controller relationship was not retained".to_owned())?;
    if binding.controller_identity != "BQG_beam"
        || binding.controller_kind != "frame_controller"
        || binding.controller_member_id != "BQG_beam"
        || binding.controller_source_ordinal != 43
        || binding.controller_version != 0
        || binding.controller_type != "BQG"
        || f32::from_bits(binding.frame_offset_bits) != 1.25
        || binding.animation_identity != "BQG_beam"
        || binding.animation_member_id.as_deref() != Some("animation_0001")
        || binding.animation_source_ordinal != Some(41)
        || binding.animation_version != Some(0)
        || binding.animation_type.as_deref() != Some("BQG_")
    {
        return Err(format!("controller relationship changed: {binding:?}"));
    }
    Ok(())
}

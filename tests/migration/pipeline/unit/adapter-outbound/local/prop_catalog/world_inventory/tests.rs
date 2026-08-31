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
use crate::adapters::driven::local::prop_catalog::texture_authority;

static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);

fn fixture_root(label: &str) -> Result<PathBuf, String> {
    let sequence = NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed);
    let root = std::env::temp_dir().join(format!(
        "shar-world-inventory-map-{label}-{}-{sequence}",
        std::process::id()
    ));
    for family in [
        "mesh",
        "quad_group",
        "shader",
        "frame_controller",
        "animation",
    ] {
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
    let package_relationships = vec![
        LedgerRow {
            ordinal: 7,
            depth: 1,
            container_ordinal: 7,
            name: "glow_m\x00".to_owned(),
            path: "shader/glow_m__ordinal_7.json".to_owned(),
            kind: "shader".to_owned(),
        },
        LedgerRow {
            ordinal: 22,
            depth: 1,
            container_ordinal: 22,
            name: "beam\x00".to_owned(),
            path: "quad_group/beam__ordinal_22.json".to_owned(),
            kind: "quad_group".to_owned(),
        },
        LedgerRow {
            ordinal: 3,
            depth: 1,
            container_ordinal: 3,
            name: "glow_m\x00".to_owned(),
            path: "shader/glow_m__ordinal_3.json".to_owned(),
            kind: "shader".to_owned(),
        },
    ];
    let meshes = BTreeMap::from([(
        "body".to_owned(),
        "body__ordinal_10".to_owned(),
    )]);

    let root = fixture_root("deferred-package-quad")?;
    fs::write(
        root.join("components/shader/glow_m__ordinal_7.json"),
        concat!(
            r#"{"schema":"shader","name":"glow_m\u0000","version":0,"#,
            r#""pddi_shader_name":"simple\u0000","has_translucency":1,"#,
            r#""vertex_needs":33,"vertex_mask":7,"num_params":2,"#,
            r#""params":[{"kind":"texture","param":"TEX","#,
            r#""value":"glow.bmp\u0000"},{"kind":"int","#,
            r#""param":"BLMD","value":3}]}"#,
        ),
    )
    .map_err(|error| error.to_string())?;
    fs::write(
        root.join("components/shader/glow_m__ordinal_3.json"),
        concat!(
            r#"{"schema":"shader","name":"glow_m\u0000","version":0,"#,
            r#""pddi_shader_name":"simple\u0000","has_translucency":1,"#,
            r#""vertex_needs":33,"vertex_mask":3,"num_params":2,"#,
            r#""params":[{"kind":"texture","param":"TEX","#,
            r#""value":"glow.bmp\u0000"},{"kind":"int","#,
            r#""param":"BLMD","value":2}]}"#,
        ),
    )
    .map_err(|error| error.to_string())?;
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
    let authority =
        texture_authority::SharedTextureAuthority::from_occurrences_for_tests(&[
        (
            "glow.bmp",
            "level-three-terrain",
            "terrain-world/level-03/terrain-mesh",
            "glow",
            182,
            "glow.png",
            "one",
        ),
        (
            "glow.bmp",
            "level-three-terrain",
            "terrain-world/level-03/terrain-mesh",
            "glow__ordinal_10591",
            10_591,
            "glow__ordinal_10591.png",
            "two",
        ),
    ]);
    let actual = deferred_render_bindings(
        &root,
        &rows,
        &package_relationships,
        &composite,
        &meshes,
        &authority,
        "terrain-world/level-03/regions/l3r1",
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
    let shader_ordinals = billboard
        .shader_occurrences
        .iter()
        .map(|shader| shader.source_ordinal)
        .collect::<Vec<_>>();
    let shader_members = billboard
        .shader_occurrences
        .iter()
        .map(|shader| shader.member_id.as_str())
        .collect::<Vec<_>>();
    let shader_textures = billboard
        .shader_occurrences
        .iter()
        .map(|shader| shader.texture_reference.as_deref())
        .collect::<Vec<_>>();
    let texture_reference = billboard
        .texture_references
        .first()
        .ok_or_else(|| "deferred texture reference is missing".to_owned())?;
    let texture_members = texture_reference
        .occurrences
        .iter()
        .map(|occurrence| occurrence.member_id.as_str())
        .collect::<Vec<_>>();
    let texture_ordinals = texture_reference
        .occurrences
        .iter()
        .map(|occurrence| occurrence.source_ordinal)
        .collect::<Vec<_>>();
    let texture_digests = texture_reference
        .occurrences
        .iter()
        .map(|occurrence| occurrence.sha256.as_str())
        .collect::<Vec<_>>();
    let blend_modes = billboard
        .shader_occurrences
        .iter()
        .map(|shader| {
            shader
                .params
                .get(1)
                .and_then(|parameter| parameter.value.as_u64())
        })
        .collect::<Vec<_>>();
    if billboard.version != 0
        || billboard.shader_identity != "glow_m"
        || shader_ordinals != [3, 7]
        || shader_members != ["glow_m__ordinal_3", "glow_m__ordinal_7"]
        || shader_textures != [Some("glow.bmp"), Some("glow.bmp")]
        || billboard.texture_references.len() != 1
        || texture_reference.identity != "glow.bmp"
        || texture_members != ["glow", "glow__ordinal_10591"]
        || texture_ordinals != [182, 10_591]
        || texture_digests != ["one", "two"]
        || blend_modes != [Some(2), Some(3)]
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

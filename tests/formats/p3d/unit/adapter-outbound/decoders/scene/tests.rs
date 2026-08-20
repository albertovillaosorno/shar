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
//   - Tests unit tests.
// - Must-Not:
//   - Own production behavior or broaden the tested API surface.
// - Allows:
//   - Private test fixtures and assertions for the owning source module.
// - Split-When:
//   - Split when an independent fixture family gains separate ownership.
// - Merge-When:
//   - Merge when another test module owns the identical evidence.
// - Summary:
//   - Tests unit tests.
// - Description:
//   - Preserves unit-test access through a test-only path module.
// - Usage:
//   - Included only by the owning source module under cfg(test).
// - Defaults:
//   - Test setup and assertions fail explicitly.
//

//! Tests unit tests.

use super::*;

/// Joins fixture field fragments so tests mirror contiguous chunk headers.
fn fields(parts: Vec<Vec<u8>>) -> Vec<u8> {
    let mut out = Vec::new();
    for part in parts {
        out.extend(part);
    }
    out
}

/// Converts optional fixture construction into a descriptive test error.
fn require<T>(value: Option<T>, context: &str) -> Result<T, String> {
    value.ok_or_else(|| String::from(context))
}

/// Checks useful JSON fields without panicking inside `Result` tests.
fn require_json(json: &str, needle: &str, context: &str) -> Result<(), String> {
    if json.contains(needle) {
        Ok(())
    } else {
        Err(String::from(context))
    }
}

/// Builds a synthetic chunk for count-checked decoder tests.
fn chunk(id: u32, fields: Vec<u8>, children: Vec<Vec<u8>>) -> Option<Vec<u8>> {
    let header_size = 12_usize.checked_add(fields.len())?;
    let child_size = children
        .iter()
        .map(Vec::len)
        .try_fold(0_usize, usize::checked_add)?;
    let total_size = header_size.checked_add(child_size)?;
    let header_size_u32 = u32::try_from(header_size).ok()?;
    let total_size_u32 = u32::try_from(total_size).ok()?;
    let mut out = Vec::new();
    out.extend_from_slice(&id.to_le_bytes());
    out.extend_from_slice(&header_size_u32.to_le_bytes());
    out.extend_from_slice(&total_size_u32.to_le_bytes());
    out.extend(fields);
    for child in children {
        out.extend(child);
    }
    Some(out)
}

/// Builds a Pure3D-style test string payload.
fn pstring(value: &str) -> Option<Vec<u8>> {
    let length = u8::try_from(value.len()).ok()?;
    let mut out = Vec::new();
    out.push(length);
    out.extend_from_slice(value.as_bytes());
    Some(out)
}

/// Builds a little-endian integer test field.
fn u32_field(value: u32) -> Vec<u8> {
    value.to_le_bytes().to_vec()
}

/// Builds a little-endian float test field.
fn f32_field(value: f32) -> Vec<u8> {
    value.to_le_bytes().to_vec()
}

/// Builds a stable identity matrix fixture for transform tests.
fn identity_matrix() -> Vec<u8> {
    let mut out = Vec::new();
    for index in 0_usize..16_usize {
        let value = if matches!(index, 0 | 5 | 10 | 15) {
            1f32
        } else {
            0f32
        };
        out.extend_from_slice(&f32_field(value));
    }
    out
}

/// Builds a reusable scenegraph fixture with configurable child count.
fn scenegraph_fixture(child_count: u32) -> Option<Vec<u8>> {
    let sort = chunk(SCENE_SORT_ORDER, f32_field(3.5_f32), Vec::new())?;
    let drawable_fields = fields(vec![
        pstring("body_node")?,
        pstring("body_mesh")?,
        u32_field(0),
    ]);
    let drawable = chunk(SCENE_DRAWABLE, drawable_fields, vec![sort])?;
    let transform_fields = fields(vec![
        pstring("body_xform")?,
        u32_field(child_count),
        identity_matrix(),
    ]);
    let transform = chunk(SCENE_TRANSFORM, transform_fields, vec![drawable])?;
    let root = chunk(SCENE_ROOT, Vec::new(), vec![transform])?;
    let graph_fields = fields(vec![pstring("entity_graph")?, u32_field(0)]);
    chunk(SCENEGRAPH, graph_fields, vec![root])
}

/// Builds a composite skin-list fixture with one sorted skin binding.
fn composite_skin_list_fixture() -> Result<Vec<u8>, String> {
    let sort = require(
        chunk(COMPOSITE_SORT_ORDER, f32_field(1.25_f32), Vec::new()),
        "sort-order fixture should build",
    )?;
    let skin_fields = fields(vec![
        require(pstring("hero_skin"), "skin name should encode")?,
        u32_field(1),
    ]);
    let skin = require(
        chunk(COMPOSITE_SKIN, skin_fields, vec![sort]),
        "skin fixture should build",
    )?;
    require(
        chunk(COMPOSITE_SKIN_LIST, u32_field(1), vec![skin]),
        "skin-list fixture should build",
    )
}

/// Builds a composite prop-list fixture with one joint-bound prop.
fn composite_prop_list_fixture() -> Result<Vec<u8>, String> {
    let prop_fields = fields(vec![
        require(pstring("hat_mesh"), "prop name should encode")?,
        u32_field(0),
        u32_field(7),
    ]);
    let prop = require(
        chunk(COMPOSITE_PROP, prop_fields, Vec::new()),
        "prop fixture should build",
    )?;
    require(
        chunk(COMPOSITE_PROP_LIST, u32_field(1), vec![prop]),
        "prop-list fixture should build",
    )
}

/// Builds an empty effect-list fixture because zero-count lists must
/// survive.
fn composite_effect_list_fixture() -> Result<Vec<u8>, String> {
    require(
        chunk(COMPOSITE_EFFECT_LIST, u32_field(0), Vec::new()),
        "effect-list fixture should build",
    )
}

/// Builds a composite drawable fixture with skin, prop, and effect lists.
fn composite_drawable_fixture() -> Result<Vec<u8>, String> {
    let composite_fields = fields(vec![
        require(pstring("hero_comp"), "composite name should encode")?,
        require(pstring("hero_skel"), "skeleton name should encode")?,
    ]);
    require(
        chunk(0x0000_4512, composite_fields, vec![
            composite_skin_list_fixture()?,
            composite_prop_list_fixture()?,
            composite_effect_list_fixture()?,
        ]),
        "composite fixture should build",
    )
}

#[test]
fn scenegraph_decodes_transform_and_drawable_refs() -> Result<(), String> {
    let fixture =
        require(scenegraph_fixture(1), "scenegraph fixture should build")?;
    let json = require(
        scenegraph_json(&fixture),
        "scenegraph fixture should decode",
    )?;

    require_json(
        &json,
        "\"schema\":\"scenegraph\"",
        "schema should be emitted",
    )?;
    require_json(
        &json,
        "\"kind\":\"transform\"",
        "transform should be emitted",
    )?;
    require_json(
        &json,
        "\"drawable_name\":\"body_mesh\"",
        "drawable ref should be emitted",
    )?;
    require_json(&json, "\"sort_order\":3.5", "sort order should be emitted")?;
    Ok(())
}

#[test]
fn scenegraph_fails_closed_on_child_count_mismatch() -> Result<(), String> {
    let fixture =
        require(scenegraph_fixture(2), "mismatch fixture should build")?;
    if scenegraph_json(&fixture).is_none() {
        Ok(())
    } else {
        Err(String::from("child-count mismatch should fail closed"))
    }
}

#[test]
fn composite_drawable_decodes_binding_lists() -> Result<(), String> {
    let comp = composite_drawable_fixture()?;
    let json = require(
        composite_drawable_json(&comp),
        "composite fixture should decode",
    )?;

    require_json(
        &json,
        "\"skeleton_name\":\"hero_skel\"",
        "skeleton binding should be emitted",
    )?;
    require_json(&json, "\"kind\":\"skin\"", "skin binding should be emitted")?;
    require_json(
        &json,
        "\"skeleton_joint_id\":7",
        "joint id should be emitted",
    )?;
    Ok(())
}

#[test]
fn entity_dsg_escapes_trailing_nul_name_padding() -> Result<(), String> {
    let mut padded_name = String::from("groupShape135_000");
    padded_name.push(char::from(0));
    let mesh_fields = fields(vec![
        require(pstring(&padded_name), "padded mesh name should encode")?,
        u32_field(0),
    ]);
    let mesh = require(
        chunk(MESH, mesh_fields, Vec::new()),
        "mesh fixture should build",
    )?;
    let entity_fields = fields(vec![
        require(pstring(&padded_name), "padded entity name should encode")?,
        u32_field(0),
        u32_field(0),
    ]);
    let entity = require(
        chunk(0x03f0_0008, entity_fields, vec![mesh]),
        "entity fixture should build",
    )?;
    let json =
        require(entity_dsg_json(&entity), "entity fixture should decode")?;
    serde_json::from_str::<serde_json::Value>(&json)
        .map(|_value| ())
        .map_err(|error| {
            format!("entity JSON must escape NUL padding: {error}")
        })
}

#[test]
fn insta_entity_decodes_render_refs_and_instance_scenegraphs()
-> Result<(), String> {
    let mesh_fields = fields(vec![
        require(pstring("crate_mesh"), "mesh name should encode")?,
        u32_field(0),
    ]);
    let mesh = require(
        chunk(MESH, mesh_fields, Vec::new()),
        "mesh fixture should build",
    )?;
    let instances = require(
        chunk(
            INSTANCES,
            require(pstring("crate_instances"), "instance name should encode")?,
            vec![require(
                scenegraph_fixture(1),
                "nested scenegraph should build",
            )?],
        ),
        "instances fixture should build",
    )?;
    let entity_fields = fields(vec![
        require(pstring("crate_entity"), "entity name should encode")?,
        u32_field(0),
        u32_field(0),
    ]);
    let entity = require(
        chunk(0x03f0_0009, entity_fields, vec![mesh, instances]),
        "entity fixture should build",
    )?;
    let json = require(
        insta_entity_dsg_json(&entity),
        "insta-entity fixture should decode",
    )?;

    require_json(
        &json,
        "\"schema\":\"srr_insta_entity_dsg\"",
        "schema should be emitted",
    )?;
    require_json(
        &json,
        "\"name\":\"crate_mesh\"",
        "mesh ref should be emitted",
    )?;
    require_json(
        &json,
        "\"scenegraphs\"",
        "instance scenegraph should be emitted",
    )?;
    Ok(())
}

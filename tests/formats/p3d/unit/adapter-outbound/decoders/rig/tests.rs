// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
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

/// Builds a synthetic chunk for rig decoder tests.
fn chunk(id: u32, fields: Vec<u8>, children: Vec<Vec<u8>>) -> Option<Vec<u8>> {
    let children_len = children
        .iter()
        .try_fold(0_usize, |acc, child| acc.checked_add(child.len()))?;
    let header_len = fields.len().checked_add(12)?;
    let total_len = header_len.checked_add(children_len)?;
    let mut out = Vec::with_capacity(total_len);
    for word in [
        id,
        u32::try_from(header_len).ok()?,
        u32::try_from(total_len).ok()?,
    ] {
        out.extend_from_slice(&word.to_le_bytes());
    }
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

/// Builds a little-endian integer field.
fn u32_field(value: u32) -> Vec<u8> {
    value.to_le_bytes().to_vec()
}

/// Builds a little-endian short field.
fn u16_field(value: u16) -> Vec<u8> {
    value.to_le_bytes().to_vec()
}

/// Builds a little-endian float field.
fn f32_field(value: f32) -> Vec<u8> {
    value.to_le_bytes().to_vec()
}

/// Builds a fixed-width fourcc field.
fn fourcc(value: &str) -> Option<Vec<u8>> {
    let mut out = vec![0_u8; 4];
    for (index, byte) in value.as_bytes().iter().take(4).enumerate() {
        *out.get_mut(index)? = *byte;
    }
    Some(out)
}

/// Joins field fragments in schema order.
fn fields(parts: Vec<Vec<u8>>) -> Vec<u8> {
    let mut out = Vec::new();
    for part in parts {
        out.extend(part);
    }
    out
}

/// Builds an identity matrix fixture.
fn identity_matrix() -> Vec<u8> {
    (0_usize..16_usize)
        .flat_map(|index| {
            let value = match index {
                0 | 5 | 10 | 15 => 1f32,
                _ => 0f32,
            };
            value.to_le_bytes()
        })
        .collect()
}

/// Converts optional fixtures into descriptive test errors.
fn require<T>(value: Option<T>, context: &str) -> Result<T, String> {
    value.ok_or_else(|| String::from(context))
}

/// Checks useful JSON output without panic-based test construction.
fn require_json(json: &str, needle: &str, context: &str) -> Result<(), String> {
    if json.contains(needle) {
        Ok(())
    } else {
        Err(String::from(context))
    }
}

/// Builds a skeleton fixture with one joint and rest pose.
fn skeleton_fixture() -> Option<Vec<u8>> {
    let joint = chunk(
        SKELETON_JOINT,
        fields(vec![
            pstring("root")?,
            u32_field(u32::MAX),
            u32_field(0),
            u32_field(7),
            u32_field(1),
            u32_field(2),
            u32_field(3),
            identity_matrix(),
        ]),
        Vec::new(),
    )?;
    chunk(
        SKELETON,
        fields(vec![pstring("skel")?, u32_field(0), u32_field(1)]),
        vec![joint],
    )
}

/// Builds an animation fixture with a float channel key list.
fn animation_fixture() -> Option<Vec<u8>> {
    animation_fixture_with_values(1., 2.5)
}

/// Builds an animation fixture with caller-selected float key values.
fn animation_fixture_with_values(first: f32, second: f32) -> Option<Vec<u8>> {
    let channel = chunk(
        CHANNEL_FLOAT1,
        fields(vec![
            u32_field(0),
            fourcc("TX  ")?,
            u32_field(2),
            u16_field(0),
            u16_field(10),
            f32_field(first),
            f32_field(second),
        ]),
        Vec::new(),
    )?;
    let group = chunk(
        ANIMATION_GROUP,
        fields(vec![
            u32_field(0),
            pstring("root")?,
            u32_field(9),
            u32_field(1),
        ]),
        vec![channel],
    )?;
    let group_list = chunk(
        ANIMATION_GROUP_LIST,
        fields(vec![u32_field(0), u32_field(1)]),
        vec![group],
    )?;
    chunk(
        ANIMATION,
        fields(vec![
            u32_field(0),
            pstring("walk")?,
            fourcc("PANM")?,
            f32_field(30.),
            f32_field(30.),
            u32_field(1),
        ]),
        vec![group_list],
    )
}

/// Builds a multi-controller fixture with one timing track.
fn multi_fixture() -> Option<Vec<u8>> {
    let track = chunk(
        MULTI_CONTROLLER_TRACK,
        fields(vec![
            pstring("walk")?,
            f32_field(0.),
            f32_field(10.),
            f32_field(1.),
        ]),
        Vec::new(),
    )?;
    chunk(
        MULTI_CONTROLLER,
        fields(vec![
            pstring("controller")?,
            u32_field(0),
            f32_field(10.),
            f32_field(30.),
            u32_field(1),
        ]),
        vec![track],
    )
}

/// Builds a vertex animation key fixture with vector offsets and indices.
fn vertex_fixture() -> Option<Vec<u8>> {
    let indices = chunk(
        VERTEX_INDEX_OFFSETS,
        fields(vec![u32_field(0), u32_field(1), u32_field(42)]),
        Vec::new(),
    )?;
    let vectors = chunk(
        VERTEX_VECTOR_OFFSETS,
        fields(vec![
            u32_field(0),
            u32_field(1),
            fourcc("POSN")?,
            f32_field(1.),
            f32_field(2.),
            f32_field(3.),
        ]),
        vec![indices],
    )?;
    chunk(
        VERTEX_KEY,
        fields(vec![u32_field(0), pstring("face")?]),
        vec![vectors],
    )
}

#[test]
/// Keeps `skeleton_decodes_joint_rest_pose` local because it shares the rig
/// binary-layout invariant.
fn skeleton_decodes_joint_rest_pose() -> Result<(), String> {
    let fixture = require(skeleton_fixture(), "skeleton fixture should build")?;
    let json =
        require(skeleton_json(&fixture), "skeleton fixture should decode")?;
    require_json(&json, "\"schema\":\"skeleton\"", "schema should be emitted")?;
    require_json(&json, "\"name\":\"root\"", "joint name should be emitted")?;
    require_json(
        &json,
        "\"parent\":4294967295",
        "parent index should be emitted",
    )?;
    require_json(&json, "\"rest_pose\":[1.0", "rest pose should be emitted")?;
    Ok(())
}

#[test]
/// Keeps `animation_decodes_group_channels_and_keys` local because it
/// shares the rig binary-layout invariant.
fn animation_decodes_group_channels_and_keys() -> Result<(), String> {
    let fixture =
        require(animation_fixture(), "animation fixture should build")?;
    let json =
        require(animation_json(&fixture), "animation fixture should decode")?;
    require_json(
        &json,
        "\"schema\":\"animation\"",
        "schema should be emitted",
    )?;
    require_json(&json, "\"groups\":[{", "group list should be emitted")?;
    require_json(
        &json,
        "\"kind\":\"float1\"",
        "channel kind should be emitted",
    )?;
    require_json(&json, "\"frames\":[0,10]", "key frames should be emitted")?;
    require_json(&json, "[1.0]", "key values should be emitted")?;
    Ok(())
}

#[test]
fn animation_renders_non_finite_keys_as_json_null() -> Result<(), String> {
    let fixture = require(
        animation_fixture_with_values(f32::NAN, f32::INFINITY),
        "non-finite animation fixture should build",
    )?;
    let json = require(
        animation_json(&fixture),
        "non-finite animation fixture should decode",
    )?;
    let _value =
        serde_json::from_str::<serde_json::Value>(&json).map_err(|error| {
            format!("non-finite animation keys must remain valid JSON: {error}")
        })?;
    require_json(
        &json,
        r#""values":[[null],[null]]"#,
        "non-finite keys should be represented as null",
    )
}

#[test]
/// Keeps `multi_controller_decodes_track_timings` local because it shares
/// the rig binary-layout invariant.
fn multi_controller_decodes_track_timings() -> Result<(), String> {
    let fixture =
        require(multi_fixture(), "multi-controller fixture should build")?;
    let json = require(
        multi_controller_json(&fixture),
        "multi-controller fixture should decode",
    )?;
    require_json(
        &json,
        "\"schema\":\"multi_controller\"",
        "schema should be emitted",
    )?;
    require_json(&json, "\"name\":\"walk\"", "track name should be emitted")?;
    require_json(&json, "\"start_time\":0.0", "track start should be emitted")?;
    require_json(&json, "\"end_time\":10.0", "track end should be emitted")?;
    Ok(())
}

#[test]
/// Keeps `vertex_key_decodes_offsets_and_indices` local because it shares
/// the rig binary-layout invariant.
fn vertex_key_decodes_offsets_and_indices() -> Result<(), String> {
    let fixture = require(vertex_fixture(), "vertex key fixture should build")?;
    let json = require(
        vertex_key_json(&fixture),
        "vertex key fixture should decode",
    )?;
    require_json(
        &json,
        "\"schema\":\"vertex_anim_key\"",
        "schema should be emitted",
    )?;
    require_json(
        &json,
        "\"kind\":\"vector\"",
        "vector offset list should be emitted",
    )?;
    require_json(
        &json,
        "\"param\":\"POSN\"",
        "offset param should be emitted",
    )?;
    require_json(
        &json,
        "\"indices\":[{\"version\":0",
        "index list should be emitted",
    )?;
    require_json(
        &json,
        "\"indices\":[42]",
        "offset indices should be emitted",
    )?;
    Ok(())
}

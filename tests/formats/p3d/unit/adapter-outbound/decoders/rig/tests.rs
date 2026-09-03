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

/// Builds one skeleton joint fixture with a caller-selected parent.
fn skeleton_joint_fixture(name: &str, parent: u32) -> Option<Vec<u8>> {
    chunk(
        SKELETON_JOINT,
        fields(vec![
            pstring(name)?,
            u32_field(parent),
            u32_field(0),
            u32_field(7),
            u32_field(1),
            u32_field(2),
            u32_field(3),
            identity_matrix(),
        ]),
        Vec::new(),
    )
}

/// Builds a skeleton fixture with one joint and rest pose.
fn skeleton_fixture() -> Option<Vec<u8>> {
    let joint = skeleton_joint_fixture("root", u32::MAX)?;
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
    multi_fixture_with_values(0, 10., 30., 0., 10., 1.)
}

/// Builds a multi-controller fixture with caller-selected root/timing evidence.
fn multi_fixture_with_values(
    version: u32,
    length: f32,
    frame_rate: f32,
    start_time: f32,
    end_time: f32,
    scale: f32,
) -> Option<Vec<u8>> {
    let track = chunk(
        MULTI_CONTROLLER_TRACK,
        fields(vec![
            pstring("walk")?,
            f32_field(start_time),
            f32_field(end_time),
            f32_field(scale),
        ]),
        Vec::new(),
    )?;
    chunk(
        MULTI_CONTROLLER,
        fields(vec![
            pstring("controller")?,
            u32_field(version),
            f32_field(length),
            f32_field(frame_rate),
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
            u32_field(VERTEX_PARAM_POSITION),
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

/// Builds one schema-backed vertex vector offset list.
fn vertex_vector_list(
    version: u32,
    param: u32,
    indices: Vec<Vec<u8>>,
) -> Option<Vec<u8>> {
    chunk(
        VERTEX_VECTOR_OFFSETS,
        fields(vec![
            u32_field(version),
            u32_field(1),
            u32_field(param),
            f32_field(1.),
            f32_field(2.),
            f32_field(3.),
        ]),
        indices,
    )
}

/// Builds a vertex-animation key around caller-provided offset lists.
fn vertex_key_with_lists(version: u32, lists: Vec<Vec<u8>>) -> Option<Vec<u8>> {
    chunk(
        VERTEX_KEY,
        fields(vec![u32_field(version), pstring("face")?]),
        lists,
    )
}

/// Builds one schema-backed vertex UV offset list.
fn vertex_vector2_list(param: u32) -> Option<Vec<u8>> {
    chunk(
        VERTEX_VECTOR2_OFFSETS,
        fields(vec![
            u32_field(0),
            u32_field(1),
            u32_field(param),
            f32_field(0.25),
            f32_field(0.75),
        ]),
        Vec::new(),
    )
}

/// Builds one schema-backed vertex colour offset list.
fn vertex_colour_list() -> Option<Vec<u8>> {
    chunk(
        VERTEX_COLOUR_OFFSETS,
        fields(vec![
            u32_field(0),
            u32_field(1),
            u16_field(1),
            u16_field(2),
            u16_field(3),
            u16_field(4),
        ]),
        Vec::new(),
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
fn skeleton_rejects_loader_contract_drift() -> Result<(), String> {
    let source = require(skeleton_fixture(), "skeleton fixture should build")?;

    let mut version_drift = source.clone();
    version_drift
        .get_mut(17..21)
        .ok_or_else(|| String::from("skeleton version field is missing"))?
        .copy_from_slice(&1_u32.to_le_bytes());
    if skeleton_json(&version_drift).is_some() {
        return Err(String::from("skeleton accepted an unobserved version"));
    }

    let mut empty_count = source;
    empty_count
        .get_mut(21..25)
        .ok_or_else(|| String::from("skeleton count field is missing"))?
        .copy_from_slice(&0_u32.to_le_bytes());
    if skeleton_json(&empty_count).is_some() {
        return Err(String::from("skeleton accepted a zero joint count"));
    }

    let root = require(
        skeleton_joint_fixture("root", u32::MAX),
        "root joint fixture should build",
    )?;
    let forward = require(
        skeleton_joint_fixture("child", 1),
        "forward-parent joint fixture should build",
    )?;
    let skeleton_name = require(pstring("skel"), "skeleton name should build")?;
    let invalid_parent = require(
        chunk(
            SKELETON,
            fields(vec![skeleton_name, u32_field(0), u32_field(2)]),
            vec![root, forward],
        ),
        "forward-parent skeleton fixture should build",
    )?;
    if skeleton_json(&invalid_parent).is_some() {
        return Err(String::from("skeleton accepted a non-previous parent"));
    }
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
fn animation_rejects_impossible_key_count() -> Result<(), String> {
    let mut fixture = require(
        animation_fixture(),
        "animation fixture should build",
    )?;
    let channel_id = CHANNEL_FLOAT1.to_le_bytes();
    let channel_start = fixture
        .windows(channel_id.len())
        .position(|window| window == channel_id)
        .ok_or_else(|| String::from("animation channel fixture is missing"))?;
    let count_offset = channel_start
        .checked_add(20)
        .ok_or_else(|| String::from("channel count offset overflowed"))?;
    fixture
        .get_mut(count_offset..count_offset + 4)
        .ok_or_else(|| {
            String::from("animation channel count field is missing")
        })?
        .copy_from_slice(&u32::MAX.to_le_bytes());
    if animation_json(&fixture).is_some() {
        return Err(String::from(
            "animation accepted an impossible fixed-width key count",
        ));
    }
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
fn multi_controller_rejects_loader_root_contract_drift() -> Result<(), String> {
    for values in [
        (1_u32, 10., 30.),
        (0, -1., 30.),
        (0, 10., 0.),
        (0, 10., -30.),
    ] {
        let fixture = require(
            multi_fixture_with_values(
                values.0, values.1, values.2, 0., 10., 1.,
            ),
            "invalid multi-controller root fixture should build",
        )?;
        if multi_controller_json(&fixture).is_some() {
            return Err(String::from(
                "multi-controller accepted loader-invalid root evidence",
            ));
        }
    }
    Ok(())
}

#[test]
fn multi_controller_rejects_impossible_track_count() -> Result<(), String> {
    let packed = require(
        chunk(
            MULTI_CONTROLLER_TRACKS,
            fields(vec![u32_field(u32::MAX)]),
            Vec::new(),
        ),
        "packed track fixture should build",
    )?;
    let fixture = require(
        chunk(
            MULTI_CONTROLLER,
            fields(vec![
                require(pstring("controller"), "controller name should build")?,
                u32_field(0),
                f32_field(10.),
                f32_field(30.),
                u32_field(1),
            ]),
            vec![packed],
        ),
        "multi-controller fixture should build",
    )?;
    if multi_controller_json(&fixture).is_some() {
        return Err(String::from(
            "multi-controller accepted an impossible packed track count",
        ));
    }
    Ok(())
}

#[test]
fn multi_controller_rejects_nonfinite_timing_evidence() -> Result<(), String> {
    for values in [
        (f32::NAN, 30., 0., 10., 1.),
        (10., f32::INFINITY, 0., 10., 1.),
        (10., 30., f32::NAN, 10., 1.),
        (10., 30., 0., f32::INFINITY, 1.),
        (10., 30., 0., 10., f32::NAN),
    ] {
        let fixture = require(
            multi_fixture_with_values(
                0, values.0, values.1, values.2, values.3, values.4,
            ),
            "non-finite multi-controller fixture should build",
        )?;
        if multi_controller_json(&fixture).is_some() {
            return Err(String::from(
                "non-finite multi-controller timing must fail closed",
            ));
        }
    }
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
        "\"param\":\"POS_\"",
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

#[test]
fn vertex_key_accepts_reference_loader_offset_shapes() -> Result<(), String> {
    let position = require(
        vertex_vector_list(0, VERTEX_PARAM_POSITION, Vec::new()),
        "position offset fixture should build",
    )?;
    let normal = require(
        vertex_vector_list(0, VERTEX_PARAM_NORMAL, Vec::new()),
        "normal offset fixture should build",
    )?;
    let uv0 = require(
        vertex_vector2_list(VERTEX_PARAM_UV0),
        "UV offset fixture should build",
    )?;
    let colour =
        require(vertex_colour_list(), "colour offset fixture should build")?;
    let fixture = require(
        vertex_key_with_lists(0, vec![colour, position, normal, uv0]),
        "vertex key fixture should build",
    )?;
    let json = require(
        vertex_key_json(&fixture),
        "reference-loader vertex offsets should decode",
    )?;
    require_json(
        &json,
        r#""offsets":[[1,2,3,4]]"#,
        "colour UWORDs should decode",
    )?;
    require_json(
        &json,
        r#""param":"POS_""#,
        "position parameter should decode",
    )?;
    require_json(&json, r#""param":"UV0_""#, "UV parameter should decode")?;
    require_json(
        &json,
        r#""indices":[]"#,
        "optional index list should remain absent",
    )
}

#[test]
fn vertex_key_rejects_impossible_offset_count() -> Result<(), String> {
    let colour = require(
        chunk(
            VERTEX_COLOUR_OFFSETS,
            fields(vec![u32_field(0), u32_field(u32::MAX)]),
            Vec::new(),
        ),
        "huge colour-offset fixture should build",
    )?;
    let fixture = require(
        vertex_key_with_lists(0, vec![colour]),
        "huge-count vertex key fixture should build",
    )?;
    if vertex_key_json(&fixture).is_some() {
        return Err(String::from(
            "vertex key accepted an impossible offset count",
        ));
    }
    Ok(())
}

#[test]
fn vertex_key_rejects_reference_loader_contract_drift() -> Result<(), String> {
    let mut bad_key_version =
        require(vertex_fixture(), "vertex fixture should build")?;
    bad_key_version
        .get_mut(12..16)
        .ok_or_else(|| String::from("vertex key version field is missing"))?
        .copy_from_slice(&1_u32.to_le_bytes());
    if vertex_key_json(&bad_key_version).is_some() {
        return Err(String::from("vertex key accepted an unobserved version"));
    }

    let bad_list = require(
        vertex_vector_list(1, VERTEX_PARAM_POSITION, Vec::new()),
        "bad list fixture should build",
    )?;
    let bad_list_key = require(
        vertex_key_with_lists(0, vec![bad_list]),
        "bad-list key fixture should build",
    )?;
    if vertex_key_json(&bad_list_key).is_some() {
        return Err(String::from(
            "vertex key accepted an unobserved list version",
        ));
    }

    let bad_param = require(
        vertex_vector_list(0, u32::from_le_bytes(*b"BAD!"), Vec::new()),
        "bad parameter fixture should build",
    )?;
    let bad_param_key = require(
        vertex_key_with_lists(0, vec![bad_param]),
        "bad-parameter key fixture should build",
    )?;
    if vertex_key_json(&bad_param_key).is_some() {
        return Err(String::from(
            "vertex key accepted an invalid vector parameter",
        ));
    }

    let duplicate = require(
        vertex_vector_list(0, VERTEX_PARAM_POSITION, Vec::new()),
        "duplicate offset fixture should build",
    )?;
    let duplicate_key = require(
        vertex_key_with_lists(0, vec![duplicate.clone(), duplicate]),
        "duplicate key fixture should build",
    )?;
    if vertex_key_json(&duplicate_key).is_some() {
        return Err(String::from(
            "vertex key accepted duplicate position offsets",
        ));
    }

    for (version, count, index) in
        [(1_u32, 1_u32, 42_u32), (0, 2, 42), (0, 1, u32::MAX)]
    {
        let indices = require(
            chunk(
                VERTEX_INDEX_OFFSETS,
                fields(vec![
                    u32_field(version),
                    u32_field(count),
                    u32_field(index),
                ]),
                Vec::new(),
            ),
            "index drift fixture should build",
        )?;
        let vector = require(
            vertex_vector_list(0, VERTEX_PARAM_POSITION, vec![indices]),
            "indexed vector fixture should build",
        )?;
        let fixture = require(
            vertex_key_with_lists(0, vec![vector]),
            "indexed key fixture should build",
        )?;
        if vertex_key_json(&fixture).is_some() {
            return Err(String::from(
                "vertex key accepted invalid index evidence",
            ));
        }
    }
    Ok(())
}

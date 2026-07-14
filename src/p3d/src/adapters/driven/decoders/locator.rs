// File:
//   - locator.rs
// Path:
//   - src/p3d/src/adapters/driven/decoders/locator.rs
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
//   - Typed interpretation of every declared SRR locator payload family.
// - Must-Not:
//   - Read package storage, decode trigger chunks, or invent unsupported types.
// - Allows:
//   - Validate locator payload shapes and render deterministic schema JSON.
// - Split-When:
//   - One locator family gains an independently reusable binary contract.
// - Merge-When:
//   - Another decoder owns every declared locator payload without fallback.
// - Summary:
//   - Complete known-type SRR locator payload decoder.
// - Description:
//   - Converts locator data words into the runtime fields they represent.
// - Usage:
//   - Called by the lossless package extractor after the locator header parses.
// - Defaults:
//   - Undeclared numeric types and malformed known payloads fail closed.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Complete known-type SRR locator payload decoding.

use super::super::json::{escape_json, render_f32};

/// Return the stable runtime name for one declared locator type.
#[must_use]
pub const fn type_name(locator_type: u32) -> Option<&'static str> {
    match locator_type {
        0 => Some("event"),
        1 => Some("script"),
        2 => Some("generic"),
        3 => Some("car_start"),
        4 => Some("spline"),
        5 => Some("dynamic_zone"),
        6 => Some("occlusion"),
        7 => Some("interior_entrance"),
        8 => Some("directional"),
        9 => Some("action"),
        10 => Some("fov"),
        11 => Some("breakable_camera"),
        12 => Some("static_camera"),
        13 => Some("ped_group"),
        14 => Some("coin"),
        15 => Some("spawn_point"),
        _ => None,
    }
}

/// Decode the type-specific payload for one declared locator.
#[must_use]
pub fn data_interpretation_json(
    locator_type: u32,
    data: &[u32],
    num_triggers: u32,
) -> Option<String> {
    match locator_type {
        0 => event_json(data),
        1 => text_json(
            "script", "script", data,
        ),
        2 => Some(
            ignored_data_json(
                "generic", data,
            ),
        ),
        3 => car_start_json(data),
        4 => Some(
            ignored_data_json(
                "spline", data,
            ),
        ),
        5 => text_json(
            "dynamic_zone",
            "zone",
            data,
        ),
        6 => occlusion_json(
            data,
            num_triggers,
        ),
        7 => interior_entrance_json(data),
        8 => matrix_json(
            "directional",
            "basis",
            data,
        ),
        9 => action_json(data),
        10 => fov_json(data),
        11 => breakable_camera_json(data),
        12 => static_camera_json(data),
        13 => ped_group_json(data),
        14 => Some(
            ignored_data_json(
                "coin", data,
            ),
        ),
        15 => Some(spawn_point_json(data)),
        _ => None,
    }
}

/// Decode an event locator's event identifier and optional event data.
fn event_json(data: &[u32]) -> Option<String> {
    let event = data
        .first()
        .copied()?;
    let extra_data = data
        .get(1)
        .copied();
    let extra_json = extra_data.map_or_else(
        || String::from("null"),
        |value| value.to_string(),
    );
    Some(
        format!(
            concat!(
                "{{\"kind\":\"event\",",
                "\"event_id\":{},",
                "\"event_name\":\"{}\",",
                "\"extra_data\":{}}}"
            ),
            event,
            event_name(event),
            extra_json
        ),
    )
}

/// Decode one word-packed text payload.
fn text_json(
    kind: &str,
    field: &str,
    data: &[u32],
) -> Option<String> {
    let value = word_text(data)?;
    Some(
        format!(
            "{{\"kind\":\"{}\",\"{}\":\"{}\"}}",
            kind,
            field,
            escape_json(&value)
        ),
    )
}

/// Preserve ignored words for a declared type whose runtime uses no fields.
fn ignored_data_json(
    kind: &str,
    data: &[u32],
) -> String {
    format!(
        "{{\"kind\":\"{}\",\"ignored_data\":[{}]}}",
        kind,
        u32_list(data)
    )
}

/// Decode car orientation, parked-car state, and an optional special car name.
fn car_start_json(data: &[u32]) -> Option<String> {
    let rotation = float_json(*data.first()?);
    let parked = data
        .get(1)
        .is_some_and(|value| *value == 1);
    let special_car = if data.len() > 2 {
        Some(word_text(data.get(2..)?)?)
    } else {
        None
    };
    let special_car_json = special_car
        .as_ref()
        .map_or_else(
            || String::from("null"),
            |value| {
                format!(
                    "\"{}\"",
                    escape_json(value)
                )
            },
        );
    Some(
        format!(
            concat!(
                "{{\"kind\":\"car_start\",",
                "\"rotation_radians\":{},",
                "\"has_parked_car\":{},",
                "\"special_car\":{}}}"
            ),
            rotation, parked, special_car_json
        ),
    )
}

/// Decode the number of occluding triggers from one occlusion locator.
fn occlusion_json(
    data: &[u32],
    num_triggers: u32,
) -> Option<String> {
    let occlusion_triggers = data
        .first()
        .copied()
        .unwrap_or_else(|| num_triggers.saturating_sub(1));
    if occlusion_triggers > num_triggers {
        return None;
    }
    Some(
        format!(
            concat!(
                "{{\"kind\":\"occlusion\",",
                "\"num_triggers\":{},",
                "\"num_occlusion_triggers\":{},",
                "\"num_visibility_triggers\":{}}}"
            ),
            num_triggers,
            occlusion_triggers,
            num_triggers.saturating_sub(occlusion_triggers)
        ),
    )
}

/// Decode an interior package name followed by a 3-by-3 transform basis.
fn interior_entrance_json(data: &[u32]) -> Option<String> {
    let bytes = word_bytes(data);
    let terminator = bytes
        .iter()
        .position(|value| *value == 0)?;
    let interior_name = std::str::from_utf8(bytes.get(..terminator)?)
        .ok()?
        .to_owned();
    let matrix_start = terminator
        .saturating_add(3)
        .checked_div(4)?;
    let matrix_words = data.get(matrix_start..matrix_start.checked_add(9)?)?;
    Some(
        format!(
            concat!(
                "{{\"kind\":\"interior_entrance\",",
                "\"interior_file\":\"{}\",",
                "\"basis\":{}}}"
            ),
            escape_json(&interior_name),
            matrix3(matrix_words)?
        ),
    )
}

/// Decode a 3-by-3 matrix-only locator family.
fn matrix_json(
    kind: &str,
    field: &str,
    data: &[u32],
) -> Option<String> {
    Some(
        format!(
            "{{\"kind\":\"{}\",\"{}\":{}}}",
            kind,
            field,
            matrix3(data)?
        ),
    )
}

/// Decode action target, joint, action, input, and transform policy.
fn action_json(data: &[u32]) -> Option<String> {
    if data.len() < 5 {
        return None;
    }
    let string_words = data.get(
        ..data
            .len()
            .checked_sub(2)?,
    )?;
    let strings = null_strings(
        &word_bytes(string_words),
        3,
    )?;
    let button_input = *data.get(
        data.len()
            .checked_sub(2)?,
    )?;
    let should_transform = *data.last()? == 1;
    Some(
        format!(
            concat!(
                "{{\"kind\":\"action\",",
                "\"object_name\":\"{}\",",
                "\"joint_name\":\"{}\",",
                "\"action_name\":\"{}\",",
                "\"button_input\":{},",
                "\"should_transform\":{}}}"
            ),
            escape_json(strings.first()?),
            escape_json(strings.get(1)?),
            escape_json(strings.get(2)?),
            button_input,
            should_transform
        ),
    )
}

/// Decode an FOV transition payload.
fn fov_json(data: &[u32]) -> Option<String> {
    Some(
        format!(
            concat!(
                "{{\"kind\":\"fov\",",
                "\"fov_degrees\":{},",
                "\"time\":{},",
                "\"rate\":{}}}"
            ),
            float_json(*data.first()?),
            float_json(*data.get(1)?),
            float_json(*data.get(2)?)
        ),
    )
}

/// Decode the authored but runtime-dormant breakable-camera payload.
fn breakable_camera_json(data: &[u32]) -> Option<String> {
    let basis = matrix3(data)?;
    Some(
        format!(
            concat!(
                "{{\"kind\":\"breakable_camera\",",
                "\"loader_behavior\":\"dormant\",",
                "\"basis\":{},",
                "\"fov_degrees\":{}}}"
            ),
            basis,
            float_json(*data.get(9)?)
        ),
    )
}

/// Decode the complete static-camera field set and bit flags.
fn static_camera_json(data: &[u32]) -> Option<String> {
    if data.len() < 6 {
        return None;
    }
    let transition_rate = data
        .get(6)
        .copied()
        .map_or_else(
            || String::from("0.04"),
            float_json,
        );
    let camera_flags = data
        .get(7)
        .copied()
        .unwrap_or_default();
    let cut_in_out = data
        .get(8)
        .is_some_and(|value| *value == 1);
    let mode_flags = data
        .get(9)
        .copied()
        .unwrap_or_default();
    Some(
        format!(
            concat!(
                "{{\"kind\":\"static_camera\",",
                "\"target_offset\":[{},{},{}],",
                "\"fov_degrees\":{},",
                "\"target_lag\":{},",
                "\"tracking\":{},",
                "\"transition_rate\":{},",
                "\"one_shot\":{},",
                "\"disable_fov_lag\":{},",
                "\"cut_in_out\":{},",
                "\"car_only\":{},",
                "\"on_foot_only\":{}}}"
            ),
            float_json(*data.first()?),
            float_json(*data.get(1)?),
            float_json(*data.get(2)?),
            float_json(*data.get(3)?),
            float_json(*data.get(4)?),
            *data.get(5)? == 1,
            transition_rate,
            camera_flags & 1 != 0,
            camera_flags & 2 != 0,
            cut_in_out,
            mode_flags & 1 != 0,
            mode_flags & 2 != 0
        ),
    )
}

/// Decode the pedestrian model-group identifier.
fn ped_group_json(data: &[u32]) -> Option<String> {
    Some(
        format!(
            "{{\"kind\":\"ped_group\",\"group\":{}}}",
            data.first()?
        ),
    )
}

/// Preserve the declared spawn-point type and its base-locator loader behavior.
fn spawn_point_json(data: &[u32]) -> String {
    format!(
        concat!(
            "{{\"kind\":\"spawn_point\",",
            "\"loader_behavior\":\"base_locator\",",
            "\"ignored_data\":[{}]}}"
        ),
        u32_list(data)
    )
}

/// Render one 3-by-3 matrix from the first nine words.
fn matrix3(data: &[u32]) -> Option<String> {
    let values = data.get(..9)?;
    Some(
        format!(
            "[[{},{},{}],[{},{},{}],[{},{},{}]]",
            float_json(*values.first()?),
            float_json(*values.get(1)?),
            float_json(*values.get(2)?),
            float_json(*values.get(3)?),
            float_json(*values.get(4)?),
            float_json(*values.get(5)?),
            float_json(*values.get(6)?),
            float_json(*values.get(7)?),
            float_json(*values.get(8)?)
        ),
    )
}

/// Render one word as a finite JSON float or `null`.
fn float_json(value: u32) -> String {
    let decoded = f32::from_bits(value);
    render_f32(
        decoded,
        decoded.to_string(),
    )
}

/// Convert little-endian data words to their authored byte stream.
fn word_bytes(data: &[u32]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(
        data.len()
            .saturating_mul(4),
    );
    for value in data {
        bytes.extend_from_slice(&value.to_le_bytes());
    }
    bytes
}

/// Decode one null-terminated word-packed UTF-8 string.
fn word_text(data: &[u32]) -> Option<String> {
    let bytes = word_bytes(data);
    let end = bytes
        .iter()
        .position(|value| *value == 0)
        .unwrap_or(bytes.len());
    std::str::from_utf8(bytes.get(..end)?)
        .ok()
        .map(ToOwned::to_owned)
}

/// Decode a fixed number of null-separated UTF-8 strings.
fn null_strings(
    bytes: &[u8],
    count: usize,
) -> Option<Vec<String>> {
    let mut strings = Vec::with_capacity(count);
    let mut cursor = 0_usize;
    while strings.len() < count {
        while bytes
            .get(cursor)
            .is_some_and(|value| *value == 0)
        {
            cursor = cursor.checked_add(1)?;
        }
        let remainder = bytes.get(cursor..)?;
        let length = remainder
            .iter()
            .position(|value| *value == 0)?;
        let end = cursor.checked_add(length)?;
        strings.push(
            std::str::from_utf8(bytes.get(cursor..end)?)
                .ok()?
                .to_owned(),
        );
        cursor = end.checked_add(1)?;
    }
    Some(strings)
}

/// Render one deterministic JSON list of unsigned words.
fn u32_list(data: &[u32]) -> String {
    data.iter()
        .map(u32::to_string)
        .collect::<Vec<_>>()
        .join(",")
}

/// Return a stable label for one known locator event identifier.
const fn event_name(event: u32) -> &'static str {
    match event {
        0 => "flag",
        1 => "camera_cut",
        2 => "check_point",
        3 => "base",
        4 => "death",
        5 => "interior_exit",
        6 => "bounce_pad",
        49 => "parked_birds",
        50 => "whacky_gravity",
        51 => "far_plane_change",
        65 => "goo_damage",
        66 => "coin_zone",
        67 => "light_change",
        68 => "trap",
        79 => "dynamic_zone",
        80 => "occlusion_zone",
        81 => "car_door",
        82 => "action_button",
        83 => "interior_entrance",
        84 => "generic_button_handler_event",
        85 => "fountain_jump",
        86 => "load_ped_model_group",
        87 => "gag",
        _ => "event",
    }
}

#[cfg(test)]
#[path = "locator_tests.rs"]
mod tests;

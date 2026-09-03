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
//   - Locator outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Locator outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for p3d.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Locator outbound adapter.

use super::super::json::{escape_json, render_f32};

/// Maximum whole-word payload stored behind one authored 255-byte size field.
const MAX_BYTE_SIZED_WORDS: usize = 63;

/// Return the stable runtime name for one declared locator type.
#[must_use]
pub const fn type_name(locator_type: u32) -> &'static str {
    match locator_type {
        0 => "event",
        1 => "script",
        2 => "generic",
        3 => "car_start",
        4 => "spline",
        5 => "dynamic_zone",
        6 => "occlusion",
        7 => "interior_entrance",
        8 => "directional",
        9 => "action",
        10 => "fov",
        11 => "breakable_camera",
        12 => "static_camera",
        13 => "ped_group",
        14 => "coin",
        15 => "spawn_point",
        _ => "unknown",
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
        1 => text_json("script", "script", data),
        2 => Some(ignored_data_json("generic", data)),
        3 => car_start_json(data),
        4 => Some(ignored_data_json("spline", data)),
        5 => byte_sized_text_json("dynamic_zone", "zone", data),
        6 => occlusion_json(data, num_triggers),
        7 => interior_entrance_json(data),
        8 => matrix_json("directional", "basis", data),
        9 => action_json(data),
        10 => fov_json(data),
        11 => Some(breakable_camera_json(data)),
        12 => static_camera_json(data),
        13 => ped_group_json(data),
        14 => Some(ignored_data_json("coin", data)),
        15 => Some(base_locator_json("spawn_point", data)),
        _ => Some(base_locator_json("unknown", data)),
    }
}

/// Decode an event locator's event identifier and optional event data.
fn event_json(data: &[u32]) -> Option<String> {
    let event = data.first().copied()?;
    let extra_data = (data.len() == 2).then(|| data.get(1).copied()).flatten();
    let extra_json = extra_data
        .map_or_else(|| String::from("null"), |value| value.to_string());
    Some(format!(
        concat!(
            "{{\"kind\":\"event\",",
            "\"event_id\":{},",
            "\"event_name\":\"{}\",",
            "\"extra_data\":{}}}"
        ),
        event,
        event_name(event),
        extra_json
    ))
}

/// Decode one word-packed text payload.
fn text_json(kind: &str, field: &str, data: &[u32]) -> Option<String> {
    let value = word_text(data)?;
    Some(format!(
        "{{\"kind\":\"{}\",\"{}\":\"{}\"}}",
        kind,
        field,
        escape_json(&value)
    ))
}

/// Decode text stored behind an authored one-byte payload-size field.
fn byte_sized_text_json(
    kind: &str,
    field: &str,
    data: &[u32],
) -> Option<String> {
    if data.len() > MAX_BYTE_SIZED_WORDS {
        return None;
    }
    text_json(kind, field, data)
}

/// Preserve ignored words for a declared type whose runtime uses no fields.
fn ignored_data_json(kind: &str, data: &[u32]) -> String {
    format!(
        "{{\"kind\":\"{}\",\"ignored_data\":[{}]}}",
        kind,
        u32_list(data)
    )
}

/// Decode car orientation, parked-car state, and an optional special car name.
fn car_start_json(data: &[u32]) -> Option<String> {
    let rotation = float_json(*data.first()?);
    let parked = data.get(1).is_some_and(|value| *value == 1);
    let special_car = if data.len() > 2 {
        Some(word_text(data.get(2..)?)?)
    } else {
        None
    };
    let special_car_json = special_car.as_ref().map_or_else(
        || String::from("null"),
        |value| format!("\"{}\"", escape_json(value)),
    );
    Some(format!(
        concat!(
            "{{\"kind\":\"car_start\",",
            "\"rotation_radians\":{},",
            "\"has_parked_car\":{},",
            "\"special_car\":{}}}"
        ),
        rotation, parked, special_car_json
    ))
}

/// Decode the number of occluding triggers from one occlusion locator.
fn occlusion_json(data: &[u32], num_triggers: u32) -> Option<String> {
    let [occlusion_triggers] = data else {
        return None;
    };
    if *occlusion_triggers > num_triggers {
        return None;
    }
    Some(format!(
        concat!(
            "{{\"kind\":\"occlusion\",",
            "\"num_triggers\":{},",
            "\"num_occlusion_triggers\":{},",
            "\"num_visibility_triggers\":{}}}"
        ),
        num_triggers,
        occlusion_triggers,
        num_triggers.saturating_sub(*occlusion_triggers)
    ))
}

/// Decode an interior package name followed by a 3-by-3 transform basis.
fn interior_entrance_json(data: &[u32]) -> Option<String> {
    if data.len() > MAX_BYTE_SIZED_WORDS {
        return None;
    }
    let bytes = word_bytes(data);
    let terminator = bytes.iter().position(|value| *value == 0)?;
    let interior_name =
        String::from_utf8_lossy(bytes.get(..terminator)?).into_owned();
    let matrix_start = terminator.saturating_add(3).checked_div(4)?;
    let matrix_words = data.get(matrix_start..matrix_start.checked_add(9)?)?;
    Some(format!(
        concat!(
            "{{\"kind\":\"interior_entrance\",",
            "\"interior_file\":\"{}\",",
            "\"basis\":{}}}"
        ),
        escape_json(&interior_name),
        matrix3(matrix_words)?
    ))
}

/// Decode a 3-by-3 matrix-only locator family.
fn matrix_json(kind: &str, field: &str, data: &[u32]) -> Option<String> {
    Some(format!(
        "{{\"kind\":\"{}\",\"{}\":{}}}",
        kind,
        field,
        matrix3(data)?
    ))
}

/// Decode action target, joint, action, input, and transform policy.
fn action_json(data: &[u32]) -> Option<String> {
    if data.len() < 5 {
        return None;
    }
    let string_words = data.get(..data.len().checked_sub(2)?)?;
    if string_words.len() > MAX_BYTE_SIZED_WORDS {
        return None;
    }
    let strings = null_strings(&word_bytes(string_words), 3)?;
    let button_input = *data.get(data.len().checked_sub(2)?)?;
    let should_transform = *data.last()? == 1;
    Some(format!(
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
    ))
}

/// Decode an FOV transition payload.
fn fov_json(data: &[u32]) -> Option<String> {
    Some(format!(
        concat!(
            "{{\"kind\":\"fov\",",
            "\"fov_degrees\":{},",
            "\"time\":{},",
            "\"rate\":{}}}"
        ),
        float_json(*data.first()?),
        float_json(*data.get(1)?),
        float_json(*data.get(2)?)
    ))
}

/// Preserve the runtime-dormant breakable-camera payload without inference.
fn breakable_camera_json(data: &[u32]) -> String {
    format!(
        concat!(
            "{{\"kind\":\"breakable_camera\",",
            "\"loader_behavior\":\"dormant\",",
            "\"ignored_data\":[{}]}}"
        ),
        u32_list(data)
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
        .map_or_else(|| String::from("0.04"), float_json);
    let camera_flags = data.get(7).copied().unwrap_or_default();
    let cut_in_out = data.get(8).is_some_and(|value| *value == 1);
    let mode_flags = data.get(9).copied().unwrap_or_default();
    Some(format!(
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
    ))
}

/// Decode the pedestrian model-group identifier.
fn ped_group_json(data: &[u32]) -> Option<String> {
    Some(format!(
        "{{\"kind\":\"ped_group\",\"group\":{}}}",
        data.first()?
    ))
}

/// Preserve a locator type whose runtime falls back to the base locator.
fn base_locator_json(kind: &str, data: &[u32]) -> String {
    format!(
        concat!(
            "{{\"kind\":\"{}\",",
            "\"loader_behavior\":\"base_locator\",",
            "\"ignored_data\":[{}]}}"
        ),
        kind,
        u32_list(data)
    )
}

/// Render one 3-by-3 matrix from the first nine words.
fn matrix3(data: &[u32]) -> Option<String> {
    let values = data.get(..9)?;
    Some(format!(
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
    ))
}

/// Render one word as a finite JSON float or `null`.
fn float_json(value: u32) -> String {
    let decoded = f32::from_bits(value);
    render_f32(decoded, decoded.to_string())
}

/// Convert little-endian data words to their authored byte stream.
fn word_bytes(data: &[u32]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(data.len().saturating_mul(4));
    for value in data {
        bytes.extend_from_slice(&value.to_le_bytes());
    }
    bytes
}

/// Decode one null-terminated word-packed byte string for a JSON text view.
fn word_text(data: &[u32]) -> Option<String> {
    let bytes = word_bytes(data);
    let end = bytes
        .iter()
        .position(|value| *value == 0)
        .unwrap_or(bytes.len());
    Some(String::from_utf8_lossy(bytes.get(..end)?).into_owned())
}

/// Decode fixed null-separated byte strings for JSON text views.
fn null_strings(bytes: &[u8], count: usize) -> Option<Vec<String>> {
    let mut strings = Vec::with_capacity(count);
    let mut cursor = 0_usize;
    while strings.len() < count {
        let remainder = bytes.get(cursor..)?;
        let length = remainder.iter().position(|value| *value == 0)?;
        let end = cursor.checked_add(length)?;
        strings.push(
            String::from_utf8_lossy(bytes.get(cursor..end)?).into_owned(),
        );
        cursor = end.checked_add(1)?;
        if strings.len() < count {
            while bytes.get(cursor).is_some_and(|value| *value == 0) {
                cursor = cursor.checked_add(1)?;
            }
        }
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
        7 => "ambient_sound_city",
        8 => "ambient_sound_seaside",
        9 => "ambient_sound_suburbs",
        10 => "ambient_sound_forest",
        11 => "ambient_kwik_e_mart_rooftop",
        12 => "ambient_sound_farm",
        13 => "ambient_sound_barn",
        14 => "ambient_sound_power_plant_exterior",
        15 => "ambient_sound_power_plant_interior",
        16 => "ambient_sound_river",
        17 => "ambient_sound_city_business",
        18 => "ambient_sound_construction",
        19 => "ambient_sound_stadium",
        20 => "ambient_sound_stadium_tunnel",
        21 => "ambient_sound_expressway",
        22 => "ambient_sound_slum",
        23 => "ambient_sound_rail_yard",
        24 => "ambient_sound_hospital",
        25 => "ambient_sound_light_city",
        26 => "ambient_sound_shipyard",
        27 => "ambient_sound_quay",
        28 => "ambient_sound_lighthouse",
        29 => "ambient_sound_country_highway",
        30 => "ambient_sound_krustylu",
        31 => "ambient_sound_dam",
        32 => "ambient_sound_forest_highway",
        33 => "ambient_sound_retaining_wall_tunnel",
        34 => "ambient_sound_krustylu_exterior",
        35 => "ambient_sound_duff_exterior",
        36 => "ambient_sound_duff_interior",
        37 => "ambient_sound_stone_cutter_tunnel",
        38 => "ambient_sound_stone_cutter_hall",
        39 => "ambient_sound_sewers",
        40 => "ambient_sound_burns_tunnel",
        41 => "ambient_sound_pp_room_1",
        42 => "ambient_sound_pp_room_2",
        43 => "ambient_sound_pp_room_3",
        44 => "ambient_sound_pp_tunnel_1",
        45 => "ambient_sound_pp_tunnel_2",
        46 => "ambient_sound_mansion_interior",
        47 => "parked_birds",
        48 => "whacky_gravity",
        49 => "far_plane_change",
        50 => "ambient_sound_country_night",
        51 => "ambient_sound_suburbs_night",
        52 => "ambient_sound_forest_night",
        53 => "ambient_sound_halloween_1",
        54 => "ambient_sound_halloween_2",
        55 => "ambient_sound_halloween_3",
        56 => "ambient_sound_placeholder_3",
        57 => "ambient_sound_placeholder_4",
        58 => "ambient_sound_placeholder_5",
        59 => "ambient_sound_placeholder_6",
        60 => "ambient_sound_placeholder_7",
        61 => "ambient_sound_placeholder_8",
        62 => "ambient_sound_placeholder_9",
        63 => "goo_damage",
        64 => "coin_zone",
        65 => "light_change",
        66 => "trap",
        67 => "ambient_sound_seaside_night",
        68 => "ambient_sound_lighthouse_night",
        69 => "ambient_sound_brewery_night",
        70 => "ambient_sound_placeholder_10",
        71 => "ambient_sound_placeholder_11",
        72 => "ambient_sound_placeholder_12",
        73 => "ambient_sound_placeholder_13",
        74 => "ambient_sound_placeholder_14",
        75 => "ambient_sound_placeholder_15",
        76 => "ambient_sound_placeholder_16",
        77 => "dynamic_zone",
        78 => "occlusion_zone",
        79 => "car_door",
        80 => "action_button",
        81 => "interior_entrance",
        82 => "generic_button_handler_event",
        83 => "fountain_jump",
        84 => "load_ped_model_group",
        85 => "gag",
        _ => "event",
    }
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../../tests/formats/p3d/unit/adapter-outbound/decoders/locator_tests.rs"]
mod tests;

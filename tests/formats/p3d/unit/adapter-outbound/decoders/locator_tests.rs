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
//   - Locator tests test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Locator tests test module.
// - Description:
//   - Implements the declared test module responsibility for p3d.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Locator tests test module.

use super::{data_interpretation_json, type_name};

/// Convert byte text into little-endian words for fixtures.
fn words(value: &[u8]) -> Vec<u32> {
    value
        .chunks(4)
        .map(|chunk| {
            let mut bytes = [0_u8; 4];
            for (target, source) in bytes.iter_mut().zip(chunk) {
                *target = *source;
            }
            u32::from_le_bytes(bytes)
        })
        .collect()
}

/// Assert one payload decodes to valid JSON without fallback markers.
fn assert_known(
    locator_type: u32,
    data: &[u32],
    num_triggers: u32,
) -> Result<String, String> {
    let json = data_interpretation_json(locator_type, data, num_triggers)
        .ok_or_else(|| {
            format!("known locator type {locator_type} did not decode")
        })?;
    let _value =
        serde_json::from_str::<serde_json::Value>(&json).map_err(|error| {
            format!("locator type {locator_type} emitted invalid JSON: {error}")
        })?;
    if json.contains("\"kind\":\"unknown\"") {
        return Err(format!(
            "locator type {locator_type} used an unknown kind"
        ));
    }
    Ok(json)
}

#[test]
fn every_declared_locator_type_has_a_stable_name() {
    for locator_type in 0_u32..=15 {
        assert_ne!(type_name(locator_type), "unknown");
    }
    assert_eq!(type_name(16), "unknown");
    assert_eq!(type_name(u32::MAX), "unknown");
}

#[test]
fn locator_types_zero_through_seven_have_typed_interpretations()
-> Result<(), String> {
    let text = words(b"script\0");
    let zone = words(b"l1z1.p3d\0");
    let mut interior = words(b"interior\0\0\0\0");
    interior.extend_from_slice(&[
        1_f32.to_bits(),
        2_f32.to_bits(),
        3_f32.to_bits(),
        4_f32.to_bits(),
        5_f32.to_bits(),
        6_f32.to_bits(),
        7_f32.to_bits(),
        8_f32.to_bits(),
        9_f32.to_bits(),
    ]);
    let fixtures = [
        (0_u32, vec![2, 7], 1_u32),
        (1, text, 1),
        (2, Vec::new(), 0),
        (3, vec![0_f32.to_bits(), 1], 0),
        (4, Vec::new(), 1),
        (5, zone, 1),
        (6, vec![1], 2),
        (7, interior, 1),
    ];
    for (locator_type, data, triggers) in fixtures {
        let _json = assert_known(locator_type, &data, triggers)?;
    }
    Ok(())
}

#[test]
fn locator_types_eight_through_fifteen_have_typed_interpretations()
-> Result<(), String> {
    let matrix = vec![
        1_f32.to_bits(),
        2_f32.to_bits(),
        3_f32.to_bits(),
        4_f32.to_bits(),
        5_f32.to_bits(),
        6_f32.to_bits(),
        7_f32.to_bits(),
        8_f32.to_bits(),
        9_f32.to_bits(),
    ];
    let mut action = words(b"object\0joint\0action\0\0");
    action.extend_from_slice(&[4, 1]);
    let mut breakable = matrix.clone();
    breakable.push(60_f32.to_bits());
    let static_camera = vec![
        1_f32.to_bits(),
        2_f32.to_bits(),
        3_f32.to_bits(),
        60_f32.to_bits(),
        0.5_f32.to_bits(),
        1,
        0.25_f32.to_bits(),
        3,
        1,
        3,
    ];
    let fixtures = [
        (8_u32, matrix, 0_u32),
        (9, action, 1),
        (
            10,
            vec![60_f32.to_bits(), 1_f32.to_bits(), 2_f32.to_bits()],
            1,
        ),
        (11, breakable, 0),
        (12, static_camera, 1),
        (13, vec![3], 1),
        (14, Vec::new(), 0),
        (15, Vec::new(), 0),
    ];
    for (locator_type, data, triggers) in fixtures {
        let _json = assert_known(locator_type, &data, triggers)?;
    }
    Ok(())
}

#[test]
fn text_and_action_payloads_preserve_authored_fields() -> Result<(), String> {
    let script = assert_known(1, &words(b"car_wreck\0"), 1)?;
    if !script.contains("\"script\":\"car_wreck\"") {
        return Err(String::from("script text was not decoded"));
    }
    let mut action_data = words(b"object\0joint\0action\0\0");
    action_data.extend_from_slice(&[4, 1]);
    let action = assert_known(9, &action_data, 1)?;
    for field in [
        "\"object_name\":\"object\"",
        "\"joint_name\":\"joint\"",
        "\"action_name\":\"action\"",
        "\"button_input\":4",
        "\"should_transform\":true",
    ] {
        if !action.contains(field) {
            return Err(format!("action output omitted {field}"));
        }
    }
    Ok(())
}

#[test]
fn interior_and_static_camera_payloads_preserve_structured_fields()
-> Result<(), String> {
    let mut interior_data = words(b"school\0\0");
    interior_data.extend_from_slice(&[
        1_f32.to_bits(),
        2_f32.to_bits(),
        3_f32.to_bits(),
        4_f32.to_bits(),
        5_f32.to_bits(),
        6_f32.to_bits(),
        7_f32.to_bits(),
        8_f32.to_bits(),
        9_f32.to_bits(),
    ]);
    let interior = assert_known(7, &interior_data, 1)?;
    if !interior.contains("\"interior_file\":\"school\"")
        || !interior.contains("\"basis\":[[1,2,3],[4,5,6],[7,8,9]]")
    {
        return Err(String::from("interior fields were not decoded"));
    }
    let camera = assert_known(
        12,
        &[
            1_f32.to_bits(),
            2_f32.to_bits(),
            3_f32.to_bits(),
            60_f32.to_bits(),
            0.5_f32.to_bits(),
            1,
            0.25_f32.to_bits(),
            3,
            1,
            3,
        ],
        1,
    )?;
    for field in [
        "\"tracking\":true",
        "\"one_shot\":true",
        "\"disable_fov_lag\":true",
        "\"cut_in_out\":true",
        "\"car_only\":true",
        "\"on_foot_only\":true",
    ] {
        if !camera.contains(field) {
            return Err(format!("static camera output omitted {field}"));
        }
    }
    Ok(())
}

#[test]
fn byte_sized_locator_payloads_enforce_runtime_storage_bounds() {
    assert!(data_interpretation_json(5, &[0; 63], 0).is_some());
    assert!(data_interpretation_json(5, &[0; 64], 0).is_none());

    let mut interior = words(b"a\0");
    interior.extend_from_slice(&[
        1_f32.to_bits(),
        2_f32.to_bits(),
        3_f32.to_bits(),
        4_f32.to_bits(),
        5_f32.to_bits(),
        6_f32.to_bits(),
        7_f32.to_bits(),
        8_f32.to_bits(),
        9_f32.to_bits(),
    ]);
    interior.resize(63, 0);
    assert!(data_interpretation_json(7, &interior, 0).is_some());
    interior.push(0);
    assert!(data_interpretation_json(7, &interior, 0).is_none());

    let mut action = words(b"a\0b\0c\0");
    action.resize(63, 0);
    action.extend_from_slice(&[4, 1]);
    assert!(data_interpretation_json(9, &action, 0).is_some());
    action.insert(63, 0);
    assert!(data_interpretation_json(9, &action, 0).is_none());
}

#[test]
fn action_payload_preserves_an_authored_empty_first_string()
-> Result<(), String> {
    let mut action_data = words(b"\0joint\0action\0");
    action_data.extend_from_slice(&[4, 1]);
    let action = assert_known(9, &action_data, 0)?;
    for field in [
        "\"object_name\":\"\"",
        "\"joint_name\":\"joint\"",
        "\"action_name\":\"action\"",
    ] {
        if !action.contains(field) {
            return Err(format!("action output omitted {field}"));
        }
    }
    Ok(())
}

#[test]
fn unknown_locator_types_preserve_base_loader_behavior() -> Result<(), String> {
    for locator_type in [16_u32, 17, u32::MAX] {
        let json = data_interpretation_json(locator_type, &[7, 9], 0)
            .ok_or_else(|| {
                format!("locator type {locator_type} did not decode")
            })?;
        let _value = serde_json::from_str::<serde_json::Value>(&json).map_err(
            |error| format!("unknown locator emitted invalid JSON: {error}"),
        )?;
        for field in [
            "\"kind\":\"unknown\"",
            "\"loader_behavior\":\"base_locator\"",
            "\"ignored_data\":[7,9]",
        ] {
            if !json.contains(field) {
                return Err(format!("unknown locator output omitted {field}"));
            }
        }
    }
    Ok(())
}

#[test]
fn byte_oriented_locator_text_does_not_require_utf8() -> Result<(), String> {
    for locator_type in [1_u32, 5] {
        let json = assert_known(locator_type, &[0x0000_00ff], 0)?;
        if !json.contains('�') {
            return Err(format!("locator type {locator_type} lost lossy text"));
        }
    }

    let mut interior = vec![0x0000_00ff];
    interior.extend_from_slice(&[
        1_f32.to_bits(),
        2_f32.to_bits(),
        3_f32.to_bits(),
        4_f32.to_bits(),
        5_f32.to_bits(),
        6_f32.to_bits(),
        7_f32.to_bits(),
        8_f32.to_bits(),
        9_f32.to_bits(),
    ]);
    let interior_json = assert_known(7, &interior, 0)?;
    if !interior_json.contains('�') {
        return Err(String::from("interior locator lost lossy text"));
    }

    let mut action_bytes = vec![0xff, 0];
    action_bytes.extend_from_slice(b"joint\0action\0");
    let mut action = words(&action_bytes);
    action.extend_from_slice(&[4, 1]);
    let action_json = assert_known(9, &action, 0)?;
    if !action_json.contains("\"object_name\":\"�\"") {
        return Err(String::from("action locator lost lossy text"));
    }
    Ok(())
}

#[test]
fn breakable_camera_payload_is_runtime_dormant() -> Result<(), String> {
    for data in [Vec::new(), vec![1, 2, 3]] {
        let json = assert_known(11, &data, 0)?;
        for field in ["\"loader_behavior\":\"dormant\"", "\"ignored_data\":"] {
            if !json.contains(field) {
                return Err(format!("breakable-camera output omitted {field}"));
            }
        }
    }
    Ok(())
}

#[test]
fn invalid_declared_locator_payloads_fail_closed() {
    assert!(data_interpretation_json(6, &[], 2).is_none());
    assert!(data_interpretation_json(6, &[0, 1], 2).is_none());
    assert!(data_interpretation_json(8, &[0; 8], 0,).is_none());
    assert!(data_interpretation_json(9, &[0; 4], 0,).is_none());
}

// File:
//   - locator_tests.rs
// Path:
//   - src/p3d/src/adapters/driven/decoders/locator_tests.rs
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
//   - Synthetic regressions for every declared locator payload type.
// - Must-Not:
//   - Read game packages, external source trees, or extraction outputs.
// - Allows:
//   - Build word-packed payloads and validate deterministic decoder JSON.
// - Split-When:
//   - One locator family requires an independent fixture framework.
// - Merge-When:
//   - The production decoder can prove the same contracts without fixtures.
// - Summary:
//   - Complete known-type locator decoder regression suite.
// - Description:
//   - Prevents known locator payloads from regressing to fallback output.
// - Usage:
//   - Compiled only with the P3D crate test target.
// - Defaults:
//   - Fixtures contain synthetic values and no game package bytes.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Synthetic regressions for complete locator payload interpretation.

use super::{data_interpretation_json, type_name};

/// Convert byte text into little-endian words for fixtures.
fn words(value: &[u8]) -> Vec<u32> {
    value
        .chunks(4)
        .map(
            |chunk| {
                let mut bytes = [0_u8; 4];
                for (target, source) in bytes
                    .iter_mut()
                    .zip(chunk)
                {
                    *target = *source;
                }
                u32::from_le_bytes(bytes)
            },
        )
        .collect()
}

/// Assert one payload decodes to valid JSON without fallback markers.
fn assert_known(
    locator_type: u32,
    data: &[u32],
    num_triggers: u32,
) -> Result<String, String> {
    let json = data_interpretation_json(
        locator_type,
        data,
        num_triggers,
    )
    .ok_or_else(
        || format!("known locator type {locator_type} did not decode"),
    )?;
    let _value = serde_json::from_str::<serde_json::Value>(&json).map_err(
        |error| {
            format!("locator type {locator_type} emitted invalid JSON: {error}")
        },
    )?;
    if json.contains("\"kind\":\"unknown\"") {
        return Err(
            format!("locator type {locator_type} used an unknown kind"),
        );
    }
    Ok(json)
}

#[test]
fn every_declared_locator_type_has_a_stable_name() {
    for locator_type in 0_u32..=15 {
        assert!(type_name(locator_type).is_some());
    }
    assert!(type_name(16).is_none());
}

#[test]
fn locator_types_zero_through_seven_have_typed_interpretations()
-> Result<(), String> {
    let text = words(b"script\0");
    let zone = words(b"l1z1.p3d\0");
    let mut interior = words(b"interior\0\0\0\0");
    interior.extend_from_slice(
        &[
            1_f32.to_bits(),
            2_f32.to_bits(),
            3_f32.to_bits(),
            4_f32.to_bits(),
            5_f32.to_bits(),
            6_f32.to_bits(),
            7_f32.to_bits(),
            8_f32.to_bits(),
            9_f32.to_bits(),
        ],
    );
    let fixtures = [
        (
            0_u32,
            vec![
                2, 7,
            ],
            1_u32,
        ),
        (
            1, text, 1,
        ),
        (
            2,
            Vec::new(),
            0,
        ),
        (
            3,
            vec![
                0_f32.to_bits(),
                1,
            ],
            0,
        ),
        (
            4,
            Vec::new(),
            1,
        ),
        (
            5, zone, 1,
        ),
        (
            6,
            vec![1],
            2,
        ),
        (
            7, interior, 1,
        ),
    ];
    for (locator_type, data, triggers) in fixtures {
        let _json = assert_known(
            locator_type,
            &data,
            triggers,
        )?;
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
    action.extend_from_slice(
        &[
            4, 1,
        ],
    );
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
        (
            8_u32, matrix, 0_u32,
        ),
        (
            9, action, 1,
        ),
        (
            10,
            vec![
                60_f32.to_bits(),
                1_f32.to_bits(),
                2_f32.to_bits(),
            ],
            1,
        ),
        (
            11, breakable, 0,
        ),
        (
            12,
            static_camera,
            1,
        ),
        (
            13,
            vec![3],
            1,
        ),
        (
            14,
            Vec::new(),
            0,
        ),
        (
            15,
            Vec::new(),
            0,
        ),
    ];
    for (locator_type, data, triggers) in fixtures {
        let _json = assert_known(
            locator_type,
            &data,
            triggers,
        )?;
    }
    Ok(())
}

#[test]
fn text_and_action_payloads_preserve_authored_fields() -> Result<(), String> {
    let script = assert_known(
        1,
        &words(b"car_wreck\0"),
        1,
    )?;
    if !script.contains("\"script\":\"car_wreck\"") {
        return Err(String::from("script text was not decoded"));
    }
    let mut action_data = words(b"object\0joint\0action\0\0");
    action_data.extend_from_slice(
        &[
            4, 1,
        ],
    );
    let action = assert_known(
        9,
        &action_data,
        1,
    )?;
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
    interior_data.extend_from_slice(
        &[
            1_f32.to_bits(),
            2_f32.to_bits(),
            3_f32.to_bits(),
            4_f32.to_bits(),
            5_f32.to_bits(),
            6_f32.to_bits(),
            7_f32.to_bits(),
            8_f32.to_bits(),
            9_f32.to_bits(),
        ],
    );
    let interior = assert_known(
        7,
        &interior_data,
        1,
    )?;
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
fn invalid_or_undeclared_locator_payloads_fail_closed() {
    assert!(
        data_interpretation_json(
            8, &[0; 8], 0,
        )
        .is_none()
    );
    assert!(
        data_interpretation_json(
            9, &[0; 4], 0,
        )
        .is_none()
    );
    assert!(
        data_interpretation_json(
            16,
            &[],
            0,
        )
        .is_none()
    );
}

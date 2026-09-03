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
//   - Expression loose unit tests.
// - Must-Not:
//   - Own production behavior or broaden the tested API surface.
// - Allows:
//   - Private assertions for the owning source module.
// - Split-When:
//   - Split when an independent fixture family gains separate ownership.
// - Merge-When:
//   - Merge when another test module owns the identical evidence.
// - Summary:
//   - Expression loose unit tests.
// - Description:
//   - Preserves unit-test access through a test-only path module.
// - Usage:
//   - Included only by the owning source module under cfg(test).
// - Defaults:
//   - Assertions fail explicitly.
//

//! Expression loose unit tests.

use super::*;

#[test]
fn expression_mixer_rejects_invalid_utf8_names() {
    let mut chunk = Vec::new();
    chunk.extend_from_slice(&0x0002_1002_u32.to_le_bytes());
    chunk.extend_from_slice(&26_u32.to_le_bytes());
    chunk.extend_from_slice(&26_u32.to_le_bytes());
    chunk.extend_from_slice(&0_u32.to_le_bytes());
    chunk.extend_from_slice(&[1, 0xff]);
    chunk.extend_from_slice(&0_u32.to_le_bytes());
    chunk.extend_from_slice(&[1, b't', 1, b'g']);

    assert!(
        vertex_expression_json("vertex_expression_mixer", &chunk,).is_none()
    );
}

#[test]
fn expression_mixer_preserves_declared_trailing_null_names()
-> Result<(), String> {
    let mut chunk = Vec::new();
    chunk.extend_from_slice(&0x0002_1002_u32.to_le_bytes());
    chunk.extend_from_slice(&27_u32.to_le_bytes());
    chunk.extend_from_slice(&27_u32.to_le_bytes());
    chunk.extend_from_slice(&0_u32.to_le_bytes());
    chunk.extend_from_slice(&[2, b'n', 0]);
    chunk.extend_from_slice(&0_u32.to_le_bytes());
    chunk.extend_from_slice(&[1, b't', 1, b'g']);

    let Some(json) = vertex_expression_json("vertex_expression_mixer", &chunk)
    else {
        return Err(String::from("valid expression mixer should decode"));
    };
    if !json.contains(r#""name":"n\u0000""#) {
        return Err(format!("trailing null was not preserved: {json:?}"));
    }
    Ok(())
}

#[test]
fn expression_group_rejects_missing_declared_children() {
    let mut chunk = Vec::new();
    chunk.extend_from_slice(&0x0002_1001_u32.to_le_bytes());
    chunk.extend_from_slice(&28_u32.to_le_bytes());
    chunk.extend_from_slice(&28_u32.to_le_bytes());
    chunk.extend_from_slice(&0_u32.to_le_bytes());
    chunk.extend_from_slice(&[1, b'n', 1, b't']);
    chunk.extend_from_slice(&1_u32.to_le_bytes());
    chunk.extend_from_slice(&0_u32.to_le_bytes());

    assert!(
        vertex_expression_json("vertex_expression_group", &chunk,).is_none()
    );
}

#[test]
fn expression_counts_reject_impossible_arrays() {
    let mut group = Vec::new();
    group.extend_from_slice(&0x0002_1001_u32.to_le_bytes());
    group.extend_from_slice(&26_u32.to_le_bytes());
    group.extend_from_slice(&26_u32.to_le_bytes());
    group.extend_from_slice(&0_u32.to_le_bytes());
    group.extend_from_slice(&[1, b'n', 1, b't']);
    group.extend_from_slice(&u32::MAX.to_le_bytes());
    assert!(vertex_expression_json("vertex_expression_group", &group).is_none());

    let mut curve = Vec::new();
    curve.extend_from_slice(&0x0002_1000_u32.to_le_bytes());
    curve.extend_from_slice(&22_u32.to_le_bytes());
    curve.extend_from_slice(&22_u32.to_le_bytes());
    curve.extend_from_slice(&0_u32.to_le_bytes());
    curve.extend_from_slice(&[1, b'e']);
    curve.extend_from_slice(&u32::MAX.to_le_bytes());
    assert!(decode_expression_json(&curve).is_none());
}

#[test]
fn expression_key_format_preserves_f32_roundtrip() {
    let value = f32::from_bits(0x3f80_0001);

    assert_eq!(format_f32(value), value.to_string());
}

#[test]
fn expression_curve_rejects_version_and_nonfinite_key_drift() {
    fn curve(version: u32, key: f32) -> Vec<u8> {
        let mut chunk = Vec::new();
        chunk.extend_from_slice(&0x0002_1000_u32.to_le_bytes());
        chunk.extend_from_slice(&30_u32.to_le_bytes());
        chunk.extend_from_slice(&30_u32.to_le_bytes());
        chunk.extend_from_slice(&version.to_le_bytes());
        chunk.extend_from_slice(&[1, b'e']);
        chunk.extend_from_slice(&1_u32.to_le_bytes());
        chunk.extend_from_slice(&key.to_le_bytes());
        chunk.extend_from_slice(&7_u32.to_le_bytes());
        chunk
    }

    assert!(decode_expression_json(&curve(0, 0.5_f32)).is_some());
    assert!(decode_expression_json(&curve(1, 0.5_f32)).is_none());
    assert!(decode_expression_json(&curve(0, f32::NAN)).is_none());
}

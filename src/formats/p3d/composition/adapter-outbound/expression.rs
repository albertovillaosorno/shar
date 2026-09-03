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
//   - Expression outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Expression outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for p3d.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Expression outbound adapter.

use super::decoders::reader::read_u32;
use super::json::{escape_json, render_f32};

/// Decode vertex expression group or mixer JSON.
pub(super) fn vertex_expression_json(
    kind: &str,
    chunk: &[u8],
) -> Option<String> {
    match read_chunk_header(chunk, 0)?.0 {
        0x0002_1001 => decode_expression_group_json(kind, chunk),
        0x0002_1002 => decode_expression_mixer_json(kind, chunk),
        _ => None,
    }
}

/// Decode expression groups with stages and child expression curves.
fn decode_expression_group_json(kind: &str, chunk: &[u8]) -> Option<String> {
    let (id, header_size, total_size) = read_chunk_header(chunk, 0)?;
    if id != 0x0002_1001 || total_size != chunk.len() {
        return None;
    }
    let mut cursor = 12;
    let version = read_u32_advance(chunk, &mut cursor)?;
    if version != 0 {
        return None;
    }
    let name = read_pstring_advance(chunk, &mut cursor)?;
    let target_name = read_pstring_advance(chunk, &mut cursor)?;
    let count = usize::try_from(read_u32_advance(chunk, &mut cursor)?).ok()?;
    if !fixed_count_bytes_fit(cursor, header_size, count, 4)? {
        return None;
    }
    let mut stages = Vec::new();
    for _ in 0..count {
        let stage = read_u32_advance(chunk, &mut cursor)?;
        if stage > 2 {
            return None;
        }
        stages.push(stage.to_string());
    }
    if cursor != header_size {
        return None;
    }
    let mut expressions = Vec::new();
    let mut child_cursor = header_size;
    while child_cursor < total_size {
        let (_, _, child_total) = read_chunk_header(chunk, child_cursor)?;
        let end = child_cursor.checked_add(child_total)?;
        let child = chunk.get(child_cursor..end)?;
        expressions.push(decode_expression_json(child)?);
        child_cursor = end;
    }
    if expressions.len() != count {
        return None;
    }
    Some(format!(
        concat!(
            "{{\"schema\":\"{}\",\"version\":{},",
            "\"name\":\"{}\",\"target_name\":\"{}\",",
            "\"num_expressions\":{},\"stages\":[{}],",
            "\"expressions\":[{}]}}\n"
        ),
        escape_json(kind),
        version,
        escape_json(&name),
        escape_json(&target_name),
        count,
        stages.join(","),
        expressions.join(",")
    ))
}

/// Decode one expression curve with keyed values and vertex indices.
fn decode_expression_json(chunk: &[u8]) -> Option<String> {
    let (id, header_size, total_size) = read_chunk_header(chunk, 0)?;
    if id != 0x0002_1000 || total_size != chunk.len() {
        return None;
    }
    let mut cursor = 12;
    let version = read_u32_advance(chunk, &mut cursor)?;
    if version != 0 {
        return None;
    }
    let name = read_pstring_advance(chunk, &mut cursor)?;
    let count = usize::try_from(read_u32_advance(chunk, &mut cursor)?).ok()?;
    if count == 0 || !fixed_count_bytes_fit(cursor, header_size, count, 8)? {
        return None;
    }
    let mut keys = Vec::new();
    let mut previous_key = None;
    for _ in 0..count {
        let key = read_f32_advance(chunk, &mut cursor)?;
        if !key.is_finite()
            || previous_key.is_some_and(|previous| key < previous)
        {
            return None;
        }
        previous_key = Some(key);
        keys.push(format_f32(key));
    }
    let mut indices = Vec::new();
    for _ in 0..count {
        indices.push(read_u32_advance(chunk, &mut cursor)?.to_string());
    }
    if cursor != header_size || header_size != total_size {
        return None;
    }
    Some(format!(
        "{{\"version\":{},\"name\":\"{}\",\"num_keys\":{},\"keys\":[{}],\"\
             indices\":[{}]}}",
        version,
        escape_json(&name),
        count,
        keys.join(","),
        indices.join(",")
    ))
}

/// Decode expression mixer metadata linking targets to expression groups.
fn decode_expression_mixer_json(kind: &str, chunk: &[u8]) -> Option<String> {
    let (id, header_size, total_size) = read_chunk_header(chunk, 0)?;
    if id != 0x0002_1002 || total_size != chunk.len() {
        return None;
    }
    let mut cursor = 12;
    let version = read_u32_advance(chunk, &mut cursor)?;
    if version != 0 {
        return None;
    }
    let name = read_pstring_advance(chunk, &mut cursor)?;
    let mixer_type = read_u32_advance(chunk, &mut cursor)?;
    let target_name = read_pstring_advance(chunk, &mut cursor)?;
    let expression_group_name = read_pstring_advance(chunk, &mut cursor)?;
    if cursor != header_size || header_size != total_size {
        return None;
    }
    Some(format!(
        concat!(
            "{{\"schema\":\"{}\",\"version\":{},",
            "\"name\":\"{}\",\"type\":{},",
            "\"target_name\":\"{}\",",
            "\"expression_group_name\":\"{}\"}}\n"
        ),
        escape_json(kind),
        version,
        escape_json(&name),
        mixer_type,
        escape_json(&target_name),
        escape_json(&expression_group_name)
    ))
}

/// Read a nested chunk header and validate structural sizes.
fn read_chunk_header(
    bytes: &[u8],
    cursor: usize,
) -> Option<(u32, usize, usize)> {
    let id = read_u32(bytes, cursor)?;
    let header_size =
        usize::try_from(read_u32(bytes, cursor.checked_add(4)?)?).ok()?;
    let total_size =
        usize::try_from(read_u32(bytes, cursor.checked_add(8)?)?).ok()?;
    if header_size < 12 || total_size < header_size {
        return None;
    }
    Some((id, header_size, total_size))
}

/// Validate fixed-width count arrays against the remaining chunk header.
fn fixed_count_bytes_fit(
    cursor: usize,
    end: usize,
    count: usize,
    width: usize,
) -> Option<bool> {
    Some(count.checked_mul(width)? == end.checked_sub(cursor)?)
}

/// Format floating point key values deterministically for JSON output.
fn format_f32(value: f32) -> String {
    let finite_rendering = value.to_string();
    render_f32(value, finite_rendering)
}

/// Read a little-endian u32 and advance the parser cursor.
fn read_u32_advance(bytes: &[u8], cursor: &mut usize) -> Option<u32> {
    let value = read_u32(bytes, *cursor)?;
    *cursor = cursor.checked_add(4)?;
    Some(value)
}

/// Read a little-endian f32 and advance the parser cursor.
fn read_f32_advance(bytes: &[u8], cursor: &mut usize) -> Option<f32> {
    let slice = bytes.get(*cursor..cursor.checked_add(4)?)?;
    let array: [u8; 4] = slice.try_into().ok()?;
    *cursor = cursor.checked_add(4)?;
    Some(f32::from_le_bytes(array))
}

/// Read a `Pure3D` Pascal string and advance the parser cursor.
fn read_pstring_advance(bytes: &[u8], cursor: &mut usize) -> Option<String> {
    let length = usize::from(*bytes.get(*cursor)?);
    *cursor = cursor.checked_add(1)?;
    let end = cursor.checked_add(length)?;
    let raw = bytes.get(*cursor..end)?;
    *cursor = end;
    Some(std::str::from_utf8(raw).ok()?.to_owned())
}
#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../tests/formats/p3d/unit/adapter-outbound/expression/loose_tests.rs"]
mod loose_tests;

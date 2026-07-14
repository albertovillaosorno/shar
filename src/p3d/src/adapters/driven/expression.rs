// File:
//   - expression.rs
// Path:
//   - src/p3d/src/adapters/driven/expression.rs
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
//   - The p3d adapter boundary for adapters driven expression.
// - Must-Not:
//   - Change domain semantics or bypass application and port contracts.
// - Allows:
//   - Filesystem, JSON, CLI, Blender, or serialization work behind explicit
//   - ports.
// - Split-When:
//   - Split when expression contains two independently testable contracts.
// - Merge-When:
//   - Another p3d module owns the same adapters boundary with no distinct
//   - invariant.
// - Summary:
//   - Vertex expression payload decoder.
// - Description:
//   - Defines expression data and behavior for p3d adapters driven.
// - Usage:
//   - Constructed by composition roots or tests and passed behind port traits.
// - Defaults:
//   - Adapter defaults stay local to the adapter constructor or config.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - true
//   - Reason: src/p3d/src/adapters/driven/expression.rs has 403 effective lines
//   - after the required header and remains cohesive until a focused split
//   - lands.
//

//! Vertex expression payload decoder.
//!
//! Vertex expression chunks are keyed animation payloads, so they live outside
//! the package writer. Keeping this parser isolated prevents formatting helpers
//! from being mistaken for filesystem or package-writing responsibilities.
use super::decoders::reader::read_u32;
use super::json::{escape_json, render_f32};

/// Decode vertex expression group or mixer JSON.
pub(super) fn vertex_expression_json(
    kind: &str,
    chunk: &[u8],
) -> Option<String> {
    match read_chunk_header(
        chunk, 0,
    )?
    .0
    {
        0x0002_1001 => decode_expression_group_json(
            kind, chunk,
        ),
        0x0002_1002 => decode_expression_mixer_json(
            kind, chunk,
        ),
        _ => None,
    }
}

/// Decode expression groups with stages and child expression curves.
fn decode_expression_group_json(
    kind: &str,
    chunk: &[u8],
) -> Option<String> {
    let (id, header_size, total_size) = read_chunk_header(
        chunk, 0,
    )?;
    if id != 0x0002_1001 || total_size != chunk.len() {
        return None;
    }
    let mut cursor = 12;
    let version = read_u32_advance(
        chunk,
        &mut cursor,
    )?;
    let name = read_pstring_advance(
        chunk,
        &mut cursor,
    )?;
    let target_name = read_pstring_advance(
        chunk,
        &mut cursor,
    )?;
    let count = usize::try_from(
        read_u32_advance(
            chunk,
            &mut cursor,
        )?,
    )
    .ok()?;
    let mut stages = Vec::new();
    for _ in 0..count {
        stages.push(
            read_u32_advance(
                chunk,
                &mut cursor,
            )?
            .to_string(),
        );
    }
    if cursor != header_size {
        return None;
    }
    let mut expressions = Vec::new();
    let mut child_cursor = header_size;
    while child_cursor < total_size {
        let (_, _, child_total) = read_chunk_header(
            chunk,
            child_cursor,
        )?;
        let end = child_cursor.checked_add(child_total)?;
        let child = chunk.get(child_cursor..end)?;
        expressions.push(decode_expression_json(child)?);
        child_cursor = end;
    }
    if expressions.len() != count {
        return None;
    }
    Some(
        format!(
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
        ),
    )
}

/// Decode one expression curve with keyed values and vertex indices.
fn decode_expression_json(chunk: &[u8]) -> Option<String> {
    let (id, header_size, total_size) = read_chunk_header(
        chunk, 0,
    )?;
    if id != 0x0002_1000 || total_size != chunk.len() {
        return None;
    }
    let mut cursor = 12;
    let version = read_u32_advance(
        chunk,
        &mut cursor,
    )?;
    let name = read_pstring_advance(
        chunk,
        &mut cursor,
    )?;
    let count = usize::try_from(
        read_u32_advance(
            chunk,
            &mut cursor,
        )?,
    )
    .ok()?;
    let mut keys = Vec::new();
    for _ in 0..count {
        keys.push(
            format_f32(
                read_f32_advance(
                    chunk,
                    &mut cursor,
                )?,
            ),
        );
    }
    let mut indices = Vec::new();
    for _ in 0..count {
        indices.push(
            read_u32_advance(
                chunk,
                &mut cursor,
            )?
            .to_string(),
        );
    }
    if cursor != header_size || header_size != total_size {
        return None;
    }
    Some(
        format!(
            "{{\"version\":{},\"name\":\"{}\",\"num_keys\":{},\"keys\":[{}],\"\
             indices\":[{}]}}",
            version,
            escape_json(&name),
            count,
            keys.join(","),
            indices.join(",")
        ),
    )
}

/// Decode expression mixer metadata linking targets to expression groups.
fn decode_expression_mixer_json(
    kind: &str,
    chunk: &[u8],
) -> Option<String> {
    let (id, header_size, total_size) = read_chunk_header(
        chunk, 0,
    )?;
    if id != 0x0002_1002 || total_size != chunk.len() {
        return None;
    }
    let mut cursor = 12;
    let version = read_u32_advance(
        chunk,
        &mut cursor,
    )?;
    let name = read_pstring_advance(
        chunk,
        &mut cursor,
    )?;
    let mixer_type = read_u32_advance(
        chunk,
        &mut cursor,
    )?;
    let target_name = read_pstring_advance(
        chunk,
        &mut cursor,
    )?;
    let expression_group_name = read_pstring_advance(
        chunk,
        &mut cursor,
    )?;
    if cursor != header_size || header_size != total_size {
        return None;
    }
    Some(
        format!(
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
        ),
    )
}

/// Read a nested chunk header and validate structural sizes.
fn read_chunk_header(
    bytes: &[u8],
    cursor: usize,
) -> Option<(
    u32,
    usize,
    usize,
)> {
    let id = read_u32(
        bytes, cursor,
    )?;
    let header_size = usize::try_from(
        read_u32(
            bytes,
            cursor.checked_add(4)?,
        )?,
    )
    .ok()?;
    let total_size = usize::try_from(
        read_u32(
            bytes,
            cursor.checked_add(8)?,
        )?,
    )
    .ok()?;
    if header_size < 12 || total_size < header_size {
        return None;
    }
    Some(
        (
            id,
            header_size,
            total_size,
        ),
    )
}

/// Format floating point key values deterministically for JSON output.
fn format_f32(value: f32) -> String {
    let finite_rendering = value.to_string();
    render_f32(
        value,
        finite_rendering,
    )
}

/// Read a little-endian u32 and advance the parser cursor.
fn read_u32_advance(
    bytes: &[u8],
    cursor: &mut usize,
) -> Option<u32> {
    let value = read_u32(
        bytes, *cursor,
    )?;
    *cursor = cursor.checked_add(4)?;
    Some(value)
}

/// Read a little-endian f32 and advance the parser cursor.
fn read_f32_advance(
    bytes: &[u8],
    cursor: &mut usize,
) -> Option<f32> {
    let slice = bytes.get(*cursor..cursor.checked_add(4)?)?;
    let array: [u8; 4] = slice
        .try_into()
        .ok()?;
    *cursor = cursor.checked_add(4)?;
    Some(f32::from_le_bytes(array))
}

/// Read a `Pure3D` Pascal string and advance the parser cursor.
fn read_pstring_advance(
    bytes: &[u8],
    cursor: &mut usize,
) -> Option<String> {
    let length = usize::from(*bytes.get(*cursor)?);
    *cursor = cursor.checked_add(1)?;
    let end = cursor.checked_add(length)?;
    let raw = bytes.get(*cursor..end)?;
    *cursor = end;
    Some(
        std::str::from_utf8(raw)
            .ok()?
            .to_owned(),
    )
}

#[cfg(test)]
#[test]
fn expression_mixer_rejects_invalid_utf8_names() {
    let mut chunk = Vec::new();
    chunk.extend_from_slice(&0x0002_1002_u32.to_le_bytes());
    chunk.extend_from_slice(&26_u32.to_le_bytes());
    chunk.extend_from_slice(&26_u32.to_le_bytes());
    chunk.extend_from_slice(&1_u32.to_le_bytes());
    chunk.extend_from_slice(
        &[
            1, 0xff,
        ],
    );
    chunk.extend_from_slice(&0_u32.to_le_bytes());
    chunk.extend_from_slice(
        &[
            1, b't', 1, b'g',
        ],
    );

    assert!(
        vertex_expression_json(
            "vertex_expression_mixer",
            &chunk,
        )
        .is_none()
    );
}

#[cfg(test)]
#[test]
fn expression_mixer_preserves_declared_trailing_null_names()
-> Result<(), String> {
    let mut chunk = Vec::new();
    chunk.extend_from_slice(&0x0002_1002_u32.to_le_bytes());
    chunk.extend_from_slice(&27_u32.to_le_bytes());
    chunk.extend_from_slice(&27_u32.to_le_bytes());
    chunk.extend_from_slice(&1_u32.to_le_bytes());
    chunk.extend_from_slice(
        &[
            2, b'n', 0,
        ],
    );
    chunk.extend_from_slice(&0_u32.to_le_bytes());
    chunk.extend_from_slice(
        &[
            1, b't', 1, b'g',
        ],
    );

    let Some(json) = vertex_expression_json(
        "vertex_expression_mixer",
        &chunk,
    ) else {
        return Err(String::from("valid expression mixer should decode"));
    };
    if !json.contains(r#""name":"n\u0000""#) {
        return Err(format!("trailing null was not preserved: {json:?}"));
    }
    Ok(())
}

#[cfg(test)]
#[test]
fn expression_group_rejects_missing_declared_children() {
    let mut chunk = Vec::new();
    chunk.extend_from_slice(&0x0002_1001_u32.to_le_bytes());
    chunk.extend_from_slice(&28_u32.to_le_bytes());
    chunk.extend_from_slice(&28_u32.to_le_bytes());
    chunk.extend_from_slice(&1_u32.to_le_bytes());
    chunk.extend_from_slice(
        &[
            1, b'n', 1, b't',
        ],
    );
    chunk.extend_from_slice(&1_u32.to_le_bytes());
    chunk.extend_from_slice(&0_u32.to_le_bytes());

    assert!(
        vertex_expression_json(
            "vertex_expression_group",
            &chunk,
        )
        .is_none()
    );
}

#[cfg(test)]
#[test]
fn expression_key_format_preserves_f32_roundtrip() {
    let value = f32::from_bits(0x3f80_0001);

    assert_eq!(
        format_f32(value),
        value.to_string()
    );
}

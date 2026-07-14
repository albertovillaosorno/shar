// File:
//   - intersect.rs
// Path:
//   - src/p3d/src/adapters/driven/decoders/intersect.rs
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
//   - The p3d adapter boundary for adapters driven decoders intersect.
// - Must-Not:
//   - Change domain semantics or bypass application and port contracts.
// - Allows:
//   - Filesystem, JSON, CLI, Blender, or serialization work behind explicit
//   - ports.
// - Split-When:
//   - Split when intersect contains two independently testable contracts.
// - Merge-When:
//   - Another p3d module owns the same adapters boundary with no distinct
//   - invariant.
// - Summary:
//   - Intersect DSG decoder for collision mesh recovery.
// - Description:
//   - Defines intersect data and behavior for p3d adapters driven decoders.
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
//   - Reason: src/p3d/src/adapters/driven/decoders/intersect.rs has 423
//   - effective
//   - lines after the required header and remains cohesive until a focused
//   - split
//   - lands.
//

//! Intersect DSG decoder for collision mesh recovery.
//!
//! The loader consumes one count-prefixed index list, two count-prefixed vector
//! lists, and optional bounds/terrain children. The decoder keeps those count
//! checks beside the reads so corrupt packages fail closed and keep raw bytes.
use super::reader::{Reader, SubChunk, read_u32, subchunks};
use crate::adapters::driven::json::render_f32;

/// Intersect DSG chunk id.
const INTERSECT_DSG: u32 = 0x03f0_0003;
/// Bounding-box child chunk id.
const BBOX: u32 = 0x0001_0003;
/// Bounding-sphere child chunk id.
const BSPHERE: u32 = 0x0001_0004;
/// Terrain-type child chunk id.
const TERRAIN_TYPE: u32 = 0x0300_000e;

/// Decode an intersect DSG payload into full array JSON.
pub fn dsg_json(chunk: &[u8]) -> Option<String> {
    let (id, header_size, total_size) = chunk_bounds(chunk)?;
    if id != INTERSECT_DSG {
        return None;
    }
    let mut reader = Reader::new(
        chunk, 12,
    );
    let indices = read_u32_array(&mut reader)?;
    let positions = read_vec3_array(&mut reader)?;
    let normals = read_vec3_array(&mut reader)?;
    if reader.pos() != header_size {
        return None;
    }
    let children = subchunks(
        chunk,
        header_size,
        total_size,
    )?;
    let mut bounds = Vec::new();
    let mut terrain_types = Vec::new();
    for child in children {
        match child.id {
            BBOX => bounds.push(
                decode_bbox(
                    chunk, &child,
                )?,
            ),
            BSPHERE => bounds.push(
                decode_bsphere(
                    chunk, &child,
                )?,
            ),
            TERRAIN_TYPE => terrain_types.push(
                decode_terrain_type(
                    chunk, &child,
                )?,
            ),
            _ => return None,
        }
    }
    Some(
        format!(
            concat!(
                "{{\"schema\":\"intersect_dsg\",",
                "\"num_indices\":{},\"indices\":[{}],",
                "\"num_positions\":{},\"positions\":[{}],",
                "\"num_normals\":{},\"normals\":[{}],",
                "\"bounds\":[{}],\"terrain_types\":[{}]}}\n"
            ),
            indices.len(),
            indices
                .iter()
                .map(u32::to_string)
                .collect::<Vec<_>>()
                .join(","),
            positions.len(),
            positions.join(","),
            normals.len(),
            normals.join(","),
            bounds.join(","),
            terrain_types.join(",")
        ),
    )
}

/// Reads a count-prefixed integer list for triangle indices.
fn read_u32_array(reader: &mut Reader<'_>) -> Option<Vec<u32>> {
    let count = usize::try_from(reader.u32()?).ok()?;
    let mut values = Vec::new();
    for _ in 0..count {
        values.push(reader.u32()?);
    }
    Some(values)
}

/// Reads a count-prefixed vector list for positions and normals.
fn read_vec3_array(reader: &mut Reader<'_>) -> Option<Vec<String>> {
    let count = usize::try_from(reader.u32()?).ok()?;
    let mut values = Vec::new();
    for _ in 0..count {
        values.push(read_vec3(reader)?);
    }
    Some(values)
}

/// Formats one vector as a JSON array without a serializer dependency.
fn read_vec3(reader: &mut Reader<'_>) -> Option<String> {
    let x = reader.f32()?;
    let y = reader.f32()?;
    let z = reader.f32()?;
    Some(
        format!(
            "[{},{},{}]",
            fmt_f32(x),
            fmt_f32(y),
            fmt_f32(z)
        ),
    )
}

/// Decodes bounding boxes so collision mesh bounds are preserved.
fn decode_bbox(
    chunk: &[u8],
    child: &SubChunk,
) -> Option<String> {
    let mut reader = Reader::new(
        chunk,
        child.data_offset(),
    );
    let min = read_vec3(&mut reader)?;
    let max = read_vec3(&mut reader)?;
    let position_mismatch = reader.pos() != child.header_end();
    let child_size_mismatch = child.header_end() != child.end();
    if position_mismatch || child_size_mismatch {
        return None;
    }
    Some(format!("{{\"kind\":\"bbox\",\"min\":{min},\"max\":{max}}}"))
}

/// Decodes bounding spheres so collision mesh bounds are preserved.
fn decode_bsphere(
    chunk: &[u8],
    child: &SubChunk,
) -> Option<String> {
    let mut reader = Reader::new(
        chunk,
        child.data_offset(),
    );
    let centre = read_vec3(&mut reader)?;
    let radius = reader.f32()?;
    let position_mismatch = reader.pos() != child.header_end();
    let child_size_mismatch = child.header_end() != child.end();
    if position_mismatch || child_size_mismatch {
        return None;
    }
    Some(
        format!(
            "{{\"kind\":\"bsphere\",\"centre\":{},\"radius\":{}}}",
            centre,
            fmt_f32(radius)
        ),
    )
}

/// Decodes terrain-type bytes because they affect collision behavior.
fn decode_terrain_type(
    chunk: &[u8],
    child: &SubChunk,
) -> Option<String> {
    let mut reader = Reader::new(
        chunk,
        child.data_offset(),
    );
    let version = reader.u32()?;
    let count = usize::try_from(reader.u32()?).ok()?;
    let start = reader.pos();
    let end = start.checked_add(count)?;
    let types = chunk.get(start..end)?;
    if end != child.header_end() || child.header_end() != child.end() {
        return None;
    }
    Some(
        format!(
            "{{\"version\":{},\"num_types\":{},\"types\":[{}]}}",
            version,
            count,
            types
                .iter()
                .map(u8::to_string)
                .collect::<Vec<_>>()
                .join(",")
        ),
    )
}

/// Reads chunk bounds so malformed sizes fail closed.
fn chunk_bounds(
    chunk: &[u8]
) -> Option<(
    u32,
    usize,
    usize,
)> {
    let id = read_u32(
        chunk, 0,
    )?;
    let header_size = usize::try_from(
        read_u32(
            chunk, 4,
        )?,
    )
    .ok()?;
    let total_size = usize::try_from(
        read_u32(
            chunk, 8,
        )?,
    )
    .ok()?;
    if header_size < 12 || total_size < header_size || total_size > chunk.len()
    {
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

/// Formats floats consistently for deterministic JSON tests.
fn fmt_f32(value: f32) -> String {
    let finite_rendering = if value.fract() == 0.0 {
        format!("{value:.1}")
    } else {
        value.to_string()
    };
    render_f32(
        value,
        finite_rendering,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Builds a synthetic chunk for fail-closed decoder tests.
    fn chunk(
        id: u32,
        fields: Vec<u8>,
        children: Vec<Vec<u8>>,
    ) -> Option<Vec<u8>> {
        let header_size = 12_usize.checked_add(fields.len())?;
        let child_size = children
            .iter()
            .map(Vec::len)
            .try_fold(
                0_usize,
                usize::checked_add,
            )?;
        let total_size = header_size.checked_add(child_size)?;
        let mut out = Vec::new();
        out.extend_from_slice(&id.to_le_bytes());
        out.extend_from_slice(
            &u32::try_from(header_size)
                .ok()?
                .to_le_bytes(),
        );
        out.extend_from_slice(
            &u32::try_from(total_size)
                .ok()?
                .to_le_bytes(),
        );
        out.extend(fields);
        for child in children {
            out.extend(child);
        }
        Some(out)
    }

    /// Builds a little-endian integer fixture field.
    fn u32_field(value: u32) -> Vec<u8> {
        value
            .to_le_bytes()
            .to_vec()
    }

    /// Builds a little-endian float fixture field.
    fn f32_field(value: f32) -> Vec<u8> {
        value
            .to_le_bytes()
            .to_vec()
    }

    /// Builds a vector fixture field.
    fn vec3(
        x: f32,
        y: f32,
        z: f32,
    ) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(&f32_field(x));
        out.extend_from_slice(&f32_field(y));
        out.extend_from_slice(&f32_field(z));
        out
    }

    /// Converts optional fixture construction into a test error.
    fn require<T>(
        value: Option<T>,
        context: &str,
    ) -> Result<T, String> {
        value.ok_or_else(|| String::from(context))
    }

    /// Checks useful JSON fields without panicking inside tests.
    fn require_json(
        json: &str,
        needle: &str,
        context: &str,
    ) -> Result<(), String> {
        if json.contains(needle) {
            Ok(())
        } else {
            Err(String::from(context))
        }
    }

    /// Builds an intersect fixture with arrays and child chunks.
    fn intersect_fixture() -> Option<Vec<u8>> {
        let mut fields = Vec::new();
        fields.extend_from_slice(&u32_field(3));
        fields.extend_from_slice(&u32_field(0));
        fields.extend_from_slice(&u32_field(1));
        fields.extend_from_slice(&u32_field(2));
        fields.extend_from_slice(&u32_field(3));
        fields.extend_from_slice(
            &vec3(
                0.0_f32, 1.0_f32, 2.0_f32,
            ),
        );
        fields.extend_from_slice(
            &vec3(
                3.0_f32, 4.0_f32, 5.0_f32,
            ),
        );
        fields.extend_from_slice(
            &vec3(
                6.0_f32, 7.0_f32, 8.0_f32,
            ),
        );
        fields.extend_from_slice(&u32_field(1));
        fields.extend_from_slice(
            &vec3(
                0.0_f32, 1.0_f32, 0.0_f32,
            ),
        );
        let mut bbox = Vec::new();
        bbox.extend_from_slice(
            &vec3(
                -1.0_f32, -2.0_f32, -3.0_f32,
            ),
        );
        bbox.extend_from_slice(
            &vec3(
                9.0_f32, 8.0_f32, 7.0_f32,
            ),
        );
        let mut terrain = Vec::new();
        terrain.extend_from_slice(&u32_field(0));
        terrain.extend_from_slice(&u32_field(3));
        terrain.extend_from_slice(
            &[
                4_u8, 5_u8, 6_u8,
            ],
        );
        chunk(
            INTERSECT_DSG,
            fields,
            vec![
                chunk(
                    BBOX,
                    bbox,
                    Vec::new(),
                )?,
                chunk(
                    TERRAIN_TYPE,
                    terrain,
                    Vec::new(),
                )?,
            ],
        )
    }

    #[test]
    fn intersect_dsg_decodes_arrays_and_children() -> Result<(), String> {
        let fixture = require(
            intersect_fixture(),
            "intersect fixture should build",
        )?;
        let json = require(
            dsg_json(&fixture),
            "intersect fixture should decode",
        )?;
        require_json(
            &json,
            "\"indices\":[0,1,2]",
            "indices should be emitted",
        )?;
        require_json(
            &json,
            "\"positions\":[[0.0,1.0,2.0]",
            "positions should be emitted",
        )?;
        require_json(
            &json,
            "\"normals\":[[0.0,1.0,0.0]]",
            "normals should be emitted",
        )?;
        require_json(
            &json,
            "\"kind\":\"bbox\"",
            "bbox child should be emitted",
        )?;
        require_json(
            &json,
            "\"types\":[4,5,6]",
            "terrain bytes should be emitted",
        )?;
        Ok(())
    }

    #[test]
    fn intersect_dsg_fails_closed_on_truncated_array() -> Result<(), String> {
        let mut fixture = require(
            intersect_fixture(),
            "intersect fixture should build",
        )?;
        let _ = fixture.pop();
        if dsg_json(&fixture).is_none() {
            Ok(())
        } else {
            Err(String::from("truncated array should fail closed"))
        }
    }
}

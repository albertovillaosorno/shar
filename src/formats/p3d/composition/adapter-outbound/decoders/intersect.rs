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
//   - Intersect outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Intersect outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for p3d.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Intersect outbound adapter.

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
    let mut reader = Reader::new(chunk, 12);
    let indices = read_u32_array(&mut reader, header_size)?;
    let positions = read_vec3_array(&mut reader, header_size)?;
    let normals = read_vec3_array(&mut reader, header_size)?;
    if reader.pos() != header_size {
        return None;
    }
    let children = subchunks(chunk, header_size, total_size)?;
    let mut bounds = Vec::new();
    let mut terrain_types = Vec::new();
    let mut bbox_seen = false;
    let mut bsphere_seen = false;
    let mut terrain_seen = false;
    for child in children {
        match child.id {
            BBOX => {
                if bbox_seen {
                    return None;
                }
                bounds.push(decode_bbox(chunk, &child)?);
                bbox_seen = true;
            },
            BSPHERE => {
                if bsphere_seen {
                    return None;
                }
                bounds.push(decode_bsphere(chunk, &child)?);
                bsphere_seen = true;
            },
            TERRAIN_TYPE => {
                if terrain_seen {
                    return None;
                }
                let (json, count) = decode_terrain_type(chunk, &child)?;
                if count.checked_mul(3)? != indices.len() {
                    return None;
                }
                terrain_types.push(json);
                terrain_seen = true;
            },
            _ => return None,
        }
    }
    if !bbox_seen || !bsphere_seen {
        return None;
    }
    Some(format!(
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
    ))
}

/// Reads a count-prefixed integer list for triangle indices.
fn read_u32_array(reader: &mut Reader<'_>, end: usize) -> Option<Vec<u32>> {
    let count = usize::try_from(reader.u32()?).ok()?;
    if !count_bytes_fit(reader, end, count, 4)? {
        return None;
    }
    let mut values = Vec::new();
    for _ in 0..count {
        values.push(reader.u32()?);
    }
    Some(values)
}

/// Reads a count-prefixed vector list for positions and normals.
fn read_vec3_array(reader: &mut Reader<'_>, end: usize) -> Option<Vec<String>> {
    let count = usize::try_from(reader.u32()?).ok()?;
    if !count_bytes_fit(reader, end, count, 12)? {
        return None;
    }
    let mut values = Vec::new();
    for _ in 0..count {
        values.push(read_vec3(reader)?);
    }
    Some(values)
}

/// Checks whether a fixed-width count fits in the remaining header bytes.
fn count_bytes_fit(
    reader: &Reader<'_>,
    end: usize,
    count: usize,
    width: usize,
) -> Option<bool> {
    Some(count.checked_mul(width)? <= end.checked_sub(reader.pos())?)
}

/// Formats one vector as a JSON array without a serializer dependency.
fn read_vec3(reader: &mut Reader<'_>) -> Option<String> {
    let x = reader.f32()?;
    let y = reader.f32()?;
    let z = reader.f32()?;
    Some(format!("[{},{},{}]", fmt_f32(x), fmt_f32(y), fmt_f32(z)))
}

/// Decodes bounding boxes so collision mesh bounds are preserved.
fn decode_bbox(chunk: &[u8], child: &SubChunk) -> Option<String> {
    let mut reader = Reader::new(chunk, child.data_offset());
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
fn decode_bsphere(chunk: &[u8], child: &SubChunk) -> Option<String> {
    let mut reader = Reader::new(chunk, child.data_offset());
    let centre = read_vec3(&mut reader)?;
    let radius = reader.f32()?;
    let position_mismatch = reader.pos() != child.header_end();
    let child_size_mismatch = child.header_end() != child.end();
    if position_mismatch || child_size_mismatch {
        return None;
    }
    Some(format!(
        "{{\"kind\":\"bsphere\",\"centre\":{},\"radius\":{}}}",
        centre,
        fmt_f32(radius)
    ))
}

/// Decodes terrain-type bytes because they affect collision behavior.
fn decode_terrain_type(
    chunk: &[u8],
    child: &SubChunk,
) -> Option<(String, usize)> {
    let mut reader = Reader::new(chunk, child.data_offset());
    let version = reader.u32()?;
    if version != 0 {
        return None;
    }
    let count = usize::try_from(reader.u32()?).ok()?;
    let start = reader.pos();
    let end = start.checked_add(count)?;
    let types = chunk.get(start..end)?;
    if end != child.header_end() || child.header_end() != child.end() {
        return None;
    }
    Some((
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
        count,
    ))
}

/// Reads chunk bounds so malformed sizes fail closed.
fn chunk_bounds(chunk: &[u8]) -> Option<(u32, usize, usize)> {
    let id = read_u32(chunk, 0)?;
    let header_size = usize::try_from(read_u32(chunk, 4)?).ok()?;
    let total_size = usize::try_from(read_u32(chunk, 8)?).ok()?;
    if header_size < 12 || total_size < header_size || total_size > chunk.len()
    {
        return None;
    }
    Some((id, header_size, total_size))
}

/// Formats floats consistently for deterministic JSON tests.
fn fmt_f32(value: f32) -> String {
    let finite_rendering = if value.fract() == 0. {
        format!("{value:.1}")
    } else {
        value.to_string()
    };
    render_f32(value, finite_rendering)
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../../tests/formats/p3d/unit/adapter-outbound/decoders/intersect/tests.rs"]
mod tests;

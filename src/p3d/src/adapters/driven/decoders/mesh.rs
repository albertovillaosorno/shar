// File:
//   - mesh.rs
// Path:
//   - src/p3d/src/adapters/driven/decoders/mesh.rs
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
//   - The p3d adapter boundary for adapters driven decoders mesh.
// - Must-Not:
//   - Change domain semantics or bypass application and port contracts.
// - Allows:
//   - Filesystem, JSON, CLI, Blender, or serialization work behind explicit
//   - ports.
// - Split-When:
//   - A recovered list family requires an independent binary invariant or a
//   - separate exported asset model beyond mesh and skin primitive groups.
// - Merge-When:
//   - Another p3d module owns the same adapters boundary with no distinct
//   - invariant.
// - Summary:
//   - Lossless decoder for `MESH` (0x0001_0000) and `SKIN` (0x0001_0001)
//   - chunks.
// - Description:
//   - Validates primitive-group headers and typed vertex lists before rendering
//   - deterministic mesh and skin component JSON.
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
//   - Reason: src/p3d/src/adapters/driven/decoders/mesh.rs has 699 effective
//   - lines after the required header and remains cohesive until a focused
//   - split
//   - lands.
//

//! Lossless decoder for `MESH` (0x0001_0000) and `SKIN` (0x0001_0001) chunks.
//!
//! Both are containers of one or more `PRIMGROUP` (0x0001_0002) chunks. Each
//! primitive group carries its shader binding plus a set of per-vertex and
//! index list sub-chunks. The exact field order and list layout follow the
//! Pure3D primitive-group chunk format:
//!
//! * PRIMGROUP header: `version:u32, shader:PString, primType:u32,
//!   vertexFormat:u32, vertexCount:u32, indexCount:u32, matrixCount:u32`.
//! * Each list sub-chunk starts with a `count:u32`, then its array:
//!   - POSITION/NORMAL/BINORMAL/TANGENT/WEIGHT: `count` * three `f32`
//!   - COLOUR: `count` * one `u32` (RGBA)
//!   - UV / MULTICOLOUR: `channel:u32`, then `count` entries (UV = two `f32`,
//!     MULTICOLOUR = one `u32`)
//!   - INDEX: `count` * one `u32`
//!   - MATRIX: `count` * four `u8` (matrix-palette indices per vertex)
//!   - MATRIXPALETTE / PACKEDNORMALLIST: `count` * one `u32`
//!
//! Decoding is verified by round-tripping: `POSITIONLIST` length must equal
//! the header `vertexCount` and `INDEXLIST` length must equal `indexCount`,
//! otherwise the decoder fails closed (returns `None`) so the caller keeps
//! the raw payload rather than emitting silently wrong geometry.
// These exact file-local lints preserve explicit domain and binary contracts.
#![expect(
    clippy::arithmetic_side_effects,
    clippy::as_conversions,
    clippy::doc_markdown,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    reason = "Tests verify these intentional explicit file-local contracts \
              remain safe."
)]

use std::fmt::Write as _;

use super::reader::{Reader, subchunks};
use crate::adapters::driven::json::{escape_json as escape, render_f32};

/// Mesh.
const MESH: u32 = 0x0001_0000;
/// Skin.
const SKIN: u32 = 0x0001_0001;
/// Primgroup.
const PRIMGROUP: u32 = 0x0001_0002;
/// Bbox.
const BBOX: u32 = 0x0001_0003;
/// Bsphere.
const BSPHERE: u32 = 0x0001_0004;
/// Renderstatus.
const RENDERSTATUS: u32 = 0x0001_0017;

/// Positionlist.
const POSITIONLIST: u32 = 0x0001_0005;
/// Normallist.
const NORMALLIST: u32 = 0x0001_0006;
/// Uvlist.
const UVLIST: u32 = 0x0001_0007;
/// Colourlist.
const COLOURLIST: u32 = 0x0001_0008;
/// Indexlist.
const INDEXLIST: u32 = 0x0001_000A;
/// Matrixlist.
const MATRIXLIST: u32 = 0x0001_000B;
/// Weightlist.
const WEIGHTLIST: u32 = 0x0001_000C;
/// Matrixpalette.
const MATRIXPALETTE: u32 = 0x0001_000D;
/// Packednormallist.
const PACKEDNORMALLIST: u32 = 0x0001_0010;
/// Vertexshader.
const VERTEXSHADER: u32 = 0x0001_0011;
/// Tangentlist.
const TANGENTLIST: u32 = 0x0001_0015;
/// Binormallist.
const BINORMALLIST: u32 = 0x0001_0016;
/// Multicolourlist.
const MULTICOLOURLIST: u32 = 0x0001_001C;

/// Decode a `MESH` chunk (`chunk` is the whole chunk including its 12-byte
/// header) into the lossless mesh JSON body, or `None` to fail closed.
pub fn mesh_json(chunk: &[u8]) -> Option<String> {
    let (name, version, prim_start, prim_end) = read_container_header(
        chunk, false,
    )?;
    let body = decode_children(
        chunk, prim_start, prim_end,
    )?;
    Some(
        format!(
            "{{\"schema\":\"mesh\",\"name\":\"{}\",\"version\":{},\"\
             num_prim_groups\":{},\"prim_groups\":[{}]{}}}\n",
            escape(&name),
            version,
            body.groups
                .len(),
            body.groups
                .join(","),
            body.trailer()
        ),
    )
}

/// Decode a `SKIN` chunk into the lossless skin JSON body, or `None`.
pub fn skin_json(chunk: &[u8]) -> Option<String> {
    let (name, version, skeleton, prim_start, prim_end) =
        read_skin_header(chunk)?;
    let body = decode_children(
        chunk, prim_start, prim_end,
    )?;
    Some(
        format!(
            "{{\"schema\":\"skin\",\"name\":\"{}\",\"version\":{},\"\
             skeleton_name\":\"{}\",\"num_prim_groups\":{},\"prim_groups\":\
             [{}]{}}}\n",
            escape(&name),
            version,
            escape(&skeleton),
            body.groups
                .len(),
            body.groups
                .join(","),
            body.trailer()
        ),
    )
}

/// Parse a mesh container header, returning `(name, version, child_start,
/// child_end)` where the child region holds the primitive groups.
fn read_container_header(
    chunk: &[u8],
    _skin: bool,
) -> Option<(
    String,
    u32,
    usize,
    usize,
)> {
    let header_size = super::reader::read_u32(
        chunk, 4,
    )? as usize;
    let total_size = super::reader::read_u32(
        chunk, 8,
    )? as usize;
    if header_size < 12 || total_size < header_size || total_size > chunk.len()
    {
        return None;
    }
    let mut reader = Reader::new(
        chunk, 12,
    );
    let name = reader.pstring()?;
    let version = reader.u32()?;
    // The remaining header field (num_prim_groups) is redundant with the
    // actual child count, which we enumerate directly.
    Some(
        (
            name,
            version,
            header_size,
            total_size,
        ),
    )
}

/// Read skin header.
fn read_skin_header(
    chunk: &[u8]
) -> Option<(
    String,
    u32,
    String,
    usize,
    usize,
)> {
    let header_size = super::reader::read_u32(
        chunk, 4,
    )? as usize;
    let total_size = super::reader::read_u32(
        chunk, 8,
    )? as usize;
    if header_size < 12 || total_size < header_size || total_size > chunk.len()
    {
        return None;
    }
    let mut reader = Reader::new(
        chunk, 12,
    );
    let name = reader.pstring()?;
    let version = reader.u32()?;
    let skeleton = reader.pstring()?;
    Some(
        (
            name,
            version,
            skeleton,
            header_size,
            total_size,
        ),
    )
}

/// The decoded content of a mesh/skin child region: the primitive groups plus
/// any bounding volumes, render status, and a record of unhandled chunk ids so
/// nothing is silently dropped.
struct MeshBody {
    /// Groups.
    groups: Vec<String>,
    /// Extras.
    extras: Vec<String>,
    /// Unhandled.
    unhandled: Vec<(
        u32,
        usize,
    )>,
}

impl MeshBody {
    /// The extra `,"field":value` pairs that follow `prim_groups` in the JSON.
    fn trailer(&self) -> String {
        let mut out = String::new();
        for extra in &self.extras {
            out.push(',');
            out.push_str(extra);
        }
        if !self
            .unhandled
            .is_empty()
        {
            out.push_str(",\"unhandled_subchunks\":[");
            for (i, (id, bytes)) in self
                .unhandled
                .iter()
                .enumerate()
            {
                if i > 0 {
                    out.push(',');
                }
                let _write_result = write!(
                    out,
                    "{{\"id\":\"0x{id:08X}\",\"bytes\":{bytes}}}"
                );
            }
            out.push(']');
        }
        out
    }
}

/// Decode children.
fn decode_children(
    chunk: &[u8],
    start: usize,
    end: usize,
) -> Option<MeshBody> {
    let mut body = MeshBody {
        groups: Vec::new(),
        extras: Vec::new(),
        unhandled: Vec::new(),
    };
    for child in subchunks(
        chunk, start, end,
    )? {
        match child.id {
            PRIMGROUP => body
                .groups
                .push(
                    decode_prim_group(
                        chunk, &child,
                    )?,
                ),
            BBOX => {
                let mut reader = Reader::new(
                    chunk,
                    child.data_offset(),
                );
                let low = read_vec3(&mut reader)?;
                let high = read_vec3(&mut reader)?;
                body.extras
                    .push(
                        format!(
                            "\"bounding_box\":{{\"low\":{low},\"high\":\
                             {high}}}"
                        ),
                    );
            }
            BSPHERE => {
                let mut reader = Reader::new(
                    chunk,
                    child.data_offset(),
                );
                let centre = read_vec3(&mut reader)?;
                let radius = reader.f32()?;
                body.extras
                    .push(
                        format!(
                            "\"bounding_sphere\":{{\"centre\":{centre},\"\
                             radius\":{}}}",
                            fmt_f32(radius)
                        ),
                    );
            }
            RENDERSTATUS => {
                let status = Reader::new(
                    chunk,
                    child.data_offset(),
                )
                .u32()?;
                body.extras
                    .push(format!("\"render_status\":{status}"));
            }
            other => body
                .unhandled
                .push(
                    (
                        other,
                        child.total_size,
                    ),
                ),
        }
    }
    Some(body)
}

/// Read vec3.
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

/// Keeps primitive header values together because count validation and JSON
/// rendering consume the same declared contract.
struct PrimitiveHeader {
    /// Shader identity remains attached because material binding depends on
    /// it.
    shader: String,
    /// Primitive topology controls how decoded indices are interpreted.
    prim_type: u32,
    /// Vertex format determines which optional lists are meaningful.
    vertex_format: u32,
    /// Declared vertex count validates the decoded position list.
    vertex_count: u32,
    /// Declared index count validates the decoded index list.
    index_count: u32,
    /// Declared matrix count is retained for skinning reconstruction.
    matrix_count: u32,
}

impl PrimitiveHeader {
    /// Reads the fixed header before any child list can affect decoder state.
    fn read(reader: &mut Reader<'_>) -> Option<Self> {
        let _version = reader.u32()?;
        Some(
            Self {
                shader: reader.pstring()?,
                prim_type: reader.u32()?,
                vertex_format: reader.u32()?,
                vertex_count: reader.u32()?,
                index_count: reader.u32()?,
                matrix_count: reader.u32()?,
            },
        )
    }
}

/// Separates scalar fields from repeatable channels because their ordering and
/// count invariants differ during deterministic rendering.
#[derive(Default)]
struct PrimitiveLists {
    /// Scalar JSON fields retain source order for deterministic output.
    fields: Vec<String>,
    /// UV channels may repeat and therefore remain ordered.
    uv_channels: Vec<String>,
    /// Multicolour channels may repeat and therefore remain ordered.
    multi_colours: Vec<String>,
    /// Vertex-shader identity must fail closed when its string is malformed.
    vertex_shader: String,
    /// Decoded position count validates the declared vertex count.
    positions: Option<usize>,
    /// Decoded index count validates the declared index count.
    indices: Option<usize>,
}

impl PrimitiveLists {
    /// Decodes every recognized child list while rejecting unknown list ids.
    fn decode(
        chunk: &[u8],
        group: &super::reader::SubChunk,
    ) -> Option<Self> {
        let mut decoded = Self::default();
        for list in subchunks(
            chunk,
            group.header_end(),
            group.end(),
        )? {
            let base = list.data_offset();
            let handled = decoded.decode_float_list(
                chunk, list.id, base,
            )? || decoded.decode_integer_list(
                chunk, list.id, base,
            )? || decoded.decode_channel_list(
                chunk, list.id, base,
            )?;
            if !handled {
                return None;
            }
        }
        Some(decoded)
    }

    /// Decodes vector list families that share floating-point payload widths.
    fn decode_float_list(
        &mut self,
        chunk: &[u8],
        id: u32,
        base: usize,
    ) -> Option<bool> {
        let field = match id {
            POSITIONLIST => {
                let (json, count) = float3_list(
                    chunk, base,
                )?;
                self.positions = Some(count);
                Some(format!("\"positions\":{json}"))
            }
            NORMALLIST => Some(
                format!(
                    "\"normals\":{}",
                    float3_list(
                        chunk, base,
                    )?
                    .0
                ),
            ),
            TANGENTLIST => Some(
                format!(
                    "\"tangents\":{}",
                    float3_list(
                        chunk, base,
                    )?
                    .0
                ),
            ),
            BINORMALLIST => Some(
                format!(
                    "\"binormals\":{}",
                    float3_list(
                        chunk, base,
                    )?
                    .0
                ),
            ),
            WEIGHTLIST => Some(
                format!(
                    "\"weights\":{}",
                    float3_list(
                        chunk, base,
                    )?
                    .0
                ),
            ),
            _ => None,
        };
        if let Some(value) = field {
            self.fields
                .push(value);
            return Some(true);
        }
        Some(false)
    }

    /// Decodes integral list families whose element widths are fixed by id.
    fn decode_integer_list(
        &mut self,
        chunk: &[u8],
        id: u32,
        base: usize,
    ) -> Option<bool> {
        let field = match id {
            COLOURLIST => Some(
                format!(
                    "\"colours\":{}",
                    u32_list(
                        chunk, base,
                    )?
                    .0
                ),
            ),
            PACKEDNORMALLIST => Some(
                format!(
                    "\"packed_normals\":{}",
                    u32_list(
                        chunk, base,
                    )?
                    .0
                ),
            ),
            MATRIXPALETTE => Some(
                format!(
                    "\"matrix_palette\":{}",
                    u32_list(
                        chunk, base,
                    )?
                    .0
                ),
            ),
            INDEXLIST => {
                let (json, count) = u32_list(
                    chunk, base,
                )?;
                self.indices = Some(count);
                Some(format!("\"indices\":{json}"))
            }
            MATRIXLIST => Some(
                format!(
                    "\"matrices\":{}",
                    byte4_list(
                        chunk, base,
                    )?
                ),
            ),
            _ => None,
        };
        if let Some(value) = field {
            self.fields
                .push(value);
            return Some(true);
        }
        Some(false)
    }

    /// Decodes repeatable channels and the optional vertex-shader reference.
    fn decode_channel_list(
        &mut self,
        chunk: &[u8],
        id: u32,
        base: usize,
    ) -> Option<bool> {
        match id {
            UVLIST => self
                .uv_channels
                .push(
                    uv_channel(
                        chunk, base,
                    )?,
                ),
            MULTICOLOURLIST => self
                .multi_colours
                .push(
                    multicolour_channel(
                        chunk, base,
                    )?,
                ),
            VERTEXSHADER => {
                self.vertex_shader = Reader::new(
                    chunk, base,
                )
                .pstring()?;
            }
            _ => return Some(false),
        }
        Some(true)
    }

    /// Confirms decoded array counts match the primitive-group declarations.
    fn counts_match(
        &self,
        header: &PrimitiveHeader,
    ) -> bool {
        let vertex_count = usize::try_from(header.vertex_count).ok();
        let index_count = usize::try_from(header.index_count).ok();
        self.positions
            .is_none_or(|count| Some(count) == vertex_count)
            && self
                .indices
                .is_none_or(|count| Some(count) == index_count)
    }

    /// Renders one deterministic JSON object after count validation succeeds.
    fn render(
        self,
        header: &PrimitiveHeader,
    ) -> String {
        let mut output = format!(
            "{{\"shader\":\"{}\",\"vertex_shader\":\"{}\",\"prim_type\":{},\"\
             vertex_format\":{},\"vertex_count\":{},\"index_count\":{},\"\
             matrix_count\":{}",
            escape(&header.shader),
            escape(&self.vertex_shader),
            header.prim_type,
            header.vertex_format,
            header.vertex_count,
            header.index_count,
            header.matrix_count
        );
        for field in self.fields {
            output.push(',');
            output.push_str(&field);
        }
        if !self
            .uv_channels
            .is_empty()
        {
            output.push_str(",\"uvs\":[");
            output.push_str(
                &self
                    .uv_channels
                    .join(","),
            );
            output.push(']');
        }
        if !self
            .multi_colours
            .is_empty()
        {
            output.push_str(",\"multi_colours\":[");
            output.push_str(
                &self
                    .multi_colours
                    .join(","),
            );
            output.push(']');
        }
        output.push('}');
        output
    }
}

/// Decodes one primitive group after validating every declared list count.
fn decode_prim_group(
    chunk: &[u8],
    group: &super::reader::SubChunk,
) -> Option<String> {
    let mut reader = Reader::new(
        chunk,
        group.data_offset(),
    );
    let header = PrimitiveHeader::read(&mut reader)?;
    let lists = PrimitiveLists::decode(
        chunk, group,
    )?;
    if !lists.counts_match(&header) {
        return None;
    }
    Some(lists.render(&header))
}

/// `count:u32` then `count` * three `f32`. Returns `(json_array, count)`.
fn float3_list(
    chunk: &[u8],
    base: usize,
) -> Option<(
    String,
    usize,
)> {
    let mut reader = Reader::new(
        chunk, base,
    );
    let count = reader.u32()? as usize;
    let mut json = String::with_capacity(count * 24 + 2);
    json.push('[');
    for i in 0..count {
        if i > 0 {
            json.push(',');
        }
        let x = reader.f32()?;
        let y = reader.f32()?;
        let z = reader.f32()?;
        json.push('[');
        json.push_str(&fmt_f32(x));
        json.push(',');
        json.push_str(&fmt_f32(y));
        json.push(',');
        json.push_str(&fmt_f32(z));
        json.push(']');
    }
    json.push(']');
    Some(
        (
            json, count,
        ),
    )
}

/// `count:u32` then `count` * one `u32`. Returns `(json_array, count)`.
fn u32_list(
    chunk: &[u8],
    base: usize,
) -> Option<(
    String,
    usize,
)> {
    let mut reader = Reader::new(
        chunk, base,
    );
    let count = reader.u32()? as usize;
    let mut json = String::with_capacity(count * 6 + 2);
    json.push('[');
    for i in 0..count {
        if i > 0 {
            json.push(',');
        }
        json.push_str(
            &reader
                .u32()?
                .to_string(),
        );
    }
    json.push(']');
    Some(
        (
            json, count,
        ),
    )
}

/// `count:u32` then `count` * four `u8`.
fn byte4_list(
    chunk: &[u8],
    base: usize,
) -> Option<String> {
    let mut reader = Reader::new(
        chunk, base,
    );
    let count = reader.u32()? as usize;
    let mut json = String::with_capacity(count * 12 + 2);
    json.push('[');
    for i in 0..count {
        if i > 0 {
            json.push(',');
        }
        let packed = reader.u32()?;
        let bytes = packed.to_le_bytes();
        json.push('[');
        json.push_str(&bytes[0].to_string());
        json.push(',');
        json.push_str(&bytes[1].to_string());
        json.push(',');
        json.push_str(&bytes[2].to_string());
        json.push(',');
        json.push_str(&bytes[3].to_string());
        json.push(']');
    }
    json.push(']');
    Some(json)
}

/// UV list: `count:u32, channel:u32`, then `count` * two `f32`. Tagged with a
/// `"uv":` prefix so the caller can group channels.
fn uv_channel(
    chunk: &[u8],
    base: usize,
) -> Option<String> {
    let mut reader = Reader::new(
        chunk, base,
    );
    let count = reader.u32()? as usize;
    let channel = reader.u32()?;
    let mut coords = String::with_capacity(count * 16 + 2);
    coords.push('[');
    for i in 0..count {
        if i > 0 {
            coords.push(',');
        }
        let u = reader.f32()?;
        let v = reader.f32()?;
        coords.push('[');
        coords.push_str(&fmt_f32(u));
        coords.push(',');
        coords.push_str(&fmt_f32(v));
        coords.push(']');
    }
    coords.push(']');
    Some(format!("{{\"channel\":{channel},\"coords\":{coords}}}"))
}

/// Multicolour list: `count:u32, channel:u32`, then `count` * one `u32`.
fn multicolour_channel(
    chunk: &[u8],
    base: usize,
) -> Option<String> {
    let mut reader = Reader::new(
        chunk, base,
    );
    let count = reader.u32()? as usize;
    let channel = reader.u32()?;
    let mut values = String::with_capacity(count * 6 + 2);
    values.push('[');
    for i in 0..count {
        if i > 0 {
            values.push(',');
        }
        values.push_str(
            &reader
                .u32()?
                .to_string(),
        );
    }
    values.push(']');
    Some(format!("{{\"channel\":{channel},\"values\":{values}}}"))
}

/// Format an `f32` as a round-trippable JSON number, or `null` if non-finite.
fn fmt_f32(value: f32) -> String {
    render_f32(
        value,
        value.to_string(),
    )
}

/// Chunk ids this module owns, for the dispatch table.
pub const IDS: [u32; 2] = [
    MESH, SKIN,
];

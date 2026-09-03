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
//   - Mesh outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Mesh outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for p3d.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Mesh outbound adapter.

#![expect(
    clippy::arithmetic_side_effects,
    clippy::as_conversions,
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
/// Expression offsets.
const EXPRESSIONOFFSETS: u32 = 0x0001_0018;
/// Offset list.
const OFFSETLIST: u32 = 0x0001_000e;

/// Positionlist.
const POSITIONLIST: u32 = 0x0001_0005;
/// Normallist.
const NORMALLIST: u32 = 0x0001_0006;
/// Uvlist.
const UVLIST: u32 = 0x0001_0007;
/// Colourlist.
const COLOURLIST: u32 = 0x0001_0008;
/// Indexlist.
const INDEXLIST: u32 = 0x0001_000a;
/// Matrixlist.
const MATRIXLIST: u32 = 0x0001_000b;
/// Weightlist.
const WEIGHTLIST: u32 = 0x0001_000c;
/// Matrixpalette.
const MATRIXPALETTE: u32 = 0x0001_000d;
/// Packednormallist.
const PACKEDNORMALLIST: u32 = 0x0001_0010;
/// Vertexshader.
const VERTEXSHADER: u32 = 0x0001_0011;
/// Tangentlist.
const TANGENTLIST: u32 = 0x0001_0015;
/// Binormallist.
const BINORMALLIST: u32 = 0x0001_0016;
/// Multicolourlist.
const MULTICOLOURLIST: u32 = 0x0001_001c;

/// Decode a `MESH` chunk (`chunk` is the whole chunk including its 12-byte
/// header) into the lossless mesh JSON body, or `None` to fail closed.
pub fn mesh_json(chunk: &[u8]) -> Option<String> {
    mesh_json_with_source_ordinals(chunk, None)
}

/// Decode a `MESH` chunk with exact package-level primitive-group ordinals.
pub(crate) fn mesh_json_with_source_ordinals(
    chunk: &[u8],
    source_ordinals: Option<&[usize]>,
) -> Option<String> {
    let (name, version, declared_groups, prim_start, prim_end) =
        read_container_header(chunk, false)?;
    let body =
        decode_children(chunk, prim_start, prim_end, source_ordinals, false)?;
    if body.groups.len() != usize::try_from(declared_groups).ok()? {
        return None;
    }
    Some(format!(
        "{{\"schema\":\"mesh\",\"name\":\"{}\",\"version\":{},\"\
             num_prim_groups\":{},\"prim_groups\":[{}]{}}}\n",
        escape(&name),
        version,
        body.groups.len(),
        body.groups.join(","),
        body.trailer()
    ))
}

/// Decode a `SKIN` chunk into the lossless skin JSON body, or `None`.
pub fn skin_json(chunk: &[u8]) -> Option<String> {
    skin_json_with_source_ordinals(chunk, None)
}

/// Decode a `SKIN` chunk with exact package-level primitive-group ordinals.
pub(crate) fn skin_json_with_source_ordinals(
    chunk: &[u8],
    source_ordinals: Option<&[usize]>,
) -> Option<String> {
    let (name, version, skeleton, declared_groups, prim_start, prim_end) =
        read_skin_header(chunk)?;
    let body =
        decode_children(chunk, prim_start, prim_end, source_ordinals, true)?;
    if body.groups.len() != usize::try_from(declared_groups).ok()? {
        return None;
    }
    Some(format!(
        "{{\"schema\":\"skin\",\"name\":\"{}\",\"version\":{},\"\
             skeleton_name\":\"{}\",\"num_prim_groups\":{},\"prim_groups\":\
             [{}]{}}}\n",
        escape(&name),
        version,
        escape(&skeleton),
        body.groups.len(),
        body.groups.join(","),
        body.trailer()
    ))
}

/// Parse a mesh container header, returning its declared source contract and
/// child region.
fn read_container_header(
    chunk: &[u8],
    _skin: bool,
) -> Option<(String, u32, u32, usize, usize)> {
    let header_size = super::reader::read_u32(chunk, 4)? as usize;
    let total_size = super::reader::read_u32(chunk, 8)? as usize;
    if header_size < 12 || total_size < header_size || total_size > chunk.len()
    {
        return None;
    }
    let mut reader = Reader::new(chunk, 12);
    let name = reader.pstring()?;
    let version = reader.u32()?;
    let declared_groups = reader.u32()?;
    if reader.pos() != header_size {
        return None;
    }
    Some((name, version, declared_groups, header_size, total_size))
}

/// Read skin header.
fn read_skin_header(
    chunk: &[u8],
) -> Option<(String, u32, String, u32, usize, usize)> {
    let header_size = super::reader::read_u32(chunk, 4)? as usize;
    let total_size = super::reader::read_u32(chunk, 8)? as usize;
    if header_size < 12 || total_size < header_size || total_size > chunk.len()
    {
        return None;
    }
    let mut reader = Reader::new(chunk, 12);
    let name = reader.pstring()?;
    let version = reader.u32()?;
    let skeleton = reader.pstring()?;
    let declared_groups = reader.u32()?;
    if reader.pos() != header_size {
        return None;
    }
    Some((
        name,
        version,
        skeleton,
        declared_groups,
        header_size,
        total_size,
    ))
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
    unhandled: Vec<(u32, usize)>,
    /// Bounding-box source evidence is singleton in the complete corpus.
    bounding_box_seen: bool,
    /// Bounding-sphere source evidence is singleton in the complete corpus.
    bounding_sphere_seen: bool,
    /// Render-status source evidence is singleton in the complete corpus.
    render_status_seen: bool,
    /// Expression-offset source evidence is singleton in the complete corpus.
    expression_offsets_seen: bool,
}

impl MeshBody {
    /// The extra `,"field":value` pairs that follow `prim_groups` in the JSON.
    fn trailer(&self) -> String {
        let mut out = String::new();
        for extra in &self.extras {
            out.push(',');
            out.push_str(extra);
        }
        if !self.unhandled.is_empty() {
            out.push_str(",\"unhandled_subchunks\":[");
            for (i, (id, bytes)) in self.unhandled.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                let _write_result =
                    write!(out, "{{\"id\":\"0x{id:08X}\",\"bytes\":{bytes}}}");
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
    source_ordinals: Option<&[usize]>,
    validate_expression_targets: bool,
) -> Option<MeshBody> {
    let children = subchunks(chunk, start, end)?;
    let expression_target_vertex_counts = if validate_expression_targets {
        Some(
            children
                .iter()
                .filter(|child| child.id == PRIMGROUP)
                .map(|child| primitive_group_vertex_count(chunk, child))
                .collect::<Option<Vec<_>>>()?,
        )
    } else {
        None
    };
    let mut body = MeshBody {
        groups: Vec::new(),
        extras: Vec::new(),
        unhandled: Vec::new(),
        bounding_box_seen: false,
        bounding_sphere_seen: false,
        render_status_seen: false,
        expression_offsets_seen: false,
    };
    let mut primitive_group_index = 0_usize;
    for child in children {
        match child.id {
            PRIMGROUP => {
                let source_ordinal = match source_ordinals {
                    Some(ordinals) => {
                        Some(*ordinals.get(primitive_group_index)?)
                    },
                    None => None,
                };
                body.groups.push(decode_prim_group(
                    chunk,
                    &child,
                    source_ordinal,
                    validate_expression_targets,
                )?);
                primitive_group_index = primitive_group_index.checked_add(1)?;
            },
            BBOX => {
                if body.bounding_box_seen
                    || child.header_size != 36
                    || child.total_size != 36
                {
                    return None;
                }
                let bounded = chunk.get(..child.header_end())?;
                let mut reader = Reader::new(bounded, child.data_offset());
                let low = read_vec3_with_bits(&mut reader)?;
                let high = read_vec3_with_bits(&mut reader)?;
                if reader.pos() != bounded.len() {
                    return None;
                }
                body.extras.push(format!(
                    concat!(
                        "\"bounding_box\":{{\"low\":{},\"high\":{}}}",
                        ",\"bounding_box_f32_bits\":{{\"low\":{},",
                        "\"high\":{}}}"
                    ),
                    low.0, high.0, low.1, high.1
                ));
                body.bounding_box_seen = true;
            },
            BSPHERE => {
                if body.bounding_sphere_seen
                    || child.header_size != 28
                    || child.total_size != 28
                {
                    return None;
                }
                let bounded = chunk.get(..child.header_end())?;
                let mut reader = Reader::new(bounded, child.data_offset());
                let centre = read_vec3_with_bits(&mut reader)?;
                let radius = reader.f32()?;
                if reader.pos() != bounded.len() {
                    return None;
                }
                body.extras.push(format!(
                    concat!(
                        "\"bounding_sphere\":{{\"centre\":{},",
                        "\"radius\":{}}},",
                        "\"bounding_sphere_f32_bits\":{{\"centre\":{},",
                        "\"radius\":{}}}"
                    ),
                    centre.0,
                    fmt_f32(radius),
                    centre.1,
                    radius.to_bits()
                ));
                body.bounding_sphere_seen = true;
            },
            RENDERSTATUS => {
                if body.render_status_seen
                    || child.header_size != 16
                    || child.total_size != 16
                {
                    return None;
                }
                let bounded = chunk.get(..child.header_end())?;
                let mut reader = Reader::new(bounded, child.data_offset());
                let status = reader.u32()?;
                if reader.pos() != bounded.len() {
                    return None;
                }
                body.extras.push(format!("\"render_status\":{status}"));
                body.render_status_seen = true;
            },
            EXPRESSIONOFFSETS => {
                if body.expression_offsets_seen {
                    return None;
                }
                body.extras.push(format!(
                    "\"expression_offsets\":{}",
                    expression_offsets_json(
                        chunk,
                        &child,
                        expression_target_vertex_counts.as_deref(),
                    )?
                ));
                body.expression_offsets_seen = true;
            },
            other => body.unhandled.push((other, child.total_size)),
        }
    }
    if source_ordinals
        .is_some_and(|ordinals| ordinals.len() != primitive_group_index)
    {
        return None;
    }
    Some(body)
}

/// Return the declared vertex count for one validated primitive group header.
fn primitive_group_vertex_count(
    chunk: &[u8],
    group: &super::reader::SubChunk,
) -> Option<u32> {
    let mut reader = Reader::new(chunk, group.data_offset());
    let header = PrimitiveHeader::read(&mut reader)?;
    (reader.pos() == group.header_end()).then_some(header.vertex_count)
}

/// Decode one expression-offset container as source evidence.
fn expression_offsets_json(
    chunk: &[u8],
    expression: &super::reader::SubChunk,
    target_vertex_counts: Option<&[u32]>,
) -> Option<String> {
    let bounded = chunk.get(..expression.header_end())?;
    let mut reader = Reader::new(bounded, expression.data_offset());
    let primitive_group_count = usize::try_from(reader.u32()?).ok()?;
    let offset_list_count = usize::try_from(reader.u32()?).ok()?;
    let group_bytes = primitive_group_count.checked_mul(4)?;
    if group_bytes > bounded.len().checked_sub(reader.pos())? {
        return None;
    }
    let mut primitive_groups = Vec::with_capacity(primitive_group_count);
    for _ in 0..primitive_group_count {
        primitive_groups.push(reader.u32()?);
    }
    if reader.pos() != bounded.len() {
        return None;
    }
    let children = subchunks(chunk, expression.header_end(), expression.end())?;
    if children.len() != offset_list_count {
        return None;
    }
    let mut lists = Vec::with_capacity(children.len());
    for child in children {
        if child.id != OFFSETLIST || child.header_size != child.total_size {
            return None;
        }
        lists.push(offset_list_json(
            chunk,
            &child,
            &primitive_groups,
            target_vertex_counts,
        )?);
    }
    Some(format!(
        concat!(
            "{{\"num_prim_groups\":{},\"num_offset_lists\":{},",
            "\"prim_group_indices\":[{}],\"offset_lists\":[{}]}}"
        ),
        primitive_group_count,
        offset_list_count,
        primitive_groups
            .iter()
            .map(u32::to_string)
            .collect::<Vec<_>>()
            .join(","),
        lists.join(",")
    ))
}

/// Decode one expression offset list without assigning morph semantics.
fn offset_list_json(
    chunk: &[u8],
    list: &super::reader::SubChunk,
    expression_primitive_groups: &[u32],
    target_vertex_counts: Option<&[u32]>,
) -> Option<String> {
    let bounded = chunk.get(..list.header_end())?;
    let mut reader = Reader::new(bounded, list.data_offset());
    let offset_count = usize::try_from(reader.u32()?).ok()?;
    let key_index = reader.u32()?;
    let offset_bytes = offset_count.checked_mul(16)?;
    let minimum_bytes = offset_bytes.checked_add(4)?;
    if minimum_bytes > bounded.len().checked_sub(reader.pos())? {
        return None;
    }
    let mut offsets = Vec::with_capacity(offset_count);
    let mut vertex_indices = Vec::with_capacity(offset_count);
    for _ in 0..offset_count {
        let vertex_index = reader.u32()?;
        vertex_indices.push(vertex_index);
        let offset = read_vec3_with_bits(&mut reader)?;
        offsets.push(format!(
            concat!(
                "{{\"vertex_index\":{},\"offset\":{},",
                "\"offset_f32_bits\":{}}}"
            ),
            vertex_index, offset.0, offset.1
        ));
    }
    let primitive_group_index = reader.u32()?;
    if reader.pos() != bounded.len() {
        return None;
    }
    if let Some(vertex_counts) = target_vertex_counts {
        let group_index = usize::try_from(primitive_group_index).ok()?;
        let vertex_count = *vertex_counts.get(group_index)?;
        if !expression_primitive_groups.contains(&primitive_group_index)
            || vertex_indices
                .iter()
                .any(|vertex_index| *vertex_index >= vertex_count)
        {
            return None;
        }
    }
    Some(format!(
        concat!(
            "{{\"num_offsets\":{},\"key_index\":{},",
            "\"offsets\":[{}],\"prim_group_index\":{}}}"
        ),
        offset_count,
        key_index,
        offsets.join(","),
        primitive_group_index
    ))
}

/// Read a vec3 while retaining each authored IEEE-754 bit pattern.
fn read_vec3_with_bits(reader: &mut Reader<'_>) -> Option<(String, String)> {
    let x = reader.f32()?;
    let y = reader.f32()?;
    let z = reader.f32()?;
    Some((
        format!("[{},{},{}]", fmt_f32(x), fmt_f32(y), fmt_f32(z)),
        format!("[{},{},{}]", x.to_bits(), y.to_bits(), z.to_bits()),
    ))
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
    /// Only version observed in the complete source game corpus.
    const SUPPORTED_VERSION: u32 = 0;

    /// Reads the fixed header before any child list can affect decoder state.
    fn read(reader: &mut Reader<'_>) -> Option<Self> {
        const MAX_RUNTIME_COUNT: u32 = 2_147_483_647;
        let version = reader.u32()?;
        if version != Self::SUPPORTED_VERSION {
            return None;
        }
        let shader = reader.pstring()?;
        let prim_type = reader.u32()?;
        let vertex_format = reader.u32()?;
        let vertex_count = reader.u32()?;
        let index_count = reader.u32()?;
        let matrix_count = reader.u32()?;
        if prim_type > 4
            || vertex_count > MAX_RUNTIME_COUNT
            || index_count > MAX_RUNTIME_COUNT
            || matrix_count > 256
        {
            return None;
        }
        Some(Self {
            shader,
            prim_type,
            vertex_format,
            vertex_count,
            index_count,
            matrix_count,
        })
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
    /// Sparse exact bits for source positions containing non-finite floats.
    position_nonfinite_bits: Option<String>,
    /// Decoded index count validates the declared index count.
    indices: Option<usize>,
    /// Matrix-palette length validates the primitive matrix declaration.
    matrix_palette: Option<usize>,
    /// Every decoded per-vertex list must agree with `NumVertices`.
    vertex_list_counts: Vec<usize>,
    /// Non-channel list families occur at most once per primitive group.
    singleton_vertex_lists: Vec<u32>,
    /// Vertex-shader chunks are singleton even when their authored name is
    /// empty.
    vertex_shader_seen: bool,
}

impl PrimitiveLists {
    /// Decodes every recognized child list while rejecting unknown list ids.
    fn decode(chunk: &[u8], group: &super::reader::SubChunk) -> Option<Self> {
        let mut decoded = Self::default();
        for list in subchunks(chunk, group.header_end(), group.end())? {
            if list.header_size != list.total_size {
                return None;
            }
            let bounded = chunk.get(..list.header_end())?;
            let base = list.data_offset();
            let handled = decoded.decode_float_list(bounded, list.id, base)?
                || decoded.decode_integer_list(bounded, list.id, base)?
                || decoded.decode_channel_list(bounded, list.id, base)?;
            if !handled {
                return None;
            }
        }
        Some(decoded)
    }

    /// Record one singleton per-vertex list and its authored element count.
    fn record_vertex_list(&mut self, id: u32, count: usize) -> Option<()> {
        if self.singleton_vertex_lists.contains(&id) {
            return None;
        }
        self.singleton_vertex_lists.push(id);
        self.vertex_list_counts.push(count);
        Some(())
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
                let (json, count, nonfinite_bits) = position_list(chunk, base)?;
                if self.positions.replace(count).is_some() {
                    return None;
                }
                self.position_nonfinite_bits = nonfinite_bits;
                Some(format!("\"positions\":{json}"))
            },
            NORMALLIST | TANGENTLIST | BINORMALLIST | WEIGHTLIST => {
                let (json, count) = float3_list(chunk, base)?;
                self.record_vertex_list(id, count)?;
                let name = match id {
                    NORMALLIST => "normals",
                    TANGENTLIST => "tangents",
                    BINORMALLIST => "binormals",
                    WEIGHTLIST => "weights",
                    _ => return None,
                };
                Some(format!("\"{name}\":{json}"))
            },
            _ => None,
        };
        if let Some(value) = field {
            self.fields.push(value);
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
            COLOURLIST => {
                let (json, count) = u32_list(chunk, base)?;
                self.record_vertex_list(id, count)?;
                Some(format!("\"colours\":{json}"))
            },
            PACKEDNORMALLIST => {
                let (json, count) = byte_list(chunk, base)?;
                self.record_vertex_list(id, count)?;
                Some(format!("\"packed_normals\":{json}"))
            },
            MATRIXPALETTE => {
                let (json, count) = u32_list(chunk, base)?;
                if self.matrix_palette.replace(count).is_some() {
                    return None;
                }
                Some(format!("\"matrix_palette\":{json}"))
            },
            INDEXLIST => {
                let (json, count) = u32_list(chunk, base)?;
                if self.indices.replace(count).is_some() {
                    return None;
                }
                Some(format!("\"indices\":{json}"))
            },
            MATRIXLIST => {
                let (json, count) = byte4_list(chunk, base)?;
                self.record_vertex_list(id, count)?;
                Some(format!("\"matrices\":{json}"))
            },
            _ => None,
        };
        if let Some(value) = field {
            self.fields.push(value);
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
            UVLIST => {
                let (json, count) = uv_channel(chunk, base)?;
                self.uv_channels.push(json);
                self.vertex_list_counts.push(count);
            },
            MULTICOLOURLIST => {
                let (json, count) = multicolour_channel(chunk, base)?;
                self.multi_colours.push(json);
                self.vertex_list_counts.push(count);
            },
            VERTEXSHADER => {
                if self.vertex_shader_seen {
                    return None;
                }
                let mut reader = Reader::new(chunk, base);
                self.vertex_shader = reader.pstring()?;
                if reader.pos() != chunk.len() {
                    return None;
                }
                self.vertex_shader_seen = true;
            },
            _ => return Some(false),
        }
        Some(true)
    }

    /// Confirms decoded array counts match the primitive-group declarations.
    fn counts_match(&self, header: &PrimitiveHeader) -> bool {
        let vertex_count = usize::try_from(header.vertex_count).ok();
        let index_count = usize::try_from(header.index_count).ok();
        let matrix_count = usize::try_from(header.matrix_count).ok();
        let matrix_count_matches = self
            .matrix_palette
            .map_or(header.matrix_count == 0, |count| {
                header.matrix_count != 0 && Some(count) == matrix_count
            });
        self.positions
            .is_some_and(|count| Some(count) == vertex_count)
            && self.indices.map_or(header.index_count == 0, |count| {
                header.index_count != 0 && Some(count) == index_count
            })
            && self
                .vertex_list_counts
                .iter()
                .all(|count| Some(*count) == vertex_count)
            && matrix_count_matches
    }

    /// Renders one deterministic JSON object after count validation succeeds.
    fn render(
        self,
        header: &PrimitiveHeader,
        source_ordinal: Option<usize>,
    ) -> String {
        let mut output = format!(
            "{{\"shader\":\"{}\",\"vertex_shader\":\"{}\",\"\
             vertex_shader_present\":{},\"prim_type\":{},\"\
             vertex_format\":{},\"vertex_count\":{},\"index_count\":{},\"\
             matrix_count\":{}",
            escape(&header.shader),
            escape(&self.vertex_shader),
            self.vertex_shader_seen,
            header.prim_type,
            header.vertex_format,
            header.vertex_count,
            header.index_count,
            header.matrix_count
        );
        if let Some(source_ordinal) = source_ordinal {
            let _write_result =
                write!(output, ",\"source_ordinal\":{source_ordinal}");
        }
        for field in self.fields {
            output.push(',');
            output.push_str(&field);
        }
        if let Some(bits) = self.position_nonfinite_bits {
            output.push_str(",\"position_nonfinite_f32_bits\":");
            output.push_str(&bits);
        }
        if !self.uv_channels.is_empty() {
            output.push_str(",\"uvs\":[");
            output.push_str(&self.uv_channels.join(","));
            output.push(']');
        }
        if !self.multi_colours.is_empty() {
            output.push_str(",\"multi_colours\":[");
            output.push_str(&self.multi_colours.join(","));
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
    source_ordinal: Option<usize>,
    validate_skin_runtime: bool,
) -> Option<String> {
    let mut reader = Reader::new(chunk, group.data_offset());
    let header = PrimitiveHeader::read(&mut reader)?;
    if reader.pos() != group.header_end()
        || !primitive_index_targets_are_valid(chunk, group, &header)?
        || (validate_skin_runtime
            && !skin_matrix_targets_are_valid(chunk, group, &header)?)
    {
        return None;
    }
    let lists = PrimitiveLists::decode(chunk, group)?;
    if !lists.counts_match(&header) {
        return None;
    }
    Some(lists.render(&header, source_ordinal))
}

/// Validate primitive indices exactly as the runtime loader does.
fn primitive_index_targets_are_valid(
    chunk: &[u8],
    group: &super::reader::SubChunk,
    header: &PrimitiveHeader,
) -> Option<bool> {
    for list in subchunks(chunk, group.header_end(), group.end())? {
        if list.id != INDEXLIST {
            continue;
        }
        let bounded = chunk.get(..list.header_end())?;
        let mut reader = Reader::new(bounded, list.data_offset());
        let count = usize::try_from(reader.u32()?).ok()?;
        for _ in 0..count {
            let index = reader.u32()?;
            if index > u32::from(u16::MAX) || index >= header.vertex_count {
                return Some(false);
            }
        }
        if reader.pos() != bounded.len() {
            return Some(false);
        }
    }
    Some(true)
}

/// Validate skin matrix references exactly as the runtime loader does.
fn skin_matrix_targets_are_valid(
    chunk: &[u8],
    group: &super::reader::SubChunk,
    header: &PrimitiveHeader,
) -> Option<bool> {
    for list in subchunks(chunk, group.header_end(), group.end())? {
        if !matches!(list.id, MATRIXLIST | MATRIXPALETTE) {
            continue;
        }
        let bounded = chunk.get(..list.header_end())?;
        let mut reader = Reader::new(bounded, list.data_offset());
        let count = usize::try_from(reader.u32()?).ok()?;
        if list.id == MATRIXLIST {
            if header.matrix_count == 0 {
                return Some(false);
            }
            for _ in 0..count {
                for _ in 0..4 {
                    if u32::from(reader.byte()?) >= header.matrix_count {
                        return Some(false);
                    }
                }
            }
        } else {
            for _ in 0..count {
                if reader.u32()? >= 256 {
                    return Some(false);
                }
            }
        }
        if reader.pos() != bounded.len() {
            return Some(false);
        }
    }
    Some(true)
}

/// Decode positions while retaining sparse raw bits for non-finite vertices.
fn position_list(
    chunk: &[u8],
    base: usize,
) -> Option<(String, usize, Option<String>)> {
    let mut reader = Reader::new(chunk, base);
    let count = reader.u32()? as usize;
    let mut json = String::with_capacity(count * 24 + 2);
    let mut nonfinite = Vec::new();
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
        if !x.is_finite() || !y.is_finite() || !z.is_finite() {
            nonfinite.push(format!(
                "{{\"vertex_index\":{i},\"xyz\":[{},{},{}]}}",
                x.to_bits(),
                y.to_bits(),
                z.to_bits()
            ));
        }
    }
    json.push(']');
    if reader.pos() != chunk.len() {
        return None;
    }
    let bits =
        (!nonfinite.is_empty()).then(|| format!("[{}]", nonfinite.join(",")));
    Some((json, count, bits))
}

/// `count:u32` then `count` * three `f32`. Returns `(json_array, count)`.
fn float3_list(chunk: &[u8], base: usize) -> Option<(String, usize)> {
    let mut reader = Reader::new(chunk, base);
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
    (reader.pos() == chunk.len()).then_some((json, count))
}

/// `count:u32` then `count` * one `u32`. Returns `(json_array, count)`.
fn u32_list(chunk: &[u8], base: usize) -> Option<(String, usize)> {
    let mut reader = Reader::new(chunk, base);
    let count = reader.u32()? as usize;
    let mut json = String::with_capacity(count * 6 + 2);
    json.push('[');
    for i in 0..count {
        if i > 0 {
            json.push(',');
        }
        json.push_str(&reader.u32()?.to_string());
    }
    json.push(']');
    (reader.pos() == chunk.len()).then_some((json, count))
}

/// `count:u32` then `count` packed-normal bytes.
fn byte_list(chunk: &[u8], base: usize) -> Option<(String, usize)> {
    let mut reader = Reader::new(chunk, base);
    let count = reader.u32()? as usize;
    let mut json = String::with_capacity(count * 4 + 2);
    json.push('[');
    for i in 0..count {
        if i > 0 {
            json.push(',');
        }
        json.push_str(&reader.byte()?.to_string());
    }
    json.push(']');
    (reader.pos() == chunk.len()).then_some((json, count))
}

/// `count:u32` then `count` * four `u8`.
fn byte4_list(chunk: &[u8], base: usize) -> Option<(String, usize)> {
    let mut reader = Reader::new(chunk, base);
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
    (reader.pos() == chunk.len()).then_some((json, count))
}

/// UV list: `count:u32, channel:u32`, then `count` * two `f32`. Tagged with a
/// `"uv":` prefix so the caller can group channels.
fn uv_channel(chunk: &[u8], base: usize) -> Option<(String, usize)> {
    let mut reader = Reader::new(chunk, base);
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
    (reader.pos() == chunk.len()).then_some((
        format!("{{\"channel\":{channel},\"coords\":{coords}}}"),
        count,
    ))
}

/// Multicolour list: `count:u32, channel:u32`, then `count` * one `u32`.
fn multicolour_channel(chunk: &[u8], base: usize) -> Option<(String, usize)> {
    let mut reader = Reader::new(chunk, base);
    let count = reader.u32()? as usize;
    let channel = reader.u32()?;
    let mut values = String::with_capacity(count * 6 + 2);
    values.push('[');
    for i in 0..count {
        if i > 0 {
            values.push(',');
        }
        values.push_str(&reader.u32()?.to_string());
    }
    values.push(']');
    (reader.pos() == chunk.len()).then_some((
        format!("{{\"channel\":{channel},\"values\":{values}}}"),
        count,
    ))
}

/// Format an `f32` as a round-trippable JSON number, or `null` if non-finite.
fn fmt_f32(value: f32) -> String {
    render_f32(value, value.to_string())
}

/// Chunk ids this module owns, for the dispatch table.
pub const IDS: [u32; 2] = [MESH, SKIN];

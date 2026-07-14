// File:
//   - rig.rs
// Path:
//   - src/p3d/src/adapters/driven/decoders/rig.rs
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
//   - The p3d adapter boundary for adapters driven decoders rig.
// - Must-Not:
//   - Change domain semantics or bypass application and port contracts.
// - Allows:
//   - Filesystem, JSON, CLI, Blender, or serialization work behind explicit
//   - ports.
// - Split-When:
//   - Split when rig contains two independently testable contracts.
// - Merge-When:
//   - Another p3d module owns the same adapters boundary with no distinct
//   - invariant.
// - Summary:
//   - Rig and motion decoders for lossless P3D recovery.
// - Description:
//   - Defines rig data and behavior for p3d adapters driven decoders.
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
//   - Reason: src/p3d/src/adapters/driven/decoders/rig.rs has 1568 effective
//   - lines after the required header and remains cohesive until a focused
//   - split
//   - lands.
//

//! Rig and motion decoders for lossless P3D recovery.
//!
//! These chunks encode skeleton hierarchy, animation tracks, controller timing,
//! and vertex-offset keys. The decoder validates declared counts before it
//! returns decoded JSON so malformed payloads fail closed before package output
//! is written.
use super::reader::{Reader, SubChunk, read_u32, subchunks};
use crate::adapters::driven::json::{escape_json as escape, render_f32};

/// Pins `SKELETON` because exact identifiers govern binary dispatch.
const SKELETON: u32 = 0x0000_4500;
/// Pins `SKELETON_JOINT` because exact identifiers govern binary dispatch.
const SKELETON_JOINT: u32 = 0x0000_4501;
/// Pins `ANIMATION` because exact identifiers govern binary dispatch.
const ANIMATION: u32 = 0x0012_1000;
/// Pins `ANIMATION_GROUP` because exact identifiers govern binary dispatch.
const ANIMATION_GROUP: u32 = 0x0012_1001;
/// Pins `ANIMATION_GROUP_LIST` because exact identifiers govern binary
/// dispatch.
const ANIMATION_GROUP_LIST: u32 = 0x0012_1002;
/// Pins `ANIMATION_SIZE` because exact identifiers govern binary dispatch.
const ANIMATION_SIZE: u32 = 0x0012_1004;
/// Pins `CHANNEL_FLOAT1` because exact identifiers govern binary dispatch.
const CHANNEL_FLOAT1: u32 = 0x0012_1100;
/// Pins `CHANNEL_FLOAT2` because exact identifiers govern binary dispatch.
const CHANNEL_FLOAT2: u32 = 0x0012_1101;
/// Pins `CHANNEL_VECTOR1` because exact identifiers govern binary dispatch.
const CHANNEL_VECTOR1: u32 = 0x0012_1102;
/// Pins `CHANNEL_VECTOR2` because exact identifiers govern binary dispatch.
const CHANNEL_VECTOR2: u32 = 0x0012_1103;
/// Pins `CHANNEL_VECTOR3` because exact identifiers govern binary dispatch.
const CHANNEL_VECTOR3: u32 = 0x0012_1104;
/// Pins `CHANNEL_QUATERNION` because exact identifiers govern binary dispatch.
const CHANNEL_QUATERNION: u32 = 0x0012_1105;
/// Pins `CHANNEL_COMPRESSED_QUATERNION` because exact identifiers govern binary
/// dispatch.
const CHANNEL_COMPRESSED_QUATERNION: u32 = 0x0012_1111;
/// Pins `CHANNEL_INTERPOLATION_MODE` because exact identifiers govern binary
/// dispatch.
const CHANNEL_INTERPOLATION_MODE: u32 = 0x0012_1110;
/// Pins `JOINT_MIRROR_MAP` because exact identifiers govern binary dispatch.
const JOINT_MIRROR_MAP: u32 = 0x0000_4503;
/// Pins `JOINT_FIX_FLAG` because exact identifiers govern binary dispatch.
const JOINT_FIX_FLAG: u32 = 0x0000_4504;
/// Pins `LEGACY_ANIMATION_EXTRA` because exact identifiers govern binary
/// dispatch.
const LEGACY_ANIMATION_EXTRA: u32 = 0x0012_1006;
/// Pins `CHANNEL_STRING` because exact identifiers govern binary dispatch.
const CHANNEL_STRING: u32 = 0x0012_1106;
/// Pins `CHANNEL_ENTITY` because exact identifiers govern binary dispatch.
const CHANNEL_ENTITY: u32 = 0x0012_1107;
/// Pins `CHANNEL_BOOL` because exact identifiers govern binary dispatch.
const CHANNEL_BOOL: u32 = 0x0012_1108;
/// Pins `CHANNEL_COLOUR` because exact identifiers govern binary dispatch.
const CHANNEL_COLOUR: u32 = 0x0012_1109;
/// Pins `CHANNEL_INT` because exact identifiers govern binary dispatch.
const CHANNEL_INT: u32 = 0x0012_110e;
/// Pins `MULTI_CONTROLLER` because exact identifiers govern binary dispatch.
const MULTI_CONTROLLER: u32 = 0x0000_48a0;
/// Pins `MULTI_CONTROLLER_TRACKS` because exact identifiers govern binary
/// dispatch.
const MULTI_CONTROLLER_TRACKS: u32 = 0x0000_48a1;
/// Pins `MULTI_CONTROLLER_TRACK` because exact identifiers govern binary
/// dispatch.
const MULTI_CONTROLLER_TRACK: u32 = 0x0000_48a2;
/// Pins `VERTEX_KEY` because exact identifiers govern binary dispatch.
const VERTEX_KEY: u32 = 0x0012_1304;
/// Pins `VERTEX_COLOUR_OFFSETS` because exact identifiers govern binary
/// dispatch.
const VERTEX_COLOUR_OFFSETS: u32 = 0x0012_1300;
/// Pins `VERTEX_VECTOR_OFFSETS` because exact identifiers govern binary
/// dispatch.
const VERTEX_VECTOR_OFFSETS: u32 = 0x0012_1301;
/// Pins `VERTEX_VECTOR2_OFFSETS` because exact identifiers govern binary
/// dispatch.
const VERTEX_VECTOR2_OFFSETS: u32 = 0x0012_1302;
/// Pins `VERTEX_INDEX_OFFSETS` because exact identifiers govern binary
/// dispatch.
const VERTEX_INDEX_OFFSETS: u32 = 0x0012_1303;

/// Decode a skeleton and its joint hierarchy.
#[must_use]
pub fn skeleton_json(chunk: &[u8]) -> Option<String> {
    let (_, header_size, total_size) = require_id(
        chunk, SKELETON,
    )?;
    let mut reader = Reader::new(
        chunk, 12,
    );
    let name = reader.pstring()?;
    let version = reader.u32()?;
    let joint_count = u32_to_usize(reader.u32()?)?;
    if reader.pos() != header_size {
        return None;
    }
    let children = subchunks(
        chunk,
        header_size,
        total_size,
    )?;
    if children.len() != joint_count {
        return None;
    }
    let mut joints = Vec::new();
    for child in children {
        if child.id != SKELETON_JOINT {
            return None;
        }
        joints.push(
            decode_joint(
                chunk, &child,
            )?,
        );
    }
    Some(
        format!(
            "{{\"schema\":\"skeleton\",\"name\":\"{}\",\"version\":{},\"\
             num_joints\":{},\"joints\":[{}]}}\n",
            escape(&name),
            version,
            joint_count,
            joints.join(",")
        ),
    )
}

/// Decode an animation and its group/channel key data.
#[must_use]
pub fn animation_json(chunk: &[u8]) -> Option<String> {
    let (_, header_size, total_size) = require_id(
        chunk, ANIMATION,
    )?;
    let mut reader = Reader::new(
        chunk, 12,
    );
    let version = reader.u32()?;
    let name = reader.pstring()?;
    let animation_type = read_fourcc(&mut reader)?;
    let frames = reader.f32()?;
    let frame_rate = reader.f32()?;
    let cyclic = reader.u32()?;
    if reader.pos() != header_size {
        return None;
    }
    let children = subchunks(
        chunk,
        header_size,
        total_size,
    )?;
    let mut sizes = Vec::new();
    let mut group_lists = Vec::new();
    let mut loose_channels = Vec::new();
    let mut legacy_animation_extras = Vec::new();
    for child in children {
        match child.id {
            ANIMATION_SIZE => sizes.push(
                decode_animation_size(
                    chunk, &child,
                )?,
            ),
            ANIMATION_GROUP_LIST => group_lists.push(
                decode_group_list(
                    chunk, &child,
                )?,
            ),
            id if channel_kind(id).is_some() => loose_channels.push(
                decode_channel(
                    chunk, &child,
                )?,
            ),
            _ => legacy_animation_extras.push(
                decode_animation_extra_child(
                    chunk, &child,
                )?,
            ),
        }
    }
    Some(
        format!(
            concat!(
                "{{\"schema\":\"animation\",\"name\":\"{}\",",
                "\"version\":{},\"type\":\"{}\",\"frames\":{},",
                "\"frame_rate\":{},\"cyclic\":{},\"sizes\":[{}],",
                "\"group_lists\":[{}],\"loose_channels\":[{}],",
                "\"legacy_animation_extras\":[{}]}}
"
            ),
            escape(&name),
            version,
            escape(&animation_type),
            fmt_f32(frames),
            fmt_f32(frame_rate),
            cyclic,
            sizes.join(","),
            group_lists.join(","),
            loose_channels.join(","),
            legacy_animation_extras.join(",")
        ),
    )
}

/// Decode multi-controller track timing data.
#[must_use]
pub fn multi_controller_json(chunk: &[u8]) -> Option<String> {
    let (_, header_size, total_size) = require_id(
        chunk,
        MULTI_CONTROLLER,
    )?;
    let mut reader = Reader::new(
        chunk, 12,
    );
    let name = reader.pstring()?;
    let version = reader.u32()?;
    let length = reader.f32()?;
    let frame_rate = reader.f32()?;
    let track_count = u32_to_usize(reader.u32()?)?;
    if reader.pos() != header_size {
        return None;
    }
    let children = subchunks(
        chunk,
        header_size,
        total_size,
    )?;
    let mut tracks = Vec::new();
    for child in children {
        match child.id {
            MULTI_CONTROLLER_TRACK => tracks.push(
                decode_multi_track(
                    chunk, &child,
                )?,
            ),
            MULTI_CONTROLLER_TRACKS => tracks.extend(
                decode_multi_tracks(
                    chunk, &child,
                )?,
            ),
            _ => return None,
        }
    }
    if tracks.len() != track_count {
        return None;
    }
    Some(
        format!(
            concat!(
                "{{\"schema\":\"multi_controller\",\"name\":\"{}\",",
                "\"version\":{},\"length\":{},\"framerate\":{},",
                "\"num_tracks\":{},\"tracks\":[{}]}}\n"
            ),
            escape(&name),
            version,
            fmt_f32(length),
            fmt_f32(frame_rate),
            track_count,
            tracks.join(",")
        ),
    )
}

/// Decode a vertex animation key and its offset lists.
#[must_use]
pub fn vertex_key_json(chunk: &[u8]) -> Option<String> {
    let (_, header_size, total_size) = require_id(
        chunk, VERTEX_KEY,
    )?;
    let mut reader = Reader::new(
        chunk, 12,
    );
    let version = reader.u32()?;
    let name = reader.pstring()?;
    if reader.pos() != header_size {
        return None;
    }
    let children = subchunks(
        chunk,
        header_size,
        total_size,
    )?;
    let mut lists = Vec::new();
    for child in children {
        lists.push(
            decode_vertex_offset_list(
                chunk, &child,
            )?,
        );
    }
    let json = format!(
        "{{\"schema\":\"vertex_anim_key\",\"name\":\"{}\",\"version\":{},\"\
         offset_lists\":[{}]}}\n",
        escape(&name),
        version,
        lists.join(",")
    );
    Some(json)
}

/// Keeps `decode_joint` local because it shares the rig binary-layout
/// invariant.
fn decode_joint(
    chunk: &[u8],
    child: &SubChunk,
) -> Option<String> {
    let mut reader = Reader::new(
        chunk,
        child.data_offset(),
    );
    let name = reader.pstring()?;
    let parent = reader.u32()?;
    let dof = reader.u32()?;
    let free_axes = reader.u32()?;
    let primary_axis = reader.u32()?;
    let secondary_axis = reader.u32()?;
    let twist_axis = reader.u32()?;
    let rest_pose = read_matrix(&mut reader)?;
    if reader.pos() != child.header_end() {
        return None;
    }
    let joint_metadata = decode_joint_metadata(
        chunk, child,
    )?;
    Some(
        format!(
            concat!(
                "{{\"name\":\"{}\",\"parent\":{},\"dof\":{},",
                "\"free_axes\":{},\"primary_axis\":{},",
                "\"secondary_axis\":{},\"twist_axis\":{},",
                "\"rest_pose\":[{}],\"joint_metadata\":[{}]}}"
            ),
            escape(&name),
            parent,
            dof,
            free_axes,
            primary_axis,
            secondary_axis,
            twist_axis,
            rest_pose.join(","),
            joint_metadata.join(",")
        ),
    )
}

/// Keeps `decode_animation_extra_child` local because it shares the rig
/// binary-layout invariant.
fn decode_animation_extra_child(
    chunk: &[u8],
    child: &SubChunk,
) -> Option<String> {
    if child.id != LEGACY_ANIMATION_EXTRA {
        return None;
    }
    let mut reader = Reader::new(
        chunk,
        child.data_offset(),
    );
    let version = reader.u32()?;
    let entry_count = u32_to_usize(reader.u32()?)?;
    let mut entries = Vec::new();
    for _ in 0..entry_count {
        let entry_chunk_id = reader.u32()?;
        let entry_header_size = reader.u32()?;
        let entry_total_size = reader.u32()?;
        let entry_version = reader.u32()?;
        let channel_chunk_id = reader.u32()?;
        let channel_count = reader.u32()?;
        let value = read_u16_from_reader(&mut reader)?;
        entries.push(
            format!(
                concat!(
                    "{{\"entry_chunk_id\":{},\"entry_header_size\":{},",
                    "\"entry_total_size\":{},\"entry_version\":{},",
                    "\"channel_chunk_id\":{},\"channel_count\":{},",
                    "\"value\":{}}}"
                ),
                entry_chunk_id,
                entry_header_size,
                entry_total_size,
                entry_version,
                channel_chunk_id,
                channel_count,
                value
            ),
        );
    }
    if reader.pos() != child.header_end() {
        return None;
    }
    Some(
        format!(
            "{{\"kind\":\"legacy_animation_extra\",\"version\":{},\"\
             entry_count\":{},\"entries\":[{}]}}",
            version,
            entry_count,
            entries.join(",")
        ),
    )
}

/// Keeps `decode_animation_size` local because it shares the rig binary-layout
/// invariant.
fn decode_animation_size(
    chunk: &[u8],
    child: &SubChunk,
) -> Option<String> {
    let data_offset = child.data_offset();
    let mut reader = Reader::new(
        chunk,
        data_offset,
    );
    let version = reader.u32()?;
    let pc = reader.u32()?;
    let ps2 = reader.u32()?;
    let xbox = reader.u32()?;
    let gc = reader.u32()?;
    if !is_leaf_at(
        reader.pos(),
        child,
    ) {
        return None;
    }
    Some(
        format!(
            "{{\"version\":{version},\"pc\":{pc},\"ps2\":{ps2},\"xbox\":\
             {xbox},\"gc\":{gc}}}"
        ),
    )
}

/// Keeps `decode_group_list` local because it shares the rig binary-layout
/// invariant.
fn decode_group_list(
    chunk: &[u8],
    child: &SubChunk,
) -> Option<String> {
    let mut reader = Reader::new(
        chunk,
        child.data_offset(),
    );
    let version = reader.u32()?;
    let group_count = u32_to_usize(reader.u32()?)?;
    if reader.pos() != child.header_end() {
        return None;
    }
    let children = subchunks(
        chunk,
        child.header_end(),
        child.end(),
    )?;
    if children.len() != group_count {
        return None;
    }
    let mut groups = Vec::new();
    for group in children {
        if group.id != ANIMATION_GROUP {
            return None;
        }
        groups.push(
            decode_group(
                chunk, &group,
            )?,
        );
    }
    Some(
        format!(
            "{{\"version\":{},\"num_groups\":{},\"groups\":[{}]}}",
            version,
            group_count,
            groups.join(",")
        ),
    )
}

/// Keeps `decode_group` local because it shares the rig binary-layout
/// invariant.
fn decode_group(
    chunk: &[u8],
    child: &SubChunk,
) -> Option<String> {
    let mut reader = Reader::new(
        chunk,
        child.data_offset(),
    );
    let version = reader.u32()?;
    let name = reader.pstring()?;
    let group_id = reader.u32()?;
    let channel_count = u32_to_usize(reader.u32()?)?;
    if reader.pos() != child.header_end() {
        return None;
    }
    let children = subchunks(
        chunk,
        child.header_end(),
        child.end(),
    )?;
    if children.len() != channel_count {
        return None;
    }
    let mut channels = Vec::new();
    for channel in children {
        channels.push(
            decode_channel(
                chunk, &channel,
            )?,
        );
    }
    Some(
        format!(
            "{{\"version\":{},\"name\":\"{}\",\"group_id\":{},\"num_channels\"\
             :{},\"channels\":[{}]}}",
            version,
            escape(&name),
            group_id,
            channel_count,
            channels.join(",")
        ),
    )
}

/// Keeps `decode_channel` local because it shares the rig binary-layout
/// invariant.
fn decode_channel(
    chunk: &[u8],
    child: &SubChunk,
) -> Option<String> {
    let kind = channel_kind(child.id)?;
    let mut reader = Reader::new(
        chunk,
        child.data_offset(),
    );
    let version = reader.u32()?;
    let param = read_fourcc(&mut reader)?;
    let body = match child.id {
        CHANNEL_FLOAT1 => decode_key_values(
            &mut reader,
            1,
        )?,
        CHANNEL_FLOAT2 => decode_key_values(
            &mut reader,
            2,
        )?,
        CHANNEL_VECTOR1 => decode_vector_dof_keys(
            &mut reader,
            1,
        )?,
        CHANNEL_VECTOR2 => decode_vector_dof_keys(
            &mut reader,
            2,
        )?,
        CHANNEL_VECTOR3 => decode_key_values(
            &mut reader,
            3,
        )?,
        CHANNEL_QUATERNION => decode_key_values(
            &mut reader,
            4,
        )?,
        CHANNEL_COMPRESSED_QUATERNION => {
            decode_compressed_quaternion_keys(&mut reader)?
        }
        CHANNEL_INT | CHANNEL_COLOUR => decode_integer_keys(&mut reader)?,
        CHANNEL_STRING | CHANNEL_ENTITY => decode_string_keys(&mut reader)?,
        CHANNEL_BOOL => decode_bool_keys(&mut reader)?,
        _ => return None,
    };
    if reader.pos() != child.header_end() {
        return None;
    }
    let channel_metadata = decode_channel_metadata(
        chunk, child,
    )?;
    Some(
        format!(
            "{{\"kind\":\"{}\",\"version\":{},\"param\":\"{}\",{},\"\
             channel_metadata\":[{}]}}",
            kind,
            version,
            escape(&param),
            body,
            channel_metadata.join(",")
        ),
    )
}

/// Keeps `decode_compressed_quaternion_keys` local because it shares the rig
/// binary-layout invariant.
fn decode_compressed_quaternion_keys(
    reader: &mut Reader<'_>
) -> Option<String> {
    let count = u32_to_usize(reader.u32()?)?;
    let frames = read_frames(
        reader, count,
    )?;
    let mut values = Vec::new();
    for _ in 0..count {
        let mut value = Vec::new();
        for _ in 0..4_usize {
            value.push(read_u16_from_reader(reader)?.to_string());
        }
        values.push(
            format!(
                "[{}]",
                value.join(",")
            ),
        );
    }
    Some(
        format!(
            "\"num_frames\":{},\"frames\":[{}],\"compressed_values\":[{}]",
            count,
            frames.join(","),
            values.join(",")
        ),
    )
}

/// Keeps `decode_vector_dof_keys` local because it shares the rig binary-layout
/// invariant.
fn decode_vector_dof_keys(
    reader: &mut Reader<'_>,
    width: usize,
) -> Option<String> {
    let mapping = read_u16_from_reader(reader)?;
    let constants = read_vector_values(
        reader, 1, 3,
    )?
    .into_iter()
    .next()?;
    let count = u32_to_usize(reader.u32()?)?;
    let frames = read_frames(
        reader, count,
    )?;
    let values = read_vector_values(
        reader, count, width,
    )?;
    Some(
        format!(
            concat!(
                "\"mapping\":{},\"constants\":{},",
                "\"num_frames\":{},\"frames\":[{}],",
                "\"values\":[{}]"
            ),
            mapping,
            constants,
            count,
            frames.join(","),
            values.join(",")
        ),
    )
}

/// Keeps `decode_key_values` local because it shares the rig binary-layout
/// invariant.
fn decode_key_values(
    reader: &mut Reader<'_>,
    width: usize,
) -> Option<String> {
    let count = u32_to_usize(reader.u32()?)?;
    let frames = read_frames(
        reader, count,
    )?;
    let mut values = Vec::new();
    for _ in 0..count {
        let mut value = Vec::new();
        for _ in 0..width {
            value.push(fmt_f32(reader.f32()?));
        }
        values.push(
            format!(
                "[{}]",
                value.join(",")
            ),
        );
    }
    Some(
        format!(
            "\"num_frames\":{},\"frames\":[{}],\"values\":[{}]",
            count,
            frames.join(","),
            values.join(",")
        ),
    )
}

/// Keeps `decode_integer_keys` local because it shares the rig binary-layout
/// invariant.
fn decode_integer_keys(reader: &mut Reader<'_>) -> Option<String> {
    let count = u32_to_usize(reader.u32()?)?;
    let frames = read_frames(
        reader, count,
    )?;
    let mut values = Vec::new();
    for _ in 0..count {
        values.push(
            reader
                .u32()?
                .to_string(),
        );
    }
    Some(
        format!(
            "\"num_frames\":{},\"frames\":[{}],\"values\":[{}]",
            count,
            frames.join(","),
            values.join(",")
        ),
    )
}

/// Keeps `decode_string_keys` local because it shares the rig binary-layout
/// invariant.
fn decode_string_keys(reader: &mut Reader<'_>) -> Option<String> {
    let count = u32_to_usize(reader.u32()?)?;
    let frames = read_frames(
        reader, count,
    )?;
    let mut values = Vec::new();
    for _ in 0..count {
        values.push(
            format!(
                "\"{}\"",
                escape(&reader.pstring()?)
            ),
        );
    }
    Some(
        format!(
            "\"num_frames\":{},\"frames\":[{}],\"values\":[{}]",
            count,
            frames.join(","),
            values.join(",")
        ),
    )
}

/// Keeps `decode_bool_keys` local because it shares the rig binary-layout
/// invariant.
fn decode_bool_keys(reader: &mut Reader<'_>) -> Option<String> {
    let start_state = read_u16_from_reader(reader)?;
    let count = u32_to_usize(reader.u32()?)?;
    let mut values = Vec::new();
    for _ in 0..count {
        values.push(read_u16_from_reader(reader)?.to_string());
    }
    Some(
        format!(
            "\"start_state\":{},\"num_frames\":{},\"values\":[{}]",
            start_state,
            count,
            values.join(",")
        ),
    )
}

/// Keeps `read_frames` local because it shares the rig binary-layout invariant.
fn read_frames(
    reader: &mut Reader<'_>,
    count: usize,
) -> Option<Vec<String>> {
    let mut frames = Vec::new();
    for _ in 0..count {
        frames.push(read_u16_from_reader(reader)?.to_string());
    }
    Some(frames)
}

/// Keeps `channel_kind` local because it shares the rig binary-layout
/// invariant.
const fn channel_kind(id: u32) -> Option<&'static str> {
    match id {
        CHANNEL_FLOAT1 => Some("float1"),
        CHANNEL_FLOAT2 => Some("float2"),
        CHANNEL_VECTOR1 => Some("vector1"),
        CHANNEL_VECTOR2 => Some("vector2"),
        CHANNEL_VECTOR3 => Some("vector3"),
        CHANNEL_QUATERNION => Some("quaternion"),
        CHANNEL_COMPRESSED_QUATERNION => Some("compressed_quaternion"),
        CHANNEL_STRING => Some("string"),
        CHANNEL_ENTITY => Some("entity"),
        CHANNEL_BOOL => Some("bool"),
        CHANNEL_COLOUR => Some("colour"),
        CHANNEL_INT => Some("int"),
        _ => None,
    }
}

/// Keeps `decode_multi_tracks` local because it shares the rig binary-layout
/// invariant.
fn decode_multi_tracks(
    chunk: &[u8],
    child: &SubChunk,
) -> Option<Vec<String>> {
    let mut reader = Reader::new(
        chunk,
        child.data_offset(),
    );
    let count = u32_to_usize(reader.u32()?)?;
    let mut tracks = Vec::new();
    for _ in 0..count {
        tracks.push(decode_track_fields(&mut reader)?);
    }
    if !is_leaf_at(
        reader.pos(),
        child,
    ) {
        return None;
    }
    Some(tracks)
}

/// Keeps `decode_multi_track` local because it shares the rig binary-layout
/// invariant.
fn decode_multi_track(
    chunk: &[u8],
    child: &SubChunk,
) -> Option<String> {
    let mut reader = Reader::new(
        chunk,
        child.data_offset(),
    );
    let track = decode_track_fields(&mut reader)?;
    if !is_leaf_at(
        reader.pos(),
        child,
    ) {
        return None;
    }
    Some(track)
}

/// Keeps `decode_track_fields` local because it shares the rig binary-layout
/// invariant.
fn decode_track_fields(reader: &mut Reader<'_>) -> Option<String> {
    let name = reader.pstring()?;
    let start_time = reader.f32()?;
    let end_time = reader.f32()?;
    let scale = reader.f32()?;
    Some(
        format!(
            "{{\"name\":\"{}\",\"start_time\":{},\"end_time\":{},\"scale\":\
             {}}}",
            escape(&name),
            fmt_f32(start_time),
            fmt_f32(end_time),
            fmt_f32(scale)
        ),
    )
}

/// Keeps `decode_vertex_offset_list` local because it shares the rig
/// binary-layout invariant.
fn decode_vertex_offset_list(
    chunk: &[u8],
    child: &SubChunk,
) -> Option<String> {
    let kind = vertex_list_kind(child.id)?;
    let mut reader = Reader::new(
        chunk,
        child.data_offset(),
    );
    let version = reader.u32()?;
    let count = u32_to_usize(reader.u32()?)?;
    let param = if matches!(
        child.id,
        VERTEX_VECTOR_OFFSETS | VERTEX_VECTOR2_OFFSETS
    ) {
        Some(read_fourcc(&mut reader)?)
    } else {
        None
    };
    let values = match child.id {
        VERTEX_COLOUR_OFFSETS => read_u32_values(
            &mut reader,
            count,
        )?,
        VERTEX_VECTOR_OFFSETS => read_vector_values(
            &mut reader,
            count,
            3,
        )?,
        VERTEX_VECTOR2_OFFSETS => read_vector_values(
            &mut reader,
            count,
            2,
        )?,
        _ => return None,
    };
    if reader.pos() != child.header_end() {
        return None;
    }
    let indices = decode_vertex_indices(
        chunk, child,
    )?;
    let param_json = param.map_or_else(
        String::new,
        |value| {
            format!(
                ",\"param\":\"{}\"",
                escape(&value)
            )
        },
    );
    Some(
        format!(
            "{{\"kind\":\"{}\",\"version\":{}{},\"num_offsets\":{},\"offsets\"\
             :[{}],\"indices\":[{}]}}",
            kind,
            version,
            param_json,
            count,
            values.join(","),
            indices.join(",")
        ),
    )
}

/// Keeps `decode_vertex_indices` local because it shares the rig binary-layout
/// invariant.
fn decode_vertex_indices(
    chunk: &[u8],
    child: &SubChunk,
) -> Option<Vec<String>> {
    let children = subchunks(
        chunk,
        child.header_end(),
        child.end(),
    )?;
    let mut lists = Vec::new();
    for index_list in children {
        if index_list.id != VERTEX_INDEX_OFFSETS {
            return None;
        }
        let mut reader = Reader::new(
            chunk,
            index_list.data_offset(),
        );
        let version = reader.u32()?;
        let count = u32_to_usize(reader.u32()?)?;
        let values = read_u32_values(
            &mut reader,
            count,
        )?;
        if !is_leaf_at(
            reader.pos(),
            &index_list,
        ) {
            return None;
        }
        lists.push(
            format!(
                "{{\"version\":{},\"num_indices\":{},\"indices\":[{}]}}",
                version,
                count,
                values.join(",")
            ),
        );
    }
    Some(lists)
}

/// Keeps `vertex_list_kind` local because it shares the rig binary-layout
/// invariant.
const fn vertex_list_kind(id: u32) -> Option<&'static str> {
    match id {
        VERTEX_COLOUR_OFFSETS => Some("colour"),
        VERTEX_VECTOR_OFFSETS => Some("vector"),
        VERTEX_VECTOR2_OFFSETS => Some("vector2"),
        _ => None,
    }
}

/// Keeps `read_u32_values` local because it shares the rig binary-layout
/// invariant.
fn read_u32_values(
    reader: &mut Reader<'_>,
    count: usize,
) -> Option<Vec<String>> {
    let mut values = Vec::new();
    for _ in 0..count {
        values.push(
            reader
                .u32()?
                .to_string(),
        );
    }
    Some(values)
}

/// Keeps `read_vector_values` local because it shares the rig binary-layout
/// invariant.
fn read_vector_values(
    reader: &mut Reader<'_>,
    count: usize,
    width: usize,
) -> Option<Vec<String>> {
    let mut values = Vec::new();
    for _ in 0..count {
        let mut value = Vec::new();
        for _ in 0..width {
            value.push(fmt_f32(reader.f32()?));
        }
        values.push(
            format!(
                "[{}]",
                value.join(",")
            ),
        );
    }
    Some(values)
}

/// Keeps `read_matrix` local because it shares the rig binary-layout invariant.
fn read_matrix(reader: &mut Reader<'_>) -> Option<Vec<String>> {
    let mut values = Vec::new();
    for _ in 0_usize..16_usize {
        values.push(fmt_f32(reader.f32()?));
    }
    Some(values)
}

/// Keeps `decode_channel_metadata` local because it shares the rig
/// binary-layout invariant.
fn decode_channel_metadata(
    chunk: &[u8],
    child: &SubChunk,
) -> Option<Vec<String>> {
    let children = subchunks(
        chunk,
        child.header_end(),
        child.end(),
    )?;
    children
        .iter()
        .map(
            |metadata| {
                decode_channel_metadata_child(
                    chunk, metadata,
                )
            },
        )
        .collect()
}

/// Keeps `decode_channel_metadata_child` local because it shares the rig
/// binary-layout invariant.
fn decode_channel_metadata_child(
    chunk: &[u8],
    child: &SubChunk,
) -> Option<String> {
    if child.id != CHANNEL_INTERPOLATION_MODE {
        return None;
    }
    let mut reader = Reader::new(
        chunk,
        child.data_offset(),
    );
    let version = reader.u32()?;
    let mode = reader.u32()?;
    let consumed_header = reader.pos() == child.header_end();
    let has_no_payload_children = child.header_end() == child.end();
    if !(consumed_header && has_no_payload_children) {
        return None;
    }
    Some(
        format!(
            "{{\"kind\":\"interpolation_mode\",\"version\":{version},\"mode\":\
             {mode}}}"
        ),
    )
}

/// Keeps `decode_joint_metadata` local because it shares the rig binary-layout
/// invariant.
fn decode_joint_metadata(
    chunk: &[u8],
    child: &SubChunk,
) -> Option<Vec<String>> {
    let children = subchunks(
        chunk,
        child.header_end(),
        child.end(),
    )?;
    children
        .iter()
        .map(
            |metadata| {
                decode_joint_metadata_child(
                    chunk, metadata,
                )
            },
        )
        .collect()
}

/// Keeps `decode_joint_metadata_child` local because it shares the rig
/// binary-layout invariant.
fn decode_joint_metadata_child(
    chunk: &[u8],
    child: &SubChunk,
) -> Option<String> {
    let mut reader = Reader::new(
        chunk,
        child.data_offset(),
    );
    match child.id {
        JOINT_MIRROR_MAP => {
            let index = reader.u32()?;
            let scale = read_vector_values(
                &mut reader,
                1,
                3,
            )?
            .into_iter()
            .next()?;
            let consumed_header = reader.pos() == child.header_end();
            let has_no_payload_children = child.header_end() == child.end();
            if !(consumed_header && has_no_payload_children) {
                return None;
            }
            Some(
                format!(
                    "{{\"kind\":\"joint_mirror_map\",\"index\":{index},\"\
                     scale\":{scale}}}"
                ),
            )
        }
        JOINT_FIX_FLAG => {
            let flags = reader.u32()?;
            let consumed_header = reader.pos() == child.header_end();
            let has_no_payload_children = child.header_end() == child.end();
            if !(consumed_header && has_no_payload_children) {
                return None;
            }
            Some(format!("{{\"kind\":\"joint_fix_flag\",\"flags\":{flags}}}"))
        }
        _ => None,
    }
}

/// Keeps `require_id` local because it shares the rig binary-layout invariant.
fn require_id(
    chunk: &[u8],
    expected: u32,
) -> Option<(
    u32,
    usize,
    usize,
)> {
    let bounds = chunk_bounds(chunk)?;
    (bounds.0 == expected).then_some(bounds)
}

/// Keeps `chunk_bounds` local because it shares the rig binary-layout
/// invariant.
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
    let header_size = u32_to_usize(
        read_u32(
            chunk, 4,
        )?,
    )?;
    let total_size = u32_to_usize(
        read_u32(
            chunk, 8,
        )?,
    )?;
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

/// Keeps `read_fourcc` local because it shares the rig binary-layout invariant.
fn read_fourcc(reader: &mut Reader<'_>) -> Option<String> {
    let value = reader.u32()?;
    let bytes = value.to_le_bytes();
    let mut output = String::new();
    for byte in bytes {
        let ch = char::from(byte);
        if ch.is_ascii_graphic() || ch == ' ' {
            output.push(ch);
        } else {
            output.push('_');
        }
    }
    Some(
        output
            .trim_end_matches(char::from(0))
            .to_owned(),
    )
}

/// Keeps `read_u16_from_reader` local because it shares the rig binary-layout
/// invariant.
fn read_u16_from_reader(reader: &mut Reader<'_>) -> Option<u16> {
    let first = reader.byte()?;
    let second = reader.byte()?;
    Some(
        u16::from_le_bytes(
            [
                first, second,
            ],
        ),
    )
}

/// Keeps `u32_to_usize` local because it shares the rig binary-layout
/// invariant.
fn u32_to_usize(value: u32) -> Option<usize> {
    usize::try_from(value).ok()
}

/// Keeps `is_leaf_at` local because it shares the rig binary-layout invariant.
fn is_leaf_at(
    pos: usize,
    child: &SubChunk,
) -> bool {
    pos == child.header_end() && child.header_end() == child.end()
}

/// Keeps `fmt_f32` local because it shares the rig binary-layout invariant.
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

    /// Builds a synthetic chunk for rig decoder tests.
    fn chunk(
        id: u32,
        fields: Vec<u8>,
        children: Vec<Vec<u8>>,
    ) -> Option<Vec<u8>> {
        let children_len = children
            .iter()
            .try_fold(
                0_usize,
                |acc, child| acc.checked_add(child.len()),
            )?;
        let header_len = fields
            .len()
            .checked_add(12)?;
        let total_len = header_len.checked_add(children_len)?;
        let mut out = Vec::with_capacity(total_len);
        for word in [
            id,
            u32::try_from(header_len).ok()?,
            u32::try_from(total_len).ok()?,
        ] {
            out.extend_from_slice(&word.to_le_bytes());
        }
        out.extend(fields);
        for child in children {
            out.extend(child);
        }
        Some(out)
    }

    /// Builds a Pure3D-style test string payload.
    fn pstring(value: &str) -> Option<Vec<u8>> {
        let length = u8::try_from(value.len()).ok()?;
        let mut out = Vec::new();
        out.push(length);
        out.extend_from_slice(value.as_bytes());
        Some(out)
    }

    /// Builds a little-endian integer field.
    fn u32_field(value: u32) -> Vec<u8> {
        value
            .to_le_bytes()
            .to_vec()
    }

    /// Builds a little-endian short field.
    fn u16_field(value: u16) -> Vec<u8> {
        value
            .to_le_bytes()
            .to_vec()
    }

    /// Builds a little-endian float field.
    fn f32_field(value: f32) -> Vec<u8> {
        value
            .to_le_bytes()
            .to_vec()
    }

    /// Builds a fixed-width fourcc field.
    fn fourcc(value: &str) -> Option<Vec<u8>> {
        let mut out = vec![0_u8; 4];
        for (index, byte) in value
            .as_bytes()
            .iter()
            .take(4)
            .enumerate()
        {
            *out.get_mut(index)? = *byte;
        }
        Some(out)
    }

    /// Joins field fragments in schema order.
    fn fields(parts: Vec<Vec<u8>>) -> Vec<u8> {
        let mut out = Vec::new();
        for part in parts {
            out.extend(part);
        }
        out
    }

    /// Builds an identity matrix fixture.
    fn identity_matrix() -> Vec<u8> {
        (0_usize..16_usize)
            .flat_map(
                |index| {
                    let value = match index {
                        0 | 5 | 10 | 15 => 1.0_f32,
                        _ => 0.0_f32,
                    };
                    value.to_le_bytes()
                },
            )
            .collect()
    }

    /// Converts optional fixtures into descriptive test errors.
    fn require<T>(
        value: Option<T>,
        context: &str,
    ) -> Result<T, String> {
        value.ok_or_else(|| String::from(context))
    }

    /// Checks useful JSON output without panic-based test construction.
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

    /// Builds a skeleton fixture with one joint and rest pose.
    fn skeleton_fixture() -> Option<Vec<u8>> {
        let joint = chunk(
            SKELETON_JOINT,
            fields(
                vec![
                    pstring("root")?,
                    u32_field(u32::MAX),
                    u32_field(0),
                    u32_field(7),
                    u32_field(1),
                    u32_field(2),
                    u32_field(3),
                    identity_matrix(),
                ],
            ),
            Vec::new(),
        )?;
        chunk(
            SKELETON,
            fields(
                vec![
                    pstring("skel")?,
                    u32_field(0),
                    u32_field(1),
                ],
            ),
            vec![joint],
        )
    }

    /// Builds an animation fixture with a float channel key list.
    fn animation_fixture() -> Option<Vec<u8>> {
        animation_fixture_with_values(
            1.0, 2.5,
        )
    }

    /// Builds an animation fixture with caller-selected float key values.
    fn animation_fixture_with_values(
        first: f32,
        second: f32,
    ) -> Option<Vec<u8>> {
        let channel = chunk(
            CHANNEL_FLOAT1,
            fields(
                vec![
                    u32_field(0),
                    fourcc("TX  ")?,
                    u32_field(2),
                    u16_field(0),
                    u16_field(10),
                    f32_field(first),
                    f32_field(second),
                ],
            ),
            Vec::new(),
        )?;
        let group = chunk(
            ANIMATION_GROUP,
            fields(
                vec![
                    u32_field(0),
                    pstring("root")?,
                    u32_field(9),
                    u32_field(1),
                ],
            ),
            vec![channel],
        )?;
        let group_list = chunk(
            ANIMATION_GROUP_LIST,
            fields(
                vec![
                    u32_field(0),
                    u32_field(1),
                ],
            ),
            vec![group],
        )?;
        chunk(
            ANIMATION,
            fields(
                vec![
                    u32_field(0),
                    pstring("walk")?,
                    fourcc("PANM")?,
                    f32_field(30.0),
                    f32_field(30.0),
                    u32_field(1),
                ],
            ),
            vec![group_list],
        )
    }

    /// Builds a multi-controller fixture with one timing track.
    fn multi_fixture() -> Option<Vec<u8>> {
        let track = chunk(
            MULTI_CONTROLLER_TRACK,
            fields(
                vec![
                    pstring("walk")?,
                    f32_field(0.0),
                    f32_field(10.0),
                    f32_field(1.0),
                ],
            ),
            Vec::new(),
        )?;
        chunk(
            MULTI_CONTROLLER,
            fields(
                vec![
                    pstring("controller")?,
                    u32_field(0),
                    f32_field(10.0),
                    f32_field(30.0),
                    u32_field(1),
                ],
            ),
            vec![track],
        )
    }

    /// Builds a vertex animation key fixture with vector offsets and indices.
    fn vertex_fixture() -> Option<Vec<u8>> {
        let indices = chunk(
            VERTEX_INDEX_OFFSETS,
            fields(
                vec![
                    u32_field(0),
                    u32_field(1),
                    u32_field(42),
                ],
            ),
            Vec::new(),
        )?;
        let vectors = chunk(
            VERTEX_VECTOR_OFFSETS,
            fields(
                vec![
                    u32_field(0),
                    u32_field(1),
                    fourcc("POSN")?,
                    f32_field(1.0),
                    f32_field(2.0),
                    f32_field(3.0),
                ],
            ),
            vec![indices],
        )?;
        chunk(
            VERTEX_KEY,
            fields(
                vec![
                    u32_field(0),
                    pstring("face")?,
                ],
            ),
            vec![vectors],
        )
    }

    #[test]
    /// Keeps `skeleton_decodes_joint_rest_pose` local because it shares the rig
    /// binary-layout invariant.
    fn skeleton_decodes_joint_rest_pose() -> Result<(), String> {
        let fixture = require(
            skeleton_fixture(),
            "skeleton fixture should build",
        )?;
        let json = require(
            skeleton_json(&fixture),
            "skeleton fixture should decode",
        )?;
        require_json(
            &json,
            "\"schema\":\"skeleton\"",
            "schema should be emitted",
        )?;
        require_json(
            &json,
            "\"name\":\"root\"",
            "joint name should be emitted",
        )?;
        require_json(
            &json,
            "\"parent\":4294967295",
            "parent index should be emitted",
        )?;
        require_json(
            &json,
            "\"rest_pose\":[1.0",
            "rest pose should be emitted",
        )?;
        Ok(())
    }

    #[test]
    /// Keeps `animation_decodes_group_channels_and_keys` local because it
    /// shares the rig binary-layout invariant.
    fn animation_decodes_group_channels_and_keys() -> Result<(), String> {
        let fixture = require(
            animation_fixture(),
            "animation fixture should build",
        )?;
        let json = require(
            animation_json(&fixture),
            "animation fixture should decode",
        )?;
        require_json(
            &json,
            "\"schema\":\"animation\"",
            "schema should be emitted",
        )?;
        require_json(
            &json,
            "\"groups\":[{",
            "group list should be emitted",
        )?;
        require_json(
            &json,
            "\"kind\":\"float1\"",
            "channel kind should be emitted",
        )?;
        require_json(
            &json,
            "\"frames\":[0,10]",
            "key frames should be emitted",
        )?;
        require_json(
            &json,
            "[1.0]",
            "key values should be emitted",
        )?;
        Ok(())
    }

    #[test]
    fn animation_renders_non_finite_keys_as_json_null() -> Result<(), String> {
        let fixture = require(
            animation_fixture_with_values(
                f32::NAN,
                f32::INFINITY,
            ),
            "non-finite animation fixture should build",
        )?;
        let json = require(
            animation_json(&fixture),
            "non-finite animation fixture should decode",
        )?;
        let _value = serde_json::from_str::<serde_json::Value>(&json).map_err(
            |error| {
                format!(
                    "non-finite animation keys must remain valid JSON: {error}"
                )
            },
        )?;
        require_json(
            &json,
            r#""values":[[null],[null]]"#,
            "non-finite keys should be represented as null",
        )
    }

    #[test]
    /// Keeps `multi_controller_decodes_track_timings` local because it shares
    /// the rig binary-layout invariant.
    fn multi_controller_decodes_track_timings() -> Result<(), String> {
        let fixture = require(
            multi_fixture(),
            "multi-controller fixture should build",
        )?;
        let json = require(
            multi_controller_json(&fixture),
            "multi-controller fixture should decode",
        )?;
        require_json(
            &json,
            "\"schema\":\"multi_controller\"",
            "schema should be emitted",
        )?;
        require_json(
            &json,
            "\"name\":\"walk\"",
            "track name should be emitted",
        )?;
        require_json(
            &json,
            "\"start_time\":0.0",
            "track start should be emitted",
        )?;
        require_json(
            &json,
            "\"end_time\":10.0",
            "track end should be emitted",
        )?;
        Ok(())
    }

    #[test]
    /// Keeps `vertex_key_decodes_offsets_and_indices` local because it shares
    /// the rig binary-layout invariant.
    fn vertex_key_decodes_offsets_and_indices() -> Result<(), String> {
        let fixture = require(
            vertex_fixture(),
            "vertex key fixture should build",
        )?;
        let json = require(
            vertex_key_json(&fixture),
            "vertex key fixture should decode",
        )?;
        require_json(
            &json,
            "\"schema\":\"vertex_anim_key\"",
            "schema should be emitted",
        )?;
        require_json(
            &json,
            "\"kind\":\"vector\"",
            "vector offset list should be emitted",
        )?;
        require_json(
            &json,
            "\"param\":\"POSN\"",
            "offset param should be emitted",
        )?;
        require_json(
            &json,
            "\"indices\":[{\"version\":0",
            "index list should be emitted",
        )?;
        require_json(
            &json,
            "\"indices\":[42]",
            "offset indices should be emitted",
        )?;
        Ok(())
    }
}

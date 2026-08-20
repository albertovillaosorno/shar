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
//   - Scene outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Scene outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for p3d.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Scene outbound adapter.

use super::reader::{
    Reader, SubChunk, read_instances_header, read_u32, subchunks,
};
use crate::adapters::driven::json::{escape_json as escape, render_f32};

/// Mesh chunk id used to recognize embedded render geometry.
const MESH: u32 = 0x0001_0000;
/// Skin chunk id used to recognize embedded skinned render geometry.
const SKIN: u32 = 0x0001_0001;
/// Composite skin-list chunk id used to verify binding-list children.
const COMPOSITE_SKIN_LIST: u32 = 0x0000_4513;
/// Composite prop-list chunk id used to verify binding-list children.
const COMPOSITE_PROP_LIST: u32 = 0x0000_4514;
/// Composite skin binding chunk id used inside skin lists.
const COMPOSITE_SKIN: u32 = 0x0000_4515;
/// Composite prop binding chunk id used inside prop lists.
const COMPOSITE_PROP: u32 = 0x0000_4516;
/// Composite effect-list chunk id used to verify binding-list children.
const COMPOSITE_EFFECT_LIST: u32 = 0x0000_4517;
/// Composite effect binding chunk id used inside effect lists.
const COMPOSITE_EFFECT: u32 = 0x0000_4518;
/// Composite sort-order chunk id used by binding elements.
const COMPOSITE_SORT_ORDER: u32 = 0x0000_4519;
/// Scenegraph chunk id used for nested instance placement graphs.
const SCENEGRAPH: u32 = 0x0012_0100;
/// Scene root chunk id used to anchor decoded hierarchies.
const SCENE_ROOT: u32 = 0x0012_0101;
/// Scene branch chunk id used for named child groups.
const SCENE_BRANCH: u32 = 0x0012_0102;
/// Scene transform chunk id used for local matrix placement.
const SCENE_TRANSFORM: u32 = 0x0012_0103;
/// Scene visibility chunk id used to preserve authored visibility gates.
const SCENE_VISIBILITY: u32 = 0x0012_0104;
/// Scene attachment chunk id used for pose attachment groups.
const SCENE_ATTACHMENT: u32 = 0x0012_0105;
/// Scene attachment-point chunk id used for joint-linked children.
const SCENE_ATTACHMENT_POINT: u32 = 0x0012_0106;
/// Scene drawable chunk id used for render target references.
const SCENE_DRAWABLE: u32 = 0x0012_0107;
/// Scene camera chunk id used for camera references.
const SCENE_CAMERA: u32 = 0x0012_0108;
/// Scene light-group chunk id used for lighting references.
const SCENE_LIGHT_GROUP: u32 = 0x0012_0109;
/// Scene sort-order chunk id used by drawable nodes.
const SCENE_SORT_ORDER: u32 = 0x0012_010a;
/// Instances chunk id used by insta-entity placement payloads.
const INSTANCES: u32 = 0x0300_0008;

/// Decode a scenegraph chunk into a lossless scene hierarchy JSON body.
#[must_use]
pub fn scenegraph_json(chunk: &[u8]) -> Option<String> {
    let (_, header_size, total_size) = chunk_bounds(chunk)?;
    let mut reader = Reader::new(chunk, 12);
    let name = reader.pstring()?;
    let version = reader.u32()?;
    if reader.pos() > header_size {
        return None;
    }
    let roots = decode_scene_children(chunk, header_size, total_size)?;
    Some(format!(
        "{{\"schema\":\"scenegraph\",\"name\":\"{}\",\"version\":{},\"\
             roots\":[{}]}}\n",
        escape(&name),
        version,
        roots.join(",")
    ))
}

/// Decode a composite drawable chunk into binding-list JSON.
#[must_use]
pub fn composite_drawable_json(chunk: &[u8]) -> Option<String> {
    let (_, header_size, total_size) = chunk_bounds(chunk)?;
    let mut reader = Reader::new(chunk, 12);
    let name = reader.pstring()?;
    let skeleton_name = reader.pstring()?;
    if reader.pos() > header_size {
        return None;
    }
    let mut skins = Vec::new();
    let mut props = Vec::new();
    let mut effects = Vec::new();
    for child in subchunks(chunk, header_size, total_size)? {
        match child.id {
            COMPOSITE_SKIN_LIST => {
                skins = decode_composite_list(
                    chunk,
                    &child,
                    COMPOSITE_SKIN,
                    CompositeElementKind::Skin,
                )?;
            },
            COMPOSITE_PROP_LIST => {
                props = decode_composite_list(
                    chunk,
                    &child,
                    COMPOSITE_PROP,
                    CompositeElementKind::Prop,
                )?;
            },
            COMPOSITE_EFFECT_LIST => {
                effects = decode_composite_list(
                    chunk,
                    &child,
                    COMPOSITE_EFFECT,
                    CompositeElementKind::Effect,
                )?;
            },
            _ => return None,
        }
    }
    Some(format!(
        "{{\"schema\":\"composite_drawable\",\"name\":\"{}\",\"\
             skeleton_name\":\"{}\",\"num_skins\":{},\"skins\":[{}],\"\
             num_props\":{},\"props\":[{}],\"num_effects\":{},\"effects\":\
             [{}]}}\n",
        escape(&name),
        escape(&skeleton_name),
        skins.len(),
        skins.join(","),
        props.len(),
        props.join(","),
        effects.len(),
        effects.join(",")
    ))
}

/// Decode an `srr_entity_dsg` chunk and its contained drawable reference.
#[must_use]
pub fn entity_dsg_json(chunk: &[u8]) -> Option<String> {
    entity_json(chunk, "srr_entity_dsg", false)
}

/// Decode an `srr_insta_entity_dsg` chunk and instance transform hierarchy.
#[must_use]
pub fn insta_entity_dsg_json(chunk: &[u8]) -> Option<String> {
    entity_json(chunk, "srr_insta_entity_dsg", true)
}

#[derive(Debug, Clone, Copy)]
/// Distinguishes composite binding shapes with different scalar fields.
enum CompositeElementKind {
    /// Skin binding has no joint id in the composite payload.
    Skin,
    /// Prop binding carries a skeleton joint id.
    Prop,
    /// Effect binding carries a skeleton joint id.
    Effect,
}

/// Shared DSG decoder keeps entity and insta-entity field handling identical.
fn entity_json(
    chunk: &[u8],
    schema: &str,
    allow_instances: bool,
) -> Option<String> {
    let (_, header_size, total_size) = chunk_bounds(chunk)?;
    let mut reader = Reader::new(chunk, 12);
    let name = reader.pstring()?;
    let version = reader.u32()?;
    let has_alpha = reader.u32()?;
    if reader.pos() > header_size {
        return None;
    }
    let mut render_refs = Vec::new();
    let mut instances = Vec::new();
    for child in subchunks(chunk, header_size, total_size)? {
        match child.id {
            MESH | SKIN => {
                render_refs.push(decode_drawable_ref(chunk, &child)?)
            },
            INSTANCES if allow_instances => {
                instances.push(decode_instances(chunk, &child)?)
            },
            _ => return None,
        }
    }
    if allow_instances && instances.is_empty() {
        return None;
    }
    let instance_field = if allow_instances {
        format!(",\"instances\":[{}]", instances.join(","))
    } else {
        String::new()
    };
    Some(
        format!(
            "{{\"schema\":\"{}\",\"name\":\"{}\",\"version\":{},\"has_alpha\":\
             {},\"render_refs\":[{}],\"collision_refs\":[]{} }}\n",
            schema,
            escape(&name),
            version,
            has_alpha,
            render_refs.join(","),
            instance_field
        )
        .replace(" }", "}"),
    )
}

/// Decodes a child region so count checks can compare authored and parsed
/// nodes.
fn decode_scene_children(
    chunk: &[u8],
    start: usize,
    end: usize,
) -> Option<Vec<String>> {
    let mut nodes = Vec::new();
    for child in subchunks(chunk, start, end)? {
        nodes.push(decode_scene_node(chunk, &child)?);
    }
    Some(nodes)
}

/// Dispatches scene nodes by id so unexpected child kinds fail closed.
fn decode_scene_node(chunk: &[u8], node: &SubChunk) -> Option<String> {
    match node.id {
        SCENE_ROOT => {
            let children =
                decode_scene_children(chunk, node.header_end(), node.end())?;
            Some(format!(
                "{{\"kind\":\"root\",\"children\":[{}]}}",
                children.join(",")
            ))
        },
        SCENE_BRANCH => decode_named_children_node(chunk, node, "branch"),
        SCENE_TRANSFORM => decode_transform_node(chunk, node),
        SCENE_VISIBILITY => decode_visibility_node(chunk, node),
        SCENE_ATTACHMENT => decode_attachment_node(chunk, node),
        SCENE_ATTACHMENT_POINT => decode_attachment_point(chunk, node),
        SCENE_DRAWABLE => decode_drawable_node(chunk, node),
        SCENE_CAMERA => {
            decode_named_ref_node(chunk, node, "camera", "camera_name")
        },
        SCENE_LIGHT_GROUP => decode_named_ref_node(
            chunk,
            node,
            "light_group",
            "light_group_name",
        ),
        _ => None,
    }
}

/// Decodes branch-like nodes whose contract is name plus child count.
fn decode_named_children_node(
    chunk: &[u8],
    node: &SubChunk,
    kind: &str,
) -> Option<String> {
    let mut reader = Reader::new(chunk, node.data_offset());
    let name = reader.pstring()?;
    let child_count = usize::try_from(reader.u32()?).ok()?;
    if reader.pos() > node.header_end() {
        return None;
    }
    let children = decode_scene_children(chunk, node.header_end(), node.end())?;
    if children.len() != child_count {
        return None;
    }
    Some(format!(
        "{{\"kind\":\"{}\",\"name\":\"{}\",\"child_count\":{},\"children\"\
             :[{}]}}",
        kind,
        escape(&name),
        child_count,
        children.join(",")
    ))
}

/// Decodes transform nodes because placement requires the full local matrix.
fn decode_transform_node(chunk: &[u8], node: &SubChunk) -> Option<String> {
    let mut reader = Reader::new(chunk, node.data_offset());
    let name = reader.pstring()?;
    let child_count = usize::try_from(reader.u32()?).ok()?;
    let matrix = read_matrix(&mut reader)?;
    if reader.pos() > node.header_end() {
        return None;
    }
    let children = decode_scene_children(chunk, node.header_end(), node.end())?;
    if children.len() != child_count {
        return None;
    }
    Some(format!(
        "{{\"kind\":\"transform\",\"name\":\"{}\",\"child_count\":{},\"\
             matrix\":[{}],\"children\":[{}]}}",
        escape(&name),
        child_count,
        matrix.join(","),
        children.join(",")
    ))
}

/// Decodes visibility nodes so authored enable flags survive extraction.
fn decode_visibility_node(chunk: &[u8], node: &SubChunk) -> Option<String> {
    let mut reader = Reader::new(chunk, node.data_offset());
    let name = reader.pstring()?;
    let child_count = usize::try_from(reader.u32()?).ok()?;
    let is_visible = reader.u32()?;
    if reader.pos() > node.header_end() {
        return None;
    }
    let children = decode_scene_children(chunk, node.header_end(), node.end())?;
    if children.len() != child_count {
        return None;
    }
    Some(format!(
        "{{\"kind\":\"visibility\",\"name\":\"{}\",\"child_count\":{},\"\
             is_visible\":{},\"children\":[{}]}}",
        escape(&name),
        child_count,
        is_visible,
        children.join(",")
    ))
}

/// Decodes attachment nodes so pose targets retain their attachment points.
fn decode_attachment_node(chunk: &[u8], node: &SubChunk) -> Option<String> {
    let mut reader = Reader::new(chunk, node.data_offset());
    let name = reader.pstring()?;
    let drawable_pose_name = reader.pstring()?;
    let point_count = usize::try_from(reader.u32()?).ok()?;
    if reader.pos() > node.header_end() {
        return None;
    }
    let mut points = Vec::new();
    for child in subchunks(chunk, node.header_end(), node.end())? {
        if child.id != SCENE_ATTACHMENT_POINT {
            return None;
        }
        points.push(decode_attachment_point(chunk, &child)?);
    }
    if points.len() != point_count {
        return None;
    }
    Some(format!(
        "{{\"kind\":\"attachment\",\"name\":\"{}\",\"drawable_pose_name\":\
             \"{}\",\"point_count\":{},\"points\":[{}]}}",
        escape(&name),
        escape(&drawable_pose_name),
        point_count,
        points.join(",")
    ))
}

/// Decodes attachment points so joint-linked child payloads retain their joint
/// id.
fn decode_attachment_point(chunk: &[u8], node: &SubChunk) -> Option<String> {
    let mut reader = Reader::new(chunk, node.data_offset());
    let joint = reader.u32()?;
    if reader.pos() > node.header_end() {
        return None;
    }
    let children = decode_scene_children(chunk, node.header_end(), node.end())?;
    Some(format!(
        "{{\"kind\":\"attachment_point\",\"joint\":{},\"children\":[{}]}}",
        joint,
        children.join(",")
    ))
}

/// Decodes drawable nodes so scene placement can resolve render targets.
fn decode_drawable_node(chunk: &[u8], node: &SubChunk) -> Option<String> {
    let mut reader = Reader::new(chunk, node.data_offset());
    let name = reader.pstring()?;
    let drawable_name = reader.pstring()?;
    let is_translucent = reader.u32()?;
    if reader.pos() > node.header_end() {
        return None;
    }
    let sort_order = decode_optional_sort_order(
        chunk,
        node.header_end(),
        node.end(),
        SCENE_SORT_ORDER,
    )?;
    Some(
        format!(
            "{{\"kind\":\"drawable\",\"name\":\"{}\",\"drawable_name\":\"{}\",\
             \"is_translucent\":{}{} }}",
            escape(&name),
            escape(&drawable_name),
            is_translucent,
            sort_order
        )
        .replace(" }", "}"),
    )
}

/// Decodes reference-only scene nodes for camera and light-group bindings.
fn decode_named_ref_node(
    chunk: &[u8],
    node: &SubChunk,
    kind: &str,
    ref_field: &str,
) -> Option<String> {
    let mut reader = Reader::new(chunk, node.data_offset());
    let name = reader.pstring()?;
    let target = reader.pstring()?;
    if reader.pos() > node.header_end() {
        return None;
    }
    if !subchunks(chunk, node.header_end(), node.end())?.is_empty() {
        return None;
    }
    Some(format!(
        "{{\"kind\":\"{}\",\"name\":\"{}\",\"{}\":\"{}\"}}",
        kind,
        escape(&name),
        ref_field,
        escape(&target)
    ))
}

/// Decodes composite lists while verifying declared element counts.
fn decode_composite_list(
    chunk: &[u8],
    list: &SubChunk,
    expected_child_id: u32,
    kind: CompositeElementKind,
) -> Option<Vec<String>> {
    let count = Reader::new(chunk, list.data_offset())
        .u32()
        .and_then(|value| usize::try_from(value).ok())?;
    let children = subchunks(chunk, list.header_end(), list.end())?;
    if children.len() != count {
        return None;
    }
    let mut elements = Vec::new();
    for child in children {
        if child.id != expected_child_id {
            return None;
        }
        elements.push(decode_composite_element(chunk, &child, kind)?);
    }
    Some(elements)
}

/// Decodes composite bindings whose fields vary by skin, prop, or effect kind.
fn decode_composite_element(
    chunk: &[u8],
    element: &SubChunk,
    kind: CompositeElementKind,
) -> Option<String> {
    let mut reader = Reader::new(chunk, element.data_offset());
    let name = reader.pstring()?;
    let is_translucent = reader.u32()?;
    let joint = match kind {
        CompositeElementKind::Skin => None,
        CompositeElementKind::Prop | CompositeElementKind::Effect => {
            Some(reader.u32()?)
        },
    };
    if reader.pos() > element.header_end() {
        return None;
    }
    let sort_order = decode_optional_sort_order(
        chunk,
        element.header_end(),
        element.end(),
        COMPOSITE_SORT_ORDER,
    )?;
    let kind_name = match kind {
        CompositeElementKind::Skin => "skin",
        CompositeElementKind::Prop => "prop",
        CompositeElementKind::Effect => "effect",
    };
    let joint_field = joint
        .map(|value| format!(",\"skeleton_joint_id\":{value}"))
        .unwrap_or_default();
    Some(
        format!(
            "{{\"kind\":\"{}\",\"name\":\"{}\",\"is_translucent\":{}{}{} }}",
            kind_name,
            escape(&name),
            is_translucent,
            joint_field,
            sort_order
        )
        .replace(" }", "}"),
    )
}

/// Decodes optional sort order children without accepting unrelated payloads.
fn decode_optional_sort_order(
    chunk: &[u8],
    start: usize,
    end: usize,
    expected_id: u32,
) -> Option<String> {
    let children = subchunks(chunk, start, end)?;
    if children.is_empty() {
        return Some(String::new());
    }
    if children.len() != 1 {
        return None;
    }
    let child = children.first()?;
    if child.id != expected_id {
        return None;
    }
    let mut reader = Reader::new(chunk, child.data_offset());
    let sort_order = reader.f32()?;
    if reader.pos() > child.header_end() {
        return None;
    }
    Some(format!(",\"sort_order\":{}", fmt_f32(sort_order)))
}

/// Decodes embedded render references without re-parsing full geometry.
fn decode_drawable_ref(chunk: &[u8], child: &SubChunk) -> Option<String> {
    let name = container_name(chunk, child)?;
    let kind = match child.id {
        MESH => "mesh",
        SKIN => "skin",
        _ => return None,
    };
    Some(format!(
        "{{\"kind\":\"{}\",\"name\":\"{}\"}}",
        kind,
        escape(&name)
    ))
}

/// Decodes instance payloads so nested scenegraph transforms are emitted once.
fn decode_instances(chunk: &[u8], child: &SubChunk) -> Option<String> {
    let (version, flags, name) = read_instances_header(chunk, child)?;
    let mut scenegraphs = Vec::new();
    for graph in subchunks(chunk, child.header_end(), child.end())? {
        if graph.id != SCENEGRAPH {
            return None;
        }
        let bytes = chunk.get(graph.offset..graph.end())?;
        let json = scenegraph_json(bytes)?;
        scenegraphs.push(json.trim().to_owned());
    }
    Some(format!(
        "{{\"version\":{},\"flags\":{},\"name\":\"{}\",\"scenegraphs\":\
             [{}]}}",
        version,
        flags,
        escape(&name),
        scenegraphs.join(",")
    ))
}

/// Reads the leading name field shared by render containers.
fn container_name(chunk: &[u8], child: &SubChunk) -> Option<String> {
    let mut reader = Reader::new(chunk, child.data_offset());
    let name = reader.pstring()?;
    if reader.pos() > child.header_end() {
        return None;
    }
    Some(name)
}

/// Reads chunk bounds so each parser stays inside the declared payload.
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

/// Reads transform matrices as JSON-ready floats for deterministic output.
fn read_matrix(reader: &mut Reader<'_>) -> Option<Vec<String>> {
    let mut values = Vec::new();
    for _ in 0_usize..16_usize {
        values.push(fmt_f32(reader.f32()?));
    }
    Some(values)
}

/// Formats floats consistently so generated JSON remains stable.
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
#[path = "../../../../../../tests/formats/p3d/unit/adapter-outbound/decoders/scene/tests.rs"]
mod tests;

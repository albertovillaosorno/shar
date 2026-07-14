// File:
//   - scene.rs
// Path:
//   - src/p3d/src/adapters/driven/decoders/scene.rs
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
//   - The p3d adapter boundary for adapters driven decoders scene.
// - Must-Not:
//   - Change domain semantics or bypass application and port contracts.
// - Allows:
//   - Filesystem, JSON, CLI, Blender, or serialization work behind explicit
//   - ports.
// - Split-When:
//   - Split when scene contains two independently testable contracts.
// - Merge-When:
//   - Another p3d module owns the same adapters boundary with no distinct
//   - invariant.
// - Summary:
//   - Scene-assembly decoders for scenegraph, composite drawable, and DSG
//   - chunks.
// - Description:
//   - Defines scene data and behavior for p3d adapters driven decoders.
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
//   - Reason: src/p3d/src/adapters/driven/decoders/scene.rs has 1211 effective
//   - lines after the required header and remains cohesive until a focused
//   - split
//   - lands.
//

//! Scene-assembly decoders for scenegraph, composite drawable, and DSG chunks.
//!
//! These chunks bind decoded geometry into world and character assemblies. They
//! are container-style chunks: scalar fields live in the chunk header region
//! and child records live after `header_size`. Decoding fails closed whenever
//! child counts or child kinds do not match the declared schema.
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
    let mut reader = Reader::new(
        chunk, 12,
    );
    let name = reader.pstring()?;
    let version = reader.u32()?;
    if reader.pos() > header_size {
        return None;
    }
    let roots = decode_scene_children(
        chunk,
        header_size,
        total_size,
    )?;
    Some(
        format!(
            "{{\"schema\":\"scenegraph\",\"name\":\"{}\",\"version\":{},\"\
             roots\":[{}]}}\n",
            escape(&name),
            version,
            roots.join(",")
        ),
    )
}

/// Decode a composite drawable chunk into binding-list JSON.
#[must_use]
pub fn composite_drawable_json(chunk: &[u8]) -> Option<String> {
    let (_, header_size, total_size) = chunk_bounds(chunk)?;
    let mut reader = Reader::new(
        chunk, 12,
    );
    let name = reader.pstring()?;
    let skeleton_name = reader.pstring()?;
    if reader.pos() > header_size {
        return None;
    }
    let mut skins = Vec::new();
    let mut props = Vec::new();
    let mut effects = Vec::new();
    for child in subchunks(
        chunk,
        header_size,
        total_size,
    )? {
        match child.id {
            COMPOSITE_SKIN_LIST => {
                skins = decode_composite_list(
                    chunk,
                    &child,
                    COMPOSITE_SKIN,
                    CompositeElementKind::Skin,
                )?;
            }
            COMPOSITE_PROP_LIST => {
                props = decode_composite_list(
                    chunk,
                    &child,
                    COMPOSITE_PROP,
                    CompositeElementKind::Prop,
                )?;
            }
            COMPOSITE_EFFECT_LIST => {
                effects = decode_composite_list(
                    chunk,
                    &child,
                    COMPOSITE_EFFECT,
                    CompositeElementKind::Effect,
                )?;
            }
            _ => return None,
        }
    }
    Some(
        format!(
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
        ),
    )
}

/// Decode an `srr_entity_dsg` chunk and its contained drawable reference.
#[must_use]
pub fn entity_dsg_json(chunk: &[u8]) -> Option<String> {
    entity_json(
        chunk,
        "srr_entity_dsg",
        false,
    )
}

/// Decode an `srr_insta_entity_dsg` chunk and instance transform hierarchy.
#[must_use]
pub fn insta_entity_dsg_json(chunk: &[u8]) -> Option<String> {
    entity_json(
        chunk,
        "srr_insta_entity_dsg",
        true,
    )
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
    let mut reader = Reader::new(
        chunk, 12,
    );
    let name = reader.pstring()?;
    let version = reader.u32()?;
    let has_alpha = reader.u32()?;
    if reader.pos() > header_size {
        return None;
    }
    let mut render_refs = Vec::new();
    let mut instances = Vec::new();
    for child in subchunks(
        chunk,
        header_size,
        total_size,
    )? {
        match child.id {
            MESH | SKIN => render_refs.push(
                decode_drawable_ref(
                    chunk, &child,
                )?,
            ),
            INSTANCES if allow_instances => instances.push(
                decode_instances(
                    chunk, &child,
                )?,
            ),
            _ => return None,
        }
    }
    if allow_instances && instances.is_empty() {
        return None;
    }
    let instance_field = if allow_instances {
        format!(
            ",\"instances\":[{}]",
            instances.join(",")
        )
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
        .replace(
            " }", "}",
        ),
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
    for child in subchunks(
        chunk, start, end,
    )? {
        nodes.push(
            decode_scene_node(
                chunk, &child,
            )?,
        );
    }
    Some(nodes)
}

/// Dispatches scene nodes by id so unexpected child kinds fail closed.
fn decode_scene_node(
    chunk: &[u8],
    node: &SubChunk,
) -> Option<String> {
    match node.id {
        SCENE_ROOT => {
            let children = decode_scene_children(
                chunk,
                node.header_end(),
                node.end(),
            )?;
            Some(
                format!(
                    "{{\"kind\":\"root\",\"children\":[{}]}}",
                    children.join(",")
                ),
            )
        }
        SCENE_BRANCH => decode_named_children_node(
            chunk, node, "branch",
        ),
        SCENE_TRANSFORM => decode_transform_node(
            chunk, node,
        ),
        SCENE_VISIBILITY => decode_visibility_node(
            chunk, node,
        ),
        SCENE_ATTACHMENT => decode_attachment_node(
            chunk, node,
        ),
        SCENE_ATTACHMENT_POINT => decode_attachment_point(
            chunk, node,
        ),
        SCENE_DRAWABLE => decode_drawable_node(
            chunk, node,
        ),
        SCENE_CAMERA => decode_named_ref_node(
            chunk,
            node,
            "camera",
            "camera_name",
        ),
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
    let mut reader = Reader::new(
        chunk,
        node.data_offset(),
    );
    let name = reader.pstring()?;
    let child_count = usize::try_from(reader.u32()?).ok()?;
    if reader.pos() > node.header_end() {
        return None;
    }
    let children = decode_scene_children(
        chunk,
        node.header_end(),
        node.end(),
    )?;
    if children.len() != child_count {
        return None;
    }
    Some(
        format!(
            "{{\"kind\":\"{}\",\"name\":\"{}\",\"child_count\":{},\"children\"\
             :[{}]}}",
            kind,
            escape(&name),
            child_count,
            children.join(",")
        ),
    )
}

/// Decodes transform nodes because placement requires the full local matrix.
fn decode_transform_node(
    chunk: &[u8],
    node: &SubChunk,
) -> Option<String> {
    let mut reader = Reader::new(
        chunk,
        node.data_offset(),
    );
    let name = reader.pstring()?;
    let child_count = usize::try_from(reader.u32()?).ok()?;
    let matrix = read_matrix(&mut reader)?;
    if reader.pos() > node.header_end() {
        return None;
    }
    let children = decode_scene_children(
        chunk,
        node.header_end(),
        node.end(),
    )?;
    if children.len() != child_count {
        return None;
    }
    Some(
        format!(
            "{{\"kind\":\"transform\",\"name\":\"{}\",\"child_count\":{},\"\
             matrix\":[{}],\"children\":[{}]}}",
            escape(&name),
            child_count,
            matrix.join(","),
            children.join(",")
        ),
    )
}

/// Decodes visibility nodes so authored enable flags survive extraction.
fn decode_visibility_node(
    chunk: &[u8],
    node: &SubChunk,
) -> Option<String> {
    let mut reader = Reader::new(
        chunk,
        node.data_offset(),
    );
    let name = reader.pstring()?;
    let child_count = usize::try_from(reader.u32()?).ok()?;
    let is_visible = reader.u32()?;
    if reader.pos() > node.header_end() {
        return None;
    }
    let children = decode_scene_children(
        chunk,
        node.header_end(),
        node.end(),
    )?;
    if children.len() != child_count {
        return None;
    }
    Some(
        format!(
            "{{\"kind\":\"visibility\",\"name\":\"{}\",\"child_count\":{},\"\
             is_visible\":{},\"children\":[{}]}}",
            escape(&name),
            child_count,
            is_visible,
            children.join(",")
        ),
    )
}

/// Decodes attachment nodes so pose targets retain their attachment points.
fn decode_attachment_node(
    chunk: &[u8],
    node: &SubChunk,
) -> Option<String> {
    let mut reader = Reader::new(
        chunk,
        node.data_offset(),
    );
    let name = reader.pstring()?;
    let drawable_pose_name = reader.pstring()?;
    let point_count = usize::try_from(reader.u32()?).ok()?;
    if reader.pos() > node.header_end() {
        return None;
    }
    let mut points = Vec::new();
    for child in subchunks(
        chunk,
        node.header_end(),
        node.end(),
    )? {
        if child.id != SCENE_ATTACHMENT_POINT {
            return None;
        }
        points.push(
            decode_attachment_point(
                chunk, &child,
            )?,
        );
    }
    if points.len() != point_count {
        return None;
    }
    Some(
        format!(
            "{{\"kind\":\"attachment\",\"name\":\"{}\",\"drawable_pose_name\":\
             \"{}\",\"point_count\":{},\"points\":[{}]}}",
            escape(&name),
            escape(&drawable_pose_name),
            point_count,
            points.join(",")
        ),
    )
}

/// Decodes attachment points so joint-linked child payloads retain their joint
/// id.
fn decode_attachment_point(
    chunk: &[u8],
    node: &SubChunk,
) -> Option<String> {
    let mut reader = Reader::new(
        chunk,
        node.data_offset(),
    );
    let joint = reader.u32()?;
    if reader.pos() > node.header_end() {
        return None;
    }
    let children = decode_scene_children(
        chunk,
        node.header_end(),
        node.end(),
    )?;
    Some(
        format!(
            "{{\"kind\":\"attachment_point\",\"joint\":{},\"children\":[{}]}}",
            joint,
            children.join(",")
        ),
    )
}

/// Decodes drawable nodes so scene placement can resolve render targets.
fn decode_drawable_node(
    chunk: &[u8],
    node: &SubChunk,
) -> Option<String> {
    let mut reader = Reader::new(
        chunk,
        node.data_offset(),
    );
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
        .replace(
            " }", "}",
        ),
    )
}

/// Decodes reference-only scene nodes for camera and light-group bindings.
fn decode_named_ref_node(
    chunk: &[u8],
    node: &SubChunk,
    kind: &str,
    ref_field: &str,
) -> Option<String> {
    let mut reader = Reader::new(
        chunk,
        node.data_offset(),
    );
    let name = reader.pstring()?;
    let target = reader.pstring()?;
    if reader.pos() > node.header_end() {
        return None;
    }
    if !subchunks(
        chunk,
        node.header_end(),
        node.end(),
    )?
    .is_empty()
    {
        return None;
    }
    Some(
        format!(
            "{{\"kind\":\"{}\",\"name\":\"{}\",\"{}\":\"{}\"}}",
            kind,
            escape(&name),
            ref_field,
            escape(&target)
        ),
    )
}

/// Decodes composite lists while verifying declared element counts.
fn decode_composite_list(
    chunk: &[u8],
    list: &SubChunk,
    expected_child_id: u32,
    kind: CompositeElementKind,
) -> Option<Vec<String>> {
    let count = Reader::new(
        chunk,
        list.data_offset(),
    )
    .u32()
    .and_then(|value| usize::try_from(value).ok())?;
    let children = subchunks(
        chunk,
        list.header_end(),
        list.end(),
    )?;
    if children.len() != count {
        return None;
    }
    let mut elements = Vec::new();
    for child in children {
        if child.id != expected_child_id {
            return None;
        }
        elements.push(
            decode_composite_element(
                chunk, &child, kind,
            )?,
        );
    }
    Some(elements)
}

/// Decodes composite bindings whose fields vary by skin, prop, or effect kind.
fn decode_composite_element(
    chunk: &[u8],
    element: &SubChunk,
    kind: CompositeElementKind,
) -> Option<String> {
    let mut reader = Reader::new(
        chunk,
        element.data_offset(),
    );
    let name = reader.pstring()?;
    let is_translucent = reader.u32()?;
    let joint = match kind {
        CompositeElementKind::Skin => None,
        CompositeElementKind::Prop | CompositeElementKind::Effect => {
            Some(reader.u32()?)
        }
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
        .replace(
            " }", "}",
        ),
    )
}

/// Decodes optional sort order children without accepting unrelated payloads.
fn decode_optional_sort_order(
    chunk: &[u8],
    start: usize,
    end: usize,
    expected_id: u32,
) -> Option<String> {
    let children = subchunks(
        chunk, start, end,
    )?;
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
    let mut reader = Reader::new(
        chunk,
        child.data_offset(),
    );
    let sort_order = reader.f32()?;
    if reader.pos() > child.header_end() {
        return None;
    }
    Some(
        format!(
            ",\"sort_order\":{}",
            fmt_f32(sort_order)
        ),
    )
}

/// Decodes embedded render references without re-parsing full geometry.
fn decode_drawable_ref(
    chunk: &[u8],
    child: &SubChunk,
) -> Option<String> {
    let name = container_name(
        chunk, child,
    )?;
    let kind = match child.id {
        MESH => "mesh",
        SKIN => "skin",
        _ => return None,
    };
    Some(
        format!(
            "{{\"kind\":\"{}\",\"name\":\"{}\"}}",
            kind,
            escape(&name)
        ),
    )
}

/// Decodes instance payloads so nested scenegraph transforms are emitted once.
fn decode_instances(
    chunk: &[u8],
    child: &SubChunk,
) -> Option<String> {
    let (version, flags, name) = read_instances_header(
        chunk, child,
    )?;
    let mut scenegraphs = Vec::new();
    for graph in subchunks(
        chunk,
        child.header_end(),
        child.end(),
    )? {
        if graph.id != SCENEGRAPH {
            return None;
        }
        let bytes = chunk.get(graph.offset..graph.end())?;
        let json = scenegraph_json(bytes)?;
        scenegraphs.push(
            json.trim()
                .to_owned(),
        );
    }
    Some(
        format!(
            "{{\"version\":{},\"flags\":{},\"name\":\"{}\",\"scenegraphs\":\
             [{}]}}",
            version,
            flags,
            escape(&name),
            scenegraphs.join(",")
        ),
    )
}

/// Reads the leading name field shared by render containers.
fn container_name(
    chunk: &[u8],
    child: &SubChunk,
) -> Option<String> {
    let mut reader = Reader::new(
        chunk,
        child.data_offset(),
    );
    let name = reader.pstring()?;
    if reader.pos() > child.header_end() {
        return None;
    }
    Some(name)
}

/// Reads chunk bounds so each parser stays inside the declared payload.
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

    /// Joins fixture field fragments so tests mirror contiguous chunk headers.
    fn fields(parts: Vec<Vec<u8>>) -> Vec<u8> {
        let mut out = Vec::new();
        for part in parts {
            out.extend(part);
        }
        out
    }

    /// Converts optional fixture construction into a descriptive test error.
    fn require<T>(
        value: Option<T>,
        context: &str,
    ) -> Result<T, String> {
        value.ok_or_else(|| String::from(context))
    }

    /// Checks useful JSON fields without panicking inside `Result` tests.
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

    /// Builds a synthetic chunk for count-checked decoder tests.
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
        let header_size_u32 = u32::try_from(header_size).ok()?;
        let total_size_u32 = u32::try_from(total_size).ok()?;
        let mut out = Vec::new();
        out.extend_from_slice(&id.to_le_bytes());
        out.extend_from_slice(&header_size_u32.to_le_bytes());
        out.extend_from_slice(&total_size_u32.to_le_bytes());
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

    /// Builds a little-endian integer test field.
    fn u32_field(value: u32) -> Vec<u8> {
        value
            .to_le_bytes()
            .to_vec()
    }

    /// Builds a little-endian float test field.
    fn f32_field(value: f32) -> Vec<u8> {
        value
            .to_le_bytes()
            .to_vec()
    }

    /// Builds a stable identity matrix fixture for transform tests.
    fn identity_matrix() -> Vec<u8> {
        let mut out = Vec::new();
        for index in 0_usize..16_usize {
            let value = if matches!(
                index,
                0 | 5 | 10 | 15
            ) {
                1.0_f32
            } else {
                0.0_f32
            };
            out.extend_from_slice(&f32_field(value));
        }
        out
    }

    /// Builds a reusable scenegraph fixture with configurable child count.
    fn scenegraph_fixture(child_count: u32) -> Option<Vec<u8>> {
        let sort = chunk(
            SCENE_SORT_ORDER,
            f32_field(3.5_f32),
            Vec::new(),
        )?;
        let drawable_fields = fields(
            vec![
                pstring("body_node")?,
                pstring("body_mesh")?,
                u32_field(0),
            ],
        );
        let drawable = chunk(
            SCENE_DRAWABLE,
            drawable_fields,
            vec![sort],
        )?;
        let transform_fields = fields(
            vec![
                pstring("body_xform")?,
                u32_field(child_count),
                identity_matrix(),
            ],
        );
        let transform = chunk(
            SCENE_TRANSFORM,
            transform_fields,
            vec![drawable],
        )?;
        let root = chunk(
            SCENE_ROOT,
            Vec::new(),
            vec![transform],
        )?;
        let graph_fields = fields(
            vec![
                pstring("entity_graph")?,
                u32_field(0),
            ],
        );
        chunk(
            SCENEGRAPH,
            graph_fields,
            vec![root],
        )
    }

    /// Builds a composite skin-list fixture with one sorted skin binding.
    fn composite_skin_list_fixture() -> Result<Vec<u8>, String> {
        let sort = require(
            chunk(
                COMPOSITE_SORT_ORDER,
                f32_field(1.25_f32),
                Vec::new(),
            ),
            "sort-order fixture should build",
        )?;
        let skin_fields = fields(
            vec![
                require(
                    pstring("hero_skin"),
                    "skin name should encode",
                )?,
                u32_field(1),
            ],
        );
        let skin = require(
            chunk(
                COMPOSITE_SKIN,
                skin_fields,
                vec![sort],
            ),
            "skin fixture should build",
        )?;
        require(
            chunk(
                COMPOSITE_SKIN_LIST,
                u32_field(1),
                vec![skin],
            ),
            "skin-list fixture should build",
        )
    }

    /// Builds a composite prop-list fixture with one joint-bound prop.
    fn composite_prop_list_fixture() -> Result<Vec<u8>, String> {
        let prop_fields = fields(
            vec![
                require(
                    pstring("hat_mesh"),
                    "prop name should encode",
                )?,
                u32_field(0),
                u32_field(7),
            ],
        );
        let prop = require(
            chunk(
                COMPOSITE_PROP,
                prop_fields,
                Vec::new(),
            ),
            "prop fixture should build",
        )?;
        require(
            chunk(
                COMPOSITE_PROP_LIST,
                u32_field(1),
                vec![prop],
            ),
            "prop-list fixture should build",
        )
    }

    /// Builds an empty effect-list fixture because zero-count lists must
    /// survive.
    fn composite_effect_list_fixture() -> Result<Vec<u8>, String> {
        require(
            chunk(
                COMPOSITE_EFFECT_LIST,
                u32_field(0),
                Vec::new(),
            ),
            "effect-list fixture should build",
        )
    }

    /// Builds a composite drawable fixture with skin, prop, and effect lists.
    fn composite_drawable_fixture() -> Result<Vec<u8>, String> {
        let composite_fields = fields(
            vec![
                require(
                    pstring("hero_comp"),
                    "composite name should encode",
                )?,
                require(
                    pstring("hero_skel"),
                    "skeleton name should encode",
                )?,
            ],
        );
        require(
            chunk(
                0x0000_4512,
                composite_fields,
                vec![
                    composite_skin_list_fixture()?,
                    composite_prop_list_fixture()?,
                    composite_effect_list_fixture()?,
                ],
            ),
            "composite fixture should build",
        )
    }

    #[test]
    fn scenegraph_decodes_transform_and_drawable_refs() -> Result<(), String> {
        let fixture = require(
            scenegraph_fixture(1),
            "scenegraph fixture should build",
        )?;
        let json = require(
            scenegraph_json(&fixture),
            "scenegraph fixture should decode",
        )?;

        require_json(
            &json,
            "\"schema\":\"scenegraph\"",
            "schema should be emitted",
        )?;
        require_json(
            &json,
            "\"kind\":\"transform\"",
            "transform should be emitted",
        )?;
        require_json(
            &json,
            "\"drawable_name\":\"body_mesh\"",
            "drawable ref should be emitted",
        )?;
        require_json(
            &json,
            "\"sort_order\":3.5",
            "sort order should be emitted",
        )?;
        Ok(())
    }

    #[test]
    fn scenegraph_fails_closed_on_child_count_mismatch() -> Result<(), String> {
        let fixture = require(
            scenegraph_fixture(2),
            "mismatch fixture should build",
        )?;
        if scenegraph_json(&fixture).is_none() {
            Ok(())
        } else {
            Err(String::from("child-count mismatch should fail closed"))
        }
    }

    #[test]
    fn composite_drawable_decodes_binding_lists() -> Result<(), String> {
        let comp = composite_drawable_fixture()?;
        let json = require(
            composite_drawable_json(&comp),
            "composite fixture should decode",
        )?;

        require_json(
            &json,
            "\"skeleton_name\":\"hero_skel\"",
            "skeleton binding should be emitted",
        )?;
        require_json(
            &json,
            "\"kind\":\"skin\"",
            "skin binding should be emitted",
        )?;
        require_json(
            &json,
            "\"skeleton_joint_id\":7",
            "joint id should be emitted",
        )?;
        Ok(())
    }

    #[test]
    fn entity_dsg_escapes_trailing_nul_name_padding() -> Result<(), String> {
        let mut padded_name = String::from("groupShape135_000");
        padded_name.push(char::from(0));
        let mesh_fields = fields(
            vec![
                require(
                    pstring(&padded_name),
                    "padded mesh name should encode",
                )?,
                u32_field(0),
            ],
        );
        let mesh = require(
            chunk(
                MESH,
                mesh_fields,
                Vec::new(),
            ),
            "mesh fixture should build",
        )?;
        let entity_fields = fields(
            vec![
                require(
                    pstring(&padded_name),
                    "padded entity name should encode",
                )?,
                u32_field(0),
                u32_field(0),
            ],
        );
        let entity = require(
            chunk(
                0x03f0_0008,
                entity_fields,
                vec![mesh],
            ),
            "entity fixture should build",
        )?;
        let json = require(
            entity_dsg_json(&entity),
            "entity fixture should decode",
        )?;
        serde_json::from_str::<serde_json::Value>(&json)
            .map(|_value| ())
            .map_err(
                |error| format!("entity JSON must escape NUL padding: {error}"),
            )
    }

    #[test]
    fn insta_entity_decodes_render_refs_and_instance_scenegraphs()
    -> Result<(), String> {
        let mesh_fields = fields(
            vec![
                require(
                    pstring("crate_mesh"),
                    "mesh name should encode",
                )?,
                u32_field(0),
            ],
        );
        let mesh = require(
            chunk(
                MESH,
                mesh_fields,
                Vec::new(),
            ),
            "mesh fixture should build",
        )?;
        let instances = require(
            chunk(
                INSTANCES,
                require(
                    pstring("crate_instances"),
                    "instance name should encode",
                )?,
                vec![
                    require(
                        scenegraph_fixture(1),
                        "nested scenegraph should build",
                    )?,
                ],
            ),
            "instances fixture should build",
        )?;
        let entity_fields = fields(
            vec![
                require(
                    pstring("crate_entity"),
                    "entity name should encode",
                )?,
                u32_field(0),
                u32_field(0),
            ],
        );
        let entity = require(
            chunk(
                0x03f0_0009,
                entity_fields,
                vec![
                    mesh, instances,
                ],
            ),
            "entity fixture should build",
        )?;
        let json = require(
            insta_entity_dsg_json(&entity),
            "insta-entity fixture should decode",
        )?;

        require_json(
            &json,
            "\"schema\":\"srr_insta_entity_dsg\"",
            "schema should be emitted",
        )?;
        require_json(
            &json,
            "\"name\":\"crate_mesh\"",
            "mesh ref should be emitted",
        )?;
        require_json(
            &json,
            "\"scenegraphs\"",
            "instance scenegraph should be emitted",
        )?;
        Ok(())
    }
}

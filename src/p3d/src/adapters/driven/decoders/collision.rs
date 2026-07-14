// File:
//   - collision.rs
// Path:
//   - src/p3d/src/adapters/driven/decoders/collision.rs
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
//   - The p3d adapter boundary for adapters driven decoders collision.
// - Must-Not:
//   - Change domain semantics or bypass application and port contracts.
// - Allows:
//   - Filesystem, JSON, CLI, Blender, or serialization work behind explicit
//   - ports.
// - Split-When:
//   - Split when collision contains two independently testable contracts.
// - Merge-When:
//   - Another p3d module owns the same adapters boundary with no distinct
//   - invariant.
// - Summary:
//   - Collision, physics, DSG wrapper, and chunk-set decoders.
// - Description:
//   - Defines collision data and behavior for p3d adapters driven decoders.
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
//   - Reason: src/p3d/src/adapters/driven/decoders/collision.rs has 1727
//   - effective lines after the required header and remains cohesive until a
//   - focused split lands.
//

//! Collision, physics, DSG wrapper, and chunk-set decoders.
//!
//! These tranche-three payloads drive simulation behavior, so each decoder
//! checks declared counts against parsed arrays/children and returns `None` on
//! malformed data instead of producing partial JSON.
use std::collections::BTreeSet;

use super::reader::{
    Reader, SubChunk, read_instances_header, read_u32, subchunks,
};
use crate::adapters::driven::json::{escape_json as escape, render_f32};

/// Texture chunk id used by chunk-set payloads.
const TEXTURE: u32 = 0x0001_9000;
/// Mesh chunk id used for drawable references.
const MESH: u32 = 0x0001_0000;
/// Skin chunk id used for drawable references.
const SKIN: u32 = 0x0001_0001;
/// Scenegraph chunk id used inside instance chunks.
const SCENEGRAPH: u32 = 0x0012_0100;
/// Instance chunk id used by instanced physics DSG wrappers.
const INSTANCES: u32 = 0x0300_0600;
/// Legacy instance chunk id used by entity and animated wrappers.
const LEGACY_INSTANCES: u32 = 0x0300_0008;
/// Animated DSG wrapper chunk id.
const ANIM_DSG_WRAPPER: u32 = 0x03f0_000f;
/// Animated object DSG wrapper chunk id.
const ANIM_OBJ_DSG_WRAPPER: u32 = 0x03f0_0010;
/// Animation chunk id.
const ANIMATION: u32 = 0x0012_1000;
/// Skeleton chunk id.
const SKELETON: u32 = 0x0000_4500;
/// Composite drawable chunk id.
const COMPOSITE_DRAWABLE: u32 = 0x0000_4512;
/// Multi-controller chunk id.
const MULTI_CONTROLLER: u32 = 0x0000_48a0;
/// Billboard quad group chunk id.
const QUAD_GROUP: u32 = 0x0001_7002;
/// Animated object factory chunk id.
const ANIMATED_OBJECT_FACTORY: u32 = 0x0002_0000;
/// Animated object chunk id.
const ANIMATED_OBJECT: u32 = 0x0002_0001;
/// Animation frame-controller chunk id.
const FRAME_CONTROLLER: u32 = 0x0012_1200;
/// State prop chunk id.
const STATE_PROP: u32 = 0x0802_0000;
/// Static physics DSG chunk id.
const STATIC_PHYS_DSG: u32 = 0x03f0_0001;
/// Dynamic physics DSG chunk id.
const DYNA_PHYS_DSG: u32 = 0x03f0_0002;
/// Instanced entity DSG chunk id.
const INSTA_ENTITY_DSG: u32 = 0x03f0_0009;
/// Instanced static physics DSG chunk id.
const INSTA_STATIC_PHYS_DSG: u32 = 0x03f0_000a;
/// Instanced animated dynamic physics DSG chunk id.
const INSTA_ANIM_DYNA_PHYS_DSG: u32 = 0x03f0_000e;
/// Chunk-set membership chunk id.
const CHUNK_SET: u32 = 0x0300_0110;
/// Simulation collision object chunk id.
const COLLISION_OBJECT: u32 = 0x0701_0000;
/// Collision volume chunk id.
const COLLISION_VOLUME: u32 = 0x0701_0001;
/// Sphere volume chunk id.
const COLLISION_SPHERE: u32 = 0x0701_0002;
/// Cylinder volume chunk id.
const COLLISION_CYLINDER: u32 = 0x0701_0003;
/// Oriented box volume chunk id.
const COLLISION_OBBOX: u32 = 0x0701_0004;
/// Wall volume chunk id.
const COLLISION_WALL: u32 = 0x0701_0005;
/// Axis-aligned box volume chunk id.
const COLLISION_BBOX: u32 = 0x0701_0006;
/// Collision vector chunk id.
const COLLISION_VECTOR: u32 = 0x0701_0007;
/// Self-collision relationship chunk id.
const SELF_COLLISION: u32 = 0x0701_0020;
/// Collision owner list chunk id.
const COLLISION_OWNER: u32 = 0x0701_0021;
/// Collision owner name chunk id.
const COLLISION_OWNER_NAME: u32 = 0x0701_0022;
/// Collision attributes chunk id.
const COLLISION_ATTRIBUTE: u32 = 0x0701_0023;
/// Simulation physics object chunk id.
const PHYSICS_OBJECT: u32 = 0x0701_1000;
/// Physics inertia matrix chunk id.
const PHYSICS_INERTIA: u32 = 0x0701_1001;
/// Physics vector chunk id.
const PHYSICS_VECTOR: u32 = 0x0701_1002;
/// Physics joint chunk id.
const PHYSICS_JOINT: u32 = 0x0701_1020;

/// Decode a simulation collision object and its collision-volume hierarchy.
#[must_use]
pub fn object_json(chunk: &[u8]) -> Option<String> {
    let (_, header_size, total_size) = require_id(
        chunk,
        COLLISION_OBJECT,
    )?;
    let mut reader = Reader::new(
        chunk, 12,
    );
    let name = reader.pstring()?;
    let version = reader.u32()?;
    let string_data = reader.pstring()?;
    let sub_object_count = u32_to_usize(reader.u32()?)?;
    let owner_count = u32_to_usize(reader.u32()?)?;
    if reader.pos() != header_size {
        return None;
    }
    let children = subchunks(
        chunk,
        header_size,
        total_size,
    )?;
    let mut owners = Vec::new();
    let mut self_collisions = Vec::new();
    let mut volumes = Vec::new();
    let mut attributes = Vec::new();
    for child in children {
        match child.id {
            COLLISION_OWNER => owners.push(
                decode_owner(
                    chunk, &child,
                )?,
            ),
            SELF_COLLISION => self_collisions.push(
                decode_self_collision(
                    chunk, &child,
                )?,
            ),
            COLLISION_VOLUME => volumes.push(
                decode_volume(
                    chunk, &child,
                )?,
            ),
            COLLISION_ATTRIBUTE => attributes.push(
                decode_attribute(
                    chunk, &child,
                )?,
            ),
            _ => return None,
        }
    }
    if owners.len() != owner_count {
        return None;
    }
    Some(
        format!(
            concat!(
                "{{\"schema\":\"simulation_collision_object\",",
                "\"name\":\"{}\",\"version\":{},\"string_data\":\"{}\",",
                "\"num_sub_objects\":{},\"num_owners\":{},",
                "\"owners\":[{}],\"self_collisions\":[{}],",
                "\"volumes\":[{}],\"attributes\":[{}]}}\n"
            ),
            escape(&name),
            version,
            escape(&string_data),
            sub_object_count,
            owner_count,
            owners.join(","),
            self_collisions.join(","),
            volumes.join(","),
            attributes.join(",")
        ),
    )
}

/// Decode a simulation physics object and its mass/joint parameters.
#[must_use]
pub fn physics_json(chunk: &[u8]) -> Option<String> {
    let (_, header_size, total_size) = require_id(
        chunk,
        PHYSICS_OBJECT,
    )?;
    let mut reader = Reader::new(
        chunk, 12,
    );
    let name = reader.pstring()?;
    let version = reader.u32()?;
    let string_data = reader.pstring()?;
    let joint_count = reader.u32()?;
    let volume = reader.f32()?;
    let resting_sensitivity = reader.f32()?;
    if reader.pos() > header_size {
        return None;
    }
    let mut children = Vec::new();
    if reader.pos() < header_size {
        // Some static-instance physics chunks inline vector and inertia chunks
        // in the header region, so rejecting them would lose authored mass
        // data.
        children.extend(
            subchunks(
                chunk,
                reader.pos(),
                header_size,
            )?,
        );
    }
    children.extend(
        subchunks(
            chunk,
            header_size,
            total_size,
        )?,
    );
    let mut vectors = Vec::new();
    let mut inertia = Vec::new();
    let mut joints = Vec::new();
    let mut joint_indices = BTreeSet::new();
    for child in children {
        match child.id {
            PHYSICS_VECTOR => vectors.push(
                decode_physics_vector(
                    chunk, &child,
                )?,
            ),
            PHYSICS_INERTIA => inertia.push(
                decode_inertia(
                    chunk, &child,
                )?,
            ),
            PHYSICS_JOINT => {
                let index = read_u32(
                    chunk,
                    child.data_offset(),
                )?;
                if index >= joint_count || !joint_indices.insert(index) {
                    return None;
                }
                joints.push(
                    decode_joint(
                        chunk, &child,
                    )?,
                );
            }
            _ => return None,
        }
    }
    Some(
        format!(
            concat!(
                "{{\"schema\":\"simulation_physics_object\",",
                "\"name\":\"{}\",\"version\":{},\"string_data\":\"{}\",",
                "\"num_joints\":{},\"volume\":{},",
                "\"resting_sensitivity\":{},\"vectors\":[{}],",
                "\"inertia_matrices\":[{}],\"joints\":[{}]}}\n"
            ),
            escape(&name),
            version,
            escape(&string_data),
            joint_count,
            fmt_f32(volume),
            fmt_f32(resting_sensitivity),
            vectors.join(","),
            inertia.join(","),
            joints.join(",")
        ),
    )
}

/// Keeps decoded DSG child groups together because the renderer must preserve
/// their category boundaries without re-reading binary payloads.
#[derive(Default)]
struct DsgChildren {
    /// Drawable references remain ordered because render binding is
    /// positional.
    render_refs: Vec<String>,
    /// Collision objects stay separate because they become simulation assets.
    collision_objects: Vec<String>,
    /// Physics objects stay separate because they carry material and mass
    /// data.
    physics_objects: Vec<String>,
    /// Instance payloads remain grouped because each owns transform records.
    instances: Vec<String>,
    /// Animation wrappers stay attached because they coordinate rig
    /// components.
    animation_wrappers: Vec<String>,
}

impl DsgChildren {
    /// Decodes recognized child families while rejecting unsupported records.
    fn decode(
        chunk: &[u8],
        children: Vec<SubChunk>,
    ) -> Option<Self> {
        let mut decoded = Self::default();
        for child in children {
            match child.id {
                MESH | SKIN => decoded
                    .render_refs
                    .push(
                        decode_named_ref(
                            chunk, &child,
                        )?,
                    ),
                COLLISION_OBJECT => decoded
                    .collision_objects
                    .push(
                        object_json(
                            child_bytes(
                                chunk, &child,
                            )?,
                        )?
                        .trim()
                        .to_owned(),
                    ),
                PHYSICS_OBJECT => decoded
                    .physics_objects
                    .push(
                        physics_json(
                            child_bytes(
                                chunk, &child,
                            )?,
                        )?
                        .trim()
                        .to_owned(),
                    ),
                INSTANCES | LEGACY_INSTANCES => decoded
                    .instances
                    .push(
                        decode_instances(
                            chunk, &child,
                        )?,
                    ),
                ANIM_DSG_WRAPPER | ANIM_OBJ_DSG_WRAPPER => {
                    decoded
                        .animation_wrappers
                        .push(
                            decode_animation_wrapper(
                                chunk, &child,
                            )?,
                        );
                }
                _ => return None,
            }
        }
        Some(decoded)
    }

    /// Renders one deterministic JSON record after every child has decoded.
    fn render(
        &self,
        schema: &str,
        name: &str,
        version: u32,
        alpha: Option<u32>,
    ) -> String {
        let alpha_json = alpha.map_or_else(
            String::new,
            |value| format!(",\"has_alpha\":{value}"),
        );
        format!(
            concat!(
                "{{\"schema\":\"{}\",\"name\":\"{}\",\"version\":{}{},",
                "\"render_refs\":[{}],\"collision_objects\":[{}],",
                "\"physics_objects\":[{}],\"instances\":[{}],",
                "\"animation_wrappers\":[{}]}}\n"
            ),
            escape(schema),
            escape(name),
            version,
            alpha_json,
            self.render_refs
                .join(","),
            self.collision_objects
                .join(","),
            self.physics_objects
                .join(","),
            self.instances
                .join(","),
            self.animation_wrappers
                .join(",")
        )
    }
}

/// Decode a physics DSG wrapper and its embedded simulation children.
#[must_use]
pub fn dsg_json(
    chunk: &[u8],
    schema: &str,
) -> Option<String> {
    let (id, header_size, total_size) = chunk_bounds(chunk)?;
    if !matches!(
        id,
        STATIC_PHYS_DSG
            | DYNA_PHYS_DSG
            | INSTA_ENTITY_DSG
            | INSTA_STATIC_PHYS_DSG
            | INSTA_ANIM_DYNA_PHYS_DSG
    ) {
        return None;
    }
    let mut reader = Reader::new(
        chunk, 12,
    );
    let name = reader.pstring()?;
    let version = reader.u32()?;
    let alpha = if id == STATIC_PHYS_DSG {
        None
    } else {
        Some(reader.u32()?)
    };
    if reader.pos() != header_size {
        return None;
    }
    let children = DsgChildren::decode(
        chunk,
        subchunks(
            chunk,
            header_size,
            total_size,
        )?,
    )?;
    Some(
        children.render(
            schema, &name, version, alpha,
        ),
    )
}

/// Decode a chunk set and its child membership ids.
#[must_use]
pub fn chunk_set_json(chunk: &[u8]) -> Option<String> {
    let (_, header_size, total_size) = require_id(
        chunk, CHUNK_SET,
    )?;
    let mut reader = Reader::new(
        chunk, 12,
    );
    let name = reader.pstring()?;
    let version = reader.u32()?;
    let child_count = usize::from(
        read_u8(
            chunk,
            reader.pos(),
        )?,
    );
    if reader
        .pos()
        .checked_add(1)?
        != header_size
    {
        return None;
    }
    let children = subchunks(
        chunk,
        header_size,
        total_size,
    )?;
    if children.len() != child_count {
        return None;
    }
    let decoded_children = children
        .iter()
        .map(
            |child| {
                decode_chunk_set_child(
                    chunk, child,
                )
            },
        )
        .collect::<Option<Vec<_>>>()?;
    Some(
        format!(
            "{{\"schema\":\"chunk_set\",\"name\":\"{}\",\"version\":{},\"\
             child_count\":{},\"children\":[{}]}}
",
            escape(&name),
            version,
            child_count,
            decoded_children.join(",")
        ),
    )
}

/// Decode chunk-set children so membership is not stored as id-only summaries.
fn decode_chunk_set_child(
    chunk: &[u8],
    child: &SubChunk,
) -> Option<String> {
    match child.id {
        TEXTURE => decode_chunk_set_texture(
            chunk, child,
        ),
        MESH | SKIN => decode_named_ref(
            chunk, child,
        ),
        _ => None,
    }
}

/// Decode texture child headers embedded in SRR chunk-set payloads.
fn decode_chunk_set_texture(
    chunk: &[u8],
    child: &SubChunk,
) -> Option<String> {
    let mut reader = Reader::new(
        chunk,
        child.data_offset(),
    );
    let name = reader.pstring()?;
    let version = reader.u32()?;
    let width = reader.u32()?;
    let height = reader.u32()?;
    let bpp = reader.u32()?;
    let alpha_depth = reader.u32()?;
    let num_mipmaps = reader.u32()?;
    let texture_type = reader.u32()?;
    let usage = reader.u32()?;
    let priority = reader.u32()?;
    if reader.pos() != child.header_end() {
        return None;
    }
    let image_refs = subchunks(
        chunk,
        child.header_end(),
        child.end(),
    )?
    .iter()
    .map(
        |image| {
            format!(
                "{{\"chunk_id\":{},\"payload_size\":{}}}",
                image.id,
                image
                    .total_size
                    .saturating_sub(image.header_size)
            )
        },
    )
    .collect::<Vec<_>>();
    Some(
        format!(
            concat!(
                "{{\"kind\":\"texture\",\"chunk_id\":{},",
                "\"name\":\"{}\",\"version\":{},",
                "\"width\":{},\"height\":{},\"bpp\":{},",
                "\"alpha_depth\":{},\"num_mipmaps\":{},",
                "\"texture_type\":{},\"usage\":{},",
                "\"priority\":{},\"image_refs\":[{}]}}"
            ),
            child.id,
            escape(&name),
            version,
            width,
            height,
            bpp,
            alpha_depth,
            num_mipmaps,
            texture_type,
            usage,
            priority,
            image_refs.join(",")
        ),
    )
}

/// Decode collision owner names and verify the declared owner count.
fn decode_owner(
    chunk: &[u8],
    child: &SubChunk,
) -> Option<String> {
    let mut reader = Reader::new(
        chunk,
        child.data_offset(),
    );
    let count = u32_to_usize(reader.u32()?)?;
    if reader.pos() != child.header_end() {
        return None;
    }
    let children = subchunks(
        chunk,
        child.header_end(),
        child.end(),
    )?;
    if children.len() != count {
        return None;
    }
    let mut names = Vec::new();
    for name_child in children {
        if name_child.id != COLLISION_OWNER_NAME {
            return None;
        }
        let mut name_reader = Reader::new(
            chunk,
            name_child.data_offset(),
        );
        let name = name_reader.pstring()?;
        if !is_leaf_at(
            name_reader.pos(),
            &name_child,
        ) {
            return None;
        }
        names.push(
            format!(
                "\"{}\"",
                escape(&name)
            ),
        );
    }
    Some(
        format!(
            "{{\"names\":[{}]}}",
            names.join(",")
        ),
    )
}

/// Decode a relationship between two collision object indices.
fn decode_self_collision(
    chunk: &[u8],
    child: &SubChunk,
) -> Option<String> {
    let mut cursor = child.data_offset();
    let joint_index1 = read_u32(
        chunk, cursor,
    )?;
    cursor = cursor.checked_add(4)?;
    let joint_index2 = read_u32(
        chunk, cursor,
    )?;
    cursor = cursor.checked_add(4)?;
    let self_only1 = read_u16(
        chunk, cursor,
    )?;
    cursor = cursor.checked_add(2)?;
    let self_only2 = read_u16(
        chunk, cursor,
    )?;
    cursor = cursor.checked_add(2)?;
    if cursor != child.header_end() || child.header_end() != child.end() {
        return None;
    }
    Some(
        format!(
            "{{\"joint_index1\":{joint_index1},\"joint_index2\":\
             {joint_index2},\"self_only1\":{self_only1},\"self_only2\":\
             {self_only2}}}"
        ),
    )
}

/// Decode collision attributes that drive possible collision events.
fn decode_attribute(
    chunk: &[u8],
    child: &SubChunk,
) -> Option<String> {
    let mut cursor = child.data_offset();
    let static_attribute = read_u16(
        chunk, cursor,
    )?;
    cursor = cursor.checked_add(2)?;
    let default_area = read_u32(
        chunk, cursor,
    )?;
    cursor = cursor.checked_add(4)?;
    let can_roll = read_u16(
        chunk, cursor,
    )?;
    cursor = cursor.checked_add(2)?;
    let can_slide = read_u16(
        chunk, cursor,
    )?;
    cursor = cursor.checked_add(2)?;
    let can_spin = read_u16(
        chunk, cursor,
    )?;
    cursor = cursor.checked_add(2)?;
    let can_bounce = read_u16(
        chunk, cursor,
    )?;
    cursor = cursor.checked_add(2)?;
    let extra1 = read_u32(
        chunk, cursor,
    )?;
    cursor = cursor.checked_add(4)?;
    let extra2 = read_u32(
        chunk, cursor,
    )?;
    cursor = cursor.checked_add(4)?;
    let extra3 = read_u32(
        chunk, cursor,
    )?;
    cursor = cursor.checked_add(4)?;
    if cursor != child.header_end() || child.header_end() != child.end() {
        return None;
    }
    Some(
        format!(
            concat!(
                "{{\"static_attribute\":{},\"default_area\":{},",
                "\"can_roll\":{},\"can_slide\":{},\"can_spin\":{},",
                "\"can_bounce\":{},\"extra\":[{},{},{}]}}"
            ),
            static_attribute,
            default_area,
            can_roll,
            can_slide,
            can_spin,
            can_bounce,
            extra1,
            extra2,
            extra3
        ),
    )
}

/// Decode a collision volume and verify its declared child count.
fn decode_volume(
    chunk: &[u8],
    child: &SubChunk,
) -> Option<String> {
    let mut reader = Reader::new(
        chunk,
        child.data_offset(),
    );
    let object_reference_index = reader.u32()?;
    let owner_index = reader.u32()?;
    let subvolume_count = u32_to_usize(reader.u32()?)?;
    if reader.pos() != child.header_end() {
        return None;
    }
    let children = subchunks(
        chunk,
        child.header_end(),
        child.end(),
    )?;
    let mut bounds = Vec::new();
    let mut primitives = Vec::new();
    let mut nested_volume_count = 0_usize;
    for primitive in children {
        if primitive.id == COLLISION_BBOX {
            bounds.push(
                decode_primitive(
                    chunk, &primitive,
                )?,
            );
        } else {
            if primitive.id == COLLISION_VOLUME {
                nested_volume_count = nested_volume_count.checked_add(1)?;
            }
            primitives.push(
                decode_primitive(
                    chunk, &primitive,
                )?,
            );
        }
    }
    if nested_volume_count != subvolume_count {
        return None;
    }
    Some(
        format!(
            concat!(
                r#"{{"object_reference_index":{},"owner_index":{},"#,
                r#""num_subvolumes":{},"bounds":[{}],"#,
                r#""primitives":[{}]}}"#
            ),
            object_reference_index,
            owner_index,
            subvolume_count,
            bounds.join(","),
            primitives.join(",")
        ),
    )
}

/// Decode one collision primitive or recursive sub-volume.
fn decode_primitive(
    chunk: &[u8],
    child: &SubChunk,
) -> Option<String> {
    if child.id == COLLISION_VOLUME {
        return decode_volume(
            chunk, child,
        );
    }
    let kind = primitive_kind(child.id)?;
    let mut reader = Reader::new(
        chunk,
        child.data_offset(),
    );
    let fields = decode_primitive_fields(
        child.id,
        chunk,
        child,
        &mut reader,
    )?;
    if reader.pos() != child.header_end() {
        return None;
    }
    let vectors = decode_collision_vectors(
        chunk, child,
    )?;
    let field_json = if fields.is_empty() {
        String::new()
    } else {
        format!(",{fields}")
    };
    Some(
        format!(
            "{{\"kind\":\"{}\"{},\"vectors\":[{}]}}",
            kind,
            field_json,
            vectors.join(",")
        ),
    )
}

/// Decode primitive-specific scalar fields before vector children.
fn decode_primitive_fields(
    id: u32,
    chunk: &[u8],
    child: &SubChunk,
    reader: &mut Reader<'_>,
) -> Option<String> {
    match id {
        COLLISION_SPHERE => Some(
            format!(
                "\"radius\":{}",
                fmt_f32(reader.f32()?)
            ),
        ),
        COLLISION_CYLINDER => decode_cylinder_fields(
            chunk, reader,
        ),
        COLLISION_OBBOX => {
            let x = reader.f32()?;
            let y = reader.f32()?;
            let z = reader.f32()?;
            Some(
                format!(
                    "\"lengths\":[{},{},{}]",
                    fmt_f32(x),
                    fmt_f32(y),
                    fmt_f32(z)
                ),
            )
        }
        COLLISION_BBOX => {
            let marker = reader.u32()?;
            Some(format!("\"marker\":{marker}"))
        }
        COLLISION_WALL => Some(String::new()),
        _ => {
            let _ = child;
            None
        }
    }
}

/// Decode cylinder scalar fields including the flat-end flag.
fn decode_cylinder_fields(
    chunk: &[u8],
    reader: &mut Reader<'_>,
) -> Option<String> {
    let radius = reader.f32()?;
    let length = reader.f32()?;
    let flat_end = read_u16(
        chunk,
        reader.pos(),
    )?;
    reader.skip(2)?;
    Some(
        format!(
            "\"radius\":{},\"length\":{},\"flat_end\":{}",
            fmt_f32(radius),
            fmt_f32(length),
            flat_end
        ),
    )
}

/// Convert a collision primitive id into a stable JSON label.
const fn primitive_kind(id: u32) -> Option<&'static str> {
    match id {
        COLLISION_SPHERE => Some("sphere"),
        COLLISION_CYLINDER => Some("cylinder"),
        COLLISION_OBBOX => Some("obbox"),
        COLLISION_WALL => Some("wall"),
        COLLISION_BBOX => Some("bbox"),
        _ => None,
    }
}

/// Decode all vector children under a collision primitive.
fn decode_collision_vectors(
    chunk: &[u8],
    child: &SubChunk,
) -> Option<Vec<String>> {
    let children = subchunks(
        chunk,
        child.header_end(),
        child.end(),
    )?;
    let mut vectors = Vec::new();
    for vector in children {
        if vector.id != COLLISION_VECTOR {
            return None;
        }
        vectors.push(
            decode_vec3_chunk(
                chunk, &vector,
            )?,
        );
    }
    Some(vectors)
}

/// Decode a physics vector chunk.
fn decode_physics_vector(
    chunk: &[u8],
    child: &SubChunk,
) -> Option<String> {
    if child.id != PHYSICS_VECTOR {
        return None;
    }
    decode_vec3_chunk(
        chunk, child,
    )
}

/// Decode a vector leaf chunk and verify that it has no children.
fn decode_vec3_chunk(
    chunk: &[u8],
    child: &SubChunk,
) -> Option<String> {
    let mut reader = Reader::new(
        chunk,
        child.data_offset(),
    );
    let value = read_vec3(&mut reader)?;
    if !is_leaf_at(
        reader.pos(),
        child,
    ) {
        return None;
    }
    Some(value)
}

/// Decode one JSON vector value.
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

/// Decode a symmetric inertia matrix chunk.
fn decode_inertia(
    chunk: &[u8],
    child: &SubChunk,
) -> Option<String> {
    if child.id != PHYSICS_INERTIA {
        return None;
    }
    let mut reader = Reader::new(
        chunk,
        child.data_offset(),
    );
    let xx = reader.f32()?;
    let xy = reader.f32()?;
    let xz = reader.f32()?;
    let yy = reader.f32()?;
    let yz = reader.f32()?;
    let zz = reader.f32()?;
    if !is_leaf_at(
        reader.pos(),
        child,
    ) {
        return None;
    }
    Some(
        format!(
            "{{\"xx\":{},\"xy\":{},\"xz\":{},\"yy\":{},\"yz\":{},\"zz\":{}}}",
            fmt_f32(xx),
            fmt_f32(xy),
            fmt_f32(xz),
            fmt_f32(yy),
            fmt_f32(yz),
            fmt_f32(zz)
        ),
    )
}

/// Decode a physics joint and its local vector/inertia children.
fn decode_joint(
    chunk: &[u8],
    child: &SubChunk,
) -> Option<String> {
    let mut reader = Reader::new(
        chunk,
        child.data_offset(),
    );
    let index = reader.u32()?;
    let volume = reader.f32()?;
    let stiffness = reader.f32()?;
    let max_angle = reader.f32()?;
    let min_angle = reader.f32()?;
    let dof = reader.u32()?;
    if reader.pos() != child.header_end() {
        return None;
    }
    let children = subchunks(
        chunk,
        child.header_end(),
        child.end(),
    )?;
    let mut vectors = Vec::new();
    let mut inertia = Vec::new();
    for joint_child in children {
        match joint_child.id {
            PHYSICS_VECTOR => vectors.push(
                decode_physics_vector(
                    chunk,
                    &joint_child,
                )?,
            ),
            PHYSICS_INERTIA => inertia.push(
                decode_inertia(
                    chunk,
                    &joint_child,
                )?,
            ),
            _ => return None,
        }
    }
    Some(
        format!(
            concat!(
                "{{\"index\":{},\"volume\":{},\"stiffness\":{},",
                "\"max_angle\":{},\"min_angle\":{},\"dof\":{},",
                "\"vectors\":[{}],\"inertia_matrices\":[{}]}}"
            ),
            index,
            fmt_f32(volume),
            fmt_f32(stiffness),
            fmt_f32(max_angle),
            fmt_f32(min_angle),
            dof,
            vectors.join(","),
            inertia.join(",")
        ),
    )
}

/// Decode an animated DSG wrapper and all typed nested child payloads.
fn decode_animation_wrapper(
    chunk: &[u8],
    child: &SubChunk,
) -> Option<String> {
    let mut reader = Reader::new(
        chunk,
        child.data_offset(),
    );
    let name = reader.pstring()?;
    let (version, alpha) = if child.id == ANIM_OBJ_DSG_WRAPPER {
        (
            u32::from(reader.byte()?),
            u32::from(reader.byte()?),
        )
    } else {
        (
            reader.u32()?,
            reader.u32()?,
        )
    };
    if reader.pos() != child.header_end() {
        return None;
    }
    let mut decoded_children = Vec::new();
    for nested in subchunks(
        chunk,
        child.header_end(),
        child.end(),
    )? {
        decoded_children.push(
            decode_wrapper_child(
                chunk, &nested,
            )?,
        );
    }
    Some(
        format!(
            "{{\"chunk_id\":{},\"name\":\"{}\",\"version\":{},\"has_alpha\":\
             {},\"children\":[{}]}}",
            child.id,
            escape(&name),
            version,
            alpha,
            decoded_children.join(",")
        ),
    )
}

/// Decode one child inside an animated wrapper without falling back to
/// summaries.
fn decode_wrapper_child(
    chunk: &[u8],
    child: &SubChunk,
) -> Option<String> {
    let payload = child_bytes(
        chunk, child,
    )?;
    match child.id {
        ANIMATION => Some(
            wrap_child_json(
                "animation",
                &super::rig::animation_json(payload)?,
            ),
        ),
        SKELETON => Some(
            wrap_child_json(
                "skeleton",
                &super::rig::skeleton_json(payload)?,
            ),
        ),
        COMPOSITE_DRAWABLE => Some(
            wrap_child_json(
                "composite_drawable",
                &super::scene::composite_drawable_json(payload)?,
            ),
        ),
        MULTI_CONTROLLER => Some(
            wrap_child_json(
                "multi_controller",
                &super::rig::multi_controller_json(payload)?,
            ),
        ),
        COLLISION_OBJECT => Some(
            wrap_child_json(
                "collision_object",
                &object_json(payload)?,
            ),
        ),
        PHYSICS_OBJECT => Some(
            wrap_child_json(
                "physics_object",
                &physics_json(payload)?,
            ),
        ),
        MESH
        | SKIN
        | QUAD_GROUP
        | ANIMATED_OBJECT_FACTORY
        | ANIMATED_OBJECT
        | FRAME_CONTROLLER
        | STATE_PROP => Some(
            format!(
                "{{\"kind\":\"reference\",\"chunk_id\":{},\"name\":\"{}\"}}",
                child.id,
                escape(
                    &decode_named_ref(
                        chunk, child,
                    )?
                )
            ),
        ),
        INSTANCES | LEGACY_INSTANCES => Some(
            wrap_child_json(
                "instances",
                &decode_instances(
                    chunk, child,
                )?,
            ),
        ),
        ANIM_DSG_WRAPPER | ANIM_OBJ_DSG_WRAPPER => Some(
            wrap_child_json(
                "animation_wrapper",
                &decode_animation_wrapper(
                    chunk, child,
                )?,
            ),
        ),
        _ => None,
    }
}

/// Wrap a nested decoded payload with its parent-child role.
fn wrap_child_json(
    kind: &str,
    json: &str,
) -> String {
    format!(
        "{{\"kind\":\"{}\",\"payload\":{}}}",
        kind,
        json.trim()
    )
}

/// Decode a mesh or skin child name as a render reference.
fn decode_named_ref(
    chunk: &[u8],
    child: &SubChunk,
) -> Option<String> {
    let mut reader = Reader::new(
        chunk,
        child.data_offset(),
    );
    let name = reader.pstring()?;
    Some(
        format!(
            "{{\"chunk_id\":{},\"name\":\"{}\"}}",
            child.id,
            escape(&name)
        ),
    )
}

/// Decode instance scenegraphs carried by instanced physics wrappers.
fn decode_instances(
    chunk: &[u8],
    child: &SubChunk,
) -> Option<String> {
    let (version, flags, name) = read_instances_header(
        chunk, child,
    )?;
    let children = subchunks(
        chunk,
        child.header_end(),
        child.end(),
    )?;
    let mut scenegraphs = Vec::new();
    for graph in children {
        if graph.id != SCENEGRAPH {
            return None;
        }
        let bytes = child_bytes(
            chunk, &graph,
        )?;
        scenegraphs.push(
            super::scene::scenegraph_json(bytes)?
                .trim()
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

/// Read and validate the chunk header, requiring the expected id.
fn require_id(
    chunk: &[u8],
    expected_id: u32,
) -> Option<(
    u32,
    usize,
    usize,
)> {
    let bounds = chunk_bounds(chunk)?;
    (bounds.0 == expected_id).then_some(bounds)
}

/// Read and validate the chunk header bounds.
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

/// Return a borrowed child chunk byte slice.
fn child_bytes<'a>(
    chunk: &'a [u8],
    child: &SubChunk,
) -> Option<&'a [u8]> {
    chunk.get(child.offset..child.end())
}

/// Verify that a child leaf consumed its full header and has no subchunks.
fn is_leaf_at(
    pos: usize,
    child: &SubChunk,
) -> bool {
    pos == child.header_end() && child.header_end() == child.end()
}

/// Read one byte at an absolute offset.
fn read_u8(
    chunk: &[u8],
    offset: usize,
) -> Option<u8> {
    chunk
        .get(offset)
        .copied()
}

/// Read one little-endian u16 at an absolute offset.
fn read_u16(
    chunk: &[u8],
    offset: usize,
) -> Option<u16> {
    let bytes = chunk.get(offset..offset.checked_add(2)?)?;
    let array: [u8; 2] = bytes
        .try_into()
        .ok()?;
    Some(u16::from_le_bytes(array))
}

/// Convert a u32 count to usize without silent truncation.
fn u32_to_usize(value: u32) -> Option<usize> {
    usize::try_from(value).ok()
}

/// Format floats consistently for deterministic JSON.
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

    /// Builds a vector payload.
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

    /// Joins field fragments in schema order.
    fn fields(parts: Vec<Vec<u8>>) -> Vec<u8> {
        let mut out = Vec::new();
        for part in parts {
            out.extend(part);
        }
        out
    }

    /// Converts optional fixtures into descriptive test errors.
    fn require<T>(
        value: Option<T>,
        context: &str,
    ) -> Result<T, String> {
        value.ok_or_else(|| String::from(context))
    }

    /// Checks useful JSON output without panicking in tests.
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

    /// Builds a collision object with owner, volume, and attributes.
    fn collision_fixture() -> Option<Vec<u8>> {
        collision_fixture_with_subvolume_count(1)
    }

    /// Builds a collision object with an explicit subvolume count.
    fn collision_fixture_with_subvolume_count(
        subvolume_count: u32
    ) -> Option<Vec<u8>> {
        let owner_name = chunk(
            COLLISION_OWNER_NAME,
            pstring("joint_a")?,
            Vec::new(),
        )?;
        let owner = chunk(
            COLLISION_OWNER,
            u32_field(1),
            vec![owner_name],
        )?;
        let self_collision = chunk(
            SELF_COLLISION,
            fields(
                vec![
                    u32_field(1),
                    u32_field(2),
                    u16_field(1),
                    u16_field(0),
                ],
            ),
            Vec::new(),
        )?;
        let vector = chunk(
            COLLISION_VECTOR,
            vec3(
                1.0, 2.0, 3.0,
            ),
            Vec::new(),
        )?;
        let sphere = chunk(
            COLLISION_SPHERE,
            f32_field(5.0),
            vec![vector],
        )?;
        let leaf_volume = chunk(
            COLLISION_VOLUME,
            fields(
                vec![
                    u32_field(9),
                    u32_field(u32::MAX),
                    u32_field(0),
                ],
            ),
            vec![sphere],
        )?;
        let bounds = chunk(
            COLLISION_BBOX,
            u32_field(0),
            Vec::new(),
        )?;
        let volume = chunk(
            COLLISION_VOLUME,
            fields(
                vec![
                    u32_field(9),
                    u32_field(0),
                    u32_field(subvolume_count),
                ],
            ),
            vec![
                bounds,
                leaf_volume,
            ],
        )?;
        let attribute = chunk(
            COLLISION_ATTRIBUTE,
            fields(
                vec![
                    u16_field(1),
                    u32_field(7),
                    u16_field(1),
                    u16_field(0),
                    u16_field(1),
                    u16_field(0),
                    u32_field(10),
                    u32_field(11),
                    u32_field(12),
                ],
            ),
            Vec::new(),
        )?;
        chunk(
            COLLISION_OBJECT,
            fields(
                vec![
                    pstring("collider")?,
                    u32_field(0),
                    pstring("material")?,
                    u32_field(1),
                    u32_field(1),
                ],
            ),
            vec![
                owner,
                self_collision,
                volume,
                attribute,
            ],
        )
    }

    /// Builds a physics object with sparse joint records.
    fn physics_fixture() -> Option<Vec<u8>> {
        physics_fixture_with_joint_indices(
            7,
            &[
                0, 2, 5,
            ],
        )
    }

    /// Builds one explicit physics-joint record.
    fn physics_joint_fixture(index: u32) -> Option<Vec<u8>> {
        let joint_vector = chunk(
            PHYSICS_VECTOR,
            vec3(
                4.0, 5.0, 6.0,
            ),
            Vec::new(),
        )?;
        chunk(
            PHYSICS_JOINT,
            fields(
                vec![
                    u32_field(index),
                    f32_field(8.0),
                    f32_field(0.25),
                    f32_field(45.0),
                    f32_field(-45.0),
                    u32_field(2),
                ],
            ),
            vec![joint_vector],
        )
    }

    /// Builds a physics object with an explicit joint-index upper bound.
    fn physics_fixture_with_joint_indices(
        joint_count: u32,
        joint_indices: &[u32],
    ) -> Option<Vec<u8>> {
        let center = chunk(
            PHYSICS_VECTOR,
            vec3(
                0.5, 1.5, 2.5,
            ),
            Vec::new(),
        )?;
        let inertia = chunk(
            PHYSICS_INERTIA,
            fields(
                vec![
                    f32_field(1.0),
                    f32_field(0.1),
                    f32_field(0.2),
                    f32_field(2.0),
                    f32_field(0.3),
                    f32_field(3.0),
                ],
            ),
            Vec::new(),
        )?;
        let mut children = vec![
            center, inertia,
        ];
        for index in joint_indices {
            children.push(physics_joint_fixture(*index)?);
        }
        chunk(
            PHYSICS_OBJECT,
            fields(
                vec![
                    pstring("physics")?,
                    u32_field(0),
                    pstring("heavy")?,
                    u32_field(joint_count),
                    f32_field(12.0),
                    f32_field(0.75),
                ],
            ),
            children,
        )
    }

    /// Builds a dynamic physics DSG wrapper fixture.
    fn dsg_fixture() -> Option<Vec<u8>> {
        let mesh = chunk(
            MESH,
            fields(
                vec![
                    pstring("mesh_ref")?,
                    u32_field(0),
                ],
            ),
            Vec::new(),
        )?;
        chunk(
            DYNA_PHYS_DSG,
            fields(
                vec![
                    pstring("crate_phys")?,
                    u32_field(0),
                    u32_field(1),
                ],
            ),
            vec![
                mesh,
                collision_fixture()?,
                physics_fixture()?,
            ],
        )
    }

    /// Builds an instanced animated dynamic-physics wrapper.
    fn insta_anim_dyna_fixture() -> Option<Vec<u8>> {
        let wrapper = chunk(
            ANIM_OBJ_DSG_WRAPPER,
            fields(
                vec![
                    pstring("animated_wrapper")?,
                    vec![
                        0, 1,
                    ],
                ],
            ),
            vec![physics_fixture()?],
        )?;
        chunk(
            INSTA_ANIM_DYNA_PHYS_DSG,
            fields(
                vec![
                    pstring("animated_root")?,
                    u32_field(0),
                    u32_field(1),
                ],
            ),
            vec![wrapper],
        )
    }

    /// Builds a chunk set with one child member.
    fn chunk_set_fixture() -> Option<Vec<u8>> {
        let child_name = pstring("member_texture")?;
        let child = chunk(
            TEXTURE,
            fields(
                vec![
                    child_name,
                    u32_field(0),
                    u32_field(64),
                    u32_field(32),
                    u32_field(32),
                    u32_field(8),
                    u32_field(1),
                    u32_field(0),
                    u32_field(0),
                    u32_field(0),
                ],
            ),
            Vec::new(),
        )?;
        chunk(
            CHUNK_SET,
            fields(
                vec![
                    pstring("set_a")?,
                    u32_field(0),
                    vec![1_u8],
                ],
            ),
            vec![child],
        )
    }

    #[test]
    fn collision_object_decodes_volume_owner_and_attributes()
    -> Result<(), String> {
        let fixture = require(
            collision_fixture(),
            "collision fixture should build",
        )?;
        let json = require(
            object_json(&fixture),
            "collision fixture should decode",
        )?;
        require_json(
            &json,
            "\"schema\":\"simulation_collision_object\"",
            "schema should be emitted",
        )?;
        require_json(
            &json,
            "\"names\":[\"joint_a\"]",
            "owner name should be emitted",
        )?;
        require_json(
            &json,
            "\"kind\":\"sphere\"",
            "sphere primitive should be emitted",
        )?;
        require_json(
            &json,
            "\"num_subvolumes\":1",
            "nested volume count should be emitted",
        )?;
        require_json(
            &json,
            "\"bounds\":[{\"kind\":\"bbox\"",
            "bounding metadata should be emitted separately",
        )?;
        require_json(
            &json,
            "\"radius\":5.0",
            "sphere radius should be emitted",
        )?;
        require_json(
            &json,
            "\"can_spin\":1",
            "attribute flags should be emitted",
        )?;
        Ok(())
    }

    #[test]
    fn collision_volume_rejects_declared_subvolume_count_mismatch()
    -> Result<(), String> {
        let fixture = require(
            collision_fixture_with_subvolume_count(2),
            "collision fixture should build",
        )?;
        if object_json(&fixture).is_some() {
            return Err(
                String::from(
                    "collision volume decoder must reject a declared \
                     subvolume count that does not match child chunks",
                ),
            );
        }
        Ok(())
    }

    #[test]
    fn physics_object_decodes_mass_and_joint_parameters() -> Result<(), String>
    {
        let fixture = require(
            physics_fixture(),
            "physics fixture should build",
        )?;
        let json = require(
            physics_json(&fixture),
            "physics fixture should decode",
        )?;
        require_json(
            &json,
            "\"schema\":\"simulation_physics_object\"",
            "schema should be emitted",
        )?;
        require_json(
            &json,
            "\"volume\":12.0",
            "object volume should be emitted",
        )?;
        require_json(
            &json,
            "\"resting_sensitivity\":0.75",
            "resting sensitivity should be emitted",
        )?;
        require_json(
            &json,
            "\"stiffness\":0.25",
            "joint stiffness should be emitted",
        )?;
        require_json(
            &json,
            "\"dof\":2",
            "joint degree of freedom should be emitted",
        )?;
        Ok(())
    }

    #[test]
    fn physics_object_accepts_sparse_joint_records() -> Result<(), String> {
        let fixture = require(
            physics_fixture(),
            "sparse physics fixture should build",
        )?;
        let json = require(
            physics_json(&fixture),
            "sparse physics fixture should decode",
        )?;
        require_json(
            &json,
            "\"num_joints\":7",
            "joint-index upper bound should be emitted",
        )?;
        for index in [
            0_u32, 2, 5,
        ] {
            require_json(
                &json,
                &format!("\"index\":{index}"),
                "sparse joint record should be emitted",
            )?;
        }
        Ok(())
    }

    #[test]
    fn physics_object_rejects_joint_index_outside_declared_bound()
    -> Result<(), String> {
        let fixture = require(
            physics_fixture_with_joint_indices(
                3,
                &[3],
            ),
            "out-of-range physics fixture should build",
        )?;
        if physics_json(&fixture).is_some() {
            return Err(
                String::from(
                    "physics decoder must reject a joint index outside the \
                     declared upper bound",
                ),
            );
        }
        Ok(())
    }

    #[test]
    fn physics_object_rejects_duplicate_joint_indices() -> Result<(), String> {
        let fixture = require(
            physics_fixture_with_joint_indices(
                4,
                &[
                    2, 2,
                ],
            ),
            "duplicate-joint physics fixture should build",
        )?;
        if physics_json(&fixture).is_some() {
            return Err(
                String::from(
                    "physics decoder must reject duplicate sparse joint \
                     indices",
                ),
            );
        }
        Ok(())
    }

    #[test]
    fn instanced_animated_dynamic_wrapper_decodes_sparse_physics()
    -> Result<(), String> {
        let fixture = require(
            insta_anim_dyna_fixture(),
            "instanced animated dynamic fixture should build",
        )?;
        let json = require(
            dsg_json(
                &fixture,
                "insta_anim_dyna_phys_dsg",
            ),
            "instanced animated dynamic fixture should decode",
        )?;
        require_json(
            &json,
            "\"schema\":\"insta_anim_dyna_phys_dsg\"",
            "instanced animated dynamic schema should be emitted",
        )?;
        require_json(
            &json,
            "\"num_joints\":7",
            "nested sparse physics should be emitted",
        )?;
        Ok(())
    }

    #[test]
    fn dsg_wrapper_decodes_render_collision_and_physics_children()
    -> Result<(), String> {
        let fixture = require(
            dsg_fixture(),
            "dsg fixture should build",
        )?;
        let json = require(
            dsg_json(
                &fixture,
                "dyna_phys_dsg",
            ),
            "dsg fixture should decode",
        )?;
        require_json(
            &json,
            "\"schema\":\"dyna_phys_dsg\"",
            "schema should be emitted",
        )?;
        require_json(
            &json,
            "\"has_alpha\":1",
            "alpha flag should be emitted",
        )?;
        require_json(
            &json,
            "\"name\":\"mesh_ref\"",
            "render ref should be emitted",
        )?;
        require_json(
            &json,
            "\"collision_objects\":[{",
            "collision child should be embedded",
        )?;
        require_json(
            &json,
            "\"physics_objects\":[{",
            "physics child should be embedded",
        )?;
        Ok(())
    }

    #[test]
    fn chunk_set_decodes_texture_membership() -> Result<(), String> {
        let fixture = require(
            chunk_set_fixture(),
            "chunk-set fixture should build",
        )?;
        let json = require(
            chunk_set_json(&fixture),
            "chunk-set fixture should decode",
        )?;
        require_json(
            &json,
            "\"kind\":\"texture\"",
            "texture child should be typed",
        )?;
        require_json(
            &json,
            "member_texture",
            "texture child name should be emitted",
        )?;
        Ok(())
    }

    #[test]
    fn empty_chunk_set_decodes_header() -> Result<(), String> {
        let name = require(
            pstring("set_empty"),
            "empty chunk-set name should encode",
        )?;
        let fixture = require(
            chunk(
                CHUNK_SET,
                fields(
                    vec![
                        name,
                        u32_field(0),
                        vec![0_u8],
                    ],
                ),
                Vec::new(),
            ),
            "empty chunk-set fixture should build",
        )?;
        let json = require(
            chunk_set_json(&fixture),
            "empty chunk-set fixture should decode",
        )?;
        require_json(
            &json,
            "\"schema\":\"chunk_set\"",
            "schema should be emitted",
        )?;
        require_json(
            &json,
            "\"child_count\":0",
            "empty child count should be emitted",
        )?;
        Ok(())
    }
}

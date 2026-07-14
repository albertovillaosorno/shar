// File:
//   - chunk.rs
// Path:
//   - src/p3d/src/domain/chunk.rs
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
//   - Pure p3d domain rules for domain chunk.
// - Must-Not:
//   - Read files, parse generated indexes, invoke CLI code, or call writer
//   - adapters.
// - Allows:
//   - Value objects, invariant checks, and pure evidence-to-domain translation.
// - Split-When:
//   - Split when chunk contains two independently testable contracts.
// - Merge-When:
//   - Another p3d module owns the same domain boundary with no distinct
//   - invariant.
// - Summary:
//   - Defines p3d error for this module boundary.
// - Description:
//   - Defines chunk data and behavior for p3d domain.
// - Usage:
//   - Imported through crate domain facades or sibling domain modules.
// - Defaults:
//   - No filesystem paths, no external process calls, and no implicit IO
//   - defaults.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - true
//   - Reason: src/p3d/src/domain/chunk.rs has 542 effective lines after the
//   - required header and remains cohesive until a focused split lands.
//

//! This code defines p3d error as pure domain behavior for domain chunk.
use std::fmt;

use super::extract::prepare_p3d_bytes;

/// Maximum supported recursive chunk nesting depth.
const MAX_CHUNK_DEPTH: usize = 256;

#[derive(Debug)]
/// P3D error.
pub struct P3dError {
    /// Message.
    message: String,
}

impl P3dError {
    /// Invalid source.
    pub fn invalid_source(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for P3dError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for P3dError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Endian.
pub enum Endian {
    /// Item.
    Little,
    /// Item.
    Big,
}

// The explicit public type name distinguishes chunk taxonomy from raw ids.
#[expect(
    clippy::module_name_repetitions,
    reason = "Callers need an explicit chunk taxonomy type across parser \
              boundaries."
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Chunkkind.
pub enum ChunkKind {
    /// Item.
    Root,
    /// Item.
    TextBible,
    /// Item.
    Language,
    /// Item.
    Texture,
    /// Item.
    ImageData,
    /// Item.
    Mesh,
    /// Item.
    Skin,
    /// Item.
    Shader,
    /// Item.
    Animation,
    /// Item.
    Skeleton,
    /// Item.
    CompositeDrawable,
    /// Item.
    Camera,
    /// Item.
    Light,
    /// Item.
    LightGroup,
    /// Item.
    GameAttr,
    /// Item.
    ParticleSystemFactory,
    /// Item.
    ParticleSystem,
    /// Item.
    Scenegraph,
    /// Item.
    SrrRoad,
    /// Item.
    SrrIntersection,
    /// Item.
    SrrRoadSegmentData,
    /// Item.
    SrrEntityDsg,
    /// Item.
    SrrStaticPhysDsg,
    /// Item.
    SrrIntersectDsg,
    /// Item.
    SrrTreeDsg,
    /// Item.
    SrrFenceDsg,
    /// Item.
    SrrAnimCollDsg,
    /// Item.
    SrrWorldSphereDsg,
    /// Item.
    SrrAnimDsg,
    /// Item.
    Locator,
    /// Item.
    Sprite,
    /// Item.
    QuadGroup,
    /// Item.
    MultiController,
    /// Item.
    History,
    /// Item.
    ExportInfo,
    /// Item.
    AnimatedObjectFactory,
    /// Item.
    AnimatedObject,
    /// Item.
    VertexExpressionGroup,
    /// Item.
    VertexExpressionMixer,
    /// Item.
    TextureFont,
    /// Item.
    ScroobyProject,
    /// Item.
    FrameController,
    /// Item.
    FrameControllerVariantA,
    /// Item.
    FrameControllerVariantB,
    /// Item.
    VertexAnimKey,
    /// Item.
    SimulationCollisionObject,
    /// Item.
    SimulationPhysicsObject,
    /// Item.
    StateProp,
    /// Item.
    SrrLocator,
    /// Item.
    SrrPedPath,
    /// Item.
    SrrChunkSet,
    /// Item.
    SrrAttributeTable,
    /// Item.
    SrrBreakableObject,
    /// Item.
    SrrInstParticleSystem,
    /// Item.
    SrrFollowCam,
    /// Item.
    SrrDynaPhysDsg,
    /// Item.
    SrrInstaEntityDsg,
    /// Item.
    SrrInstaStaticPhysDsg,
    /// Item.
    SrrInstaAnimDynaPhysDsg,
    /// Item.
    SrrLensFlareDsg,
    /// Item.
    Unknown,
}

impl ChunkKind {
    /// Label.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Root => "root",
            Self::TextBible => "text_bible",
            Self::Language => "language",
            Self::Texture => "texture",
            Self::ImageData => "image_data",
            Self::Mesh => "mesh",
            Self::Skin => "skin",
            Self::Shader => "shader",
            Self::Animation => "animation",
            Self::Skeleton => "skeleton",
            Self::CompositeDrawable => "composite_drawable",
            Self::Camera => "camera",
            Self::Light => "light",
            Self::LightGroup => "light_group",
            Self::GameAttr => "game_attr",
            Self::ParticleSystemFactory => "particle_system_factory",
            Self::ParticleSystem => "particle_system",
            Self::Scenegraph => "scenegraph",
            Self::SrrRoad => "srr_road",
            Self::SrrIntersection => "srr_intersection",
            Self::SrrRoadSegmentData => "srr_road_segment_data",
            Self::SrrEntityDsg => "srr_entity_dsg",
            Self::SrrStaticPhysDsg => "srr_static_phys_dsg",
            Self::SrrIntersectDsg => "srr_intersect_dsg",
            Self::SrrTreeDsg => "srr_tree_dsg",
            Self::SrrFenceDsg => "srr_fence_dsg",
            Self::SrrAnimCollDsg => "srr_anim_coll_dsg",
            Self::SrrWorldSphereDsg => "srr_world_sphere_dsg",
            Self::SrrAnimDsg => "srr_anim_dsg",
            Self::Locator => "locator",
            Self::Sprite => "sprite",
            Self::QuadGroup => "quad_group",
            Self::MultiController => "multi_controller",
            Self::History => "history",
            Self::ExportInfo => "export_info",
            Self::AnimatedObjectFactory => "animated_object_factory",
            Self::AnimatedObject => "animated_object",
            Self::VertexExpressionGroup => "vertex_expression_group",
            Self::VertexExpressionMixer => "vertex_expression_mixer",
            Self::TextureFont => "texture_font",
            Self::ScroobyProject => "scrooby_project",
            Self::FrameController => "frame_controller",
            Self::FrameControllerVariantA => "frame_controller_variant_a",
            Self::FrameControllerVariantB => "frame_controller_variant_b",
            Self::VertexAnimKey => "vertex_anim_key",
            Self::SimulationCollisionObject => "simulation_collision_object",
            Self::SimulationPhysicsObject => "simulation_physics_object",
            Self::StateProp => "state_prop",
            Self::SrrLocator => "srr_locator",
            Self::SrrPedPath => "srr_ped_path",
            Self::SrrChunkSet => "srr_chunk_set",
            Self::SrrAttributeTable => "srr_attribute_table",
            Self::SrrBreakableObject => "srr_breakable_object",
            Self::SrrInstParticleSystem => "srr_inst_particle_system",
            Self::SrrFollowCam => "srr_follow_cam",
            Self::SrrDynaPhysDsg => "srr_dyna_phys_dsg",
            Self::SrrInstaEntityDsg => "srr_insta_entity_dsg",
            Self::SrrInstaStaticPhysDsg => "srr_insta_static_phys_dsg",
            Self::SrrInstaAnimDynaPhysDsg => "srr_insta_anim_dyna_phys_dsg",
            Self::SrrLensFlareDsg => "srr_lens_flare_dsg",
            Self::Unknown => "unknown",
        }
    }
}

// The explicit record name separates parsed chunks from schema constants.
#[expect(
    clippy::module_name_repetitions,
    reason = "Callers need an explicit parsed chunk record across parser \
              boundaries."
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Chunkrecord.
pub struct ChunkRecord {
    /// Ordinal.
    pub ordinal: usize,
    /// Depth.
    pub depth: usize,
    /// Parent ordinal.
    pub parent_ordinal: Option<usize>,
    /// Id.
    pub id: u32,
    /// Kind.
    pub kind: ChunkKind,
    /// Offset.
    pub offset: usize,
    /// Header size.
    pub header_size: usize,
    /// Total size.
    pub total_size: usize,
    /// Payload offset.
    pub payload_offset: usize,
    /// Payload size.
    pub payload_size: usize,
    /// Child count.
    pub child_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// P3D document.
pub struct P3dDocument {
    /// Endian.
    pub endian: Endian,
    /// Compression.
    pub compression: &'static str,
    /// Byte len.
    pub byte_len: usize,
    /// Chunks.
    pub chunks: Vec<ChunkRecord>,
}

/// Analyze p3d.
///
/// # Errors
///
/// Returns an error when source parsing or filesystem output fails.
pub fn analyze_p3d(source: &[u8]) -> Result<P3dDocument, P3dError> {
    let prepared = prepare_p3d_bytes(source)?;
    let bytes = prepared
        .bytes
        .as_ref();
    let endian = detect_endian(bytes)?;
    let mut chunks = Vec::new();
    parse_range(
        bytes,
        endian,
        0,
        bytes.len(),
        0,
        None,
        &mut chunks,
    )?;
    let root = chunks
        .first()
        .ok_or_else(|| P3dError::invalid_source("missing root chunk"))?;
    if root.total_size != bytes.len() {
        return Err(
            P3dError::invalid_source("root chunk does not span the document"),
        );
    }
    let child_counts = compute_child_counts(&chunks)?;
    for (chunk, count) in chunks
        .iter_mut()
        .zip(child_counts)
    {
        chunk.child_count = count;
    }
    Ok(
        P3dDocument {
            endian,
            compression: prepared.compression,
            byte_len: bytes.len(),
            chunks,
        },
    )
}

/// Parse range.
// Recursive parsing uses validated binary offsets and checked outer bounds.
#[expect(
    clippy::arithmetic_side_effects,
    clippy::as_conversions,
    reason = "Validated Pure3D chunk bounds constrain recursive offset \
              arithmetic."
)]
fn parse_range(
    bytes: &[u8],
    endian: Endian,
    start: usize,
    end: usize,
    depth: usize,
    parent_ordinal: Option<usize>,
    chunks: &mut Vec<ChunkRecord>,
) -> Result<(), P3dError> {
    if depth > MAX_CHUNK_DEPTH {
        return Err(
            P3dError::invalid_source("chunk nesting exceeds safe limit"),
        );
    }
    let mut cursor = start;
    while cursor < end {
        let header_end = cursor
            .checked_add(12)
            .ok_or_else(
                || P3dError::invalid_source("chunk header end overflow"),
            )?;
        if header_end > end {
            break;
        }
        let id = read_u32(
            bytes, cursor, endian,
        )?;
        let header_size = read_u32(
            bytes,
            cursor + 4,
            endian,
        )? as usize;
        let total_size = read_u32(
            bytes,
            cursor + 8,
            endian,
        )? as usize;
        let chunk_end = cursor
            .checked_add(total_size)
            .ok_or_else(|| P3dError::invalid_source("chunk end overflow"))?;
        if header_size < 12 || total_size < header_size || chunk_end > end {
            return Err(P3dError::invalid_source("invalid chunk bounds"));
        }
        let ordinal = chunks.len();
        chunks.push(
            ChunkRecord {
                ordinal,
                depth,
                parent_ordinal,
                id,
                kind: classify_chunk(id),
                offset: cursor,
                header_size,
                total_size,
                payload_offset: cursor + header_size,
                payload_size: total_size - header_size,
                child_count: 0,
            },
        );
        parse_range(
            bytes,
            endian,
            cursor + header_size,
            chunk_end,
            depth + 1,
            Some(ordinal),
            chunks,
        )?;
        cursor = chunk_end;
    }
    if cursor != end {
        return Err(
            P3dError::invalid_source("trailing bytes after chunk stream"),
        );
    }
    Ok(())
}

/// Compute child counts.
fn compute_child_counts(
    chunks: &[ChunkRecord]
) -> Result<Vec<usize>, P3dError> {
    let mut counts = vec![0usize; chunks.len()];
    for chunk in chunks {
        let Some(parent) = chunk.parent_ordinal else {
            continue;
        };
        let count = counts
            .get_mut(parent)
            .ok_or_else(
                || {
                    P3dError::invalid_source(
                        "chunk parent ordinal is out of bounds",
                    )
                },
            )?;
        *count = count
            .checked_add(1)
            .ok_or_else(
                || P3dError::invalid_source("chunk child count overflowed"),
            )?;
    }
    Ok(counts)
}

/// Detect endian.
fn detect_endian(bytes: &[u8]) -> Result<Endian, P3dError> {
    if read_u32(
        bytes,
        0,
        Endian::Little,
    )? == 0xFF44_3350
    {
        return Ok(Endian::Little);
    }
    if read_u32(
        bytes,
        0,
        Endian::Big,
    )? == 0xFF44_3350
    {
        return Ok(Endian::Big);
    }
    Err(P3dError::invalid_source("missing Pure3D root chunk"))
}

/// Read u32.
// The four-byte slice is bounds checked before fixed-width binary conversion.
#[expect(
    clippy::indexing_slicing,
    reason = "A checked four-byte window guarantees safe fixed-width decoding."
)]
fn read_u32(
    bytes: &[u8],
    offset: usize,
    endian: Endian,
) -> Result<u32, P3dError> {
    let end = offset
        .checked_add(4)
        .ok_or_else(|| P3dError::invalid_source("u32 offset overflow"))?;
    let slice = bytes
        .get(offset..end)
        .ok_or_else(|| P3dError::invalid_source("unexpected end of u32"))?;
    let array = [
        slice[0], slice[1], slice[2], slice[3],
    ];
    Ok(
        match endian {
            Endian::Little => u32::from_le_bytes(array),
            Endian::Big => u32::from_be_bytes(array),
        },
    )
}

/// Classify chunk.
const fn classify_chunk(id: u32) -> ChunkKind {
    match id {
        0xFF44_3350 => ChunkKind::Root,
        0x0001_800D => ChunkKind::TextBible,
        0x0001_800E => ChunkKind::Language,
        0x0001_9000 => ChunkKind::Texture,
        0x0001_9002 => ChunkKind::ImageData,
        0x0001_0000 => ChunkKind::Mesh,
        0x0001_0001 => ChunkKind::Skin,
        0x0001_1000 => ChunkKind::Shader,
        0x0012_1000 => ChunkKind::Animation,
        0x0000_4500 => ChunkKind::Skeleton,
        0x0000_4512 => ChunkKind::CompositeDrawable,
        0x0000_2200 => ChunkKind::Camera,
        0x0000_2380 => ChunkKind::LightGroup,
        0x0001_2000 => ChunkKind::GameAttr,
        0x0001_3000 => ChunkKind::Light,
        0x0001_5800 => ChunkKind::ParticleSystemFactory,
        0x0001_5801 => ChunkKind::ParticleSystem,
        0x0012_0100 => ChunkKind::Scenegraph,
        0x0300_0003 => ChunkKind::SrrRoad,
        0x0300_0004 => ChunkKind::SrrIntersection,
        0x0300_0009 => ChunkKind::SrrRoadSegmentData,
        0x03f0_0000 => ChunkKind::SrrEntityDsg,
        0x03f0_0001 => ChunkKind::SrrStaticPhysDsg,
        0x03f0_0003 => ChunkKind::SrrIntersectDsg,
        0x03f0_0004 => ChunkKind::SrrTreeDsg,
        0x03f0_0007 => ChunkKind::SrrFenceDsg,
        0x03f0_0008 => ChunkKind::SrrAnimCollDsg,
        0x03f0_000b => ChunkKind::SrrWorldSphereDsg,
        0x03f0_000c => ChunkKind::SrrAnimDsg,
        0x0001_4000 => ChunkKind::Locator,
        0x0001_9005 => ChunkKind::Sprite,
        0x0001_7002 => ChunkKind::QuadGroup,
        0x0000_48a0 => ChunkKind::MultiController,
        0x0000_7000 => ChunkKind::History,
        0x0000_7030 => ChunkKind::ExportInfo,
        0x0002_0000 => ChunkKind::AnimatedObjectFactory,
        0x0002_0001 => ChunkKind::AnimatedObject,
        0x0002_1001 => ChunkKind::VertexExpressionGroup,
        0x0002_1002 => ChunkKind::VertexExpressionMixer,
        0x0002_2000 => ChunkKind::TextureFont,
        0x0001_8000 => ChunkKind::ScroobyProject,
        0x0012_1200 => ChunkKind::FrameController,
        0x0012_1201 => ChunkKind::FrameControllerVariantA,
        0x0012_1202 => ChunkKind::FrameControllerVariantB,
        0x0012_1304 => ChunkKind::VertexAnimKey,
        0x0701_0000 => ChunkKind::SimulationCollisionObject,
        0x0701_1000 => ChunkKind::SimulationPhysicsObject,
        0x0802_0000 => ChunkKind::StateProp,
        0x0300_0005 => ChunkKind::SrrLocator,
        0x0300_000b => ChunkKind::SrrPedPath,
        0x0300_0110 => ChunkKind::SrrChunkSet,
        0x0300_0602 => ChunkKind::SrrAttributeTable,
        0x0300_1000 => ChunkKind::SrrBreakableObject,
        0x0300_1001 => ChunkKind::SrrInstParticleSystem,
        0x0300_0100 => ChunkKind::SrrFollowCam,
        0x03f0_0002 => ChunkKind::SrrDynaPhysDsg,
        0x03f0_0009 => ChunkKind::SrrInstaEntityDsg,
        0x03f0_000a => ChunkKind::SrrInstaStaticPhysDsg,
        0x03f0_000e => ChunkKind::SrrInstaAnimDynaPhysDsg,
        0x03f0_000d => ChunkKind::SrrLensFlareDsg,
        _ => ChunkKind::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::{Endian, analyze_p3d, parse_range, read_u32};

    #[test]
    fn chunk_u32_reader_rejects_offset_overflow() -> Result<(), String> {
        let read = read_u32(
            &[],
            usize::MAX,
            Endian::Little,
        );
        if read.is_ok() {
            return Err(
                String::from(
                    "chunk u32 reads must reject an offset that cannot \
                     contain four bytes",
                ),
            );
        }
        Ok(())
    }

    #[test]
    fn empty_maximum_range_does_not_overflow() -> Result<(), String> {
        let mut chunks = Vec::new();
        parse_range(
            &[],
            Endian::Little,
            usize::MAX,
            usize::MAX,
            0,
            None,
            &mut chunks,
        )
        .map_err(|error| error.to_string())
    }
    #[test]
    fn parses_minimal_root() -> Result<(), String> {
        let bytes = [
            0x50, 0x33, 0x44, 0xFF, 12, 0, 0, 0, 12, 0, 0, 0,
        ];
        let document =
            analyze_p3d(&bytes).map_err(|error| error.to_string())?;
        if document
            .chunks
            .len()
            != 1
        {
            return Err(
                "minimal root must produce exactly one chunk".to_owned(),
            );
        }
        let root = document
            .chunks
            .first()
            .ok_or_else(
                || "parsed document must contain the root chunk".to_owned(),
            )?;
        if root
            .kind
            .label()
            != "root"
        {
            return Err(
                "minimal root chunk must retain the root kind".to_owned(),
            );
        }
        Ok(())
    }
}

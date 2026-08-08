// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Nested model component tests unit tests.
// - Must-Not:
//   - Own production behavior or broaden the tested API surface.
// - Allows:
//   - Private test fixtures and assertions for the owning source module.
// - Split-When:
//   - Split when an independent fixture family gains separate ownership.
// - Merge-When:
//   - Merge when another test module owns the identical evidence.
// - Summary:
//   - Nested model component tests unit tests.
// - Description:
//   - Preserves unit-test access through a test-only path module.
// - Usage:
//   - Included only by the owning source module under cfg(test).
// - Defaults:
//   - Test setup and assertions fail explicitly.
//

//! Nested model component tests unit tests.

use super::{should_publish_component, top_level_ancestor_ordinal};
use crate::{ChunkKind, ChunkRecord};

fn chunk(
    ordinal: usize,
    parent_ordinal: Option<usize>,
    kind: ChunkKind,
) -> ChunkRecord {
    ChunkRecord {
        ordinal,
        depth: ordinal,
        parent_ordinal,
        id: 0,
        kind,
        offset: 0,
        header_size: 12,
        total_size: 12,
        payload_offset: 12,
        payload_size: 0,
        child_count: 0,
    }
}

#[test]
fn nested_mesh_under_dynamic_world_container_is_published() {
    let chunks = [
        chunk(0, None, ChunkKind::Root),
        chunk(1, Some(0), ChunkKind::SrrDynaPhysDsg),
        chunk(2, Some(1), ChunkKind::Mesh),
    ];

    assert!(should_publish_component(&chunks[1], &chunks));
    assert!(should_publish_component(&chunks[2], &chunks));
}

#[test]
fn nested_mesh_under_static_world_containers_is_published() {
    for kind in [
        ChunkKind::SrrEntityDsg,
        ChunkKind::SrrInstaEntityDsg,
        ChunkKind::SrrStaticPhysDsg,
        ChunkKind::SrrInstaStaticPhysDsg,
    ] {
        let chunks = [
            chunk(0, None, ChunkKind::Root),
            chunk(1, Some(0), kind),
            chunk(2, Some(1), ChunkKind::Mesh),
        ];
        assert!(should_publish_component(&chunks[2], &chunks));
    }
}

#[test]
fn nested_chunk_set_texture_is_published() {
    let chunks = [
        chunk(0, None, ChunkKind::Root),
        chunk(1, Some(0), ChunkKind::SrrChunkSet),
        chunk(2, Some(1), ChunkKind::Texture),
    ];
    assert!(should_publish_component(&chunks[2], &chunks));
}

#[test]
fn nested_texture_font_atlas_is_published() {
    let chunks = [
        chunk(0, None, ChunkKind::Root),
        chunk(1, Some(0), ChunkKind::TextureFont),
        chunk(2, Some(1), ChunkKind::Texture),
    ];
    assert!(should_publish_component(&chunks[2], &chunks));
}

#[test]
fn unrelated_nested_texture_is_not_published() {
    let chunks = [
        chunk(0, None, ChunkKind::Root),
        chunk(1, Some(0), ChunkKind::Mesh),
        chunk(2, Some(1), ChunkKind::Texture),
    ];
    assert!(!should_publish_component(&chunks[2], &chunks));
}

#[test]
fn nested_model_support_may_pass_through_mesh_ancestors() {
    let chunks = [
        chunk(0, None, ChunkKind::Root),
        chunk(1, Some(0), ChunkKind::SrrBreakableObject),
        chunk(2, Some(1), ChunkKind::Mesh),
        chunk(3, Some(2), ChunkKind::Skeleton),
    ];

    assert!(should_publish_component(&chunks[3], &chunks));
}

#[test]
fn nested_component_records_owning_root_child() -> Result<(), String> {
    let chunks = [
        chunk(0, None, ChunkKind::Root),
        chunk(1, Some(0), ChunkKind::SrrDynaPhysDsg),
        chunk(2, Some(1), ChunkKind::Mesh),
        chunk(3, Some(2), ChunkKind::Skeleton),
    ];

    let container = top_level_ancestor_ordinal(&chunks[3], &chunks)
        .map_err(|error| error.to_string())?;

    if container != 1 {
        return Err(format!(
            "expected owning root child 1, received {container}"
        ));
    }
    Ok(())
}

#[test]
fn unrelated_nested_components_remain_inside_parent_evidence() {
    let chunks = [
        chunk(0, None, ChunkKind::Root),
        chunk(1, Some(0), ChunkKind::SrrDynaPhysDsg),
        chunk(2, Some(1), ChunkKind::ParticleSystem),
        chunk(3, Some(2), ChunkKind::Mesh),
    ];

    assert!(!should_publish_component(&chunks[2], &chunks));
    assert!(should_publish_component(&chunks[3], &chunks));
}

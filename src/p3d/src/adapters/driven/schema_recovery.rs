// File:
//   - schema_recovery.rs
// Path:
//   - src/p3d/src/adapters/driven/schema_recovery.rs
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
//   - Render, rig, scene, and controller schema dispatch for P3D recovery.
// - Must-Not:
//   - Read files, write artifacts, or choose package output locations.
// - Allows:
//   - Typed dispatch to decoder functions owned by the parent extractor module.
// - Split-When:
//   - Split when one schema family requires independent dispatch invariants.
// - Merge-When:
//   - Merge when the parent extractor no longer separates schema families.
// - Summary:
//   - Render schema recovery dispatch.
// - Description:
//   - Maps validated chunk kinds to render-oriented recovery functions.
// - Usage:
//   - Called by the parent extractor after foundational schema dispatch fails.
// - Defaults:
//   - Unknown chunk kinds return no recovered component and remain visible.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Render-oriented schema recovery dispatch for validated P3D chunks.
//!
//! This child module separates presentation decoding from filesystem package
//! export while preserving the parent module's private recovery helpers.

/// Routes render, rig, scene, and controller schema families.
pub(super) fn recover_render_schema_json(
    component: &crate::ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<super::extractor::RecoveredComponent> {
    match component
        .kind
        .label()
    {
        "mesh" => super::extractor::recover_mesh_json(
            component, source, kind_index,
        ),
        "skin" => super::extractor::recover_skin_json(
            component, source, kind_index,
        ),
        "skeleton" => super::extractor::recover_skeleton_json(
            component, source, kind_index,
        ),
        "camera" => super::extractor::recover_camera_json(
            component, source, kind_index,
        ),
        "composite_drawable" => super::extractor::recover_composite_json(
            component, source, kind_index,
        ),
        "animation" => super::extractor::recover_animation_json(
            component, source, kind_index,
        ),
        "particle_system_factory" => {
            super::extractor::recover_particle_factory_json(
                component, source, kind_index,
            )
        }
        "particle_system" => super::extractor::recover_particle_system_json(
            component, source, kind_index,
        ),
        "scenegraph" => super::extractor::recover_scenegraph_json(
            component, source, kind_index,
        ),
        "light_group" => super::extractor::recover_light_group_json(
            component, source, kind_index,
        ),
        "srr_world_sphere_dsg" => super::extractor::recover_world_sphere_json(
            component, source, kind_index,
        ),
        "frame_controller"
        | "frame_controller_variant_a"
        | "frame_controller_variant_b" => {
            super::extractor::recover_frame_controller_json(
                component, source, kind_index,
            )
        }
        "sprite" => super::extractor::recover_sprite_json(
            component, source, kind_index,
        ),
        "multi_controller" => super::extractor::recover_multi_controller_json(
            component, source, kind_index,
        ),
        "vertex_anim_key" => super::extractor::recover_vertex_anim_key_json(
            component, source, kind_index,
        ),
        "history" => super::extractor::recover_history_json(
            component, source, kind_index,
        ),
        _ => None,
    }
}

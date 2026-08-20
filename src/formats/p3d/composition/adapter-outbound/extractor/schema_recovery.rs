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
//   - Schema recovery outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Schema recovery outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for p3d.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Schema recovery outbound adapter.

pub(super) fn recover_render_schema_json(
    component: &crate::ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<super::RecoveredComponent> {
    match component.kind.label() {
        "mesh" => {
            super::render::recover_mesh_json(component, source, kind_index)
        },
        "skin" => {
            super::render::recover_skin_json(component, source, kind_index)
        },
        "skeleton" => {
            super::render::recover_skeleton_json(component, source, kind_index)
        },
        "camera" => {
            super::render::recover_camera_json(component, source, kind_index)
        },
        "composite_drawable" => {
            super::render::recover_composite_json(component, source, kind_index)
        },
        "animation" => {
            super::render::recover_animation_json(component, source, kind_index)
        },
        "particle_system_factory" => {
            super::render::recover_particle_factory_json(
                component, source, kind_index,
            )
        },
        "particle_system" => super::render::recover_particle_system_json(
            component, source, kind_index,
        ),
        "scenegraph" => super::render::recover_scenegraph_json(
            component, source, kind_index,
        ),
        "light_group" => super::render::recover_light_group_json(
            component, source, kind_index,
        ),
        "srr_world_sphere_dsg" => super::render::recover_world_sphere_json(
            component, source, kind_index,
        ),
        "frame_controller"
        | "frame_controller_variant_a"
        | "frame_controller_variant_b" => {
            super::render::recover_frame_controller_json(
                component, source, kind_index,
            )
        },
        "sprite" => {
            super::render::recover_sprite_json(component, source, kind_index)
        },
        "multi_controller" => super::auxiliary::recover_multi_controller_json(
            component, source, kind_index,
        ),
        "vertex_anim_key" => super::auxiliary::recover_vertex_anim_key_json(
            component, source, kind_index,
        ),
        "history" => super::auxiliary::recover_history_json(
            component, source, kind_index,
        ),
        _ => None,
    }
}

// File:
//   - build_scene.rs
// Path:
//   - src/fbx/src/application/build_scene.rs
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
//   - fbx use-case orchestration for application build scene.
// - Must-Not:
//   - Depend on driven adapters, parse local routes, or encode writer-specific
//   - syntax.
// - Allows:
//   - Use-case orchestration, planning, reporting, and calls through declared
//   - ports.
// - Split-When:
//   - Split when build scene contains two independently testable contracts.
// - Merge-When:
//   - Another fbx module owns the same application boundary with no distinct
//   - invariant.
// - Summary:
//   - Build a domain scene for one planned model export.
// - Description:
//   - Defines build scene data and behavior for fbx application.
// - Usage:
//   - Called by entrypoints after ports and adapters are selected by the
//   - caller.
// - Defaults:
//   - No concrete adapter is selected unless the caller supplies one through a
//   - port.
//
// ADRs:
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
// - docs/adr/pipeline/unreal/unreal-manifest-and-package-taxonomy.md
//
// Large file:
//   - false
//

//! Build a domain scene for one planned model export.
use crate::application::planning::ModelExportPlan;
use crate::domain::material::material_bindings_to_materials;
use crate::domain::mesh::mesh_asset_to_geometry;
use crate::domain::scene::{Scene, SceneNode};
use crate::domain::transform::Transform;
use crate::ports::component_source::ComponentSource;

/// Build a domain scene for one planned model export.
///
/// # Errors
///
/// Returns a component-source error when required decoded evidence is missing.
pub fn build_scene<Source>(
    plan: &ModelExportPlan,
    source: &Source,
) -> Result<Scene, Source::Error>
where
    Source: ComponentSource,
{
    let mut nodes = Vec::new();
    for member_id in &plan.model_member_ids {
        let mesh = source.load_mesh(member_id)?;
        for geometry in mesh_asset_to_geometry(&mesh) {
            nodes.push(
                SceneNode {
                    id: geometry
                        .id
                        .clone(),
                    parent_id: None,
                    local_transform: Transform::identity(),
                    geometry: Some(geometry),
                },
            );
        }
    }
    let mut bindings = Vec::with_capacity(
        plan.material_member_ids
            .len(),
    );
    for material_id in &plan.material_member_ids {
        bindings.push(source.resolve_material(material_id)?);
    }
    Ok(
        Scene {
            id: plan
                .package_id
                .clone(),
            nodes,
            materials: material_bindings_to_materials(&bindings),
            capabilities: plan
                .capability_report
                .clone(),
        },
    )
}

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
//   - Build scene application service.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Build scene application service.
// - Description:
//   - Implements the declared application service responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Build scene application service.

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
            nodes.push(SceneNode {
                id: geometry.id.clone(),
                parent_id: None,
                local_transform: Transform::identity(),
                geometry: Some(geometry),
            });
        }
    }
    let mut bindings = Vec::with_capacity(plan.material_member_ids.len());
    for material_id in &plan.material_member_ids {
        bindings.push(source.resolve_material(material_id)?);
    }
    Ok(Scene {
        id: plan.package_id.clone(),
        nodes,
        materials: material_bindings_to_materials(&bindings),
        capabilities: plan.capability_report.clone(),
    })
}

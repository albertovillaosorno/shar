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
//   - Translator domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Translator domain module.
// - Description:
//   - Implements the declared domain module responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Translator domain module.

use super::definition::Material;
use crate::domain::texture::{MaterialBinding, TextureReference};

/// Convert resolved material bindings into domain materials.
#[must_use]
pub fn material_bindings_to_materials(
    bindings: &[MaterialBinding],
) -> Vec<Material> {
    bindings
        .iter()
        .map(|binding| Material {
            id: binding.material_name.clone(),
            diffuse_texture: binding.texture_file_name.as_ref().map(|name| {
                TextureReference {
                    id: name.clone(),
                    label: name.clone(),
                }
            }),
            preserved_channels: Vec::new(),
        })
        .collect()
}

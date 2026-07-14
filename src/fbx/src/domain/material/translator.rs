// File:
//   - translator.rs
// Path:
//   - src/fbx/src/domain/material/translator.rs
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
//   - Pure fbx domain rules for domain material translator.
// - Must-Not:
//   - Read files, parse generated indexes, invoke CLI code, or call writer
//   - adapters.
// - Allows:
//   - Value objects, invariant checks, and pure evidence-to-domain translation.
// - Split-When:
//   - Split when translator contains two independently testable contracts.
// - Merge-When:
//   - Another fbx module owns the same domain boundary with no distinct
//   - invariant.
// - Summary:
//   - Convert resolved material bindings into domain materials.
// - Description:
//   - Defines translator data and behavior for fbx domain material.
// - Usage:
//   - Imported through crate domain facades or sibling domain modules.
// - Defaults:
//   - No filesystem paths, no external process calls, and no implicit IO
//   - defaults.
//
// ADRs:
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
// - docs/adr/pipeline/unreal/unreal-manifest-and-package-taxonomy.md
//
// Large file:
//   - false
//

//! Convert resolved material bindings into domain materials.
//!
//! This boundary keeps convert resolved material bindings into domain
//! materials explicit and returns deterministic results to fbx callers.
use super::definition::Material;
use crate::domain::texture::{MaterialBinding, TextureReference};

/// Convert resolved material bindings into domain materials.
#[must_use]
pub fn material_bindings_to_materials(
    bindings: &[MaterialBinding]
) -> Vec<Material> {
    bindings
        .iter()
        .map(
            |binding| Material {
                id: binding
                    .material_name
                    .clone(),
                diffuse_texture: binding
                    .texture_file_name
                    .as_ref()
                    .map(
                        |name| TextureReference {
                            id: name.clone(),
                            label: name.clone(),
                        },
                    ),
                preserved_channels: Vec::new(),
            },
        )
        .collect()
}

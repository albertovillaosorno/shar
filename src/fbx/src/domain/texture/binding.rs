// File:
//   - binding.rs
// Path:
//   - src/fbx/src/domain/texture/binding.rs
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
//   - Pure fbx domain rules for domain texture binding.
// - Must-Not:
//   - Read files, parse generated indexes, invoke CLI code, or call writer
//   - adapters.
// - Allows:
//   - Value objects, invariant checks, and pure evidence-to-domain translation.
// - Split-When:
//   - Split when binding contains two independently testable contracts.
// - Merge-When:
//   - Another fbx module owns the same domain boundary with no distinct
//   - invariant.
// - Summary:
//   - Material binding resolved for one mesh primitive group.
// - Description:
//   - Defines binding data and behavior for fbx domain texture.
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

//! Material binding resolved for one mesh primitive group.
// These exact file-local lints preserve explicit domain and binary contracts.
#![expect(
    clippy::module_name_repetitions,
    reason = "Tests verify these intentional explicit file-local contracts \
              remain safe."
)]

use std::path::{Component, Path};

/// Material-binding validation failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MaterialBindingError {
    /// Material identity was empty or whitespace-only.
    MissingMaterialName,
    /// Material identity carried surrounding whitespace.
    NonCanonicalMaterialName,
    /// Texture file identity was empty or whitespace-only.
    BlankTextureFileName,
    /// Texture file identity carried surrounding whitespace.
    NonCanonicalTextureFileName,
    /// Texture identity was not exactly one file-name component.
    InvalidTextureFileName,
}

/// Texture binding associated with a material channel.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MaterialBinding {
    /// Shader/material name referenced by the mesh.
    pub material_name: String,
    /// Texture file name chosen by a driven adapter when available.
    pub texture_file_name: Option<String>,
}

/// Return whether a texture identity is exactly one normal file component.
fn is_single_file_name(file_name: &str) -> bool {
    let mut components = Path::new(file_name).components();
    matches!(
        components.next(),
        Some(Component::Normal(_))
    ) && components
        .next()
        .is_none()
}

impl MaterialBinding {
    /// Create a material binding with an optional texture file.
    ///
    /// # Errors
    ///
    /// Returns an error when a required identity is blank.
    pub fn new(
        material_name: impl Into<String>,
        texture_file_name: Option<String>,
    ) -> Result<Self, MaterialBindingError> {
        let normalized_material_name = material_name.into();
        if normalized_material_name
            .trim()
            .is_empty()
        {
            return Err(MaterialBindingError::MissingMaterialName);
        }
        if normalized_material_name != normalized_material_name.trim()
            || normalized_material_name
                .chars()
                .any(char::is_control)
        {
            return Err(MaterialBindingError::NonCanonicalMaterialName);
        }
        if texture_file_name
            .as_ref()
            .is_some_and(
                |file_name| {
                    file_name
                        .trim()
                        .is_empty()
                },
            )
        {
            return Err(MaterialBindingError::BlankTextureFileName);
        }
        if texture_file_name
            .as_ref()
            .is_some_and(|file_name| file_name != file_name.trim())
        {
            return Err(MaterialBindingError::NonCanonicalTextureFileName);
        }
        if texture_file_name
            .as_ref()
            .is_some_and(|file_name| !is_single_file_name(file_name))
        {
            return Err(MaterialBindingError::InvalidTextureFileName);
        }
        Ok(
            Self {
                material_name: normalized_material_name,
                texture_file_name,
            },
        )
    }
}

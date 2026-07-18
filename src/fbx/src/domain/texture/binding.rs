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

/// Semantic surface flags preserved for downstream FBX material automation.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MaterialSemantics {
    /// Surface requires alpha or transparency handling.
    transparent: bool,
    /// Surface is authored as glass, a window, visor, or optical lens.
    glass: bool,
    /// Surface is authored as a mirror or reflective rear-view element.
    mirror: bool,
    /// Surface represents an emissive, additive, flare, lamp, or light source.
    light_emitter: bool,
}

impl MaterialSemantics {
    /// Build semantics from explicit evidence flags.
    #[must_use]
    pub const fn new(
        transparent: bool,
        glass: bool,
        mirror: bool,
        light_emitter: bool,
    ) -> Self {
        Self {
            transparent: transparent || glass,
            glass,
            mirror,
            light_emitter,
        }
    }

    /// Infer conservative semantics from material and texture identities.
    #[must_use]
    pub fn from_identities(
        material_name: &str,
        texture_file_name: Option<&str>,
    ) -> Self {
        let mut evidence = material_name.to_ascii_lowercase();
        if let Some(texture) = texture_file_name {
            evidence.push(' ');
            evidence.push_str(&texture.to_ascii_lowercase());
        }
        let glass = contains_any(
            &evidence,
            &[
                "glass",
                "window",
                "windshield",
                "windsheild",
                "windscreen",
                "visor",
                "goggle",
                "spectacle",
                "eyeglass",
                "lens",
            ],
        );
        let transparent = glass
            || contains_any(
                &evidence,
                &[
                    "transparent",
                    "translucent",
                ],
            );
        let mirror = contains_any(
            &evidence,
            &[
                "mirror",
                "rearview",
                "rear-view",
                "sideview",
                "side-view",
            ],
        );
        let light_emitter = contains_any(
            &evidence,
            &[
                "headlight",
                "head-light",
                "taillight",
                "tail-light",
                "brakelight",
                "brake-light",
                "reverse-light",
                "reverselight",
                "siren",
                "lamp",
                "bulb",
                "flare",
                "glow",
                "emiss",
                "neon",
                "backfire",
                "fireseq",
                "fire-seq",
                "frinkarc",
                "frink-arc",
            ],
        );
        Self::new(
            transparent,
            glass,
            mirror,
            light_emitter,
        )
    }

    /// Merge additional decoded shader or runtime evidence.
    #[must_use]
    pub const fn merge(
        self,
        additional: Self,
    ) -> Self {
        Self::new(
            self.transparent || additional.transparent,
            self.glass || additional.glass,
            self.mirror || additional.mirror,
            self.light_emitter || additional.light_emitter,
        )
    }

    /// Return whether standard FBX transparency properties are required.
    #[must_use]
    pub const fn is_transparent(self) -> bool {
        self.transparent
    }

    /// Return whether the surface is specifically glass or an optical lens.
    #[must_use]
    pub const fn is_glass(self) -> bool {
        self.glass
    }

    /// Return whether the surface is a mirror or reflective view element.
    #[must_use]
    pub const fn is_mirror(self) -> bool {
        self.mirror
    }

    /// Return whether standard FBX emissive properties are required.
    #[must_use]
    pub const fn is_light_emitter(self) -> bool {
        self.light_emitter
    }

    /// Build a deterministic object-name suffix for semantic automation.
    #[must_use]
    pub fn suffix(self) -> Option<String> {
        let mut labels = Vec::new();
        if self.glass {
            labels.push("glass");
        } else if self.transparent {
            labels.push("transparent");
        }
        if self.mirror {
            labels.push("mirror");
        }
        if self.light_emitter {
            labels.push("light-emitter");
        }
        (!labels.is_empty()).then(|| labels.join("-"))
    }
}

/// Return whether identity evidence contains any conservative semantic token.
fn contains_any(
    evidence: &str,
    needles: &[&str],
) -> bool {
    needles
        .iter()
        .any(|needle| evidence.contains(needle))
}

/// Texture binding associated with a material channel.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MaterialBinding {
    /// Shader/material name referenced by the mesh.
    pub material_name: String,
    /// Texture file name chosen by a driven adapter when available.
    pub texture_file_name: Option<String>,
    /// Shared transparency, glass, mirror, and emitter semantics.
    pub semantics: MaterialSemantics,
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
        let semantics = MaterialSemantics::from_identities(
            &normalized_material_name,
            texture_file_name.as_deref(),
        );
        Ok(
            Self {
                material_name: normalized_material_name,
                texture_file_name,
                semantics,
            },
        )
    }

    /// Merge decoded shader or runtime semantics into this binding.
    #[must_use]
    pub const fn with_semantics(
        mut self,
        semantics: MaterialSemantics,
    ) -> Self {
        self.semantics = self
            .semantics
            .merge(semantics);
        self
    }
}

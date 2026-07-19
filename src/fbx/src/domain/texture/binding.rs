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
    /// Compact independent semantic flags.
    flags: u8,
}

impl MaterialSemantics {
    /// Surface requires alpha or transparency handling.
    const TRANSPARENT: u8 = 1 << 0;
    /// Surface is authored as glass or another optical lens.
    const GLASS: u8 = 1 << 1;
    /// Surface is authored as a mirror or rear-view element.
    const MIRROR: u8 = 1 << 2;
    /// Surface has source-backed reflection or metallic evidence.
    const REFLECTIVE: u8 = 1 << 3;
    /// Surface represents a lamp, flare, or other light emitter.
    const LIGHT_EMITTER: u8 = 1 << 4;
    /// Surface is a non-luminous smoke, fire, or particle plane.
    const VISUAL_EFFECT: u8 = 1 << 5;

    /// Infer conservative semantics from material and texture identities.
    #[must_use]
    pub fn from_identities(
        material_name: &str,
        texture_file_name: Option<&str>,
    ) -> Self {
        infer_identity_semantics(
            &identity_evidence(
                material_name,
                texture_file_name,
            ),
        )
    }

    /// Merge additional decoded shader or runtime evidence.
    #[must_use]
    pub const fn merge(
        self,
        additional: Self,
    ) -> Self {
        let mut merged = Self {
            flags: self.flags | additional.flags,
        };
        if merged.is_visual_effect() {
            merged.flags &= !Self::LIGHT_EMITTER;
        }
        merged
    }

    /// Merge explicit transparency evidence.
    #[must_use]
    pub const fn with_transparent(
        self,
        transparent: bool,
    ) -> Self {
        self.with_flag(
            Self::TRANSPARENT,
            transparent,
        )
    }

    /// Merge explicit glass evidence and its required transparency.
    #[must_use]
    pub const fn with_glass(
        self,
        glass: bool,
    ) -> Self {
        self.with_flag(
            Self::GLASS | Self::TRANSPARENT,
            glass,
        )
    }

    /// Merge explicit mirror evidence and its required reflection.
    #[must_use]
    pub const fn with_mirror(
        self,
        mirror: bool,
    ) -> Self {
        self.with_flag(
            Self::MIRROR | Self::REFLECTIVE,
            mirror,
        )
    }

    /// Merge source-backed metallic or reflective presentation evidence.
    #[must_use]
    pub const fn with_reflective(
        self,
        reflective: bool,
    ) -> Self {
        self.with_flag(
            Self::REFLECTIVE,
            reflective,
        )
    }

    /// Merge explicit light-emitter evidence unless the surface is VFX.
    #[must_use]
    pub const fn with_light_emitter(
        self,
        light_emitter: bool,
    ) -> Self {
        if self.is_visual_effect() {
            return self;
        }
        self.with_flag(
            Self::LIGHT_EMITTER,
            light_emitter,
        )
    }

    /// Mark one non-luminous visual effect and suppress light emission.
    #[must_use]
    pub const fn with_visual_effect(
        mut self,
        visual_effect: bool,
    ) -> Self {
        self = self.with_flag(
            Self::VISUAL_EFFECT,
            visual_effect,
        );
        if visual_effect {
            self.flags &= !Self::LIGHT_EMITTER;
        }
        self
    }

    /// Return whether standard FBX transparency properties are required.
    #[must_use]
    pub const fn is_transparent(self) -> bool {
        self.has(Self::TRANSPARENT)
    }

    /// Return whether the surface is specifically glass or an optical lens.
    #[must_use]
    pub const fn is_glass(self) -> bool {
        self.has(Self::GLASS)
    }

    /// Return whether the surface is a mirror or reflective view element.
    #[must_use]
    pub const fn is_mirror(self) -> bool {
        self.has(Self::MIRROR)
    }

    /// Return whether source-backed metallic/reflection properties are
    /// required.
    #[must_use]
    pub const fn is_reflective(self) -> bool {
        self.has(Self::REFLECTIVE)
    }

    /// Return whether standard FBX emissive properties are required.
    #[must_use]
    pub const fn is_light_emitter(self) -> bool {
        self.has(Self::LIGHT_EMITTER)
    }

    /// Return whether the surface is a non-luminous visual-effect plane.
    #[must_use]
    pub const fn is_visual_effect(self) -> bool {
        self.has(Self::VISUAL_EFFECT)
    }

    /// Build a deterministic object-name suffix for semantic automation.
    #[must_use]
    pub fn suffix(self) -> Option<String> {
        let mut labels = Vec::new();
        if self.is_glass() {
            labels.push("glass");
        } else if self.is_transparent() {
            labels.push("transparent");
        }
        if self.is_mirror() {
            labels.push("mirror");
        }
        if self.is_reflective() && !self.is_mirror() {
            labels.push("reflective");
        }
        if self.is_light_emitter() {
            labels.push("light-emitter");
        }
        if self.is_visual_effect() {
            labels.push("vfx");
        }
        (!labels.is_empty()).then(|| labels.join("-"))
    }

    /// Return whether one semantic flag is present.
    const fn has(
        self,
        flag: u8,
    ) -> bool {
        self.flags & flag != 0
    }

    /// Merge one conditional semantic flag.
    const fn with_flag(
        mut self,
        flag: u8,
        enabled: bool,
    ) -> Self {
        if enabled {
            self.flags |= flag;
        }
        self
    }
}

/// Build normalized combined material and texture identity evidence.
fn identity_evidence(
    material_name: &str,
    texture_file_name: Option<&str>,
) -> String {
    let mut evidence = material_name.to_ascii_lowercase();
    if let Some(texture) = texture_file_name {
        evidence.push(' ');
        evidence.push_str(&texture.to_ascii_lowercase());
    }
    evidence
}

/// Infer all semantic families from normalized identity evidence.
fn infer_identity_semantics(evidence: &str) -> MaterialSemantics {
    let glass = glass_evidence(evidence);
    let mirror = mirror_evidence(evidence);
    let visual_effect = visual_effect_evidence(evidence);
    MaterialSemantics::default()
        .with_transparent(
            glass
                || contains_any(
                    evidence,
                    &[
                        "transparent",
                        "translucent",
                    ],
                ),
        )
        .with_glass(glass)
        .with_mirror(mirror)
        .with_reflective(mirror || reflective_evidence(evidence))
        .with_light_emitter(!visual_effect && light_emitter_evidence(evidence))
        .with_visual_effect(visual_effect)
}

/// Return whether identity evidence names glass or an optical lens.
fn glass_evidence(evidence: &str) -> bool {
    contains_any(
        evidence,
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
    )
}

/// Return whether identity evidence names a mirror surface.
fn mirror_evidence(evidence: &str) -> bool {
    contains_any(
        evidence,
        &[
            "mirror",
            "rearview",
            "rear-view",
            "sideview",
            "side-view",
        ],
    )
}

/// Return whether identity evidence names a reflective material family.
fn reflective_evidence(evidence: &str) -> bool {
    contains_any(
        evidence,
        &[
            "chrome",
            "metal",
            "metallic",
            "steel",
            "spheremap",
            "sphere-map",
            "reflective",
            "reflection",
            "envmap",
            "env-map",
        ],
    )
}

/// Return whether identity evidence names a non-luminous visual effect.
fn visual_effect_evidence(evidence: &str) -> bool {
    contains_token_start(
        evidence,
        &[
            "smoke", "fire", "flame", "exhaust", "steam", "dust", "mist",
            "fog", "vapor", "vapour", "particle", "spark", "backfire",
        ],
    )
}

/// Return whether identity evidence names an emissive or luminous surface.
fn light_emitter_evidence(evidence: &str) -> bool {
    if contains_any(
        evidence,
        &[
            "dontlight",
            "do-not-light",
            "relight",
            "lighthouse",
        ],
    ) {
        return false;
    }
    contains_any(
        evidence,
        &[
            "headlight",
            "head-light",
            "taillight",
            "tail-light",
            "brakelight",
            "brake-light",
            "reverse-light",
            "reverselight",
            "lights",
            "lightbar",
            "light-bar",
            "parkinglight",
            "parking-light",
            "globelight",
            "globe-light",
            "streetlight",
            "street-light",
            "trafficlight",
            "traffic-light",
            "lightshape",
            "light-shape",
            "light_shape",
            "siren",
            "lamp",
            "bulb",
            "indicator",
            "turnsignal",
            "turn-signal",
            "beacon",
            "strobe",
            "flare",
            "glow",
            "emiss",
            "illum",
            "neon",
            "frinkarc",
            "frink-arc",
        ],
    )
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

/// Return whether a token starts at one semantic identity boundary.
fn contains_token_start(
    evidence: &str,
    needles: &[&str],
) -> bool {
    needles
        .iter()
        .any(
            |needle| {
                evidence
                    .match_indices(needle)
                    .any(
                        |(index, _matched)| {
                            index == 0
                                || evidence
                                    .get(..index)
                                    .and_then(
                                        |prefix| {
                                            prefix
                                                .chars()
                                                .next_back()
                                        },
                                    )
                                    .is_some_and(
                                        |character| {
                                            !character.is_ascii_alphanumeric()
                                        },
                                    )
                        },
                    )
            },
        )
}

/// Texture binding associated with a material channel.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MaterialBinding {
    /// Shader/material name referenced by the mesh.
    pub material_name: String,
    /// Texture file name chosen by a driven adapter when available.
    pub texture_file_name: Option<String>,
    /// Shared transparency, glass, reflection, emitter, and VFX semantics.
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

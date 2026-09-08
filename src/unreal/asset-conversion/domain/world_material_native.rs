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
//   - Compatibility selection for the reviewed native simple-unlit world
//     master-material builder.
// - Must-Not:
//   - Claim Texture2D, Material Instance, slot-assignment, save, or full
//     presentation readiness.
// - Allows:
//   - Verified world raster families, effective surface semantics, and the
//     source-backed regular-scene no-cull contract.
// - Split-When:
//   - Lit, environment, runtime-error, or special-surface builders gain an
//     independently reviewed native construction lifecycle.
// - Merge-When:
//   - Another domain module owns identical native world-master compatibility.
// - Summary:
//   - Reviewed native world-master compatibility classifier.
// - Description:
//   - Maps only representable simple/unlit world presentation state to the
//   - bounded native master-builder recipe and reports every unsupported
//   - construction concern without approximating it.
// - Usage:
//   - Consumed after world material evidence verification and before native
//   - construction operations are planned.
// - Defaults:
//   - Unsupported shader, lighting, and special-surface semantics stay blocked.
//

//! Reviewed native world-master compatibility classification.

use super::{
    WorldMaterialBlendFamily, WorldMaterialMasterFamily,
    WorldMaterialSemantics, WorldMaterialShaderFamily,
};

/// Blend token accepted by the reviewed native simple/unlit master builder.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorldMaterialNativeBlend {
    /// Native opaque blend family.
    Opaque,
    /// Native source-alpha translucent blend family.
    SourceAlpha,
    /// Native additive blend family.
    Additive,
}

impl WorldMaterialNativeBlend {
    /// Return the exact token accepted by `CreateSimpleUnlitWorldMaster`.
    #[must_use]
    pub const fn tool_token(self) -> &'static str {
        match self {
            Self::Opaque => "opaque",
            Self::SourceAlpha => "alpha",
            Self::Additive => "additive",
        }
    }
}

/// Native master-builder inputs proven representable by current policy.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorldMaterialNativeMasterRecipe {
    /// Reviewed native blend family.
    pub blend: WorldMaterialNativeBlend,
    /// Whether the master graph must reproduce source alpha comparison.
    pub alpha_test: bool,
    /// Regular world presentation always reproduces source `PddiCullNone`.
    pub render_both_faces: bool,
}

impl WorldMaterialNativeMasterRecipe {
    /// Return the canonical recipe identity used across planning boundaries.
    #[must_use]
    pub fn identity(self) -> String {
        let alpha_test = if self.alpha_test {
            "on"
        } else {
            "off"
        };
        let sidedness = if self.render_both_faces {
            "both-faces"
        } else {
            "one-sided"
        };
        format!(
            "simple-unlit__blend-{}__alpha-test-{alpha_test}__{sidedness}",
            self.blend.tool_token()
        )
    }
}

/// Validated destination and recipe inputs for one native world master.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorldMaterialNativeMasterRequest {
    recipe: WorldMaterialNativeMasterRecipe,
    folder_path: String,
    asset_name: String,
    package_path: String,
    object_path: String,
}

impl WorldMaterialNativeMasterRequest {
    /// Bind one reviewed recipe to a caller-selected generated destination.
    ///
    /// Destination selection remains planning policy outside this domain value.
    /// This constructor only proves that the selected destination can be passed
    /// to the reviewed native world-master constructor without weakening the
    /// project naming and regular-world raster contracts.
    ///
    /// # Errors
    ///
    /// Returns an error when the recipe does not reproduce regular-world
    /// both-face rendering or the destination is not a canonical generated
    /// Material asset.
    pub fn new(
        recipe: WorldMaterialNativeMasterRecipe,
        folder_path: &str,
        asset_name: &str,
    ) -> Result<Self, String> {
        if !recipe.render_both_faces {
            return Err(
                "world master request must reproduce regular-world CullNone"
                    .to_owned(),
            );
        }
        if !is_generated_material_folder(folder_path) {
            return Err(
                "world master folder is not a canonical generated material path"
                    .to_owned(),
            );
        }
        if !is_material_asset_name(asset_name) {
            return Err("world master asset name is not canonical".to_owned());
        }
        let package_path = format!("{folder_path}/{asset_name}");
        let object_path = format!("{package_path}.{asset_name}");
        if object_path.len() > 240 {
            return Err("world master object path is too long".to_owned());
        }
        Ok(Self {
            recipe,
            folder_path: folder_path.to_owned(),
            asset_name: asset_name.to_owned(),
            package_path,
            object_path,
        })
    }

    /// Return the reviewed native master recipe.
    #[must_use]
    pub const fn recipe(&self) -> WorldMaterialNativeMasterRecipe {
        self.recipe
    }

    /// Return the exact folder argument accepted by the native constructor.
    #[must_use]
    pub fn folder_path(&self) -> &str {
        &self.folder_path
    }

    /// Return the exact asset-name argument accepted by the native constructor.
    #[must_use]
    pub fn asset_name(&self) -> &str {
        &self.asset_name
    }

    /// Return the generated package path owned by this request.
    #[must_use]
    pub fn package_path(&self) -> &str {
        &self.package_path
    }

    /// Return the canonical object path expected from native construction.
    #[must_use]
    pub fn object_path(&self) -> &str {
        &self.object_path
    }
}

/// One reason current native master construction cannot represent a
/// presentation faithfully.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorldMaterialNativeMasterBlocker {
    /// Environment and runtime-error shaders need dedicated native policy.
    UnsupportedShaderFamily,
    /// Fixed-function lit source presentation needs reviewed native lighting.
    Lit,
    /// Glass needs reviewed optical presentation policy.
    Glass,
    /// Mirrors need dedicated reflection/view policy.
    Mirror,
    /// Non-mirror reflection needs reviewed native reflection policy.
    Reflective,
    /// Luminous surfaces need source-backed emission policy.
    LightEmitter,
    /// Non-luminous VFX surfaces need dedicated native VFX policy.
    VisualEffect,
}

impl WorldMaterialNativeMasterBlocker {
    /// Return a stable public-safe blocker code for plan diagnostics.
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::UnsupportedShaderFamily => "unsupported-shader-family",
            Self::Lit => "lit-presentation",
            Self::Glass => "glass-presentation",
            Self::Mirror => "mirror-presentation",
            Self::Reflective => "reflective-presentation",
            Self::LightEmitter => "light-emitter-presentation",
            Self::VisualEffect => "visual-effect-presentation",
        }
    }
}

/// Exact compatibility result for the currently reviewed native master builder.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorldMaterialNativeMasterClassification {
    recipe: Option<WorldMaterialNativeMasterRecipe>,
    blockers: Vec<WorldMaterialNativeMasterBlocker>,
}

impl WorldMaterialNativeMasterClassification {
    /// Return the native recipe only when no unsupported state remains.
    #[must_use]
    pub const fn recipe(&self) -> Option<WorldMaterialNativeMasterRecipe> {
        self.recipe
    }

    /// Return every blocker in stable construction-policy order.
    #[must_use]
    pub fn blockers(&self) -> &[WorldMaterialNativeMasterBlocker] {
        &self.blockers
    }
}

/// Classify one verified world presentation against the current native
/// simple/unlit master builder.
///
/// `two_sided` remains part of source presentation identity upstream, but it
/// does not split this native recipe. Ordinary world rendering enters `Pure3D`
/// with `PddiCullNone`, so both source `2SID` values resolve to the same native
/// two-sided raster requirement.
#[must_use]
pub fn classify_world_material_native_master(
    master: WorldMaterialMasterFamily,
    semantics: WorldMaterialSemantics,
) -> WorldMaterialNativeMasterClassification {
    let mut blockers = Vec::new();
    if master.shader != WorldMaterialShaderFamily::Simple {
        blockers
            .push(WorldMaterialNativeMasterBlocker::UnsupportedShaderFamily);
    }
    if master.lit {
        blockers.push(WorldMaterialNativeMasterBlocker::Lit);
    }
    if semantics.glass {
        blockers.push(WorldMaterialNativeMasterBlocker::Glass);
    }
    if semantics.mirror {
        blockers.push(WorldMaterialNativeMasterBlocker::Mirror);
    } else if semantics.reflective {
        blockers.push(WorldMaterialNativeMasterBlocker::Reflective);
    }
    if semantics.light_emitter {
        blockers.push(WorldMaterialNativeMasterBlocker::LightEmitter);
    }
    if semantics.visual_effect {
        blockers.push(WorldMaterialNativeMasterBlocker::VisualEffect);
    }

    let recipe = blockers
        .is_empty()
        .then(|| WorldMaterialNativeMasterRecipe {
            blend: native_blend(master.blend),
            alpha_test: master.alpha_compare.is_some(),
            render_both_faces: true,
        });
    WorldMaterialNativeMasterClassification { recipe, blockers }
}

fn is_generated_material_folder(value: &str) -> bool {
    const ROOT: &str = "/Game/Generated/SHAR/Materials";
    const PREFIX: &str = "/Game/Generated/SHAR/Materials/";
    if value != ROOT && !value.starts_with(PREFIX) {
        return false;
    }
    let Some(relative) = value.strip_prefix('/') else {
        return false;
    };
    !relative.is_empty() && relative.split('/').all(is_unreal_name)
}

fn is_material_asset_name(value: &str) -> bool {
    value
        .strip_prefix("M_")
        .is_some_and(|suffix| !suffix.is_empty())
        && is_unreal_name(value)
}

fn is_unreal_name(value: &str) -> bool {
    !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
}

const fn native_blend(
    blend: WorldMaterialBlendFamily,
) -> WorldMaterialNativeBlend {
    match blend {
        WorldMaterialBlendFamily::Disabled => WorldMaterialNativeBlend::Opaque,
        WorldMaterialBlendFamily::SourceAlpha => {
            WorldMaterialNativeBlend::SourceAlpha
        },
        WorldMaterialBlendFamily::Additive => {
            WorldMaterialNativeBlend::Additive
        },
    }
}

#[cfg(test)]
// jig-ignore-next-line: exact test-module path syntax is indivisible
#[path = "../../../../tests/unreal/asset-conversion/unit/domain/world_material_native/tests.rs"]
mod tests;

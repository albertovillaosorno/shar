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

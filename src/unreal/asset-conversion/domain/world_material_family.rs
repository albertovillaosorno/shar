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
//   - Source-backed world material master-family and raster-instance selection.
// - Must-Not:
//   - Parse catalogs, create Unreal assets, or infer shader state from names.
// - Allows:
//   - Reviewed PDDI raster state already decoded from world shader evidence.
// - Split-When:
//   - Additional reviewed source shader families gain independent policy.
// - Merge-When:
//   - Another domain module owns identical world raster-family selection.
// - Summary:
//   - World material family selection contract.
// - Description:
//   - Separates compile-time master-family state from per-instance alpha data
//   - while reproducing reviewed PDDI defaults and active-state semantics.
// - Usage:
//   - Built from one verified v8 source shader before native material planning.
// - Defaults:
//   - Unreviewed active blend or alpha-compare modes fail closed.
//

//! Source-backed world material master-family and raster-instance selection.

/// PDDI's default alpha-reference value (`0.5f`) as exact IEEE-754 bits.
pub const PDDI_DEFAULT_ALPHA_REFERENCE_BITS: u32 = 0x3f00_0000;

/// Reviewed source shader family retained for native material construction.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorldMaterialShaderFamily {
    /// Ordinary `Pure3D` simple shader presentation.
    Simple,
    /// Source environment shader presentation requiring reflection policy.
    Environment,
    /// Shipped missing-shader fallback created as `TShader("error")`.
    RuntimeError,
}

impl WorldMaterialShaderFamily {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Simple => "simple",
            Self::Environment => "environment",
            Self::RuntimeError => "runtime-error",
        }
    }
}

/// Reviewed effective PDDI blend family.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorldMaterialBlendFamily {
    /// `PddiBlendNone`: blending disabled.
    Disabled,
    /// `PddiBlendAlpha`: source-alpha over destination.
    SourceAlpha,
    /// `PddiBlendAdd`: additive source plus destination.
    Additive,
}

impl WorldMaterialBlendFamily {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Disabled => "blend-none",
            Self::SourceAlpha => "blend-alpha",
            Self::Additive => "blend-additive",
        }
    }
}

/// Reviewed active PDDI alpha comparison used by the accepted world corpus.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorldMaterialAlphaCompare {
    /// `PddiCompareGreater`, source alpha-compare value `4`.
    Greater,
}

impl WorldMaterialAlphaCompare {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Greater => "alpha-test-greater",
        }
    }
}

/// Compile-time material-family state selected from active source raster state.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorldMaterialMasterFamily {
    /// Reviewed source shader family.
    pub shader: WorldMaterialShaderFamily,
    /// Reviewed source blend behavior.
    pub blend: WorldMaterialBlendFamily,
    /// Active alpha comparison, or `None` when alpha test is disabled.
    pub alpha_compare: Option<WorldMaterialAlphaCompare>,
    /// Whether source presentation disables back-face culling.
    pub two_sided: bool,
    /// Whether source presentation enables fixed-function lighting.
    pub lit: bool,
}

impl WorldMaterialMasterFamily {
    /// Return a stable public-safe family identity for deterministic planning.
    #[must_use]
    pub fn identity(self) -> String {
        let alpha = self
            .alpha_compare
            .map_or("alpha-test-off", WorldMaterialAlphaCompare::as_str);
        let sided = if self.two_sided {
            "two-sided"
        } else {
            "one-sided"
        };
        let lighting = if self.lit {
            "lit"
        } else {
            "unlit"
        };
        format!(
            "{}__{}__{}__{}__{}",
            self.shader.as_str(),
            self.blend.as_str(),
            alpha,
            sided,
            lighting,
        )
    }
}

/// Per-instance raster values that do not select a compiled master family.
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorldMaterialInstanceRaster {
    /// Effective PDDI alpha-reference bits when alpha test is active.
    pub alpha_reference_bits: Option<u32>,
}

/// Native-planning projection of one verified PDDI world shader raster state.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorldMaterialRasterProjection {
    /// Compile-time master family.
    pub master: WorldMaterialMasterFamily,
    /// Per-instance raster parameters.
    pub instance: WorldMaterialInstanceRaster,
}

impl WorldMaterialRasterProjection {
    /// Project one decoded PDDI shader state into reviewed native families.
    ///
    /// Inputs use the exact integer values stored by world catalog v8 for
    /// blend, alpha-test, alpha-compare, two-sided, and lighting state.
    /// The optional alpha reference is
    /// supplied as exact `f32::to_bits()` output.
    ///
    /// # Errors
    ///
    /// Returns an error for non-boolean flags, unreviewed active blend modes,
    /// or an active alpha comparison without a reviewed native representation.
    pub fn from_pddi(
        shader: WorldMaterialShaderFamily,
        blend_mode: u8,
        alpha_test: u8,
        alpha_compare: u8,
        alpha_reference_bits: Option<u32>,
        two_sided: u8,
        lit: u8,
    ) -> Result<Self, String> {
        if shader == WorldMaterialShaderFamily::RuntimeError {
            return Err(
                "runtime error material must use its dedicated constructor"
                    .to_owned(),
            );
        }
        let blend = match blend_mode {
            0 => WorldMaterialBlendFamily::Disabled,
            1 => WorldMaterialBlendFamily::SourceAlpha,
            2 => WorldMaterialBlendFamily::Additive,
            _ => {
                return Err(
                    "world material uses unreviewed active PDDI blend mode"
                        .to_owned(),
                );
            },
        };
        let alpha_test = binary_flag(alpha_test, "PDDI alpha-test flag")?;
        let two_sided = binary_flag(two_sided, "PDDI two-sided flag")?;
        let lit = binary_flag(lit, "PDDI lighting flag")?;
        let (alpha_compare, alpha_reference_bits) = if alpha_test {
            if alpha_compare != 4 {
                return Err(
                    "world material uses unreviewed active PDDI alpha compare"
                        .to_owned(),
                );
            }
            let reference = alpha_reference_bits
                .unwrap_or(PDDI_DEFAULT_ALPHA_REFERENCE_BITS);
            (
                Some(WorldMaterialAlphaCompare::Greater),
                Some(normalize_alpha_reference(reference)),
            )
        } else {
            (None, None)
        };
        Ok(Self {
            master: WorldMaterialMasterFamily {
                shader,
                blend,
                alpha_compare,
                two_sided,
                lit,
            },
            instance: WorldMaterialInstanceRaster { alpha_reference_bits },
        })
    }

    /// Reproduce the shipped primitive-group missing-shader fallback family.
    #[must_use]
    pub const fn runtime_error() -> Self {
        Self {
            master: WorldMaterialMasterFamily {
                shader: WorldMaterialShaderFamily::RuntimeError,
                blend: WorldMaterialBlendFamily::Disabled,
                alpha_compare: None,
                two_sided: false,
                lit: false,
            },
            instance: WorldMaterialInstanceRaster {
                alpha_reference_bits: None,
            },
        }
    }
}

fn binary_flag(value: u8, label: &str) -> Result<bool, String> {
    match value {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(format!("{label} is not binary")),
    }
}

fn normalize_alpha_reference(bits: u32) -> u32 {
    let reference = f32::from_bits(bits);
    let normalized = if reference > 0. {
        if reference < 1. {
            reference
        } else {
            1.
        }
    } else {
        0.
    };
    normalized.to_bits()
}

#[cfg(test)]
// jig-ignore-next-line: exact test-module path syntax is indivisible
#[path = "../../../../tests/unreal/asset-conversion/unit/domain/world_material_family/tests.rs"]
mod tests;

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
//   - Exact native Material Instance request values for reviewed simple/unlit
//     world presentation.
// - Must-Not:
//   - Select destination identities, load Unreal assets, contact Unreal Editor,
//     assign mesh slots, save packages, or reinterpret source shader state.
// - Allows:
//   - Bind verified world presentation to a reviewed master recipe, generated
//     parent/texture object paths, neutral unlit tint, and effective alpha
//     state.
// - Split-When:
//   - Another native instance family gains independent parameters or lifecycle.
// - Merge-When:
//   - Another domain module owns the identical simple/unlit instance request.
// - Summary:
//   - Reviewed native simple-unlit Material Instance request.
// - Description:
//   - Produces only the arguments accepted by the reviewed native world
//   - Material Instance constructor after upstream presentation verification.
// - Usage:
//   - Built after a caller selects canonical generated parent, texture, and
//   - destination identities for one verified presentation.
// - Defaults:
//   - Simple/unlit DIFF never replaces authored vertex colour; native tint is
//   - neutral white and active alpha state is copied from reviewed raster
//   - state.
//

//! Reviewed native simple-unlit world Material Instance request.

use super::world_material_native::{
    is_generated_material_folder, is_unreal_name,
};
use super::{
    WorldMaterialNativeMasterRecipe, WorldMaterialPresentation,
    classify_world_material_native_master,
};

const NEUTRAL_TINT_BITS: [u32; 4] = [
    1f32.to_bits(),
    1f32.to_bits(),
    1f32.to_bits(),
    1f32.to_bits(),
];

/// Exact validated inputs for one native simple/unlit world Material Instance.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorldMaterialNativeInstanceRequest {
    folder_path: String,
    asset_name: String,
    package_path: String,
    object_path: String,
    parent_material_path: String,
    base_color_texture_path: Option<String>,
    base_color_tint_bits: [u32; 4],
    alpha_reference_bits: Option<u32>,
}

impl WorldMaterialNativeInstanceRequest {
    /// Bind one verified presentation to caller-selected generated objects.
    ///
    /// The caller owns deterministic destination selection and the mapping from
    /// verified texture digest to generated `Texture2D` object path. This value
    /// verifies only that those selected identities and native arguments match
    /// the reviewed simple/unlit constructor contract.
    ///
    /// # Errors
    ///
    /// Returns an error when the presentation is not representable by the
    /// supplied master recipe, source texture presence does not match the
    /// selected `Texture2D` path, or any generated object identity is
    /// malformed.
    pub fn new(
        presentation: &WorldMaterialPresentation,
        recipe: WorldMaterialNativeMasterRecipe,
        folder_path: &str,
        asset_name: &str,
        parent_material_path: &str,
        base_color_texture_path: Option<&str>,
    ) -> Result<Self, String> {
        let classification = classify_world_material_native_master(
            presentation.raster.master,
            presentation.semantics,
        );
        if classification.recipe() != Some(recipe) {
            return Err(
                "world material instance recipe does not match presentation"
                    .to_owned(),
            );
        }
        if !is_generated_material_folder(folder_path) {
            return Err(
                "world material instance folder is not canonical".to_owned()
            );
        }
        if !is_material_instance_asset_name(asset_name) {
            return Err("world material instance asset name is not canonical"
                .to_owned());
        }
        if !is_generated_object_path(parent_material_path, true) {
            return Err("world material instance parent path is not canonical"
                .to_owned());
        }
        let source_is_textured = presentation.texture_sha256.is_some();
        if source_is_textured != base_color_texture_path.is_some() {
            return Err(
                "world material instance texture presence differs from source"
                    .to_owned(),
            );
        }
        if let Some(texture_path) = base_color_texture_path
            && !is_generated_object_path(texture_path, false)
        {
            return Err(
                "world material instance texture path is not canonical"
                    .to_owned(),
            );
        }
        let alpha_reference_bits =
            presentation.raster.instance.alpha_reference_bits;
        if recipe.alpha_test != alpha_reference_bits.is_some() {
            return Err(
                "world material instance alpha state differs from master recipe"
                    .to_owned(),
            );
        }
        if alpha_reference_bits.is_some_and(|bits| {
            let value = f32::from_bits(bits);
            !value.is_finite() || !(0. ..=1.).contains(&value)
        }) {
            return Err(
                "world material instance alpha reference is not normalized"
                    .to_owned(),
            );
        }
        let package_path = format!("{folder_path}/{asset_name}");
        let object_path = format!("{package_path}.{asset_name}");
        if object_path.len() > 240 {
            return Err(
                "world material instance object path is too long".to_owned()
            );
        }
        Ok(Self {
            folder_path: folder_path.to_owned(),
            asset_name: asset_name.to_owned(),
            package_path,
            object_path,
            parent_material_path: parent_material_path.to_owned(),
            base_color_texture_path: base_color_texture_path.map(str::to_owned),
            base_color_tint_bits: NEUTRAL_TINT_BITS,
            alpha_reference_bits,
        })
    }

    /// Return the exact destination folder argument for native construction.
    #[must_use]
    pub fn folder_path(&self) -> &str {
        &self.folder_path
    }

    /// Return the exact destination asset name for native construction.
    #[must_use]
    pub fn asset_name(&self) -> &str {
        &self.asset_name
    }

    /// Return the generated destination package path.
    #[must_use]
    pub fn package_path(&self) -> &str {
        &self.package_path
    }

    /// Return the generated destination object path expected after
    /// construction.
    #[must_use]
    pub fn object_path(&self) -> &str {
        &self.object_path
    }

    /// Return the exact generated parent Material object path.
    #[must_use]
    pub fn parent_material_path(&self) -> &str {
        &self.parent_material_path
    }

    /// Return the optional exact generated `Texture2D` object path.
    #[must_use]
    pub fn base_color_texture_path(&self) -> Option<&str> {
        self.base_color_texture_path.as_deref()
    }

    /// Return neutral linear tint values accepted by the native toolset.
    #[must_use]
    pub fn base_color_tint(&self) -> [f32; 4] {
        self.base_color_tint_bits.map(f32::from_bits)
    }

    /// Return whether the parent family owns an alpha-reference parameter.
    #[must_use]
    pub const fn set_alpha_reference(&self) -> bool {
        self.alpha_reference_bits.is_some()
    }

    /// Return the effective normalized source alpha reference when active.
    #[must_use]
    pub fn alpha_reference(&self) -> Option<f32> {
        self.alpha_reference_bits.map(f32::from_bits)
    }
}

fn is_material_instance_asset_name(value: &str) -> bool {
    value
        .strip_prefix("MI_")
        .is_some_and(|suffix| !suffix.is_empty())
        && is_unreal_name(value)
}

fn is_generated_object_path(value: &str, require_material: bool) -> bool {
    const ROOT_PREFIX: &str = "/Game/Generated/SHAR/";
    const MATERIAL_PREFIX: &str = "/Game/Generated/SHAR/Materials/";
    let required_prefix = if require_material {
        MATERIAL_PREFIX
    } else {
        ROOT_PREFIX
    };
    if !value.starts_with(required_prefix) || value.len() > 240 {
        return false;
    }
    let Some((package_path, object_name)) = value.rsplit_once('.') else {
        return false;
    };
    let Some(package_name) = package_path.rsplit('/').next() else {
        return false;
    };
    package_name == object_name
        && !package_name.is_empty()
        && package_path
            .strip_prefix('/')
            .is_some_and(|relative| relative.split('/').all(is_unreal_name))
        && is_unreal_name(object_name)
        && (!require_material || object_name.starts_with("M_"))
}

#[cfg(test)]
// jig-ignore-next-line: exact test-module path syntax is indivisible
#[path = "../../../../tests/unreal/asset-conversion/unit/domain/world_material_instance_native/tests.rs"]
mod tests;

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
//   - Native simple/unlit world Material Instance request tests.
// - Must-Not:
//   - Contact Unreal Editor, select production object names, or claim saves.
// - Allows:
//   - Synthetic verified presentations and generated object identities.
// - Split-When:
//   - Another native instance family gains an independent test lifecycle.
// - Merge-When:
//   - Another test module owns identical Material Instance request assertions.
// - Summary:
//   - Native world Material Instance request tests.
// - Description:
//   - Proves neutral simple/unlit tint, exact alpha state, texture presence,
//   - canonical generated object guards before native construction.
// - Usage:
//   - Included by the native world Material Instance request domain module.
// - Defaults:
//   - Mismatched recipe, texture, alpha, or object identity fails closed.
//

//! Native simple/unlit world Material Instance request tests.

use super::WorldMaterialNativeInstanceRequest;
use crate::domain::{
    WorldMaterialNativeBlend, WorldMaterialNativeMasterRecipe,
    WorldMaterialPresentation, WorldMaterialRasterProjection,
    WorldMaterialSemantics, WorldMaterialShaderFamily,
};

fn presentation(
    textured: bool,
    alpha_test: bool,
    alpha_reference: Option<f32>,
) -> Result<WorldMaterialPresentation, String> {
    Ok(WorldMaterialPresentation {
        slot_presentation_sha256: "a".repeat(64),
        texture_file_name: textured
            .then(|| format!("texture-{}.png", "b".repeat(64))),
        texture_sha256: textured.then(|| "b".repeat(64)),
        base_color_rgba8: [17, 34, 51, 68],
        raster: WorldMaterialRasterProjection::from_pddi(
            WorldMaterialShaderFamily::Simple,
            1,
            u8::from(alpha_test),
            4,
            alpha_reference.map(f32::to_bits),
            0,
            0,
        )?,
        semantics: WorldMaterialSemantics {
            transparent: true,
            ..WorldMaterialSemantics::default()
        },
    })
}

fn recipe(alpha_test: bool) -> WorldMaterialNativeMasterRecipe {
    WorldMaterialNativeMasterRecipe {
        blend: WorldMaterialNativeBlend::SourceAlpha,
        alpha_test,
        render_both_faces: true,
    }
}

#[test]
fn textured_simple_unlit_request_uses_vertex_color_neutral_tint()
-> Result<(), String> {
    let source = presentation(true, true, Some(0.375))?;
    let request = WorldMaterialNativeInstanceRequest::new(
        &source,
        recipe(true),
        "/Game/Generated/SHAR/Materials/World/Instances",
        "MI_WorldSurface",
        concat!(
            "/Game/Generated/SHAR/Materials/World/Masters/",
            "M_WorldSimpleAlphaMasked.M_WorldSimpleAlphaMasked"
        ),
        Some("/Game/Generated/SHAR/Textures/World/T_Source.T_Source"),
    )?;
    assert_eq!(request.base_color_tint(), [1., 1., 1., 1.]);
    assert!(request.set_alpha_reference());
    assert_eq!(request.alpha_reference(), Some(0.375));
    assert_eq!(
        request.base_color_texture_path(),
        Some("/Game/Generated/SHAR/Textures/World/T_Source.T_Source")
    );
    assert_eq!(
        request.object_path(),
        concat!(
            "/Game/Generated/SHAR/Materials/World/Instances/",
            "MI_WorldSurface.MI_WorldSurface"
        )
    );
    assert_eq!(source.base_color_rgba8, [17, 34, 51, 68]);
    Ok(())
}

#[test]
fn untextured_opaque_request_has_no_texture_or_alpha_override()
-> Result<(), String> {
    let mut source = presentation(false, false, Some(0.9))?;
    source.raster = WorldMaterialRasterProjection::from_pddi(
        WorldMaterialShaderFamily::Simple,
        0,
        0,
        4,
        Some(0.9_f32.to_bits()),
        0,
        0,
    )?;
    source.semantics = WorldMaterialSemantics::default();
    let request = WorldMaterialNativeInstanceRequest::new(
        &source,
        WorldMaterialNativeMasterRecipe {
            blend: WorldMaterialNativeBlend::Opaque,
            alpha_test: false,
            render_both_faces: true,
        },
        "/Game/Generated/SHAR/Materials",
        "MI_Untextured",
        "/Game/Generated/SHAR/Materials/M_Opaque.M_Opaque",
        None,
    )?;
    assert_eq!(request.base_color_texture_path(), None);
    assert!(!request.set_alpha_reference());
    assert_eq!(request.alpha_reference(), None);
    Ok(())
}

#[test]
fn request_rejects_recipe_that_does_not_match_presentation()
-> Result<(), String> {
    let source = presentation(true, true, Some(0.5))?;
    let result = WorldMaterialNativeInstanceRequest::new(
        &source,
        recipe(false),
        "/Game/Generated/SHAR/Materials",
        "MI_Surface",
        "/Game/Generated/SHAR/Materials/M_Parent.M_Parent",
        Some("/Game/Generated/SHAR/Textures/T_Source.T_Source"),
    );
    assert_eq!(
        result,
        Err("world material instance recipe does not match presentation"
            .to_owned())
    );
    Ok(())
}

#[test]
fn request_rejects_texture_presence_drift() -> Result<(), String> {
    let textured = presentation(true, false, None)?;
    assert_eq!(
        WorldMaterialNativeInstanceRequest::new(
            &textured,
            recipe(false),
            "/Game/Generated/SHAR/Materials",
            "MI_Surface",
            "/Game/Generated/SHAR/Materials/M_Parent.M_Parent",
            None,
        ),
        Err(
            "world material instance texture presence differs from source"
                .to_owned()
        )
    );
    let untextured = presentation(false, false, None)?;
    assert_eq!(
        WorldMaterialNativeInstanceRequest::new(
            &untextured,
            recipe(false),
            "/Game/Generated/SHAR/Materials",
            "MI_Surface",
            "/Game/Generated/SHAR/Materials/M_Parent.M_Parent",
            Some("/Game/Generated/SHAR/Textures/T_Source.T_Source"),
        ),
        Err(
            "world material instance texture presence differs from source"
                .to_owned()
        )
    );
    Ok(())
}

#[test]
fn request_rejects_noncanonical_generated_object_shapes() -> Result<(), String>
{
    let source = presentation(true, false, None)?;
    for (folder, asset_name, parent, texture) in [
        (
            "/Game/Generated/SHAR/Textures",
            "MI_Surface",
            "/Game/Generated/SHAR/Materials/M_Parent.M_Parent",
            "/Game/Generated/SHAR/Textures/T_Source.T_Source",
        ),
        (
            "/Game/Generated/SHAR/Materials",
            "M_NotInstance",
            "/Game/Generated/SHAR/Materials/M_Parent.M_Parent",
            "/Game/Generated/SHAR/Textures/T_Source.T_Source",
        ),
        (
            "/Game/Generated/SHAR/Materials",
            "MI_Surface",
            "/Game/Generated/SHAR/Textures/M_Parent.M_Parent",
            "/Game/Generated/SHAR/Textures/T_Source.T_Source",
        ),
        (
            "/Game/Generated/SHAR/Materials",
            "MI_Surface",
            "/Game/Generated/SHAR/Materials/M_Parent.M_Parent",
            "/Engine/T_Source.T_Source",
        ),
    ] {
        assert!(
            WorldMaterialNativeInstanceRequest::new(
                &source,
                recipe(false),
                folder,
                asset_name,
                parent,
                Some(texture),
            )
            .is_err()
        );
    }
    Ok(())
}

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
//   - Native world-master compatibility classification tests.
// - Must-Not:
//   - Contact Unreal Editor or claim full material-presentation readiness.
// - Allows:
//   - Synthetic reviewed raster families and surface semantics.
// - Split-When:
//   - Additional native material policies gain independent test lifecycles.
// - Merge-When:
//   - Another test module owns identical master compatibility assertions.
// - Summary:
//   - Native world-master classifier tests.
// - Description:
//   - Proves exact tool tokens, source no-cull convergence, and fail-closed
//   - blockers for unsupported shader, lighting, and special surfaces.
// - Usage:
//   - Included by the world-material native compatibility domain module.
// - Defaults:
//   - No unsupported state may yield a native recipe.
//

//! Native world-master compatibility classifier tests.

use super::{
    WorldMaterialNativeBlend, WorldMaterialNativeMasterBlocker,
    WorldMaterialNativeMasterRecipe, WorldMaterialNativeMasterRequest,
    classify_world_material_native_master,
};
use crate::domain::{
    WorldMaterialAlphaCompare, WorldMaterialBlendFamily,
    WorldMaterialMasterFamily, WorldMaterialSemantics,
    WorldMaterialShaderFamily,
};

fn master(
    blend: WorldMaterialBlendFamily,
    alpha_test: bool,
    two_sided: bool,
) -> WorldMaterialMasterFamily {
    WorldMaterialMasterFamily {
        shader: WorldMaterialShaderFamily::Simple,
        blend,
        alpha_compare: alpha_test.then_some(WorldMaterialAlphaCompare::Greater),
        two_sided,
        lit: false,
    }
}

#[test]
fn reviewed_simple_unlit_state_maps_to_exact_native_tool_inputs()
-> Result<(), String> {
    for (blend, expected, token) in [
        (
            WorldMaterialBlendFamily::Disabled,
            WorldMaterialNativeBlend::Opaque,
            "opaque",
        ),
        (
            WorldMaterialBlendFamily::SourceAlpha,
            WorldMaterialNativeBlend::SourceAlpha,
            "alpha",
        ),
        (
            WorldMaterialBlendFamily::Additive,
            WorldMaterialNativeBlend::Additive,
            "additive",
        ),
    ] {
        for alpha_test in [false, true] {
            let classification = classify_world_material_native_master(
                master(blend, alpha_test, false),
                WorldMaterialSemantics::default(),
            );
            let recipe = classification
                .recipe()
                .ok_or("reviewed simple-unlit state has no native recipe")?;
            assert_eq!(recipe.blend, expected);
            assert_eq!(recipe.blend.tool_token(), token);
            assert_eq!(recipe.alpha_test, alpha_test);
            assert!(recipe.render_both_faces);
            assert!(classification.blockers().is_empty());
        }
    }
    Ok(())
}

#[test]
fn source_two_sided_flag_does_not_split_regular_world_native_recipe() {
    let one_sided = classify_world_material_native_master(
        master(WorldMaterialBlendFamily::Disabled, false, false),
        WorldMaterialSemantics::default(),
    );
    let two_sided = classify_world_material_native_master(
        master(WorldMaterialBlendFamily::Disabled, false, true),
        WorldMaterialSemantics::default(),
    );
    assert_eq!(one_sided.recipe(), two_sided.recipe());
    assert!(
        one_sided
            .recipe()
            .is_some_and(|recipe| recipe.render_both_faces)
    );
}

#[test]
fn source_alpha_transparency_is_representable_without_special_surface_policy()
-> Result<(), String> {
    let classification = classify_world_material_native_master(
        master(WorldMaterialBlendFamily::SourceAlpha, true, false),
        WorldMaterialSemantics {
            transparent: true,
            ..WorldMaterialSemantics::default()
        },
    );
    let recipe = classification
        .recipe()
        .ok_or("plain source-alpha transparency has no native recipe")?;
    assert_eq!(recipe.blend, WorldMaterialNativeBlend::SourceAlpha);
    assert!(recipe.alpha_test);
    assert!(classification.blockers().is_empty());
    Ok(())
}

#[test]
fn unsupported_shader_and_lighting_report_all_master_blockers() {
    let classification = classify_world_material_native_master(
        WorldMaterialMasterFamily {
            shader: WorldMaterialShaderFamily::Environment,
            blend: WorldMaterialBlendFamily::Disabled,
            alpha_compare: None,
            two_sided: false,
            lit: true,
        },
        WorldMaterialSemantics::default(),
    );
    assert_eq!(classification.recipe(), None);
    assert_eq!(classification.blockers(), &[
        WorldMaterialNativeMasterBlocker::UnsupportedShaderFamily,
        WorldMaterialNativeMasterBlocker::Lit,
    ]);
}

#[test]
fn overlapping_special_surface_state_stays_blocked_without_approximation() {
    let classification = classify_world_material_native_master(
        master(WorldMaterialBlendFamily::SourceAlpha, false, true),
        WorldMaterialSemantics {
            transparent: true,
            glass: true,
            mirror: false,
            reflective: true,
            light_emitter: true,
            visual_effect: true,
        },
    );
    assert_eq!(classification.recipe(), None);
    assert_eq!(classification.blockers(), &[
        WorldMaterialNativeMasterBlocker::Glass,
        WorldMaterialNativeMasterBlocker::Reflective,
        WorldMaterialNativeMasterBlocker::LightEmitter,
        WorldMaterialNativeMasterBlocker::VisualEffect,
    ]);
}

#[test]
fn mirror_uses_specific_blocker_instead_of_duplicate_reflective_blocker() {
    let classification = classify_world_material_native_master(
        master(WorldMaterialBlendFamily::Disabled, false, false),
        WorldMaterialSemantics {
            mirror: true,
            reflective: true,
            ..WorldMaterialSemantics::default()
        },
    );
    assert_eq!(classification.blockers(), &[
        WorldMaterialNativeMasterBlocker::Mirror
    ]);
    assert_eq!(classification.recipe(), None);
}

#[test]
fn runtime_error_fallback_never_maps_to_simple_native_master() {
    let classification = classify_world_material_native_master(
        WorldMaterialMasterFamily {
            shader: WorldMaterialShaderFamily::RuntimeError,
            blend: WorldMaterialBlendFamily::Disabled,
            alpha_compare: None,
            two_sided: false,
            lit: false,
        },
        WorldMaterialSemantics::default(),
    );
    assert_eq!(classification.recipe(), None);
    assert_eq!(classification.blockers(), &[
        WorldMaterialNativeMasterBlocker::UnsupportedShaderFamily
    ]);
}

#[test]
fn blocker_codes_are_stable_and_public_safe() {
    for (blocker, expected) in [
        (
            WorldMaterialNativeMasterBlocker::UnsupportedShaderFamily,
            "unsupported-shader-family",
        ),
        (WorldMaterialNativeMasterBlocker::Lit, "lit-presentation"),
        (
            WorldMaterialNativeMasterBlocker::Glass,
            "glass-presentation",
        ),
        (
            WorldMaterialNativeMasterBlocker::Mirror,
            "mirror-presentation",
        ),
        (
            WorldMaterialNativeMasterBlocker::Reflective,
            "reflective-presentation",
        ),
        (
            WorldMaterialNativeMasterBlocker::LightEmitter,
            "light-emitter-presentation",
        ),
        (
            WorldMaterialNativeMasterBlocker::VisualEffect,
            "visual-effect-presentation",
        ),
    ] {
        assert_eq!(blocker.code(), expected);
    }
}

#[test]
fn native_recipe_identities_are_stable_across_reviewed_tool_inputs() {
    for (recipe, expected) in [
        (
            WorldMaterialNativeMasterRecipe {
                blend: WorldMaterialNativeBlend::Opaque,
                alpha_test: false,
                render_both_faces: true,
            },
            "simple-unlit__blend-opaque__alpha-test-off__both-faces",
        ),
        (
            WorldMaterialNativeMasterRecipe {
                blend: WorldMaterialNativeBlend::Opaque,
                alpha_test: true,
                render_both_faces: true,
            },
            "simple-unlit__blend-opaque__alpha-test-on__both-faces",
        ),
        (
            WorldMaterialNativeMasterRecipe {
                blend: WorldMaterialNativeBlend::SourceAlpha,
                alpha_test: false,
                render_both_faces: true,
            },
            "simple-unlit__blend-alpha__alpha-test-off__both-faces",
        ),
        (
            WorldMaterialNativeMasterRecipe {
                blend: WorldMaterialNativeBlend::SourceAlpha,
                alpha_test: true,
                render_both_faces: true,
            },
            "simple-unlit__blend-alpha__alpha-test-on__both-faces",
        ),
        (
            WorldMaterialNativeMasterRecipe {
                blend: WorldMaterialNativeBlend::Additive,
                alpha_test: false,
                render_both_faces: true,
            },
            "simple-unlit__blend-additive__alpha-test-off__both-faces",
        ),
        (
            WorldMaterialNativeMasterRecipe {
                blend: WorldMaterialNativeBlend::Additive,
                alpha_test: true,
                render_both_faces: true,
            },
            "simple-unlit__blend-additive__alpha-test-on__both-faces",
        ),
    ] {
        assert_eq!(recipe.identity(), expected);
    }
    assert_eq!(
        WorldMaterialNativeMasterRecipe {
            blend: WorldMaterialNativeBlend::Opaque,
            alpha_test: false,
            render_both_faces: false,
        }
        .identity(),
        "simple-unlit__blend-opaque__alpha-test-off__one-sided"
    );
}

#[test]
fn native_master_request_binds_recipe_to_canonical_generated_destination()
-> Result<(), String> {
    let recipe = WorldMaterialNativeMasterRecipe {
        blend: WorldMaterialNativeBlend::SourceAlpha,
        alpha_test: true,
        render_both_faces: true,
    };
    let request = WorldMaterialNativeMasterRequest::new(
        recipe,
        "/Game/Generated/SHAR/Materials/World/Masters",
        "M_WorldSimpleAlphaMasked",
    )?;
    assert_eq!(request.recipe(), recipe);
    assert_eq!(
        request.recipe().identity(),
        "simple-unlit__blend-alpha__alpha-test-on__both-faces"
    );
    assert_eq!(
        request.folder_path(),
        "/Game/Generated/SHAR/Materials/World/Masters"
    );
    assert_eq!(request.asset_name(), "M_WorldSimpleAlphaMasked");
    assert_eq!(
        request.package_path(),
        concat!(
            "/Game/Generated/SHAR/Materials/World/Masters/",
            "M_WorldSimpleAlphaMasked"
        )
    );
    assert_eq!(
        request.object_path(),
        concat!(
            "/Game/Generated/SHAR/Materials/World/Masters/",
            "M_WorldSimpleAlphaMasked.M_WorldSimpleAlphaMasked"
        )
    );
    Ok(())
}

#[test]
fn native_master_request_rejects_unreviewed_destination_shapes() {
    let recipe = WorldMaterialNativeMasterRecipe {
        blend: WorldMaterialNativeBlend::Opaque,
        alpha_test: false,
        render_both_faces: true,
    };
    for folder in [
        "/Game/Generated/SHAR/Textures",
        "/Game/Generated/SHAR/Materials/",
        "/Game/Generated/SHAR/Materials/../Escape",
        "/Game/Generated/SHAR/Materials/World-Masters",
    ] {
        assert!(
            WorldMaterialNativeMasterRequest::new(recipe, folder, "M_World")
                .is_err()
        );
    }
    for asset_name in [
        "",
        "WorldMaster",
        "M_",
        "M_Bad.Name",
        "M_Bad/Name",
        "M_Bad-Name",
    ] {
        assert!(
            WorldMaterialNativeMasterRequest::new(
                recipe,
                "/Game/Generated/SHAR/Materials",
                asset_name,
            )
            .is_err()
        );
    }
}

#[test]
fn native_master_request_rejects_one_sided_recipe() {
    let result = WorldMaterialNativeMasterRequest::new(
        WorldMaterialNativeMasterRecipe {
            blend: WorldMaterialNativeBlend::Opaque,
            alpha_test: false,
            render_both_faces: false,
        },
        "/Game/Generated/SHAR/Materials",
        "M_WorldOpaque",
    );
    assert_eq!(
        result,
        Err("world master request must reproduce regular-world CullNone"
            .to_owned())
    );
}

#[test]
fn native_master_request_rejects_oversized_object_path() {
    let asset_name = format!("M_{}", "A".repeat(220));
    let result = WorldMaterialNativeMasterRequest::new(
        WorldMaterialNativeMasterRecipe {
            blend: WorldMaterialNativeBlend::Opaque,
            alpha_test: false,
            render_both_faces: true,
        },
        "/Game/Generated/SHAR/Materials",
        &asset_name,
    );
    assert_eq!(
        result,
        Err("world master object path is too long".to_owned())
    );
}

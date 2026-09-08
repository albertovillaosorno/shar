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

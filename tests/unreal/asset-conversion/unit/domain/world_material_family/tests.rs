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
//   - World-material master-family selection tests.
// - Must-Not:
//   - Read private game content or contact Unreal Editor.
// - Allows:
//   - Synthetic reviewed PDDI raster values.
// - Split-When:
//   - Additional material-family policies gain independent ownership.
// - Merge-When:
//   - Another test module owns identical raster-family validation.
// - Summary:
//   - World-material family selection tests.
// - Description:
//   - Proves active-state selection, PDDI alpha-reference defaults, dormant
//   - state elision, error fallback identity, and fail-closed unsupported
//   - modes.
// - Usage:
//   - Included by the asset-conversion world-material family domain module.
// - Defaults:
//   - Unreviewed active raster state is rejected.
//

//! World-material master-family selection tests.

use super::{
    PDDI_DEFAULT_ALPHA_REFERENCE_BITS, WorldMaterialAlphaCompare,
    WorldMaterialBlendFamily, WorldMaterialRasterProjection,
    WorldMaterialShaderFamily,
};

#[test]
fn opaque_simple_state_selects_stable_master_family() -> Result<(), String> {
    let projection = WorldMaterialRasterProjection::from_pddi(
        WorldMaterialShaderFamily::Simple,
        0,
        0,
        4,
        Some(0.8_f32.to_bits()),
        0,
        0,
    )?;
    assert_eq!(projection.master.shader, WorldMaterialShaderFamily::Simple);
    assert_eq!(projection.master.blend, WorldMaterialBlendFamily::Disabled);
    assert_eq!(projection.master.alpha_compare, None);
    assert!(!projection.master.two_sided);
    assert!(!projection.master.lit);
    assert_eq!(projection.instance.alpha_reference_bits, None);
    assert_eq!(
        projection.master.identity(),
        "simple__blend-none__alpha-test-off__one-sided__unlit"
    );
    Ok(())
}

#[test]
fn active_alpha_test_uses_pddi_default_when_threshold_is_absent()
-> Result<(), String> {
    let projection = WorldMaterialRasterProjection::from_pddi(
        WorldMaterialShaderFamily::Simple,
        1,
        1,
        4,
        None,
        1,
        0,
    )?;
    assert_eq!(
        projection.master.blend,
        WorldMaterialBlendFamily::SourceAlpha
    );
    assert_eq!(
        projection.master.alpha_compare,
        Some(WorldMaterialAlphaCompare::Greater)
    );
    assert_eq!(
        projection.instance.alpha_reference_bits,
        Some(PDDI_DEFAULT_ALPHA_REFERENCE_BITS)
    );
    assert_eq!(
        projection.master.identity(),
        "simple__blend-alpha__alpha-test-greater__two-sided__unlit"
    );
    Ok(())
}

#[test]
fn alpha_threshold_is_instance_state_not_master_identity() -> Result<(), String>
{
    let half = WorldMaterialRasterProjection::from_pddi(
        WorldMaterialShaderFamily::Simple,
        0,
        1,
        4,
        Some(0.5_f32.to_bits()),
        1,
        0,
    )?;
    let eighty_percent = WorldMaterialRasterProjection::from_pddi(
        WorldMaterialShaderFamily::Simple,
        0,
        1,
        4,
        Some(0.8_f32.to_bits()),
        1,
        0,
    )?;
    assert_eq!(half.master, eighty_percent.master);
    assert_ne!(half.instance, eighty_percent.instance);
    Ok(())
}

#[test]
fn additive_blend_and_alpha_test_remain_independent() -> Result<(), String> {
    let projection = WorldMaterialRasterProjection::from_pddi(
        WorldMaterialShaderFamily::Simple,
        2,
        1,
        4,
        Some(0.5_f32.to_bits()),
        0,
        0,
    )?;
    assert_eq!(projection.master.blend, WorldMaterialBlendFamily::Additive);
    assert_eq!(
        projection.master.alpha_compare,
        Some(WorldMaterialAlphaCompare::Greater)
    );
    Ok(())
}

#[test]
fn dormant_alpha_state_does_not_split_master_or_instance() -> Result<(), String>
{
    let canonical = WorldMaterialRasterProjection::from_pddi(
        WorldMaterialShaderFamily::Simple,
        0,
        0,
        4,
        Some(0.5_f32.to_bits()),
        0,
        0,
    )?;
    let dormant_variant = WorldMaterialRasterProjection::from_pddi(
        WorldMaterialShaderFamily::Simple,
        0,
        0,
        0,
        Some(0.8_f32.to_bits()),
        0,
        0,
    )?;
    assert_eq!(canonical, dormant_variant);
    Ok(())
}

#[test]
fn pddi_alpha_reference_clamp_is_reproduced() -> Result<(), String> {
    let high = WorldMaterialRasterProjection::from_pddi(
        WorldMaterialShaderFamily::Simple,
        0,
        1,
        4,
        Some(1.5_f32.to_bits()),
        0,
        0,
    )?;
    let negative = WorldMaterialRasterProjection::from_pddi(
        WorldMaterialShaderFamily::Simple,
        0,
        1,
        4,
        Some((-0.25_f32).to_bits()),
        0,
        0,
    )?;
    let nan = WorldMaterialRasterProjection::from_pddi(
        WorldMaterialShaderFamily::Simple,
        0,
        1,
        4,
        Some(f32::NAN.to_bits()),
        0,
        0,
    )?;
    assert_eq!(high.instance.alpha_reference_bits, Some(1f32.to_bits()));
    assert_eq!(negative.instance.alpha_reference_bits, Some(0f32.to_bits()));
    assert_eq!(nan.instance.alpha_reference_bits, Some(0f32.to_bits()));
    Ok(())
}

#[test]
fn runtime_error_fallback_stays_distinct_from_simple_default()
-> Result<(), String> {
    let fallback = WorldMaterialRasterProjection::runtime_error();
    let simple = WorldMaterialRasterProjection::from_pddi(
        WorldMaterialShaderFamily::Simple,
        0,
        0,
        5,
        None,
        0,
        0,
    )?;
    assert_ne!(fallback.master, simple.master);
    assert_eq!(
        fallback.master.identity(),
        "runtime-error__blend-none__alpha-test-off__one-sided__unlit"
    );
    Ok(())
}

#[test]
fn decoded_path_rejects_runtime_error_shader_family() {
    let result = WorldMaterialRasterProjection::from_pddi(
        WorldMaterialShaderFamily::RuntimeError,
        0,
        0,
        5,
        None,
        0,
        0,
    );
    assert_eq!(
        result,
        Err("runtime error material must use its dedicated constructor"
            .to_owned())
    );
}

#[test]
fn unreviewed_active_blend_mode_fails_closed() {
    let result = WorldMaterialRasterProjection::from_pddi(
        WorldMaterialShaderFamily::Simple,
        3,
        0,
        4,
        None,
        0,
        0,
    );
    assert_eq!(
        result,
        Err("world material uses unreviewed active PDDI blend mode".to_owned())
    );
}

#[test]
fn unreviewed_active_alpha_compare_fails_closed() {
    let result = WorldMaterialRasterProjection::from_pddi(
        WorldMaterialShaderFamily::Simple,
        0,
        1,
        5,
        None,
        0,
        0,
    );
    assert_eq!(
        result,
        Err("world material uses unreviewed active PDDI alpha compare"
            .to_owned())
    );
}

#[test]
fn non_binary_source_flags_fail_closed() {
    for (alpha_test, two_sided, lit, expected) in [
        (2, 0, 0, "PDDI alpha-test flag is not binary"),
        (0, 2, 0, "PDDI two-sided flag is not binary"),
        (0, 0, 2, "PDDI lighting flag is not binary"),
    ] {
        let result = WorldMaterialRasterProjection::from_pddi(
            WorldMaterialShaderFamily::Environment,
            0,
            alpha_test,
            4,
            None,
            two_sided,
            lit,
        );
        assert_eq!(result, Err(expected.to_owned()));
    }
}

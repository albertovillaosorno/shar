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
//   - Native world-material construction request projection tests.
// - Must-Not:
//   - Read generated caches, contact Unreal Editor, or claim execution
//     readiness.
// - Allows:
//   - Synthetic verified textures, presentations, and reviewed recipes.
// - Split-When:
//   - Another native construction family gains an independent test lifecycle.
// - Merge-When:
//   - Another test module owns identical destination-selection assertions.
// - Summary:
//   - Native world-material construction projection tests.
// - Description:
//   - Proves content-addressed textures, recipe-addressed masters,
//   - presentation-addressed instances, and fail-closed dependency joins.
// - Usage:
//   - Included only by the native world-material planning adapter.
// - Defaults:
//   - Unsupported presentation emits no instance request.
//

//! Native world-material construction request projection tests.

use std::collections::BTreeMap;

use serde_json::Value;
use shar_unreal_conversion::domain::{
    WorldMaterialNativeMasterRecipe, WorldMaterialPresentation,
    WorldMaterialRasterProjection, WorldMaterialSemantics,
    WorldMaterialShaderFamily, classify_world_material_native_master,
};

use super::plan_world_material_native_construction;
use crate::adapters::driven::local::unreal_world_material_catalog::
    VerifiedWorldTexture;

const PRESENTATION: &str =
    "3333333333333333333333333333333333333333333333333333333333333333";
const TEXTURE: &str =
    "4444444444444444444444444444444444444444444444444444444444444444";

fn presentation(
    semantics: WorldMaterialSemantics,
) -> Result<WorldMaterialPresentation, String> {
    Ok(WorldMaterialPresentation {
        slot_presentation_sha256: PRESENTATION.to_owned(),
        texture_file_name: Some(format!("texture-{TEXTURE}.png")),
        texture_sha256: Some(TEXTURE.to_owned()),
        base_color_rgba8: [10, 20, 30, 40],
        raster: WorldMaterialRasterProjection::from_pddi(
            WorldMaterialShaderFamily::Simple,
            1,
            1,
            4,
            Some(0.375_f32.to_bits()),
            0,
            0,
        )?,
        semantics,
    })
}

fn texture() -> VerifiedWorldTexture {
    texture_for(TEXTURE)
}

fn texture_for(sha256: &str) -> VerifiedWorldTexture {
    VerifiedWorldTexture {
        file_name: format!("texture-{sha256}.png"),
        bytes: 99,
        sha256: sha256.to_owned(),
    }
}

fn recipe(
    source: &WorldMaterialPresentation,
) -> Result<WorldMaterialNativeMasterRecipe, String> {
    let classification = classify_world_material_native_master(
        source.raster.master,
        source.semantics,
    );
    classification
        .recipe()
        .ok_or_else(|| "synthetic presentation has no native recipe".to_owned())
}

#[test]
fn plans_content_recipe_and_presentation_addressed_native_requests()
-> Result<(), String> {
    let source = presentation(WorldMaterialSemantics::default())?;
    let selected_recipe = recipe(&source)?;
    let presentations = BTreeMap::from([(PRESENTATION.to_owned(), source)]);
    let recipes = BTreeMap::from([(
        selected_recipe.identity(),
        selected_recipe,
    )]);
    let plan = plan_world_material_native_construction(
        &[texture()],
        &presentations,
        &recipes,
    )
    .map_err(|error| error.to_string())?;
    assert_eq!(plan.texture_requests.len(), 1);
    assert_eq!(plan.master_requests.len(), 1);
    assert_eq!(plan.instance_requests.len(), 1);
    let texture_request = plan
        .texture_requests
        .first()
        .ok_or_else(|| "native texture request disappeared".to_owned())?;
    let master_request = plan
        .master_requests
        .first()
        .ok_or_else(|| "native master request disappeared".to_owned())?;
    let instance_request = plan
        .instance_requests
        .first()
        .ok_or_else(|| "native instance request disappeared".to_owned())?;
    let expected_texture = format!(
        concat!(
            "/Game/Generated/SHAR/Textures/World/",
            "T_World_{}.T_World_{}"
        ),
        TEXTURE,
        TEXTURE
    );
    assert_eq!(
        texture_request
            .pointer("/object_path")
            .and_then(Value::as_str),
        Some(expected_texture.as_str())
    );
    assert_eq!(
        master_request.pointer("/asset_name").and_then(Value::as_str),
        Some(concat!(
            "M_SHAR_World_simple_unlit_blend_alpha_alpha_test_on_",
            "both_faces"
        ))
    );
    let expected_instance = format!(
        concat!(
            "/Game/Generated/SHAR/Materials/World/Instances/",
            "MI_World_{}.MI_World_{}"
        ),
        PRESENTATION,
        PRESENTATION
    );
    assert_eq!(
        instance_request
            .pointer("/object_path")
            .and_then(Value::as_str),
        Some(expected_instance.as_str())
    );
    assert_eq!(
        instance_request
            .pointer("/base_color_texture_path")
            .and_then(Value::as_str),
        texture_request.pointer("/object_path").and_then(Value::as_str)
    );
    assert_eq!(
        instance_request
            .pointer("/parent_material_path")
            .and_then(Value::as_str),
        master_request.pointer("/object_path").and_then(Value::as_str)
    );
    assert_eq!(
        instance_request
            .pointer("/base_color_tint/0")
            .and_then(Value::as_f64),
        Some(1.)
    );
    assert_eq!(
        instance_request
            .pointer("/set_alpha_reference")
            .and_then(Value::as_bool),
        Some(true)
    );
    assert_eq!(
        instance_request
            .pointer("/alpha_reference")
            .and_then(Value::as_f64),
        Some(0.375)
    );
    Ok(())
}

#[test]
fn blocked_presentation_keeps_texture_but_emits_no_native_instance()
-> Result<(), String> {
    let source = presentation(WorldMaterialSemantics {
        glass: true,
        transparent: true,
        ..WorldMaterialSemantics::default()
    })?;
    let presentations = BTreeMap::from([(PRESENTATION.to_owned(), source)]);
    let plan = plan_world_material_native_construction(
        &[texture()],
        &presentations,
        &BTreeMap::new(),
    )
    .map_err(|error| error.to_string())?;
    assert_eq!(plan.texture_requests.len(), 1);
    assert!(plan.master_requests.is_empty());
    assert!(plan.instance_requests.is_empty());
    Ok(())
}

#[test]
fn missing_planned_texture_fails_closed() -> Result<(), String> {
    let source = presentation(WorldMaterialSemantics::default())?;
    let selected_recipe = recipe(&source)?;
    let presentations = BTreeMap::from([(PRESENTATION.to_owned(), source)]);
    let recipes = BTreeMap::from([(
        selected_recipe.identity(),
        selected_recipe,
    )]);
    let result = plan_world_material_native_construction(
        &[],
        &presentations,
        &recipes,
    );
    let Err(error) = result else {
        return Err(
            "presentation without planned texture unexpectedly succeeded"
                .to_owned(),
        );
    };
    assert!(error.to_string().contains("texture is not planned"));
    Ok(())
}


#[test]
fn texture_requests_are_sorted_by_content_identity() -> Result<(), String> {
    let low = concat!(
        "11111111111111111111111111111111",
        "11111111111111111111111111111111"
    );
    let high = concat!(
        "ffffffffffffffffffffffffffffffff",
        "ffffffffffffffffffffffffffffffff"
    );
    let plan = plan_world_material_native_construction(
        &[texture_for(high), texture_for(low)],
        &BTreeMap::new(),
        &BTreeMap::new(),
    )
    .map_err(|error| error.to_string())?;
    let first = plan
        .texture_requests
        .first()
        .and_then(|value| value.pointer("/sha256"))
        .and_then(Value::as_str);
    let last = plan
        .texture_requests
        .last()
        .and_then(|value| value.pointer("/sha256"))
        .and_then(Value::as_str);
    assert_eq!(first, Some(low));
    assert_eq!(last, Some(high));
    Ok(())
}

#[test]
fn duplicate_verified_texture_digest_fails_closed() -> Result<(), String> {
    let result = plan_world_material_native_construction(
        &[texture(), texture()],
        &BTreeMap::new(),
        &BTreeMap::new(),
    );
    let Err(error) = result else {
        return Err(
            "duplicate verified texture unexpectedly planned".to_owned(),
        );
    };
    assert!(error.to_string().contains("digest is duplicated"));
    Ok(())
}

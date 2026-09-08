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
//   - Canonical world-material plan-evidence renderer tests.
// - Must-Not:
//   - Read game assets, mutate generated caches, or contact Unreal Editor.
// - Allows:
//   - Synthetic verified projections and deterministic JSON assertions.
// - Split-When:
//   - Native readiness selection gains independent tests.
// - Merge-When:
//   - Another test owns the identical world-material sidecar contract.
// - Summary:
//   - World-material plan-evidence rendering tests.
// - Description:
//   - Proves exact evidence retention, empty rendering, and global conflict
//   - rejection before native material planning.
// - Usage:
//   - Included only by the owning local adapter under cfg(test).
// - Defaults:
//   - Synthetic evidence is source-independent and deterministic.
//

//! World-material plan-evidence rendering tests.

use serde_json::Value;
use shar_unreal_conversion::domain::{
    WorldMaterialBindingSource, WorldMaterialProjection,
    WorldMaterialRasterProjection, WorldMaterialSemantics,
    WorldMaterialShaderFamily, WorldMaterialSlotSource,
};

use super::{WORLD_MATERIAL_PLAN_SCHEMA, render_world_material_plan};
use crate::adapters::driven::local::unreal_world_material_catalog::{
    VerifiedWorldMaterialArtifact, VerifiedWorldMaterialCatalog,
    VerifiedWorldTexture,
};

const BINDING: &str =
    "1111111111111111111111111111111111111111111111111111111111111111";
const PRESENTATION: &str =
    "2222222222222222222222222222222222222222222222222222222222222222";
const EFFECTIVE: &str =
    "3333333333333333333333333333333333333333333333333333333333333333";
const TEXTURE: &str =
    "4444444444444444444444444444444444444444444444444444444444444444";

fn projection(color: [u8; 4]) -> Result<WorldMaterialProjection, String> {
    let raster = WorldMaterialRasterProjection::from_pddi(
        WorldMaterialShaderFamily::Simple,
        1,
        1,
        4,
        Some(0.375_f32.to_bits()),
        1,
        0,
    )?;
    let material = format!("material-{BINDING}");
    WorldMaterialProjection::build(
        &[WorldMaterialBindingSource {
            material_name: material.clone(),
            texture_file_name: Some(format!("texture-{TEXTURE}.png")),
            texture_sha256: Some(TEXTURE.to_owned()),
            binding_sha256: BINDING.to_owned(),
            presentation_sha256: PRESENTATION.to_owned(),
            base_color_rgba8: color,
            raster,
            semantics: WorldMaterialSemantics::default(),
        }],
        &[WorldMaterialSlotSource {
            slot_name: format!("{material}__glass"),
            source_material_name: material,
            binding_sha256: BINDING.to_owned(),
            presentation_sha256: PRESENTATION.to_owned(),
            slot_presentation_sha256: EFFECTIVE.to_owned(),
            semantics: WorldMaterialSemantics {
                transparent: true,
                glass: true,
                ..WorldMaterialSemantics::default()
            },
        }],
    )
}

fn ordinary_projection(
    color: [u8; 4],
) -> Result<WorldMaterialProjection, String> {
    let raster = WorldMaterialRasterProjection::from_pddi(
        WorldMaterialShaderFamily::Simple,
        1,
        1,
        4,
        Some(0.375_f32.to_bits()),
        0,
        0,
    )?;
    let material = format!("material-{BINDING}");
    WorldMaterialProjection::build(
        &[WorldMaterialBindingSource {
            material_name: material.clone(),
            texture_file_name: Some(format!("texture-{TEXTURE}.png")),
            texture_sha256: Some(TEXTURE.to_owned()),
            binding_sha256: BINDING.to_owned(),
            presentation_sha256: PRESENTATION.to_owned(),
            base_color_rgba8: color,
            raster,
            semantics: WorldMaterialSemantics::default(),
        }],
        &[WorldMaterialSlotSource {
            slot_name: material.clone(),
            source_material_name: material,
            binding_sha256: BINDING.to_owned(),
            presentation_sha256: PRESENTATION.to_owned(),
            slot_presentation_sha256: EFFECTIVE.to_owned(),
            semantics: WorldMaterialSemantics::default(),
        }],
    )
}

fn catalog() -> Result<VerifiedWorldMaterialCatalog, String> {
    Ok(VerifiedWorldMaterialCatalog {
        artifact_count: 1,
        binding_count: 1,
        slot_count: 1,
        master_family_count: 1,
        artifacts: vec![VerifiedWorldMaterialArtifact {
            path: "levels/level-01-zones-l1z1.fbx".to_owned(),
            bytes: 123,
            sha256: "5".repeat(64),
            projection: projection([10, 20, 30, 40])?,
        }],
        textures: vec![VerifiedWorldTexture {
            file_name: format!("texture-{TEXTURE}.png"),
            bytes: 99,
            sha256: TEXTURE.to_owned(),
        }],
    })
}

#[test]
fn renders_exact_verified_world_material_evidence() -> Result<(), String> {
    let catalog = catalog()?;
    let text = render_world_material_plan(Some(&catalog))
        .map_err(|error| error.to_string())?;
    if !text.ends_with('\n') || text.lines().count() != 1 {
        return Err(
            "world material evidence is not canonical JSON plus LF".to_owned(),
        );
    }
    let value: Value = serde_json::from_str(&text)
        .map_err(|error| error.to_string())?;
    if value.get("schema").and_then(Value::as_str)
        != Some(WORLD_MATERIAL_PLAN_SCHEMA)
        || value.pointer("/counts/artifacts").and_then(Value::as_u64)
            != Some(1)
        || value.pointer("/counts/presentations").and_then(Value::as_u64)
            != Some(1)
        || value.pointer("/textures/0/source_path").and_then(Value::as_str)
            != Some(concat!(
                "world-assets/textures/texture-",
                concat!(
                    "44444444444444444444444444444444",
                    "44444444444444444444444444444444"
                ),
                ".png"
            ))
        || value.pointer("/master_families/0/shader").and_then(Value::as_str)
            != Some("simple")
        || value.pointer("/master_families/0/blend").and_then(Value::as_str)
            != Some("source-alpha")
        || value
            .pointer("/master_families/0/two_sided")
            .and_then(Value::as_bool)
            != Some(true)
        || value
            .pointer("/counts/native_master_recipes")
            .and_then(Value::as_u64)
            != Some(0)
        || value
            .pointer("/counts/native_master_ready_presentations")
            .and_then(Value::as_u64)
            != Some(0)
        || value
            .pointer("/counts/native_master_blocked_presentations")
            .and_then(Value::as_u64)
            != Some(1)
        || !value
            .pointer("/presentations/0/native_master_recipe")
            .is_some_and(Value::is_null)
        || value
            .pointer("/presentations/0/native_master_blockers/0")
            .and_then(Value::as_str)
            != Some("glass-presentation")
        || value.pointer("/presentations/0/base_color_rgba8/2")
            .and_then(Value::as_u64)
            != Some(30)
        || value.pointer("/presentations/0/alpha_reference_bits")
            .and_then(Value::as_u64)
            != Some(u64::from(0.375_f32.to_bits()))
        || value.pointer("/artifacts/0/assignments/0/slot_name")
            .and_then(Value::as_str)
            != Some(concat!(
                "material-",
                concat!(
                    "11111111111111111111111111111111",
                    "11111111111111111111111111111111"
                ),
                "__glass"
            ))
    {
        return Err(
            "world material evidence lost verified projection state".to_owned(),
        );
    }
    Ok(())
}

#[test]
fn ordinary_presentation_references_deduplicated_native_master_recipe()
-> Result<(), String> {
    let mut catalog = catalog()?;
    let artifact = catalog
        .artifacts
        .first_mut()
        .ok_or("synthetic verified world artifact disappeared")?;
    artifact.projection = ordinary_projection([10, 20, 30, 40])?;
    let text = render_world_material_plan(Some(&catalog))
        .map_err(|error| error.to_string())?;
    let value: Value = serde_json::from_str(&text)
        .map_err(|error| error.to_string())?;
    let identity = concat!(
        "simple-unlit__blend-alpha__alpha-test-on__",
        "both-faces"
    );
    if value.pointer("/counts/native_master_recipes").and_then(Value::as_u64)
        != Some(1)
        || value
            .pointer("/counts/native_master_ready_presentations")
            .and_then(Value::as_u64)
            != Some(1)
        || value
            .pointer("/counts/native_master_blocked_presentations")
            .and_then(Value::as_u64)
            != Some(0)
        || value
            .pointer("/native_master_recipes/0/identity")
            .and_then(Value::as_str)
            != Some(identity)
        || value
            .pointer("/native_master_recipes/0/blend_family")
            .and_then(Value::as_str)
            != Some("alpha")
        || value
            .pointer("/native_master_recipes/0/alpha_test")
            .and_then(Value::as_bool)
            != Some(true)
        || value
            .pointer("/native_master_recipes/0/render_both_faces")
            .and_then(Value::as_bool)
            != Some(true)
        || value
            .pointer("/presentations/0/native_master_recipe")
            .and_then(Value::as_str)
            != Some(identity)
        || value
            .pointer("/presentations/0/native_master_blockers")
            .and_then(Value::as_array)
            .is_none_or(|blockers| !blockers.is_empty())
    {
        return Err(
            "ordinary native world master recipe projection drifted".to_owned(),
        );
    }
    Ok(())
}

#[test]
fn missing_catalog_renders_explicit_empty_evidence() -> Result<(), String> {
    let text = render_world_material_plan(None)
        .map_err(|error| error.to_string())?;
    let value: Value = serde_json::from_str(&text)
        .map_err(|error| error.to_string())?;
    for path in [
        "/counts/artifacts",
        "/counts/bindings",
        "/counts/slots",
        "/counts/master_families",
        "/counts/native_master_recipes",
        "/counts/native_master_ready_presentations",
        "/counts/native_master_blocked_presentations",
        "/counts/textures",
        "/counts/presentations",
    ] {
        if value.pointer(path).and_then(Value::as_u64) != Some(0) {
            return Err(format!(
                "empty world material evidence drifted at {path}"
            ));
        }
    }
    Ok(())
}

#[test]
fn global_presentation_digest_conflict_fails_closed() -> Result<(), String> {
    let mut catalog = catalog()?;
    catalog.artifact_count = 2;
    catalog.binding_count = 2;
    catalog.slot_count = 2;
    catalog.artifacts.push(VerifiedWorldMaterialArtifact {
        path: "levels/level-01-zones-l1z2.fbx".to_owned(),
        bytes: 124,
        sha256: "6".repeat(64),
        projection: projection([11, 20, 30, 40])?,
    });
    let Err(error) = render_world_material_plan(Some(&catalog)) else {
        return Err(
            "conflicting global presentation unexpectedly rendered".to_owned(),
        );
    };
    if !error.to_string().contains("conflicts globally") {
        return Err(format!("unexpected global presentation failure: {error}"));
    }
    Ok(())
}

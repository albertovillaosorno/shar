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
//   - Normalized UI-sprite raster compiler unit tests.
// - Must-Not:
//   - Depend on proprietary source fixtures or sibling PNG files.
// - Allows:
//   - Synthetic normalized sprite ledgers and legacy DDS blocks.
// - Split-When:
//   - Catalog publication gains independent fixture ownership.
// - Merge-When:
//   - Another test module owns the identical compiler boundary.
// - Summary:
//   - Pins deterministic PNG compilation and fail-closed provenance checks.
// - Description:
//   - Builds redistributable one-tile sprite fixtures in temporary storage.
// - Usage:
//   - Included by the local UI-sprite raster compiler under cfg(test).
// - Defaults:
//   - Missing or mismatched child evidence is rejected.
//

//! Normalized UI-sprite raster compiler tests.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use fbx::adapters::driven::semantic_texture_png::{
    decode_png_bytes, encode_png_bytes,
};
use fbx::domain::texture::semantic::{Rgba8, RgbaImage};

use super::{
    SpriteTileEncoding, compile_ui_sprite_raster,
    publish_complete_ui_sprite_raster_catalog, transaction_paths,
    validate_p3dimage_history, verified_ui_sprite_raster_catalog,
};
use crate::domain::PhaseThreePackageIndex;

static CASE_ID: AtomicUsize = AtomicUsize::new(0);

fn case_dir(label: &str) -> Result<PathBuf, String> {
    let root = std::env::temp_dir().join(format!(
        "shar-ui-raster-{label}-{}-{}",
        std::process::id(),
        CASE_ID.fetch_add(1, Ordering::Relaxed),
    ));
    if root.exists() {
        fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    }
    fs::create_dir_all(root.join("components/sprite"))
        .map_err(|error| error.to_string())?;
    fs::create_dir_all(root.join("components/image"))
        .map_err(|error| error.to_string())?;
    Ok(root)
}

fn write_u32(
    bytes: &mut [u8],
    offset: usize,
    value: u32,
) -> Result<(), String> {
    let end = offset.saturating_add(4);
    bytes
        .get_mut(offset..end)
        .ok_or_else(|| "synthetic DDS write exceeded header".to_owned())?
        .copy_from_slice(&value.to_le_bytes());
    Ok(())
}

fn red_bc1_dds() -> Result<Vec<u8>, String> {
    let mut bytes = vec![0_u8; 136];
    bytes
        .get_mut(..4)
        .ok_or_else(|| "synthetic DDS signature range missing".to_owned())?
        .copy_from_slice(b"DDS ");
    write_u32(&mut bytes, 4, 124)?;
    write_u32(&mut bytes, 8, 0x000a_1007)?;
    write_u32(&mut bytes, 12, 4)?;
    write_u32(&mut bytes, 16, 4)?;
    write_u32(&mut bytes, 20, 8)?;
    write_u32(&mut bytes, 76, 32)?;
    write_u32(&mut bytes, 80, 4)?;
    bytes
        .get_mut(84..88)
        .ok_or_else(|| "synthetic DDS FourCC range missing".to_owned())?
        .copy_from_slice(b"DXT1");
    write_u32(&mut bytes, 108, 0x0040_1008)?;
    bytes
        .get_mut(128..130)
        .ok_or_else(|| "synthetic DDS endpoint range missing".to_owned())?
        .copy_from_slice(&0xf800_u16.to_le_bytes());
    bytes
        .get_mut(130..132)
        .ok_or_else(|| "synthetic DDS endpoint range missing".to_owned())?
        .copy_from_slice(&0_u16.to_le_bytes());
    Ok(bytes)
}

fn write_fixture(root: &Path, declared_count: usize) -> Result<(), String> {
    fs::write(
        root.join("components/sprite/main.json"),
        format!(
            concat!(
                "{{\"image_size\":[4,4],\"image_count\":{},",
                "\"blit_border\":0}}\n"
            ),
            declared_count,
        ),
    )
    .map_err(|error| error.to_string())?;
    fs::write(root.join("components/image/tile.dds"), red_bc1_dds()?)
        .map_err(|error| error.to_string())?;
    fs::write(
        root.join("components.jsonl"),
        concat!(
            "{\"schema\":\"p3d.package.v1\",\"component_count\":2}\n",
            "{\"ordinal\":1,\"parent_ordinal\":0,\"kind\":\"sprite\",",
            "\"path\":\"sprite/main.json\"}\n",
            "{\"ordinal\":2,\"parent_ordinal\":1,\"kind\":\"image\",",
            "\"path\":\"image/tile.dds\"}\n",
        ),
    )
    .map_err(|error| error.to_string())?;
    Ok(())
}

fn p3dimage_history_bytes(run_day: u8) -> Result<Vec<u8>, String> {
    serde_json::to_vec(&serde_json::json!({
        "schema": "history",
        "num_lines": 3,
        "history": [
            "p3dimage version 1.4.0 (with ATG 2.0)\\x00",
            concat!(
                "..\\bin\\p3dimage --ntsc_fix -S -o ",
                "resource\\test.p3d resource\\test.png\\x00"
            ),
            format!("Run at August {run_day}, 2003, 11:21:18 by tester\\x00"),
        ],
    }))
    .map_err(|error| error.to_string())
}

fn source_row_png() -> Result<Vec<u8>, String> {
    let pixels = [10_u8, 20, 30, 40]
        .into_iter()
        .flat_map(|red| std::iter::repeat_n(Rgba8::new(red, 0, 0, 255), 4))
        .collect::<Vec<_>>();
    let image =
        RgbaImage::new(4, 4, pixels).map_err(|error| format!("{error:?}"))?;
    encode_png_bytes(&image).map_err(|error| format!("{error:?}"))
}

fn write_png_history_fixture(root: &Path, run_day: u8) -> Result<(), String> {
    fs::create_dir_all(root.join("components/history"))
        .map_err(|error| error.to_string())?;
    fs::write(
        root.join("components/sprite/main.json"),
        "{\"image_size\":[4,4],\"image_count\":1,\"blit_border\":0}\n",
    )
    .map_err(|error| error.to_string())?;
    fs::write(root.join("components/image/tile.png"), source_row_png()?)
        .map_err(|error| error.to_string())?;
    fs::write(
        root.join("components/history/history.json"),
        p3dimage_history_bytes(run_day)?,
    )
    .map_err(|error| error.to_string())?;
    fs::write(
        root.join("components.jsonl"),
        concat!(
            "{\"schema\":\"p3d.package.v1\",\"component_count\":3}\n",
            "{\"ordinal\":1,\"parent_ordinal\":0,\"kind\":\"history\",",
            "\"path\":\"history/history.json\"}\n",
            "{\"ordinal\":2,\"parent_ordinal\":0,\"kind\":\"sprite\",",
            "\"path\":\"sprite/main.json\"}\n",
            "{\"ordinal\":3,\"parent_ordinal\":2,\"kind\":\"image\",",
            "\"path\":\"image/tile.png\"}\n",
        ),
    )
    .map_err(|error| error.to_string())?;
    Ok(())
}

fn sprite_index(package_root: &Path) -> Result<PhaseThreePackageIndex, String> {
    let extracted_root = package_root
        .parent()
        .ok_or_else(|| "synthetic package has no extracted root".to_owned())?;
    let root_name = extracted_root
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| "synthetic extracted root has no name".to_owned())?;
    let package_name = package_root
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| "synthetic package has no name".to_owned())?;
    let published_root = format!("{root_name}/{package_name}");
    let package_id = published_root.replace('/', "-");
    let canonical = concat!(
        "{\"package_id\":\"ui-test\",",
        "\"package_root\":\"extracted/ui/sprite\",",
        "\"package_category\":\"ui-images\",",
        "\"package_subcategory\":\"ui-images/test\",",
        "\"unit_count\":2,\"text_key_count\":0,",
        "\"unit_ids\":[\"image-a\",\"sprite-a\"],\"world_ids\":[],",
        "\"texture_ids\":[\"image-a\"],\"material_ids\":[],",
        "\"model_ids\":[],\"physics_ids\":[],",
        "\"animation_ids\":[],\"scene_ids\":[],",
        "\"locator_ids\":[],\"camera_ids\":[],",
        "\"light_ids\":[],\"particle_ids\":[],",
        "\"controller_ids\":[],\"audio_ids\":[],",
        "\"movie_ids\":[],\"script_ids\":[],",
        "\"text_ids\":[],\"ui_ids\":[\"sprite-a\"],",
        "\"metadata_ids\":[],\"error_ids\":[],",
        "\"source_unit_ids\":[],\"text_key_ids\":[],",
        "\"members\":[",
        "{\"id\":\"image-a\",\"role\":\"texture\",",
        "\"path\":\"extracted/ui/sprite/components/image/tile.dds\",",
        "\"type\":\"texture\",\"kind\":\"p3d-image\",",
        "\"source_chunk_kind\":\"image\"},",
        "{\"id\":\"sprite-a\",\"role\":\"ui\",",
        "\"path\":\"extracted/ui/sprite/components/sprite/main.json\",",
        "\"type\":\"ui\",\"kind\":\"p3d-sprite\",",
        "\"source_chunk_kind\":\"sprite\"}],\"text_keys\":[]}",
    )
    .replace("extracted/ui/sprite", &published_root)
    .replace("\"ui-test\"", &format!("\"{package_id}\""));
    PhaseThreePackageIndex::from_jsonl(&format!("{canonical}\n"))
        .map_err(|error| error.to_string())
}

#[test]
fn compiles_normalized_sprite_to_deterministic_png() -> Result<(), String> {
    let root = case_dir("compile")?;
    write_fixture(&root, 1)?;
    let first = compile_ui_sprite_raster(
        "ui-test",
        &root,
        SpriteTileEncoding::LegacyDds,
    )
    .map_err(|error| error.to_string())?;
    let second = compile_ui_sprite_raster(
        "ui-test",
        &root,
        SpriteTileEncoding::LegacyDds,
    )
    .map_err(|error| error.to_string())?;
    if first != second {
        return Err("identical normalized sprite produced different artifacts"
            .to_owned());
    }
    if first.filename != "ui-test.png"
        || first.width != 4
        || first.height != 4
        || first.tile_count != 1
    {
        return Err("compiled sprite metadata drifted".to_owned());
    }
    let decoded = decode_png_bytes(&first.png_bytes)
        .map_err(|error| format!("compiled PNG decode failed: {error:?}"))?;
    if decoded.rgba_bytes() != [255, 0, 0, 255].repeat(16) {
        return Err(
            "compiled sprite PNG pixels do not match DDS evidence".to_owned()
        );
    }
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    Ok(())
}

#[test]
fn rejects_declared_image_count_mismatch() -> Result<(), String> {
    let root = case_dir("count-mismatch")?;
    write_fixture(&root, 2)?;
    let result = compile_ui_sprite_raster(
        "ui-test",
        &root,
        SpriteTileEncoding::LegacyDds,
    );
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    if result.is_ok() {
        return Err("sprite image-count mismatch was accepted".to_owned());
    }
    Ok(())
}

#[test]
fn source_revision_changes_with_ordered_dds_evidence() -> Result<(), String> {
    let root = case_dir("revision")?;
    write_fixture(&root, 1)?;
    let before = compile_ui_sprite_raster(
        "ui-test",
        &root,
        SpriteTileEncoding::LegacyDds,
    )
    .map_err(|error| error.to_string())?;
    let mut changed = red_bc1_dds()?;
    let byte = changed
        .get_mut(132)
        .ok_or_else(|| "synthetic DDS index byte missing".to_owned())?;
    *byte = 1;
    fs::write(root.join("components/image/tile.dds"), changed)
        .map_err(|error| error.to_string())?;
    let after = compile_ui_sprite_raster(
        "ui-test",
        &root,
        SpriteTileEncoding::LegacyDds,
    )
    .map_err(|error| error.to_string())?;
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    if before.source_revision == after.source_revision {
        return Err(
            "sprite source revision ignored changed DDS bytes".to_owned()
        );
    }
    Ok(())
}

#[test]
fn validates_p3dimage_history_and_binds_it_to_revision() -> Result<(), String> {
    validate_p3dimage_history(&p3dimage_history_bytes(13)?)
        .map_err(|error| error.to_string())?;
    let mut wrong_executable: serde_json::Value =
        serde_json::from_slice(&p3dimage_history_bytes(13)?)
            .map_err(|error| error.to_string())?;
    let command = wrong_executable
        .get_mut("history")
        .and_then(serde_json::Value::as_array_mut)
        .and_then(|values| values.get_mut(1))
        .ok_or_else(|| "synthetic history command is missing".to_owned())?;
    *command = serde_json::json!(concat!(
        "..\\bin\\badimage --ntsc_fix -S -o ",
        "resource\\test.p3d resource\\test.png\\x00"
    ));
    let wrong_executable = serde_json::to_vec(&wrong_executable)
        .map_err(|error| error.to_string())?;
    if validate_p3dimage_history(&wrong_executable).is_ok() {
        return Err("non-p3dimage executable was accepted as provenance".to_owned());
    }
    let root = case_dir("png-history")?;
    write_png_history_fixture(&root, 13)?;
    let before = compile_ui_sprite_raster(
        "ui-test",
        &root,
        SpriteTileEncoding::P3dImagePng,
    )
    .map_err(|error| error.to_string())?;
    let decoded = decode_png_bytes(&before.png_bytes)
        .map_err(|error| format!("compiled PNG decode failed: {error:?}"))?;
    if decoded.pixel(0, 0).map_err(|error| format!("{error:?}"))?
        != Rgba8::new(10, 0, 0, 255)
        || decoded.pixel(0, 3).map_err(|error| format!("{error:?}"))?
            != Rgba8::new(40, 0, 0, 255)
    {
        return Err("PNG sprite tile rows were vertically inverted".to_owned());
    }
    write_png_history_fixture(&root, 14)?;
    let after = compile_ui_sprite_raster(
        "ui-test",
        &root,
        SpriteTileEncoding::P3dImagePng,
    )
    .map_err(|error| error.to_string())?;
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    if before.source_revision == after.source_revision {
        return Err(
            "UI raster revision ignored changed p3dimage provenance".to_owned()
        );
    }
    let mut invalid: serde_json::Value =
        serde_json::from_slice(&p3dimage_history_bytes(13)?)
            .map_err(|error| error.to_string())?;
    let history = invalid
        .get_mut("history")
        .and_then(serde_json::Value::as_array_mut)
        .and_then(|values| values.first_mut())
        .ok_or_else(|| "synthetic history row is missing".to_owned())?;
    *history = serde_json::json!("other tool");
    let invalid =
        serde_json::to_vec(&invalid).map_err(|error| error.to_string())?;
    if validate_p3dimage_history(&invalid).is_ok() {
        return Err(
            "non-p3dimage history was accepted as provenance".to_owned()
        );
    }
    Ok(())
}

#[test]
fn publication_replaces_complete_catalog_atomically() -> Result<(), String> {
    let package_root = case_dir("publication")?;
    write_fixture(&package_root, 1)?;
    let extracted_root = package_root.parent().ok_or_else(|| {
        "publication package has no extracted root".to_owned()
    })?;
    let index = sprite_index(&package_root)?;
    let expected_id = index
        .packages()
        .first()
        .ok_or_else(|| "publication index is empty".to_owned())?
        .package_id
        .clone();
    let output = package_root.with_file_name(format!(
        "{}-accepted",
        package_root
            .file_name()
            .and_then(|value| value.to_str())
            .ok_or_else(
                || "publication package has no portable name".to_owned()
            )?,
    ));
    if output.exists() {
        fs::remove_dir_all(&output).map_err(|error| error.to_string())?;
    }
    let (staging, backup) =
        transaction_paths(&output).map_err(|error| error.to_string())?;
    if staging.exists() || backup.exists() {
        return Err("publication transaction paths were unexpectedly occupied"
            .to_owned());
    }
    let first = publish_complete_ui_sprite_raster_catalog(
        &index,
        extracted_root,
        &output,
    )
    .map_err(|error| error.to_string())?;
    let first_evidence = first
        .first()
        .ok_or_else(|| "first UI raster publication is empty".to_owned())?;
    let expected_path = format!("ui-raster-assets/rasters/{expected_id}.png");
    if first.len() != 1
        || first_evidence.package_id != expected_id
        || first_evidence.path != expected_path
    {
        return Err(
            "first UI raster publication returned stale evidence".to_owned()
        );
    }
    let mut changed = red_bc1_dds()?;
    let selector = changed
        .get_mut(132)
        .ok_or_else(|| "synthetic DDS selector byte missing".to_owned())?;
    *selector = 1;
    fs::write(package_root.join("components/image/tile.dds"), changed)
        .map_err(|error| error.to_string())?;
    let second = publish_complete_ui_sprite_raster_catalog(
        &index,
        extracted_root,
        &output,
    )
    .map_err(|error| error.to_string())?;
    let second_evidence = second.first().ok_or_else(|| {
        "replacement UI raster publication is empty".to_owned()
    })?;
    if first_evidence.source_revision == second_evidence.source_revision {
        return Err("replacement publication retained a stale source revision"
            .to_owned());
    }
    if staging.exists() || backup.exists() {
        return Err("successful UI raster publication left transaction debris"
            .to_owned());
    }
    let readback = verified_ui_sprite_raster_catalog(&output)
        .map_err(|error| error.to_string())?
        .ok_or_else(|| "accepted UI raster catalog disappeared".to_owned())?;
    if readback != second {
        return Err("accepted UI raster read-back disagrees with publication"
            .to_owned());
    }
    fs::remove_dir_all(&output).map_err(|error| error.to_string())?;
    fs::remove_dir_all(&package_root).map_err(|error| error.to_string())?;
    Ok(())
}

#[test]
fn verifier_rejects_extra_catalog_inventory() -> Result<(), String> {
    let package_root = case_dir("extra-inventory")?;
    write_fixture(&package_root, 1)?;
    let extracted_root = package_root
        .parent()
        .ok_or_else(|| "inventory package has no extracted root".to_owned())?;
    let index = sprite_index(&package_root)?;
    let output = package_root.with_file_name(format!(
        "{}-accepted",
        package_root
            .file_name()
            .and_then(|value| value.to_str())
            .ok_or_else(
                || "inventory package has no portable name".to_owned()
            )?,
    ));
    if output.exists() {
        fs::remove_dir_all(&output).map_err(|error| error.to_string())?;
    }
    let _published = publish_complete_ui_sprite_raster_catalog(
        &index,
        extracted_root,
        &output,
    )
    .map_err(|error| error.to_string())?;
    fs::write(output.join("rasters/unclaimed.png"), b"not catalogued")
        .map_err(|error| error.to_string())?;
    let result = verified_ui_sprite_raster_catalog(&output);
    fs::remove_dir_all(&output).map_err(|error| error.to_string())?;
    fs::remove_dir_all(&package_root).map_err(|error| error.to_string())?;
    if result.is_ok() {
        return Err("UI raster verifier accepted extra inventory".to_owned());
    }
    Ok(())
}

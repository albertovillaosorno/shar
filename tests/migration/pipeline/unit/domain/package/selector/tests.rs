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
//   - Tests unit tests.
// - Must-Not:
//   - Own production behavior or broaden the tested API surface.
// - Allows:
//   - Private test fixtures and assertions for the owning source module.
// - Split-When:
//   - Split when an independent fixture family gains separate ownership.
// - Merge-When:
//   - Merge when another test module owns the identical evidence.
// - Summary:
//   - Tests unit tests.
// - Description:
//   - Preserves unit-test access through a test-only path module.
// - Usage:
//   - Included only by the owning source module under cfg(test).
// - Defaults:
//   - Test setup and assertions fail explicitly.
//

//! Tests unit tests.

use super::{
    PhaseThreePackageIndex, PhaseThreePackageSelector, normalize_selector_token,
};

#[test]
fn rejects_whitespace_padded_compact_selectors() -> Result<(), String> {
    for raw in [
        "package: ",
        "subcategory:\t",
        "prefix:  ",
        "prop:\r",
        "vehicle:\n",
        "character:\t ",
        "package: pkg-car",
        "subcategory:cars/example ",
        "prefix:\tcars/",
        "prop:wrench ",
        "vehicle: homer-v",
        "character:homer\t",
    ] {
        if PhaseThreePackageSelector::parse(raw).is_ok() {
            return Err(format!(
                "whitespace-padded selector must be rejected: {raw:?}",
            ));
        }
    }
    Ok(())
}

#[test]
fn rejects_empty_or_multi_colon_selectors() -> Result<(), String> {
    for raw in [
        "package:",
        "subcategory:",
        "prefix:",
        "prop:",
        "vehicle:",
        "character:",
        "prop:wrench:extra",
    ] {
        if PhaseThreePackageSelector::parse(raw).is_ok() {
            return Err(format!("malformed selector must be rejected: {raw}"));
        }
    }
    Ok(())
}

#[test]
fn normalizes_tokens_like_generated_package_ids() -> Result<(), String> {
    for (input, expected) in [
        (" Homer V.2 ", "homer-v-2"),
        ("SNAKE_CASE", "snake-case"),
        ("café", "caf-"),
    ] {
        let actual = normalize_selector_token(input);
        if actual != expected {
            return Err(format!(
                "selector token {input:?} normalized to {actual:?}"
            ));
        }
    }
    Ok(())
}

#[test]
fn rejects_empty_programmatic_prefixes() -> Result<(), String> {
    let row = concat!(
        "{\"package_id\":\"pkg-car\",",
        "\"package_root\":\"pkg-car\",",
        "\"package_category\":\"cars\",",
        "\"package_subcategory\":",
        "\"cars/character-rigs/homer-v\",",
        "\"unit_count\":1,\"text_key_count\":0,",
        "\"unit_ids\":[\"model-a\"],",
        "\"world_ids\":[],\"texture_ids\":[],",
        "\"material_ids\":[],",
        "\"model_ids\":[\"model-a\"],",
        "\"physics_ids\":[],\"animation_ids\":[],",
        "\"scene_ids\":[],\"locator_ids\":[],",
        "\"camera_ids\":[],\"light_ids\":[],",
        "\"particle_ids\":[],\"controller_ids\":[],",
        "\"audio_ids\":[],\"movie_ids\":[],",
        "\"script_ids\":[],\"text_ids\":[],",
        "\"ui_ids\":[],\"metadata_ids\":[],",
        "\"error_ids\":[],\"source_unit_ids\":[],",
        "\"text_key_ids\":[],",
        "\"members\":[{",
        "\"id\":\"model-a\",",
        "\"role\":\"model\",",
        "\"path\":\"extracted/model.p3d\",",
        "\"type\":\"model\",",
        "\"kind\":\"mesh\",",
        "\"source_chunk_kind\":\"mesh\"}],",
        "\"text_keys\":[]}"
    );
    let index = PhaseThreePackageIndex::from_jsonl(row)
        .map_err(|error| error.to_string())?;
    if PhaseThreePackageSelector::subcategory_prefix("")
        .resolve(&index)
        .is_ok()
    {
        return Err(
            "empty programmatic prefix must not select a package".to_owned()
        );
    }
    Ok(())
}

#[test]
fn vehicle_selector_rejects_intermediate_subcategory_segments()
-> Result<(), String> {
    let row = concat!(
        "{\"package_id\":\"pkg-car\",",
        "\"package_root\":\"pkg-car\",",
        "\"package_category\":\"cars\",",
        "\"package_subcategory\":",
        "\"cars/character-rigs/homer-v\",",
        "\"unit_count\":1,\"text_key_count\":0,",
        "\"unit_ids\":[\"model-a\"],",
        "\"world_ids\":[],\"texture_ids\":[],",
        "\"material_ids\":[],",
        "\"model_ids\":[\"model-a\"],",
        "\"physics_ids\":[],\"animation_ids\":[],",
        "\"scene_ids\":[],\"locator_ids\":[],",
        "\"camera_ids\":[],\"light_ids\":[],",
        "\"particle_ids\":[],\"controller_ids\":[],",
        "\"audio_ids\":[],\"movie_ids\":[],",
        "\"script_ids\":[],\"text_ids\":[],",
        "\"ui_ids\":[],\"metadata_ids\":[],",
        "\"error_ids\":[],\"source_unit_ids\":[],",
        "\"text_key_ids\":[],",
        "\"members\":[{",
        "\"id\":\"model-a\",",
        "\"role\":\"model\",",
        "\"path\":\"extracted/model.p3d\",",
        "\"type\":\"model\",",
        "\"kind\":\"mesh\",",
        "\"source_chunk_kind\":\"mesh\"}],",
        "\"text_keys\":[]}"
    );
    let index = PhaseThreePackageIndex::from_jsonl(row)
        .map_err(|error| error.to_string())?;
    if PhaseThreePackageSelector::vehicle("character-rigs")
        .resolve(&index)
        .is_ok()
    {
        return Err(
            "vehicle selectors must match the terminal model token".to_owned()
        );
    }
    Ok(())
}

#[test]
fn rejects_invalid_programmatic_selector_values() -> Result<(), String> {
    let row = concat!(
        "{\"package_id\":\"pkg-car\",",
        "\"package_root\":\"pkg-car\",",
        "\"package_category\":\"cars\",",
        "\"package_subcategory\":",
        "\"cars/character-rigs/homer-v\",",
        "\"unit_count\":1,\"text_key_count\":0,",
        "\"unit_ids\":[\"model-a\"],",
        "\"world_ids\":[],\"texture_ids\":[],",
        "\"material_ids\":[],",
        "\"model_ids\":[\"model-a\"],",
        "\"physics_ids\":[],\"animation_ids\":[],",
        "\"scene_ids\":[],\"locator_ids\":[],",
        "\"camera_ids\":[],\"light_ids\":[],",
        "\"particle_ids\":[],\"controller_ids\":[],",
        "\"audio_ids\":[],\"movie_ids\":[],",
        "\"script_ids\":[],\"text_ids\":[],",
        "\"ui_ids\":[],\"metadata_ids\":[],",
        "\"error_ids\":[],\"source_unit_ids\":[],",
        "\"text_key_ids\":[],",
        "\"members\":[{",
        "\"id\":\"model-a\",",
        "\"role\":\"model\",",
        "\"path\":\"extracted/model.p3d\",",
        "\"type\":\"model\",",
        "\"kind\":\"mesh\",",
        "\"source_chunk_kind\":\"mesh\"}],",
        "\"text_keys\":[]}"
    );
    let index = PhaseThreePackageIndex::from_jsonl(row)
        .map_err(|error| error.to_string())?;
    for invalid in [" homer-v ", "homer\u{0}v", "homer:v"] {
        if PhaseThreePackageSelector::vehicle(invalid)
            .resolve(&index)
            .is_ok()
        {
            return Err(format!(
                "invalid programmatic selector must fail: {invalid:?}"
            ));
        }
    }
    Ok(())
}

#[test]
fn parses_compact_selectors() -> Result<(), String> {
    if PhaseThreePackageSelector::parse("prop:wrench")
        .map_err(|error| error.to_string())?
        != PhaseThreePackageSelector::prop("wrench")
    {
        return Err("prop selector should parse".to_owned());
    }
    if PhaseThreePackageSelector::parse("package:pkg")
        .map_err(|error| error.to_string())?
        != PhaseThreePackageSelector::package_id("pkg")
    {
        return Err("package selector should parse".to_owned());
    }
    Ok(())
}

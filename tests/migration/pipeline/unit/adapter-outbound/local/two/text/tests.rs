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
//   - Tests test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Tests test module.
// - Description:
//   - Implements the declared test module responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Tests test module.

use super::classification::classify_text_key;

#[test]
fn classifies_level_mission_text_without_file_routes() {
    assert_eq!(
        classify_text_key("MISSION_TITLE_L4_M7"),
        "language/text/missions/level-04/title"
    );
    assert_eq!(
        classify_text_key("MISSION_INFO_L2_M10"),
        "language/text/missions/level-02/info"
    );
}

#[test]
fn classifies_global_objectives_without_guessing_levels() {
    assert_eq!(
        classify_text_key("MISSION_OBJECTIVE_42"),
        "language/text/missions/objective-lines"
    );
}

#[test]
fn classifies_vehicle_and_costume_keys() {
    assert_eq!(classify_text_key("SEDANA"), "language/text/vehicles");
    assert_eq!(
        classify_text_key("H_DONUT"),
        "language/text/characters/homer"
    );
}

#[test]
fn parses_source_text_table_keys_after_metadata_header() -> Result<(), String> {
    let json = serde_json::json!({
        "schema": "shar-schoenwald.straggler.text-bible.v1",
        "source_extension": "txt",
        "language_channel": "source-text",
        "entry_count": 5,
        "source_entries": [
            "Languages\tEFGIS",
            "Screen\tPHRASE TABLE",
            "TERM\tCRITICAL",
            "INGAME\tMISSION_OBJECTIVE_00\tignored",
            "INGAME\tINGAME_MESSAGE_00\tignored"
        ]
    });
    let keys = super::source::parse_source_text_keys(&json.to_string())
        .map_err(|error| error.to_string())?;
    assert_eq!(
        keys,
        ["MISSION_OBJECTIVE_00", "INGAME_MESSAGE_00"]
    );
    Ok(())
}

#[test]
fn source_text_table_rejects_duplicate_key_identity() -> Result<(), String> {
    let json = serde_json::json!({
        "schema": "shar-schoenwald.straggler.text-bible.v1",
        "source_extension": "txt",
        "language_channel": "source-text",
        "entry_count": 5,
        "source_entries": [
            "Languages\tEFGIS",
            "Screen\tPHRASE TABLE",
            "TERM\tCRITICAL",
            "INGAME\tDUPLICATE",
            "FRONTEND\tDUPLICATE"
        ]
    });
    let result = super::source::parse_source_text_keys(&json.to_string());
    let Err(error) = result else {
        return Err("duplicate source-text key unexpectedly passed".to_owned());
    };
    if error.to_string().contains("duplicated") {
        Ok(())
    } else {
        Err(format!("unexpected duplicate-key error: {error}"))
    }
}

#[test]
fn non_source_text_channel_produces_no_derived_keys() -> Result<(), String> {
    let json = serde_json::json!({
        "schema": "shar-schoenwald.straggler.text-bible.v1",
        "source_extension": "e",
        "language_channel": "english",
        "entry_count": 1,
        "source_entries": ["0020 value"]
    });
    let keys = super::source::parse_source_text_keys(&json.to_string())
        .map_err(|error| error.to_string())?;
    if keys.is_empty() {
        Ok(())
    } else {
        Err("non-source TextBible channel exposed derived keys".to_owned())
    }
}

#[test]
fn derives_source_text_keys_into_classified_packages() -> Result<(), String> {
    let root = std::env::temp_dir().join(format!(
        "pipeline-source-text-{}",
        std::process::id()
    ));
    std::fs::create_dir_all(&root).map_err(|error| error.to_string())?;
    let path = root.join("table.json");
    let json = serde_json::json!({
        "schema": "shar-schoenwald.straggler.text-bible.v1",
        "source_extension": "txt",
        "language_channel": "source-text",
        "entry_count": 5,
        "source_entries": [
            "Languages\tEFGIS",
            "Screen\tPHRASE TABLE",
            "TERM\tCRITICAL",
            "INGAME\tMISSION_OBJECTIVE_00\tignored",
            "INGAME\tINGAME_MESSAGE_00\tignored"
        ]
    });
    std::fs::write(&path, json.to_string())
        .map_err(|error| error.to_string())?;
    let result = super::derive_text_packages(
        &root,
        "source-table",
        "extracted/table.json",
        "localization-table",
    )
    .map_err(|error| error.to_string());
    std::fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    let packages = result?;
    let objective = packages
        .iter()
        .find(|package| {
            package.subcategory == "language/text/missions/objective-lines"
        })
        .ok_or_else(|| "objective source-text package disappeared".to_owned())?;
    let [key] = objective.keys.as_slice() else {
        return Err("objective source-text key count changed".to_owned());
    };
    if key.key != "MISSION_OBJECTIVE_00" {
        return Err("objective source-text key binding changed".to_owned());
    }
    if packages
        .iter()
        .flat_map(|package| package.keys.iter())
        .any(|key| key.key == "Languages")
    {
        return Err(
            "source-text metadata row leaked into derived keys".to_owned(),
        );
    }
    Ok(())
}

#[test]
fn flat_source_text_document_produces_no_derived_keys() -> Result<(), String> {
    let json = serde_json::json!({
        "schema": "shar-schoenwald.straggler.text-bible.v1",
        "source_extension": "txt",
        "language_channel": "source-text",
        "entry_count": 2,
        "source_entries": [
            "Flat authored text block",
            "Another authored text block"
        ]
    });
    let keys = super::source::parse_source_text_keys(&json.to_string())
        .map_err(|error| error.to_string())?;
    if keys.is_empty() {
        Ok(())
    } else {
        Err("flat source text exposed derived table keys".to_owned())
    }
}

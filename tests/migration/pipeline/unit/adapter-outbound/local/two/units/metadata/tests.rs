// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Minor-unit metadata schema regression tests.
// - Must-Not:
//   - Read proprietary source data or mutate accepted generated evidence.
// - Allows:
//   - Synthetic bounded JSON prefixes.
// - Split-When:
//   - Split when metadata schema extraction gains an independent lifecycle.
// - Merge-When:
//   - Merge when another test module owns the identical evidence.
// - Summary:
//   - Exact normalized schema metadata tests.
// - Description:
//   - Proves manifest metadata preserves producer schema revisions.
// - Usage:
//   - Included only by the owning metadata module under cfg(test).
// - Defaults:
//   - Missing or mismatched schema evidence fails closed.
//

//! Exact normalized schema metadata tests.

use std::fs;
use std::path::PathBuf;

use super::{classify_minor_unit, json_string_field_prefix, straggler_schema};

#[test]
fn bounded_schema_reader_preserves_exact_revision() -> Result<(), String> {
    let text = r#"{"schema" : "shar-schoenwald.straggler.mission-script.v3","semantic_family":"mission-script"}"#;
    if json_string_field_prefix(text, "schema")
        != Some("shar-schoenwald.straggler.mission-script.v3")
    {
        return Err("bounded schema reader changed canonical value".to_owned());
    }
    let schema = straggler_schema(text, "straggler.mission-script")
        .map_err(|error| error.to_string())?;
    if schema != "shar-schoenwald.straggler.mission-script.v3" {
        return Err("mission schema revision was not preserved".to_owned());
    }
    Ok(())
}

#[test]
fn straggler_schema_rejects_missing_mismatched_or_invalid_revision() {
    for (text, family) in [
        (r#"{"semantic_family":"mission-script"}"#, "straggler.mission-script"),
        (r#"{"schema":"shar-schoenwald.straggler.config-script.v2"}"#, "straggler.mission-script"),
        (r#"{"schema":"shar-schoenwald.straggler.mission-script.vx"}"#, "straggler.mission-script"),
    ] {
        assert!(straggler_schema(text, family).is_err());
    }
}

#[test]
fn classifier_preserves_physical_mission_schema_revision() -> Result<(), String> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join(".temp")
        .join(format!("metadata-schema-{}", std::process::id()));
    if root.exists() {
        fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    }
    let path = root.join("game/mission.json");
    fs::create_dir_all(path.parent().ok_or("missing synthetic parent")?)
        .map_err(|error| error.to_string())?;
    fs::write(
        &path,
        br#"{"schema":"shar-schoenwald.straggler.mission-script.v3","semantic_family":"mission-script"}"#,
    )
    .map_err(|error| error.to_string())?;
    let result = classify_minor_unit(&root, "extracted/game/mission.json", "json")
        .map_err(|error| error.to_string());
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    let metadata = result?;
    if metadata.schema != "shar-schoenwald.straggler.mission-script.v3"
        || metadata.kind != "mission-script"
        || metadata.origin != "game-straggler-normalize"
        || metadata.unreal_import_relation != "compose-into-asset"
    {
        return Err(format!(
            "classifier changed normalized mission schema: {:?}",
            (
                metadata.schema,
                metadata.kind,
                metadata.origin,
                metadata.unreal_import_relation
            )
        ));
    }
    Ok(())
}

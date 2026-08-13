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

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use super::{
    is_pipeline_run_report, obfuscated_route, parse_component_line,
    read_number_field, should_skip_local_game_file, source_object_key,
    write_manifest_minor_units,
};

/// Distinguishes concurrent synthetic manifest cases within one process.
static CASE_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Create one collision-resistant synthetic case root.
fn case_root(label: &str) -> PathBuf {
    let mut repository = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    while !repository.join("TODO.md").is_file() {
        assert!(repository.pop(), "repository root must exist");
    }
    repository.join(".temp/tests").join(format!(
        "pipeline-manifest-{label}-{}-{}",
        std::process::id(),
        CASE_COUNTER.fetch_add(1, Ordering::Relaxed,),
    ))
}

/// Write one synthetic extracted file and its parent directories.
fn write_sample(
    root: &Path,
    relative: &str,
    contents: &[u8],
) -> Result<(), String> {
    let path = root.join(relative);
    let parent = path
        .parent()
        .ok_or_else(|| String::from("synthetic sample must have a parent"))?;
    fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    fs::write(path, contents).map_err(|error| error.to_string())
}


fn write_game_manifest_ledger(
    game_root: &Path,
    extracted_root: &Path,
) -> Result<(), String> {
    let manifest = format!(
        "{}
{}
",
        game_manifest::kind_taxonomy_jsonl(),
        r#"{"dir":"","ext":"png","min":0,"kind":"generated_artifact"}"#,
    );
    write_sample(
        game_root,
        game_manifest::MANIFEST_FILE_NAME,
        manifest.as_bytes(),
    )?;
    write_sample(game_root, "README.rtf", b"fixture")?;
    write_sample(game_root, "Simpsons.exe", b"fixture")?;
    write_sample(game_root, "Simpsons.ico", b"fixture")?;
    write_sample(game_root, "dialog.rcf", b"fixture")?;
    write_sample(
        game_root,
        "art/frontend/scrooby2/resource/txtbible/srr2.E",
        b"fixture",
    )?;
    write_sample(
        game_root,
        "art/frontend/scrooby2/resource/txtbible/srr2.txt",
        b"fixture",
    )?;
    write_sample(extracted_root, "dialog/fixture.wav", b"fixture")
}

#[test]
fn game_manifest_directory_is_repository_metadata() {
    for path in ["manifest/game.jsonl", "MANIFEST/UNREAL.JSONL"] {
        assert!(should_skip_local_game_file(Path::new(path), "jsonl"));
    }
    assert!(!should_skip_local_game_file(
        Path::new("scripts/manifest.jsonl"),
        "jsonl",
    ));
}


#[test]
fn pipeline_report_exclusion_matches_only_the_extracted_root() {
    let root = Path::new("extracted");
    assert!(is_pipeline_run_report(
        root,
        Path::new("extracted/pipeline-report.jsonl"),
    ));
    assert!(!is_pipeline_run_report(
        root,
        Path::new("extracted/art/pipeline-report.jsonl"),
    ));
    assert!(!is_pipeline_run_report(
        root,
        Path::new("extracted/pipeline-report.json"),
    ));
}


#[test]
fn manifest_rejects_unprepared_game_root_before_straggler_mutation() {
    assert_eq!(run_unprepared_game_root_case(), Ok(()));
}

fn run_unprepared_game_root_case() -> Result<(), String> {
    let case = case_root("unprepared-game-root");
    let game_root = case.join("game");
    let extracted_root = case.join("extracted");
    write_sample(
        &game_root,
        "scripts/sample.mfk",
        b"SelectMission(\"m1\");",
    )?;
    write_sample(&extracted_root, "game/accepted.txt", b"accepted")?;

    let error = match write_manifest_minor_units(&game_root, &extracted_root) {
        Ok(_report) => return Err("unprepared game root must fail".to_owned()),
        Err(error) => error.to_string(),
    };
    let accepted = fs::read(extracted_root.join("game/accepted.txt"))
        .map_err(|error| error.to_string())?;
    fs::remove_dir_all(&case).map_err(|error| error.to_string())?;
    if !error.contains("prepared game manifest validation") || accepted != b"accepted" {
        return Err("unprepared root did not fail before mutation".to_owned());
    }
    Ok(())
}

#[test]
fn manifest_rejects_prepared_root_with_requirement_shortfall() {
    assert_eq!(run_prepared_root_shortfall_case(), Ok(()));
}

fn run_prepared_root_shortfall_case() -> Result<(), String> {
    let case = case_root("prepared-shortfall");
    let game_root = case.join("game");
    let extracted_root = case.join("extracted");
    let manifest = format!(
        "{}
{}
{}
",
        game_manifest::kind_taxonomy_jsonl(),
        r#"{"dir":"","ext":"mfk","min":2,"kind":"script"}"#,
        r#"{"dir":"","ext":"png","min":0,"kind":"generated_artifact"}"#,
    );
    write_sample(
        &game_root,
        game_manifest::MANIFEST_FILE_NAME,
        manifest.as_bytes(),
    )?;
    write_sample(&game_root, "sample.mfk", b"SelectMission(\"m1\");")?;
    write_sample(&extracted_root, "game/accepted.txt", b"accepted")?;

    let error = match write_manifest_minor_units(&game_root, &extracted_root) {
        Ok(_report) => return Err("manifest shortfall must fail".to_owned()),
        Err(error) => error.to_string(),
    };
    let accepted = fs::read(extracted_root.join("game/accepted.txt"))
        .map_err(|error| error.to_string())?;
    fs::remove_dir_all(&case).map_err(|error| error.to_string())?;
    if !error.contains("requirement shortfall") || accepted != b"accepted" {
        return Err("manifest shortfall did not fail before mutation".to_owned());
    }
    Ok(())
}

#[test]
fn manifest_is_independent_of_run_report_presence() {
    assert_eq!(run_report_presence_case(), Ok(()));
}

/// Run the synthetic report-presence determinism case.
fn run_report_presence_case() -> Result<(), String> {
    let case = case_root("report");
    let game_root = case.join("game");
    let extracted_root = case.join("extracted");
    fs::create_dir_all(&game_root).map_err(|error| error.to_string())?;
    write_game_manifest_ledger(&game_root, &extracted_root)?;
    write_sample(&extracted_root, "art/sample.json", b"{}")?;

    let first_report = write_manifest_minor_units(&game_root, &extracted_root)
        .map_err(|error| error.to_string())?;
    let manifest_path =
        extracted_root.join("minor-unit").join("manifest.jsonl");
    let before = fs::read_to_string(&manifest_path)
        .map_err(|error| error.to_string())?;

    write_sample(
        &extracted_root,
        "pipeline-report.jsonl",
        br#"{"stage":"manifest","files":1}
"#,
    )?;
    let second_report = write_manifest_minor_units(&game_root, &extracted_root)
        .map_err(|error| error.to_string())?;
    let after = fs::read_to_string(&manifest_path)
        .map_err(|error| error.to_string())?;
    fs::remove_dir_all(&case).map_err(|error| error.to_string())?;

    if before.contains("pipeline-report") {
        return Err(String::from("manifest covered pipeline run telemetry"));
    }
    if before != after {
        return Err(String::from("manifest changed after report creation"));
    }
    if first_report.files != second_report.files {
        return Err(String::from(
            "manifest unit count changed after report creation",
        ));
    }
    Ok(())
}

#[test]
fn manifest_ignores_non_asset_installation_files() {
    assert_eq!(run_non_asset_installation_file_case(), Ok(()));
}

/// Run one manifest case containing non-asset installation files.
fn run_non_asset_installation_file_case() -> Result<(), String> {
    let case = case_root("non-asset-installation-files");
    let game_root = case.join("game");
    let extracted_root = case.join("extracted");
    write_game_manifest_ledger(&game_root, &extracted_root)?;
    write_sample(&game_root, "copy/disc_one.iso", b"disc image")?;
    write_sample(&game_root, "Simpsons.exe", b"executable")?;
    write_sample(&game_root, "binkw32.dll", b"runtime library")?;
    write_sample(&game_root, "Simpsons.ico", b"application icon")?;
    write_sample(&game_root, "scripts/sample.mfk", b"sample script")?;
    fs::create_dir_all(&extracted_root).map_err(|error| error.to_string())?;

    let _report = write_manifest_minor_units(&game_root, &extracted_root)
        .map_err(|error| error.to_string())?;
    let manifest = fs::read_to_string(
        extracted_root.join("minor-unit").join("manifest.jsonl"),
    )
    .map_err(|error| error.to_string())?;
    fs::remove_dir_all(&case).map_err(|error| error.to_string())?;

    for extension in ["dll", "exe", "ico", "iso"] {
        let field = format!("\"file_extension\":\"{extension}\"");
        if manifest.contains(&field) {
            return Err(format!(
                "non-asset installation file entered the manifest: {extension}"
            ));
        }
    }
    if !manifest.contains("game/scripts/sample.mfk") {
        return Err(String::from("legitimate game input was omitted"));
    }
    Ok(())
}

#[test]
fn manifest_is_independent_of_file_creation_order() {
    assert_eq!(run_creation_order_case(), Ok(()));
}

/// Run the synthetic creation-order determinism case.
fn run_creation_order_case() -> Result<(), String> {
    let case = case_root("order");
    let game_root = case.join("game");
    let first_root = case.join("first");
    let second_root = case.join("second");
    fs::create_dir_all(&game_root).map_err(|error| error.to_string())?;
    write_game_manifest_ledger(&game_root, &first_root)?;
    write_sample(&second_root, "dialog/fixture.wav", b"fixture")?;
    write_sample(&first_root, "art/z.json", br#"{"value":2}"#)?;
    write_sample(&first_root, "art/a.json", br#"{"value":1}"#)?;
    write_sample(&second_root, "art/a.json", br#"{"value":1}"#)?;
    write_sample(&second_root, "art/z.json", br#"{"value":2}"#)?;

    let _first_report = write_manifest_minor_units(&game_root, &first_root)
        .map_err(|error| error.to_string())?;
    let _second_report = write_manifest_minor_units(&game_root, &second_root)
        .map_err(|error| error.to_string())?;
    let first = fs::read_to_string(
        first_root.join("minor-unit").join("manifest.jsonl"),
    )
    .map_err(|error| error.to_string())?;
    let second = fs::read_to_string(
        second_root.join("minor-unit").join("manifest.jsonl"),
    )
    .map_err(|error| error.to_string())?;
    fs::remove_dir_all(&case).map_err(|error| error.to_string())?;

    if first != second {
        return Err(String::from("manifest changed with file creation order"));
    }
    Ok(())
}

#[test]
fn source_object_key_groups_package_components() {
    assert_eq!(
        source_object_key("packages/sample/components/mesh/a.json"),
        "packages/sample"
    );
    assert_eq!(
        source_object_key("packages/scripts/level.mfk"),
        "packages/scripts/level.mfk"
    );
}

#[test]
fn obfuscated_route_hides_names_and_counts_per_folder() {
    let mut index = BTreeMap::new();
    let first = obfuscated_route(
        "packages/sample/components/texture/circle.png",
        "png",
        &mut index,
    );
    let second = obfuscated_route(
        "packages/sample/components/texture/nostril.png",
        "png",
        &mut index,
    );
    assert_eq!(first, "ps/se/cs/te/#1.png");
    assert_eq!(second, "ps/se/cs/te/#2.png");
    assert!(!first.contains("wiggum"));
    assert!(!first.contains("circle"));
}

#[test]
fn obfuscated_route_is_unique_across_folders_sharing_a_shape() {
    let mut index = BTreeMap::new();
    // "aro" and "ado" both obfuscate to "ao", so a per-exact-folder counter
    // would give both "#1"; the per-shape counter must keep them distinct
    // so the route stays a unique identity seed.
    let first = obfuscated_route("game/aro/x.json", "json", &mut index);
    let second = obfuscated_route("game/ado/y.json", "json", &mut index);
    assert_ne!(first, second);
}

#[test]
fn read_number_field_reads_bare_integers_only() {
    assert_eq!(
        read_number_field("{\"ordinal\":42,\"x\":1}", "ordinal"),
        Some("42".to_owned())
    );
    assert_eq!(read_number_field("{\"a\":1}", "ordinal"), None);
}

#[test]
fn parse_component_line_maps_provenance() {
    let line = [
        "{\"ordinal\":7",
        "\"name\":\"x\"",
        "\"path\":\"texture/c.png\"",
        "\"kind\":\"texture\"",
        "\"payload_format\":\"image/png\"",
        "\"schema_ref\":\"texture\"",
        "\"recovery_status\":\"recovered_embedded_image_payload\"}",
    ]
    .join(",");
    let parsed = parse_component_line(&line);
    assert!(parsed.is_some());
    if let Some((component_rel, provenance)) = parsed {
        assert_eq!(component_rel, "texture/c.png");
        assert_eq!(provenance.chunk_kind, "texture");
        assert_eq!(provenance.chunk_ordinal, "7");
        assert_eq!(provenance.recovery_status, "fully-decoded");
    }
}

#[test]
fn parse_component_line_skips_header() {
    assert!(
        parse_component_line(
            "{\"schema\":\"p3d.package.v1\",\"chunk_count\":3}"
        )
        .is_none()
    );
}

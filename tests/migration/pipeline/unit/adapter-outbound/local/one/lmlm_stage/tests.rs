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
//   - Optional LMLM stage unit tests.
// - Must-Not:
//   - Own production behavior or require an installed language mod.
// - Allows:
//   - Isolated filesystem fixtures for optional-package behavior.
// - Split-When:
//   - Split when present-package extraction needs independent fixtures.
// - Merge-When:
//   - Merge when another module owns the identical evidence.
// - Summary:
//   - Optional LMLM stage unit tests.
// - Description:
//   - Proves an English-only game creates no synthetic LMLM output.
// - Usage:
//   - Included only by the owning source module under cfg(test).
// - Defaults:
//   - Missing optional packages succeed with zero output.
//

//! Optional LMLM stage unit tests.

use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

use lmlm::FileEntry;

use super::{extract_lmlm, preview_optional_mods};
use super::preview::create_preview_work_root;
use super::optional_mods::{
    OptionalModRole, apply_remaster, discover_optional_mods,
    existing_file_index, is_latino_audio_path, is_latino_movie_path,
};

#[test]
fn missing_optional_lmlm_creates_no_output() -> Result<(), String> {
    let case = temp_root("missing");
    if case.exists() {
        fs::remove_dir_all(&case).map_err(|error| error.to_string())?;
    }
    let game_root = case.join("game");
    let extracted_root = case.join("extracted");
    let stale_output = extracted_root.join("lmlm");
    fs::create_dir_all(&game_root).map_err(|error| error.to_string())?;
    fs::create_dir_all(&stale_output).map_err(|error| error.to_string())?;
    fs::write(stale_output.join("manifest.json"), b"stale")
        .map_err(|error| error.to_string())?;

    let report = extract_lmlm(&game_root, &extracted_root)
        .map_err(|error| error.to_string())?;

    if report.name != "lmlm" || report.files != 0 || report.bytes != 0 {
        return Err(format!(
            "unexpected optional-stage report: name={} files={} bytes={}",
            report.name, report.files, report.bytes
        ));
    }
    if report.note != "no supported optional LMLM packages present" {
        return Err(format!("unexpected optional-stage note: {}", report.note));
    }
    if stale_output.exists() {
        return Err(String::from(
            "missing optional LMLM package left synthetic output behind",
        ));
    }
    fs::remove_dir_all(&case).map_err(|error| error.to_string())?;
    Ok(())
}

#[test]
fn missing_optional_lmlm_preview_is_read_only() -> Result<(), String> {
    let case = temp_root("preview-missing");
    if case.exists() {
        fs::remove_dir_all(&case).map_err(|error| error.to_string())?;
    }
    let game_root = case.join("game");
    let extracted_root = case.join("extracted");
    fs::create_dir_all(&game_root).map_err(|error| error.to_string())?;
    fs::create_dir_all(&extracted_root).map_err(|error| error.to_string())?;
    let sentinel = extracted_root.join("sentinel.txt");
    fs::write(&sentinel, b"unchanged").map_err(|error| error.to_string())?;

    let preview = preview_optional_mods(&game_root, &extracted_root)
        .map_err(|error| error.to_string())?;
    if preview.package_count() != 0
        || preview.write_count() != 0
        || preview.skip_count() != 0
        || preview.normalized_bytes() != 0
    {
        return Err("empty preview reported package changes".to_owned());
    }
    let document: serde_json::Value = serde_json::from_str(preview.json())
        .map_err(|error| error.to_string())?;
    if document.get("schema").and_then(serde_json::Value::as_str)
        != Some("shar-schoenwald.optional-mod-preview.v1")
        || document.get("dry_run").and_then(serde_json::Value::as_bool)
            != Some(true)
    {
        return Err("empty preview did not use the canonical schema".to_owned());
    }
    if fs::read(&sentinel).map_err(|error| error.to_string())? != b"unchanged"
        || extracted_root.join("lmlm").exists()
    {
        return Err("optional-mod preview changed extraction output".to_owned());
    }
    fs::remove_dir_all(&case).map_err(|error| error.to_string())?;
    Ok(())
}

#[test]
fn corrupt_optional_package_diagnostics_use_public_aliases() -> Result<(), String> {
    let case = temp_root("corrupt-diagnostic");
    if case.exists() {
        fs::remove_dir_all(&case).map_err(|error| error.to_string())?;
    }
    let game_root = case.join("game");
    let extracted_root = case.join("extracted");
    let mods_root = game_root.join("mods");
    fs::create_dir_all(&mods_root).map_err(|error| error.to_string())?;
    fs::create_dir_all(&extracted_root).map_err(|error| error.to_string())?;
    fs::write(mods_root.join("m.lmlm"), b"not an archive")
        .map_err(|error| error.to_string())?;
    let private_fragment = case
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| "fixture path lacks a portable name".to_owned())?;

    let extract_error = match extract_lmlm(&game_root, &extracted_root) {
        Ok(_report) => return Err("corrupt extraction package succeeded".to_owned()),
        Err(error) => error.to_string(),
    };
    let preview_error = match preview_optional_mods(&game_root, &extracted_root) {
        Ok(_preview) => return Err("corrupt preview package succeeded".to_owned()),
        Err(error) => error.to_string(),
    };
    for error in [&extract_error, &preview_error] {
        if !error.contains("m.lmlm") || error.contains(private_fragment) {
            return Err(format!("package diagnostic is not public-safe: {error}"));
        }
    }
    if extracted_root.join("lmlm").exists() {
        return Err("corrupt package changed extraction output".to_owned());
    }
    fs::remove_dir_all(&case).map_err(|error| error.to_string())?;
    Ok(())
}

#[test]
fn preview_workspaces_are_unique_and_independently_owned() -> Result<(), String> {
    let first = create_preview_work_root().map_err(|error| error.to_string())?;
    let second = create_preview_work_root().map_err(|error| error.to_string())?;
    if first == second || !first.is_dir() || !second.is_dir() {
        return Err("preview workspaces were not unique real directories".to_owned());
    }
    fs::write(first.join("first.txt"), b"first")
        .map_err(|error| error.to_string())?;
    fs::write(second.join("second.txt"), b"second")
        .map_err(|error| error.to_string())?;
    fs::remove_dir_all(&first).map_err(|error| error.to_string())?;
    if !second.join("second.txt").is_file() {
        return Err("cleaning one preview workspace affected another".to_owned());
    }
    fs::remove_dir_all(&second).map_err(|error| error.to_string())?;
    Ok(())
}

static NEXT_FIXTURE: AtomicUsize = AtomicUsize::new(0);

#[test]
fn canonical_aliases_support_one_both_or_neither() -> Result<(), String> {
    let cases = [
        ("none", Vec::<&str>::new(), Vec::<OptionalModRole>::new()),
        ("m", vec!["m.lmlm"], vec![OptionalModRole::Remaster]),
        ("j", vec!["j.lmlm"], vec![OptionalModRole::Latino]),
        ("both", vec!["j.lmlm", "m.lmlm"], vec![
            OptionalModRole::Remaster,
            OptionalModRole::Latino,
        ]),
    ];
    for (label, names, expected) in cases {
        let root = temp_root(label);
        let mods = root.join("mods");
        fs::create_dir_all(&mods).map_err(|error| error.to_string())?;
        for name in names {
            fs::write(mods.join(name), b"fixture")
                .map_err(|error| error.to_string())?;
        }
        let actual = discover_optional_mods(&root)
            .map_err(|error| error.to_string())?
            .into_iter()
            .map(|archive| archive.role)
            .collect::<Vec<_>>();
        if actual != expected {
            return Err(format!("unexpected aliases for {label}: {actual:?}"));
        }
        fs::remove_dir_all(root).map_err(|error| error.to_string())?;
    }
    Ok(())
}

#[test]
fn unknown_lmlm_alias_fails_closed() -> Result<(), String> {
    let root = temp_root("unknown");
    let mods = root.join("mods");
    fs::create_dir_all(&mods).map_err(|error| error.to_string())?;
    fs::write(mods.join("release.lmlm"), b"fixture")
        .map_err(|error| error.to_string())?;
    let error = match discover_optional_mods(&root) {
        Ok(_archives) => {
            return Err("unknown alias unexpectedly succeeded".to_owned());
        },
        Err(error) => error.to_string(),
    };
    if !error.contains("use m.lmlm or j.lmlm") {
        return Err(format!("unexpected alias diagnostic: {error}"));
    }
    fs::remove_dir_all(root).map_err(|error| error.to_string())?;
    Ok(())
}

#[test]
fn remaster_replaces_only_existing_base_files() -> Result<(), String> {
    let root = temp_root("remaster");
    let game = root.join("game");
    let extracted = root.join("extracted");
    let original = game.join("art").join("base.p3d");
    let parent = original
        .parent()
        .ok_or_else(|| "fixture path has no parent".to_owned())?;
    fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    fs::create_dir_all(&extracted).map_err(|error| error.to_string())?;
    fs::write(&original, b"old").map_err(|error| error.to_string())?;
    let base_files =
        existing_file_index(&game, &extracted, &extracted.join("lmlm"))
            .map_err(|error| error.to_string())?;
    let data = [b'n'; 128];
    let entries = vec![
        FileEntry {
            path: "CustomFiles/art/base.p3d".to_owned(),
            offset: 0,
            size: 64,
        },
        FileEntry {
            path: "CustomFiles/art/extra.p3d".to_owned(),
            offset: 64,
            size: 64,
        },
    ];
    let mut records = Vec::new();
    let counts =
        apply_remaster(&data, &entries, &extracted, &base_files, &mut records)
            .map_err(|error| error.to_string())?;
    let replacement = extracted.join("art").join("base.p3d");
    if fs::read(&replacement).map_err(|error| error.to_string())?
        != vec![b'n'; 64]
    {
        return Err("existing base file was not replaced".to_owned());
    }
    if extracted.join("art").join("extra.p3d").exists() {
        return Err("remaster created an additional file".to_owned());
    }
    if counts.written != 1 || counts.skipped != 1 || records.len() != 1 {
        return Err(format!("unexpected remaster counts: {counts:?}"));
    }
    fs::remove_dir_all(root).map_err(|error| error.to_string())?;
    Ok(())
}

#[test]
fn latino_role_accepts_only_voice_and_cinematic_media() {
    assert!(is_latino_audio_path("CustomFiles/homer/line.rsd"));
    assert!(is_latino_movie_path("CustomFiles/movies/intro.rmv"));
    assert!(is_latino_movie_path("CustomFiles/movies/intro.bik"));
    assert!(!is_latino_audio_path("Resources/line.rsd"));
    assert!(!is_latino_audio_path("CustomFiles/homer/line.txt"));
    assert!(!is_latino_movie_path("CustomFiles/movies/intro.txt"));
}

fn temp_root(label: &str) -> PathBuf {
    let sequence = NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "pipeline-lmlm-{label}-{}-{sequence}",
        std::process::id()
    ))
}

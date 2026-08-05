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
use rmv::Sha256;

use super::optional_mods::{
    OptionalModRole, apply_remaster, create_optional_mod_work_root,
    discover_optional_mods, existing_file_index, is_latino_audio_path,
    is_latino_movie_path, optional_workspace_error,
};
use super::{extract_lmlm, preview_optional_mods};

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

    let report = extract_lmlm(&game_root, &extracted_root, false)
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
        || preview.approval_token().is_some()
    {
        return Err("empty preview reported package changes".to_owned());
    }
    let document: serde_json::Value = serde_json::from_str(preview.json())
        .map_err(|error| error.to_string())?;
    if document.get("schema").and_then(serde_json::Value::as_str)
        != Some("shar-schoenwald.optional-mod-preview.v2")
        || document.get("dry_run").and_then(serde_json::Value::as_bool)
            != Some(true)
        || !document
            .get("approval_token")
            .is_some_and(serde_json::Value::is_null)
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
fn direct_optional_stage_requires_approval() -> Result<(), String> {
    let case = temp_root("direct-approval");
    if case.exists() {
        fs::remove_dir_all(&case).map_err(|error| error.to_string())?;
    }
    let game_root = case.join("game");
    let mods_root = game_root.join("mods");
    let extracted_root = case.join("extracted");
    fs::create_dir_all(&mods_root).map_err(|error| error.to_string())?;
    fs::write(mods_root.join("m.lmlm"), b"fixture")
        .map_err(|error| error.to_string())?;

    let error = match extract_lmlm(&game_root, &extracted_root, false) {
        Ok(_report) => {
            return Err("direct optional stage bypassed approval".to_owned());
        }
        Err(error) => error.to_string(),
    };
    if !error.contains("require explicit approval") || extracted_root.exists() {
        return Err("direct stage approval failure mutated output".to_owned());
    }
    fs::remove_dir_all(&case).map_err(|error| error.to_string())?;
    Ok(())
}

#[test]
fn corrupt_optional_package_diagnostics_use_public_aliases()
-> Result<(), String> {
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

    let extract_error = match extract_lmlm(&game_root, &extracted_root, true) {
        Ok(_report) => {
            return Err("corrupt extraction package succeeded".to_owned());
        }
        Err(error) => error.to_string(),
    };
    let preview_error = match preview_optional_mods(&game_root, &extracted_root)
    {
        Ok(_preview) => {
            return Err("corrupt preview package succeeded".to_owned());
        }
        Err(error) => error.to_string(),
    };
    for error in [&extract_error, &preview_error] {
        if !error.contains("m.lmlm") || error.contains(private_fragment) {
            return Err(format!(
                "package diagnostic is not public-safe: {error}"
            ));
        }
    }
    if extracted_root.join("lmlm").exists() {
        return Err("corrupt package changed extraction output".to_owned());
    }
    fs::remove_dir_all(&case).map_err(|error| error.to_string())?;
    Ok(())
}

#[test]
fn optional_workspace_diagnostics_hide_local_paths() -> Result<(), String> {
    let private_fragment = "private-workstation-root";
    let error = std::io::Error::other(format!(
        "{private_fragment}/temporary/lmlm-preview"
    ));
    let rendered = optional_workspace_error("allocation", &error).to_string();
    if rendered.contains(private_fragment)
        || !rendered.contains("allocation")
        || !rendered.contains("Other")
    {
        return Err(format!(
            "workspace diagnostic was not public-safe: {rendered}"
        ));
    }
    Ok(())
}

#[test]
fn optional_package_workspaces_are_unique_and_self_cleaning()
-> Result<(), String> {
    let first = create_optional_mod_work_root("preview")
        .map_err(|error| error.to_string())?;
    let second = create_optional_mod_work_root("preview")
        .map_err(|error| error.to_string())?;
    let first_path = first.path().to_path_buf();
    let second_path = second.path().to_path_buf();
    if first_path == second_path
        || !first_path.is_dir()
        || !second_path.is_dir()
    {
        return Err(
            "preview workspaces were not unique real directories".to_owned()
        );
    }
    fs::write(first_path.join("first.txt"), b"first")
        .map_err(|error| error.to_string())?;
    fs::write(second_path.join("second.txt"), b"second")
        .map_err(|error| error.to_string())?;
    first.cleanup().map_err(|error| error.to_string())?;
    if !second_path.join("second.txt").is_file() {
        return Err(
            "cleaning one preview workspace affected another".to_owned()
        );
    }
    second.cleanup().map_err(|error| error.to_string())?;

    let abandoned = create_optional_mod_work_root("extract")
        .map_err(|error| error.to_string())?;
    let abandoned_path = abandoned.path().to_path_buf();
    drop(abandoned);
    if abandoned_path.exists() {
        return Err(
            "abandoned optional-package workspace was not cleaned".to_owned()
        );
    }
    Ok(())
}

const LMLM_BLOCK: usize = 0x200;
const LMLM_ROOT_BLOCK: usize = 0x400;
const LMLM_FIRST_ENTRY: usize = 0x600;
const LMLM_PAYLOAD_OFFSET: usize = 0x1800;

fn copy_lmlm_fixture_bytes(
    archive: &mut [u8],
    start: usize,
    bytes: &[u8],
) -> Result<(), String> {
    let end = start
        .checked_add(bytes.len())
        .ok_or_else(|| "LMLM fixture range overflowed".to_owned())?;
    let target = archive
        .get_mut(start..end)
        .ok_or_else(|| "LMLM fixture range was out of bounds".to_owned())?;
    target.copy_from_slice(bytes);
    Ok(())
}

fn write_lmlm_entry_name(
    archive: &mut [u8],
    position: usize,
    name: &str,
) -> Result<(), String> {
    copy_lmlm_fixture_bytes(archive, position, &2_u16.to_le_bytes())?;
    let mut encoded_name = Vec::new();
    for unit in name.encode_utf16() {
        encoded_name.extend_from_slice(&unit.to_le_bytes());
    }
    encoded_name.extend_from_slice(&0_u16.to_le_bytes());
    if encoded_name.len() > LMLM_BLOCK.saturating_sub(2) {
        return Err(
            "LMLM fixture name exceeded one structural block".to_owned()
        );
    }
    copy_lmlm_fixture_bytes(archive, position.saturating_add(2), &encoded_name)
}

fn write_lmlm_directory(
    archive: &mut [u8],
    position: usize,
    name: &str,
    contains_directory: bool,
) -> Result<(), String> {
    write_lmlm_entry_name(archive, position, name)?;
    let metadata = position.saturating_add(LMLM_BLOCK);
    copy_lmlm_fixture_bytes(
        archive,
        metadata.saturating_add(0x0c),
        &1_u16.to_le_bytes(),
    )?;
    copy_lmlm_fixture_bytes(
        archive,
        metadata.saturating_add(0x0e),
        &[u8::from(contains_directory)],
    )
}

fn write_lmlm_file(
    archive: &mut [u8],
    position: usize,
    name: &str,
    payload: &[u8],
) -> Result<(), String> {
    write_lmlm_entry_name(archive, position, name)?;
    let metadata = position.saturating_add(LMLM_BLOCK);
    copy_lmlm_fixture_bytes(
        archive,
        metadata.saturating_add(0x0c),
        &u64::try_from(payload.len())
            .map_err(|_error| "LMLM payload length did not fit u64".to_owned())?
            .to_le_bytes(),
    )?;
    copy_lmlm_fixture_bytes(
        archive,
        metadata.saturating_add(0x14),
        &u64::try_from(LMLM_PAYLOAD_OFFSET)
            .map_err(|_error| "LMLM payload offset did not fit u64".to_owned())?
            .to_le_bytes(),
    )
}

fn single_file_lmlm(
    directory: &str,
    file_name: &str,
    payload: &[u8],
) -> Result<Vec<u8>, String> {
    let archive_len = LMLM_PAYLOAD_OFFSET
        .checked_add(payload.len())
        .ok_or_else(|| "LMLM fixture length overflowed".to_owned())?;
    let mut archive = vec![0_u8; archive_len];
    copy_lmlm_fixture_bytes(&mut archive, 0, b"LSPA")?;
    copy_lmlm_fixture_bytes(&mut archive, 4, &5_u32.to_le_bytes())?;
    copy_lmlm_fixture_bytes(
        &mut archive,
        0x0c,
        &0x0200_0000_u32.to_le_bytes(),
    )?;
    copy_lmlm_fixture_bytes(
        &mut archive,
        LMLM_ROOT_BLOCK.saturating_add(2),
        &1_u16.to_le_bytes(),
    )?;
    write_lmlm_directory(&mut archive, LMLM_FIRST_ENTRY, "CustomFiles", true)?;
    let art_position = LMLM_FIRST_ENTRY.saturating_add(LMLM_BLOCK * 2);
    write_lmlm_directory(&mut archive, art_position, directory, false)?;
    let file_position = art_position.saturating_add(LMLM_BLOCK * 2);
    write_lmlm_file(&mut archive, file_position, file_name, payload)?;
    copy_lmlm_fixture_bytes(&mut archive, LMLM_PAYLOAD_OFFSET, payload)?;
    Ok(archive)
}

fn compact_pcm_rsd(payload: &[u8]) -> Result<Vec<u8>, String> {
    let mut data = vec![0_u8; 0x80];
    copy_lmlm_fixture_bytes(&mut data, 0, b"RSD4")?;
    copy_lmlm_fixture_bytes(&mut data, 4, b"PCM ")?;
    copy_lmlm_fixture_bytes(&mut data, 8, &1_u32.to_le_bytes())?;
    copy_lmlm_fixture_bytes(&mut data, 12, &16_u32.to_le_bytes())?;
    copy_lmlm_fixture_bytes(&mut data, 16, &24_000_u32.to_le_bytes())?;
    data.extend_from_slice(payload);
    Ok(data)
}

static NEXT_FIXTURE: AtomicUsize = AtomicUsize::new(0);

#[test]
fn preview_matches_extracted_remaster_evidence() -> Result<(), String> {
    let case = temp_root("preview-extract-parity");
    if case.exists() {
        fs::remove_dir_all(&case).map_err(|error| error.to_string())?;
    }
    let game_root = case.join("game");
    let extracted_root = case.join("extracted");
    let source = game_root.join("art").join("base.p3d");
    let source_parent = source
        .parent()
        .ok_or_else(|| "source fixture path has no parent".to_owned())?;
    let mods_root = game_root.join("mods");
    fs::create_dir_all(source_parent).map_err(|error| error.to_string())?;
    fs::create_dir_all(&mods_root).map_err(|error| error.to_string())?;
    fs::create_dir_all(&extracted_root).map_err(|error| error.to_string())?;
    fs::write(&source, b"source-old").map_err(|error| error.to_string())?;
    let replacement = b"replacement-bytes";
    let archive = single_file_lmlm("art", "base.p3d", replacement)?;
    fs::write(mods_root.join("m.lmlm"), archive)
        .map_err(|error| error.to_string())?;

    let preview = preview_optional_mods(&game_root, &extracted_root)
        .map_err(|error| error.to_string())?;
    let preview_json: serde_json::Value = serde_json::from_str(preview.json())
        .map_err(|error| error.to_string())?;
    let preview_changes = preview_json
        .get("changes")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| "preview changes were not an array".to_owned())?;
    let [preview_change] = preview_changes.as_slice() else {
        return Err("single-member preview emitted an unexpected change count"
            .to_owned());
    };
    if extracted_root.join("art").join("base.p3d").exists() {
        return Err("preview wrote the predicted remaster output".to_owned());
    }

    let report = extract_lmlm(&game_root, &extracted_root, true)
        .map_err(|error| error.to_string())?;
    if report.files != 2 {
        return Err(format!(
            "unexpected extraction file count: {}",
            report.files
        ));
    }
    let manifest: serde_json::Value = serde_json::from_slice(
        &fs::read(extracted_root.join("lmlm").join("manifest.json"))
            .map_err(|error| error.to_string())?,
    )
    .map_err(|error| error.to_string())?;
    let records = manifest
        .get("records")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| "extraction records were not an array".to_owned())?;
    let [record] = records.as_slice() else {
        return Err(
            "single-member extraction emitted an unexpected record count"
                .to_owned(),
        );
    };
    for field in ["source", "output", "sha256"] {
        if preview_change.get(field) != record.get(field) {
            return Err(format!("preview and extraction differ for {field}"));
        }
    }
    if preview_change.get("normalized_bytes") != record.get("bytes") {
        return Err(
            "preview and extraction differ for normalized bytes".to_owned()
        );
    }
    if preview_change
        .get("action")
        .and_then(serde_json::Value::as_str)
        != Some("replace")
        || fs::read(extracted_root.join("art").join("base.p3d"))
            .map_err(|error| error.to_string())?
            != replacement
        || fs::read(&source).map_err(|error| error.to_string())?
            != b"source-old"
    {
        return Err(
            "remaster preview/extraction parity contract failed".to_owned()
        );
    }
    fs::remove_dir_all(&case).map_err(|error| error.to_string())?;
    Ok(())
}

#[test]
fn preview_token_tracks_exact_package_bytes() -> Result<(), String> {
    let case = temp_root("preview-approval-token");
    if case.exists() {
        fs::remove_dir_all(&case).map_err(|error| error.to_string())?;
    }
    let game_root = case.join("game");
    let source = game_root.join("art").join("base.p3d");
    let source_parent = source
        .parent()
        .ok_or_else(|| "token fixture source has no parent".to_owned())?;
    let mods_root = game_root.join("mods");
    let extracted_root = case.join("extracted");
    fs::create_dir_all(source_parent).map_err(|error| error.to_string())?;
    fs::create_dir_all(&mods_root).map_err(|error| error.to_string())?;
    fs::write(&source, b"source").map_err(|error| error.to_string())?;

    let first_archive = single_file_lmlm("art", "base.p3d", b"first")?;
    fs::write(mods_root.join("m.lmlm"), &first_archive)
        .map_err(|error| error.to_string())?;
    let first = preview_optional_mods(&game_root, &extracted_root)
        .map_err(|error| error.to_string())?;
    let first_token = first
        .approval_token()
        .ok_or_else(|| "package preview omitted approval token".to_owned())?
        .to_owned();
    let first_json: serde_json::Value = serde_json::from_str(first.json())
        .map_err(|error| error.to_string())?;
    let packages = first_json
        .get("packages")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| "preview packages were not an array".to_owned())?;
    let [package] = packages.as_slice() else {
        return Err(
            "token preview emitted an unexpected package count".to_owned()
        );
    };
    if first_token.len() != 64
        || !first_token.bytes().all(|byte| byte.is_ascii_hexdigit())
        || package
            .get("package_bytes")
            .and_then(serde_json::Value::as_u64)
            != u64::try_from(first_archive.len()).ok()
        || package
            .get("package_sha256")
            .and_then(serde_json::Value::as_str)
            != Some(Sha256::digest(&first_archive).hex().as_str())
    {
        return Err("preview package identity evidence was invalid".to_owned());
    }

    let second_archive = single_file_lmlm("art", "base.p3d", b"second")?;
    fs::write(mods_root.join("m.lmlm"), second_archive)
        .map_err(|error| error.to_string())?;
    let second = preview_optional_mods(&game_root, &extracted_root)
        .map_err(|error| error.to_string())?;
    let second_token = second
        .approval_token()
        .ok_or_else(|| "changed package preview omitted token".to_owned())?;
    if second_token == first_token {
        return Err(
            "package payload change preserved approval token".to_owned()
        );
    }
    fs::remove_dir_all(&case).map_err(|error| error.to_string())?;
    Ok(())
}

#[test]
fn preview_matches_extracted_latino_voice_evidence() -> Result<(), String> {
    let case = temp_root("preview-latino-voice-parity");
    if case.exists() {
        fs::remove_dir_all(&case).map_err(|error| error.to_string())?;
    }
    let game_root = case.join("game");
    let extracted_root = case.join("extracted");
    let mods_root = game_root.join("mods");
    fs::create_dir_all(&mods_root).map_err(|error| error.to_string())?;
    fs::create_dir_all(&extracted_root).map_err(|error| error.to_string())?;
    let rsd = compact_pcm_rsd(&[1, 0, 2, 0])?;
    let archive = single_file_lmlm("homer", "line.rsd", &rsd)?;
    fs::write(mods_root.join("j.lmlm"), archive)
        .map_err(|error| error.to_string())?;

    let preview = preview_optional_mods(&game_root, &extracted_root)
        .map_err(|error| error.to_string())?;
    let preview_json: serde_json::Value = serde_json::from_str(preview.json())
        .map_err(|error| error.to_string())?;
    let preview_changes = preview_json
        .get("changes")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| "Latino preview changes were not an array".to_owned())?;
    let [preview_change] = preview_changes.as_slice() else {
        return Err("single-voice preview emitted an unexpected change count"
            .to_owned());
    };
    let output = preview_change
        .get("output")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| "Latino preview omitted its output path".to_owned())?;
    let expected_output = "lmlm/latino/customfiles/homer/line.wav";
    if output != expected_output
        || preview_change
            .get("action")
            .and_then(serde_json::Value::as_str)
            != Some("add")
        || extracted_root.join(expected_output).exists()
    {
        return Err(
            "Latino preview did not describe one read-only add".to_owned()
        );
    }

    let report = extract_lmlm(&game_root, &extracted_root, true)
        .map_err(|error| error.to_string())?;
    if report.files != 2 {
        return Err(format!(
            "unexpected Latino extraction file count: {}",
            report.files
        ));
    }
    let manifest: serde_json::Value = serde_json::from_slice(
        &fs::read(extracted_root.join("lmlm").join("manifest.json"))
            .map_err(|error| error.to_string())?,
    )
    .map_err(|error| error.to_string())?;
    let records = manifest
        .get("records")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| {
            "Latino extraction records were not an array".to_owned()
        })?;
    let [record] = records.as_slice() else {
        return Err(
            "single-voice extraction emitted an unexpected record count"
                .to_owned(),
        );
    };
    for field in ["source", "output", "sha256"] {
        if preview_change.get(field) != record.get(field) {
            return Err(format!(
                "Latino preview and extraction differ for {field}"
            ));
        }
    }
    if preview_change.get("normalized_bytes") != record.get("bytes") {
        return Err(
            "Latino preview and extraction differ for normalized bytes"
                .to_owned(),
        );
    }
    let wav = fs::read(extracted_root.join(expected_output))
        .map_err(|error| error.to_string())?;
    if !wav.starts_with(b"RIFF")
        || wav.get(8..12).is_none_or(|tag| tag != b"WAVE")
    {
        return Err(
            "Latino extraction did not publish a canonical WAV".to_owned()
        );
    }
    fs::remove_dir_all(&case).map_err(|error| error.to_string())?;
    Ok(())
}

#[test]
fn canonical_aliases_support_one_both_or_neither() -> Result<(), String> {
    let cases = [
        ("none", Vec::<&str>::new(), Vec::<OptionalModRole>::new()),
        ("m", vec!["m.lmlm"], vec![OptionalModRole::Remaster]),
        ("j", vec!["j.lmlm"], vec![OptionalModRole::Latino]),
        (
            "both",
            vec!["j.lmlm", "m.lmlm"],
            vec![OptionalModRole::Remaster, OptionalModRole::Latino],
        ),
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
        }
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
    let derived = extracted.join("art").join("derived.p3d");
    let derived_parent = derived
        .parent()
        .ok_or_else(|| "derived fixture path has no parent".to_owned())?;
    fs::create_dir_all(derived_parent).map_err(|error| error.to_string())?;
    fs::write(&original, b"old").map_err(|error| error.to_string())?;
    fs::write(&derived, b"derived-old").map_err(|error| error.to_string())?;
    let base_files =
        existing_file_index(&game, &extracted, &extracted.join("lmlm"))
            .map_err(|error| error.to_string())?;
    let data = [b'n'; 192];
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
        FileEntry {
            path: "CustomFiles/art/derived.p3d".to_owned(),
            offset: 128,
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
    if fs::read(&original).map_err(|error| error.to_string())? != b"old" {
        return Err("remaster modified the source installation".to_owned());
    }
    if fs::read(&derived).map_err(|error| error.to_string())? != b"derived-old"
    {
        return Err("remaster modified an extracted-only identity".to_owned());
    }
    if counts.written != 1 || counts.skipped != 2 || records.len() != 1 {
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

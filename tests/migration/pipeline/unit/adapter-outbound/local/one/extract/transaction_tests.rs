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
//   - Full-extraction transaction integration tests.
// - Must-Not:
//   - Read the local installation or publish repository-generated output.
// - Allows:
//   - Isolated temporary game and extraction roots.
// - Split-When:
//   - Split when successful full-extraction fixtures become lightweight.
// - Merge-When:
//   - Merge when transaction behavior is no longer visible through extraction.
// - Summary:
//   - Extraction transaction integration tests.
// - Description:
//   - Proves clean and resume failures preserve the accepted output root.
// - Usage:
//   - Included only by the extraction adapter under cfg(test).
// - Defaults:
//   - Missing source manifest forces failure before any accepted publication.
//

//! Full-extraction transaction integration tests.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use super::super::extraction_transaction::ExtractionTransaction;
use super::{ExtractGameAssets, guard_paths};
use crate::domain::PipelineConfig;

static CASE_ID: AtomicUsize = AtomicUsize::new(0);

type TestResult = Result<(), String>;

#[test]
fn partial_export_respects_active_full_transaction_lease() -> TestResult {
    let root = case_root("partial-export-active-lease");
    prepare_root(&root)?;
    let game_root = root.join("game");
    let extracted_root = root.join("extracted");
    fs::create_dir_all(&game_root).map_err(|error| error.to_string())?;
    fs::create_dir_all(&extracted_root).map_err(|error| error.to_string())?;
    let sentinel = extracted_root.join("accepted.txt");
    fs::write(&sentinel, b"accepted").map_err(|error| error.to_string())?;
    let transaction = ExtractionTransaction::begin(&extracted_root)
        .map_err(|error| error.to_string())?;
    let config = PipelineConfig {
        game_root,
        extracted_root: extracted_root.clone(),
        clean_extracted: true,
    };

    let error = match ExtractGameAssets::export_movies_only(&config) {
        Ok(_report) => {
            return Err("partial export bypassed active lease".to_owned());
        }
        Err(error) => error.to_string(),
    };
    if !error.contains("active extraction transaction") {
        return Err(format!("unexpected active-lease error: {error}"));
    }
    let bytes = fs::read(&sentinel).map_err(|error| error.to_string())?;
    if bytes != b"accepted" {
        return Err("lease contention changed accepted output".to_owned());
    }
    transaction.abort().map_err(|error| error.to_string())?;
    require_transaction_absent(&extracted_root)?;
    fs::remove_dir_all(root).map_err(|error| error.to_string())
}

#[test]
fn partial_export_recovers_interrupted_full_transaction() -> TestResult {
    let root = case_root("partial-export-recovery");
    prepare_root(&root)?;
    let game_root = root.join("game");
    let extracted_root = root.join("extracted");
    fs::create_dir_all(&game_root).map_err(|error| error.to_string())?;
    fs::create_dir_all(&extracted_root).map_err(|error| error.to_string())?;
    let sentinel = extracted_root.join("accepted.txt");
    fs::write(&sentinel, b"accepted").map_err(|error| error.to_string())?;
    let paths = transaction_paths(&extracted_root)?;
    write_transaction_state(&paths.state)?;
    fs::rename(&extracted_root, &paths.backup)
        .map_err(|error| error.to_string())?;
    fs::create_dir_all(&paths.staging).map_err(|error| error.to_string())?;
    fs::write(paths.staging.join("partial.txt"), b"partial")
        .map_err(|error| error.to_string())?;
    let config = PipelineConfig {
        game_root,
        extracted_root: extracted_root.clone(),
        clean_extracted: true,
    };

    let error = match ExtractGameAssets::export_movies_only(&config) {
        Ok(_report) => {
            return Err("empty movie export unexpectedly passed".to_owned());
        }
        Err(error) => error.to_string(),
    };
    if !error.contains("no .rmv movie inputs") {
        return Err(format!("unexpected post-recovery error: {error}"));
    }
    require_restored_accepted_output(&extracted_root, &sentinel)?;
    require_transaction_absent(&extracted_root)?;
    fs::remove_dir_all(root).map_err(|error| error.to_string())
}

#[test]
fn partial_export_rejects_symlinked_parent_prefix() -> TestResult {
    let root = case_root("partial-export-symlink-prefix");
    prepare_root(&root)?;
    let game_root = root.join("game");
    let target = root.join("target");
    let link = root.join("link");
    fs::create_dir_all(&game_root).map_err(|error| error.to_string())?;
    fs::create_dir_all(&target).map_err(|error| error.to_string())?;
    create_directory_link(&target, &link)?;
    let extracted_root = link.join("created-through-link").join("extracted");
    let config = PipelineConfig {
        game_root,
        extracted_root,
        clean_extracted: true,
    };

    let error = match ExtractGameAssets::export_movies_only(&config) {
        Ok(_report) => {
            return Err(
                "symlinked partial output unexpectedly passed".to_owned(),
            );
        }
        Err(error) => error.to_string(),
    };
    if !error.contains("output path failed (InvalidInput)") {
        return Err(format!("unexpected symlink-prefix error: {error}"));
    }
    if target.join("created-through-link").exists() {
        return Err("partial export created output through a link".to_owned());
    }
    remove_directory_link(&link)?;
    fs::remove_dir_all(root).map_err(|error| error.to_string())
}

#[test]
fn existing_output_file_is_rejected_before_lock_creation() -> TestResult {
    let root = case_root("output-file-before-lock");
    prepare_root(&root)?;
    let game_root = root.join("game");
    let extracted_root = root.join("extracted");
    fs::create_dir_all(&game_root).map_err(|error| error.to_string())?;
    fs::write(&extracted_root, b"original")
        .map_err(|error| error.to_string())?;
    let lock = root.join(".extracted.pipeline-lock");

    let error = match guard_paths(&game_root, &extracted_root) {
        Ok(_identity) => {
            return Err("output file unexpectedly passed".to_owned());
        }
        Err(error) => error.to_string(),
    };
    if !error.contains("output must be a directory") {
        return Err(format!("unexpected output-file error: {error}"));
    }
    let bytes = fs::read(&extracted_root).map_err(|error| error.to_string())?;
    if bytes != b"original" {
        return Err("output file changed during validation".to_owned());
    }
    if lock.exists() {
        return Err("output-file validation created a lock".to_owned());
    }
    fs::remove_dir_all(root).map_err(|error| error.to_string())
}

#[test]
fn missing_parent_components_normalize_without_side_effects() -> TestResult {
    let root = case_root("missing-parent-normalization");
    prepare_root(&root)?;
    let game_root = root.join("game");
    let missing = root.join("missing");
    fs::create_dir_all(&game_root).map_err(|error| error.to_string())?;
    let extracted_root = missing.join("..").join("output");

    let identity = guard_paths(&game_root, &extracted_root)
        .map_err(|error| error.to_string())?;
    let expected = fs::canonicalize(&root)
        .map_err(|error| error.to_string())?
        .join("output");
    if identity != expected {
        return Err(format!(
            "unexpected normalized output identity: {}",
            identity.display()
        ));
    }
    if missing.exists() || expected.exists() {
        return Err("path validation created a discarded component".to_owned());
    }
    fs::remove_dir_all(root).map_err(|error| error.to_string())
}

#[test]
fn canonical_output_inside_game_is_rejected_before_creation() -> TestResult {
    let root = case_root("canonical-output-containment");
    prepare_root(&root)?;
    let game_root = root.join("game");
    let detour = root.join("detour");
    fs::create_dir_all(&game_root).map_err(|error| error.to_string())?;
    fs::create_dir_all(&detour).map_err(|error| error.to_string())?;
    let extracted_root = detour.join("..").join("game").join("generated");

    let error = match guard_paths(&game_root, &extracted_root) {
        Ok(_identity) => {
            return Err(
                "canonical in-game output unexpectedly passed".to_owned(),
            );
        }
        Err(error) => error.to_string(),
    };
    if !error.contains("refusing to write inside game") {
        return Err(format!("unexpected containment error: {error}"));
    }
    if game_root.join("generated").exists() {
        return Err("containment guard created an in-game output".to_owned());
    }
    fs::remove_dir_all(root).map_err(|error| error.to_string())
}

#[test]
fn failed_clean_and_resume_runs_preserve_accepted_output() -> TestResult {
    for clean_extracted in [true, false] {
        let root = case_root(if clean_extracted { "clean" } else { "resume" });
        prepare_root(&root)?;
        let game_root = root.join("game");
        let extracted_root = root.join("extracted");
        fs::create_dir_all(&game_root).map_err(|error| error.to_string())?;
        fs::create_dir_all(&extracted_root)
            .map_err(|error| error.to_string())?;
        let sentinel = extracted_root.join("accepted.txt");
        fs::write(&sentinel, b"accepted").map_err(|error| error.to_string())?;
        let config = PipelineConfig {
            game_root,
            extracted_root: extracted_root.clone(),
            clean_extracted,
        };

        let error = match ExtractGameAssets::run(&config) {
            Ok(_report) => {
                return Err("invalid extraction unexpectedly passed".to_owned());
            }
            Err(error) => error.to_string(),
        };
        if !error.contains("game manifest validation failed") {
            return Err(format!("unexpected staged extraction error: {error}"));
        }
        let bytes = fs::read(&sentinel).map_err(|error| error.to_string())?;
        if bytes != b"accepted" {
            return Err("failed extraction changed accepted output".to_owned());
        }
        require_transaction_absent(&extracted_root)?;
        fs::remove_dir_all(root).map_err(|error| error.to_string())?;
    }
    Ok(())
}

fn write_transaction_state(path: &Path) -> TestResult {
    fs::write(
        path,
        b"{\"schema\":\"shar-schoenwald.extraction-transaction.v1\"}
",
    )
    .map_err(|error| error.to_string())
}

fn require_restored_accepted_output(
    extracted_root: &Path,
    sentinel: &Path,
) -> TestResult {
    let bytes = fs::read(sentinel).map_err(|error| error.to_string())?;
    if bytes == b"accepted" && !extracted_root.join("partial.txt").exists() {
        Ok(())
    } else {
        Err("resume recovery did not restore accepted output".to_owned())
    }
}

#[cfg(unix)]
fn create_directory_link(target: &Path, link: &Path) -> TestResult {
    std::os::unix::fs::symlink(target, link).map_err(|error| error.to_string())
}

#[cfg(windows)]
fn create_directory_link(target: &Path, link: &Path) -> TestResult {
    std::os::windows::fs::symlink_dir(target, link)
        .map_err(|error| error.to_string())
}

#[cfg(unix)]
fn remove_directory_link(link: &Path) -> TestResult {
    fs::remove_file(link).map_err(|error| error.to_string())
}

#[cfg(windows)]
fn remove_directory_link(link: &Path) -> TestResult {
    fs::remove_dir(link).map_err(|error| error.to_string())
}

fn case_root(label: &str) -> PathBuf {
    let id = CASE_ID.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "pipeline-extraction-boundary-{label}-{}-{id}",
        std::process::id()
    ))
}

fn prepare_root(root: &Path) -> TestResult {
    if root.exists() {
        fs::remove_dir_all(root).map_err(|error| error.to_string())?;
    }
    fs::create_dir_all(root).map_err(|error| error.to_string())
}

#[derive(Debug)]
struct TestTransactionPaths {
    staging: PathBuf,
    backup: PathBuf,
    state: PathBuf,
}

fn transaction_paths(
    destination: &Path,
) -> Result<TestTransactionPaths, String> {
    let parent = destination
        .parent()
        .ok_or_else(|| "test extraction root has no parent".to_owned())?;
    let name = destination
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| "test extraction root has no UTF-8 name".to_owned())?;
    Ok(TestTransactionPaths {
        staging: parent.join(format!(".{name}.pipeline-staging")),
        backup: parent.join(format!(".{name}.pipeline-backup")),
        state: parent.join(format!(".{name}.pipeline-transaction.json")),
    })
}

fn require_transaction_absent(destination: &Path) -> TestResult {
    let paths = transaction_paths(destination)?;
    for path in [paths.staging, paths.backup, paths.state] {
        if path.exists() {
            return Err("transaction artifact remained".to_owned());
        }
    }
    Ok(())
}

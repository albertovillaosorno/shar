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

use super::ExtractGameAssets;
use crate::domain::PipelineConfig;

static CASE_ID: AtomicUsize = AtomicUsize::new(0);

type TestResult = Result<(), String>;

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
            optional_mod_approval: None,
        };

        let error = match ExtractGameAssets::run(&config) {
            Ok(_report) => {
                return Err("invalid extraction unexpectedly passed".to_owned());
            }
            Err(error) => error.to_string(),
        };
        if !error.contains("manifest.jsonl") {
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

#[test]
fn recovery_precedes_optional_package_approval() -> TestResult {
    let root = case_root("recovery-before-approval");
    prepare_root(&root)?;
    let game_root = root.join("game");
    let mods_root = game_root.join("mods");
    let extracted_root = root.join("extracted");
    fs::create_dir_all(&mods_root).map_err(|error| error.to_string())?;
    fs::create_dir_all(&extracted_root).map_err(|error| error.to_string())?;
    fs::write(mods_root.join("m.lmlm"), b"fixture")
        .map_err(|error| error.to_string())?;
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
        clean_extracted: false,
        optional_mod_approval: None,
    };

    let error = match ExtractGameAssets::run(&config) {
        Ok(_report) => return Err("unapproved package resumed".to_owned()),
        Err(error) => error.to_string(),
    };
    if !error.contains("approval token") {
        return Err(format!("unexpected approval error: {error}"));
    }
    require_restored_accepted_output(&extracted_root, &sentinel)?;
    require_transaction_absent(&extracted_root)?;
    fs::remove_dir_all(root).map_err(|error| error.to_string())
}

#[test]
fn recovery_precedes_resume_package_continuity() -> TestResult {
    let root = case_root("recovery-before-continuity");
    prepare_root(&root)?;
    let game_root = root.join("game");
    let extracted_root = root.join("extracted");
    let lmlm_root = extracted_root.join("lmlm");
    fs::create_dir_all(&game_root).map_err(|error| error.to_string())?;
    fs::create_dir_all(&lmlm_root).map_err(|error| error.to_string())?;
    let sentinel = extracted_root.join("accepted.txt");
    fs::write(&sentinel, b"accepted").map_err(|error| error.to_string())?;
    fs::write(
        lmlm_root.join("manifest.json"),
        format!(
            concat!(
                "{{\"schema\":",
                "\"shar-schoenwald.optional-mod-extract.v3\",",
                "\"approval_token\":\"{}\"}}"
            ),
            "a".repeat(64)
        ),
    )
    .map_err(|error| error.to_string())?;
    let paths = transaction_paths(&extracted_root)?;
    fs::write(
        &paths.state,
        b"{\"schema\":\"shar-schoenwald.extraction-transaction.v1\"}\n",
    )
    .map_err(|error| error.to_string())?;
    fs::rename(&extracted_root, &paths.backup)
        .map_err(|error| error.to_string())?;
    fs::create_dir_all(&paths.staging).map_err(|error| error.to_string())?;
    fs::write(paths.staging.join("partial.txt"), b"partial")
        .map_err(|error| error.to_string())?;
    let config = PipelineConfig {
        game_root,
        extracted_root: extracted_root.clone(),
        clean_extracted: false,
        optional_mod_approval: None,
    };

    let error = match ExtractGameAssets::run(&config) {
        Ok(_report) => return Err("removed package set resumed".to_owned()),
        Err(error) => error.to_string(),
    };
    if !error.contains("optional package set changed") {
        return Err(format!("unexpected continuity error: {error}"));
    }
    require_restored_accepted_output(&extracted_root, &sentinel)?;
    require_transaction_absent(&extracted_root)?;
    fs::remove_dir_all(root).map_err(|error| error.to_string())
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

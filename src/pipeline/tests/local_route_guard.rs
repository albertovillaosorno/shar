// File:
//   - local_route_guard.rs
// Path:
//   - src/pipeline/tests/local_route_guard.rs
//
// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE
// Path-Rule:
//   - All paths in this header are repository-root relative.
//
// Boundary-Contract:
// - Owns:
//   - Deterministic pipeline regression coverage for tests local route guard.
// - Must-Not:
//   - Depend on private local outputs or non-deterministic repository state.
// - Allows:
//   - Focused fixtures and deterministic assertions for the owned behavior.
// - Split-When:
//   - Split when local route guard contains two independently testable
//   - contracts.
// - Merge-When:
//   - Another pipeline module owns the same tests boundary with no distinct
//   - invariant.
// - Summary:
//   - Repository-level guard for local asset route literals in public files.
// - Description:
//   - Defines local route guard data and behavior for pipeline tests.
// - Usage:
//   - Executed through cargo test for the owning crate or focused target.
// - Defaults:
//   - Fixtures remain deterministic and repository-local.
//
// ADRs:
// - docs/adr/pipeline/minor-unit-taxonomy-value-case.md
//
// Large file:
//   - false
//

//! Repository-level guard for local asset route literals in public files.
use std::path::{Path, PathBuf};
use std::process::Command;

use fbx as _;
use game_manifest as _;
use lmlm as _;
use p3d as _;
use pipeline as _;
use rcf as _;
use rmv as _;
use rsd as _;
use rtf as _;
use schoenwald_cli as _;
use schoenwald_filesystem as _;
use serde_json as _;
use shar_json_text as _;
use shar_sha256 as _;

#[test]
fn public_files_reject_local_asset_route_literals() -> Result<(), String> {
    let route_literal = concat!(
        "game",
        "/",
        "extracted"
    );
    let root = repository_root()?;
    let failures = route_failures(
        &root,
        route_literal,
    )?;
    if failures.is_empty() {
        Ok(())
    } else {
        Err(
            format!(
                "local asset route literal requires exact marker and 50-word \
                 English justification: {}",
                failures.join(", ")
            ),
        )
    }
}

#[test]
fn lists_untracked_nonignored_public_files() -> Result<(), String> {
    let files = untracked_repository_fixture()?;
    if files
        .lines()
        .any(|line| line == "canary.md")
    {
        Ok(())
    } else {
        Err(
            "repository file listing omitted an untracked public file"
                .to_owned(),
        )
    }
}

#[test]
fn detects_local_asset_route_literal_in_all_untracked_files() -> Result<(), String> {
    let root = std::env::temp_dir().join(
        format!(
            "shar-route-guard-files-{}",
            std::process::id()
        ),
    );
    drop(std::fs::remove_dir_all(&root));
    let result = (|| {
        std::fs::create_dir_all(&root)
            .map_err(|error| format!("fixture create failed: {error}"))?;
        let output = Command::new("git")
            .arg("init")
            .arg("--quiet")
            .current_dir(&root)
            .output()
            .map_err(|error| format!("fixture git init failed: {error}"))?;
        if !output
            .status
            .success()
        {
            return Err(
                format!(
                    "fixture git init returned failure: {}",
                    String::from_utf8_lossy(&output.stderr)
                ),
            );
        }
        let route_literal = concat!(
            "game",
            "/",
            "extracted"
        );
        std::fs::write(
            root.join("canary.yml"),
            format!("path: {route_literal}\n"),
        )
        .map_err(|error| format!("YAML fixture write failed: {error}"))?;
        let mut binary = vec![0xff_u8];
        binary.extend_from_slice(route_literal.as_bytes());
        binary.push(b'\n');
        std::fs::write(
            root.join("canary.bin"),
            binary,
        )
        .map_err(|error| format!("binary fixture write failed: {error}"))?;
        let failures = route_failures(
            &root,
            route_literal,
        )?;
        if failures
            == [
                "canary.bin:1",
                "canary.yml:1",
            ]
        {
            Ok(())
        } else {
            Err(
                format!(
                    "public-file route literals were not detected: {failures:?}"
                ),
            )
        }
    })();
    drop(std::fs::remove_dir_all(&root));
    result
}

fn route_failures(
    root: &Path,
    route_literal: &str,
) -> Result<Vec<String>, String> {
    let files = repository_files(root)?;
    let mut failures = Vec::new();
    for relative in files.lines() {
        let absolute = root.join(relative);
        let Ok(bytes) = std::fs::read(&absolute) else {
            continue;
        };
        let text = String::from_utf8_lossy(&bytes);
        let lines = text
            .lines()
            .collect::<Vec<_>>();
        for (index, line) in lines
            .iter()
            .enumerate()
        {
            if line.contains(route_literal)
                && !has_route_exception(
                    &lines, index,
                )
            {
                failures.push(
                    format!(
                        "{relative}:{}",
                        index.saturating_add(1)
                    ),
                );
            }
        }
    }
    failures.sort();
    Ok(failures)
}

fn untracked_repository_fixture() -> Result<String, String> {
    let root = std::env::temp_dir().join(
        format!(
            "shar-route-guard-untracked-{}",
            std::process::id()
        ),
    );
    drop(std::fs::remove_dir_all(&root));
    let result = (|| {
        std::fs::create_dir_all(&root)
            .map_err(|error| format!("fixture create failed: {error}"))?;
        let output = Command::new("git")
            .arg("init")
            .arg("--quiet")
            .current_dir(&root)
            .output()
            .map_err(|error| format!("fixture git init failed: {error}"))?;
        if !output
            .status
            .success()
        {
            return Err(
                format!(
                    "fixture git init returned failure: {}",
                    String::from_utf8_lossy(&output.stderr)
                ),
            );
        }
        std::fs::write(
            root.join("canary.md"),
            "# Canary\n",
        )
        .map_err(|error| format!("fixture write failed: {error}"))?;
        repository_files(&root)
    })();
    drop(std::fs::remove_dir_all(&root));
    result
}

/// Return every tracked or untracked nonignored public repository file.
fn repository_files(root: &Path) -> Result<String, String> {
    let output = Command::new("git")
        .arg("ls-files")
        .arg("--cached")
        .arg("--others")
        .arg("--exclude-standard")
        .current_dir(root)
        .output()
        .map_err(|error| format!("repository listing failed: {error}"))?;
    if !output
        .status
        .success()
    {
        return Err(
            format!(
                "git ls-files failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ),
        );
    }
    String::from_utf8(output.stdout)
        .map_err(|error| format!("repository file list was not UTF-8: {error}"))
}

/// Locate the repository root from the package directory so the integration
/// test can run from Cargo's target directory without depending on cwd.
fn repository_root() -> Result<PathBuf, String> {
    let mut current = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    loop {
        if current
            .join(".git")
            .exists()
        {
            return Ok(current);
        }
        if !current.pop() {
            return Err(
                "could not find repository root from Cargo manifest".to_owned(),
            );
        }
    }
}

/// Accept only the exact marker plus a long nearby explanation because route
/// exceptions should be rare, intentional, and self-reviewing in code review.
fn has_route_exception(
    lines: &[&str],
    index: usize,
) -> bool {
    let start = index.saturating_sub(6);
    let Some(window) = lines.get(start..=index) else {
        return false;
    };
    let Some(marker_index) = window
        .iter()
        .rposition(
            |line| {
                // cspell:disable-next-line -- shcoenwald
                line.contains("except shcoenwald")
            },
        )
    else {
        return false;
    };
    let Some(explanation_lines) =
        lines.get(start.saturating_add(marker_index)..=index)
    else {
        return false;
    };
    let explanation = explanation_lines.join(" ");
    explanation
        .split(
            |character: char| {
                !character.is_ascii_alphabetic() && character != '\''
            },
        )
        .filter(|word| !word.is_empty())
        .count()
        >= 50
}

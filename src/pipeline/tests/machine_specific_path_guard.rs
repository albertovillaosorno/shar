// File:
//   - machine_specific_path_guard.rs
// Path:
//   - src/pipeline/tests/machine_specific_path_guard.rs
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
//   - Regression coverage for machine-specific home paths in public files.
// - Must-Not:
//   - Depend on private local outputs or a specific operator profile path.
// - Allows:
//   - Synthetic path prefixes, temporary Git repositories, and byte scans.
// - Split-When:
//   - Another machine-specific path family needs distinct parsing semantics.
// - Merge-When:
//   - Another public-file guard owns the same confidentiality invariant.
// - Summary:
//   - Rejects Windows, Linux, and macOS home paths in public files.
// - Description:
//   - Scans tracked and untracked nonignored files without extension bypasses.
// - Usage:
//   - Executed through cargo test for the pipeline crate.
// - Defaults:
//   - Findings are repository-relative and deterministically ordered.
//
// ADRs:
// - docs/adr/engineering/quality/strict-validation-and-linting.md
//
// Large file:
//   - false
//

//! Repository-level guard for machine-specific home paths in public files.

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
fn public_files_reject_machine_specific_home_paths() -> Result<(), String> {
    let root = repository_root()?;
    let failures = machine_path_failures(&root)?;
    if failures.is_empty() {
        Ok(())
    } else {
        Err(
            format!(
                "machine-specific home paths are forbidden in public files: {}",
                failures.join(", ")
            ),
        )
    }
}

#[test]
fn detects_windows_linux_and_macos_paths_in_untracked_files()
-> Result<(), String> {
    let root = std::env::temp_dir().join(
        format!(
            "shar-machine-path-guard-{}",
            std::process::id()
        ),
    );
    drop(std::fs::remove_dir_all(&root));
    let result = (|| {
        initialize_fixture_repository(&root)?;
        let prefixes = machine_path_prefixes();
        std::fs::write(
            root.join("windows.txt"),
            format!(
                "first = {}example\\project\nsecond = {}example/project\n",
                prefixes[0], prefixes[1]
            ),
        )
        .map_err(|error| format!("Windows fixture write failed: {error}"))?;
        std::fs::write(
            root.join("unix.yml"),
            format!(
                "linux: {}example/project\nmacos: {}example/project\n",
                prefixes[2], prefixes[3]
            ),
        )
        .map_err(|error| format!("Unix fixture write failed: {error}"))?;
        let failures = machine_path_failures(&root)?;
        let expected = [
            "unix.yml:1",
            "unix.yml:2",
            "windows.txt:1",
            "windows.txt:2",
        ];
        if failures == expected {
            Ok(())
        } else {
            Err(
                format!(
                    "machine-specific path fixtures were not detected: \
                     {failures:?}"
                ),
            )
        }
    })();
    drop(std::fs::remove_dir_all(&root));
    result
}

fn machine_path_failures(root: &Path) -> Result<Vec<String>, String> {
    let files = repository_files(root)?;
    let prefixes =
        machine_path_prefixes().map(|prefix| prefix.to_ascii_lowercase());
    let mut failures = Vec::new();
    for relative in files.lines() {
        let absolute = root.join(relative);
        let Ok(bytes) = std::fs::read(&absolute) else {
            continue;
        };
        let text = String::from_utf8_lossy(&bytes);
        for (index, line) in text
            .lines()
            .enumerate()
        {
            if line_has_machine_path(
                line, &prefixes,
            ) {
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

fn line_has_machine_path(
    line: &str,
    prefixes: &[String; 4],
) -> bool {
    let normalized = line.to_ascii_lowercase();
    prefixes
        .iter()
        .any(
            |prefix| {
                normalized
                    .match_indices(prefix)
                    .any(
                        |(index, _)| {
                            has_path_boundary(
                                normalized.as_bytes(),
                                index,
                            )
                        },
                    )
            },
        )
}

fn has_path_boundary(
    bytes: &[u8],
    index: usize,
) -> bool {
    index == 0
        || index
            .checked_sub(1)
            .and_then(|previous| bytes.get(previous))
            .is_some_and(|byte| is_path_boundary(*byte))
}

const fn is_path_boundary(byte: u8) -> bool {
    byte.is_ascii_whitespace()
        || matches!(
            byte,
            b'"' | b'\'' | b'`' | b'=' | b'(' | b'[' | b'{' | b':' | b'/'
        )
}

fn machine_path_prefixes() -> [String; 4] {
    let backslash = char::from(92);
    [
        format!("C:{backslash}Users{backslash}"),
        [
            "C:", "/", "Users", "/",
        ]
        .concat(),
        [
            "/", "home", "/",
        ]
        .concat(),
        [
            "/", "Users", "/",
        ]
        .concat(),
    ]
}

fn initialize_fixture_repository(root: &Path) -> Result<(), String> {
    std::fs::create_dir_all(root)
        .map_err(|error| format!("fixture create failed: {error}"))?;
    let output = Command::new("git")
        .arg("init")
        .arg("--quiet")
        .current_dir(root)
        .output()
        .map_err(|error| format!("fixture git init failed: {error}"))?;
    if output
        .status
        .success()
    {
        Ok(())
    } else {
        Err(
            format!(
                "fixture git init returned failure: {}",
                String::from_utf8_lossy(&output.stderr)
            ),
        )
    }
}

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

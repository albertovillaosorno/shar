// File:
//   - validate_cli_contract.rs
// Path:
//   - src/game-manifest/tests/validate_cli_contract.rs
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
//   - Canonical input and argument regressions for validate-game.
// - Must-Not:
//   - Read licensed inputs or repository-local generated trees.
// - Allows:
//   - Synthetic temporary manifests and compiled command execution.
// - Split-When:
//   - Split when fixture setup exceeds the focused command-contract boundary.
// - Merge-When:
//   - Another test owns the same canonical validate-game command contract.
// - Summary:
//   - Protects canonical command and manifest input validation.
// - Description:
//   - Executes validate-game against isolated synthetic directory trees.
// - Usage:
//   - Executed through cargo test for the game-manifest crate.
// - Defaults:
//   - Temporary fixtures are removed after each command invocation.
//
// ADRs:
// - docs/adr/pipeline/game-manifest-ledger.md
//
// Large file:
//   - false
//

//! Canonical validate-game command contract regression coverage.
//!
//! Synthetic manifests prove strict command behavior without reading operator
//! inputs or repository-local output trees.

use std::fs;
use std::io::{self, ErrorKind};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};

use game_manifest::{MANIFEST_FILE_NAME, kind_taxonomy_jsonl};
use schoenwald_cli as _;
use schoenwald_filesystem as _;

static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);

fn run_validator(
    manifest: &str,
    extra_argument: Option<&str>,
    empty_argument: bool,
) -> io::Result<Output> {
    let sequence = NEXT_FIXTURE.fetch_add(
        1,
        Ordering::Relaxed,
    );
    let root = std::env::temp_dir().join(
        format!(
            "game-manifest-contract-{}-{sequence}",
            std::process::id()
        ),
    );
    match fs::remove_dir_all(&root) {
        Ok(()) => {}
        Err(error) if error.kind() == ErrorKind::NotFound => {}
        Err(error) => return Err(error),
    }
    fs::create_dir_all(&root)?;
    let result = (|| {
        fs::write(
            root.join(MANIFEST_FILE_NAME),
            manifest,
        )?;
        let mut command = Command::new(env!("CARGO_BIN_EXE_validate-game"));
        if empty_argument {
            let _current_dir = command.current_dir(&root);
            let _argument = command.arg("");
        } else {
            let _argument = command.arg(&root);
        }
        if let Some(extra) = extra_argument {
            let _extra = command.arg(extra);
        }
        command.output()
    })();
    drop(fs::remove_dir_all(&root));
    result
}

fn validate_manifest(manifest: &str) -> io::Result<Output> {
    run_validator(
        manifest, None, false,
    )
}

#[test]
fn validator_rejects_missing_final_newline() {
    let row =
        "{\"dir\":\"\",\"ext\":\"lmlm\",\"min\":0,\"kind\":\"language_mod\"}";
    let manifest = format!(
        "{}\n{row}",
        kind_taxonomy_jsonl()
    );
    let result = validate_manifest(&manifest);
    assert!(result.is_ok());
    let Some(output) = result.ok() else {
        return;
    };
    assert!(
        !output
            .status
            .success()
    );
}

#[test]
fn validator_rejects_mismatched_kind() {
    let row = "{\"dir\":\"\",\"ext\":\"lmlm\",\"min\":0,\"kind\":\"audio\"}";
    let manifest = format!(
        "{}\n{row}\n",
        kind_taxonomy_jsonl()
    );
    let result = validate_manifest(&manifest);
    assert!(result.is_ok());
    let Some(output) = result.ok() else {
        return;
    };
    assert!(
        !output
            .status
            .success()
    );
}

#[test]
fn validator_rejects_error_classification_before_counting() {
    let row =
        "{\"dir\":\"aa\",\"ext\":\"mystery\",\"min\":1,\"kind\":\"error\"}";
    let manifest = format!(
        "{}\n{row}\n",
        kind_taxonomy_jsonl()
    );
    let result = validate_manifest(&manifest);
    assert!(result.is_ok());
    let Some(output) = result.ok() else {
        return;
    };
    assert!(
        !output
            .status
            .success()
    );
    assert!(
        String::from_utf8_lossy(&output.stderr)
            .contains("unclassified coordinate")
    );
}

#[test]
fn validator_rejects_extra_arguments() {
    let row =
        "{\"dir\":\"\",\"ext\":\"lmlm\",\"min\":0,\"kind\":\"language_mod\"}";
    let manifest = format!(
        "{}\n{row}\n",
        kind_taxonomy_jsonl()
    );
    let result = run_validator(
        &manifest,
        Some("unexpected"),
        false,
    );
    assert!(result.is_ok());
    let Some(output) = result.ok() else {
        return;
    };
    assert!(
        !output
            .status
            .success()
    );
}

#[test]
fn validator_rejects_crlf_line_endings() {
    let row =
        "{\"dir\":\"\",\"ext\":\"lmlm\",\"min\":0,\"kind\":\"language_mod\"}";
    let manifest = format!(
        "{}\r\n{row}\r\n",
        kind_taxonomy_jsonl()
    );
    let result = validate_manifest(&manifest);
    assert!(result.is_ok());
    let Some(output) = result.ok() else {
        return;
    };
    assert!(
        !output
            .status
            .success()
    );
}

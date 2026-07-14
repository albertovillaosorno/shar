// File:
//   - validate_cli.rs
// Path:
//   - src/game-manifest/tests/validate_cli.rs
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
//   - End-to-end validator command regressions.
// - Must-Not:
//   - Read licensed inputs or repository-local generated trees.
// - Allows:
//   - Synthetic temporary manifests and compiled validator execution.
// - Split-When:
//   - Split when argument and schema validation need independent fixtures.
// - Merge-When:
//   - Another test owns the same validator process boundary.
// - Summary:
//   - Protects fail-closed validator behavior.
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

//! End-to-end validator command regression coverage.
//!
//! Synthetic manifests prove command exit behavior without reading operator
//! inputs or repository-local output trees.

use std::fs;
use std::io::{self, ErrorKind};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};

use game_manifest::{MANIFEST_FILE_NAME, kind_taxonomy_jsonl};
use schoenwald_cli as _;
use schoenwald_filesystem as _;

static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);

fn validate_manifest(manifest: &str) -> io::Result<Output> {
    let sequence = NEXT_FIXTURE.fetch_add(
        1,
        Ordering::Relaxed,
    );
    let root = std::env::temp_dir().join(
        format!(
            "game-manifest-validate-{}-{sequence}",
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
        Command::new(env!("CARGO_BIN_EXE_validate-game"))
            .arg(&root)
            .output()
    })();
    drop(fs::remove_dir_all(&root));
    result
}

#[test]
fn validator_rejects_empty_manifest() {
    let result = validate_manifest("");
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
fn validator_rejects_malformed_rows() {
    let manifest = format!(
        "{}
{{\"dir\":\"\",\"ext\":\"lmlm\",\"min\":0,\"kind\":\"language_mod\"}}
not-json
",
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
fn validator_requires_current_taxonomy_header() {
    let row =
        "{\"dir\":\"\",\"ext\":\"lmlm\",\"min\":0,\"kind\":\"language_mod\"}";
    for manifest in [
        row.to_owned(),
        format!("{{\"kind_taxonomy\":[]}}\n{row}"),
    ] {
        let result = validate_manifest(&manifest);
        assert!(result.is_ok());
        let Some(output) = result.ok() else {
            continue;
        };
        assert!(
            !output
                .status
                .success()
        );
    }
}

#[test]
fn validator_rejects_duplicate_coordinates() {
    let row =
        "{\"dir\":\"\",\"ext\":\"lmlm\",\"min\":0,\"kind\":\"language_mod\"}";
    let manifest = format!(
        "{}\n{row}\n{row}\n",
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
fn validator_rejects_unsorted_coordinates() {
    let first = "{\"dir\":\"\",\"ext\":\"png\",\"min\":0,\"kind\":\"\
                 generated_artifact\"}";
    let second =
        "{\"dir\":\"\",\"ext\":\"lmlm\",\"min\":0,\"kind\":\"language_mod\"}";
    let manifest = format!(
        "{}\n{first}\n{second}\n",
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
fn validator_rejects_zero_required_minimums() {
    let row =
        "{\"dir\":\"aa\",\"ext\":\"p3d\",\"min\":0,\"kind\":\"p3d_container\"}";
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
fn validator_requires_language_mod_root_coordinate() {
    let row = "{\"dir\":\"\",\"ext\":\"png\",\"min\":0,\"kind\":\"\
               generated_artifact\"}";
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
fn validator_requires_generated_image_root_coordinate() {
    let row =
        "{\"dir\":\"\",\"ext\":\"lmlm\",\"min\":0,\"kind\":\"language_mod\"}";
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

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
//   - Validate cli contract test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Validate cli contract test module.
// - Description:
//   - Implements the declared test module responsibility for manifest.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Validate cli contract test module.

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
    let sequence = NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed);
    let root = std::env::temp_dir().join(format!(
        "game-manifest-contract-{}-{sequence}",
        std::process::id()
    ));
    match fs::remove_dir_all(&root) {
        Ok(()) => {},
        Err(error) if error.kind() == ErrorKind::NotFound => {},
        Err(error) => return Err(error),
    }
    fs::create_dir_all(&root)?;
    let result = (|| {
        fs::create_dir_all(root.join("manifest"))?;
        fs::write(root.join("Simpsons.exe"), b"fixture")?;
        fs::write(root.join("Simpsons.ico"), b"fixture")?;
        fs::write(root.join("README.rtf"), b"fixture")?;
        fs::write(root.join("dialog.rcf"), b"fixture")?;
        let english = root.join("art/frontend/scrooby2/resource/txtbible");
        fs::create_dir_all(&english)?;
        fs::write(english.join("srr2.E"), b"fixture")?;
        fs::write(english.join("srr2.txt"), b"fixture")?;
        fs::write(root.join(MANIFEST_FILE_NAME), manifest)?;
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
    run_validator(manifest, None, false)
}

#[test]
fn validator_rejects_missing_final_newline() {
    let row =
        "{\"dir\":\"\",\"ext\":\"png\",\"min\":0,\"kind\":\"generated_artifact\"}";
    let manifest = format!("{}\n{row}", kind_taxonomy_jsonl());
    let result = validate_manifest(&manifest);
    assert!(result.is_ok());
    let Some(output) = result.ok() else {
        return;
    };
    assert!(!output.status.success());
}

#[test]
fn validator_rejects_mismatched_kind() {
    let row = "{\"dir\":\"\",\"ext\":\"png\",\"min\":0,\"kind\":\"audio\"}";
    let manifest = format!("{}\n{row}\n", kind_taxonomy_jsonl());
    let result = validate_manifest(&manifest);
    assert!(result.is_ok());
    let Some(output) = result.ok() else {
        return;
    };
    assert!(!output.status.success());
}

#[test]
fn validator_rejects_error_classification_before_counting() {
    let row =
        "{\"dir\":\"aa\",\"ext\":\"mystery\",\"min\":1,\"kind\":\"error\"}";
    let manifest = format!("{}\n{row}\n", kind_taxonomy_jsonl());
    let result = validate_manifest(&manifest);
    assert!(result.is_ok());
    let Some(output) = result.ok() else {
        return;
    };
    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr)
            .contains("unclassified coordinate")
    );
}

#[test]
fn validator_rejects_extra_arguments() {
    let row =
        "{\"dir\":\"\",\"ext\":\"png\",\"min\":0,\"kind\":\"generated_artifact\"}";
    let manifest = format!("{}\n{row}\n", kind_taxonomy_jsonl());
    let result = run_validator(&manifest, Some("unexpected"), false);
    assert!(result.is_ok());
    let Some(output) = result.ok() else {
        return;
    };
    assert!(!output.status.success());
}

#[test]
fn validator_rejects_crlf_line_endings() {
    let row =
        "{\"dir\":\"\",\"ext\":\"png\",\"min\":0,\"kind\":\"generated_artifact\"}";
    let manifest = format!("{}\r\n{row}\r\n", kind_taxonomy_jsonl());
    let result = validate_manifest(&manifest);
    assert!(result.is_ok());
    let Some(output) = result.ok() else {
        return;
    };
    assert!(!output.status.success());
}

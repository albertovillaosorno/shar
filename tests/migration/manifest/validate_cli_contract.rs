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

fn run_validator_with_executable(
    manifest: &str,
    extra_argument: Option<&str>,
    empty_argument: bool,
    executable_bytes: &[u8],
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
        fs::write(root.join("Simpsons.exe"), executable_bytes)?;
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
            let _manifest = command.arg(root.join(MANIFEST_FILE_NAME));
            let _extra = command.arg(extra);
        }
        command.output()
    })();
    drop(fs::remove_dir_all(&root));
    result
}

fn run_validator(
    manifest: &str,
    extra_argument: Option<&str>,
    empty_argument: bool,
) -> io::Result<Output> {
    run_validator_with_executable(
        manifest,
        extra_argument,
        empty_argument,
        b"fixture",
    )
}

fn validate_manifest(manifest: &str) -> io::Result<Output> {
    run_validator(manifest, None, false)
}

#[test]
fn executable_requirement_is_path_not_byte_identity() -> io::Result<()> {
    let row = concat!(
        "{\"dir\":\"\",\"ext\":\"png\",\"min\":0,",
        "\"kind\":\"generated_artifact\"}"
    );
    let manifest = format!("{}\n{row}\n", kind_taxonomy_jsonl());

    let variants = [b"edition-a".as_slice(), b"different-edition".as_slice()];
    for executable in variants {
        let output = run_validator_with_executable(
            &manifest,
            None,
            false,
            executable,
        )?;
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(())
}

#[test]
fn validator_rejects_missing_final_newline() {
    // jig-ignore-next-line: literal
    let row = "{\"dir\":\"\",\"ext\":\"png\",\"min\":0,\"kind\":\"generated_artifact\"}";
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
    // jig-ignore-next-line: literal
    let row = "{\"dir\":\"\",\"ext\":\"png\",\"min\":0,\"kind\":\"generated_artifact\"}";
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
    // jig-ignore-next-line: literal
    let row = "{\"dir\":\"\",\"ext\":\"png\",\"min\":0,\"kind\":\"generated_artifact\"}";
    let manifest = format!("{}\r\n{row}\r\n", kind_taxonomy_jsonl());
    let result = validate_manifest(&manifest);
    assert!(result.is_ok());
    let Some(output) = result.ok() else {
        return;
    };
    assert!(!output.status.success());
}

#[test]
fn validator_accepts_manifest_outside_source_tree() -> io::Result<()> {
    let sequence = NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed);
    let root = std::env::temp_dir().join(format!(
        "game-manifest-external-policy-{}-{sequence}",
        std::process::id()
    ));
    match fs::remove_dir_all(&root) {
        Ok(()) => {},
        Err(error) if error.kind() == ErrorKind::NotFound => {},
        Err(error) => return Err(error),
    }
    let source = root.join("source");
    let policy = root.join("policy/game.jsonl");
    fs::create_dir_all(source.join("art/frontend/scrooby2/resource/txtbible"))?;
    fs::create_dir_all(
        policy
            .parent()
            .ok_or_else(|| io::Error::other("policy path has no parent"))?,
    )?;
    fs::write(source.join("Simpsons.exe"), b"fixture")?;
    fs::write(source.join("Simpsons.ico"), b"fixture")?;
    fs::write(source.join("README.rtf"), b"fixture")?;
    fs::write(source.join("dialog.rcf"), b"fixture")?;
    let english = source.join("art/frontend/scrooby2/resource/txtbible");
    fs::write(english.join("srr2.E"), b"fixture")?;
    fs::write(english.join("srr2.txt"), b"fixture")?;
    let row = concat!(
        "{\"dir\":\"\",\"ext\":\"png\",\"min\":0,",
        "\"kind\":\"generated_artifact\"}"
    );
    let manifest = format!("{}\n{row}\n", kind_taxonomy_jsonl());
    fs::write(&policy, manifest)?;

    let output = Command::new(env!("CARGO_BIN_EXE_validate-game"))
        .arg(&source)
        .arg(&policy)
        .output()?;
    let source_manifest = source.join("manifest");
    let result = if output.status.success() && !source_manifest.exists() {
        Ok(())
    } else {
        Err(io::Error::other(format!(
            "external manifest validation failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )))
    };
    drop(fs::remove_dir_all(&root));
    result
}

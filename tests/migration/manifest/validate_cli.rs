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
//   - Validate cli test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Validate cli test module.
// - Description:
//   - Implements the declared test module responsibility for manifest.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Validate cli test module.

use std::fs;
use std::io::{self, ErrorKind};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};

use game_manifest::{MANIFEST_FILE_NAME, kind_taxonomy_jsonl};
use schoenwald_cli as _;
use schoenwald_filesystem as _;

static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);

fn validate_manifest(manifest: &str) -> io::Result<Output> {
    let sequence = NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed);
    let root = std::env::temp_dir().join(format!(
        "game-manifest-validate-{}-{sequence}",
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
    assert!(!output.status.success());
}

#[test]
fn validator_rejects_malformed_rows() {
    let manifest = format!(
        "{}
{{\"dir\":\"\",\"ext\":\"png\",\"min\":0,\"kind\":\"generated_artifact\"}}
not-json
",
        kind_taxonomy_jsonl()
    );
    let result = validate_manifest(&manifest);
    assert!(result.is_ok());
    let Some(output) = result.ok() else {
        return;
    };
    assert!(!output.status.success());
}

#[test]
fn validator_requires_current_taxonomy_header() {
    // jig-ignore-next-line: literal
    let row = "{\"dir\":\"\",\"ext\":\"png\",\"min\":0,\"kind\":\"generated_artifact\"}";
    for manifest in [row.to_owned(), format!("{{\"kind_taxonomy\":[]}}\n{row}")]
    {
        let result = validate_manifest(&manifest);
        assert!(result.is_ok());
        let Some(output) = result.ok() else {
            continue;
        };
        assert!(!output.status.success());
    }
}

#[test]
fn validator_rejects_duplicate_coordinates() {
    // jig-ignore-next-line: literal
    let row = "{\"dir\":\"\",\"ext\":\"png\",\"min\":0,\"kind\":\"generated_artifact\"}";
    let manifest = format!("{}\n{row}\n{row}\n", kind_taxonomy_jsonl());
    let result = validate_manifest(&manifest);
    assert!(result.is_ok());
    let Some(output) = result.ok() else {
        return;
    };
    assert!(!output.status.success());
}

#[test]
fn validator_rejects_unsorted_coordinates() {
    let first =
        "{\"dir\":\"aa\",\"ext\":\"p3d\",\"min\":0,\"kind\":\"p3d_container\"}";
    // jig-ignore-next-line: literal
    let second = "{\"dir\":\"\",\"ext\":\"png\",\"min\":0,\"kind\":\"generated_artifact\"}";
    let manifest = format!(
        "{}
{first}
{second}
",
        kind_taxonomy_jsonl()
    );
    let result = validate_manifest(&manifest);
    assert!(result.is_ok());
    let Some(output) = result.ok() else {
        return;
    };
    assert!(!output.status.success());
}

#[test]
fn validator_accepts_zero_minimum_as_optional() {
    // jig-ignore-next-line: literal
    let generated_image = "{\"dir\":\"\",\"ext\":\"png\",\"min\":0,\"kind\":\"generated_artifact\"}";
    let optional =
        "{\"dir\":\"aa\",\"ext\":\"p3d\",\"min\":0,\"kind\":\"p3d_container\"}";
    let manifest = format!(
        "{}
{generated_image}
{optional}
",
        kind_taxonomy_jsonl()
    );
    let result = validate_manifest(&manifest);
    assert!(result.is_ok());
    let Some(output) = result.ok() else {
        return;
    };
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn validator_requires_generated_image_root_coordinate() {
    let row =
        "{\"dir\":\"aa\",\"ext\":\"p3d\",\"min\":0,\"kind\":\"p3d_container\"}";
    let manifest = format!("{}\n{row}\n", kind_taxonomy_jsonl());
    let result = validate_manifest(&manifest);
    assert!(result.is_ok());
    let Some(output) = result.ok() else {
        return;
    };
    assert!(!output.status.success());
}

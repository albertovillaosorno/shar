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
//   - Expanded output contract test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Expanded output contract test module.
// - Description:
//   - Implements the declared test module responsibility for manifest.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Expanded output contract test module.

use std::fs;
use std::io::{self, ErrorKind};
use std::path::Path;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

use game_manifest::{EXPANDED_SCHEMA_LINE, MANIFEST_FILE_NAME};
use schoenwald_cli as _;
use schoenwald_filesystem as _;

static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);

fn with_fixture<T>(
    label: &str,
    operation: impl FnOnce(&Path, &Path, &Path) -> io::Result<T>,
) -> io::Result<T> {
    let sequence = NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed);
    let root = std::env::temp_dir().join(format!(
        "game-manifest-output-{label}-{}-{sequence}",
        std::process::id()
    ));
    match fs::remove_dir_all(&root) {
        Ok(()) => {},
        Err(error) if error.kind() == ErrorKind::NotFound => {},
        Err(error) => return Err(error),
    }
    let game = root.join("input");
    let extracted = root.join("decoded");
    fs::create_dir_all(&game)?;
    fs::create_dir_all(&extracted)?;
    let result = operation(&root, &game, &extracted);
    drop(fs::remove_dir_all(&root));
    result
}

fn run_generator(
    game: &Path,
    extracted: &Path,
    output: &Path,
) -> io::Result<std::process::Output> {
    Command::new(env!("CARGO_BIN_EXE_generate-expanded-manifest"))
        .arg(game)
        .arg(extracted)
        .arg(output)
        .output()
}

#[test]
fn custom_output_does_not_self_include() {
    let result = with_fixture("self-include", |_, game, extracted| {
        fs::write(game.join("asset.p3d"), b"fixture")?;
        let output_path = game.join("custom.jsonl");
        let first = run_generator(game, extracted, &output_path)?;
        let first_manifest = fs::read_to_string(&output_path)?;
        let second = run_generator(game, extracted, &output_path)?;
        let second_manifest = fs::read_to_string(&output_path)?;
        Ok((first, second, first_manifest, second_manifest))
    });
    assert!(result.is_ok());
    let Some((first, second, first_manifest, second_manifest)) = result.ok()
    else {
        return;
    };
    assert!(first.status.success());
    assert!(second.status.success());
    assert_eq!(first_manifest, second_manifest);
}

#[test]
fn nested_manifest_names_remain_source_files() {
    let result = with_fixture("nested-manifests", |root, game, extracted| {
        let nested = game.join("area");
        fs::create_dir_all(&nested)?;
        fs::write(nested.join("manifest.jsonl"), b"fixture")?;
        fs::write(nested.join("expanded-manifest.jsonl"), b"fixture")?;
        let output_path = root.join("result.jsonl");
        let output = run_generator(game, extracted, &output_path)?;
        let manifest = fs::read_to_string(output_path)?;
        Ok((output, manifest))
    });
    assert!(result.is_ok());
    let Some((output, manifest)) = result.ok() else {
        return;
    };
    assert!(output.status.success());
    assert!(manifest.contains("area/manifest.jsonl"));
    assert!(manifest.contains("area/expanded-manifest.jsonl"));
}

#[test]
fn expanded_output_cannot_replace_minimum_manifest() {
    let result = with_fixture("minimum-overwrite", |_, game, extracted| {
        fs::write(game.join("asset.p3d"), b"fixture")?;
        let output_path = game.join(MANIFEST_FILE_NAME);
        fs::create_dir_all(output_path.parent().ok_or_else(|| {
            io::Error::other("minimum manifest has no parent")
        })?)?;
        fs::write(&output_path, b"minimum-ledger")?;
        let output = run_generator(game, extracted, &output_path)?;
        let remaining = fs::read_to_string(output_path)?;
        Ok((output, remaining))
    });
    assert!(result.is_ok());
    let Some((output, remaining)) = result.ok() else {
        return;
    };
    assert!(!output.status.success());
    assert_eq!(remaining, "minimum-ledger");
}

#[test]
fn expanded_output_alias_cannot_replace_minimum_manifest() {
    let result =
        with_fixture("minimum-alias-overwrite", |_, game, extracted| {
            fs::create_dir_all(game.join("sub"))?;
            fs::write(game.join("asset.p3d"), b"fixture")?;
            let minimum_path = game.join(MANIFEST_FILE_NAME);
            fs::create_dir_all(minimum_path.parent().ok_or_else(|| {
                io::Error::other("minimum manifest has no parent")
            })?)?;
            let original = format!(
                "minimum-ledger
{EXPANDED_SCHEMA_LINE}
",
            );
            fs::write(&minimum_path, original.as_bytes())?;
            let aliased_output =
                game.join("sub").join("..").join(MANIFEST_FILE_NAME);
            let output = run_generator(game, extracted, &aliased_output)?;
            let remaining = fs::read_to_string(minimum_path)?;
            Ok((output, remaining, original))
        });
    assert!(result.is_ok());
    let Some((output, remaining, original)) = result.ok() else {
        return;
    };
    assert!(!output.status.success());
    assert_eq!(remaining, original);
}

#[test]
fn expanded_output_requires_jsonl_extension() {
    let result = with_fixture("source-overwrite", |_, game, extracted| {
        let output_path = game.join("asset.p3d");
        fs::write(&output_path, b"source-content")?;
        let output = run_generator(game, extracted, &output_path)?;
        let remaining = fs::read(&output_path)?;
        Ok((output, remaining))
    });
    assert!(result.is_ok());
    let Some((output, remaining)) = result.ok() else {
        return;
    };
    assert!(!output.status.success());
    assert_eq!(remaining, b"source-content");
}

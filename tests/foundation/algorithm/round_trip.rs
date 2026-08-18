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
//   - External source-binding, tamper, and replay regression evidence.
// - Must-Not:
//   - Depend on proprietary source data or mutate caller source inputs.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - One responsibility gains an independent lifecycle.
// - Merge-When:
//   - Another module owns the identical responsibility.
// - Summary:
//   - Algorithm external round-trip tests.
// - Description:
//   - External source-binding, tamper, and replay regression evidence.
// - Usage:
//   - Used through the owning algorithm function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! External round-trip and tamper tests for the generic algorithm engine.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use chacha20poly1305 as _;
use same_file as _;
use schoenwald_cli as _;
use schoenwald_filesystem as _;
use serde as _;
use serde_json as _;
use shar_algorithm::{Settings, create_algorithm, replay_algorithm};
use shar_sha256 as _;

static TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(0);

struct TempTree {
    path: PathBuf,
}

impl TempTree {
    fn create(label: &str) -> Result<Self, std::io::Error> {
        let sequence = TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "shar-algorithm-{label}-{}-{sequence}",
            std::process::id()
        ));
        if path.exists() {
            fs::remove_dir_all(&path)?;
        }
        fs::create_dir_all(&path)?;
        Ok(Self { path })
    }
}

impl Drop for TempTree {
    fn drop(&mut self) {
        let _cleanup_result = fs::remove_dir_all(&self.path);
    }
}

fn settings() -> Result<Settings, Box<dyn std::error::Error>> {
    let text = r#"{
      "schema":"shar.algorithm.settings.v1",
      "minimum_source_files":1,
      "minimum_source_bytes":1024,
      "maximum_source_files":16,
      "maximum_target_files":16,
      "maximum_file_bytes":1048576,
      "maximum_source_bytes":4194304,
      "maximum_target_bytes":4194304
    }"#;
    Ok(Settings::from_json(text)?)
}

fn write_fixture_tree(root: &Path) -> Result<(PathBuf, PathBuf), std::io::Error> {
    let source = root.join("source");
    let target = root.join("target");
    fs::create_dir_all(source.join("nested"))?;
    fs::create_dir_all(target.join("nested"))?;
    fs::write(source.join("one.bin"), vec![0x31_u8; 1536])?;
    fs::write(source.join("nested").join("two.bin"), vec![0x57_u8; 768])?;
    fs::write(target.join("alpha.txt"), b"synthetic target alpha\n")?;
    fs::write(
        target.join("nested").join("beta.bin"),
        [0_u8, 1, 2, 3, 250, 251, 252, 253],
    )?;
    Ok((source, target))
}

fn assert_tree_equal(left: &Path, right: &Path) -> Result<(), Box<dyn std::error::Error>> {
    for relative in ["alpha.txt", "nested/beta.bin"] {
        let left_bytes = fs::read(left.join(relative))?;
        let right_bytes = fs::read(right.join(relative))?;
        if left_bytes != right_bytes {
            return Err(format!("replayed file differs: {relative}").into());
        }
    }
    Ok(())
}

fn run_directory_round_trip_is_deterministic() -> Result<(), Box<dyn std::error::Error>> {
    let temp = TempTree::create("round-trip")?;
    let (source, target) = write_fixture_tree(&temp.path)?;
    let first = temp.path.join("first.txt");
    let second = temp.path.join("second.txt");
    let replayed = temp.path.join("replayed");
    let settings = settings()?;

    create_algorithm(&settings, std::slice::from_ref(&source), &target, &first)?;
    create_algorithm(&settings, std::slice::from_ref(&source), &target, &second)?;
    if fs::read(&first)? != fs::read(&second)? {
        return Err("identical inputs must create byte-identical algorithms".into());
    }
    let plan_text = fs::read_to_string(&first)?;
    if plan_text.contains("synthetic target alpha") {
        return Err("protected target plaintext leaked into algorithm text".into());
    }

    replay_algorithm(&settings, std::slice::from_ref(&source), &first, &replayed)?;
    assert_tree_equal(&target, &replayed)
}

#[test]
fn directory_round_trip_is_deterministic() {
    let result = run_directory_round_trip_is_deterministic();
    assert!(result.is_ok(), "directory round trip failed: {result:?}");
}

fn run_wrong_source_is_rejected() -> Result<(), Box<dyn std::error::Error>> {
    let temp = TempTree::create("wrong-source")?;
    let (source, target) = write_fixture_tree(&temp.path)?;
    let algorithm = temp.path.join("plan.txt");
    let output = temp.path.join("output");
    let settings = settings()?;
    create_algorithm(
        &settings,
        std::slice::from_ref(&source),
        &target,
        &algorithm,
    )?;
    fs::write(source.join("one.bin"), vec![0x32_u8; 1536])?;

    let result = replay_algorithm(
        &settings,
        std::slice::from_ref(&source),
        &algorithm,
        &output,
    );
    if result.is_ok() {
        return Err("wrong source must not replay".into());
    }
    if output.exists() {
        return Err("wrong source must not create replay output".into());
    }
    Ok(())
}

#[test]
fn wrong_source_is_rejected_before_output() {
    let result = run_wrong_source_is_rejected();
    assert!(result.is_ok(), "wrong-source rejection failed: {result:?}");
}

fn run_plan_without_source_is_rejected(
) -> Result<(), Box<dyn std::error::Error>> {
    let temp = TempTree::create("missing-source")?;
    let (source, target) = write_fixture_tree(&temp.path)?;
    let algorithm = temp.path.join("plan.txt");
    let output = temp.path.join("output");
    let settings = settings()?;
    create_algorithm(
        &settings,
        std::slice::from_ref(&source),
        &target,
        &algorithm,
    )?;

    let result = replay_algorithm(&settings, &[], &algorithm, &output);
    if result.is_ok() {
        return Err("plan without caller source must not replay".into());
    }
    if output.exists() {
        return Err("missing source must not create replay output".into());
    }
    Ok(())
}

#[test]
fn plan_without_caller_source_is_rejected_before_output() {
    let result = run_plan_without_source_is_rejected();
    assert!(result.is_ok(), "missing-source rejection failed: {result:?}");
}

fn run_source_tree_remains_unchanged(
) -> Result<(), Box<dyn std::error::Error>> {
    let temp = TempTree::create("source-unchanged")?;
    let (source, target) = write_fixture_tree(&temp.path)?;
    let algorithm = temp.path.join("plan.txt");
    let output = temp.path.join("output");
    let before_one = fs::read(source.join("one.bin"))?;
    let before_two = fs::read(source.join("nested").join("two.bin"))?;
    let settings = settings()?;

    create_algorithm(
        &settings,
        std::slice::from_ref(&source),
        &target,
        &algorithm,
    )?;
    replay_algorithm(
        &settings,
        std::slice::from_ref(&source),
        &algorithm,
        &output,
    )?;

    if fs::read(source.join("one.bin"))? != before_one
        || fs::read(source.join("nested").join("two.bin"))? != before_two
    {
        return Err("algorithm execution modified caller source bytes".into());
    }
    let source_entries = [
        source.join("one.bin"),
        source.join("nested"),
        source.join("nested").join("two.bin"),
    ];
    for entry in source_entries {
        if !entry.exists() {
            return Err("algorithm execution changed caller source layout".into());
        }
    }
    Ok(())
}

#[test]
fn caller_source_tree_remains_unchanged_after_create_and_replay() {
    let result = run_source_tree_remains_unchanged();
    assert!(result.is_ok(), "source immutability failed: {result:?}");
}

fn tamper_first_ciphertext(text: &str) -> Result<String, Box<dyn std::error::Error>> {
    let marker = "\"ciphertext\": \"";
    let Some(marker_start) = text.find(marker) else {
        return Err("ciphertext marker missing".into());
    };
    let value_start = marker_start.saturating_add(marker.len());
    let Some(original) = text.as_bytes().get(value_start).copied() else {
        return Err("ciphertext value missing".into());
    };
    let replacement = if original == b'0' { '1' } else { '0' };
    let mut tampered = text.to_owned();
    tampered.replace_range(
        value_start..value_start.saturating_add(1),
        &replacement.to_string(),
    );
    Ok(tampered)
}

fn run_tampered_payload_is_rejected() -> Result<(), Box<dyn std::error::Error>> {
    let temp = TempTree::create("tamper")?;
    let (source, target) = write_fixture_tree(&temp.path)?;
    let algorithm = temp.path.join("plan.txt");
    let output = temp.path.join("output");
    let settings = settings()?;
    create_algorithm(
        &settings,
        std::slice::from_ref(&source),
        &target,
        &algorithm,
    )?;
    let text = fs::read_to_string(&algorithm)?;
    fs::write(&algorithm, tamper_first_ciphertext(&text)?)?;

    let result = replay_algorithm(
        &settings,
        std::slice::from_ref(&source),
        &algorithm,
        &output,
    );
    if result.is_ok() {
        return Err("tampered protected payload must not replay".into());
    }
    if output.exists() {
        return Err("tampered payload must not create replay output".into());
    }
    Ok(())
}

#[test]
fn tampered_payload_is_rejected_before_output() {
    let result = run_tampered_payload_is_rejected();
    assert!(result.is_ok(), "tamper rejection failed: {result:?}");
}

fn run_file_target_round_trip() -> Result<(), Box<dyn std::error::Error>> {
    let temp = TempTree::create("file")?;
    let source = temp.path.join("source.bin");
    let target = temp.path.join("target.bin");
    let algorithm = temp.path.join("plan.txt");
    let output = temp.path.join("output.bin");
    fs::write(&source, vec![0xA5_u8; 2048])?;
    fs::write(&target, b"synthetic single-file target")?;
    let settings = settings()?;
    create_algorithm(
        &settings,
        std::slice::from_ref(&source),
        &target,
        &algorithm,
    )?;
    replay_algorithm(
        &settings,
        std::slice::from_ref(&source),
        &algorithm,
        &output,
    )?;
    if fs::read(target)? != fs::read(output)? {
        return Err("single-file replay differs from target".into());
    }
    Ok(())
}

#[test]
fn file_target_round_trip_is_exact() {
    let result = run_file_target_round_trip();
    assert!(result.is_ok(), "file round trip failed: {result:?}");
}

#[test]
fn settings_reject_unknown_and_inconsistent_policy() {
    let unknown = r#"{
      "schema":"shar.algorithm.settings.v1",
      "minimum_source_files":1,
      "minimum_source_bytes":1,
      "maximum_source_files":1,
      "maximum_target_files":1,
      "maximum_file_bytes":1,
      "maximum_source_bytes":1,
      "maximum_target_bytes":1,
      "product_policy":"not-generic"
    }"#;
    let inconsistent = r#"{
      "schema":"shar.algorithm.settings.v1",
      "minimum_source_files":2,
      "minimum_source_bytes":1,
      "maximum_source_files":1,
      "maximum_target_files":1,
      "maximum_file_bytes":1,
      "maximum_source_bytes":1,
      "maximum_target_bytes":1
    }"#;
    assert!(
        Settings::from_json(unknown).is_err(),
        "unknown settings fields must fail closed"
    );
    assert!(
        Settings::from_json(inconsistent).is_err(),
        "inconsistent settings limits must fail closed"
    );
}

fn run_non_txt_algorithm_output_is_rejected() -> Result<(), Box<dyn std::error::Error>> {
    let temp = TempTree::create("extension")?;
    let source = temp.path.join("source.bin");
    let target = temp.path.join("target.bin");
    let output = temp.path.join("plan.json");
    fs::write(&source, vec![0x44_u8; 2048])?;
    fs::write(&target, b"synthetic target")?;
    let result = create_algorithm(
        &settings()?,
        std::slice::from_ref(&source),
        &target,
        &output,
    );
    if result.is_ok() {
        return Err("non-txt algorithm output must be rejected".into());
    }
    if output.exists() {
        return Err("rejected algorithm output must not be created".into());
    }
    Ok(())
}

#[test]
fn non_txt_algorithm_output_is_rejected() {
    let result = run_non_txt_algorithm_output_is_rejected();
    assert!(result.is_ok(), "extension rejection failed: {result:?}");
}

fn run_existing_algorithm_output_is_preserved(
) -> Result<(), Box<dyn std::error::Error>> {
    let temp = TempTree::create("existing-algorithm-output")?;
    let source = temp.path.join("source.bin");
    let target = temp.path.join("target.bin");
    let algorithm = temp.path.join("plan.txt");
    let sentinel = b"preserve existing plan\n";
    fs::write(&source, vec![0x48_u8; 2048])?;
    fs::write(&target, b"synthetic target")?;
    fs::write(&algorithm, sentinel)?;

    let result = create_algorithm(
        &settings()?,
        std::slice::from_ref(&source),
        &target,
        &algorithm,
    );
    if result.is_ok() {
        return Err("existing algorithm output was replaced".into());
    }
    if fs::read(&algorithm)? != sentinel {
        return Err("existing algorithm output was modified".into());
    }
    Ok(())
}

#[test]
fn existing_algorithm_output_collision_is_preserved() {
    let result = run_existing_algorithm_output_is_preserved();
    assert!(
        result.is_ok(),
        "algorithm output collision handling failed: {result:?}"
    );
}

fn run_missing_source_error_is_path_free(
) -> Result<(), Box<dyn std::error::Error>> {
    let temp = TempTree::create("private-source-diagnostic")?;
    let source = temp.path.join("private-user-installation").join("missing.bin");
    let target = temp.path.join("target.bin");
    let algorithm = temp.path.join("plan.txt");
    fs::write(&target, b"synthetic target")?;

    let result = create_algorithm(
        &settings()?,
        std::slice::from_ref(&source),
        &target,
        &algorithm,
    );
    let Err(error) = result else {
        return Err("missing source path was accepted".into());
    };
    let message = error.to_string();
    if message.contains(&source.display().to_string())
        || message.contains("private-user-installation")
    {
        return Err("algorithm error disclosed the private source path".into());
    }
    if !message.contains("input") {
        return Err("algorithm error lost its operation context".into());
    }
    Ok(())
}

#[test]
fn missing_source_error_does_not_disclose_private_path() {
    let result = run_missing_source_error_is_path_free();
    assert!(result.is_ok(), "private-path diagnostic failed: {result:?}");
}

fn run_algorithm_output_inside_source_is_rejected() -> Result<(), Box<dyn std::error::Error>> {
    let temp = TempTree::create("source-overlap")?;
    let source = temp.path.join("source");
    let target = temp.path.join("target.bin");
    let algorithm = source.join("plan.txt");
    fs::create_dir_all(&source)?;
    fs::write(source.join("evidence.bin"), vec![0x61_u8; 2048])?;
    fs::write(&target, b"synthetic target")?;

    let result = create_algorithm(
        &settings()?,
        std::slice::from_ref(&source),
        &target,
        &algorithm,
    );
    if result.is_ok() {
        return Err("algorithm output inside source must be rejected".into());
    }
    if algorithm.exists() {
        return Err("rejected source-overlap output must not be created".into());
    }
    Ok(())
}

#[test]
fn algorithm_output_inside_source_is_rejected() {
    let result = run_algorithm_output_inside_source_is_rejected();
    assert!(
        result.is_ok(),
        "source-overlap rejection failed: {result:?}"
    );
}

fn run_source_target_overlap_is_rejected() -> Result<(), Box<dyn std::error::Error>> {
    let temp = TempTree::create("source-target-overlap")?;
    let (source, _target) = write_fixture_tree(&temp.path)?;
    let algorithm = temp.path.join("plan.txt");

    let result = create_algorithm(
        &settings()?,
        std::slice::from_ref(&source),
        &source,
        &algorithm,
    );
    if result.is_ok() {
        return Err("source tree must not be accepted as algorithm target".into());
    }
    if algorithm.exists() {
        return Err("source-target overlap must not create an algorithm".into());
    }
    Ok(())
}

#[test]
fn source_target_overlap_is_rejected_before_publication() {
    let result = run_source_target_overlap_is_rejected();
    assert!(result.is_ok(), "source-target overlap rejection failed: {result:?}");
}

fn run_nested_target_overlap_is_rejected() -> Result<(), Box<dyn std::error::Error>> {
    let temp = TempTree::create("nested-target-overlap")?;
    let (source, _target) = write_fixture_tree(&temp.path)?;
    let nested_target = source.join("nested");
    let algorithm = temp.path.join("plan.txt");

    let result = create_algorithm(
        &settings()?,
        std::slice::from_ref(&source),
        &nested_target,
        &algorithm,
    );
    if result.is_ok() || algorithm.exists() {
        return Err("nested source target overlap must fail before output".into());
    }
    Ok(())
}

#[test]
fn nested_target_inside_source_is_rejected_before_publication() {
    let result = run_nested_target_overlap_is_rejected();
    assert!(result.is_ok(), "nested target overlap rejection failed: {result:?}");
}

fn run_target_parent_overlap_is_rejected() -> Result<(), Box<dyn std::error::Error>> {
    let temp = TempTree::create("target-parent-overlap")?;
    let target = temp.path.join("target");
    let source = target.join("source.bin");
    let sibling = target.join("sibling.bin");
    let algorithm = temp.path.join("plan.txt");
    fs::create_dir_all(&target)?;
    fs::write(&source, vec![0x73_u8; 2048])?;
    fs::write(&sibling, b"synthetic target sibling")?;

    let result = create_algorithm(
        &settings()?,
        std::slice::from_ref(&source),
        &target,
        &algorithm,
    );
    if result.is_ok() || algorithm.exists() {
        return Err("target containing source input must fail before output".into());
    }
    Ok(())
}

#[test]
fn target_parent_containing_source_is_rejected_before_publication() {
    let result = run_target_parent_overlap_is_rejected();
    assert!(result.is_ok(), "target parent overlap rejection failed: {result:?}");
}

fn run_hard_link_target_alias_is_rejected() -> Result<(), Box<dyn std::error::Error>> {
    let temp = TempTree::create("hard-link-target-alias")?;
    let source = temp.path.join("source.bin");
    let target = temp.path.join("target.bin");
    let algorithm = temp.path.join("plan.txt");
    fs::write(&source, vec![0x41_u8; 2048])?;
    fs::hard_link(&source, &target)?;

    let result = create_algorithm(
        &settings()?,
        std::slice::from_ref(&source),
        &target,
        &algorithm,
    );
    if result.is_ok() {
        return Err("hard-linked source alias was accepted as target".into());
    }
    if algorithm.exists() {
        return Err("hard-link source alias created an algorithm".into());
    }
    Ok(())
}

#[test]
fn hard_link_target_alias_is_rejected_before_publication() {
    let result = run_hard_link_target_alias_is_rejected();
    assert!(result.is_ok(), "hard-link target rejection failed: {result:?}");
}

fn run_duplicate_hard_link_source_is_rejected() -> Result<(), Box<dyn std::error::Error>> {
    let temp = TempTree::create("duplicate-hard-link-source")?;
    let source = temp.path.join("source");
    let target = temp.path.join("target.bin");
    let algorithm = temp.path.join("plan.txt");
    fs::create_dir_all(&source)?;
    let original = source.join("original.bin");
    let alias = source.join("alias.bin");
    fs::write(&original, vec![0x52_u8; 2048])?;
    fs::hard_link(&original, &alias)?;
    fs::write(&target, b"synthetic target")?;

    let result = create_algorithm(
        &settings()?,
        std::slice::from_ref(&source),
        &target,
        &algorithm,
    );
    if result.is_ok() {
        return Err("duplicate physical source evidence was accepted".into());
    }
    if algorithm.exists() {
        return Err("duplicate source evidence created an algorithm".into());
    }
    Ok(())
}

#[test]
fn duplicate_hard_link_source_is_rejected_before_publication() {
    let result = run_duplicate_hard_link_source_is_rejected();
    assert!(result.is_ok(), "duplicate source rejection failed: {result:?}");
}

fn run_replay_parent_traversal_is_rejected() -> Result<(), Box<dyn std::error::Error>> {
    let temp = TempTree::create("replay-parent")?;
    let source = temp.path.join("source.bin");
    let target = temp.path.join("target.bin");
    let algorithm = temp.path.join("plan.txt");
    let output = temp.path.join("safe").join("..").join("escaped.bin");
    fs::write(&source, vec![0x33_u8; 2048])?;
    fs::write(&target, b"synthetic target")?;
    let settings = settings()?;
    create_algorithm(
        &settings,
        std::slice::from_ref(&source),
        &target,
        &algorithm,
    )?;

    let result = replay_algorithm(
        &settings,
        std::slice::from_ref(&source),
        &algorithm,
        &output,
    );
    if result.is_ok() {
        return Err("parent-traversing replay output must be rejected".into());
    }
    if temp.path.join("escaped.bin").exists() {
        return Err("parent-traversing replay created escaped output".into());
    }
    Ok(())
}

#[test]
fn replay_parent_traversal_is_rejected() {
    let result = run_replay_parent_traversal_is_rejected();
    assert!(result.is_ok(), "replay path rejection failed: {result:?}");
}

fn tamper_first_target_path(text: &str) -> Result<String, Box<dyn std::error::Error>> {
    let marker = "\"path\": \"alpha.txt\"";
    if !text.contains(marker) {
        return Err("target path marker missing".into());
    }
    Ok(text.replacen(marker, "\"path\": \"../escape.txt\"", 1))
}

fn run_tampered_target_path_is_rejected() -> Result<(), Box<dyn std::error::Error>> {
    let temp = TempTree::create("target-path-tamper")?;
    let (source, target) = write_fixture_tree(&temp.path)?;
    let algorithm = temp.path.join("plan.txt");
    let output = temp.path.join("output");
    let settings = settings()?;
    create_algorithm(
        &settings,
        std::slice::from_ref(&source),
        &target,
        &algorithm,
    )?;
    let text = fs::read_to_string(&algorithm)?;
    fs::write(&algorithm, tamper_first_target_path(&text)?)?;

    let result = replay_algorithm(
        &settings,
        std::slice::from_ref(&source),
        &algorithm,
        &output,
    );
    if result.is_ok() {
        return Err("tampered target path must be rejected".into());
    }
    if output.exists() || temp.path.join("escape.txt").exists() {
        return Err("tampered target path must not create output".into());
    }
    Ok(())
}

#[test]
fn tampered_target_path_is_rejected_before_output() {
    let result = run_tampered_target_path_is_rejected();
    assert!(
        result.is_ok(),
        "target-path tamper rejection failed: {result:?}"
    );
}

#[cfg(windows)]
#[path = "../filesystem/support/junction.rs"]
pub mod junction_support;

#[cfg(windows)]
fn run_source_tree_redirect_is_rejected() -> Result<(), Box<dyn std::error::Error>> {
    let temp = TempTree::create("source-redirect")?;
    let source = temp.path.join("source");
    let outside = temp.path.join("outside");
    let target = temp.path.join("target.bin");
    let algorithm = temp.path.join("plan.txt");
    fs::create_dir_all(&source)?;
    fs::create_dir_all(&outside)?;
    fs::write(source.join("evidence.bin"), vec![0x55_u8; 2048])?;
    fs::write(outside.join("private.bin"), b"private")?;
    fs::write(&target, b"synthetic target")?;
    junction_support::create_junction(&source.join("linked"), &outside)?;

    let result = create_algorithm(
        &settings()?,
        std::slice::from_ref(&source),
        &target,
        &algorithm,
    );
    if result.is_ok() {
        return Err("source tree redirect must be rejected".into());
    }
    if algorithm.exists() {
        return Err("redirected source must not create algorithm output".into());
    }
    Ok(())
}

#[cfg(windows)]
#[test]
fn source_tree_redirect_is_rejected() {
    let result = run_source_tree_redirect_is_rejected();
    assert!(
        result.is_ok(),
        "source redirect rejection failed: {result:?}"
    );
}

fn run_existing_replay_output_is_preserved() -> Result<(), Box<dyn std::error::Error>> {
    let temp = TempTree::create("existing-output")?;
    let source = temp.path.join("source.bin");
    let target = temp.path.join("target.bin");
    let algorithm = temp.path.join("plan.txt");
    let output = temp.path.join("output.bin");
    fs::write(&source, vec![0x22_u8; 2048])?;
    fs::write(&target, b"synthetic target")?;
    fs::write(&output, b"preserve me")?;
    let settings = settings()?;
    create_algorithm(
        &settings,
        std::slice::from_ref(&source),
        &target,
        &algorithm,
    )?;

    let result = replay_algorithm(
        &settings,
        std::slice::from_ref(&source),
        &algorithm,
        &output,
    );
    if result.is_ok() {
        return Err("existing replay output must be rejected".into());
    }
    if fs::read(&output)? != b"preserve me" {
        return Err("existing replay output was modified".into());
    }
    Ok(())
}

#[test]
fn existing_replay_output_collision_is_preserved() {
    let result = run_existing_replay_output_is_preserved();
    assert!(
        result.is_ok(),
        "output collision handling failed: {result:?}"
    );
}

fn run_tampered_target_hash_is_rejected() -> Result<(), Box<dyn std::error::Error>> {
    let temp = TempTree::create("target-hash-tamper")?;
    let source = temp.path.join("source.bin");
    let target = temp.path.join("target.bin");
    let algorithm = temp.path.join("plan.txt");
    let output = temp.path.join("output.bin");
    fs::write(&source, vec![0x77_u8; 2048])?;
    fs::write(&target, b"synthetic target")?;
    let settings = settings()?;
    create_algorithm(
        &settings,
        std::slice::from_ref(&source),
        &target,
        &algorithm,
    )?;
    let text = fs::read_to_string(&algorithm)?;
    let mut document: serde_json::Value = serde_json::from_str(&text)?;
    let first_target = document
        .get_mut("target")
        .and_then(serde_json::Value::as_array_mut)
        .and_then(|targets| targets.first_mut())
        .and_then(serde_json::Value::as_object_mut)
        .ok_or("algorithm target record missing")?;
    let Some(_previous_hash) = first_target.insert(
        "sha256".to_owned(),
        serde_json::Value::String("0".repeat(64)),
    ) else {
        return Err("algorithm target hash missing".into());
    };
    fs::write(&algorithm, serde_json::to_vec_pretty(&document)?)?;

    let result = replay_algorithm(
        &settings,
        std::slice::from_ref(&source),
        &algorithm,
        &output,
    );
    if result.is_ok() {
        return Err("tampered target hash must be rejected".into());
    }
    if output.exists() {
        return Err("tampered target hash must not create output".into());
    }
    Ok(())
}

#[test]
fn tampered_target_hash_is_rejected_before_output() {
    let result = run_tampered_target_hash_is_rejected();
    assert!(
        result.is_ok(),
        "target-hash tamper rejection failed: {result:?}"
    );
}

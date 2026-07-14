// File:
//   - filesystem_export.rs
// Path:
//   - src/rsd/tests/filesystem_export.rs
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
//   - Public regression coverage for filesystem RSD export behavior.
// - Must-Not:
//   - Depend on repository-local game assets or fixed machine paths.
// - Allows:
//   - Synthetic RSD files, temporary trees, and public export assertions.
// - Split-When:
//   - Split when platform-specific filesystem semantics need isolated fixtures.
// - Merge-When:
//   - Another RSD test module owns the same filesystem export contract.
// - Summary:
//   - Verifies filesystem exports materialize current deterministic WAV files.
// - Description:
//   - Exercises public batch export behavior with synthetic temporary trees.
// - Usage:
//   - Executed through cargo test for the rsd crate.
// - Defaults:
//   - Temporary fixtures are unique and removed when each test completes.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - true
//   - Reason: Temporary-tree helpers and adapter regressions share one
//   - filesystem export contract without production responsibility.
//

//! Public regression coverage for filesystem RSD export behavior.
//!
//! Synthetic temporary trees keep adapter failures reproducible and isolated.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use rsd::Exporter;
use rsd::adapters::FilesystemExporter;
use schoenwald_cli as _;
use schoenwald_filesystem as _;

static TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(0);

struct TempTree {
    path: PathBuf,
}

impl TempTree {
    fn create(label: &str) -> Result<Self, std::io::Error> {
        let sequence = TEMP_SEQUENCE.fetch_add(
            1_u64,
            Ordering::Relaxed,
        );
        let path = std::env::temp_dir().join(
            format!(
                "schoenwald-rsd-{label}-{}-{sequence}",
                std::process::id()
            ),
        );
        if path.exists() {
            fs::remove_dir_all(&path)?;
        }
        fs::create_dir_all(&path)?;
        Ok(
            Self {
                path,
            },
        )
    }
}

impl Drop for TempTree {
    fn drop(&mut self) {
        let _cleanup_result = fs::remove_dir_all(&self.path);
    }
}

fn copy_fixture_bytes(
    data: &mut [u8],
    start: usize,
    bytes: &[u8],
) -> bool {
    let Some(end) = start.checked_add(bytes.len()) else {
        return false;
    };
    let Some(target) = data.get_mut(start..end) else {
        return false;
    };
    target.copy_from_slice(bytes);
    true
}

fn compact_pcm(payload: &[u8]) -> Vec<u8> {
    let mut data = vec![0_u8; 0x80];
    assert!(
        copy_fixture_bytes(
            &mut data, 0, b"RSD4"
        ),
        "fixture magic should fit"
    );
    assert!(
        copy_fixture_bytes(
            &mut data, 4, b"PCM "
        ),
        "fixture encoding should fit"
    );
    assert!(
        copy_fixture_bytes(
            &mut data,
            8,
            &1_u32.to_le_bytes(),
        ),
        "fixture channel count should fit"
    );
    assert!(
        copy_fixture_bytes(
            &mut data,
            12,
            &16_u32.to_le_bytes(),
        ),
        "fixture bit depth should fit"
    );
    assert!(
        copy_fixture_bytes(
            &mut data,
            16,
            &24_000_u32.to_le_bytes(),
        ),
        "fixture sample rate should fit"
    );
    data.extend_from_slice(payload);
    data
}

fn exported_path(
    output: &Path,
    root: &Path,
    name: &str,
) -> Result<PathBuf, std::io::Error> {
    let Some(root_name) = root.file_name() else {
        return Err(
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "fixture root needs a name",
            ),
        );
    };
    Ok(
        output
            .join(root_name)
            .join(name),
    )
}

fn run_existing_wav_is_refreshed() -> Result<(), Box<dyn std::error::Error>> {
    let temp = TempTree::create("refresh")?;
    let source = temp
        .path
        .join("source");
    let output = temp
        .path
        .join("output");
    fs::create_dir_all(&source)?;
    fs::write(
        source.join("tone.rsd"),
        compact_pcm(
            &[
                1, 0,
            ],
        ),
    )?;
    let destination = exported_path(
        &output, &source, "tone.wav",
    )?;
    let Some(parent) = destination.parent() else {
        return Err("destination needs a parent".into());
    };
    fs::create_dir_all(parent)?;
    fs::write(
        &destination,
        b"stale",
    )?;

    let report = FilesystemExporter.export_roots(
        std::slice::from_ref(&source),
        &output,
    )?;
    let bytes = fs::read(destination)?;

    if report.total_files != 1_usize {
        return Err("refreshed output must be reported".into());
    }
    if !bytes.starts_with(b"RIFF") {
        return Err("existing output was not refreshed".into());
    }
    Ok(())
}

#[test]
fn existing_wav_is_refreshed() {
    let result = run_existing_wav_is_refreshed();

    assert!(
        result.is_ok(),
        "existing outputs must be refreshed: {result:?}"
    );
}

fn run_colliding_root_names_are_rejected()
-> Result<(), Box<dyn std::error::Error>> {
    let temp = TempTree::create("root-collision")?;
    let first = temp
        .path
        .join("first")
        .join("source");
    let second = temp
        .path
        .join("second")
        .join("source");
    let output = temp
        .path
        .join("output");
    fs::create_dir_all(&first)?;
    fs::create_dir_all(&second)?;
    fs::write(
        first.join("tone.rsd"),
        compact_pcm(
            &[
                1, 0,
            ],
        ),
    )?;
    fs::write(
        second.join("tone.rsd"),
        compact_pcm(
            &[
                2, 0,
            ],
        ),
    )?;

    let result = FilesystemExporter.export_roots(
        &[
            first, second,
        ],
        &output,
    );
    if result.is_ok() {
        return Err("colliding root names must fail before export".into());
    }
    if output
        .join("source")
        .join("tone.wav")
        .exists()
    {
        return Err("colliding roots must not leave ambiguous output".into());
    }
    Ok(())
}

#[test]
fn colliding_root_names_are_rejected() {
    let result = run_colliding_root_names_are_rejected();

    assert!(
        result.is_ok(),
        "root output identities must remain unique: {result:?}"
    );
}

fn run_malformed_source_error_names_file()
-> Result<(), Box<dyn std::error::Error>> {
    let temp = TempTree::create("error-path")?;
    let source = temp
        .path
        .join("source");
    let output = temp
        .path
        .join("output");
    fs::create_dir_all(&source)?;
    fs::write(
        source.join("broken.rsd"),
        b"not-rsd",
    )?;

    let result = FilesystemExporter.export_roots(
        std::slice::from_ref(&source),
        &output,
    );
    let Err(error) = result else {
        return Err("malformed source must fail export".into());
    };
    if !error
        .to_string()
        .contains("broken.rsd")
    {
        return Err("malformed source error omitted its path".into());
    }
    Ok(())
}

#[test]
fn malformed_source_error_names_file() {
    let result = run_malformed_source_error_names_file();

    assert!(
        result.is_ok(),
        "source conversion errors must identify the RSD file: {result:?}"
    );
}

fn run_output_inside_source_is_rejected()
-> Result<(), Box<dyn std::error::Error>> {
    let temp = TempTree::create("output-overlap")?;
    let source = temp
        .path
        .join("source");
    let output = source.join("output");
    fs::create_dir_all(&output)?;
    fs::write(
        source.join("tone.rsd"),
        compact_pcm(
            &[
                1, 0,
            ],
        ),
    )?;
    fs::write(
        output.join("foreign.rsd"),
        compact_pcm(
            &[
                2, 0,
            ],
        ),
    )?;

    let result = FilesystemExporter.export_roots(
        std::slice::from_ref(&source),
        &output,
    );
    if result.is_ok() {
        return Err("output nested inside a source root must fail".into());
    }
    if output
        .join("source")
        .exists()
    {
        return Err(
            "overlap rejection must happen before output writes".into(),
        );
    }
    Ok(())
}

#[test]
fn output_inside_source_is_rejected() {
    let result = run_output_inside_source_is_rejected();

    assert!(
        result.is_ok(),
        "output roots must not contaminate source discovery: {result:?}"
    );
}

fn run_non_directory_root_fails_before_writes()
-> Result<(), Box<dyn std::error::Error>> {
    let temp = TempTree::create("root-type")?;
    let source = temp
        .path
        .join("source");
    let invalid = temp
        .path
        .join("not-a-directory");
    let output = temp
        .path
        .join("output");
    fs::create_dir_all(&source)?;
    fs::write(
        source.join("tone.rsd"),
        compact_pcm(
            &[
                1, 0,
            ],
        ),
    )?;
    fs::write(
        &invalid, b"file",
    )?;

    let result = FilesystemExporter.export_roots(
        &[
            source.clone(),
            invalid,
        ],
        &output,
    );
    if result.is_ok() {
        return Err("non-directory roots must fail export".into());
    }
    let destination = exported_path(
        &output, &source, "tone.wav",
    )?;
    if destination.exists() {
        return Err("root preflight happened after an output write".into());
    }
    Ok(())
}

#[test]
fn non_directory_root_fails_before_writes() {
    let result = run_non_directory_root_fails_before_writes();

    assert!(
        result.is_ok(),
        "all source roots must be directories before export: {result:?}"
    );
}

fn run_file_output_root_is_rejected() -> Result<(), Box<dyn std::error::Error>>
{
    let temp = TempTree::create("output-type")?;
    let source = temp
        .path
        .join("source");
    let output = temp
        .path
        .join("output-file");
    fs::create_dir_all(&source)?;
    fs::write(
        &output,
        b"not-a-directory",
    )?;

    let result = FilesystemExporter.export_roots(
        std::slice::from_ref(&source),
        &output,
    );
    if result.is_ok() {
        return Err("file output roots must fail export".into());
    }
    Ok(())
}

#[test]
fn file_output_root_is_rejected() {
    let result = run_file_output_root_is_rejected();

    assert!(
        result.is_ok(),
        "an export destination must be a directory tree: {result:?}"
    );
}

fn run_duplicate_root_is_idempotent() -> Result<(), Box<dyn std::error::Error>>
{
    let temp = TempTree::create("duplicate-root")?;
    let root = temp
        .path
        .join("source");
    let output = temp
        .path
        .join("output");
    fs::create_dir_all(&root)?;
    fs::write(
        root.join("tone.rsd"),
        compact_pcm(
            &[
                1, 0,
            ],
        ),
    )?;

    let report = FilesystemExporter.export_roots(
        &[
            root.clone(),
            root,
        ],
        &output,
    )?;
    if report.total_files != 1_usize
        || report
            .source_roots
            .len()
            != 1_usize
    {
        return Err(
            "duplicate roots must collapse to one export request".into(),
        );
    }
    if !output
        .join("source")
        .join("tone.wav")
        .is_file()
    {
        return Err("deduplicated root did not produce its WAV output".into());
    }
    Ok(())
}

#[test]
fn duplicate_root_is_idempotent() {
    let result = run_duplicate_root_is_idempotent();

    assert!(
        result.is_ok(),
        "repeating one source root must not create a collision: {result:?}"
    );
}

fn run_overlapping_roots_export_each_source_once()
-> Result<(), Box<dyn std::error::Error>> {
    let temp = TempTree::create("source-overlap")?;
    let root = temp
        .path
        .join("source");
    let nested = root.join("nested");
    let output = temp
        .path
        .join("output");
    fs::create_dir_all(&nested)?;
    fs::write(
        nested.join("tone.rsd"),
        compact_pcm(
            &[
                1, 0,
            ],
        ),
    )?;

    let report = FilesystemExporter.export_roots(
        &[
            root, nested,
        ],
        &output,
    )?;
    if report.total_files != 1_usize {
        return Err("overlapping roots duplicated one physical source".into());
    }
    Ok(())
}

#[test]
fn overlapping_roots_export_each_source_once() {
    let result = run_overlapping_roots_export_each_source_once();

    assert!(
        result.is_ok(),
        "physical RSD sources must have one export identity: {result:?}"
    );
}

fn run_empty_roots_rejected() -> Result<(), Box<dyn std::error::Error>> {
    let temp = TempTree::create("empty-request")?;
    let output = temp
        .path
        .join("output");

    let result = FilesystemExporter.export_roots(
        &[],
        &output,
    );
    if result.is_ok() {
        return Err("empty source-root requests must fail".into());
    }
    Ok(())
}

#[test]
fn empty_root_request_is_rejected() {
    let result = run_empty_roots_rejected();

    assert!(
        result.is_ok(),
        "export requests require at least one source root: {result:?}"
    );
}

fn run_no_audio_rejected() -> Result<(), Box<dyn std::error::Error>> {
    let temp = TempTree::create("no-audio")?;
    let source = temp
        .path
        .join("source");
    let output = temp
        .path
        .join("output");
    fs::create_dir_all(&source)?;
    fs::write(
        source.join("readme.txt"),
        b"not audio",
    )?;

    let result = FilesystemExporter.export_roots(
        std::slice::from_ref(&source),
        &output,
    );
    if result.is_ok() {
        return Err("roots without RSD inputs must fail".into());
    }
    Ok(())
}

#[test]
fn root_without_audio_is_rejected() {
    let result = run_no_audio_rejected();

    assert!(
        result.is_ok(),
        "an export request must discover at least one RSD input: {result:?}"
    );
}

#[cfg(windows)]
fn run_case_folded_root_collision_is_rejected()
-> Result<(), Box<dyn std::error::Error>> {
    let temp = TempTree::create("case-root-collision")?;
    let first = temp
        .path
        .join("first")
        .join("Source");
    let second = temp
        .path
        .join("second")
        .join("source");
    let output = temp
        .path
        .join("output");
    fs::create_dir_all(&first)?;
    fs::create_dir_all(&second)?;
    fs::write(
        first.join("tone.rsd"),
        compact_pcm(
            &[
                1, 0,
            ],
        ),
    )?;
    fs::write(
        second.join("tone.rsd"),
        compact_pcm(
            &[
                2, 0,
            ],
        ),
    )?;

    let result = FilesystemExporter.export_roots(
        &[
            first, second,
        ],
        &output,
    );
    if result.is_ok() {
        return Err(
            "case-equivalent root names must collide on Windows".into(),
        );
    }
    Ok(())
}

#[cfg(windows)]
#[test]
fn case_folded_root_collision_is_rejected() {
    let result = run_case_folded_root_collision_is_rejected();

    assert!(
        result.is_ok(),
        "root identities must match Windows output semantics: {result:?}"
    );
}

fn run_conversion_failure_leaves_no_outputs()
-> Result<(), Box<dyn std::error::Error>> {
    let temp = TempTree::create("conversion-transaction")?;
    let first = temp
        .path
        .join("first");
    let second = temp
        .path
        .join("second");
    let output = temp
        .path
        .join("output");
    fs::create_dir_all(&first)?;
    fs::create_dir_all(&second)?;
    fs::write(
        first.join("good.rsd"),
        compact_pcm(
            &[
                1, 0,
            ],
        ),
    )?;
    fs::write(
        second.join("broken.rsd"),
        b"broken",
    )?;

    let result = FilesystemExporter.export_roots(
        &[
            first.clone(),
            second,
        ],
        &output,
    );
    if result.is_ok() {
        return Err("malformed later sources must fail the batch".into());
    }
    let destination = exported_path(
        &output, &first, "good.wav",
    )?;
    if destination.exists() {
        return Err("failed conversion batch left a partial WAV".into());
    }
    Ok(())
}

#[test]
fn conversion_failure_leaves_no_outputs() {
    let result = run_conversion_failure_leaves_no_outputs();

    assert!(
        result.is_ok(),
        "conversion must complete before output mutation: {result:?}"
    );
}

fn run_destination_conflict_leaves_no_outputs()
-> Result<(), Box<dyn std::error::Error>> {
    let temp = TempTree::create("destination-transaction")?;
    let first = temp
        .path
        .join("first");
    let second = temp
        .path
        .join("second");
    let output = temp
        .path
        .join("output");
    fs::create_dir_all(&first)?;
    fs::create_dir_all(&second)?;
    fs::create_dir_all(&output)?;
    fs::write(
        first.join("good.rsd"),
        compact_pcm(
            &[
                1, 0,
            ],
        ),
    )?;
    fs::write(
        second.join("good.rsd"),
        compact_pcm(
            &[
                2, 0,
            ],
        ),
    )?;
    fs::write(
        output.join("second"),
        b"blocks-directory",
    )?;

    let result = FilesystemExporter.export_roots(
        &[
            first.clone(),
            second,
        ],
        &output,
    );
    if result.is_ok() {
        return Err("conflicting destination paths must fail the batch".into());
    }
    let destination = exported_path(
        &output, &first, "good.wav",
    )?;
    if destination.exists() {
        return Err("destination conflict left a partial WAV".into());
    }
    Ok(())
}

#[test]
fn destination_conflict_leaves_no_outputs() {
    let result = run_destination_conflict_leaves_no_outputs();

    assert!(
        result.is_ok(),
        "destination shape checks must precede output mutation: {result:?}"
    );
}

fn run_hard_link_destination_is_replaced_safely()
-> Result<(), Box<dyn std::error::Error>> {
    let temp = TempTree::create("hard-link-output")?;
    let source = temp
        .path
        .join("source");
    let output = temp
        .path
        .join("output");
    let outside = temp
        .path
        .join("outside.wav");
    fs::create_dir_all(&source)?;
    fs::write(
        source.join("tone.rsd"),
        compact_pcm(
            &[
                1, 0,
            ],
        ),
    )?;
    fs::write(
        &outside, b"outside",
    )?;
    let destination = exported_path(
        &output, &source, "tone.wav",
    )?;
    let Some(parent) = destination.parent() else {
        return Err("destination needs a parent".into());
    };
    fs::create_dir_all(parent)?;
    fs::hard_link(
        &outside,
        &destination,
    )?;

    let _report = FilesystemExporter.export_roots(
        std::slice::from_ref(&source),
        &output,
    )?;
    if fs::read(&outside)? != b"outside" {
        return Err(
            "export modified a file outside its output identity".into(),
        );
    }
    if !fs::read(destination)?.starts_with(b"RIFF") {
        return Err(
            "destination was not replaced with current WAV bytes".into(),
        );
    }
    Ok(())
}

#[test]
fn hard_link_destination_is_replaced_safely() {
    let result = run_hard_link_destination_is_replaced_safely();

    assert!(
        result.is_ok(),
        "existing outputs must not alias external files: {result:?}"
    );
}

#[cfg(windows)]
fn run_readonly_destination_fails_before_writes()
-> Result<(), Box<dyn std::error::Error>> {
    let temp = TempTree::create("readonly-output")?;
    let first = temp
        .path
        .join("first");
    let second = temp
        .path
        .join("second");
    let output = temp
        .path
        .join("output");
    fs::create_dir_all(&first)?;
    fs::create_dir_all(&second)?;
    fs::write(
        first.join("tone.rsd"),
        compact_pcm(
            &[
                1, 0,
            ],
        ),
    )?;
    fs::write(
        second.join("tone.rsd"),
        compact_pcm(
            &[
                2, 0,
            ],
        ),
    )?;
    let blocked = exported_path(
        &output, &second, "tone.wav",
    )?;
    let Some(parent) = blocked.parent() else {
        return Err("destination needs a parent".into());
    };
    fs::create_dir_all(parent)?;
    fs::write(
        &blocked,
        b"readonly",
    )?;
    let mut permissions = fs::metadata(&blocked)?.permissions();
    permissions.set_readonly(true);
    fs::set_permissions(
        &blocked,
        permissions,
    )?;

    let result = FilesystemExporter.export_roots(
        &[
            first.clone(),
            second,
        ],
        &output,
    );
    let cleanup_status = std::process::Command::new("attrib")
        .arg("-R")
        .arg(&blocked)
        .status()?;
    if !cleanup_status.success() {
        return Err("failed to clear read-only test fixture".into());
    }
    if result.is_ok() {
        return Err("read-only destinations must fail export".into());
    }
    let first_destination = exported_path(
        &output, &first, "tone.wav",
    )?;
    if first_destination.exists() {
        return Err("read-only destination left a partial batch".into());
    }
    Ok(())
}

#[cfg(windows)]
#[test]
fn readonly_destination_fails_before_writes() {
    let result = run_readonly_destination_fails_before_writes();

    assert!(
        result.is_ok(),
        "destination permissions must be preflighted: {result:?}"
    );
}

fn run_nested_order_check() -> Result<(), Box<dyn std::error::Error>> {
    let temp = TempTree::create("nested-order")?;
    let source = temp
        .path
        .join("source");
    let output = temp
        .path
        .join("output");
    let first = source.join("a");
    let last = source.join("z");
    fs::create_dir_all(&first)?;
    fs::create_dir_all(&last)?;
    fs::write(
        first.join("broken.rsd"),
        b"broken-a",
    )?;
    fs::write(
        last.join("broken.rsd"),
        b"broken-z",
    )?;

    let result = FilesystemExporter.export_roots(
        std::slice::from_ref(&source),
        &output,
    );
    let Err(error) = result else {
        return Err("malformed nested sources must fail export".into());
    };
    let expected: String = first
        .join("broken.rsd")
        .to_string_lossy()
        .chars()
        .flat_map(char::escape_default)
        .collect();
    if !error
        .to_string()
        .contains(&expected)
    {
        return Err("nested discovery did not use sorted source paths".into());
    }
    Ok(())
}

#[test]
fn nested_failure_order_is_sorted() {
    let result = run_nested_order_check();

    assert!(
        result.is_ok(),
        "nested source discovery must be deterministic: {result:?}"
    );
}

#[cfg(windows)]
fn run_unicode_root_collision_is_rejected()
-> Result<(), Box<dyn std::error::Error>> {
    let temp = TempTree::create("unicode-root-collision")?;
    let first = temp
        .path
        .join("first")
        .join(
            {
                // cspell:disable-next-line -- Äudio
                "Äudio"
            },
        );
    let second = temp
        .path
        .join("second")
        .join(
            {
                // cspell:disable-next-line -- äudio
                "äudio"
            },
        );
    let output = temp
        .path
        .join("output");
    fs::create_dir_all(&first)?;
    fs::create_dir_all(&second)?;
    fs::write(
        first.join("tone.rsd"),
        compact_pcm(
            &[
                1, 0,
            ],
        ),
    )?;
    fs::write(
        second.join("tone.rsd"),
        compact_pcm(
            &[
                2, 0,
            ],
        ),
    )?;

    let result = FilesystemExporter.export_roots(
        &[
            first, second,
        ],
        &output,
    );
    if result.is_ok() {
        return Err("Unicode-equivalent root names must collide".into());
    }
    Ok(())
}

#[cfg(windows)]
#[test]
fn unicode_root_collision_is_rejected() {
    let result = run_unicode_root_collision_is_rejected();

    assert!(
        result.is_ok(),
        "root identity folding must include Unicode case: {result:?}"
    );
}

#[cfg(windows)]
fn run_junction_output_escape_is_rejected()
-> Result<(), Box<dyn std::error::Error>> {
    let temp = TempTree::create("junction-output")?;
    let source = temp
        .path
        .join("source");
    let output = temp
        .path
        .join("output");
    let outside = temp
        .path
        .join("outside");
    fs::create_dir_all(&source)?;
    fs::create_dir_all(&output)?;
    fs::create_dir_all(&outside)?;
    fs::write(
        source.join("tone.rsd"),
        compact_pcm(
            &[
                1, 0,
            ],
        ),
    )?;
    let junction = output.join("source");
    let link_status = std::process::Command::new("cmd")
        .arg("/C")
        .arg("mklink")
        .arg("/J")
        .arg(&junction)
        .arg(&outside)
        .status()?;
    if !link_status.success() {
        return Err("failed to create junction fixture".into());
    }

    let result = FilesystemExporter.export_roots(
        std::slice::from_ref(&source),
        &output,
    );
    fs::remove_dir(&junction)?;
    if result.is_ok() {
        return Err("junction output escapes must fail export".into());
    }
    if outside
        .join("tone.wav")
        .exists()
    {
        return Err("export wrote outside the selected output root".into());
    }
    Ok(())
}

#[cfg(windows)]
#[test]
fn junction_output_escape_is_rejected() {
    let result = run_junction_output_escape_is_rejected();

    assert!(
        result.is_ok(),
        "resolved destination parents must stay inside output: {result:?}"
    );
}

fn run_long_destination_name_exports() -> Result<(), Box<dyn std::error::Error>>
{
    let temp = TempTree::create("long-output-name")?;
    let source = temp
        .path
        .join("source");
    let output = temp
        .path
        .join("output");
    fs::create_dir_all(&source)?;
    let stem = "a".repeat(240);
    let source_name = format!("{stem}.rsd");
    fs::write(
        source.join(&source_name),
        compact_pcm(
            &[
                1, 0,
            ],
        ),
    )?;

    let _report = FilesystemExporter.export_roots(
        std::slice::from_ref(&source),
        &output,
    )?;
    let destination = exported_path(
        &output,
        &source,
        &format!("{stem}.wav"),
    )?;
    if !destination.exists() {
        return Err("long valid destination was not exported".into());
    }
    Ok(())
}

#[test]
fn long_destination_name_exports() {
    let result = run_long_destination_name_exports();

    assert!(
        result.is_ok(),
        "staging names must not exceed destination name limits: {result:?}"
    );
}

#[cfg(windows)]
fn run_locked_destination_rolls_back_batch()
-> Result<(), Box<dyn std::error::Error>> {
    use std::os::windows::fs::OpenOptionsExt as _;

    let temp = TempTree::create("locked-output")?;
    let first = temp
        .path
        .join("first");
    let second = temp
        .path
        .join("second");
    let output = temp
        .path
        .join("output");
    fs::create_dir_all(&first)?;
    fs::create_dir_all(&second)?;
    fs::write(
        first.join("tone.rsd"),
        compact_pcm(
            &[
                1, 0,
            ],
        ),
    )?;
    fs::write(
        second.join("tone.rsd"),
        compact_pcm(
            &[
                2, 0,
            ],
        ),
    )?;
    let blocked = exported_path(
        &output, &second, "tone.wav",
    )?;
    let Some(parent) = blocked.parent() else {
        return Err("destination needs a parent".into());
    };
    fs::create_dir_all(parent)?;
    fs::write(
        &blocked, b"locked",
    )?;
    let lock = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .share_mode(0_u32)
        .open(&blocked)?;

    let result = FilesystemExporter.export_roots(
        &[
            first.clone(),
            second,
        ],
        &output,
    );
    drop(lock);
    if result.is_ok() {
        return Err("locked destinations must fail export".into());
    }
    let first_destination = exported_path(
        &output, &first, "tone.wav",
    )?;
    if first_destination.exists() {
        return Err("commit failure left an earlier WAV".into());
    }
    if fs::read(&blocked)? != b"locked" {
        return Err("commit failure changed the locked destination".into());
    }
    Ok(())
}

#[cfg(windows)]
#[test]
fn locked_destination_rolls_back_batch() {
    let result = run_locked_destination_rolls_back_batch();

    assert!(
        result.is_ok(),
        "write-phase failures must roll back the batch: {result:?}"
    );
}

fn run_parent_alias_root_is_idempotent()
-> Result<(), Box<dyn std::error::Error>> {
    let temp = TempTree::create("parent-alias-root")?;
    let parent = temp
        .path
        .join("parent");
    let source = parent.join("source");
    let alias_anchor = parent.join("alias");
    let output = temp
        .path
        .join("output");
    fs::create_dir_all(&source)?;
    fs::create_dir_all(&alias_anchor)?;
    fs::write(
        source.join("tone.rsd"),
        compact_pcm(
            &[
                1, 0,
            ],
        ),
    )?;
    let aliased_source = alias_anchor
        .join("..")
        .join("source");

    let report = FilesystemExporter.export_roots(
        &[
            source,
            aliased_source,
        ],
        &output,
    )?;
    if report.total_files != 1_usize
        || report
            .source_roots
            .len()
            != 1_usize
    {
        return Err(
            "one canonical root must collapse to one export request".into(),
        );
    }
    Ok(())
}

#[test]
fn parent_alias_root_is_idempotent() {
    let result = run_parent_alias_root_is_idempotent();

    assert!(
        result.is_ok(),
        "canonical root aliases must remain idempotent: {result:?}"
    );
}

fn run_nested_destination_collision_leaves_no_output()
-> Result<(), Box<dyn std::error::Error>> {
    let temp = TempTree::create("nested-destination")?;
    let source = temp
        .path
        .join("source");
    let nested = source.join("tone.wav");
    let output = temp
        .path
        .join("output");
    fs::create_dir_all(&nested)?;
    fs::write(
        source.join("tone.rsd"),
        compact_pcm(
            &[
                1, 0,
            ],
        ),
    )?;
    fs::write(
        nested.join("nested.rsd"),
        compact_pcm(
            &[
                2, 0,
            ],
        ),
    )?;

    let result = FilesystemExporter.export_roots(
        std::slice::from_ref(&source),
        &output,
    );
    if result.is_ok() {
        return Err("nested WAV destinations must fail the batch".into());
    }
    if output.exists() {
        return Err(
            "nested destination preflight must not create output state".into(),
        );
    }
    Ok(())
}

#[test]
fn nested_destination_collision_leaves_no_output() {
    let result = run_nested_destination_collision_leaves_no_output();

    assert!(
        result.is_ok(),
        "nested output destinations must fail before publication: {result:?}"
    );
}

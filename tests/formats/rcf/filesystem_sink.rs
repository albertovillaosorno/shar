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
//   - Filesystem sink test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Filesystem sink test module.
// - Description:
//   - Implements the declared test module responsibility for rcf.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Filesystem sink test module.

use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use rcf::adapters::FileEntrySink;
use rcf::domain::ArchiveError;
use rcf::ports::EntrySink;
use schoenwald_cli as _;
use schoenwald_filesystem as _;

static NEXT_PATH_ID: AtomicU64 = AtomicU64::new(0);

#[test]
fn rejects_archive_stem_parent_traversal() {
    let path_id = NEXT_PATH_ID.fetch_add(1, Ordering::Relaxed);
    let root = temporary_path("root", path_id);
    let escaped = temporary_path("escaped", path_id);
    let created = fs::create_dir_all(&root);
    assert!(
        created.is_ok(),
        "the isolated output root must be constructible"
    );
    if created.is_err() {
        return;
    }
    let Some(escaped_name) =
        escaped.file_name().and_then(|value| value.to_str())
    else {
        cleanup(&root);
        return;
    };
    let archive_stem = format!("../{escaped_name}");
    let mut sink = FileEntrySink::new(&root);
    let result = sink.write_entry(&archive_stem, "sound/file.rsd", b"payload");
    cleanup(&escaped);
    cleanup(&root);

    assert!(
        matches!(result, Err(ArchiveError::UnsafeEntryPath(_))),
        "archive stems must not escape the configured output root"
    );
}

#[test]
fn rejects_control_characters_in_archive_stems() {
    let path_id = NEXT_PATH_ID.fetch_add(1, Ordering::Relaxed);
    let root = temporary_path("control-stem", path_id);
    let mut sink = FileEntrySink::new(&root);
    let result = sink.write_entry("bad\nstem", "sound/file.rsd", b"payload");
    cleanup(&root);

    assert!(
        matches!(result, Err(ArchiveError::UnsafeEntryPath(_))),
        "archive stems must not inject control characters into paths or logs"
    );
}

#[test]
fn rejects_control_characters_in_entry_paths() {
    let path_id = NEXT_PATH_ID.fetch_add(1, Ordering::Relaxed);
    let root = temporary_path("control-entry", path_id);
    let mut sink = FileEntrySink::new(&root);
    let result = sink.write_entry("archive", "sound/bad\nfile.rsd", b"payload");
    cleanup(&root);

    assert!(
        matches!(result, Err(ArchiveError::UnsafeEntryPath(_))),
        "entry paths must not inject control characters into paths or logs"
    );
}

#[test]
fn rejects_windows_invalid_characters_in_entry_paths() {
    let path_id = NEXT_PATH_ID.fetch_add(1, Ordering::Relaxed);
    let root = temporary_path("invalid-entry", path_id);
    let mut sink = FileEntrySink::new(&root);
    let result = sink.write_entry("archive", "sound/bad?.rsd", b"payload");
    cleanup(&root);

    assert!(
        matches!(result, Err(ArchiveError::UnsafeEntryPath(_))),
        "entry paths must be portable to the supported Windows target"
    );
}

#[test]
fn rejects_windows_invalid_characters_in_archive_stems() {
    let path_id = NEXT_PATH_ID.fetch_add(1, Ordering::Relaxed);
    let root = temporary_path("invalid-stem", path_id);
    let mut sink = FileEntrySink::new(&root);
    let result = sink.write_entry("bad?stem", "sound/file.rsd", b"payload");
    cleanup(&root);

    assert!(
        matches!(result, Err(ArchiveError::UnsafeEntryPath(_))),
        "archive stems must be portable to the supported Windows target"
    );
}

#[test]
fn rejects_trimmed_suffixes_in_entry_components() {
    let path_id = NEXT_PATH_ID.fetch_add(1, Ordering::Relaxed);
    let root = temporary_path("trimmed-entry", path_id);
    let mut sink = FileEntrySink::new(&root);
    let result = sink.write_entry("archive", "sound/file.rsd.", b"payload");
    cleanup(&root);

    assert!(
        matches!(result, Err(ArchiveError::UnsafeEntryPath(_))),
        "entry components must not collapse after Windows suffix trimming"
    );
}

#[test]
fn rejects_trimmed_suffixes_in_archive_stems() {
    let path_id = NEXT_PATH_ID.fetch_add(1, Ordering::Relaxed);
    let root = temporary_path("trimmed-stem", path_id);
    let mut sink = FileEntrySink::new(&root);
    let result = sink.write_entry("archive.", "sound/file.rsd", b"payload");
    cleanup(&root);

    assert!(
        matches!(result, Err(ArchiveError::UnsafeEntryPath(_))),
        "archive stems must not collapse after Windows suffix trimming"
    );
}

#[test]
fn rejects_windows_device_names_in_entry_components() {
    let path_id = NEXT_PATH_ID.fetch_add(1, Ordering::Relaxed);
    let root = temporary_path("device-entry", path_id);
    let mut sink = FileEntrySink::new(&root);
    let result = sink.write_entry("archive", "sound/con.rsd", b"payload");
    cleanup(&root);

    assert!(
        matches!(result, Err(ArchiveError::UnsafeEntryPath(_))),
        "entry components must not resolve to Windows device namespaces"
    );
}

#[test]
fn rejects_windows_device_names_in_archive_stems() {
    let path_id = NEXT_PATH_ID.fetch_add(1, Ordering::Relaxed);
    let root = temporary_path("device-stem", path_id);
    let mut sink = FileEntrySink::new(&root);
    let result = sink.write_entry("con", "sound/file.rsd", b"payload");
    cleanup(&root);

    assert!(
        matches!(result, Err(ArchiveError::UnsafeEntryPath(_))),
        "archive stems must not resolve to Windows device namespaces"
    );
}

#[test]
fn rejects_oversized_entry_components() {
    let path_id = NEXT_PATH_ID.fetch_add(1, Ordering::Relaxed);
    let root = temporary_path("oversized-entry", path_id);
    let long_component = "a".repeat(256);
    let entry_name = format!("sound/{long_component}");
    let mut sink = FileEntrySink::new(&root);
    let result = sink.write_entry("archive", &entry_name, b"payload");
    cleanup(&root);

    assert!(
        matches!(result, Err(ArchiveError::UnsafeEntryPath(_))),
        "entry components must fit the supported Windows component limit"
    );
}

#[test]
fn rejects_oversized_archive_stems() {
    let path_id = NEXT_PATH_ID.fetch_add(1, Ordering::Relaxed);
    let root = temporary_path("oversized-stem", path_id);
    let archive_stem = "a".repeat(256);
    let mut sink = FileEntrySink::new(&root);
    let result = sink.write_entry(&archive_stem, "sound/file.rsd", b"payload");
    cleanup(&root);

    assert!(
        matches!(result, Err(ArchiveError::UnsafeEntryPath(_))),
        "archive stems must fit the supported Windows component limit"
    );
}

#[test]
fn rejects_unicode_path_controls_in_entry_components() {
    let path_id = NEXT_PATH_ID.fetch_add(1, Ordering::Relaxed);
    let root = temporary_path("unicode-entry", path_id);
    let mut sink = FileEntrySink::new(&root);
    let result =
        sink.write_entry("archive", "sound/\u{202e}file.rsd", b"payload");
    cleanup(&root);

    assert!(
        matches!(result, Err(ArchiveError::UnsafeEntryPath(_))),
        "entry components must not contain invisible Unicode path controls"
    );
}

#[test]
fn rejects_unicode_path_controls_in_archive_stems() {
    let path_id = NEXT_PATH_ID.fetch_add(1, Ordering::Relaxed);
    let root = temporary_path("unicode-stem", path_id);
    let mut sink = FileEntrySink::new(&root);
    let result =
        sink.write_entry("archive\u{202e}", "sound/file.rsd", b"payload");
    cleanup(&root);

    assert!(
        matches!(result, Err(ArchiveError::UnsafeEntryPath(_))),
        "archive stems must not contain invisible Unicode path controls"
    );
}

#[test]
fn rejects_superscript_device_names_in_entry_components() {
    let path_id = NEXT_PATH_ID.fetch_add(1, Ordering::Relaxed);
    let root = temporary_path("superscript-device-entry", path_id);
    let mut sink = FileEntrySink::new(&root);
    let result =
        sink.write_entry("archive", "sound/COM\u{00b9}.rsd", b"payload");
    cleanup(&root);

    assert!(
        matches!(result, Err(ArchiveError::UnsafeEntryPath(_))),
        "superscript DOS devices must be rejected in entry paths"
    );
}

#[test]
fn rejects_superscript_device_names_in_archive_stems() {
    let path_id = NEXT_PATH_ID.fetch_add(1, Ordering::Relaxed);
    let root = temporary_path("superscript-device-stem", path_id);
    let mut sink = FileEntrySink::new(&root);
    let result = sink.write_entry("LPT\u{00b2}", "sound/file.rsd", b"payload");
    cleanup(&root);

    assert!(
        matches!(result, Err(ArchiveError::UnsafeEntryPath(_))),
        "superscript DOS devices must be rejected in archive stems"
    );
}

fn temporary_path(label: &str, path_id: u64) -> PathBuf {
    std::env::temp_dir()
        .join(format!("rcf-{label}-{}-{path_id}", std::process::id()))
}

fn cleanup(path: &PathBuf) {
    let _ignored = fs::remove_dir_all(path);
}

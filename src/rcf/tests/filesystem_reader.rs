// File:
//   - filesystem_reader.rs
// Path:
//   - src/rcf/tests/filesystem_reader.rs
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
//   - Caller-visible file-reader range regressions.
// - Must-Not:
//   - Read private assets or repository-local game inputs.
// - Allows:
//   - Isolated temporary files and public archive-reader operations.
// - Split-When:
//   - Another reader adapter needs an independent integration target.
// - Merge-When:
//   - File-reader range safety no longer needs a distinct test boundary.
// - Summary:
//   - Protects bounded RCF file reads.
// - Description:
//   - Exercises public file-backed range reads against declared file length.
// - Usage:
//   - Run through the RCF crate integration test target.
// - Defaults:
//   - Temporary files are process-local and removed after each regression.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Caller-visible regressions for the RCF file-backed byte reader.
//!
//! Tests use isolated temporary files and remove them after each assertion.

use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use rcf::adapters::FileArchiveSource;
use rcf::domain::ArchiveError;
use rcf::ports::ArchiveSource;
use schoenwald_cli as _;
use schoenwald_filesystem as _;

static NEXT_FILE_ID: AtomicU64 = AtomicU64::new(0);

#[test]
fn rejects_ranges_beyond_the_file_before_reading() {
    let file_id = NEXT_FILE_ID.fetch_add(
        1,
        Ordering::Relaxed,
    );
    let path = temporary_file_path(file_id);
    let written = fs::write(
        &path, b"ab",
    );
    assert!(
        written.is_ok(),
        "the isolated fixture file must be writable"
    );
    if written.is_err() {
        return;
    }
    let source = FileArchiveSource::new(&path);
    let opened_reader = source.open_reader();
    assert!(
        opened_reader.is_ok(),
        "the fixture reader must open"
    );
    let Ok(mut reader) = opened_reader else {
        cleanup(&path);
        return;
    };
    let result = reader.read_range(
        1, 2,
    );
    cleanup(&path);

    assert!(
        matches!(
            result,
            Err(ArchiveError::InvalidArchive(message))
                if message.contains("range exceeds archive length")
        ),
        "invalid ranges must be rejected before allocation and IO"
    );
}

#[test]
fn reader_length_remains_stable_after_source_growth() {
    let file_id = NEXT_FILE_ID.fetch_add(
        1,
        Ordering::Relaxed,
    );
    let path = temporary_file_path(file_id);
    let written = fs::write(
        &path, b"ab",
    );
    assert!(
        written.is_ok(),
        "the isolated fixture file must be writable"
    );
    if written.is_err() {
        return;
    }
    let source = FileArchiveSource::new(&path);
    let opened_reader = source.open_reader();
    assert!(
        opened_reader.is_ok(),
        "the fixture reader must open"
    );
    let Ok(mut reader) = opened_reader else {
        cleanup(&path);
        return;
    };
    let append_result = fs::OpenOptions::new()
        .append(true)
        .open(&path);
    assert!(
        append_result.is_ok(),
        "the fixture file must reopen for appending"
    );
    let Ok(mut append_file) = append_result else {
        cleanup(&path);
        return;
    };
    let appended = append_file.write_all(b"cd");
    assert!(
        appended.is_ok(),
        "the fixture file must grow after the reader opens"
    );
    if appended.is_err() {
        cleanup(&path);
        return;
    }

    let length = reader.len();
    let result = reader.read_range(
        0, 4,
    );
    cleanup(&path);

    assert!(
        matches!(
            length,
            Ok(2)
        ),
        "one opened reader must preserve its original archive length"
    );
    assert!(
        matches!(
            result,
            Err(ArchiveError::InvalidArchive(message))
                if message.contains("range exceeds archive length")
        ),
        "bytes appended after open must remain outside the archive snapshot"
    );
}

#[test]
fn truncation_after_open_is_reported_as_invalid_archive() {
    let file_id = NEXT_FILE_ID.fetch_add(
        1,
        Ordering::Relaxed,
    );
    let path = temporary_file_path(file_id);
    let written = fs::write(
        &path, b"abcd",
    );
    assert!(
        written.is_ok(),
        "the isolated fixture file must be writable"
    );
    if written.is_err() {
        return;
    }
    let source = FileArchiveSource::new(&path);
    let opened_reader = source.open_reader();
    assert!(
        opened_reader.is_ok(),
        "the fixture reader must open"
    );
    let Ok(mut reader) = opened_reader else {
        cleanup(&path);
        return;
    };
    let truncated = fs::write(
        &path, b"ab",
    );
    assert!(
        truncated.is_ok(),
        "the fixture file must truncate after the reader opens"
    );
    if truncated.is_err() {
        cleanup(&path);
        return;
    }

    let result = reader.read_range(
        0, 4,
    );
    cleanup(&path);

    assert!(
        matches!(
            result,
            Err(ArchiveError::InvalidArchive(message))
                if message.contains("archive changed after reader opened")
        ),
        "snapshot truncation must be classified as malformed archive data"
    );
}

fn temporary_file_path(file_id: u64) -> PathBuf {
    std::env::temp_dir().join(
        format!(
            "rcf-reader-{}-{file_id}.bin",
            std::process::id()
        ),
    )
}

fn cleanup(path: &PathBuf) {
    let _ignored = fs::remove_file(path);
}

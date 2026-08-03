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
//   - Filesystem entry sink test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Filesystem entry sink test module.
// - Description:
//   - Implements the declared test module responsibility for lmlm.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Filesystem entry sink test module.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::{fs, io};

use lmlm::{FileEntry, materialize_entries};
use schoenwald_cli as _;
use schoenwald_filesystem as _;

static NEXT_TEST_ROOT: AtomicU64 = AtomicU64::new(0);

fn remove_test_root(root: &Path) -> Result<(), String> {
    match fs::remove_dir_all(root) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error.to_string()),
    }
}

fn test_root(label: &str) -> PathBuf {
    let sequence = NEXT_TEST_ROOT.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir()
        .join(format!("lmlm-{label}-{}-{sequence}", std::process::id()))
}

#[test]
fn rejects_empty_output_roots_before_writing() {
    let result = materialize_entries(b"", &[], Path::new(""));

    assert!(
        matches!(
            result,
            Err(error) if error.kind() == io::ErrorKind::InvalidInput
        ),
        "empty output root must fail before writing"
    );
}

#[test]
fn rejects_case_colliding_destinations_before_writing() -> Result<(), String> {
    let root = test_root("case-collision");
    remove_test_root(&root)?;
    let entries = [
        FileEntry {
            path: "Shared.bin".to_owned(),
            offset: 0,
            size: 1,
        },
        FileEntry {
            path: "shared.bin".to_owned(),
            offset: 1,
            size: 1,
        },
    ];
    let result = materialize_entries(b"ab", &entries, &root);
    let upper_exists = root.join("Shared.bin").exists();
    let lower_exists = root.join("shared.bin").exists();
    remove_test_root(&root)?;
    match result {
        Err(error)
            if error.kind() == io::ErrorKind::AlreadyExists
                && !upper_exists
                && !lower_exists =>
        {
            Ok(())
        },
        other => Err(format!(
            "case-colliding destinations must fail before writes, got \
                 {other:?}, upper_exists={upper_exists}, \
                 lower_exists={lower_exists}"
        )),
    }
}

#[test]
fn rejects_duplicate_destinations_before_writing() -> Result<(), String> {
    let root = test_root("duplicate-destination");
    remove_test_root(&root)?;
    let entries = [
        FileEntry {
            path: "same.bin".to_owned(),
            offset: 0,
            size: 1,
        },
        FileEntry {
            path: "same.bin".to_owned(),
            offset: 1,
            size: 1,
        },
    ];
    let result = materialize_entries(b"ab", &entries, &root);
    let destination_exists = root.join("same.bin").exists();
    remove_test_root(&root)?;
    match result {
        Err(error)
            if error.kind() == io::ErrorKind::AlreadyExists
                && !destination_exists =>
        {
            Ok(())
        },
        other => Err(format!(
            "duplicate destinations must fail before writes, got \
                 {other:?} and destination_exists={destination_exists}"
        )),
    }
}

#[test]
fn rejects_paths_that_escape_the_output_root() -> Result<(), String> {
    let root = test_root("path-escape");
    remove_test_root(&root)?;
    let parent = root
        .parent()
        .ok_or_else(|| "test root must have a parent".to_owned())?;
    let escape = parent.join(format!("lmlm-escape-{}", std::process::id()));
    match fs::remove_file(&escape) {
        Ok(()) => {},
        Err(error) if error.kind() == io::ErrorKind::NotFound => {},
        Err(error) => return Err(error.to_string()),
    }
    let escape_name = escape
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| "escape filename must be UTF-8".to_owned())?;
    let entries = [FileEntry {
        path: format!("../{escape_name}"),
        offset: 0,
        size: 1,
    }];
    let result = materialize_entries(b"x", &entries, &root);
    let escaped = escape.exists();
    if escaped {
        fs::remove_file(&escape).map_err(|error| error.to_string())?;
    }
    remove_test_root(&root)?;
    match result {
        Err(error)
            if error.kind() == io::ErrorKind::InvalidInput && !escaped =>
        {
            Ok(())
        },
        other => Err(format!(
            "escaping path must fail without an external write, got \
                 {other:?} and escaped={escaped}"
        )),
    }
}

#[test]
fn preflights_parent_conflicts_before_writing() -> Result<(), String> {
    let root = test_root("parent-conflict");
    remove_test_root(&root)?;
    fs::create_dir_all(&root).map_err(|error| error.to_string())?;
    fs::write(root.join("blocked"), b"not-a-directory")
        .map_err(|error| error.to_string())?;
    let entries = [
        FileEntry {
            path: "first.bin".to_owned(),
            offset: 0,
            size: 1,
        },
        FileEntry {
            path: "blocked/second.bin".to_owned(),
            offset: 1,
            size: 1,
        },
    ];
    let result = materialize_entries(b"ab", &entries, &root);
    let first_exists = root.join("first.bin").exists();
    remove_test_root(&root)?;
    match result {
        Err(_) if !first_exists => Ok(()),
        other => Err(format!(
            "known parent conflict must fail before writes, got {other:?} \
                 and first_exists={first_exists}"
        )),
    }
}

#[test]
fn refuses_to_overwrite_existing_files() -> Result<(), String> {
    let root = test_root("existing-file");
    remove_test_root(&root)?;
    fs::create_dir_all(&root).map_err(|error| error.to_string())?;
    let destination = root.join("existing.bin");
    fs::write(&destination, b"original").map_err(|error| error.to_string())?;
    let entries = [FileEntry {
        path: "existing.bin".to_owned(),
        offset: 0,
        size: 3,
    }];
    let result = materialize_entries(b"new", &entries, &root);
    let content = fs::read(&destination).map_err(|error| error.to_string())?;
    remove_test_root(&root)?;
    match result {
        Err(error)
            if error.kind() == io::ErrorKind::AlreadyExists
                && content == b"original" =>
        {
            Ok(())
        },
        other => Err(format!(
            "existing destination must remain unchanged, got {other:?} \
                 and {content:?}"
        )),
    }
}

#[path = "filesystem_entry_sink/diagnostics.rs"]
mod diagnostics;
#[path = "filesystem_entry_sink/metadata.rs"]
mod metadata;
#[path = "filesystem_entry_sink/paths.rs"]
mod paths;
#[path = "filesystem_entry_sink/payloads.rs"]
mod payloads;
#[path = "filesystem_entry_sink/prefix.rs"]
mod prefix;
#[path = "filesystem_entry_sink/unicode.rs"]
mod unicode;

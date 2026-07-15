// File:
//   - io_error_context.rs
// Path:
//   - src/filesystem/tests/io_error_context.rs
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
//   - Regression coverage for actionable adapter IO diagnostics.
// - Must-Not:
//   - Depend on localized operating-system error text.
// - Allows:
//   - Assert stable operation and path context around provider failures.
// - Split-When:
//   - Split when another diagnostic surface has independent error policy.
// - Merge-When:
//   - Another test target owns the same adapter context contract.
// - Summary:
//   - Filesystem IO context regression tests.
// - Description:
//   - Ensures native errors retain enough context for production diagnosis.
// - Usage:
//   - Runs through the filesystem crate test target.
// - Defaults:
//   - Assertions ignore localized source-error wording.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Regression coverage for actionable adapter IO diagnostics.
//!
//! Stable operation and path context must surround native source errors.

#[cfg(windows)]
#[path = "support/junction.rs"]
pub mod support;

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{fs, io};

use schoenwald_filesystem::DiagnosticPath;
use schoenwald_filesystem::adapters::driving::local;

static CASE_ID: AtomicUsize = AtomicUsize::new(0);

fn case_root(label: &str) -> PathBuf {
    let id = CASE_ID.fetch_add(
        1,
        Ordering::Relaxed,
    );
    std::env::temp_dir().join(
        format!(
            "schoenwald-filesystem-context-{label}-{}-{id}",
            std::process::id()
        ),
    )
}

fn require_context(
    error: &io::Error,
    operation: &str,
    path: &Path,
) -> Result<(), String> {
    let rendered = error.to_string();
    if !rendered.contains(operation) {
        return Err(format!("missing operation context: {rendered}"));
    }
    let displayed_path = DiagnosticPath::new(path).to_string();
    if !rendered.contains(&displayed_path) {
        return Err(
            format!(
                "missing path context: expected {displayed_path:?} in {rendered:?}"
            ),
        );
    }
    Ok(())
}

fn require_native_source(error: &io::Error) -> Result<(), String> {
    let native_code = error
        .get_ref()
        .and_then(|context| context.source())
        .and_then(|source| source.downcast_ref::<io::Error>())
        .and_then(io::Error::raw_os_error);
    if native_code.is_none() {
        return Err("contextual error discarded its native source".to_owned());
    }
    Ok(())
}

#[test]
fn missing_read_error_includes_operation_and_path() -> Result<(), String> {
    let path = case_root("missing-read");

    let result = local::read_bytes(&path);
    let Err(error) = result else {
        return Err("missing path unexpectedly read bytes".to_owned());
    };
    require_context(
        &error,
        "read file",
        &path,
    )?;
    require_native_source(&error)
}

#[test]
fn failed_write_error_includes_operation_and_path() -> Result<(), String> {
    let path = case_root("failed-write");
    fs::create_dir_all(&path).map_err(|error| error.to_string())?;
    let result = local::write_bytes(
        &path, b"payload", false,
    );
    fs::remove_dir_all(&path).map_err(|error| error.to_string())?;
    let Err(error) = result else {
        return Err("directory unexpectedly accepted file bytes".to_owned());
    };
    require_context(
        &error,
        "write file",
        &path,
    )
}

#[test]
fn failed_directory_error_has_context() -> Result<(), String> {
    let root = case_root("failed-directory");
    let blocker = root.join("blocker");
    let path = blocker.join("child");
    fs::create_dir_all(&root).map_err(|error| error.to_string())?;
    drop(fs::File::create(&blocker).map_err(|error| error.to_string())?);
    let result = local::create_dir_all(&path);
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    let Err(error) = result else {
        return Err("file ancestor allowed directory creation".to_owned());
    };
    require_context(
        &error,
        "create directory tree",
        &path,
    )
}

#[test]
fn missing_length_error_includes_operation_and_path() -> Result<(), String> {
    let path = case_root("missing-length");
    let result = local::file_len(&path);
    let Err(error) = result else {
        return Err("missing path unexpectedly reported a length".to_owned());
    };
    require_context(
        &error,
        "inspect file metadata",
        &path,
    )
}

#[test]
fn missing_canonical_error_has_context() -> Result<(), String> {
    let path = case_root("missing-canonical");
    let result = local::canonicalize(&path);
    let Err(error) = result else {
        return Err("missing path unexpectedly canonicalized".to_owned());
    };
    require_context(
        &error,
        "canonicalize path",
        &path,
    )
}

#[test]
fn missing_tree_error_includes_operation_and_path() -> Result<(), String> {
    let path = case_root("missing-tree");
    let result = local::regular_files(&path);
    let Err(error) = result else {
        return Err("missing root unexpectedly produced a tree".to_owned());
    };
    require_context(
        &error,
        "inspect traversal root",
        &path,
    )
}

#[test]
fn directory_length_error_includes_operation_and_path() -> Result<(), String> {
    let path = case_root("directory-length");
    fs::create_dir_all(&path).map_err(|error| error.to_string())?;

    let result = local::file_len(&path);

    fs::remove_dir_all(&path).map_err(|error| error.to_string())?;
    let Err(error) = result else {
        return Err("directory unexpectedly reported a file length".to_owned());
    };
    require_context(
        &error,
        "inspect file metadata",
        &path,
    )
}

#[test]
fn file_traversal_error_includes_operation_and_path() -> Result<(), String> {
    let path = case_root("file-traversal");
    drop(fs::File::create(&path).map_err(|error| error.to_string())?);

    let result = local::regular_files(&path);

    fs::remove_file(&path).map_err(|error| error.to_string())?;
    let Err(error) = result else {
        return Err("file unexpectedly accepted as traversal root".to_owned());
    };
    require_context(
        &error,
        "inspect traversal root",
        &path,
    )
}

#[cfg(windows)]
#[test]
fn linked_access_error_includes_operation_and_path() -> Result<(), String> {
    let root = case_root("linked-access");
    let target = root.join("target");
    let link = root.join("link");
    local::write_bytes(
        &target.join("private.bin"),
        b"private",
        true,
    )
    .map_err(|error| error.to_string())?;
    support::create_junction(
        &link, &target,
    )?;

    let result = local::read_bytes(&link.join("private.bin"));

    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    let Err(error) = result else {
        return Err("linked access unexpectedly read bytes".to_owned());
    };
    require_context(
        &error,
        "validate filesystem access",
        &link,
    )
}

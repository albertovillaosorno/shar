// File:
//   - write_parent_creation.rs
// Path:
//   - src/filesystem/tests/write_parent_creation.rs
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
//   - Regression coverage for write parent-creation requests.
// - Must-Not:
//   - Depend on local storage or caller-specific output policy.
// - Allows:
//   - Record port calls and reject current-directory creation noise.
// - Split-When:
//   - Split when another write orchestration invariant gains fixtures.
// - Merge-When:
//   - Another test file owns the same parent-creation behavior.
// - Summary:
//   - Write parent-creation regression tests.
// - Description:
//   - Prevents writes from requesting a no-op current directory creation.
// - Usage:
//   - Runs through the filesystem crate test target.
// - Defaults:
//   - Only a meaningful missing parent is created.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Regression coverage for write parent-creation requests.
//!
//! An explicit current-directory marker is not a missing parent directory.

#[cfg(windows)]
#[path = "support/junction.rs"]
pub mod support;

use std::cell::Cell;
use std::path::Path;
use std::{fs, io};

use schoenwald_filesystem::adapters::driving::local;
use schoenwald_filesystem::application::WriteFile;
use schoenwald_filesystem::ports::FileWriter;

#[derive(Default)]
struct RecordingWriter {
    create_calls: Cell<usize>,
    write_calls: Cell<usize>,
}

impl FileWriter for RecordingWriter {
    fn create_dir_all(
        &self,
        _path: &Path,
    ) -> io::Result<()> {
        let next_count = self
            .create_calls
            .get()
            .saturating_add(1);
        self.create_calls
            .set(next_count);
        Ok(())
    }

    fn write_bytes(
        &self,
        _path: &Path,
        _bytes: &[u8],
    ) -> io::Result<()> {
        let next_count = self
            .write_calls
            .get()
            .saturating_add(1);
        self.write_calls
            .set(next_count);
        Ok(())
    }
}

#[test]
fn current_directory_parent_is_not_created() -> Result<(), String> {
    let writer = RecordingWriter::default();

    WriteFile::bytes(
        &writer,
        Path::new("./report.txt"),
        b"report",
        true,
    )
    .map_err(|error| error.to_string())?;

    if writer
        .create_calls
        .get()
        != 0
    {
        return Err("write requested current-directory creation".to_owned());
    }
    Ok(())
}

#[cfg(windows)]
#[test]
fn write_rejects_linked_parent_without_parent_creation() -> Result<(), String> {
    let root = std::env::temp_dir().join(
        format!(
            "schoenwald-filesystem-linked-write-{}",
            std::process::id()
        ),
    );
    let target = root.join("target");
    let link = root.join("link");
    fs::create_dir_all(&target).map_err(|error| error.to_string())?;
    support::create_junction(
        &link, &target,
    )?;

    let escaped = target.join("escaped.txt");
    let result = local::write_text(
        &link.join("escaped.txt"),
        "escaped",
        false,
    );
    let escaped_exists = escaped.exists();

    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    let Err(error) = result else {
        return Err("write followed linked parent".to_owned());
    };
    if error.kind() != io::ErrorKind::InvalidInput {
        return Err(
            format!(
                "unexpected linked-write error kind: {:?}",
                error.kind()
            ),
        );
    }
    if escaped_exists {
        return Err("linked write created target content".to_owned());
    }
    Ok(())
}

#[test]
fn filesystem_root_parent_is_not_created() -> Result<(), String> {
    let writer = RecordingWriter::default();
    let destination = if cfg!(windows) {
        Path::new(r"C:\report.txt")
    } else {
        Path::new("/report.txt")
    };

    WriteFile::bytes(
        &writer,
        destination,
        b"report",
        true,
    )
    .map_err(|error| error.to_string())?;

    if writer
        .create_calls
        .get()
        != 0
    {
        return Err("write requested filesystem-root creation".to_owned());
    }
    Ok(())
}

#[test]
fn named_parent_chain_before_parent_marker_is_created() -> Result<(), String> {
    let writer = RecordingWriter::default();

    WriteFile::bytes(
        &writer,
        Path::new("missing/../report.txt"),
        b"report",
        true,
    )
    .map_err(|error| error.to_string())?;

    if writer
        .create_calls
        .get()
        != 1
    {
        return Err("named lexical parent chain was not created".to_owned());
    }
    Ok(())
}

#[test]
fn parent_marker_write_destination_is_rejected() -> Result<(), String> {
    let writer = RecordingWriter::default();
    let result = WriteFile::bytes(
        &writer,
        Path::new("scratch/.."),
        b"report",
        true,
    );

    let Err(error) = result else {
        return Err(
            "parent marker unexpectedly accepted file bytes".to_owned(),
        );
    };
    if error.kind() != io::ErrorKind::InvalidInput {
        return Err(
            format!(
                "unexpected write-destination error kind: {:?}",
                error.kind()
            ),
        );
    }
    if writer
        .create_calls
        .get()
        != 0
    {
        return Err(
            "invalid write destination created parent state".to_owned(),
        );
    }
    if writer
        .write_calls
        .get()
        != 0
    {
        return Err("invalid write destination reached provider".to_owned());
    }
    Ok(())
}

#[test]
fn directory_syntax_write_destination_is_rejected() -> Result<(), String> {
    let writer = RecordingWriter::default();
    let result = WriteFile::bytes(
        &writer,
        Path::new("report/"),
        b"report",
        true,
    );

    let Err(error) = result else {
        return Err("directory syntax accepted file bytes".to_owned());
    };
    if error.kind() != io::ErrorKind::InvalidInput {
        return Err(
            format!(
                "unexpected directory-syntax error kind: {:?}",
                error.kind()
            ),
        );
    }
    if writer
        .create_calls
        .get()
        != 0
    {
        return Err("directory syntax created parent state".to_owned());
    }
    if writer
        .write_calls
        .get()
        != 0
    {
        return Err("directory syntax reached the provider".to_owned());
    }
    Ok(())
}

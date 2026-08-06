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
//   - Output inventory outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Output inventory outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Output inventory outbound adapter.

use std::collections::BTreeSet;
use std::path::Path;

use schoenwald_filesystem::adapters::driving::local::{
    canonicalize, file_len, path_kind, regular_files,
};
use schoenwald_filesystem::{PathKind, validate_portable_path};

use crate::domain::{
    DirectorySummary, OutputSummary, PipelineError, PipelineOutcome,
};
use crate::ports::OutputInventory;

/// Reject directory labels that can escape or ambiguously address the root.
fn validate_directory_name(name: &str) -> PipelineOutcome<()> {
    if name.is_empty()
        || name != name.trim()
        || matches!(name, "." | "..")
        || name.chars().any(|character| {
            character.is_control()
                || character == '/'
                || character == char::from(92)
                || character == ':'
        })
    {
        return Err(PipelineError::new(format!(
            "invalid named output directory: {name:?}"
        )));
    }
    let validation = validate_portable_path(Path::new(name));
    validation.map_err(|error| PipelineError::new(error.to_string()))
}

/// Builds one public-safe output-inventory I/O diagnostic.
fn inventory_io_error(action: &str, error: &std::io::Error) -> PipelineError {
    PipelineError::new(format!("{action} failed ({:?})", error.kind()))
}

/// Returns one public file label without its physical ancestors.
fn public_output_file(file: &Path) -> String {
    file.file_name()
        .and_then(|value| value.to_str())
        .filter(|value| !value.is_empty())
        .unwrap_or("unnamed-file")
        .to_owned()
}

/// Inventory one validated caller-selected output directory.
fn summarize_named_directory(
    root: &Path,
    name: &'static str,
) -> PipelineOutcome<DirectorySummary> {
    validate_directory_name(name)?;
    let directory = root.join(name);
    let files = match path_kind(&directory) {
        Ok(PathKind::Directory) => regular_files(&directory)
            .map_err(|error| {
                inventory_io_error(&format!("inventory output/{name}"), &error)
            })?
            .len(),
        Ok(PathKind::Missing) => 0,
        Ok(PathKind::File | PathKind::Other) => {
            return Err(PipelineError::new(format!(
                "output/{name} is not a directory"
            )));
        },
        Err(error) => {
            return Err(inventory_io_error(
                &format!("inspect output/{name}"),
                &error,
            ));
        },
    };
    Ok(DirectorySummary { name, files })
}

/// Adds one file length without silently saturating the output total.
fn checked_byte_total(
    total: u64,
    length: u64,
    file: &Path,
) -> PipelineOutcome<u64> {
    total.checked_add(length).ok_or_else(|| {
        PipelineError::new(format!(
            "output byte total overflowed at {}",
            public_output_file(file),
        ))
    })
}

/// Local filesystem provider for output inventory evidence.
#[derive(Debug, Default, Clone, Copy)]
pub struct FilesystemOutputInventory;

impl OutputInventory for FilesystemOutputInventory {
    fn summarize(
        &self,
        root: &Path,
        directories: &'static [&'static str],
    ) -> PipelineOutcome<OutputSummary> {
        let mut seen_directories = BTreeSet::new();
        for &name in directories {
            validate_directory_name(name)?;
            let portable_identity = name.to_ascii_lowercase();
            if !seen_directories.insert(portable_identity) {
                return Err(PipelineError::new(format!(
                    "duplicate named output directory: {name}"
                )));
            }
        }
        let root_kind = path_kind(root).map_err(|error| {
            inventory_io_error("inspect output root", &error)
        })?;
        let files = match root_kind {
            PathKind::Directory => regular_files(root).map_err(|error| {
                inventory_io_error("inventory output root", &error)
            })?,
            PathKind::Missing => Vec::new(),
            PathKind::File | PathKind::Other => {
                return Err(PipelineError::new(
                    "output root is not a directory",
                ));
            },
        };
        let mut bytes = 0u64;
        for file in &files {
            let length = file_len(file).map_err(|error| {
                inventory_io_error(
                    &format!("inspect output/{}", public_output_file(file)),
                    &error,
                )
            })?;
            bytes = checked_byte_total(bytes, length, file)?;
        }
        let mut directory_summaries = Vec::with_capacity(directories.len());
        for &name in directories {
            directory_summaries.push(summarize_named_directory(root, name)?);
        }
        Ok(OutputSummary {
            root: canonicalize(root).unwrap_or_else(|_| root.to_path_buf()),
            files: files.len(),
            bytes,
            directories: directory_summaries,
        })
    }
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../tests/migration/pipeline/unit/adapter-outbound/output_inventory_tests.rs"]
mod tests;

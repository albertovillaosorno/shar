// File:
//   - output_inventory.rs
// Path:
//   - src/pipeline/src/adapters/driven/output_inventory.rs
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
//   - Local filesystem implementation of pipeline output inventory.
// - Must-Not:
//   - Render CLI text or choose pipeline execution commands.
// - Allows:
//   - Compose shared filesystem mechanisms into pipeline output evidence.
// - Split-When:
//   - Split when another storage provider needs an independent adapter.
// - Merge-When:
//   - Another adapter owns the same local output inventory contract.
// - Summary:
//   - Filesystem output-inventory adapter.
// - Description:
//   - Maps shared local IO evidence into pipeline domain summaries.
// - Usage:
//   - Selected by the pipeline driving CLI after successful execution.
// - Defaults:
//   - Missing roots and named directories produce zero-count summaries.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Driven adapter for pipeline output inventory.
//!
//! Reusable local IO comes from `schoenwald-filesystem`.
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
        || matches!(
            name,
            "." | ".."
        )
        || name
            .chars()
            .any(
                |character| {
                    character.is_control()
                        || character == '/'
                        || character == char::from(92)
                        || character == ':'
                },
            )
    {
        return Err(
            PipelineError::new(
                format!("invalid named output directory: {name:?}"),
            ),
        );
    }
    let validation = validate_portable_path(Path::new(name));
    validation.map_err(|error| PipelineError::new(error.to_string()))
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
            .map_err(
                |error| {
                    PipelineError::new(
                        format!(
                            "failed to inventory {}: {error}",
                            directory.display()
                        ),
                    )
                },
            )?
            .len(),
        Ok(PathKind::Missing) => 0,
        Ok(PathKind::File | PathKind::Other) => {
            return Err(
                PipelineError::new(
                    format!(
                        "named output is not a directory: {}",
                        directory.display()
                    ),
                ),
            );
        }
        Err(error) => {
            return Err(
                PipelineError::new(
                    format!(
                        "failed to inspect {}: {error}",
                        directory.display()
                    ),
                ),
            );
        }
    };
    Ok(
        DirectorySummary {
            name,
            files,
        },
    )
}

/// Adds one file length without silently saturating the output total.
fn checked_byte_total(
    total: u64,
    length: u64,
    file: &Path,
) -> PipelineOutcome<u64> {
    total
        .checked_add(length)
        .ok_or_else(
            || {
                PipelineError::new(
                    format!(
                        "output byte total overflowed at {}",
                        file.display()
                    ),
                )
            },
        )
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
                return Err(
                    PipelineError::new(
                        format!("duplicate named output directory: {name}"),
                    ),
                );
            }
        }
        let root_kind = path_kind(root).map_err(
            |error| {
                PipelineError::new(
                    format!(
                        "failed to inspect {}: {error}",
                        root.display()
                    ),
                )
            },
        )?;
        let files = match root_kind {
            PathKind::Directory => regular_files(root).map_err(
                |error| {
                    PipelineError::new(
                        format!(
                            "failed to inventory {}: {error}",
                            root.display()
                        ),
                    )
                },
            )?,
            PathKind::Missing => Vec::new(),
            PathKind::File | PathKind::Other => {
                return Err(
                    PipelineError::new(
                        format!(
                            "output root is not a directory: {}",
                            root.display()
                        ),
                    ),
                );
            }
        };
        let mut bytes = 0u64;
        for file in &files {
            let length = file_len(file).map_err(
                |error| {
                    PipelineError::new(
                        format!(
                            "failed to inspect {}: {error}",
                            file.display()
                        ),
                    )
                },
            )?;
            bytes = checked_byte_total(
                bytes, length, file,
            )?;
        }
        let mut directory_summaries = Vec::with_capacity(directories.len());
        for &name in directories {
            directory_summaries.push(
                summarize_named_directory(
                    root, name,
                )?,
            );
        }
        Ok(
            OutputSummary {
                root: canonicalize(root).unwrap_or_else(|_| root.to_path_buf()),
                files: files.len(),
                bytes,
                directories: directory_summaries,
            },
        )
    }
}

#[cfg(test)]
#[path = "output_inventory_tests.rs"]
mod tests;

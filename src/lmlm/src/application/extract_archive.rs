// File:
//   - extract_archive.rs
// Path:
//   - src/lmlm/src/application/extract_archive.rs
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
//   - The validated LMLM extraction application command.
// - Must-Not:
//   - Read files, write payloads, or parse command-line arguments directly.
// - Allows:
//   - Load, parse, and publish through explicit ports.
// - Split-When:
//   - Split when validation and publication become independent commands.
// - Merge-When:
//   - Another use case owns the complete archive extraction sequence.
// - Summary:
//   - Application command for validated LMLM extraction.
// - Description:
//   - Coordinates source loading, fail-closed parsing, and publication.
// - Usage:
//   - Invoked by driving adapters after selecting concrete ports.
// - Defaults:
//   - Publication begins only after parsing succeeds.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Application command for validated LMLM extraction.
//!
//! The use case preserves stage-specific failures while depending only on
//! ports.
use std::io;
use std::path::{Path, PathBuf};

use schoenwald_filesystem::DiagnosticPath;

use crate::diagnostic::EscapedText;
use crate::domain::{LmlmError, parse};
use crate::ports::{ArchiveSource, EntrySink};

/// Failure from one extraction command stage.
#[derive(Debug)]
pub enum ExtractArchiveError {
    /// Archive source could not be read.
    Read {
        /// Input path that failed.
        path: PathBuf,
        /// Underlying storage failure.
        source: io::Error,
    },
    /// Archive bytes failed domain validation.
    Parse {
        /// Input path whose bytes were rejected.
        path: PathBuf,
        /// Typed parser failure.
        source: LmlmError,
    },
    /// Validated entries could not be published.
    Materialize {
        /// Output root that failed.
        path: PathBuf,
        /// Underlying sink failure.
        source: io::Error,
    },
}

impl core::fmt::Display for ExtractArchiveError {
    fn fmt(
        &self,
        formatter: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        match self {
            Self::Read {
                path,
                source,
            } => {
                let source_text = source.to_string();
                write!(
                    formatter,
                    "read {}: {}",
                    DiagnosticPath::new(path),
                    EscapedText::new(&source_text)
                )
            }
            Self::Parse {
                path,
                source,
            } => {
                write!(
                    formatter,
                    "parse {}: {source}",
                    DiagnosticPath::new(path)
                )
            }
            Self::Materialize {
                path,
                source,
            } => {
                let source_text = source.to_string();
                write!(
                    formatter,
                    "materialize {}: {}",
                    DiagnosticPath::new(path),
                    EscapedText::new(&source_text)
                )
            }
        }
    }
}

impl std::error::Error for ExtractArchiveError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Read {
                source,
                ..
            }
            | Self::Materialize {
                source,
                ..
            } => Some(source),
            Self::Parse {
                source,
                ..
            } => Some(source),
        }
    }
}

/// Stateless validated extraction use case.
#[derive(Debug, Clone, Copy)]
pub struct ExtractArchive;

impl ExtractArchive {
    /// Executes one complete archive extraction.
    ///
    /// # Errors
    ///
    /// Returns a typed stage failure while preserving the owning path.
    pub fn execute(
        source: &impl ArchiveSource,
        sink: &impl EntrySink,
        input: &Path,
        output_root: &Path,
    ) -> Result<usize, ExtractArchiveError> {
        let archive = source
            .read_archive(input)
            .map_err(
                |read_error| ExtractArchiveError::Read {
                    path: input.to_path_buf(),
                    source: read_error,
                },
            )?;
        let entries = parse(&archive).map_err(
            |parse_error| ExtractArchiveError::Parse {
                path: input.to_path_buf(),
                source: parse_error,
            },
        )?;
        sink.materialize(
            &archive,
            &entries,
            output_root,
        )
        .map_err(
            |write_error| ExtractArchiveError::Materialize {
                path: output_root.to_path_buf(),
                source: write_error,
            },
        )
    }
}

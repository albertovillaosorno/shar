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
//   - Port outbound outbound port.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Port outbound outbound port.
// - Description:
//   - Implements the declared outbound port responsibility for rcf.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Port outbound outbound port.

use std::path::{Path, PathBuf};

use crate::domain::{ArchiveEntry, ArchiveError, IndexRecord};

/// Reads archive bytes by absolute range.
pub trait ArchiveByteReader {
    /// Returns the total archive length.
    ///
    /// # Errors
    ///
    /// Returns an error when the underlying source cannot report metadata.
    fn len(&self) -> Result<u64, ArchiveError>;

    /// Returns whether the archive has no bytes.
    ///
    /// # Errors
    ///
    /// Returns an error when `len` cannot read source metadata.
    fn is_empty(&self) -> Result<bool, ArchiveError> {
        Ok(self.len()? == 0)
    }

    /// Reads exactly `length` bytes from `offset`.
    ///
    /// # Errors
    ///
    /// Returns an error when the requested range cannot be read in full.
    fn read_range(
        &mut self,
        offset: u64,
        length: u64,
    ) -> Result<Vec<u8>, ArchiveError>;

    /// Reads and verifies exactly `length` bytes from `offset`.
    ///
    /// # Errors
    ///
    /// Returns an error when the provider fails or violates the declared range
    /// contract by returning a different number of bytes.
    fn read_exact_range(
        &mut self,
        offset: u64,
        length: u64,
    ) -> Result<Vec<u8>, ArchiveError> {
        let bytes = self.read_range(offset, length)?;
        let actual = u64::try_from(bytes.len()).map_err(|source| {
            ArchiveError::invalid_archive(format!(
                "reader byte count does not fit u64: {source}"
            ))
        })?;
        if actual != length {
            return Err(ArchiveError::invalid_archive(format!(
                "reader returned {actual} bytes for requested {length}"
            )));
        }
        Ok(bytes)
    }
}

/// Writes extracted archive payloads.
pub trait EntrySink {
    /// Validates every planned entry before payload output begins.
    ///
    /// The default accepts all entries for sinks without path-specific rules.
    /// Implementations must not write output from this preflight method.
    ///
    /// # Errors
    ///
    /// Returns an error when the archive directory or any entry cannot be
    /// represented by the sink.
    fn prepare_archive(
        &mut self,
        _archive_stem: &str,
        _entries: &[ArchiveEntry],
    ) -> Result<(), ArchiveError> {
        Ok(())
    }

    /// Writes one extracted entry.
    ///
    /// # Errors
    ///
    /// Returns an error when the entry name is unsafe or filesystem output
    /// fails.
    fn write_entry(
        &mut self,
        archive_stem: &str,
        entry_name: &str,
        payload: &[u8],
    ) -> Result<PathBuf, ArchiveError>;
}

/// Provides archive source bytes and display metadata.
pub trait ArchiveSource {
    /// Opens a byte reader.
    ///
    /// # Errors
    ///
    /// Returns an error when the archive source cannot be opened.
    fn open_reader(
        &self,
    ) -> Result<Box<dyn ArchiveByteReader + '_>, ArchiveError>;

    /// Returns the archive stem used for output directories.
    ///
    /// # Errors
    ///
    /// Returns an error when the source cannot provide a safe UTF-8 stem.
    fn archive_stem(&self) -> Result<String, ArchiveError>;
}

/// Optional diagnostics hook for extraction progress.
pub trait ExtractionObserver {
    /// Called after one entry is extracted.
    ///
    /// # Errors
    ///
    /// Returns an error when observer-side diagnostics cannot be recorded.
    fn entry_extracted(
        &mut self,
        entry: &IndexRecord,
        output_path: &Path,
    ) -> Result<(), ArchiveError>;
}

/// No-op extraction observer.
#[derive(Debug, Default, Clone, Copy)]
pub struct NoopObserver;

impl ExtractionObserver for NoopObserver {
    fn entry_extracted(
        &mut self,
        _entry: &IndexRecord,
        _output_path: &Path,
    ) -> Result<(), ArchiveError> {
        Ok(())
    }
}

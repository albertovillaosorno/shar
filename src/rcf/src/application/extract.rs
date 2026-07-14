// File:
//   - extract.rs
// Path:
//   - src/rcf/src/application/extract.rs
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
//   - rcf use-case orchestration for application extract.
// - Must-Not:
//   - Depend on driven adapters, parse local routes, or encode writer-specific
//   - syntax.
// - Allows:
//   - Use-case orchestration, planning, reporting, and calls through declared
//   - ports.
// - Split-When:
//   - Split when extract contains two independently testable contracts.
// - Merge-When:
//   - Another rcf module owns the same application boundary with no distinct
//   - invariant.
// - Summary:
//   - Archive extraction command use cases.
// - Description:
//   - Defines extract data and behavior for rcf application.
// - Usage:
//   - Called by entrypoints after ports and adapters are selected by the
//   - caller.
// - Defaults:
//   - No concrete adapter is selected unless the caller supplies one through a
//   - port.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Archive extraction command use cases.
//!
//! This boundary keeps archive extraction command use cases explicit and
//! returns deterministic results to rcf callers.
use crate::application::ArchiveParser;
use crate::domain::{ArchiveEntry, ArchiveError, IndexRecord};
use crate::ports::{ArchiveSource, EntrySink, ExtractionObserver};

/// Extraction report for one archive.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtractionReport {
    /// Archive stem used as the output directory name.
    pub archive_stem: String,
    /// Source archive byte length.
    pub archive_bytes: u64,
    /// Sum of extracted payload bytes.
    pub extracted_bytes: u64,
    /// Number of entries in the archive.
    pub entry_count: usize,
    /// Number of zero-length entries.
    pub zero_length_entries: usize,
}

impl ExtractionReport {
    /// Returns source bytes not represented as entry payload bytes.
    #[must_use]
    pub const fn overhead_bytes(&self) -> u64 {
        self.archive_bytes
            .saturating_sub(self.extracted_bytes)
    }

    /// Returns the extracted/source byte ratio.
    // This scoped expectation preserves a documented boundary with explicit
    // validation.
    #[expect(
        clippy::as_conversions,
        clippy::cast_precision_loss,
        reason = "The ratio is human-facing diagnostics; exact byte counts \
                  remain available."
    )]
    #[must_use]
    pub fn extracted_ratio(&self) -> f64 {
        if self.archive_bytes == 0 {
            return 0.0;
        }
        self.extracted_bytes as f64 / self.archive_bytes as f64
    }
}

/// Extracts archives to a sink.
#[derive(Debug, Default, Clone, Copy)]
pub struct Extractor;

impl Extractor {
    /// Extracts one archive.
    ///
    /// # Errors
    ///
    /// Returns an error when archive parsing, byte reads, output writes, or
    /// observer callbacks fail.
    pub fn extract(
        source: &impl ArchiveSource,
        sink: &mut impl EntrySink,
        observer: &mut impl ExtractionObserver,
    ) -> Result<ExtractionReport, ArchiveError> {
        let archive_stem = source.archive_stem()?;
        let mut reader = source.open_reader()?;
        let archive = ArchiveParser::from_reader(reader.as_mut())?;
        sink.prepare_archive(
            &archive_stem,
            &archive.entries,
        )?;
        for entry in &archive.entries {
            let payload = reader.read_exact_range(
                entry.offset,
                entry.length,
            )?;
            let output_path = sink.write_entry(
                &archive_stem,
                &entry.name,
                &payload,
            )?;
            observer.entry_extracted(
                &entry_record(entry),
                &output_path,
            )?;
        }
        Ok(
            ExtractionReport {
                archive_stem,
                archive_bytes: archive.archive_size,
                extracted_bytes: archive.payload_bytes(),
                entry_count: archive
                    .entries
                    .len(),
                zero_length_entries: archive.zero_length_entries(),
            },
        )
    }
}

/// Projects entry metadata into the observer contract without exposing names.
const fn entry_record(entry: &ArchiveEntry) -> IndexRecord {
    IndexRecord {
        hash: entry.hash,
        offset: entry.offset,
        length: entry.length,
    }
}

// File:
//   - extraction_preflight.rs
// Path:
//   - src/rcf/tests/extraction_preflight.rs
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
//   - Caller-visible extraction sink-preflight regressions.
// - Must-Not:
//   - Read private assets, use filesystem adapters, or inspect implementation
//   - details outside public ports.
// - Allows:
//   - Synthetic archives, in-memory readers, and recording sinks.
// - Split-When:
//   - Another extraction transaction boundary needs an independent fixture.
// - Merge-When:
//   - Sink preflight no longer needs a distinct integration target.
// - Summary:
//   - Protects extraction from partial sink writes.
// - Description:
//   - Proves every archive entry is accepted before payload output begins.
// - Usage:
//   - Run through the RCF crate integration test target.
// - Defaults:
//   - No local files, generated assets, or external processes are required.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Caller-visible extraction sink-preflight regressions.
//!
//! A synthetic two-entry archive proves that a later sink rejection cannot
//! leave an earlier entry written.

use std::path::PathBuf;

use rcf::Extractor;
use rcf::domain::{ArchiveEntry, ArchiveError};
use rcf::ports::{ArchiveByteReader, ArchiveSource, EntrySink, NoopObserver};
use schoenwald_cli as _;
use schoenwald_filesystem as _;
// This focused regression consumes only the shared two-entry builder.
#[expect(
    dead_code,
    reason = "This focused extraction regression consumes only the shared \
              two-entry archive builder."
)]
#[path = "fixture/archive.rs"]
mod fixture;

use fixture::archive_with_stored_names;

#[test]
fn sink_rejection_happens_before_any_entry_write() {
    let fixture = archive_with_stored_names(
        &[
            b"sound/file.rsd\0",
            b"sound/CON\0",
        ],
    );
    assert!(
        fixture.is_ok(),
        "the two-entry archive fixture must be constructible"
    );
    let Ok(bytes) = fixture else {
        return;
    };
    let source = MemorySource {
        bytes,
    };
    let mut sink = RejectingSink::default();
    let mut observer = NoopObserver;

    let result = Extractor::extract(
        &source,
        &mut sink,
        &mut observer,
    );

    assert!(
        matches!(
            result,
            Err(ArchiveError::UnsafeEntryPath(path)) if path == "sound/CON"
        ),
        "the sink must reject its unsafe second entry"
    );
    assert!(
        sink.written_names
            .is_empty(),
        "all sink paths must be accepted before the first write"
    );
}

struct MemorySource {
    bytes: Vec<u8>,
}

impl ArchiveSource for MemorySource {
    fn open_reader(
        &self
    ) -> Result<Box<dyn ArchiveByteReader + '_>, ArchiveError> {
        Ok(
            Box::new(
                MemoryReader {
                    bytes: self
                        .bytes
                        .clone(),
                },
            ),
        )
    }

    fn archive_stem(&self) -> Result<String, ArchiveError> {
        Ok("archive".to_owned())
    }
}

struct MemoryReader {
    bytes: Vec<u8>,
}

impl ArchiveByteReader for MemoryReader {
    fn len(&self) -> Result<u64, ArchiveError> {
        u64::try_from(
            self.bytes
                .len(),
        )
        .map_err(
            |source| {
                ArchiveError::invalid_archive(
                    format!("fixture length does not fit u64: {source}"),
                )
            },
        )
    }

    fn read_range(
        &mut self,
        offset: u64,
        length: u64,
    ) -> Result<Vec<u8>, ArchiveError> {
        let start = usize::try_from(offset).map_err(
            |source| {
                ArchiveError::invalid_archive(
                    format!("fixture offset does not fit usize: {source}"),
                )
            },
        )?;
        let count = usize::try_from(length).map_err(
            |source| {
                ArchiveError::invalid_archive(
                    format!("fixture length does not fit usize: {source}"),
                )
            },
        )?;
        let end = start
            .checked_add(count)
            .ok_or_else(
                || ArchiveError::invalid_archive("fixture range overflow"),
            )?;
        self.bytes
            .get(start..end)
            .map(ToOwned::to_owned)
            .ok_or_else(
                || ArchiveError::invalid_archive("fixture range exceeds bytes"),
            )
    }
}

#[derive(Default)]
struct RejectingSink {
    written_names: Vec<String>,
}

impl EntrySink for RejectingSink {
    fn prepare_archive(
        &mut self,
        _archive_stem: &str,
        entries: &[ArchiveEntry],
    ) -> Result<(), ArchiveError> {
        let rejected = entries
            .iter()
            .find(|entry| entry.name == "sound/CON");
        if let Some(entry) = rejected {
            return Err(ArchiveError::unsafe_entry_path(&entry.name));
        }
        Ok(())
    }

    fn write_entry(
        &mut self,
        _archive_stem: &str,
        entry_name: &str,
        _payload: &[u8],
    ) -> Result<PathBuf, ArchiveError> {
        if entry_name == "sound/CON" {
            return Err(ArchiveError::unsafe_entry_path(entry_name));
        }
        self.written_names
            .push(entry_name.to_owned());
        Ok(PathBuf::from(entry_name))
    }
}

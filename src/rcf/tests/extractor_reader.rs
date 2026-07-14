// File:
//   - extractor_reader.rs
// Path:
//   - src/rcf/tests/extractor_reader.rs
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
//   - Caller-visible extractor reader-lifetime regressions.
// - Must-Not:
//   - Read private assets or depend on filesystem adapters.
// - Allows:
//   - Synthetic archive readers, sources, sinks, and public extraction calls.
// - Split-When:
//   - Another extraction orchestration concern needs independent fixtures.
// - Merge-When:
//   - Reader lifetime no longer needs a distinct integration boundary.
// - Summary:
//   - Protects one-reader RCF extraction.
// - Description:
//   - Verifies parsing and payload reads share one opened source snapshot.
// - Usage:
//   - Run through the RCF crate integration test target.
// - Defaults:
//   - No local files, generated assets, or external processes are required.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - true
//   - Reason: Reader-lifetime regressions and their synthetic archive boundary
//   - remain cohesive while validating one opened extraction snapshot.
//

//! Caller-visible regressions for extractor reader lifetime.
//!
//! A synthetic source records opens while serving one lawful archive snapshot.

use std::cell::Cell;
use std::path::PathBuf;

use rcf::Extractor;
use rcf::domain::ArchiveError;
use rcf::ports::{ArchiveByteReader, ArchiveSource, EntrySink, NoopObserver};
use schoenwald_cli as _;
use schoenwald_filesystem as _;

const ARCHIVE_LENGTH: u64 = 0x1001;
const CATALOG_LENGTH: usize = 0x810;
const MAGIC: &[u8] = b"RADCORE CEMENT LIBRARY";

#[test]
fn extraction_uses_one_reader_snapshot() {
    let source = CountingSource {
        open_count: Cell::new(0),
    };
    let mut sink = RecordingSink::default();
    let mut observer = NoopObserver;

    let result = Extractor::extract(
        &source,
        &mut sink,
        &mut observer,
    );

    assert!(
        result.is_ok(),
        "the synthetic archive must extract"
    );
    assert_eq!(
        source
            .open_count
            .get(),
        1,
        "parsing and payload reads must share one source snapshot"
    );
    assert_eq!(
        sink.payload,
        vec![7]
    );
}

#[test]
fn extraction_rejects_short_payload_reads() {
    let source = PayloadSource {
        payload: Vec::new(),
    };
    let mut sink = RecordingSink::default();
    let mut observer = NoopObserver;

    let result = Extractor::extract(
        &source,
        &mut sink,
        &mut observer,
    );

    assert!(
        matches!(
            result,
            Err(ArchiveError::InvalidArchive(message))
                if message.contains("reader returned 0 bytes for requested 1")
        ),
        "short payload reads must fail before output"
    );
    assert!(
        sink.payload
            .is_empty(),
        "short payload reads must not reach the sink"
    );
}

#[test]
fn extraction_rejects_oversized_payload_reads() {
    let source = PayloadSource {
        payload: vec![
            7, 8,
        ],
    };
    let mut sink = RecordingSink::default();
    let mut observer = NoopObserver;

    let result = Extractor::extract(
        &source,
        &mut sink,
        &mut observer,
    );

    assert!(
        matches!(
            result,
            Err(ArchiveError::InvalidArchive(message))
                if message.contains("reader returned 2 bytes for requested 1")
        ),
        "oversized payload reads must fail before output"
    );
    assert!(
        sink.payload
            .is_empty(),
        "oversized payload reads must not reach the sink"
    );
}

struct PayloadSource {
    payload: Vec<u8>,
}

impl ArchiveSource for PayloadSource {
    fn open_reader(
        &self
    ) -> Result<Box<dyn ArchiveByteReader + '_>, ArchiveError> {
        Ok(
            Box::new(
                SyntheticReader {
                    payload: self
                        .payload
                        .clone(),
                },
            ),
        )
    }

    fn archive_stem(&self) -> Result<String, ArchiveError> {
        Ok("archive".to_owned())
    }
}

struct CountingSource {
    open_count: Cell<u32>,
}

impl ArchiveSource for CountingSource {
    fn open_reader(
        &self
    ) -> Result<Box<dyn ArchiveByteReader + '_>, ArchiveError> {
        self.open_count
            .set(
                self.open_count
                    .get()
                    .saturating_add(1),
            );
        Ok(
            Box::new(
                SyntheticReader {
                    payload: vec![7],
                },
            ),
        )
    }

    fn archive_stem(&self) -> Result<String, ArchiveError> {
        Ok("archive".to_owned())
    }
}

struct SyntheticReader {
    payload: Vec<u8>,
}

impl ArchiveByteReader for SyntheticReader {
    fn len(&self) -> Result<u64, ArchiveError> {
        Ok(ARCHIVE_LENGTH)
    }

    fn read_range(
        &mut self,
        offset: u64,
        length: u64,
    ) -> Result<Vec<u8>, ArchiveError> {
        match (
            offset, length,
        ) {
            (0, 48) => catalog_range(
                0, 48,
            ),
            (0x800, 16) => catalog_range(
                0x800, 16,
            ),
            (0x810, 12) => Ok(
                words(
                    &[
                        97, 0x1000, 1,
                    ],
                ),
            ),
            (0x81c, 4) => Ok(words(&[1])),
            (0x824, 4) => Ok(words(&[2])),
            (0x828, 2) => Ok(
                vec![
                    b'a', 0,
                ],
            ),
            (0x82a, 4) => Ok(words(&[0])),
            (0x1000, 1) => Ok(
                self.payload
                    .clone(),
            ),
            _ => Err(
                ArchiveError::invalid_archive(
                    format!("unexpected synthetic range: {offset:#x}+{length}"),
                ),
            ),
        }
    }
}

fn catalog_prefix() -> Result<Vec<u8>, ArchiveError> {
    let mut bytes = vec![0_u8; CATALOG_LENGTH];
    copy_at(
        &mut bytes, 0, MAGIC,
    )?;
    copy_at(
        &mut bytes,
        32,
        &[
            1, 2, 0, 1,
        ],
    )?;
    copy_at(
        &mut bytes,
        36,
        &0x800_u32.to_le_bytes(),
    )?;
    copy_at(
        &mut bytes,
        44,
        &0x800_u32.to_le_bytes(),
    )?;
    copy_at(
        &mut bytes,
        0x800,
        &1_u32.to_le_bytes(),
    )?;
    copy_at(
        &mut bytes,
        0x804,
        &0x81c_u32.to_le_bytes(),
    )?;
    copy_at(
        &mut bytes,
        0x808,
        &0x1000_u32.to_le_bytes(),
    )?;
    Ok(bytes)
}

fn catalog_range(
    offset: usize,
    length: usize,
) -> Result<Vec<u8>, ArchiveError> {
    let bytes = catalog_prefix()?;
    let end = offset
        .checked_add(length)
        .ok_or_else(
            || ArchiveError::invalid_archive("fixture range overflow"),
        )?;
    let range = bytes
        .get(offset..end)
        .ok_or_else(
            || ArchiveError::invalid_archive("fixture range is invalid"),
        )?;
    Ok(range.to_vec())
}

fn words(values: &[u32]) -> Vec<u8> {
    values
        .iter()
        .flat_map(|value| value.to_le_bytes())
        .collect()
}

fn copy_at(
    bytes: &mut [u8],
    offset: usize,
    value: &[u8],
) -> Result<(), ArchiveError> {
    let end = offset
        .checked_add(value.len())
        .ok_or_else(
            || ArchiveError::invalid_archive("fixture range overflow"),
        )?;
    let target = bytes
        .get_mut(offset..end)
        .ok_or_else(
            || ArchiveError::invalid_archive("fixture range is invalid"),
        )?;
    target.copy_from_slice(value);
    Ok(())
}

#[derive(Default)]
struct RecordingSink {
    payload: Vec<u8>,
}

impl EntrySink for RecordingSink {
    fn write_entry(
        &mut self,
        _archive_stem: &str,
        _entry_name: &str,
        payload: &[u8],
    ) -> Result<PathBuf, ArchiveError> {
        self.payload = payload.to_vec();
        Ok(PathBuf::from("output/a"))
    }
}

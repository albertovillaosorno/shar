// File:
//   - file_rtf_source.rs
// Path:
//   - src/rtf/src/adapters/driven/file_rtf_source.rs
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
//   - Filesystem implementation of the RTF source snapshot port.
// - Must-Not:
//   - Convert text, infer input paths, or publish Markdown.
// - Allows:
//   - Read complete bytes and optional modification-time evidence.
// - Split-When:
//   - Split when streaming input requires an independent adapter.
// - Merge-When:
//   - Another adapter owns the same filesystem source contract.
// - Summary:
//   - Driven file source for RTF documents.
// - Description:
//   - Implements source loading behind the RTF source port.
// - Usage:
//   - Selected by CLI and filesystem composition roots.
// - Defaults:
//   - Missing metadata remains optional evidence.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Filesystem source adapter for RTF documents.
//!
//! The adapter returns complete bytes and weak timestamp provenance through the
//! source port.
use std::path::Path;
use std::time::UNIX_EPOCH;
use std::{fs, io};

use schoenwald_filesystem::adapters::driving::local;

use crate::ports::{RtfSnapshot, RtfSource};

/// Loads RTF snapshots from local files.
#[derive(Debug, Default, Clone, Copy)]
pub struct FileRtfSource;

impl RtfSource for FileRtfSource {
    fn load(
        &self,
        path: &Path,
    ) -> io::Result<RtfSnapshot> {
        let bytes = local::read_bytes(path)?;
        let modified_unix_seconds = fs::metadata(path)
            .ok()
            .and_then(
                |metadata| {
                    metadata
                        .modified()
                        .ok()
                },
            )
            .and_then(
                |modified| {
                    modified
                        .duration_since(UNIX_EPOCH)
                        .ok()
                },
            )
            .and_then(|duration| i64::try_from(duration.as_secs()).ok());
        Ok(
            RtfSnapshot {
                bytes,
                modified_unix_seconds,
            },
        )
    }
}

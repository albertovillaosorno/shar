// File:
//   - rtf_source.rs
// Path:
//   - src/rtf/src/ports/rtf_source.rs
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
//   - Outbound source snapshot contract for RTF documents.
// - Must-Not:
//   - Convert text, infer input paths, or publish Markdown.
// - Allows:
//   - Return complete bytes and optional modification-time evidence.
// - Split-When:
//   - Split when streaming and snapshot reads need independent contracts.
// - Merge-When:
//   - Another port owns the same RTF source snapshot boundary.
// - Summary:
//   - Port for loading RTF source documents.
// - Description:
//   - Isolates conversion use cases from concrete source storage.
// - Usage:
//   - Implemented by driven adapters and supplied by driving adapters.
// - Defaults:
//   - Missing timestamp evidence is represented explicitly.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Outbound port for loading complete RTF source snapshots.
use std::io;
use std::path::Path;

/// Complete source evidence needed by the conversion use case.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RtfSnapshot {
    /// Raw RTF bytes.
    pub bytes: Vec<u8>,
    /// Optional Unix timestamp from weak filesystem provenance.
    pub modified_unix_seconds: Option<i64>,
}

/// Loads RTF bytes and optional provenance evidence.
pub trait RtfSource {
    /// Loads one complete source snapshot.
    ///
    /// # Errors
    ///
    /// Returns an I/O error when the document bytes cannot be read.
    fn load(
        &self,
        path: &Path,
    ) -> io::Result<RtfSnapshot>;
}

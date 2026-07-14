// File:
//   - archive_source.rs
// Path:
//   - src/lmlm/src/ports/archive_source.rs
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
//   - Outbound source-byte loading contract for LMLM archives.
// - Must-Not:
//   - Parse package bytes, choose input paths, or write outputs.
// - Allows:
//   - Return complete archive bytes from a caller-supplied path.
// - Split-When:
//   - Split when streaming and snapshot reads need independent contracts.
// - Merge-When:
//   - Another port owns the same archive-byte loading boundary.
// - Summary:
//   - Port for loading LMLM archive bytes.
// - Description:
//   - Isolates application commands from concrete archive storage.
// - Usage:
//   - Implemented by driven adapters and supplied by driving adapters.
// - Defaults:
//   - No input path is inferred.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Outbound port for loading complete LMLM archive snapshots.
use std::io;
use std::path::Path;

/// Loads archive bytes from a caller-selected source.
pub trait ArchiveSource {
    /// Reads one complete archive snapshot.
    ///
    /// # Errors
    ///
    /// Returns an I/O error when the source cannot be read completely.
    fn read_archive(
        &self,
        path: &Path,
    ) -> io::Result<Vec<u8>>;
}

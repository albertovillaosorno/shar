// File:
//   - file_archive_source.rs
// Path:
//   - src/lmlm/src/adapters/driven/file_archive_source.rs
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
//   - Filesystem implementation of the archive source port.
// - Must-Not:
//   - Parse bytes, infer input paths, or write outputs.
// - Allows:
//   - Read one complete caller-selected file snapshot.
// - Split-When:
//   - Split when streaming input requires an independent adapter.
// - Merge-When:
//   - Another adapter owns the same filesystem snapshot contract.
// - Summary:
//   - Driven file source for LMLM archives.
// - Description:
//   - Implements archive loading behind the source port.
// - Usage:
//   - Selected by CLI and filesystem composition roots.
// - Defaults:
//   - No path is inferred.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Filesystem archive source adapter for LMLM packages.
//!
//! Complete local snapshots delegate to the shared filesystem mechanism.
use std::io;
use std::path::Path;

use schoenwald_filesystem::adapters::driving::local;

use crate::ports::ArchiveSource;

/// Reads complete archive snapshots from local files.
#[derive(Debug, Default, Clone, Copy)]
pub struct FileArchiveSource;

impl ArchiveSource for FileArchiveSource {
    fn read_archive(
        &self,
        path: &Path,
    ) -> io::Result<Vec<u8>> {
        local::read_bytes(path)
    }
}

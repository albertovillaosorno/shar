// File:
//   - filesystem_text_store.rs
// Path:
//   - src/game-manifest/src/adapters/driven/filesystem_text_store.rs
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
//   - Filesystem implementation of complete text artifact storage.
// - Must-Not:
//   - Traverse game trees, classify records, or infer paths.
// - Allows:
//   - Read optional UTF-8 text and write complete text artifacts.
// - Split-When:
//   - Split when atomic publication requires an independent adapter.
// - Merge-When:
//   - Another adapter owns the same filesystem text contract.
// - Summary:
//   - Driven filesystem text store.
// - Description:
//   - Implements manifest text reading and publication behind a port.
// - Usage:
//   - Selected by manifest CLI composition roots.
// - Defaults:
//   - Parent directories are created for explicit outputs.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Filesystem adapter for complete manifest text artifacts.
//!
//! Shared local UTF-8 reads and complete writes remain centralized.
use std::io;
use std::path::Path;

use schoenwald_filesystem::adapters::driving::local;

use crate::ports::TextArtifactStore;

/// Reads and writes local UTF-8 manifest artifacts.
#[derive(Debug, Default, Clone, Copy)]
pub struct FilesystemTextStore;

impl TextArtifactStore for FilesystemTextStore {
    fn read_optional(
        &self,
        path: &Path,
    ) -> io::Result<Option<String>> {
        local::read_optional_utf8(path)
    }

    fn write(
        &self,
        path: &Path,
        text: &str,
    ) -> io::Result<()> {
        local::write_text(
            path, text, true,
        )
    }
}

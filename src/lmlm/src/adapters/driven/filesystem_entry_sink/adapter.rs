// File:
//   - adapter.rs
// Path:
//   - src/lmlm/src/adapters/driven/filesystem_entry_sink/adapter.rs
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
//   - The filesystem implementation of the LMLM entry sink port.
// - Must-Not:
//   - Validate destinations, slice payloads, or perform publication directly.
// - Allows:
//   - Delegate one complete materialization request to the parent facade.
// - Split-When:
//   - Another filesystem sink variant needs independent composition.
// - Merge-When:
//   - The entry sink port no longer has a concrete filesystem adapter.
// - Summary:
//   - Adapts the entry sink port to filesystem materialization.
// - Description:
//   - Keeps port implementation separate from publication mechanics.
// - Usage:
//   - Selected by the LMLM command-line composition root.
// - Defaults:
//   - No output root is inferred.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Filesystem implementation of the LMLM entry sink port.
//!
//! Delegates validated entry publication to the parent materialization facade.

use std::io;
use std::path::Path;

use super::materialize_entries;
use crate::domain::FileEntry;
use crate::ports::EntrySink;

/// Filesystem-backed sink for validated LMLM entries.
#[derive(Debug, Default, Clone, Copy)]
pub struct FilesystemEntrySink;

impl EntrySink for FilesystemEntrySink {
    fn materialize(
        &self,
        archive: &[u8],
        entries: &[FileEntry],
        output_root: &Path,
    ) -> io::Result<usize> {
        materialize_entries(
            archive,
            entries,
            output_root,
        )
    }
}

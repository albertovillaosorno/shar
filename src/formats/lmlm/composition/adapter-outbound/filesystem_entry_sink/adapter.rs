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
//   - Adapter outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Adapter outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for lmlm.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Adapter outbound adapter.

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
        materialize_entries(archive, entries, output_root)
    }
}

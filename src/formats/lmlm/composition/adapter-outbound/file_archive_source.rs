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
//   - File archive source outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - File archive source outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for lmlm.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! File archive source outbound adapter.

use std::io;
use std::path::Path;

use schoenwald_filesystem::adapters::driving::local;

use crate::ports::ArchiveSource;

/// Reads complete archive snapshots from local files.
#[derive(Debug, Default, Clone, Copy)]
pub struct FileArchiveSource;

impl ArchiveSource for FileArchiveSource {
    fn read_archive(&self, path: &Path) -> io::Result<Vec<u8>> {
        local::read_bytes(path)
    }
}

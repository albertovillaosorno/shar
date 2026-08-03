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
//   - Filesystem text store outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Filesystem text store outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for manifest.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Filesystem text store outbound adapter.

use std::io;
use std::path::Path;

use schoenwald_filesystem::adapters::driving::local;

use crate::ports::TextArtifactStore;

/// Reads and writes local UTF-8 manifest artifacts.
#[derive(Debug, Default, Clone, Copy)]
pub struct FilesystemTextStore;

impl TextArtifactStore for FilesystemTextStore {
    fn read_optional(&self, path: &Path) -> io::Result<Option<String>> {
        local::read_optional_utf8(path)
    }

    fn write(&self, path: &Path, text: &str) -> io::Result<()> {
        local::write_text(path, text, true)
    }
}

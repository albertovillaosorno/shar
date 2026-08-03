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
//   - File markdown sink outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - File markdown sink outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for rtf.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! File markdown sink outbound adapter.

use std::io;
use std::path::Path;

use schoenwald_filesystem::adapters::driving::local;

use crate::ports::MarkdownSink;

/// Writes complete Markdown documents to local files.
#[derive(Debug, Default, Clone, Copy)]
pub struct FileMarkdownSink;

impl MarkdownSink for FileMarkdownSink {
    fn write(&self, path: &Path, document: &str) -> io::Result<()> {
        local::write_text(path, document, false)
    }
}

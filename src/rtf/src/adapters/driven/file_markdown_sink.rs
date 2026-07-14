// File:
//   - file_markdown_sink.rs
// Path:
//   - src/rtf/src/adapters/driven/file_markdown_sink.rs
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
//   - Filesystem implementation of the Markdown sink port.
// - Must-Not:
//   - Convert RTF, infer destinations, or print diagnostics.
// - Allows:
//   - Write one complete caller-supplied document to an explicit path.
// - Split-When:
//   - Split when atomic replacement requires an independent adapter.
// - Merge-When:
//   - Another adapter owns the same filesystem publication contract.
// - Summary:
//   - Driven file sink for converted Markdown.
// - Description:
//   - Implements explicit file publication behind the Markdown sink port.
// - Usage:
//   - Selected by the CLI when an output path is supplied.
// - Defaults:
//   - Existing files follow the standard filesystem write contract.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Filesystem sink adapter for complete Markdown documents.
//!
//! The adapter publishes only to explicit caller-supplied destinations.
use std::io;
use std::path::Path;

use schoenwald_filesystem::adapters::driving::local;

use crate::ports::MarkdownSink;

/// Writes complete Markdown documents to local files.
#[derive(Debug, Default, Clone, Copy)]
pub struct FileMarkdownSink;

impl MarkdownSink for FileMarkdownSink {
    fn write(
        &self,
        path: &Path,
        document: &str,
    ) -> io::Result<()> {
        local::write_text(
            path, document, false,
        )
    }
}

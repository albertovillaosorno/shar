// File:
//   - markdown_sink.rs
// Path:
//   - src/rtf/src/ports/markdown_sink.rs
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
//   - Outbound publication contract for complete Markdown documents.
// - Must-Not:
//   - Convert RTF, infer output paths, or print operator diagnostics.
// - Allows:
//   - Write one complete document to a caller-supplied destination.
// - Split-When:
//   - Split when atomic and remote publication need independent contracts.
// - Merge-When:
//   - Another port owns the same complete-document publication boundary.
// - Summary:
//   - Port for publishing converted Markdown.
// - Description:
//   - Keeps file publication outside application and domain layers.
// - Usage:
//   - Implemented by driven adapters and selected by driving adapters.
// - Defaults:
//   - No destination is inferred.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Outbound port for publishing complete Markdown documents.
//!
//! Application commands depend on this contract instead of concrete file IO.
use std::io;
use std::path::Path;

/// Publishes a complete converted document.
pub trait MarkdownSink {
    /// Writes one complete document to an explicit path.
    ///
    /// # Errors
    ///
    /// Returns an I/O error when publication fails.
    fn write(
        &self,
        path: &Path,
        document: &str,
    ) -> io::Result<()>;
}

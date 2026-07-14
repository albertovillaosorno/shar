// File:
//   - entry.rs
// Path:
//   - src/lmlm/src/domain/entry.rs
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
//   - The immutable public identity of one parsed LMLM file payload.
// - Must-Not:
//   - Parse archives, validate byte ranges, or write extracted files.
// - Allows:
//   - Archive-relative path, offset, and size data.
// - Split-When:
//   - Entry identity gains independently versioned domain concepts.
// - Merge-When:
//   - Another LMLM domain record proves the same identity contract.
// - Summary:
//   - Defines the parsed file-entry value object.
// - Description:
//   - Carries the stable path and payload range produced by parsing.
// - Usage:
//   - Returned by parse and consumed by bounded extraction adapters.
// - Defaults:
//   - Values are validated before the parser returns them.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Domain value for one parsed LMLM file entry.
//!
//! The record carries identity and declared payload location only. Parsing,
//! structural validation, and filesystem materialization remain separate.

/// A single extractable file and its archive-relative payload range.
#[derive(Debug)]
pub struct FileEntry {
    /// Path relative to the archive root, using `/` separators.
    pub path: String,
    /// Byte offset of the file data within the archive.
    pub offset: u64,
    /// Length of the file data in bytes.
    pub size: u64,
}

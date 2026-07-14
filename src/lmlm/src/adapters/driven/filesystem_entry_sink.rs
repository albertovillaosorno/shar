// File:
//   - filesystem_entry_sink.rs
// Path:
//   - src/lmlm/src/adapters/driven/filesystem_entry_sink.rs
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
//   - The filesystem entry-sink facade and materialization sequence.
// - Must-Not:
//   - Duplicate destination, payload, or publication implementation details.
// - Allows:
//   - Sequence complete preflight before create-new publication.
// - Split-When:
//   - Another publication backend needs an independent facade.
// - Merge-When:
//   - Another adapter owns the same filesystem materialization sequence.
// - Summary:
//   - Coordinates validated LMLM filesystem materialization.
// - Description:
//   - Preserves the public sink API while delegating cohesive mechanics.
// - Usage:
//   - Called by the LMLM command and direct library consumers.
// - Defaults:
//   - No filesystem mutation begins before every preflight passes.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Filesystem materialization facade for validated LMLM entries.
//!
//! Payload and destination preflight complete before publication begins.

use std::io;
use std::path::Path;

use crate::domain::FileEntry;

mod adapter;
mod destination;
mod inspection;
mod payload;
mod publication;

pub use adapter::FilesystemEntrySink;
use destination::preflight_destinations;
use payload::preflight_payloads;
use publication::publish_entries;

/// Writes every validated entry below `output_root` and returns the file count.
///
/// # Errors
///
/// Returns an I/O error when preflight or create-new publication fails.
pub fn materialize_entries(
    data: &[u8],
    entries: &[FileEntry],
    output_root: &Path,
) -> io::Result<usize> {
    let payloads = preflight_payloads(
        data, entries,
    )?;
    let destinations = preflight_destinations(
        entries,
        output_root,
    )?;
    publish_entries(
        destinations,
        payloads,
    )?;
    Ok(entries.len())
}

#[cfg(test)]
mod tests;

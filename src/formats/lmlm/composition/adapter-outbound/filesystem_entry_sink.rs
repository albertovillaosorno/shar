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
//   - Filesystem entry sink outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Filesystem entry sink outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for lmlm.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Filesystem entry sink outbound adapter.

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
    let payloads = preflight_payloads(data, entries)?;
    let destinations = preflight_destinations(entries, output_root)?;
    publish_entries(destinations, payloads)?;
    Ok(entries.len())
}

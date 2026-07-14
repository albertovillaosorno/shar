// File:
//   - payload.rs
// Path:
//   - src/lmlm/src/domain/payload.rs
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
//   - Bounded public access to one parsed file payload.
// - Must-Not:
//   - Duplicate owned parsing rules or write extracted files.
// - Allows:
//   - Ordered calls across validated LMLM modules.
// - Split-When:
//   - Orchestration gains independently testable state.
// - Merge-When:
//   - Another facade proves the same orchestration contract.
// - Summary:
//   - Owns bounded public access to one parsed file payload.
// - Description:
//   - Keeps the public parser path explicit and deterministic.
// - Usage:
//   - Re-exported through the crate facade.
// - Defaults:
//   - No entry returns before every validation gate passes.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Bounded payload access for parsed LMLM entries.
//!
//! Converts validated offset and size values into a borrowed archive slice.

use super::FileEntry;

/// Returns the data slice for a parsed entry, or `None` if it lies outside the
/// archive bounds.
#[must_use]
pub fn entry_bytes<'a>(
    data: &'a [u8],
    entry: &FileEntry,
) -> Option<&'a [u8]> {
    let start = usize::try_from(entry.offset).ok()?;
    let len = usize::try_from(entry.size).ok()?;
    data.get(start..start.checked_add(len)?)
}

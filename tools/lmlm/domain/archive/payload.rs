// Copyright:
//   - Copyright © 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Payload domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Payload domain module.
// - Description:
//   - Implements the declared domain module responsibility for lmlm.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Payload domain module.

use super::FileEntry;

/// Returns the data slice for a parsed entry, or `None` if it lies outside the
/// archive bounds.
#[must_use]
pub fn entry_bytes<'a>(data: &'a [u8], entry: &FileEntry) -> Option<&'a [u8]> {
    let start = usize::try_from(entry.offset).ok()?;
    let len = usize::try_from(entry.size).ok()?;
    data.get(start..start.checked_add(len)?)
}
